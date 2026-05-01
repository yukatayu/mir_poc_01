#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Sequence

import current_l2_source_sample_regression as regression


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
DEFAULT_ARTIFACT_ROOT = REPO_ROOT / "target" / "current-l2-model-check-carrier"
DEFAULT_RUN_LABEL = "model-check-carrier"


@dataclass(frozen=True)
class PipelineCommand:
    name: str
    argv: tuple[str, ...]


@dataclass(frozen=True)
class ModelCheckCarrierPipelinePlan:
    sample_stem: str
    smoke_mode: str
    formal_hook_command: PipelineCommand
    model_check_command: PipelineCommand
    formal_hook_output: Path
    model_check_output: Path


@dataclass(frozen=True)
class ConformanceSummary:
    formal_hook_pair_count: int
    model_check_carrier_count: int
    matched_pairs: int


def artifact_path(
    artifact_root: Path,
    lane: str,
    run_label: str,
    sample_stem: str,
    suffix: str,
) -> Path:
    return artifact_root / lane / regression.ensure_run_label(run_label) / f"{sample_stem}.{suffix}"


def sample_inventory_row(sample_stem: str) -> regression.InventoryRow:
    for row in regression.inventory_rows():
        if row.sample_stem == sample_stem:
            return row
    raise ValueError(f"unknown current L2 sample stem: {sample_stem}")


def smoke_mode_for_row(row: regression.InventoryRow) -> str:
    if row.formal_hook == "runtime_try_cut_cluster":
        return "runtime"
    if row.formal_hook == "fixture_static_cluster":
        return "static"
    raise ValueError(
        f"sample {row.sample_stem} does not reach a formal hook "
        f"(formal_hook={row.formal_hook})"
    )


def plan_model_check_carrier_pipeline(
    sample_stem: str,
    artifact_root: Path,
    run_label: str | None = None,
    python_executable: str | None = None,
) -> ModelCheckCarrierPipelinePlan:
    row = sample_inventory_row(sample_stem)
    smoke_mode = smoke_mode_for_row(row)
    detached_loop = SCRIPT_DIR / "current_l2_detached_loop.py"
    python_cmd = python_executable or sys.executable
    effective_label = regression.effective_run_label(run_label or DEFAULT_RUN_LABEL)
    sample_run_label = regression.smoke_run_label(effective_label, sample_stem)
    smoke_subcommand = (
        "smoke-formal-hook-runtime" if smoke_mode == "runtime" else "smoke-formal-hook-static"
    )

    formal_hook_output = artifact_path(
        artifact_root=artifact_root,
        lane="formal-hooks",
        run_label=sample_run_label,
        sample_stem=sample_stem,
        suffix="formal-hook.json",
    )
    model_check_output = artifact_path(
        artifact_root=artifact_root,
        lane="model-check-carriers",
        run_label=sample_run_label,
        sample_stem=sample_stem,
        suffix="model-check-carrier.json",
    )

    formal_hook_command = PipelineCommand(
        name=f"{smoke_mode} formal hook smoke for {sample_stem}",
        argv=(
            python_cmd,
            str(detached_loop),
            smoke_subcommand,
            sample_stem,
            "--artifact-root",
            str(artifact_root),
            "--run-label",
            sample_run_label,
            "--overwrite",
        ),
    )
    model_check_command = PipelineCommand(
        name=f"model-check carrier emit for {sample_stem}",
        argv=(
            "cargo",
            "run",
            "-p",
            "mir-semantics",
            "--example",
            "current_l2_emit_model_check_carrier",
            "--",
            str(formal_hook_output),
            "--output",
            str(model_check_output),
        ),
    )

    return ModelCheckCarrierPipelinePlan(
        sample_stem=sample_stem,
        smoke_mode=smoke_mode,
        formal_hook_command=formal_hook_command,
        model_check_command=model_check_command,
        formal_hook_output=formal_hook_output,
        model_check_output=model_check_output,
    )


def pair_key(subject_ref: str, obligation_kind: str) -> tuple[str, str]:
    return (subject_ref, obligation_kind)


def formal_hook_pairs(formal_hook: dict[str, Any]) -> list[tuple[str, str]]:
    subject_ref = formal_hook.get("subject_ref")
    rows = formal_hook.get("contract_rows")
    if not isinstance(subject_ref, str) or not subject_ref:
        raise ValueError("formal hook subject_ref must be a non-empty string")
    if not isinstance(rows, list) or not rows:
        raise ValueError("formal hook contract_rows must be a non-empty array")

    pairs: list[tuple[str, str]] = []
    for row in rows:
        if not isinstance(row, dict):
            raise ValueError("formal hook contract row must be an object")
        obligation_kind = row.get("obligation_kind")
        if not isinstance(obligation_kind, str) or not obligation_kind:
            raise ValueError("formal hook obligation_kind must be a non-empty string")
        pairs.append(pair_key(subject_ref, obligation_kind))
    return pairs


def carrier_key(carrier: dict[str, Any]) -> tuple[str, str]:
    subject_ref = carrier.get("subject_ref")
    case = carrier.get("case")
    if not isinstance(subject_ref, str) or not subject_ref:
        raise ValueError("model-check carrier subject_ref must be a non-empty string")
    if not isinstance(case, dict):
        raise ValueError("model-check carrier case must be an object")
    obligation_kind = case.get("obligation_kind")
    evidence_refs = case.get("evidence_refs")
    if not isinstance(obligation_kind, str) or not obligation_kind:
        raise ValueError("model-check carrier obligation_kind must be a non-empty string")
    if not isinstance(evidence_refs, list) or not evidence_refs:
        raise ValueError("model-check carrier evidence_refs must be a non-empty array")
    return pair_key(subject_ref, obligation_kind)


def validate_conformance(
    formal_hook: dict[str, Any],
    carriers: Sequence[dict[str, Any]],
) -> ConformanceSummary:
    if not carriers:
        raise ValueError("model-check carriers must not be empty")

    hook_pairs = sorted(formal_hook_pairs(formal_hook))
    carrier_pairs = sorted(carrier_key(carrier) for carrier in carriers)

    if hook_pairs != carrier_pairs:
        raise ValueError(
            "pair mismatch between formal hook and model-check carriers: "
            f"formal_hook_pairs={hook_pairs}, carrier_pairs={carrier_pairs}"
        )

    return ConformanceSummary(
        formal_hook_pair_count=len(hook_pairs),
        model_check_carrier_count=len(carriers),
        matched_pairs=len(hook_pairs),
    )


def read_json_object(path: Path) -> dict[str, Any]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(payload, dict):
        raise ValueError(f"expected JSON object at {path}")
    return payload


def read_json_array(path: Path) -> list[dict[str, Any]]:
    payload = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(payload, list):
        raise ValueError(f"expected JSON array at {path}")
    if not all(isinstance(item, dict) for item in payload):
        raise ValueError(f"expected JSON object array at {path}")
    return list(payload)


def run_command(command: PipelineCommand) -> None:
    completed = subprocess.run(command.argv, cwd=REPO_ROOT)
    if completed.returncode != 0:
        raise RuntimeError(
            f"{command.name} failed with exit status {completed.returncode}"
        )


def execute_pipeline(plan: ModelCheckCarrierPipelinePlan) -> ConformanceSummary:
    plan.model_check_output.parent.mkdir(parents=True, exist_ok=True)

    run_command(plan.formal_hook_command)
    run_command(plan.model_check_command)

    formal_hook = read_json_object(plan.formal_hook_output)
    carriers = read_json_array(plan.model_check_output)
    return validate_conformance(formal_hook, carriers)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Run the current L2 model-check carrier repo-local conformance pipeline "
            "for one authored source sample."
        )
    )
    parser.add_argument("sample_stem", help="sample stem in samples/current-l2/")
    parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root directory (default: target/current-l2-model-check-carrier)",
    )
    parser.add_argument(
        "--run-label",
        default=DEFAULT_RUN_LABEL,
        help="artifact run label prefix",
    )
    parser.add_argument(
        "--python",
        default=sys.executable,
        help="Python executable used for the detached formal-hook smoke helper",
    )
    parser.add_argument(
        "--plan-only",
        action="store_true",
        help="print the planned commands and output paths without executing them",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    plan = plan_model_check_carrier_pipeline(
        sample_stem=args.sample_stem,
        artifact_root=Path(args.artifact_root),
        run_label=args.run_label,
        python_executable=args.python,
    )

    if args.plan_only:
        print(
            json.dumps(
                {
                    "sample_stem": plan.sample_stem,
                    "smoke_mode": plan.smoke_mode,
                    "formal_hook_command": list(plan.formal_hook_command.argv),
                    "model_check_command": list(plan.model_check_command.argv),
                    "formal_hook_output": str(plan.formal_hook_output),
                    "model_check_output": str(plan.model_check_output),
                },
                indent=2,
            )
        )
        return 0

    summary = execute_pipeline(plan)
    print(
        json.dumps(
            {
                "sample_stem": plan.sample_stem,
                "smoke_mode": plan.smoke_mode,
                "formal_hook_output": str(plan.formal_hook_output),
                "model_check_output": str(plan.model_check_output),
                "formal_hook_pair_count": summary.formal_hook_pair_count,
                "model_check_carrier_count": summary.model_check_carrier_count,
                "matched_pairs": summary.matched_pairs,
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
