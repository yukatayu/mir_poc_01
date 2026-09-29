import MixedOwnerSourceFootprint
import MixedReferenceAuthority
import MixedReferenceSourceTrace
namespace MirroreaProofFirst.MixedOwnerSourceIssue
open ReferenceSourceData MixedNamedOwnerSource
variable {id : Nat} {definition : MixedOperationDefinitions.OwnerDefinition}

deriving instance DecidableEq for MixedNamedOwnerSource.Assignment
structure OwnerWaiting where
 saved : OwnerSavedPending.Saved
 statement : Assignment
 dependencies : List ReferenceSource.Read
 beforeCount : Nat
 -- A callable binding read is distinct from integer captures/argument slots.
 operationDependency : ReferenceSource.Read
 deriving DecidableEq
abbrev Waiting := Sum ReferenceSource.Awaiting OwnerWaiting
structure State (p a : Nat) where
 machine : MixedReferenceExecution.Machine p a
 values : Values
 nextRequest : Nat
 writes : List ReferenceSource.Write
 origins : List ReferenceSource.Origin
 waiting : Option Waiting

def pureWaiting : Option Waiting → Option ReferenceSource.Awaiting
 | some (.inl saved) => some saved
 | _ => none
def ownerWaiting : Option Waiting → Option OwnerWaiting
 | some (.inr saved) => some saved
 | _ => none

def base (s : State p a) : MixedReferenceSource.State p a :=
 ⟨s.machine,s.values,s.nextRequest,s.writes,s.origins,pureWaiting s.waiting⟩
def embed (s : MixedReferenceSource.State p a) : State p a :=
 ⟨s.machine,s.values,s.nextRequest,s.writes,s.origins,s.waiting.map Sum.inl⟩
def merge (s : State p a) (next : MixedReferenceSource.State p a) : State p a :=
 ⟨next.machine,next.values,next.nextRequest,next.writes,next.origins,
 match ownerWaiting s.waiting with | some saved => some (.inr saved) | none => next.waiting.map Sum.inl⟩

theorem base_embed (s : MixedReferenceSource.State p a) : base (embed s) = s := by
 cases s with | mk machine values request writes origins waiting => cases waiting <;> rfl

theorem merge_embed (s next : MixedReferenceSource.State p a) : merge (embed s) next = embed next := by
 cases s with | mk machine values request writes origins waiting => cases waiting <;> rfl

theorem owner_preserved (waiting : s.waiting = some (.inr saved)) :
 (merge s next).waiting = some (.inr saved) := by simp [merge,ownerWaiting,waiting]

structure Outcome (p a : Nat) where
 state : State p a
 status : ReferenceSource.Status

def fromPure (s : State p a) (result : MixedReferenceSource.Outcome p a) : Outcome p a :=
 ⟨merge s result.state,result.status⟩
def advancePure (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located) : Outcome p a :=
 if (ownerWaiting s.waiting).isSome then ⟨s,.failed .awaiting⟩ else
 fromPure s (MixedReferenceSource.advance (base s) member place principal item)
def completePure (s : State p a) : Outcome p a :=
 if (ownerWaiting s.waiting).isSome then ⟨s,.failed .awaiting⟩ else
 fromPure s (MixedReferenceSource.complete (base s))
def cancelPure (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) : Outcome p a :=
 if (ownerWaiting s.waiting).isSome then ⟨s,.failed .rejected⟩ else
 fromPure s (MixedReferenceSource.cancel (base s) member place principal)

def authorityHead (s : State p a) (view : WorldProjection.AuthorityView a) : Option (State p a) :=
 (MixedReferenceAuthority.install (base s) view).map (merge s)
def controlInput (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
 (raw : MixedCompositionCore.Raw) : Option (State p a × Option Nat) :=
 (MixedReferenceSource.controlInput (base s) member place principal raw).map fun pair => (merge s pair.1,pair.2)

-- operationName is the checked compiler/deployment binding, not new user-written
-- send/receipt syntax. Its source-to-catalog introduction must be established by
-- the enclosing program/deployment transition before claiming full owner source.
-- activation/ordinal/control come from that SAME retained Session, not a message.
def issue (s : State p a) (ctx : Fields) (labels : List (String × Nat))
 (member : Fin a) (principal activation ordinal control : Nat)
 (operationName : String) (statement : Assignment) : Option (State p a × OwnerWaiting) := do
 if s.waiting.isSome || !s.machine.store.core.pending.isEmpty then none else do
 let plan ← MixedOwnerSourceFootprint.elaborate ctx s.values labels control statement
 if !MixedOwnerMaterialization.sourcePayloadCheck plan then none else do
 let raw ← instanceKey s.values operationName
 let key ← CompositionCore.index s.machine.store.core.system.configuration.count raw
 let .owner definition := MixedCatalogService.operationDefinition s.machine.store.core.system.configuration.state key | none
 if definition != plan.operation then none else do
 let place ← CompositionCore.index p (MixedOperationDefinitions.ownerAt definition.contract definition.body.target)
 let origin : OwnerEffectService.Origin := ⟨statement.site.document,statement.site.byteOffset,activation,ordinal,control⟩
 let (machine,saved) ← MixedReferenceExecution.startOwner s.machine member place principal s.nextRequest key.val origin
   (MixedOwnerMaterialization.sourceArguments plan)
 let waiting : OwnerWaiting := ⟨saved,statement,(parameters statement.rhs).eraseDups.map (MixedReferenceSource.read (base s)),s.machine.store.events.length,MixedReferenceSource.read (base s) operationName⟩
 return (⟨machine,s.values,s.nextRequest+1,s.writes,
   ⟨some statement.site,.request,s.machine.store.events.length,machine.store.events.length⟩::s.origins,
   some (.inr waiting)⟩,waiting)

-- Independent source/capture elaboration is retained alongside the actual lower
-- request transition. No claim of service, ack, catalog installation or issuance
-- authority for supplied source context is made from this issue-only wrapper.
theorem issue_parts {s next : State p a} (control : Nat)
 (accepted : issue s ctx labels member principal activation ordinal control operationName statement = some (next,waiting)) :
 s.waiting = none ∧ s.machine.store.core.pending = [] ∧
 ∃ plan, ∃ (key : Fin s.machine.store.core.system.configuration.count) (place : Fin p),
 MixedOwnerSourceFootprint.Elaborates ctx s.values labels control statement plan ∧
 MixedOwnerMaterialization.sourcePayloadCheck plan = true ∧
 instanceKey s.values operationName = some key.val ∧
 MixedCatalogService.operationDefinition s.machine.store.core.system.configuration.state key = .owner plan.operation ∧
 place.val = MixedOperationDefinitions.ownerAt plan.operation.contract plan.operation.body.target ∧
 MixedReferenceExecution.startOwner s.machine member place principal s.nextRequest key.val
   ⟨statement.site.document,statement.site.byteOffset,activation,ordinal,control⟩
   (MixedOwnerMaterialization.sourceArguments plan) = some (next.machine,waiting.saved) ∧
 next.waiting = some (.inr waiting) ∧ next.nextRequest = s.nextRequest+1 ∧
 next.values = s.values ∧ next.writes = s.writes ∧ waiting.statement = statement ∧
 waiting.operationDependency = MixedReferenceSource.read (base s) operationName ∧
 waiting.dependencies = (parameters statement.rhs).eraseDups.map (MixedReferenceSource.read (base s)) := by
 unfold issue at accepted
 split at accepted
 · cases accepted
 · rename_i guard
   have idle : s.waiting = none ∧ s.machine.store.core.pending = [] := by
     cases w : s.waiting <;> cases c : s.machine.store.core.pending <;> simp_all
   cases hp : MixedOwnerSourceFootprint.elaborate ctx s.values labels control statement with
   | none => simp [hp] at accepted
   | some plan =>
     simp only [hp,Option.bind_eq_bind,Option.bind_some] at accepted
     split at accepted
     · cases accepted
     · rename_i payload
       have payloadOk : MixedOwnerMaterialization.sourcePayloadCheck plan = true := by simpa using payload
       cases hn : instanceKey s.values operationName with
       | none => simp [hn] at accepted
       | some raw =>
         simp only [hn,Option.bind_some] at accepted
         cases hk : CompositionCore.index s.machine.store.core.system.configuration.count raw with
         | none => simp [hk] at accepted
         | some key =>
           simp only [hk,Option.bind_some] at accepted
           cases hd : MixedCatalogService.operationDefinition s.machine.store.core.system.configuration.state key with
           | pure _ => simp [hd] at accepted
           | owner definition =>
             simp only [hd] at accepted
             split at accepted
             · cases accepted
             · rename_i equalDefinition
               have defEq : definition = plan.operation := by simpa using equalDefinition
               subst definition
               cases hl : CompositionCore.index p (MixedOperationDefinitions.ownerAt plan.operation.contract plan.operation.body.target) with
               | none => simp [hl] at accepted
               | some place =>
                 simp only [hl,Option.bind_some] at accepted
                 cases run : MixedReferenceExecution.startOwner s.machine member place principal s.nextRequest key.val
                   ⟨statement.site.document,statement.site.byteOffset,activation,ordinal,control⟩
                   (MixedOwnerMaterialization.sourceArguments plan) with
                 | none => simp [run] at accepted
                 | some pair =>
                   obtain ⟨machine,saved⟩ := pair
                   simp only [run,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at accepted
                   obtain ⟨rfl,rfl⟩ := accepted
                   exact ⟨idle.1,idle.2,plan,key,place,(MixedOwnerSourceFootprint.elaborate_exact control).mp hp,payloadOk,
                     by simpa [CompositionCore.index_sound _ hk] using hn,hd,CompositionCore.index_sound _ hl,run,rfl,rfl,rfl,rfl,rfl,rfl,rfl⟩

-- All local value reads used by issue are retained: the compiler-generated
-- callable binding separately, plus the ordered stable-name capture reads.
-- This does not assert authenticated lexical provenance for an arbitrary state.
theorem issue_dependencies {s next : State p a} (control : Nat)
 (accepted : issue s ctx labels member principal activation ordinal control operationName statement = some (next,waiting)) :
 waiting.operationDependency = MixedReferenceSource.read (base s) operationName ∧
 waiting.dependencies = (parameters statement.rhs).eraseDups.map (MixedReferenceSource.read (base s)) := by
 obtain ⟨_,_,_,_,_,_,_,_,_,_,_,_,_,_,_,_,binding,captures⟩ := issue_parts control accepted
 exact ⟨binding,captures⟩

-- Lift the actual start to its core transition, so allocator preservation does
-- not posit a second allocator or merely assume freshness of the result.
theorem start_core (m : MixedReferenceExecution.Machine p a)
 (accepted : MixedReferenceExecution.startOwner m member place principal id key origin args = some (next,saved)) :
 MixedRequestCore.startOwner m.store.core member place principal id key origin args = some (next.store.core,saved) := by
 have run := (MixedReferenceExecution.startOwner_parts _ _ _ _ _ _ _ _ _ accepted).1
 unfold MixedReferenceStore.startOwner at run
 cases coreRun : MixedRequestCore.startOwner m.store.core member place principal id key origin args with
 | none => simp [coreRun] at run
 | some pair =>
   obtain ⟨core,payload⟩ := pair
   simp only [coreRun,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at run
   obtain ⟨same,rfl⟩ := run
   have coreEq := congrArg MixedReferenceStore.Machine.core same
   exact congrArg (fun c => some (c,payload)) coreEq

theorem start_below (m : MixedReferenceExecution.Machine p a)
 (valid : MixedReferenceAllocation.Below m id)
 (accepted : MixedReferenceExecution.startOwner m member place principal id key origin args = some (next,saved)) :
 MixedReferenceAllocation.Below next (id+1) :=
 MixedRequestAllocation.startOwner_below _ _ _ _ _ _ _ _ _ valid (start_core m accepted)

theorem issue_preserves {s next : State p a} (control : Nat)
 (valid : MixedReferenceExecution.Invariant s.machine)
 (bound : MixedReferenceAllocation.Below s.machine s.nextRequest)
 (accepted : issue s ctx labels member principal activation ordinal control operationName statement = some (next,waiting)) :
 MixedReferenceExecution.Invariant next.machine ∧
 MixedReferenceAllocation.Below next.machine next.nextRequest ∧
 next.machine.store.core.pending = [.owner waiting.saved] ∧
 next.machine.pending = [] ∧ next.waiting = some (.inr waiting) := by
 obtain ⟨_,drained,plan,key,place,_,_,_,_,_,run,wait,count,rest⟩ := issue_parts control accepted
 have parts := MixedReferenceExecution.startOwner_parts _ _ _ _ _ _ _ _ _ run
 refine ⟨MixedReferenceExecution.startOwner_preserves _ _ _ _ _ _ _ _ _ valid run,?_,?_,?_,wait⟩
 · rw [count]; exact start_below _ bound run
 · simpa [drained] using parts.2.2.1
 · have empty : s.machine.pending = [] := by
     have h := valid.2
     simp only [drained,MixedPendingProjection.tickets] at h
     exact List.map_eq_nil_iff.mp h
   simpa [empty] using parts.2.1

theorem advance_embed (s : MixedReferenceSource.State p a) :
 advancePure (embed s) member place principal item =
 ⟨embed (MixedReferenceSource.advance s member place principal item).state,
 (MixedReferenceSource.advance s member place principal item).status⟩ := by
 have owner : ownerWaiting (embed s).waiting = none := by cases h : s.waiting <;> simp [embed,ownerWaiting,h]
 simp [advancePure,owner,fromPure,base_embed,merge_embed]

theorem complete_embed (s : MixedReferenceSource.State p a) :
 completePure (embed s) = ⟨embed (MixedReferenceSource.complete s).state,(MixedReferenceSource.complete s).status⟩ := by
 have owner : ownerWaiting (embed s).waiting = none := by cases h : s.waiting <;> simp [embed,ownerWaiting,h]
 simp [completePure,owner,fromPure,base_embed,merge_embed]

theorem cancel_embed (s : MixedReferenceSource.State p a) :
 cancelPure (embed s) member place principal =
 ⟨embed (MixedReferenceSource.cancel s member place principal).state,(MixedReferenceSource.cancel s member place principal).status⟩ := by
 have owner : ownerWaiting (embed s).waiting = none := by cases h : s.waiting <;> simp [embed,ownerWaiting,h]
 simp [cancelPure,owner,fromPure,base_embed,merge_embed]

theorem pure_refuses_owner (s : State p a) (held : s.waiting = some (.inr waiting)) :
 (advancePure s member place principal item).state = s ∧
 (completePure s).state = s ∧ (cancelPure s member place principal).state = s := by
 simp [advancePure,completePure,cancelPure,ownerWaiting,held]

-- This positive existence theorem uses an independent current-use judgment;
-- it does not assume successful startOwner or the desired source result.
theorem start_exists (m : MixedReferenceExecution.Machine p a)
 (key : Fin m.store.core.system.configuration.count) (place : Fin p) (id : Nat)
 (bound : MixedReferenceAllocation.Below m id)
 (kind : MixedCatalogService.operationDefinition m.store.core.system.configuration.state key = .owner definition)
 (use : CurrentUse.CurrentUse (MixedCatalogService.world m.store.core.system.configuration.state m.store.core.system.view)
   {MixedPureInvocation.request m.store.core.system.configuration.state m.store.core.system.view member key place principal id 0 with arguments := args}) :
 ∃ next saved, MixedReferenceExecution.startOwner m member place principal id key.val origin args = some (next,saved) := by
 obtain ⟨saved,prepared⟩ := MixedRequestCore.prepareOwner_complete (origin:=origin) kind use
 have fresh := MixedReferenceAllocation.fresh_bound m id m.store.core.system.configuration.state.realm principal bound
 have exactId := MixedRequestCore.prepareOwner_id prepared
 have core : MixedRequestCore.startOwner m.store.core member place principal id key.val origin args =
   some (MixedRequestCore.enqueue m.store.core (.owner saved),saved) := by
   simp [MixedRequestCore.startOwner,CompositionCore.index_roundtrip,prepared,exactId,fresh]
 simp only [MixedReferenceExecution.startOwner,MixedReferenceStore.startOwner,core,
   Option.bind_eq_bind,Option.bind_some,Option.pure_def]
 exact ⟨_,saved,rfl⟩

theorem issue_exists {s : State p a} (control : Nat) (plan : Plan)
 (key : Fin s.machine.store.core.system.configuration.count) (place : Fin p)
 (idle : s.waiting = none) (drained : s.machine.store.core.pending = [])
 (bound : MixedReferenceAllocation.Below s.machine s.nextRequest)
 (typed : MixedOwnerSourceFootprint.Elaborates ctx s.values labels control statement plan)
 (payload : MixedOwnerMaterialization.sourcePayloadCheck plan = true)
 (binding : instanceKey s.values operationName = some key.val)
 (kind : MixedCatalogService.operationDefinition s.machine.store.core.system.configuration.state key = .owner plan.operation)
 (atOwner : place.val = MixedOperationDefinitions.ownerAt plan.operation.contract plan.operation.body.target)
 (use : CurrentUse.CurrentUse (MixedCatalogService.world s.machine.store.core.system.configuration.state s.machine.store.core.system.view)
   {MixedPureInvocation.request s.machine.store.core.system.configuration.state s.machine.store.core.system.view member key place principal s.nextRequest 0
    with arguments := MixedOwnerMaterialization.sourceArguments plan}) :
 ∃ next waiting, issue s ctx labels member principal activation ordinal control operationName statement = some (next,waiting) := by
 have elaborated := (MixedOwnerSourceFootprint.elaborate_exact control).mpr typed
 have located : CompositionCore.index p (MixedOperationDefinitions.ownerAt plan.operation.contract plan.operation.body.target) = some place := by
   rw [←atOwner]; exact CompositionCore.index_roundtrip _
 obtain ⟨machine,saved,run⟩ := start_exists s.machine key place s.nextRequest bound kind
   (origin:=⟨statement.site.document,statement.site.byteOffset,activation,ordinal,control⟩) use
 simp [issue,idle,drained,elaborated,payload,binding,CompositionCore.index_roundtrip,kind,located,run]

theorem start_saved (m : MixedReferenceExecution.Machine p a)
 (key : Fin m.store.core.system.configuration.count)
 (kind : MixedCatalogService.operationDefinition m.store.core.system.configuration.state key = .owner definition)
 (accepted : MixedReferenceExecution.startOwner m member place principal id key.val origin args = some (next,saved)) :
 ∃ evidence, saved = OwnerSavedPending.save ⟨origin,
   {MixedPureInvocation.request m.store.core.system.configuration.state m.store.core.system.view member key place principal id 0 with arguments := args},
   evidence,definition.body⟩ := by
 have run := start_core m accepted
 unfold MixedRequestCore.startOwner at run
 simp only [CompositionCore.index_roundtrip,Option.bind_eq_bind,Option.bind_some] at run
 cases hp : MixedRequestCore.prepareOwner m.store.core.system.configuration.state m.store.core.system.view member key place principal id origin args with
 | none => simp [hp] at run
 | some kept =>
   simp only [hp,Option.bind_some] at run
   split at run
   · obtain ⟨_,rfl⟩ := (Prod.mk.inj (Option.some.inj run))
     obtain ⟨d,e,hk,_,eq⟩ := MixedRequestCore.prepareOwner_parts hp
     have deq : d = definition := by simpa [kind] using hk.symm
     exact ⟨e,by simpa [deq] using eq⟩
   · cases run

theorem issue_payload {s next : State p a} (control : Nat)
 (accepted : issue s ctx labels member principal activation ordinal control operationName statement = some (next,waiting)) :
 ∃ plan, MixedOwnerSourceFootprint.Elaborates ctx s.values labels control statement plan ∧
 waiting.saved.body = plan.operation.body ∧
 waiting.saved.request.arguments = MixedOwnerMaterialization.sourceArguments plan ∧
 waiting.saved.origin = ⟨statement.site.document,statement.site.byteOffset,activation,ordinal,control⟩ ∧
 waiting.saved.request.arguments.length = plan.operation.contract.arguments.length ∧
 (∀ arg ∈ waiting.saved.request.arguments, MixedOwnerMaterialization.IntegerArgument arg) ∧
 plan.operation.contract.control = control := by
 obtain ⟨_,_,plan,key,place,typed,bounded,_,kind,_,run,_⟩ := issue_parts control accepted
 obtain ⟨e,eq⟩ := start_saved s.machine key kind run
 have layout := MixedOwnerSourceFootprint.source_payload control
   ((MixedOwnerSourceFootprint.elaborate_exact control).mpr typed)
 refine ⟨plan,typed,?_,?_,?_,?_,?_,layout.2.2.2.1⟩
 · rw [eq]; rfl
 · rw [eq]; rfl
 · rw [eq]; rfl
 · rw [eq,layout.2.2.2.2.1]
   simp [OwnerSavedPending.save,OwnerSavedPending.saveRequest,MixedOwnerMaterialization.sourceArguments]
 · rw [eq]
   exact MixedOwnerMaterialization.source_arguments_range bounded

-- The source receives only a unit acknowledgment. No owner value is returned
-- or inserted into a local source binding. Actual receipt provenance is a
-- property of the enclosing issue/service/inbox trace, not raw construction.
def acknowledge (s : State p a) (received : OwnerSavedPending.Saved) (evidence : CurrentUse.Evidence) : Option (State p a) := do
 let waiting ← ownerWaiting s.waiting
 let machine ← MixedReferenceExecution.finishOwner s.machine waiting.saved received evidence
 return {s with machine := machine,waiting := none, origins := ⟨some waiting.statement.site,.result,s.machine.store.events.length,machine.store.events.length⟩ :: s.origins}

theorem acknowledge_parts {s next : State p a}
 (accepted : acknowledge s received evidence = some next) :
 ∃ waiting, s.waiting = some (.inr waiting) ∧
 MixedReferenceExecution.finishOwner s.machine waiting.saved received evidence = some next.machine ∧
 next = {s with machine := next.machine,waiting := none, origins := ⟨some waiting.statement.site,.result,s.machine.store.events.length,next.machine.store.events.length⟩ :: s.origins} := by
 unfold acknowledge at accepted
 cases w : s.waiting with
 | none => simp [w,ownerWaiting] at accepted
 | some entry =>
   cases entry with
   | inl pure => simp [w,ownerWaiting] at accepted
   | inr waiting =>
     simp only [w,ownerWaiting,Option.bind_eq_bind,Option.bind_some] at accepted
     cases run : MixedReferenceExecution.finishOwner s.machine waiting.saved received evidence with
     | none => simp [run] at accepted
     | some machine =>
       simp only [run,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
       subst next
       exact ⟨waiting,rfl,run,rfl⟩

theorem acknowledge_preserves {s next : State p a}
 (valid : MixedReferenceExecution.Invariant s.machine)
 (bound : MixedReferenceAllocation.Below s.machine s.nextRequest)
 (accepted : acknowledge s received evidence = some next) :
 MixedReferenceExecution.Invariant next.machine ∧
 MixedReferenceAllocation.Below next.machine next.nextRequest ∧
 next.waiting = none ∧ next.values = s.values ∧ next.writes = s.writes ∧ next.nextRequest = s.nextRequest := by
 obtain ⟨waiting,_,run,eq⟩ := acknowledge_parts accepted
 have allocator := MixedReferenceAllocation.finishOwner_below _ bound run
 have count := congrArg State.nextRequest eq
 exact ⟨MixedReferenceExecution.finishOwner_preserves _ valid run,by simpa only [count] using allocator,
   congrArg State.waiting eq,by simpa only using congrArg (fun t : State p a => t.values) eq,by simpa only using congrArg (fun t : State p a => t.writes) eq,count⟩

theorem double_acknowledge_refused {s next : State p a}
 (accepted : acknowledge s received evidence = some next) : acknowledge next later proof = none := by
 obtain ⟨_,_,_,eq⟩ := acknowledge_parts accepted
 have cleared := congrArg State.waiting eq
 simp [acknowledge,ownerWaiting,cleared]

theorem pure_acknowledge_refused (s : MixedReferenceSource.State p a) :
 acknowledge (embed s) received evidence = none := by
 cases w : s.waiting <;> simp [acknowledge,embed,ownerWaiting,w]

#print axioms issue_dependencies
#print axioms acknowledge_parts
#print axioms acknowledge_preserves
#print axioms double_acknowledge_refused
#print axioms pure_acknowledge_refused
#print axioms start_exists
#print axioms issue_exists
#print axioms start_saved
#print axioms issue_payload
#print axioms start_core
#print axioms start_below
#print axioms issue_preserves
#print axioms advance_embed
#print axioms complete_embed
#print axioms cancel_embed
#print axioms pure_refuses_owner
#print axioms base_embed
#print axioms owner_preserved
#print axioms issue_parts
end MirroreaProofFirst.MixedOwnerSourceIssue
