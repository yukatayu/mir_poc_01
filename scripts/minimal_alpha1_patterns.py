#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import Any, Callable

import engine_adapter_boundary_samples
import mir_computational_samples
import posegraph_samples
import projection_boundary_samples


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
PACKAGE_ID = "P-PAT-01"
PACKAGE_NAME = "minimal alpha-1 pattern guide and strict sample verifier"
KNOWN_COMMANDS = {"list", "matrix", "run", "check-all", "closeout"}

EXPECTED_COMPUTATIONAL_ACCEPTED = [
    "comp-02-pure-add-one",
    "comp-03-variables-scope-positive",
    "comp-03-arrays-bounds-positive",
    "comp-03-records-vec3-positive",
    "comp-03-control-flow-positive",
    "comp-03-imports-functions-positive",
    "comp-04-host-io-internal-transform-positive",
]
EXPECTED_COMPUTATIONAL_RUNTIME_REJECTIONS = [
    "comp-03-variables-scope-negative",
    "comp-03-arrays-bounds-negative",
    "comp-03-records-vec3-negative",
    "comp-03-control-flow-negative",
    "comp-03-imports-functions-negative",
]
EXPECTED_COMPUTATIONAL_CHECK_REJECTIONS = [
    "comp-04-host-io-internal-transform-negative-undeclared-effect",
    "comp-04-host-io-internal-transform-negative-undeclared-failure",
    "comp-04-host-io-internal-transform-negative-missing-capability",
]
EXPECTED_POSEGRAPH_PLANNED = [
    "pose-01-avatar-head-transform",
    "pose-02-anchored-object",
    "pose-03-sparkle-fallback-anchor",
    "pose-06-save-load-roundtrip",
    "pose-07-stale-anchor-after-membership-advance",
    "pose-08-anchor-switch-frontier-negative",
    "pose-09-stale-anchor-reacquire-required",
]
EXPECTED_PROJECTION_PLANNED = [
    "proj-01-server-client-target-manifest",
    "proj-01-packet-boundary-schema",
    "proj-01-ffi-boundary-schema",
    "proj-01-manifest-provider-compatibility",
]
EXPECTED_ENGINE_PROVIDERS = [
    "renderer",
    "input-device",
    "asset-loader",
    "physics-spatial-query",
    "host-runtime-bridge",
    "wasm-sandbox",
    "native-library-bridge",
    "viewer-diagnostic-exporter",
]
EXPECTED_PROJECTION_ACCEPTED_COMPAT = ["compat-accepted-renderer-view"]
EXPECTED_PROJECTION_REJECTED_COMPAT = ["compat-rejected-missing-capability"]

NON_CLAIMS = [
    "no final grammar",
    "no final public product",
    "no final public API/SDK",
    "no direct LLVM/native backend",
    "no final server/client binary split",
    "no provider admission or arbitrary native/WASM execution",
    "no full PoseGraph runtime/save-load/devtools completion",
    "no WAN/federation or distributed durable save/load",
]
VALIDATION_FLOOR = [
    "python3 -m unittest scripts.tests.test_minimal_alpha1_patterns",
    "python3 scripts/minimal_alpha1_patterns.py matrix --format json",
    "python3 scripts/minimal_alpha1_patterns.py check-all --format json",
    "python3 scripts/minimal_alpha1_patterns.py run mir-compute-host-io-transform --format json",
    "python3 scripts/minimal_alpha1_patterns.py run mir-compute-missing-effect-reject --format json",
    "python3 scripts/minimal_alpha1_patterns.py run posegraph-no-split-frame --format json",
    "python3 scripts/minimal_alpha1_patterns.py run posegraph-split-frame-violation --format json",
]


PATTERNS: list[dict[str, Any]] = [
    {
        "pattern_id": "product-alpha1-release-candidate",
        "pattern_kind": "workflow_anchor",
        "sample_root": "samples/product-alpha1/demo",
        "runnable": True,
        "default_strict_check": False,
        "command": (
            "python3 scripts/product_alpha1_release_check.py --format json "
            "check-all --out /tmp/mirrorea-alpha1-release"
        ),
        "expected_outcome": "accepted release-candidate workflow, not final product",
        "theory_anchor": (
            "versioned package.mir.json front door; typed host/devtools/native "
            "bundle boundary; no final grammar/API"
        ),
    },
    {
        "pattern_id": "operational-sugoroku-workflow",
        "pattern_kind": "workflow_anchor",
        "sample_root": "samples/product-alpha1/operational/sugoroku-world",
        "runnable": True,
        "default_strict_check": False,
        "command": (
            "python3 scripts/operational_product_samples.py run-sugoroku "
            "--format json"
        ),
        "expected_outcome": (
            "bounded same-session roll/publish/witness/handoff/stale-reject evidence"
        ),
        "theory_anchor": (
            "Place is execution locus; membership/witness/failure rows stay explicit"
        ),
    },
    {
        "pattern_id": "mir-compute-add-one",
        "pattern_kind": "strict_executable_pattern",
        "sample_root": "samples/product-alpha1/computational/add-one-pure-mir",
        "runnable": True,
        "default_strict_check": True,
        "command": (
            "python3 scripts/mir_computational_samples.py run "
            "comp-02-pure-add-one --format json"
        ),
        "expected_outcome": "accepted direct ReadInt -> add_one -> WriteInt row",
        "theory_anchor": (
            "Mir-owned computation is bounded to the declared direct runtime row"
        ),
    },
    {
        "pattern_id": "mir-compute-host-io-transform",
        "pattern_kind": "strict_executable_pattern",
        "sample_root": (
            "samples/product-alpha1/computational/host-io-internal-transform/positive"
        ),
        "runnable": True,
        "default_strict_check": True,
        "command": (
            "python3 scripts/mir_computational_samples.py run "
            "comp-04-host-io-internal-transform-positive --format json"
        ),
        "expected_outcome": "accepted typed host read/write boundary with Mir transform",
        "theory_anchor": (
            "stdio is not a Mir core primitive; host I/O crosses a typed adapter boundary"
        ),
    },
    {
        "pattern_id": "mir-compute-missing-effect-reject",
        "pattern_kind": "strict_negative_pattern",
        "sample_root": (
            "samples/product-alpha1/computational/host-io-internal-transform/"
            "negative-undeclared-effect"
        ),
        "runnable": True,
        "default_strict_check": True,
        "command": (
            "python3 scripts/mir_computational_samples.py run "
            "comp-04-host-io-internal-transform-negative-undeclared-effect "
            "--format json"
        ),
        "expected_outcome": "check_rejection with SchemaDecode / undeclared effect",
        "theory_anchor": "effects, failures, and capabilities must be declared explicitly",
    },
    {
        "pattern_id": "posegraph-no-split-frame",
        "pattern_kind": "strict_executable_pattern",
        "sample_root": "samples/product-alpha1/posegraph/no-split-frame-positive",
        "runnable": True,
        "default_strict_check": True,
        "command": (
            "python3 scripts/posegraph_samples.py run "
            "pose-04-no-split-frame-positive --format json"
        ),
        "expected_outcome": "accepted same snapshot / same pose version row",
        "theory_anchor": "same observation snapshot prevents split-frame pose reads",
    },
    {
        "pattern_id": "posegraph-split-frame-violation",
        "pattern_kind": "strict_negative_pattern",
        "sample_root": "samples/product-alpha1/posegraph/split-frame-negative",
        "runnable": True,
        "default_strict_check": True,
        "command": (
            "python3 scripts/posegraph_samples.py run "
            "pose-05-split-frame-negative --format json"
        ),
        "expected_outcome": "violation_export for snapshot / pose-version mismatch",
        "theory_anchor": "violations export evidence rather than silently repairing state",
    },
    {
        "pattern_id": "projection-inventory-boundary",
        "pattern_kind": "strict_inventory_pattern",
        "sample_root": "samples/product-alpha1/projection",
        "runnable": True,
        "default_strict_check": True,
        "command": (
            "python3 scripts/projection_boundary_samples.py run "
            "proj-01-server-client-target-manifest --format json"
        ),
        "expected_outcome": "planned_only rejection; compatibility rows remain inventory",
        "theory_anchor": (
            "server/client projection intent is documented without claiming codegen"
        ),
    },
    {
        "pattern_id": "engine-adapter-wasm-inventory",
        "pattern_kind": "strict_inventory_pattern",
        "sample_root": "samples/product-alpha1/engine-adapter",
        "runnable": True,
        "default_strict_check": True,
        "command": (
            "python3 scripts/engine_adapter_boundary_samples.py run "
            "wasm-sandbox --format json"
        ),
        "expected_outcome": "planned_only rejection; WASM remains InventoryOnly",
        "theory_anchor": (
            "engine/FFI providers do not own world semantics and are not admitted yet"
        ),
    },
]


def _require(condition: bool, detail: str) -> None:
    if not condition:
        raise RuntimeError(detail)


def _require_equal(actual: Any, expected: Any, detail: str) -> None:
    if actual != expected:
        raise RuntimeError(f"{detail}: expected {expected!r}, actual {actual!r}")


def _run_json_command(argv: list[str]) -> dict[str, Any]:
    completed = subprocess.run(
        argv,
        cwd=REPO_ROOT,
        check=False,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    if completed.returncode != 0:
        raise RuntimeError(
            f"command failed ({completed.returncode}): {' '.join(argv)}\n"
            f"{completed.stderr.strip()}"
        )
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as error:
        raise RuntimeError(
            f"command did not return JSON: {' '.join(argv)}"
        ) from error


def _computational_check_all() -> dict[str, Any]:
    return mir_computational_samples.check_all()


def _posegraph_check_all() -> dict[str, Any]:
    return posegraph_samples.check_all()


def _projection_check_all() -> dict[str, Any]:
    return projection_boundary_samples.check_all()


def _engine_check_all() -> dict[str, Any]:
    return engine_adapter_boundary_samples.check_all()


def _validate_computational(payload: dict[str, Any]) -> dict[str, Any]:
    _require_equal(payload.get("failed"), [], "computational failed rows drifted")
    _require_equal(payload.get("sample_count"), 15, "computational sample count drifted")
    _require_equal(payload.get("planned"), [], "computational planned rows drifted")
    _require_equal(
        payload.get("accepted"),
        EXPECTED_COMPUTATIONAL_ACCEPTED,
        "computational accepted rows drifted",
    )
    _require_equal(
        payload.get("expected_runtime_rejections"),
        EXPECTED_COMPUTATIONAL_RUNTIME_REJECTIONS,
        "computational runtime rejection rows drifted",
    )
    _require_equal(
        payload.get("expected_check_rejections"),
        EXPECTED_COMPUTATIONAL_CHECK_REJECTIONS,
        "computational check rejection rows drifted",
    )
    _require(
        payload.get("workflow_ready") is False,
        "computational helper must not claim workflow-ready product status",
    )
    return {
        "sample_root": payload.get("sample_root"),
        "sample_count": payload.get("sample_count"),
        "accepted": list(payload["accepted"]),
        "expected_runtime_rejections": list(payload["expected_runtime_rejections"]),
        "expected_check_rejections": list(payload["expected_check_rejections"]),
        "workflow_ready": payload.get("workflow_ready"),
    }


def _validate_posegraph(payload: dict[str, Any]) -> dict[str, Any]:
    _require_equal(payload.get("failed"), [], "posegraph failed rows drifted")
    _require_equal(payload.get("sample_count"), 9, "posegraph sample count drifted")
    _require_equal(payload.get("planned"), EXPECTED_POSEGRAPH_PLANNED, "posegraph planned rows drifted")
    _require_equal(
        payload.get("accepted"),
        ["pose-04-no-split-frame-positive"],
        "posegraph accepted rows drifted",
    )
    _require_equal(
        payload.get("violations"),
        ["pose-05-split-frame-negative"],
        "posegraph violation rows drifted",
    )
    _require(
        payload.get("workflow_ready") is False,
        "posegraph helper must not claim workflow-ready product status",
    )
    return {
        "sample_root": payload.get("sample_root"),
        "sample_count": payload.get("sample_count"),
        "planned": list(payload["planned"]),
        "accepted": list(payload["accepted"]),
        "violations": list(payload["violations"]),
        "workflow_ready": payload.get("workflow_ready"),
    }


def _validate_projection(payload: dict[str, Any]) -> dict[str, Any]:
    _require_equal(payload.get("failed"), [], "projection failed rows drifted")
    _require_equal(payload.get("sample_count"), 4, "projection sample count drifted")
    _require_equal(payload.get("planned"), EXPECTED_PROJECTION_PLANNED, "projection planned rows drifted")
    _require_equal(
        payload.get("accepted_rows"),
        EXPECTED_PROJECTION_ACCEPTED_COMPAT,
        "projection accepted compatibility rows drifted",
    )
    _require_equal(
        payload.get("rejected_rows"),
        EXPECTED_PROJECTION_REJECTED_COMPAT,
        "projection rejected compatibility rows drifted",
    )
    _require(
        payload.get("workflow_ready") is False,
        "projection helper must stay inventory-only",
    )
    return {
        "sample_root": payload.get("sample_root"),
        "sample_count": payload.get("sample_count"),
        "planned": list(payload["planned"]),
        "accepted_rows": list(payload["accepted_rows"]),
        "rejected_rows": list(payload["rejected_rows"]),
        "workflow_ready": payload.get("workflow_ready"),
    }


def _validate_engine_adapter(payload: dict[str, Any]) -> dict[str, Any]:
    _require_equal(payload.get("failed"), [], "engine-adapter failed rows drifted")
    _require_equal(payload.get("provider_count"), 8, "engine-adapter provider count drifted")
    _require_equal(payload.get("planned"), EXPECTED_ENGINE_PROVIDERS, "engine-adapter provider rows drifted")
    _require_equal(
        payload.get("world_semantics_owner"),
        "mir_mirrorea",
        "engine-adapter semantic owner drifted",
    )
    _require_equal(
        payload.get("default_native_execution_policy"),
        "Disabled",
        "engine-adapter native execution policy drifted",
    )
    _require_equal(
        payload.get("default_wasm_execution_policy"),
        "InventoryOnly",
        "engine-adapter WASM execution policy drifted",
    )
    _require(
        payload.get("workflow_ready") is False,
        "engine-adapter helper must stay inventory-only",
    )
    return {
        "provider_root": payload.get("provider_root"),
        "provider_count": payload.get("provider_count"),
        "planned": list(payload["planned"]),
        "world_semantics_owner": payload.get("world_semantics_owner"),
        "default_native_execution_policy": payload.get("default_native_execution_policy"),
        "default_wasm_execution_policy": payload.get("default_wasm_execution_policy"),
        "workflow_ready": payload.get("workflow_ready"),
    }


def _pattern_by_id(pattern_id: str) -> dict[str, Any]:
    pattern = next((row for row in PATTERNS if row["pattern_id"] == pattern_id), None)
    if pattern is None:
        raise ValueError(f"unknown minimal alpha-1 pattern `{pattern_id}`")
    return dict(pattern)


def _run_product_release_pattern() -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix="mirrorea-minimal-pattern-release-") as out_dir:
        return _run_json_command(
            [
                sys.executable,
                str(SCRIPT_DIR / "product_alpha1_release_check.py"),
                "--format",
                "json",
                "check-all",
                "--out",
                out_dir,
            ]
        )


def _run_operational_sugoroku_pattern() -> dict[str, Any]:
    return _run_json_command(
        [
            sys.executable,
            str(SCRIPT_DIR / "operational_product_samples.py"),
            "--format",
            "json",
            "run-sugoroku",
        ]
    )


def list_patterns() -> list[dict[str, Any]]:
    return [dict(row) for row in PATTERNS]


def matrix() -> dict[str, Any]:
    strict_patterns = [row["pattern_id"] for row in PATTERNS if row["default_strict_check"]]
    workflow_anchors = [
        row["pattern_id"] for row in PATTERNS if row["pattern_kind"] == "workflow_anchor"
    ]
    return {
        "command": "matrix",
        "package_id": PACKAGE_ID,
        "package_name": PACKAGE_NAME,
        "pattern_count": len(PATTERNS),
        "workflow_anchor_count": len(workflow_anchors),
        "default_strict_pattern_count": len(strict_patterns),
        "default_strict_family_count": 4,
        "workflow_anchors": workflow_anchors,
        "default_strict_patterns": strict_patterns,
        "sample_roots": sorted({row["sample_root"] for row in PATTERNS}),
        "final_public_product_claimed": False,
        "non_claims": list(NON_CLAIMS),
        "validation_floor": list(VALIDATION_FLOOR),
        "patterns": list_patterns(),
    }


def _run_and_validate_family(
    family_id: str,
    runner: Callable[[], dict[str, Any]],
    validator: Callable[[dict[str, Any]], dict[str, Any]],
) -> tuple[dict[str, Any] | None, dict[str, str] | None]:
    try:
        return validator(runner()), None
    except Exception as error:
        return None, {"family": family_id, "detail": str(error)}


def _workflow_check_all(out_dir: Path, skip_docker: bool) -> dict[str, Any]:
    product_out = out_dir / "product-release"
    product_cmd = [
        sys.executable,
        str(SCRIPT_DIR / "product_alpha1_release_check.py"),
        "--format",
        "json",
        "check-all",
        "--out",
        str(product_out),
    ]
    if skip_docker:
        product_cmd.append("--skip-docker")
    operational_cmd = [
        sys.executable,
        str(SCRIPT_DIR / "operational_product_samples.py"),
        "--format",
        "json",
        "check-all",
    ]
    product = _run_json_command(product_cmd)
    operational = _run_json_command(operational_cmd)
    _require_equal(product.get("status"), "accepted", "product workflow status drifted")
    _require_equal(
        operational.get("status"),
        "accepted",
        "operational workflow status drifted",
    )
    return {
        "product_alpha1_release_check": product,
        "operational_product_samples": operational,
        "skip_docker": skip_docker,
    }


def check_all(
    *, include_workflows: bool = False, out_dir: str | None = None, skip_docker: bool = False
) -> dict[str, Any]:
    strict_families: dict[str, Any] = {}
    failures: list[dict[str, str]] = []

    checks = [
        ("computational", _computational_check_all, _validate_computational),
        ("posegraph", _posegraph_check_all, _validate_posegraph),
        ("projection", _projection_check_all, _validate_projection),
        ("engine_adapter", _engine_check_all, _validate_engine_adapter),
    ]
    for family_id, runner, validator in checks:
        result, failure = _run_and_validate_family(family_id, runner, validator)
        if failure is not None:
            failures.append(failure)
            continue
        strict_families[family_id] = result

    workflow_results: dict[str, Any] | None = None
    if include_workflows:
        try:
            if out_dir is None:
                with tempfile.TemporaryDirectory(prefix="mirrorea-minimal-pattern-workflows-") as tmp:
                    workflow_results = _workflow_check_all(Path(tmp), skip_docker)
            else:
                workflow_path = Path(out_dir)
                workflow_path.mkdir(parents=True, exist_ok=True)
                workflow_results = _workflow_check_all(workflow_path, skip_docker)
        except Exception as error:
            failures.append({"family": "workflow_anchors", "detail": str(error)})

    failed = [failure["family"] for failure in failures]
    return {
        "command": "check-all",
        "package_id": PACKAGE_ID,
        "package_name": PACKAGE_NAME,
        "status": "accepted" if not failures else "rejected",
        "strict_family_count": len(strict_families),
        "strict_families": strict_families,
        "workflow_anchors_checked": include_workflows,
        "workflow_results": workflow_results,
        "failed": failed,
        "failures": failures,
        "final_public_product_claimed": False,
        "non_claims": list(NON_CLAIMS),
    }


def _run_pattern_payload(pattern_id: str) -> dict[str, Any]:
    if pattern_id == "product-alpha1-release-candidate":
        return _run_product_release_pattern()
    if pattern_id == "operational-sugoroku-workflow":
        return _run_operational_sugoroku_pattern()
    if pattern_id == "mir-compute-add-one":
        return mir_computational_samples.run_sample("comp-02-pure-add-one")
    if pattern_id == "mir-compute-host-io-transform":
        return mir_computational_samples.run_sample(
            "comp-04-host-io-internal-transform-positive"
        )
    if pattern_id == "mir-compute-missing-effect-reject":
        return mir_computational_samples.run_sample(
            "comp-04-host-io-internal-transform-negative-undeclared-effect"
        )
    if pattern_id == "posegraph-no-split-frame":
        return posegraph_samples.run_sample("pose-04-no-split-frame-positive")
    if pattern_id == "posegraph-split-frame-violation":
        return posegraph_samples.run_sample("pose-05-split-frame-negative")
    if pattern_id == "projection-inventory-boundary":
        return projection_boundary_samples.run_sample(
            "proj-01-server-client-target-manifest"
        )
    if pattern_id == "engine-adapter-wasm-inventory":
        return engine_adapter_boundary_samples.run_provider("wasm-sandbox")
    raise ValueError(f"unknown minimal alpha-1 pattern `{pattern_id}`")


def _validate_run_pattern(pattern_id: str, payload: dict[str, Any]) -> None:
    if pattern_id == "product-alpha1-release-candidate":
        _require_equal(payload.get("status"), "accepted", "product release pattern status drifted")
        return
    if pattern_id == "operational-sugoroku-workflow":
        _require(
            payload.get("status") in {None, "accepted"} or payload.get("terminal_outcome") == "accepted",
            "operational Sugoroku pattern did not report accepted status",
        )
        return
    expected_terminal = {
        "mir-compute-add-one": "accepted",
        "mir-compute-host-io-transform": "accepted",
        "mir-compute-missing-effect-reject": "check_rejection",
        "posegraph-no-split-frame": "accepted",
        "posegraph-split-frame-violation": "violation_export",
        "projection-inventory-boundary": "planned_only",
        "engine-adapter-wasm-inventory": "planned_only",
    }[pattern_id]
    _require_equal(
        payload.get("terminal_outcome"),
        expected_terminal,
        f"{pattern_id} terminal outcome drifted",
    )
    if "outcome_matches_expected" in payload:
        _require(
            payload.get("outcome_matches_expected") is True,
            f"{pattern_id} no longer matches expected outcome",
        )


def run_pattern(pattern_id: str) -> dict[str, Any]:
    pattern = _pattern_by_id(pattern_id)
    payload = _run_pattern_payload(pattern_id)
    _validate_run_pattern(pattern_id, payload)
    result = dict(payload)
    result["command"] = "run"
    result["package_id"] = PACKAGE_ID
    result["pattern_id"] = pattern_id
    result["pattern_kind"] = pattern["pattern_kind"]
    result["theory_anchor"] = pattern["theory_anchor"]
    result["final_public_product_claimed"] = False
    return result


def closeout() -> dict[str, Any]:
    status = matrix()
    return {
        "command": "closeout",
        "package_id": PACKAGE_ID,
        "package_name": PACKAGE_NAME,
        "pattern_ids": [row["pattern_id"] for row in PATTERNS],
        "strict_pattern_ids": list(status["default_strict_patterns"]),
        "workflow_anchor_ids": list(status["workflow_anchors"]),
        "validation_floor": list(VALIDATION_FLOOR),
        "non_claims": list(NON_CLAIMS),
        "final_public_product_claimed": False,
        "theory_summary": [
            "Mir core has no stdio builtin; host I/O stays typed adapter boundary.",
            "Place is execution locus, not participant identity.",
            "Effect, failure, capability, witness, and redaction rows remain explicit.",
            "Projection/backend/engine rows are inventory until separately actualized.",
        ],
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        lines = ["MINIMAL ALPHA-1 PATTERNS"]
        for row in payload:
            lines.append(
                f"- {row['pattern_id']} [{row['pattern_kind']}] -> {row['command']}"
            )
        return "\n".join(lines)

    command = payload.get("command")
    if command == "matrix":
        return "\n".join(
            [
                "MINIMAL ALPHA-1 PATTERN MATRIX",
                f"package: {payload['package_id']}",
                f"patterns: {payload['pattern_count']}",
                f"workflow anchors: {payload['workflow_anchor_count']}",
                f"default strict patterns: {payload['default_strict_pattern_count']}",
                f"default strict families: {payload['default_strict_family_count']}",
            ]
        )
    if command == "check-all":
        return "\n".join(
            [
                "MINIMAL ALPHA-1 PATTERN CHECK",
                f"status: {payload['status']}",
                f"strict families: {payload['strict_family_count']}",
                f"workflow anchors checked: {payload['workflow_anchors_checked']}",
                f"failed families: {', '.join(payload['failed']) or '(none)'}",
            ]
        )
    if command == "run":
        lines = [
            "MINIMAL ALPHA-1 PATTERN RUN",
            f"pattern: {payload['pattern_id']}",
            f"kind: {payload['pattern_kind']}",
        ]
        if "terminal_outcome" in payload:
            lines.append(f"outcome: {payload['terminal_outcome']}")
        elif "status" in payload:
            lines.append(f"status: {payload['status']}")
        return "\n".join(lines)
    if command == "closeout":
        return "\n".join(
            [
                "MINIMAL ALPHA-1 PATTERN CLOSEOUT",
                f"package: {payload['package_id']}",
                "strict patterns: " + ", ".join(payload["strict_pattern_ids"]),
                "workflow anchors: " + ", ".join(payload["workflow_anchor_ids"]),
                "non-claims: " + "; ".join(payload["non_claims"]),
            ]
        )
    return json.dumps(payload, indent=2, ensure_ascii=False)


def _print(payload: Any, fmt: str) -> None:
    if fmt == "json":
        print(json.dumps(payload, indent=2, ensure_ascii=False))
    else:
        print(format_pretty(payload))


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()
    parser.add_argument("--format", choices=["pretty", "json"], default="pretty")
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("list")
    subparsers.add_parser("matrix")
    subparsers.add_parser("closeout")
    check_parser = subparsers.add_parser("check-all")
    check_parser.add_argument("--include-workflows", action="store_true")
    check_parser.add_argument("--out")
    check_parser.add_argument("--skip-docker", action="store_true")
    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("pattern_id")
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
    if remainder and remainder[0] not in KNOWN_COMMANDS and not remainder[0].startswith("-"):
        return [*hoisted_root_options, "run", *remainder]
    return [*hoisted_root_options, *remainder]


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(normalize_argv(argv))

    if args.command == "list":
        payload = list_patterns()
    elif args.command == "matrix":
        payload = matrix()
    elif args.command == "check-all":
        payload = check_all(
            include_workflows=args.include_workflows,
            out_dir=args.out,
            skip_docker=args.skip_docker,
        )
    elif args.command == "closeout":
        payload = closeout()
    else:
        payload = run_pattern(args.pattern_id)

    _print(payload, args.format)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
