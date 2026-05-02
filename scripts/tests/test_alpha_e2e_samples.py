import json
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import alpha_e2e_samples as runner  # noqa: E402


class AlphaE2ESamplesTests(unittest.TestCase):
    def sidecar_path(self, relative: str) -> Path:
        return REPO_ROOT / relative

    def test_list_samples_matches_implemented_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [row["sample_id"] for row in runner.IMPLEMENTED_ROWS],
        )
        self.assertTrue(all(row["family"] == "alpha-e2e" for row in rows))

    def test_closeout_keeps_stage_e_and_f_incomplete(self) -> None:
        payload = runner.closeout()
        self.assertEqual(payload["planned_only_rows"], ["E2E-08"])
        self.assertFalse(payload["stage_e_complete"])
        self.assertFalse(payload["stage_f_complete"])
        self.assertIn(
            "cargo test -p mirrorea-core --test runtime_substrate",
            payload["validation_floor"],
        )
        self.assertIn(
            "python3 scripts/alpha_e2e_samples.py run E2E-06 --format json",
            payload["validation_floor"],
        )
        self.assertIn(
            "CUT-05 -> E2E-07", payload["negative_coverage_refs"]["invalid_cut_reject"]
        )
        self.assertIn("CUT-04 -> E2E-06", payload["positive_coverage_refs"]["local_save_load"])

    def test_build_e2e10_report_records_placeholder_fallback(self) -> None:
        component_report = {
            "terminal_outcome": "accepted",
            "reason_family": None,
            "representation_state": {
                "selected_package_id": "RuntimePackage[avatar.placeholder.basic@1.0.0]",
                "selected_representation": "static_capsule_placeholder",
                "fallback_applied": True,
                "fallback_reason": "runtime_unavailable",
            },
            "hotplug_skeleton": {"verdict": {"verdict_kind": "rejected"}},
        }
        with mock.patch.object(
            runner, "_load_component_report", return_value=component_report
        ):
            report = runner._build_e2e_10_report()

        self.assertEqual(report["sample_id"], "E2E-10")
        self.assertEqual(report["terminal_outcome"], "accepted")
        self.assertEqual(
            report["evidence_summary"]["requested_package_verdict_kind"], "rejected"
        )
        self.assertTrue(report["evidence_summary"]["fallback_applied"])

    def test_build_e2e07_report_keeps_checker_nonclaim_explicit(self) -> None:
        checker_bridge = {
            "fixture_sample_id": "CUT-05",
            "checker_cluster": "alpha_cut_save_load_static_floor",
            "reason_codes_scope": "alpha-static-floor",
            "status": "matched",
            "matched_reason_rows": [{"kind": "orphan_receive"}],
            "note": "checker-backed invalid distributed cut rejection only; no distributed save/load runtime claim",
        }
        with mock.patch.object(
            runner, "_build_cut05_bridge_report", return_value=checker_bridge
        ):
            report = runner._build_e2e_07_report()

        self.assertEqual(report["sample_id"], "E2E-07")
        self.assertEqual(report["terminal_outcome"], "rejected")
        self.assertEqual(report["reason_family"], "invalid_cut")
        self.assertIn(
            "distributed save/load runtime", "\n".join(report["what_it_does_not_prove"])
        )

    def test_build_e2e06_report_records_local_save_nonclaim(self) -> None:
        save_load_report = {
            "sample_id": "CUT-04",
            "terminal_outcome": "accepted",
            "state_roundtrip_equal": True,
            "saved_owner": "Bob",
            "resumed_owner": "Alice",
            "saved_runtime_snapshot": {
                "membership": {"membership_epoch": 0, "members": {}}
            },
            "restored_visible_history": [
                "publish roll_result(Alice, 4) witness=draw_pub#1",
                "handoff dice_owner Alice -> Bob using witness=draw_pub#1",
            ],
            "visible_history_after_resume": [
                "publish roll_result(Alice, 4) witness=draw_pub#1",
                "handoff dice_owner Alice -> Bob using witness=draw_pub#1",
                "publish roll_result(Bob, 2) witness=draw_pub#2",
                "handoff dice_owner Bob -> Alice using witness=draw_pub#2",
            ],
            "resumed_dispatch_records": [
                {"dispatch_outcome": "accepted", "reason_refs": ["local_save_only"]}
            ],
        }
        with mock.patch.object(
            runner, "_load_component_report", return_value=save_load_report
        ):
            report = runner._build_e2e_06_report()

        self.assertEqual(report["sample_id"], "E2E-06")
        self.assertEqual(report["terminal_outcome"], "accepted")
        self.assertTrue(report["evidence_summary"]["state_roundtrip_equal"])
        self.assertIn(
            "distributed save/load runtime", "\n".join(report["what_it_does_not_prove"])
        )

    def test_run_sample_accepts_exact_sidecar(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        sidecar = json.loads(self.sidecar_path(row["expected_sidecar"]).read_text())
        with mock.patch.object(runner, "_build_report", return_value=sidecar):
            report = runner.run_sample(row["sample_id"])
        self.assertEqual(report, sidecar)

    def test_run_sample_rejects_sidecar_drift(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        sidecar = json.loads(self.sidecar_path(row["expected_sidecar"]).read_text())
        drifted = dict(sidecar)
        drifted["terminal_outcome"] = "rejected"
        with mock.patch.object(runner, "_build_report", return_value=drifted):
            with self.assertRaisesRegex(RuntimeError, "expected terminal_outcome"):
                runner.run_sample(row["sample_id"])

    def test_check_all_collects_failures(self) -> None:
        def fake_run(sample_id: str) -> dict:
            if sample_id == "E2E-02":
                raise RuntimeError("docker unavailable")
            return {"sample_id": sample_id}

        with mock.patch.object(runner, "run_sample", side_effect=fake_run):
            payload = runner.check_all()

        self.assertIn("E2E-01", payload["passed"])
        self.assertEqual(payload["failed"][0]["sample_id"], "E2E-02")
        self.assertFalse(payload["stage_f_complete"])


if __name__ == "__main__":
    unittest.main()
