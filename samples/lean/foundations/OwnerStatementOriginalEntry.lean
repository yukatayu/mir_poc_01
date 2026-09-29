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

namespace MirroreaProofFirst.OwnerStatementOriginalEntry
variable {α : Type u} {κ : Type v}

def selectSource [DecidableEq κ] (handler : α → κ) (original : List α) (name : κ) : List α :=
 original.filter (fun plan => decide (handler plan = name))

-- Declarative positional agreement, including the absence of every extra
-- position. The independently admitted original inventory supplies order.
-- Neither an authentic subset nor a prefix establishes source completeness.
def ManifestAdmissible [DecidableEq κ] (handler : α → κ)
 (original manifest : List α) : Prop :=
 manifest ≠ [] ∧ ∀ first, manifest.head? = some first →
   ∀ i : Nat, manifest[i]? = (selectSource handler original (handler first))[i]?

def manifestCheck [DecidableEq α] [DecidableEq κ] (handler : α → κ)
 (original manifest : List α) : Bool :=
 match manifest with
 | [] => false
 | first :: rest => decide (first :: rest = selectSource handler original (handler first))

theorem manifestCheck_exact [DecidableEq α] [DecidableEq κ]
 (handler : α → κ) (original manifest : List α) :
 manifestCheck handler original manifest = true ↔ ManifestAdmissible handler original manifest := by
 cases manifest with
 | nil => simp [manifestCheck,ManifestAdmissible]
 | cons first rest =>
   simp only [manifestCheck,decide_eq_true_eq]
   constructor
   · intro same
     refine ⟨by simp, ?_⟩
     intro head firstEq i
     have headEq : first = head := by simpa using firstEq
     subst head
     rw [same]
   · intro ready
     apply List.ext_getElem?
     intro i
     exact ready.2 first rfl i

theorem manifest_ready_index [DecidableEq α] [DecidableEq κ]
 (handler : α → κ) (original manifest : List α) (first : α)
 (accepted : manifestCheck handler original manifest = true)
 (head : manifest.head? = some first) (i : Nat) :
 manifest[i]? = (selectSource handler original (handler first))[i]? :=
 (manifestCheck_exact handler original manifest).mp accepted |>.2 first head i

theorem manifest_rejects_different_list [DecidableEq α] [DecidableEq κ]
 (handler : α → κ) (original : List α) (first : α) (rest : List α)
 (different : first :: rest ≠ selectSource handler original (handler first)) :
 manifestCheck handler original (first :: rest) = false := by
 simp [manifestCheck,different]

theorem manifest_rejects_length_loss [DecidableEq α] [DecidableEq κ]
 (handler : α → κ) (original : List α) (first : α) (rest : List α)
 (different : (first :: rest).length ≠ (selectSource handler original (handler first)).length) :
 manifestCheck handler original (first :: rest) = false := by
 apply manifest_rejects_different_list
 intro same
 exact different (congrArg List.length same)

theorem manifest_original_accepted [DecidableEq α] [DecidableEq κ]
 (handler : α → κ) (original : List α) (name : κ)
 (nonempty : selectSource handler original name ≠ []) :
 manifestCheck handler original (selectSource handler original name) = true := by
 cases selected : selectSource handler original name with
 | nil => exact False.elim (nonempty selected)
 | cons first rest =>
   have member : first ∈ selectSource handler original name := by rw [selected]; simp
   have same : handler first = name := by
     simpa [selectSource] using (List.mem_filter.mp member).2
   simp [manifestCheck,same,selected]

-- Positional exactness is not origin authentication: decoding and trusted
-- compiler/admission promotion, no-shadow, code/currentness and patch frames
-- remain distinct obligations. The checker grants no authority.
#print axioms manifestCheck_exact
#print axioms manifest_ready_index
#print axioms manifest_rejects_different_list
#print axioms manifest_rejects_length_loss
#print axioms manifest_original_accepted
end MirroreaProofFirst.OwnerStatementOriginalEntry
