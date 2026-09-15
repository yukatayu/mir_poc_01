import MirroreaProofFirstSourceOwnerFloor

namespace MirroreaProofFirst.SourceCurrentFrame

-- Complete current source data may change only when the actual publication
-- revision strictly increases. Staging a change is not its publication.
theorem image_current
    (ran : PublicationImage.execute CustodyPublication.image QualifiedCustody.evaluate source action = some next) :
    (next.barrier.published = source.barrier.published ∧ next.current = source.current) ∨
      source.barrier.published < next.barrier.published := by
  unfold PublicationImage.execute at ran
  split at ran
  · rename_i checked
    cases ran
    cases action with
    | enter i => exact Or.inl ⟨rfl,rfl⟩
    | finish i => exact Or.inl ⟨rfl,rfl⟩
    | administrative action =>
      cases action with
      | publish =>
        have newer : source.barrier.published < source.barrier.announced := by
          have checks := checked
          simp only [PublicationImage.check,PublicationPayload.projected,Bool.and_eq_true] at checks
          exact ((Publication.check_exact _ _).mp checks.1).1
        exact Or.inr newer
      | stage c => exact Or.inl ⟨rfl,rfl⟩
      | freeze i r => exact Or.inl ⟨rfl,rfl⟩
      | acknowledge i r => exact Or.inl ⟨rfl,rfl⟩
      | install i r => exact Or.inl ⟨rfl,rfl⟩
      | use i => exact Or.inl ⟨rfl,rfl⟩
  · cases ran

theorem source_current (ran : PublicationInput.execute scopeId source command = some next) :
    (next.publication.barrier.published = source.publication.barrier.published ∧
      next.publication.current = source.publication.current) ∨
      source.publication.barrier.published < next.publication.barrier.published := by
  obtain ⟨_,_,executed,_⟩ := PublicationInput.executed_action ran
  exact image_current executed

-- Source-side install eligibility plus reached numerical history determines
-- its actual publication/fence. No physical owner-currentness premise is used.
theorem install_current
    (path : PublicationInput.Reached scopeId initial source)
    (ran : PublicationInput.execute scopeId source (.install i revision) = some next) :
    revision = source.publication.barrier.published ∧
      source.publication.barrier.fence i = revision := by
  obtain ⟨action,selected,executed,_⟩ := PublicationInput.executed_action ran
  have same : action = .administrative (.install i revision) := (Option.some.inj selected).symm
  subst action
  unfold PublicationImage.execute at executed
  split at executed
  · rename_i checked
    have numeric : Publication.check source.publication.barrier (.install i revision) = true := by
      simp only [PublicationImage.check,PublicationPayload.projected,Bool.and_eq_true] at checked
      exact checked.1
    have allowed := (Publication.check_exact _ _).mp numeric
    have inv := Publication.reached_invariant (SourceOwnerFloor.source_numeric path)
    have certificate := inv.2.2.2 revision allowed.1
    have node := inv.2.1 i
    have lower : source.publication.barrier.fence i ≤ revision := allowed.2
    exact ⟨by omega,by omega⟩
  · cases executed

#print axioms image_current
#print axioms source_current
#print axioms install_current
end MirroreaProofFirst.SourceCurrentFrame
