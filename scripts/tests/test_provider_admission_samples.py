from __future__ import annotations

import importlib
import json
import subprocess
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

try:
    provider_admission_samples = importlib.import_module("provider_admission_samples")
except ModuleNotFoundError:
    provider_admission_samples = None


REPO_ROOT = Path(__file__).resolve().parents[2]
EXPECTED_SAMPLE_IDS = [
    "eng-02-viewer-diagnostic-positive",
    "eng-02-over-capability-negative",
    "eng-02-missing-rollback-negative",
    "eng-02-native-disabled-negative",
    "eng-02-wasm-inventory-positive",
]


def _run_helper(*args: str) -> dict:
    completed = subprocess.run(
        ["python3", "scripts/provider_admission_samples.py", *args, "--format", "json"],
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


class ProviderAdmissionSamplesTests(unittest.TestCase):
    def test_helper_module_exists(self) -> None:
        self.assertIsNotNone(
            provider_admission_samples,
            "scripts/provider_admission_samples.py must exist for P-ENG-02",
        )

    def test_provider_root_and_matrix_exist(self) -> None:
        provider_root = REPO_ROOT / "samples" / "full-system-v1" / "provider-adapter"

        self.assertTrue(provider_root.exists(), "provider-adapter sample root is missing")
        self.assertTrue(
            (provider_root / "matrix.json").exists(),
            "provider-adapter matrix.json is missing",
        )

    def test_matrix_declares_provider_rows(self) -> None:
        if provider_admission_samples is None:
            self.fail("provider admission helper missing")

        matrix = provider_admission_samples.matrix()

        self.assertEqual(matrix["family"], "full_system_v1_provider_admission")
        self.assertEqual(
            [row["sample_id"] for row in matrix["rows"]],
            EXPECTED_SAMPLE_IDS,
        )

    def test_matrix_reports_executable_rows(self) -> None:
        payload = _run_helper("matrix")

        self.assertEqual(payload["sample_count"], 5)
        self.assertEqual(payload["executable_count"], 5)
        self.assertEqual(payload["validation_errors"], [])

    def test_positive_viewer_diagnostic_row_reports_inventory_admission(self) -> None:
        payload = _run_helper("run", "eng-02-viewer-diagnostic-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["provider_id"], "viewer-diagnostic-exporter")
        self.assertEqual(payload["actual"]["target_id"], "diagnostic-adapter")
        self.assertEqual(payload["actual"]["terminal_outcome"], "inventory_admitted")
        self.assertFalse(payload["actual"]["execution_admitted"])
        self.assertEqual(payload["actual"]["diagnostic_codes"], [])
        self.assertEqual(payload["actual"]["local_split_launched_targets"], ["world-client"])

    def test_negative_over_capability_row_reports_rejection(self) -> None:
        payload = _run_helper("run", "eng-02-over-capability-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["provider_over_capability"],
        )

    def test_negative_missing_rollback_row_reports_rejection(self) -> None:
        payload = _run_helper("run", "eng-02-missing-rollback-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["missing_rollback_replay_cut_policy"],
        )

    def test_negative_native_disabled_row_reports_disabled_execution(self) -> None:
        payload = _run_helper("run", "eng-02-native-disabled-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(payload["actual"]["terminal_outcome"], "native_execution_disabled")
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["native_execution_disabled_by_default"],
        )

    def test_positive_wasm_inventory_row_reports_deferred_execution(self) -> None:
        payload = _run_helper("run", "eng-02-wasm-inventory-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["terminal_outcome"], "wasm_inventory_only")
        self.assertFalse(payload["actual"]["execution_admitted"])
        self.assertIn(
            "sandboxed_wasm_execution_deferred",
            payload["actual"]["residual_obligation_codes"],
        )

    def test_check_all_passes_every_row(self) -> None:
        payload = _run_helper("check-all")

        self.assertEqual(payload["failed"], [])
        self.assertEqual(payload["validation_errors"], [])
        self.assertEqual(payload["passed"], EXPECTED_SAMPLE_IDS)


if __name__ == "__main__":
    unittest.main()
