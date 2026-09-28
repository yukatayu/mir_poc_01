import MirroreaProofFirstOwnerAssignment
/-! Unadopted LAB correspondence for lookup lookups. No source/runtime adoption. -/
namespace MirroreaProofFirst.OwnerPartial
open ProducerFlow
variable {K J : Type}

-- Existing expression syntax and arithmetic; missing lookup is a real failure.
def eval (ops : FallibleFlow.Arithmetic) (env : K → Option Value) : Expr K → Option Value
 | .lit v => some v
 | .read k => env k
 | .add a b => match eval ops env a, eval ops env b with
   | some (.int x), some (.int y) => (ops.add x y).map Value.int
   | _, _ => none
 | .sub a b => match eval ops env a, eval ops env b with
   | some (.int x), some (.int y) => (ops.sub x y).map Value.int
   | _, _ => none
 | .choose c a b => match eval ops env c with
   | some (.bool true) => eval ops env a
   | some (.bool false) => eval ops env b
   | _ => none

-- Sufficient static read coverage (all branches), not a claim that every runtime
-- success requires all branches present. The M8 subset has no conditional.
def Agrees (lookup : K → Option Value) (total : K → Value) : Expr K → Prop
 | .lit _ => True
 | .read k => lookup k = some (total k)
 | .add a b | .sub a b => Agrees lookup total a ∧ Agrees lookup total b
 | .choose c a b => Agrees lookup total c ∧ Agrees lookup total a ∧ Agrees lookup total b

theorem eval_refines (ops : FallibleFlow.Arithmetic) (lookup : K → Option Value)
 (total : K → Value) (e : Expr K) (covered : Agrees lookup total e) :
 eval ops lookup e = FallibleFlow.eval ops total e := by
 induction e with
 | lit => rfl
 | read k => exact covered
 | add a b ha hb =>
   simp only [eval, FallibleFlow.eval, ha covered.1, hb covered.2]
   split <;> simp_all
 | sub a b ha hb =>
   simp only [eval, FallibleFlow.eval, ha covered.1, hb covered.2]
   split <;> simp_all
 | choose c a b hc ha hb =>
   simp only [eval, FallibleFlow.eval, hc covered.1, ha covered.2.1, hb covered.2.2]
   split <;> simp_all

-- Presence belongs to the observation boundary; comparing only present values
-- would permit a low observable success/failure distinction.
def LowEq (labels : K → Nat) (level : Nat) (s t : K → Option Value) : Prop :=
 ∀ k, labels k ≤ level → s k = t k

theorem eval_low (ops : FallibleFlow.Arithmetic) {labels : K → Nat} {level : Nat}
 {s t : K → Option Value} (low : LowEq labels level s t) (e : Expr K)
 (flow : rank labels e ≤ level) : eval ops s e = eval ops t e := by
 induction e with
 | lit => rfl
 | read k => exact low k flow
 | add a b ha hb => simp only [eval,ha (Nat.max_le.mp flow).1,hb (Nat.max_le.mp flow).2]
 | sub a b ha hb => simp only [eval,ha (Nat.max_le.mp flow).1,hb (Nat.max_le.mp flow).2]
 | choose c a b hc ha hb =>
   have h := Nat.max_le.mp flow
   simp only [eval,hc h.1,ha (Nat.max_le.mp h.2).1,hb (Nat.max_le.mp h.2).2]

variable [DecidableEq K]
def put (s : K → Option Value) (target : K) (v : Value) : K → Option Value :=
 fun k => if k = target then some v else s k

def run (ops : FallibleFlow.Arithmetic) (s : K → Option Value) (args : J → Option Value)
 (target : K) (e : Expr (OwnerAssignment.Ref K J)) :
 (K → Option Value) × FallibleFlow.Outcome K :=
 match eval ops (Sum.elim s args) e with
 | none => (s,.failed target)
 | some v => (put s target v,.wrote target v)

theorem run_frame (ops : FallibleFlow.Arithmetic) (s : K → Option Value)
 (args : J → Option Value) (target : K) (e : Expr (OwnerAssignment.Ref K J))
 (k : K) (different : k ≠ target) : (run ops s args target e).1 k = s k := by
 unfold run; split <;> simp [put,different]

theorem run_failed (ops : FallibleFlow.Arithmetic) (s : K → Option Value)
 (args : J → Option Value) (target : K) (e : Expr (OwnerAssignment.Ref K J))
 (failed : eval ops (Sum.elim s args) e = none) :
 run ops s args target e = (s,.failed target) := by simp [run,failed]

theorem run_outcome_refines (ops : FallibleFlow.Arithmetic)
 (s : K → Option Value) (args : J → Option Value) (total : K → Value) (frozen : J → Value)
 (target : K) (e : Expr (OwnerAssignment.Ref K J))
 (covered : Agrees (Sum.elim s args) (OwnerAssignment.environment total frozen) e) :
 (run ops s args target e).2 = (OwnerAssignment.run ops total frozen target e).2 := by
 have same := eval_refines ops _ _ e covered
 cases result : FallibleFlow.eval ops (OwnerAssignment.environment total frozen) e <;>
   simp only [run,OwnerAssignment.run,same,result]

theorem run_low (ops : FallibleFlow.Arithmetic)
 {L : K → Nat} {SL : J → Nat} {level : Nat}
 {s t : K → Option Value} {args args' : J → Option Value}
 (low : LowEq L level s t) (argLow : LowEq SL level args args')
 (target : K) (e : Expr (OwnerAssignment.Ref K J))
 (flow : rank (OwnerAssignment.classes L SL) e ≤ L target) :
 LowEq L level (run ops s args target e).1 (run ops t args' target e).1 ∧
 FallibleFlow.project L level (run ops s args target e).2 =
 FallibleFlow.project L level (run ops t args' target e).2 := by
 by_cases visible : L target ≤ level
 · have both : LowEq (OwnerAssignment.classes L SL) level (Sum.elim s args) (Sum.elim t args') := by
     intro r hr
     cases r with
     | inl k => exact low k hr
     | inr j => exact argLow j hr
   have same := eval_low ops both e (Nat.le_trans flow visible)
   cases result : eval ops (Sum.elim t args') e with
   | none => simpa only [run,same,result] using And.intro low (Eq.refl (FallibleFlow.project L level (.failed target)))
   | some v =>
     simp only [run,same,result]
     constructor
     · intro k hk; simp only [put]; split
       · rfl
       · exact low k hk
     · trivial
 · constructor
   · intro k hk
     have different : k ≠ target := by intro eq; subst k; exact visible hk
     rw [run_frame _ _ _ _ _ _ different,run_frame _ _ _ _ _ _ different]
     exact low k hk
   · simp only [run]; split <;> split <;>
       simp [FallibleFlow.project,FallibleFlow.Outcome.key,visible]

namespace Controls
abbrev Cell := Fin 2
abbrev Arg := Fin 1
def source : Expr (OwnerAssignment.Ref Cell Arg) := .add (.read (.inl 0)) (.read (.inr 0))
def store (value : Option Int) : Cell → Option Value :=
 fun k => if k = 0 then value.map Value.int else some (.int 99)
def args (value : Option Int) : Arg → Option Value := fun _ => value.map Value.int
def execute (live argument : Option Int) := run (FallibleFlow.signed 63) (store live) (args argument) 0 source
example : (execute (some 3) (some 2)).2 = .wrote 0 (.int 5) := by decide
example : (execute none (some 2)).2 = .failed 0 := by decide
example : (execute (some 3) none).2 = .failed 0 := by decide
example : (execute (some 9223372036854775807) (some 1)).2 = .failed 0 := by decide
example : (execute (some 3) (some 2)).1 1 = some (.int 99) := by decide
example : (execute none (some 2)).1 0 = none := by decide
-- Equal values where both present do not imply equal public outcomes.
example : (execute none (some 2)).2 ≠ (execute (some 3) (some 2)).2 := by decide
end Controls
#print axioms eval_refines
#print axioms eval_low
#print axioms run_frame
#print axioms run_failed
#print axioms run_outcome_refines
#print axioms run_low
end MirroreaProofFirst.OwnerPartial
