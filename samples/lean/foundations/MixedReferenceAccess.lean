import MixedFallbackStatic
import MixedRequestEmbedding
import MirroreaProofFirstReferenceAccess
namespace MirroreaProofFirst.MixedReferenceAccess
open MixedInstanceState WorldProjection CurrentUse FallbackStatic
open ReferenceAccess (Request Guard WitnessMeaning witness_exact Coordinates)
-- Keep the existing reference payloads, choices and guards. A current context
-- exists only for a real pure definition in the COMMON catalog; no fabricated
-- default definition or second catalog is introduced for owner entries.
def currentWith (s : MixedManagementEntry.System p a) (r : Request)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (definition : InstancePrograms.Definition) : ReferenceAccess.Context :=
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
   request := r, memberIdentity := (MixedCatalogService.slotRecord data s.view (.member member)).identity,
   readerIdentity := (MixedCatalogService.slotRecord data s.view (.moduleSlot reader)).identity,
   targetCapture := ⟨data.realm,key.val,(data.instances key).revision,place.val,data.placeIncarnation place⟩,
   definition := definition,
   arithmeticProfile := 1, contractTheoryVersion := 1}

def current (s : MixedManagementEntry.System p a) (r : Request)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) : Option ReferenceAccess.Context :=
 match MixedCatalogService.operationDefinition s.configuration.state key with
 | .pure d => some (currentWith s r member reader key place d)
 | .owner _ => none

def ContextAt (s : MixedManagementEntry.System p a) (r : Request)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (ctx : ReferenceAccess.Context) : Prop :=
 ∃ d, MixedCatalogService.operationDefinition s.configuration.state key = .pure d ∧
   ctx = currentWith s r member reader key place d

theorem current_exact : current s r member reader key place = some ctx ↔ ContextAt s r member reader key place ctx := by
 unfold current ContextAt
 cases MixedCatalogService.operationDefinition s.configuration.state key <;> simp [eq_comm]

-- Read-reference policies are supplied independently by the authority boundary.
-- The full chain, access annotation, epoch, lineage and code are bound by the
-- structural context even though this W1 claim language has no argument predicate.
def revalidate (s : MixedManagementEntry.System p a) (policy : Policy) (ctx : ReferenceAccess.Context) (g : Guard) : Bool :=
  decide (g.context = ctx) && decide (g.policy = policy.id) && decide (g.version = policy.version) &&
    checkWitness s.view.authority ctx.scope policy.label policy.expression g.witness

def Physical (s : MixedManagementEntry.System p a) (r : Request) (member : Fin a)
    (reader key : Fin s.configuration.count) (place : Fin p) : Prop :=
  MixedManagementEntry.CurrentActor s member place r.principal ∧
  Support.Grounded (snapshot s.configuration.state).forms
    (fun k => (snapshot s.configuration.state).eligible k = true) reader ∧
  CurrentCapture s.configuration.state (capture s.configuration.state key place) key place

def physicalCheck (s : MixedManagementEntry.System p a) (r : Request) (member : Fin a)
    (reader key : Fin s.configuration.count) (place : Fin p) : Bool :=
  MixedManagementEntry.actorCheck s member place r.principal &&
    Support.snapshotLive (snapshot s.configuration.state) reader &&
    captureCheck s.configuration.state (capture s.configuration.state key place) key place

theorem physical_exact (s : MixedManagementEntry.System p a) (r : Request) (member : Fin a)
    (reader key : Fin s.configuration.count) (place : Fin p) :
    physicalCheck s r member reader key place = true ↔ Physical s r member reader key place := by
  simp only [physicalCheck,Bool.and_eq_true,MixedManagementEntry.actor_exact,
    Support.snapshot_live_exact,capture_exact,Physical,and_assoc]

def Ready (s : MixedManagementEntry.System p a) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) : Prop :=
  Coordinates r option member reader key place ∧ Physical s r member reader key place ∧
  s.serial < option.leaseUntil ∧ MixedOperationDefinitions.Contract.pure option.contract = (s.configuration.state.instances key).interface ∧
  MixedFallbackStatic.Admissible s.configuration.state r.chain

def readyCheck (s : MixedManagementEntry.System p a) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) : Bool :=
  decide (Coordinates r option member reader key place) && physicalCheck s r member reader key place &&
  decide (s.serial < option.leaseUntil) &&
  decide (MixedOperationDefinitions.Contract.pure option.contract = (s.configuration.state.instances key).interface) &&
  (match MixedFallbackStatic.check s.configuration.state r.chain with | .ok _ => true | .error _ => false)

theorem ready_exact (s : MixedManagementEntry.System p a) (r : Request) (option : OptionDecl)
    (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) :
    readyCheck s r option member reader key place = true ↔ Ready s r option member reader key place := by
  have static : (match MixedFallbackStatic.check s.configuration.state r.chain with
      | .ok _ => true | .error _ => false) = true ↔ MixedFallbackStatic.Admissible s.configuration.state r.chain := by
    rw [← MixedFallbackStatic.check_exact]
    cases MixedFallbackStatic.check s.configuration.state r.chain with
    | error reason => simp
    | ok value => cases value; simp
  simp only [readyCheck,Bool.and_eq_true,decide_eq_true_eq,physical_exact,static,Ready,and_assoc]

def Allowed (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) : Prop :=
 Ready s r option member reader key place ∧ ∃ ctx,
 ContextAt s r member reader key place ctx ∧
 Authorized s.view.authority ctx.scope policy.label policy.expression

def SavedAt (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (g : Guard) : Prop :=
 Ready s r option member reader key place ∧ ContextAt s r member reader key place g.context ∧
 g.policy = policy.id ∧ g.version = policy.version ∧
 WitnessMeaning s.view.authority g.context.scope policy.label policy.expression g.witness

def checkAt (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (g : Guard) : Bool :=
 readyCheck s r option member reader key place &&
 match current s r member reader key place with
 | none => false
 | some ctx => revalidate s policy ctx g

theorem checkAt_exact (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (g : Guard) :
 checkAt s policy r option member reader key place g = true ↔ SavedAt s policy r option member reader key place g := by
 simp only [SavedAt,← current_exact]
 cases h : current s r member reader key place with
 | none => simp [checkAt,h]
 | some ctx =>
   simp only [checkAt,h,Bool.and_eq_true,ready_exact,revalidate,decide_eq_true_eq,
     witness_exact,Option.some.injEq,and_assoc]
   constructor
   · rintro ⟨ready,rfl,policyEq,versionEq,witness⟩
     exact ⟨ready,rfl,policyEq,versionEq,witness⟩
   · rintro ⟨ready,rfl,policyEq,versionEq,witness⟩
     exact ⟨ready,rfl,policyEq,versionEq,witness⟩

theorem checkAt_sound (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (g : Guard)
 (h : checkAt s policy r option member reader key place g = true) :
 Allowed s policy r option member reader key place ∧ ContextAt s r member reader key place g.context := by
 obtain ⟨ready,ctx,_,_,w⟩ := (checkAt_exact _ _ _ _ _ _ _ _ _).mp h
 exact ⟨⟨ready,g.context,ctx,checkWitness_sound _ _ _ _ _ ((witness_exact _ _ _ _ _).mpr w)⟩,ctx⟩

def prepareAt (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) : Option Guard := do
 if !readyCheck s r option member reader key place then none else do
 let ctx ← current s r member reader key place
 let witness ← produce s.view.authority ctx.scope policy.label policy.expression
 return ⟨ctx,policy.id,policy.version,witness⟩

theorem prepareAt_checked (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (g : Guard)
 (h : prepareAt s policy r option member reader key place = some g) :
 checkAt s policy r option member reader key place g = true := by
 unfold prepareAt at h
 split at h
 · cases h
 · rename_i permitted
   have ready : readyCheck s r option member reader key place = true := by simpa using permitted
   cases hc : current s r member reader key place with
   | none => simp [hc] at h
   | some ctx =>
     simp only [hc,Option.bind_eq_bind,Option.bind_some] at h
     cases hw : produce s.view.authority ctx.scope policy.label policy.expression with
     | none => simp [hw] at h
     | some w =>
       simp only [hw,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at h
       subst g
       simp [checkAt,ready,hc,revalidate,produce_checked _ _ _ _ _ hw]

theorem prepareAt_complete (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p)
 (h : Allowed s policy r option member reader key place) :
 ∃ g, prepareAt s policy r option member reader key place = some g := by
 obtain ⟨ready,ctx,currentCtx,auth⟩ := h
 obtain ⟨w,hw⟩ := produce_complete _ _ _ _ auth
 exact ⟨⟨ctx,policy.id,policy.version,w⟩,
   by simp [prepareAt,(ready_exact _ _ _ _ _ _ _).mpr ready,current_exact.mpr currentCtx,hw]⟩

-- The pure reference boundary rejects owner definitions even on a malformed
-- raw state whose stored interface lies about its kind.
theorem owner_rejected (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (option : OptionDecl)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p) (g : Guard)
 (d : MixedOperationDefinitions.OwnerDefinition) (kind : MixedCatalogService.operationDefinition s.configuration.state key = .owner d) :
 checkAt s policy r option member reader key place g = false := by
 simp [checkAt,current,kind]

def resolve (s : MixedManagementEntry.System p a) (r : Request) :
 Option (OptionDecl × Fin a × Fin s.configuration.count × Fin s.configuration.count × Fin p) := do
 let option ← r.chain.options[r.optionIndex]?
 let member ← CompositionCore.index a r.member
 let reader ← CompositionCore.index s.configuration.count r.chain.reader
 let key ← CompositionCore.index s.configuration.count option.target
 let place ← CompositionCore.index p r.place
 return (option,member,reader,key,place)

theorem resolve_complete (s : MixedManagementEntry.System p a) (r : Request) (option : OptionDecl)
 (member : Fin a) (reader key : Fin s.configuration.count) (place : Fin p)
 (coords : Coordinates r option member reader key place) :
 resolve s r = some (option,member,reader,key,place) := by
 obtain ⟨hm,hp,hr,ho,hk⟩ := coords
 simp [resolve,ho,hm,hp,hr,hk,CompositionCore.index_roundtrip]

def check (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (g : Guard) : Bool :=
 match resolve s r with
 | none => false
 | some (option,member,reader,key,place) => checkAt s policy r option member reader key place g

def prepare (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) : Option Guard := do
 let (option,member,reader,key,place) ← resolve s r
 prepareAt s policy r option member reader key place

theorem prepare_checked (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (g : Guard)
 (h : prepare s policy r = some g) : check s policy r g = true := by
 unfold prepare at h
 cases hr : resolve s r with
 | none => simp [hr] at h
 | some coords =>
   obtain ⟨option,member,reader,key,place⟩ := coords
   simp only [hr,Option.bind_eq_bind,Option.bind_some] at h
   simpa only [check,hr] using prepareAt_checked _ _ _ _ _ _ _ _ _ h

def Saved (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (g : Guard) : Prop :=
 ∃ option member reader key place, SavedAt s policy r option member reader key place g

theorem check_exact (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (g : Guard) :
 check s policy r g = true ↔ Saved s policy r g := by
 constructor
 · intro h
   unfold check at h
   cases hr : resolve s r with
   | none => simp [hr] at h
   | some coords =>
     obtain ⟨option,member,reader,key,place⟩ := coords
     exact ⟨option,member,reader,key,place,(checkAt_exact _ _ _ _ _ _ _ _ _).mp (by simpa [hr] using h)⟩
 · rintro ⟨option,member,reader,key,place,h⟩
   have resolved := resolve_complete _ _ _ _ _ _ _ h.1.1
   simpa [check,resolved] using (checkAt_exact _ _ _ _ _ _ _ _ _).mpr h

theorem check_sound (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request) (g : Guard)
 (h : check s policy r g = true) :
 ∃ option member reader key place, Allowed s policy r option member reader key place ∧
 ContextAt s r member reader key place g.context := by
 obtain ⟨option,member,reader,key,place,saved⟩ := (check_exact _ _ _ _).mp h
 exact ⟨option,member,reader,key,place,checkAt_sound _ _ _ _ _ _ _ _ _ ((checkAt_exact _ _ _ _ _ _ _ _ _).mpr saved)⟩

theorem prepare_complete (s : MixedManagementEntry.System p a) (policy : Policy) (r : Request)
 (h : ∃ option member reader key place, Allowed s policy r option member reader key place) :
 ∃ g, prepare s policy r = some g := by
 obtain ⟨option,member,reader,key,place,h⟩ := h
 have resolved := resolve_complete _ _ _ _ _ _ _ h.1.1
 obtain ⟨g,hg⟩ := prepareAt_complete _ _ _ _ _ _ _ _ h
 exact ⟨g,by simpa [prepare,resolved] using hg⟩

#print axioms current_exact
#print axioms ready_exact
#print axioms checkAt_exact
#print axioms checkAt_sound
#print axioms prepareAt_checked
#print axioms prepareAt_complete
#print axioms owner_rejected
#print axioms check_exact
#print axioms prepare_complete
end MirroreaProofFirst.MixedReferenceAccess
