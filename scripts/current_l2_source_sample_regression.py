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


CURRENT_FIXED_SUBSET_FIRST_CLUSTER: tuple[InventoryRow, ...] = (
    InventoryRow(
        sample_stem="e2-try-fallback",
        authored_status="source-authored",
        expected_static="valid",
        expected_runtime="success",
        formal_hook="runtime_try_cut_cluster",
        note="first authored trio runtime path",
    ),
    InventoryRow(
        sample_stem="e4-malformed-lineage",
        authored_status="source-authored",
        expected_static="malformed",
        expected_runtime="not_evaluated",
        formal_hook="fixture_static_cluster",
        note="first authored trio static stop",
    ),
    InventoryRow(
        sample_stem="e23-malformed-try-fallback-missing-fallback-body",
        authored_status="source-authored",
        expected_static="malformed",
        expected_runtime="not_evaluated",
        formal_hook="fixture_static_cluster",
        note="first authored trio static stop",
    ),
    InventoryRow(
        sample_stem="e1-place-atomic-cut",
        authored_status="source-target-only",
        expected_static="not_yet_authored",
        expected_runtime="not_yet_authored",
        formal_hook="not_yet_authored",
        note="deferred authored row",
    ),
    InventoryRow(
        sample_stem="e3-option-admit-chain",
        authored_status="source-target-only",
        expected_static="not_yet_authored",
        expected_runtime="not_yet_authored",
        formal_hook="not_yet_authored",
        note="deferred authored row",
    ),
    InventoryRow(
        sample_stem="e21-try-atomic-cut-frontier",
        authored_status="source-target-only",
        expected_static="not_yet_authored",
        expected_runtime="not_yet_authored",
        formal_hook="not_yet_authored",
        note="deferred authored row",
    ),
)


def inventory_rows() -> list[InventoryRow]:
    return list(CURRENT_FIXED_SUBSET_FIRST_CLUSTER)


def format_inventory_text(rows: Sequence[InventoryRow]) -> str:
    header = (
        "sample stem | authored status | expected static | "
        "expected runtime | formal hook | note"
    )
    divider = "-" * len(header)
    body = [
        (
            f"{row.sample_stem} | {row.authored_status} | {row.expected_static} | "
            f"{row.expected_runtime} | {row.formal_hook} | {row.note}"
        )
        for row in rows
    ]
    return "\n".join(
        [
            "current L2 fixed-subset first-cluster inventory",
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
            name="formal hook support test",
            argv=("cargo", "test", "-p", "mir-semantics", "--test", "current_l2_formal_hook_support"),
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


def command_inventory(_args: argparse.Namespace, stdout = sys.stdout) -> int:
    print(format_inventory_text(inventory_rows()), file=stdout)
    return 0


def command_regression(args: argparse.Namespace, stdout = sys.stdout) -> int:
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
    inventory_parser.set_defaults(handler=command_inventory)

    regression_parser = subparsers.add_parser(
        "regression",
        help="run the authored-sample regression command set for the first authored trio",
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
        return args.handler(args, stdout=stdout)
    except ValueError as error:
        print(f"error: {error}", file=stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
