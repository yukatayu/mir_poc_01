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
    "proj-03-effectful-sugoroku-positive",
    "proj-03-client-write-authority-negative",
    "proj-03-effect-contract-mismatch-negative",
    "proj-03-payload-shape-mismatch-negative",
    "proj-04-local-role-split-positive",
    "proj-04-client-entry-override-negative",
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
            "scripts/projection_v1_samples.py must exist for P-PROJ-03/P-PROJ-04",
        )

    def test_projection_root_and_matrix_exist(self) -> None:
        projection_root = REPO_ROOT / "samples" / "full-system-v1" / "projection"
        server_client_root = REPO_ROOT / "samples" / "full-system-v1" / "server-client"

        self.assertTrue(projection_root.exists(), "projection sample root is missing")
        self.assertTrue(
            (projection_root / "matrix.json").exists(),
            "projection matrix.json is missing",
        )
        self.assertTrue(server_client_root.exists(), "server-client sample root is missing")
        self.assertTrue(
            (server_client_root / "matrix.json").exists(),
            "server-client matrix.json is missing",
        )

    def test_matrix_declares_projection_rows(self) -> None:
        if projection_v1_samples is None:
            self.fail("projection helper missing")

        matrix = projection_v1_samples.matrix()

        self.assertEqual(matrix["family"], "full_system_v1_projection_backend")
        self.assertEqual(
            [row["sample_id"] for row in matrix["rows"]],
            EXPECTED_SAMPLE_IDS,
        )

    def test_matrix_reports_executable_rows(self) -> None:
        payload = _run_helper("matrix")

        self.assertEqual(payload["sample_count"], 6)
        self.assertEqual(payload["executable_count"], 6)
        self.assertEqual(payload["validation_errors"], [])

    def test_positive_projection_keeps_target_manifest_summary(self) -> None:
        payload = _run_helper("run", "proj-03-effectful-sugoroku-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["target_roles"],
            ["adapter", "client", "server"],
        )
        self.assertIn(
            "ffi.host_output.write_int",
            payload["actual"]["ffi_schema_refs"],
        )
        self.assertEqual(payload["actual"]["packet_schema_count"], 6)
        self.assertEqual(payload["actual"]["ffi_schema_count"], 2)
        self.assertIn(
            "publish_roll",
            payload["actual"]["checked_effect_rows"],
        )
        target_manifests = {
            row["target_id"]: row
            for row in payload["generated_actual"]["target_manifests"]
        }
        self.assertIn("HostWrite", target_manifests["world-server"]["capability_row"])
        self.assertIn("Publisher", target_manifests["world-server"]["capability_row"])
        self.assertEqual(target_manifests["world-client"]["capability_row"], [])
        self.assertEqual(target_manifests["host-adapter"]["capability_row"], [])
        packet_schemas = {
            row["boundary_ref"]: row
            for row in payload["generated_actual"]["packet_schemas"]
        }
        self.assertEqual(packet_schemas["publish_bus"]["request_fields"], [
            {"name": "value", "ty": "Int64"}
        ])

    def test_negative_projection_reports_authority_rejection(self) -> None:
        payload = _run_helper("run", "proj-03-client-write-authority-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["client_write_authority_escalation"],
        )
        self.assertIn(
            "client_output:client_write_authority_escalation",
            payload["actual"]["rejected_rows"],
        )

    def test_negative_projection_reports_payload_shape_rejection(self) -> None:
        payload = _run_helper("run", "proj-03-payload-shape-mismatch-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["boundary_payload_shape_mismatch"],
        )
        self.assertIn(
            "publish_bus:boundary_payload_shape_mismatch",
            payload["actual"]["rejected_rows"],
        )

    def test_negative_projection_reports_effect_contract_rejection(self) -> None:
        payload = _run_helper("run", "proj-03-effect-contract-mismatch-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["boundary_effect_contract_mismatch"],
        )
        self.assertIn(
            "shared_bus:boundary_effect_contract_mismatch",
            payload["actual"]["rejected_rows"],
        )

    def test_positive_local_split_reports_server_and_client_execution(self) -> None:
        payload = _run_helper("run", "proj-04-local-role-split-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(
            payload["actual"]["target_ids"],
            ["host-adapter", "world-client", "world-server"],
        )
        server = payload["actual"]["target_summaries"]["world-server"]
        client = payload["actual"]["target_summaries"]["world-client"]
        adapter = payload["actual"]["target_summaries"]["host-adapter"]
        self.assertEqual(server["execution_kind"], "authoritative_runtime")
        self.assertEqual(client["execution_kind"], "authoritative_runtime")
        self.assertEqual(adapter["execution_kind"], "passive_endpoint")
        self.assertEqual(server["host_output_summaries"], ["Int64(41)"])
        self.assertEqual(server["published_channels"], ["roll"])
        self.assertEqual(client["launched_entry_transitions"], ["render_preview"])
        self.assertEqual(adapter["runtime_session_count"], 0)

    def test_negative_local_split_reports_entry_override_rejection(self) -> None:
        payload = _run_helper("run", "proj-04-client-entry-override-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["entry_transition_not_admitted"],
        )
        self.assertEqual(
            payload["actual"]["rejected_rows"],
            ["world-client:entry_transition_not_admitted"],
        )

    def test_check_all_passes_every_row(self) -> None:
        payload = _run_helper("check-all")

        self.assertEqual(payload["failed"], [])
        self.assertEqual(payload["validation_errors"], [])
        self.assertEqual(payload["passed"], EXPECTED_SAMPLE_IDS)


if __name__ == "__main__":
    unittest.main()
