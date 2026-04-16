#!/usr/bin/env python3

from __future__ import annotations

import argparse
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Callable, Sequence


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
DEFAULT_ARTIFACT_ROOT = REPO_ROOT / "target" / "current-l2-source-sample-regression"
DEFAULT_RUN_LABEL = "source-sample-regression"
RUN_LABEL_PATTERN = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._-]*$")


@dataclass(frozen=True)
class InventoryRow:
    sample_stem: str
    authored_status: str
    expected_static: str
    expected_runtime: str
    formal_hook: str
    note: str


@dataclass(frozen=True)
class RegressionCommand:
    name: str
    argv: tuple[str, ...]


@dataclass(frozen=True)
class InventoryStatus:
    row: InventoryRow
    sample_path: Path
    file_exists: bool
    status_ok: bool


CURRENT_FIXED_SUBSET_FIRST_CLUSTER: tuple[InventoryRow, ...] = (
    InventoryRow(
        sample_stem="e1-place-atomic-cut",
        authored_status="source-authored",
        expected_static="valid",
        expected_runtime="explicit_failure",
        formal_hook="runtime_try_cut_cluster",
        note="first widened authored row runtime path",
    ),
    InventoryRow(
        sample_stem="e2-try-fallback",
        authored_status="source-authored",
        expected_static="valid",
        expected_runtime="success",
        formal_hook="runtime_try_cut_cluster",
        note="current authored runtime path",
    ),
    InventoryRow(
        sample_stem="e3-option-admit-chain",
        authored_status="source-authored",
        expected_static="valid",
        expected_runtime="success",
        formal_hook="not_reached_guarded",
        note="third widened authored row runtime path; formal hook stays guarded",
    ),
    InventoryRow(
        sample_stem="e4-malformed-lineage",
        authored_status="source-authored",
        expected_static="malformed",
        expected_runtime="not_evaluated",
        formal_hook="fixture_static_cluster",
        note="current authored static stop",
    ),
    InventoryRow(
        sample_stem="e16-malformed-missing-chain-head-option",
        authored_status="source-authored",
        expected_static="malformed",
        expected_runtime="not_evaluated",
        formal_hook="fixture_static_cluster",
        note="missing-option family head-gap static stop",
    ),
    InventoryRow(
        sample_stem="e13-malformed-capability-strengthening",
        authored_status="source-authored",
        expected_static="malformed",
        expected_runtime="not_evaluated",
        formal_hook="fixture_static_cluster",
        note="capability family first static stop",
    ),
    InventoryRow(
        sample_stem="e19-malformed-target-mismatch",
        authored_status="source-authored",
        expected_static="malformed",
        expected_runtime="not_evaluated",
        formal_hook="fixture_static_cluster",
        note="stable-static edge-pair target-mismatch row",
    ),
    InventoryRow(
        sample_stem="e21-try-atomic-cut-frontier",
        authored_status="source-authored",
        expected_static="valid",
        expected_runtime="success",
        formal_hook="runtime_try_cut_cluster",
        note="second widened authored row runtime path",
    ),
    InventoryRow(
        sample_stem="e22-try-atomic-cut-place-mismatch",
        authored_status="source-authored",
        expected_static="valid",
        expected_runtime="success",
        formal_hook="runtime_try_cut_cluster",
        note="post-sextet first contrast-row runtime path",
    ),
    InventoryRow(
        sample_stem="e18-malformed-missing-successor-option",
        authored_status="source-authored",
        expected_static="malformed",
        expected_runtime="not_evaluated",
        formal_hook="fixture_static_cluster",
        note="missing-option family successor-gap static stop",
    ),
    InventoryRow(
        sample_stem="e20-malformed-late-capability-strengthening",
        authored_status="source-authored",
        expected_static="malformed",
        expected_runtime="not_evaluated",
        formal_hook="fixture_static_cluster",
        note="capability family later-edge static stop",
    ),
    InventoryRow(
        sample_stem="e23-malformed-try-fallback-missing-fallback-body",
        authored_status="source-authored",
        expected_static="malformed",
        expected_runtime="not_evaluated",
        formal_hook="fixture_static_cluster",
        note="current authored static stop",
    ),
)


def inventory_rows() -> list[InventoryRow]:
    return list(CURRENT_FIXED_SUBSET_FIRST_CLUSTER)


def default_sample_root() -> Path:
    return REPO_ROOT / "samples" / "current-l2"


def sample_path(sample_root: Path, sample_stem: str) -> Path:
    return sample_root / f"{sample_stem}.txt"


def inventory_statuses(sample_root: Path | None = None) -> list[InventoryStatus]:
    root = sample_root or default_sample_root()
    statuses: list[InventoryStatus] = []
    for row in inventory_rows():
        path = sample_path(root, row.sample_stem)
        exists = path.is_file()
        expected_exists = row.authored_status == "source-authored"
        statuses.append(
            InventoryStatus(
                row=row,
                sample_path=path,
                file_exists=exists,
                status_ok=exists == expected_exists,
            )
        )
    return statuses


def format_inventory_text(statuses: Sequence[InventoryStatus]) -> str:
    header = (
        "sample stem | authored status | expected static | "
        "expected runtime | formal hook | file state | note"
    )
    divider = "-" * len(header)
    body = [
        (
            f"{status.row.sample_stem} | {status.row.authored_status} | "
            f"{status.row.expected_static} | {status.row.expected_runtime} | "
            f"{status.row.formal_hook} | "
            f"{'present' if status.file_exists else 'absent'}"
            f"{'' if status.status_ok else ' (mismatch)'} | {status.row.note}"
        )
        for status in statuses
    ]
    return "\n".join(
        [
            "current L2 fixed-subset authored inventory",
            header,
            divider,
            *body,
        ]
    )


def ensure_run_label(label: str) -> str:
    if not label:
        raise ValueError("run label must not be empty")
    if label in {".", ".."}:
        raise ValueError("run label must not be '.' or '..'")
    if not RUN_LABEL_PATTERN.fullmatch(label):
        raise ValueError(
            "run label must match [A-Za-z0-9][A-Za-z0-9._-]* and stay a single segment"
        )
    return label


def effective_run_label(label: str | None) -> str:
    if label is None:
        return ensure_run_label(DEFAULT_RUN_LABEL)
    return ensure_run_label(label)


def smoke_run_label(base_label: str, sample_stem: str) -> str:
    return ensure_run_label(f"{ensure_run_label(base_label)}-{sample_stem}")


def inventory_mismatches(statuses: Sequence[InventoryStatus]) -> list[str]:
    mismatches: list[str] = []
    for status in statuses:
        if status.status_ok:
            continue
        expected = (
            "present"
            if status.row.authored_status == "source-authored"
            else "absent"
        )
        observed = "present" if status.file_exists else "absent"
        mismatches.append(
            f"{status.row.sample_stem}: expected sample file {expected}, observed {observed}"
        )
    return mismatches


def plan_regression_commands(
    artifact_root: Path,
    run_label: str | None = None,
    python_executable: str | None = None,
) -> list[RegressionCommand]:
    effective_label = effective_run_label(run_label)
    python_cmd = python_executable or sys.executable
    detached_loop = SCRIPT_DIR / "current_l2_detached_loop.py"

    commands = [
        RegressionCommand(
            name="runtime lowering test",
            argv=("cargo", "test", "-p", "mir-runtime", "--test", "current_l2_source_lowering"),
        ),
        RegressionCommand(
            name="source sample runner test",
            argv=("cargo", "test", "-p", "mir-runtime", "--test", "current_l2_source_sample_runner"),
        ),
        RegressionCommand(
            name="verification ladder test",
            argv=(
                "cargo",
                "test",
                "-p",
                "mir-runtime",
                "--test",
                "current_l2_source_sample_verification_ladder",
            ),
        ),
        RegressionCommand(
            name="emitted artifact wiring test",
            argv=(
                "cargo",
                "test",
                "-p",
                "mir-runtime",
                "--test",
                "current_l2_source_sample_emitted_artifact_wiring",
            ),
        ),
        RegressionCommand(
            name="formal hook support test",
            argv=("cargo", "test", "-p", "mir-semantics", "--test", "current_l2_formal_hook_support"),
        ),
        RegressionCommand(
            name="runtime formal hook smoke for e1-place-atomic-cut",
            argv=(
                python_cmd,
                str(detached_loop),
                "smoke-formal-hook-runtime",
                "e1-place-atomic-cut",
                "--artifact-root",
                str(artifact_root),
                "--run-label",
                smoke_run_label(effective_label, "e1-place-atomic-cut"),
                "--overwrite",
            ),
        ),
        RegressionCommand(
            name="runtime formal hook smoke for e2-try-fallback",
            argv=(
                python_cmd,
                str(detached_loop),
                "smoke-formal-hook-runtime",
                "e2-try-fallback",
                "--artifact-root",
                str(artifact_root),
                "--run-label",
                smoke_run_label(effective_label, "e2-try-fallback"),
                "--overwrite",
            ),
        ),
        RegressionCommand(
            name="runtime formal hook smoke for e21-try-atomic-cut-frontier",
            argv=(
                python_cmd,
                str(detached_loop),
                "smoke-formal-hook-runtime",
                "e21-try-atomic-cut-frontier",
                "--artifact-root",
                str(artifact_root),
                "--run-label",
                smoke_run_label(effective_label, "e21-try-atomic-cut-frontier"),
                "--overwrite",
            ),
        ),
        RegressionCommand(
            name="runtime formal hook smoke for e22-try-atomic-cut-place-mismatch",
            argv=(
                python_cmd,
                str(detached_loop),
                "smoke-formal-hook-runtime",
                "e22-try-atomic-cut-place-mismatch",
                "--artifact-root",
                str(artifact_root),
                "--run-label",
                smoke_run_label(effective_label, "e22-try-atomic-cut-place-mismatch"),
                "--overwrite",
            ),
        ),
        RegressionCommand(
            name="static formal hook smoke for e4-malformed-lineage",
            argv=(
                python_cmd,
                str(detached_loop),
                "smoke-formal-hook-static",
                "e4-malformed-lineage",
                "--artifact-root",
                str(artifact_root),
                "--run-label",
                smoke_run_label(effective_label, "e4-malformed-lineage"),
                "--overwrite",
            ),
        ),
        RegressionCommand(
            name="static formal hook smoke for e16-malformed-missing-chain-head-option",
            argv=(
                python_cmd,
                str(detached_loop),
                "smoke-formal-hook-static",
                "e16-malformed-missing-chain-head-option",
                "--artifact-root",
                str(artifact_root),
                "--run-label",
                smoke_run_label(
                    effective_label, "e16-malformed-missing-chain-head-option"
                ),
                "--overwrite",
            ),
        ),
        RegressionCommand(
            name="static formal hook smoke for e13-malformed-capability-strengthening",
            argv=(
                python_cmd,
                str(detached_loop),
                "smoke-formal-hook-static",
                "e13-malformed-capability-strengthening",
                "--artifact-root",
                str(artifact_root),
                "--run-label",
                smoke_run_label(
                    effective_label, "e13-malformed-capability-strengthening"
                ),
                "--overwrite",
            ),
        ),
        RegressionCommand(
            name="static formal hook smoke for e19-malformed-target-mismatch",
            argv=(
                python_cmd,
                str(detached_loop),
                "smoke-formal-hook-static",
                "e19-malformed-target-mismatch",
                "--artifact-root",
                str(artifact_root),
                "--run-label",
                smoke_run_label(effective_label, "e19-malformed-target-mismatch"),
                "--overwrite",
            ),
        ),
        RegressionCommand(
            name="static formal hook smoke for e18-malformed-missing-successor-option",
            argv=(
                python_cmd,
                str(detached_loop),
                "smoke-formal-hook-static",
                "e18-malformed-missing-successor-option",
                "--artifact-root",
                str(artifact_root),
                "--run-label",
                smoke_run_label(
                    effective_label, "e18-malformed-missing-successor-option"
                ),
                "--overwrite",
            ),
        ),
        RegressionCommand(
            name="static formal hook smoke for e20-malformed-late-capability-strengthening",
            argv=(
                python_cmd,
                str(detached_loop),
                "smoke-formal-hook-static",
                "e20-malformed-late-capability-strengthening",
                "--artifact-root",
                str(artifact_root),
                "--run-label",
                smoke_run_label(
                    effective_label, "e20-malformed-late-capability-strengthening"
                ),
                "--overwrite",
            ),
        ),
        RegressionCommand(
            name="static formal hook smoke for e23-malformed-try-fallback-missing-fallback-body",
            argv=(
                python_cmd,
                str(detached_loop),
                "smoke-formal-hook-static",
                "e23-malformed-try-fallback-missing-fallback-body",
                "--artifact-root",
                str(artifact_root),
                "--run-label",
                smoke_run_label(
                    effective_label,
                    "e23-malformed-try-fallback-missing-fallback-body",
                ),
                "--overwrite",
            ),
        ),
    ]
    return commands


def run_subprocess(argv: Sequence[str]) -> int:
    completed = subprocess.run(list(argv), cwd=REPO_ROOT)
    return completed.returncode


def run_regression_commands(
    commands: Sequence[RegressionCommand],
    runner: Callable[[Sequence[str]], int] = run_subprocess,
    stdout = sys.stdout,
) -> int:
    total = len(commands)
    for index, command in enumerate(commands, start=1):
        print(f"[{index}/{total}] {command.name}", file=stdout, flush=True)
        exit_code = runner(command.argv)
        if exit_code != 0:
            print(
                f"stopped after failure in '{command.name}' (exit {exit_code})",
                file=stdout,
                flush=True,
            )
            return exit_code
    print("all regression commands passed", file=stdout, flush=True)
    return 0


def command_inventory(
    args: argparse.Namespace,
    stdout = sys.stdout,
    stderr = sys.stderr,
) -> int:
    statuses = inventory_statuses(Path(args.sample_root))
    print(format_inventory_text(statuses), file=stdout)
    mismatches = inventory_mismatches(statuses)
    if mismatches:
        for mismatch in mismatches:
            print(mismatch, file=stderr)
        return 2
    return 0


def command_regression(
    args: argparse.Namespace,
    stdout = sys.stdout,
    stderr = sys.stderr,
) -> int:
    statuses = inventory_statuses()
    mismatches = inventory_mismatches(statuses)
    if mismatches:
        for mismatch in mismatches:
            print(mismatch, file=stderr)
        return 2

    commands = plan_regression_commands(
        artifact_root=Path(args.artifact_root),
        run_label=args.run_label,
        python_executable=sys.executable,
    )
    return run_regression_commands(commands, stdout=stdout)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "Thin repo-local helper for current L2 fixed-subset source sample "
            "inventory and authored-sample regression."
        )
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    inventory_parser = subparsers.add_parser(
        "inventory",
        help="print the current authored/deferred fixed-subset inventory",
    )
    inventory_parser.add_argument(
        "--sample-root",
        default=str(default_sample_root()),
        help="source-sample root directory to inspect",
    )
    inventory_parser.set_defaults(handler=command_inventory)

    regression_parser = subparsers.add_parser(
        "regression",
        help="run the authored-sample regression command set for the current authored subset",
    )
    regression_parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root passed to detached-loop smoke commands",
    )
    regression_parser.add_argument(
        "--run-label",
        default=DEFAULT_RUN_LABEL,
        help="base run label for detached-loop smoke commands",
    )
    regression_parser.set_defaults(handler=command_regression)

    return parser


def main(argv: Sequence[str] | None = None, stdout = sys.stdout, stderr = sys.stderr) -> int:
    parser = build_parser()
    try:
        args = parser.parse_args(list(argv) if argv is not None else None)
        return args.handler(args, stdout=stdout, stderr=stderr)
    except ValueError as error:
        print(f"error: {error}", file=stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
