#!/usr/bin/env python3

import argparse
import subprocess
import sys
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
DEFAULT_ARTIFACT_ROOT = REPO_ROOT / "target" / "current-l2-detached"
EMITTER_CMD = [
    "cargo",
    "run",
    "-q",
    "-p",
    "mir-semantics",
    "--example",
    "current_l2_emit_detached_bundle",
    "--",
]
AGGREGATE_EMITTER_CMD = [
    "cargo",
    "run",
    "-q",
    "-p",
    "mir-semantics",
    "--example",
    "current_l2_emit_detached_aggregate",
    "--",
]
DIFF_HELPER = SCRIPT_DIR / "current_l2_diff_detached_artifacts.py"


def ensure_run_label(label: str) -> str:
    if not label:
        raise ValueError("run label must not be empty")
    if label in {".", ".."} or "/" in label:
        raise ValueError("run label must be a single path segment")
    return label


def bundle_artifact_path(
    artifact_root: Path,
    run_label: str,
    fixture_path: Path,
) -> Path:
    return (
        artifact_root
        / "bundles"
        / ensure_run_label(run_label)
        / f"{fixture_path.stem}.detached.json"
    )


def aggregate_artifact_path(
    artifact_root: Path,
    run_label: str,
) -> Path:
    return (
        artifact_root
        / "aggregates"
        / ensure_run_label(run_label)
        / "batch-summary.detached.json"
    )


def run_subprocess(cmd: list[str]) -> int:
    completed = subprocess.run(cmd, cwd=REPO_ROOT)
    return completed.returncode


def emit_fixture(
    fixture_path: Path,
    output_path: Path,
    overwrite: bool,
) -> int:
    output_path.parent.mkdir(parents=True, exist_ok=True)
    if output_path.exists() and not overwrite:
        print(
            f"artifact already exists: {output_path} (use --overwrite to replace)",
            file=sys.stderr,
        )
        return 2
    cmd = [
        *EMITTER_CMD,
        str(fixture_path),
        "--output",
        str(output_path),
    ]
    return run_subprocess(cmd)


def emit_aggregate(
    fixture_directory: Path,
    output_path: Path,
    overwrite: bool,
) -> int:
    output_path.parent.mkdir(parents=True, exist_ok=True)
    if output_path.exists() and not overwrite:
        print(
            f"artifact already exists: {output_path} (use --overwrite to replace)",
            file=sys.stderr,
        )
        return 2
    cmd = [
        *AGGREGATE_EMITTER_CMD,
        str(fixture_directory),
        "--output",
        str(output_path),
    ]
    return run_subprocess(cmd)


def compare_artifacts(left: Path, right: Path) -> int:
    cmd = [sys.executable, str(DIFF_HELPER), str(left), str(right)]
    return run_subprocess(cmd)


def command_emit_fixture(args: argparse.Namespace) -> int:
    fixture_path = Path(args.fixture_path)
    output_path = (
        Path(args.output_path)
        if args.output_path
        else bundle_artifact_path(Path(args.artifact_root), args.run_label, fixture_path)
    )
    exit_code = emit_fixture(fixture_path, output_path, args.overwrite)
    if exit_code == 0:
        print(output_path)
    return exit_code


def command_compare_artifacts(args: argparse.Namespace) -> int:
    return compare_artifacts(Path(args.left_artifact), Path(args.right_artifact))


def command_emit_aggregate(args: argparse.Namespace) -> int:
    fixture_directory = Path(args.fixture_directory)
    output_path = (
        Path(args.output_path)
        if args.output_path
        else aggregate_artifact_path(Path(args.artifact_root), args.run_label)
    )
    exit_code = emit_aggregate(fixture_directory, output_path, args.overwrite)
    if exit_code == 0:
        print(output_path)
    return exit_code


def command_compare_fixtures(args: argparse.Namespace) -> int:
    artifact_root = Path(args.artifact_root)
    left_fixture = Path(args.left_fixture)
    right_fixture = Path(args.right_fixture)
    left_artifact = bundle_artifact_path(artifact_root, args.left_label, left_fixture)
    right_artifact = bundle_artifact_path(artifact_root, args.right_label, right_fixture)

    left_exit = emit_fixture(left_fixture, left_artifact, args.overwrite)
    if left_exit != 0:
        return left_exit

    right_exit = emit_fixture(right_fixture, right_artifact, args.overwrite)
    if right_exit != 0:
        return right_exit

    print(f"left artifact : {left_artifact}", flush=True)
    print(f"right artifact: {right_artifact}", flush=True)
    return compare_artifacts(left_artifact, right_artifact)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "current L2 detached validation loop を回すための non-production helper。"
            " bundle-first emitter と payload-core diff helper を薄くつなぐ。"
        )
    )
    parser.set_defaults(func=None)

    subparsers = parser.add_subparsers(dest="command")

    emit_fixture_parser = subparsers.add_parser(
        "emit-fixture",
        help="1 fixture を detached bundle artifact として保存する",
    )
    emit_fixture_parser.add_argument("fixture_path")
    emit_fixture_parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root directory (default: target/current-l2-detached)",
    )
    emit_fixture_parser.add_argument(
        "--run-label",
        default="manual",
        help="bundle artifact を保存する run label",
    )
    emit_fixture_parser.add_argument(
        "--output-path",
        help="explicit output path; when omitted, bundle path is derived from root/label/stem",
    )
    emit_fixture_parser.add_argument(
        "--overwrite",
        action="store_true",
        help="existing artifact を明示的に上書きする",
    )
    emit_fixture_parser.set_defaults(func=command_emit_fixture)

    emit_aggregate_parser = subparsers.add_parser(
        "emit-aggregate",
        help="fixture directory を aggregate detached artifact として保存する",
    )
    emit_aggregate_parser.add_argument("fixture_directory")
    emit_aggregate_parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root directory (default: target/current-l2-detached)",
    )
    emit_aggregate_parser.add_argument(
        "--run-label",
        default="manual",
        help="aggregate artifact を保存する run label",
    )
    emit_aggregate_parser.add_argument(
        "--output-path",
        help="explicit output path; when omitted, aggregate path is derived from root/label",
    )
    emit_aggregate_parser.add_argument(
        "--overwrite",
        action="store_true",
        help="existing artifact を明示的に上書きする",
    )
    emit_aggregate_parser.set_defaults(func=command_emit_aggregate)

    compare_artifacts_parser = subparsers.add_parser(
        "compare-artifacts",
        help="既存 2 artifact の payload_core を比較する",
    )
    compare_artifacts_parser.add_argument("left_artifact")
    compare_artifacts_parser.add_argument("right_artifact")
    compare_artifacts_parser.set_defaults(func=command_compare_artifacts)

    compare_fixtures_parser = subparsers.add_parser(
        "compare-fixtures",
        help="2 fixture を emit して payload_core を比較する",
    )
    compare_fixtures_parser.add_argument("left_fixture")
    compare_fixtures_parser.add_argument("right_fixture")
    compare_fixtures_parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root directory (default: target/current-l2-detached)",
    )
    compare_fixtures_parser.add_argument(
        "--left-label",
        default="left",
        help="left artifact を保存する run label",
    )
    compare_fixtures_parser.add_argument(
        "--right-label",
        default="right",
        help="right artifact を保存する run label",
    )
    compare_fixtures_parser.add_argument(
        "--overwrite",
        action="store_true",
        help="existing artifact を明示的に上書きする",
    )
    compare_fixtures_parser.set_defaults(func=command_compare_fixtures)

    return parser


def main() -> int:
    parser = build_parser()
    args = parser.parse_args()
    if args.func is None:
        parser.print_help()
        return 2
    try:
        return args.func(args)
    except ValueError as error:
        print(error, file=sys.stderr)
        return 2


if __name__ == "__main__":
    sys.exit(main())
