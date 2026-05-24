#!/usr/bin/env python3
from __future__ import annotations

import argparse
import html
import json
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Any


REPO_ROOT = Path(__file__).resolve().parents[1]


@dataclass(frozen=True)
class PlannedCommand:
    name: str
    argv: list[str]
    json_required: bool = True


@dataclass(frozen=True)
class CommandPlan:
    out_dir: Path
    reports_dir: Path
    bundle_path: Path
    html_path: Path
    commands: list[PlannedCommand]


@dataclass(frozen=True)
class CommandResult:
    name: str
    argv: list[str]
    returncode: int
    stdout: str
    stderr: str
    payload: dict[str, Any] | None
    semantic_errors: list[str]


def validation_command(name: str, argv: list[str]) -> PlannedCommand:
    return PlannedCommand(name=name, argv=argv, json_required=False)


def helper_command(name: str, script_name: str, *args: str) -> PlannedCommand:
    return PlannedCommand(
        name=name,
        argv=["python3", f"scripts/{script_name}", *args, "--format", "json"],
    )


def cargo_test_args(*args: str) -> list[str]:
    return ["cargo", "test", *args, "--", "--nocapture"]


def plan_check_all(out_dir: Path) -> CommandPlan:
    reports_dir = out_dir / "reports"
    return CommandPlan(
        out_dir=out_dir,
        reports_dir=reports_dir,
        bundle_path=out_dir / "bundle.json",
        html_path=out_dir / "index.html",
        commands=[
            validation_command(
                "validation:test-validate-docs",
                ["python3", "-m", "unittest", "scripts.tests.test_validate_docs"],
            ),
            validation_command(
                "validation:source-hierarchy",
                ["python3", "scripts/check_source_hierarchy.py"],
            ),
            validation_command(
                "validation:validate-docs",
                ["python3", "scripts/validate_docs.py"],
            ),
            validation_command("validation:cargo-fmt", ["cargo", "fmt", "--check"]),
            validation_command("validation:git-diff-check", ["git", "diff", "--check"]),
            validation_command(
                "test:surface-parser",
                cargo_test_args("-p", "mir-ast", "--test", "surface_mir_parser"),
            ),
            validation_command(
                "test:indexed-state-semantics",
                cargo_test_args("-p", "mir-semantics", "--test", "indexed_state_semantics"),
            ),
            validation_command(
                "test:surface-to-core-elaboration",
                cargo_test_args("-p", "mir-semantics", "--test", "surface_to_core_elaboration"),
            ),
            validation_command(
                "test:surface-samples",
                ["python3", "-m", "unittest", "scripts.tests.test_surface_mir_samples"],
            ),
            validation_command(
                "test:surface-release-check",
                ["python3", "-m", "unittest", "scripts.tests.test_surface_mir_release_check"],
            ),
            helper_command("helper:surface-samples", "surface_mir_samples.py", "check-all"),
            helper_command(
                "helper:surface-authoring",
                "surface_mir_authoring_check.py",
                "check-all",
            ),
        ],
    )


def command_plan_payload(plan: CommandPlan) -> dict[str, Any]:
    return {
        "surface_kind": "surface_mir_release_check_plan",
        "out_dir": str(plan.out_dir),
        "reports_dir": str(plan.reports_dir),
        "bundle_path": str(plan.bundle_path),
        "html_path": str(plan.html_path),
        "commands": [
            {
                "name": command.name,
                "argv": command.argv,
                "json_required": command.json_required,
            }
            for command in plan.commands
        ],
        "final_public_grammar_frozen": False,
    }


def semantic_errors_for_result(command: PlannedCommand, payload: dict[str, Any] | None) -> list[str]:
    if payload is None:
        return []
    errors: list[str] = []
    if command.name == "helper:surface-samples":
        if payload.get("failed"):
            errors.append("surface samples helper reported failed rows")
        if payload.get("sample_count") != 21:
            errors.append("surface samples helper sample_count mismatch for P-SURF-03")
        if payload.get("workflow_ready") is not False:
            errors.append("P-SURF-03 helper must not claim workflow_ready")
    if command.name == "helper:surface-authoring" and payload.get("accepted") is not True:
        errors.append("surface authoring check rejected current source root")
    return errors


def run_command(command: PlannedCommand) -> CommandResult:
    completed = subprocess.run(
        command.argv,
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    payload = None
    semantic_errors: list[str] = []
    if command.json_required and completed.stdout.strip():
        try:
            payload = json.loads(completed.stdout)
        except json.JSONDecodeError as error:
            semantic_errors.append(f"stdout was not JSON: {error}")
    if completed.returncode != 0:
        semantic_errors.append(f"returncode {completed.returncode}")
    semantic_errors.extend(semantic_errors_for_result(command, payload))
    return CommandResult(
        name=command.name,
        argv=command.argv,
        returncode=completed.returncode,
        stdout=completed.stdout,
        stderr=completed.stderr,
        payload=payload,
        semantic_errors=semantic_errors,
    )


def result_payload(result: CommandResult) -> dict[str, Any]:
    return {
        "name": result.name,
        "argv": result.argv,
        "returncode": result.returncode,
        "stdout": result.stdout,
        "stderr": result.stderr,
        "payload": result.payload,
        "semantic_errors": result.semantic_errors,
        "accepted": not result.semantic_errors,
    }


def write_report(plan: CommandPlan, result: CommandResult) -> None:
    plan.reports_dir.mkdir(parents=True, exist_ok=True)
    safe_name = result.name.replace(":", "__").replace("/", "_")
    path = plan.reports_dir / f"{safe_name}.json"
    path.write_text(json.dumps(result_payload(result), indent=2, ensure_ascii=False), encoding="utf-8")


def render_html(bundle: dict[str, Any]) -> str:
    rows = []
    for result in bundle["results"]:
        status = "ok" if result["accepted"] else "failed"
        rows.append(
            "<tr>"
            f"<td>{html.escape(result['name'])}</td>"
            f"<td>{html.escape(status)}</td>"
            f"<td>{html.escape(str(result['returncode']))}</td>"
            f"<td>{html.escape(', '.join(result['semantic_errors']))}</td>"
            "</tr>"
        )
    return "\n".join(
        [
            "<!doctype html>",
            "<meta charset=\"utf-8\">",
            "<title>Surface Mir Release Check</title>",
            "<h1>Surface Mir Release Check</h1>",
            f"<p>ready: {html.escape(str(bundle['surface_mir_release_check_ready']))}</p>",
            "<table><thead><tr><th>command</th><th>status</th><th>returncode</th><th>errors</th></tr></thead><tbody>",
            *rows,
            "</tbody></table>",
        ]
    )


def run_check_all(out_dir: Path) -> dict[str, Any]:
    plan = plan_check_all(out_dir)
    plan.out_dir.mkdir(parents=True, exist_ok=True)
    results = []
    for command in plan.commands:
        result = run_command(command)
        write_report(plan, result)
        results.append(result_payload(result))
    failed = [result["name"] for result in results if not result["accepted"]]
    bundle = {
        "surface_kind": "surface_mir_release_check_report",
        "scope": "p_surf_03_surface_to_core_elaboration",
        "out_dir": str(plan.out_dir),
        "reports_dir": str(plan.reports_dir),
        "bundle_path": str(plan.bundle_path),
        "html_path": str(plan.html_path),
        "surface_mir_release_check_ready": not failed,
        "failed_commands": failed,
        "results": results,
        "non_claims": [
            "no final public grammar / ABI / SDK",
            "no auto communication publish/observe completion",
            "no role-admission capability grant completion",
            "no runtime or source patch hot-plug completion",
            "no generated package artifact authority",
        ],
        "final_public_grammar_frozen": False,
    }
    plan.bundle_path.write_text(json.dumps(bundle, indent=2, ensure_ascii=False), encoding="utf-8")
    plan.html_path.write_text(render_html(bundle), encoding="utf-8")
    return bundle


def normalize_argv(argv: list[str]) -> list[str]:
    if "--format" not in argv:
        return argv
    index = argv.index("--format")
    if index + 1 >= len(argv) or index == 0:
        return argv
    value = argv[index + 1]
    stripped = argv[:index] + argv[index + 2 :]
    return ["--format", value, *stripped]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--format", choices=["json", "pretty"], default="pretty")
    subparsers = parser.add_subparsers(dest="command", required=True)
    plan_parser = subparsers.add_parser("plan")
    plan_parser.add_argument("--out", default="/tmp/mirrorea-surface-release")
    check_parser = subparsers.add_parser("check-all")
    check_parser.add_argument("--out", default="/tmp/mirrorea-surface-release")
    args = parser.parse_args(normalize_argv(list(argv)) if argv is not None else None)

    if args.command == "plan":
        payload = command_plan_payload(plan_check_all(Path(args.out)))
    else:
        payload = run_check_all(Path(args.out))

    if args.format == "json":
        print(json.dumps(payload, indent=2, ensure_ascii=False))
    else:
        print(json.dumps(payload, indent=2, ensure_ascii=False))

    if args.command == "check-all":
        return 0 if payload["surface_mir_release_check_ready"] else 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
