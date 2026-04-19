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
DEFAULT_ARTIFACT_ROOT = REPO_ROOT / "target" / "current-l2-theorem-lean-stub"
DEFAULT_RUN_LABEL = "theorem-lean-stub"


@dataclass(frozen=True)
class PipelineCommand:
    name: str
    argv: tuple[str, ...]


@dataclass(frozen=True)
class TheoremLeanStubPipelinePlan:
    sample_stem: str
    smoke_mode: str
    formal_hook_command: PipelineCommand
    review_unit_command: PipelineCommand
    lean_stub_command: PipelineCommand
    formal_hook_output: Path
    review_units_output: Path
    lean_stubs_output: Path


@dataclass(frozen=True)
class ConformanceSummary:
    review_unit_count: int
    lean_stub_count: int
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


def plan_theorem_lean_stub_pipeline(
    sample_stem: str,
    artifact_root: Path,
    run_label: str | None = None,
    python_executable: str | None = None,
) -> TheoremLeanStubPipelinePlan:
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
    review_units_output = artifact_path(
        artifact_root=artifact_root,
        lane="proof-notebook-review-units",
        run_label=sample_run_label,
        sample_stem=sample_stem,
        suffix="proof-notebook-review-unit.json",
    )
    lean_stubs_output = artifact_path(
        artifact_root=artifact_root,
        lane="lean-theorem-stubs",
        run_label=sample_run_label,
        sample_stem=sample_stem,
        suffix="lean-theorem-stub.json",
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
    review_unit_command = PipelineCommand(
        name=f"proof notebook review unit emit for {sample_stem}",
        argv=(
            "cargo",
            "run",
            "-p",
            "mir-semantics",
            "--example",
            "current_l2_emit_proof_notebook_review_unit",
            "--",
            str(formal_hook_output),
            "--output",
            str(review_units_output),
        ),
    )
    lean_stub_command = PipelineCommand(
        name=f"Lean theorem stub emit for {sample_stem}",
        argv=(
            "cargo",
            "run",
            "-p",
            "mir-semantics",
            "--example",
            "current_l2_emit_lean_theorem_stub",
            "--",
            str(review_units_output),
            "--output",
            str(lean_stubs_output),
        ),
    )

    return TheoremLeanStubPipelinePlan(
        sample_stem=sample_stem,
        smoke_mode=smoke_mode,
        formal_hook_command=formal_hook_command,
        review_unit_command=review_unit_command,
        lean_stub_command=lean_stub_command,
        formal_hook_output=formal_hook_output,
        review_units_output=review_units_output,
        lean_stubs_output=lean_stubs_output,
    )


def pair_key(subject_ref: str, obligation_kind: str) -> tuple[str, str]:
    return (subject_ref, obligation_kind)


def review_unit_key(review_unit: dict[str, Any]) -> tuple[str, str]:
    subject_ref = review_unit.get("subject_ref")
    row = review_unit.get("row")
    if not isinstance(subject_ref, str) or not subject_ref:
        raise ValueError("review unit subject_ref must be a non-empty string")
    if not isinstance(row, dict):
        raise ValueError("review unit row must be an object")
    obligation_kind = row.get("obligation_kind")
    if not isinstance(obligation_kind, str) or not obligation_kind:
        raise ValueError("review unit row obligation_kind must be a non-empty string")
    return pair_key(subject_ref, obligation_kind)


def lean_stub_key(stub: dict[str, Any]) -> tuple[str, str]:
    subject_ref = stub.get("subject_ref")
    obligation_kind = stub.get("obligation_kind")
    tool_family = stub.get("tool_family")
    source_text = stub.get("source_text")
    if tool_family != "lean-first":
        raise ValueError(f"expected tool_family lean-first, got {tool_family!r}")
    if not isinstance(subject_ref, str) or not subject_ref:
        raise ValueError("Lean stub subject_ref must be a non-empty string")
    if not isinstance(obligation_kind, str) or not obligation_kind:
        raise ValueError("Lean stub obligation_kind must be a non-empty string")
    if not isinstance(source_text, str) or not source_text.strip():
        raise ValueError("Lean stub source_text must be a non-empty string")
    return pair_key(subject_ref, obligation_kind)


def validate_conformance(
    review_units: Sequence[dict[str, Any]],
    stubs: Sequence[dict[str, Any]],
) -> ConformanceSummary:
    if not review_units:
        raise ValueError("review_units must not be empty")
    if not stubs:
        raise ValueError("lean stubs must not be empty")

    review_pairs = sorted(review_unit_key(review_unit) for review_unit in review_units)
    stub_pairs = sorted(lean_stub_key(stub) for stub in stubs)

    if review_pairs != stub_pairs:
        raise ValueError(
            "pair mismatch between review units and Lean stubs: "
            f"review_pairs={review_pairs}, stub_pairs={stub_pairs}"
        )

    return ConformanceSummary(
        review_unit_count=len(review_units),
        lean_stub_count=len(stubs),
        matched_pairs=len(review_pairs),
    )


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


def execute_pipeline(plan: TheoremLeanStubPipelinePlan) -> ConformanceSummary:
    plan.review_units_output.parent.mkdir(parents=True, exist_ok=True)
    plan.lean_stubs_output.parent.mkdir(parents=True, exist_ok=True)

    run_command(plan.formal_hook_command)
    run_command(plan.review_unit_command)
    run_command(plan.lean_stub_command)

    review_units = read_json_array(plan.review_units_output)
    stubs = read_json_array(plan.lean_stubs_output)
    return validate_conformance(review_units, stubs)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Run the current L2 theorem Lean-stub repo-local conformance pipeline "
            "for one authored source sample."
        )
    )
    parser.add_argument("sample_stem", help="sample stem in samples/current-l2/")
    parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root directory (default: target/current-l2-theorem-lean-stub)",
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
    plan = plan_theorem_lean_stub_pipeline(
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
                    "review_unit_command": list(plan.review_unit_command.argv),
                    "lean_stub_command": list(plan.lean_stub_command.argv),
                    "formal_hook_output": str(plan.formal_hook_output),
                    "review_units_output": str(plan.review_units_output),
                    "lean_stubs_output": str(plan.lean_stubs_output),
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
                "review_units_output": str(plan.review_units_output),
                "lean_stubs_output": str(plan.lean_stubs_output),
                "review_unit_count": summary.review_unit_count,
                "lean_stub_count": summary.lean_stub_count,
                "matched_pairs": summary.matched_pairs,
            },
            indent=2,
        )
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
