from __future__ import annotations

import importlib
import json
import subprocess
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

try:
    projection_v1_samples = importlib.import_module("projection_v1_samples")
except ModuleNotFoundError:
    projection_v1_samples = None


REPO_ROOT = Path(__file__).resolve().parents[2]
EXPECTED_SAMPLE_IDS = [
    "proj-02-effectful-sugoroku-positive",
    "proj-02-client-write-authority-negative",
]


def _run_helper(*args: str) -> dict:
    completed = subprocess.run(
        ["python3", "scripts/projection_v1_samples.py", *args, "--format", "json"],
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


class ProjectionV1SamplesTests(unittest.TestCase):
    def test_helper_module_exists(self) -> None:
        self.assertIsNotNone(
            projection_v1_samples,
            "scripts/projection_v1_samples.py must exist for P-PROJ-02",
        )

    def test_projection_root_and_matrix_exist(self) -> None:
        sample_root = (
            REPO_ROOT / "samples" / "full-system-v1" / "projection"
        )
        matrix_path = sample_root / "matrix.json"

        self.assertTrue(sample_root.exists(), "projection sample root is missing")
        self.assertTrue(matrix_path.exists(), "projection matrix.json is missing")

    def test_matrix_declares_projection_rows(self) -> None:
        if projection_v1_samples is None:
            self.fail("projection helper missing")

        matrix = json.loads(projection_v1_samples.MATRIX_PATH.read_text())

        self.assertEqual(matrix["family"], "full_system_v1_projection")
        self.assertEqual(
            [row["sample_id"] for row in matrix["rows"]],
            EXPECTED_SAMPLE_IDS,
        )

    def test_matrix_reports_executable_rows(self) -> None:
        payload = _run_helper("matrix")

        self.assertEqual(payload["sample_count"], 2)
        self.assertEqual(payload["executable_count"], 2)
        self.assertEqual(payload["validation_errors"], [])

    def test_positive_projection_keeps_target_manifest_summary(self) -> None:
        payload = _run_helper("run", "proj-02-effectful-sugoroku-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["target_roles"],
            ["adapter", "client", "server"],
        )
        self.assertIn(
            "ffi.host_output.write_int",
            payload["actual"]["ffi_schema_refs"],
        )
        self.assertIn(
            "publish_roll",
            payload["actual"]["checked_effect_rows"],
        )
        target_manifests = {
            row["target_id"]: row for row in payload["generated_actual"]
        }
        self.assertIn("HostWrite", target_manifests["world-server"]["capability_row"])
        self.assertIn("Publisher", target_manifests["world-server"]["capability_row"])
        self.assertEqual(target_manifests["world-client"]["capability_row"], [])
        self.assertEqual(target_manifests["host-adapter"]["capability_row"], [])

    def test_negative_projection_reports_authority_rejection(self) -> None:
        payload = _run_helper("run", "proj-02-client-write-authority-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["client_write_authority_escalation"],
        )
        self.assertIn(
            "client_output:client_write_authority_escalation",
            payload["actual"]["rejected_rows"],
        )

    def test_check_all_passes_every_row(self) -> None:
        payload = _run_helper("check-all")

        self.assertEqual(payload["failed"], [])
        self.assertEqual(payload["validation_errors"], [])
        self.assertEqual(payload["passed"], EXPECTED_SAMPLE_IDS)


if __name__ == "__main__":
    unittest.main()
