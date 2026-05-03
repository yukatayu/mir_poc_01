import json
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import alpha_local_runtime_samples as runner  # noqa: E402


class AlphaLocalRuntimeSamplesTests(unittest.TestCase):
    def sidecar_path(self, relative: str) -> Path:
        return REPO_ROOT / relative

    def test_list_samples_exposes_runtime_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual([row["sample_id"] for row in rows], ["LR-01", "LR-02"])
        self.assertTrue(all(row["family"] == "alpha-local-runtime" for row in rows))

    def test_closeout_reports_stage_b_support_subset(self) -> None:
        payload = runner.closeout()
        self.assertEqual(payload["implemented_rows"], ["LR-01", "LR-02"])
        self.assertEqual(payload["stage_b_support_rows"], ["CUT-04", "CUT-17"])
        self.assertIn(
            "cargo test -p mirrorea-core --test runtime_substrate",
            payload["validation_floor"],
        )
        self.assertIn(
            "python3 scripts/alpha_local_runtime_samples.py stage-b-closeout --format json",
            payload["validation_floor"],
        )
        self.assertFalse(payload["active_root_promoted"])
        self.assertFalse(payload["parser_runtime_bridge_claimed"])

    def test_run_sample_accepts_positive_runtime_report(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        sidecar = json.loads(self.sidecar_path(row["expected_sidecar"]).read_text())
        report = {
            "sample_id": "LR-01",
            "runtime_scope": "alpha_local_runtime_stage_b_narrow",
            "dispatch_records": [{"dispatch_outcome": "accepted"}],
            "event_dag": {
                "edges": [
                    {"relation": "publication_order"},
                    {"relation": "witness_order"},
                    {"relation": "handoff_order"},
                ]
            },
            "current_owner": "Bob",
            "visible_history": ["handoff Alice -> Bob"],
            "terminal_outcome": "accepted",
            "retained_later_refs": [
                "layer_insertion_runtime",
                "network_docker_runtime",
                "runtime_package_avatar_admission",
                "distributed_save_load",
                "final_public_abi",
            ],
        }
        with mock.patch.object(runner, "_build_runtime_report", return_value=report):
            observed = runner.run_sample("LR-01")
        self.assertEqual(observed, report)

    def test_run_sample_rejects_missing_relation(self) -> None:
        report = {
            "sample_id": "LR-01",
            "runtime_scope": "alpha_local_runtime_stage_b_narrow",
            "dispatch_records": [{"dispatch_outcome": "accepted"}],
            "event_dag": {"edges": [{"relation": "publication_order"}]},
            "current_owner": "Bob",
            "visible_history": ["handoff Alice -> Bob"],
            "terminal_outcome": "accepted",
            "retained_later_refs": [
                "layer_insertion_runtime",
                "network_docker_runtime",
                "runtime_package_avatar_admission",
                "distributed_save_load",
                "final_public_abi",
            ],
        }
        with mock.patch.object(runner, "_build_runtime_report", return_value=report):
            with self.assertRaisesRegex(RuntimeError, "missing event DAG relations"):
                runner.run_sample("LR-01")

    def test_run_sample_rejects_missing_reason_ref(self) -> None:
        report = {
            "sample_id": "LR-02",
            "runtime_scope": "alpha_local_runtime_stage_b_narrow",
            "dispatch_records": [{"dispatch_outcome": "rejected_stale_membership", "reason_refs": []}],
            "event_dag": {"edges": []},
            "current_owner": "Alice",
            "visible_history": [],
            "terminal_outcome": "rejected",
            "reason_family": "membership_freshness",
            "retained_later_refs": [
                "layer_insertion_runtime",
                "network_docker_runtime",
                "runtime_package_avatar_admission",
                "distributed_save_load",
                "final_public_abi",
            ],
        }
        with mock.patch.object(runner, "_build_runtime_report", return_value=report):
            with self.assertRaisesRegex(RuntimeError, "missing required reason ref"):
                runner.run_sample("LR-02")

    def test_stage_b_closeout_requires_local_and_cut_subset(self) -> None:
        with (
            mock.patch.object(
                runner,
                "check_all",
                return_value={"sample_count": 2, "passed": ["LR-01", "LR-02"], "failed": []},
            ),
            mock.patch.object(
                runner.cut_save_load_samples,
                "check_all",
                return_value={"sample_count": 2, "passed": ["CUT-04", "CUT-17"], "failed": []},
            ),
        ):
            payload = runner.stage_b_closeout()
        self.assertTrue(payload["stage_b_complete"])
        self.assertFalse(payload["distributed_save_load_claimed"])
        self.assertFalse(payload["cut_family_complete"])

    def test_stage_b_closeout_surfaces_failures(self) -> None:
        with (
            mock.patch.object(
                runner,
                "check_all",
                return_value={
                    "sample_count": 2,
                    "passed": ["LR-01"],
                    "failed": [{"sample_id": "LR-02", "error": "boom"}],
                },
            ),
            mock.patch.object(
                runner.cut_save_load_samples,
                "check_all",
                return_value={"sample_count": 2, "passed": ["CUT-04", "CUT-17"], "failed": []},
            ),
        ):
            payload = runner.stage_b_closeout()
        self.assertFalse(payload["stage_b_complete"])
        self.assertEqual(payload["local_runtime_check"]["failed"][0]["sample_id"], "LR-02")


if __name__ == "__main__":
    unittest.main()
