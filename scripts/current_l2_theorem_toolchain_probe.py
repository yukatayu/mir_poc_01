#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
from dataclasses import asdict, dataclass
from pathlib import Path

import current_l2_theorem_lean_stub_pipeline as lean_stub_pipeline


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
DEFAULT_ARTIFACT_ROOT = REPO_ROOT / "target" / "current-l2-theorem-toolchain-probe"
DEFAULT_RUN_LABEL = "theorem-toolchain-probe"
REQUIRED_TOOLS = ("lean", "lake", "elan")


@dataclass(frozen=True)
class ToolProbeRow:
    name: str
    available: bool
    path: str | None
    version_line: str | None
    error: str | None


@dataclass(frozen=True)
class TheoremToolchainProbeResult:
    status: str
    reopen_condition_met: bool
    available_tools: tuple[str, ...]
    missing_tools: tuple[str, ...]
    tool_versions: dict[str, str]
    rows: tuple[ToolProbeRow, ...]


@dataclass(frozen=True)
class TheoremActualLeanReopenManifest:
    sample_stem: str | None
    smoke_mode: str | None
    toolchain_status: str
    reopen_condition_met: bool
    available_tools: tuple[str, ...]
    missing_tools: tuple[str, ...]
    tool_versions: dict[str, str]
    pipeline_plan: dict[str, object]
    reference_refs: tuple[str, ...]
    next_step_refs: tuple[str, ...]


def run_version_command(tool_path: str) -> tuple[str | None, str | None]:
    try:
        completed = subprocess.run(
            [tool_path, "--version"],
            check=False,
            capture_output=True,
            text=True,
            cwd=REPO_ROOT,
        )
    except OSError as exc:
        return (None, str(exc))

    if completed.returncode != 0:
        stderr = completed.stderr.strip() or completed.stdout.strip()
        return (None, stderr or f"exit status {completed.returncode}")

    version_line = completed.stdout.strip().splitlines()
    if not version_line:
        return (None, "empty version output")
    return (version_line[0], None)


def discover_tool(tool_name: str, search_path: str | None = None) -> ToolProbeRow:
    tool_path = shutil.which(tool_name, path=search_path)
    if tool_path is None:
        return ToolProbeRow(
            name=tool_name,
            available=False,
            path=None,
            version_line=None,
            error="command not found",
        )

    version_line, error = run_version_command(tool_path)
    return ToolProbeRow(
        name=tool_name,
        available=error is None,
        path=tool_path,
        version_line=version_line,
        error=error,
    )


def probe_theorem_toolchain(search_path: str | None = None) -> TheoremToolchainProbeResult:
    rows = tuple(discover_tool(tool_name, search_path=search_path) for tool_name in REQUIRED_TOOLS)
    available_tools = tuple(sorted(row.name for row in rows if row.available))
    missing_tools = tuple(row.name for row in rows if not row.available)
    tool_versions = {
        row.name: row.version_line for row in rows if row.available and row.version_line is not None
    }

    if len(available_tools) == len(REQUIRED_TOOLS):
        status = "ready"
    elif available_tools:
        status = "partial"
    else:
        status = "unavailable"

    return TheoremToolchainProbeResult(
        status=status,
        reopen_condition_met=status == "ready",
        available_tools=available_tools,
        missing_tools=missing_tools,
        tool_versions=tool_versions,
        rows=rows,
    )


def pipeline_plan_payload(
    sample_stem: str,
    artifact_root: Path,
    run_label: str,
    python_executable: str,
) -> tuple[str, dict[str, object]]:
    plan = lean_stub_pipeline.plan_theorem_lean_stub_pipeline(
        sample_stem=sample_stem,
        artifact_root=artifact_root,
        run_label=run_label,
        python_executable=python_executable,
    )
    return (
        plan.smoke_mode,
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
    )


def build_theorem_actual_lean_reopen_manifest(
    sample_stem: str | None,
    artifact_root: Path,
    run_label: str,
    python_executable: str,
    search_path: str | None = None,
) -> TheoremActualLeanReopenManifest:
    probe_result = probe_theorem_toolchain(search_path=search_path)
    smoke_mode = None
    plan_payload: dict[str, object] = {}
    if sample_stem is not None:
        smoke_mode, plan_payload = pipeline_plan_payload(
            sample_stem=sample_stem,
            artifact_root=artifact_root,
            run_label=run_label,
            python_executable=python_executable,
        )

    reference_refs = (
        "environment_probe:current_l2.theorem_actual_lean_execution_toolchain_probe",
        "environment_stop_line:actual_lean_execution_requires_ready_lean_lake_elan",
        "environment_stop_line:lean_stub_pipeline_plan_carried_forward_until_toolchain_ready",
    )
    next_step_refs = (
        "next_step:keep_lean_stub_bridge_until_toolchain_ready",
        "next_step:actual_lean_execution_narrow_probe_reopen",
    )

    return TheoremActualLeanReopenManifest(
        sample_stem=sample_stem,
        smoke_mode=smoke_mode,
        toolchain_status=probe_result.status,
        reopen_condition_met=probe_result.reopen_condition_met,
        available_tools=probe_result.available_tools,
        missing_tools=probe_result.missing_tools,
        tool_versions=probe_result.tool_versions,
        pipeline_plan=plan_payload,
        reference_refs=reference_refs,
        next_step_refs=next_step_refs,
    )


def manifest_to_dict(manifest: TheoremActualLeanReopenManifest) -> dict[str, object]:
    payload = asdict(manifest)
    return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description=(
            "Probe local Lean toolchain availability for the current L2 theorem line "
            "and emit a sample-aware reopen manifest."
        )
    )
    parser.add_argument(
        "sample_stem",
        nargs="?",
        help="optional current L2 authored sample stem used to embed the existing Lean-stub pipeline plan",
    )
    parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root for embedded pipeline planning",
    )
    parser.add_argument(
        "--run-label",
        default=DEFAULT_RUN_LABEL,
        help="artifact run label prefix for embedded pipeline planning",
    )
    parser.add_argument(
        "--python",
        default=sys.executable,
        help="Python executable used when embedding the existing Lean-stub pipeline plan",
    )
    parser.add_argument(
        "--path",
        default=os.environ.get("PATH", ""),
        help="PATH used for probing lean/lake/elan",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    manifest = build_theorem_actual_lean_reopen_manifest(
        sample_stem=args.sample_stem,
        artifact_root=Path(args.artifact_root),
        run_label=args.run_label,
        python_executable=args.python,
        search_path=args.path,
    )
    print(json.dumps(manifest_to_dict(manifest), indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
