#!/usr/bin/env python3

import argparse
import shutil
import subprocess
import sys
import tempfile
import json
from pathlib import Path

import current_l2_checked_reasons_assist as checked_reasons_assist
import current_l2_missing_option_checker as missing_option_checker
import current_l2_same_lineage_checker as same_lineage_checker
import current_l2_reason_code_readiness as reason_code_readiness
import current_l2_reason_codes_assist as reason_codes_assist


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
STATIC_GATE_EMITTER_CMD = [
    "cargo",
    "run",
    "-q",
    "-p",
    "mir-semantics",
    "--example",
    "current_l2_emit_static_gate",
    "--",
]
DIFF_HELPER = SCRIPT_DIR / "current_l2_diff_detached_artifacts.py"
AGGREGATE_DIFF_HELPER = SCRIPT_DIR / "current_l2_diff_detached_aggregates.py"
STATIC_GATE_DIFF_HELPER = SCRIPT_DIR / "current_l2_diff_static_gate_artifacts.py"


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


def static_gate_artifact_path(
    artifact_root: Path,
    run_label: str,
    fixture_path: Path,
) -> Path:
    return (
        artifact_root
        / "static-gates"
        / ensure_run_label(run_label)
        / f"{fixture_path.stem}.static-gate.json"
    )


def smoke_full_run_label(run_label: str) -> str:
    return ensure_run_label(f"{ensure_run_label(run_label)}-full")


def smoke_single_run_label(run_label: str) -> str:
    return ensure_run_label(f"{ensure_run_label(run_label)}-single")


def run_subprocess(cmd: list[str]) -> int:
    completed = subprocess.run(cmd, cwd=REPO_ROOT)
    return completed.returncode


def read_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def fixture_enters_evaluation(fixture_path: Path) -> bool:
    fixture = read_json(fixture_path)
    expected_runtime = fixture.get("expected_runtime")
    if not isinstance(expected_runtime, dict):
        return False
    return bool(expected_runtime.get("enters_evaluation"))


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


def emit_static_gate(
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
        *STATIC_GATE_EMITTER_CMD,
        str(fixture_path),
        "--output",
        str(output_path),
    ]
    return run_subprocess(cmd)


def compare_artifacts(left: Path, right: Path) -> int:
    cmd = [sys.executable, str(DIFF_HELPER), str(left), str(right)]
    return run_subprocess(cmd)


def compare_aggregates(left: Path, right: Path) -> int:
    cmd = [sys.executable, str(AGGREGATE_DIFF_HELPER), str(left), str(right)]
    return run_subprocess(cmd)


def compare_static_gates(left: Path, right: Path) -> int:
    cmd = [sys.executable, str(STATIC_GATE_DIFF_HELPER), str(left), str(right)]
    return run_subprocess(cmd)


def check_same_lineage_first_checker(fixture_path: Path, artifact_path: Path) -> int:
    return same_lineage_checker.main([str(fixture_path), str(artifact_path)])


def check_missing_option_second_checker(fixture_path: Path, artifact_path: Path) -> int:
    return missing_option_checker.main([str(fixture_path), str(artifact_path)])


def copy_fixture_bundle_to_directory(fixture_path: Path, output_dir: Path) -> None:
    output_dir.mkdir(parents=True, exist_ok=True)
    shutil.copy2(fixture_path, output_dir / fixture_path.name)

    sidecar_path = fixture_path.with_name(f"{fixture_path.stem}.host-plan.json")
    if sidecar_path.exists():
        shutil.copy2(sidecar_path, output_dir / sidecar_path.name)


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


def command_compare_aggregates(args: argparse.Namespace) -> int:
    artifact_root = Path(args.artifact_root)
    left_artifact = aggregate_artifact_path(artifact_root, args.left_run_label)
    right_artifact = aggregate_artifact_path(artifact_root, args.right_run_label)
    return compare_aggregates(left_artifact, right_artifact)


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


def command_emit_static_gate(args: argparse.Namespace) -> int:
    fixture_path = Path(args.fixture_path)
    output_path = (
        Path(args.output_path)
        if args.output_path
        else static_gate_artifact_path(
            Path(args.artifact_root), args.run_label, fixture_path
        )
    )
    exit_code = emit_static_gate(fixture_path, output_path, args.overwrite)
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


def command_smoke_fixture(args: argparse.Namespace) -> int:
    artifact_root = Path(args.artifact_root)
    fixture_path = Path(args.fixture_path)
    run_label = ensure_run_label(args.run_label)
    bundle_artifact = bundle_artifact_path(artifact_root, run_label, fixture_path)

    emit_exit = emit_fixture(fixture_path, bundle_artifact, args.overwrite)
    if emit_exit != 0:
        return emit_exit

    print(f"fixture artifact: {bundle_artifact}", flush=True)

    if args.reference_fixture:
        reference_fixture = Path(args.reference_fixture)
        reference_label = ensure_run_label(args.reference_label)
        reference_artifact = bundle_artifact_path(
            artifact_root, reference_label, reference_fixture
        )
        reference_exit = emit_fixture(
            reference_fixture, reference_artifact, args.overwrite
        )
        if reference_exit != 0:
            return reference_exit

        print(f"reference artifact: {reference_artifact}", flush=True)
        compare_exit = compare_artifacts(bundle_artifact, reference_artifact)
        if compare_exit not in {0, 1}:
            return compare_exit

    full_aggregate = aggregate_artifact_path(
        artifact_root,
        smoke_full_run_label(run_label),
    )
    single_aggregate = aggregate_artifact_path(
        artifact_root,
        smoke_single_run_label(run_label),
    )

    full_exit = emit_aggregate(fixture_path.parent, full_aggregate, args.overwrite)
    if full_exit != 0:
        return full_exit

    with tempfile.TemporaryDirectory() as temp_dir:
        single_fixture_dir = Path(temp_dir)
        copy_fixture_bundle_to_directory(fixture_path, single_fixture_dir)
        single_exit = emit_aggregate(
            single_fixture_dir,
            single_aggregate,
            args.overwrite,
        )
        if single_exit != 0:
            return single_exit

    print(f"aggregate artifact (full)  : {full_aggregate}", flush=True)
    print(f"aggregate artifact (single): {single_aggregate}", flush=True)
    compare_exit = compare_aggregates(full_aggregate, single_aggregate)
    if compare_exit not in {0, 1}:
        return compare_exit

    return 0


def command_compare_static_gates(args: argparse.Namespace) -> int:
    return compare_static_gates(
        Path(args.left_artifact),
        Path(args.right_artifact),
    )


def command_smoke_static_gate(args: argparse.Namespace) -> int:
    artifact_root = Path(args.artifact_root)
    fixture_path = Path(args.fixture_path)
    run_label = ensure_run_label(args.run_label)
    left_artifact = static_gate_artifact_path(artifact_root, run_label, fixture_path)

    emit_exit = emit_static_gate(fixture_path, left_artifact, args.overwrite)
    if emit_exit != 0:
        return emit_exit

    print(f"static gate artifact: {left_artifact}", flush=True)

    if not args.reference_fixture:
        return 0

    reference_fixture = Path(args.reference_fixture)
    reference_label = ensure_run_label(args.reference_label)
    right_artifact = static_gate_artifact_path(
        artifact_root,
        reference_label,
        reference_fixture,
    )
    reference_exit = emit_static_gate(
        reference_fixture,
        right_artifact,
        args.overwrite,
    )
    if reference_exit != 0:
        return reference_exit

    print(f"reference static gate artifact: {right_artifact}", flush=True)
    compare_exit = compare_static_gates(left_artifact, right_artifact)
    if compare_exit not in {0, 1}:
        return compare_exit
    return 0


def command_suggest_checked_reasons(args: argparse.Namespace) -> int:
    fixture_path = Path(args.fixture_path)
    output_path = (
        Path(args.output_path)
        if args.output_path
        else static_gate_artifact_path(
            Path(args.artifact_root), args.run_label, fixture_path
        )
    )

    emit_exit = emit_static_gate(fixture_path, output_path, args.overwrite)
    if emit_exit != 0:
        return emit_exit

    print(f"static gate artifact: {output_path}", flush=True)
    return checked_reasons_assist.main([str(fixture_path), str(output_path)])


def command_suggest_reason_codes(args: argparse.Namespace) -> int:
    fixture_path = Path(args.fixture_path)
    output_path = (
        Path(args.output_path)
        if args.output_path
        else static_gate_artifact_path(
            Path(args.artifact_root), args.run_label, fixture_path
        )
    )

    emit_exit = emit_static_gate(fixture_path, output_path, args.overwrite)
    if emit_exit != 0:
        return emit_exit

    print(f"static gate artifact: {output_path}", flush=True)
    return reason_codes_assist.main([str(fixture_path), str(output_path)])


def command_scan_reason_code_readiness(args: argparse.Namespace) -> int:
    fixture_directory = Path(args.fixture_directory)
    artifact_root = Path(args.artifact_root)
    artifact_directory = artifact_root / "static-gates" / ensure_run_label(args.run_label)

    fixture_paths = sorted(
        path
        for path in fixture_directory.glob("*.json")
        if not path.name.endswith(".host-plan.json")
    )
    for fixture_path in fixture_paths:
        if fixture_enters_evaluation(fixture_path):
            continue
        output_path = static_gate_artifact_path(
            artifact_root, args.run_label, fixture_path
        )
        exit_code = emit_static_gate(fixture_path, output_path, args.overwrite)
        if exit_code != 0:
            return exit_code

    return reason_code_readiness.main(
        [str(fixture_directory), str(artifact_directory)]
    )


def command_smoke_same_lineage_checker(args: argparse.Namespace) -> int:
    fixture_path = Path(args.fixture_path)
    output_path = (
        Path(args.output_path)
        if args.output_path
        else static_gate_artifact_path(
            Path(args.artifact_root), args.run_label, fixture_path
        )
    )

    emit_exit = emit_static_gate(fixture_path, output_path, args.overwrite)
    if emit_exit != 0:
        return emit_exit

    print(f"static gate artifact: {output_path}", flush=True)
    return check_same_lineage_first_checker(fixture_path, output_path)


def command_smoke_missing_option_checker(args: argparse.Namespace) -> int:
    fixture_path = Path(args.fixture_path)
    output_path = (
        Path(args.output_path)
        if args.output_path
        else static_gate_artifact_path(
            Path(args.artifact_root), args.run_label, fixture_path
        )
    )

    emit_exit = emit_static_gate(fixture_path, output_path, args.overwrite)
    if emit_exit != 0:
        return emit_exit

    print(f"static gate artifact: {output_path}", flush=True)
    return check_missing_option_second_checker(fixture_path, output_path)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "current L2 detached validation loop を回すための non-production helper。"
            " bundle-first / aggregate emitter と bundle / aggregate diff helper を薄くつなぐ。"
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

    emit_static_gate_parser = subparsers.add_parser(
        "emit-static-gate",
        help="1 fixture の static gate artifact を保存する",
    )
    emit_static_gate_parser.add_argument("fixture_path")
    emit_static_gate_parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root directory (default: target/current-l2-detached)",
    )
    emit_static_gate_parser.add_argument(
        "--run-label",
        default="manual",
        help="static gate artifact を保存する run label",
    )
    emit_static_gate_parser.add_argument(
        "--output-path",
        help="explicit output path; when omitted, static gate path is derived from root/label/stem",
    )
    emit_static_gate_parser.add_argument(
        "--overwrite",
        action="store_true",
        help="existing artifact を明示的に上書きする",
    )
    emit_static_gate_parser.set_defaults(func=command_emit_static_gate)

    compare_artifacts_parser = subparsers.add_parser(
        "compare-artifacts",
        help="既存 2 artifact の payload_core を比較する",
    )
    compare_artifacts_parser.add_argument("left_artifact")
    compare_artifacts_parser.add_argument("right_artifact")
    compare_artifacts_parser.set_defaults(func=command_compare_artifacts)

    compare_aggregates_parser = subparsers.add_parser(
        "compare-aggregates",
        help="2 run label から aggregate artifact を導出して summary_core を比較する",
    )
    compare_aggregates_parser.add_argument("left_run_label")
    compare_aggregates_parser.add_argument("right_run_label")
    compare_aggregates_parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root directory (default: target/current-l2-detached)",
    )
    compare_aggregates_parser.set_defaults(func=command_compare_aggregates)

    compare_static_gate_parser = subparsers.add_parser(
        "compare-static-gates",
        help="既存 2 static gate artifact の checker_core を比較する",
    )
    compare_static_gate_parser.add_argument("left_artifact")
    compare_static_gate_parser.add_argument("right_artifact")
    compare_static_gate_parser.set_defaults(func=command_compare_static_gates)

    suggest_checked_reasons_parser = subparsers.add_parser(
        "suggest-checked-reasons",
        help=(
            "1 fixture の static gate artifact を emit して "
            "checked_reasons 候補を display-only で表示する"
        ),
    )
    suggest_checked_reasons_parser.add_argument("fixture_path")
    suggest_checked_reasons_parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root directory (default: target/current-l2-detached)",
    )
    suggest_checked_reasons_parser.add_argument(
        "--run-label",
        default="manual",
        help="static gate artifact を保存する run label",
    )
    suggest_checked_reasons_parser.add_argument(
        "--output-path",
        help="explicit output path; when omitted, static gate path is derived from root/label/stem",
    )
    suggest_checked_reasons_parser.add_argument(
        "--overwrite",
        action="store_true",
        help="existing artifact を明示的に上書きする",
    )
    suggest_checked_reasons_parser.set_defaults(func=command_suggest_checked_reasons)

    suggest_reason_codes_parser = subparsers.add_parser(
        "suggest-reason-codes",
        help=(
            "1 fixture の static gate artifact を emit して "
            "helper-local reason_codes 候補を display-only で表示する"
        ),
    )
    suggest_reason_codes_parser.add_argument("fixture_path")
    suggest_reason_codes_parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root directory (default: target/current-l2-detached)",
    )
    suggest_reason_codes_parser.add_argument(
        "--run-label",
        default="manual",
        help="static gate artifact を保存する run label",
    )
    suggest_reason_codes_parser.add_argument(
        "--output-path",
        help="explicit output path; when omitted, static gate path is derived from root/label/stem",
    )
    suggest_reason_codes_parser.add_argument(
        "--overwrite",
        action="store_true",
        help="existing artifact を明示的に上書きする",
    )
    suggest_reason_codes_parser.set_defaults(func=command_suggest_reason_codes)

    scan_reason_code_readiness_parser = subparsers.add_parser(
        "scan-reason-code-readiness",
        help=(
            "static-only fixture corpus を走査し、display-only reason_codes suggestion "
            "の readiness を detached static gate artifact から要約する"
        ),
    )
    scan_reason_code_readiness_parser.add_argument("fixture_directory")
    scan_reason_code_readiness_parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
    )
    scan_reason_code_readiness_parser.add_argument(
        "--run-label",
        required=True,
    )
    scan_reason_code_readiness_parser.add_argument(
        "--overwrite",
        action="store_true",
        help="既存 artifact を上書きする",
    )
    scan_reason_code_readiness_parser.set_defaults(
        func=command_scan_reason_code_readiness
    )

    smoke_same_lineage_checker_parser = subparsers.add_parser(
        "smoke-same-lineage-checker",
        help=(
            "1 fixture の static gate artifact を保存し、same-lineage first checker "
            "spike をその artifact に対して回す"
        ),
    )
    smoke_same_lineage_checker_parser.add_argument("fixture_path")
    smoke_same_lineage_checker_parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root directory (default: target/current-l2-detached)",
    )
    smoke_same_lineage_checker_parser.add_argument(
        "--run-label",
        default="same-lineage",
        help="static gate artifact を保存する run label",
    )
    smoke_same_lineage_checker_parser.add_argument(
        "--output-path",
        help="explicit output path; when omitted, static gate path is derived from root/label/stem",
    )
    smoke_same_lineage_checker_parser.add_argument(
        "--overwrite",
        action="store_true",
        help="existing artifacts を明示的に上書きする",
    )
    smoke_same_lineage_checker_parser.set_defaults(
        func=command_smoke_same_lineage_checker
    )

    smoke_missing_option_checker_parser = subparsers.add_parser(
        "smoke-missing-option-checker",
        help=(
            "1 fixture の static gate artifact を保存し、missing-option second "
            "checker spike をその artifact に対して回す"
        ),
    )
    smoke_missing_option_checker_parser.add_argument("fixture_path")
    smoke_missing_option_checker_parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root directory (default: target/current-l2-detached)",
    )
    smoke_missing_option_checker_parser.add_argument(
        "--run-label",
        default="missing-option",
        help="static gate artifact を保存する run label",
    )
    smoke_missing_option_checker_parser.add_argument(
        "--output-path",
        help="explicit output path; when omitted, static gate path is derived from root/label/stem",
    )
    smoke_missing_option_checker_parser.add_argument(
        "--overwrite",
        action="store_true",
        help="existing artifacts を明示的に上書きする",
    )
    smoke_missing_option_checker_parser.set_defaults(
        func=command_smoke_missing_option_checker
    )

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

    smoke_fixture_parser = subparsers.add_parser(
        "smoke-fixture",
        help=(
            "1 fixture を bundle-first detached loop に載せ、必要なら reference fixture compare と "
            "single-fixture aggregate smoke までまとめて回す"
        ),
    )
    smoke_fixture_parser.add_argument("fixture_path")
    smoke_fixture_parser.add_argument(
        "--reference-fixture",
        help="optional reference fixture for payload_core compare",
    )
    smoke_fixture_parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root directory (default: target/current-l2-detached)",
    )
    smoke_fixture_parser.add_argument(
        "--run-label",
        default="smoke",
        help="primary run label for the fixture artifact and aggregate smoke",
    )
    smoke_fixture_parser.add_argument(
        "--reference-label",
        default="reference",
        help="run label used when --reference-fixture is supplied",
    )
    smoke_fixture_parser.add_argument(
        "--overwrite",
        action="store_true",
        help="existing artifacts を明示的に上書きする",
    )
    smoke_fixture_parser.set_defaults(func=command_smoke_fixture)

    smoke_static_gate_parser = subparsers.add_parser(
        "smoke-static-gate",
        help="1 fixture の static gate artifact を保存し、必要なら reference fixture compare まで回す",
    )
    smoke_static_gate_parser.add_argument("fixture_path")
    smoke_static_gate_parser.add_argument(
        "--reference-fixture",
        help="optional reference fixture for checker_core compare",
    )
    smoke_static_gate_parser.add_argument(
        "--artifact-root",
        default=str(DEFAULT_ARTIFACT_ROOT),
        help="artifact root directory (default: target/current-l2-detached)",
    )
    smoke_static_gate_parser.add_argument(
        "--run-label",
        default="static-gate",
        help="primary run label for the static gate artifact",
    )
    smoke_static_gate_parser.add_argument(
        "--reference-label",
        default="reference",
        help="run label used when --reference-fixture is supplied",
    )
    smoke_static_gate_parser.add_argument(
        "--overwrite",
        action="store_true",
        help="existing artifacts を明示的に上書きする",
    )
    smoke_static_gate_parser.set_defaults(func=command_smoke_static_gate)

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
