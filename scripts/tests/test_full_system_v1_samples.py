from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]


def _run_helper(*args: str) -> dict:
    completed = subprocess.run(
        ["python3", "scripts/full_system_v1_samples.py", *args, "--format", "json"],
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if completed.returncode != 0:
        raise AssertionError(
            f"helper failed ({completed.returncode}): {completed.stderr}\n{completed.stdout}"
        )
    return json.loads(completed.stdout)


class FullSystemV1SamplesTests(unittest.TestCase):
    def test_matrix_reports_checker_row_set(self) -> None:
        payload = _run_helper("matrix")

        self.assertEqual(payload["family"], "full_system_v1_typed_ir")
        self.assertEqual(payload["sample_count"], 12)
        self.assertEqual(payload["executable_count"], 12)
        self.assertEqual(payload["validation_errors"], [])

    def test_runtime_matrix_reports_runtime_row_set(self) -> None:
        payload = _run_helper("runtime-matrix")

        self.assertEqual(payload["family"], "full_system_v1_runtime")
        self.assertEqual(payload["sample_count"], 17)
        self.assertEqual(payload["executable_count"], 17)
        self.assertEqual(payload["validation_errors"], [])

    def test_operational_matrix_reports_source_operational_rows(self) -> None:
        payload = _run_helper("operational-matrix")

        self.assertEqual(payload["family"], "full_system_v1_source_operational_suite")
        self.assertEqual(payload["sample_count"], 6)
        self.assertEqual(payload["executable_count"], 6)
        self.assertFalse(payload["workflow_ready"])
        self.assertEqual(payload["validation_errors"], [])
        self.assertEqual(
            [row["family_id"] for row in payload["family_counts"]],
            ["world-core", "membership-chat", "sugoroku-world"],
        )

    def test_record_positive_sample_keeps_record_summary(self) -> None:
        payload = _run_helper("run", "mir-02-record-field-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["record_summaries"][0]["record_name"], "Pair")
        self.assertEqual(payload["actual"]["diagnostic_codes"], [])

    def test_runtime_positive_sample_keeps_trace_summary(self) -> None:
        payload = _run_helper("run-runtime", "mir-03-control-flow-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["outcome"], "Accepted")
        self.assertEqual(payload["actual"]["entry_kind"], "Function")
        self.assertEqual(payload["actual"]["output_summary"], "Int64(10)")
        self.assertIn("while", payload["actual"]["trace_event_kinds"])
        self.assertEqual(payload["actual"]["trace_branch_taken"][-1], "while-break@5")

    def test_effectful_positive_sample_keeps_session_summary(self) -> None:
        payload = _run_helper("run-runtime", "mir-04-effectful-sugoroku-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["entry_kind"], "Transition")
        self.assertEqual(payload["actual"]["output_summary"], "Unit")
        self.assertIn("publish", payload["actual"]["trace_event_kinds"])
        self.assertIn("atomic_cut", payload["actual"]["trace_event_kinds"])
        self.assertEqual(
            payload["actual"]["effect_session"]["host_output_summaries"], ["Int64(42)"]
        )
        self.assertEqual(
            payload["actual"]["effect_session"]["published_channels"], ["roll"]
        )
        self.assertEqual(
            payload["actual"]["effect_session"]["accepted_cuts"], ["turn-finished"]
        )

    def test_operational_world_core_positive_keeps_generated_manifest(self) -> None:
        payload = _run_helper("run-operational", "fsv1-ops-world-core-positive")

        self.assertTrue(payload["passed"])
        self.assertEqual(payload["package_kind"], "world_core")
        self.assertEqual(
            payload["manifest_actual"]["schema_version"],
            "full-system-v1-generated-package-manifest-v0",
        )
        self.assertEqual(
            payload["manifest_actual"]["transition_summaries"][0]["place_ref"],
            "WorldServerPlace",
        )
        self.assertTrue(payload["manifest_returncode_passed"])
        self.assertTrue(payload["runtime_returncode_passed"])
        self.assertTrue(payload["runtime_actual"]["accepted"])
        self.assertIn("publish", payload["runtime_actual"]["trace_event_kinds"])
        self.assertIn("provider_boundary", payload["runtime_actual"]["trace_event_kinds"])

    def test_operational_membership_negative_reports_expected_reason(self) -> None:
        payload = _run_helper("run-operational", "fsv1-ops-membership-chat-negative")

        self.assertTrue(payload["manifest_passed"])
        self.assertTrue(payload["runtime_passed"])
        self.assertTrue(payload["manifest_returncode_passed"])
        self.assertTrue(payload["runtime_returncode_passed"])
        self.assertEqual(payload["runtime_returncode_expected"], 2)
        self.assertFalse(payload["runtime_actual"]["accepted"])
        self.assertEqual(
            payload["runtime_actual"]["runtime_rejection_code"],
            "contract_require_failed",
        )
        self.assertEqual(
            payload["runtime_actual"]["effect_session"]["published_channels"],
            ["membership_epoch"],
        )

    def test_operational_sugoroku_positive_keeps_witness_and_cut_summary(self) -> None:
        payload = _run_helper("run-operational", "fsv1-ops-sugoroku-world-positive")

        self.assertTrue(payload["passed"])
        self.assertEqual(payload["package_kind"], "sugoroku_world")
        self.assertTrue(payload["manifest_returncode_passed"])
        self.assertTrue(payload["runtime_returncode_passed"])
        self.assertEqual(
            payload["runtime_actual"]["effect_session"]["witness_refs"],
            ["witness#1"],
        )
        self.assertEqual(
            payload["runtime_actual"]["effect_session"]["accepted_cuts"],
            ["sugoroku-turn-finished"],
        )
        self.assertIn("handoff", payload["runtime_actual"]["trace_event_kinds"])

    def test_pure_runtime_sample_keeps_empty_effect_session(self) -> None:
        payload = _run_helper("run-runtime", "mir-03-add-one-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["entry_kind"], "Function")
        self.assertEqual(payload["actual"]["effect_session"]["host_input_remaining"], 0)
        self.assertEqual(payload["actual"]["effect_session"]["host_output_summaries"], [])
        self.assertTrue(payload["actual"]["effect_session"]["no_in_flight"])

    def test_host_output_does_not_flip_quiescence_bits(self) -> None:
        payload = _run_helper("run-runtime", "mir-04-host-boundary-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["effect_session"]["host_output_summaries"], ["Int64(42)"]
        )
        self.assertTrue(payload["actual"]["effect_session"]["no_in_flight"])

    def test_runtime_negative_sample_reports_runtime_split(self) -> None:
        payload = _run_helper("run-runtime", "mir-03-dynamic-array-runtime-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(payload["actual"]["outcome"], "RuntimeRejection")
        self.assertEqual(payload["actual"]["runtime_rejection_code"], "runtime_out_of_bounds")
        self.assertEqual(payload["actual"]["diagnostic_codes"], [])

    def test_runtime_static_negative_preserves_checker_diagnostics(self) -> None:
        payload = _run_helper("run-runtime", "mir-03-import-static-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(payload["actual"]["outcome"], "StaticRejection")
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["return_type_mismatch", "effect_failure_row_missing"],
        )
        self.assertEqual(payload["actual"]["trace_functions"], [])

    def test_check_all_passes_every_row(self) -> None:
        payload = _run_helper("check-all")

        self.assertEqual(payload["failed"], [])
        self.assertEqual(payload["validation_errors"], [])
        self.assertEqual(len(payload["checker"]["passed"]), 12)
        self.assertEqual(len(payload["runtime"]["passed"]), 17)
        self.assertEqual(len(payload["operational"]["passed"]), 6)
        self.assertEqual(len(payload["passed"]), 35)
