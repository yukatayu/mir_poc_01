import MixedPureInvocation
import MixedManagementEmbedding
import OwnerSavedPending
import OwnerEffectReceipt
namespace MirroreaProofFirst.MixedRequestCore
open CurrentUse WorldProjection
variable {id : Nat}

-- One retained catalog/system and one pending-ID space. This is the bookkeeping
-- core, not a source entry, physical carrier, owner service queue, or image loader.
inductive Pending where
 | pure (ticket : InvocationBoundary.Ticket)
 | owner (saved : OwnerSavedPending.Saved)
 deriving DecidableEq

def pendingId : Pending → UseId
 | .pure t => CompositionMachine.ticketId t
 | .owner s => ⟨s.request.operation.instanceId,s.request.principal,s.request.request⟩

inductive Occurrence where
 | management (context : MixedManagementEntry.Context) (created : Option Nat)
 | requested (pending : Pending)
 | pureResult (ticket : InvocationBoundary.Ticket) (value : Int)
 | ownerAcknowledged (saved : OwnerSavedPending.Saved) (serviceEvidence : Evidence)
 | cancelled (permit : ReferenceCancellationBoundary.Permit)
 | authorityHead (generation : Nat)
 deriving DecidableEq

structure Machine (p a : Nat) where
 system : MixedManagementEntry.System p a
 pending : List Pending
 events : List Occurrence

def Invariant (m : Machine p a) : Prop :=
 MixedManagementEntry.Invariant m.system ∧ (m.pending.map pendingId).Nodup ∧
 ∀ t ∈ m.pending, pendingId t ∉ m.system.used

def fresh (m : Machine p a) (id : UseId) : Bool :=
 !m.system.used.contains id && !(m.pending.map pendingId).contains id

theorem fresh_exact {id : UseId} : fresh m id = true ↔ id ∉ m.system.used ∧ id ∉ m.pending.map pendingId := by simp [fresh]

def manage (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat) (raw : MixedCompositionCore.Raw) :
 Option (Machine p a × Option Nat) := do
 if !fresh m (MixedManagementEntry.useId m.system principal id) then none else do
 let (system,created) ← MixedManagementEntry.perform m.system member place principal id raw
 return ({m with system := system,events := .management (MixedManagementEntry.current m.system member place principal id raw) created :: m.events},created)

theorem manage_parts (accepted : manage m member place principal id raw = some next) :
 fresh m (MixedManagementEntry.useId m.system principal id) = true ∧
 ∃ e cfg created, MixedManagementEntry.commit m.system member place principal id raw e =
 some (MixedManagementEntry.after m.system cfg principal id,created) ∧
 MixedCompositionCore.run m.system.configuration raw = some (cfg,created) ∧
 next = ({m with
   system := MixedManagementEntry.after m.system cfg principal id,
   events := .management (MixedManagementEntry.current m.system member place principal id raw) created :: m.events},created) := by
 unfold manage at accepted
 split at accepted
 · cases accepted
 · rename_i hf
   have hfresh : fresh m (MixedManagementEntry.useId m.system principal id) = true := by simpa using hf
   unfold MixedManagementEntry.perform at accepted
   cases he : MixedManagementEntry.authorize m.system member place principal id raw with
   | none => simp [he] at accepted
   | some e =>
     simp only [he,Option.bind_eq_bind,Option.bind_some] at accepted
     cases hc : MixedManagementEntry.commit m.system member place principal id raw e with
     | none => simp [hc] at accepted
     | some result =>
       obtain ⟨_,cfg,created,hr,eq⟩ := MixedManagementEntry.commit_parts m.system member place principal id raw e result hc
       subst result
       rw [hc] at accepted
       exact ⟨hfresh,e,cfg,created,hc,hr,(Option.some.inj accepted).symm⟩

theorem manage_preserves (valid : Invariant m) (accepted : manage m member place principal id raw = some next) : Invariant next.1 := by
 obtain ⟨hf,e,cfg,created,hc,_,rfl⟩ := manage_parts accepted
 have free := fresh_exact.mp hf
 refine ⟨(MixedManagementEntry.commit_preserves _ _ _ _ _ _ _ _ valid.1 hc).1,valid.2.1,?_⟩
 intro t ht
 simp only [MixedManagementEntry.after,List.mem_cons,not_or]
 refine ⟨?_,valid.2.2 t ht⟩
 intro same
 exact free.2 (same ▸ List.mem_map.mpr ⟨t,ht,rfl⟩)

def enqueue (m : Machine p a) (t : Pending) : Machine p a :=
 {m with pending := t :: m.pending,events := .requested t :: m.events}

def startPure (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) (arg : Int) :
 Option (Machine p a × InvocationBoundary.Ticket) := do
 let unit ← CompositionCore.index m.system.configuration.count key
 let t ← MixedPureInvocation.prepare m.system.configuration.state m.system.view member unit place principal id arg
 if fresh m (pendingId (.pure t)) then some (enqueue m (.pure t),t) else none

-- Uses supplied source-origin/parsed captures and CURRENT common catalog. Source
-- provenance, actual field/capture labels, resources and admission of the realm
-- are still outer obligations. No free body registry or caller-chosen body.
def prepareOwner (s : MixedInstanceState.State d p n) (v : AuthorityView a) (member : Fin a)
 (key : Fin n) (place : Fin p) (principal id : Nat) (origin : OwnerEffectService.Origin)
 (args : List Scalar) : Option OwnerSavedPending.Saved := do
 let .owner defn := MixedCatalogService.operationDefinition s key | none
 let u := {MixedPureInvocation.request s v member key place principal id 0 with arguments := args}
 let w := MixedCatalogService.world s v
 let e ← CurrentUse.authorize w.authority (w.policies u.operation.key) (currentContext w u)
 if checkUse w u e then some (OwnerSavedPending.save ⟨origin,u,e,defn.body⟩) else none

theorem prepareOwner_parts (h : prepareOwner s v member key place principal id origin args = some saved) :
 ∃ defn e, MixedCatalogService.operationDefinition s key = .owner defn ∧
 checkUse (MixedCatalogService.world s v)
   {MixedPureInvocation.request s v member key place principal id 0 with arguments := args} e = true ∧
 saved = OwnerSavedPending.save ⟨origin,
   {MixedPureInvocation.request s v member key place principal id 0 with arguments := args},e,defn.body⟩ := by
 unfold prepareOwner at h
 cases kind : MixedCatalogService.operationDefinition s key with
 | pure _ => simp [kind] at h
 | owner defn =>
   simp only [kind] at h
   cases ha : CurrentUse.authorize (MixedCatalogService.world s v).authority
     ((MixedCatalogService.world s v).policies
       ({MixedPureInvocation.request s v member key place principal id 0 with arguments := args}).operation.key)
     (currentContext (MixedCatalogService.world s v)
       {MixedPureInvocation.request s v member key place principal id 0 with arguments := args}) with
   | none => simp [ha] at h
   | some e =>
     simp only [ha,Option.bind_eq_bind,Option.bind_some] at h
     split at h
     · exact ⟨defn,e,rfl,‹_›,(Option.some.inj h).symm⟩
     · cases h

theorem prepareOwner_complete (kind : MixedCatalogService.operationDefinition s key = .owner defn)
 (use : CurrentUse.CurrentUse (MixedCatalogService.world s v)
   {MixedPureInvocation.request s v member key place principal id 0 with arguments := args}) :
 ∃ saved, prepareOwner s v member key place principal id origin args = some saved := by
 obtain ⟨e,authorized,checked⟩ := authorize_use_complete _ _ use
 refine ⟨OwnerSavedPending.save ⟨origin,
   {MixedPureInvocation.request s v member key place principal id 0 with arguments := args},e,defn.body⟩,?_⟩
 simp [prepareOwner,kind,authorized,checked]

theorem prepareOwner_id (h : prepareOwner s v member key place principal id origin args = some saved) :
 pendingId (.owner saved) = ⟨s.realm,principal,id⟩ := by
 obtain ⟨_,_,_,_,rfl⟩ := prepareOwner_parts h
 rfl

def startOwner (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
 (origin : OwnerEffectService.Origin) (args : List Scalar) : Option (Machine p a × OwnerSavedPending.Saved) := do
 let unit ← CompositionCore.index m.system.configuration.count key
 let saved ← prepareOwner m.system.configuration.state m.system.view member unit place principal id origin args
 if fresh m (pendingId (.owner saved)) then some (enqueue m (.owner saved),saved) else none

theorem enqueue_preserves (valid : Invariant m) (hf : fresh m (pendingId t) = true) : Invariant (enqueue m t) := by
 obtain ⟨unused,unpending⟩ := fresh_exact.mp hf
 refine ⟨valid.1,List.nodup_cons.mpr ⟨unpending,valid.2.1⟩,?_⟩
 intro other member
 rcases List.mem_cons.mp member with rfl | old
 · exact unused
 · exact valid.2.2 other old

theorem startPure_parts (accepted : startPure m member place principal id key arg = some next) :
 fresh m (pendingId (.pure next.2)) = true ∧
 MixedPureInvocation.check m.system.configuration.state m.system.view next.2 = true ∧
 next.1 = enqueue m (.pure next.2) := by
 unfold startPure at accepted
 cases hi : CompositionCore.index m.system.configuration.count key with
 | none => simp [hi] at accepted
 | some unit =>
   simp only [hi,Option.bind_eq_bind,Option.bind_some] at accepted
   cases hp : MixedPureInvocation.prepare m.system.configuration.state m.system.view member unit place principal id arg with
   | none => simp [hp] at accepted
   | some t =>
     simp only [hp,Option.bind_some] at accepted
     split at accepted
     · cases accepted; exact ⟨‹_›,MixedPureInvocation.prepare_checked hp,rfl⟩
     · cases accepted

theorem startOwner_parts (accepted : startOwner m member place principal id key origin args = some next) :
 fresh m (pendingId (.owner next.2)) = true ∧ next.1 = enqueue m (.owner next.2) := by
 unfold startOwner at accepted
 cases hi : CompositionCore.index m.system.configuration.count key with
 | none => simp [hi] at accepted
 | some unit =>
   simp only [hi,Option.bind_eq_bind,Option.bind_some] at accepted
   cases hp : prepareOwner m.system.configuration.state m.system.view member unit place principal id origin args with
   | none => simp [hp] at accepted
   | some saved =>
     simp only [hp,Option.bind_some] at accepted
     split at accepted
     · cases accepted; exact ⟨‹_›,rfl⟩
     · cases accepted

theorem startPure_preserves (valid : Invariant m) (accepted : startPure m member place principal id key arg = some next) : Invariant next.1 := by
 obtain ⟨hf,_,eq⟩ := startPure_parts accepted
 rw [eq]
 exact enqueue_preserves valid hf

theorem startOwner_preserves (valid : Invariant m) (accepted : startOwner m member place principal id key origin args = some next) : Invariant next.1 := by
 obtain ⟨hf,eq⟩ := startOwner_parts accepted
 rw [eq]
 exact enqueue_preserves valid hf

def finishCheck (m : Machine p a) (t : InvocationBoundary.Ticket) (value : Int) : Bool :=
 m.pending.contains (.pure t) && !m.system.used.contains (CompositionMachine.ticketId t) &&
 MixedPureInvocation.resultCheck m.system.configuration.state m.system.view t value

def consumePure (m : Machine p a) (t : InvocationBoundary.Ticket) (value : Int) : Machine p a :=
 {system := {m.system with serial := m.system.serial+1,used := CompositionMachine.ticketId t :: m.system.used},
  pending := m.pending.filter (fun other => decide (pendingId other ≠ CompositionMachine.ticketId t)),
  events := .pureResult t value :: m.events}

def finishPure (m : Machine p a) (t : InvocationBoundary.Ticket) (value : Int) : Option (Machine p a) :=
 if finishCheck m t value then some (consumePure m t value) else none

theorem finishPure_parts (accepted : finishPure m t value = some next) :
 .pure t ∈ m.pending ∧ CompositionMachine.ticketId t ∉ m.system.used ∧
 MixedPureInvocation.resultCheck m.system.configuration.state m.system.view t value = true ∧
 next = consumePure m t value := by
 unfold finishPure at accepted
 split at accepted
 · rename_i h
   have parts : .pure t ∈ m.pending ∧ CompositionMachine.ticketId t ∉ m.system.used ∧
    MixedPureInvocation.resultCheck m.system.configuration.state m.system.view t value = true := by
     simpa [finishCheck,and_assoc] using h
   cases accepted
   exact ⟨parts.1,parts.2.1,parts.2.2,rfl⟩
 · cases accepted

theorem finishPure_preserves (valid : Invariant m) (accepted : finishPure m t value = some next) : Invariant next := by
 obtain ⟨_,unused,_,rfl⟩ := finishPure_parts accepted
 refine ⟨⟨valid.1.1,List.nodup_cons.mpr ⟨unused,valid.1.2⟩⟩,?_,?_⟩
 · exact List.Nodup.sublist ((List.filter_sublist).map pendingId) valid.2.1
 · intro other member
   have mem : other ∈ m.pending ∧ pendingId other ≠ CompositionMachine.ticketId t := by simpa [consumePure] using member
   simp only [consumePure,List.mem_cons,not_or]
   exact ⟨mem.2,valid.2.2 other mem.1⟩

theorem no_double_consume (same : CompositionMachine.ticketId other = CompositionMachine.ticketId t) :
 finishPure (consumePure m t value) other otherValue = none := by
 simp [finishPure,finishCheck,consumePure,same]

theorem enqueue_blocks (same : pendingId other = pendingId t) : fresh (enqueue m t) (pendingId other) = false := by
 simp [fresh,enqueue,same]

theorem same_id_same_pending (xs : List Pending) (valid : (xs.map pendingId).Nodup)
 (mx : x ∈ xs) (my : y ∈ xs) (same : pendingId x = pendingId y) : x = y := by
 induction xs with
 | nil => simp at mx
 | cons head tail ih =>
   have parts := List.nodup_cons.mp valid
   rcases List.mem_cons.mp mx with sameX | mxTail
   · subst x
     rcases List.mem_cons.mp my with sameY | myTail
     · exact sameY.symm
     · have mem : pendingId y ∈ tail.map pendingId := List.mem_map.mpr ⟨y,myTail,rfl⟩
       exact False.elim (parts.1 (same.symm ▸ mem))
   · rcases List.mem_cons.mp my with sameY | myTail
     · subst y
       have mem : pendingId x ∈ tail.map pendingId := List.mem_map.mpr ⟨x,mxTail,rfl⟩
       exact False.elim (parts.1 (same ▸ mem))
     · exact ih parts.2 mxTail myTail

theorem pure_finish_keeps_owner (valid : Invariant m) (accepted : finishPure m t value = some next)
 (pending : .owner saved ∈ m.pending) : .owner saved ∈ next.pending := by
 obtain ⟨pureMem,_,_,rfl⟩ := finishPure_parts accepted
 have distinct : pendingId (.owner saved) ≠ CompositionMachine.ticketId t := by
   intro same
   have eq := same_id_same_pending m.pending valid.2.1 pending pureMem same
   cases eq
 simpa [consumePure] using And.intro pending distinct

-- Raw head transition preserves stored historical requests and ID bookkeeping;
-- it is NOT an authorization for installing an arbitrary observed head.
def authorityHead (m : Machine p a) (view : AuthorityView a) : Machine p a :=
 {m with system := MixedManagementEntry.installAuthorityHead m.system view,events := .authorityHead view.generation :: m.events}
theorem head_preserves (valid : Invariant m) : Invariant (authorityHead m view) := valid

#print axioms manage_preserves
#print axioms prepareOwner_parts
#print axioms prepareOwner_complete
#print axioms prepareOwner_id
#print axioms pure_finish_keeps_owner
#print axioms startPure_parts
#print axioms startPure_preserves
#print axioms startOwner_parts
#print axioms startOwner_preserves
#print axioms finishPure_preserves
#print axioms no_double_consume
#print axioms head_preserves
-- A unit acknowledgment consumes the exact retained owner request. It neither
-- executes owner arithmetic nor authenticates an arbitrary constructed reply.
-- The enclosing admitted trace must derive the reply from real service history.
def consumeOwner (m : Machine p a) (saved : OwnerSavedPending.Saved) (serviceEvidence : Evidence) : Machine p a :=
 {system := {m.system with serial := m.system.serial+1,used := pendingId (.owner saved) :: m.system.used},
  pending := m.pending.filter (fun other => decide (pendingId other ≠ pendingId (.owner saved))),
  events := .ownerAcknowledged saved serviceEvidence :: m.events}

def finishOwner (m : Machine p a) (saved received : OwnerSavedPending.Saved) (serviceEvidence : Evidence) : Option (Machine p a) := do
 if !m.pending.contains (.owner saved) || m.system.used.contains (pendingId (.owner saved)) then none else do
 let pending ← OwnerSavedPending.materialize (WorldProjection.size a p m.system.configuration.count) saved
 let delivered ← OwnerSavedPending.materialize (WorldProjection.size a p m.system.configuration.count) received
 if OwnerEffectReceipt.ackCheck (MixedCatalogService.world m.system.configuration.state m.system.view)
   pending ⟨delivered,serviceEvidence⟩ then some (consumeOwner m saved serviceEvidence) else none

theorem finishOwner_parts {m next : Machine p a} (accepted : finishOwner m saved received serviceEvidence = some next) :
 .owner saved ∈ m.pending ∧ pendingId (.owner saved) ∉ m.system.used ∧
 ∃ pending delivered,
 OwnerSavedPending.save pending = saved ∧ OwnerSavedPending.save delivered = received ∧
 OwnerEffectReceipt.Accepted (MixedCatalogService.world m.system.configuration.state m.system.view)
   pending ⟨delivered,serviceEvidence⟩ ∧ next = consumeOwner m saved serviceEvidence := by
 unfold finishOwner at accepted
 split at accepted
 · cases accepted
 · rename_i guard
   have allowed : .owner saved ∈ m.pending ∧ pendingId (.owner saved) ∉ m.system.used := by simpa using guard
   cases hp : OwnerSavedPending.materialize (WorldProjection.size a p m.system.configuration.count) saved with
   | none => simp [hp] at accepted
   | some pending =>
     simp only [hp,Option.bind_eq_bind,Option.bind_some] at accepted
     cases hd : OwnerSavedPending.materialize (WorldProjection.size a p m.system.configuration.count) received with
     | none => simp [hd] at accepted
     | some delivered =>
       simp only [hd,Option.bind_some] at accepted
       split at accepted
       · exact ⟨allowed.1,allowed.2,pending,delivered,OwnerSavedPending.materialize_exact.mp hp,
           OwnerSavedPending.materialize_exact.mp hd,OwnerEffectReceipt.ack_exact.mp ‹_›,(Option.some.inj accepted).symm⟩
       · cases accepted

theorem finishOwner_preserves (valid : Invariant m)
 (accepted : finishOwner m saved received serviceEvidence = some next) : Invariant next := by
 obtain ⟨_,unused,_,_,_,_,_,rfl⟩ := finishOwner_parts accepted
 refine ⟨⟨valid.1.1,List.nodup_cons.mpr ⟨unused,valid.1.2⟩⟩,?_,?_⟩
 · exact List.Nodup.sublist ((List.filter_sublist).map pendingId) valid.2.1
 · intro other member
   have mem : other ∈ m.pending ∧ pendingId other ≠ pendingId (.owner saved) := by simpa [consumeOwner] using member
   simp only [consumeOwner,List.mem_cons,not_or]
   exact ⟨mem.2,valid.2.2 other mem.1⟩

theorem no_double_owner_consume (same : pendingId (.owner other) = pendingId (.owner saved)) :
 finishOwner (consumeOwner m saved serviceEvidence) other received later = none := by
 simp [finishOwner,consumeOwner,same]

theorem finishOwner_pure_kept (valid : Invariant m)
 (accepted : finishOwner m saved received evidence = some next) (present : .pure ticket ∈ m.pending) :
 .pure ticket ∈ next.pending := by
 obtain ⟨owner,_,_,_,_,_,_,rfl⟩ := finishOwner_parts accepted
 have distinct : pendingId (.pure ticket) ≠ pendingId (.owner saved) := by
   intro same
   have equal := same_id_same_pending m.pending valid.2.1 present owner same
   cases equal
 simpa [consumeOwner,distinct] using present

#print axioms finishOwner_parts
#print axioms finishOwner_preserves
#print axioms no_double_owner_consume
#print axioms finishOwner_pure_kept
end MirroreaProofFirst.MixedRequestCore
