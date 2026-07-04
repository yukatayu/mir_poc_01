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

    def test_statement_drafts_include_obl021_draft(self) -> None:
        entries = {spec.draft_id: spec for spec in sync.STATEMENT_DRAFTS}
        draft = entries["obl021-elab-determinism-statement-draft"]

        self.assertEqual(
            draft.filename,
            "ElabDeterminismStatementDraft.lean",
        )
        self.assertEqual(
            draft.explanation_path,
            "samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.md",
        )

    def test_statement_drafts_include_obl024_draft(self) -> None:
        entries = {spec.draft_id: spec for spec in sync.STATEMENT_DRAFTS}
        draft = entries["obl024-diagnostic-soundness-statement-draft"]

        self.assertEqual(
            draft.filename,
            "DiagnosticSoundnessStatementDraft.lean",
        )
        self.assertEqual(
            draft.explanation_path,
            "samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md",
        )

    def test_statement_drafts_include_obl020_draft(self) -> None:
        entries = {spec.draft_id: spec for spec in sync.STATEMENT_DRAFTS}
        draft = entries["obl020-step-wf-statement-draft"]

        self.assertEqual(
            draft.filename,
            "StepWFStatementDraft.lean",
        )
        self.assertEqual(
            draft.explanation_path,
            "samples/lean/lab-statements/obl020/StepWFStatementDraft.md",
        )

    def test_statement_drafts_include_obl025_draft(self) -> None:
        entries = {spec.draft_id: spec for spec in sync.STATEMENT_DRAFTS}
        draft = entries["obl025-repair-completeness-statement-draft"]

        self.assertEqual(
            draft.filename,
            "RepairCompletenessStatementDraft.lean",
        )
        self.assertEqual(
            draft.explanation_path,
            "samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md",
        )

    def test_obl024_draft_names_replay_vocabulary_boundary(self) -> None:
        lean_path = (
            sync.REPO_ROOT
            / "samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean"
        )
        explanation_path = (
            sync.REPO_ROOT
            / "samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md"
        )

        lean_text = lean_path.read_text(encoding="utf-8")
        explanation_text = explanation_path.read_text(encoding="utf-8")

        self.assertIn("ReportLocalReplayAnchor : Type u", lean_text)
        self.assertIn("ProofLevelReplayWitness : Type u", lean_text)
        self.assertIn("DiagnosticReportsReplayAnchor", lean_text)
        self.assertIn("ProofLevelReplayRelation", lean_text)
        self.assertIn("ReportLocalReplayAnchorCompatible", lean_text)
        self.assertNotRegex(lean_text, r"(?m)^\s*ReplayWitness\s*:\s*Type u\b")
        self.assertNotIn("TraceLocalReplayFailsExactlyAt", lean_text)
        self.assertIn("V.ReportLocalReplayAnchor ->\n      V.ProofLevelReplayWitness ->", lean_text)
        self.assertIn("env ctx locus input rejection rule bindings anchor replay", lean_text)
        self.assertIn("report-local replay anchor", explanation_text)
        self.assertIn("proof-level replay relation", explanation_text)

    def test_obl024_draft_names_association_vocabulary_boundary(self) -> None:
        lean_path = (
            sync.REPO_ROOT
            / "samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean"
        )
        explanation_path = (
            sync.REPO_ROOT
            / "samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.md"
        )

        lean_text = lean_path.read_text(encoding="utf-8")
        explanation_text = explanation_path.read_text(encoding="utf-8")

        self.assertIn("ReportLocalAssociationKey : Type u", lean_text)
        self.assertIn("ProofLevelAssociationWitness : Type u", lean_text)
        self.assertIn("DiagnosticAssociatedToRejection", lean_text)
        self.assertIn("DiagnosticReportsReportLocalAssociationKey", lean_text)
        self.assertIn("ReportLocalAssociationKeyFor", lean_text)
        self.assertIn("ReportLocalAssociationKeyCompatible", lean_text)
        self.assertIn("DiagnosticAssociationCompatible", lean_text)
        self.assertIn("ProofLevelAssociationRelation", lean_text)
        self.assertNotRegex(lean_text, r"(?m)^\s*AssociationKey\s*:\s*Type u\b")
        self.assertNotIn("DiagnosticAssociationKey", lean_text)
        self.assertNotIn("AssociatedEmittedDiagnostic", lean_text)
        self.assertNotIn("RequestId", lean_text)
        self.assertNotIn("BranchAssociationKey", lean_text)
        self.assertNotIn("FinalAssociationKey", lean_text)
        self.assertNotIn("DecidableEq V.ReportLocalAssociationKey", lean_text)
        self.assertNotIn("Function.Injective", lean_text)
        self.assertNotIn("Function.Surjective", lean_text)
        self.assertIn("env ctx locus input rejection diagnostic key association", lean_text)
        self.assertIn("report-local association key", explanation_text)
        self.assertIn("diagnostic-to-rejection association", explanation_text)
        self.assertIn("proof-level association relation", explanation_text)


if __name__ == "__main__":
    unittest.main()
