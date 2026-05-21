from __future__ import annotations

import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import mir_computational_samples


EXPECTED_SAMPLE_IDS = [
    "comp-02-pure-add-one",
    "comp-03-variables-scope-positive",
    "comp-03-variables-scope-negative",
    "comp-03-arrays-bounds-positive",
    "comp-03-arrays-bounds-negative",
    "comp-03-records-vec3-positive",
    "comp-03-records-vec3-negative",
    "comp-03-control-flow-positive",
    "comp-03-control-flow-negative",
    "comp-03-imports-functions-positive",
    "comp-03-imports-functions-negative",
    "comp-04-host-io-internal-transform",
]
ACCEPTED_EXECUTABLE_SAMPLE_IDS = [
    "comp-02-pure-add-one",
    "comp-03-variables-scope-positive",
    "comp-03-arrays-bounds-positive",
    "comp-03-records-vec3-positive",
    "comp-03-control-flow-positive",
    "comp-03-imports-functions-positive",
]
EXPECTED_RUNTIME_REJECTION_SAMPLE_IDS = [
    "comp-03-variables-scope-negative",
    "comp-03-arrays-bounds-negative",
    "comp-03-records-vec3-negative",
    "comp-03-control-flow-negative",
    "comp-03-imports-functions-negative",
]
PLANNED_ONLY_SAMPLE_IDS = ["comp-04-host-io-internal-transform"]


class MirComputationalSamplesTests(unittest.TestCase):
    def _fake_comp02_runtime_payload(self) -> dict[str, object]:
        return {
            "surface_kind": "product_alpha1_run_local_report",
            "typed_host_io_claimed": True,
            "mir_computation_claimed": True,
            "product_alpha1_ready": False,
            "session": {
                "phase": "run_local",
                "host_io_history": [
                    {
                        "adapter_kind": "ReadInt",
                        "request_summary": "Int(41)",
                        "response_summary": "Int(41)",
                    },
                    {
                        "adapter_kind": "WriteInt",
                        "request_summary": "Int(42)",
                        "response_summary": "Int(42)",
                    },
                ],
                "mir_compute_history": [
                    {
                        "function_id": "add_one",
                        "input_summary": "Int(41)",
                        "output_summary": "Int(42)",
                    }
                ],
                "event_dag": {
                    "nodes": [
                        {"event_kind": "session_started"},
                        {"event_kind": "host_input_received"},
                        {"event_kind": "mir_compute_step"},
                        {"event_kind": "host_output_emitted"},
                    ]
                },
            },
        }

    def test_list_contains_all_rows_with_machine_readable_expectations(self) -> None:
        rows = mir_computational_samples.list_samples()
        by_id = {row["sample_id"]: row for row in rows}

        self.assertEqual([row["sample_id"] for row in rows], EXPECTED_SAMPLE_IDS)
        self.assertEqual(rows[0]["current_status"], "executable")
        self.assertTrue(
            all(by_id[sample_id]["current_status"] == "executable" for sample_id in EXPECTED_SAMPLE_IDS[:-1])
        )
        self.assertEqual(
            by_id["comp-03-variables-scope-positive"]["expected_outcome"],
            {
                "terminal_outcome": "accepted",
                "output_summary": "Int(0)",
            },
        )
        self.assertEqual(
            by_id["comp-03-variables-scope-negative"]["expected_outcome"],
            {
                "terminal_outcome": "runtime_rejection",
                "rejection_contains": "unbound variable",
            },
        )
        self.assertEqual(
            by_id["comp-03-imports-functions-positive"]["package_input"],
            "samples/product-alpha1/computational/imports-functions/positive/package.mir.json",
        )
        self.assertEqual(
            by_id["comp-04-host-io-internal-transform"]["current_status"],
            "planned_only",
        )

    def test_matrix_reports_executable_and_expected_rejection_rows(self) -> None:
        result = mir_computational_samples.matrix()

        self.assertEqual(result["sample_count"], 12)
        self.assertEqual(result["planned_count"], 1)
        self.assertEqual(result["executable_count"], 11)
        self.assertEqual(result["accepted_count"], 6)
        self.assertEqual(result["expected_runtime_rejection_count"], 5)
        self.assertEqual(result["planned_only_rows"], PLANNED_ONLY_SAMPLE_IDS)
        self.assertEqual(
            result["expected_runtime_rejection_rows"],
            EXPECTED_RUNTIME_REJECTION_SAMPLE_IDS,
        )
        self.assertEqual(result["matrix_status"], "mixed")
        self.assertFalse(result["workflow_ready"])

    def test_run_comp_02_reports_mir_owned_path(self) -> None:
        with mock.patch.object(
            mir_computational_samples,
            "_run_product_alpha1_local_session",
            return_value=self._fake_comp02_runtime_payload(),
        ):
            result = mir_computational_samples.run_sample("comp-02-pure-add-one")

        self.assertEqual(result["current_status"], "executable")
        self.assertEqual(result["terminal_outcome"], "accepted")
        self.assertEqual(result["mir_compute_function"], "add_one")
        self.assertTrue(result["mir_computation_claimed"])
        self.assertEqual(
            result["event_kinds_after"],
            [
                "session_started",
                "host_input_received",
                "mir_compute_step",
                "host_output_emitted",
            ],
        )
        self.assertTrue(result["outcome_matches_expected"])

    def test_run_comp_03_positive_row_is_accepted(self) -> None:
        result = mir_computational_samples.run_sample(
            "comp-03-control-flow-positive"
        )

        self.assertEqual(result["current_status"], "executable")
        self.assertEqual(result["execution_surface"], "helper_package_runtime")
        self.assertEqual(result["terminal_outcome"], "accepted")
        self.assertEqual(result["actual_output_summary"], "Int(15)")
        self.assertEqual(
            result["expected_outcome"],
            {
                "terminal_outcome": "accepted",
                "output_summary": "Int(15)",
            },
        )
        self.assertTrue(result["outcome_matches_expected"])

    def test_run_comp_03_negative_row_is_runtime_rejection(self) -> None:
        result = mir_computational_samples.run_sample(
            "comp-03-variables-scope-negative"
        )

        self.assertEqual(result["current_status"], "executable")
        self.assertEqual(result["execution_surface"], "helper_package_runtime")
        self.assertEqual(result["terminal_outcome"], "runtime_rejection")
        self.assertIn("unbound variable", result["actual_rejection_detail"])
        self.assertTrue(result["outcome_matches_expected"])

    def test_check_all_passes_when_executable_rows_match_contract(self) -> None:
        with mock.patch.object(
            mir_computational_samples,
            "_run_product_alpha1_local_session",
            return_value=self._fake_comp02_runtime_payload(),
        ):
            result = mir_computational_samples.check_all()

        self.assertEqual(result["failed"], [])
        self.assertEqual(result["accepted"], ACCEPTED_EXECUTABLE_SAMPLE_IDS)
        self.assertEqual(
            result["expected_runtime_rejections"],
            EXPECTED_RUNTIME_REJECTION_SAMPLE_IDS,
        )
        self.assertEqual(
            result["passed"],
            ACCEPTED_EXECUTABLE_SAMPLE_IDS + EXPECTED_RUNTIME_REJECTION_SAMPLE_IDS,
        )
        self.assertEqual(result["planned"], PLANNED_ONLY_SAMPLE_IDS)
        self.assertEqual(result["sample_count"], 12)
        self.assertFalse(result["workflow_ready"])

    def test_closeout_records_stop_lines_and_non_claims(self) -> None:
        result = mir_computational_samples.closeout()

        self.assertEqual(result["planned_sample_ids"], EXPECTED_SAMPLE_IDS)
        self.assertEqual(
            result["current_add_one_reading"],
            "one_mir_owned_path_plus_host_boundary_legacy",
        )
        self.assertIn(
            "do not treat current AddOne as Mir-owned computation everywhere",
            result["stop_lines"],
        )
        self.assertIn(
            "python3 scripts/mir_computational_samples.py check-all --format json",
            result["validation_floor"],
        )

    def test_validate_rows_rejects_missing_root(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            sample_root = Path(tmp)
            rows = [
                {
                    "sample_id": "comp-missing",
                    "root_name": "missing-root",
                    "current_status": "executable",
                    "representative_source": "missing-root/missing-root.mir",
                }
            ]

            errors = mir_computational_samples.validate_rows(sample_root, rows)

        self.assertEqual(len(errors), 3)
        self.assertEqual(errors[0]["kind"], "missing_root")
        self.assertEqual(errors[1]["kind"], "missing_representative_source")
        self.assertEqual(errors[2]["kind"], "missing_package_input")

    def test_pretty_formats_check_all_summary(self) -> None:
        with mock.patch.object(
            mir_computational_samples,
            "_run_product_alpha1_local_session",
            return_value=self._fake_comp02_runtime_payload(),
        ):
            pretty = mir_computational_samples.format_pretty(
                mir_computational_samples.check_all()
            )

        self.assertIn("CHECK-ALL SUMMARY", pretty)
        self.assertIn("planned-only: 1", pretty)
        self.assertIn("expected runtime rejections: 5", pretty)

    def test_normalize_argv_hoists_root_format_before_known_subcommand(self) -> None:
        args = mir_computational_samples.normalize_argv(["check-all", "--format", "json"])

        self.assertEqual(args, ["--format", "json", "check-all"])

    def test_normalize_argv_promotes_bare_sample_id_to_run(self) -> None:
        args = mir_computational_samples.normalize_argv(
            ["comp-02-pure-add-one", "--format", "json"]
        )

        self.assertEqual(
            args,
            ["--format", "json", "run", "comp-02-pure-add-one"],
        )


if __name__ == "__main__":
    unittest.main()
