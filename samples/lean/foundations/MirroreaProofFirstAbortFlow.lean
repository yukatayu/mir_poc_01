import MirroreaProofFirstFallibleFlow
namespace MirroreaProofFirst.AbortFlow
open ProducerFlow
open FallibleFlow (Arithmetic Outcome)
variable {K : Type} [DecidableEq K]
abbrev Program (K : Type) := List (K × Expr K)
-- Actual execution aborts at the first failed assignment.
def run (ops : Arithmetic) (s : K → Value) : Program K → (K → Value) × List (Outcome K)
 | [] => (s,[])
 | (k,e)::tail => match FallibleFlow.eval ops s e with
   | none => (s,[.failed k])
   | some v => let rest := run ops (put s k v) tail
               (rest.1,.wrote k v::rest.2)
def visible (L : K → Nat) (level : Nat) (events : List (Outcome K)) :=
 events.filterMap (FallibleFlow.project L level)
-- Completion dependencies raise the program counter for all following operations.
inductive Safe (G : K → Ty) (L : K → Nat) : Nat → Program K → Prop
 | nil : Safe G L pc []
 | cons : Typed G e (G k) → Flows L (L k) e → pc ≤ L k →
          Safe G L (max pc (rank L e)) tail → Safe G L pc ((k,e)::tail)
def check (G : K → Ty) (L : K → Nat) (pc : Nat) : Program K → Bool
 | [] => true
 | (k,e)::tail => decide (infer G e = some (G k) ∧ rank L e ≤ L k ∧ pc ≤ L k) &&
                 check G L (max pc (rank L e)) tail
 omit [DecidableEq K] in
 theorem check_exact (G : K → Ty) (L : K → Nat) (pc : Nat) (p : Program K) :
 check G L pc p = true ↔ Safe G L pc p := by
  induction p generalizing pc with
  | nil => constructor <;> intro h; exact .nil; rfl
  | cons head tail ih =>
    obtain ⟨k,e⟩ := head
    simp only [check,Bool.and_eq_true,decide_eq_true_eq]
    constructor
    · rintro ⟨⟨ht,hf,hp⟩,hr⟩
      exact .cons (infer_typed _ _ _ ht) (rank_flows _ _ _ hf) hp ((ih _).mp hr)
    · intro h; cases h with
      | cons ht hf hp hr => exact ⟨⟨typed_infer ht, flows_rank hf,hp⟩,(ih _).mpr hr⟩
 theorem confinement {G : K → Ty} {L : K → Nat} {pc p level}
 (h : Safe G L pc p) (high : level < pc) (ops : Arithmetic) (s : K → Value) :
 LowEq L level (run ops s p).1 s ∧ visible L level (run ops s p).2 = [] := by
  induction h generalizing s with
  | nil => exact ⟨fun _ _ => rfl,rfl⟩
  | cons ht hf hp hr ih =>
    rename_i e k pc tail
    have hk : ¬ L k ≤ level := by omega
    simp only [run]
    split
    next he => exact ⟨fun _ _ => rfl,by simp [visible,FallibleFlow.project,Outcome.key,hk]⟩
    next v he =>
      have rest := ih (by omega) (put s k v)
      constructor
      · apply low_trans rest.1
        intro x hx
        have hn : x ≠ k := by intro eq; subst x; exact hk hx
        simp [put,hn]
      · simpa [visible,FallibleFlow.project,Outcome.key,hk] using rest.2
 theorem noninterference {G : K → Ty} {L : K → Nat} {pc p level}
 (h : Safe G L pc p) (ops : Arithmetic) (s t : K → Value) (low : LowEq L level s t) :
 LowEq L level (run ops s p).1 (run ops t p).1 ∧
 visible L level (run ops s p).2 = visible L level (run ops t p).2 := by
  induction h generalizing s t with
  | nil => exact ⟨low,rfl⟩
  | cons ht hf hp hr ih =>
    rename_i e k pc tail
    have flow := flows_rank hf
    by_cases heLow : rank L e ≤ level
    · have eq := FallibleFlow.eval_low ops low e heLow
      simp only [run,eq]
      split
      next he => exact ⟨low,rfl⟩
      next v he =>
        have lp : LowEq L level (put s k v) (put t k v) := by
          intro x hx; by_cases h : x = k
          · simp [put,h]
          · simpa [put,h] using low x hx
        have rest := ih (put s k v) (put t k v) lp
        constructor
        · exact rest.1
        · change (List.filterMap (FallibleFlow.project L level) (.wrote k v :: (run ops (put s k v) tail).2)) = _
          simp only [visible,List.filterMap_cons]
          cases hproj : FallibleFlow.project L level (.wrote k v)
          · exact rest.2
          · exact congrArg (List.cons _) rest.2
    · have hk : ¬ L k ≤ level := by omega
      have high : level < max pc (rank L e) := by omega
      have frame (u : K → Value) :
       LowEq L level (run ops u ((k,e)::tail)).1 u ∧
       visible L level (run ops u ((k,e)::tail)).2 = [] := by
        simp only [run]
        split
        next he => exact ⟨fun _ _ => rfl,by simp [visible,FallibleFlow.project,Outcome.key,hk]⟩
        next v he =>
          have rest := confinement hr high ops (put u k v)
          constructor
          · apply low_trans rest.1
            intro x hx; have hn : x ≠ k := by intro eq; subst x; exact hk hx
            simp [put,hn]
          · simpa [visible,FallibleFlow.project,Outcome.key,hk] using rest.2
      exact ⟨low_trans (frame s).1 (low_trans low (low_symm (frame t).1)),(frame s).2.trans (frame t).2.symm⟩
theorem run_typed {G : K → Ty} {L : K → Nat} {pc p}
 (h : Safe G L pc p) (ops : Arithmetic) (s : K → Value) (hs : ∀ k, (s k).ty = G k) :
 ∀ k, ((run ops s p).1 k).ty = G k := by
  induction h generalizing s with
  | nil => exact hs
  | cons ht hf hp hr ih =>
    simp only [run]
    split
    next he => exact hs
    next v he => exact ih _ (put_typed hs (FallibleFlow.eval_typed ht ops s hs v he))
-- This includes only arithmetic completion. Authority/existence/resources remain separate.
def completionLevel (L : K → Nat) : Program K → Nat
 | [] => 0
 | (_,e)::tail => max (rank L e) (completionLevel L tail)
def completed (events : List (Outcome K)) : Bool :=
 events.all (fun event => match event with | .failed _ => false | .wrote _ _ => true)
theorem full_trace_when_inputs_low (ops : Arithmetic) (L : K → Nat) (level : Nat)
 (p : Program K) (s t : K → Value) (low : LowEq L level s t)
 (dep : completionLevel L p ≤ level) : (run ops s p).2 = (run ops t p).2 := by
  induction p generalizing s t with
  | nil => rfl
  | cons head tail ih =>
    obtain ⟨k,e⟩ := head
    have he : rank L e ≤ level := (Nat.max_le.mp dep).1
    have ht : completionLevel L tail ≤ level := (Nat.max_le.mp dep).2
    have eq := FallibleFlow.eval_low ops low e he
    simp only [run,eq]
    split
    next => rfl
    next v hv =>
      have lp : LowEq L level (put s k v) (put t k v) := by
        intro x hx; by_cases h : x=k
        · simp [put,h]
        · simpa [put,h] using low x hx
      exact congrArg (List.cons _) (ih _ _ lp ht)
theorem completion_noninterference (ops : Arithmetic) (L : K → Nat) (level : Nat)
 (p : Program K) (s t : K → Value) (low : LowEq L level s t) :
 (if completionLevel L p ≤ level then some (completed (run ops s p).2) else none) =
 (if completionLevel L p ≤ level then some (completed (run ops t p).2) else none) := by
  split
  next h => rw [full_trace_when_inputs_low ops L level p s t low h]
  next h => rfl
theorem retained_noninterference {G : K → Ty} {L : K → Nat} {pc p level}
 (h : Safe G L pc p) (ops : Arithmetic) (s t : K → Value) (low : LowEq L level s t)
 (cap : Nat) (initial : List (Outcome K)) :
 Passive.feed (FallibleFlow.project L level) cap initial (run ops s p).2 =
 Passive.feed (FallibleFlow.project L level) cap initial (run ops t p).2 := by
  rw [Passive.filter_before_retention,Passive.filter_before_retention]
  exact congrArg (Passive.feedRows cap initial) (noninterference h ops s t low).2
namespace Controls
open FallibleFlow.Controls (Key labels state increment lowIncrement)
def G : Key → Ty := fun _ => .int
def leakySequence : Program Key := [(1,increment),(0,.lit (.int 11))]
def safe : Program Key := [(0,lowIncrement),(1,increment),(1,.lit (.int 11))]
example : check G labels 0 leakySequence = false := by decide
example : check G labels 0 safe = true := by decide
example : visible labels 0 (run (FallibleFlow.signed 63) (state 0) safe).2 = [.wrote 0 (.int 11)] := by decide
example : visible labels 0 (run (FallibleFlow.signed 63) (state 9223372036854775807) safe).2 = [.wrote 0 (.int 11)] := by decide
-- Rejected counterexample really has different public state and actual events.
example : (run (FallibleFlow.signed 63) (state 0) leakySequence).1 0 ≠
 (run (FallibleFlow.signed 63) (state 9223372036854775807) leakySequence).1 0 := by decide
-- Always exposing completion leaks even for accepted safe.
example : completed (run (FallibleFlow.signed 63) (state 0) safe).2 ≠
 completed (run (FallibleFlow.signed 63) (state 9223372036854775807) safe).2 := by decide
-- Not all high-target operations force later low work high: constant arithmetic is public.
example : check G labels 0 [(1,.lit (.int 7)),(0,lowIncrement)] = true := by decide
end Controls
#print axioms run_typed
#print axioms full_trace_when_inputs_low
#print axioms completion_noninterference
#print axioms retained_noninterference
#print axioms check_exact
#print axioms confinement
#print axioms noninterference
end MirroreaProofFirst.AbortFlow
