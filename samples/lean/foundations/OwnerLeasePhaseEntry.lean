import FundingEnteredDispatch
import OwnerLeasePhaseFunding
open MirroreaProofFirst SharedHostCaptureReplay
namespace OwnerLeasePhaseEntry
open OwnerLeasePhaseRelation CohortHostReceipt SourceEntryJournal
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

theorem started_released
    (ran : SourceEntryCapture.start history endpoint vector before staged observedSnapshot = some opened) :
    expected history.current.mode owner = released := by
  apply released_mode
  intro dispatch mode
  obtain ⟨source,present,dispatched⟩ := FundingEnteredDispatch.native_dispatch history.current mode
  obtain ⟨_,_,actual,ticket,actualAt,_,idle,_,_,_⟩ := (SourceEntryCapture.start_sound ran).1
  have same : source = actual := Option.some.inj (present.symm.trans actualAt)
  subst actual
  change source.dispatch.isNone = true at idle
  simp [dispatched] at idle

theorem accepted_expected (opened : SourceEntryCapture.Open base)
    (phase : AcceptedPhase opened.state.phase) (owner : Fin p) :
    expected opened.state.history.current.mode owner =
      if owner = opened.context.endpoint then (some opened.context.ticket,true) else released := by
  obtain ⟨⟨bytes,known⟩,accepted⟩ := accepted_known opened.valid phase
  obtain ⟨dispatch,mode⟩ := OwnerLeasePhaseFunding.source_accepted_enter (history_funding known) accepted
  change opened.state.history.current.mode = .entered dispatch at mode
  obtain ⟨source,present,dispatched⟩ := FundingEnteredDispatch.native_dispatch opened.state.history.current mode
  obtain ⟨actual,actualAt,actualDispatch⟩ := opened.accepted_dispatch phase
  have same : source = actual := Option.some.inj (present.symm.trans actualAt)
  subst actual
  have bound := Option.some.inj (dispatched.symm.trans actualDispatch)
  simp [expected,mode,bound]

theorem refused_expected (opened : SourceEntryCapture.Open base)
    (known : Known opened.context opened.state)
    (refused : opened.context.status opened.context.before ≠ .accepted) (owner : Fin p) :
    expected opened.state.history.current.mode owner = released := by
  obtain ⟨bytes,ran⟩ := known
  exact OwnerLeasePhaseFunding.source_refused_released (history_funding ran) refused

-- At the actual snapshot-store closure the stored lease pair is exactly the
-- current native phase's focused obligation; every other owner is released.
theorem stored_expected (opened : SourceEntryCapture.Open base)
    (phase : opened.state.phase = .acceptedStored ∨ opened.state.phase = .refusedStored) (owner : Fin p) :
    expected opened.state.history.current.mode owner =
      if owner = opened.context.endpoint then leasePair opened.state.writer else released := by
  have fields := opened.valid.2
  rcases phase with accepted | refused
  · have meaning := accepted_expected opened (by simp [AcceptedPhase,accepted]) owner
    simp only [accepted] at fields
    simpa [leasePair,fields.2.2.1,fields.2.2.2.1] using meaning
  · simp only [refused] at fields
    rw [refused_expected opened fields.1 fields.2.1 owner]
    simp [leasePair,fields.2.2.1,fields.2.2.2.1,released]

#print axioms started_released
#print axioms accepted_expected
#print axioms refused_expected
#print axioms stored_expected
end OwnerLeasePhaseEntry
