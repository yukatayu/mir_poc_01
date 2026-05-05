import sys
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha08_session_hotplug as runner  # noqa: E402


class PracticalAlpha08SessionHotPlugTests(unittest.TestCase):
    def test_list_samples_covers_operational_alpha08_matrix(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [
                "OA08-01",
                "OA08-02",
                "OA08-03",
                "OA08-04",
                "OA08-05",
                "OA08-06",
                "OA08-07",
                "OA08-08",
                "OA08-09",
                "OA08-10",
            ],
        )

    def test_closeout_claims_same_session_hotplug_operational_readiness(self) -> None:
        payload = runner.closeout()
        self.assertTrue(payload["same_session_hotplug_ready"])
        self.assertTrue(payload["accepted_debug_attach_present"])
        self.assertTrue(payload["auth_contract_update_present"])
        self.assertTrue(payload["rate_limit_preview_present"])
        self.assertTrue(payload["rejected_attach_no_mutation_present"])
        self.assertTrue(payload["object_preview_present"])
        self.assertTrue(payload["unsupported_runtime_fallback_present"])
        self.assertTrue(payload["deferred_detach_boundary_present"])
        self.assertTrue(payload["hotplug_lifecycle_export_present"])
        self.assertTrue(payload["same_session_behavior_change_present"])
        self.assertTrue(payload["operational_alpha08_ready"])

    def test_run_sample_oa08_03_records_auth_activation_cut(self) -> None:
        payload = runner.run_sample("OA08-03")
        report = payload["attach_report"]
        observer = payload["observer_safe_export_after_attach"]

        self.assertEqual(report["terminal_outcome"], "accepted_contract_update")
        self.assertEqual(
            report["activation_cut_ref"], "activation_cut#auth_contract_update"
        )
        self.assertIn("auth_gate_layer", report["active_layers_after"])
        self.assertIn("auth_contract_update_active", observer["runtime_behavior_markers"])


if __name__ == "__main__":
    unittest.main()
