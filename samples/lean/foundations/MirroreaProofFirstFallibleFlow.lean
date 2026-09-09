import MirroreaProofFirstProducerFlow
/-! Task-local LAB fixed-invocation fallible assignment; no sequential/runtime promotion. -/
namespace MirroreaProofFirst.FallibleFlow
open ProducerFlow
variable {K : Type}
-- Operators may fail; their deterministic implementation is an explicit parameter.
structure Arithmetic where
 add : Int → Int → Option Int
 sub : Int → Int → Option Int
def eval (ops : Arithmetic) (env : K → Value) : Expr K → Option Value
 | .lit v => some v
 | .read k => some (env k)
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
 theorem eval_low (ops : Arithmetic) {L : K → Nat} {level} {s t : K → Value}
 (low : LowEq L level s t) (e : Expr K) (h : rank L e ≤ level) :
 eval ops s e = eval ops t e := by
  induction e with
  | lit => rfl
  | read k => exact congrArg some (low k h)
  | add a b ha hb => simp only [eval, ha (Nat.max_le.mp h).1, hb (Nat.max_le.mp h).2]
  | sub a b ha hb => simp only [eval, ha (Nat.max_le.mp h).1, hb (Nat.max_le.mp h).2]
  | choose c a b hc ha hb =>
    have hh := Nat.max_le.mp h
    have hab := Nat.max_le.mp hh.2
    simp only [eval, hc hh.1, ha hab.1, hb hab.2]
-- Failure is a real outcome, never fabricated as a successful write.
inductive Outcome (K : Type) where
 | failed (key : K)
 | wrote (key : K) (value : Value)
 deriving DecidableEq, Repr
def Outcome.key : Outcome K → K | .failed k => k | .wrote k _ => k
variable [DecidableEq K]
def step (ops : Arithmetic) (s : K → Value) (k : K) (e : Expr K) :
 (K → Value) × Outcome K :=
 match eval ops s e with
 | none => (s, .failed k)
 | some v => (put s k v, .wrote k v)
def project (L : K → Nat) (level : Nat) (outcome : Outcome K) : Option (Outcome K) :=
 if L outcome.key ≤ level then some outcome else none
 theorem step_noninterference (ops : Arithmetic) {L : K → Nat} {level} {s t : K → Value}
 (low : LowEq L level s t) (k : K) (e : Expr K) (flow : rank L e ≤ L k) :
 LowEq L level (step ops s k e).1 (step ops t k e).1 ∧
 project L level (step ops s k e).2 = project L level (step ops t k e).2 := by
  by_cases hk : L k ≤ level
  · have ev := eval_low ops low e (Nat.le_trans flow hk)
    simp only [step, ev]
    split
    next => exact ⟨low, rfl⟩
    next v hv =>
      constructor
      · intro x hx; simp only [put]; split
        next => rfl
        next => exact low x hx
      · rfl
  · constructor
    · intro x hx
      have hn : x ≠ k := by intro eq; subst x; exact hk hx
      simp only [step]
      split <;> split <;> simpa [put, hn] using low x hx
    · simp only [step]
      split <;> split <;> simp [project, Outcome.key, hk]
def bounded (bits : Nat) (x : Int) : Option Int :=
 if -(2 ^ bits : Int) ≤ x ∧ x < (2 ^ bits : Int) then some x else none
theorem bounded_exact (bits : Nat) (x y : Int) :
 bounded bits x = some y ↔ y = x ∧ -(2 ^ bits : Int) ≤ x ∧ x < (2 ^ bits : Int) := by
  by_cases h : -(2 ^ bits : Int) ≤ x ∧ x < (2 ^ bits : Int)
  · simp [bounded, h, eq_comm]
  · simp [bounded, h]
 theorem bounded_failure (bits : Nat) (x : Int) :
 bounded bits x = none ↔ x < -(2 ^ bits : Int) ∨ (2 ^ bits : Int) ≤ x := by
  simp only [bounded]
  split <;> simp_all <;> omega
def signed (bits : Nat) : Arithmetic :=
 ⟨fun x y => bounded bits (x+y), fun x y => bounded bits (x-y)⟩

 theorem failure_exact (ops : Arithmetic) (s : K → Value) (k : K) (e : Expr K) :
 (step ops s k e).2 = .failed k ↔ eval ops s e = none := by
  simp only [step]; split <;> simp_all
 theorem success_exact (ops : Arithmetic) (s : K → Value) (k : K) (e : Expr K) (v : Value) :
 (step ops s k e).2 = .wrote k v ↔ eval ops s e = some v := by
  simp only [step]; split <;> simp_all
 theorem failed_store (ops : Arithmetic) (s : K → Value) (k : K) (e : Expr K)
 (h : eval ops s e = none) : (step ops s k e).1 = s := by simp [step,h]
 theorem successful_store (ops : Arithmetic) (s : K → Value) (k : K) (e : Expr K) (v : Value)
 (h : eval ops s e = some v) : (step ops s k e).1 = put s k v := by simp [step,h]
 theorem singleton_retention (ops : Arithmetic) {L : K → Nat} {level} {s t : K → Value}
 (low : LowEq L level s t) (k : K) (e : Expr K) (flow : rank L e ≤ L k)
 (cap : Nat) (initial : List (Outcome K)) :
 Passive.feed (project L level) cap initial [(step ops s k e).2] =
 Passive.feed (project L level) cap initial [(step ops t k e).2] := by
  have h := (step_noninterference ops low k e flow).2
  simp only [Passive.feed,Passive.ingest,h]
 omit [DecidableEq K] in
 theorem eval_typed {G : K → Ty} {e t} (h : Typed G e t) (ops : Arithmetic)
 (s : K → Value) (hs : ∀ k, (s k).ty = G k) (v : Value)
 (he : eval ops s e = some v) : v.ty = t := by
  induction h with
  | lit => simp only [eval,Option.some.injEq] at he; subst v; rfl
  | read => simp only [eval,Option.some.injEq] at he; subst v; exact hs _
  | add ha hb iha ihb =>
    simp only [eval] at he
    split at he
    next x y ea eb =>
      cases ho : ops.add x y with
      | none => simp [ho] at he
      | some z =>
        simp only [ho,Option.map_some,Option.some.injEq] at he
        rw [←he]; rfl
    next => contradiction
  | sub ha hb iha ihb =>
    simp only [eval] at he
    split at he
    next x y ea eb =>
      cases ho : ops.sub x y with
      | none => simp [ho] at he
      | some z =>
        simp only [ho,Option.map_some,Option.some.injEq] at he
        rw [←he]; rfl
    next => contradiction
  | choose hc ha hb ihc iha ihb =>
    simp only [eval] at he
    split at he
    next => exact iha he
    next => exact ihb he
    next => contradiction
 theorem step_typed {G : K → Ty} (ops : Arithmetic) (s : K → Value) (k : K) (e : Expr K)
 (ht : Typed G e (G k)) (hs : ∀ x, (s x).ty = G x) :
 ∀ x, ((step ops s k e).1 x).ty = G x := by
  simp only [step]
  split
  next he => exact hs
  next v he => exact put_typed hs (eval_typed ht ops s hs v he)
#print axioms eval_typed
#print axioms step_typed
#print axioms singleton_retention

def InRange (bits : Nat) : Value → Prop
 | .int x => -(2 ^ bits : Int) ≤ x ∧ x < (2 ^ bits : Int)
 | .bool _ => True
def LiteralsInRange (bits : Nat) : Expr K → Prop
 | .lit v => InRange bits v
 | .read _ => True
 | .add a b => LiteralsInRange bits a ∧ LiteralsInRange bits b
 | .sub a b => LiteralsInRange bits a ∧ LiteralsInRange bits b
 | .choose c a b => LiteralsInRange bits c ∧ LiteralsInRange bits a ∧ LiteralsInRange bits b
 omit [DecidableEq K] in
 theorem eval_in_range (bits : Nat) (s : K → Value) (e : Expr K)
 (hs : ∀ k, InRange bits (s k)) (hl : LiteralsInRange bits e)
 (v : Value) (he : eval (signed bits) s e = some v) : InRange bits v := by
  induction e generalizing v with
  | lit w => simp only [eval,Option.some.injEq] at he; subst v; exact hl
  | read k => simp only [eval,Option.some.injEq] at he; subst v; exact hs k
  | add a b ha hb =>
    simp only [eval,signed] at he
    split at he
    next x y ea eb =>
      cases ho : bounded bits (x+y) with
      | none => simp [ho] at he
      | some z =>
        simp only [ho,Option.map_some,Option.some.injEq] at he
        rw [←he]; have hz := (bounded_exact bits (x+y) z).mp ho; simpa only [InRange, hz.1] using hz.2
    next => contradiction
  | sub a b ha hb =>
    simp only [eval,signed] at he
    split at he
    next x y ea eb =>
      cases ho : bounded bits (x-y) with
      | none => simp [ho] at he
      | some z =>
        simp only [ho,Option.map_some,Option.some.injEq] at he
        rw [←he]; have hz := (bounded_exact bits (x-y) z).mp ho; simpa only [InRange, hz.1] using hz.2
    next => contradiction
  | choose c a b hc ha hb =>
    simp only [eval] at he
    split at he
    next => exact ha hl.2.1 v he
    next => exact hb hl.2.2 v he
    next => contradiction
 theorem step_in_range (bits : Nat) (s : K → Value) (k : K) (e : Expr K)
 (hs : ∀ x, InRange bits (s x)) (hl : LiteralsInRange bits e) :
 ∀ x, InRange bits ((step (signed bits) s k e).1 x) := by
  simp only [step]
  split
  next => exact hs
  next v he =>
    have hv := eval_in_range bits s e hs hl v he
    intro x; by_cases hx : x = k <;> simp_all [put]
#print axioms eval_in_range
#print axioms step_in_range
namespace Controls
abbrev Key := Fin 2
def labels (k : Key) : Nat := k.val
def state (secret : Int) (k : Key) : Value := if k = 0 then .int 10 else .int secret
def increment : Expr Key := .add (.read 1) (.lit (.int 1))
example : (step (signed 63) (state 0) 1 increment).2 = .wrote 1 (.int 1) := by decide
example : (step (signed 63) (state 9223372036854775807) 1 increment).2 = .failed 1 := by decide
example : project labels 0 (step (signed 63) (state 0) 1 increment).2 =
 project labels 0 (step (signed 63) (state 9223372036854775807) 1 increment).2 := by decide
-- Smallest alternative leaks: always release whether the hidden operation failed.
def publicFailure : Outcome Key → Bool | .failed _ => true | .wrote _ _ => false
example : publicFailure (step (signed 63) (state 0) 1 increment).2 ≠
 publicFailure (step (signed 63) (state 9223372036854775807) 1 increment).2 := by decide
-- Nonempty low success and failure controls exclude filter-everything evidence.
example : project labels 0 (step (signed 63) (state 0) 0
 (.add (.read 0) (.lit (.int 1)))).2 = some (.wrote 0 (.int 11)) := by decide
example : project labels 0 (step (signed 63) (state 0) 0
 (.add (.lit (.int 9223372036854775807)) (.lit (.int 1)))).2 = some (.failed 0) := by decide
-- The common initial public view and actual flow premises hold for these pairs.
theorem pair_low : LowEq labels 0 (state 0) (state 9223372036854775807) := by
 intro k hk
 have h : k = 0 := Fin.ext (by simpa [labels] using hk)
 subst k; rfl
example : rank labels increment ≤ labels 1 := by decide
def lowIncrement : Expr Key := .add (.read 0) (.lit (.int 1))
def lowOverflow : Expr Key := .add (.lit (.int 9223372036854775807)) (.lit (.int 1))
example : rank labels lowIncrement ≤ labels 0 := by decide
example : rank labels lowOverflow ≤ labels 0 := by decide
example : Passive.feed (project labels 0) 1 []
 [(step (signed 63) (state 0) 0 lowIncrement).2] = [.wrote 0 (.int 11)] := by decide
example : Passive.feed (project labels 0) 1 []
 [(step (signed 63) (state 9223372036854775807) 0 lowIncrement).2] = [.wrote 0 (.int 11)] := by decide
example : Passive.feed (project labels 0) 1 []
 [(step (signed 63) (state 0) 0 lowOverflow).2] = [.failed 0] := by decide
example : Passive.feed (project labels 0) 1 []
 [(step (signed 63) (state 9223372036854775807) 0 lowOverflow).2] = [.failed 0] := by decide
-- Omitting the flow premise permits a visible secret-dependent outcome.
example : ¬ rank labels increment ≤ labels 0 := by decide
example : project labels 0 (step (signed 63) (state 0) 0 increment).2 ≠
 project labels 0 (step (signed 63) (state 9223372036854775807) 0 increment).2 := by decide
-- Countermodel ONLY: aborting a sequence is not the proved fixed-step semantics.
def abortTwo (s : Key → Value) : (Key → Value) × List (Outcome Key) :=
 let first := step (signed 63) s 1 increment
 match first.2 with
 | .failed _ => (first.1, [first.2])
 | .wrote _ _ =>
   let second := step (signed 63) first.1 0 (.lit (.int 11))
   (second.1, [first.2,second.2])
example : rank labels (.lit (.int 11) : Expr Key) ≤ labels 0 := by decide
example : (abortTwo (state 0)).1 0 = .int 11 := by decide
example : (abortTwo (state 9223372036854775807)).1 0 = .int 10 := by decide
example : (abortTwo (state 0)).2.filterMap (project labels 0) ≠
 (abortTwo (state 9223372036854775807)).2.filterMap (project labels 0) := by decide
-- Result checks alone do not imply admissible input or literal ranges.
example : (signed 63).add 9223372036854775808 (-1) = some 9223372036854775807 := by decide
example : eval (signed 63) (state 0) (.lit (.int 9223372036854775808)) =
 some (.int 9223372036854775808) := by decide
example : ¬ LiteralsInRange 63 (.lit (.int 9223372036854775808) : Expr Key) := by
 simp [LiteralsInRange,InRange]
example : eval (signed 63) (state 0)
 (.sub (.add (.lit (.int 9223372036854775807)) (.lit (.int 1))) (.lit (.int 1))) = none := by decide
end Controls
#print axioms bounded_exact
#print axioms bounded_failure
#print axioms eval_low
#print axioms step_noninterference
end MirroreaProofFirst.FallibleFlow
