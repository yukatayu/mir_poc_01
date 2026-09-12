import MirroreaProofFirstDynamicSupport

namespace MirroreaProofFirst.NamedCatalog
open Support Growth

-- Names bind within an instance namespace. The namespace is not an auth token.
structure Name where
  instanceKey : Nat
  localName : String
  deriving DecidableEq, Repr

def lookup (names : Fin n → Name) (name : Name) : Option (Fin n) :=
  (List.finRange n).find? fun k => decide (names k = name)

theorem lookup_sound (names : Fin n → Name) (h : lookup names name = some k) :
    names k = name := by
  unfold lookup at h
  have ok := List.find?_some h
  exact of_decide_eq_true ok

theorem lookup_none (names : Fin n → Name) :
    lookup names name = none ↔ ∀ k, names k ≠ name := by
  simp [lookup,List.find?_eq_none]

theorem lookup_exact (names : Fin n → Name) (unique : Function.Injective names) :
    lookup names name = some k ↔ names k = name := by
  constructor
  · exact lookup_sound names
  · intro h
    cases found : lookup names name with
    | none => exact False.elim ((lookup_none names).mp found k h)
    | some j =>
      have eq : j = k := unique ((lookup_sound names found).trans h.symm)
      cases eq; rfl

def uniqueNames (names : Fin n → Name) : Bool :=
  (List.finRange n).all fun k => (List.finRange n).all fun j =>
    decide (names k = names j → k = j)

theorem uniqueNames_exact (names : Fin n → Name) :
    uniqueNames names = true ↔ Function.Injective names := by
  simp only [uniqueNames,List.all_eq_true,List.mem_finRange,forall_const,decide_eq_true_eq]
  exact ⟨fun h => fun {_ _} => h _ _,fun h _ _ eq => h eq⟩

def compile (names : Fin n → Name) : Formula Name → Option (Formula (Fin n))
  | .top => some .top
  | .bottom => some .bottom
  | .ref name => (lookup names name).map .ref
  | .both p q => do return .both (← compile names p) (← compile names q)
  | .either p q => do return .either (← compile names p) (← compile names q)

-- Declarative meaning does not refer to lookup/compile or checking flags.
inductive Elaborates (names : Fin n → Name) : Formula Name → Formula (Fin n) → Prop where
  | top : Elaborates names .top .top
  | bottom : Elaborates names .bottom .bottom
  | reference : names k = name → Elaborates names (.ref name) (.ref k)
  | both : Elaborates names p cp → Elaborates names q cq → Elaborates names (.both p q) (.both cp cq)
  | either : Elaborates names p cp → Elaborates names q cq → Elaborates names (.either p q) (.either cp cq)

theorem compile_sound (names : Fin n → Name) (source : Formula Name)
    (h : compile names source = some core) : Elaborates names source core := by
  induction source generalizing core with
  | top => cases h; exact .top
  | bottom => cases h; exact .bottom
  | ref name =>
    simp only [compile] at h
    cases found : lookup names name with
    | none => simp [found] at h
    | some k =>
      simp only [found,Option.map_some,Option.some.injEq] at h
      cases h
      exact .reference (lookup_sound names found)
  | both p q hp hq =>
    cases ep : compile names p <;> cases eq : compile names q <;>
      simp [compile,ep,eq] at h
    case some.some cp cq =>
      cases h
      exact .both (hp ep) (hq eq)
  | either p q hp hq =>
    cases ep : compile names p <;> cases eq : compile names q <;>
      simp [compile,ep,eq] at h
    case some.some cp cq =>
      cases h
      exact .either (hp ep) (hq eq)

theorem compile_complete (names : Fin n → Name) (unique : Function.Injective names)
    (h : Elaborates names source core) : compile names source = some core := by
  induction h with
  | top => rfl
  | bottom => rfl
  | reference he => simp [compile,(lookup_exact names unique).mpr he]
  | both _ _ hp hq => simp [compile,hp,hq]
  | either _ _ hp hq => simp [compile,hp,hq]

theorem Elaborates.retains (h : Elaborates names source core) : mapFormula names core = source := by
  induction h with
  | top => rfl
  | bottom => rfl
  | reference he => simp [mapFormula,he]
  | both _ _ hp hq => simp [mapFormula,hp,hq]
  | either _ _ hp hq => simp [mapFormula,hp,hq]

structure Catalog (n : Nat) where
  names : Fin n → Name
  dependencies : Fin n → Formula Name
  eligible : Fin n → Bool

def check (c : Catalog n) : Bool :=
  uniqueNames c.names && (List.finRange n).all fun k =>
    (compile c.names (c.dependencies k)).isSome

def snapshot (c : Catalog n) : Snapshot n :=
  ⟨fun k => (compile c.names (c.dependencies k)).getD .bottom,c.eligible⟩

def namedForms (c : Catalog n) (name : Name) : Formula Name :=
  match lookup c.names name with | some k => c.dependencies k | none => .bottom

def namedEligible (c : Catalog n) (name : Name) : Bool :=
  match lookup c.names name with | some k => c.eligible k | none => false

theorem checked_unique (c : Catalog n) (h : check c = true) : Function.Injective c.names := by
  simp only [check,Bool.and_eq_true] at h
  exact (uniqueNames_exact c.names).mp h.1

theorem checked_elaboration (c : Catalog n) (h : check c = true) (k : Fin n) :
    Elaborates c.names (c.dependencies k) ((snapshot c).forms k) := by
  simp only [check,Bool.and_eq_true] at h
  have complete := h.2
  have present := (List.all_eq_true.mp complete) k (by simp)
  cases hc : compile c.names (c.dependencies k) with
  | none => simp [hc] at present
  | some core =>
    simpa [snapshot,hc] using compile_sound c.names (c.dependencies k) hc

def Resolved (c : Catalog n) : Prop :=
  Function.Injective c.names ∧ ∀ k, ∃ core, Elaborates c.names (c.dependencies k) core

theorem check_exact (c : Catalog n) : check c = true ↔ Resolved c := by
  constructor
  · intro h
    exact ⟨checked_unique c h,fun k => ⟨_,checked_elaboration c h k⟩⟩
  · rintro ⟨unique,complete⟩
    simp only [check,Bool.and_eq_true]
    refine ⟨(uniqueNames_exact c.names).mpr unique,List.all_eq_true.mpr ?_⟩
    intro k _
    obtain ⟨core,hc⟩ := complete k
    simp [compile_complete c.names unique hc]

theorem Elaborates.transport {names : Fin n → Name} (h : Elaborates names source core)
    (embed : Fin n → Fin m) (newNames : Fin m → Name)
    (preserve : ∀ k, newNames (embed k) = names k) :
    Elaborates newNames source (mapFormula embed core) := by
  induction h with
  | top => exact .top
  | bottom => exact .bottom
  | reference he => exact .reference ((preserve _).trans he)
  | both _ _ hp hq => exact .both hp hq
  | either _ _ hp hq => exact .either hp hq

theorem compile_embedding (names : Fin n → Name) (newNames : Fin m → Name)
    (embed : Fin n → Fin m) (preserve : ∀ k, newNames (embed k) = names k)
    (unique : Function.Injective newNames) (h : compile names source = some core) :
    compile newNames source = some (mapFormula embed core) :=
  compile_complete newNames unique ((compile_sound names source h).transport embed newNames preserve)

theorem checked_named_grounded (c : Catalog n) (h : check c = true) (k : Fin n) :
    snapshotLive (snapshot c) k = true ↔
      Grounded (namedForms c) (fun name => namedEligible c name = true) (c.names k) := by
  rw [snapshot_live_exact]
  apply Iff.symm
  apply grounded_embedding c.names (snapshot c).forms (namedForms c)
    (snapshot c).eligible (namedEligible c)
  · intro j
    have found := (lookup_exact c.names (checked_unique c h)).mpr (rfl : c.names j = c.names j)
    simp only [namedForms,found]
    exact (checked_elaboration c h j).retains.symm
  · intro j
    have found := (lookup_exact c.names (checked_unique c h)).mpr (rfl : c.names j = c.names j)
    simp [namedEligible,found,snapshot]

theorem missing_eligibility_false (c : Catalog n) (absent : ∀ k, c.names k ≠ name) :
    namedEligible c name = false := by
  simp [namedEligible,(lookup_none c.names).mpr absent]

theorem missing_unusable (c : Catalog n) (absent : ∀ k, c.names k ≠ name) :
    ¬ Grounded (namedForms c) (fun name => namedEligible c name = true) name := by
  have none := (lookup_none c.names).mpr absent
  intro h
  simpa [namedEligible,none] using h.1

-- Concrete catalog insertion: complete inventory grows, old declared sources
-- are preserved as names, and the checker rejects aliasing or missing refs.
def extend (old : Fin n → A) (fresh : Fin m → A) (k : Fin (n+m)) : A :=
  if h : k.val < n then old ⟨k.val,h⟩ else fresh ⟨k.val-n,by omega⟩

@[simp] theorem extend_old (old : Fin n → A) (fresh : Fin m → A) (k : Fin n) :
    extend old fresh (left n m k) = old k := by simp [extend,left,k.isLt]

def grow (c : Catalog n) (names : Fin m → Name) (forms : Fin m → Formula Name)
    (eligible : Fin m → Bool) : Catalog (n+m) :=
  ⟨extend c.names names,extend c.dependencies forms,extend c.eligible eligible⟩

theorem grow_compiled_old (c : Catalog n) (names : Fin m → Name)
    (forms : Fin m → Formula Name) (eligible : Fin m → Bool)
    (oldChecked : check c = true) (newChecked : check (grow c names forms eligible) = true)
    (k : Fin n) :
    (snapshot (grow c names forms eligible)).forms (left n m k) =
      mapFormula (left n m) ((snapshot c).forms k) := by
  have transported := (checked_elaboration c oldChecked k).transport
    (left n m) (grow c names forms eligible).names
    (fun j => extend_old c.names names j)
  have hc := compile_complete _ (checked_unique _ newChecked) transported
  simpa [snapshot,grow] using congrArg (fun x => x.getD .bottom) hc

theorem grow_live_old (c : Catalog n) (names : Fin m → Name)
    (forms : Fin m → Formula Name) (eligible : Fin m → Bool)
    (oldChecked : check c = true) (newChecked : check (grow c names forms eligible) = true)
    (k : Fin n) :
    snapshotLive (snapshot (grow c names forms eligible)) (left n m k) = snapshotLive (snapshot c) k := by
  apply Bool.eq_iff_iff.mpr
  rw [snapshot_live_exact,snapshot_live_exact]
  apply grounded_embedding (left n m) (snapshot c).forms (snapshot (grow c names forms eligible)).forms
    (snapshot c).eligible (snapshot (grow c names forms eligible)).eligible
  · exact grow_compiled_old c names forms eligible oldChecked newChecked
  · intro j; exact extend_old c.eligible eligible j

namespace Controls
def a : Name := ⟨10,"base"⟩
def b : Name := ⟨20,"extension"⟩
def unknown : Name := ⟨30,"missing"⟩
def base : Catalog 1 := ⟨fun _ => a,fun _ => .top,fun _ => true⟩
def added := grow base (m:=1) (fun _ => b) (fun _ => .ref a) (fun _ => true)
#guard check base
#guard check added
#guard snapshotLive (snapshot added) 0 && snapshotLive (snapshot added) 1
#guard !check (grow base (m:=1) (fun _ => a) (fun _ => .top) (fun _ => true))
#guard !check (grow base (m:=1) (fun _ => b) (fun _ => .ref unknown) (fun _ => true))
-- Equal local spelling in DIFFERENT instance namespaces is legal.
#guard check (grow base (m:=1) (fun _ => ⟨20,"base"⟩) (fun _ => .ref a) (fun _ => true))
#guard namedEligible base unknown = false
-- Omitting oldChecked would admit a formerly unresolved reference after growth.
def unresolved : Catalog 1 := ⟨fun _ => a,fun _ => .ref b,fun _ => true⟩
def newlyResolved := grow unresolved (m:=1) (fun _ => b) (fun _ => .top) (fun _ => true)
#guard !check unresolved
#guard check newlyResolved
#guard !snapshotLive (snapshot unresolved) 0
#guard snapshotLive (snapshot newlyResolved) 0
-- A support certificate alone does not certify unique catalog names.
def aliasing : Catalog 2 := ⟨fun _ => a,fun k => if k = 0 then .bottom else .top,fun _ => true⟩
#guard !check aliasing
#guard snapshotLive (snapshot aliasing) 1
#guard !snapshotLive (snapshot aliasing) 0
end Controls

#print axioms lookup_exact
#print axioms uniqueNames_exact
#print axioms compile_sound
#print axioms compile_complete
#print axioms Elaborates.retains
#print axioms checked_elaboration
#print axioms check_exact
#print axioms compile_embedding
#print axioms grow_compiled_old
#print axioms grow_live_old
#print axioms checked_named_grounded
#print axioms missing_eligibility_false
#print axioms missing_unusable
end MirroreaProofFirst.NamedCatalog
