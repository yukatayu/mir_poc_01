import OwnerMetadataRegistry
import MixedManagementEntry
import MixedCatalogService
namespace MirroreaProofFirst.OwnerMetadataManagement
open OwnerMetadataRegistry OwnerSourceContext

-- Successor connection candidate: genuine existing management actor/view/cut,
-- no hand-created invocation World or synthetic action22 operation record.
-- Schema-to-source attachment is still an explicit compiler/installation input.
structure State (p a : Nat) where
 management : MixedManagementEntry.System p a
 metadata : Registry

-- Current owner/module identity and code are recovered from the SAME actual
-- configuration used by the existing management boundary. Names are not auth.
def Common (s : State p a) (place : Fin p) : Prop :=
 s.metadata.instanceId = s.management.configuration.state.realm ∧
 s.metadata.ownerKey = (WorldProjection.encode (a:=a) (n:=s.management.configuration.count) (.locus place)).val ∧
 s.metadata.ownerIdentity = ⟨.locus,s.management.configuration.state.placeIncarnation place,0⟩ ∧
 s.management.configuration.state.participating place = true

instance : Decidable (Common s place) := by unfold Common; infer_instance

def ModuleMatches (s : State p a) (place : Fin p) (key : Fin s.management.configuration.count) : Prop :=
 s.metadata.moduleKey = (WorldProjection.encode (a:=a) (p:=p) (.moduleSlot key)).val ∧
 s.metadata.moduleIdentity = ⟨.module,1,(s.management.configuration.state.instances key).revision⟩ ∧
 s.metadata.code = (s.management.configuration.state.instances key).definition.val ∧
 s.metadata.contract = (s.management.configuration.state.instances key).definition.val ∧
 place ∈ (s.management.configuration.state.instances key).placements
instance : Decidable (ModuleMatches s place key) := by unfold ModuleMatches; infer_instance

def Attached (s : State p a) (place : Fin p) : Prop :=
 Common s place ∧ ∃ key, ModuleMatches s place key ∧
 Support.Grounded (MixedInstanceState.snapshot s.management.configuration.state).forms
  (fun k => (MixedInstanceState.snapshot s.management.configuration.state).eligible k = true) key

def attachmentCheck (s : State p a) (place : Fin p) : Bool :=
 decide (Common s place) && (List.finRange s.management.configuration.count).any fun key =>
   decide (ModuleMatches s place key) && Support.snapshotLive (MixedInstanceState.snapshot s.management.configuration.state) key

theorem attachment_exact : attachmentCheck s place = true ↔ Attached s place := by
 simp [attachmentCheck,Attached,List.any_eq_true,Support.snapshot_live_exact]

structure Context where
 use : CurrentUse.Context
 memberIdentity : CurrentUse.RecordIdentity
 ownerName : String
 ownerIdentity : CurrentUse.RecordIdentity
 moduleIdentity : CurrentUse.RecordIdentity
 schema : Schema
 managementCut : Nat
 registryCut : Nat
 previous : Slot
 change : Change
 deriving DecidableEq, Repr

-- Claims use action-specific module-control targets. The entire structured key,
-- requested classification and schema remain in the exact payload context;
-- policy leaves do not silently gain arbitrary argument predicates.
def context (s : State p a) (member : Fin a) (place : Fin p) (principal requestId : Nat) (change : Change) : Context :=
 {use :=
   {principal := principal,member := member.val,
    memberIncarnation := (s.management.view.members member).incarnation,
    locus := place.val,locusIncarnation := s.management.configuration.state.placeIncarnation place,
    moduleKey := s.metadata.moduleKey,moduleIncarnation := s.metadata.moduleIdentity.incarnation,
    action := change.action,target := s.metadata.moduleKey,
    targetIncarnation := s.metadata.moduleIdentity.incarnation,targetRevision := s.metadata.moduleIdentity.revision,
    instanceId := s.management.configuration.state.realm,request := requestId,arguments := [],
    code := s.metadata.code,contract := s.metadata.contract,generation := s.management.view.generation},
  memberIdentity := ⟨.member,(s.management.view.members member).incarnation,(s.management.view.members member).revision⟩,
  ownerName := s.metadata.ownerName,ownerIdentity := s.metadata.ownerIdentity,moduleIdentity := s.metadata.moduleIdentity,
  schema := s.metadata.schema,managementCut := s.management.serial,registryCut := s.metadata.serial,
  previous := slot s.metadata change.key,change := change}

structure Evidence where
 context : Context
 policy : Nat
 version : Nat
 witness : CurrentUse.Witness
 deriving DecidableEq, Repr

def Allowed (s : State p a) (member : Fin a) (place : Fin p) (principal requestId : Nat) (change : Change) : Prop :=
 MixedManagementEntry.CurrentActor s.management member place principal ∧ Attached s place ∧
 CurrentUse.Authorized s.management.view.authority (context s member place principal requestId change).use
  (s.management.controlPolicy change.action).label (s.management.controlPolicy change.action).expression

def check (s : State p a) (member : Fin a) (place : Fin p) (principal requestId : Nat)
 (change : Change) (evidence : Evidence) : Bool :=
 MixedManagementEntry.actorCheck s.management member place principal && attachmentCheck s place &&
 decide (evidence.context = context s member place principal requestId change) &&
 decide (evidence.policy = (s.management.controlPolicy change.action).id) &&
 decide (evidence.version = (s.management.controlPolicy change.action).version) &&
 CurrentUse.checkWitness s.management.view.authority (context s member place principal requestId change).use
  (s.management.controlPolicy change.action).label (s.management.controlPolicy change.action).expression evidence.witness

theorem check_sound (accepted : check s member place principal requestId change evidence = true) :
 Allowed s member place principal requestId change ∧ evidence.context = context s member place principal requestId change := by
 simp only [check,Bool.and_eq_true,decide_eq_true_eq,MixedManagementEntry.actor_exact,attachment_exact] at accepted
 obtain ⟨⟨⟨⟨⟨actor,attached⟩,same⟩,_⟩,_⟩,witness⟩ := accepted
 exact ⟨⟨actor,attached,CurrentUse.checkWitness_sound _ _ _ _ _ witness⟩,same⟩

def authorize (s : State p a) (member : Fin a) (place : Fin p) (principal requestId : Nat) (change : Change) : Option Evidence :=
 if MixedManagementEntry.actorCheck s.management member place principal && attachmentCheck s place then
 (CurrentUse.produce s.management.view.authority (context s member place principal requestId change).use
   (s.management.controlPolicy change.action).label (s.management.controlPolicy change.action).expression).map fun witness =>
   ⟨context s member place principal requestId change,(s.management.controlPolicy change.action).id,
    (s.management.controlPolicy change.action).version,witness⟩
 else none

theorem authorize_sound (produced : authorize s member place principal requestId change = some evidence) :
 check s member place principal requestId change evidence = true := by
 unfold authorize at produced
 split at produced
 · rename_i available
   have pair : MixedManagementEntry.actorCheck s.management member place principal = true ∧ attachmentCheck s place = true := by
     simpa using available
   cases issued : CurrentUse.produce s.management.view.authority (context s member place principal requestId change).use
     (s.management.controlPolicy change.action).label (s.management.controlPolicy change.action).expression with
   | none => simp [issued] at produced
   | some witness =>
     simp only [issued,Option.map_some,Option.some.injEq] at produced
     subst evidence
     simp [check,pair.1,pair.2,CurrentUse.produce_checked _ _ _ _ _ issued]
 · cases produced

theorem authorize_complete (allowed : Allowed s member place principal requestId change) :
 ∃ evidence, authorize s member place principal requestId change = some evidence := by
 obtain ⟨witness,issued⟩ := CurrentUse.produce_complete _ _ _ _ allowed.2.2
 refine ⟨⟨context s member place principal requestId change,(s.management.controlPolicy change.action).id,
   (s.management.controlPolicy change.action).version,witness⟩,?_⟩
 simp [authorize,(MixedManagementEntry.actor_exact _ _ _ _).mpr allowed.1,attachment_exact.mpr allowed.2.1,issued]

-- One realm/principal/request lock is shared with the real management path.
-- Registry.used is retained as historical producer data, never an alternative
-- authorization or freshness gate.
def useId (s : State p a) (principal requestId : Nat) : CurrentUse.UseId :=
 MixedManagementEntry.useId s.management principal requestId

def after (s : State p a) (principal requestId : Nat) (change : Change) (next : Slot) : State p a :=
 {management := MixedManagementEntry.after s.management s.management.configuration principal requestId,
  metadata := {setSlot s.metadata change.key next with used := useId s principal requestId :: s.metadata.used}}

def commit (s : State p a) (member : Fin a) (place : Fin p) (principal requestId : Nat)
 (change : Change) (evidence : Evidence) : Option (State p a) := do
 if s.management.used.contains (useId s principal requestId) || !check s member place principal requestId change evidence then none else do
 let next ← prepare s.metadata change
 return after s principal requestId change next

theorem commit_parts (accepted : commit s member place principal requestId change evidence = some result) :
 useId s principal requestId ∉ s.management.used ∧ Allowed s member place principal requestId change ∧
 evidence.context = context s member place principal requestId change ∧
 ∃ next, Prepares s.metadata change next ∧ result = after s principal requestId change next := by
 unfold commit at accepted
 split at accepted
 · cases accepted
 · rename_i gate
   have gates : useId s principal requestId ∉ s.management.used ∧ check s member place principal requestId change evidence = true := by
     simpa using gate
   cases prepared : prepare s.metadata change with
   | none => simp [prepared] at accepted
   | some next =>
     simp only [prepared,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
     exact ⟨gates.1,(check_sound gates.2).1,(check_sound gates.2).2,next,prepare_exact.mp prepared,accepted.symm⟩

theorem commit_complete (fresh : useId s principal requestId ∉ s.management.used)
 (allowed : Allowed s member place principal requestId change) (rule : Prepares s.metadata change next) :
 ∃ evidence, commit s member place principal requestId change evidence = some (after s principal requestId change next) := by
 obtain ⟨evidence,produced⟩ := authorize_complete allowed
 exact ⟨evidence,by simp [commit,fresh,authorize_sound produced,prepare_exact.mpr rule]⟩

def perform (s : State p a) (member : Fin a) (place : Fin p) (principal requestId : Nat) (change : Change) : Option (State p a) := do
 let evidence ← authorize s member place principal requestId change
 commit s member place principal requestId change evidence

theorem perform_complete (fresh : useId s principal requestId ∉ s.management.used)
 (allowed : Allowed s member place principal requestId change) (rule : Prepares s.metadata change next) :
 perform s member place principal requestId change = some (after s principal requestId change next) := by
 obtain ⟨evidence,produced⟩ := authorize_complete allowed
 simp [perform,produced,commit,fresh,authorize_sound produced,prepare_exact.mpr rule]

-- Existing controls mutate their genuine configuration/view/serial, sharing the
-- same lock. They cannot silently keep metadata current: usability below also
-- rechecks current locus/module identity, placement, code and support.
def manage (s : State p a) (member : Fin a) (place : Fin p) (principal requestId : Nat)
 (raw : MixedCompositionCore.Raw) (evidence : MixedManagementEntry.Evidence) : Option (State p a × Option Nat) := do
 let (next,created) ← MixedManagementEntry.commit s.management member place principal requestId raw evidence
 return ({s with management := next},created)

theorem manage_parts (accepted : manage s member place principal requestId raw evidence = some result) :
 ∃ next created, MixedManagementEntry.commit s.management member place principal requestId raw evidence = some (next,created) ∧
 result = ({s with management := next},created) := by
 unfold manage at accepted
 cases done : MixedManagementEntry.commit s.management member place principal requestId raw evidence with
 | none => simp [done] at accepted
 | some next =>
   obtain ⟨system,created⟩ := next
   simp only [done,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
   exact ⟨system,created,rfl,accepted.symm⟩

-- This raw head transition is still a trusted-input boundary. No theorem here
-- authenticates who may supply a new authority view.
def installAuthorityHead (s : State p a) (view : WorldProjection.AuthorityView a) : State p a :=
 {s with management := MixedManagementEntry.installAuthorityHead s.management view}

def Invariant (s : State p a) : Prop :=
 MixedManagementEntry.Invariant s.management ∧ Consistent s.metadata ∧
 ∀ request, request ∈ s.metadata.used → request ∈ s.management.used

theorem prepared_update_consistent (valid : Consistent s) (rule : Prepares s change next) :
 Consistent (setSlot s change.key next) := by
 intro key metadata found
 by_cases same : key = change.key
 · subst key
   have currentNext : next.current = some metadata := by
     change (slot (setSlot s change.key next) change.key).current = _ at found
     simpa only [set_same] using found
   have correct := prepared_consistent rule currentNext
   simpa only [setSlot,slot,List.lookup_cons,beq_self_eq_true,cond_true,Option.getD_some] using correct
 · have retained := set_other (s:=s) (value:=next) same
   have oldFound : current s key = some metadata := by
     change (slot (setSlot s change.key next) key).current = _ at found
     rw [retained] at found
     exact found
   have correct := valid key metadata oldFound
   change Declared s.schema key metadata.declaration ∧ metadata.declaration.owner = s.ownerName ∧
     metadata.declaration.typeName = "Int" ∧ metadata.generation = (slot (setSlot s change.key next) key).generation ∧
     metadata.label = (slot (setSlot s change.key next) key).labelFloor
   rw [retained]
   exact correct

theorem commit_preserves (valid : Invariant s)
 (accepted : commit s member place principal requestId change evidence = some result) : Invariant result := by
 obtain ⟨fresh,_,_,next,rule,rfl⟩ := commit_parts accepted
 refine ⟨⟨valid.1.1,List.nodup_cons.mpr ⟨fresh,valid.1.2⟩⟩,prepared_update_consistent valid.2.1 rule,?_⟩
 intro request present
 have cases : request = useId s principal requestId ∨ request ∈ s.metadata.used := by simpa [after] using present
 rcases cases with rfl | old
 · simp [after,MixedManagementEntry.after,useId]
 · exact List.mem_cons_of_mem _ (valid.2.2 request old)

theorem manage_preserves (valid : Invariant s)
 (accepted : manage s member place principal requestId raw evidence = some result) : Invariant result.1 := by
 obtain ⟨system,created,done,rfl⟩ := manage_parts accepted
 have nextValid := (MixedManagementEntry.commit_preserves _ _ _ _ _ _ _ _ valid.1 done).1
 obtain ⟨_,cfg,out,_,equal⟩ := MixedManagementEntry.commit_parts _ _ _ _ _ _ _ _ done
 cases equal
 exact ⟨nextValid,valid.2.1,fun request present => List.mem_cons_of_mem _ (valid.2.2 request present)⟩

theorem head_preserves (valid : Invariant s) : Invariant (installAuthorityHead s view) := valid

theorem commit_generation (accepted : commit s member place principal requestId change evidence = some result) :
 (slot result.metadata change.key).generation = (slot s.metadata change.key).generation+1 := by
 obtain ⟨_,_,_,next,rule,rfl⟩ := commit_parts accepted
 change (slot (setSlot s.metadata change.key next) change.key).generation = _
 rw [set_same]
 exact prepared_strict_generation rule

theorem commit_floor (accepted : commit s member place principal requestId change evidence = some result) :
 (slot s.metadata change.key).labelFloor ≤ (slot result.metadata change.key).labelFloor := by
 obtain ⟨_,_,_,next,rule,rfl⟩ := commit_parts accepted
 change _ ≤ (slot (setSlot s.metadata change.key next) change.key).labelFloor
 rw [set_same]
 exact prepared_label_floor rule

theorem commit_other_key (accepted : commit s member place principal requestId change evidence = some result)
 (different : key ≠ change.key) : current result.metadata key = current s.metadata key := by
 obtain ⟨_,_,_,next,_,rfl⟩ := commit_parts accepted
 exact congrArg Slot.current (set_other (s:=s.metadata) (value:=next) different)

theorem metadata_then_metadata_refused (accepted : commit s member place principal requestId change evidence = some result) :
 commit result otherMember otherPlace principal requestId other otherEvidence = none := by
 obtain ⟨_,_,_,next,_,rfl⟩ := commit_parts accepted
 simp [commit,after,useId,MixedManagementEntry.useId,MixedManagementEntry.after]

theorem metadata_then_management_refused (accepted : commit s member place principal requestId change evidence = some result) :
 manage result otherMember otherPlace principal requestId raw otherEvidence = none := by
 obtain ⟨_,_,_,next,_,rfl⟩ := commit_parts accepted
 simp [manage,MixedManagementEntry.commit,after,useId,MixedManagementEntry.useId,MixedManagementEntry.after]

theorem management_then_metadata_refused (accepted : manage s member place principal requestId raw evidence = some result) :
 commit result.1 otherMember otherPlace principal requestId other otherEvidence = none := by
 obtain ⟨system,created,done,rfl⟩ := manage_parts accepted
 obtain ⟨_,cfg,out,run,equal⟩ := MixedManagementEntry.commit_parts _ _ _ _ _ _ _ _ done
 cases equal
 have realm : cfg.state.realm = s.management.configuration.state.realm := MixedCompositionCore.run_realm _ _ _ run
 simp [commit,useId,MixedManagementEntry.after,MixedManagementEntry.useId,realm]

theorem changed_context_refused (different : evidence.context ≠ context s member place principal requestId change) :
 check s member place principal requestId change evidence = false := by simp [check,different]

theorem changed_payload_refused (different : evidence.context.change ≠ change) :
 check s member place principal requestId change evidence = false := by
 apply changed_context_refused
 intro same
 exact different (congrArg Context.change same)

theorem changed_cut_refused (different : evidence.context.managementCut ≠ s.management.serial) :
 check s member place principal requestId change evidence = false := by
 apply changed_context_refused
 intro same
 exact different (congrArg Context.managementCut same)

theorem no_authority_refused (empty : s.management.view.authority.issued = []) :
 commit s member place principal requestId change evidence = none := by
 cases done : commit s member place principal requestId change evidence with
 | none => rfl
 | some result =>
   have auth := (commit_parts done).2.1.2.2
   exact False.elim ((CurrentUse.no_claim_no_authorization _ _ _ _ empty) auth)

-- Current access requires both live structural attachment and current metadata;
-- a surviving record alone is only historical information.
def Usable (s : State p a) (place : Fin p) (binding : Binding) : Prop :=
 Attached s place ∧ OwnerSourceContext.Usable s.metadata.schema (current s.metadata) binding

def usable (s : State p a) (place : Fin p) (binding : Binding) : Bool :=
 attachmentCheck s place && OwnerSourceContext.usable s.metadata.schema (current s.metadata) binding

theorem usable_exact : usable s place binding = true ↔ Usable s place binding := by
 simp [usable,Usable,attachment_exact,OwnerSourceContext.usable_exact]

theorem departed_owner_refused (departed : s.management.configuration.state.participating place = false) :
 usable s place binding = false := by
 simp [usable,attachmentCheck,Common,departed]

#print axioms departed_owner_refused

theorem changed_binding_refused (valid : Invariant s)
 (before : current s.metadata binding.key = some binding.metadata)
 (changedKey : change.key = binding.key)
 (accepted : commit s member place principal requestId change evidence = some result) :
 usable result usePlace binding = false := by
 cases checked : usable result usePlace binding with
 | false => rfl
 | true =>
   have present := (usable_exact.mp checked).2.2
   have oldGen := (valid.2.1 binding.key binding.metadata before).2.2.2.1
   have newGen := ((commit_preserves valid accepted).2.1 binding.key binding.metadata present).2.2.2.1
   have bump := commit_generation accepted
   rw [changedKey] at bump
   omega

inductive Rooted (root : State p a) : State p a → Prop where
 | initial : MixedManagementEntry.Invariant root.management → root.metadata.slots = [] →
   root.metadata.used = [] → Rooted root root
 | metadata : Rooted root s → commit s member place principal requestId change evidence = some result → Rooted root result
 | management : Rooted root s → manage s member place principal requestId raw evidence = some result → Rooted root result.1
 | authority : Rooted root s → Rooted root (installAuthorityHead s view)

theorem rooted_invariant (path : Rooted root s) : Invariant s := by
 induction path with
 | initial valid empty unused => exact ⟨valid,empty_consistent empty,by simp [unused]⟩
 | metadata _ done ih => exact commit_preserves ih done
 | management _ done ih => exact manage_preserves ih done
 | authority _ ih => exact head_preserves ih

def IssuedAt (root : State p a) (key : OwnerStructuredKeys.Key) (metadata : Metadata) : Prop :=
 ∃ (before result : State p a) (member : Fin a) (place : Fin p) (principal requestId : Nat)
  (label : Nat) (evidence : Evidence), Rooted root before ∧
  commit before member place principal requestId (.activate key label) evidence = some result ∧
  current result.metadata key = some metadata

theorem rooted_origin (path : Rooted root s) (present : current s.metadata key = some metadata) :
 IssuedAt root key metadata := by
 induction path with
 | initial _ empty _ => simp [current,slot,empty] at present
 | @metadata before member place principal requestId change evidence result path done ih =>
   by_cases same : key = change.key
   · obtain ⟨_,_,_,next,rule,equal⟩ := commit_parts done
     cases rule with
     | @activate target declaration label _ _ _ _ =>
       subst key
       exact ⟨before,result,member,place,principal,requestId,label,evidence,path,done,present⟩
     | @retire target old found =>
       subst key
       rw [equal] at present
       change (slot (setSlot before.metadata target _) target).current = some metadata at present
       rw [set_same] at present
       cases present
   · exact ih ((commit_other_key done same) ▸ present)
 | management _ done ih =>
   obtain ⟨system,created,_,rfl⟩ := manage_parts done
   exact ih present
 | authority _ ih => exact ih present

#print axioms changed_binding_refused
#print axioms rooted_invariant
#print axioms rooted_origin

#print axioms commit_parts
#print axioms commit_complete
#print axioms perform_complete
#print axioms commit_preserves
#print axioms manage_preserves
#print axioms head_preserves
#print axioms commit_generation
#print axioms commit_floor
#print axioms commit_other_key
#print axioms metadata_then_metadata_refused
#print axioms metadata_then_management_refused
#print axioms management_then_metadata_refused
#print axioms no_authority_refused
#print axioms usable_exact

#print axioms attachment_exact
#print axioms check_sound
#print axioms authorize_sound
#print axioms authorize_complete
end MirroreaProofFirst.OwnerMetadataManagement
