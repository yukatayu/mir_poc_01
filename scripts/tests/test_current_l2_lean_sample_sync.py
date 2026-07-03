from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import current_l2_lean_sample_sync as sync


class CurrentL2LeanSampleSyncTests(unittest.TestCase):
    def test_sanitize_module_name_handles_numeric_prefix(self) -> None:
        self.assertEqual(
            sync.sanitize_module_name("01_authorized_declassification"),
            "CleanNearEnd.S01.Authorized.Declassification",
        )

    def test_theorem_name_is_stable(self) -> None:
        self.assertEqual(
            sync.theorem_name("03_broken_mutex_counterexample"),
            "s03_broken_mutex_counterexample__alpha_ready_subject",
        )

    def test_render_lean_stub_mentions_theorem(self) -> None:
        text = sync.render_lean_stub("01_authorized_declassification")
        self.assertIn("alpha_ready_subject", text)
        self.assertIn("namespace CleanNearEnd", text)

    def test_repo_relative_source_path_normalizes_repo_absolute_path(self) -> None:
        path = (
            sync.REPO_ROOT
            / "crates"
            / "mir-runtime"
            / "../../samples/clean-near-end/typing/01_authorized_declassification.mir"
        )

        self.assertEqual(
            sync.repo_relative_source_path(str(path)),
            "samples/clean-near-end/typing/01_authorized_declassification.mir",
        )

    def test_repo_relative_source_path_preserves_external_path(self) -> None:
        self.assertEqual(
            sync.repo_relative_source_path("/tmp/outside-sample.mir"),
            "/tmp/outside-sample.mir",
        )

    def test_statement_drafts_include_obl001_draft(self) -> None:
        entries = {spec.draft_id: spec for spec in sync.STATEMENT_DRAFTS}
        draft = entries["obl001-thm001-statement-draft"]

        self.assertEqual(
            draft.filename,
            "THM001StatementDraft.lean",
        )
        self.assertEqual(
            draft.explanation_path,
            "samples/lean/lab-statements/obl001/THM001StatementDraft.md",
        )


if __name__ == "__main__":
    unittest.main()
