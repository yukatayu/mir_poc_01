#!/usr/bin/env python3

from __future__ import annotations

import argparse
import html
import json
import sys
from functools import lru_cache
from pathlib import Path
from typing import Any


SCRIPT_PATH = Path(__file__).resolve()
SCRIPT_DIR = SCRIPT_PATH.parent
REPO_ROOT = SCRIPT_DIR.parent
sys.path.insert(0, str(SCRIPT_DIR))

import practical_alpha1_attach  # noqa: E402
import practical_alpha1_avatar  # noqa: E402
import practical_alpha1_check  # noqa: E402
import practical_alpha1_export_devtools  # noqa: E402
import practical_alpha1_run_local  # noqa: E402
import practical_alpha1_save_load  # noqa: E402
import practical_alpha1_transport  # noqa: E402


PREVIEW_SCOPE = "practical-alpha1-product-preview-floor"
SURFACE_KIND = "practical_alpha1_nonfinal_product_preview_bundle"
PREVIEW_MODE = "thin_exact_report_bundle"
PREVIEW_BOUNDARY = (
    "non-final practical product-preview bundle over preview manifests, exact practical "
    "reports, and source package paths; not a final product runtime or public CLI"
)

STOP_LINES = [
    "do not treat this product-preview bundle as a final practical product runtime",
    "do not treat this product-preview bundle as active canonical runnable-root promotion",
    "do not treat avatar companion preview evidence as native execution, same-session runtime completion, or unsupported-runtime execution success",
    "do not treat this product-preview bundle as a final public CLI, viewer, transport, save-load, or package/avatar API",
]

LIMITATIONS = [
    "non-final practical product-preview floor only",
    "thin bundle over preview manifests, exact practical reports, exact practical devtools bundles, and exact avatar preview reports only",
    "no native avatar execution, same-session runtime attach/detach execution, or monolithic same-session product runtime",
]

COMMON_NON_CLAIMS = [
    "final public product runtime",
    "active canonical runnable root promotion",
    "final public CLI or viewer API",
]

DEFERRED_AVATAR_SEMANTICS: list[str] = []

IMPLEMENTED_ROWS: list[dict[str, str]] = [
    {
        "sample_id": "PE2E-01",
        "summary": "preview one local full-toolchain world bundle from a preview manifest plus exact reports",
        "preview_dir": "samples/practical-alpha1/previews/pe2e-a1-01-local-full-toolchain-preview",
        "expected_report": "samples/practical-alpha1/expected/pe2e-a1-01-local-full-toolchain-preview.expected.json",
        "bundle_kind": "local_full_toolchain_preview",
    },
    {
        "sample_id": "PE2E-02",
        "summary": "preview one Docker full-toolchain world bundle from a preview manifest plus exact reports",
        "preview_dir": "samples/practical-alpha1/previews/pe2e-a1-02-docker-full-toolchain-preview",
        "expected_report": "samples/practical-alpha1/expected/pe2e-a1-02-docker-full-toolchain-preview.expected.json",
        "bundle_kind": "docker_full_toolchain_preview",
    },
    {
        "sample_id": "PE2E-03",
        "summary": "preview a companion debug-layer attach bundle without claiming same-session hot-plug execution",
        "preview_dir": "samples/practical-alpha1/previews/pe2e-a1-03-hotplug-debug-layer-preview",
        "expected_report": "samples/practical-alpha1/expected/pe2e-a1-03-hotplug-debug-layer-preview.expected.json",
        "bundle_kind": "debug_layer_companion_preview",
    },
    {
        "sample_id": "PE2E-04",
        "summary": "preview a companion placeholder object package without claiming custom avatar runtime or unsupported fallback completion",
        "preview_dir": "samples/practical-alpha1/previews/pe2e-a1-04-placeholder-object-preview",
        "expected_report": "samples/practical-alpha1/expected/pe2e-a1-04-placeholder-object-preview.expected.json",
        "bundle_kind": "placeholder_object_companion_preview",
    },
    {
        "sample_id": "PE2E-05",
        "summary": "preview local save/load continue through the distinct saved-frontier report",
        "preview_dir": "samples/practical-alpha1/previews/pe2e-a1-05-local-save-load-continue",
        "expected_report": "samples/practical-alpha1/expected/pe2e-a1-05-local-save-load-continue.expected.json",
        "bundle_kind": "local_save_load_continue_preview",
    },
    {
        "sample_id": "PE2E-06",
        "summary": "preview invalid distributed save rejection through the exact save-load preflight reject report",
        "preview_dir": "samples/practical-alpha1/previews/pe2e-a1-06-invalid-distributed-save-rejected",
        "expected_report": "samples/practical-alpha1/expected/pe2e-a1-06-invalid-distributed-save-rejected.expected.json",
        "bundle_kind": "invalid_distributed_save_rejected_preview",
    },
    {
        "sample_id": "PE2E-07",
        "summary": "preview devtools export and viewer-openability over exact practical bundles",
        "preview_dir": "samples/practical-alpha1/previews/pe2e-a1-07-devtools-viewer-preview",
        "expected_report": "samples/practical-alpha1/expected/pe2e-a1-07-devtools-viewer-preview.expected.json",
        "bundle_kind": "devtools_viewer_preview",
    },
    {
        "sample_id": "PE2E-08",
        "summary": "preview a companion custom-avatar bundle over an exact avatar preview report without claiming same-session runtime execution",
        "preview_dir": "samples/practical-alpha1/previews/pe2e-a1-08-custom-avatar-companion-preview",
        "expected_report": "samples/practical-alpha1/expected/pe2e-a1-08-custom-avatar-companion-preview.expected.json",
        "bundle_kind": "custom_avatar_companion_preview",
    },
    {
        "sample_id": "PE2E-09",
        "summary": "preview a companion unsupported-runtime visible fallback bundle over an exact avatar preview report without claiming execution success",
        "preview_dir": "samples/practical-alpha1/previews/pe2e-a1-09-unsupported-runtime-visible-fallback-preview",
        "expected_report": "samples/practical-alpha1/expected/pe2e-a1-09-unsupported-runtime-visible-fallback-preview.expected.json",
        "bundle_kind": "unsupported_runtime_visible_fallback_preview",
    },
]


def _implemented_row(sample_id: str) -> dict[str, str]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown practical alpha-1 product-preview sample {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "practical-alpha1-product-preview",
            "source_root": "samples/practical-alpha1/previews",
            "summary": row["summary"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def _preview_file(preview_path: str | Path) -> Path:
    path = Path(preview_path)
    if path.is_dir():
        return path / "preview.json"
    return path


def _load_preview(preview_path: str | Path) -> tuple[dict[str, Any], Path]:
    preview_file = _preview_file(preview_path)
    payload = json.loads(preview_file.read_text())
    return payload, preview_file


def _package_file(package_path: str | Path) -> Path:
    path = Path(package_path)
    if path.is_dir():
        return path / "package.mir.json"
    return path


def _load_package(package_path: str | Path) -> dict[str, Any]:
    return json.loads(_package_file(package_path).read_text())


def _resolve_preview_package(preview_file: Path, relative_path: str) -> Path:
    return (preview_file.parent / relative_path).resolve()


def _package_ref(package_path: Path, role: str) -> dict[str, str]:
    package_file = _package_file(package_path)
    package = _load_package(package_file)
    return {
        "role": role,
        "package_id": package["package_id"],
        "package_kind": package["package_kind"],
        "front_door_entry": package_file.relative_to(REPO_ROOT).as_posix(),
        "front_door_mode": "package.mir.json directory_or_file_loader",
    }


def _source_report_ref(
    *,
    family: str,
    sample_id: str,
    carrier_scope: str,
    surface_kind: str,
) -> dict[str, str]:
    return {
        "family": family,
        "sample_id": sample_id,
        "carrier_scope": carrier_scope,
        "surface_kind": surface_kind,
    }


def _load_expected_report(row: dict[str, str]) -> dict[str, Any]:
    return json.loads((REPO_ROOT / row["expected_report"]).read_text())


def _expect_id(preview: dict[str, Any], expected_id: str) -> None:
    actual = preview.get("preview_id")
    if actual != expected_id:
        raise RuntimeError(f"preview_id mismatch: expected {expected_id}, got {actual!r}")


def _expect_kind(preview: dict[str, Any], expected_kind: str) -> None:
    actual = preview.get("preview_kind")
    if actual != expected_kind:
        raise RuntimeError(
            f"preview_kind mismatch for {preview.get('preview_id')}: expected {expected_kind}, got {actual!r}"
        )


def _required_source_reports(preview: dict[str, Any], expected_ids: list[str]) -> None:
    refs = list(preview.get("required_source_reports", []))
    actual = [ref.get("sample_id") for ref in refs]
    if actual != expected_ids:
        raise RuntimeError(
            f"required_source_reports mismatch for {preview.get('preview_id')}: expected {expected_ids}, got {actual}"
        )


def _required_viewer_bundles(preview: dict[str, Any], expected_ids: list[str]) -> None:
    actual = list(preview.get("viewer_bundle_ids", []))
    if actual != expected_ids:
        raise RuntimeError(
            f"viewer_bundle_ids mismatch for {preview.get('preview_id')}: expected {expected_ids}, got {actual}"
        )


@lru_cache(maxsize=None)
def _checker_report(sample_id: str) -> dict[str, Any]:
    return practical_alpha1_check.run_sample(sample_id)


@lru_cache(maxsize=None)
def _run_local_report(sample_id: str) -> dict[str, Any]:
    return practical_alpha1_run_local.run_sample(sample_id)


@lru_cache(maxsize=None)
def _hotplug_report(sample_id: str) -> dict[str, Any]:
    return practical_alpha1_attach.run_sample(sample_id)


@lru_cache(maxsize=None)
def _transport_report(sample_id: str) -> dict[str, Any]:
    return practical_alpha1_transport.run_sample(sample_id)


@lru_cache(maxsize=None)
def _avatar_report(sample_id: str) -> dict[str, Any]:
    return practical_alpha1_avatar.run_sample(sample_id)


@lru_cache(maxsize=None)
def _save_load_report(sample_id: str) -> dict[str, Any]:
    return practical_alpha1_save_load.run_sample(sample_id)


@lru_cache(maxsize=None)
def _devtools_bundle(sample_id: str) -> dict[str, Any]:
    return practical_alpha1_export_devtools.run_sample(sample_id)


def _base_bundle(
    *,
    sample_id: str,
    bundle_kind: str,
    source_packages: list[dict[str, str]],
    source_reports: list[dict[str, str]],
) -> dict[str, Any]:
    return {
        "sample_id": sample_id,
        "bundle_kind": bundle_kind,
        "family": "practical-alpha1-product-preview",
        "preview_scope": PREVIEW_SCOPE,
        "surface_kind": SURFACE_KIND,
        "preview_mode": PREVIEW_MODE,
        "preview_boundary": PREVIEW_BOUNDARY,
        "source_packages": source_packages,
        "source_reports": source_reports,
    }


def _local_full_toolchain_bundle(preview: dict[str, Any], preview_file: Path) -> dict[str, Any]:
    _expect_id(preview, "PE2E-01")
    _expect_kind(preview, "local_full_toolchain_preview")
    _required_source_reports(preview, ["RUN-01", "VIS-A1-01"])
    world_package = _resolve_preview_package(preview_file, preview["world_package"])
    runtime_report = _run_local_report("RUN-01")
    devtools_bundle = _devtools_bundle("VIS-A1-01")
    payload = _base_bundle(
        sample_id="PE2E-01",
        bundle_kind="local_full_toolchain_preview",
        source_packages=[_package_ref(world_package, "world_package")],
        source_reports=[
            _source_report_ref(
                family="practical-alpha1-local-runtime",
                sample_id="RUN-01",
                carrier_scope=runtime_report["runtime_scope"],
                surface_kind=runtime_report["surface_kind"],
            ),
            _source_report_ref(
                family="practical-alpha1-devtools-export",
                sample_id="VIS-A1-01",
                carrier_scope=devtools_bundle["devtools_scope"],
                surface_kind=devtools_bundle["surface_kind"],
            ),
        ],
    )
    payload["preview_sections"] = {
        "local_runtime": {
            "package_id": runtime_report["package_id"],
            "world_id": runtime_report["world_id"],
            "entry_place": runtime_report["entry_place"],
            "runtime_scope": runtime_report["runtime_scope"],
            "runtime_plan_scope": runtime_report["runtime_plan_scope"],
            "terminal_outcome": runtime_report["terminal_outcome"],
            "current_owner": runtime_report["current_owner"],
            "requested_layers": [layer["id"] for layer in runtime_report["requested_layers"]],
        },
        "event_dag_export": {
            "bundle_ref": "VIS-A1-01",
            "panel_ids": devtools_bundle["panel_ids"],
            "telemetry_kinds": devtools_bundle["telemetry_kinds"],
        },
    }
    payload["what_it_proves"] = [
        "one practical world package is consumable from the package.mir.json front door through the local runtime floor",
        "the same preview keeps the runtime-plan boundary explicit and reuses the exact event-DAG export bundle",
        "product-preview composition stays a thin bundle over exact practical reports rather than a new runtime semantic lane",
    ]
    payload["what_it_does_not_prove"] = list(COMMON_NON_CLAIMS) + [
        "Docker transport completion",
        "package hot-plug execution completion",
        "local save/load completion",
        "custom Mir avatar runtime",
    ]
    return payload


def _docker_full_toolchain_bundle(preview: dict[str, Any], preview_file: Path) -> dict[str, Any]:
    _expect_id(preview, "PE2E-02")
    _expect_kind(preview, "docker_full_toolchain_preview")
    _required_source_reports(preview, ["TR-A1-02"])
    world_package = _resolve_preview_package(preview_file, preview["world_package"])
    transport_report = _transport_report("TR-A1-02")
    payload = _base_bundle(
        sample_id="PE2E-02",
        bundle_kind="docker_full_toolchain_preview",
        source_packages=[_package_ref(world_package, "world_package")],
        source_reports=[
            _source_report_ref(
                family="practical-alpha1-transport",
                sample_id="TR-A1-02",
                carrier_scope=transport_report["transport_scope"],
                surface_kind=transport_report["surface_kind"],
            )
        ],
    )
    payload["preview_sections"] = {
        "docker_runtime": {
            "package_id": transport_report["package_id"],
            "world_id": transport_report["world_id"],
            "transport_surface": transport_report["transport_surface"],
            "transport_medium": transport_report["transport_medium"],
            "bridge_kind": transport_report["bridge_kind"],
            "terminal_outcome": transport_report["terminal_outcome"],
            "bridge_process_ids": [
                row["process_id"] for row in transport_report["bridge_processes"]
            ],
            "observer_route_hop_count": len(transport_report["observer_route_trace"]),
        }
    }
    payload["what_it_proves"] = [
        "one practical world package is consumable from the package.mir.json front door through the Docker Compose TCP floor",
        "transport, membership, capability, and witness lanes remain explicit in the exact transport report",
        "the preview bundle reuses the existing Docker practical report without introducing a new transport semantic lane",
    ]
    payload["what_it_does_not_prove"] = list(COMMON_NON_CLAIMS) + [
        "WAN or federation completion",
        "local save/load completion",
        "package hot-plug execution completion",
        "custom Mir avatar runtime",
    ]
    return payload


def _debug_layer_companion_bundle(preview: dict[str, Any], preview_file: Path) -> dict[str, Any]:
    _expect_id(preview, "PE2E-03")
    _expect_kind(preview, "debug_layer_companion_preview")
    _required_source_reports(preview, ["RUN-01", "HP-A1-01", "VIS-A1-04"])
    world_package = _resolve_preview_package(preview_file, preview["world_package"])
    layer_package = _resolve_preview_package(preview_file, preview["layer_package"])
    runtime_report = _run_local_report("RUN-01")
    attach_report = _hotplug_report("HP-A1-01")
    lifecycle_bundle = _devtools_bundle("VIS-A1-04")
    payload = _base_bundle(
        sample_id="PE2E-03",
        bundle_kind="debug_layer_companion_preview",
        source_packages=[
            _package_ref(world_package, "world_package"),
            _package_ref(layer_package, "companion_layer_package"),
        ],
        source_reports=[
            _source_report_ref(
                family="practical-alpha1-local-runtime",
                sample_id="RUN-01",
                carrier_scope=runtime_report["runtime_scope"],
                surface_kind=runtime_report["surface_kind"],
            ),
            _source_report_ref(
                family="practical-alpha1-hotplug",
                sample_id="HP-A1-01",
                carrier_scope=attach_report["hotplug_scope"],
                surface_kind=attach_report["surface_kind"],
            ),
            _source_report_ref(
                family="practical-alpha1-devtools-export",
                sample_id="VIS-A1-04",
                carrier_scope=lifecycle_bundle["devtools_scope"],
                surface_kind=lifecycle_bundle["surface_kind"],
            ),
        ],
    )
    payload["preview_sections"] = {
        "runtime_world": {
            "package_id": runtime_report["package_id"],
            "world_id": runtime_report["world_id"],
            "requested_layers": [layer["id"] for layer in runtime_report["requested_layers"]],
        },
        "debug_layer_attach": {
            "package_id": attach_report["package_id"],
            "attach_profile": attach_report["attach_profile"],
            "activation_cut_ref": attach_report["activation_cut_ref"],
            "terminal_outcome": attach_report["terminal_outcome"],
        },
        "lifecycle_export": {
            "bundle_ref": "VIS-A1-04",
            "panel_ids": lifecycle_bundle["panel_ids"],
            "telemetry_ids": lifecycle_bundle["telemetry_ids"],
        },
    }
    payload["what_it_proves"] = [
        "one practical world package can be previewed with exact companion debug-layer attach evidence",
        "the attach boundary remains explicit as an exact hot-plug report plus a distinct lifecycle export bundle",
        "layer/product composition is kept at the thin preview-bundle level rather than collapsed into a new runtime path",
    ]
    payload["what_it_does_not_prove"] = list(COMMON_NON_CLAIMS) + [
        "same-session runtime attach execution",
        "detach runtime lifecycle execution",
        "final object package attach completion",
        "custom Mir avatar runtime",
    ]
    return payload


def _placeholder_object_companion_bundle(preview: dict[str, Any], preview_file: Path) -> dict[str, Any]:
    _expect_id(preview, "PE2E-04")
    _expect_kind(preview, "placeholder_object_companion_preview")
    _required_source_reports(preview, ["RUN-01", "HP-A1-06"])
    world_package = _resolve_preview_package(preview_file, preview["world_package"])
    object_package = _resolve_preview_package(preview_file, preview["object_package"])
    runtime_report = _run_local_report("RUN-01")
    attach_report = _hotplug_report("HP-A1-06")
    object_preview = attach_report["object_attach_preview"]
    payload = _base_bundle(
        sample_id="PE2E-04",
        bundle_kind="placeholder_object_companion_preview",
        source_packages=[
            _package_ref(world_package, "world_package"),
            _package_ref(object_package, "companion_object_package"),
        ],
        source_reports=[
            _source_report_ref(
                family="practical-alpha1-local-runtime",
                sample_id="RUN-01",
                carrier_scope=runtime_report["runtime_scope"],
                surface_kind=runtime_report["surface_kind"],
            ),
            _source_report_ref(
                family="practical-alpha1-hotplug",
                sample_id="HP-A1-06",
                carrier_scope=attach_report["hotplug_scope"],
                surface_kind=attach_report["surface_kind"],
            ),
        ],
    )
    payload["preview_sections"] = {
        "runtime_world": {
            "package_id": runtime_report["package_id"],
            "world_id": runtime_report["world_id"],
            "terminal_outcome": runtime_report["terminal_outcome"],
        },
        "object_package": {
            "package_id": attach_report["package_id"],
            "terminal_outcome": attach_report["terminal_outcome"],
            "selected_representation": object_preview["selected_representation"],
            "fallback_representation": object_preview["fallback_representation"],
            "provided_roles": object_preview["provided_roles"],
            "native_execution_performed": object_preview["native_execution_performed"],
        },
    }
    payload["what_it_proves"] = [
        "one practical world package can be previewed with exact companion placeholder object-package evidence",
        "the object path keeps fallback representation explicit in the exact hot-plug preview seam",
        "product-preview composition can include placeholder object evidence without promoting final avatar runtime semantics",
    ]
    payload["what_it_does_not_prove"] = list(COMMON_NON_CLAIMS) + [
        "same-session runtime attach execution",
        "custom Mir avatar runtime",
        "unsupported runtime fallback",
        "native avatar execution",
        "final object package attach completion",
    ]
    return payload


def _local_save_load_continue_bundle(preview: dict[str, Any], preview_file: Path) -> dict[str, Any]:
    _expect_id(preview, "PE2E-05")
    _expect_kind(preview, "local_save_load_continue_preview")
    _required_source_reports(preview, ["SL-A1-01"])
    world_package = _resolve_preview_package(preview_file, preview["world_package"])
    save_load_report = _save_load_report("SL-A1-01")
    payload = _base_bundle(
        sample_id="PE2E-05",
        bundle_kind="local_save_load_continue_preview",
        source_packages=[_package_ref(world_package, "world_package")],
        source_reports=[
            _source_report_ref(
                family="practical-alpha1-save-load",
                sample_id="SL-A1-01",
                carrier_scope=save_load_report["save_load_scope"],
                surface_kind=save_load_report["surface_kind"],
            )
        ],
    )
    payload["preview_sections"] = {
        "save_load_roundtrip": {
            "package_id": save_load_report["package_id"],
            "terminal_outcome": save_load_report["terminal_outcome"],
            "save_load_scope": save_load_report["save_load_scope"],
            "runtime_plan_scope": save_load_report["runtime_plan_scope"],
            "saved_owner": save_load_report["saved_owner"],
            "resumed_owner": save_load_report["resumed_owner"],
            "resumed_dispatch_ids": [
                row["envelope_id"] for row in save_load_report["resumed_dispatch_records"]
            ],
        }
    }
    payload["what_it_proves"] = [
        "one practical world package can be previewed through the distinct local save/load floor",
        "saved local frontier and resumed dispatch remain explicit in the exact save-load report",
        "the preview keeps local-only save/load separate from Docker transport and distributed durability claims",
    ]
    payload["what_it_does_not_prove"] = list(COMMON_NON_CLAIMS) + [
        "distributed durable save/load",
        "stale witness non-resurrection completion",
        "stale lease non-resurrection completion",
        "queue/channel/transport persistence completion",
    ]
    return payload


def _invalid_distributed_save_reject_bundle(
    preview: dict[str, Any], preview_file: Path
) -> dict[str, Any]:
    _expect_id(preview, "PE2E-06")
    _expect_kind(preview, "invalid_distributed_save_rejected_preview")
    _required_source_reports(preview, ["SL-A1-03"])
    save_load_package = _resolve_preview_package(preview_file, preview["save_load_package"])
    save_load_report = _save_load_report("SL-A1-03")
    source_checker_report = save_load_report["source_checker_report"]
    payload = _base_bundle(
        sample_id="PE2E-06",
        bundle_kind="invalid_distributed_save_rejected_preview",
        source_packages=[_package_ref(save_load_package, "save_load_package")],
        source_reports=[
            _source_report_ref(
                family="practical-alpha1-save-load",
                sample_id="SL-A1-03",
                carrier_scope=save_load_report["save_load_scope"],
                surface_kind=save_load_report["surface_kind"],
            )
        ],
    )
    payload["preview_sections"] = {
        "save_load_preflight_reject": {
            "package_id": save_load_report["package_id"],
            "terminal_outcome": save_load_report["terminal_outcome"],
            "save_load_scope": save_load_report["save_load_scope"],
            "preflight_scope": save_load_report["preflight_scope"],
            "checker_guard_refs": save_load_report["checker_guard_refs"],
            "source_checker_sample_id": source_checker_report["sample_id"],
            "source_rejected_kind": source_checker_report["rejected_kind"],
            "diagnostic_message": source_checker_report["diagnostic_message"],
            "saved_local_frontier_emitted": save_load_report[
                "saved_local_frontier_emitted"
            ],
            "distributed_save_load_claimed": save_load_report[
                "distributed_save_load_claimed"
            ],
        }
    }
    payload["what_it_proves"] = [
        "invalid distributed save/load remains an explicit rejected practical preview row",
        "the preview consumes the exact save-load preflight reject row rather than bypassing the widened save/load lane",
        "product-preview composition can include a negative save/load result without collapsing the runtime-backed saved-frontier branch",
    ]
    payload["what_it_does_not_prove"] = list(COMMON_NON_CLAIMS) + [
        "distributed durable save/load",
        "runtime distributed checkpoint execution",
        "saved local frontier build for an invalid distributed cut",
        "full consistent-cut completion",
    ]
    return payload


def _devtools_viewer_bundle(preview: dict[str, Any], preview_file: Path) -> dict[str, Any]:
    _expect_id(preview, "PE2E-07")
    _expect_kind(preview, "devtools_viewer_preview")
    bundle_ids = ["VIS-A1-01", "VIS-A1-02", "VIS-A1-04", "VIS-A1-06"]
    _required_viewer_bundles(preview, bundle_ids)
    source_packages = [
        _package_ref(
            _resolve_preview_package(preview_file, relative_path),
            role,
        )
        for role, relative_path in [
            ("world_package", preview["world_package"]),
            ("route_trace_package", preview["route_trace_package"]),
            ("attach_package", preview["attach_package"]),
            ("detach_package", preview["detach_package"]),
            ("auth_lane_package", preview["auth_lane_package"]),
        ]
    ]
    bundles = [_devtools_bundle(sample_id) for sample_id in bundle_ids]
    payload = _base_bundle(
        sample_id="PE2E-07",
        bundle_kind="devtools_viewer_preview",
        source_packages=source_packages,
        source_reports=[
            _source_report_ref(
                family="practical-alpha1-devtools-export",
                sample_id=bundle["sample_id"],
                carrier_scope=bundle["devtools_scope"],
                surface_kind=bundle["surface_kind"],
            )
            for bundle in bundles
        ],
    )
    payload["preview_sections"] = {
        "viewer_exports": [
            {
                "bundle_ref": bundle["sample_id"],
                "bundle_kind": bundle["bundle_kind"],
                "panel_ids": bundle["panel_ids"],
                "viewer_mode": bundle["viewer_mode"],
            }
            for bundle in bundles
        ],
        "render_html_available": True,
    }
    payload["what_it_proves"] = [
        "exact practical devtools bundles can be gathered under one thin product-preview manifest",
        "viewer-openability remains grounded in the same exact bundle payloads that validate the devtools floor",
        "the preview keeps devtools export separate from runtime, transport, and hot-plug carriers",
    ]
    payload["what_it_does_not_prove"] = list(COMMON_NON_CLAIMS) + [
        "full devtools stage completion",
        "membership timeline completion",
        "fallback degradation completion",
        "retention/on-demand trace completion",
    ]
    return payload


def _custom_avatar_companion_bundle(preview: dict[str, Any], preview_file: Path) -> dict[str, Any]:
    _expect_id(preview, "PE2E-08")
    _expect_kind(preview, "custom_avatar_companion_preview")
    _required_source_reports(preview, ["RUN-01", "AV-A1-02"])
    world_package = _resolve_preview_package(preview_file, preview["world_package"])
    avatar_package = _resolve_preview_package(preview_file, preview["avatar_package"])
    runtime_report = _run_local_report("RUN-01")
    avatar_report = _avatar_report("AV-A1-02")
    payload = _base_bundle(
        sample_id="PE2E-08",
        bundle_kind="custom_avatar_companion_preview",
        source_packages=[
            _package_ref(world_package, "world_package"),
            _package_ref(avatar_package, "companion_avatar_package"),
        ],
        source_reports=[
            _source_report_ref(
                family="practical-alpha1-local-runtime",
                sample_id="RUN-01",
                carrier_scope=runtime_report["runtime_scope"],
                surface_kind=runtime_report["surface_kind"],
            ),
            _source_report_ref(
                family="practical-alpha1-avatar-preview",
                sample_id="AV-A1-02",
                carrier_scope=avatar_report["avatar_scope"],
                surface_kind=avatar_report["surface_kind"],
            ),
        ],
    )
    payload["preview_sections"] = {
        "runtime_world": {
            "package_id": runtime_report["package_id"],
            "world_id": runtime_report["world_id"],
            "terminal_outcome": runtime_report["terminal_outcome"],
        },
        "avatar_preview": {
            "package_id": avatar_report["package_id"],
            "preview_kind": avatar_report["preview_kind"],
            "source_hotplug_terminal_outcome": avatar_report["source_hotplug_terminal_outcome"],
            "selected_representation": avatar_report["selected_representation"],
            "fallback_applied": avatar_report["fallback_applied"],
            "active_roles": avatar_report["active_roles"],
            "native_execution_performed": avatar_report["native_execution_performed"],
        },
    }
    payload["what_it_proves"] = [
        "one practical world package can be previewed with exact companion custom-avatar preview evidence",
        "custom avatar selection remains explicit in the exact avatar preview report while native execution stays unclaimed",
        "product-preview composition can consume avatar preview reports without collapsing them into same-session runtime attachment",
    ]
    payload["what_it_does_not_prove"] = list(COMMON_NON_CLAIMS) + [
        "native execution",
        "same-session runtime attach execution",
        "final avatar ABI",
        "successful custom-avatar runtime execution",
        "VRM/VRChat/Unity compatibility",
    ]
    return payload


def _unsupported_runtime_visible_fallback_bundle(
    preview: dict[str, Any], preview_file: Path
) -> dict[str, Any]:
    _expect_id(preview, "PE2E-09")
    _expect_kind(preview, "unsupported_runtime_visible_fallback_preview")
    _required_source_reports(preview, ["RUN-01", "AV-A1-03"])
    world_package = _resolve_preview_package(preview_file, preview["world_package"])
    avatar_package = _resolve_preview_package(preview_file, preview["avatar_package"])
    runtime_report = _run_local_report("RUN-01")
    avatar_report = _avatar_report("AV-A1-03")
    payload = _base_bundle(
        sample_id="PE2E-09",
        bundle_kind="unsupported_runtime_visible_fallback_preview",
        source_packages=[
            _package_ref(world_package, "world_package"),
            _package_ref(avatar_package, "companion_avatar_package"),
        ],
        source_reports=[
            _source_report_ref(
                family="practical-alpha1-local-runtime",
                sample_id="RUN-01",
                carrier_scope=runtime_report["runtime_scope"],
                surface_kind=runtime_report["surface_kind"],
            ),
            _source_report_ref(
                family="practical-alpha1-avatar-preview",
                sample_id="AV-A1-03",
                carrier_scope=avatar_report["avatar_scope"],
                surface_kind=avatar_report["surface_kind"],
            ),
        ],
    )
    payload["preview_sections"] = {
        "runtime_world": {
            "package_id": runtime_report["package_id"],
            "world_id": runtime_report["world_id"],
            "terminal_outcome": runtime_report["terminal_outcome"],
        },
        "avatar_preview": {
            "package_id": avatar_report["package_id"],
            "preview_kind": avatar_report["preview_kind"],
            "source_hotplug_terminal_outcome": avatar_report["source_hotplug_terminal_outcome"],
            "source_hotplug_reason_family": avatar_report["source_hotplug_reason_family"],
            "source_hotplug_rejection_reason_refs": avatar_report[
                "source_hotplug_rejection_reason_refs"
            ],
            "selected_representation": avatar_report["selected_representation"],
            "fallback_applied": avatar_report["fallback_applied"],
            "fallback_reason": avatar_report["fallback_reason"],
            "active_roles": avatar_report["active_roles"],
            "degraded_roles": avatar_report["degraded_roles"],
            "native_execution_performed": avatar_report["native_execution_performed"],
        },
    }
    payload["what_it_proves"] = [
        "one practical world package can be previewed with exact companion unsupported-runtime visible fallback evidence",
        "the source avatar preview retains rejected runtime-route evidence and degraded roles explicitly",
        "product-preview composition can show unsupported-runtime fallback without reclassifying the source lane as execution success",
    ]
    payload["what_it_does_not_prove"] = list(COMMON_NON_CLAIMS) + [
        "native execution",
        "same-session runtime attach execution",
        "final avatar ABI",
        "successful unsupported-runtime execution",
        "VRM/VRChat/Unity compatibility",
    ]
    return payload


def build_bundle(preview: dict[str, Any], preview_file: Path) -> dict[str, Any]:
    sample_id = preview.get("preview_id")
    if sample_id == "PE2E-01":
        return _local_full_toolchain_bundle(preview, preview_file)
    if sample_id == "PE2E-02":
        return _docker_full_toolchain_bundle(preview, preview_file)
    if sample_id == "PE2E-03":
        return _debug_layer_companion_bundle(preview, preview_file)
    if sample_id == "PE2E-04":
        return _placeholder_object_companion_bundle(preview, preview_file)
    if sample_id == "PE2E-05":
        return _local_save_load_continue_bundle(preview, preview_file)
    if sample_id == "PE2E-06":
        return _invalid_distributed_save_reject_bundle(preview, preview_file)
    if sample_id == "PE2E-07":
        return _devtools_viewer_bundle(preview, preview_file)
    if sample_id == "PE2E-08":
        return _custom_avatar_companion_bundle(preview, preview_file)
    if sample_id == "PE2E-09":
        return _unsupported_runtime_visible_fallback_bundle(preview, preview_file)
    raise RuntimeError(f"unsupported practical product preview id {sample_id!r}")


def check_path(preview_path: str | Path) -> dict[str, Any]:
    preview, preview_file = _load_preview(preview_path)
    return build_bundle(preview, preview_file)


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    expected = _load_expected_report(row)
    actual = check_path(REPO_ROOT / row["preview_dir"])
    if actual != expected:
        raise RuntimeError(
            f"{sample_id}: expected product-preview bundle drift\n"
            f"expected={json.dumps(expected, ensure_ascii=False, sort_keys=True)}\n"
            f"actual={json.dumps(actual, ensure_ascii=False, sort_keys=True)}"
        )
    return actual


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    bundles: list[dict[str, Any]] = []
    html_error: str | None = None
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            bundle = run_sample(sample_id)
            passed.append(sample_id)
            bundles.append(bundle)
        except Exception as error:  # pragma: no cover
            failed.append({"sample_id": sample_id, "error": str(error)})
    try:
        render_html("PE2E-07")
        viewer_html_available = True
    except Exception as error:  # pragma: no cover
        viewer_html_available = False
        html_error = str(error)
    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "actualized_rows": [bundle["sample_id"] for bundle in bundles],
        "product_preview_first_floor_complete": not failed and viewer_html_available,
        "stage_pa1_8_complete": False,
        "deferred_avatar_semantics": list(DEFERRED_AVATAR_SEMANTICS),
        "viewer_html_available": viewer_html_available,
        "viewer_html_error": html_error,
    }


def render_html(sample_id: str) -> dict[str, str]:
    bundle = run_sample(sample_id)
    title = html.escape(f"{bundle['sample_id']} {bundle['bundle_kind']}")
    source_package_rows = "".join(
        (
            "<li>"
            f"{html.escape(package['role'])}: "
            f"{html.escape(package['package_id'])} "
            f"({html.escape(package['front_door_entry'])})"
            "</li>"
        )
        for package in bundle["source_packages"]
    )
    source_report_rows = "".join(
        (
            "<li>"
            f"{html.escape(report['sample_id'])} "
            f"[{html.escape(report.get('family', 'unknown-family'))}]"
            "</li>"
        )
        for report in bundle["source_reports"]
    )
    sections = html.escape(
        json.dumps(bundle["preview_sections"], ensure_ascii=False, indent=2)
    )
    proves = "".join(
        f"<li>{html.escape(line)}</li>" for line in bundle["what_it_proves"]
    )
    non_claims = "".join(
        f"<li>{html.escape(line)}</li>" for line in bundle["what_it_does_not_prove"]
    )
    document = "\n".join(
        [
            "<!DOCTYPE html>",
            "<html lang='en'>",
            "<head>",
            "  <meta charset='utf-8' />",
            f"  <title>{title}</title>",
            "</head>",
            "<body>",
            f"  <h1>{title}</h1>",
            f"  <p>{html.escape(PREVIEW_BOUNDARY)}</p>",
            "  <h2>Source Packages</h2>",
            f"  <ul>{source_package_rows}</ul>",
            "  <h2>Source Reports</h2>",
            f"  <ul>{source_report_rows}</ul>",
            "  <h2>Preview Sections</h2>",
            f"  <pre>{sections}</pre>",
            "  <h2>What It Proves</h2>",
            f"  <ul>{proves}</ul>",
            "  <h2>What It Does Not Prove</h2>",
            f"  <ul>{non_claims}</ul>",
            "</body>",
            "</html>",
        ]
    )
    return {
        "sample_id": bundle["sample_id"],
        "bundle_kind": bundle["bundle_kind"],
        "html": document,
    }


def closeout() -> dict[str, Any]:
    summary = check_all()
    return {
        "sample_root": "samples/practical-alpha1",
        "preview_root": "samples/practical-alpha1/previews",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "validation_floor": [
            "cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture",
            "cargo test -p mir-ast practical_alpha1_checker -- --nocapture",
            "cargo test -p mir-ast practical_alpha1_runtime_plan -- --nocapture",
            "cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture",
            "cargo test -p mir-ast --test practical_alpha1_save_load_plan -- --nocapture",
            "cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture",
            "cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture",
            "cargo test -p mir-runtime --test practical_alpha1_transport -- --nocapture",
            "python3 scripts/practical_alpha1_check.py check-all --format json",
            "python3 scripts/practical_alpha1_run_local.py check-all --format json",
            "python3 scripts/practical_alpha1_attach.py check-all --format json",
            "python3 scripts/practical_alpha1_avatar.py check-all --format json",
            "python3 scripts/practical_alpha1_transport.py check-all --format json",
            "python3 scripts/practical_alpha1_export_devtools.py check-all --format json",
            "python3 scripts/practical_alpha1_save_load.py check-all --format json",
            "python3 scripts/practical_alpha1_product_preview.py check-all --format json",
            "python3 scripts/practical_alpha1_product_preview.py render-html PE2E-07 --format json",
            "python3 -m unittest scripts.tests.test_practical_alpha1_product_preview scripts.tests.test_validate_docs",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        "product_preview_first_floor_complete": summary["product_preview_first_floor_complete"],
        "stage_pa1_8_complete": summary["stage_pa1_8_complete"],
        "deferred_avatar_semantics": summary["deferred_avatar_semantics"],
        "viewer_html_available": summary.get("viewer_html_available", False),
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(
            f"{row['sample_id']} {row['summary']}" for row in payload
        )
    if isinstance(payload, dict) and payload.keys() == {"sample_id", "bundle_kind", "html"}:
        return f"{payload['sample_id']} {payload['bundle_kind']}\n{payload['html']}"
    return json.dumps(payload, ensure_ascii=False, indent=2)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "Practical alpha-1 product-preview command surface. This consumes preview "
            "manifests over exact practical reports and remains non-final."
        )
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    subparsers.add_parser("list")

    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")

    check_parser = subparsers.add_parser("check")
    check_parser.add_argument("preview_path")

    render_parser = subparsers.add_parser("render-html")
    render_parser.add_argument("sample_id")

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
    known = {"list", "run", "check", "render-html", "check-all", "closeout"}
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
        payload = check_path(args.preview_path)
    elif args.command == "render-html":
        payload = render_html(args.sample_id)
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
