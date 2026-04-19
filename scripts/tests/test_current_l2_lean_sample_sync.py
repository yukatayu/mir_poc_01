import sys
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_lean_sample_sync as sync  # noqa: E402


class CurrentL2LeanSampleSyncTests(unittest.TestCase):
    def test_current_l2_specs_cover_representative_quartet(self) -> None:
        specs = sync.current_l2_export_specs()
        sample_ids = {spec.sample_id for spec in specs}

        self.assertEqual(
            sample_ids,
            {
                "e5-underdeclared-lineage",
                "p06-typed-proof-owner-handoff",
                "p07-dice-late-join-visible-history",
                "p08-dice-stale-reconnect-refresh",
            },
        )

    def test_foundation_specs_cover_label_model_and_proof_skeleton(self) -> None:
        specs = sync.foundation_specs()
        filenames = {spec.filename for spec in specs}

        self.assertEqual(
            filenames,
            {
                "CurrentL2LabelModel.lean",
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
        self.assertIn("not the final public theorem contract", explanation)


if __name__ == "__main__":
    unittest.main()
