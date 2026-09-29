import MixedCancellation
import MixedReferenceOwner
import MirroreaProofFirstReferenceCancellation
namespace MirroreaProofFirst.MixedReferenceCancellation
open ReferenceExecution
abbrev Permit := ReferenceCancellation.Permit
-- Reuse the full saved reference classification and original nested pure permit.
def check (s : MixedManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
    (member : Fin a) (place : Fin p) (principal id : Nat) (permit : Permit) : Bool :=
  decide (permit.pending = entry ∧ permit.frontier = frontier) &&
    MixedCancellation.check s coreFrontier entry.ticket member place principal id permit.core

def Submitted (s : MixedManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
    (member : Fin a) (place : Fin p) (principal id : Nat) (permit : Permit) : Prop :=
  permit.pending = entry ∧ permit.frontier = frontier ∧
    MixedCancellation.Submitted s coreFrontier entry.ticket member place principal id permit.core

theorem check_exact (s : MixedManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
    (member : Fin a) (place : Fin p) (principal id : Nat) (permit : Permit) :
    check s coreFrontier frontier entry member place principal id permit = true ↔
      Submitted s coreFrontier frontier entry member place principal id permit := by
  simp only [check,Bool.and_eq_true,decide_eq_true_eq,MixedCancellation.check_exact,Submitted,and_assoc]

def authorize (s : MixedManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
    (member : Fin a) (place : Fin p) (principal id : Nat) : Option Permit := do
  let core ← MixedCancellation.authorize s coreFrontier entry.ticket member place principal id
  return ⟨core,entry,frontier⟩

theorem authorize_checked (s : MixedManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
    (member : Fin a) (place : Fin p) (principal id : Nat) (permit : Permit)
    (accepted : authorize s coreFrontier frontier entry member place principal id = some permit) :
    check s coreFrontier frontier entry member place principal id permit = true := by
  unfold authorize at accepted
  cases run : MixedCancellation.authorize s coreFrontier entry.ticket member place principal id with
  | none => simp [run] at accepted
  | some core =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst permit
      simp [check,MixedCancellation.authorize_checked _ _ _ _ _ _ _ _ run]

theorem authorize_complete (s : MixedManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
    (member : Fin a) (place : Fin p) (principal id : Nat)
    (allowed : MixedCancellation.Allowed s coreFrontier entry.ticket member place principal id) :
    ∃ permit, authorize s coreFrontier frontier entry member place principal id = some permit := by
  obtain ⟨permit,run⟩ := MixedCancellation.authorize_complete _ _ _ _ _ _ _ allowed
  exact ⟨⟨permit,entry,frontier⟩,by simp [authorize,run]⟩

theorem classification_erasure_rejected (s : MixedManagementEntry.System p a) (coreFrontier frontier : Nat)
    (entry : Pending) (member : Fin a) (place : Fin p) (principal id : Nat) (permit : Permit)
    (different : permit.pending ≠ entry) :
    check s coreFrontier frontier entry member place principal id permit = false := by
  simp [check,different]

theorem pure_check (s : ManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
 (member : Fin a) (place : Fin p) (principal id : Nat) (permit : Permit) :
 check (MixedManagementEmbedding.system s) coreFrontier frontier entry member place principal id permit =
 ReferenceCancellation.check s coreFrontier frontier entry member place principal id permit := rfl

theorem pure_authorize (s : ManagementEntry.System p a) (coreFrontier frontier : Nat) (entry : Pending)
 (member : Fin a) (place : Fin p) (principal id : Nat) :
 authorize (MixedManagementEmbedding.system s) coreFrontier frontier entry member place principal id =
 ReferenceCancellation.authorize s coreFrontier frontier entry member place principal id := rfl

#print axioms pure_check
#print axioms pure_authorize
-- The actual consumer checks membership of this full saved entry, even if its
-- binding has since degraded, been reacquired, or been released. Equality with
-- the CURRENT binding and reference/invocation protection are not required.
#print axioms check_exact
#print axioms authorize_checked
#print axioms authorize_complete
#print axioms classification_erasure_rejected
end MirroreaProofFirst.MixedReferenceCancellation
