import MirroreaProofFirstOwnerReadCoverage
namespace MirroreaProofFirst.OwnerCheckedArithmetic
open ProducerFlow
variable {K J : Type}

/-! LAB algorithm/extraction obligation for the actual strict checked tree.
This reuses OwnerPartial evaluation; Checked models the existing Rust enum.
It does NOT prove Rust compiler, key resolution, parsing, authority or i64 leaf
bounds. State lookup includes owner/key-resolution failure; argument lookup is
AFTER the actual i64 parser. Scratch reads on failure are intentionally erased. -/
inductive Checked (K J : Type) where
 | state (key : K)
 | parameter (name : J)
 | integer (value : Int)
 | add (left right : Checked K J)
 | sub (left right : Checked K J)

def translate : Checked K J → Expr (Sum K J)
 | .state k => .read (.inl k)
 | .parameter j => .read (.inr j)
 | .integer n => .lit (.int n)
 | .add l r => .add (translate l) (translate r)
 | .sub l r => .sub (translate l) (translate r)

-- Independently matches the five concrete evaluator cases and error propagation.
def evaluate (ops : FallibleFlow.Arithmetic) (store : K → Option Int)
 (args : J → Option Int) : Checked K J → Option Int
 | .state k => store k
 | .parameter j => args j
 | .integer n => some n
 | .add l r => do
     let x ← evaluate ops store args l
     let y ← evaluate ops store args r
     ops.add x y
 | .sub l r => do
     let x ← evaluate ops store args l
     let y ← evaluate ops store args r
     ops.sub x y

def environment (store : K → Option Int) (args : J → Option Int) : Sum K J → Option Value :=
 Sum.elim (fun k => (store k).map Value.int) (fun j => (args j).map Value.int)

theorem extraction (ops : FallibleFlow.Arithmetic) (store : K → Option Int)
 (args : J → Option Int) (tree : Checked K J) :
 OwnerPartial.eval ops (environment store args) (translate tree) =
 (evaluate ops store args tree).map Value.int := by
 induction tree with
 | state => rfl
 | parameter => rfl
 | integer => rfl
 | add l r hl hr =>
   simp only [translate,OwnerPartial.eval,hl,hr,evaluate]
   cases left : evaluate ops store args l <;> cases right : evaluate ops store args r <;> rfl
 | sub l r hl hr =>
   simp only [translate,OwnerPartial.eval,hl,hr,evaluate]
   cases left : evaluate ops store args l <;> cases right : evaluate ops store args r <;> rfl

theorem strict_translation (tree : Checked K J) : OwnerPartial.Strict (translate tree) := by
 induction tree with
 | state => trivial
 | parameter => trivial
 | integer => trivial
 | add l r hl hr => exact ⟨hl,hr⟩
 | sub l r hl hr => exact ⟨hl,hr⟩

theorem failed_extraction (ops : FallibleFlow.Arithmetic) (store : K → Option Int)
 (args : J → Option Int) (tree : Checked K J) :
 OwnerPartial.eval ops (environment store args) (translate tree) = none ↔
 evaluate ops store args tree = none := by
 rw [extraction]
 cases evaluate ops store args tree <;> simp

-- No read-coverage premise: missing state and failed argument parse are included.
example : evaluate (FallibleFlow.signed 63) (fun (_ : Nat) => none)
 (fun (_ : Nat) => some 2) (.add (.state 0) (.parameter 0)) = none := by decide
example : evaluate (FallibleFlow.signed 63) (fun (_ : Nat) => some 3)
 (fun (_ : Nat) => some 2) (.add (.state 0) (.parameter 0)) = some 5 := by decide
#print axioms extraction
#print axioms strict_translation
#print axioms failed_extraction
end MirroreaProofFirst.OwnerCheckedArithmetic
