import sys
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha09_devtools as runner  # noqa: E402


class PracticalAlpha09DevtoolsTests(unittest.TestCase):
    def test_list_samples_covers_operational_alpha09_matrix(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [
                "OA09-01",
                "OA09-02",
                "OA09-03",
                "OA09-04",
                "OA09-05",
                "OA09-06",
                "OA09-07",
                "OA09-08",
                "OA09-09",
            ],
        )

    def test_closeout_claims_session_bound_devtools_operational_readiness(self) -> None:
        payload = runner.closeout()
        self.assertTrue(payload["session_bound_devtools_ready"])
        self.assertTrue(payload["event_dag_live_session_present"])
        self.assertTrue(payload["route_trace_present"])
        self.assertTrue(payload["membership_timeline_present"])
        self.assertTrue(payload["witness_relation_present"])
        self.assertTrue(payload["hotplug_lifecycle_present"])
        self.assertTrue(payload["fallback_degradation_present"])
        self.assertTrue(payload["save_load_timeline_present"])
        self.assertTrue(payload["observer_safe_redacted_view_present"])
        self.assertTrue(payload["retention_on_demand_trace_present"])
        self.assertTrue(payload["operational_alpha09_ready"])

    def test_run_sample_oa09_05_includes_accepted_rejected_and_deferred_hotplug(self) -> None:
        payload = runner.run_sample("OA09-05")
        section = payload["devtools_export"]["export_sections"]["hotplug_lifecycle"]
        outcomes = {
            (entry["sample_id"], entry["terminal_outcome"], entry["session_mutated"])
            for entry in section
        }
        self.assertIn(("HP-A1-01", "accepted", True), outcomes)
        self.assertIn(("HP-A1-02", "rejected", False), outcomes)
        self.assertIn(("HP-A1-07", "deferred_detach_minimal_contract", True), outcomes)

    def test_render_html_uses_same_session_export_payload(self) -> None:
        rendered = runner.render_html()
        self.assertEqual(rendered["sample_id"], "OA09-09")
        self.assertIn("event_dag_live_session", rendered["html"])
        self.assertIn("retention_on_demand_trace", rendered["html"])


if __name__ == "__main__":
    unittest.main()
