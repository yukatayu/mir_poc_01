/-!
LAB-only OBL-024 diagnostic-soundness statement-shape draft.

This file checks that diagnostic explanation soundness can be expressed as Lean
propositions without importing final MirCore diagnostic datatypes, final JSON
fields, replay engines, conformance evidence, repair payloads, or
proof-obligation status changes. The report-local replay anchor is kept
separate from the future proof-level replay witness/relation.
-/

namespace MirCore.Lab.OBL024.StatementDraft

universe u

structure Vocab where
  Env : Type u
  Ctx : Type u
  Locus : Type u
  JudgmentInput : Type u
  Rejection : Type u
  Diagnostic : Type u
  DiagnosticId : Type u
  RuleInstance : Type u
  FailedPremise : Type u
  Bindings : Type u
  DiagnosticFamily : Type u
  MissingEvidenceKind : Type u
  Span : Type u
  AssociationKey : Type u
  DiagnosticBranch : Type u
  ReportLocalReplayAnchor : Type u
  ProofLevelReplayWitness : Type u

structure Pred (V : Vocab.{u}) where
  WellScopedInput :
    V.Env -> V.Ctx -> V.Locus -> V.JudgmentInput -> Prop
  CurrentEvidenceBoundary :
    V.Diagnostic -> Prop
  CoveredDiagnosticSoundnessCase :
    V.JudgmentInput -> V.Rejection -> V.Diagnostic -> Prop
  Rejects :
    V.Env -> V.Ctx -> V.Locus -> V.JudgmentInput -> V.Rejection -> Prop
  AssociatedEmittedDiagnostic :
    V.JudgmentInput -> V.Rejection -> V.Diagnostic -> Prop
  RejectionAssociationKey :
    V.JudgmentInput -> V.Rejection -> V.AssociationKey -> Prop
  DiagnosticAssociationKey :
    V.Diagnostic -> V.AssociationKey -> Prop
  DiagnosticReportsId :
    V.Diagnostic -> V.DiagnosticId -> Prop
  DiagnosticReportsRuleInstance :
    V.Diagnostic -> V.RuleInstance -> Prop
  DiagnosticReportsFailedPremise :
    V.Diagnostic -> V.FailedPremise -> Prop
  DiagnosticReportsBindings :
    V.Diagnostic -> V.Bindings -> Prop
  DiagnosticFamilyOf :
    V.Diagnostic -> V.DiagnosticFamily -> Prop
  DiagnosticMissingEvidence :
    V.Diagnostic -> V.MissingEvidenceKind -> Prop
  DiagnosticPrimarySpan :
    V.Diagnostic -> V.Span -> Prop
  DiagnosticReportsReplayAnchor :
    V.Diagnostic -> V.ReportLocalReplayAnchor -> Prop
  ActualRuleInstance :
    V.Env ->
      V.Ctx ->
      V.Locus ->
      V.JudgmentInput ->
      V.Rejection ->
      V.RuleInstance ->
      V.Bindings ->
      Prop
  PremiseOfRuleInstance :
    V.RuleInstance -> V.FailedPremise -> V.Bindings -> Prop
  BindingsReconstructFailedPremise :
    V.Env ->
      V.Ctx ->
      V.Locus ->
      V.JudgmentInput ->
      V.Rejection ->
      V.RuleInstance ->
      V.FailedPremise ->
      V.Bindings ->
      Prop
  ReportLocalReplayAnchorFor :
    V.JudgmentInput ->
      V.Rejection ->
      V.RuleInstance ->
      V.FailedPremise ->
      V.Bindings ->
      V.ReportLocalReplayAnchor ->
      Prop
  ReportLocalReplayAnchorNonFinal :
    V.ReportLocalReplayAnchor -> Prop
  ProofLevelReplayWitnessFor :
    V.Env ->
      V.Ctx ->
      V.Locus ->
      V.JudgmentInput ->
      V.Rejection ->
      V.RuleInstance ->
      V.Bindings ->
      V.ReportLocalReplayAnchor ->
      V.ProofLevelReplayWitness ->
      Prop
  ProofLevelReplayRelation :
    V.ProofLevelReplayWitness -> V.FailedPremise -> Prop
  DiagnosticIdMatchesPremise :
    V.DiagnosticId -> V.FailedPremise -> Prop
  DiagnosticFamilyMatchesPremise :
    V.DiagnosticFamily -> V.FailedPremise -> Prop
  MissingEvidenceMatchesPremise :
    V.MissingEvidenceKind -> V.FailedPremise -> Prop
  SpanBlamesFailedPremise :
    V.Span -> V.FailedPremise -> V.MissingEvidenceKind -> Prop
  MixedRowDiagnostic :
    V.Diagnostic -> Prop
  DiagnosticOwnsWholeFailedPremise :
    V.Diagnostic -> V.FailedPremise -> Prop
  BranchOfDiagnosticGap :
    V.Diagnostic -> V.DiagnosticBranch -> Prop
  BranchClassifiesMissingEvidence :
    V.DiagnosticBranch -> V.MissingEvidenceKind -> Prop
  BranchPartitionExact :
    V.Diagnostic -> V.DiagnosticBranch -> Prop
  BranchesAreNotIndependentPremises :
    V.Diagnostic -> V.DiagnosticBranch -> V.FailedPremise -> Prop

def DiagnosticAssociatedToRejection
    {V : Vocab.{u}}
    (P : Pred V)
    (input : V.JudgmentInput)
    (rejection : V.Rejection)
    (diagnostic : V.Diagnostic)
    (key : V.AssociationKey) : Prop :=
  P.AssociatedEmittedDiagnostic input rejection diagnostic /\
    P.RejectionAssociationKey input rejection key /\
    P.DiagnosticAssociationKey diagnostic key

def ReportedDiagnosticShape
    {V : Vocab.{u}}
    (P : Pred V)
    (diagnostic : V.Diagnostic)
    (diagnosticId : V.DiagnosticId)
    (rule : V.RuleInstance)
    (premise : V.FailedPremise)
    (bindings : V.Bindings)
    (family : V.DiagnosticFamily)
    (missing : V.MissingEvidenceKind)
    (span : V.Span)
    (anchor : V.ReportLocalReplayAnchor) : Prop :=
  P.DiagnosticReportsId diagnostic diagnosticId /\
    P.DiagnosticReportsRuleInstance diagnostic rule /\
    P.DiagnosticReportsFailedPremise diagnostic premise /\
    P.DiagnosticReportsBindings diagnostic bindings /\
    P.DiagnosticFamilyOf diagnostic family /\
    P.DiagnosticMissingEvidence diagnostic missing /\
    P.DiagnosticPrimarySpan diagnostic span /\
    P.DiagnosticReportsReplayAnchor diagnostic anchor

def MixedDiagnosticBranchBoundary
    {V : Vocab.{u}}
    (P : Pred V)
    (diagnostic : V.Diagnostic)
    (premise : V.FailedPremise) : Prop :=
  P.MixedRowDiagnostic diagnostic ->
    P.DiagnosticOwnsWholeFailedPremise diagnostic premise /\
      forall branch,
        P.BranchOfDiagnosticGap diagnostic branch ->
          (exists branchMissing,
            P.BranchClassifiesMissingEvidence branch branchMissing) /\
            P.BranchPartitionExact diagnostic branch /\
              P.BranchesAreNotIndependentPremises diagnostic branch premise

def ReportLocalReplayAnchorCompatible
    {V : Vocab.{u}}
    (P : Pred V)
    (input : V.JudgmentInput)
    (rejection : V.Rejection)
    (rule : V.RuleInstance)
    (premise : V.FailedPremise)
    (bindings : V.Bindings)
    (anchor : V.ReportLocalReplayAnchor) : Prop :=
  P.ReportLocalReplayAnchorFor input rejection rule premise bindings anchor /\
    P.ReportLocalReplayAnchorNonFinal anchor

def ReplaySoundAtReportedPremise
    {V : Vocab.{u}}
    (P : Pred V)
    (env : V.Env)
    (ctx : V.Ctx)
    (locus : V.Locus)
    (input : V.JudgmentInput)
    (rejection : V.Rejection)
    (rule : V.RuleInstance)
    (premise : V.FailedPremise)
    (bindings : V.Bindings)
    (anchor : V.ReportLocalReplayAnchor)
    (replay : V.ProofLevelReplayWitness) : Prop :=
  P.ActualRuleInstance env ctx locus input rejection rule bindings /\
    P.PremiseOfRuleInstance rule premise bindings /\
    P.BindingsReconstructFailedPremise
      env ctx locus input rejection rule premise bindings /\
    ReportLocalReplayAnchorCompatible
      P input rejection rule premise bindings anchor /\
    P.ProofLevelReplayWitnessFor
      env ctx locus input rejection rule bindings anchor replay /\
    P.ProofLevelReplayRelation replay premise

def DiagnosticSoundForRejection
    {V : Vocab.{u}}
    (P : Pred V)
    (env : V.Env)
    (ctx : V.Ctx)
    (locus : V.Locus)
    (input : V.JudgmentInput)
    (rejection : V.Rejection)
    (diagnostic : V.Diagnostic) : Prop :=
  exists key diagnosticId rule premise bindings family missing span anchor replay,
    DiagnosticAssociatedToRejection P input rejection diagnostic key /\
      ReportedDiagnosticShape
        P diagnostic diagnosticId rule premise bindings family missing span anchor /\
      ReplaySoundAtReportedPremise
        P env ctx locus input rejection rule premise bindings anchor replay /\
      P.DiagnosticIdMatchesPremise diagnosticId premise /\
      P.DiagnosticFamilyMatchesPremise family premise /\
      P.MissingEvidenceMatchesPremise missing premise /\
      P.SpanBlamesFailedPremise span premise missing /\
      MixedDiagnosticBranchBoundary P diagnostic premise

def OBL024StatementDraft
    (V : Vocab.{u})
    (P : Pred V) : Prop :=
  forall
    (env : V.Env)
    (ctx : V.Ctx)
    (locus : V.Locus)
    (input : V.JudgmentInput)
    (rejection : V.Rejection)
    (diagnostic : V.Diagnostic),
      P.WellScopedInput env ctx locus input ->
      P.CurrentEvidenceBoundary diagnostic ->
      P.CoveredDiagnosticSoundnessCase input rejection diagnostic ->
      P.Rejects env ctx locus input rejection ->
      P.AssociatedEmittedDiagnostic input rejection diagnostic ->
        DiagnosticSoundForRejection
          P env ctx locus input rejection diagnostic

end MirCore.Lab.OBL024.StatementDraft
