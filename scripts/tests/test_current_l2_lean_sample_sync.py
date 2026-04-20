import sys
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_lean_sample_sync as sync  # noqa: E402


class CurrentL2LeanSampleSyncTests(unittest.TestCase):
    def test_current_l2_specs_cover_representative_sample_set(self) -> None:
        specs = sync.current_l2_export_specs()
        sample_ids = {spec.sample_id for spec in specs}

        self.assertEqual(
            sample_ids,
            {
                "e5-underdeclared-lineage",
                "p06-typed-proof-owner-handoff",
                "p10-typed-authorized-fingerprint-declassification",
                "p11-typed-unauthorized-fingerprint-release",
                "p12-typed-classified-fingerprint-publication-block",
                "p15-typed-capture-escape-rejected",
                "p16-typed-remote-call-budget-exceeded",
                "p07-dice-late-join-visible-history",
                "p08-dice-stale-reconnect-refresh",
                "p09-dice-delegated-rng-provider-placement",
                "p13-dice-late-join-missing-publication-witness",
                "p14-dice-late-join-handoff-before-publication",
            },
        )

    def test_foundation_specs_cover_label_model_and_proof_skeleton(self) -> None:
        specs = sync.foundation_specs()
        filenames = {spec.filename for spec in specs}

        self.assertEqual(
            filenames,
            {
                "CurrentL2LabelModel.lean",
                "CurrentL2IfcSecretExamples.lean",
                "CurrentL2FiniteIndexFirstLayer.lean",
                "CurrentL2ProofSkeleton.lean",
            },
        )

    def test_current_l2_explanation_calls_out_sorry_and_scope(self) -> None:
        spec = next(
            spec
            for spec in sync.current_l2_export_specs()
            if spec.sample_id == "p06-typed-proof-owner-handoff"
        )

        explanation = sync.build_current_l2_explanation(spec)

        self.assertIn("sorry", explanation)
        self.assertIn("artifact well-formedness", explanation)
        self.assertIn("これは最終的な public theorem contract", explanation)
        self.assertIn("この Lean ファイルは", explanation)

    def test_foundation_explanation_is_written_in_japanese(self) -> None:
        spec = next(
            spec
            for spec in sync.foundation_specs()
            if spec.filename == "CurrentL2LabelModel.lean"
        )

        explanation = sync.build_foundation_explanation(spec)

        self.assertIn("このファイル", explanation)
        self.assertIn("実際に小さな証明", explanation)

    def test_ifc_foundation_spec_carries_valid_invalid_reusable_facts(self) -> None:
        spec = next(
            spec
            for spec in sync.foundation_specs()
            if spec.filename == "CurrentL2IfcSecretExamples.lean"
        )

        self.assertIn("theorem declassify_preserves_value", spec.source_text)
        self.assertIn(
            "theorem low_to_low_release_without_authority_is_available", spec.source_text
        )
        self.assertIn(
            "theorem unauthorized_live_fingerprint_release_is_impossible",
            spec.source_text,
        )

    def test_finite_index_foundation_spec_carries_reusable_first_layer_facts(self) -> None:
        spec = next(
            spec
            for spec in sync.foundation_specs()
            if spec.filename == "CurrentL2FiniteIndexFirstLayer.lean"
        )

        self.assertIn("theorem outlives_trans", spec.source_text)
        self.assertIn("theorem capture_subset_trans", spec.source_text)
        self.assertIn("def spendRemoteCall", spec.source_text)
        self.assertIn("theorem single_budget_is_exhausted_after_one_call", spec.source_text)

    def test_foundation_explanation_calls_out_valid_and_invalid_patterns(self) -> None:
        spec = next(
            spec
            for spec in sync.foundation_specs()
            if spec.filename == "CurrentL2IfcSecretExamples.lean"
        )

        explanation = sync.build_foundation_explanation(spec)

        self.assertIn("valid pattern がなぜ通るか", explanation)
        self.assertIn("invalid pattern がなぜ witness を持てないか", explanation)

    def test_top_level_readme_is_written_in_japanese(self) -> None:
        readme = sync.build_top_level_readme()

        self.assertIn("このディレクトリ", readme)
        self.assertIn("現在の current-L2 定理ブリッジ", readme)
        self.assertIn("実際に小さな証明", readme)
        self.assertIn("valid pattern がなぜ通るか", readme)
        self.assertIn("invalid pattern がなぜ不可能か", readme)


if __name__ == "__main__":
    unittest.main()
