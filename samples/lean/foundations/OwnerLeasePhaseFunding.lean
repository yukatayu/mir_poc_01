import OwnerLeasePhaseRelation
open MirroreaProofFirst SharedHostCaptureReplay
namespace OwnerLeasePhaseFunding
open OwnerLeasePhaseRelation CohortPhase
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat}

-- Independently declarative funding rules, with no host-memory premise.
theorem owner_other_before
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before) (.owner target command) (some after))
    (different : owner ≠ target) : expected before.mode owner = released := by
  cases step <;> simp_all [expected,released]

theorem owner_after
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before) (.owner target command) (some after)) :
    expected after.mode owner = released := by
  cases step <;> simp_all [expected,ownerResult,released]

theorem source_ordinary_frame
    (allowed : SourceEntryPrefix.ordinaryAllowed (.source (.inr (.inr request)) bytes) = true)
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before) (.source request) (some after)) :
    expected after.mode owner = expected before.mode owner := by
  cases step <;> simp_all [expected,sourceResult,released,SourceEntryPrefix.ordinaryAllowed]

theorem query_frame
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before) (.query request) (some after)) :
    expected after.mode owner = expected before.mode owner := by
  cases step
  rfl

theorem local_frame
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before) .local (some after)) :
    expected after.mode owner = expected before.mode owner := by
  cases step <;> simp_all [expected,released]

-- Entry source requests choose their phase from actual accepted/refused native
-- semantics. No host entered flag or proposed lease invariant selects it.
theorem source_accepted_enter
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before)
      (.source (vector,.step (.enter endpoint))) (some after))
    (accepted : (SourceFundingInput.execute assigned scope bootstrap before.driver
      (vector,.step (.enter endpoint))).2 = .accepted) :
    ∃ dispatch, after.mode = .entered dispatch := by
  cases step <;> simp_all [sourceResult,PublicJointHistory.publicSource]
  case notify mode permitted bound ran =>
    rw [PaidHeadPhase.permits_exact] at bound
    rcases bound with ⟨_,command⟩
    unfold PaidHeadPhase.command at command
    split at command <;> contradiction

theorem source_refused_released
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before) (.source request) (some after))
    (refused : (SourceFundingInput.execute assigned scope bootstrap before.driver request).2 ≠ .accepted) :
    expected after.mode owner = released := by
  cases step with
  | launch mode fresh bound ran => exact False.elim (refused (by rw [ran]))
  | source mode allowed bound ran => exact False.elim (refused (by rw [ran]))
  | refused modes bound denied =>
    rcases modes with ordinary | ⟨owed,prelude⟩ <;> simp_all [expected,released]
  | notify mode allowed bound ran => exact False.elim (refused (by rw [ran]))
  | enter mode bound ran present dispatched head => exact False.elim (refused (by rw [ran]))
  | finish mode bound ran => exact False.elim (refused (by rw [ran]))

#print axioms owner_other_before
#print axioms owner_after
#print axioms source_ordinary_frame
#print axioms query_frame
#print axioms local_frame
#print axioms source_accepted_enter
#print axioms source_refused_released
end OwnerLeasePhaseFunding
