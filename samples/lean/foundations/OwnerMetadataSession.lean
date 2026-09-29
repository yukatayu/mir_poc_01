import OwnerMetadataSource
import OwnerMetadataPending
import MixedOwnerContinuation
namespace MirroreaProofFirst.OwnerMetadataSession
open OwnerMetadataPending

-- One retained existing Session, not a second source/owner scheduler. This
-- bounded connection uses one registry attachment; multi-owner selection and
-- authenticated schema installation remain separate consumer obligations.
structure State (p a : Nat) where
 session : MixedOwnerContinuation.Session p a
 registry : OwnerMetadataRegistry.Registry
 addresses : OwnerStructuredKeys.Addresses
 packet : Option Packet

def management (s : State p a) : OwnerMetadataManagement.State p a :=
 OwnerMetadataStore.state s.session.state.source.machine.store s.registry

-- Definition/place come from the actual saved operation in the SAME catalog.
-- The caller cannot supply a shorter list of fields or a separate body.
def bindWaiting (addresses : OwnerStructuredKeys.Addresses) (registry : OwnerMetadataRegistry.Registry)
 (session : MixedOwnerContinuation.Session p a) : Option Packet := do
 let waiting ← MixedOwnerSourceIssue.ownerWaiting session.state.source.waiting
 let system := session.state.source.machine.store.core.system
 let pending ← OwnerSavedPending.materialize (WorldProjection.size a p system.configuration.count) waiting.saved
 let .operation key place := WorldProjection.decode pending.request.operation.key | none
 let .owner definition := MixedCatalogService.operationDefinition system.configuration.state key | none
 if !MixedOperationDefinitions.ownerCheck definition then none else
 OwnerMetadataPending.bind addresses ⟨system,registry⟩ place definition waiting.saved

def WaitingBinds (addresses : OwnerStructuredKeys.Addresses) (registry : OwnerMetadataRegistry.Registry)
 (session : MixedOwnerContinuation.Session p a) (packet : Packet) : Prop :=
 ∃ (waiting : MixedOwnerSourceIssue.OwnerWaiting) (pending : OwnerEffectService.Pending
     (WorldProjection.size a p session.state.source.machine.store.core.system.configuration.count))
   (key : Fin session.state.source.machine.store.core.system.configuration.count) (place : Fin p)
   (definition : MixedOperationDefinitions.OwnerDefinition),
 MixedOwnerSourceIssue.ownerWaiting session.state.source.waiting = some waiting ∧
 OwnerSavedPending.save pending = waiting.saved ∧
 WorldProjection.decode pending.request.operation.key = .operation key place ∧
 MixedCatalogService.operationDefinition session.state.source.machine.store.core.system.configuration.state key = .owner definition ∧
 MixedOperationDefinitions.OwnerSatisfies definition ∧
 OwnerMetadataPending.Binds addresses ⟨session.state.source.machine.store.core.system,registry⟩ place definition waiting.saved packet

theorem bind_waiting_exact {p a : Nat} {session : MixedOwnerContinuation.Session p a} :
 bindWaiting addresses registry session = some packet ↔ WaitingBinds addresses registry session packet := by
 constructor
 · intro accepted
   unfold bindWaiting at accepted
   cases held : MixedOwnerSourceIssue.ownerWaiting session.state.source.waiting with
   | none => simp [held] at accepted
   | some waiting =>
     simp only [held,Option.bind_eq_bind,Option.bind_some] at accepted
     cases loaded : OwnerSavedPending.materialize
       (WorldProjection.size a p session.state.source.machine.store.core.system.configuration.count) waiting.saved with
     | none => simp [loaded] at accepted
     | some pending =>
       simp only [loaded,Option.bind_some] at accepted
       split at accepted
       · rename_i key place decoded
         split at accepted
         · rename_i definition kind
           split at accepted
           · cases accepted
           · rename_i typed
             have satisfies : MixedOperationDefinitions.OwnerSatisfies definition := by
               apply MixedOperationDefinitions.owner_check_exact.mp
               simpa using typed
             exact ⟨waiting,pending,key,place,definition,held,OwnerSavedPending.materialize_exact.mp loaded,
               decoded,kind,satisfies,OwnerMetadataPending.bind_exact.mp accepted⟩
         · cases accepted
       · cases accepted
 · rintro ⟨waiting,pending,key,place,definition,held,saved,decoded,kind,typed,bound⟩
   have loaded := OwnerSavedPending.materialize_exact.mpr saved
   simp [bindWaiting,held,loaded,decoded,kind,MixedOperationDefinitions.owner_check_exact.mpr typed,
     OwnerMetadataPending.bind_exact.mpr bound]

theorem bound_rows_length (bound : RowsBind addresses registry owner fields out) : out.length = fields.length := by
 induction bound with
 | nil => rfl
 | cons _ _ ih => simp only [List.length_cons,ih]

-- Nonvacuous origin: a produced packet cannot omit ALL target/read bindings.
-- The independent checked definition requires its target in the field table.
theorem bound_waiting_nonempty (accepted : bindWaiting addresses registry session = some packet) :
 packet.fields ≠ [] := by
 obtain ⟨_,_,_,_,definition,_,_,_,_,typed,_,_,fields,rows,rfl⟩ := bind_waiting_exact.mp accepted
 intro empty
 change fields = [] at empty
 have length := bound_rows_length rows
 have fieldsEmpty : definition.contract.fields = [] := by
   apply List.length_eq_zero_iff.mp
   rw [empty] at length
   exact length.symm
 have target := typed.2.1
 simp [fieldsEmpty] at target

-- Pure logical preparation of ONE existing source step; no backend is called.
-- If a newly issued owner request cannot bind metadata, do not publish that
-- tentative enqueue/event. Retain prior committed state and attempted pc floor.
-- A physical implementation owes one admission cut before queue publication.
def refused (s : State p a) (candidate : MixedOwnerContinuation.Session p a) : State p a :=
 {s with session := {s.session with status := .failed .rejected,control := candidate.control}}

def tick (s : State p a) (member : Fin a) (principal : Nat) : State p a :=
 let candidate := MixedOwnerContinuation.tick s.session member principal
 match MixedOwnerSourceIssue.ownerWaiting candidate.state.source.waiting with
 | none => {s with session := candidate,packet := none}
 | some waiting =>
   match MixedOwnerSourceIssue.ownerWaiting s.session.state.source.waiting with
   | some prior =>
     match s.packet with
     | some packet =>
       if packet.saved = waiting.saved ∧ prior.saved = waiting.saved then {s with session := candidate}
       else refused s candidate
     | none => refused s candidate
   | none =>
     match bindWaiting s.addresses s.registry candidate with
     | none => refused s candidate
     | some packet => {s with session := candidate,packet := some packet}

def transfer (s : State p a) : Option (State p a) :=
 (MixedOwnerContinuation.transfer s.session).map fun next => {s with session := next}

def service (s : State p a) (ops : FallibleFlow.Arithmetic) : State p a :=
 match s.packet,s.session.state.owner.queued with
 | some packet,some saved =>
   match CompositionCore.index p packet.owner with
   | none => s
   | some place =>
     if OwnerMetadataPending.check s.addresses (management s) place saved packet then
       {s with session := (MixedOwnerContinuation.service s.session ops
         (currentFields s.addresses s.registry place.val))}
     else s
 | _,_ => s

def changeMetadata (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
 (site : ReferenceSourceData.Site) (change : OwnerMetadataRegistry.Change) : Option (State p a) := do
 let (source,registry) ← OwnerMetadataSource.changeMetadata s.session.state.source s.registry member place principal site change
 return {s with session := {s.session with state := {s.session.state with source := source}},registry := registry}

def authorityHead (s : State p a) (view : WorldProjection.AuthorityView a) : Option (State p a) :=
 (MixedOwnerContinuation.authorityHead s.session view).map fun next => {s with session := next}
def controlInput (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (raw : MixedCompositionCore.Raw) :
 Option (State p a × Option Nat) :=
 (MixedOwnerContinuation.controlInput s.session member place principal raw).map fun (next,key) => ({s with session := next},key)
def continueWith (s : State p a) (program : MixedOwnerProgram.Program p) : Option (State p a) :=
 (MixedOwnerContinuation.continueWith s.session program).map fun next => {s with session := next}
def replaceResidual (s : State p a) (program : MixedOwnerProgram.Program p) : Option (State p a) :=
 (MixedOwnerContinuation.replaceResidual s.session program).map fun next => {s with session := next}
def cancel (s : State p a) (member : Fin a) (principal : Nat) : State p a :=
 {s with session := MixedOwnerContinuation.cancel s.session member principal}

def drive : Nat → State p a → Fin a → Nat → State p a
 | 0,s,_,_ => s
 | fuel+1,s,member,principal => drive fuel (tick s member principal) member principal

-- Initial registry is explicit trusted input here; empty packet admission does
-- not authenticate its schema, initial classifications, ownership or authority.
def attach (session : MixedOwnerContinuation.Session p a) (registry : OwnerMetadataRegistry.Registry)
 (addresses : OwnerStructuredKeys.Addresses) : Option (State p a) :=
 if MixedOwnerContinuation.drained session then some ⟨session,registry,addresses,none⟩ else none

theorem bind_waiting_saved {p a : Nat} {session : MixedOwnerContinuation.Session p a}
 (accepted : bindWaiting addresses registry session = some packet) :
 ∃ waiting, MixedOwnerSourceIssue.ownerWaiting session.state.source.waiting = some waiting ∧
 packet.saved = waiting.saved := by
 unfold bindWaiting at accepted
 cases held : MixedOwnerSourceIssue.ownerWaiting session.state.source.waiting with
 | none => simp [held] at accepted
 | some waiting =>
   simp only [held,Option.bind_eq_bind,Option.bind_some] at accepted
   cases loaded : OwnerSavedPending.materialize
     (WorldProjection.size a p session.state.source.machine.store.core.system.configuration.count) waiting.saved with
   | none => simp [loaded] at accepted
   | some pending =>
     simp only [loaded,Option.bind_some] at accepted
     split at accepted
     · split at accepted
       · split at accepted
         · cases accepted
         · obtain ⟨_,_,_,_,same⟩ := OwnerMetadataPending.bind_exact.mp accepted
           exact ⟨waiting,rfl,congrArg Packet.saved same⟩
       · cases accepted
     · cases accepted

theorem tick_control : s.session.control ≤ (tick s member principal).session.control := by
 unfold tick
 dsimp only
 split
 · exact MixedOwnerContinuation.tick_control_monotone (s:=s.session) (member:=member) (principal:=principal)
 · split
   · split
     · split <;> exact MixedOwnerContinuation.tick_control_monotone (s:=s.session) (member:=member) (principal:=principal)
     · exact MixedOwnerContinuation.tick_control_monotone (s:=s.session) (member:=member) (principal:=principal)
   · split <;> exact MixedOwnerContinuation.tick_control_monotone (s:=s.session) (member:=member) (principal:=principal)

theorem metadata_keeps_packet (accepted : changeMetadata s member place principal site change = some next) :
 next.packet = s.packet ∧ next.addresses = s.addresses ∧
 next.session.state.owner = s.session.state.owner ∧
 next.session.state.outbox = s.session.state.outbox ∧ next.session.state.inbox = s.session.state.inbox := by
 unfold changeMetadata at accepted
 cases run : OwnerMetadataSource.changeMetadata s.session.state.source s.registry member place principal site change with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨source,registry⟩ := pair
   simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
   subst next
   exact ⟨rfl,rfl,rfl,rfl,rfl⟩

theorem tick_packet_origin (kept : (tick s member principal).packet = some packet) :
 s.packet = some packet ∨
 (MixedOwnerSourceIssue.ownerWaiting s.session.state.source.waiting = none ∧
 bindWaiting s.addresses s.registry (MixedOwnerContinuation.tick s.session member principal) = some packet) := by
 unfold tick at kept
 dsimp only at kept
 split at kept
 · cases kept
 · split at kept
   · split at kept
     · split at kept <;> exact Or.inl kept
     · exact Or.inl kept
   · rename_i noPrior
     split at kept
     · exact Or.inl kept
     · rename_i found
       exact Or.inr ⟨noPrior,by simpa only [Option.some.injEq] using found.trans kept⟩

theorem transfer_packet (accepted : transfer s = some next) : next.packet = s.packet := by
 unfold transfer at accepted
 cases run : MixedOwnerContinuation.transfer s.session with
 | none => simp [run] at accepted
 | some session => simp only [run,Option.map_some,Option.some.injEq] at accepted; subst next; rfl

theorem service_packet : (service s ops).packet = s.packet := by
 unfold service
 split
 · split
   · rfl
   · split <;> rfl
 · rfl

theorem authority_packet (accepted : authorityHead s view = some next) : next.packet = s.packet := by
 unfold authorityHead at accepted
 cases run : MixedOwnerContinuation.authorityHead s.session view with
 | none => simp [run] at accepted
 | some session => simp only [run,Option.map_some,Option.some.injEq] at accepted; subst next; rfl

theorem control_packet (accepted : controlInput s member place principal raw = some (next,key)) : next.packet = s.packet := by
 unfold controlInput at accepted
 cases run : MixedOwnerContinuation.controlInput s.session member place principal raw with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨session,created⟩ := pair
   simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
   obtain ⟨rfl,rfl⟩ := accepted
   rfl

theorem continued_packet (accepted : continueWith s program = some next) : next.packet = s.packet := by
 unfold continueWith at accepted
 cases run : MixedOwnerContinuation.continueWith s.session program with
 | none => simp [run] at accepted
 | some session => simp only [run,Option.map_some,Option.some.injEq] at accepted; subst next; rfl

theorem replaced_packet (accepted : replaceResidual s program = some next) : next.packet = s.packet := by
 unfold replaceResidual at accepted
 cases run : MixedOwnerContinuation.replaceResidual s.session program with
 | none => simp [run] at accepted
 | some session => simp only [run,Option.map_some,Option.some.injEq] at accepted; subst next; rfl

-- Closed entry relation for this bounded connection. Images/raw packet or
-- populated State imports are deliberately not entries; they still owe custody
-- and provenance checks before a future physical/admitted constructor exists.
inductive Rooted (initial : State p a) : State p a → Prop where
 | initial : Rooted initial initial
 | tick : Rooted initial s → Rooted initial (tick s member principal)
 | transfer : Rooted initial s → transfer s = some next → Rooted initial next
 | service : Rooted initial s → Rooted initial (service s ops)
 | metadata : Rooted initial s → changeMetadata s member place principal site change = some next → Rooted initial next
 | authority : Rooted initial s → authorityHead s view = some next → Rooted initial next
 | control : Rooted initial s → controlInput s member place principal raw = some (next,key) → Rooted initial next
 | continued : Rooted initial s → continueWith s program = some next → Rooted initial next
 | replaced : Rooted initial s → replaceResidual s program = some next → Rooted initial next
 | cancel : Rooted initial s → Rooted initial (cancel s member principal)

def Produced (initial : State p a) (packet : Packet) : Prop :=
 ∃ before member principal, Rooted initial before ∧
 MixedOwnerSourceIssue.ownerWaiting before.session.state.source.waiting = none ∧
 bindWaiting before.addresses before.registry (MixedOwnerContinuation.tick before.session member principal) = some packet

theorem rooted_packet_origin (empty : initial.packet = none) (path : Rooted initial s)
 (kept : s.packet = some packet) : Produced initial packet := by
 induction path with
 | initial => rw [empty] at kept; cases kept
 | @tick s member principal prior ih =>
   rcases tick_packet_origin kept with old | fresh
   · exact ih old
   · exact ⟨s,member,principal,prior,fresh⟩
 | transfer _ accepted ih => exact ih ((transfer_packet accepted).symm.trans kept)
 | service _ ih => exact ih (service_packet.symm.trans kept)
 | metadata _ accepted ih => exact ih ((metadata_keeps_packet accepted).1.symm.trans kept)
 | authority _ accepted ih => exact ih ((authority_packet accepted).symm.trans kept)
 | control _ accepted ih => exact ih ((control_packet accepted).symm.trans kept)
 | continued _ accepted ih => exact ih ((continued_packet accepted).symm.trans kept)
 | replaced _ accepted ih => exact ih ((replaced_packet accepted).symm.trans kept)
 | cancel _ ih => exact ih kept

theorem service_route : service s ops = s ∨
 ∃ packet saved place, s.packet = some packet ∧ s.session.state.owner.queued = some saved ∧
 OwnerMetadataPending.Current s.addresses (management s) place saved packet ∧
 (service s ops).session = MixedOwnerContinuation.service s.session ops
   (currentFields s.addresses s.registry place.val) := by
 unfold service
 split
 · rename_i packet saved packets queued
   split
   · exact Or.inl rfl
   · rename_i place indexed
     split
     · rename_i checked
       exact Or.inr ⟨packet,saved,place,packets,queued,OwnerMetadataPending.check_exact.mp checked,rfl⟩
     · exact Or.inl rfl
 · exact Or.inl rfl

theorem rooted_service_binding (empty : initial.packet = none) (path : Rooted initial s)
 (changed : service s ops ≠ s) :
 ∃ packet saved place, Produced initial packet ∧ s.packet = some packet ∧
 s.session.state.owner.queued = some saved ∧
 OwnerMetadataPending.Current s.addresses (management s) place saved packet := by
 obtain ⟨packet,saved,place,held,queued,current,_⟩ := service_route.resolve_left changed
 exact ⟨packet,saved,place,rooted_packet_origin empty path held,held,queued,current⟩

theorem rooted_no_empty_packet (empty : initial.packet = none) (path : Rooted initial s)
 (held : s.packet = some packet) : packet.fields ≠ [] := by
 obtain ⟨_,_,_,_,_,bound⟩ := rooted_packet_origin empty path held
 exact bound_waiting_nonempty bound

theorem empty_packet_not_admitted (empty : initial.packet = none) (held : s.packet = some packet)
 (noFields : packet.fields = []) : ¬Rooted initial s := by
 intro path
 exact rooted_no_empty_packet empty path held noFields

theorem drive_rooted (path : Rooted initial s) : Rooted initial (drive fuel s member principal) := by
 induction fuel generalizing s with
 | zero => exact path
 | succ fuel ih => exact ih (.tick path)

#print axioms bind_waiting_saved
#print axioms bind_waiting_exact
#print axioms bound_waiting_nonempty
#print axioms tick_control
#print axioms metadata_keeps_packet
#print axioms tick_packet_origin
#print axioms rooted_packet_origin
#print axioms service_route
#print axioms rooted_service_binding
#print axioms rooted_no_empty_packet
#print axioms empty_packet_not_admitted
end MirroreaProofFirst.OwnerMetadataSession
