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
        self.assertEqual(payload["sample_count"], 10)
        self.assertEqual(payload["executable_count"], 10)
        self.assertEqual(payload["validation_errors"], [])

    def test_record_positive_sample_keeps_record_summary(self) -> None:
        payload = _run_helper("run", "mir-02-record-field-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["record_summaries"][0]["record_name"], "Pair")
        self.assertEqual(payload["actual"]["diagnostic_codes"], [])

    def test_runtime_positive_sample_keeps_trace_summary(self) -> None:
        payload = _run_helper("run-runtime", "mir-03-control-flow-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["outcome"], "Accepted")
        self.assertEqual(payload["actual"]["output_summary"], "Int64(10)")
        self.assertIn("while", payload["actual"]["trace_event_kinds"])
        self.assertEqual(payload["actual"]["trace_branch_taken"][-1], "while-break@5")

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
        self.assertEqual(len(payload["runtime"]["passed"]), 10)
        self.assertEqual(len(payload["passed"]), 22)
