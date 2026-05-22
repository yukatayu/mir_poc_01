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

    def test_record_positive_sample_keeps_record_summary(self) -> None:
        payload = _run_helper("run", "mir-02-record-field-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["record_summaries"][0]["record_name"], "Pair")
        self.assertEqual(payload["actual"]["diagnostic_codes"], [])

    def test_host_boundary_sample_keeps_effect_and_transition_rows(self) -> None:
        payload = _run_helper("run", "mir-02-host-boundary-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            [row["effect_name"] for row in payload["actual"]["effect_summaries"]],
            ["read_int", "write_int"],
        )
        self.assertEqual(len(payload["actual"]["resolved_paths"]), 1)
        self.assertEqual(
            payload["actual"]["transition_summaries"][0]["perform_effects"],
            ["read_int", "write_int"],
        )

    def test_negative_sample_returns_expected_diagnostic_code(self) -> None:
        payload = _run_helper("run", "mir-02-static-array-bounds-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(payload["actual"]["diagnostic_codes"], ["static_index_out_of_bounds"])

    def test_imported_semantic_negative_reports_reachable_module_failures(self) -> None:
        payload = _run_helper("run", "mir-02-imported-semantic-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["return_type_mismatch", "effect_failure_row_missing"],
        )

    def test_check_all_passes_every_row(self) -> None:
        payload = _run_helper("check-all")

        self.assertEqual(payload["failed"], [])
        self.assertEqual(len(payload["passed"]), 12)
