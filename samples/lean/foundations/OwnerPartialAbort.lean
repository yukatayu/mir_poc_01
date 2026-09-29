import MirroreaProofFirstOwnerPartial
import MirroreaProofFirstAbortFlow
namespace MirroreaProofFirst.OwnerPartialAbort
open ProducerFlow
open FallibleFlow (Arithmetic Outcome)
variable {K : Type} [DecidableEq K]
abbrev Program (K : Type) := AbortFlow.Program K
abbrev Safe := @AbortFlow.Safe
abbrev check := @AbortFlow.check
abbrev LowEq := @OwnerPartial.LowEq
abbrev put := @OwnerPartial.put

-- Reuse the established completion-sensitive judgment. Extend only evaluation
-- to genuine absent lookups, never an arbitrary numeric default. These labels
-- are supplied/authenticity-unproved; authority/resource/elapsed-time effects
-- remain additional completion dependencies at physical/source integration.
def run (ops : Arithmetic) (s : K → Option Value) : Program K →
 (K → Option Value) × List (Outcome K)
 | [] => (s,[])
 | (k,e)::tail => match OwnerPartial.eval ops s e with
   | none => (s,[.failed k])
   | some v => let rest := run ops (put s k v) tail
               (rest.1,.wrote k v::rest.2)
def visible (L : K → Nat) (level : Nat) (events : List (Outcome K)) :=
 events.filterMap (FallibleFlow.project L level)

theorem low_trans (left : LowEq L level s t) (right : LowEq L level t u) : LowEq L level s u :=
 fun k h => (left k h).trans (right k h)
theorem low_symm (low : LowEq L level s t) : LowEq L level t s := fun k h => (low k h).symm

 theorem confinement {G : K → Ty} {L : K → Nat} {pc p level}
 (h : Safe G L pc p) (high : level < pc) (ops : Arithmetic) (s : K → Option Value) :
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
        simp [put,OwnerPartial.put,hn]
      · simpa [visible,FallibleFlow.project,Outcome.key,hk] using rest.2
 theorem noninterference {G : K → Ty} {L : K → Nat} {pc p level}
 (h : Safe G L pc p) (ops : Arithmetic) (s t : K → Option Value) (low : LowEq L level s t) :
 LowEq L level (run ops s p).1 (run ops t p).1 ∧
 visible L level (run ops s p).2 = visible L level (run ops t p).2 := by
  induction h generalizing s t with
  | nil => exact ⟨low,rfl⟩
  | cons ht hf hp hr ih =>
    rename_i e k pc tail
    have flow := flows_rank hf
    by_cases heLow : rank L e ≤ level
    · have eq := OwnerPartial.eval_low ops low e heLow
      simp only [run,eq]
      split
      next he => exact ⟨low,rfl⟩
      next v he =>
        have lp : LowEq L level (put s k v) (put t k v) := by
          intro x hx; by_cases h : x = k
          · simp [put,OwnerPartial.put,h]
          · simpa [put,OwnerPartial.put,h] using low x hx
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
      have frame (u : K → Option Value) :
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
            simp [put,OwnerPartial.put,hn]
          · simpa [visible,FallibleFlow.project,Outcome.key,hk] using rest.2
      exact ⟨low_trans (frame s).1 (low_trans low (low_symm (frame t).1)),(frame s).2.trans (frame t).2.symm⟩

-- No need to re-prove or weaken the inherited declarative/checker equivalence.
omit [DecidableEq K] in
theorem check_exact (G : K → Ty) (L : K → Nat) (pc : Nat) (p : Program K) :
 check G L pc p = true ↔ Safe G L pc p := AbortFlow.check_exact G L pc p

-- Optional presence does not weaken the type of values that actually exist.
omit [DecidableEq K] in
theorem eval_typed {G : K → Ty} {e : Expr K} {ty : Ty} (typed : Typed G e ty)
 (ops : Arithmetic) (state : K → Option Value)
 (valid : ∀ k v, state k = some v → v.ty = G k)
 (v : Value) (evaluated : OwnerPartial.eval ops state e = some v) : v.ty = ty := by
 induction typed with
 | lit => simp only [OwnerPartial.eval,Option.some.injEq] at evaluated; subst v; rfl
 | read => exact valid _ v evaluated
 | add left right ihl ihr =>
   simp only [OwnerPartial.eval] at evaluated
   split at evaluated
   · rename_i x y ex ey
     cases result : ops.add x y with
     | none => simp [result] at evaluated
     | some z => simp only [result,Option.map_some,Option.some.injEq] at evaluated; rw [←evaluated]; rfl
   · contradiction
 | sub left right ihl ihr =>
   simp only [OwnerPartial.eval] at evaluated
   split at evaluated
   · rename_i x y ex ey
     cases result : ops.sub x y with
     | none => simp [result] at evaluated
     | some z => simp only [result,Option.map_some,Option.some.injEq] at evaluated; rw [←evaluated]; rfl
   · contradiction
 | choose cond left right ihc ihl ihr =>
   simp only [OwnerPartial.eval] at evaluated
   split at evaluated
   · exact ihl evaluated
   · exact ihr evaluated
   · contradiction

theorem run_typed (safe : Safe G L pc program) (ops : Arithmetic) (state : K → Option Value)
 (valid : ∀ k v, state k = some v → v.ty = G k) :
 ∀ k v, (run ops state program).1 k = some v → v.ty = G k := by
 induction safe generalizing state with
 | nil => exact valid
 | cons typed flow control tail ih =>
   rename_i e target pc rest
   simp only [run]
   split
   · exact valid
   · rename_i value evaluated
     apply ih
     intro k v present
     by_cases same : k = target
     · subst k
       have equal : value = v := by simpa [put,OwnerPartial.put] using present
       subst v
       exact eval_typed typed ops state valid value evaluated
     · exact valid k v (by simpa [put,OwnerPartial.put,same] using present)

abbrev completionLevel := @AbortFlow.completionLevel
abbrev completed := @AbortFlow.completed
theorem full_trace_when_inputs_low (ops : Arithmetic) (L : K → Nat) (level : Nat)
 (p : Program K) (s t : K → Option Value) (low : LowEq L level s t)
 (dep : completionLevel L p ≤ level) : (run ops s p).2 = (run ops t p).2 := by
  induction p generalizing s t with
  | nil => rfl
  | cons head tail ih =>
    obtain ⟨k,e⟩ := head
    have he : rank L e ≤ level := (Nat.max_le.mp dep).1
    have ht : completionLevel L tail ≤ level := (Nat.max_le.mp dep).2
    have eq := OwnerPartial.eval_low ops low e he
    simp only [run,eq]
    split
    next => rfl
    next v hv =>
      have lp : LowEq L level (put s k v) (put t k v) := by
        intro x hx; by_cases h : x=k
        · simp [put,OwnerPartial.put,h]
        · simpa [put,OwnerPartial.put,h] using low x hx
      exact congrArg (List.cons _) (ih _ _ lp ht)
theorem completion_noninterference (ops : Arithmetic) (L : K → Nat) (level : Nat)
 (p : Program K) (s t : K → Option Value) (low : LowEq L level s t) :
 (if completionLevel L p ≤ level then some (completed (run ops s p).2) else none) =
 (if completionLevel L p ≤ level then some (completed (run ops t p).2) else none) := by
  split
  next h => rw [full_trace_when_inputs_low ops L level p s t low h]
  next h => rfl
theorem retained_noninterference {G : K → Ty} {L : K → Nat} {pc p level}
 (h : Safe G L pc p) (ops : Arithmetic) (s t : K → Option Value) (low : LowEq L level s t)
 (cap : Nat) (initial : List (Outcome K)) :
 Passive.feed (FallibleFlow.project L level) cap initial (run ops s p).2 =
 Passive.feed (FallibleFlow.project L level) cap initial (run ops t p).2 := by
  rw [Passive.filter_before_retention,Passive.filter_before_retention]
  exact congrArg (Passive.feedRows cap initial) (noninterference h ops s t low).2

#print axioms eval_typed
#print axioms run_typed
#print axioms completion_noninterference
#print axioms retained_noninterference
#print axioms check_exact
#print axioms confinement
#print axioms noninterference
end MirroreaProofFirst.OwnerPartialAbort
