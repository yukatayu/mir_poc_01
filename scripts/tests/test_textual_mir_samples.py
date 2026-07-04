from __future__ import annotations

import json
import subprocess
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]


def _run_helper(*args: str) -> dict:
    completed = subprocess.run(
        ["python3", "scripts/textual_mir_samples.py", *args, "--format", "json"],
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


class TextualMirSamplesTests(unittest.TestCase):
    def test_matrix_reports_full_alpha_row_set(self) -> None:
        payload = _run_helper("matrix")

        self.assertEqual(payload["family"], "textual_mir_alpha")
        self.assertEqual(payload["sample_count"], 10)
        self.assertEqual(payload["executable_count"], 10)
        self.assertEqual(payload["validation_errors"], [])

    def test_positive_sample_is_accepted(self) -> None:
        payload = _run_helper("run", "mir-01-add-one-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["module_path"], "Computational.AddOne")
        self.assertEqual(payload["actual"]["function_summaries"][0]["statement_kinds"], ["Let", "Return"])
        self.assertNotEqual(payload["actual"]["expr_span_markers"], [])
        self.assertEqual(payload["actual"]["diagnostics"], [])

    def test_raw_parse_report_source_path_is_repo_relative(self) -> None:
        payload = _run_helper("run", "mir-01-add-one-positive")
        serialized = json.dumps(payload, ensure_ascii=False)

        self.assertEqual(
            payload["raw_parse_report"]["source_path"],
            "samples/full-system-v1/computational/add-one-positive/src/add-one.mir",
        )
        self.assertNotIn(str(REPO_ROOT), serialized)

    def test_negative_raw_parse_report_diagnostics_are_repo_relative(self) -> None:
        payload = _run_helper("run", "mir-01-unresolved-import-negative")
        serialized = json.dumps(payload, ensure_ascii=False)

        self.assertTrue(payload["accepted"])
        self.assertNotIn(str(REPO_ROOT), serialized)
        self.assertIn(
            "samples/full-system-v1/computational/unresolved-import-negative/src/unresolved-import.mir",
            payload["raw_parse_report"]["diagnostics"][0]["message"],
        )

    def test_host_boundary_sample_keeps_effect_and_contract_structure(self) -> None:
        payload = _run_helper("run", "mir-01-host-boundary-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            [row["effect_name"] for row in payload["actual"]["effect_summaries"]],
            ["read_int", "write_int"],
        )
        self.assertEqual(
            payload["actual"]["transition_summaries"][0]["contract_clause_kinds"],
            ["Ensure"],
        )
        self.assertNotEqual(payload["actual"]["contract_span_markers"], [])

    def test_negative_sample_returns_expected_diagnostic_code(self) -> None:
        payload = _run_helper("run", "mir-01-unresolved-import-negative")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["diagnostics"][0]["code"], "unresolved_import")

    def test_check_all_passes_every_row(self) -> None:
        payload = _run_helper("check-all")

        self.assertEqual(payload["failed"], [])
        self.assertEqual(len(payload["passed"]), 10)
