from __future__ import annotations

import re
import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import current_l2_lean_sample_sync as sync


def lean_def_body(text: str, name: str) -> str:
    match = re.search(
        rf"(?ms)^def\s+{re.escape(name)}\b(?P<body>.*?)(?=^def\s|\Z)",
        text,
    )
    if match is None:
        raise ValueError(f"Lean definition not found: {name}")
    return match.group("body")


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
        self.assertNotIn("BranchId", lean_text)
        self.assertNotIn("BranchAssociationKey", lean_text)
        self.assertNotIn("DiagnosticBranchAssociationKey", lean_text)
        self.assertNotIn("FinalAssociationKey", lean_text)
        self.assertNotIn("AssociationKeyABI", lean_text)
        for typeclass_name in ("DecidableEq", "BEq", "Hashable", "Ord", "LT", "LE"):
            self.assertNotRegex(
                lean_text,
                rf"{typeclass_name}\s*\(?\s*V\.ReportLocalAssociationKey\b",
            )
        for relation_name in ("Function.Injective", "Function.Surjective"):
            self.assertNotIn(relation_name, lean_text)
        self.assertNotRegex(
            lean_text,
            r"V\.DiagnosticBranch\s*->\s*V\.ReportLocalAssociationKey",
        )
        self.assertNotRegex(
            lean_text,
            r"V\.ReportLocalAssociationKey\s*->\s*V\.DiagnosticBranch",
        )
        self.assertNotRegex(
            lean_text,
            r"(?i)(ReportLocalAssociationKey[A-Za-z]*(Unique|Uniqueness|Collision|Stable|Determines)|"
            r"(Unique|Uniqueness|Collision|Stable|Determines)[A-Za-z]*ReportLocalAssociationKey)",
        )
        self.assertIn("CurrentEvidenceBoundary", lean_text)
        self.assertIn("CoveredDiagnosticSoundnessCase", lean_text)
        self.assertIn("Rejects", lean_text)
        self.assertIn("env ctx locus input rejection diagnostic key association", lean_text)
        self.assertIn("report-local association key", explanation_text)
        self.assertIn("diagnostic-to-rejection association", explanation_text)
        self.assertIn("proof-level association relation", explanation_text)
        self.assertIn("not semantic association by key equality", explanation_text)
        self.assertIn("not a branch-local association key", explanation_text)

    def test_obl025_draft_names_repair_completeness_boundary(self) -> None:
        lean_path = (
            sync.REPO_ROOT
            / "samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean"
        )
        explanation_path = (
            sync.REPO_ROOT
            / "samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.md"
        )

        lean_text = lean_path.read_text(encoding="utf-8")
        explanation_text = explanation_path.read_text(encoding="utf-8")

        self.assertIn("EligibleSingleEditRepair", lean_text)
        self.assertIn("SuggestionCoversWitness", lean_text)
        self.assertIn("RepairWitnessCoversRejectedGap", lean_text)
        self.assertIn("SuggestedRepairCoversRejectedGap", lean_text)
        self.assertIn("CompleteGroupedMultiEditRepair", lean_text)
        self.assertIn("PartialGuidanceNonCoverage", lean_text)
        self.assertIn("BranchLocalRepairNonCoverage", lean_text)
        self.assertIn("BranchLocalSuggestionNonCoverage", lean_text)
        self.assertIn("¬ P.GroupedMultiEditRepairWitness", lean_text)
        self.assertIn("¬ P.SuggestedRepairPartialGuidance", lean_text)
        self.assertRegex(
            lean_text,
            r"forall\s+witness,\s+¬\s+SuggestionCoversWitness",
        )
        for misleading_name in (
            "RepairRanking",
            "RankedRepair",
            "AllRepairs",
            "MinimalRepair",
            "OptimalRepair",
            "FinalRepair",
            "RepairABI",
            "RepairJson",
            "PlaceholderRepair",
            "NonEmptyPlaceholder",
        ):
            self.assertNotIn(misleading_name, lean_text)
        self.assertIn("not a placeholder non-empty repair list", explanation_text)
        self.assertIn("not repair ranking", explanation_text)
        self.assertIn("not all possible repairs", explanation_text)
        self.assertIn("branch-local guidance is not whole-gap coverage", explanation_text)

        repair_completeness_body = lean_def_body(
            lean_text,
            "RepairCompletenessForRejection",
        )
        self.assertRegex(
            repair_completeness_body,
            r"\(exists witness,\s+EligibleSingleEditRepair P fragment rejection witness\)\s*->\s+"
            r"exists diagnostic suggestion witness",
        )
        self.assertIn("P.SuggestedRepairOf diagnostic suggestion", repair_completeness_body)
        self.assertIn(
            "SuggestionCoversWitness P rejection suggestion witness",
            repair_completeness_body,
        )
        self.assertIn(
            "P.AssociatedEmittedDiagnostic input rejection diagnostic",
            repair_completeness_body,
        )

        eligible_body = lean_def_body(lean_text, "EligibleSingleEditRepair")
        self.assertIn("P.SingleEditRepairWitness fragment rejection witness", eligible_body)
        self.assertIn("P.RepairWitnessInDeclaredFragment fragment witness", eligible_body)
        self.assertIn("P.RepairWitnessCoversRejectedGap rejection witness", eligible_body)
        self.assertIn("¬ P.GroupedMultiEditRepairWitness", eligible_body)

        suggestion_body = lean_def_body(lean_text, "SuggestionCoversWitness")
        self.assertIn(
            "P.SuggestedRepairRealizesCompatibleWitness suggestion witness",
            suggestion_body,
        )
        self.assertIn(
            "P.SuggestedRepairCompleteLocalRepair rejection suggestion",
            suggestion_body,
        )
        self.assertIn("¬ P.SuggestedRepairPartialGuidance", suggestion_body)
        self.assertIn("P.SuggestedRepairCoversRejectedGap rejection suggestion", suggestion_body)

        grouped_body = lean_def_body(lean_text, "CompleteGroupedMultiEditRepair")
        self.assertIn("¬ P.SingleEditRepairWitness", grouped_body)

        partial_body = lean_def_body(lean_text, "PartialGuidanceNonCoverage")
        self.assertIn("¬ P.SuggestedRepairCompleteLocalRepair", partial_body)
        self.assertRegex(
            partial_body,
            r"forall\s+witness,\s+¬\s+SuggestionCoversWitness",
        )

        branch_repair_body = lean_def_body(lean_text, "BranchLocalRepairNonCoverage")
        self.assertIn("¬ P.RepairWitnessCoversRejectedGap", branch_repair_body)
        self.assertIn("¬ EligibleSingleEditRepair", branch_repair_body)

        branch_suggestion_body = lean_def_body(
            lean_text,
            "BranchLocalSuggestionNonCoverage",
        )
        self.assertIn("¬ P.SuggestedRepairCoversRejectedGap", branch_suggestion_body)
        self.assertRegex(
            branch_suggestion_body,
            r"forall\s+witness,\s+¬\s+SuggestionCoversWitness",
        )


if __name__ == "__main__":
    unittest.main()
