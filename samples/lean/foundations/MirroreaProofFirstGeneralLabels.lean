import MirroreaProofFirstProducerFlow
/-! Task-local LAB: explicit label algebra, no authority or production promotion. -/
namespace MirroreaProofFirst.GeneralLabels
open ProducerFlow
structure LabelTheory (A : Type) where
 le : A → A → Bool
 bottom : A
 join : A → A → A
 refl : ∀ a, le a a = true
 trans : ∀ {a b c}, le a b = true → le b c = true → le a c = true
 bottom_le : ∀ a, le bottom a = true
 join_le : ∀ a b c, le (join a b) c = true ↔ le a c = true ∧ le b c = true
variable {A K : Type}
namespace LabelTheory
 theorem left_le_join (T : LabelTheory A) (a b : A) : T.le a (T.join a b) = true :=
  ((T.join_le a b _).mp (T.refl _)).1
 theorem right_le_join (T : LabelTheory A) (a b : A) : T.le b (T.join a b) = true :=
  ((T.join_le a b _).mp (T.refl _)).2
end LabelTheory
def rank (T : LabelTheory A) (L : K → A) : Expr K → A
 | .lit _ => T.bottom
 | .read k => L k
 | .add a b => T.join (rank T L a) (rank T L b)
 | .sub a b => T.join (rank T L a) (rank T L b)
 | .choose c a b => T.join (rank T L c) (T.join (rank T L a) (rank T L b))
inductive Flows (T : LabelTheory A) (L : K → A) (level : A) : Expr K → Prop
 | lit : Flows T L level (.lit v)
 | read : T.le (L k) level = true → Flows T L level (.read k)
 | add : Flows T L level a → Flows T L level b → Flows T L level (.add a b)
 | sub : Flows T L level a → Flows T L level b → Flows T L level (.sub a b)
 | choose : Flows T L level c → Flows T L level a → Flows T L level b → Flows T L level (.choose c a b)
 theorem flows_rank {T : LabelTheory A} {L : K → A} {level e}
 (h : Flows T L level e) : T.le (rank T L e) level = true := by
  induction h with
  | lit => exact T.bottom_le _
  | read h => exact h
  | add _ _ ha hb => exact (T.join_le _ _ _).mpr ⟨ha,hb⟩
  | sub _ _ ha hb => exact (T.join_le _ _ _).mpr ⟨ha,hb⟩
  | choose _ _ _ hc ha hb => exact (T.join_le _ _ _).mpr ⟨hc,(T.join_le _ _ _).mpr ⟨ha,hb⟩⟩
 theorem rank_flows (T : LabelTheory A) (L : K → A) (level : A) (e : Expr K)
 (h : T.le (rank T L e) level = true) : Flows T L level e := by
  induction e with
  | lit => exact .lit
  | read k => exact .read h
  | add a b ha hb =>
    have hh := (T.join_le _ _ _).mp h
    exact .add (ha hh.1) (hb hh.2)
  | sub a b ha hb =>
    have hh := (T.join_le _ _ _).mp h
    exact .sub (ha hh.1) (hb hh.2)
  | choose c a b hc ha hb =>
    have hh := (T.join_le _ _ _).mp h
    have hab := (T.join_le _ _ _).mp hh.2
    exact .choose (hc hh.1) (ha hab.1) (hb hab.2)
 theorem flows_exact (T : LabelTheory A) (L : K → A) (level : A) (e : Expr K) :
 T.le (rank T L e) level = true ↔ Flows T L level e :=
 ⟨rank_flows T L level e,flows_rank⟩
def LowEq (T : LabelTheory A) (L : K → A) (level : A) (s t : K → Value) : Prop :=
 ∀ k, T.le (L k) level = true → s k = t k
 theorem eval_low {T : LabelTheory A} {L : K → A} {level} {s t : K → Value}
 (low : LowEq T L level s t) (e : Expr K) (h : T.le (rank T L e) level = true) :
 ProducerFlow.eval s e = ProducerFlow.eval t e := by
  induction e with
  | lit => rfl
  | read k => exact congrArg some (low k h)
  | add a b ha hb =>
    have hh := (T.join_le _ _ _).mp h
    simp only [ProducerFlow.eval,ha hh.1,hb hh.2]
  | sub a b ha hb =>
    have hh := (T.join_le _ _ _).mp h
    simp only [ProducerFlow.eval,ha hh.1,hb hh.2]
  | choose c a b hc ha hb =>
    have hh := (T.join_le _ _ _).mp h
    have hab := (T.join_le _ _ _).mp hh.2
    simp only [ProducerFlow.eval,hc hh.1,ha hab.1,hb hab.2]
inductive Safe (T : LabelTheory A) (G : K → Ty) (L : K → A) : A → Stmt K → Prop
 | skip : Safe T G L pc .skip
 | write : Typed G e (G k) → Flows T L (L k) e → T.le pc (L k) = true → Safe T G L pc (.write k e)
 | seq : Safe T G L pc a → Safe T G L pc b → Safe T G L pc (.seq a b)
 | branch : Typed G c .bool → Safe T G L (T.join pc (rank T L c)) a →
     Safe T G L (T.join pc (rank T L c)) b → Safe T G L pc (.branch c a b)
def check (T : LabelTheory A) (G : K → Ty) (L : K → A) (pc : A) : Stmt K → Bool
 | .skip => true
 | .write k e => decide (infer G e = some (G k)) && T.le (rank T L e) (L k) && T.le pc (L k)
 | .seq a b => check T G L pc a && check T G L pc b
 | .branch c a b => decide (infer G c = some .bool) &&
    check T G L (T.join pc (rank T L c)) a && check T G L (T.join pc (rank T L c)) b
 theorem safe_check {T : LabelTheory A} {G : K → Ty} {L : K → A} {pc cmd}
 (h : Safe T G L pc cmd) : check T G L pc cmd = true := by
  induction h with
  | skip => rfl
  | write ht hf hp => simp [check,typed_infer ht,flows_rank hf,hp]
  | seq _ _ ha hb => simp [check,ha,hb]
  | branch ht _ _ ha hb => simp [check,typed_infer ht,ha,hb]
 theorem check_safe (T : LabelTheory A) (G : K → Ty) (L : K → A) (pc : A) (cmd : Stmt K)
 (h : check T G L pc cmd = true) : Safe T G L pc cmd := by
  induction cmd generalizing pc with
  | skip => exact .skip
  | write k e =>
    simp only [check,Bool.and_eq_true,decide_eq_true_eq] at h
    exact .write (infer_typed _ _ _ h.1.1) (rank_flows _ _ _ _ h.1.2) h.2
  | seq a b ha hb =>
    simp only [check,Bool.and_eq_true] at h
    exact .seq (ha _ h.1) (hb _ h.2)
  | branch c a b ha hb =>
    simp only [check,Bool.and_eq_true,decide_eq_true_eq] at h
    exact .branch (infer_typed _ _ _ h.1.1) (ha _ h.1.2) (hb _ h.2)
 theorem check_exact (T : LabelTheory A) (G : K → Ty) (L : K → A) (pc : A) (cmd : Stmt K) :
 check T G L pc cmd = true ↔ Safe T G L pc cmd := ⟨check_safe T G L pc cmd,safe_check⟩
#print axioms flows_exact
#print axioms eval_low
#print axioms check_exact

variable [DecidableEq K]
def visible (T : LabelTheory A) (L : K → A) (level : A) (rows : List (K × Value)) :=
 rows.filter (fun row => T.le (L row.1) level)
 theorem run_typed {T : LabelTheory A} {G : K → Ty} {L : K → A} {pc cmd}
 (h : Safe T G L pc cmd) (s : K → Value) (hs : ∀ k, (s k).ty = G k) :
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
 theorem low_trans {T : LabelTheory A} {L : K → A} {level} {a b c : K → Value}
 (h : LowEq T L level a b) (g : LowEq T L level b c) : LowEq T L level a c := fun k hk => (h k hk).trans (g k hk)
 omit [DecidableEq K] in
 theorem low_symm {T : LabelTheory A} {L : K → A} {level} {a b : K → Value}
 (h : LowEq T L level a b) : LowEq T L level b a := fun k hk => (h k hk).symm
 theorem confinement {T : LabelTheory A} {G : K → Ty} {L : K → A} {pc cmd level}
 (h : Safe T G L pc cmd) (high : T.le pc level ≠ true) (s : K → Value) :
 LowEq T L level (run s cmd).1 s ∧ visible T L level (run s cmd).2 = [] := by
  induction h generalizing s with
  | skip => exact ⟨fun _ _ => rfl, rfl⟩
  | write ht hf hp =>
    rename_i e k pc0
    have hk : ¬ T.le (L k) level = true := by
      intro hle; exact high (T.trans hp hle)
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
    · change visible T L level ((run s _).2 ++ (run (run s _).1 _).2) = []
      simp only [visible, List.filter_append] at *
      rw [va, vb]; rfl
  | branch ht hsa hsb ha hb =>
    simp only [run]
    split
    next => exact ha (fun hle => high ((T.join_le _ _ _).mp hle).1) s
    next => exact hb (fun hle => high ((T.join_le _ _ _).mp hle).1) s
    next => exact ⟨fun _ _ => rfl, rfl⟩
#print axioms check_exact
#print axioms run_typed
#print axioms confinement

 theorem run_noninterference {T : LabelTheory A} {G : K → Ty} {L : K → A} {pc cmd level}
 (h : Safe T G L pc cmd) (s t : K → Value)
 (hs : ∀ k, (s k).ty = G k) (ht : ∀ k, (t k).ty = G k)
 (low : LowEq T L level s t) :
 LowEq T L level (run s cmd).1 (run t cmd).1 ∧
 visible T L level (run s cmd).2 = visible T L level (run t cmd).2 := by
  induction h generalizing s t with
  | skip => exact ⟨low, rfl⟩
  | write he hf hp =>
    rename_i e k pc0
    obtain ⟨vs, es, ts⟩ := typed_evaluates he s hs
    obtain ⟨vt, et, tt⟩ := typed_evaluates he t ht
    by_cases hk : T.le (L k) level = true
    · have ev := eval_low low e (T.trans (flows_rank hf) hk)
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
    · change visible T L level ((run s _).2 ++ (run (run s _).1 _).2) =
        visible T L level ((run t _).2 ++ (run (run t _).1 _).2)
      simp only [visible, List.filter_append] at *
      rw [va, vb]
  | branch hc haSafe hbSafe ha hb =>
    rename_i c pc0 a b
    by_cases hcl : T.le (rank T L c) level = true
    · have ec := eval_low low c hcl
      obtain ⟨v, es, tv⟩ := typed_evaluates hc s hs
      have et := ec.symm.trans es
      cases v with
      | int n => contradiction
      | bool q =>
        cases q
        · simpa [run, es, et] using hb s t hs ht low
        · simpa [run, es, et] using ha s t hs ht low
    · have conf : ∀ u, LowEq T L level (run u (.branch c a b)).1 u ∧
          visible T L level (run u (.branch c a b)).2 = [] := by
        intro u
        simp only [run]
        split
        next => exact confinement haSafe (fun hle => hcl ((T.join_le _ _ _).mp hle).2) u
        next => exact confinement hbSafe (fun hle => hcl ((T.join_le _ _ _).mp hle).2) u
        next => exact ⟨fun _ _ => rfl, rfl⟩
      exact ⟨low_trans (low_trans (conf s).1 low) (low_symm (conf t).1),
        (conf s).2.trans (conf t).2.symm⟩
#print axioms run_noninterference


def project (T : LabelTheory A) (L : K → A) (level : A) (row : K × Value) : Option (K × Value) :=
 if T.le (L row.1) level then some row else none
 omit [DecidableEq K] in
 theorem projected_visible (T : LabelTheory A) (L : K → A) (level : A) (rows : List (K × Value)) :
 rows.filterMap (project T L level) = visible T L level rows := by
  induction rows with
  | nil => rfl
  | cons r rs ih =>
    by_cases h : T.le (L r.1) level = true <;> simp [project,visible,h,ih,visible] at *
 theorem checked_retention_noninterference {T : LabelTheory A} {G : K → Ty} {L : K → A} {pc cmd level}
 (h : check T G L pc cmd = true) (s t : K → Value)
 (hs : ∀ k, (s k).ty = G k) (ht : ∀ k, (t k).ty = G k)
 (low : LowEq T L level s t) (cap : Nat) (initial : List (K × Value)) :
 Passive.feed (project T L level) cap initial (run s cmd).2 =
 Passive.feed (project T L level) cap initial (run t cmd).2 := by
  have eqrows := (run_noninterference (check_safe _ _ _ _ _ h) s t hs ht low).2
  rw [Passive.filter_before_retention,Passive.filter_before_retention,
      projected_visible,projected_visible,eqrows]
#print axioms checked_retention_noninterference
namespace Controls
-- Finite concrete algebra certificate; not a general source-theory verifier.
abbrev Label := Bool × Bool
def le (a b : Label) : Bool := ((!a.1) || b.1) && ((!a.2) || b.2)
def join (a b : Label) : Label := (a.1 || b.1, a.2 || b.2)
def diamond : LabelTheory Label where
 le := le
 bottom := (false,false)
 join := join
 refl := by
  intro ⟨a,b⟩; cases a <;> cases b <;> rfl
 trans := by
  intro ⟨a,b⟩ ⟨c,d⟩ ⟨e,f⟩
  cases a <;> cases b <;> cases c <;> cases d <;> cases e <;> cases f <;> decide
 bottom_le := by
  intro ⟨a,b⟩; cases a <;> cases b <;> rfl
 join_le := by
  intro ⟨a,b⟩ ⟨c,d⟩ ⟨e,f⟩
  cases a <;> cases b <;> cases c <;> cases d <;> cases e <;> cases f <;> decide
abbrev Key := Fin 5
def labels (k : Key) : Label :=
 if k = 0 then (false,false) else if k = 1 then (true,false)
 else if k = 2 then (false,true) else (true,true)
def types (k : Key) : Ty := if k = 4 then .bool else .int
def state (secret : Int) (flag : Bool) (k : Key) : Value :=
 if k = 0 then .int 10 else if k = 1 then .int 5 else if k = 2 then .int secret
 else if k = 3 then .int 20 else .bool flag
def good : Stmt Key := .seq (.write 0 (.add (.read 0) (.lit (.int 1))))
 (.seq (.write 3 (.add (.read 1) (.read 2)))
  (.seq (.branch (.read 4) (.write 3 (.lit (.int 1))) (.write 3 (.lit (.int 2))))
   (.write 1 (.read 0))))
example : diamond.le (labels 1) (labels 2) = false := by decide
example : diamond.le (labels 2) (labels 1) = false := by decide
example : check diamond types labels diamond.bottom good = true := by decide
example : visible diamond labels (labels 1) (run (state 0 true) good).2 =
 [(0,.int 11),(1,.int 11)] := by decide
example : visible diamond labels (labels 1) (run (state 42 false) good).2 =
 [(0,.int 11),(1,.int 11)] := by decide
example : check diamond types labels diamond.bottom (.write 2 (.read 1)) = false := by decide
example : check diamond types labels diamond.bottom
 (.branch (.read 4) (.write 1 (.lit (.int 10))) .skip) = false := by decide
-- A lawful preorder need not preserve the intended security interpretation.
def collapsed : LabelTheory Label where
 le := fun _ _ => true
 bottom := (false,false)
 join := fun a _ => a
 refl := by intro a; rfl
 trans := by intro a b c _ _; rfl
 bottom_le := by intro a; rfl
 join_le := by intro a b c; simp
example : check collapsed types labels collapsed.bottom (.write 0 (.read 2)) = true := by decide
example : check diamond types labels diamond.bottom (.write 0 (.read 2)) = false := by decide
-- Switching the policy invalidates the old interpretation, not the conditional theorem.
example : visible diamond labels (labels 0) (run (state 0 true) (.write 0 (.read 2))).2 ≠
 visible diamond labels (labels 0) (run (state 1 true) (.write 0 (.read 2))).2 := by decide
end Controls

#print axioms Controls.diamond
#check @run_noninterference

namespace FiniteTheory
structure Data (n : Nat) where
 le : Fin n → Fin n → Bool
 bottom : Fin n
 join : Fin n → Fin n → Fin n
def Laws {n : Nat} (d : Data n) : Prop :=
 (∀ a, d.le a a = true) ∧
 (∀ a b c, d.le a b = true → d.le b c = true → d.le a c = true) ∧
 (∀ a, d.le d.bottom a = true) ∧
 (∀ a b c, d.le (d.join a b) c = true ↔ d.le a c = true ∧ d.le b c = true)
def all (n : Nat) (p : Fin n → Bool) : Bool := (List.finRange n).all p
 theorem all_exact (n : Nat) (p : Fin n → Bool) : all n p = true ↔ ∀ i, p i = true := by
  simp [all,List.all_eq_true,List.mem_finRange]
def check {n : Nat} (d : Data n) : Bool :=
 all n (fun a => d.le a a) &&
 all n (fun a => all n (fun b => all n (fun c => !d.le a b || !d.le b c || d.le a c))) &&
 all n (fun a => d.le d.bottom a) &&
 all n (fun a => all n (fun b => all n (fun c => d.le (d.join a b) c == (d.le a c && d.le b c))))
 theorem implies_exact (a b c : Bool) : (!a || !b || c) = true ↔ (a = true → b = true → c = true) := by
  cases a <;> cases b <;> cases c <;> decide
 theorem equiv_exact (a b : Bool) : (a == b) = true ↔ (a = true ↔ b = true) := by
  cases a <;> cases b <;> decide
 theorem check_exact {n : Nat} (d : Data n) : check d = true ↔ Laws d := by
  simp only [check,Bool.and_eq_true,all_exact,implies_exact,equiv_exact,Laws,and_assoc]
def certify {n : Nat} (d : Data n) (h : check d = true) : LabelTheory (Fin n) where
 le := d.le
 bottom := d.bottom
 join := d.join
 refl := ((check_exact d).mp h).1
 trans := ((check_exact d).mp h).2.1 _ _ _
 bottom_le := ((check_exact d).mp h).2.2.1
 join_le := ((check_exact d).mp h).2.2.2
-- Exact table fields are retained, rather than a proof about another policy.
 theorem certify_le {n : Nat} (d : Data n) (h : check d = true) : (certify d h).le = d.le := rfl
 theorem certify_join {n : Nat} (d : Data n) (h : check d = true) : (certify d h).join = d.join := rfl
#print axioms check_exact
#print axioms certify

namespace Controls
 def chain : Data 2 where
  le := fun a b => decide (a.val ≤ b.val)
  bottom := 0
  join := fun a b => if a.val ≤ b.val then b else a
 def badJoin : Data 2 := { chain with join := fun _ _ => 0 }
 def badBottom : Data 2 := { chain with bottom := 1 }
 def noReflexivity : Data 2 := { chain with le := fun _ _ => false }
 example : check chain = true := by decide
 example : check badJoin = false := by decide
 example : check badBottom = false := by decide
 example : check noReflexivity = false := by decide
 -- Ignoring law validation would lose the high operands when computing the join.
 example : badJoin.le (badJoin.join 1 1) 0 = true := by decide
 example : chain.le (chain.join 1 1) 0 = false := by decide
 -- Transitivity-only countermodel: the other listed laws hold.
 def nontransitive : Data 3 where
  le a b := !(a == 2 && b == 0)
  bottom := 0
  join a b := if a == 2 || b == 2 then 2 else 0
 example : ∀ a, nontransitive.le a a = true := by decide
 example : ∀ a, nontransitive.le nontransitive.bottom a = true := by decide
 example : ∀ a b c, nontransitive.le (nontransitive.join a b) c = true ↔
  nontransitive.le a c = true ∧ nontransitive.le b c = true := by decide
 example : nontransitive.le 2 1 = true ∧ nontransitive.le 1 0 = true ∧
  nontransitive.le 2 0 = false := by decide
 example : check nontransitive = false := by decide
 -- An overlarge join preserves upper bounds but loses necessary low programs.
 def overlargeJoin : Data 2 := { chain with join := fun _ _ => 1 }
 example : check overlargeJoin = false := by decide
 example : overlargeJoin.le (overlargeJoin.join 0 0) 0 = false := by decide
end Controls

end FiniteTheory

end MirroreaProofFirst.GeneralLabels
