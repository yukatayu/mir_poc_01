import Std
namespace MirroreaProofFirst.OperationIdentity
variable {A : Type}

-- Declarative positional rules: exactly one matching occurrence, even when two
-- entries are equal. Distinct value uniqueness alone would lose this condition.
inductive Selects (matchKey : A → Bool) : List A → A → Prop where
 | here (hit : matchKey x = true) (rest : ∀ y ∈ xs, matchKey y = false) :
     Selects matchKey (x :: xs) x
 | skip (miss : matchKey x = false) (tail : Selects matchKey xs y) :
     Selects matchKey (x :: xs) y

-- Mirrors the finite iterator's first matching item / reject second match.
def resolve (matchKey : A → Bool) (xs : List A) : Option A :=
 match xs.filter matchKey with
 | [x] => some x
 | _ => none

theorem filter_empty_iff (matchKey : A → Bool) (xs : List A) :
 xs.filter matchKey = [] ↔ ∀ y ∈ xs, matchKey y = false := by
 induction xs with
 | nil => simp
 | cons x xs ih =>
   cases h : matchKey x <;> simp [h,ih]

theorem declarative_filter (matchKey : A → Bool) (xs : List A) (x : A) :
 Selects matchKey xs x ↔ xs.filter matchKey = [x] := by
 constructor
 · intro selected
   induction selected with
   | here hit rest => simp [hit, (filter_empty_iff _ _).mpr rest]
   | skip miss _ ih => simpa [miss] using ih
 · induction xs with
   | nil => simp
   | cons y ys ih =>
     intro equal
     cases hit : matchKey y with
     | false => exact .skip hit (ih (by simpa [hit] using equal))
     | true =>
       have parts : y = x ∧ ys.filter matchKey = [] := by simpa [hit] using equal
       obtain ⟨rfl,rest⟩ := parts
       exact .here hit ((filter_empty_iff _ _).mp rest)

theorem resolve_exact (matchKey : A → Bool) (xs : List A) (x : A) :
 resolve matchKey xs = some x ↔ Selects matchKey xs x := by
 rw [declarative_filter]
 unfold resolve
 cases filtered : xs.filter matchKey with
 | nil => simp
 | cons y ys =>
   cases ys with
   | nil => simp
   | cons z zs => simp

theorem selected_member (matchKey : A → Bool) (xs : List A) (x : A)
 (selected : Selects matchKey xs x) : x ∈ xs ∧ matchKey x = true := by
 induction selected with
 | here hit _ => exact ⟨by simp,hit⟩
 | skip _ _ ih => exact ⟨List.mem_cons_of_mem _ ih.1,ih.2⟩

theorem matching_singleton_accepted (matchKey : A → Bool) (x : A)
 (hit : matchKey x = true) : resolve matchKey [x] = some x := by simp [resolve,hit]

theorem repeated_occurrence_refused (matchKey : A → Bool) (x : A)
 (hit : matchKey x = true) : resolve matchKey [x,x] = none := by simp [resolve,hit]

-- Existing Rust matcher includes operation name, kind and owner. This lemma is
-- parametric in that predicate; it does not prove those fields are authentic,
-- or that per-statement identities / continuations have been elaborated.
#print axioms resolve_exact
#print axioms selected_member
#print axioms matching_singleton_accepted
#print axioms repeated_occurrence_refused
end MirroreaProofFirst.OperationIdentity
