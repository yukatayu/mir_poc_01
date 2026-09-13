import MirroreaProofFirstReferenceOwner
import MirroreaProofFirstReferenceCancellationBoundary

namespace MirroreaProofFirst.ReferenceExecution
-- Shared classified data is below the executor but above the core. The core
-- does not depend on the reference catalog; its exact ticket permit is nested.
structure Pending where
  ticket : InvocationBoundary.Ticket
  binding : Option ReferenceOwner.Binding
  deriving DecidableEq, Repr
end MirroreaProofFirst.ReferenceExecution

namespace MirroreaProofFirst.ReferenceCancellation
open ReferenceExecution
structure Permit where
  core : ReferenceCancellationBoundary.Permit
  pending : Pending
  frontier : Nat
  deriving DecidableEq, Repr

def check (s : ManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
    (member : Fin a) (place : Fin p) (principal id : Nat) (permit : Permit) : Bool :=
  decide (permit.pending = entry ∧ permit.frontier = frontier) &&
    ReferenceCancellationBoundary.check s coreFrontier entry.ticket member place principal id permit.core

def Submitted (s : ManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
    (member : Fin a) (place : Fin p) (principal id : Nat) (permit : Permit) : Prop :=
  permit.pending = entry ∧ permit.frontier = frontier ∧
    ReferenceCancellationBoundary.Submitted s coreFrontier entry.ticket member place principal id permit.core

theorem check_exact (s : ManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
    (member : Fin a) (place : Fin p) (principal id : Nat) (permit : Permit) :
    check s coreFrontier frontier entry member place principal id permit = true ↔
      Submitted s coreFrontier frontier entry member place principal id permit := by
  simp only [check,Bool.and_eq_true,decide_eq_true_eq,ReferenceCancellationBoundary.check_exact,Submitted,and_assoc]

def authorize (s : ManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
    (member : Fin a) (place : Fin p) (principal id : Nat) : Option Permit := do
  let core ← ReferenceCancellationBoundary.authorize s coreFrontier entry.ticket member place principal id
  return ⟨core,entry,frontier⟩

theorem authorize_checked (s : ManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
    (member : Fin a) (place : Fin p) (principal id : Nat) (permit : Permit)
    (accepted : authorize s coreFrontier frontier entry member place principal id = some permit) :
    check s coreFrontier frontier entry member place principal id permit = true := by
  unfold authorize at accepted
  cases run : ReferenceCancellationBoundary.authorize s coreFrontier entry.ticket member place principal id with
  | none => simp [run] at accepted
  | some core =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst permit
      simp [check,ReferenceCancellationBoundary.authorize_checked _ _ _ _ _ _ _ _ run]

theorem authorize_complete (s : ManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
    (member : Fin a) (place : Fin p) (principal id : Nat)
    (allowed : ReferenceCancellationBoundary.Allowed s coreFrontier entry.ticket member place principal id) :
    ∃ permit, authorize s coreFrontier frontier entry member place principal id = some permit := by
  obtain ⟨permit,run⟩ := ReferenceCancellationBoundary.authorize_complete _ _ _ _ _ _ _ allowed
  exact ⟨⟨permit,entry,frontier⟩,by simp [authorize,run]⟩

theorem classification_erasure_rejected (s : ManagementEntry.System p a) (coreFrontier frontier : Nat)
    (entry : Pending) (member : Fin a) (place : Fin p) (principal id : Nat) (permit : Permit)
    (different : permit.pending ≠ entry) :
    check s coreFrontier frontier entry member place principal id permit = false := by
  simp [check,different]

-- The actual consumer checks membership of this full saved entry, even if its
-- binding has since degraded, been reacquired, or been released. Equality with
-- the CURRENT binding and reference/invocation protection are not required.
#print axioms check_exact
#print axioms authorize_checked
#print axioms authorize_complete
#print axioms classification_erasure_rejected
end MirroreaProofFirst.ReferenceCancellation
