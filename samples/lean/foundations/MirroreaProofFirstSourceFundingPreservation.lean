import MirroreaProofFirstSourceFundingFrame

namespace MirroreaProofFirst.SourceFundingPreservation
open PublicationCapacityDriver (State Status)
open PublicationOwnerBudget

-- The owner really paid this one head occurrence; its actual monitor supplies
-- the resulting vector. The arithmetic equality below identifies the debit,
-- not an authorization, owner operation, or evidence of physical completion.
-- Source execution is constructed from its independently certified path.
theorem paid_head_admitted {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a} {old : State p a}
    {source : PublicationInput.State p a} {command : PublicationInput.Command p a}
    {rest : List (PublicationInput.Command p a)} {before : Fin p → Nat}
    {actual : Vector (Fin 513) p}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed old)
    (present : old.source = some source) (pending : old.suffix = command::rest)
    (covered : Covered old.suffix before)
    (paid : ∀ owner, SourceFundingInput.credits (actual,.step command) owner =
      before owner-commandCost command owner) :
    ∃ next, SourceFundingInput.execute assigned scopeId seed old (actual,.step command) = (next,.accepted) ∧
      next.suffix = rest ∧ PublicationLifecycle.Invariant assigned scopeId seed next ∧
      Covered next.suffix (SourceFundingInput.credits (actual,.step command)) := by
  have after : Covered rest (SourceFundingInput.credits (actual,.step command)) := by
    have tail := covered_tail (by simpa [pending] using covered)
    intro owner
    rw [paid owner]
    exact tail owner
  obtain ⟨space,final,path,quiet⟩ := valid.2 source present
  rw [pending] at space path
  cases path with
  | cons fit executed tail =>
    rename_i middle
    cases budget : old.remaining with
    | zero => simp [budget] at space
    | succ remaining =>
      have base := PublicationLifecycle.scheduled_successor valid present pending budget executed
      have fast := (PublicationLifecycle.transitionFast_exact valid (input:=.step command)).trans base
      let next : State p a := ⟨some middle,remaining,rest⟩
      have admitted : SourceFundingInput.execute assigned scopeId seed old (actual,.step command) = (next,.accepted) := by
        apply SourceFundingInput.accepted_exact.mpr
        refine ⟨fast,?_⟩
        intro owner
        simpa [SourceFundingInput.initialDebit,present,next] using after owner
      refine ⟨next,admitted,rfl,?_,after⟩
      have preserved := SourceFundingInput.preserves valid (value:=(actual,.step command))
      simpa [admitted] using preserved

-- The actual source's read-only head query supplies only the syntactic head
-- premise above. Physical payment origin remains an additional separate fact.
theorem queried_payment_admitted {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a} {old : State p a}
    {source : PublicationInput.State p a} {request : SourceFundingQuery.HeadRequest p}
    {before : Fin p → Nat} {actual : Vector (Fin 513) p}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed old)
    (present : old.source = some source)
    (query : SourceFundingQuery.matchesHead old request = true)
    (covered : Covered old.suffix before)
    (paid : ∀ owner, SourceFundingInput.credits (actual,.step (SourceFundingQuery.headCommand (a:=a) request)) owner =
      before owner-commandCost (SourceFundingQuery.headCommand (a:=a) request) owner) :
    ∃ next, SourceFundingInput.execute assigned scopeId seed old
      (actual,.step (SourceFundingQuery.headCommand (a:=a) request)) = (next,.accepted) ∧
      PublicationLifecycle.Invariant assigned scopeId seed next ∧
      Covered next.suffix (SourceFundingInput.credits (actual,.step (SourceFundingQuery.headCommand (a:=a) request))) := by
  obtain ⟨rest,pending⟩ := SourceFundingQuery.matches_head_exact.mp query
  obtain ⟨next,ran,_,preserved,funded⟩ := paid_head_admitted valid present pending covered paid
  exact ⟨next,ran,preserved,funded⟩

-- A new certified plan can be adopted only with a fresh current vector. This
-- is the existing exact source admission law, not a reset/refund operation.
-- Initial debt is carried by the separate enforced prelude until all owners
-- actually initialized; source present alone is not used as that evidence.
theorem prelude_paid_entry {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a} {old next : State p a}
    {value : SourceFundingInput.Request p a}
    (accepted : SourceFundingInput.execute assigned scopeId seed old value = (next,.accepted))
    (fresh : old.source = none) :
    OwnerFundingCursor.InitCovered next.suffix (fun _ => true) (SourceFundingInput.credits value) := by
  have reserved := SourceFundingInput.initial_reserved accepted fresh
  intro owner
  exact reserved owner

#print axioms paid_head_admitted
#print axioms queried_payment_admitted
#print axioms prelude_paid_entry
end MirroreaProofFirst.SourceFundingPreservation
