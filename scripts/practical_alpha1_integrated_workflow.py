#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import sys
from functools import lru_cache
from pathlib import Path
from typing import Any


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent
sys.path.insert(0, str(SCRIPT_DIR))

import practical_alpha05_session  # noqa: E402
import practical_alpha08_session_hotplug  # noqa: E402
import practical_alpha09_devtools  # noqa: E402
import practical_alpha1_check  # noqa: E402
import practical_alpha1_product_preview  # noqa: E402
import practical_alpha1_run_local  # noqa: E402


WORKFLOW_SCOPE = "practical-alpha1-integrated-workflow-floor"
SURFACE_KIND = "practical_alpha1_nonfinal_integrated_workflow_report"

IMPLEMENTED_ROWS: list[dict[str, str]] = [
    {
        "sample_id": "PA1W-01",
        "summary": "source front-door, checker, runtime-plan, and local-runtime evidence are present",
        "section": "source_frontdoor",
    },
    {
        "sample_id": "PA1W-02",
        "summary": "same-session runtime carrier executes local runtime plus typed host-I/O",
        "section": "same_session_runtime",
    },
    {
        "sample_id": "PA1W-03",
        "summary": "same-session hot-plug lifecycle includes accepted, rejected, and deferred rows",
        "section": "same_session_hotplug",
    },
    {
        "sample_id": "PA1W-04",
        "summary": "local save/load timeline is tied to the same session",
        "section": "save_load",
    },
    {
        "sample_id": "PA1W-05",
        "summary": "session-bound devtools export and non-final viewer are available",
        "section": "session_devtools",
    },
    {
        "sample_id": "PA1W-06",
        "summary": "product-preview evidence is consumed without rebranding it as product-ready",
        "section": "product_preview",
    },
    {
        "sample_id": "PA1W-07",
        "summary": "negative membership, capability, witness, and hot-plug guards remain visible",
        "section": "negative_guards",
    },
    {
        "sample_id": "PA1W-08",
        "summary": "non-final boundaries and product/public stop lines remain explicit",
        "section": "non_final_boundaries",
    },
]

STOP_LINES = [
    "do not treat this integrated workflow as final public alpha-1 product readiness",
    "do not treat the non-final viewer as a public viewer/telemetry ABI",
    "do not treat local save/load as distributed durable save/load",
    "do not treat first-floor Docker/product-preview evidence as production WAN/federation",
]

LIMITATIONS = [
    "bounded practical alpha-1 integrated workflow floor only",
    "composes existing first-floor and bounded operational carriers without widening public API",
    "final public packaging, U1, durable audit, and distributed durability remain later",
]

WHAT_IT_DOES_NOT_PROVE = [
    "final public parser grammar",
    "final public checker/runtime API",
    "final public viewer / telemetry ABI",
    "distributed durable save/load",
    "production WAN/federation",
    "native avatar execution",
    "product/public-ready alpha-1",
]


def _implemented_row(sample_id: str) -> dict[str, str]:
    for row in IMPLEMENTED_ROWS:
        if row["sample_id"] == sample_id:
            return row
    raise ValueError(f"unknown practical alpha-1 integrated workflow sample {sample_id}")


def list_samples() -> list[dict[str, str]]:
    return [
        {
            "sample_id": row["sample_id"],
            "family": "practical-alpha1-integrated-workflow",
            "summary": row["summary"],
            "section": row["section"],
        }
        for row in IMPLEMENTED_ROWS
    ]


@lru_cache(maxsize=1)
def build_workflow_report() -> dict[str, Any]:
    checker_report = practical_alpha1_check.run_sample("CHK-LIF-02")
    local_runtime_report = practical_alpha1_run_local.run_sample("RUN-01")
    session_payload = practical_alpha09_devtools.build_session_devtools_payload()
    devtools_export = session_payload["devtools_export"]
    viewer = practical_alpha09_devtools.render_html()
    product_previews = {
        "PE2E-01": practical_alpha1_product_preview.run_sample("PE2E-01"),
        "PE2E-02": practical_alpha1_product_preview.run_sample("PE2E-02"),
        "PE2E-07": practical_alpha1_product_preview.run_sample("PE2E-07"),
    }
    alpha05_summary = practical_alpha05_session.check_all()
    alpha08_summary = practical_alpha08_session_hotplug.check_all()

    workflow_steps = [
        workflow_step(
            "source_check",
            "source_frontdoor",
            checker_report["sample_id"],
            checker_report["verdict"],
        ),
        workflow_step(
            "local_runtime",
            "runtime_plan_and_run_local",
            local_runtime_report["package_id"],
            local_runtime_report["terminal_outcome"],
        ),
        workflow_step(
            "same_session_runtime",
            "start_host_io_attach_save_load",
            session_payload["loaded_session"]["session_id"],
            session_payload["loaded_session"]["session_phase"],
        ),
        workflow_step(
            "session_devtools",
            "export_devtools",
            devtools_export["session_id"],
            "exported",
        ),
        workflow_step(
            "product_preview",
            "consume_first_floor_preview",
            "PE2E-01/02/07",
            "consumed",
        ),
    ]

    report = {
        "surface_kind": SURFACE_KIND,
        "scope_kind": "alpha_local",
        "workflow_scope": WORKFLOW_SCOPE,
        "workflow_id": "PA1W-bounded-integrated-workflow",
        "sample_root": "samples/practical-alpha1",
        "source_frontdoor": {
            "checker_sample_id": checker_report["sample_id"],
            "checker_verdict": checker_report["verdict"],
            "accepted_obligation_count": len(checker_report["accepted_obligations"]),
            "local_runtime_package_id": local_runtime_report["package_id"],
            "runtime_plan_scope": local_runtime_report["runtime_plan_scope"],
            "local_runtime_terminal_outcome": local_runtime_report["terminal_outcome"],
        },
        "same_session_runtime": {
            "session_id": session_payload["loaded_session"]["session_id"],
            "session_phase": session_payload["loaded_session"]["session_phase"],
            "typed_host_io_claimed": session_payload["loaded_session"]["typed_host_io_claimed"],
            "same_session_hotplug_claimed": session_payload["loaded_session"][
                "same_session_hotplug_claimed"
            ],
            "active_layers": session_payload["loaded_session"]["active_layers"],
            "host_io_events": devtools_export["export_sections"]["redacted_observer_view"][
                "host_io_events"
            ],
        },
        "same_session_hotplug": {
            "lifecycle": devtools_export["export_sections"]["hotplug_lifecycle"],
            "observer_hotplug_events": devtools_export["export_sections"][
                "redacted_observer_view"
            ]["hotplug_events"],
        },
        "save_load": {
            "savepoints": [
                {
                    "savepoint_id": entry["savepoint_id"],
                    "state_roundtrip_equal": entry["state_roundtrip_equal"],
                    "saved_event_count": entry["saved_event_count"],
                }
                for entry in devtools_export["export_sections"]["save_load_timeline"]
            ],
        },
        "session_devtools_export": devtools_export,
        "viewer": {
            "viewer_mode": viewer["viewer_mode"],
            "html_path": viewer["html_path"],
            "contains_session_panels": all(
                panel_id in viewer["html"]
                for panel_id in [
                    "event_dag_live_session",
                    "hotplug_lifecycle",
                    "retention_on_demand_trace",
                ]
            ),
        },
        "product_preview_evidence": {
            sample_id: {
                "sample_id": bundle["sample_id"],
                "bundle_kind": bundle["bundle_kind"],
                "what_it_does_not_prove": bundle["what_it_does_not_prove"],
            }
            for sample_id, bundle in product_previews.items()
        },
        "negative_guards": {
            "alpha05_negative_rows_ready": alpha05_summary[
                "stale_membership_non_resurrection_present"
            ]
            and alpha05_summary["operational_alpha05_ready"],
            "alpha08_rejected_attach_no_mutation_present": alpha08_summary[
                "rejected_attach_no_mutation_present"
            ],
            "hotplug_rejected_observations": [
                entry["sample_id"]
                for entry in devtools_export["export_sections"]["hotplug_lifecycle"]
                if entry["terminal_outcome"] == "rejected"
            ],
        },
        "workflow_steps": workflow_steps,
        "what_it_proves": [
            "existing practical alpha-1 first-floor inputs can be followed through a bounded developer workflow",
            "same-session local runtime, typed host-I/O, hot-plug, save/load, and devtools export compose on one session carrier",
            "product-preview evidence is visible as first-floor evidence, not as a final product claim",
        ],
        "what_it_does_not_prove": list(WHAT_IT_DOES_NOT_PROVE),
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        "bounded_practical_alpha1_workflow_ready": True,
        "product_public_ready": False,
        "final_public_api_frozen": False,
        "distributed_durable_save_load_claimed": False,
    }
    return report


def workflow_step(step_id: str, step_kind: str, source_ref: str, terminal_outcome: str) -> dict[str, str]:
    return {
        "step_id": step_id,
        "step_kind": step_kind,
        "source_ref": source_ref,
        "terminal_outcome": terminal_outcome,
    }


def run_sample(sample_id: str) -> dict[str, Any]:
    row = _implemented_row(sample_id)
    report = build_workflow_report()
    return {
        "sample_id": sample_id,
        "family": "practical-alpha1-integrated-workflow",
        "section": row["section"],
        "workflow_report": report,
    }


def _validate_sample(sample_id: str, payload: dict[str, Any]) -> list[str]:
    failures: list[str] = []
    report = payload["workflow_report"]
    if report["product_public_ready"] is not False:
        failures.append("integrated workflow must not claim product/public readiness")
    if sample_id == "PA1W-01":
        source = report["source_frontdoor"]
        if source["checker_verdict"] != "accepted":
            failures.append("source/checker evidence must include one accepted checker report")
        if source["local_runtime_terminal_outcome"] != "accepted":
            failures.append("local runtime evidence must include accepted execution")
    elif sample_id == "PA1W-02":
        session = report["same_session_runtime"]
        if not session["typed_host_io_claimed"]:
            failures.append("same-session runtime must include typed host-I/O")
        if "host_io:AddOne(41)->42" not in session["host_io_events"]:
            failures.append("same-session runtime must include AddOne host-I/O event")
    elif sample_id == "PA1W-03":
        outcomes = {
            (entry["terminal_outcome"], entry["session_mutated"])
            for entry in report["same_session_hotplug"]["lifecycle"]
        }
        for expected in [
            ("accepted", True),
            ("rejected", False),
            ("deferred_detach_minimal_contract", True),
        ]:
            if expected not in outcomes:
                failures.append(f"hot-plug lifecycle missing {expected}")
    elif sample_id == "PA1W-04":
        if not any(
            savepoint["savepoint_id"] == "savepoint#oa09"
            and savepoint["state_roundtrip_equal"]
            for savepoint in report["save_load"]["savepoints"]
        ):
            failures.append("save/load section must include the alpha-0.9 session savepoint")
    elif sample_id == "PA1W-05":
        if not report["viewer"]["contains_session_panels"]:
            failures.append("viewer must include session-bound devtools panels")
        if not report["session_devtools_export"]["operational_alpha09_export_ready"]:
            failures.append("session devtools export must be alpha-0.9 ready")
    elif sample_id == "PA1W-06":
        previews = report["product_preview_evidence"]
        if set(previews) != {"PE2E-01", "PE2E-02", "PE2E-07"}:
            failures.append("product preview evidence must include local, Docker, and viewer rows")
        if not all(
            "final public CLI" in " ".join(bundle["what_it_does_not_prove"])
            or "final public viewer API" in " ".join(bundle["what_it_does_not_prove"])
            for bundle in previews.values()
        ):
            failures.append("product preview evidence must keep final-public non-claims")
    elif sample_id == "PA1W-07":
        guards = report["negative_guards"]
        if not guards["alpha05_negative_rows_ready"]:
            failures.append("alpha-0.5 negative guard rows must remain ready")
        if not guards["alpha08_rejected_attach_no_mutation_present"]:
            failures.append("alpha-0.8 rejected attach no-mutation guard must remain ready")
        if "HP-A1-02" not in guards["hotplug_rejected_observations"]:
            failures.append("rejected hot-plug observations must include HP-A1-02")
    else:
        for non_claim in WHAT_IT_DOES_NOT_PROVE:
            if non_claim not in report["what_it_does_not_prove"]:
                failures.append(f"missing non-claim {non_claim}")
    return failures


def check_all() -> dict[str, Any]:
    passed: list[str] = []
    failed: list[dict[str, str]] = []
    for row in IMPLEMENTED_ROWS:
        sample_id = row["sample_id"]
        try:
            payload = run_sample(sample_id)
            failures = _validate_sample(sample_id, payload)
            if failures:
                raise RuntimeError("; ".join(failures))
            passed.append(sample_id)
        except Exception as error:  # pragma: no cover
            failed.append({"sample_id": sample_id, "error": str(error)})
    return {
        "sample_count": len(IMPLEMENTED_ROWS),
        "passed": passed,
        "failed": failed,
        "bounded_practical_alpha1_workflow_ready": not failed,
        "source_frontdoor_present": "PA1W-01" in passed,
        "same_session_runtime_present": "PA1W-02" in passed,
        "same_session_hotplug_present": "PA1W-03" in passed,
        "save_load_present": "PA1W-04" in passed,
        "session_devtools_present": "PA1W-05" in passed,
        "product_preview_evidence_present": "PA1W-06" in passed,
        "negative_guards_present": "PA1W-07" in passed,
        "non_final_boundaries_present": "PA1W-08" in passed,
        "product_public_ready": False,
    }


def closeout() -> dict[str, Any]:
    payload = check_all()
    return {
        "sample_root": "samples/practical-alpha1",
        "implemented_rows": [row["sample_id"] for row in IMPLEMENTED_ROWS],
        "validation_floor": [
            "python3 scripts/practical_alpha1_integrated_workflow.py check-all --format json",
            "python3 -m unittest scripts.tests.test_practical_alpha1_integrated_workflow",
            "python3 scripts/practical_alpha09_devtools.py check-all --format json",
            "python3 scripts/practical_alpha1_export_devtools.py check-all --format json",
            "python3 scripts/practical_alpha1_product_preview.py check-all --format json",
        ],
        "stop_lines": list(STOP_LINES),
        "limitations": list(LIMITATIONS),
        **payload,
    }


def format_pretty(payload: Any) -> str:
    if isinstance(payload, list):
        return "\n".join(
            f"{row['sample_id']} {row['summary']} [{row['section']}]" for row in payload
        )
    return json.dumps(payload, ensure_ascii=False, indent=2)


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description=(
            "Practical alpha-1 integrated workflow helper. This composes existing "
            "first-floor and bounded operational carriers without claiming product/public readiness."
        )
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("list")
    run_parser = subparsers.add_parser("run")
    run_parser.add_argument("sample_id")
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
    else:
        payload = closeout()
    if args.format == "json":
        print(json.dumps(payload, ensure_ascii=False, indent=2))
    else:
        print(format_pretty(payload))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
