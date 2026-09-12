import MirroreaProofFirstGraphValidation

namespace MirroreaProofFirst.LocalImpact
open Support GraphValidation

-- Include BOTH branches. A witness's selected branch is not the full footprint
-- of a validator which may search, fail over, or certify missing support.
def refs : Formula K → List K
  | .top | .bottom => []
  | .ref k => [k]
  | .both p q | .either p q => refs p ++ refs q

theorem only_refs (P : K → Prop) (f : Formula K) :
    Only P f ↔ ∀ k ∈ refs f, P k := by
  induction f with
  | top => simp [Only,refs]
  | bottom => simp [Only,refs]
  | ref k => simp [Only,refs]
  | both p q hp hq => simp [Only,refs,hp,hq,List.mem_append,or_imp,forall_and]
  | either p q hp hq => simp [Only,refs,hp,hq,List.mem_append,or_imp,forall_and]

def dependency (forms : K → Formula K) (k j : K) : Prop := j ∈ refs (forms k)

def Affected (forms : K → Formula K) (changed : K → Prop) (k : K) : Prop :=
  ∃ j, changed j ∧ Path (dependency forms) k j

theorem changed_affected (forms : K → Formula K) (changed : K → Prop)
    (h : changed k) : Affected forms changed k := ⟨k,h,.refl⟩

theorem unaffected_closed (forms : K → Formula K) (changed : K → Prop)
    (clean : ¬ Affected forms changed k) : Only (fun j => ¬ Affected forms changed j) (forms k) := by
  apply (only_refs _ _).mpr
  intro j dep bad
  obtain ⟨z,hz,path⟩ := bad
  exact clean ⟨z,hz,path_trans (.step .refl dep) path⟩

-- Coverage concerns changed INPUTS, not the derived live result. Any mutation
-- of formulas or eligibility is allowed inside the declared changed set.
theorem unaffected_grounded (forms other : K → Formula K) (E F : K → Bool)
    (changed : K → Prop)
    (coveredForms : ∀ k, ¬ changed k → forms k = other k)
    (coveredEligibility : ∀ k, ¬ changed k → E k = F k)
    (clean : ¬ Affected forms changed k) :
    Grounded forms (fun j => E j = true) k ↔ Grounded other (fun j => F j = true) k := by
  exact low_grounded_independent (fun j => ¬ Affected forms changed j) forms other E F
    (fun j hj => coveredForms j (fun hc => hj (changed_affected forms changed hc)))
    (fun j hj => coveredEligibility j (fun hc => hj (changed_affected forms changed hc)))
    (fun _ hj => unaffected_closed forms changed hj) k clean

def dependencyBool (forms : Fin n → Formula (Fin n)) (k j : Fin n) : Bool :=
  (refs (forms k)).contains j

theorem dependencyBool_exact (forms : Fin n → Formula (Fin n)) (k j : Fin n) :
    dependencyBool forms k j = true ↔ dependency forms k j := by
  simp [dependencyBool,dependency]

def affected (forms : Fin n → Formula (Fin n)) (changed : Fin n → Bool)
    (k : Fin n) : Bool :=
  (List.finRange n).any fun j => changed j && reachable (dependencyBool forms) k j

theorem affected_exact (forms : Fin n → Formula (Fin n)) (changed : Fin n → Bool)
    (k : Fin n) : affected forms changed k = true ↔ Affected forms (fun j => changed j = true) k := by
  have edgeEq : (fun x y => dependencyBool forms x y = true) = dependency forms := by
    funext x y
    exact propext (dependencyBool_exact forms x y)
  simp only [affected,List.any_eq_true,Bool.and_eq_true,reachable_exact,edgeEq,Affected]
  simp

theorem unaffected_computed (s t : Snapshot n) (changed : Fin n → Bool)
    (forms : ∀ k, changed k = false → s.forms k = t.forms k)
    (eligibility : ∀ k, changed k = false → s.eligible k = t.eligible k)
    (clean : affected s.forms changed k = false) : snapshotLive s k = snapshotLive t k := by
  apply Bool.eq_iff_iff.mpr
  rw [snapshot_live_exact,snapshot_live_exact]
  apply unaffected_grounded s.forms t.forms s.eligible t.eligible (fun j => changed j = true)
    (fun j hj => forms j (by cases h : changed j <;> simp_all))
    (fun j hj => eligibility j (by cases h : changed j <;> simp_all))
  intro h
  have bad := (affected_exact s.forms changed k).mpr h
  simp [clean] at bad

namespace Controls
def forms (k : Fin 4) : Formula (Fin 4) :=
  if k = 0 then .either (.ref 1) (.ref 2) else .top
#guard affected forms (fun k => k == 2) 0
#guard !affected forms (fun k => k == 2) 1
#guard !affected forms (fun k => k == 2) 3
def base : Snapshot 4 := ⟨forms,fun _ => true⟩
def changedUnused : Snapshot 4 := ⟨forms,fun k => k != 2⟩
-- The syntactic affected set is deliberately conservative, not an assertion
-- that every included node's current truth value necessarily changes.
#guard snapshotLive base 0 && snapshotLive changedUnused 0
#guard snapshotLive base 3 == snapshotLive changedUnused 3
end Controls

#print axioms only_refs
#print axioms unaffected_closed
#print axioms unaffected_grounded
#print axioms affected_exact
#print axioms unaffected_computed
end MirroreaProofFirst.LocalImpact
