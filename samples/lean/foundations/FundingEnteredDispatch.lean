import OwnerCurrentCorrespondence
open MirroreaProofFirst
namespace FundingEnteredDispatch
open CohortPhase
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}

-- The entered phase names the actual current native source dispatch. This is
-- independent of host lease values and is derived from every funding step.
def AtState (state : CohortPhase.State p a) : Prop :=
  ∀ actual, state = some actual → ∀ dispatch, actual.mode = .entered dispatch →
    ∃ source, actual.driver.source = some source ∧ source.dispatch = some dispatch

theorem preserves (prior : AtState before)
    (step : CohortPhase.Step assigned scope bootstrap capacity before event after) : AtState after := by
  cases step <;> simp_all [AtState,sourceResult,ownerResult]

theorem rooted (path : CohortPhysicalOrdinal.Runs assigned scope bootstrap capacity sourceBudget ownerBudget events state) :
    AtState state := by
  induction path with
  | postBootstrap =>
    simp [AtState,CohortPhysicalOrdinal.postBootstrap,CohortPhysicalOrdinal.stateAt,
      CohortPhysicalOrdinal.liveAt,CohortPhysicalOrdinal.modeAt,CohortPhase.initial]
  | step prior step ih => exact preserves ih step

theorem native_dispatch
    (state : SharedFundedDriver.State assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (entered : state.mode = .entered dispatch) :
    ∃ source, (SharedJointDriver.native state.joint).driver.source = some source ∧
      source.dispatch = some dispatch :=
  rooted state.path _ rfl _ entered

#print axioms preserves
#print axioms rooted
#print axioms native_dispatch
end FundingEnteredDispatch
