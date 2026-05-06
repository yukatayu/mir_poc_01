from __future__ import annotations

import io
import json
import sys
import unittest
from contextlib import redirect_stdout
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import operational_product_samples


class OperationalProductSamplesTests(unittest.TestCase):
    def test_list_samples_includes_operational_roots(self) -> None:
        payload = operational_product_samples.list_samples()

        self.assertEqual(
            payload["package_name"],
            "P-OPS-01 operational product sample suite scaffold and first workflow",
        )
        roots = {row["root"] for row in payload["samples"]}
        self.assertIn("samples/product-alpha1/operational/world-core", roots)
        self.assertIn("samples/product-alpha1/operational/membership-chat", roots)
        self.assertIn("samples/product-alpha1/operational/sugoroku-world", roots)

    def test_sample_rows_marks_future_portal_as_non_runnable(self) -> None:
        rows = operational_product_samples.sample_rows()
        portal = next(row for row in rows if row["sample_id"] == "OPS-06")

        self.assertFalse(portal["runnable"])
        self.assertEqual(portal["package_kind"], "portal_worldlink")

    def test_operational_attach_specs_include_deferred_boundaries(self) -> None:
        specs = operational_product_samples.operational_attach_specs()

        self.assertIn(
            ("placeholder-object", operational_product_samples.LAYERS_ROOT / "placeholder-object", "deferred"),
            specs,
        )
        self.assertIn(
            (
                "custom-avatar-preview",
                operational_product_samples.LAYERS_ROOT / "custom-avatar-preview",
                "deferred",
            ),
            specs,
        )

    def test_main_accepts_global_format_before_subcommand(self) -> None:
        stdout = io.StringIO()
        with redirect_stdout(stdout):
            exit_code = operational_product_samples.main(["--format", "json", "list"])

        self.assertEqual(exit_code, 0)
        payload = json.loads(stdout.getvalue())
        self.assertEqual(
            payload["surface_kind"], "operational_product_sample_suite_list"
        )

    def test_membership_chat_echo_text_observed_requires_expected_event(self) -> None:
        result = operational_product_samples.CommandResult(
            name="run-local:membership-chat",
            argv=[],
            returncode=0,
            stdout="",
            stderr="",
            payload={
                "typed_host_io_claimed": True,
                "session": {
                    "host_io_history": [{"adapter_kind": "EchoText"}],
                    "observer_safe_export": {
                        "visible_host_io_events": [
                            operational_product_samples.EXPECTED_MEMBERSHIP_CHAT_HOST_IO_EVENT
                        ]
                    },
                },
            },
        )

        self.assertTrue(
            operational_product_samples.membership_chat_echo_text_observed(result)
        )

    def test_membership_chat_devtools_check_requires_event_dag_and_echo_text(self) -> None:
        result = operational_product_samples.CommandResult(
            name="export-devtools:membership-chat",
            argv=[],
            returncode=0,
            stdout="",
            stderr="",
            payload={
                "panel_ids": ["event_dag", "message_route_graph"],
                "session": {
                    "observer_safe_export": {
                        "visible_host_io_events": [
                            operational_product_samples.EXPECTED_MEMBERSHIP_CHAT_HOST_IO_EVENT
                        ]
                    }
                },
            },
        )

        self.assertTrue(
            operational_product_samples.membership_chat_devtools_echo_text_observed(
                result
            )
        )

    def test_sugoroku_run_check_requires_bounded_runtime_evidence(self) -> None:
        result = operational_product_samples.CommandResult(
            name="run-local:sugoroku",
            argv=[],
            returncode=0,
            stdout="",
            stderr="",
            payload={
                "session": {
                    "event_dag": {
                        "nodes": [
                            {"event_kind": "sugoroku_roll_requested"},
                            {"event_kind": "sugoroku_roll_published"},
                            {"event_kind": "sugoroku_witness_emitted"},
                            {"event_kind": "sugoroku_turn_handoff"},
                            {"event_kind": "sugoroku_stale_membership_rejected"},
                        ]
                    },
                    "route_graph": {
                        "routes": [
                            {"transport_lane": "same_session_sugoroku_roll"},
                            {"transport_lane": "same_session_sugoroku_handoff"},
                            {
                                "transport_lane": "same_session_sugoroku_membership_reject"
                            },
                        ]
                    },
                    "message_recovery_state": {
                        "message_state_lane": [
                            {
                                "state": "Rejected",
                                "failure_class": "StaleMembership",
                            }
                        ]
                    },
                }
            },
        )

        self.assertTrue(
            operational_product_samples.sugoroku_runtime_evidence_observed(result)
        )

    def test_sugoroku_devtools_check_requires_event_dag_panel_and_runtime_evidence(
        self,
    ) -> None:
        result = operational_product_samples.CommandResult(
            name="export-devtools:sugoroku",
            argv=[],
            returncode=0,
            stdout="",
            stderr="",
            payload={
                "panel_ids": ["event_dag", "message_route_graph"],
                "session": {
                    "event_dag": {
                        "nodes": [
                            {"event_kind": "sugoroku_roll_requested"},
                            {"event_kind": "sugoroku_roll_published"},
                            {"event_kind": "sugoroku_witness_emitted"},
                            {"event_kind": "sugoroku_turn_handoff"},
                            {"event_kind": "sugoroku_stale_membership_rejected"},
                        ]
                    },
                    "route_graph": {
                        "routes": [
                            {"transport_lane": "same_session_sugoroku_roll"},
                            {"transport_lane": "same_session_sugoroku_handoff"},
                            {
                                "transport_lane": "same_session_sugoroku_membership_reject"
                            },
                        ]
                    },
                    "message_recovery_state": {
                        "message_state_lane": [
                            {
                                "state": "Rejected",
                                "failure_class": "StaleMembership",
                            }
                        ]
                    },
                },
            },
        )

        self.assertTrue(
            operational_product_samples.sugoroku_devtools_runtime_evidence_observed(
                result
            )
        )

    def test_main_accepts_format_after_subcommand(self) -> None:
        stdout = io.StringIO()
        with redirect_stdout(stdout):
            exit_code = operational_product_samples.main(["list", "--format", "json"])

        self.assertEqual(exit_code, 0)
        payload = json.loads(stdout.getvalue())
        self.assertEqual(
            payload["surface_kind"], "operational_product_sample_suite_list"
        )


if __name__ == "__main__":
    unittest.main()
