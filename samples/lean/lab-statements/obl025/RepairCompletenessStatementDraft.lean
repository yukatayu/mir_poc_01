/-!
LAB-only OBL-025 repair-completeness statement-shape draft.

This file checks that Line-1 diagnostic repair completeness can be expressed as
Lean propositions without importing final MirCore diagnostic datatypes, repair
payload JSON, edit scripts, ranking semantics, conformance evidence, or proof
obligation status changes.
-/

namespace MirCore.Lab.OBL025.StatementDraft

universe u

structure Vocab where
  Env : Type u
  Ctx : Type u
  Locus : Type u
  Line1Input : Type u
  Line1Rejection : Type u
  Diagnostic : Type u
  StatementFragment : Type u
  RepairWitness : Type u
  SuggestedRepair : Type u
  DiagnosticFamily : Type u
  MissingEvidenceKind : Type u
  FailedPremise : Type u
  BlameTarget : Type u

structure Pred (V : Vocab.{u}) where
  Line1InputWellScoped :
    V.Env -> V.Ctx -> V.Locus -> V.Line1Input -> Prop
  CurrentEvidenceBoundary :
    V.StatementFragment -> Prop
  CoveredLine1RepairCase :
    V.StatementFragment -> V.Line1Input -> V.Line1Rejection -> Prop
  Line1Rejected :
    V.Env -> V.Ctx -> V.Locus -> V.Line1Input -> V.Line1Rejection -> Prop
  InDeclaredStatementFragment :
    V.StatementFragment -> V.Line1Input -> V.Line1Rejection -> Prop
  RejectionDiagnosticFamily :
    V.Line1Rejection -> V.DiagnosticFamily -> Prop
  RejectionMissingEvidence :
    V.Line1Rejection -> V.MissingEvidenceKind -> Prop
  RejectionFailedPremise :
    V.Line1Rejection -> V.FailedPremise -> Prop
  BlameTargetOf :
    V.Line1Rejection -> V.BlameTarget -> Prop
  AssociatedEmittedDiagnostic :
    V.Line1Input -> V.Line1Rejection -> V.Diagnostic -> Prop
  SingleEditRepairWitness :
    V.StatementFragment -> V.Line1Rejection -> V.RepairWitness -> Prop
  SetInsertionRepairWitness :
    V.StatementFragment -> V.Line1Rejection -> V.RepairWitness -> Prop
  GroupedMultiEditRepairWitness :
    V.StatementFragment -> V.Line1Rejection -> V.RepairWitness -> Prop
  RepairWitnessInDeclaredFragment :
    V.StatementFragment -> V.RepairWitness -> Prop
  RepairWitnessCoversRejectedGap :
    V.Line1Rejection -> V.RepairWitness -> Prop
  RepairWitnessMatchesDiagnosticFamily :
    V.RepairWitness -> V.DiagnosticFamily -> Prop
  RepairWitnessMatchesMissingEvidence :
    V.RepairWitness -> V.MissingEvidenceKind -> Prop
  RepairWitnessDischargesLocalPremise :
    V.RepairWitness -> V.FailedPremise -> Prop
  RepairWitnessTargetsBlame :
    V.RepairWitness -> V.BlameTarget -> Prop
  SuggestedRepairOf :
    V.Diagnostic -> V.SuggestedRepair -> Prop
  SuggestedRepairRealizesCompatibleWitness :
    V.SuggestedRepair -> V.RepairWitness -> Prop
  SuggestedRepairCoversRejectedGap :
    V.Line1Rejection -> V.SuggestedRepair -> Prop
  SuggestedRepairCompleteLocalRepair :
    V.Line1Rejection -> V.SuggestedRepair -> Prop
  SuggestedRepairPartialGuidance :
    V.Line1Rejection -> V.SuggestedRepair -> Prop
  SuggestedRepairMatchesDiagnosticFamily :
    V.SuggestedRepair -> V.DiagnosticFamily -> Prop
  SuggestedRepairMatchesMissingEvidence :
    V.SuggestedRepair -> V.MissingEvidenceKind -> Prop
  SuggestedRepairDischargesLocalPremise :
    V.SuggestedRepair -> V.FailedPremise -> Prop
  SuggestedRepairTargetsBlame :
    V.SuggestedRepair -> V.BlameTarget -> Prop

def EligibleSingleEditRepair
    {V : Vocab.{u}}
    (P : Pred V)
    (fragment : V.StatementFragment)
    (rejection : V.Line1Rejection)
    (witness : V.RepairWitness) : Prop :=
  exists family missing premise target,
    P.RejectionDiagnosticFamily rejection family /\
    P.RejectionMissingEvidence rejection missing /\
    P.RejectionFailedPremise rejection premise /\
    P.BlameTargetOf rejection target /\
    P.SingleEditRepairWitness fragment rejection witness /\
    ¬ P.GroupedMultiEditRepairWitness fragment rejection witness /\
    P.RepairWitnessInDeclaredFragment fragment witness /\
    P.RepairWitnessCoversRejectedGap rejection witness /\
    P.RepairWitnessMatchesDiagnosticFamily witness family /\
    P.RepairWitnessMatchesMissingEvidence witness missing /\
    P.RepairWitnessDischargesLocalPremise witness premise /\
    P.RepairWitnessTargetsBlame witness target

def SuggestionCoversWitness
    {V : Vocab.{u}}
    (P : Pred V)
    (rejection : V.Line1Rejection)
    (suggestion : V.SuggestedRepair)
    (witness : V.RepairWitness) : Prop :=
  exists family missing premise target,
    P.RejectionDiagnosticFamily rejection family /\
    P.RejectionMissingEvidence rejection missing /\
    P.RejectionFailedPremise rejection premise /\
    P.BlameTargetOf rejection target /\
    P.SuggestedRepairRealizesCompatibleWitness suggestion witness /\
    P.SuggestedRepairCompleteLocalRepair rejection suggestion /\
    ¬ P.SuggestedRepairPartialGuidance rejection suggestion /\
    P.SuggestedRepairCoversRejectedGap rejection suggestion /\
    P.SuggestedRepairMatchesDiagnosticFamily suggestion family /\
    P.SuggestedRepairMatchesMissingEvidence suggestion missing /\
    P.SuggestedRepairDischargesLocalPremise suggestion premise /\
    P.SuggestedRepairTargetsBlame suggestion target

def EligibleSetInsertionRepair
    {V : Vocab.{u}}
    (P : Pred V)
    (fragment : V.StatementFragment)
    (rejection : V.Line1Rejection)
    (witness : V.RepairWitness) : Prop :=
  EligibleSingleEditRepair P fragment rejection witness /\
    P.SetInsertionRepairWitness fragment rejection witness

def CompleteGroupedMultiEditRepair
    {V : Vocab.{u}}
    (P : Pred V)
    (fragment : V.StatementFragment)
    (rejection : V.Line1Rejection)
    (witness : V.RepairWitness) : Prop :=
  exists family missing premise target,
    P.RejectionDiagnosticFamily rejection family /\
    P.RejectionMissingEvidence rejection missing /\
    P.RejectionFailedPremise rejection premise /\
    P.BlameTargetOf rejection target /\
    P.GroupedMultiEditRepairWitness fragment rejection witness /\
    ¬ P.SingleEditRepairWitness fragment rejection witness /\
    P.RepairWitnessInDeclaredFragment fragment witness /\
    P.RepairWitnessCoversRejectedGap rejection witness /\
    P.RepairWitnessMatchesDiagnosticFamily witness family /\
    P.RepairWitnessMatchesMissingEvidence witness missing /\
    P.RepairWitnessDischargesLocalPremise witness premise /\
    P.RepairWitnessTargetsBlame witness target

def PartialGuidanceNonCoverage
    {V : Vocab.{u}}
    (P : Pred V)
    (rejection : V.Line1Rejection)
    (suggestion : V.SuggestedRepair) : Prop :=
  P.SuggestedRepairPartialGuidance rejection suggestion /\
    ¬ P.SuggestedRepairCompleteLocalRepair rejection suggestion /\
    forall witness,
      ¬ SuggestionCoversWitness P rejection suggestion witness

def RepairCompletenessForRejection
    {V : Vocab.{u}}
    (P : Pred V)
    (env : V.Env)
    (ctx : V.Ctx)
    (locus : V.Locus)
    (fragment : V.StatementFragment)
    (input : V.Line1Input)
    (rejection : V.Line1Rejection) : Prop :=
  P.InDeclaredStatementFragment fragment input rejection ->
  P.Line1Rejected env ctx locus input rejection ->
  (exists witness,
    EligibleSingleEditRepair P fragment rejection witness) ->
    exists diagnostic suggestion witness,
      P.AssociatedEmittedDiagnostic input rejection diagnostic /\
      P.SuggestedRepairOf diagnostic suggestion /\
      EligibleSingleEditRepair P fragment rejection witness /\
      SuggestionCoversWitness P rejection suggestion witness

def OBL025StatementDraft
    (V : Vocab.{u})
    (P : Pred V) : Prop :=
  forall
    (env : V.Env)
    (ctx : V.Ctx)
    (locus : V.Locus)
    (fragment : V.StatementFragment)
    (input : V.Line1Input)
    (rejection : V.Line1Rejection),
      P.Line1InputWellScoped env ctx locus input ->
      P.CurrentEvidenceBoundary fragment ->
      P.CoveredLine1RepairCase fragment input rejection ->
        RepairCompletenessForRejection P env ctx locus fragment input rejection

end MirCore.Lab.OBL025.StatementDraft
