import MirroreaProofFirstPassive
import Std
/-!
Task-local LAB W1 observation producer dependency; no Canon/production promotion.
Operational theorems require Safe/check acceptance and typed initial stores.
Safe is independent of check; its branch guard uses rank, separately characterized
by the declarative Flows judgment. Keys need decidable equality, not finiteness.
-/
namespace MirroreaProofFirst.ProducerFlow
inductive Ty where | int | bool deriving DecidableEq, Repr
inductive Value where | int (i : Int) | bool (b : Bool) deriving DecidableEq, Repr
def Value.ty : Value → Ty | .int _ => .int | .bool _ => .bool
inductive Expr (K : Type) where
 | lit (v : Value) | read (k : K) | add (a b : Expr K) | sub (a b : Expr K) | choose (c a b : Expr K)
 deriving Repr
variable {K : Type}
def infer (G : K → Ty) : Expr K → Option Ty
 | .lit v => some v.ty
 | .read k => some (G k)
 | .add a b => if infer G a = some .int ∧ infer G b = some .int then some .int else none
 | .sub a b => if infer G a = some .int ∧ infer G b = some .int then some .int else none
 | .choose c a b => if infer G c = some .bool ∧ infer G a = infer G b then infer G a else none
inductive Typed (G : K → Ty) : Expr K → Ty → Prop
 | lit : Typed G (.lit v) v.ty
 | read : Typed G (.read k) (G k)
 | add : Typed G a .int → Typed G b .int → Typed G (.add a b) .int
 | sub : Typed G a .int → Typed G b .int → Typed G (.sub a b) .int
 | choose : Typed G c .bool → Typed G a t → Typed G b t → Typed G (.choose c a b) t
 theorem typed_infer {G : K → Ty} {e t} (h : Typed G e t) : infer G e = some t := by
  induction h with
  | lit => rfl
  | read => rfl
  | add _ _ ha hb => simp [infer, ha, hb]
  | sub _ _ ha hb => simp [infer, ha, hb]
  | choose _ _ _ hc ha hb => simp [infer, hc, ha, hb]
 theorem infer_typed (G : K → Ty) (e : Expr K) (t : Ty) (h : infer G e = some t) : Typed G e t := by
  induction e generalizing t with
  | lit v => simp only [infer, Option.some.injEq] at h; subst t; exact .lit
  | read k => simp only [infer, Option.some.injEq] at h; subst t; exact .read
  | add a b ha hb =>
    simp only [infer] at h
    split at h
    next hcond =>
      cases h
      exact .add (ha _ hcond.1) (hb _ hcond.2)
    next => contradiction
  | sub a b ha hb =>
    simp only [infer] at h
    split at h
    next hcond =>
      cases h
      exact .sub (ha _ hcond.1) (hb _ hcond.2)
    next => contradiction
  | choose c a b hc ha hb =>
    simp only [infer] at h
    split at h
    next hcond => exact .choose (hc _ hcond.1) (ha _ h) (hb _ (hcond.2.symm.trans h))
    next => contradiction
 theorem infer_exact (G : K → Ty) (e : Expr K) (t : Ty) : infer G e = some t ↔ Typed G e t :=
 ⟨infer_typed G e t, typed_infer⟩
def eval (env : K → Value) : Expr K → Option Value
 | .lit v => some v
 | .read k => some (env k)
 | .add a b => match eval env a, eval env b with
   | some (.int x), some (.int y) => some (.int (x+y))
   | _, _ => none
 | .sub a b => match eval env a, eval env b with
   | some (.int x), some (.int y) => some (.int (x-y))
   | _, _ => none
 | .choose c a b => match eval env c with
   | some (.bool true) => eval env a
   | some (.bool false) => eval env b
   | _ => none
 theorem typed_evaluates {G : K → Ty} {e t} (h : Typed G e t) (env : K → Value)
 (henv : ∀ k, (env k).ty = G k) : ∃ v, eval env e = some v ∧ v.ty = t := by
  induction h with
  | lit => exact ⟨_, rfl, rfl⟩
  | read => exact ⟨_, rfl, henv _⟩
  | add _ _ ha hb =>
    obtain ⟨va, ea, ta⟩ := ha
    obtain ⟨vb, eb, tb⟩ := hb
    cases va <;> cases vb <;> simp_all [Value.ty, eval]
  | sub _ _ ha hb =>
    obtain ⟨va, ea, ta⟩ := ha
    obtain ⟨vb, eb, tb⟩ := hb
    cases va <;> cases vb <;> simp_all [Value.ty, eval]
  | choose _ _ _ hc ha hb =>
    obtain ⟨vc, ec, tc⟩ := hc
    cases vc with
    | int n => contradiction
    | bool b => cases b <;> simp_all [eval]
-- Rank is a reversible finite-profile carrier, not an authority or final policy.
def rank (L : K → Nat) : Expr K → Nat
 | .lit _ => 0
 | .read k => L k
 | .add a b => max (rank L a) (rank L b)
 | .sub a b => max (rank L a) (rank L b)
 | .choose c a b => max (rank L c) (max (rank L a) (rank L b))
inductive Flows (L : K → Nat) (level : Nat) : Expr K → Prop
 | lit : Flows L level (.lit v)
 | read : L k ≤ level → Flows L level (.read k)
 | add : Flows L level a → Flows L level b → Flows L level (.add a b)
 | sub : Flows L level a → Flows L level b → Flows L level (.sub a b)
 | choose : Flows L level c → Flows L level a → Flows L level b → Flows L level (.choose c a b)
 theorem flows_rank {L : K → Nat} {level e} (h : Flows L level e) : rank L e ≤ level := by
  induction h <;> simp_all [rank, Nat.max_le]
 theorem rank_flows (L : K → Nat) (level : Nat) (e : Expr K) (h : rank L e ≤ level) : Flows L level e := by
  induction e with
  | lit => exact .lit
  | read k => exact .read h
  | add a b ha hb =>
    have hh := (Nat.max_le.mp h)
    exact .add (ha hh.1) (hb hh.2)
  | sub a b ha hb =>
    have hh := (Nat.max_le.mp h)
    exact .sub (ha hh.1) (hb hh.2)
  | choose c a b hc ha hb =>
    have hh := (Nat.max_le.mp h)
    have hab := Nat.max_le.mp hh.2
    exact .choose (hc hh.1) (ha hab.1) (hb hab.2)
 theorem flows_exact (L : K → Nat) (level : Nat) (e : Expr K) : rank L e ≤ level ↔ Flows L level e :=
 ⟨rank_flows L level e, flows_rank⟩
def LowEq (L : K → Nat) (level : Nat) (s t : K → Value) : Prop :=
 ∀ k, L k ≤ level → s k = t k
 theorem eval_low {L : K → Nat} {level : Nat} {s t : K → Value}
 (low : LowEq L level s t) (e : Expr K) (h : rank L e ≤ level) : eval s e = eval t e := by
  induction e with
  | lit => rfl
  | read k => exact congrArg some (low k h)
  | add a b ha hb =>
    have hh := Nat.max_le.mp h
    simp only [eval, ha hh.1, hb hh.2]
  | sub a b ha hb =>
    have hh := Nat.max_le.mp h
    simp only [eval, ha hh.1, hb hh.2]
  | choose c a b hc ha hb =>
    have hh := Nat.max_le.mp h
    have hab := Nat.max_le.mp hh.2
    simp only [eval, hc hh.1, ha hab.1, hb hab.2]
#print axioms infer_exact
#print axioms typed_evaluates
#print axioms flows_exact
#print axioms eval_low

inductive Stmt (K : Type) where
 | skip | write (key : K) (e : Expr K) | seq (a b : Stmt K)
 | branch (c : Expr K) (a b : Stmt K)
inductive Safe (G : K → Ty) (L : K → Nat) : Nat → Stmt K → Prop
 | skip : Safe G L pc .skip
 | write : Typed G e (G k) → Flows L (L k) e → pc ≤ L k → Safe G L pc (.write k e)
 | seq : Safe G L pc a → Safe G L pc b → Safe G L pc (.seq a b)
 | branch : Typed G c .bool → Safe G L (max pc (rank L c)) a →
     Safe G L (max pc (rank L c)) b → Safe G L pc (.branch c a b)
def check (G : K → Ty) (L : K → Nat) (pc : Nat) : Stmt K → Bool
 | .skip => true
 | .write k e => decide (infer G e = some (G k) ∧ rank L e ≤ L k ∧ pc ≤ L k)
 | .seq a b => check G L pc a && check G L pc b
 | .branch c a b => decide (infer G c = some .bool) &&
    check G L (max pc (rank L c)) a && check G L (max pc (rank L c)) b
 theorem safe_check {G : K → Ty} {L : K → Nat} {pc cmd} (h : Safe G L pc cmd) : check G L pc cmd = true := by
  induction h with
  | skip => rfl
  | write ht hf hp => simp [check, typed_infer ht, flows_rank hf, hp]
  | seq _ _ ha hb => simp [check, ha, hb]
  | branch ht _ _ ha hb => simp [check, typed_infer ht, ha, hb]
 theorem check_safe (G : K → Ty) (L : K → Nat) (pc : Nat) (cmd : Stmt K)
 (h : check G L pc cmd = true) : Safe G L pc cmd := by
  induction cmd generalizing pc with
  | skip => exact .skip
  | write k e =>
    simp only [check, decide_eq_true_eq] at h
    exact .write (infer_typed _ _ _ h.1) (rank_flows _ _ _ h.2.1) h.2.2
  | seq a b ha hb =>
    simp only [check, Bool.and_eq_true] at h
    exact .seq (ha _ h.1) (hb _ h.2)
  | branch c a b ha hb =>
    simp only [check, Bool.and_eq_true, decide_eq_true_eq] at h
    exact .branch (infer_typed _ _ _ h.1.1) (ha _ h.1.2) (hb _ h.2)
 theorem check_exact (G : K → Ty) (L : K → Nat) (pc : Nat) (cmd : Stmt K) :
 check G L pc cmd = true ↔ Safe G L pc cmd := ⟨check_safe G L pc cmd, safe_check⟩
variable [DecidableEq K]
def put (s : K → Value) (k : K) (v : Value) : K → Value := fun x => if x = k then v else s x
-- Run emits actual writes only; raw trace itself may contain secrets.
def run (s : K → Value) : Stmt K → (K → Value) × List (K × Value)
 | .skip => (s, [])
 | .write k e => match eval s e with
   | some v => (put s k v, [(k,v)])
   | none => (s, [])
 | .seq a b => let first := run s a; let second := run first.1 b
               (second.1, first.2 ++ second.2)
 | .branch c a b => match eval s c with
   | some (.bool true) => run s a
   | some (.bool false) => run s b
   | _ => (s, [])
def visible (L : K → Nat) (level : Nat) (rows : List (K × Value)) :=
 rows.filter (fun row => decide (L row.1 ≤ level))
 theorem put_typed {G : K → Ty} {s : K → Value} {k v}
 (hs : ∀ x, (s x).ty = G x) (hv : v.ty = G k) : ∀ x, (put s k v x).ty = G x := by
  intro x; by_cases hx : x = k <;> simp_all [put]
 theorem run_typed {G : K → Ty} {L : K → Nat} {pc cmd}
 (h : Safe G L pc cmd) (s : K → Value) (hs : ∀ k, (s k).ty = G k) :
 ∀ k, ((run s cmd).1 k).ty = G k := by
  induction h generalizing s with
  | skip => exact hs
  | write ht _ _ =>
    obtain ⟨v, he, hv⟩ := typed_evaluates ht s hs
    simpa [run, he] using put_typed hs hv
  | seq _ _ ha hb => exact hb _ (ha _ hs)
  | branch ht _ _ ha hb =>
    obtain ⟨v, he, hv⟩ := typed_evaluates ht s hs
    cases v with
    | int n => contradiction
    | bool b => cases b <;> simp_all [run]
 omit [DecidableEq K] in
 theorem low_trans {L : K → Nat} {level} {a b c : K → Value}
 (h : LowEq L level a b) (g : LowEq L level b c) : LowEq L level a c := fun k hk => (h k hk).trans (g k hk)
 omit [DecidableEq K] in
 theorem low_symm {L : K → Nat} {level} {a b : K → Value}
 (h : LowEq L level a b) : LowEq L level b a := fun k hk => (h k hk).symm
 theorem confinement {G : K → Ty} {L : K → Nat} {pc cmd level}
 (h : Safe G L pc cmd) (high : level < pc) (s : K → Value) :
 LowEq L level (run s cmd).1 s ∧ visible L level (run s cmd).2 = [] := by
  induction h generalizing s with
  | skip => exact ⟨fun _ _ => rfl, rfl⟩
  | write ht hf hp =>
    rename_i e k pc0
    have hk : ¬ L k ≤ level := by omega
    simp only [run]
    split
    next v he =>
      constructor
      · intro x hx
        have hn : x ≠ k := by intro he; subst x; exact hk hx
        simp [put, hn]
      · simp [visible, hk]
    next => exact ⟨fun _ _ => rfl, rfl⟩
  | seq _ _ ha hb =>
    obtain ⟨ea, va⟩ := ha high s
    obtain ⟨eb, vb⟩ := hb high (run s _).1
    constructor
    · exact low_trans eb ea
    · change visible L level ((run s _).2 ++ (run (run s _).1 _).2) = []
      simp only [visible, List.filter_append] at *
      rw [va, vb]; rfl
  | branch ht hsa hsb ha hb =>
    simp only [run]
    split
    next => exact ha (by omega) s
    next => exact hb (by omega) s
    next => exact ⟨fun _ _ => rfl, rfl⟩
#print axioms check_exact
#print axioms run_typed
#print axioms confinement

 theorem run_noninterference {G : K → Ty} {L : K → Nat} {pc cmd level}
 (h : Safe G L pc cmd) (s t : K → Value)
 (hs : ∀ k, (s k).ty = G k) (ht : ∀ k, (t k).ty = G k)
 (low : LowEq L level s t) :
 LowEq L level (run s cmd).1 (run t cmd).1 ∧
 visible L level (run s cmd).2 = visible L level (run t cmd).2 := by
  induction h generalizing s t with
  | skip => exact ⟨low, rfl⟩
  | write he hf hp =>
    rename_i e k pc0
    obtain ⟨vs, es, ts⟩ := typed_evaluates he s hs
    obtain ⟨vt, et, tt⟩ := typed_evaluates he t ht
    by_cases hk : L k ≤ level
    · have ev := eval_low low e (Nat.le_trans (flows_rank hf) hk)
      have eqv : vs = vt := Option.some.inj (es.symm.trans (ev.trans et))
      subst vt
      constructor
      · intro x hx; simp only [run, es, et, put]; split
        next => rfl
        next => exact low x hx
      · simp [run, es, et]
    · constructor
      · intro x hx
        have hn : x ≠ k := by intro hh; subst x; exact hk hx
        simpa [run, es, et, put, hn] using low x hx
      · simp [run, es, et, visible, hk]
  | seq haSafe hbSafe ha hb =>
    obtain ⟨la, va⟩ := ha s t hs ht low
    obtain ⟨lb, vb⟩ := hb (run s _).1 (run t _).1
      (run_typed haSafe s hs) (run_typed haSafe t ht) la
    constructor
    · exact lb
    · change visible L level ((run s _).2 ++ (run (run s _).1 _).2) =
        visible L level ((run t _).2 ++ (run (run t _).1 _).2)
      simp only [visible, List.filter_append] at *
      rw [va, vb]
  | branch hc haSafe hbSafe ha hb =>
    rename_i c pc0 a b
    by_cases hcl : rank L c ≤ level
    · have ec := eval_low low c hcl
      obtain ⟨v, es, tv⟩ := typed_evaluates hc s hs
      have et := ec.symm.trans es
      cases v with
      | int n => contradiction
      | bool q =>
        cases q
        · simpa [run, es, et] using hb s t hs ht low
        · simpa [run, es, et] using ha s t hs ht low
    · have conf : ∀ u, LowEq L level (run u (.branch c a b)).1 u ∧
          visible L level (run u (.branch c a b)).2 = [] := by
        intro u
        simp only [run]
        split
        next => exact confinement haSafe (by omega) u
        next => exact confinement hbSafe (by omega) u
        next => exact ⟨fun _ _ => rfl, rfl⟩
      exact ⟨low_trans (low_trans (conf s).1 low) (low_symm (conf t).1),
        (conf s).2.trans (conf t).2.symm⟩
#print axioms run_noninterference

def project (L : K → Nat) (level : Nat) (row : K × Value) : Option (K × Value) :=
 if L row.1 ≤ level then some row else none
 omit [DecidableEq K] in
 theorem projected_visible (L : K → Nat) (level : Nat) (rows : List (K × Value)) :
 rows.filterMap (project L level) = visible L level rows := by
  induction rows with
  | nil => rfl
  | cons r rs ih =>
    by_cases h : L r.1 ≤ level <;> simp [project, visible, h, ih, visible] at *
 theorem checked_retention_noninterference {G : K → Ty} {L : K → Nat} {pc cmd level}
 (h : check G L pc cmd = true) (s t : K → Value)
 (hs : ∀ k, (s k).ty = G k) (ht : ∀ k, (t k).ty = G k)
 (low : LowEq L level s t) (cap : Nat) (initial : List (K × Value)) :
 Passive.feed (project L level) cap initial (run s cmd).2 =
 Passive.feed (project L level) cap initial (run t cmd).2 := by
  have eqrows := (run_noninterference (check_safe _ _ _ _ h) s t hs ht low).2
  rw [Passive.filter_before_retention, Passive.filter_before_retention,
    projected_visible, projected_visible, eqrows]
namespace Controls
abbrev Key := Fin 3
def types (k : Key) : Ty := if k = 2 then .bool else .int
def labels (k : Key) : Nat := if k = 0 then 0 else 1
def state (secret : Int) (flag : Bool) (k : Key) : Value :=
 if k = 0 then .int 10 else if k = 1 then .int secret else .bool flag
def good : Stmt Key := .seq (.write 0 (.add (.read 0) (.lit (.int 1))))
 (.seq (.branch (.read 2) (.write 1 (.read 0))
   (.seq (.write 1 (.lit (.int 99))) (.write 1 (.read 1))))
 (.write 0 (.sub (.read 0) (.lit (.int 1)))))
example : check types labels 0 good = true := by decide
example : visible labels 0 (run (state 0 true) good).2 = [(0,.int 11),(0,.int 10)] := by decide
example : visible labels 0 (run (state 42 false) good).2 = [(0,.int 11),(0,.int 10)] := by decide
example : Passive.feed (project labels 0) 2 [] (run (state 42 false) good).2 =
 [(0,.int 10),(0,.int 11)] := by decide
example : (run (state 0 true) good).2 ≠ (run (state 42 false) good).2 := by decide
def directLeak : Stmt Key := .write 0 (.add (.read 1) (.lit (.int 1)))
example : check types labels 0 directLeak = false := by decide
example : infer types (.add (.read 1) (.lit (.int 1))) = some .int := by decide
example : visible labels 0 (run (state 0 true) directLeak).2 ≠
 visible labels 0 (run (state 1 true) directLeak).2 := by decide
def controlLeak : Stmt Key := .branch (.read 2) (.write 0 (.lit (.int 0))) (.write 0 (.lit (.int 1)))
example : check types labels 0 controlLeak = false := by decide
example : visible labels 0 (run (state 0 true) controlLeak).2 ≠
 visible labels 0 (run (state 0 false) controlLeak).2 := by decide
def expressionLeak : Stmt Key := .write 0 (.choose (.read 2) (.lit (.int 0)) (.lit (.int 1)))
example : check types labels 0 expressionLeak = false := by decide
example : check types labels 0 (.write 0 (.add (.read 0) (.lit (.bool true)))) = false := by decide
-- Completeness is relative to the declared syntax-directed rules, not all semantic NI.
def cancellation : Stmt Key := .write 0 (.sub (.read 1) (.read 1))
example : check types labels 0 cancellation = false := by decide
example : visible labels 0 (run (state 0 true) cancellation).2 =
 visible labels 0 (run (state 17 false) cancellation).2 := by decide

-- Oracle review controls: equal final low value does not hide write occurrence.
def occurrenceLeak : Stmt Key := .branch (.read 2) (.write 0 (.lit (.int 10))) .skip
example : check types labels 0 occurrenceLeak = false := by decide
example : (run (state 0 true) occurrenceLeak).1 0 =
 (run (state 0 false) occurrenceLeak).1 0 := by decide
example : visible labels 0 (run (state 0 true) occurrenceLeak).2 ≠
 visible labels 0 (run (state 0 false) occurrenceLeak).2 := by decide
example : Passive.feed (project labels 0) 1 [] (run (state 0 true) occurrenceLeak).2 ≠
 Passive.feed (project labels 0) 1 [] (run (state 0 false) occurrenceLeak).2 := by decide
-- Completed equality supplies no arbitrary raw-write-count cut equality.
example : visible labels 0 ((run (state 0 true) good).2.take 3) ≠
 visible labels 0 ((run (state 42 false) good).2.take 3) := by decide
-- Equality alone does not validate initial rows or establish a capacity bound.
example : Passive.feed (project labels 0) 0 [(1, .int 99)]
 (run (state 0 true) .skip).2 = [(1, .int 99)] := by decide
end Controls
#print axioms projected_visible
#print axioms checked_retention_noninterference
end MirroreaProofFirst.ProducerFlow
