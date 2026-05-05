#!/usr/bin/env python3

from __future__ import annotations

import argparse
import html
import json
import subprocess
import sys
import tempfile
from functools import lru_cache
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent

BASE_SESSION_PACKAGE = "samples/practical-alpha1/packages/run-01-local-sugoroku"
HOST_IO_PACKAGE = "samples/practical-alpha1/packages/oa05-07-add-one-host-io"
ATTACH_SEQUENCE = [
    "samples/practical-alpha1/packages/hp-a1-01-debug-layer-attach",
    "samples/practical-alpha1/packages/hp-a1-02-non-admin-debug-rejected",
    "samples/practical-alpha1/packages/hp-a1-03-auth-layer-contract-update",
    "samples/practical-alpha1/packages/hp-a1-04-ratelimit-declared-failure",
    "samples/practical-alpha1/packages/hp-a1-05-incompatible-patch-rejected",
    "samples/practical-alpha1/packages/hp-a1-04b1-stale-membership-attach-rejected",
    "samples/practical-alpha1/packages/hp-a1-04b2-missing-witness-attach-rejected",
    "samples/practical-alpha1/packages/hp-a1-06-object-package-attach",
    "samples/practical-alpha1/packages/av-a1-03-unsupported-runtime-fallback",
    "samples/practical-alpha1/packages/hp-a1-07-detach-minimal-contract",
]

IMPLEMENTED_ROWS: list[dict[str, str]] = [
    {
        "sample_id": "OA09-01",
        "summary": "export the live/session event DAG from the same runtime session",
        "panel_id": "event_dag_live_session",
    },
    {
        "sample_id": "OA09-02",
        "summary": "export the session-local route trace from dispatch records",
        "panel_id": "local_route_trace",
    },
    {
        "sample_id": "OA09-03",
        "summary": "export current and saved membership frontiers as a session timeline",
        "panel_id": "membership_timeline",
    },
    {
        "sample_id": "OA09-04",
        "summary": "export witness references and generated event relations",
        "panel_id": "witness_relation",
    },
    {
        "sample_id": "OA09-05",
        "summary": "export accepted, rejected, and deferred hot-plug lifecycle rows",
        "panel_id": "hotplug_lifecycle",
    },
    {
        "sample_id": "OA09-06",
        "summary": "export fallback degradation without claiming native execution",
        "panel_id": "fallback_degradation",
    },
    {
        "sample_id": "OA09-07",
        "summary": "export the local save/load timeline from a session savepoint",
        "panel_id": "save_load_timeline",
    },
    {
        "sample_id": "OA09-08",
        "summary": "export the observer-safe redacted view and kept-later admin marker",
        "panel_id": "observer_safe_redacted_view",
    },
    {
        "sample_id": "OA09-09",
        "summary": "export retention/on-demand trace and render the non-final viewer",
        "panel_id": "retention_on_demand_trace",
    },
]

STOP_LINES = [
    "do not treat practical alpha-0.9 session devtools as a final public viewer or telemetry ABI",
    "do not treat session-local retention/on-demand trace as durable audit or remote retrieval",
    "do not treat local route trace as WAN/federation transport completion",
]

LIMITATIONS = [
    "alpha-local non-final practical alpha-0.9 session-bound devtools floor only",
    "admin/full debug view is explicitly kept-later",
    "distributed durable save/load and final public product runtime remain later",
]


def _implemented_row(sample_id: str) -> dict[str, str]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown practical alpha-0.9 devtools sample {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "practical-alpha09-session-devtools",
            "source_root": "samples/practical-alpha1/packages",
            "summary": row["summary"],
            "panel_id": row["panel_id"],
        }
        for row in IMPLEMENTED_ROWS
    ]


def _cargo_session(*args: str) -> dict[str, Any]:
    completed = subprocess.run(
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mir_practical_alpha05_session",
            "--",
            *args,
        ],
        cwd=REPO_ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    return json.loads(completed.stdout)


@lru_cache(maxsize=1)
def build_session_devtools_payload() -> dict[str, Any]:
    with tempfile.TemporaryDirectory(prefix="oa09-") as temp_dir:
        session_path = Path(temp_dir) / "session.json"
        started = _cargo_session(
            "start", str(REPO_ROOT / BASE_SESSION_PACKAGE), str(session_path)
        )
        host_io_report = _cargo_session(
            "host-io",
            str(session_path),
            str(REPO_ROOT / HOST_IO_PACKAGE),
            str(session_path),
        )
        attach_reports = [
            _cargo_session(
                "attach",
                str(session_path),
                str(REPO_ROOT / package),
                str(session_path),
            )
            for package in ATTACH_SEQUENCE
        ]
        saved = _cargo_session("save", str(session_path), "savepoint#oa09", str(session_path))
        loaded = _cargo_session("load", str(session_path), "savepoint#oa09", str(session_path))
        devtools_export = _cargo_session("export-devtools", str(session_path))
        return {
            "family": "practical-alpha09-session-devtools",
            "session_report_started": started,
            "host_io_report": host_io_report,
            "attach_reports": attach_reports,
            "saved_session": saved,
            "loaded_session": loaded,
            "devtools_export": devtools_export,
        }


def _panel(export: dict[str, Any], panel_id: str) -> dict[str, Any]:
    for panel in export["panels"]:
        if panel["panel_id"] == panel_id:
            return panel
    raise AssertionError(f"missing panel {panel_id}")


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    payload = build_session_devtools_payload()
    export = payload["devtools_export"]
    panel = _panel(export, row["panel_id"])
    return {
        "sample_id": sample_id,
        "family": "practical-alpha09-session-devtools",
        "panel": panel,
        "devtools_export": export,
    }


def _validate_sample(sample_id: str, payload: dict[str, Any]) -> list[str]:
    failures: list[str] = []
    export = payload["devtools_export"]
    sections = export["export_sections"]
    if export["export_source_kind"] != "same_runtime_session_carrier":
        failures.append("export source must be the same runtime session carrier")
    if export["final_public_viewer_frozen"] is not False:
        failures.append("final public viewer must remain unfrozen")
    if sample_id == "OA09-01":
        if not sections["event_dag"]["nodes"]:
            failures.append("event DAG panel must export live session nodes")
    elif sample_id == "OA09-02":
        if not sections["route_trace"]:
            failures.append("route trace panel must export session dispatch route rows")
        if sections["route_trace"][0]["source_kind"] != "session_dispatch_record":
            failures.append("route trace must be sourced from session dispatch records")
    elif sample_id == "OA09-03":
        if len(sections["membership_timeline"]) < 2:
            failures.append("membership timeline must include current and savepoint frontiers")
    elif sample_id == "OA09-04":
        if not sections["witness_relation"]:
            failures.append("witness relation panel must contain at least one witness relation")
    elif sample_id == "OA09-05":
        outcomes = {
            (entry["sample_id"], entry["terminal_outcome"], entry["session_mutated"])
            for entry in sections["hotplug_lifecycle"]
        }
        for expected in [
            ("HP-A1-01", "accepted", True),
            ("HP-A1-02", "rejected", False),
            ("HP-A1-07", "deferred_detach_minimal_contract", True),
        ]:
            if expected not in outcomes:
                failures.append(f"hot-plug lifecycle missing {expected}")
    elif sample_id == "OA09-06":
        fallback = sections["fallback_degradation"]
        if not any(
            entry["source_sample_id"] == "AV-A1-03"
            and entry["native_execution_performed"] is False
            for entry in fallback
        ):
            failures.append("fallback degradation must include rejected AV-A1-03 without native execution")
    elif sample_id == "OA09-07":
        if not any(
            entry["savepoint_id"] == "savepoint#oa09" and entry["state_roundtrip_equal"]
            for entry in sections["save_load_timeline"]
        ):
            failures.append("save/load timeline must include the session savepoint")
    elif sample_id == "OA09-08":
        redacted = sections["redacted_observer_view"]
        if redacted["redaction"] != "observer_safe_session_summary":
            failures.append("observer view must stay observer-safe redacted")
        if export["admin_debug_view_status"] != "kept_later":
            failures.append("admin/debug full view must be explicitly kept-later")
    else:
        outcomes = [entry["query_outcome"] for entry in sections["retention_query_trace"]]
        if outcomes != ["hit", "hit", "miss"]:
            failures.append("retention/on-demand trace must expose hit/hit/miss query outcomes")
        rendered = render_html()
        if "retention_on_demand_trace" not in rendered["html"]:
            failures.append("rendered viewer must include the retention/on-demand panel")
    return failures


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    rows: dict[str, dict[str, Any]] = {}
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            payload = run_sample(sample_id)
            failures = _validate_sample(sample_id, payload)
            if failures:
                raise RuntimeError("; ".join(failures))
            passed.append(sample_id)
            rows[sample_id] = payload
        except Exception as error:  # pragma: no cover
            failed.append({"sample_id": sample_id, "error": str(error)})

    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "session_bound_devtools_ready": not failed,
        "event_dag_live_session_present": "OA09-01" in passed,
        "route_trace_present": "OA09-02" in passed,
        "membership_timeline_present": "OA09-03" in passed,
        "witness_relation_present": "OA09-04" in passed,
        "hotplug_lifecycle_present": "OA09-05" in passed,
        "fallback_degradation_present": "OA09-06" in passed,
        "save_load_timeline_present": "OA09-07" in passed,
        "observer_safe_redacted_view_present": "OA09-08" in passed,
        "retention_on_demand_trace_present": "OA09-09" in passed,
        "operational_alpha09_ready": not failed and len(passed) == len(IMPLEMENTED_ROWS),
    }


def _render_export_html(export: dict[str, Any]) -> str:
    panel_items = "\n".join(
        "<li><strong>{}</strong> [{}] - {}</li>".format(
            html.escape(panel["panel_id"]),
            html.escape(panel["panel_kind"]),
            html.escape(panel["label"]),
        )
        for panel in export["panels"]
    )
    telemetry_items = "\n".join(
        "<li><strong>{}</strong> [{}] - {}</li>".format(
            html.escape(row["telemetry_id"]),
            html.escape(row["telemetry_kind"]),
            html.escape(row["value_summary"]),
        )
        for row in export["telemetry_rows"]
    )
    export_json = html.escape(json.dumps(export["export_sections"], ensure_ascii=False, indent=2))
    return f"""<!DOCTYPE html>
<html lang="ja">
<head>
  <meta charset="utf-8" />
  <title>OA09 session-bound devtools export</title>
  <style>
    body {{ font-family: sans-serif; margin: 2rem; line-height: 1.5; }}
    code, pre {{ font-family: monospace; }}
    pre {{ background: #f6f6f6; padding: 1rem; overflow-x: auto; }}
    section {{ margin-bottom: 2rem; }}
  </style>
</head>
<body>
  <h1>OA09 session-bound devtools export</h1>
  <p>{html.escape(export["export_source_kind"])} / {html.escape(export["viewer_mode"])}</p>
  <section>
    <h2>Panels</h2>
    <ul>{panel_items}</ul>
  </section>
  <section>
    <h2>Telemetry</h2>
    <ul>{telemetry_items}</ul>
  </section>
  <section>
    <h2>Export Sections</h2>
    <pre>{export_json}</pre>
  </section>
</body>
</html>
"""


def render_html(output_path: str | None = None) -> dict[str, Any]:
    export = build_session_devtools_payload()["devtools_export"]
    rendered = _render_export_html(export)
    if output_path is None:
        temp = tempfile.NamedTemporaryFile(
            prefix="practical-a09-devtools-",
            suffix=".html",
            delete=False,
        )
        temp_path = Path(temp.name)
        temp.close()
    else:
        temp_path = Path(output_path)
        temp_path.parent.mkdir(parents=True, exist_ok=True)
    temp_path.write_text(rendered)
    return {
        "sample_id": "OA09-09",
        "viewer_mode": export["viewer_mode"],
        "html_path": str(temp_path),
        "html": rendered,
    }


def closeout() -> dict[str, Any]:
    payload = check_all()
    return {
        "sample_root": "samples/practical-alpha1",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "validation_floor": [
            "cargo test -p mir-runtime --test practical_alpha09_devtools -- --nocapture",
            "cargo test -p mir-runtime --test practical_alpha08_session_hotplug -- --nocapture",
            "python3 scripts/practical_alpha09_devtools.py check-all --format json",
            "python3 scripts/practical_alpha09_devtools.py render-html --format json",
            "python3 -m unittest scripts.tests.test_practical_alpha09_devtools",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        **payload,
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(
            f"{row['sample_id']} {row['summary']} [{row['panel_id']}]" for row in payload
        )
    if "html_path" in payload:
        return f"OA09 viewer\nhtml_path: {payload['html_path']}"
    return json.dumps(payload, ensure_ascii=False, indent=2)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "Practical alpha-0.9 session-bound devtools helper over the alpha-0.5/0.8 session carrier. "
            "This remains non-final and bounded to local observer-safe session exports."
        )
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("list")
    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")
    subparsers.add_parser("check-all")
    subparsers.add_parser("render-html")
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
    return [*hoisted_root_options, *remainder]


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(normalize_argv(argv))
    if args.command == "list":
        payload: Any = list_samples()
    elif args.command == "run":
        payload = run_sample(args.sample_id)
    elif args.command == "check-all":
        payload = check_all()
    elif args.command == "render-html":
        payload = render_html()
    else:
        payload = closeout()
    if args.format == "json":
        print(json.dumps(payload, ensure_ascii=False, indent=2))
    else:
        print(format_pretty(payload))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
