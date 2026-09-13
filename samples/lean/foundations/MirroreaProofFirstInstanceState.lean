import MirroreaProofFirstInstancePrograms
import MirroreaProofFirstDynamicIdentity
import MirroreaProofFirstDynamicGraphs

namespace MirroreaProofFirst.InstanceState
open InstancePrograms Support Growth IdentityGrowth GraphValidation

def last (n : Nat) : Fin (n+1) := ⟨n,by omega⟩

theorem old_or_last (k : Fin (n+1)) :
    (∃ old : Fin n, k = left n 1 old) ∨ k = last n := by
  by_cases h : k.val < n
  · exact Or.inl ⟨⟨k.val,h⟩, rfl⟩
  · right
    apply Fin.ext
    have := k.isLt
    simp only [last]
    omega

@[simp] theorem extend_last (old : Fin n → A) (value : A) :
    extend old (fun _ : Fin 1 => value) (last n) = value := by
  simp [extend,last]

def classify (k : Fin (n+1)) : Sum (Fin n) Unit :=
  if h : k.val < n then .inl ⟨k.val,h⟩ else .inr ()

@[simp] theorem classify_old (k : Fin n) : classify (left n 1 k) = .inl k := by
  simp [classify,left,k.isLt]
@[simp] theorem classify_last : classify (last n) = .inr () := by
  simp [classify,last]

theorem path_map (edge : A → A → Prop) (target : B → B → Prop) (f : A → B)
    (maps : ∀ a b, edge a b → target (f a) (f b)) (h : Path edge x y) :
    Path target (f x) (f y) := by
  induction h with
  | refl => exact .refl
  | step _ e ih => exact .step ih (maps _ _ e)

theorem acyclic_map (edge : A → A → Prop) (target : B → B → Prop) (f : A → B)
    (maps : ∀ a b, edge a b → target (f a) (f b)) (acyclic : Acyclic target) :
    Acyclic edge := by
  intro a b he back
  exact acyclic (f a) (f b) (maps a b he) (path_map edge target f maps back)

-- Each instance has its own live bit, parent, support declaration and revision.
-- Definition identity is an immutable catalog slot, never the instance key.
structure Instance (d p n : Nat) where
  definition : Fin d
  interface : Contract
  owner : Nat
  revision : Nat
  enabled : Bool
  placements : List (Fin p)
  parent : Option (Fin n)
  dependencies : Formula (Fin n)

structure State (d p n : Nat) where
  realm : Nat
  placeIncarnation : Fin p → Nat
  definitions : Fin d → Definition
  predecessors : Fin d → Option (Fin d)
  instances : Fin n → Instance d p n
  participating : Fin p → Bool

def ParentEdge (s : State d p n) (a b : Fin n) : Prop :=
  (s.instances a).parent = some b

structure Valid (s : State d p n) : Prop where
  definitions : ∀ k, Satisfies (s.definitions k)
  versions : Acyclic (fun a b => s.predecessors a = some b)
  interfaces : ∀ k, Refines (s.instances k).interface
    (s.definitions (s.instances k).definition).contract
  placed : ∀ k, (s.instances k).placements ≠ []
  parents : Acyclic (ParentEdge s)

def mapInstance (i : Instance d p n) : Instance d p (n+1) :=
  {definition := i.definition, interface := i.interface, owner := i.owner,
   revision := i.revision, enabled := i.enabled, placements := i.placements,
   parent := i.parent.map (left n 1), dependencies := mapFormula (left n 1) i.dependencies}

def instantiate (s : State d p n) (definition : Fin d) (owner : Nat)
    (placements : List (Fin p)) (parent : Option (Fin n))
    (dependencies : Formula (Fin n)) : State d p (n+1) :=
  {realm := s.realm, placeIncarnation := s.placeIncarnation, definitions := s.definitions, predecessors := s.predecessors, participating := s.participating,
   instances := extend (fun k => mapInstance (s.instances k)) (fun _ : Fin 1 =>
     {definition := definition, interface := (s.definitions definition).contract,
      owner := owner, revision := 0, enabled := true, placements := placements,
      parent := parent.map (left n 1), dependencies := mapFormula (left n 1) dependencies})}

@[simp] theorem instantiate_old (s : State d p n) (definition : Fin d) (owner : Nat)
    (placements : List (Fin p)) (parent : Option (Fin n))
    (dependencies : Formula (Fin n)) (k : Fin n) :
    (instantiate s definition owner placements parent dependencies).instances (left n 1 k) =
      mapInstance (s.instances k) := by
  simp [instantiate,extend_old]

@[simp] theorem instantiate_new (s : State d p n) (definition : Fin d) (owner : Nat)
    (placements : List (Fin p)) (parent : Option (Fin n))
    (dependencies : Formula (Fin n)) :
    (instantiate s definition owner placements parent dependencies).instances (last n) =
      {definition := definition, interface := (s.definitions definition).contract,
       owner := owner, revision := 0, enabled := true, placements := placements,
       parent := parent.map (left n 1), dependencies := mapFormula (left n 1) dependencies} :=
  by simp [instantiate,extend_last]

theorem mapped_parent (parent : Option (Fin n)) (target : Fin (n+1))
    (h : parent.map (left n 1) = some target) :
    ∃ old : Fin n, parent = some old ∧ target = left n 1 old := by
  cases parent with
  | none => cases h
  | some old => cases h; exact ⟨old,rfl,rfl⟩

theorem append_parent_acyclic (old : Fin n → Option (Fin n)) (parent : Option (Fin n))
    (valid : Acyclic (fun a b => old a = some b)) :
    Acyclic (fun a b => extend (fun k => (old k).map (left n 1))
      (fun _ : Fin 1 => parent.map (left n 1)) a = some b) := by
  let target := GraphGrowth.downstream (fun a b => old a = some b)
    (fun _ _ : Unit => False) (fun _ : Unit => fun old => parent = some old)
  have acyclic : Acyclic target := (GraphGrowth.downstream_acyclic_exact).mpr
    ⟨valid,fun _ _ h => False.elim h⟩
  apply acyclic_map _ target classify ?_ acyclic
  intro a b edge
  rcases old_or_last a with ⟨previous,rfl⟩ | rfl
  · simp only [extend_old] at edge
    obtain ⟨previous,hp,rfl⟩ := mapped_parent _ _ edge
    simpa [target,GraphGrowth.downstream] using hp
  · simp only [extend_last] at edge
    obtain ⟨previous,hp,rfl⟩ := mapped_parent _ _ edge
    simpa [target,GraphGrowth.downstream] using hp

theorem instantiate_parents (s : State d p n) (definition : Fin d) (owner : Nat)
    (placements : List (Fin p)) (parent : Option (Fin n))
    (dependencies : Formula (Fin n)) (valid : Acyclic (ParentEdge s)) :
    Acyclic (ParentEdge (instantiate s definition owner placements parent dependencies)) := by
  have mapped : ParentEdge (instantiate s definition owner placements parent dependencies) =
      (fun a b => extend (fun k => ((s.instances k).parent).map (left n 1))
        (fun _ : Fin 1 => parent.map (left n 1)) a = some b) := by
    funext a b
    rcases old_or_last a with ⟨previous,rfl⟩ | rfl <;>
      simp only [ParentEdge,instantiate_old,instantiate_new,mapInstance,extend_old,extend_last]
  rw [mapped]
  exact append_parent_acyclic _ _ valid

theorem refines_self (c : Contract) : Refines c c :=
  ⟨fun _ h => h,Int.le_refl _,Int.le_refl _⟩

theorem instantiate_valid (s : State d p n) (definition : Fin d) (owner : Nat)
    (placements : List (Fin p)) (parent : Option (Fin n))
    (dependencies : Formula (Fin n)) (valid : Valid s) (nonempty : placements ≠ []) :
    Valid (instantiate s definition owner placements parent dependencies) := by
  refine ⟨valid.definitions,valid.versions,?_,?_,instantiate_parents s definition owner placements parent dependencies valid.parents⟩
  · intro k
    rcases old_or_last k with ⟨old,rfl⟩ | rfl
    · simpa only [instantiate,mapInstance,extend_old] using valid.interfaces old
    · simpa only [instantiate,extend_last] using refines_self (s.definitions definition).contract
  · intro k
    rcases old_or_last k with ⟨old,rfl⟩ | rfl
    · simpa only [instantiate_old,mapInstance] using valid.placed old
    · simpa only [instantiate_new] using nonempty

def modify (s : State d p n) (key : Fin n) (f : Instance d p n → Instance d p n) : State d p n :=
  {s with instances := fun k => if k = key then f (s.instances k) else s.instances k}

@[simp] theorem modify_same (s : State d p n) (key : Fin n) (f : Instance d p n → Instance d p n) :
    (modify s key f).instances key = f (s.instances key) := by simp [modify]
@[simp] theorem modify_other (s : State d p n) (key k : Fin n) (f : Instance d p n → Instance d p n)
    (different : k ≠ key) : (modify s key f).instances k = s.instances k := by simp [modify,different]

def retire (s : State d p n) (key : Fin n) : State d p n :=
  modify s key fun i => {i with enabled := false, revision := i.revision+1}

theorem retire_valid (s : State d p n) (key : Fin n) (valid : Valid s) :
    Valid (retire s key) := by
  refine ⟨valid.definitions,valid.versions,?_,?_,?_⟩
  · intro k
    by_cases h : k = key <;> simpa [retire,modify,h] using valid.interfaces k
  · intro k
    by_cases h : k = key <;> simpa [retire,modify,h] using valid.placed k
  · have same : ParentEdge (retire s key) = ParentEdge s := by
      funext a b
      simp only [ParentEdge,retire,modify]
      split <;> rfl
    rw [same]
    exact valid.parents

theorem retirement_retains_record (s : State d p n) (key : Fin n) :
    (retire s key).instances key = {(s.instances key) with
      enabled := false, revision := (s.instances key).revision+1} := by simp [retire]

theorem retirement_is_instance_local (s : State d p n) (key other : Fin n)
    (distinct : other ≠ key) : (retire s key).instances other = s.instances other :=
  modify_other _ _ _ _ distinct

def mapDefinition (i : Instance d p n) : Instance (d+1) p n :=
  {definition := left d 1 i.definition, interface := i.interface, owner := i.owner,
   revision := i.revision, enabled := i.enabled, placements := i.placements,
   parent := i.parent, dependencies := i.dependencies}

def register (s : State d p n) (new : Definition) (predecessor : Option (Fin d)) : State (d+1) p n :=
  {realm := s.realm, placeIncarnation := s.placeIncarnation, definitions := extend s.definitions (fun _ : Fin 1 => new),
   predecessors := extend (fun k => (s.predecessors k).map (left d 1))
     (fun _ : Fin 1 => predecessor.map (left d 1)),
   instances := fun k => mapDefinition (s.instances k), participating := s.participating}

def Registration (s : State d p n) (new : Definition) (predecessor : Option (Fin d)) : Prop :=
  Satisfies new ∧ ∀ old, predecessor = some old → Refines (s.definitions old).contract new.contract

def registrationCheck (s : State d p n) (new : Definition) (predecessor : Option (Fin d)) : Bool :=
  InstancePrograms.check new && match predecessor with
    | none => true
    | some old => refinementCheck (s.definitions old).contract new.contract

theorem registration_exact (s : State d p n) (new : Definition) (predecessor : Option (Fin d)) :
    registrationCheck s new predecessor = true ↔ Registration s new predecessor := by
  cases predecessor <;> simp [registrationCheck,Registration,InstancePrograms.check_exact,refinement_exact]

theorem register_valid (s : State d p n) (new : Definition) (predecessor : Option (Fin d))
    (valid : Valid s) (accepted : registrationCheck s new predecessor = true) :
    Valid (register s new predecessor) := by
  have checked := ((registration_exact _ _ _).mp accepted).1
  refine ⟨?_,append_parent_acyclic _ _ valid.versions,?_,?_,?_⟩
  · intro k
    rcases old_or_last k with ⟨old,rfl⟩ | rfl
    · simpa [register,extend_old] using valid.definitions old
    · simpa [register,extend_last] using checked
  · intro k
    simpa [register,mapDefinition,extend_old] using valid.interfaces k
  · exact valid.placed
  · exact valid.parents

theorem register_old_definition (s : State d p n) (new : Definition) (predecessor : Option (Fin d))
    (key : Fin d) : (register s new predecessor).definitions (left d 1 key) = s.definitions key :=
  by simp [register,extend_old]

theorem register_instance_locator (s : State d p n) (new : Definition) (predecessor : Option (Fin d))
    (key : Fin n) :
    ((register s new predecessor).instances key).definition.val = (s.instances key).definition.val ∧
    ((register s new predecessor).instances key).revision = (s.instances key).revision := ⟨rfl,rfl⟩

def setParent (s : State d p n) (key : Fin n) (parent : Option (Fin n)) : State d p n :=
  modify s key fun i => {i with parent := parent, revision := i.revision+1}

def pruned (s : State d p n) (key a b : Fin n) : Bool :=
  decide (a ≠ key ∧ (s.instances a).parent = some b)

def reparentCheck (s : State d p n) (key : Fin n) (parent : Option (Fin n)) : Bool :=
  match parent with
  | none => true
  | some target => checkAdd (pruned s key) key target

theorem pruned_valid (s : State d p n) (key : Fin n) (valid : Acyclic (ParentEdge s)) :
    Acyclic (fun a b => pruned s key a b = true) := by
  apply acyclic_subgraph (other := ParentEdge s) ?_ valid
  intro a b h
  have checked : a ≠ key ∧ ParentEdge s a b := by simpa [pruned,ParentEdge] using h
  exact checked.2

theorem parent_none_edge (s : State d p n) (key : Fin n) :
    ParentEdge (setParent s key none) = (fun a b => pruned s key a b = true) := by
  funext a b
  apply propext
  by_cases h : a = key <;> simp [ParentEdge,setParent,modify,pruned,h]

theorem parent_some_edge (s : State d p n) (key target : Fin n) :
    ParentEdge (setParent s key (some target)) =
      add (fun a b => pruned s key a b = true) key target := by
  funext a b
  apply propext
  by_cases h : a = key
  · subst a; simp [ParentEdge,setParent,modify,pruned,add,eq_comm]
  · simp [ParentEdge,setParent,modify,pruned,add,h]

-- Actual algorithm removes this instance's old parent before checking the new
-- edge. The independent property ranges over paths of the resulting parent DAG.
theorem reparent_exact (s : State d p n) (key : Fin n) (parent : Option (Fin n))
    (valid : Acyclic (ParentEdge s)) :
    reparentCheck s key parent = true ↔ Acyclic (ParentEdge (setParent s key parent)) := by
  cases parent with
  | none => simp only [reparentCheck,parent_none_edge]; exact ⟨fun _ => pruned_valid s key valid,fun _ => True.intro⟩
  | some target =>
    rw [parent_some_edge]
    exact checkAdd_exact _ key target (pruned_valid s key valid)

theorem reparent_valid (s : State d p n) (key : Fin n) (parent : Option (Fin n))
    (valid : Valid s) (checked : reparentCheck s key parent = true) :
    Valid (setParent s key parent) := by
  refine ⟨valid.definitions,valid.versions,?_,?_,(reparent_exact s key parent valid.parents).mp checked⟩
  · intro k
    by_cases h : k = key <;> simpa [setParent,modify,h] using valid.interfaces k
  · intro k
    by_cases h : k = key <;> simpa [setParent,modify,h] using valid.placed k

theorem refines_trans (a b c : Contract) (ab : Refines a b) (bc : Refines b c) : Refines a c :=
  ⟨fun x h => bc.inputs x (ab.inputs x h),Int.le_trans ab.lower bc.lower,Int.le_trans bc.upper ab.upper⟩

def replace (s : State d p n) (key : Fin n) (definition : Fin d) : State d p n :=
  modify s key fun i => {i with definition := definition, revision := i.revision+1}

def replacementCheck (s : State d p n) (key : Fin n) (definition : Fin d) : Bool :=
  exchangeCheck (s.definitions (s.instances key).definition).contract (s.definitions definition)

theorem replacement_valid (s : State d p n) (key : Fin n) (definition : Fin d)
    (valid : Valid s) (checked : replacementCheck s key definition = true) :
    Valid (replace s key definition) := by
  have compatible := ((exchange_exact _ _).mp checked).1
  refine ⟨valid.definitions,valid.versions,?_,?_,?_⟩
  · intro k
    by_cases h : k = key
    · subst k
      simp only [replace,modify_same]
      exact refines_trans _ _ _ (valid.interfaces key) compatible
    · simpa [replace,modify,h] using valid.interfaces k
  · intro k
    by_cases h : k = key <;> simpa [replace,modify,h] using valid.placed k
  · have same : ParentEdge (replace s key definition) = ParentEdge s := by
      funext a b
      simp only [ParentEdge,replace,modify]
      split <;> rfl
    rw [same]
    exact valid.parents

-- This dependency source is computed from the SAME concrete instance fields.
def parentFormula (parent : Option (Fin n)) : Formula (Fin n) :=
  match parent with | none => .top | some k => .ref k

def snapshot (s : State d p n) : Snapshot n :=
  {forms := fun k => .both (parentFormula (s.instances k).parent) (s.instances k).dependencies,
   eligible := fun k => (s.instances k).enabled && (s.instances k).placements.any s.participating}

theorem parentFormula_map (parent : Option (Fin n)) :
    parentFormula (parent.map (left n 1)) = mapFormula (left n 1) (parentFormula parent) := by
  cases parent <;> rfl

theorem instantiate_forms (s : State d p n) (definition : Fin d) (owner : Nat)
    (placements : List (Fin p)) (parent : Option (Fin n)) (dependencies : Formula (Fin n))
    (k : Fin n) :
    (snapshot (instantiate s definition owner placements parent dependencies)).forms (left n 1 k) =
      mapFormula (left n 1) ((snapshot s).forms k) := by
  simp only [snapshot,instantiate_old,mapInstance,parentFormula_map,mapFormula]

theorem instantiate_eligible (s : State d p n) (definition : Fin d) (owner : Nat)
    (placements : List (Fin p)) (parent : Option (Fin n)) (dependencies : Formula (Fin n))
    (k : Fin n) :
    (snapshot (instantiate s definition owner placements parent dependencies)).eligible (left n 1 k) =
      (snapshot s).eligible k := by
  simp only [snapshot,mapInstance,instantiate]
  simp only [extend_old]

theorem instantiate_old_live (s : State d p n) (definition : Fin d) (owner : Nat)
    (placements : List (Fin p)) (parent : Option (Fin n)) (dependencies : Formula (Fin n))
    (k : Fin n) :
    snapshotLive (snapshot (instantiate s definition owner placements parent dependencies)) (left n 1 k) =
      snapshotLive (snapshot s) k := by
  apply Bool.eq_iff_iff.mpr
  rw [snapshot_live_exact,snapshot_live_exact]
  exact grounded_embedding _ _ _ _ _
    (instantiate_forms s definition owner placements parent dependencies)
    (instantiate_eligible s definition owner placements parent dependencies) k

theorem retire_forms (s : State d p n) (key : Fin n) :
    (snapshot (retire s key)).forms = (snapshot s).forms := by
  funext k
  simp only [snapshot,retire,modify]
  split <;> rfl

theorem retire_eligibility (s : State d p n) (key k : Fin n) :
    (snapshot (retire s key)).eligible k = true → (snapshot s).eligible k = true := by
  by_cases h : k = key <;> simp [snapshot,retire,modify,h]

theorem retirement_no_resurrection (s : State d p n) (key k : Fin n) :
    snapshotLive (snapshot (retire s key)) k = true → snapshotLive (snapshot s) k = true := by
  rw [snapshot_live_exact,snapshot_live_exact,retire_forms]
  exact retirement_mono (retire_eligibility s key) k

theorem register_support_unchanged (s : State d p n) (new : Definition) (predecessor : Option (Fin d)) :
    snapshot (register s new predecessor) = snapshot s := rfl

-- Leaving does not erase history. Rejoining starts a new locus incarnation.
-- Permission to execute these transitions is a separate current-auth boundary.
def leave (s : State d p n) (place : Fin p) : State d p n :=
  {s with participating := fun k => if k = place then false else s.participating k}

def join (s : State d p n) (place : Fin p) : State d p n :=
  {s with
    participating := (fun k => if k = place then true else s.participating k)
    placeIncarnation := (fun k => if k = place then s.placeIncarnation k + 1 else s.placeIncarnation k)}

theorem leave_valid (s : State d p n) (place : Fin p) (valid : Valid s) : Valid (leave s place) :=
  ⟨valid.definitions,valid.versions,valid.interfaces,valid.placed,valid.parents⟩
theorem join_valid (s : State d p n) (place : Fin p) (valid : Valid s) : Valid (join s place) :=
  ⟨valid.definitions,valid.versions,valid.interfaces,valid.placed,valid.parents⟩
theorem join_incarnation (s : State d p n) (place : Fin p) :
    (join s place).placeIncarnation place = s.placeIncarnation place+1 := by simp [join]

-- A captured invocation stamp is not the persistent source-level fallback
-- binding. Pending work retains this exact stamp; a fresh call is a separate entry.
structure Capture where
  realm : Nat
  key : Nat
  revision : Nat
  place : Nat
  placeIncarnation : Nat
  deriving DecidableEq, Repr

def capture (s : State d p n) (key : Fin n) (place : Fin p) : Capture :=
  ⟨s.realm,key.val,(s.instances key).revision,place.val,s.placeIncarnation place⟩

def CurrentCapture (s : State d p n) (c : Capture) (key : Fin n) (place : Fin p) : Prop :=
  c = capture s key place ∧ place ∈ (s.instances key).placements ∧
    s.participating place = true ∧
    Grounded (snapshot s).forms (fun k => (snapshot s).eligible k = true) key

def captureCheck (s : State d p n) (c : Capture) (key : Fin n) (place : Fin p) : Bool :=
  decide (c = capture s key place) && (s.instances key).placements.contains place &&
    s.participating place && snapshotLive (snapshot s) key

theorem capture_exact (s : State d p n) (c : Capture) (key : Fin n) (place : Fin p) :
    captureCheck s c key place = true ↔ CurrentCapture s c key place := by
  simp [captureCheck,CurrentCapture,snapshot_live_exact,and_assoc]

theorem join_old_capture_rejected (s : State d p n) (key : Fin n) (place : Fin p) :
    captureCheck (join (leave s place) place) (capture s key place) key place = false := by
  have changed : capture s key place ≠ capture (join (leave s place) place) key place := by
    intro h
    have eq := congrArg Capture.placeIncarnation h
    simp [capture,join,leave] at eq
  simp [captureCheck,changed]

theorem retired_capture_rejected (s : State d p n) (key : Fin n) (place : Fin p) :
    captureCheck (retire s key) (capture s key place) key place = false := by
  have changed : capture s key place ≠ capture (retire s key) key place := by
    intro h
    have eq := congrArg Capture.revision h
    simp [capture,retire,modify] at eq
  simp [captureCheck,changed]

theorem grown_capture_retained (s : State d p n) (definition : Fin d) (owner : Nat)
    (placements : List (Fin p)) (parent : Option (Fin n)) (dependencies : Formula (Fin n))
    (key : Fin n) (place : Fin p) (c : Capture) :
    captureCheck (instantiate s definition owner placements parent dependencies) c (left n 1 key) place =
      captureCheck s c key place := by
  simp only [captureCheck,instantiate_old,instantiate_old_live,mapInstance]
  have same : capture (instantiate s definition owner placements parent dependencies) (left n 1 key) place =
      capture s key place := by
    simp [capture,instantiate,extend,mapInstance,left]
  rw [same]
  rfl

namespace Controls
open InstancePrograms.Controls
def empty : State 2 3 0 :=
  ⟨91,fun _ => 0,fun k => if k = 0 then original else replacement,fun _ => none,Fin.elim0,fun _ => true⟩
def base := instantiate empty 0 7 [0,1,2] none .top
def two := instantiate base 0 7 [0,2] none .top
def withFollower := instantiate two 0 7 [2] none (.ref 1)
#guard (two.instances 0).definition = (two.instances 1).definition
#guard (two.instances 0).placements.length = 3
#guard (two.instances 1).placements.length = 2
#guard snapshotLive (snapshot two) 0 && snapshotLive (snapshot two) 1
def retiredExtra := retire withFollower 1
#guard (retiredExtra.instances 1).enabled = false
#guard (retiredExtra.instances 1).definition = 0
#guard snapshotLive (snapshot retiredExtra) 0
#guard !snapshotLive (snapshot retiredExtra) 1
#guard !snapshotLive (snapshot retiredExtra) 2
#guard reparentCheck two 1 (some 0)
def reparented := setParent two 1 (some 0)
#guard !reparentCheck reparented 0 (some 1)
#guard !reparentCheck two 1 (some 1)
-- A previously existing instance can lawfully depend on a newer root.
#guard reparentCheck two 0 (some 1)
#guard replacementCheck two 1 1
def replaced := replace two 1 1
#guard (replaced.instances 0).definition = 0
#guard (replaced.instances 1).definition = 1
#guard Machine.run (replaced.definitions (replaced.instances 0).definition).code 2 = some 5
#guard Machine.run (replaced.definitions (replaced.instances 1).definition).code 2 = some 6
def registered := register base replacement (some 0)
#guard registrationCheck base replacement (some 0)
#guard !registrationCheck base ⟨.input 0,contract⟩ (some 0)
#guard (registered.definitions 2).code = replacement.code
#guard (registered.predecessors 2).map Fin.val = some 0
#guard (registered.instances 0).definition.val = 0
#guard snapshotLive (snapshot registered) 0
#guard captureCheck two (capture two 1 2) 1 2
#guard !captureCheck two (capture two 1 1) 1 1
#guard !captureCheck (leave two 2) (capture two 1 2) 1 2
-- A surviving placement does not make this departed placement current.
#guard snapshotLive (snapshot (leave two 2)) 1
#guard !captureCheck (join (leave two 2) 2) (capture two 1 2) 1 2
#guard captureCheck (join (leave two 2) 2) (capture (join (leave two 2) 2) 1 2) 1 2
#guard !captureCheck {two with realm := 92} (capture two 1 2) 1 2
end Controls

#print axioms capture_exact
#print axioms join_old_capture_rejected
#print axioms retired_capture_rejected
#print axioms grown_capture_retained
#print axioms leave_valid
#print axioms join_valid
#print axioms join_incarnation
#print axioms registration_exact
#print axioms register_valid
#print axioms register_old_definition
#print axioms register_instance_locator
#print axioms register_support_unchanged
#print axioms instantiate_forms
#print axioms instantiate_eligible
#print axioms instantiate_old_live
#print axioms retire_valid
#print axioms retirement_retains_record
#print axioms retirement_is_instance_local
#print axioms reparent_exact
#print axioms reparent_valid
#print axioms replacement_valid
#print axioms retirement_no_resurrection
#print axioms instantiate_parents
#print axioms instantiate_valid
end MirroreaProofFirst.InstanceState
