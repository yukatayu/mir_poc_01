import samples.lean.«lab-statements».obl020.StepWFStatementDraft

/-!
LAB-only boundary experiment for the OBL-020 statement-shape draft.

The implications here remain in the draft's abstract vocabulary.  The finite
model and its family name are experiment-local: they neither identify a Canon
step family nor prescribe a coverage rule for MirCore.
-/

namespace MirCore.Lab.OBL020.FamilywiseGlobalBoundary

open MirCore.Lab.OBL020.StatementDraft

universe u

theorem global_implies_familywise
    {V : Vocab.{u}}
    (P : Pred V)
    (global : OBL020StatementDraft V P) :
    forall before label after family,
      FamilyStepPreservesWF P before label after family := by
  intro before label after family _ _
  exact global before label after

theorem coverage_and_familywise_imply_global
    {V : Vocab.{u}}
    (P : Pred V)
    (coverage :
      forall before label after,
        P.WellFormed before ->
        P.Step before label after ->
        Exists fun family =>
          P.CanonStepFamily family /\ P.StepHasFamily label family)
    (familywise :
      forall before label after family,
        FamilyStepPreservesWF P before label after family) :
    OBL020StatementDraft V P := by
  intro before label after beforeWf step
  obtain ⟨family, canonical, classified⟩ :=
    coverage before label after beforeWf step
  exact familywise before label after family canonical classified beforeWf step

inductive ModelConfig where
  | good
  | bad

inductive ModelLabel where
  | classified
  | unclassified

inductive ModelFamily where
  | ordinary

def ModelVocab : Vocab where
  Config := ModelConfig
  StepLabel := ModelLabel
  StepFamily := ModelFamily

def ModelPred : Pred ModelVocab where
  WellFormed := fun config =>
    match config with
    | .good => True
    | .bad => False
  Step := fun before label after =>
    match before, label, after with
    | .good, .classified, .good => True
    | .good, .unclassified, .bad => True
    | _, _, _ => False
  CanonStepFamily := fun family =>
    match family with
    | .ordinary => True
  StepHasFamily := fun label family =>
    match label, family with
    | .classified, .ordinary => True
    | _, _ => False

theorem model_has_nonvacuous_canonical_family_and_classified_step :
    ModelPred.CanonStepFamily .ordinary /\
      ModelPred.StepHasFamily .classified .ordinary /\
      ModelPred.Step .good .classified .good := by
  simp [ModelPred]

theorem model_has_unclassified_nonpreserving_step :
    ModelPred.WellFormed .good /\
      ModelPred.Step .good .unclassified .bad /\
      forall family, Not (ModelPred.StepHasFamily .unclassified family) := by
  constructor
  · simp [ModelPred]
  constructor
  · simp [ModelPred]
  intro family
  cases family
  simp [ModelPred]

theorem familywise_model_holds :
    forall before label after family,
      FamilyStepPreservesWF ModelPred before label after family := by
  intro before label after family
  cases before <;> cases label <;> cases after <;> cases family <;>
    simp [ModelPred, FamilyStepPreservesWF, PreservesWF]

theorem global_model_fails : Not (OBL020StatementDraft ModelVocab ModelPred) := by
  intro global
  have bad : ModelPred.WellFormed .bad :=
    global .good .unclassified .bad trivial trivial
  simp [ModelPred] at bad

theorem familywise_without_coverage_can_hold_while_global_fails :
    (forall before label after family,
      FamilyStepPreservesWF ModelPred before label after family) /\
      Not (OBL020StatementDraft ModelVocab ModelPred) :=
  ⟨familywise_model_holds, global_model_fails⟩

end MirCore.Lab.OBL020.FamilywiseGlobalBoundary
