import sys
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha1_integrated_workflow as runner  # noqa: E402


class PracticalAlpha1IntegratedWorkflowTests(unittest.TestCase):
    def test_list_samples_covers_bounded_workflow_matrix(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [
                "PA1W-01",
                "PA1W-02",
                "PA1W-03",
                "PA1W-04",
                "PA1W-05",
                "PA1W-06",
                "PA1W-07",
                "PA1W-08",
            ],
        )

    def test_closeout_claims_integrated_workflow_but_not_product_ready(self) -> None:
        payload = runner.closeout()
        self.assertTrue(payload["bounded_practical_alpha1_workflow_ready"])
        self.assertTrue(payload["source_frontdoor_present"])
        self.assertTrue(payload["same_session_runtime_present"])
        self.assertTrue(payload["same_session_hotplug_present"])
        self.assertTrue(payload["save_load_present"])
        self.assertTrue(payload["session_devtools_present"])
        self.assertTrue(payload["product_preview_evidence_present"])
        self.assertTrue(payload["negative_guards_present"])
        self.assertFalse(payload["product_public_ready"])

    def test_run_sample_pa1w_03_uses_same_session_hotplug_lifecycle(self) -> None:
        payload = runner.run_sample("PA1W-03")
        lifecycle = payload["workflow_report"]["session_devtools_export"]["export_sections"][
            "hotplug_lifecycle"
        ]
        outcomes = {
            (entry["sample_id"], entry["terminal_outcome"], entry["session_mutated"])
            for entry in lifecycle
        }
        self.assertIn(("HP-A1-01", "accepted", True), outcomes)
        self.assertIn(("HP-A1-02", "rejected", False), outcomes)
        self.assertIn(("HP-A1-07", "deferred_detach_minimal_contract", True), outcomes)

    def test_run_sample_pa1w_08_keeps_public_boundaries_unclaimed(self) -> None:
        payload = runner.run_sample("PA1W-08")
        report = payload["workflow_report"]
        self.assertFalse(report["product_public_ready"])
        self.assertIn("final public viewer / telemetry ABI", report["what_it_does_not_prove"])
        self.assertIn("distributed durable save/load", report["what_it_does_not_prove"])


if __name__ == "__main__":
    unittest.main()
