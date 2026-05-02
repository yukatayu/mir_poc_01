import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import alpha_lifetime_fallback_snapshot as checker  # noqa: E402


class AlphaLifetimeFallbackSnapshotTests(unittest.TestCase):
    def write_json(self, path: Path, payload: dict) -> None:
        path.write_text(json.dumps(payload), encoding="utf-8")

    def sidecar_path(self) -> Path:
        return (
            REPO_ROOT
            / "samples"
            / "alpha"
            / "lifetime-fallback"
            / "lif-13-snapshot_selected_anchor.expected.json"
        )

    def test_sidecar_exposes_expected_snapshot_rows(self) -> None:
        payload = json.loads(self.sidecar_path().read_text(encoding="utf-8"))
        expected_snapshot = payload["expected_snapshot"]
        self.assertEqual(
            expected_snapshot["checked_snapshot_scope"],
            "alpha-snapshot-selected-floor",
        )
        self.assertEqual(
            expected_snapshot["checked_snapshot_rows"][0]["kind"],
            "snapshot_selected_anchor",
        )

    def test_main_reports_matched_rows_from_real_seed_sidecar(self) -> None:
        sidecar_path = self.sidecar_path()

        with tempfile.TemporaryDirectory() as temp_dir:
            artifact_path = Path(temp_dir) / "artifact.json"
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "snapshot_scope": "alpha-snapshot-selected-floor",
                        "snapshot_rows": [
                            {
                                "kind": "snapshot_selected_anchor",
                                "sample_id": "LIF-13",
                                "source_chain": ["FriendHead", "SelfShoulder"],
                                "selected_option": "FriendHead",
                                "appended_fallback": "WorldOrigin",
                                "resulting_chain": ["FriendHead", "WorldOrigin"],
                                "excluded_options": ["SelfShoulder"],
                                "full_chain_inherited": False,
                                "snapshot_freezes_selected_option": True,
                            },
                            {
                                "kind": "plain_ref_boundary_preserved",
                                "sample_id": "LIF-04",
                            },
                        ],
                        "reason_codes_scope": "wrong-floor",
                        "reason_codes": [
                            {"kind": "missing_lineage_evidence", "sample_id": "LIF-05"}
                        ],
                        "acceptance_scope": "wrong-floor",
                        "acceptance_rows": [
                            {
                                "kind": "fallback_chain_canonicalized",
                                "sample_id": "LIF-02",
                            }
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(sidecar_path), str(artifact_path)])

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("cluster: alpha_lifetime_fallback_snapshot_floor", output)
        self.assertIn("status: matched", output)
        self.assertIn("snapshot_selected_anchor", output)
        self.assertNotIn("plain_ref_boundary_preserved", output)
        self.assertNotIn("missing_lineage_evidence", output)
        self.assertNotIn("fallback_chain_canonicalized", output)

    def test_main_reports_scope_mismatch(self) -> None:
        sidecar_path = self.sidecar_path()

        with tempfile.TemporaryDirectory() as temp_dir:
            artifact_path = Path(temp_dir) / "artifact.json"
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "snapshot_scope": "wrong-floor",
                        "snapshot_rows": [
                            {
                                "kind": "snapshot_selected_anchor",
                                "sample_id": "LIF-13",
                                "source_chain": ["FriendHead", "SelfShoulder"],
                                "selected_option": "FriendHead",
                                "appended_fallback": "WorldOrigin",
                                "resulting_chain": ["FriendHead", "WorldOrigin"],
                                "excluded_options": ["SelfShoulder"],
                                "full_chain_inherited": False,
                                "snapshot_freezes_selected_option": True,
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
        self.assertIn("status: scope_mismatch", output)
        self.assertIn("expected_snapshot_scope: alpha-snapshot-selected-floor", output)
        self.assertIn("artifact_snapshot_scope: wrong-floor", output)

    def test_main_reports_missing_expected_rows_when_sidecar_has_no_snapshot(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            sidecar_path = temp_root / "lif-negative.expected.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                sidecar_path,
                {
                    "sample_id": "LIF-05",
                    "family": "lifetime-fallback",
                    "expected_static": {
                        "checked_reason_codes": [{"kind": "missing_lineage_evidence"}]
                    },
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "snapshot_scope": "alpha-snapshot-selected-floor",
                        "snapshot_rows": [
                            {
                                "kind": "snapshot_selected_anchor",
                                "sample_id": "LIF-13",
                                "source_chain": ["FriendHead", "SelfShoulder"],
                                "selected_option": "FriendHead",
                                "appended_fallback": "WorldOrigin",
                                "resulting_chain": ["FriendHead", "WorldOrigin"],
                                "excluded_options": ["SelfShoulder"],
                                "full_chain_inherited": False,
                                "snapshot_freezes_selected_option": True,
                            }
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(sidecar_path), str(artifact_path)])

        self.assertEqual(exit_code, 1)
        self.assertIn(
            "status: sample_expected_snapshot_rows_missing",
            stdout.getvalue(),
        )

    def test_main_reports_mismatch(self) -> None:
        sidecar_path = self.sidecar_path()

        with tempfile.TemporaryDirectory() as temp_dir:
            artifact_path = Path(temp_dir) / "artifact.json"
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "snapshot_scope": "alpha-snapshot-selected-floor",
                        "snapshot_rows": [
                            {
                                "kind": "snapshot_selected_anchor",
                                "sample_id": "LIF-13",
                                "source_chain": ["FriendHead", "SelfShoulder"],
                                "selected_option": "SelfShoulder",
                                "appended_fallback": "WorldOrigin",
                                "resulting_chain": ["SelfShoulder", "WorldOrigin"],
                                "excluded_options": ["FriendHead"],
                                "full_chain_inherited": True,
                                "snapshot_freezes_selected_option": False,
                            }
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = checker.main([str(sidecar_path), str(artifact_path)])

        self.assertEqual(exit_code, 1)
        self.assertIn("status: mismatch", stdout.getvalue())


if __name__ == "__main__":
    unittest.main()
