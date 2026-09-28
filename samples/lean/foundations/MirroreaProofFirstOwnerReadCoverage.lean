import MirroreaProofFirstOwnerPartial
namespace MirroreaProofFirst.OwnerPartial
open ProducerFlow
variable {K : Type}

-- Existing syntax restricted to the strict M8 shape, not a new evaluator.
def Strict : Expr K → Prop
 | .lit _ | .read _ => True
 | .add a b | .sub a b => Strict a ∧ Strict b
 | .choose _ _ _ => False

def readKeys : Expr K → List K
 | .lit _ => []
 | .read k => [k]
 | .add a b | .sub a b => readKeys a ++ readKeys b
 | .choose c a b => readKeys c ++ readKeys a ++ readKeys b

theorem strict_success_covers (ops : FallibleFlow.Arithmetic) (env : K → Option Value)
 (e : Expr K) (strict : Strict e) (v : Value) (success : eval ops env e = some v) :
 ∀ k ∈ readKeys e, ∃ value, env k = some value := by
 induction e generalizing v with
 | lit => simp [readKeys]
 | read key =>
   intro k member
   have same : k = key := by simpa [readKeys] using member
   subst k
   exact ⟨v,success⟩
 | add a b ha hb =>
   cases left : eval ops env a with
   | none => simp [eval,left] at success
   | some av =>
     cases right : eval ops env b with
     | none => simp [eval,left,right] at success
     | some bv =>
       intro k member
       cases List.mem_append.mp member with
       | inl member => exact ha strict.1 av left k member
       | inr member => exact hb strict.2 bv right k member
 | sub a b ha hb =>
   cases left : eval ops env a with
   | none => simp [eval,left] at success
   | some av =>
     cases right : eval ops env b with
     | none => simp [eval,left,right] at success
     | some bv =>
       intro k member
       cases List.mem_append.mp member with
       | inl member => exact ha strict.1 av left k member
       | inr member => exact hb strict.2 bv right k member
 | choose => exact False.elim strict

-- Presence is necessary, never sufficient for arithmetic success.
example : eval (FallibleFlow.signed 63) (fun (_ : Nat) => some (.int 9223372036854775807))
 (.add (.read 0) (.lit (.int 1))) = none := by decide
-- The strictness premise matters: a successful branch need not read the other.
example : eval (FallibleFlow.signed 63) (fun (_ : Nat) => none)
 (.choose (.lit (.bool true)) (.lit (.int 1)) (.read 0)) = some (.int 1) := by decide
#print axioms strict_success_covers
end MirroreaProofFirst.OwnerPartial
