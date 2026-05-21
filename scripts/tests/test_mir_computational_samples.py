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
    "comp-03-variables-scope",
    "comp-03-arrays-bounds",
    "comp-03-records-vec3",
    "comp-03-control-flow",
    "comp-03-imports-functions",
    "comp-04-host-io-internal-transform",
]


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

    def test_list_contains_all_planned_rows(self) -> None:
        rows = mir_computational_samples.list_samples()

        self.assertEqual([row["sample_id"] for row in rows], EXPECTED_SAMPLE_IDS)
        self.assertEqual(rows[0]["current_status"], "executable")
        self.assertTrue(
            all(row["current_status"] == "planned_only" for row in rows[1:])
        )

    def test_matrix_reports_planned_only_family(self) -> None:
        result = mir_computational_samples.matrix()

        self.assertEqual(result["sample_count"], 7)
        self.assertEqual(result["planned_count"], 6)
        self.assertEqual(result["executable_count"], 1)
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

    def test_check_all_passes_when_planned_roots_exist(self) -> None:
        with mock.patch.object(
            mir_computational_samples,
            "_run_product_alpha1_local_session",
            return_value=self._fake_comp02_runtime_payload(),
        ):
            result = mir_computational_samples.check_all()

        self.assertEqual(result["failed"], [])
        self.assertEqual(result["passed"], ["comp-02-pure-add-one"])
        self.assertEqual(result["planned"], EXPECTED_SAMPLE_IDS[1:])
        self.assertEqual(result["sample_count"], 7)
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
                    "current_status": "planned_only",
                    "representative_source": "missing-root/missing-root.mir",
                }
            ]

            errors = mir_computational_samples.validate_rows(sample_root, rows)

        self.assertEqual(len(errors), 2)
        self.assertEqual(errors[0]["kind"], "missing_root")
        self.assertEqual(errors[1]["kind"], "missing_representative_source")

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
        self.assertIn("planned-only: 6", pretty)

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
