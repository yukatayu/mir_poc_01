/-!
current-l2 first IFC / label-model fragment

This file is intentionally small and self-contained. It is not the final public type
system. It fixes only the minimal lattice and authority-sensitive lemmas needed for the
current checker-adjacent reading.
-/

namespace CurrentL2

inductive SecurityLabel where
  | low
  | high
deriving DecidableEq, Repr

open SecurityLabel

def flowsTo : SecurityLabel → SecurityLabel → Prop
  | low, _ => True
  | high, high => True
  | high, low => False

def join : SecurityLabel → SecurityLabel → SecurityLabel
  | high, _ => high
  | _, high => high
  | low, low => low

def CanDeclassify (hasAuthority : Bool) (fromLabel toLabel : SecurityLabel) : Prop :=
  hasAuthority = true ∨ flowsTo fromLabel toLabel

theorem flowsTo_refl (label : SecurityLabel) : flowsTo label label := by
  cases label <;> simp [flowsTo]

theorem flowsTo_trans
    {a b c : SecurityLabel}
    (hab : flowsTo a b)
    (hbc : flowsTo b c) :
    flowsTo a c := by
  cases a <;> cases b <;> cases c <;> simp [flowsTo] at hab hbc ⊢

theorem flowsTo_join_left (a b : SecurityLabel) : flowsTo a (join a b) := by
  cases a <;> cases b <;> simp [flowsTo, join]

theorem flowsTo_join_right (a b : SecurityLabel) : flowsTo b (join a b) := by
  cases a <;> cases b <;> simp [flowsTo, join]

theorem secret_to_public_requires_authority :
    ¬ flowsTo high low := by
  simp [flowsTo]

theorem no_declassify_without_authority :
    ¬ CanDeclassify false high low := by
  simp [CanDeclassify, flowsTo]

theorem authority_enables_secret_to_public :
    CanDeclassify true high low := by
  simp [CanDeclassify]

end CurrentL2
