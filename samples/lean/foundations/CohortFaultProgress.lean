import CohortRetiredProbe
open MirroreaProofFirst
namespace CohortFaultProgress
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Runtime ownership of the complete unresolved occurrence survives both
-- cleanup transitions. The current host is derived from this one stage value.
inductive State (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  | stopped (origin : CohortHostFailure.Outstanding base)
  | retired (checked : CohortHostFailure.Retired base)
  | released (checked : CohortHostFailure.Released base)

def State.current : State base → CohortHostExecution.Live base
  | .stopped origin => origin.anchor
  | .retired checked => checked.current
  | .released checked => checked.current

def State.origin : State base → CohortHostFailure.Outstanding base
  | .stopped origin => origin
  | .retired checked => checked.origin
  | .released checked => checked.retired.origin

def State.slot (state : State base) : SharedReplyRetention.Slot p a := state.origin.slot

def State.retire : State base → Option (State base)
  | .stopped origin => origin.retire.map .retired
  | _ => none

def State.release : State base → Option (State base)
  | .retired checked => checked.release.map .released
  | _ => none

theorem State.retire_total (origin : CohortHostFailure.Outstanding base) :
    (State.stopped origin).retire.isSome = true := by
  simpa [State.retire] using origin.retire_total

theorem State.release_total (checked : CohortHostFailure.Retired base) :
    (State.retired checked).release.isSome = true := by
  simpa [State.release] using checked.release_total

theorem State.retire_origin (checked : State.retire before = some after) :
    after.origin = before.origin := by
  cases before with
  | stopped origin =>
    obtain ⟨retired,ran,equal⟩ := Option.map_eq_some_iff.mp checked
    cases equal
    exact CohortHostFailure.Outstanding.retire_origin ran
  | retired | released => cases checked

theorem State.release_origin (checked : State.release before = some after) :
    after.origin = before.origin := by
  cases before with
  | retired retired =>
    obtain ⟨released,ran,equal⟩ := Option.map_eq_some_iff.mp checked
    cases equal
    change released.retired.origin = retired.origin
    rw [CohortHostFailure.Retired.release_origin ran]
  | stopped | released => cases checked

theorem State.keeps (state : State base) :
    state.current.current.state.cohort.memory = state.origin.anchor.current.state.cohort.memory ∧
    state.current.current.state.cohort.pending = state.origin.anchor.current.state.cohort.pending ∧
    state.current.source.state.joined.history = state.origin.anchor.source.state.joined.history := by
  cases state with
  | stopped => exact ⟨rfl,rfl,rfl⟩
  | retired checked => exact checked.keeps
  | released checked => exact checked.keeps

theorem State.rooted (state : State base) :
    SharedWireLifetime.Runs (SharedWireLifetime.initial base)
      (state.current.source.state.joined.history.actions++state.origin.stopped.pending.residual.actions)
      (SharedReplyRetention.wire state.origin.armed.wire.attempt state.slot) := by
  rw [state.keeps.2.2]
  exact state.origin.stopped.rooted

theorem State.no_absorb (state : State base) :
    HostReplyPrefix.absorb state.origin.stopped.pending = none := state.origin.stopped.no_absorb

theorem State.retire_slot (checked : State.retire before = some after) : after.slot = before.slot := by
  simp only [State.slot,State.retire_origin checked]

theorem State.release_slot (checked : State.release before = some after) : after.slot = before.slot := by
  simp only [State.slot,State.release_origin checked]

theorem release_not_early (origin : CohortHostFailure.Outstanding base) :
    (State.stopped origin).release = none := rfl

theorem no_second_retire (checked : CohortHostFailure.Retired base) :
    (State.retired checked).retire = none := rfl

theorem no_second_release (checked : CohortHostFailure.Released base) :
    (State.released checked).release = none := rfl

#print axioms State.retire_total
#print axioms State.release_total
#print axioms State.retire_origin
#print axioms State.release_origin
#print axioms State.keeps
#print axioms State.rooted
#print axioms State.no_absorb
#print axioms State.retire_slot
#print axioms State.release_slot
#print axioms release_not_early
#print axioms no_second_retire
#print axioms no_second_release
end CohortFaultProgress
