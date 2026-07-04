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
RELEASE_CHECK_SCOPE = "p_surf_99_final_surface_alpha_audit"
SURFACE_SAMPLE_COUNT_FOR_P_SURF_99 = 53


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


def json_command(name: str, argv: list[str]) -> PlannedCommand:
    return PlannedCommand(name=name, argv=argv, json_required=True)


def cargo_test_args(*args: str) -> list[str]:
    return ["cargo", "test", *args, "--", "--nocapture"]


def release_relative_path(path: Path, out_dir: Path) -> str:
    try:
        return path.relative_to(out_dir).as_posix()
    except ValueError:
        return str(path)


def release_display_text(value: str, out_dir: Path) -> str:
    text = value
    for root in sorted({str(REPO_ROOT), str(out_dir)}, key=len, reverse=True):
        text = text.replace(root + "/", "")
        text = text.replace(root, ".")
    return text


def release_display_value(value: Any, out_dir: Path) -> Any:
    if isinstance(value, str):
        display = release_display_text(value, out_dir)
        path = Path(display)
        if path.is_absolute():
            return release_relative_path(path, out_dir)
        return display
    if isinstance(value, list):
        return [release_display_value(item, out_dir) for item in value]
    if isinstance(value, dict):
        return {
            key: release_display_value(item, out_dir)
            for key, item in value.items()
        }
    return value


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
                "test:role-admission-capability-grant",
                cargo_test_args(
                    "-p",
                    "mir-semantics",
                    "--test",
                    "role_admission_capability_grant",
                ),
            ),
            validation_command(
                "test:source-patch-hotplug",
                cargo_test_args("-p", "mir-runtime", "--test", "source_patch_hotplug"),
            ),
            validation_command(
                "test:surface-mir-cli",
                cargo_test_args("-p", "mirrorea-cli", "--test", "surface_mir_cli"),
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
            json_command(
                "anchor:product-alpha1-release",
                [
                    "python3",
                    "scripts/product_alpha1_release_check.py",
                    "--format",
                    "json",
                    "check-all",
                ],
            ),
            json_command(
                "anchor:operational-product-samples",
                [
                    "python3",
                    "scripts/operational_product_samples.py",
                    "check-all",
                    "--format",
                    "json",
                ],
            ),
            json_command(
                "anchor:minimal-alpha1-patterns",
                [
                    "python3",
                    "scripts/minimal_alpha1_patterns.py",
                    "check-all",
                    "--format",
                    "json",
                ],
            ),
        ],
    )


def command_plan_payload(plan: CommandPlan) -> dict[str, Any]:
    return release_display_value({
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
    }, plan.out_dir)


SENSITIVE_DEVTOOLS_KEYS = {
    "activation_cut",
    "auth_evidence_ref",
    "capability_frontier_ref",
    "capability_refs",
    "hotplug_request",
    "membership_frontier_ref",
    "required_capability_witness_refs",
    "required_membership_witness_refs",
    "witness_refs",
}
SENSITIVE_DEVTOOLS_STRING_MARKERS = {
    "admission-witness-",
    "auth-evidence-",
    "capability-frontier-",
    "membership-frontier-",
    "private_token",
    "witness-",
}
REQUIRED_DEVTOOLS_PANELS = {
    "surface_source",
    "generated_core_ir",
    "indexed_state_map",
    "generated_communication",
    "role_admission",
    "patch_lifecycle",
    "source_spans",
}


def contains_sensitive_devtools_material(value: Any) -> bool:
    if isinstance(value, dict):
        return any(
            key in SENSITIVE_DEVTOOLS_KEYS
            or contains_sensitive_devtools_material(nested)
            for key, nested in value.items()
        )
    if isinstance(value, list):
        return any(contains_sensitive_devtools_material(nested) for nested in value)
    if isinstance(value, str):
        return any(marker in value for marker in SENSITIVE_DEVTOOLS_STRING_MARKERS)
    return False


def devtools_semantic_errors(payload: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    results = {
        row.get("sample_id"): row
        for row in payload.get("results") or []
        if row.get("sample_id") in {"DEV-01", "DEV-02"}
    }
    if set(results) != {"DEV-01", "DEV-02"}:
        errors.append("P-SURF-08 DEV-01/DEV-02 rows missing from surface samples")
        return errors
    for sample_id, result in results.items():
        if "raw_parse_report" in result:
            errors.append(f"{sample_id} exposes raw_parse_report")
        actual = result.get("actual") or {}
        verification_report = result.get("verification_report") or {}
        if not REQUIRED_DEVTOOLS_PANELS.issubset(set(actual.get("panel_ids") or [])):
            errors.append(f"{sample_id} missing required devtools panels")
        if actual.get("all_required_panels_present") is not True:
            errors.append(f"{sample_id} did not confirm required devtools panels")
        if actual.get("observer_safe") is not True:
            errors.append(f"{sample_id} did not remain observer-safe")
        if actual.get("raw_private_payload_exposed") is not False:
            errors.append(f"{sample_id} exposed raw private payload")
        if actual.get("source_authority") != ".mir":
            errors.append(f"{sample_id} did not preserve .mir source authority")
        if actual.get("final_public_viewer_frozen") is not False:
            errors.append(f"{sample_id} claimed final viewer ABI")
        if actual.get("indexed_state_semantic_backing") is not True:
            errors.append(f"{sample_id} indexed_state_map is not semantics-backed")
        if verification_report.get("redacted") is not True:
            errors.append(f"{sample_id} verification report is not marked redacted")
        if verification_report.get("contains_sensitive_devtools_material") is not False:
            errors.append(f"{sample_id} verification report detected sensitive material")
        if contains_sensitive_devtools_material(verification_report):
            errors.append(f"{sample_id} verification report contains sensitive material")
    dev01_actual = results["DEV-01"].get("actual") or {}
    dev02_actual = results["DEV-02"].get("actual") or {}
    if results["DEV-01"].get("accepted") is not True or dev01_actual.get("accepted") is not True:
        errors.append("DEV-01 positive devtools row did not pass")
    if results["DEV-02"].get("accepted") is not True or dev02_actual.get("accepted") is not False:
        errors.append("DEV-02 private-field negative row did not fail as expected")
    if "private_field_auto_publish_rejected" not in (
        dev02_actual.get("diagnostic_codes") or []
    ):
        errors.append("DEV-02 missing private-field rejection diagnostic")
    return errors


def semantic_errors_for_result(command: PlannedCommand, payload: dict[str, Any] | None) -> list[str]:
    if payload is None:
        return []
    errors: list[str] = []
    if command.name == "helper:surface-samples":
        if payload.get("failed"):
            errors.append("surface samples helper reported failed rows")
        if payload.get("sample_count") != SURFACE_SAMPLE_COUNT_FOR_P_SURF_99:
            errors.append("surface samples helper sample_count mismatch for P-SURF-99")
        if payload.get("workflow_ready") is not False:
            errors.append("P-SURF-99 helper must not claim workflow_ready")
        errors.extend(devtools_semantic_errors(payload))
    if command.name == "helper:surface-authoring" and payload.get("accepted") is not True:
        errors.append("surface authoring check rejected current source root")
    if command.name == "anchor:product-alpha1-release":
        if payload.get("failed_commands"):
            errors.append("Product Alpha release anchor reported failed commands")
        if payload.get("product_alpha1_release_candidate_ready") is not True:
            errors.append("Product Alpha release anchor is not release-candidate ready")
        if payload.get("product_alpha1_ready") is not True:
            errors.append("Product Alpha release anchor is not product-alpha ready")
    if command.name == "anchor:operational-product-samples":
        if payload.get("failed_commands"):
            errors.append("operational product helper reported failed commands")
        if payload.get("status") != "accepted":
            errors.append("operational product helper was not accepted")
    if command.name == "anchor:minimal-alpha1-patterns":
        if payload.get("failed"):
            errors.append("minimal alpha1 pattern verifier reported failed rows")
        if payload.get("status") != "accepted":
            errors.append("minimal alpha1 pattern verifier was not accepted")
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


def surface_samples_payload_summary(payload: dict[str, Any]) -> dict[str, Any]:
    devtools_results = []
    result_summaries = []
    for row in payload.get("results") or []:
        summary = {
            "sample_id": row.get("sample_id"),
            "family": row.get("family"),
            "runner": row.get("runner"),
            "accepted": row.get("accepted"),
            "mismatches": row.get("mismatches") or [],
        }
        result_summaries.append(summary)
        if row.get("sample_id") in {"DEV-01", "DEV-02"}:
            devtools_results.append(
                {
                    **summary,
                    "actual": row.get("actual") or {},
                    "verification_report": row.get("verification_report") or {},
                }
            )
    return {
        "command": payload.get("command"),
        "family": payload.get("family"),
        "sample_root": payload.get("sample_root"),
        "sample_count": payload.get("sample_count"),
        "passed": payload.get("passed") or [],
        "failed": payload.get("failed") or [],
        "workflow_ready": payload.get("workflow_ready"),
        "results": result_summaries,
        "devtools_results": devtools_results,
        "validation_errors": payload.get("validation_errors") or [],
        "stop_lines": payload.get("stop_lines") or [],
        "non_claims": payload.get("non_claims") or [],
        "redacted": True,
    }


def payload_summary_for_result(result: CommandResult) -> dict[str, Any] | None:
    if result.payload is None:
        return None
    if result.name == "helper:surface-samples":
        return surface_samples_payload_summary(result.payload)
    if result.name == "helper:surface-authoring":
        return {
            "command": result.payload.get("command"),
            "accepted": result.payload.get("accepted"),
            "source_count": result.payload.get("source_count"),
            "source_authority": result.payload.get("source_authority"),
            "final_public_api_frozen": result.payload.get("final_public_api_frozen"),
            "redacted": True,
        }
    if result.name == "anchor:product-alpha1-release":
        return {
            "surface_kind": result.payload.get("surface_kind"),
            "status": result.payload.get("status"),
            "product_alpha1_release_candidate_ready": result.payload.get(
                "product_alpha1_release_candidate_ready"
            ),
            "product_alpha1_ready": result.payload.get("product_alpha1_ready"),
            "failed_commands": result.payload.get("failed_commands") or [],
            "command_result_count": len(result.payload.get("command_results") or []),
            "final_product_claimed": result.payload.get("final_product_claimed"),
            "final_public_api_frozen": result.payload.get("final_public_api_frozen"),
            "redacted": True,
        }
    if result.name == "anchor:operational-product-samples":
        return {
            "surface_kind": result.payload.get("surface_kind"),
            "status": result.payload.get("status"),
            "product_alpha1_ready": result.payload.get("product_alpha1_ready"),
            "failed_commands": result.payload.get("failed_commands") or [],
            "final_public_api_frozen": result.payload.get("final_public_api_frozen"),
            "redacted": True,
        }
    if result.name == "anchor:minimal-alpha1-patterns":
        return {
            "package_id": result.payload.get("package_id"),
            "status": result.payload.get("status"),
            "failed": result.payload.get("failed") or [],
            "strict_family_count": result.payload.get("strict_family_count"),
            "workflow_anchors_checked": result.payload.get(
                "workflow_anchors_checked"
            ),
            "final_public_product_claimed": result.payload.get(
                "final_public_product_claimed"
            ),
            "redacted": True,
        }
    return result.payload


def stdout_summary_for_result(result: CommandResult) -> str:
    if result.name.startswith("helper:surface-"):
        return "<json stdout redacted; see payload summary>"
    if result.name.startswith("anchor:"):
        return "<json stdout summarized; see payload summary>"
    return result.stdout


def result_payload(result: CommandResult) -> dict[str, Any]:
    return {
        "name": result.name,
        "argv": result.argv,
        "returncode": result.returncode,
        "stdout": stdout_summary_for_result(result),
        "stderr": result.stderr,
        "payload": payload_summary_for_result(result),
        "semantic_errors": result.semantic_errors,
        "accepted": not result.semantic_errors,
    }


def write_report(plan: CommandPlan, result: CommandResult) -> dict[str, Any]:
    plan.reports_dir.mkdir(parents=True, exist_ok=True)
    safe_name = result.name.replace(":", "__").replace("/", "_")
    path = plan.reports_dir / f"{safe_name}.json"
    record = release_display_value(result_payload(result), plan.out_dir)
    path.write_text(json.dumps(record, indent=2, ensure_ascii=False), encoding="utf-8")
    return record


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
        results.append(write_report(plan, result))
    failed = [result["name"] for result in results if not result["accepted"]]
    bundle = release_display_value({
        "surface_kind": "surface_mir_release_check_report",
        "scope": RELEASE_CHECK_SCOPE,
        "out_dir": str(plan.out_dir),
        "reports_dir": str(plan.reports_dir),
        "bundle_path": str(plan.bundle_path),
        "html_path": str(plan.html_path),
        "surface_mir_release_check_ready": not failed,
        "failed_commands": failed,
        "results": results,
        "non_claims": [
            "no final public grammar / ABI / SDK",
            "no final source patch hot-plug ABI completion",
            "no final Surface devtools viewer or telemetry ABI completion",
            "no final Surface operational runtime or transport completion",
            "no runtime MessageEnvelope dispatch completion",
            "no production identity provider or hardware attestation",
            "no distributed durable source patch migration",
            "no generated package artifact authority",
        ],
        "final_public_grammar_frozen": False,
    }, plan.out_dir)
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
