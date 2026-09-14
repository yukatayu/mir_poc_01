import MirroreaProofFirstOwnerValidity

namespace MirroreaProofFirst.OwnerEvaluator
open InstancePrograms

-- This coordinate is supplied by the endpoint launcher, separately from a
-- received image/ticket. It is not an authentication or disclosure credential.
structure Assignment (p : Nat) where
  realm : Nat
  place : Fin p

inductive Failure where
  | wrongRealm | invalidConfiguration | unauthorized | executionFailure
  deriving DecidableEq, Repr
inductive Result where
  | rejected (reason : Failure)
  | value (result : Int)
  deriving DecidableEq, Repr

def Admitted (assigned : Assignment p) (image : OwnerImage.Image p a)
    (ticket : InvocationBoundary.Ticket) : Prop :=
  image.state.realm = assigned.realm ∧ image.view.realm = assigned.realm ∧
  CompositionCore.Invariant (OwnerImage.restore image).configuration ∧
  OwnerProjection.Allowed (OwnerImage.restore image) assigned.place ticket

def run (assigned : Assignment p) (image : OwnerImage.Image p a)
    (ticket : InvocationBoundary.Ticket) : Result :=
  if image.state.realm ≠ assigned.realm ∨ image.view.realm ≠ assigned.realm then .rejected .wrongRealm
  else if !OwnerValidity.check image then .rejected .invalidConfiguration
  else if !OwnerProjection.check (OwnerImage.restore image) assigned.place ticket then .rejected .unauthorized
  else match InvocationBoundary.execute ticket with
    | none => .rejected .executionFailure
    | some result => .value result

theorem result_exact (assigned : Assignment p) (image : OwnerImage.Image p a)
    (ticket : InvocationBoundary.Ticket) (value : Int) :
    run assigned image ticket = .value value ↔
      Admitted assigned image ticket ∧ Machine.Executes ticket.definition.code ticket.argument value := by
  by_cases hr : image.state.realm = assigned.realm ∧ image.view.realm = assigned.realm
  · by_cases hv : OwnerValidity.check image = true
    · by_cases ha : OwnerProjection.check (OwnerImage.restore image) assigned.place ticket = true
      · have admitted : Admitted assigned image ticket :=
          ⟨hr.1,hr.2,(OwnerValidity.check_exact _).mp hv,(OwnerProjection.check_exact _ _ _).mp ha⟩
        simp only [admitted,true_and]
        rw [← Machine.run_exact]
        cases executed : Machine.run ticket.definition.code ticket.argument <;>
          simp [run,hr.1,hr.2,hv,ha,InvocationBoundary.execute,executed]
      · have denied : ¬ OwnerProjection.Allowed (OwnerImage.restore image) assigned.place ticket :=
          fun allowed => ha ((OwnerProjection.check_exact _ _ _).mpr allowed)
        simp [run,Admitted,hr.1,hr.2,hv,ha,denied]
    · have invalid : ¬ CompositionCore.Invariant (OwnerImage.restore image).configuration :=
        fun valid => hv ((OwnerValidity.check_exact _).mpr valid)
      simp [run,Admitted,hr.1,hr.2,hv,invalid]
  · have wrong : image.state.realm ≠ assigned.realm ∨ image.view.realm ≠ assigned.realm := by
      by_cases hs : image.state.realm = assigned.realm
      · exact Or.inr (fun h => hr ⟨hs,h⟩)
      · exact Or.inl hs
    have denied : ¬ Admitted assigned image ticket := fun h => hr ⟨h.1,h.2.1⟩
    simp [run,wrong,denied]

theorem admitted_completes (assigned : Assignment p) (image : OwnerImage.Image p a)
    (ticket : InvocationBoundary.Ticket) (admitted : Admitted assigned image ticket) :
    ∃ value, run assigned image ticket = .value value ∧ Output ticket.definition.contract value := by
  obtain ⟨value,served,output⟩ := OwnerProjection.admitted_serve_exists
    (OwnerImage.restore image) assigned.place ticket admitted.2.2.1 admitted.2.2.2
  exact ⟨value,(result_exact _ _ _ _).mpr ⟨admitted,(OwnerProjection.serve_exact.mp served).2⟩,output⟩

theorem rejection_exact (assigned : Assignment p) (image : OwnerImage.Image p a)
    (ticket : InvocationBoundary.Ticket) :
    (∃ reason, run assigned image ticket = .rejected reason) ↔ ¬ Admitted assigned image ticket := by
  constructor
  · rintro ⟨reason,rejected⟩ admitted
    obtain ⟨value,completed,_⟩ := admitted_completes assigned image ticket admitted
    rw [rejected] at completed
    cases completed
  · intro denied
    cases outcome : run assigned image ticket with
    | rejected reason => exact ⟨reason,rfl⟩
    | value value => exact False.elim (denied ((result_exact _ _ _ _).mp outcome).1)

theorem no_execution_failure (assigned : Assignment p) (image : OwnerImage.Image p a)
    (ticket : InvocationBoundary.Ticket) : run assigned image ticket ≠ .rejected .executionFailure := by
  intro failed
  have rejected := failed
  unfold run at failed
  split at failed
  · cases failed
  · rename_i realm
    split at failed
    · cases failed
    · rename_i valid
      split at failed
      · cases failed
      · rename_i allowed
        have hr : image.state.realm = assigned.realm ∧ image.view.realm = assigned.realm := by
          simpa only [not_or,Decidable.not_not] using realm
        have hv : OwnerValidity.check image = true := by simpa using valid
        have ha : OwnerProjection.check (OwnerImage.restore image) assigned.place ticket = true := by simpa using allowed
        exact (rejection_exact _ _ _).mp ⟨_,rejected⟩
          ⟨hr.1,hr.2,(OwnerValidity.check_exact _).mp hv,(OwnerProjection.check_exact _ _ _).mp ha⟩

#print axioms result_exact
#print axioms admitted_completes
#print axioms rejection_exact
#print axioms no_execution_failure
end MirroreaProofFirst.OwnerEvaluator
