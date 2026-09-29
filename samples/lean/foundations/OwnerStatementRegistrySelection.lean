import OwnerStatementEntryGate
namespace MirroreaProofFirst.OwnerStatementRegistrySelection

-- Finite per-owner inventory. Values are supplied checked-installation inputs;
-- this function does not authenticate, issue or construct a schema. Absence is
-- explicit and cannot be replaced with another owner's registry.
abbrev Bank (p : Nat) := Fin p → Option OwnerMetadataRegistry.Registry

-- Select from the actual saved operation in the same evolving catalog. No
-- source-name/fixture branch, queue-position hint or caller-selected owner.
def select (session : MixedOwnerContinuation.Session p a) (bank : Bank p)
 (saved : OwnerSavedPending.Saved) : Option (Fin p × OwnerMetadataRegistry.Registry) := do
 let system := session.state.source.machine.store.core.system
 let pending ← OwnerSavedPending.materialize (WorldProjection.size a p system.configuration.count) saved
 let .operation _ place := WorldProjection.decode pending.request.operation.key | none
 let registry ← bank place
 if OwnerMetadataManagement.attachmentCheck ⟨system,registry⟩ place then some (place,registry) else none

def Selected (session : MixedOwnerContinuation.Session p a) (bank : Bank p)
 (saved : OwnerSavedPending.Saved) (place : Fin p) (registry : OwnerMetadataRegistry.Registry) : Prop :=
 ∃ (pending : OwnerEffectService.Pending
     (WorldProjection.size a p session.state.source.machine.store.core.system.configuration.count))
   (key : Fin session.state.source.machine.store.core.system.configuration.count),
 OwnerSavedPending.save pending = saved ∧
 WorldProjection.decode pending.request.operation.key = .operation key place ∧
 bank place = some registry ∧
 OwnerMetadataManagement.Attached ⟨session.state.source.machine.store.core.system,registry⟩ place

theorem select_exact {p a : Nat} {session : MixedOwnerContinuation.Session p a}
 {bank : Bank p} {saved : OwnerSavedPending.Saved} {place : Fin p} {registry : OwnerMetadataRegistry.Registry} :
 select session bank saved = some (place,registry) ↔ Selected session bank saved place registry := by
 constructor
 · intro run
   unfold select at run
   cases loaded : OwnerSavedPending.materialize
     (WorldProjection.size a p session.state.source.machine.store.core.system.configuration.count) saved with
   | none => simp [loaded] at run
   | some pending =>
     simp only [loaded,Option.bind_eq_bind,Option.bind_some] at run
     split at run
     · rename_i key owner decoded
       cases found : bank owner with
       | none => simp [found] at run
       | some metadata =>
         simp only [found,Option.bind_some] at run
         split at run
         · rename_i attached
           simp only [Option.some.injEq,Prod.mk.injEq] at run
           obtain ⟨rfl,rfl⟩ := run
           exact ⟨pending,key,OwnerSavedPending.materialize_exact.mp loaded,decoded,found,
             OwnerMetadataManagement.attachment_exact.mp attached⟩
         · cases run
     · cases run
 · rintro ⟨pending,key,loaded,decoded,found,attached⟩
   simp [select,OwnerSavedPending.materialize_exact.mpr loaded,decoded,found,
     OwnerMetadataManagement.attachment_exact.mpr attached]

theorem selected_unique (left : Selected session bank saved place registry)
 (right : Selected session bank saved other otherRegistry) : place = other ∧ registry = otherRegistry := by
 have same := (select_exact.mpr left).symm.trans (select_exact.mpr right)
 exact Prod.mk.inj (Option.some.inj same)

theorem no_substitute (run : select session bank saved = some (place,registry)) :
 bank place = some registry := by
 obtain ⟨_,_,_,_,held,_⟩ := select_exact.mp run
 exact held

-- Bank updates are pointwise and retain EVERY other owner slot. Authenticated
-- metadata change must supply this value through existing changeMetadata; set
-- is data bookkeeping, not an admitted mutation or a permission constructor.
def set (bank : Bank p) (place : Fin p) (registry : OwnerMetadataRegistry.Registry) : Bank p :=
 fun other => if other = place then some registry else bank other

theorem set_same : set bank place registry place = some registry := by simp [set]
theorem set_other (different : other ≠ place) : set bank place registry other = bank other := by simp [set,different]

-- Read-only selection may be repeated. Only existing guarded service can
-- execute; selection itself never changes source, queue, history or authority.
structure State (p a : Nat) where
 session : MixedOwnerContinuation.Session p a
 bank : Bank p
 addresses : OwnerStructuredKeys.Addresses
 packet : Option OwnerMetadataPending.Packet

def view (s : State p a) (registry : OwnerMetadataRegistry.Registry) : OwnerMetadataSession.State p a :=
 ⟨s.session,registry,s.addresses,s.packet⟩

def service (s : State p a) (ops : FallibleFlow.Arithmetic) : State p a :=
 match s.packet,s.session.state.owner.queued with
 | some packet,some saved =>
   match select s.session s.bank saved with
   | none => s
   | some (place,registry) =>
     if packet.owner = place.val then
       {s with session := (OwnerStatementEntryGate.execute (view s registry) saved ops).session}
     else s
 | _,_ => s

theorem service_bank : (service s ops).bank = s.bank := by
 unfold service
 split
 · split
   · rfl
   · split <;> rfl
 · rfl

theorem service_route : service s ops = s ∨
 ∃ packet saved place registry,
 s.packet = some packet ∧ s.session.state.owner.queued = some saved ∧
 Selected s.session s.bank saved place registry ∧ packet.owner = place.val ∧
 (service s ops).session = (OwnerStatementEntryGate.execute (view s registry) saved ops).session := by
 unfold service
 split
 · rename_i packet saved packets queued
   split
   · exact Or.inl rfl
   · rename_i place registry selected
     split
     · exact Or.inr ⟨packet,saved,place,registry,packets,queued,select_exact.mp selected,‹_›,rfl⟩
     · exact Or.inl rfl
 · exact Or.inl rfl

-- The candidate source step is pure preparation. Registry lookup must succeed
-- before its tentative owner request is published. No owner is run here.
def refused (s : State p a) (candidate : MixedOwnerContinuation.Session p a) : State p a :=
 {s with session := {s.session with status := .failed .rejected,control := candidate.control}}

def tick (s : State p a) (member : Fin a) (principal : Nat) : State p a :=
 let candidate := MixedOwnerContinuation.tick s.session member principal
 match MixedOwnerSourceIssue.ownerWaiting candidate.state.source.waiting with
 | none => {s with session := candidate,packet := none}
 | some waiting =>
   match select candidate s.bank waiting.saved with
   | none => refused s candidate
   | some (_,registry) =>
     let next := OwnerMetadataSession.tick (view s registry) member principal
     {s with session := next.session,packet := next.packet}

def transfer (s : State p a) : Option (State p a) :=
 (MixedOwnerContinuation.transfer s.session).map fun session => {s with session := session}

def changeMetadata (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
 (site : ReferenceSourceData.Site) (change : OwnerMetadataRegistry.Change) : Option (State p a) := do
 let registry ← s.bank place
 let next ← OwnerMetadataSession.changeMetadata (view s registry) member place principal site change
 return {s with session := next.session,bank := set s.bank place next.registry,packet := next.packet}

-- These entries preserve the entire registry inventory. A stale attachment
-- stays retained as history; select/current checks decide present usability.
def authorityHead (s : State p a) (authority : WorldProjection.AuthorityView a) : Option (State p a) :=
 (MixedOwnerContinuation.authorityHead s.session authority).map fun session => {s with session := session}
def controlInput (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
 (raw : MixedCompositionCore.Raw) : Option (State p a × Option Nat) :=
 (MixedOwnerContinuation.controlInput s.session member place principal raw).map fun (session,key) =>
 ({s with session := session},key)
def continueWith (s : State p a) (program : MixedOwnerProgram.Program p) : Option (State p a) :=
 (MixedOwnerContinuation.continueWith s.session program).map fun session => {s with session := session}
def replaceResidual (s : State p a) (program : MixedOwnerProgram.Program p) : Option (State p a) :=
 (MixedOwnerContinuation.replaceResidual s.session program).map fun session => {s with session := session}
def cancel (s : State p a) (member : Fin a) (principal : Nat) : State p a :=
 {s with session := MixedOwnerContinuation.cancel s.session member principal}

-- Explicit INITIAL schema installation input. This check is not authority
-- issuance and does not verify a source schema elaborator. A trusted installer
-- remains in this candidate's TCB; no populated slot/reset/replacement entry.
-- Dynamic field activation/retirement must use the authenticated change path.
def install (s : State p a) (place : Fin p) (registry : OwnerMetadataRegistry.Registry) : Option (State p a) :=
 if (s.bank place).isNone && registry.slots.isEmpty && registry.used.isEmpty && registry.serial == 0 &&
    OwnerMetadataManagement.attachmentCheck ⟨s.session.state.source.machine.store.core.system,registry⟩ place
 then some {s with bank := set s.bank place registry} else none

def drive : Nat → State p a → Fin a → Nat → State p a
 | 0,s,_,_ => s
 | fuel+1,s,member,principal => drive fuel (tick s member principal) member principal

theorem tick_bank : (tick s member principal).bank = s.bank := by
 unfold tick
 dsimp only
 split
 · rfl
 · split <;> rfl

theorem tick_control : s.session.control ≤ (tick s member principal).session.control := by
 have base := MixedOwnerContinuation.tick_control_monotone (s:=s.session) (member:=member) (principal:=principal)
 unfold tick
 dsimp only
 split
 · exact base
 · split
   · exact base
   · exact OwnerMetadataSession.tick_control (s:=view s _) (member:=member) (principal:=principal)

theorem metadata_other (accepted : changeMetadata s member place principal site change = some next)
 (different : other ≠ place) : next.bank other = s.bank other := by
 unfold changeMetadata at accepted
 cases found : s.bank place with
 | none => simp [found] at accepted
 | some registry =>
   simp only [found,Option.bind_eq_bind,Option.bind_some] at accepted
   cases run : OwnerMetadataSession.changeMetadata (view s registry) member place principal site change with
   | none => simp [run] at accepted
   | some result =>
     simp only [run,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
     subst next
     exact set_other different

theorem service_no_selection (queued : s.session.state.owner.queued = some saved)
 (missing : select s.session s.bank saved = none) : service s ops = s := by
 cases packet : s.packet <;> simp [service,packet,queued,missing]

-- A path contains only the actual entries above. Initial attachment remains
-- an explicit external input; it is never inferred from proof validity.
inductive Rooted (initial : State p a) : State p a → Prop where
 | initial : Rooted initial initial
 | tick : Rooted initial s → Rooted initial (tick s member principal)
 | transfer : Rooted initial s → transfer s = some next → Rooted initial next
 | service : Rooted initial s → Rooted initial (service s ops)
 | metadata : Rooted initial s → changeMetadata s member place principal site change = some next → Rooted initial next
 | authority : Rooted initial s → authorityHead s authority = some next → Rooted initial next
 | control : Rooted initial s → controlInput s member place principal raw = some (next,key) → Rooted initial next
 | continued : Rooted initial s → continueWith s program = some next → Rooted initial next
 | replaced : Rooted initial s → replaceResidual s program = some next → Rooted initial next
 | cancel : Rooted initial s → Rooted initial (cancel s member principal)
 | install : Rooted initial s → install s place registry = some next → Rooted initial next

theorem rooted_trans (before : Rooted initial middle) (after : Rooted middle final) : Rooted initial final := by
 induction after with
 | initial => exact before
 | tick _ ih => exact .tick ih
 | transfer _ run ih => exact .transfer ih run
 | service _ ih => exact .service ih
 | metadata _ run ih => exact .metadata ih run
 | authority _ run ih => exact .authority ih run
 | control _ run ih => exact .control ih run
 | continued _ run ih => exact .continued ih run
 | replaced _ run ih => exact .replaced ih run
 | cancel _ ih => exact .cancel ih
 | install _ run ih => exact .install ih run

theorem install_fresh (accepted : install s place registry = some next) :
 s.bank place = none ∧ registry.slots = [] ∧ next.session = s.session := by
 unfold install at accepted
 split at accepted
 · rename_i checked
   simp only [Bool.and_eq_true,Option.isNone_iff_eq_none,List.isEmpty_iff,beq_iff_eq] at checked
   simp only [Option.some.injEq] at accepted
   subst next
   exact ⟨checked.1.1.1.1,checked.1.1.1.2,rfl⟩
 · cases accepted

theorem no_registry_reset (held : s.bank place = some old) : install s place registry = none := by
 simp [install,held]

theorem drive_rooted (path : Rooted initial s) : Rooted initial (drive fuel s member principal) := by
 induction fuel generalizing s with
 | zero => exact path
 | succ fuel ih => exact ih (.tick path)

theorem metadata_parts (accepted : changeMetadata s member place principal site change = some next) :
 ∃ registry result, s.bank place = some registry ∧
 OwnerMetadataSession.changeMetadata (view s registry) member place principal site change = some result ∧
 next = {s with session := result.session,bank := set s.bank place result.registry,packet := result.packet} := by
 unfold changeMetadata at accepted
 cases found : s.bank place with
 | none => simp [found] at accepted
 | some registry =>
   simp only [found,Option.bind_eq_bind,Option.bind_some] at accepted
   cases run : OwnerMetadataSession.changeMetadata (view s registry) member place principal site change with
   | none => simp [run] at accepted
   | some result =>
     simp only [run,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
     exact ⟨registry,result,rfl,run,accepted.symm⟩

theorem mapped_packet {s : State p a} {input : Option (MixedOwnerContinuation.Session p a)}
 (accepted : (input.map fun session => {s with session := session}) = some next) : next.packet = s.packet := by
 cases input with
 | none => cases accepted
 | some session => cases accepted; rfl

theorem control_packet (accepted : controlInput s member place principal raw = some (next,key)) :
 next.packet = s.packet := by
 unfold controlInput at accepted
 cases run : MixedOwnerContinuation.controlInput s.session member place principal raw with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨session,created⟩ := pair
   simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   rfl

theorem install_packet (accepted : install s place registry = some next) : next.packet = s.packet := by
 unfold install at accepted
 split at accepted
 · cases accepted; rfl
 · cases accepted

theorem service_packet : (service s ops).packet = s.packet := by
 unfold service
 split
 · split
   · rfl
   · split <;> rfl
 · rfl

theorem tick_packet (held : (tick s member principal).packet = some packet) :
 s.packet = some packet ∨
 ∃ place registry, s.bank place = some registry ∧
 MixedOwnerSourceIssue.ownerWaiting s.session.state.source.waiting = none ∧
 OwnerMetadataSession.bindWaiting s.addresses registry (MixedOwnerContinuation.tick s.session member principal) = some packet := by
 unfold tick at held
 dsimp only at held
 split at held
 · cases held
 · split at held
   · exact Or.inl held
   · rename_i place registry selected
     rcases OwnerMetadataSession.tick_packet_origin held with old | fresh
     · exact Or.inl old
     · exact Or.inr ⟨place,registry,no_substitute selected,fresh⟩

def Produced (initial : State p a) (packet : OwnerMetadataPending.Packet) : Prop :=
 ∃ before member principal place registry, Rooted initial before ∧
 before.bank place = some registry ∧
 MixedOwnerSourceIssue.ownerWaiting before.session.state.source.waiting = none ∧
 OwnerMetadataSession.bindWaiting before.addresses registry
   (MixedOwnerContinuation.tick before.session member principal) = some packet

theorem rooted_packet_origin (empty : initial.packet = none) (path : Rooted initial s)
 (held : s.packet = some packet) : Produced initial packet := by
 induction path with
 | initial => rw [empty] at held; cases held
 | @tick s member principal prior ih =>
   rcases tick_packet held with old | ⟨place,registry,found,fresh⟩
   · exact ih old
   · exact ⟨s,member,principal,place,registry,prior,found,fresh⟩
 | transfer _ run ih => exact ih ((mapped_packet run).symm.trans held)
 | service _ ih => exact ih (service_packet.symm.trans held)
 | metadata _ run ih =>
   obtain ⟨_,result,_,changed,rfl⟩ := metadata_parts run
   exact ih ((OwnerMetadataSession.metadata_keeps_packet changed).1.symm.trans held)
 | authority _ run ih => exact ih ((mapped_packet run).symm.trans held)
 | control _ run ih => exact ih ((control_packet run).symm.trans held)
 | continued _ run ih => exact ih ((mapped_packet run).symm.trans held)
 | replaced _ run ih => exact ih ((mapped_packet run).symm.trans held)
 | cancel _ ih => exact ih held
 | install _ run ih => exact ih ((install_packet run).symm.trans held)

theorem rooted_packet_nonempty (empty : initial.packet = none) (path : Rooted initial s)
 (held : s.packet = some packet) : packet.fields ≠ [] := by
 obtain ⟨_,_,_,_,_,_,_,_,bound⟩ := rooted_packet_origin empty path held
 exact OwnerMetadataSession.bound_waiting_nonempty bound

theorem service_changed_empty (changed : service s ops ≠ s) :
 (service s ops).session.state.owner.queued = none := by
 obtain ⟨packet,saved,place,registry,packets,queued,selected,owner,same⟩ :=
   (service_route (s:=s) (ops:=ops)).resolve_left changed
 have execute : OwnerStatementEntryGate.execute (view s registry) saved ops =
   OwnerMetadataSession.service (view s registry) ops := by
   simp [OwnerStatementEntryGate.execute,view,queued]
 rw [same,execute]
 apply OwnerStatementEntryGate.service_changed_empty
 intro unchanged
 apply changed
 simp only [service,packets,queued,select_exact.mpr selected,if_pos owner]
 rw [execute,unchanged,←packets]
 rfl

theorem service_empty (empty : s.session.state.owner.queued = none) : service s ops = s := by
 cases packet : s.packet <;> simp [service,packet,empty]

theorem no_repeated_service (changed : service s ops ≠ s) :
 service (service s ops) nextOps = service s ops := service_empty (service_changed_empty changed)

-- Independent successful lower semantics is the premise; this statement does
-- not assume the new selector/gateway succeeds or embed success as a premise.
theorem service_committed_complete {p a : Nat} {s : State p a}
 {place : Fin p} {registry : OwnerMetadataRegistry.Registry}
 {saved : OwnerSavedPending.Saved} {packet : OwnerMetadataPending.Packet}
 {pending : OwnerEffectService.Pending
   (WorldProjection.size a p s.session.state.source.machine.store.core.system.configuration.count)}
 {written : OwnerEffectService.Write
   (WorldProjection.size a p s.session.state.source.machine.store.core.system.configuration.count)}
 (selected : Selected s.session s.bank saved place registry)
 (queued : s.session.state.owner.queued = some saved)
 (packetHeld : s.packet = some packet)
 (current : OwnerMetadataPending.Current s.addresses (OwnerMetadataSession.management (view s registry)) place saved packet)
 (waiting : s.session.state.source.waiting = some (.inr held))
 (same : held.saved = saved)
 (retained : .owner saved ∈ s.session.state.source.machine.store.core.pending)
 (materialized : OwnerSavedPending.materialize
   (WorldProjection.size a p s.session.state.source.machine.store.core.system.configuration.count)
   saved = some pending)
 (admitted : MixedOwnerMaterialization.Admitted
   s.session.state.source.machine.store.core.system.configuration.state
   (OwnerMetadataPending.currentFields s.addresses registry place.val)
   ⟨s.session.state.owner.store,[]⟩ pending)
 (meaning : OwnerEffectService.Commits ops
   (MixedCatalogService.world s.session.state.source.machine.store.core.system.configuration.state
     s.session.state.source.machine.store.core.system.view)
   (MixedCatalogService.registry s.session.state.source.machine.store.core.system.configuration.state)
   ⟨s.session.state.owner.store,[]⟩ pending written) :
 (service s ops).session.state.owner.history =
   s.session.state.owner.history ++ [MixedOwnerAttemptQueue.record written] ∧
 (service s ops).session.state.owner.queued = none := by
 have lower := OwnerStatementEntryGate.execute_committed_complete (live:=view s registry)
   queued packetHeld current waiting same retained materialized admitted meaning
 simpa only [service,packetHeld,queued,select_exact.mpr selected,if_pos current.2.2.2.1] using lower

#print axioms service_changed_empty
#print axioms no_repeated_service
#print axioms service_committed_complete
#print axioms metadata_parts
#print axioms tick_packet
#print axioms rooted_packet_origin
#print axioms rooted_packet_nonempty
#print axioms install_fresh
#print axioms no_registry_reset
#print axioms drive_rooted
#print axioms tick_bank
#print axioms tick_control
#print axioms metadata_other
#print axioms service_no_selection

#print axioms select_exact
#print axioms selected_unique
#print axioms no_substitute
#print axioms set_other
#print axioms service_bank
#print axioms service_route
end MirroreaProofFirst.OwnerStatementRegistrySelection
