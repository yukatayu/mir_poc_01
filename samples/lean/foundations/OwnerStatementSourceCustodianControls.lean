import OwnerStatementSourceCustodian
import OwnerStatementRegistryControls
namespace MirroreaProofFirst.OwnerStatementSourceCustodianControls
open OwnerStatementLiveCustodian OwnerStatementSourceCustodian
open OwnerStatementRegistrySelection

def hp : OwnerStructuredKeys.Key := ⟨"player","target","hp"⟩
def atk : OwnerStructuredKeys.Key := ⟨"player","self","atk"⟩
def fields : MixedNamedOwnerSource.Fields := ⟨[("hp",0),("atk",1)],[(0,(0,0)),(1,(0,0))]⟩
def program : MixedOwnerProgram.Program 3 :=
 ⟨2,fields,[],0,[.assignment ⟨⟨"two.mir",10⟩,"hp",.sub (.state "hp") (.state "atk")⟩,
 .assignment ⟨⟨"two.mir",20⟩,"hp",.integer 17⟩]⟩
def store (k : Nat) : Option Int := if k=0 then some 200 else if k=1 then some 10 else none

def begun := (MixedOwnerContinuation.launch 91 OwnerMetadataStoreControls.settings.view
 OwnerMetadataStoreControls.settings.controlPolicy store program).map fun session =>
 (⟨session,fun _ => none,[hp,atk],none⟩ : OwnerStatementRegistrySelection.State 3 1)

def attach (s : OwnerStatementRegistrySelection.State 3 1) : Option (OwnerStatementRegistrySelection.State 3 1) := do
 let system := s.session.state.source.machine.store.core.system
 let moduleKey ← CompositionCore.index system.configuration.count 0
 let entry := system.configuration.state.instances moduleKey
 let registry : OwnerMetadataRegistry.Registry :=
  {instanceId := system.configuration.state.realm,
   ownerKey := (WorldProjection.encode (a:=1) (n:=system.configuration.count) (.locus (0:Fin 3))).val,
   ownerIdentity := ⟨.locus,system.configuration.state.placeIncarnation 0,0⟩,
   moduleKey := (WorldProjection.encode (a:=1) (p:=3) (.moduleSlot moduleKey)).val,
   moduleIdentity := ⟨.module,1,entry.revision⟩,
   code := entry.definition.val,contract := entry.definition.val,ownerName := "S",
   schema := [⟨"player","hp","S","Int",some "observer_safe"⟩,⟨"player","atk","S","Int",some "observer_safe"⟩],serial := 0,slots := [],used := []}
 install s 0 registry

def prepared := begun.map fun s => drive 2 s 0 7
def attached := prepared.bind attach
def hpLive := attached.bind fun s => changeMetadata s 0 0 7 ⟨"two.mir",2⟩ (.activate hp 0)
def allLive := hpLive.bind fun s => changeMetadata s 0 0 7 ⟨"two.mir",3⟩ (.activate atk 0)
def initial := allLive.map (start 8)

-- A driver on the real functions, with no expected store/reply injection.
def progressN : Nat → OwnerStatementLiveCustodian.State p a → Fin a → Nat → Option (OwnerStatementLiveCustodian.State p a)
 | 0,s,_,_ => some s
 | fuel+1,s,member,principal => do
   let next ← progress s member principal
   progressN fuel next member principal

def issue (fuel : Nat) (s : OwnerStatementLiveCustodian.State p a) (member : Fin a) (principal : Nat) := do
 let ready ← progressN fuel s member principal
 OwnerStatementSourceCustodian.transfer ready

def runOne (capacity fuel : Nat) (s : OwnerStatementLiveCustodian.State p a)
 (member : Fin a) (principal : Nat) : Option (OwnerStatementLiveCustodian.State p a) := do
 let queued ← issue fuel s member principal
 let (staged,ticket) ← stage capacity queued
 let executed ← execute staged ticket (FallibleFlow.signed 63)
 let reported ← report executed ticket
 consume reported ticket member principal

def first := initial.bind fun s => runOne 2 1 s 0 7
def second := first.bind fun s => runOne 2 3 s 0 7
#guard initial.isSome && first.isSome && second.isSome
#guard (initial.map fun s => s.live.session.state.owner.store 0) = some (some 200)
#guard (first.map fun s => (s.live.session.state.owner.store 0,count s.live.session,s.nextSerial,s.dispatched.length)) = some (some 190,1,1,1)
#guard (second.map fun s => (s.live.session.state.owner.store 0,count s.live.session,s.nextSerial,s.dispatched.length,
 (OwnerStatementPosition.writes s.live.session.cursor.completed).map Prod.fst)) = some (some 17,2,2,2,[0,1])
#guard (first.bind fun s => runOne 1 3 s 0 7).isNone

-- Compare the same actual completed service before source consumption.
def staged := initial.bind fun s => (issue 1 s 0 7).bind (stage 2)
def executed := staged.bind fun (s,ticket) => (execute s ticket (FallibleFlow.signed 63)).map fun next => (next,ticket)
def reported := executed.bind fun (s,ticket) => (report s ticket).map fun next => (next,ticket)
#guard (executed.map fun (s,_) => (s.live.session.state.owner.store 0,count s.live.session,s.held.isSome)) = some (some 190,0,true)
#guard (executed.bind fun (s,ticket) => consume s ticket 0 7).isNone
#guard (reported.bind fun (s,ticket) => consume s {ticket with designation := 9} 0 7).isNone
#guard (reported.bind fun (s,ticket) => consume s {ticket with saved := {ticket.saved with origin := {ticket.saved.origin with ordinal := 1}}} 0 7).isNone
#guard (reported.bind fun (s,ticket) => consume s ticket 0 7 |>.bind fun next => consume next ticket 0 7).isNone

-- Reusing the same live world is a real source continuation, not re-launch.
def twoWithRoom := initial.bind fun s => (runOne 4 1 s 0 7).bind fun first => runOne 4 3 first 0 7
def continued := twoWithRoom.bind invokeAgain
def nextFirst := continued.bind fun s => runOne 4 3 s 0 7
def nextSecond := nextFirst.bind fun s => runOne 4 3 s 0 7
#guard continued.isSome && nextFirst.isSome && nextSecond.isSome
#guard (continued.map fun s => (s.live.session.state.owner.store 0,count s.live.session,s.live.session.activation,s.dispatched.length)) = some (some 17,0,1,2)
#guard (nextFirst.map fun s => (s.live.session.state.owner.store 0,count s.live.session,s.dispatched.length)) = some (some 7,1,3)
#guard (nextSecond.map fun s => (s.live.session.state.owner.store 0,count s.live.session,s.live.session.state.owner.history.length,s.dispatched.length)) = some (some 17,2,4,4)
#guard (executed.bind fun (s,_) => invokeAgain s).isNone
#guard (first.bind invokeAgain).isNone

-- These derivations tie the executable multi-step driver to the general Path.
theorem progressN_path (run : progressN fuel s member principal = some next) :
 OwnerStatementSourceCustodian.Path capacity s next := by
 induction fuel generalizing s with
 | zero => cases run; exact .initial
 | succ fuel ih =>
   unfold progressN at run
   cases step : progress s member principal with
   | none => simp [step] at run
   | some middle =>
     simp only [step,Option.bind_eq_bind,Option.bind_some] at run
     have tail := ih run
     exact OwnerStatementSourceCustodian.path_trans (.progress .initial step) tail

theorem runOne_path (run : runOne capacity fuel s member principal = some next) :
 OwnerStatementSourceCustodian.Path capacity s next := by
 unfold runOne issue at run
 cases progressed : progressN fuel s member principal with
 | none => simp [progressed] at run
 | some ready =>
   simp only [progressed,Option.bind_eq_bind,Option.bind_some] at run
   cases transferred : OwnerStatementSourceCustodian.transfer ready with
   | none => simp [transferred] at run
   | some queued =>
     simp only [transferred,Option.bind_some] at run
     cases reserved : stage capacity queued with
     | none => simp [reserved] at run
     | some pair =>
       obtain ⟨staged,ticket⟩ := pair
       simp only [reserved,Option.bind_some] at run
       cases dispatched : execute staged ticket (FallibleFlow.signed 63) with
       | none => simp [dispatched] at run
       | some done =>
         simp only [dispatched,Option.bind_some] at run
         cases delivered : report done ticket with
         | none => simp [delivered] at run
         | some response =>
           simp only [delivered,Option.bind_some] at run
           exact .consume (.report (.execute (.stage (.transfer (progressN_path progressed) transferred) reserved) dispatched) delivered) run

private def AdmittedBank (result : Option (OwnerStatementRegistrySelection.State 3 1)) : Prop :=
 ∀ s, result = some s → OwnerStatementRegistryInvariant.Admitted 91
 OwnerMetadataStoreControls.settings.view OwnerMetadataStoreControls.settings.controlPolicy store s

private theorem bank_follow
 (admitted : OwnerStatementRegistryInvariant.Admitted realm authority policy initialStore s)
 (path : OwnerStatementRegistrySelection.Rooted s next) :
 OwnerStatementRegistryInvariant.Admitted realm authority policy initialStore next := by
 obtain ⟨origin,entered,prior⟩ := admitted
 exact ⟨origin,entered,OwnerStatementRegistrySelection.rooted_trans prior path⟩

private theorem bank_map {input : Option (OwnerStatementRegistrySelection.State 3 1)}
 {f : OwnerStatementRegistrySelection.State 3 1 → OwnerStatementRegistrySelection.State 3 1}
 (prior : AdmittedBank input) (step : ∀ s, OwnerStatementRegistrySelection.Rooted s (f s)) : AdmittedBank (input.map f) := by
 intro next run
 cases found : input with
 | none => simp [found] at run
 | some s =>
   simp only [found,Option.map_some,Option.some.injEq] at run
   subst next
   exact bank_follow (prior s found) (step s)

private theorem bank_bind {input : Option (OwnerStatementRegistrySelection.State 3 1)}
 {f : OwnerStatementRegistrySelection.State 3 1 → Option (OwnerStatementRegistrySelection.State 3 1)}
 (prior : AdmittedBank input) (step : ∀ s next, f s = some next → OwnerStatementRegistrySelection.Rooted s next) : AdmittedBank (input.bind f) := by
 intro next run
 cases found : input with
 | none => simp [found] at run
 | some s =>
   simp only [found,Option.bind_some] at run
   exact bank_follow (prior s found) (step s next run)

private theorem attach_path (run : attach s = some next) : OwnerStatementRegistrySelection.Rooted s next := by
 unfold attach at run
 dsimp only at run
 cases indexed : CompositionCore.index s.session.state.source.machine.store.core.system.configuration.count 0 with
 | none => simp [indexed] at run
 | some moduleKey =>
   simp only [indexed,Option.bind_eq_bind,Option.bind_some] at run
   exact .install .initial run

private theorem begun_admitted : AdmittedBank begun := by
 intro s run
 unfold begun at run
 cases launch : MixedOwnerContinuation.launch 91 OwnerMetadataStoreControls.settings.view
   OwnerMetadataStoreControls.settings.controlPolicy store program with
 | none => simp [launch] at run
 | some session =>
   simp only [launch,Option.map_some,Option.some.injEq] at run
   subst s
   exact ⟨_,⟨⟨_,launch⟩,rfl,rfl⟩,.initial⟩

private theorem allLive_admitted : AdmittedBank allLive := by
 apply bank_bind (step:=fun _ _ run => .metadata .initial run)
 apply bank_bind (step:=fun _ _ run => .metadata .initial run)
 apply bank_bind (step:=fun _ _ run => attach_path run)
 apply bank_map (step:=fun _ => drive_rooted .initial)
 exact begun_admitted

private def AdmittedResult (result : Option (OwnerStatementLiveCustodian.State 3 1)) : Prop :=
 ∀ s, result = some s → OwnerStatementRegistryInvariant.Admitted 91
 OwnerMetadataStoreControls.settings.view OwnerMetadataStoreControls.settings.controlPolicy store s.live

private theorem initial_admitted : AdmittedResult initial := by
 intro s run
 unfold initial at run
 cases found : allLive with
 | none => simp [found] at run
 | some live =>
   simp only [found,Option.map_some,Option.some.injEq] at run
   subst s
   exact allLive_admitted live found

private theorem driver_admitted {input : Option (OwnerStatementLiveCustodian.State 3 1)}
 (prior : AdmittedResult input) : AdmittedResult (input.bind fun s => runOne capacity fuel s 0 7) := by
 intro next run
 cases found : input with
 | none => simp [found] at run
 | some s =>
   simp only [found,Option.bind_some] at run
   exact OwnerStatementSourceCustodian.path_admitted (prior s found) (runOne_path run)

theorem actual_two_statement_admitted : AdmittedResult second :=
 driver_admitted (driver_admitted initial_admitted)

private theorem invocation_admitted {input : Option (OwnerStatementLiveCustodian.State 3 1)}
 (prior : AdmittedResult input) : AdmittedResult (input.bind invokeAgain) := by
 intro next run
 cases found : input with
 | none => simp [found] at run
 | some s =>
   simp only [found,Option.bind_some] at run
   exact OwnerStatementSourceCustodian.path_admitted (capacity:=0) (prior s found) (.invokeAgain .initial run)

theorem actual_four_statement_admitted : AdmittedResult nextSecond := by
 have htwo : AdmittedResult twoWithRoom := by
  unfold twoWithRoom
  rw [← Option.bind_assoc]
  exact driver_admitted (driver_admitted initial_admitted)
 exact driver_admitted (driver_admitted (invocation_admitted htwo))


-- Idle successor installation uses the same authenticated-head boundary as
-- the source machine. Neither freshness nor these controls issue authority.
def successor (s : OwnerStatementLiveCustodian.State 3 1) : WorldProjection.AuthorityView 1 :=
 let old := s.live.session.state.source.machine.store.core.system.view
 {old with generation := old.generation+1}

def refreshed := first.bind fun s => refreshAuthority s (successor s)
def afterRefresh := refreshed.bind fun s => runOne 2 3 s 0 7
#guard refreshed.isSome && afterRefresh.isSome
#guard (refreshed.map fun s => (s.live.session.state.owner.store 0,count s.live.session,s.nextSerial,s.dispatched.length)) = some (some 190,1,1,1)
#guard (afterRefresh.map fun s => (s.live.session.state.owner.store 0,count s.live.session,s.nextSerial,s.dispatched.length)) = some (some 17,2,2,2)
#guard (first.bind fun s => refreshAuthority s s.live.session.state.source.machine.store.core.system.view).isNone
#guard (staged.bind fun (s,_) => refreshAuthority s (successor s)).isNone
#guard (executed.bind fun (s,_) => refreshAuthority s (successor s)).isNone
#guard (reported.bind fun (s,_) => refreshAuthority s (successor s)).isNone
-- Waiting already exists before stage acquires custody; held = none alone is
-- insufficient to establish the supported scheduling boundary.
def issued := initial.bind fun s => issue 1 s 0 7
#guard (issued.map fun s => (s.held.isNone,(MixedOwnerSourceIssue.ownerWaiting s.live.session.state.source.waiting).isSome)) = some (true,true)
#guard (issued.bind fun s => refreshAuthority s (successor s)).isNone

def revokeAll (s : OwnerStatementLiveCustodian.State 3 1) : WorldProjection.AuthorityView 1 :=
 let next := successor s
 {next with authority := {next.authority with revoked := next.authority.issued.map CurrentUse.Claim.id ++ next.authority.revoked}}
def revoked := first.bind fun s => refreshAuthority s (revokeAll s)
#guard revoked.isSome
#guard (revoked.map fun s => (s.live.session.state.owner.store 0,count s.live.session,s.nextSerial,s.dispatched.length)) = some (some 190,1,1,1)
#guard (revoked.bind fun s => runOne 2 3 s 0 7).isNone

private theorem refresh_admitted {input : Option (OwnerStatementLiveCustodian.State 3 1)}
 (prior : AdmittedResult input) : AdmittedResult (input.bind fun s => refreshAuthority s (successor s)) := by
 intro next run
 cases found : input with
 | none => simp [found] at run
 | some s =>
   simp only [found,Option.bind_some] at run
   exact OwnerStatementSourceCustodian.path_admitted (capacity:=2) (prior s found) (.authority .initial run)

theorem actual_refreshed_two_statement_admitted : AdmittedResult afterRefresh :=
 driver_admitted (refresh_admitted (driver_admitted initial_admitted))

#print axioms actual_refreshed_two_statement_admitted

#print axioms actual_four_statement_admitted

#print axioms actual_two_statement_admitted

#print axioms progressN_path
#print axioms runOne_path
end MirroreaProofFirst.OwnerStatementSourceCustodianControls
