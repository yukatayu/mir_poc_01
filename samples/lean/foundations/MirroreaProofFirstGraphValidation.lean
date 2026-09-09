import MirroreaProofFirstSupport
import MirroreaProofFirstTrackedValidation
namespace MirroreaProofFirst.GraphValidation
open Support
-- One explicit relation graph, not a universal DAG across all support kinds.
inductive Path (edge : K → K → Prop) : K → K → Prop where
  | refl : Path edge x x
  | step : Path edge x y → edge y z → Path edge x z

theorem path_trans {edge : K → K → Prop} (h : Path edge x y) (t : Path edge y z) :
    Path edge x z := by
  induction t with
  | refl => exact h
  | step _ e ih => exact .step ih e

def Acyclic (edge : K → K → Prop) : Prop := ∀ x y, edge x y → ¬ Path edge y x

def add (edge : K → K → Prop) (source target : K) : K → K → Prop :=
  fun x y => edge x y ∨ (x = source ∧ y = target)

theorem added_path {edge : K → K → Prop} {source target : K}
    (noBack : ¬ Path edge target source) {x y : K}
    (h : Path (add edge source target) x y) :
    Path edge x y ∨ (Path edge x source ∧ Path edge target y) := by
  induction h with
  | refl => exact Or.inl .refl
  | @step y z h e ih =>
    rcases e with old | ⟨rfl,rfl⟩
    · exact ih.elim (fun p => Or.inl (.step p old))
        (fun p => Or.inr ⟨p.1,.step p.2 old⟩)
    · exact ih.elim (fun p => Or.inr ⟨p,.refl⟩)
        (fun p => False.elim (noBack p.2))

theorem add_preserves {edge : K → K → Prop} {source target : K}
    (acyclic : Acyclic edge) (noBack : ¬ Path edge target source) :
    Acyclic (add edge source target) := by
  intro x y e back
  rcases e with old | ⟨rfl,rfl⟩
  · rcases added_path noBack back with p | ⟨p,q⟩
    · exact acyclic x y old p
    · exact noBack (path_trans (.step q old) p)
  · rcases added_path noBack back with p | ⟨p,_⟩
    · exact noBack p
    · exact noBack p

def reachForms (edge : Fin n → Fin n → Bool) (start k : Fin n) : Formula (Fin n) :=
  if k = start then .top else
    anyOf (((List.finRange n).filter fun j => edge j k).map Formula.ref)

theorem reachForms_holds (edge : Fin n → Fin n → Bool) (start k : Fin n)
    (A : Fin n → Prop) :
    Holds A (reachForms edge start k) ↔ k = start ∨ ∃ j, edge j k = true ∧ A j := by
  by_cases h : k = start
  · simp [reachForms,h,Holds]
  · simp only [reachForms, h, ↓reduceIte, anyOf_correct, false_or]
    constructor
    · rintro ⟨f,hf,holds⟩
      obtain ⟨j,hj,rfl⟩ := List.mem_map.mp hf
      exact ⟨j,(List.mem_filter.mp hj).2,holds⟩
    · rintro ⟨j,he,ha⟩
      exact ⟨.ref j, List.mem_map.mpr ⟨j,by simp [he],rfl⟩,ha⟩

theorem grounded_path (edge : Fin n → Fin n → Bool) (start k : Fin n) :
    Grounded (reachForms edge start) (fun _ => True) k ↔
      Path (fun x y => edge x y = true) start k := by
  constructor
  · apply grounded_least
    intro j _ hf
    rcases (reachForms_holds edge start j _).mp hf with eq | ⟨x,he,hp⟩
    · subst j; exact .refl
    · exact .step hp he
  · intro h
    induction h with
    | refl =>
      apply grounded_closed (reachForms edge start) (fun _ => True) _ True.intro
      exact (reachForms_holds edge start start _).mpr (Or.inl rfl)
    | @step y z hp he ih =>
      apply grounded_closed (reachForms edge start) (fun _ => True) _ True.intro
      exact (reachForms_holds edge start z _).mpr (Or.inr ⟨y,he,ih⟩)

def reachable (edge : Fin n → Fin n → Bool) (start target : Fin n) : Bool :=
  rounds (reachForms edge start) (fun _ => true) n target

theorem reachable_exact (edge : Fin n → Fin n → Bool) (start target : Fin n) :
    reachable edge start target = true ↔ Path (fun x y => edge x y = true) start target := by
  have hb := bounded_rounds_exact (List.finRange n) (fun k => by simp) (reachForms edge start) (fun _ => true) target
  simp only [List.length_finRange] at hb
  exact hb.trans (grounded_path edge start target)

#print axioms add_preserves
#print axioms reachable_exact

theorem path_mono {edge other : K → K → Prop}
    (subset : ∀ x y, edge x y → other x y) {x y : K} (h : Path edge x y) :
    Path other x y := by
  induction h with
  | refl => exact .refl
  | step _ he ih => exact .step ih (subset _ _ he)

theorem acyclic_subgraph {edge other : K → K → Prop}
    (subset : ∀ x y, edge x y → other x y) (valid : Acyclic other) : Acyclic edge := by
  intro x y he back
  exact valid x y (subset x y he) (path_mono subset back)

theorem add_requires_no_back {edge : K → K → Prop} {source target : K}
    (valid : Acyclic (add edge source target)) : ¬ Path edge target source := by
  intro back
  exact valid source target (Or.inr ⟨rfl,rfl⟩)
    (path_mono (fun _ _ he => Or.inl he) back)

def checkAdd (edge : Fin n → Fin n → Bool) (source target : Fin n) : Bool :=
  decide (reachable edge target source ≠ true)

theorem checkAdd_exact (edge : Fin n → Fin n → Bool) (source target : Fin n)
    (valid : Acyclic (fun x y => edge x y = true)) :
    checkAdd edge source target = true ↔
      Acyclic (add (fun x y => edge x y = true) source target) := by
  simp only [checkAdd, decide_eq_true_eq]
  constructor
  · intro checked
    apply add_preserves valid
    intro back
    exact checked ((reachable_exact edge target source).mpr back)
  · intro h accepted
    exact add_requires_no_back h ((reachable_exact edge target source).mp accepted)

-- Whole-graph checker is an alternative validation strategy for unrestricted
-- finite edge patches; checking only one named edge cannot accept their shape.
def checkAcyclic (edge : Fin n → Fin n → Bool) : Bool :=
  (List.finRange n).all fun x => (List.finRange n).all fun y =>
    decide (edge x y = true → reachable edge y x ≠ true)

theorem checkAcyclic_exact (edge : Fin n → Fin n → Bool) :
    checkAcyclic edge = true ↔ Acyclic (fun x y => edge x y = true) := by
  simp only [checkAcyclic, List.all_eq_true, List.mem_finRange, forall_const, decide_eq_true_eq]
  constructor
  · intro h x y he back
    exact h x y he ((reachable_exact edge y x).mpr back)
  · intro h x y he back
    exact h x y he ((reachable_exact edge y x).mp back)

namespace Controls
def empty : Fin 3 → Fin 3 → Bool := fun _ _ => false
def line : Fin 3 → Fin 3 → Bool := fun x y => decide (x.val + 1 = y.val)
def extra : Fin 3 → Fin 3 → Bool := fun x y => decide (x = 0 ∧ (y = 0 ∨ y = 1))
#guard checkAcyclic empty
#guard checkAcyclic line
#guard checkAdd line 0 2
#guard !checkAdd line 2 0
#guard !checkAdd empty 0 0
#guard !checkAcyclic extra
-- Validating designated edge 0→1 alone misses another added self-loop at0.
#guard checkAdd extra 0 1
end Controls
#print axioms acyclic_subgraph
#print axioms checkAdd_exact
#print axioms checkAcyclic_exact

open TrackedValidation

def scan [DecidableEq K] (keys : List K) (acc : K → Option V)
    (finish : (K → Option V) → A) : Query K V A :=
  match keys with
  | [] => .done (finish acc)
  | k :: rest => .read k fun v => scan rest (fun j => if j = k then v else acc j) finish

def sampled [DecidableEq K] (s : K → Option V) (keys : List K) (acc : K → Option V) : K → Option V :=
  match keys with
  | [] => acc
  | k :: rest => sampled s rest (fun j => if j = k then s k else acc j)

theorem scan_evaluates [DecidableEq K] (s : K → Option V) (keys : List K)
    (acc : K → Option V) (finish : (K → Option V) → A) :
    evaluate s (scan keys acc finish) = (finish (sampled s keys acc), keys) := by
  induction keys generalizing acc with
  | nil => rfl
  | cons k rest ih => simp [scan, evaluate, ih, sampled]

theorem sampled_exact [DecidableEq K] (s : K → Option V) (keys : List K)
    (acc : K → Option V) (k : K) :
    sampled s keys acc k = if k ∈ keys then s k else acc k := by
  induction keys generalizing acc with
  | nil => simp [sampled]
  | cons j rest ih =>
    simp only [sampled, ih, List.mem_cons]
    by_cases hr : k ∈ rest <;> by_cases he : k = j <;> simp_all

def graphOf (s : Fin n → Option (List (Fin n))) : Fin n → Fin n → Bool :=
  fun x y => ((s x).getD []).contains y

def dagQuery (n : Nat) : Query (Fin n) (List (Fin n)) Bool :=
  scan (List.finRange n) (fun _ => none) (fun s => checkAcyclic (graphOf s))

theorem dagQuery_exact (s : Fin n → Option (List (Fin n))) :
    (evaluate s (dagQuery n)).1 = true ↔ Acyclic (fun x y => graphOf s x y = true) := by
  have all : sampled s (List.finRange n) (fun _ => none) = s := by
    funext k
    simp [sampled_exact]
  simp only [dagQuery, scan_evaluates, all]
  exact checkAcyclic_exact (graphOf s)

theorem committed_graph_acyclic (s : Fin n → Cell (List (Fin n)))
    (ws patch : List (Fin n × Option (List (Fin n))))
    (out : Fin n → Cell (List (Fin n)))
    (accepted : commit s (writes s ws) patch (dagQuery n) = some out) :
    Acyclic (fun x y => graphOf (fun k => (out k).value) x y = true) :=
  (dagQuery_exact _).mp (commit_rechecks_result s ws patch (dagQuery n) out accepted)

namespace GraphCommitControls
def start : Fin 3 → Cell (List (Fin 3)) := fun _ => ⟨0,none⟩
#guard (commit start start [(0,some [1]),(1,some [2])] (dagQuery 3)).isSome
#guard (commit start start [(0,some [1,0])] (dagQuery 3)).isNone
#guard (commit start start [(0,some [1]),(1,some [0])] (dagQuery 3)).isNone
#guard (commit start start [(0,some [])] (dagQuery 3)).isSome
end GraphCommitControls
#print axioms scan_evaluates
#print axioms sampled_exact
#print axioms dagQuery_exact
#print axioms committed_graph_acyclic

-- The same property-specific consequence for a live preparation trace, without
-- an immutable preparation-start snapshot. Raw row reads remain coherent;
-- the patch is fixed, and publication at final validation remains logical atomic.
theorem interleaved_graph_publication
    {patch : List (Fin n × Option (List (Fin n)))}
    {s t : Fin n → Cell (List (Fin n))}
    {rs : List (Fin n × Cell (List (Fin n)))}
    (h : PatchPreparation patch s (dagQuery n) t true rs)
    (checked : checkObservedStamps t rs = true) :
    Acyclic (fun x y => graphOf (fun k => (publish t patch k).value) x y = true) := by
  apply (dagQuery_exact _).mp
  exact congrArg Prod.fst (patch_publication_replay h checked)
#print axioms interleaved_graph_publication

open TrackedValidation
namespace LivePositive
def start : Fin 3 → Cell (List (Fin 3)) := fun _ => ⟨0,none⟩
def patch : List (Fin 3 × Option (List (Fin 3))) := [(0,some [1])]
def finish := write start 2 (some [])
def reads : List (Fin 3 × Cell (List (Fin 3))) := [(0,start 0),(1,finish 1),(2,finish 2)]
example : PatchPreparation patch start (dagQuery 3) finish true reads := by
  apply PatchPreparation.read
  apply PatchPreparation.mutate (k := 2) (v := some [])
  apply PatchPreparation.read
  apply PatchPreparation.read
  exact PatchPreparation.done
#guard checkObservedStamps finish reads
#guard checkAcyclic (graphOf (fun k => (publish finish patch k).value))
#guard !checkObservedStamps (write finish 0 (some [0])) reads
end LivePositive

namespace IntermediateControl
def start := TrackedValidation.write GraphCommitControls.start 0 (some [1])
def patch : List (Fin 3 × Option (List (Fin 3))) := [(0,some []),(1,some [0])]
#guard checkAcyclic (graphOf (fun k => (start k).value))
#guard checkAcyclic (graphOf (fun k => (publish start patch k).value))
-- Reversed logical publication first adds1→0, creating a temporary cycle.
#guard !checkAcyclic (graphOf (fun k => (TrackedValidation.write start 1 (some [0]) k).value))
end IntermediateControl
end MirroreaProofFirst.GraphValidation
