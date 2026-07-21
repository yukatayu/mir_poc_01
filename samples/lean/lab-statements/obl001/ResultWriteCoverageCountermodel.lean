import samples.lean.«lab-statements».obl001.THM001StatementDraft

/-!
LAB-only countermodel for result/write coverage in the OBL-001 statement-shape
draft. An experiment-local Result contains a labeled cross-write that the
draft does not enumerate with GeneratedWrite. This does not define Canon Core.
-/

namespace MirCore.Lab.OBL001.ResultWriteCoverageCountermodel

open MirCore.Lab.OBL001.StatementDraft

inductive ExperimentResult where
  | safe
  | untrackedCross

inductive ExperimentWrite where
  | only

def V : Vocab where
  Env := Unit
  Ctx := Unit
  Locus := Unit
  Assign := Unit
  Result := ExperimentResult
  Write := ExperimentWrite
  Request := Unit

def P : Pred V where
  SurfaceAssignment := fun _ => True
  SimpleAssign := fun _ => True
  ElaboratesAssignment := fun _ _ _ _ _ => True
  GeneratedWrite := fun _ _ => False
  OwnerLocalWriteAt := fun _ _ _ _ => False
  RequestForWrite := fun _ _ _ => False
  OwnerDirectedRequest := fun _ _ _ _ => False
  RequestCarriesFailureContainment := fun _ _ => False
  RequestCarriesAuthorityObligations := fun _ _ _ _ => False
  RequestCarriesDependencyEvidence := fun _ _ => False
  RequestCarriesSpanEvidence := fun _ _ => False
  AllRhsReadsRecorded := fun _ _ => True
  GeneratedFailuresContained := fun _ _ => True
  AuthorityObligationsRepresented := fun _ _ _ _ => True
  SourceSpansPreserved := fun _ _ => True
  VisibleWriteConsequencesExplicit := fun _ _ _ => True
  NoAmbientAuthorityFromNestedLocus := fun _ _ _ _ => True

def ExperimentOnlyWriteMembership : V.Result -> V.Write -> Prop
  | .untrackedCross, .only => True
  | _, _ => False

def ResultWriteCoverage : Prop :=
  forall result write,
    ExperimentOnlyWriteMembership result write -> P.GeneratedWrite result write

theorem statement_draft_holds : THM001StatementDraft V P := by
  simp [THM001StatementDraft, AssignmentElabSoundnessPost,
    AllGeneratedWritesSound, GeneratedWriteSound, V, P]

theorem elaborates_untracked_result :
    P.ElaboratesAssignment () () () () .untrackedCross := by
  simp [P]

theorem untracked_result_contains_experiment_only_write :
    ExperimentOnlyWriteMembership .untrackedCross .only := by
  simp [ExperimentOnlyWriteMembership]

theorem untracked_result_has_no_generated_write :
    ¬ P.GeneratedWrite .untrackedCross .only := by
  simp [P]

theorem result_write_coverage_fails : ¬ ResultWriteCoverage := by
  intro coverage
  have generated := coverage .untrackedCross .only
    untracked_result_contains_experiment_only_write
  exact untracked_result_has_no_generated_write generated

theorem countermodel_exhibits_untracked_write :
    P.ElaboratesAssignment () () () () .untrackedCross /\
      ExperimentOnlyWriteMembership .untrackedCross .only /\
      (¬ P.GeneratedWrite .untrackedCross .only) /\
      THM001StatementDraft V P :=
  ⟨elaborates_untracked_result, untracked_result_contains_experiment_only_write,
    untracked_result_has_no_generated_write, statement_draft_holds⟩

end MirCore.Lab.OBL001.ResultWriteCoverageCountermodel
