import MirroreaProofFirstReferenceSourceTyping

namespace MirroreaProofFirst.ReferenceSourceTrace
open ReferenceSourceData ReferenceSource ReferenceSourceElaboration

-- Only these source entries are admitted. Raw Plan execution, mark/write,
-- populated-state import, and constructor replacement are not transitions.
-- Failed outcomes remain transitions to their actual retained microstep state.
inductive Step : State p a → State p a → Prop where
  | advance : Step s (ReferenceSource.advance s member place principal item).state
  | cancellation : Step s (ReferenceSource.cancel s member place principal).state
  | complete : Step s (ReferenceSource.complete s).state
  | head : Step s (authorityHead s view)
  | control : controlInput s member place principal raw = some (next,created) → Step s next

inductive Reached : State p a → State p a → Prop where
  | refl : Reached s s
  | step : Reached first s → Step s next → Reached first next

-- Root parameters are explicit authority INPUTS, not generated from a proof.
def Rooted (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (s : State p a) : Prop :=
  Reached (ReferenceSource.initial realm view policy) s

theorem trans (first middle last : State p a) (left : Reached first middle) (right : Reached middle last) :
    Reached first last := by
  induction right with
  | refl => exact left
  | step _ step ih => exact .step ih step

theorem run_reached (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (program : List Located) :
    Reached s (run s member place principal program).state := by
  induction program generalizing s with
  | nil => exact .refl
  | cons item rest ih =>
      simp only [run]
      cases advanced : (advance s member place principal item).status with
      | failed reason => exact .step .refl .advance
      | ready => exact trans _ _ _ (.step .refl .advance) (ih _)
      | waiting =>
          dsimp only
          cases resumed : (complete (advance s member place principal item).state).status with
          | ready => exact trans _ _ _ (.step (.step .refl .advance) .complete) (ih _)
          | waiting => exact .step (.step .refl .advance) .complete
          | failed reason => exact .step (.step .refl .advance) .complete

theorem step_machine (s next : State p a) (step : Step s next) :
    ReferenceExecution.Reached s.machine next.machine := by
  cases step with
  | advance => exact advance_projects _ _ _ _ _
  | cancellation => exact cancel_projects _ _ _ _
  | complete => exact complete_projects _
  | head => exact head_projects _ _
  | control run => exact (controlInput_projects _ _ _ _ _ _ run).1

theorem step_allocated (s next : State p a) (valid : Allocated s) (step : Step s next) : Allocated next := by
  cases step with
  | advance => exact advance_allocated _ _ _ _ _ valid
  | cancellation => exact cancel_allocated _ _ _ _ valid
  | complete => exact complete_allocated _ valid
  | head => exact head_allocated _ _ valid
  | control run => exact controlInput_allocated _ _ _ _ _ _ valid run

theorem rooted_invariants (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (s : State p a) (path : Rooted realm view policy s) :
    ReferenceExecution.Invariant s.machine ∧ Allocated s := by
  induction path with
  | refl => exact ⟨ReferenceExecution.initial_invariant _ _ _,initial_allocated _ _ _⟩
  | step previous step ih =>
      exact ⟨ReferenceExecution.reached_preserves _ _ ih.1 (step_machine _ _ step),step_allocated _ _ ih.2 step⟩

-- This exposes a substantive independent meaning for EVERY changing source
-- advance, including normalization committed before a later failure. A machine
-- projection alone could not exclude a mismatched source item / plain plan.
theorem advance_effect_basis (s : State p a) (member : Fin a) (place : Fin p)
    (principal : Nat) (item : Located)
    (changed : (advance s member place principal item).state ≠ s) :
    s.waiting = none ∧ ∃ env plan,
      StatementTyped p (environment s.values) item.statement env ∧
      PlanFor s.machine.store.core.system.configuration.state s.values item.statement plan ∧
      advance s member place principal item = executePlan s member place principal item plan := by
  cases waiting : s.waiting with
  | some saved => simp [advance,waiting] at changed
  | none =>
      refine ⟨rfl,?_⟩
      cases typed : checkStatement p (environment s.values) item.statement with
      | none => simp [advance,waiting,typed] at changed
      | some env =>
          cases compiled : elaborate s.machine.store.core.system.configuration.state s.values item.statement with
          | none => simp [advance,waiting,typed,compiled] at changed
          | some plan =>
              exact ⟨env,plan,(statement_exact _ _ _ _).mp typed,(elaborate_exact _ _ _ _).mp compiled,
                by simp [advance,waiting,typed,compiled]⟩

theorem checked_reference_target (s : State p a) (out name : String) (expression : SourceAuthoring.Expr)
    (binding : Nat) (plan : Plan) (found : lookup s.values name = some (.reference binding))
    (meaning : PlanFor s.machine.store.core.system.configuration.state s.values (.plain (.invoke out name expression)) plan) :
    ∃ argument, Evaluates s.values expression argument ∧ plan = .call out (.reference binding) argument := by
  obtain ⟨argument,target,evaluated,named,rfl⟩ := meaning
  cases target with
  | «instance» key => simp [TargetMeans,found] at named
  | reference key =>
      have equal : key = binding := by simpa [TargetMeans,found,eq_comm] using named
      subst key
      exact ⟨argument,evaluated,rfl⟩

theorem step_pending (s next : State p a) (valid : PendingAgrees s) (step : Step s next) : PendingAgrees next := by
  cases step with
  | advance => exact advance_pending _ _ _ _ _ valid
  | cancellation => exact cancel_pending _ _ _ _ valid
  | complete => exact complete_pending _ valid
  | head => exact head_pending _ _ valid
  | control run => exact controlInput_pending _ _ _ _ _ _ valid run

theorem rooted_pending (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (s : State p a) (path : Rooted realm view policy s) : PendingAgrees s := by
  induction path with
  | refl => exact initial_pending _ _ _
  | step previous step ih => exact step_pending _ _ ih step

theorem rooted_single_pending (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (s : State p a) (path : Rooted realm view policy s) :
    (s.waiting = none ↔ s.machine.pending = []) ∧
      (∀ saved, s.waiting = some saved → s.machine.pending = [saved.entry]) := by
  have agrees := rooted_pending _ _ _ _ path
  cases waiting : s.waiting <;> simp_all [PendingAgrees]

theorem step_names (s next : State p a) (unique : ReferenceSourceTyping.Unique s)
    (fresh : ReferenceSourceTyping.WaitingFresh s) (step : Step s next) :
    ReferenceSourceTyping.Unique next ∧ ReferenceSourceTyping.WaitingFresh next := by
  cases step with
  | advance => exact ⟨ReferenceSourceTyping.advance_unique _ _ _ _ _ unique,ReferenceSourceTyping.advance_fresh _ _ _ _ _ fresh⟩
  | cancellation => exact ReferenceSourceTyping.cancel_names _ _ _ _ unique fresh
  | complete => exact ⟨ReferenceSourceTyping.complete_unique _ unique fresh,ReferenceSourceTyping.complete_fresh _ fresh⟩
  | head => exact ReferenceSourceTyping.head_names _ _ unique fresh
  | control run => exact ReferenceSourceTyping.controlInput_names _ _ _ _ _ _ unique fresh run

theorem rooted_names (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (s : State p a) (path : Rooted realm view policy s) :
    ReferenceSourceTyping.Unique s ∧ ReferenceSourceTyping.WaitingFresh s := by
  induction path with
  | refl => exact ReferenceSourceTyping.initial_names _ _ _
  | step previous step ih => exact step_names _ _ ih.1 ih.2 step

-- Useful source-state invariants after ANY finite sequence of actual entries,
-- including authority/control interleavings and failed normalization prefixes.
-- Locator validity and complete read/origin provenance remain distinct further
-- obligations; they are deliberately not inferred from this conjunction.
theorem rooted_source_invariants (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (s : State p a) (path : Rooted realm view policy s) :
    ReferenceExecution.Invariant s.machine ∧ Allocated s ∧ PendingAgrees s ∧
      ReferenceSourceTyping.Unique s ∧ ReferenceSourceTyping.WaitingFresh s := by
  exact ⟨(rooted_invariants _ _ _ _ path).1,(rooted_invariants _ _ _ _ path).2,
    rooted_pending _ _ _ _ path,rooted_names _ _ _ _ path⟩

#print axioms step_names
#print axioms rooted_names
#print axioms rooted_source_invariants
#print axioms step_pending
#print axioms rooted_pending
#print axioms rooted_single_pending
#print axioms run_reached
#print axioms rooted_invariants
#print axioms advance_effect_basis
#print axioms checked_reference_target
end MirroreaProofFirst.ReferenceSourceTrace
