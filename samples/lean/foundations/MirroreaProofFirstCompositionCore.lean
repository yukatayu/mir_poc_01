import MirroreaProofFirstWorldProjection

namespace MirroreaProofFirst.CompositionCore
open InstancePrograms InstanceState Support

deriving instance DecidableEq for Formula

-- Private checked Core of the selected reference profile. This module is a
-- structural interpreter; the authorized source entry wraps it, never exports
-- its raw mutators as an authority-bearing control API.
inductive Command (d p n : Nat) where
  | register (definition : Definition) (predecessor : Option (Fin d))
  | instantiate (definition : Fin d) (owner : Nat) (placements : List (Fin p))
      (parent : Option (Fin n)) (dependencies : Formula (Fin n))
  | retire (key : Fin n)
  | reparent (key : Fin n) (parent : Option (Fin n))
  | replace (key : Fin n) (definition : Fin d)
  | leave (place : Fin p)
  | join (place : Fin p)
  deriving DecidableEq, Repr

structure Config (p : Nat) where
  definitions : Nat
  count : Nat
  state : State definitions p count

def config (s : State d p n) : Config p := ⟨d,n,s⟩
def Invariant (s : Config p) : Prop := Valid s.state

-- Independent admissibility specification. It mentions contract meanings,
-- nonempty placements and the resulting relation DAG, not the checker.
def Allowed (s : State d p n) : Command d p n → Prop
  | .register definition predecessor => Registration s definition predecessor
  | .instantiate _ _ placements _ _ => placements ≠ []
  | .retire key => (s.instances key).enabled = true
  | .reparent key parent => (s.instances key).enabled = true ∧
      GraphValidation.Acyclic (ParentEdge (setParent s key parent))
  | .replace key definition => (s.instances key).enabled = true ∧
      Refines (s.definitions (s.instances key).definition).contract (s.definitions definition).contract ∧
      Satisfies (s.definitions definition)
  | .leave place => s.participating place = true
  | .join place => s.participating place = false

def check (s : State d p n) : Command d p n → Bool
  | .register definition predecessor => registrationCheck s definition predecessor
  | .instantiate _ _ placements _ _ => !placements.isEmpty
  | .retire key => (s.instances key).enabled
  | .reparent key parent => (s.instances key).enabled && reparentCheck s key parent
  | .replace key definition => (s.instances key).enabled && replacementCheck s key definition
  | .leave place => s.participating place
  | .join place => !s.participating place

theorem check_exact (s : State d p n) (c : Command d p n) (valid : Valid s) :
    check s c = true ↔ Allowed s c := by
  cases c with
  | register definition predecessor => exact registration_exact _ _ _
  | instantiate => simp [check,Allowed]
  | retire => rfl
  | reparent key parent => simp only [check,Allowed,Bool.and_eq_true,reparent_exact _ _ _ valid.parents]
  | replace => simp [check,Allowed,replacementCheck,exchange_exact]
  | leave => rfl
  | join => simp [check,Allowed]

def outcome (s : State d p n) : Command d p n → Config p × Option Nat
  | .register definition predecessor => (config (register s definition predecessor),some d)
  | .instantiate definition owner placements parent dependencies =>
      (config (instantiate s definition owner placements parent dependencies),some n)
  | .retire key => (config (retire s key),none)
  | .reparent key parent => (config (setParent s key parent),none)
  | .replace key definition => (config (replace s key definition),none)
  | .leave place => (config (leave s place),none)
  | .join place => (config (join s place),none)

def apply (s : State d p n) (c : Command d p n) : Option (Config p × Option Nat) :=
  if check s c then some (outcome s c) else none

-- These are semantic steps, not a definition that equates admission with a
-- checker verdict. All selected structural mutations occur in outcome.
def Steps (s : State d p n) (c : Command d p n) (next : Config p × Option Nat) : Prop :=
  Allowed s c ∧ next = outcome s c

theorem apply_exact (s : State d p n) (c : Command d p n) (next : Config p × Option Nat)
    (valid : Valid s) : apply s c = some next ↔ Steps s c next := by
  simp only [apply,Steps]
  split
  · rename_i h
    simp [((check_exact s c valid).mp h),eq_comm]
  · rename_i h
    simp [show ¬ Allowed s c from fun a => h ((check_exact s c valid).mpr a)]

theorem outcome_valid (s : State d p n) (c : Command d p n) (valid : Valid s)
    (allowed : Allowed s c) : Invariant (outcome s c).1 := by
  cases c with
  | register definition predecessor =>
      exact register_valid s definition predecessor valid ((registration_exact _ _ _).mpr allowed)
  | instantiate definition owner placements parent dependencies =>
      exact instantiate_valid s definition owner placements parent dependencies valid allowed
  | retire key => exact retire_valid s key valid
  | reparent key parent =>
      exact reparent_valid s key parent valid ((reparent_exact _ _ _ valid.parents).mpr allowed.2)
  | replace key definition =>
      exact replacement_valid s key definition valid ((exchange_exact _ _).mpr allowed.2)
  | leave place => exact leave_valid s place valid
  | join place => exact join_valid s place valid

theorem apply_preserves (s : State d p n) (c : Command d p n) (next : Config p × Option Nat)
    (valid : Valid s) (accepted : apply s c = some next) : Invariant next.1 := by
  obtain ⟨allowed,rfl⟩ := (apply_exact s c next valid).mp accepted
  exact outcome_valid s c valid allowed

-- Raw Core coordinates come from name elaboration, not from wrapping modulo
-- Fin.ofNat. Every missing definition/instance/locus is rejected independently.
inductive Raw where
  | register (definition : Definition) (predecessor : Option Nat)
  | instantiate (definition owner : Nat) (placements : List Nat)
      (parent : Option Nat) (dependencies : Formula Nat)
  | retire (key : Nat)
  | reparent (key : Nat) (parent : Option Nat)
  | replace (key definition : Nat)
  | leave (place : Nat)
  | join (place : Nat)
  deriving DecidableEq, Repr

def index (n key : Nat) : Option (Fin n) := if h : key < n then some ⟨key,h⟩ else none
def optionalIndex (n : Nat) : Option Nat → Option (Option (Fin n))
  | none => some none
  | some key => (index n key).map some
def formula (n : Nat) : Formula Nat → Option (Formula (Fin n))
  | .top => some .top
  | .bottom => some .bottom
  | .ref key => (index n key).map Formula.ref
  | .both a b => do return .both (← formula n a) (← formula n b)
  | .either a b => do return .either (← formula n a) (← formula n b)

def elaborate (d p n : Nat) : Raw → Option (Command d p n)
  | .register definition predecessor => do return .register definition (← optionalIndex d predecessor)
  | .instantiate definition owner placements parent dependencies => do
      return .instantiate (← index d definition) owner (← placements.mapM (index p))
        (← optionalIndex n parent) (← formula n dependencies)
  | .retire key => (index n key).map Command.retire
  | .reparent key parent => do return .reparent (← index n key) (← optionalIndex n parent)
  | .replace key definition => do return .replace (← index n key) (← index d definition)
  | .leave place => (index p place).map Command.leave
  | .join place => (index p place).map Command.join

def erase : Command d p n → Raw
  | .register definition predecessor => .register definition (predecessor.map Fin.val)
  | .instantiate definition owner placements parent dependencies =>
      .instantiate definition.val owner (placements.map Fin.val) (parent.map Fin.val)
        (Growth.mapFormula Fin.val dependencies)
  | .retire key => .retire key.val
  | .reparent key parent => .reparent key.val (parent.map Fin.val)
  | .replace key definition => .replace key.val definition.val
  | .leave place => .leave place.val
  | .join place => .join place.val

theorem index_roundtrip (key : Fin n) : index n key.val = some key := by simp [index,key.isLt]
theorem optional_roundtrip (key : Option (Fin n)) :
    optionalIndex n (key.map Fin.val) = some key := by cases key <;> simp [optionalIndex,index_roundtrip]
theorem list_roundtrip (keys : List (Fin n)) :
    (keys.map Fin.val).mapM (index n) = some keys := by
  induction keys with
  | nil => rfl
  | cons key keys ih => simp only [List.map_cons,List.mapM_cons,index_roundtrip,ih]; rfl
theorem formula_roundtrip (f : Formula (Fin n)) :
    formula n (Growth.mapFormula Fin.val f) = some f := by
  induction f <;> simp_all [formula,Growth.mapFormula,index_roundtrip]

theorem elaborate_roundtrip (c : Command d p n) : elaborate d p n (erase c) = some c := by
  cases c <;> simp only [elaborate,erase,index_roundtrip,optional_roundtrip,list_roundtrip,formula_roundtrip] <;> rfl

theorem index_sound (key : Nat) (checked : index n key = some k) : k.val = key := by
  unfold index at checked
  split at checked
  · cases checked; rfl
  · cases checked

theorem optional_sound (raw : Option Nat) (checked : optionalIndex n raw = some key) :
    key.map Fin.val = raw := by
  cases raw with
  | none => cases checked; rfl
  | some raw =>
      simp only [optionalIndex] at checked
      cases h : index n raw with
      | none => simp [h] at checked
      | some k =>
          simp only [h,Option.map_some,Option.some.injEq] at checked
          subst key
          simp [index_sound raw h]

theorem list_sound (raw : List Nat) (checked : raw.mapM (index n) = some keys) :
    keys.map Fin.val = raw := by
  induction raw generalizing keys with
  | nil => cases checked; rfl
  | cons raw raws ih =>
      simp only [List.mapM_cons] at checked
      cases h : index n raw <;> cases hs : raws.mapM (index n) <;> simp [h,hs] at checked
      subst keys
      simp [index_sound raw h,ih hs]

theorem formula_sound (raw : Formula Nat) (checked : formula n raw = some f) :
    Growth.mapFormula Fin.val f = raw := by
  induction raw generalizing f with
  | top => cases checked; rfl
  | bottom => cases checked; rfl
  | ref raw =>
      simp only [formula] at checked
      cases h : index n raw <;> simp [h] at checked
      subst f
      simp [Growth.mapFormula,index_sound raw h]
  | both a b ia ib =>
      simp only [formula] at checked
      cases ha : formula n a <;> cases hb : formula n b <;> simp [ha,hb] at checked
      subst f
      simp [Growth.mapFormula,ia ha,ib hb]
  | either a b ia ib =>
      simp only [formula] at checked
      cases ha : formula n a <;> cases hb : formula n b <;> simp [ha,hb] at checked
      subst f
      simp [Growth.mapFormula,ia ha,ib hb]

theorem elaborate_sound (raw : Raw) (checked : elaborate d p n raw = some c) : erase c = raw := by
  cases raw with
  | register definition predecessor =>
      simp only [elaborate] at checked
      cases h : optionalIndex d predecessor <;> simp [h] at checked
      subst c
      simp [erase,optional_sound predecessor h]
  | instantiate definition owner placements parent dependencies =>
      simp only [elaborate] at checked
      cases hd : index d definition <;> cases hp : placements.mapM (index p) <;>
        cases hr : optionalIndex n parent <;> cases hf : formula n dependencies <;>
        simp [hd,hp,hr,hf] at checked
      subst c
      simp [erase,index_sound definition hd,list_sound placements hp,optional_sound parent hr,formula_sound dependencies hf]
  | retire key =>
      simp only [elaborate] at checked
      cases h : index n key <;> simp [h] at checked
      subst c
      simp [erase,index_sound key h]
  | reparent key parent =>
      simp only [elaborate] at checked
      cases h : index n key <;> cases hp : optionalIndex n parent <;> simp [h,hp] at checked
      subst c
      simp [erase,index_sound key h,optional_sound parent hp]
  | replace key definition =>
      simp only [elaborate] at checked
      cases h : index n key <;> cases hd : index d definition <;> simp [h,hd] at checked
      subst c
      simp [erase,index_sound key h,index_sound definition hd]
  | leave place =>
      simp only [elaborate] at checked
      cases h : index p place <;> simp [h] at checked
      subst c
      simp [erase,index_sound place h]
  | join place =>
      simp only [elaborate] at checked
      cases h : index p place <;> simp [h] at checked
      subst c
      simp [erase,index_sound place h]

-- The declarative erasure relation fully characterizes successful elaboration;
-- missing or wrong-sort coordinates cannot be silently replaced or wrapped.
theorem elaborate_exact (raw : Raw) (c : Command d p n) :
    elaborate d p n raw = some c ↔ erase c = raw := by
  exact ⟨elaborate_sound raw,fun eq => eq ▸ elaborate_roundtrip c⟩

def run (s : Config p) (raw : Raw) : Option (Config p × Option Nat) := do
  let c ← elaborate s.definitions p s.count raw
  apply s.state c

theorem run_preserves (s : Config p) (raw : Raw) (next : Config p × Option Nat)
    (valid : Invariant s) (accepted : run s raw = some next) : Invariant next.1 := by
  unfold run at accepted
  cases hc : elaborate s.definitions p s.count raw with
  | none => simp [hc] at accepted
  | some c => exact apply_preserves s.state c next valid (by simpa [hc] using accepted)

theorem run_realm (s : Config p) (raw : Raw) (next : Config p × Option Nat)
    (accepted : run s raw = some next) : next.1.state.realm = s.state.realm := by
  unfold run at accepted
  cases hc : elaborate s.definitions p s.count raw with
  | none => simp [hc] at accepted
  | some c =>
      simp only [hc] at accepted
      change apply s.state c = some next at accepted
      unfold apply at accepted
      split at accepted
      · cases accepted
        cases c <;> rfl
      · cases accepted

namespace Controls
open InstanceState.Controls
#guard (run (config empty) (.instantiate 0 7 [0,1,2] none .top)).isSome
#guard (run (config base) (.instantiate 0 7 [0,2] none .top)).isSome
#guard (run (config base) (.instantiate 0 7 [] none .top)).isNone
#guard (run (config base) (.instantiate 0 7 [0,3] none .top)).isNone
#guard (run (config base) (.instantiate 7 7 [0] none .top)).isNone
#guard (run (config base) (.instantiate 0 7 [0] none (.ref 1))).isNone
#guard (run (config two) (.reparent 0 (some 1))).isSome
#guard (run (config reparented) (.reparent 0 (some 1))).isNone
#guard (run (config retiredExtra) (.replace 1 1)).isNone
#guard (run (config two) (.join 2)).isNone
#guard (run (config (leave two 2)) (.join 2)).isSome
end Controls

#print axioms check_exact
#print axioms apply_exact
#print axioms apply_preserves
#print axioms elaborate_roundtrip
#print axioms elaborate_exact
#print axioms run_preserves
#print axioms run_realm
end MirroreaProofFirst.CompositionCore
