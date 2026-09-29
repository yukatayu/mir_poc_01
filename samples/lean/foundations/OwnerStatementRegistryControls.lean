import OwnerStatementRegistryInvariant
import OwnerStatementExecutionControls
import OwnerStatementRetainedControls
namespace MirroreaProofFirst.OwnerStatementRegistryControls
open OwnerStatementRegistrySelection
deriving instance DecidableEq for OwnerMetadataRegistry.Registry

-- Same ordinary checked-IR S-T-S program and real source/metadata management.
-- Schema attachment below is explicit test installation input, NOT a source
-- construction implementation, permission issuer or physical custody proof.
def keyS : OwnerStructuredKeys.Key := ⟨"a","self","hp"⟩
def keyT : OwnerStructuredKeys.Key := ⟨"b","self","hp"⟩
def declaration (owner fieldSpace : String) : OwnerSourceContext.Declaration :=
 ⟨fieldSpace,"hp",owner,"Int",some "observer_safe"⟩
def attachOwner (s : State 3 1) (place : Fin 3) (moduleId : Nat)
 (owner fieldSpace : String) : Option (State 3 1) := do
 let system := s.session.state.source.machine.store.core.system
 let moduleKey ← CompositionCore.index system.configuration.count moduleId
 let entry := system.configuration.state.instances moduleKey
 let registry : OwnerMetadataRegistry.Registry :=
  {instanceId := system.configuration.state.realm,
   ownerKey := (WorldProjection.encode (a:=1) (n:=system.configuration.count) (.locus place)).val,
   ownerIdentity := ⟨.locus,system.configuration.state.placeIncarnation place,0⟩,
   moduleKey := (WorldProjection.encode (a:=1) (p:=3) (.moduleSlot moduleKey)).val,
   moduleIdentity := ⟨.module,1,entry.revision⟩,
   code := entry.definition.val,contract := entry.definition.val,
   ownerName := owner,schema := [declaration owner fieldSpace],serial := 0,slots := [],used := []}
 install s place registry

def begun := (MixedOwnerContinuation.launch 91 OwnerMetadataStoreControls.settings.view
 OwnerMetadataStoreControls.settings.controlPolicy OwnerStatementExecutionControls.store
 OwnerStatementExecutionControls.program).map fun session =>
 (⟨session,fun _ => none,[keyS,keyT],none⟩ : State 3 1)
def preparedS := begun.map fun s => drive 2 s 0 7
def installedS := preparedS.bind fun s => attachOwner s 0 0 "S" "a"
def activatedS := installedS.bind fun s => changeMetadata s 0 0 7 ⟨"sequence.mir",7⟩ (.activate keyS 0)
def pendingS := activatedS.map fun s => tick s 0 7
def queuedS := pendingS.bind transfer
def servedS := queuedS.map fun s => service s (FallibleFlow.signed 63)
def acceptedS := servedS.map fun s => tick s 0 7
#guard begun.isSome && preparedS.isSome && installedS.isSome && activatedS.isSome
#guard (servedS.map fun s => (s.session.state.owner.store 0,s.session.state.owner.history.length)) = some (some 34,1)

def preparedT := acceptedS.map fun s => drive 2 s 0 7
def installedT := preparedT.bind fun s => attachOwner s 1 1 "T" "b"
def activatedT := installedT.bind fun s => changeMetadata s 0 1 7 ⟨"sequence.mir",17⟩ (.activate keyT 0)
def pendingT := activatedT.map fun s => tick s 0 7
def queuedT := pendingT.bind transfer
def servedT := queuedT.map fun s => service s (FallibleFlow.signed 63)
def acceptedT := servedT.map fun s => tick s 0 7
#guard installedT.isSome && activatedT.isSome
#guard (servedT.map fun s => (s.session.state.owner.store 1,s.session.state.owner.history.length)) = some (some 35,2)
-- The third statement reuses the retained S registry, not a re-created slot.
def queuedLast := (acceptedT.map fun s => drive 3 s 0 7).bind transfer
def servedLast := queuedLast.map fun s => service s (FallibleFlow.signed 63)
def acceptedLast := servedLast.map fun s => tick s 0 7
#guard (acceptedLast.map fun s => (s.session.state.owner.store 0,s.session.state.owner.history.length,
 (OwnerStatementPosition.writes s.session.cursor.completed).map Prod.fst)) = some (some 35,3,[0,1,2])
#guard (acceptedLast.bind fun s => s.bank 0) = (activatedS.bind fun s => s.bank 0)

-- Retirement of T is routed to T's registry and consumes the actual shared
-- source allocator. S's committed prefix and current registry survive.
def retiredT := queuedT.bind fun s => changeMetadata s 0 1 7 ⟨"sequence.mir",21⟩ (.retire keyT)
def refusedT := retiredT.map fun s => service s (FallibleFlow.signed 63)
def blockedLast := refusedT.map fun s => drive 6 s 0 7
#guard retiredT.isSome
#guard (refusedT.map fun s => (s.session.state.owner.store 0,s.session.state.owner.store 1,
 s.session.state.owner.history.length,s.session.state.owner.attempts.length,
 s.session.state.owner.queued.isSome)) = some (some 34,some 10,1,1,true)
#guard (blockedLast.map fun s => (OwnerStatementPosition.writes s.session.cursor.completed).map Prod.fst) = some [0]
#guard (retiredT.bind fun s => s.bank 0) = (queuedT.bind fun s => s.bank 0)

-- Raw bank substitution is not an admitted schema-management step. The
-- execution selector rejects it instead of using any convenient registry.
def swappedT := queuedT.map fun s => {s with bank := fun place =>
   if place = 1 then (s.bank (0 : Fin 3)) else (s.bank place)}
def absentT := queuedT.map fun s => {s with bank := fun place =>
   if place = 1 then none else (s.bank place)}
#guard (swappedT.map fun s => (service s (FallibleFlow.signed 63)).session.state.owner.history.length) = some 1
#guard (absentT.map fun s => (service s (FallibleFlow.signed 63)).session.state.owner.history.length) = some 1
#guard (swappedT.bind fun s => do
 let saved ← s.session.state.owner.queued
 return (select s.session s.bank saved).isNone) = some true
#guard (queuedT.bind fun s => do
 let saved ← s.session.state.owner.queued
 let (place,registry) ← select s.session s.bank saved
 return (place.val,registry.ownerName)) = some (1,"T")
-- Generation advance alone is not revocation of the selected claims. Existing
-- AdmissionPhases.resolve revalidates the SAME witness at the current context;
-- this candidate does not select a reservation/commit/disclosure policy (Q18).
def authorityChanged := queuedT.bind fun s => authorityHead s
 {s.session.state.source.machine.store.core.system.view with
  generation := s.session.state.source.machine.store.core.system.view.generation+1}
#guard authorityChanged.isSome
#guard (authorityChanged.map fun s => ((service s (FallibleFlow.signed 63)).session.state.owner.store 1,
 (service s (FallibleFlow.signed 63)).session.state.owner.history.length,
 (service s (FallibleFlow.signed 63)).session.state.owner.attempts.length)) = some (some 35,2,2)

-- Revoke exactly the witness actually selected by the queued T request.
-- A genuine lower authority refusal is recorded as an attempt and drains its
-- queue; it does not add a committed row or change T's store.
def authorityRevoked := queuedT.bind fun s => do
 let saved ← s.session.state.owner.queued
 let old := s.session.state.source.machine.store.core.system.view
 let revoked := ModuleContractBoundary.CurrentPolicyFrame.used saved.original.witness ++ old.authority.revoked
 let updated : WorldProjection.AuthorityView 1 := {old with
   generation := old.generation+1
   authority := {old.authority with revoked := revoked}}
 authorityHead s updated
#guard authorityRevoked.isSome
#guard (authorityRevoked.map fun s => ((service s (FallibleFlow.signed 63)).session.state.owner.store 1,
 (service s (FallibleFlow.signed 63)).session.state.owner.history.length,
 (service s (FallibleFlow.signed 63)).session.state.owner.attempts.length,
 (service s (FallibleFlow.signed 63)).session.state.owner.queued.isSome)) = some (some 10,1,2,false)
#guard (authorityRevoked.map fun s => ((service s (FallibleFlow.signed 63)).session.state.owner.attempts.getLast?).map Prod.snd) = some (some (.refused .authority))

def reactivatedT := retiredT.bind fun s => changeMetadata s 0 1 7 ⟨"sequence.mir",22⟩ (.activate keyT 1)
#guard reactivatedT.isSome
#guard (reactivatedT.map fun s => (service s (FallibleFlow.signed 63)).session.state.owner.history.length) = some 1
#guard (retiredT.bind fun s => do
 let old ← installedT
 let original ← old.bank 1
 return (install s 1 original).isNone) = some true
#guard (servedT.map fun s => (service s (FallibleFlow.signed 63)).session.state.owner.attempts.length) = some 2

-- Same ordinary program originating at S or A. No origin-specific executor
-- branch; trusted initial schema input stays explicit at both attachment sites.
def sequenceFrom (origin : Fin 3) : Option (State 3 1) := do
 let session ← MixedOwnerContinuation.launch 91 OwnerMetadataStoreControls.settings.view
  OwnerMetadataStoreControls.settings.controlPolicy OwnerStatementExecutionControls.store
  {OwnerStatementExecutionControls.program with place := origin}
 let initial : State 3 1 := ⟨session,fun _ => none,[keyS,keyT],none⟩
 let first ← attachOwner (drive 2 initial 0 7) 0 0 "S" "a"
 let first ← changeMetadata first 0 0 7 ⟨"sequence.mir",7⟩ (.activate keyS 0)
 let first ← transfer (tick first 0 7)
 let first := tick (service first (FallibleFlow.signed 63)) 0 7
 let second ← attachOwner (drive 2 first 0 7) 1 1 "T" "b"
 let second ← changeMetadata second 0 1 7 ⟨"sequence.mir",17⟩ (.activate keyT 0)
 let second ← transfer (tick second 0 7)
 let second := tick (service second (FallibleFlow.signed 63)) 0 7
 let third ← transfer (drive 3 second 0 7)
 return tick (service third (FallibleFlow.signed 63)) 0 7
#guard (sequenceFrom 0 |>.map fun s => (s.session.program.place.val,s.session.state.owner.store 0,
 s.session.state.owner.store 1,s.session.state.owner.history.length,
 (OwnerStatementPosition.writes s.session.cursor.completed).map Prod.fst)) = some (0,some 35,some 35,3,[0,1,2])
#guard (sequenceFrom 2 |>.map fun s => (s.session.program.place.val,s.session.state.owner.store 0,
 s.session.state.owner.store 1,s.session.state.owner.history.length)) = some (2,some 35,some 35,3)

-- Explicit finite controls and actual admission chain are separate evidence.
-- The predicates below follow the ACTUAL functions; no expected-state fixture
-- supplies a transition or authenticates initial schema data.
private def AdmittedResult (result : Option (State 3 1)) : Prop :=
 ∀ s, result = some s → OwnerStatementRegistryInvariant.Admitted 91
   OwnerMetadataStoreControls.settings.view OwnerMetadataStoreControls.settings.controlPolicy
   OwnerStatementExecutionControls.store s

private theorem follow {s next : State 3 1}
 (entered : OwnerStatementRegistryInvariant.Admitted realm authority policy store s)
 (path : Rooted s next) : OwnerStatementRegistryInvariant.Admitted realm authority policy store next := by
 obtain ⟨initial,start,prior⟩ := entered
 exact ⟨initial,start,rooted_trans prior path⟩

private theorem map_path {input : Option (State 3 1)} {f : State 3 1 → State 3 1}
 (prior : AdmittedResult input) (step : ∀ s, Rooted s (f s)) : AdmittedResult (input.map f) := by
 intro next run
 cases found : input with
 | none => simp [found] at run
 | some s =>
   simp only [found,Option.map_some,Option.some.injEq] at run
   subst next
   exact follow (prior s found) (step s)

private theorem bind_path {input : Option (State 3 1)} {f : State 3 1 → Option (State 3 1)}
 (prior : AdmittedResult input) (step : ∀ s next, f s = some next → Rooted s next) : AdmittedResult (input.bind f) := by
 intro next run
 cases found : input with
 | none => simp [found] at run
 | some s =>
   simp only [found,Option.bind_some] at run
   exact follow (prior s found) (step s next run)

private theorem attachment_path (run : attachOwner s place moduleId owner fieldSpace = some next) : Rooted s next := by
 unfold attachOwner at run
 dsimp only at run
 cases indexed : CompositionCore.index s.session.state.source.machine.store.core.system.configuration.count moduleId with
 | none => simp [indexed] at run
 | some moduleKey =>
   simp only [indexed,Option.bind_eq_bind,Option.bind_some] at run
   exact .install .initial run

private theorem begun_admitted : AdmittedResult begun := by
 intro s run
 unfold begun at run
 cases launch : MixedOwnerContinuation.launch 91 OwnerMetadataStoreControls.settings.view
   OwnerMetadataStoreControls.settings.controlPolicy OwnerStatementExecutionControls.store
   OwnerStatementExecutionControls.program with
 | none => simp [launch] at run
 | some session =>
   simp only [launch,Option.map_some,Option.some.injEq] at run
   subst s
   exact ⟨_,⟨⟨_,launch⟩,rfl,rfl⟩,.initial⟩

theorem actual_completed_admitted : AdmittedResult acceptedLast := by
 apply map_path (step:=fun _ => .tick .initial)
 apply map_path (step:=fun _ => .service .initial)
 apply bind_path (step:=fun _ _ run => .transfer .initial run)
 apply map_path (step:=fun _ => drive_rooted .initial)
 apply map_path (step:=fun _ => .tick .initial)
 apply map_path (step:=fun _ => .service .initial)
 apply bind_path (step:=fun _ _ run => .transfer .initial run)
 apply map_path (step:=fun _ => .tick .initial)
 apply bind_path (step:=fun _ _ run => .metadata .initial run)
 apply bind_path (step:=fun _ _ run => attachment_path run)
 apply map_path (step:=fun _ => drive_rooted .initial)
 apply map_path (step:=fun _ => .tick .initial)
 apply map_path (step:=fun _ => .service .initial)
 apply bind_path (step:=fun _ _ run => .transfer .initial run)
 apply map_path (step:=fun _ => .tick .initial)
 apply bind_path (step:=fun _ _ run => .metadata .initial run)
 apply bind_path (step:=fun _ _ run => attachment_path run)
 apply map_path (step:=fun _ => drive_rooted .initial)
 exact begun_admitted

#print axioms actual_completed_admitted
end MirroreaProofFirst.OwnerStatementRegistryControls
