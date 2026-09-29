import OwnerFlowComposition
namespace MirroreaProofFirst.OwnerNumericBoundary
open OwnerCheckedArithmetic OwnerEffectService
variable {K J : Type}

-- Leaf bounds are explicit. Checked operators alone do not validate an unused
-- argument or a literal; Rust i64 parsing/storage must realize these premises.
abbrev Range (bits : Nat) (v : Int) : Prop := -(2^bits : Int) ≤ v ∧ v < (2^bits : Int)
def Bounded (bits : Nat) (store : K → Option Int) : Prop :=
 ∀ key value, store key = some value → Range bits value

def Literals (bits : Nat) : Checked K J → Prop
 | .state _ | .parameter _ => True
 | .integer value => Range bits value
 | .add l r | .sub l r => Literals bits l ∧ Literals bits r

def literalCheck (bits : Nat) : Checked K J → Bool
 | .state _ | .parameter _ => true
 | .integer value => decide (Range bits value)
 | .add l r | .sub l r => literalCheck bits l && literalCheck bits r

theorem literals_exact (tree : Checked K J) : literalCheck bits tree = true ↔ Literals bits tree := by
 induction tree <;> simp_all [literalCheck,Literals]

theorem bounded_result (h : FallibleFlow.bounded bits v = some result) : Range bits result := by
 obtain ⟨rfl,lo,hi⟩ := FallibleFlow.bounded_exact _ _ _ |>.mp h
 exact ⟨lo,hi⟩

theorem evaluate_bounded (store : K → Option Int) (args : J → Option Int)
 (tree : Checked K J) (state : Bounded bits store) (captured : Bounded bits args)
 (literals : Literals bits tree)
 (success : evaluate (FallibleFlow.signed bits) store args tree = some value) : Range bits value := by
 cases tree with
 | state key => exact state key value success
 | parameter key => exact captured key value success
 | integer v => cases success; exact literals
 | add l r =>
   simp only [evaluate] at success
   cases left : evaluate (FallibleFlow.signed bits) store args l with
   | none => simp [left] at success
   | some x =>
     cases right : evaluate (FallibleFlow.signed bits) store args r with
     | none => simp [left,right] at success
     | some y =>
       simp only [left,right,Option.bind_eq_bind,Option.bind_some] at success
       exact bounded_result success
 | sub l r =>
   simp only [evaluate] at success
   cases left : evaluate (FallibleFlow.signed bits) store args l with
   | none => simp [left] at success
   | some x =>
     cases right : evaluate (FallibleFlow.signed bits) store args r with
     | none => simp [left,right] at success
     | some y =>
       simp only [left,right,Option.bind_eq_bind,Option.bind_some] at success
       exact bounded_result success

theorem put_bounded {store : Nat → Option Int} (state : Bounded bits store) (value : Range bits v) :
 Bounded bits (OwnerEffectService.put store target v) := by
 intro key result found
 by_cases same : key=target
 · have eq : v = result := by simpa [OwnerEffectService.put,same] using found
   simpa [←eq] using value
 · exact state key result (by simpa [OwnerEffectService.put,same] using found)

-- Current authorization/code matching and actual recorded evaluation are still
-- independently required by serve; range premises do not create permissions.
theorem service_bounded (state : Bounded bits s.store) (captured : Bounded bits (OwnerEffectService.args e))
 (literals : Literals bits e.body.tree) :
 Bounded bits (serve (FallibleFlow.signed bits) world registry s e).1.store := by
 cases outcome : (serve (FallibleFlow.signed bits) world registry s e).2 with
 | refused why => rw [refused_retains outcome]; exact state
 | committed out =>
   rw [(committed_state outcome).1]
   apply put_bounded state
   exact evaluate_bounded s.store (OwnerEffectService.args e) e.body.tree state captured literals
     (committed_exact.mp outcome).2.2.1

theorem committed_value_bounded (state : Bounded bits s.store)
 (captured : Bounded bits (OwnerEffectService.args e)) (literals : Literals bits e.body.tree)
 (commit : (serve (FallibleFlow.signed bits) world registry s e).2 = .committed out) : Range bits out.value :=
 evaluate_bounded s.store (OwnerEffectService.args e) e.body.tree state captured literals
   (committed_exact.mp commit).2.2.1

-- This exact boundary is [-2^63,2^63), not an arbitrary mathematical Int claim.
theorem i64_range (v : Int) : Range 63 v ↔ InstancePrograms.Machine.lo ≤ v ∧ v ≤ InstancePrograms.Machine.hi := by
 change (-9223372036854775808 ≤ v ∧ v < 9223372036854775808) ↔
   (-9223372036854775808 ≤ v ∧ v ≤ 9223372036854775807)
 omega

namespace Controls
def hi : Int := 2^63-1
def noValues (_ : Nat) : Option Int := none
#guard evaluate (FallibleFlow.signed 63) noValues noValues (.integer (hi+1)) = some (hi+1)
#guard !literalCheck 63 (Checked.integer (K:=Nat) (J:=Nat) (hi+1))
#guard literalCheck 63 (Checked.integer (K:=Nat) (J:=Nat) hi)
#guard evaluate (FallibleFlow.signed 63) (fun (_ : Nat) => some hi) noValues
 (.add (.state 0) (.integer 1)) = none
#guard evaluate (FallibleFlow.signed 63) (fun (_ : Nat) => none) noValues (.state 0) = none
#guard evaluate (FallibleFlow.signed 63) noValues (fun (_ : Nat) => some (hi+1)) (.integer 1) = some 1
-- The last positive does not discharge the external argument boundary.
end Controls
#print axioms literals_exact
#print axioms evaluate_bounded
#print axioms put_bounded
#print axioms service_bounded
#print axioms committed_value_bounded
#print axioms i64_range
end MirroreaProofFirst.OwnerNumericBoundary
