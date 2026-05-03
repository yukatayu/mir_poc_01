#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
PRACTICAL_ROOT = REPO_ROOT / "samples" / "practical-alpha1"

IMPLEMENTED_ROWS: list[dict[str, str]] = [
    {
        "sample_id": "CHK-LIF-01",
        "summary": "reject raw dangling reference in the practical checker floor",
        "package_dir": "samples/practical-alpha1/packages/chk-lif-01-raw-dangling",
        "expected_report": "samples/practical-alpha1/expected/chk-lif-01-raw-dangling.expected.json",
    },
    {
        "sample_id": "CHK-LIF-02",
        "summary": "accept explicit fallback-chain degradation with explicit accepted evidence",
        "package_dir": "samples/practical-alpha1/packages/chk-lif-02-fallback-access-valid",
        "expected_report": "samples/practical-alpha1/expected/chk-lif-02-fallback-access-valid.expected.json",
    },
    {
        "sample_id": "CHK-LIF-03",
        "summary": "accept inherited chain splice with explicit lineage evidence",
        "package_dir": "samples/practical-alpha1/packages/chk-lif-03-inherited-chain-valid",
        "expected_report": "samples/practical-alpha1/expected/chk-lif-03-inherited-chain-valid.expected.json",
    },
    {
        "sample_id": "CHK-LIF-04",
        "summary": "accept snapshot-selected distinction with explicit selected-option evidence",
        "package_dir": "samples/practical-alpha1/packages/chk-lif-04-snapshot-selected-distinction",
        "expected_report": "samples/practical-alpha1/expected/chk-lif-04-snapshot-selected-distinction.expected.json",
    },
    {
        "sample_id": "CHK-VAR-01",
        "summary": "accept logging/debug layer as observe-only contract overlay",
        "package_dir": "samples/practical-alpha1/packages/chk-var-01-logging-layer-valid",
        "expected_report": "samples/practical-alpha1/expected/chk-var-01-logging-layer-valid.expected.json",
    },
    {
        "sample_id": "CHK-VAR-02",
        "summary": "reject layer precondition strengthening",
        "package_dir": "samples/practical-alpha1/packages/chk-var-02-precondition-strengthening-rejected",
        "expected_report": "samples/practical-alpha1/expected/chk-var-02-precondition-strengthening-rejected.expected.json",
    },
    {
        "sample_id": "CHK-VAR-03",
        "summary": "reject mutable/read-write covariance",
        "package_dir": "samples/practical-alpha1/packages/chk-var-03-mutable-covariance-rejected",
        "expected_report": "samples/practical-alpha1/expected/chk-var-03-mutable-covariance-rejected.expected.json",
    },
    {
        "sample_id": "CHK-CUT-01",
        "summary": "reject invalid distributed cut with explicit orphan-receive evidence",
        "package_dir": "samples/practical-alpha1/packages/chk-cut-01-invalid-distributed-cut-rejected",
        "expected_report": "samples/practical-alpha1/expected/chk-cut-01-invalid-distributed-cut-rejected.expected.json",
    },
    {
        "sample_id": "CHK-PKG-01",
        "summary": "reject unsigned native package at checker-only package-admission preview",
        "package_dir": "samples/practical-alpha1/packages/chk-pkg-01-unsigned-native-rejected",
        "expected_report": "samples/practical-alpha1/expected/chk-pkg-01-unsigned-native-rejected.expected.json",
    },
    {
        "sample_id": "CHK-PKG-02",
        "summary": "reject over-capability package at checker-only package-admission preview",
        "package_dir": "samples/practical-alpha1/packages/chk-pkg-02-over-capability-rejected",
        "expected_report": "samples/practical-alpha1/expected/chk-pkg-02-over-capability-rejected.expected.json",
    },
]

STOP_LINES = [
    "do not treat the practical alpha-1 check command as runtime plan generation",
    "do not treat the practical alpha-1 check command as run-local or run-docker completion",
    "do not treat the practical alpha-1 check command as a final public CLI/API freeze",
    "do not promote samples/practical-alpha1 to an active runnable root in the checker package",
]

LIMITATIONS = [
    "alpha-local non-final checker floor only",
    "limited CHK practical sample families only",
    "no runtime plan, local runtime, Docker runtime, save/load, or devtools export",
]


def _implemented_row(sample_id: str) -> dict[str, str]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown practical alpha-1 checker sample {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "practical-alpha1-checker",
            "source_root": "samples/practical-alpha1/packages",
            "summary": row["summary"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def _load_expected_report(row: dict[str, str]) -> dict[str, Any]:
    return json.loads((REPO_ROOT / row["expected_report"]).read_text())


def _build_check_report(package_path: str | Path) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-ast",
            "--example",
            "mir_practical_alpha1_check",
            "--",
            str(package_path),
        ],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as error:  # pragma: no cover
        raise RuntimeError(
            f"checker command did not return JSON for {package_path}: {completed.stdout}"
        ) from error


def check_path(package_path: str | Path) -> dict[str, Any]:
    return _build_check_report(package_path)


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    expected = _load_expected_report(row)
    actual = check_path(REPO_ROOT / row["package_dir"])
    if actual != expected:
        raise RuntimeError(
            f"{sample_id}: expected checker report drift\n"
            f"expected={json.dumps(expected, ensure_ascii=False, sort_keys=True)}\n"
            f"actual={json.dumps(actual, ensure_ascii=False, sort_keys=True)}"
        )
    return actual


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            run_sample(sample_id)
            passed.append(sample_id)
        except Exception as error:  # pragma: no cover
            failed.append({"sample_id": sample_id, "error": str(error)})
    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "first_checker_floor_complete": not failed,
        "spec18_typed_checking_complete": False,
        "public_cli_frozen": False,
        "runtime_plan_emitted": False,
        "run_local_claimed": False,
        "run_docker_claimed": False,
    }


def closeout() -> dict[str, Any]:
    return {
        "sample_root": "samples/practical-alpha1",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "validation_floor": [
            "cargo test -p mir-ast practical_alpha1_checker -- --nocapture",
            "cargo test -p mir-ast",
            "python3 scripts/practical_alpha1_check.py check samples/practical-alpha1/packages/chk-lif-02-fallback-access-valid --format json",
            "python3 scripts/practical_alpha1_check.py check-all --format json",
            "python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_validate_docs",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        "first_checker_floor_complete": True,
        "spec18_typed_checking_complete": False,
        "public_cli_frozen": False,
        "runtime_plan_emitted": False,
        "run_local_claimed": False,
        "run_docker_claimed": False,
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(f"{row['sample_id']} {row['summary']}" for row in payload)
    return json.dumps(payload, ensure_ascii=False, indent=2)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "Practical alpha-1 checker-only command surface. This is an alpha-local "
            "non-final front-door over practical package inputs and does not emit runtime plans."
        )
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser("list")

    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")

    check_parser = subparsers.add_parser("check")
    check_parser.add_argument("package_path")

    subparsers.add_parser("check-all")
    subparsers.add_parser("closeout")

    parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    return parser


def normalize_argv(argv: list[str] | None) -> list[str]:
    values = list(sys.argv[1:] if argv is None else argv)
    hoisted_root_options: list[str] = []
    remainder: list[str] = []
    index = 0
    while index < len(values):
        current = values[index]
        if current == "--format" and index + 1 < len(values):
            hoisted_root_options.extend(values[index : index + 2])
            index += 2
            continue
        remainder.append(current)
        index += 1

    values = [*hoisted_root_options, *remainder]
    known = {"list", "run", "check", "check-all", "closeout"}
    payload = list(remainder)
    if payload and payload[0] not in known and not payload[0].startswith("-"):
        return [*hoisted_root_options, "check", *payload]
    return values


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(normalize_argv(argv))

    if args.command == "list":
        payload: Any = list_samples()
    elif args.command == "run":
        payload = run_sample(args.sample_id)
    elif args.command == "check":
        payload = check_path(args.package_path)
    elif args.command == "check-all":
        payload = check_all()
    else:
        payload = closeout()

    if args.format == "json":
        print(json.dumps(payload, ensure_ascii=False, indent=2))
    else:
        print(format_pretty(payload))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
