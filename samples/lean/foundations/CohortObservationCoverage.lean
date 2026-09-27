import Std
namespace CohortObservationCoverage

-- A finite capture has independently bound rows [0,total). These rules describe
-- their consumption, not a claim that arbitrary observation values are honest.
inductive Rows : Nat → List Nat → Prop where
  | nil : Rows next []
  | cons : Rows (next+1) rest → Rows next (next::rest)

def checkFrom (next : Nat) : List Nat → Bool
  | [] => true
  | row::rest => row == next && checkFrom (next+1) rest

theorem checkFrom_rules : checkFrom next rows = true ↔ Rows next rows := by
  induction rows generalizing next with
  | nil => simp [checkFrom]; exact Rows.nil
  | cons row rest ih =>
    simp only [checkFrom, Bool.and_eq_true, beq_iff_eq, ih]
    constructor
    · rintro ⟨rfl,tail⟩; exact .cons tail
    · intro rule; cases rule with | cons tail => exact ⟨rfl,tail⟩

theorem rows_shape (rule : Rows next rows) : rows = List.range' next rows.length := by
  induction rule with
  | nil => rfl
  | @cons next rest tail ih => simpa [List.range'_succ] using congrArg (List.cons next) ih

theorem range_rows (next count : Nat) : Rows next (List.range' next count) := by
  induction count generalizing next with
  | zero => exact .nil
  | succ count ih => simpa [List.range'_succ] using Rows.cons (ih (next+1))

def Complete (total : Nat) (rows : List Nat) : Prop := Rows 0 rows ∧ rows.length = total

def check (total : Nat) (rows : List Nat) : Bool := checkFrom 0 rows && rows.length == total

theorem check_exact : check total rows = true ↔ Complete total rows := by
  simp [check,Complete,checkFrom_rules]

theorem complete_shape (done : Complete total rows) : rows = List.range total := by
  rw [List.range_eq_range',← done.2]; exact rows_shape done.1

theorem all_rows_accept (total : Nat) : check total (List.range total) = true := by
  apply check_exact.mpr
  exact ⟨by simpa [List.range_eq_range'] using range_rows 0 total,by simp⟩

theorem missing_refused (different : rows.length ≠ total) : check total rows = false := by
  cases h : check total rows with
  | false => rfl
  | true => exact False.elim (different (check_exact.mp h).2)

theorem altered_order_refused (different : rows ≠ List.range total) : check total rows = false := by
  cases h : check total rows with
  | false => rfl
  | true => exact False.elim (different (complete_shape (check_exact.mp h)))

theorem every_index (checked : check total rows = true) (index : Nat) : index ∈ rows ↔ index < total := by
  rw [complete_shape (check_exact.mp checked)]; exact List.mem_range

#print axioms checkFrom_rules
#print axioms rows_shape
#print axioms range_rows
#print axioms check_exact
#print axioms complete_shape
#print axioms all_rows_accept
#print axioms missing_refused
#print axioms altered_order_refused
#print axioms every_index
end CohortObservationCoverage
