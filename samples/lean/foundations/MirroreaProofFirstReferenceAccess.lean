import MirroreaProofFirstFallbackStatic
import MirroreaProofFirstInvocationBoundary

namespace MirroreaProofFirst.ReferenceAccess
open InstanceState InstancePrograms WorldProjection CurrentUse FallbackStatic

-- LAB read-reference permission, separate from owner mutation and action-5
-- invocation. No dummy argument, no authority issuance, no public action ABI.
structure Request where
  binding : Nat
  epoch : Nat
  lineage : Nat
  chain : Chain
  optionIndex : Nat
  member : Nat
  place : Nat
  principal : Nat
  deriving DecidableEq, Repr

structure Context where
  scope : CurrentUse.Context
  request : Request
  memberIdentity : RecordIdentity
  readerIdentity : RecordIdentity
  targetCapture : Capture
  definition : Definition
  arithmeticProfile : Nat
  contractTheoryVersion : Nat
  deriving DecidableEq, Repr

structure Guard where
  context : Context
  policy : Nat
  version : Nat
  witness : Witness
  deriving DecidableEq, Repr

-- The proof tree itself is part of the judgment. Existential authorization
-- alone could silently swap a revoked disjunction branch for another one.
inductive WitnessMeaning (authority : Authority) (ctx : CurrentUse.Context) (label : Nat) :
    PolicyExpr → Witness → Prop where
  | leaf : ValidClaim authority ctx label need claim → WitnessMeaning authority ctx label (.leaf need) (.leaf claim)
  | both : WitnessMeaning authority ctx label left l → WitnessMeaning authority ctx label right r →
      WitnessMeaning authority ctx label (.both left right) (.both l r)
  | left : WitnessMeaning authority ctx label left l → WitnessMeaning authority ctx label (.either left right) (.left l)
  | right : WitnessMeaning authority ctx label right r → WitnessMeaning authority ctx label (.either left right) (.right r)

theorem witness_exact (authority : Authority) (ctx : CurrentUse.Context) (label : Nat)
    (policy : PolicyExpr) (witness : Witness) :
    checkWitness authority ctx label policy witness = true ↔ WitnessMeaning authority ctx label policy witness := by
  constructor
  · intro checked
    induction policy generalizing witness with
    | leaf need =>
        cases witness <;> simp only [checkWitness,Bool.false_eq_true] at checked
        exact .leaf ((checkClaim_exact _ _ _ _ _).mp checked)
    | both left right il ir =>
        cases witness <;> simp only [checkWitness,Bool.false_eq_true,Bool.and_eq_true] at checked
        exact .both (il _ checked.1) (ir _ checked.2)
    | either left right il ir =>
        cases witness <;> simp only [checkWitness,Bool.false_eq_true] at checked
        · exact .left (il _ checked)
        · exact .right (ir _ checked)
  · intro meaning
    induction meaning with
    | leaf valid => exact (checkClaim_exact _ _ _ _ _).mpr valid
    | both _ _ il ir => simp [checkWitness,il,ir]
    | left _ ih => exact ih
    | right _ ih => exact ih

def current (s : ManagementEntry.System p a) (r : Request)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) : Context :=
  let data := s.configuration.state
  {scope :=
    {principal := r.principal, member := member.val,
     memberIncarnation := (s.view.members member).incarnation,
     locus := place.val, locusIncarnation := data.placeIncarnation place,
     moduleKey := key.val, moduleIncarnation := 1, action := 13, target := key.val,
     targetIncarnation := 1, targetRevision := (data.instances key).revision,
     instanceId := data.realm, request := r.binding, arguments := [],
     code := (data.instances key).definition.val, contract := (data.instances key).definition.val,
     generation := s.view.generation},
   request := r, memberIdentity := (slotRecord data s.view (.member member)).identity,
   readerIdentity := (slotRecord data s.view (.moduleSlot reader)).identity,
   targetCapture := capture data key place,
   definition := data.definitions (data.instances key).definition,
   arithmeticProfile := 1, contractTheoryVersion := 1}

-- Read-reference policies are supplied independently by the authority boundary.
-- The full chain, access annotation, epoch, lineage and code are bound by the
-- structural context even though this W1 claim language has no argument predicate.
def revalidate (s : ManagementEntry.System p a) (policy : Policy) (ctx : Context) (g : Guard) : Bool :=
  decide (g.context = ctx) && decide (g.policy = policy.id) && decide (g.version = policy.version) &&
    checkWitness s.view.authority ctx.scope policy.label policy.expression g.witness

def Coordinates (r : Request) (option : OptionDecl) (member : Fin a)
    (reader key : Fin n) (place : Fin p) : Prop :=
  r.member = member.val ∧ r.place = place.val ∧ r.chain.reader = reader.val ∧
  r.chain.options[r.optionIndex]? = some option ∧ option.target = key.val

instance (r : Request) (option : OptionDecl) (member : Fin a)
    (reader key : Fin n) (place : Fin p) : Decidable (Coordinates r option member reader key place) :=
  inferInstanceAs (Decidable (_ ∧ _ ∧ _ ∧ _ ∧ _))

def Physical (s : ManagementEntry.System p a) (r : Request) (member : Fin a)
    (reader key : Fin s.configuration.count) (place : Fin p) : Prop :=
  ManagementEntry.CurrentActor s member place r.principal ∧
  Support.Grounded (snapshot s.configuration.state).forms
    (fun k => (snapshot s.configuration.state).eligible k = true) reader ∧
  CurrentCapture s.configuration.state (capture s.configuration.state key place) key place

def physicalCheck (s : ManagementEntry.System p a) (r : Request) (member : Fin a)
    (reader key : Fin s.configuration.count) (place : Fin p) : Bool :=
  ManagementEntry.actorCheck s member place r.principal &&
    Support.snapshotLive (snapshot s.configuration.state) reader &&
    captureCheck s.configuration.state (capture s.configuration.state key place) key place

theorem physical_exact (s : ManagementEntry.System p a) (r : Request) (member : Fin a)
    (reader key : Fin s.configuration.count) (place : Fin p) :
    physicalCheck s r member reader key place = true ↔ Physical s r member reader key place := by
  simp only [physicalCheck,Bool.and_eq_true,ManagementEntry.actor_exact,
    Support.snapshot_live_exact,capture_exact,Physical,and_assoc]

def Ready (s : ManagementEntry.System p a) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) : Prop :=
  Coordinates r option member reader key place ∧ Physical s r member reader key place ∧
  s.serial < option.leaseUntil ∧ option.contract = (s.configuration.state.instances key).interface ∧
  Admissible s.configuration.state r.chain

def readyCheck (s : ManagementEntry.System p a) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) : Bool :=
  decide (Coordinates r option member reader key place) && physicalCheck s r member reader key place &&
  decide (s.serial < option.leaseUntil) &&
  decide (option.contract = (s.configuration.state.instances key).interface) &&
  (match FallbackStatic.check s.configuration.state r.chain with | .ok _ => true | .error _ => false)

theorem ready_exact (s : ManagementEntry.System p a) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) :
    readyCheck s r option member reader key place = true ↔ Ready s r option member reader key place := by
  have static : (match FallbackStatic.check s.configuration.state r.chain with
      | .ok _ => true | .error _ => false) = true ↔ Admissible s.configuration.state r.chain := by
    rw [← FallbackStatic.check_exact]
    cases FallbackStatic.check s.configuration.state r.chain with
    | error reason => simp
    | ok value => cases value; simp
  simp only [readyCheck,Bool.and_eq_true,decide_eq_true_eq,physical_exact,static,Ready,and_assoc]

def Allowed (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) : Prop :=
  Ready s r option member reader key place ∧
  Authorized s.view.authority (current s r member reader key place).scope policy.label policy.expression

def checkAt (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (g : Guard) : Bool :=
  readyCheck s r option member reader key place && revalidate s policy (current s r member reader key place) g

def SavedAt (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (g : Guard) : Prop :=
  Ready s r option member reader key place ∧ g.context = current s r member reader key place ∧
  g.policy = policy.id ∧ g.version = policy.version ∧
  WitnessMeaning s.view.authority (current s r member reader key place).scope policy.label policy.expression g.witness

theorem checkAt_exact (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (g : Guard) :
    checkAt s policy r option member reader key place g = true ↔ SavedAt s policy r option member reader key place g := by
  simp only [checkAt,Bool.and_eq_true,ready_exact,revalidate,decide_eq_true_eq,witness_exact,SavedAt,and_assoc]

theorem checkAt_sound (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (g : Guard)
    (accepted : checkAt s policy r option member reader key place g = true) :
    Allowed s policy r option member reader key place ∧ g.context = current s r member reader key place := by
  simp only [checkAt,Bool.and_eq_true,ready_exact,revalidate,decide_eq_true_eq] at accepted
  exact ⟨⟨accepted.1,checkWitness_sound _ _ _ _ _ accepted.2.2⟩,accepted.2.1.1.1⟩

def prepareAt (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) : Option Guard := do
  if !readyCheck s r option member reader key place then none else do
    let ctx := current s r member reader key place
    let witness ← produce s.view.authority ctx.scope policy.label policy.expression
    return ⟨ctx,policy.id,policy.version,witness⟩

theorem prepareAt_checked (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (g : Guard)
    (produced : prepareAt s policy r option member reader key place = some g) :
    checkAt s policy r option member reader key place g = true := by
  unfold prepareAt at produced
  split at produced
  · cases produced
  · rename_i ready
    have hr : readyCheck s r option member reader key place = true := by simpa using ready
    cases hw : produce s.view.authority (current s r member reader key place).scope policy.label policy.expression with
    | none => simp [hw] at produced
    | some witness =>
        simp only [hw,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at produced
        subst g
        simp [checkAt,hr,revalidate,produce_checked _ _ _ _ _ hw]

theorem prepareAt_complete (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p)
    (allowed : Allowed s policy r option member reader key place) :
    ∃ g, prepareAt s policy r option member reader key place = some g := by
  obtain ⟨w,hw⟩ := produce_complete _ _ _ _ allowed.2
  exact ⟨⟨current s r member reader key place,policy.id,policy.version,w⟩,
    by simp [prepareAt,(ready_exact _ _ _ _ _ _ _).mpr allowed.1,hw]⟩

-- A saved guard is checked as supplied: context and witness are never refreshed.
theorem changed_epoch_rejected (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (g : Guard)
    (changed : g.context.request.epoch ≠ r.epoch) :
    checkAt s policy r option member reader key place g = false := by
  have different : g.context ≠ current s r member reader key place := by
    intro eq
    exact changed (congrArg (fun ctx => ctx.request.epoch) eq)
  simp [checkAt,revalidate,different]

theorem no_authority_rejected (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (g : Guard)
    (empty : s.view.authority.issued = []) :
    checkAt s policy r option member reader key place g = false := by
  cases accepted : checkAt s policy r option member reader key place g with
  | false => rfl
  | true => exact False.elim (no_claim_no_authorization _ _ _ _ empty
      (checkAt_sound _ _ _ _ _ _ _ _ _ accepted).1.2)

def resolve (s : ManagementEntry.System p a) (r : Request) :
    Option (OptionDecl × Fin a × Fin s.configuration.count × Fin s.configuration.count × Fin p) := do
  let option ← r.chain.options[r.optionIndex]?
  let member ← CompositionCore.index a r.member
  let reader ← CompositionCore.index s.configuration.count r.chain.reader
  let key ← CompositionCore.index s.configuration.count option.target
  let place ← CompositionCore.index p r.place
  return (option,member,reader,key,place)

theorem resolve_complete (s : ManagementEntry.System p a) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p)
    (coords : Coordinates r option member reader key place) :
    resolve s r = some (option,member,reader,key,place) := by
  obtain ⟨hm,hp,hr,ho,hk⟩ := coords
  simp [resolve,ho,hm,hp,hr,hk,CompositionCore.index_roundtrip]

def check (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (g : Guard) : Bool :=
  match resolve s r with
  | none => false
  | some (option,member,reader,key,place) => checkAt s policy r option member reader key place g

def prepare (s : ManagementEntry.System p a) (policy : Policy) (r : Request) : Option Guard := do
  let (option,member,reader,key,place) ← resolve s r
  prepareAt s policy r option member reader key place

theorem prepare_checked (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (g : Guard)
    (produced : prepare s policy r = some g) : check s policy r g = true := by
  unfold prepare at produced
  cases hr : resolve s r with
  | none => simp [hr] at produced
  | some coords =>
      obtain ⟨option,member,reader,key,place⟩ := coords
      simp only [hr,Option.bind_eq_bind,Option.bind_some] at produced
      simpa only [check,hr] using prepareAt_checked _ _ _ _ _ _ _ _ _ produced

theorem check_sound (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (g : Guard)
    (accepted : check s policy r g = true) :
    ∃ option member reader key place, Allowed s policy r option member reader key place ∧
      g.context = current s r member reader key place := by
  unfold check at accepted
  cases hr : resolve s r with
  | none => simp [hr] at accepted
  | some coords =>
      obtain ⟨option,member,reader,key,place⟩ := coords
      exact ⟨option,member,reader,key,place,checkAt_sound _ _ _ _ _ _ _ _ _ (by simpa [hr] using accepted)⟩

def Saved (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (g : Guard) : Prop :=
  ∃ option member reader key place, SavedAt s policy r option member reader key place g

theorem check_exact (s : ManagementEntry.System p a) (policy : Policy) (r : Request) (g : Guard) :
    check s policy r g = true ↔ Saved s policy r g := by
  constructor
  · intro accepted
    unfold check at accepted
    cases hr : resolve s r with
    | none => simp [hr] at accepted
    | some coords =>
        obtain ⟨option,member,reader,key,place⟩ := coords
        exact ⟨option,member,reader,key,place,(checkAt_exact _ _ _ _ _ _ _ _ _).mp (by simpa [hr] using accepted)⟩
  · rintro ⟨option,member,reader,key,place,saved⟩
    have resolved := resolve_complete _ _ _ _ _ _ _ saved.1.1
    simpa [check,resolved] using (checkAt_exact _ _ _ _ _ _ _ _ _).mpr saved

theorem prepare_complete (s : ManagementEntry.System p a) (policy : Policy) (r : Request)
    (allowed : ∃ option member reader key place, Allowed s policy r option member reader key place) :
    ∃ g, prepare s policy r = some g := by
  obtain ⟨option,member,reader,key,place,allowed⟩ := allowed
  have resolved := resolve_complete _ _ _ _ _ _ _ allowed.1.1
  obtain ⟨g,hg⟩ := prepareAt_complete _ _ _ _ _ _ _ _ allowed
  exact ⟨g,by simpa [prepare,resolved] using hg⟩

#print axioms physical_exact
#print axioms ready_exact
#print axioms checkAt_sound
#print axioms prepareAt_checked
#print axioms prepareAt_complete
#print axioms changed_epoch_rejected
#print axioms no_authority_rejected
#print axioms prepare_checked
#print axioms check_sound
#print axioms witness_exact
#print axioms checkAt_exact
#print axioms resolve_complete
#print axioms check_exact
#print axioms prepare_complete
end MirroreaProofFirst.ReferenceAccess
