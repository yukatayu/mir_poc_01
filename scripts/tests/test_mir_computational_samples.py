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
    "comp-04-host-io-internal-transform-positive",
    "comp-04-host-io-internal-transform-negative-undeclared-effect",
    "comp-04-host-io-internal-transform-negative-undeclared-failure",
    "comp-04-host-io-internal-transform-negative-missing-capability",
]
ACCEPTED_EXECUTABLE_SAMPLE_IDS = [
    "comp-02-pure-add-one",
    "comp-03-variables-scope-positive",
    "comp-03-arrays-bounds-positive",
    "comp-03-records-vec3-positive",
    "comp-03-control-flow-positive",
    "comp-03-imports-functions-positive",
    "comp-04-host-io-internal-transform-positive",
]
EXPECTED_RUNTIME_REJECTION_SAMPLE_IDS = [
    "comp-03-variables-scope-negative",
    "comp-03-arrays-bounds-negative",
    "comp-03-records-vec3-negative",
    "comp-03-control-flow-negative",
    "comp-03-imports-functions-negative",
]
EXPECTED_CHECK_REJECTION_SAMPLE_IDS = [
    "comp-04-host-io-internal-transform-negative-undeclared-effect",
    "comp-04-host-io-internal-transform-negative-undeclared-failure",
    "comp-04-host-io-internal-transform-negative-missing-capability",
]
PLANNED_ONLY_SAMPLE_IDS: list[str] = []


class MirComputationalSamplesTests(unittest.TestCase):
    def _fake_run_local_payload(
        self, function_id: str, input_value: int, output_value: int
    ) -> dict[str, object]:
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
                        "request_summary": f"Int({input_value})",
                        "response_summary": f"Int({input_value})",
                    },
                    {
                        "adapter_kind": "WriteInt",
                        "request_summary": f"Int({output_value})",
                        "response_summary": f"Int({output_value})",
                    },
                ],
                "mir_compute_history": [
                    {
                        "function_id": function_id,
                        "input_summary": f"Int({input_value})",
                        "output_summary": f"Int({output_value})",
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

    def _fake_check_rejection_payload(
        self, message: str, diagnostic_code: str = "SchemaDecode"
    ) -> dict[str, object]:
        return {
            "status": "error",
            "command": "check",
            "diagnostic_code": diagnostic_code,
            "message": message,
            "implemented": True,
            "product_alpha1_ready": False,
            "final_public_api_frozen": False,
        }

    def test_list_contains_all_rows_with_machine_readable_expectations(self) -> None:
        rows = mir_computational_samples.list_samples()
        by_id = {row["sample_id"]: row for row in rows}

        self.assertEqual([row["sample_id"] for row in rows], EXPECTED_SAMPLE_IDS)
        self.assertEqual(rows[0]["current_status"], "executable")
        self.assertTrue(
            all(by_id[sample_id]["current_status"] == "executable" for sample_id in EXPECTED_SAMPLE_IDS)
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
            by_id["comp-04-host-io-internal-transform-positive"]["expected_outcome"],
            {
                "terminal_outcome": "accepted",
                "output_summary": "Int(42)",
            },
        )
        self.assertEqual(
            by_id["comp-04-host-io-internal-transform-negative-undeclared-effect"][
                "expected_outcome"
            ],
            {
                "terminal_outcome": "check_rejection",
                "diagnostic_code": "SchemaDecode",
                "rejection_contains": "host_output.effect_ref",
            },
        )
        self.assertEqual(
            by_id[
                "comp-04-host-io-internal-transform-negative-missing-capability"
            ]["execution_surface"],
            "product_alpha1_check",
        )

    def test_matrix_reports_executable_and_expected_rejection_rows(self) -> None:
        result = mir_computational_samples.matrix()

        self.assertEqual(result["sample_count"], 15)
        self.assertEqual(result["planned_count"], 0)
        self.assertEqual(result["executable_count"], 15)
        self.assertEqual(result["accepted_count"], 7)
        self.assertEqual(result["expected_runtime_rejection_count"], 5)
        self.assertEqual(result["expected_check_rejection_count"], 3)
        self.assertEqual(result["planned_only_rows"], PLANNED_ONLY_SAMPLE_IDS)
        self.assertEqual(
            result["expected_runtime_rejection_rows"],
            EXPECTED_RUNTIME_REJECTION_SAMPLE_IDS,
        )
        self.assertEqual(
            result["expected_check_rejection_rows"],
            EXPECTED_CHECK_REJECTION_SAMPLE_IDS,
        )
        self.assertEqual(result["matrix_status"], "mixed")
        self.assertFalse(result["workflow_ready"])

    def test_run_comp_02_reports_mir_owned_path(self) -> None:
        with mock.patch.object(
            mir_computational_samples,
            "_run_product_alpha1_local_session",
            return_value=self._fake_run_local_payload("add_one", 41, 42),
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

    def test_run_comp_04_positive_row_is_accepted(self) -> None:
        with mock.patch.object(
            mir_computational_samples,
            "_run_product_alpha1_local_session",
            return_value=self._fake_run_local_payload("add_two", 40, 42),
        ):
            result = mir_computational_samples.run_sample(
                "comp-04-host-io-internal-transform-positive"
            )

        self.assertEqual(result["current_status"], "executable")
        self.assertEqual(result["execution_surface"], "product_alpha1_run_local")
        self.assertEqual(result["terminal_outcome"], "accepted")
        self.assertEqual(result["mir_compute_function"], "add_two")
        self.assertEqual(result["actual_output_summary"], "Int(42)")
        self.assertTrue(result["outcome_matches_expected"])

    def test_run_comp_04_check_row_is_check_rejection(self) -> None:
        with mock.patch.object(
            mir_computational_samples,
            "_run_product_alpha1_check",
            return_value=self._fake_check_rejection_payload(
                "runtime_input.host_output.effect_ref `typed_host_io.write_int` is not declared in package effects"
            ),
        ):
            result = mir_computational_samples.run_sample(
                "comp-04-host-io-internal-transform-negative-undeclared-effect"
            )

        self.assertEqual(result["current_status"], "executable")
        self.assertEqual(result["execution_surface"], "product_alpha1_check")
        self.assertEqual(result["terminal_outcome"], "check_rejection")
        self.assertEqual(result["actual_diagnostic_code"], "SchemaDecode")
        self.assertIn("host_output.effect_ref", result["actual_rejection_detail"])
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
        def fake_run_local(sample_root: Path) -> dict[str, object]:
            sample_path = str(sample_root)
            if sample_path.endswith("add-one-pure-mir"):
                return self._fake_run_local_payload("add_one", 41, 42)
            if sample_path.endswith("host-io-internal-transform/positive"):
                return self._fake_run_local_payload("add_two", 40, 42)
            raise AssertionError(f"unexpected run-local sample root: {sample_path}")

        def fake_check(sample_root: Path) -> dict[str, object]:
            sample_path = str(sample_root)
            if sample_path.endswith("negative-undeclared-effect"):
                return self._fake_check_rejection_payload(
                    "runtime_input.host_output.effect_ref `typed_host_io.write_int` is not declared in package effects"
                )
            if sample_path.endswith("negative-undeclared-failure"):
                return self._fake_check_rejection_payload(
                    "runtime_input.mir_compute.failure_tag `MirComputeRejected` is not declared in computational contract `computational-host-io-internal-transform-negative-undeclared-failure-contract` failure_row"
                )
            if sample_path.endswith("negative-missing-capability"):
                return self._fake_check_rejection_payload(
                    "runtime_input.mir_compute.required_capabilities includes `RunComputationalTransform` which is not declared in package capabilities"
                )
            raise AssertionError(f"unexpected check sample root: {sample_path}")

        with mock.patch.object(
            mir_computational_samples,
            "_run_product_alpha1_local_session",
            side_effect=fake_run_local,
        ), mock.patch.object(
            mir_computational_samples,
            "_run_product_alpha1_check",
            side_effect=fake_check,
        ):
            result = mir_computational_samples.check_all()

        self.assertEqual(result["failed"], [])
        self.assertEqual(result["accepted"], ACCEPTED_EXECUTABLE_SAMPLE_IDS)
        self.assertEqual(
            result["expected_runtime_rejections"],
            EXPECTED_RUNTIME_REJECTION_SAMPLE_IDS,
        )
        self.assertEqual(
            result["expected_check_rejections"],
            EXPECTED_CHECK_REJECTION_SAMPLE_IDS,
        )
        self.assertEqual(
            result["passed"],
            ACCEPTED_EXECUTABLE_SAMPLE_IDS
            + EXPECTED_RUNTIME_REJECTION_SAMPLE_IDS
            + EXPECTED_CHECK_REJECTION_SAMPLE_IDS,
        )
        self.assertEqual(result["planned"], PLANNED_ONLY_SAMPLE_IDS)
        self.assertEqual(result["sample_count"], 15)
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
        def fake_run_local(sample_root: Path) -> dict[str, object]:
            if str(sample_root).endswith("add-one-pure-mir"):
                return self._fake_run_local_payload("add_one", 41, 42)
            return self._fake_run_local_payload("add_two", 40, 42)

        def fake_check(sample_root: Path) -> dict[str, object]:
            if str(sample_root).endswith("negative-undeclared-effect"):
                return self._fake_check_rejection_payload("host_output.effect_ref")
            if str(sample_root).endswith("negative-undeclared-failure"):
                return self._fake_check_rejection_payload(
                    "runtime_input.mir_compute.failure_tag `MirComputeRejected` is not declared in computational contract"
                )
            return self._fake_check_rejection_payload("RunComputationalTransform")

        with mock.patch.object(
            mir_computational_samples,
            "_run_product_alpha1_local_session",
            side_effect=fake_run_local,
        ), mock.patch.object(
            mir_computational_samples,
            "_run_product_alpha1_check",
            side_effect=fake_check,
        ):
            pretty = mir_computational_samples.format_pretty(
                mir_computational_samples.check_all()
            )

        self.assertIn("CHECK-ALL SUMMARY", pretty)
        self.assertIn("planned-only: 0", pretty)
        self.assertIn("expected runtime rejections: 5", pretty)
        self.assertIn("expected check rejections: 3", pretty)

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
