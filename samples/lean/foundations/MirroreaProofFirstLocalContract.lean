import MirroreaProofFirstResourceBoundary
namespace MirroreaProofFirst.LocalContract
open ResourceBoundary
inductive Term where
 | input (index : Nat)
 | integer (value : Int)
 | add (left right : Term)
 | mul (left right : Term)
 | square (term : Term)
 deriving DecidableEq, Repr

def eval (input : Nat → Int) : Term → Int
 | .input i => input i
 | .integer z => z
 | .add a b => eval input a + eval input b
 | .mul a b => eval input a * eval input b
 | .square a => eval input a * eval input a

-- Declarative meaning is independent of the evaluator and certificate checker.
inductive Denotes (input : Nat → Int) : Term → Int → Prop where
 | input (i) : Denotes input (.input i) (input i)
 | integer (z) : Denotes input (.integer z) z
 | add {a b x y} : Denotes input a x → Denotes input b y → Denotes input (.add a b) (x+y)
 | mul {a b x y} : Denotes input a x → Denotes input b y → Denotes input (.mul a b) (x*y)
 | square {a x} : Denotes input a x → Denotes input (.square a) (x*x)
theorem eval_denotes (input : Nat → Int) (e : Term) : Denotes input e (eval input e) := by
 induction e with
 | input i => exact .input i
 | integer z => exact .integer z
 | add a b ha hb => exact .add ha hb
 | mul a b ha hb => exact .mul ha hb
 | square a ha => exact .square ha
theorem denotes_eval {input : Nat → Int} {e : Term} {v : Int} (h : Denotes input e v) : eval input e = v := by
 induction h <;> simp_all [eval]
theorem eval_exact (input : Nat → Int) (e : Term) (v : Int) : eval input e = v ↔ Denotes input e v := by
 constructor
 · intro h; rw [← h]; exact eval_denotes input e
 · exact denotes_eval

inductive Nonnegative (assumptions : List Term) : Term → Prop where
 | hypothesis {e} : e ∈ assumptions → Nonnegative assumptions e
 | integer (n : Nat) : Nonnegative assumptions (.integer (Int.ofNat n))
 | add {a b} : Nonnegative assumptions a → Nonnegative assumptions b → Nonnegative assumptions (.add a b)
 | mul {a b} : Nonnegative assumptions a → Nonnegative assumptions b → Nonnegative assumptions (.mul a b)
 | square (a : Term) : Nonnegative assumptions (.square a)

def AssumptionsHold (input : Nat → Int) (assumptions : List Term) : Prop :=
 ∀ e, e ∈ assumptions → 0 ≤ eval input e

theorem nonnegative_sound {input : Nat → Int} {assumptions : List Term} {e : Term}
 (h : Nonnegative assumptions e) (valid : AssumptionsHold input assumptions) : 0 ≤ eval input e := by
 induction h with
 | hypothesis he => exact valid _ he
 | integer n => exact Int.natCast_nonneg n
 | add ha hb ia ib => exact Int.add_nonneg ia ib
 | mul ha hb ia ib => exact Int.mul_nonneg ia ib
 | square a =>
   by_cases hn : 0 ≤ eval input a
   · exact Int.mul_nonneg hn hn
   · have hp : eval input a ≤ 0 := by omega
     exact Int.mul_nonneg_of_nonpos_of_nonpos hp hp

inductive Certificate where
 | hypothesis (e : Term)
 | integer (n : Nat)
 | add (left right : Certificate)
 | mul (left right : Certificate)
 | square (term : Term)
 deriving DecidableEq, Repr

def infer (assumptions : List Term) : Certificate → Option Term
 | .hypothesis e => if e ∈ assumptions then some e else none
 | .integer n => some (.integer (Int.ofNat n))
 | .add a b => do return .add (← infer assumptions a) (← infer assumptions b)
 | .mul a b => do return .mul (← infer assumptions a) (← infer assumptions b)
 | .square e => some (.square e)

theorem infer_sound {as : List Term} {c : Certificate} {e : Term}
 (h : infer as c = some e) : Nonnegative as e := by
 induction c generalizing e with
 | hypothesis t =>
   simp only [infer] at h; split at h
   · rename_i member; cases h; exact .hypothesis member
   · contradiction
 | integer n => cases h; exact .integer n
 | square t => cases h; exact .square t
 | add a b ha hb =>
   cases ea : infer as a <;> cases eb : infer as b <;> simp [infer,ea,eb] at h
   cases h; exact .add (ha ea) (hb eb)
 | mul a b ha hb =>
   cases ea : infer as a <;> cases eb : infer as b <;> simp [infer,ea,eb] at h
   cases h; exact .mul (ha ea) (hb eb)

theorem infer_complete {as : List Term} {e : Term} (h : Nonnegative as e) :
 ∃ c, infer as c = some e := by
 induction h with
 | @hypothesis e he => exact ⟨.hypothesis e,by simp [infer,he]⟩
 | integer n => exact ⟨.integer n,rfl⟩
 | square a => exact ⟨.square a,rfl⟩
 | add ha hb ia ib =>
   obtain ⟨a,ea⟩ := ia; obtain ⟨b,eb⟩ := ib
   exact ⟨.add a b,by simp [infer,ea,eb]⟩
 | mul ha hb ia ib =>
   obtain ⟨a,ea⟩ := ia; obtain ⟨b,eb⟩ := ib
   exact ⟨.mul a b,by simp [infer,ea,eb]⟩

def check (as : List Term) (e : Term) (c : Certificate) : Bool := decide (infer as c = some e)
theorem check_sound {as : List Term} {e : Term} {c : Certificate} (h : check as e c = true) : Nonnegative as e :=
 infer_sound (by simpa [check] using h)
theorem check_complete {as : List Term} {e : Term} (h : Nonnegative as e) :
 ∃ c, check as e c = true := by
 obtain ⟨c,hc⟩ := infer_complete h
 exact ⟨c,by simp [check,hc]⟩

-- This profile checks the actual arithmetic hypotheses at this invocation;
-- their validity is not silently inferred from a successful proof search.
def assumptionsCheck (input : Nat → Int) (as : List Term) : Bool :=
 as.all (fun e => decide (0 ≤ eval input e))
theorem assumptions_exact (input : Nat → Int) (as : List Term) :
 assumptionsCheck input as = true ↔ AssumptionsHold input as := by
 simp [assumptionsCheck,AssumptionsHold,List.all_eq_true]

def positiveTerm (e : Term) : Term := .add e (.integer 1)
theorem positive_result {input : Nat → Int} {as : List Term} {e : Term} {c : Certificate}
 (checked : check as e c = true) (valid : assumptionsCheck input as = true) :
 0 < eval input (positiveTerm e) := by
 have h := nonnegative_sound (check_sound checked) ((assumptions_exact _ _).mp valid)
 simp only [positiveTerm,eval]; omega

namespace Controls
def squareLength : Term := positiveTerm (.square (.input 0))
example : check [] (.square (.input 0)) (.square (.input 0)) = true := by decide
example : eval (fun _ => -3) squareLength = 10 := by decide
example : check [] (.input 0) (.hypothesis (.input 0)) = false := by decide
example : check [.input 0] (.input 0) (.hypothesis (.input 0)) = true := by decide
example : assumptionsCheck (fun _ => -3) [.input 0] = false := by decide
-- A checked certificate cannot be reused for a different expression.
example : check [] (.integer (-1)) (.square (.input 0)) = false := by decide
end Controls
#print axioms eval_exact
#print axioms nonnegative_sound
#print axioms infer_sound
#print axioms infer_complete
#print axioms assumptions_exact
#print axioms positive_result
end MirroreaProofFirst.LocalContract

-- Nonproduction bridge for pure arithmetic; not source/authentication provenance.
namespace MirroreaProofFirst.LocalContract.ProductNormalization

def normalize : Term → Term
 | .input i => .input i
 | .integer z => .integer z
 | .add a b => .add (normalize a) (normalize b)
 | .square a => .square (normalize a)
 | .mul a b =>
   let x := normalize a
   let y := normalize b
   if x = y then .square x else .mul x y

inductive Normalizes : Term → Term → Prop where
 | input (i) : Normalizes (.input i) (.input i)
 | integer (z) : Normalizes (.integer z) (.integer z)
 | add {a b x y} : Normalizes a x → Normalizes b y → Normalizes (.add a b) (.add x y)
 | square {a x} : Normalizes a x → Normalizes (.square a) (.square x)
 | same {a b x} : Normalizes a x → Normalizes b x → Normalizes (.mul a b) (.square x)
 | different {a b x y} : Normalizes a x → Normalizes b y → x ≠ y →
     Normalizes (.mul a b) (.mul x y)

theorem normalizes_sound {a b : Term} (h : Normalizes a b) : normalize a = b := by
 induction h <;> simp_all [normalize]

theorem normalizes_complete (a : Term) : Normalizes a (normalize a) := by
 induction a with
 | input i => exact .input i
 | integer z => exact .integer z
 | add a b ha hb => exact .add ha hb
 | square a ha => exact .square ha
 | mul a b ha hb =>
   by_cases eq : normalize a = normalize b
   · simpa [normalize,eq] using Normalizes.same (eq ▸ ha) hb
   · simpa [normalize,eq] using Normalizes.different ha hb eq

theorem normalization_exact (a b : Term) : normalize a = b ↔ Normalizes a b := by
 constructor
 · intro h; rw [← h]; exact normalizes_complete a
 · exact normalizes_sound

theorem evaluation_preserved (input : Nat → Int) (a : Term) :
 eval input (normalize a) = eval input a := by
 induction a with
 | input i => rfl
 | integer z => rfl
 | add a b ha hb => simp [normalize,eval,ha,hb]
 | square a ha => simp [normalize,eval,ha]
 | mul a b ha hb =>
   by_cases eq : normalize a = normalize b
   · have same : eval input a = eval input b := by rw [← ha,← hb,eq]
     simp [normalize,eq,eval,hb,same]
   · simp [normalize,eq,eval,ha,hb]

theorem self_product (a : Term) : normalize (.mul a a) = .square (normalize a) := by
 simp [normalize]

theorem self_product_certificate (a : Term) :
 check [] (normalize (.mul a a)) (.square (normalize a)) = true := by
 simp [self_product,check,infer]

namespace Controls
example : normalize (.mul (.integer 3) (.integer 4)) = .mul (.integer 3) (.integer 4) := by decide
example : eval (fun _ => -3) (normalize (positiveTerm (.mul (.input 0) (.input 0)))) = 10 := by decide
example : check [] (normalize (.mul (.input 0) (.input 0))) (.square (.input 0)) = true := by decide
example : check [] (normalize (.mul (.input 0) (.input 1))) (.square (.input 0)) = false := by decide
end Controls
#print axioms normalization_exact
#print axioms evaluation_preserved
#print axioms self_product
#print axioms self_product_certificate
end MirroreaProofFirst.LocalContract.ProductNormalization
