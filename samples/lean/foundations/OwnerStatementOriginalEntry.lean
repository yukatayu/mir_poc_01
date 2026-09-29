import OwnerStatementAcknowledgment
namespace MirroreaProofFirst.OwnerStatementOriginalEntry
universe u v
variable {α : Type u} {κ : Type v}

-- The payload is the WHOLE checked owner plan (signature/Core/site/owner),
-- not a mutable ordinal or an operation-name label. Original is independently
-- retained admitted data. Its constructor/patch/restore frame is a separate
-- obligation; this checker neither establishes that provenance nor authority.
def Admissible (ordinal : α → Option Nat) (original : List α)
 (bound : Bool) (candidate : α) : Prop :=
 bound = false ∧ candidate ∈ original ∧ ordinal candidate = none

def check [DecidableEq α] (ordinal : α → Option Nat) (original : List α)
 (bound : Bool) (candidate : α) : Bool :=
 !bound && original.any (fun prior => decide (prior = candidate ∧ ordinal prior = none))

variable {ordinal : α → Option Nat} {original next : List α} {bound : Bool}
variable {candidate prior : α} {operation : α → κ}

theorem check_exact [DecidableEq α] :
 check ordinal original bound candidate = true ↔ Admissible ordinal original bound candidate := by
 simp [check,Admissible,List.any_eq_true]
 intro _
 constructor
 · rintro ⟨prior,present,rfl,plain⟩
   exact ⟨present,plain⟩
 · rintro ⟨present,plain⟩
   exact ⟨candidate,present,rfl,plain⟩

def Unambiguous (operation : α → κ) (original : List α) : Prop :=
 ∀ first ∈ original, ∀ second ∈ original, operation first = operation second → first = second

-- This applies to every current candidate with the protected operation's
-- identity, even if its mutable ordinal has been erased or its Core replaced.
theorem protected_rejected [DecidableEq α]
 (unique : Unambiguous operation original) (stored : prior ∈ original)
 (guarded : ordinal prior ≠ none) (identity : operation candidate = operation prior) :
 check ordinal original bound candidate = false := by
 apply Bool.eq_false_iff.mpr
 intro accepted
 obtain ⟨_,present,plain⟩ := check_exact.mp accepted
 have same := unique candidate present prior stored identity
 subst candidate
 exact guarded plain

theorem bound_rejected [DecidableEq α] :
 check ordinal original true candidate = false := by simp [check]

theorem unknown_rejected [DecidableEq α] (absent : candidate ∉ original) :
 check ordinal original bound candidate = false := by
 apply Bool.eq_false_iff.mpr
 intro accepted
 exact absent (check_exact.mp accepted).2.1

-- Positive relative completeness is retained beside protected siblings.
-- This is not an all-refusing source-entry policy.
theorem ordinary_accepted [DecidableEq α]
 (stored : candidate ∈ original) (plain : ordinal candidate = none) :
 check ordinal original false candidate = true :=
 check_exact.mpr ⟨rfl,stored,plain⟩

theorem preserved_original_addition [DecidableEq α]
 (accepted : check ordinal original bound candidate = true)
 (retained : ∀ plan ∈ original, plan ∈ next) :
 check ordinal next bound candidate = true := by
 obtain ⟨free,stored,plain⟩ := check_exact.mp accepted
 exact check_exact.mpr ⟨free,retained candidate stored,plain⟩

-- Admission monotonicity above grants no permission to add an arbitrary plan:
-- authenticated checked addition and no-shadow uniqueness remain independent.
#print axioms check_exact
#print axioms protected_rejected
#print axioms bound_rejected
#print axioms unknown_rejected
#print axioms ordinary_accepted
#print axioms preserved_original_addition
end MirroreaProofFirst.OwnerStatementOriginalEntry
