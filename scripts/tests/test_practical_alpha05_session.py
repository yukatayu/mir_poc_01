import sys
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha05_session as runner  # noqa: E402


class PracticalAlpha05SessionTests(unittest.TestCase):
    def test_list_samples_covers_operational_alpha05_matrix(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            ["OA05-01", "OA05-02", "OA05-03", "OA05-04", "OA05-05", "OA05-06", "OA05-07"],
        )

    def test_closeout_claims_session_bound_event_dag_and_observer_safe_export(self) -> None:
        payload = runner.closeout()
        self.assertTrue(payload["session_carrier_ready"])
        self.assertTrue(payload["session_bound_event_dag_present"])
        self.assertTrue(payload["observer_safe_export_present"])
        self.assertTrue(payload["local_save_load_roundtrip_present"])
        self.assertTrue(payload["stale_membership_non_resurrection_present"])
        self.assertTrue(payload["typed_host_io_demo_present"])
        self.assertTrue(payload["operational_alpha05_ready"])

    def test_run_sample_oa05_07_records_add_one_host_receipt(self) -> None:
        payload = runner.run_sample("OA05-07")
        report = payload["host_io_report"]
        observer = payload["observer_safe_export_after_host_io"]

        self.assertEqual(report["adapter_kind"], "add_one")
        self.assertEqual(report["request_payload"], {"kind": "int", "value": 41})
        self.assertEqual(report["response_payload"], {"kind": "int", "value": 42})
        self.assertEqual(report["terminal_outcome"], "accepted")
        self.assertIn("host_response#1", report["session_event_ids_after"])
        self.assertIn("host_io:AddOne(41)->42", observer["host_io_events"])


if __name__ == "__main__":
    unittest.main()
