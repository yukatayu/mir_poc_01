import MirroreaProofFirstReferenceAccess

namespace MirroreaProofFirst.ReferenceAccessHistory
open ReferenceAccess

-- A finite local history window starts when this exact guard is acquired.
-- Every subsequent admitted semantic state is included. This is a reference
-- representation, not a physical log, synchronization or durable-store claim.
-- Selection is owner state; recording an authority input is not an owner
-- mutation and cannot advance a selection or mint permission.
structure Frame (p a : Nat) where
  system : ManagementEntry.System p a
  policy : CurrentUse.Policy

def checks (frame : Frame p a) (request : Request) (guard : Guard) : Bool :=
  ReferenceAccess.check frame.system frame.policy request guard

def Meaning (frame : Frame p a) (request : Request) (guard : Guard) : Prop :=
  Saved frame.system frame.policy request guard

inductive Continuous (request : Request) (guard : Guard) : List (Frame p a) → Prop where
  | nil : Continuous request guard []
  | cons : Meaning frame request guard → Continuous request guard rest →
      Continuous request guard (frame :: rest)

def continuousCheck (history : List (Frame p a)) (request : Request) (guard : Guard) : Bool :=
  history.all (fun frame => checks frame request guard)

theorem continuous_exact (history : List (Frame p a)) (request : Request) (guard : Guard) :
    continuousCheck history request guard = true ↔ Continuous request guard history := by
  induction history with
  | nil => simp only [continuousCheck,List.all_nil,true_iff]; exact .nil
  | cons frame rest ih =>
      simp only [continuousCheck,List.all_cons,Bool.and_eq_true,checks,check_exact]
      change (Meaning frame request guard ∧ continuousCheck rest request guard = true) ↔ _
      rw [ih]
      exact ⟨fun ⟨head,tail⟩ => .cons head tail,fun h => by cases h with | cons head tail => exact ⟨head,tail⟩⟩

theorem append_check (older newer : List (Frame p a)) (request : Request) (guard : Guard) :
    continuousCheck (older ++ newer) request guard =
      (continuousCheck older request guard && continuousCheck newer request guard) := by
  simp [continuousCheck,List.all_append]

theorem invalidation_persists (older newer : List (Frame p a)) (request : Request) (guard : Guard)
    (invalid : continuousCheck older request guard = false) :
    continuousCheck (older ++ newer) request guard = false := by
  simp [append_check,invalid]

theorem member_valid (history : List (Frame p a)) (request : Request) (guard : Guard)
    (valid : continuousCheck history request guard = true) (frame : Frame p a) (mem : frame ∈ history) :
    Meaning frame request guard := by
  exact (check_exact _ _ _ _).mp ((List.all_eq_true.mp valid) frame mem)

theorem invalid_member_rejects (history : List (Frame p a)) (request : Request) (guard : Guard)
    (frame : Frame p a) (mem : frame ∈ history) (invalid : checks frame request guard = false) :
    continuousCheck history request guard = false := by
  cases h : continuousCheck history request guard with
  | false => rfl
  | true =>
      have accepted := (List.all_eq_true.mp h) frame mem
      simp [invalid] at accepted

-- The consumer must use a nonempty current window. The empty-list induction
-- base cannot itself authorize access or hide the absence of the current head.
def usable (current : Frame p a) (past : List (Frame p a)) (request : Request) (guard : Guard) : Bool :=
  continuousCheck (past ++ [current]) request guard

theorem usable_current (current : Frame p a) (past : List (Frame p a)) (request : Request) (guard : Guard)
    (valid : usable current past request guard = true) : Meaning current request guard := by
  exact member_valid _ _ _ valid current (by simp)

theorem usable_no_resurrection (current : Frame p a) (past : List (Frame p a)) (request : Request) (guard : Guard)
    (invalid : continuousCheck past request guard = false) : usable current past request guard = false :=
  invalidation_persists _ _ _ _ invalid

#print axioms continuous_exact
#print axioms append_check
#print axioms invalidation_persists
#print axioms member_valid
#print axioms invalid_member_rejects
#print axioms usable_current
#print axioms usable_no_resurrection
end MirroreaProofFirst.ReferenceAccessHistory
