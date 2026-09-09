import MirroreaProofFirstAbortFlow
namespace MirroreaProofFirst.AddressFlow
open ProducerFlow
variable {S K : Type}
def rename (resolve : S → K) : Expr S → Expr K
 | .lit v => .lit v
 | .read x => .read (resolve x)
 | .add a b => .add (rename resolve a) (rename resolve b)
 | .sub a b => .sub (rename resolve a) (rename resolve b)
 | .choose c a b => .choose (rename resolve c) (rename resolve a) (rename resolve b)
theorem eval_commutes (ops : FallibleFlow.Arithmetic) (resolve : S → K) (s : K → Value) (e : Expr S) :
 FallibleFlow.eval ops s (rename resolve e) = FallibleFlow.eval ops (s ∘ resolve) e := by
 induction e <;> simp_all [rename,FallibleFlow.eval,Function.comp_def]
theorem rank_commutes (resolve : S → K) (L : K → Nat) (e : Expr S) :
 rank L (rename resolve e) = rank (L ∘ resolve) e := by
 induction e <;> simp_all [rename,rank,Function.comp_def]
theorem infer_commutes (resolve : S → K) (G : K → Ty) (e : Expr S) :
 infer G (rename resolve e) = infer (G ∘ resolve) e := by
 induction e <;> simp_all [rename,infer,Function.comp_def]
-- Source references may alias; treating each spelling as an independent cell is wrong.
variable [DecidableEq K]
def aliasPut (resolve : S → K) (s : S → Value) (x : S) (v : Value) : S → Value :=
 fun y => if resolve y = resolve x then v else s y
theorem put_commutes (resolve : S → K) (s : K → Value) (x : S) (v : Value) :
 (put s (resolve x) v) ∘ resolve = aliasPut resolve (s ∘ resolve) x v := rfl
def lower (resolve : S → K) (p : AbortFlow.Program S) : AbortFlow.Program K :=
 p.map (fun (x,e) => (resolve x,rename resolve e))
def outcome (resolve : S → K) : FallibleFlow.Outcome S → FallibleFlow.Outcome K
 | .failed x => .failed (resolve x)
 | .wrote x v => .wrote (resolve x) v
-- Alias-aware source reference model, not a source byte parser or authority resolver.
def run (ops : FallibleFlow.Arithmetic) (resolve : S → K) (s : S → Value) :
 AbortFlow.Program S → (S → Value) × List (FallibleFlow.Outcome S)
 | [] => (s,[])
 | (x,e)::tail => match FallibleFlow.eval ops s e with
   | none => (s,[.failed x])
   | some v => let rest := run ops resolve (aliasPut resolve s x v) tail
               (rest.1,.wrote x v::rest.2)
theorem run_commutes (ops : FallibleFlow.Arithmetic) (resolve : S → K) (s : K → Value)
 (p : AbortFlow.Program S) :
 (AbortFlow.run ops s (lower resolve p)).1 ∘ resolve = (run ops resolve (s ∘ resolve) p).1 ∧
 (AbortFlow.run ops s (lower resolve p)).2 = (run ops resolve (s ∘ resolve) p).2.map (outcome resolve) := by
 induction p generalizing s with
 | nil => exact ⟨rfl,rfl⟩
 | cons head tail ih =>
   obtain ⟨x,e⟩ := head
   cases he : FallibleFlow.eval ops (s ∘ resolve) e with
   | none => simp [lower,AbortFlow.run,run,eval_commutes,he,outcome]
   | some v =>
     have h := ih (put s (resolve x) v)
     rw [put_commutes] at h
     simpa [lower,AbortFlow.run,run,eval_commutes,he,outcome] using
       And.intro h.1 (congrArg (List.cons (.wrote (resolve x) v)) h.2)
omit [DecidableEq K] in
theorem check_commutes (resolve : S → K) (G : K → Ty) (L : K → Nat) (pc : Nat)
 (p : AbortFlow.Program S) :
 AbortFlow.check G L pc (lower resolve p) = AbortFlow.check (G ∘ resolve) (L ∘ resolve) pc p := by
 induction p generalizing pc with
 | nil => rfl
 | cons head tail ih =>
   obtain ⟨x,e⟩ := head
   simp only [lower,List.map_cons,AbortFlow.check,rank_commutes,infer_commutes,Function.comp_apply]
   exact congrArg (fun b => decide (infer (G ∘ resolve) e = some (G (resolve x)) ∧
    rank (L ∘ resolve) e ≤ L (resolve x) ∧ pc ≤ L (resolve x)) && b) (ih _)
namespace Controls
abbrev Ref := Fin 2
abbrev Cell := Fin 1
def resolve (_ : Ref) : Cell := 0
def store (_ : Cell) : Value := .int 10
def program : AbortFlow.Program Ref := [(0,.lit (.int 11)),(1,.add (.read 1) (.lit (.int 1)))]
example : (AbortFlow.run (FallibleFlow.signed 63) store (lower resolve program)).1 0 = .int 12 := by decide
example : (run (FallibleFlow.signed 63) resolve (store ∘ resolve) program).1 0 = .int 12 := by decide
example : (AbortFlow.run (FallibleFlow.signed 63) (store ∘ resolve) program).1 0 = .int 11 := by decide
example : (AbortFlow.run (FallibleFlow.signed 63) (store ∘ resolve) program).1 1 = .int 11 := by decide
-- Inconsistent classes for aliased references cannot be a pullback of one cell policy.
example : ¬ ∃ L : Cell → Nat, (L ∘ resolve) 0 = 0 ∧ (L ∘ resolve) 1 = 1 := by
 rintro ⟨L,h0,h1⟩; simp [resolve] at h0 h1; omega
end Controls
#print axioms eval_commutes
#print axioms rank_commutes
#print axioms infer_commutes
#print axioms put_commutes
#print axioms run_commutes
#print axioms check_commutes
end MirroreaProofFirst.AddressFlow
