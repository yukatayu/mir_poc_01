import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import alpha_cut_save_load_checker as checker  # noqa: E402


class AlphaCutSaveLoadCheckerTests(unittest.TestCase):
    def write_json(self, path: Path, payload: dict) -> None:
        path.write_text(json.dumps(payload), encoding="utf-8")

    def sidecar_path(self, stem: str) -> Path:
        return (
            REPO_ROOT
            / "samples"
            / "alpha"
            / "cut-save-load"
            / f"{stem}.expected.json"
        )

    def sample_path(self, stem: str) -> Path:
        return (
            REPO_ROOT
            / "samples"
            / "alpha"
            / "cut-save-load"
            / f"{stem}.mir"
        )

    def test_seed_sidecars_expose_expected_checker_rows(self) -> None:
        cases = {
            "cut-05-inconsistent_distributed_snapshot_rejected": "orphan_receive",
            "cut-07-observe_without_publish_rejected": "orphan_observe",
            "cut-08-witness_use_without_create_rejected": "orphan_witness_use",
            "cut-09-hotplug_activation_without_request_rejected": "orphan_hotplug_activation",
            "cut-11-zcycle_checkpoint_invalid": "zcycle_checkpoint_inadmissible",
            "cut-13-durable_cut_deferred_in_mir0": "durable_cut_deferred",
            "cut-14-capability_use_without_grant_rejected": "orphan_capability_use",
            "cut-15-auth_evidence_use_without_create_rejected": "orphan_auth_evidence_use",
        }

        for stem, expected_kind in cases.items():
            payload = json.loads(self.sidecar_path(stem).read_text(encoding="utf-8"))
            checker_rows = payload["expected_static"]["checked_reason_codes"]
            self.assertEqual(checker_rows[0]["kind"], expected_kind)

    def test_main_reports_matched_rows_from_real_seed_sidecar(self) -> None:
        sidecar_path = self.sidecar_path(
            "cut-05-inconsistent_distributed_snapshot_rejected"
        )

        with tempfile.TemporaryDirectory() as temp_dir:
            artifact_path = Path(temp_dir) / "artifact.json"
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes_scope": "alpha-static-floor",
                        "reason_codes": [
                            {
                                "kind": "orphan_receive",
                                "receive_event": "receive_envelope",
                                "missing_predecessor": "send_envelope",
                            },
                            {
                                "kind": "precondition_strengthening",
                                "base_precondition": "member",
                                "layer_precondition": "admin",
                            },
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(sidecar_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("cluster: alpha_cut_save_load_static_floor", output)
        self.assertIn("status: matched", output)
        self.assertIn("orphan_receive", output)
        self.assertNotIn("precondition_strengthening", output)

    def test_main_reports_missing_expected_rows_when_artifact_has_cut_kind(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            sidecar_path = temp_root / "cut-positive.expected.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                sidecar_path,
                {
                    "sample_id": "CUT-04",
                    "family": "cut-save-load",
                    "expected": "load restores local state",
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes_scope": "alpha-static-floor",
                        "reason_codes": [
                            {
                                "kind": "orphan_observe",
                                "observe_event": "observe",
                                "missing_predecessor": "publish",
                            }
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(sidecar_path), str(artifact_path)])

        self.assertEqual(exit_code, 1)
        output = stdout.getvalue()
        self.assertIn("status: sample_expected_reason_rows_missing", output)
        self.assertIn("orphan_observe", output)

    def test_main_reports_matched_rows_for_cut11_zcycle_kind(self) -> None:
        sidecar_path = self.sidecar_path("cut-11-zcycle_checkpoint_invalid")
        payload = json.loads(sidecar_path.read_text(encoding="utf-8"))
        fixture_row = payload["expected_static"]["checked_reason_codes"][0]

        with tempfile.TemporaryDirectory() as temp_dir:
            artifact_path = Path(temp_dir) / "artifact.json"
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "reason_codes_scope": "alpha-static-floor",
                        "reason_codes": [
                            fixture_row,
                            {
                                "kind": "precondition_strengthening",
                                "base_precondition": "member",
                                "layer_precondition": "admin",
                            },
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(sidecar_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("status: matched", output)
        self.assertIn(fixture_row["kind"], output)

    def test_cut13_seed_matches_sample_purpose(self) -> None:
        stem = "cut-13-durable_cut_deferred_in_mir0"
        payload = json.loads(self.sidecar_path(stem).read_text(encoding="utf-8"))
        sample_text = self.sample_path(stem).read_text(encoding="utf-8")
        row = payload["expected_static"]["checked_reason_codes"][0]

        self.assertEqual(row["kind"], "durable_cut_deferred")
        self.assertEqual(row["construct"], "durable_cut")
        self.assertIn(f"Purpose: use {row['construct']} in Mir-0", sample_text)


if __name__ == "__main__":
    unittest.main()
