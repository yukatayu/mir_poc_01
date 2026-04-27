from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import avatar_follow_samples


class AvatarFollowSamplesTests(unittest.TestCase):
    def test_residual_planned_family_contains_only_unpromoted_samples(self) -> None:
        planned_dir = (
            Path(__file__).resolve().parents[2]
            / "samples"
            / "not_implemented"
            / "avatar-fairy-follow"
        )
        sample_names = sorted(path.name for path in planned_dir.glob("*.mir"))

        self.assertEqual(
            sample_names,
            [
                "02_remote_head_not_visible_falls_back_to_local.mir",
                "05_follow_target_reacquired_after_return.mir",
            ],
        )

    def test_list_contains_promoted_representative_slice_samples(self) -> None:
        samples = avatar_follow_samples.list_samples()
        sample_ids = [sample["sample_id"] for sample in samples]

        self.assertEqual(
            sample_ids,
            [
                "01_follow_remote_head_with_local_fallback",
                "03_remote_avatar_leaves_falls_back_to_local",
                "04_invalid_cross_anchor_chain_rejected",
                "06_model_check_no_detached_anchor_observed",
            ],
        )

    def test_follow_remote_head_establishes_explicit_fallback_lineage(self) -> None:
        result = avatar_follow_samples.run_sample(
            "01_follow_remote_head_with_local_fallback"
        )

        self.assertEqual(result["terminal_outcome"], "success")
        self.assertEqual(result["anchor_state"]["attached_anchor"], "remote_head_anchor")
        self.assertEqual(result["anchor_state"]["fallback_anchor"], "local_head_anchor")
        self.assertEqual(
            result["anchor_state"]["lineage"],
            [
                {
                    "kind": "follow",
                    "source": "follow_anchor",
                    "target": "remote_head_anchor",
                },
                {
                    "kind": "fallback",
                    "source": "remote_head_anchor",
                    "target": "local_head_anchor",
                },
            ],
        )
        self.assertIn("fallback_lineage_is_explicit", result["properties_passed"])

    def test_leave_fallback_rejects_stale_anchor(self) -> None:
        result = avatar_follow_samples.run_sample(
            "03_remote_avatar_leaves_falls_back_to_local"
        )

        self.assertEqual(result["terminal_outcome"], "fallback_after_reject")
        self.assertEqual(result["membership_epoch"], 1)
        self.assertEqual(result["anchor_state"]["attached_anchor"], "local_head_anchor")
        self.assertEqual(
            result["anchor_state"]["rejected_anchors"]["remote_head_anchor"],
            "StaleMembershipEpoch",
        )
        self.assertIn("stale_anchor_is_rejected", result["properties_passed"])

    def test_invalid_cross_anchor_chain_is_rejected_without_hidden_repair(self) -> None:
        result = avatar_follow_samples.run_sample(
            "04_invalid_cross_anchor_chain_rejected"
        )

        self.assertEqual(result["static_or_runtime_verdict"], "reject")
        self.assertEqual(result["reason_family"], "invalid_cross_anchor_lineage")
        self.assertTrue(result["no_hidden_repair"])

    def test_model_check_reports_no_detached_anchor_observed(self) -> None:
        result = avatar_follow_samples.run_sample(
            "06_model_check_no_detached_anchor_observed"
        )

        self.assertEqual(result["terminal_outcome"], "model_check_pass")
        self.assertEqual(
            result["model_check"]["property"], "no_detached_anchor_observed"
        )
        self.assertEqual(result["model_check"]["verdict"], "pass")

    def test_anchors_debug_prints_anchor_inventory(self) -> None:
        pretty = avatar_follow_samples.format_pretty(
            avatar_follow_samples.run_sample(
                "01_follow_remote_head_with_local_fallback"
            ),
            debug="anchors",
        )

        self.assertIn("Anchor inventory:", pretty)
        self.assertIn("attached_anchor: remote_head_anchor", pretty)
        self.assertIn("fallback_anchor: local_head_anchor", pretty)

    def test_closeout_reports_debug_modes_and_planned_samples(self) -> None:
        result = avatar_follow_samples.closeout()

        self.assertEqual(result["sample_count"], 4)
        self.assertIn("anchors", result["debug_output_modes"])
        self.assertIn("membership", result["debug_output_modes"])
        self.assertIn("verification", result["debug_output_modes"])
        self.assertEqual(
            result["planned_sample_ids"],
            [
                "02_remote_head_not_visible_falls_back_to_local",
                "05_follow_target_reacquired_after_return",
            ],
        )
        self.assertIn(
            "no_detached_anchor_observed", result["model_check_properties"]
        )

    def test_check_all_passes_for_promoted_avatar_slice(self) -> None:
        result = avatar_follow_samples.check_all()

        self.assertEqual(result["failed"], [])


if __name__ == "__main__":
    unittest.main()
