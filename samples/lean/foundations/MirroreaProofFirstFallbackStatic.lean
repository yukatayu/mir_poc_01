import MirroreaProofFirstCatalogHistory

namespace MirroreaProofFirst.FallbackStatic
open InstancePrograms InstanceState CompositionCore

inductive Capability where
  | read | readWrite
  deriving DecidableEq, Repr
def capabilityRank : Capability → Nat | .read => 0 | .readWrite => 1

structure OptionDecl where
  name : String
  target : Nat
  declaredAccess : Option String
  capability : Capability
  contract : Contract
  -- Selected W3 guard: a deadline in the proved monotone local actor cut.
  -- This is not elapsed wall-clock time or a promise about physical leases.
  leaseUntil : Nat
  deriving DecidableEq, Repr

structure EdgeDecl where
  predecessor : String
  successor : String
  sameLineage : Bool
  deriving DecidableEq, Repr

structure Chain where
  reader : Nat
  options : List OptionDecl
  edges : List (Option EdgeDecl)
  deriving DecidableEq, Repr

inductive Error where
  | underdeclared | malformed | outsideProfile | unresolved | lifetime
  deriving DecidableEq, Repr

-- Equality of exported contracts is the first W3 invocation-chain profile;
-- compatible but unequal contracts are outsideProfile, not falsely labeled as
-- an explicit Canon contradiction. Invocation itself currently implements reads.
def Contradicts (a b : Contract) : Prop := a.upper < b.lower ∨ b.upper < a.lower
def contradictionCheck (a b : Contract) : Bool := decide (a.upper < b.lower ∨ b.upper < a.lower)

def EdgeOK (left right : OptionDecl) (edge : Option EdgeDecl) : Prop :=
  ∃ access annotation, left.declaredAccess = some access ∧ right.declaredAccess = some access ∧
    edge = some annotation ∧ annotation.predecessor = left.name ∧ annotation.successor = right.name ∧
    annotation.sameLineage = true ∧ capabilityRank right.capability ≤ capabilityRank left.capability ∧
    ¬ Contradicts left.contract right.contract ∧ left.contract = right.contract

def checkEdge (left right : OptionDecl) (edge : Option EdgeDecl) : Except Error Unit :=
  match left.declaredAccess,right.declaredAccess,edge with
  | some a,some b,some e =>
      if a ≠ b ∨ e.predecessor ≠ left.name ∨ e.successor ≠ right.name ∨ e.sameLineage = false ∨
          capabilityRank left.capability < capabilityRank right.capability ∨
          contradictionCheck left.contract right.contract = true then .error .malformed
      else if left.contract ≠ right.contract then .error .outsideProfile else .ok ()
  | _,_,_ => .error .underdeclared

theorem checkEdge_exact (left right : OptionDecl) (edge : Option EdgeDecl) :
    checkEdge left right edge = .ok () ↔ EdgeOK left right edge := by
  cases ha : left.declaredAccess <;> cases hb : right.declaredAccess <;> cases he : edge <;>
    simp [checkEdge,EdgeOK,ha,hb,Contradicts,contradictionCheck,not_or,Nat.not_lt,Bool.not_eq_false] <;>
    grind

inductive Links : List OptionDecl → List (Option EdgeDecl) → Prop where
  | nil : Links [] []
  | singleton : Links [option] []
  | cons : EdgeOK left right edge → Links (right :: rest) edges → Links (left :: right :: rest) (edge :: edges)

-- A private profile restriction cannot conceal a later definite static error.
-- Whole-chain diagnostics preserve malformed > underdeclared > outsideProfile.
def combine (left right : Except Error Unit) : Except Error Unit :=
  match left,right with
  | .error .malformed,_ | _,.error .malformed => .error .malformed
  | .error .underdeclared,_ | _,.error .underdeclared => .error .underdeclared
  | .error reason,_ => .error reason
  | .ok _,other => other

theorem combine_ok (left right : Except Error Unit) :
    combine left right = .ok () ↔ left = .ok () ∧ right = .ok () := by
  cases left with
  | error a => cases right with
    | error b => cases a <;> cases b <;> simp [combine]
    | ok b => cases a <;> cases b <;> simp [combine]
  | ok a => cases right with
    | error b => cases a <;> cases b <;> simp [combine]
    | ok b => cases a <;> cases b <;> simp [combine]

theorem combine_malformed_left (other : Except Error Unit) :
    combine (.error .malformed) other = .error .malformed := by cases other <;> rfl

theorem combine_malformed_right (other : Except Error Unit) :
    combine other (.error .malformed) = .error .malformed := by
  cases other with
  | error reason => cases reason <;> rfl
  | ok unit => rfl

def checkLinks : List OptionDecl → List (Option EdgeDecl) → Except Error Unit
  | [],[] | [_],[] => .ok ()
  | [],_ :: _ | [_],_ :: _ => .error .malformed
  | _ :: _ :: _,[] => .error .underdeclared
  | left :: right :: rest,edge :: edges =>
      combine (checkEdge left right edge) (checkLinks (right :: rest) edges)

theorem checkLinks_exact (options : List OptionDecl) (edges : List (Option EdgeDecl)) :
    checkLinks options edges = .ok () ↔ Links options edges := by
  induction options generalizing edges with
  | nil =>
      cases edges <;> constructor <;> intro h <;> cases h <;> constructor
  | cons left rest ih =>
      cases rest with
      | nil => cases edges <;> constructor <;> intro h <;> cases h <;> constructor
      | cons right rest =>
          cases edges with
          | nil => constructor <;> intro h <;> cases h
          | cons edge edges =>
              simp only [checkLinks,combine_ok,checkEdge_exact,ih]
              exact ⟨fun h => .cons h.1 h.2,fun h => by cases h with | cons head tail => exact ⟨head,tail⟩⟩

def Shape (chain : Chain) : Prop := chain.options ≠ [] ∧ (chain.options.map OptionDecl.name).Nodup ∧
  (∀ option ∈ chain.options, ∃ access, option.declaredAccess = some access) ∧ Links chain.options chain.edges

def checkShape (chain : Chain) : Except Error Unit :=
  if chain.options.isEmpty ∨ ¬ (chain.options.map OptionDecl.name).Nodup then .error .malformed
  else combine
    (if !(chain.options.all (fun option => option.declaredAccess.isSome)) then .error .underdeclared else .ok ())
    (checkLinks chain.options chain.edges)

theorem checkShape_exact (chain : Chain) : checkShape chain = .ok () ↔ Shape chain := by
  unfold checkShape
  split
  · rename_i bad
    simp only [Shape,Except.error.injEq,reduceCtorEq,false_iff,not_and]
    intro nonempty unique
    exact False.elim (bad.elim (by simpa using nonempty) (fun denied => denied unique))
  · rename_i good
    have base : chain.options ≠ [] ∧ (chain.options.map OptionDecl.name).Nodup := by simpa [not_or] using good
    rw [combine_ok,checkLinks_exact]
    by_cases floor : chain.options.all (fun option => option.declaredAccess.isSome) = true
    · have declared : ∀ option ∈ chain.options, ∃ access, option.declaredAccess = some access := by
        simpa [List.all_eq_true,Option.isSome_iff_exists] using floor
      simp [floor,Shape,base.1,base.2]
      exact fun _ => declared
    · have noFloor : ¬ (∀ option ∈ chain.options, ∃ access, option.declaredAccess = some access) := by
        simpa [List.all_eq_true,Option.isSome_iff_exists] using floor
      have absent : chain.options.all (fun option => option.declaredAccess.isSome) = false := by
        cases h : chain.options.all (fun option => option.declaredAccess.isSome) <;> simp_all
      simp [absent,Shape,noFloor]

-- Independent explicit malformed-edge judgment. Missing fields alone belong
-- to underdeclared; a present, contradictory edge may not be hidden elsewhere.
def EdgeMalformed (left right : OptionDecl) (edge : Option EdgeDecl) : Prop :=
  ∃ a b e, left.declaredAccess = some a ∧ right.declaredAccess = some b ∧ edge = some e ∧
    (a ≠ b ∨ e.predecessor ≠ left.name ∨ e.successor ≠ right.name ∨ e.sameLineage = false ∨
      capabilityRank left.capability < capabilityRank right.capability ∨ Contradicts left.contract right.contract)

theorem checkEdge_malformed (left right : OptionDecl) (edge : Option EdgeDecl) :
    checkEdge left right edge = .error .malformed ↔ EdgeMalformed left right edge := by
  cases ha : left.declaredAccess <;> cases hb : right.declaredAccess <;> cases he : edge <;>
    simp [checkEdge,EdgeMalformed,ha,hb,he,Contradicts,contradictionCheck] <;> grind

inductive MalformedLinks : List OptionDecl → List (Option EdgeDecl) → Prop where
  | extraNil : MalformedLinks [] (edge :: more)
  | extraSingleton : MalformedLinks [option] (edge :: more)
  | head : EdgeMalformed left right edge → MalformedLinks (left :: right :: rest) (edge :: edges)
  | tail : MalformedLinks (right :: rest) edges → MalformedLinks (left :: right :: rest) (edge :: edges)

theorem checkLinks_malformed (options : List OptionDecl) (edges : List (Option EdgeDecl))
    (bad : MalformedLinks options edges) : checkLinks options edges = .error .malformed := by
  induction bad with
  | extraNil => rfl
  | extraSingleton => rfl
  | head edgeBad => simp only [checkLinks,(checkEdge_malformed _ _ _).mpr edgeBad,combine_malformed_left]
  | tail _ ih => simp only [checkLinks,ih,combine_malformed_right]

theorem checkShape_malformed (chain : Chain) (bad : MalformedLinks chain.options chain.edges) :
    checkShape chain = .error .malformed := by
  unfold checkShape
  split
  · rfl
  · rw [checkLinks_malformed _ _ bad,combine_malformed_right]

#print axioms checkShape_malformed

def OptionResolved (s : State d p n) (option : OptionDecl) : Prop :=
  ∃ key : Fin n, option.target = key.val ∧ option.contract = (s.instances key).interface
def optionCheck (s : State d p n) (option : OptionDecl) : Bool :=
  match index n option.target with
  | none => false
  | some key => decide (option.contract = (s.instances key).interface)

theorem option_exact (s : State d p n) (option : OptionDecl) : optionCheck s option = true ↔ OptionResolved s option := by
  constructor
  · intro accepted
    unfold optionCheck at accepted
    cases h : index n option.target with
    | none => simp [h] at accepted
    | some key => exact ⟨key,(index_sound _ h).symm,by simpa [h] using accepted⟩
  · rintro ⟨key,target,contract⟩
    simp [optionCheck,target,index_roundtrip,contract]

-- The terminal target is the reader itself or a genuine ancestor in the SAME
-- instance parent graph. Arbitrary unrelated long-lived constants do not count.
def Dominates (s : State d p n) (reader target : Nat) : Prop :=
  ∃ r t : Fin n, reader = r.val ∧ target = t.val ∧ GraphValidation.Path (ParentEdge s) r t
def dominatesCheck (s : State d p n) (reader target : Nat) : Bool :=
  match index n reader,index n target with
  | some r,some t => GraphValidation.reachable (fun a b => decide ((s.instances a).parent = some b)) r t
  | _,_ => false

theorem dominates_exact (s : State d p n) (reader target : Nat) :
    dominatesCheck s reader target = true ↔ Dominates s reader target := by
  constructor
  · intro checked
    unfold dominatesCheck at checked
    cases hr : index n reader <;> cases ht : index n target <;> simp [hr,ht] at checked
    rename_i r t
    refine ⟨r,t,(index_sound _ hr).symm,(index_sound _ ht).symm,?_⟩
    have path := (GraphValidation.reachable_exact _ _ _).mp checked
    simpa only [decide_eq_true_eq,ParentEdge] using path
  · rintro ⟨r,t,er,et,path⟩
    simp only [dominatesCheck,er,et,index_roundtrip]
    apply (GraphValidation.reachable_exact _ _ _).mpr
    simpa only [decide_eq_true_eq,ParentEdge] using path

def Lifetime (s : State d p n) (chain : Chain) : Prop :=
  ∃ last, chain.options.getLast? = some last ∧ Dominates s chain.reader last.target
def lifetimeCheck (s : State d p n) (chain : Chain) : Bool :=
  match chain.options.getLast? with | none => false | some last => dominatesCheck s chain.reader last.target
theorem lifetime_exact (s : State d p n) (chain : Chain) : lifetimeCheck s chain = true ↔ Lifetime s chain := by
  cases h : chain.options.getLast? <;> simp [lifetimeCheck,Lifetime,h,dominates_exact]

def Admissible (s : State d p n) (chain : Chain) : Prop :=
  Shape chain ∧ (∀ option ∈ chain.options, OptionResolved s option) ∧ Lifetime s chain
def check (s : State d p n) (chain : Chain) : Except Error Unit :=
  match checkShape chain with
  | .error reason => .error reason
  | .ok _ =>
      if !(chain.options.all (optionCheck s)) then .error .unresolved
      else if !lifetimeCheck s chain then .error .lifetime else .ok ()

theorem check_exact (s : State d p n) (chain : Chain) : check s chain = .ok () ↔ Admissible s chain := by
  unfold check
  cases hs : checkShape chain with
  | error reason =>
      have bad : ¬ Shape chain := by intro h; have := (checkShape_exact chain).mpr h; simp [hs] at this
      simp [Admissible,bad]
  | ok unit =>
      cases unit
      have resolved : chain.options.all (optionCheck s) = true ↔
          ∀ option ∈ chain.options, OptionResolved s option := by simp only [List.all_eq_true,option_exact]
      simp only [Admissible,(checkShape_exact chain).mp hs,true_and,← resolved,← lifetime_exact]
      cases chain.options.all (optionCheck s) <;> cases lifetimeCheck s chain <;> simp

#print axioms checkEdge_exact
#print axioms checkLinks_exact
#print axioms checkShape_exact
#print axioms option_exact
#print axioms dominates_exact
#print axioms lifetime_exact
#print axioms check_exact
end MirroreaProofFirst.FallbackStatic
