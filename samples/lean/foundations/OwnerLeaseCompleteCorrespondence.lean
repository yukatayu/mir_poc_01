import LeaseCohortCorrespondence
open MirroreaProofFirst SharedHostCaptureReplay
namespace OwnerLeaseCompleteCorrespondence
open OwnerLeasePhaseRelation
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Aligned/check ALONE deliberately excludes the active focused owner. This
-- exported composition states its exact full journal memory as well. It is
-- derived from the old invariants, never inserted into operational admission.
def AtOwner (state : SourceEntryPrefix.State base) (owner : Fin p) : Prop :=
  match state.joined.active,state.active with
  | some writer,_ =>
      if owner = writer.bound.writer.owner then state.writers owner = writer.bound.writer.current
      else leasePair (state.writers owner) = released
  | none,some entry =>
      if owner = entry.entry.context.endpoint then
        state.writers owner = entry.entry.state.writer ∧ entry.entry.state.history = state.joined.history
      else leasePair (state.writers owner) = released
  | none,none => leasePair (state.writers owner) = expected state.joined.history.current.mode owner

theorem complete (relation : LeaseCohortCorrespondence.Correspondence live) :
    ∀ owner, AtOwner live.current.state.inner owner := by
  intro owner
  have other := relation.leases owner
  have writers := relation.fields.owners
  cases active : live.current.state.inner.joined.active with
  | some writer =>
    simp only [OwnerCurrentCorrespondence.Aligned,active] at writers
    simp only [OwnerLeasePhaseRelation.AtOwner,active] at other
    by_cases focused : owner = writer.bound.writer.owner
    · simpa [AtOwner,active,focused] using writers.1
    · simpa [AtOwner,active,focused,OwnerLeasePhaseRelation.AtOwner] using other focused
  | none =>
    cases entry : live.current.state.inner.active with
    | none => simpa [AtOwner,active,entry,OwnerLeasePhaseRelation.AtOwner] using other
    | some opened =>
      have exactMemory := (relation.fields.entry.1 opened entry)
      simp only [OwnerLeasePhaseRelation.AtOwner,active,entry] at other
      by_cases focused : owner = opened.entry.context.endpoint
      · simpa [AtOwner,active,entry,focused] using And.intro exactMemory.2.1 exactMemory.1
      · simpa [AtOwner,active,entry,focused,OwnerLeasePhaseRelation.AtOwner] using other focused

theorem idle_aligned_iff (state : SourceEntryPrefix.State base)
    (noWriter : state.joined.active = none) (noEntry : state.active = none) :
    Aligned state ↔ ∀ owner, leasePair (state.writers owner) = expected state.joined.history.current.mode owner := by
  simp [Aligned,OwnerLeasePhaseRelation.AtOwner,noWriter,noEntry]

theorem idle_entered_exact (state : SourceEntryPrefix.State base)
    (noWriter : state.joined.active = none) (noEntry : state.active = none)
    (mode : state.joined.history.current.mode = .entered dispatch) (aligned : Aligned state) :
    leasePair (state.writers dispatch.endpoint) = (some dispatch.ticket,true) ∧
    ∀ owner, owner ≠ dispatch.endpoint → leasePair (state.writers owner) = released := by
  have each := (idle_aligned_iff state noWriter noEntry).mp aligned
  constructor
  · simpa [mode,expected] using each dispatch.endpoint
  · intro owner different
    simpa [mode,expected,different] using each owner

-- The general theorem's premise permits any rooted prelude base. The actual
-- capture consumer obtains its narrower origin from fromLaunch by rfl. Neither
-- theorem grants a same-instance recovery or owner authority claim.
#print axioms complete
#print axioms idle_aligned_iff
#print axioms idle_entered_exact
end OwnerLeaseCompleteCorrespondence
