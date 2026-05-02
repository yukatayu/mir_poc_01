import contextlib
import io
import json
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_family_anchor_handoff_support as support  # noqa: E402


class CurrentL2FamilyAnchorHandoffSupportTests(unittest.TestCase):
    def write_json(self, path: Path, payload: dict) -> None:
        path.write_text(json.dumps(payload), encoding="utf-8")

    def test_run_family_anchor_handoff_reports_matched_filtered_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_anchor_handoff": {
                        "verdict": "valid",
                        "checked_anchor_handoff_scope": "alpha-anchor-handoff-floor",
                        "checked_anchor_handoff_rows": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 1,
                            },
                            {"kind": "other_kind", "value": 2},
                        ],
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "anchor_handoff_scope": "alpha-anchor-handoff-floor",
                        "anchor_handoff_rows": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 1,
                            },
                            {"kind": "other_kind", "value": 2},
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_anchor_handoff_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_anchor_handoff_cluster",
                    description="alpha anchor handoff helper",
                    kinds={"bird_sparkle_anchor_chain_inherited_after_object_delete"},
                    missing_status="sample_expected_anchor_handoff_rows_missing",
                    expected_scope="alpha-anchor-handoff-floor",
                )

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("cluster: alpha_anchor_handoff_cluster", output)
        self.assertIn("status: matched", output)
        self.assertIn(
            "bird_sparkle_anchor_chain_inherited_after_object_delete",
            output,
        )
        self.assertNotIn("other_kind", output)

    def test_run_family_anchor_handoff_reports_missing_fixture_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(fixture_path, {"sample_id": "LIF-11"})
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "anchor_handoff_scope": "alpha-anchor-handoff-floor",
                        "anchor_handoff_rows": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 1,
                            }
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_anchor_handoff_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_anchor_handoff_cluster",
                    description="alpha anchor handoff helper",
                    kinds={"bird_sparkle_anchor_chain_inherited_after_object_delete"},
                    missing_status="sample_expected_anchor_handoff_rows_missing",
                    expected_scope="alpha-anchor-handoff-floor",
                )

        self.assertEqual(exit_code, 1)
        self.assertIn(
            "status: sample_expected_anchor_handoff_rows_missing",
            stdout.getvalue(),
        )

    def test_run_family_anchor_handoff_reports_out_of_scope(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_anchor_handoff": {
                        "checked_anchor_handoff_rows": [
                            {"kind": "other_kind", "value": 2}
                        ]
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "anchor_handoff_scope": "alpha-anchor-handoff-floor",
                        "anchor_handoff_rows": [{"kind": "other_kind", "value": 2}],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_anchor_handoff_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_anchor_handoff_cluster",
                    description="alpha anchor handoff helper",
                    kinds={"bird_sparkle_anchor_chain_inherited_after_object_delete"},
                    missing_status="sample_expected_anchor_handoff_rows_missing",
                    expected_scope="alpha-anchor-handoff-floor",
                )

        self.assertEqual(exit_code, 0)
        self.assertIn("status: out_of_scope", stdout.getvalue())

    def test_run_family_anchor_handoff_reports_scope_mismatch(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_anchor_handoff": {
                        "checked_anchor_handoff_scope": "alpha-anchor-handoff-floor",
                        "checked_anchor_handoff_rows": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 1,
                            }
                        ],
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "anchor_handoff_scope": "wrong-floor",
                        "anchor_handoff_rows": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 1,
                            }
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_anchor_handoff_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_anchor_handoff_cluster",
                    description="alpha anchor handoff helper",
                    kinds={"bird_sparkle_anchor_chain_inherited_after_object_delete"},
                    missing_status="sample_expected_anchor_handoff_rows_missing",
                    expected_scope="alpha-anchor-handoff-floor",
                )

        self.assertEqual(exit_code, 1)
        output = stdout.getvalue()
        self.assertIn("status: scope_mismatch", output)
        self.assertIn(
            "expected_anchor_handoff_scope: alpha-anchor-handoff-floor",
            output,
        )
        self.assertIn("artifact_anchor_handoff_scope: wrong-floor", output)

    def test_run_family_anchor_handoff_requires_fixture_scope_when_rows_exist(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_anchor_handoff": {
                        "checked_anchor_handoff_rows": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 1,
                            }
                        ]
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "anchor_handoff_scope": "alpha-anchor-handoff-floor",
                        "anchor_handoff_rows": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 1,
                            }
                        ],
                    }
                },
            )

            with self.assertRaisesRegex(
                ValueError,
                "fixture checked_anchor_handoff_scope is required",
            ):
                support.run_family_anchor_handoff_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_anchor_handoff_cluster",
                    description="alpha anchor handoff helper",
                    kinds={"bird_sparkle_anchor_chain_inherited_after_object_delete"},
                    missing_status="sample_expected_anchor_handoff_rows_missing",
                    expected_scope="alpha-anchor-handoff-floor",
                )

    def test_run_family_anchor_handoff_reports_mismatch(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_anchor_handoff": {
                        "checked_anchor_handoff_scope": "alpha-anchor-handoff-floor",
                        "checked_anchor_handoff_rows": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 1,
                            }
                        ],
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "anchor_handoff_scope": "alpha-anchor-handoff-floor",
                        "anchor_handoff_rows": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 2,
                            }
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_anchor_handoff_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_anchor_handoff_cluster",
                    description="alpha anchor handoff helper",
                    kinds={"bird_sparkle_anchor_chain_inherited_after_object_delete"},
                    missing_status="sample_expected_anchor_handoff_rows_missing",
                    expected_scope="alpha-anchor-handoff-floor",
                )

        self.assertEqual(exit_code, 1)
        self.assertIn("status: mismatch", stdout.getvalue())

    def test_run_family_anchor_handoff_ignores_other_carriers(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_path = temp_root / "fixture.json"
            artifact_path = temp_root / "artifact.json"
            self.write_json(
                fixture_path,
                {
                    "expected_anchor_handoff": {
                        "checked_anchor_handoff_scope": "alpha-anchor-handoff-floor",
                        "checked_anchor_handoff_rows": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 1,
                            }
                        ],
                    }
                },
            )
            self.write_json(
                artifact_path,
                {
                    "detached_noncore": {
                        "anchor_handoff_scope": "alpha-anchor-handoff-floor",
                        "anchor_handoff_rows": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 1,
                            }
                        ],
                        "reason_codes_scope": "wrong-floor",
                        "reason_codes": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 999,
                            }
                        ],
                        "acceptance_scope": "wrong-floor",
                        "acceptance_rows": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 998,
                            }
                        ],
                        "snapshot_scope": "wrong-floor",
                        "snapshot_rows": [
                            {
                                "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                                "value": 997,
                            }
                        ],
                    }
                },
            )

            stdout = io.StringIO()
            with contextlib.redirect_stdout(stdout):
                exit_code = support.run_family_anchor_handoff_checker(
                    argv=[str(fixture_path), str(artifact_path)],
                    cluster_name="alpha_anchor_handoff_cluster",
                    description="alpha anchor handoff helper",
                    kinds={"bird_sparkle_anchor_chain_inherited_after_object_delete"},
                    missing_status="sample_expected_anchor_handoff_rows_missing",
                    expected_scope="alpha-anchor-handoff-floor",
                )

        self.assertEqual(exit_code, 0)
        output = stdout.getvalue()
        self.assertIn("status: matched", output)
        self.assertNotIn("999", output)
        self.assertNotIn("998", output)
        self.assertNotIn("997", output)
