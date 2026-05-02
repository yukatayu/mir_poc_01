import json
import subprocess
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPT = REPO_ROOT / "scripts" / "alpha_lifetime_fallback_anchor_handoff.py"
FIXTURE = (
    REPO_ROOT
    / "samples"
    / "alpha"
    / "lifetime-fallback"
    / "lif-11-bird_sparkle_anchor_inheritance.expected.json"
)


class AlphaLifetimeFallbackAnchorHandoffTests(unittest.TestCase):
    def run_helper(self, artifact_payload: dict) -> subprocess.CompletedProcess[str]:
        with tempfile.TemporaryDirectory() as temp_dir:
            artifact_path = Path(temp_dir) / "artifact.json"
            artifact_path.write_text(json.dumps(artifact_payload), encoding="utf-8")
            return subprocess.run(
                ["python3", str(SCRIPT), str(FIXTURE), str(artifact_path)],
                cwd=REPO_ROOT,
                text=True,
                capture_output=True,
                check=False,
            )

    def test_real_sidecar_exposes_expected_anchor_handoff_rows(self) -> None:
        fixture = json.loads(FIXTURE.read_text(encoding="utf-8"))
        expected_anchor_handoff = fixture.get("expected_anchor_handoff")
        self.assertIsInstance(expected_anchor_handoff, dict)
        self.assertEqual(
            expected_anchor_handoff.get("checked_anchor_handoff_scope"),
            "alpha-anchor-handoff-floor",
        )
        self.assertEqual(
            expected_anchor_handoff.get("checked_anchor_handoff_rows"),
            [
                {
                    "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                    "sample_id": "LIF-11",
                    "deleted_object": "Bird",
                    "inheriting_effect": "Sparkle",
                    "source_chain": ["FriendHead", "SelfShoulder"],
                    "appended_fallback": "WorldOrigin",
                    "post_delete_chain": [
                        "FriendHead",
                        "SelfShoulder",
                        "WorldOrigin",
                    ],
                    "object_lifetime_extended": False,
                    "chain_inheritance_explicit": True,
                    "hidden_object_reference_retained": False,
                    "later_degradation_order": [
                        {"when": "FriendHeadInvalid", "select": "SelfShoulder"},
                        {"when": "SelfShoulderInvalid", "select": "WorldOrigin"},
                    ],
                }
            ],
        )

    def test_matching_artifact_returns_matched(self) -> None:
        result = self.run_helper(
            {
                "detached_noncore": {
                    "anchor_handoff_scope": "alpha-anchor-handoff-floor",
                    "anchor_handoff_rows": [
                        {
                            "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                            "sample_id": "LIF-11",
                            "deleted_object": "Bird",
                            "inheriting_effect": "Sparkle",
                            "source_chain": ["FriendHead", "SelfShoulder"],
                            "appended_fallback": "WorldOrigin",
                            "post_delete_chain": [
                                "FriendHead",
                                "SelfShoulder",
                                "WorldOrigin",
                            ],
                            "object_lifetime_extended": False,
                            "chain_inheritance_explicit": True,
                            "hidden_object_reference_retained": False,
                            "later_degradation_order": [
                                {
                                    "when": "FriendHeadInvalid",
                                    "select": "SelfShoulder",
                                },
                                {
                                    "when": "SelfShoulderInvalid",
                                    "select": "WorldOrigin",
                                },
                            ],
                        }
                    ],
                }
            }
        )
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertIn("status: matched", result.stdout)

    def test_wrong_scope_returns_scope_mismatch(self) -> None:
        result = self.run_helper(
            {
                "detached_noncore": {
                    "anchor_handoff_scope": "wrong-floor",
                    "anchor_handoff_rows": [
                        {
                            "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                            "sample_id": "LIF-11",
                            "deleted_object": "Bird",
                            "inheriting_effect": "Sparkle",
                            "source_chain": ["FriendHead", "SelfShoulder"],
                            "appended_fallback": "WorldOrigin",
                            "post_delete_chain": [
                                "FriendHead",
                                "SelfShoulder",
                                "WorldOrigin",
                            ],
                            "object_lifetime_extended": False,
                            "chain_inheritance_explicit": True,
                            "hidden_object_reference_retained": False,
                            "later_degradation_order": [
                                {
                                    "when": "FriendHeadInvalid",
                                    "select": "SelfShoulder",
                                },
                                {
                                    "when": "SelfShoulderInvalid",
                                    "select": "WorldOrigin",
                                },
                            ],
                        }
                    ],
                }
            }
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("status: scope_mismatch", result.stdout)

    def test_mismatched_artifact_returns_mismatch(self) -> None:
        result = self.run_helper(
            {
                "detached_noncore": {
                    "anchor_handoff_scope": "alpha-anchor-handoff-floor",
                    "anchor_handoff_rows": [
                        {
                            "kind": "bird_sparkle_anchor_chain_inherited_after_object_delete",
                            "sample_id": "LIF-11",
                            "deleted_object": "Bird",
                            "inheriting_effect": "Sparkle",
                            "source_chain": ["FriendHead"],
                            "appended_fallback": "WorldOrigin",
                            "post_delete_chain": ["FriendHead", "WorldOrigin"],
                            "object_lifetime_extended": False,
                            "chain_inheritance_explicit": True,
                            "hidden_object_reference_retained": False,
                            "later_degradation_order": [],
                        }
                    ],
                }
            }
        )
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("status: mismatch", result.stdout)
