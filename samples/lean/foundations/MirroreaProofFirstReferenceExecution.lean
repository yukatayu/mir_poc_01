import MirroreaProofFirstReferenceResult
import MirroreaProofFirstReferenceTrace

namespace MirroreaProofFirst.ReferenceExecution
open ReferenceOwner

-- Every actual core pending request is classified exactly once. A reference
-- request cannot be completed through a caller-supplied unprotected ticket.
-- Constructors are mathematical data; admissible executions start at initial
-- and use the transitions below. There is no arbitrary-state import/restore.
structure Machine (p a : Nat) where
  store : ReferenceStore.Machine p a
  pending : List Pending

def Invariant (m : Machine p a) : Prop :=
  ReferenceCoherence.Invariant m.store ∧ m.pending.map Pending.ticket = m.store.core.pending

def initial (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy) : Machine p a :=
  ⟨ReferenceStore.initial realm view policy,[]⟩

theorem initial_invariant (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy) :
    Invariant (initial (p:=p) realm view policy) := ⟨ReferenceCoherence.initial_invariant _ _ _,rfl⟩

def protectionCheck (m : ReferenceStore.Machine p a) (entry : Pending) : Bool :=
  match entry.binding with | none => true | some binding => ReferenceResult.check m binding entry.ticket

def Protected (m : ReferenceStore.Machine p a) (entry : Pending) : Prop :=
  ∀ binding, entry.binding = some binding → ReferenceResult.Current m binding entry.ticket

theorem protection_exact (m : ReferenceStore.Machine p a) (entry : Pending) :
    protectionCheck m entry = true ↔ Protected m entry := by
  cases h : entry.binding <;> simp [protectionCheck,Protected,h,ReferenceResult.check_exact]

private def beginRequest (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (binding : Option Binding) : Option (Machine p a × Pending) := do
  let (next,ticket) ← ReferenceStore.start m.store member place principal id key argument
  let entry : Pending := ⟨ticket,binding⟩
  if protectionCheck m.store entry then some (⟨next,entry :: m.pending⟩,entry) else none

private theorem begin_parts (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (binding : Option Binding) (result : Machine p a × Pending)
    (accepted : beginRequest m member place principal id key argument binding = some result) :
    ∃ next ticket, ReferenceStore.start m.store member place principal id key argument = some (next,ticket) ∧
      protectionCheck m.store ⟨ticket,binding⟩ = true ∧ result = (⟨next,⟨ticket,binding⟩ :: m.pending⟩,⟨ticket,binding⟩) := by
  unfold beginRequest at accepted
  cases run : ReferenceStore.start m.store member place principal id key argument with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨next,ticket⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some] at accepted
      split at accepted
      · exact ⟨next,ticket,rfl,‹_›,(Option.some.inj accepted).symm⟩
      · cases accepted

private theorem start_pending (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (next : ReferenceStore.Machine p a) (ticket : InvocationBoundary.Ticket)
    (accepted : ReferenceStore.start m member place principal id key argument = some (next,ticket)) :
    next.core.pending = ticket :: m.core.pending := by
  unfold ReferenceStore.start at accepted
  cases run : CompositionMachine.start m.core member place principal id key argument with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨core,saved⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at accepted
      obtain ⟨rfl,rfl⟩ := accepted
      change core.pending = saved :: m.core.pending
      have equal := (CompositionMachine.start_parts _ _ _ _ _ _ _ _ run).2.2
      change core = CompositionMachine.enqueue m.core saved at equal
      rw [equal]
      rfl

private theorem begin_preserves (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (binding : Option Binding) (result : Machine p a × Pending) (valid : Invariant m)
    (accepted : beginRequest m member place principal id key argument binding = some result) : Invariant result.1 := by
  obtain ⟨next,ticket,run,_,rfl⟩ := begin_parts _ _ _ _ _ _ _ _ _ accepted
  refine ⟨ReferenceCoherence.start_preserves _ _ _ _ _ _ _ _ valid.1 run,?_⟩
  simpa [valid.2] using (start_pending _ _ _ _ _ _ _ _ _ run).symm

def startPlain (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) (argument : Int) :
    Option (Machine p a × Pending) := beginRequest m member place principal id key argument none

-- Normalization is a separate committed source microstep. This entry never
-- normalizes a saved reference implicitly, nor consumes its owner's change id.
def startReference (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) (argument : Int) :
    Option (Machine p a × Pending) := do
  let binding ← ReferenceMutation.lookup m.store key
  let choice ← binding.selected
  let option ← binding.request.chain.options[choice.index]?
  beginRequest m member place principal id option.target argument (some binding)

theorem startPlain_preserves (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : Machine p a × Pending) (valid : Invariant m)
    (accepted : startPlain m member place principal id key argument = some result) : Invariant result.1 :=
  begin_preserves _ _ _ _ _ _ _ _ _ valid accepted

theorem startReference_preserves (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : Machine p a × Pending) (valid : Invariant m)
    (accepted : startReference m member place principal id key argument = some result) : Invariant result.1 := by
  unfold startReference at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · dsimp only at accepted
      split at accepted
      · cases accepted
      · exact begin_preserves _ _ _ _ _ _ _ _ _ valid accepted

def keep (entry other : Pending) : Bool :=
  decide (CompositionMachine.ticketId other.ticket ≠ CompositionMachine.ticketId entry.ticket)

def finish (m : Machine p a) (entry : Pending) (value : Int) : Option (Machine p a) := do
  if !m.pending.contains entry || !protectionCheck m.store entry then none else do
    let next ← ReferenceStore.finishPlain m.store entry.ticket value
    return ⟨next,m.pending.filter (keep entry)⟩

theorem finish_parts (m : Machine p a) (entry : Pending) (value : Int) (result : Machine p a)
    (accepted : finish m entry value = some result) :
    entry ∈ m.pending ∧ Protected m.store entry ∧
      ∃ next, ReferenceStore.finishPlain m.store entry.ticket value = some next ∧
        result = ⟨next,m.pending.filter (keep entry)⟩ := by
  unfold finish at accepted
  split at accepted
  · cases accepted
  · rename_i ready
    have readyParts : entry ∈ m.pending ∧ protectionCheck m.store entry = true := by simpa using ready
    cases run : ReferenceStore.finishPlain m.store entry.ticket value with
    | none => simp [run] at accepted
    | some next =>
        simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
        exact ⟨readyParts.1,(protection_exact _ _).mp readyParts.2,next,rfl,accepted.symm⟩

private theorem finish_core (m : ReferenceStore.Machine p a) (ticket : InvocationBoundary.Ticket) (value : Int)
    (next : ReferenceStore.Machine p a) (accepted : ReferenceStore.finishPlain m ticket value = some next) :
    next.core = CompositionMachine.consume m.core ticket value ∧
      InvocationBoundary.ResultMeaning m.core.system.configuration.state m.core.system.view ticket value := by
  unfold ReferenceStore.finishPlain at accepted
  cases run : CompositionMachine.finish m.core ticket value with
  | none => simp [run] at accepted
  | some core =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst next
      exact ⟨(CompositionMachine.finish_parts _ _ _ _ run).2.2.2,(CompositionMachine.finish_current _ _ _ _ run).1⟩

theorem finish_preserves (m : Machine p a) (entry : Pending) (value : Int) (result : Machine p a)
    (valid : Invariant m) (accepted : finish m entry value = some result) : Invariant result := by
  obtain ⟨_,_,next,run,rfl⟩ := finish_parts _ _ _ _ accepted
  refine ⟨ReferenceCoherence.finishPlain_preserves _ _ _ _ valid.1 run,?_⟩
  change (m.pending.filter (keep entry)).map Pending.ticket = next.core.pending
  rw [(finish_core _ _ _ _ run).1]
  simp only [CompositionMachine.consume]
  rw [← valid.2,List.filter_map]
  rfl

theorem finish_meaning (m : Machine p a) (entry : Pending) (value : Int) (result : Machine p a)
    (accepted : finish m entry value = some result) :
    Protected m.store entry ∧
      InvocationBoundary.ResultMeaning m.store.core.system.configuration.state m.store.core.system.view entry.ticket value := by
  obtain ⟨_,guarded,next,run,_⟩ := finish_parts _ _ _ _ accepted
  exact ⟨guarded,(finish_core _ _ _ _ run).2⟩

def finishPlain (m : Machine p a) (ticket : InvocationBoundary.Ticket) (value : Int) : Option (Machine p a) :=
  finish m ⟨ticket,none⟩ value

def resume (m : Machine p a) (entry : Pending) : Option (Machine p a × Int) := do
  let value ← InvocationBoundary.execute entry.ticket
  let next ← finish m entry value
  return (next,value)

private theorem unique_payload (entries : List Pending)
    (unique : (entries.map (fun e => CompositionMachine.ticketId e.ticket)).Nodup)
    (left right : Pending) (lm : left ∈ entries) (rm : right ∈ entries)
    (same : CompositionMachine.ticketId left.ticket = CompositionMachine.ticketId right.ticket) : left = right := by
  induction entries with
  | nil => cases lm
  | cons head tail ih =>
      have nodup := List.nodup_cons.mp unique
      rcases List.mem_cons.mp lm with rfl | leftTail
      · rcases List.mem_cons.mp rm with rfl | rightTail
        · rfl
        · exact False.elim (nodup.1 (List.mem_map.mpr ⟨right,rightTail,same.symm⟩))
      · rcases List.mem_cons.mp rm with rfl | rightTail
        · exact False.elim (nodup.1 (List.mem_map.mpr ⟨left,leftTail,same⟩))
        · exact ih nodup.2 leftTail rightTail

theorem reference_cannot_finish_plain (m : Machine p a) (ticket : InvocationBoundary.Ticket)
    (binding : Binding) (value : Int) (valid : Invariant m)
    (pending : (⟨ticket,some binding⟩ : Pending) ∈ m.pending) : finishPlain m ticket value = none := by
  have nodup : (m.pending.map (fun e => CompositionMachine.ticketId e.ticket)).Nodup := by
    simpa only [← valid.2,List.map_map] using valid.1.1.1.2.1
  have absent : (⟨ticket,none⟩ : Pending) ∉ m.pending := by
    intro member
    have impossible := unique_payload _ nodup ⟨ticket,some binding⟩ ⟨ticket,none⟩ pending member rfl
    cases impossible
  simp [finishPlain,finish,absent]

theorem finish_reference_invalid (m : Machine p a) (entry : Pending) (binding : Binding) (value : Int)
    (bound : entry.binding = some binding) (invalid : ReferenceResult.check m.store binding entry.ticket = false) :
    finish m entry value = none := by simp [finish,protectionCheck,bound,invalid]

theorem no_double_consume (m : Machine p a) (entry other : Pending) (value otherValue : Int) (result : Machine p a)
    (accepted : finish m entry value = some result)
    (same : CompositionMachine.ticketId other.ticket = CompositionMachine.ticketId entry.ticket) :
    finish result other otherValue = none := by
  obtain ⟨_,_,next,run,rfl⟩ := finish_parts _ _ _ _ accepted
  simp [finish,keep,same]

-- Automatic current authorization for the exact saved pending classification.
-- A cancel permission does not require the original invocation/protection to
-- remain live and does not rewrite any current binding or synthesize a result.
def cancel (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (entry : Pending) : Option (Machine p a) := do
  if !m.pending.contains entry then none else do
    let permit ← ReferenceCancellation.authorize m.store.core.system m.store.core.events.length m.store.events.length entry member place principal id
    let next ← ReferenceStore.cancel m.store member place principal id entry permit
    return ⟨next,m.pending.filter (keep entry)⟩

theorem cancel_parts (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (entry : Pending) (result : Machine p a)
    (accepted : cancel m member place principal id entry = some result) :
    entry ∈ m.pending ∧ ∃ permit next,
      ReferenceStore.cancel m.store member place principal id entry permit = some next ∧
      result = ⟨next,m.pending.filter (keep entry)⟩ := by
  unfold cancel at accepted
  split at accepted
  · cases accepted
  · rename_i present
    have mem : entry ∈ m.pending := by simpa using present
    cases auth : ReferenceCancellation.authorize m.store.core.system m.store.core.events.length m.store.events.length entry member place principal id with
    | none => simp [auth] at accepted
    | some permit =>
        simp only [auth,Option.bind_eq_bind,Option.bind_some] at accepted
        cases run : ReferenceStore.cancel m.store member place principal id entry permit with
        | none => simp [run] at accepted
        | some next =>
            simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
            exact ⟨mem,permit,next,run,accepted.symm⟩

theorem cancel_preserves (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (entry : Pending) (result : Machine p a) (valid : Invariant m)
    (accepted : cancel m member place principal id entry = some result) : Invariant result := by
  obtain ⟨_,permit,next,run,rfl⟩ := cancel_parts _ _ _ _ _ _ _ accepted
  refine ⟨ReferenceCoherence.cancel_preserves _ _ _ _ _ _ _ _ valid.1 run,?_⟩
  obtain ⟨_,core,committed,rfl⟩ := ReferenceStore.cancel_parts _ _ _ _ _ _ _ _ run
  obtain ⟨_,_,_,_,rfl⟩ := CompositionMachine.cancel_parts _ _ _ _ _ _ _ _ committed
  change (m.pending.filter (keep entry)).map Pending.ticket =
    m.store.core.pending.filter (fun other => decide (CompositionMachine.ticketId other ≠ CompositionMachine.ticketId entry.ticket))
  rw [← valid.2,List.filter_map]
  rfl

theorem cancel_core (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (entry : Pending) (result : Machine p a) (accepted : cancel m member place principal id entry = some result) :
    ∃ permit, CompositionMachine.cancel m.store.core member place principal id entry.ticket permit = some result.store.core := by
  obtain ⟨_,permit,next,run,rfl⟩ := cancel_parts _ _ _ _ _ _ _ accepted
  obtain ⟨_,core,committed,rfl⟩ := ReferenceStore.cancel_parts _ _ _ _ _ _ _ _ run
  exact ⟨permit.core,committed⟩

theorem cancelled_pending_removed (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (entry other : Pending) (result : Machine p a) (accepted : cancel m member place principal id entry = some result)
    (same : CompositionMachine.ticketId other.ticket = CompositionMachine.ticketId entry.ticket) :
    other ∉ result.pending := by
  obtain ⟨_,permit,next,run,rfl⟩ := cancel_parts _ _ _ _ _ _ _ accepted
  simp [keep,same]

theorem cancelled_cannot_finish (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (entry other : Pending) (result : Machine p a) (value : Int)
    (accepted : cancel m member place principal id entry = some result)
    (same : CompositionMachine.ticketId other.ticket = CompositionMachine.ticketId entry.ticket) :
    finish result other value = none := by
  have absent := cancelled_pending_removed _ _ _ _ _ _ _ _ accepted same
  simp [finish,absent]

private def liftStore (m : Machine p a) (next : ReferenceStore.Machine p a) : Machine p a := ⟨next,m.pending⟩

private theorem lift_preserves (m : Machine p a) (next : ReferenceStore.Machine p a) (valid : Invariant m)
    (step : ReferenceTrace.Transition m.store next) (pending : next.core.pending = m.store.core.pending) :
    Invariant (liftStore m next) :=
  ⟨ReferenceTrace.transition_preserves _ _ valid.1 step,valid.2.trans pending.symm⟩

def manage (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat) (raw : CompositionCore.Raw) :
    Option (Machine p a × Option Nat) := do
  let (next,created) ← ReferenceStore.manage m.store member place principal id raw
  return (liftStore m next,created)

theorem manage_preserves (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : CompositionCore.Raw) (result : Machine p a × Option Nat) (valid : Invariant m)
    (accepted : manage m member place principal id raw = some result) : Invariant result.1 := by
  unfold manage at accepted
  cases run : ReferenceStore.manage m.store member place principal id raw with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨next,created⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      apply lift_preserves _ _ valid (.management run)
      obtain ⟨core,key,coreRun,_,eq⟩ := ReferenceStore.manage_parts _ _ _ _ _ _ _ run
      have nextEq := congrArg (fun pair : ReferenceStore.Machine p a × Option Nat => pair.1.core.pending) eq
      change next.core.pending = core.pending at nextEq
      obtain ⟨_,_,_,_,_,_,coreEq⟩ := CompositionMachine.manage_parts _ _ _ _ _ _ _ coreRun
      have pendingEq := congrArg (fun pair : CompositionMachine.Machine p a × Option Nat => pair.1.pending) coreEq
      exact nextEq.trans pendingEq

def authorityHead (m : Machine p a) (view : WorldProjection.AuthorityView a) : Machine p a :=
  liftStore m (ReferenceStore.authorityHead m.store view)

theorem head_preserves (m : Machine p a) (view : WorldProjection.AuthorityView a) (valid : Invariant m) :
    Invariant (authorityHead m view) := lift_preserves _ _ valid .authority rfl

def acquire (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat) (chain : FallbackStatic.Chain) :
    Option (Machine p a × Nat) := do
  let (next,key) ← ReferenceMutation.acquire m.store member place principal id chain
  return (liftStore m next,key)

theorem acquire_preserves (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (chain : FallbackStatic.Chain) (result : Machine p a × Nat) (valid : Invariant m)
    (accepted : acquire m member place principal id chain = some result) : Invariant result.1 := by
  unfold acquire at accepted
  cases run : ReferenceMutation.acquire m.store member place principal id chain with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨next,key⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      exact lift_preserves _ _ valid (.acquire run) (ReferenceMutation.acquire_pending _ _ _ _ _ _ _ run)

def reacquire (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) : Option (Machine p a) := do
  let next ← ReferenceMutation.reacquire m.store member place principal id key
  return liftStore m next

theorem reacquire_preserves (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : Machine p a) (valid : Invariant m)
    (accepted : reacquire m member place principal id key = some result) : Invariant result := by
  unfold reacquire at accepted
  cases run : ReferenceMutation.reacquire m.store member place principal id key with
  | none => simp [run] at accepted
  | some next =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      exact lift_preserves _ _ valid (.reacquire run) (ReferenceMutation.reacquire_pending _ _ _ _ _ _ _ run)

def release (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) : Option (Machine p a) := do
  let next ← ReferenceMutation.release m.store member place principal id key
  return liftStore m next

theorem release_preserves (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : Machine p a) (valid : Invariant m)
    (accepted : release m member place principal id key = some result) : Invariant result := by
  unfold release at accepted
  cases run : ReferenceMutation.release m.store member place principal id key with
  | none => simp [run] at accepted
  | some next =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      exact lift_preserves _ _ valid (.release run) (ReferenceMutation.release_pending _ _ _ _ _ _ _ run)

structure Outcome (p a : Nat) where
  state : Machine p a
  result : Except ReferenceMutation.Error ReferenceSelection.Choice
  consumed : Bool

def normalize (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) : Outcome p a :=
  let result := ReferenceMutation.normalize m.store member place principal id key
  ⟨liftStore m result.state,result.result,result.consumed⟩

theorem normalize_preserves (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (valid : Invariant m) : Invariant (normalize m member place principal id key).state :=
  lift_preserves _ _ valid .normalize (ReferenceMutation.normalize_pending _ _ _ _ _ _)

theorem normalize_unconsumed (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (unconsumed : (normalize m member place principal id key).consumed = false) :
    (normalize m member place principal id key).state = m := by
  have same := ReferenceMutation.normalize_unconsumed_unchanged _ _ _ _ _ _ unconsumed
  change liftStore m (ReferenceMutation.normalize m.store member place principal id key).state = m
  rw [same]
  rfl

inductive Transition : Machine p a → Machine p a → Prop where
  | management : manage m member place principal requestId raw = some (next,created) → Transition m next
  | authority : Transition m (authorityHead m view)
  | startPlain : startPlain m member place principal requestId key argument = some (next,entry) → Transition m next
  | startReference : startReference m member place principal requestId key argument = some (next,entry) → Transition m next
  | cancellation : cancel m member place principal requestId entry = some next → Transition m next
  | finish : finish m entry value = some next → Transition m next
  | acquire : acquire m member place principal requestId chain = some (next,key) → Transition m next
  | normalize : Transition m (normalize m member place principal requestId key).state
  | reacquire : reacquire m member place principal requestId key = some next → Transition m next
  | release : release m member place principal requestId key = some next → Transition m next

theorem transition_preserves (m next : Machine p a) (valid : Invariant m) (step : Transition m next) : Invariant next := by
  cases step with
  | management run => exact manage_preserves _ _ _ _ _ _ _ valid run
  | authority => exact head_preserves _ _ valid
  | startPlain run => exact startPlain_preserves _ _ _ _ _ _ _ _ valid run
  | startReference run => exact startReference_preserves _ _ _ _ _ _ _ _ valid run
  | cancellation run => exact cancel_preserves _ _ _ _ _ _ _ valid run
  | finish run => exact finish_preserves _ _ _ _ valid run
  | acquire run => exact acquire_preserves _ _ _ _ _ _ _ valid run
  | normalize => exact normalize_preserves _ _ _ _ _ _ valid
  | reacquire run => exact reacquire_preserves _ _ _ _ _ _ _ valid run
  | release run => exact release_preserves _ _ _ _ _ _ _ valid run

private theorem begin_transition (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (binding : Option Binding) (result : Machine p a × Pending)
    (accepted : beginRequest m member place principal id key argument binding = some result) :
    ReferenceTrace.Transition m.store result.1.store := by
  obtain ⟨next,ticket,run,_,rfl⟩ := begin_parts _ _ _ _ _ _ _ _ _ accepted
  exact .start run

theorem transition_projects (m next : Machine p a) (step : Transition m next) :
    ReferenceTrace.Transition m.store next.store := by
  cases step with
  | authority => exact .authority
  | normalize => exact .normalize
  | cancellation run =>
      obtain ⟨_,permit,store,committed,rfl⟩ := cancel_parts _ _ _ _ _ _ _ run
      exact .cancellation committed
  | finish run =>
      obtain ⟨_,_,store,committed,rfl⟩ := finish_parts _ _ _ _ run
      exact .finishPlain committed
  | startPlain run => exact begin_transition _ _ _ _ _ _ _ _ _ run
  | startReference run =>
      unfold startReference at run
      simp only [Option.bind_eq_bind,Option.bind] at run
      split at run
      · cases run
      · dsimp only at run
        split at run
        · cases run
        · dsimp only at run
          split at run
          · cases run
          · exact begin_transition _ _ _ _ _ _ _ _ _ run
  | management run =>
      unfold manage at run
      simp only [Option.bind_eq_bind,Option.bind,Option.pure_def] at run
      split at run
      · cases run
      · rename_i pair committed
        obtain ⟨store,created⟩ := pair
        simp only [Option.some.injEq,Prod.mk.injEq] at run
        obtain ⟨rfl,rfl⟩ := run
        exact .management committed
  | acquire run =>
      unfold acquire at run
      simp only [Option.bind_eq_bind,Option.bind,Option.pure_def] at run
      split at run
      · cases run
      · rename_i pair committed
        obtain ⟨store,key⟩ := pair
        simp only [Option.some.injEq,Prod.mk.injEq] at run
        obtain ⟨rfl,rfl⟩ := run
        exact .acquire committed
  | reacquire run =>
      unfold reacquire at run
      simp only [Option.bind_eq_bind,Option.bind,Option.pure_def] at run
      split at run
      · cases run
      · rename_i store committed
        cases run
        exact .reacquire committed
  | release run =>
      unfold release at run
      simp only [Option.bind_eq_bind,Option.bind,Option.pure_def] at run
      split at run
      · cases run
      · rename_i store committed
        cases run
        exact .release committed

inductive Reached : Machine p a → Machine p a → Prop where
  | refl : Reached m m
  | step {first m next : Machine p a} : Reached first m → Transition m next → Reached first next

theorem reached_preserves (first next : Machine p a) (valid : Invariant first) (path : Reached first next) :
    Invariant next := by
  induction path with
  | refl => exact valid
  | step _ step ih => exact transition_preserves _ _ ih step

theorem reached_projects (first next : Machine p a) (path : Reached first next) :
    ReferenceTrace.Reached first.store next.store := by
  induction path with
  | refl => exact .refl
  | step _ step ih => exact .step ih (transition_projects _ _ step)

theorem reached_slots (first next : Machine p a) (valid : Invariant first) (path : Reached first next) (key : Nat) :
    ReferenceTrace.SlotProgress first.store.bindings[key]? next.store.bindings[key]? :=
  ReferenceTrace.reached_slots _ _ valid.1 (reached_projects _ _ path) key

private theorem store_start_core (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (next : ReferenceStore.Machine p a) (ticket : InvocationBoundary.Ticket)
    (accepted : ReferenceStore.start m member place principal id key argument = some (next,ticket)) :
    CompositionMachine.start m.core member place principal id key argument = some (next.core,ticket) := by
  unfold ReferenceStore.start at accepted
  cases run : CompositionMachine.start m.core member place principal id key argument with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨core,saved⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at accepted
      obtain ⟨rfl,rfl⟩ := accepted
      rfl

theorem startPlain_core (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : Machine p a × Pending)
    (accepted : startPlain m member place principal id key argument = some result) :
    CompositionMachine.start m.store.core member place principal id key argument =
      some (result.1.store.core,result.2.ticket) := by
  obtain ⟨next,ticket,run,_,rfl⟩ := begin_parts _ _ _ _ _ _ _ _ _ accepted
  exact store_start_core _ _ _ _ _ _ _ _ _ run

theorem startReference_core (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : Machine p a × Pending)
    (accepted : startReference m member place principal id key argument = some result) :
    ∃ target, CompositionMachine.start m.store.core member place principal id target argument =
      some (result.1.store.core,result.2.ticket) := by
  unfold startReference at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · dsimp only at accepted
      split at accepted
      · cases accepted
      · obtain ⟨next,ticket,run,_,rfl⟩ := begin_parts _ _ _ _ _ _ _ _ _ accepted
        exact ⟨_,store_start_core _ _ _ _ _ _ _ _ _ run⟩

theorem finish_core_run (m : Machine p a) (entry : Pending) (value : Int) (next : Machine p a)
    (accepted : finish m entry value = some next) :
    CompositionMachine.finish m.store.core entry.ticket value = some next.store.core := by
  obtain ⟨_,_,store,run,rfl⟩ := finish_parts _ _ _ _ accepted
  unfold ReferenceStore.finishPlain at run
  cases completed : CompositionMachine.finish m.store.core entry.ticket value with
  | none => simp [completed] at run
  | some core =>
      simp only [completed,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at run
      subst store
      rfl

theorem manage_core (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : CompositionCore.Raw) (result : Machine p a × Option Nat)
    (accepted : manage m member place principal id raw = some result) :
    CompositionMachine.manage m.store.core member place principal id raw = some (result.1.store.core,result.2) := by
  unfold manage at accepted
  cases run : ReferenceStore.manage m.store member place principal id raw with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨next,created⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      obtain ⟨core,key,coreRun,_,equal⟩ := ReferenceStore.manage_parts _ _ _ _ _ _ _ run
      have nextEq := congrArg Prod.fst equal
      have keyEq := congrArg Prod.snd equal
      change next = ReferenceStore.advanceCore m.store core _ at nextEq
      change created = key at keyEq
      simp only [liftStore]
      rw [nextEq,keyEq]
      exact coreRun

theorem acquire_core (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (chain : FallbackStatic.Chain) (result : Machine p a × Nat)
    (accepted : acquire m member place principal id chain = some result) :
    result.1.store.core = ReferenceMutation.postCore m.store principal id := by
  unfold acquire at accepted
  cases run : ReferenceMutation.acquire m.store member place principal id chain with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨next,key⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      exact ReferenceMutation.acquire_core _ _ _ _ _ _ _ run

theorem reacquire_core (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : Machine p a) (accepted : reacquire m member place principal id key = some result) :
    result.store.core = ReferenceMutation.postCore m.store principal id := by
  unfold reacquire at accepted
  cases run : ReferenceMutation.reacquire m.store member place principal id key with
  | none => simp [run] at accepted
  | some next =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      exact ReferenceMutation.reacquire_core _ _ _ _ _ _ _ run

theorem release_core (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : Machine p a) (accepted : release m member place principal id key = some result) :
    result.store.core = ReferenceMutation.postCore m.store principal id := by
  unfold release at accepted
  cases run : ReferenceMutation.release m.store member place principal id key with
  | none => simp [run] at accepted
  | some next =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      exact ReferenceMutation.release_core _ _ _ _ _ _ _ run

theorem normalize_consumed_core (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (consumed : (normalize m member place principal id key).consumed = true) :
    (normalize m member place principal id key).state.store.core = ReferenceMutation.postCore m.store principal id :=
  ReferenceMutation.normalize_consumed_core _ _ _ _ _ _ consumed

-- Source continuation consumer: exact classified pending shape at admission.
theorem startPlain_pending (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : Machine p a × Pending)
    (accepted : startPlain m member place principal id key argument = some result) :
    result.1.pending = result.2 :: m.pending ∧ result.2.binding = none := by
  obtain ⟨next,ticket,_,_,rfl⟩ := begin_parts _ _ _ _ _ _ _ _ _ accepted
  exact ⟨rfl,rfl⟩

theorem startReference_pending (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : Machine p a × Pending)
    (accepted : startReference m member place principal id key argument = some result) :
    result.1.pending = result.2 :: m.pending ∧ ∃ binding,
      ReferenceMutation.lookup m.store key = some binding ∧ result.2.binding = some binding := by
  unfold startReference at accepted
  cases bound : ReferenceMutation.lookup m.store key with
  | none => simp [bound] at accepted
  | some binding =>
      simp only [bound,Option.bind_eq_bind,Option.bind_some] at accepted
      cases selected : binding.selected with
      | none => simp [selected] at accepted
      | some choice =>
          simp only [selected,Option.bind_some] at accepted
          cases indexed : binding.request.chain.options[choice.index]? with
          | none => simp [indexed] at accepted
          | some option =>
              simp only [indexed,Option.bind_some] at accepted
              obtain ⟨next,ticket,_,_,rfl⟩ := begin_parts _ _ _ _ _ _ _ _ _ accepted
              exact ⟨rfl,binding,rfl,rfl⟩

theorem manage_pending (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : CompositionCore.Raw) (result : Machine p a × Option Nat)
    (accepted : manage m member place principal id raw = some result) :
    result.1.pending = m.pending := by
  unfold manage at accepted
  cases run : ReferenceStore.manage m.store member place principal id raw with
  | none => simp [run] at accepted
  | some next =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      rfl

theorem acquire_pending (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (chain : FallbackStatic.Chain) (result : Machine p a × Nat)
    (accepted : acquire m member place principal id chain = some result) :
    result.1.pending = m.pending := by
  unfold acquire at accepted
  cases run : ReferenceMutation.acquire m.store member place principal id chain with
  | none => simp [run] at accepted
  | some next =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      rfl

theorem reacquire_pending (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (key : Nat) (result : Machine p a)
    (accepted : reacquire m member place principal id key = some result) :
    result.pending = m.pending := by
  unfold reacquire at accepted
  cases run : ReferenceMutation.reacquire m.store member place principal id key with
  | none => simp [run] at accepted
  | some next =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      rfl

theorem release_pending (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (key : Nat) (result : Machine p a)
    (accepted : release m member place principal id key = some result) :
    result.pending = m.pending := by
  unfold release at accepted
  cases run : ReferenceMutation.release m.store member place principal id key with
  | none => simp [run] at accepted
  | some next =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      rfl

theorem normalize_pending (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) :
    (normalize m member place principal id key).state.pending = m.pending := rfl

#print axioms startPlain_pending
#print axioms startReference_pending
#print axioms manage_pending
#print axioms acquire_pending
#print axioms reacquire_pending
#print axioms release_pending
#print axioms normalize_pending
#print axioms startPlain_core
#print axioms startReference_core
#print axioms finish_core_run
#print axioms manage_core
#print axioms acquire_core
#print axioms reacquire_core
#print axioms release_core
#print axioms normalize_consumed_core
#print axioms transition_preserves
#print axioms transition_projects
#print axioms reached_preserves
#print axioms reached_projects
#print axioms reached_slots
#print axioms cancel_preserves
#print axioms cancelled_cannot_finish
#print axioms initial_invariant
#print axioms protection_exact
#print axioms begin_parts
#print axioms startPlain_preserves
#print axioms startReference_preserves
#print axioms finish_parts
#print axioms finish_preserves
#print axioms finish_meaning
#print axioms reference_cannot_finish_plain
#print axioms finish_reference_invalid
#print axioms no_double_consume
#print axioms manage_preserves
#print axioms head_preserves
#print axioms acquire_preserves
#print axioms reacquire_preserves
#print axioms release_preserves
#print axioms normalize_preserves
#print axioms normalize_unconsumed
-- Exact source-consumer occurrence exports. These are consequences of the
-- same executable entries, with no extra events or trusted observer callbacks.
private theorem begin_event (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (binding : Option Binding) (result : Machine p a × Pending)
    (accepted : beginRequest m member place principal id key argument binding = some result) :
    ∃ ticket, result.1.store.events = .core (.requested ticket) :: m.store.events := by
  obtain ⟨next,ticket,run,_,rfl⟩ := begin_parts _ _ _ _ _ _ _ _ _ accepted
  simp only [ReferenceStore.start,Option.bind_eq_bind,Option.bind,Option.pure_def] at run
  split at run
  · cases run
  · rename_i pair started
    obtain ⟨core,ticket'⟩ := pair
    simp only [Option.some.injEq,Prod.mk.injEq] at run
    obtain ⟨rfl,rfl⟩ := run
    exact ⟨ticket',rfl⟩

theorem startPlain_event (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : Machine p a × Pending)
    (accepted : startPlain m member place principal id key argument = some result) :
    ∃ ticket, result.1.store.events = .core (.requested ticket) :: m.store.events :=
  begin_event _ _ _ _ _ _ _ _ _ accepted

theorem startReference_event (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : Machine p a × Pending)
    (accepted : startReference m member place principal id key argument = some result) : ∃ ticket, result.1.store.events = .core (.requested ticket) :: m.store.events := by
  unfold startReference at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · dsimp only at accepted
      split at accepted
      · cases accepted
      · exact begin_event _ _ _ _ _ _ _ _ _ accepted

theorem finish_event (m : Machine p a) (entry : Pending) (value : Int) (result : Machine p a)
    (accepted : finish m entry value = some result) :
    result.store.events = .core (.result entry.ticket value) :: m.store.events := by
  obtain ⟨_,_,store,run,rfl⟩ := finish_parts _ _ _ _ accepted
  simp only [ReferenceStore.finishPlain,Option.bind_eq_bind,Option.bind,Option.pure_def] at run
  split at run
  · cases run
  · cases run
    rfl

theorem cancel_event (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (entry : Pending) (result : Machine p a) (accepted : cancel m member place principal id entry = some result) :
    ∃ permit, result.store.events = .cancellation permit :: m.store.events := by
  obtain ⟨_,permit,store,run,rfl⟩ := cancel_parts _ _ _ _ _ _ _ accepted
  obtain ⟨_,core,_,rfl⟩ := ReferenceStore.cancel_parts _ _ _ _ _ _ _ _ run
  exact ⟨permit,rfl⟩

theorem manage_event (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : CompositionCore.Raw) (result : Machine p a × Option Nat)
    (accepted : manage m member place principal id raw = some result) : ∃ context created, result.1.store.events = .core (.management context created) :: m.store.events := by
  unfold manage at accepted
  cases run : ReferenceStore.manage m.store member place principal id raw with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨next,created⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      obtain ⟨core,created,_,_,equal⟩ := ReferenceStore.manage_parts _ _ _ _ _ _ _ run
      obtain ⟨rfl,rfl⟩ := Prod.mk.inj equal
      exact ⟨_,_,rfl⟩

theorem acquire_event (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (chain : FallbackStatic.Chain) (result : Machine p a × Nat)
    (accepted : acquire m member place principal id chain = some result) : ∃ context next, context.change = .acquire next ∧ result.1.store.events = .binding context :: m.store.events := by
  unfold acquire at accepted
  cases run : ReferenceMutation.acquire m.store member place principal id chain with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨next,key⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      obtain ⟨context,⟨next,kind⟩,events⟩ := ReferenceMutation.acquire_event _ _ _ _ _ _ _ run
      exact ⟨context,next,kind,events⟩

theorem reacquire_event (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : Machine p a)
    (accepted : reacquire m member place principal id key = some result) : ∃ context old next, context.change = .reacquire old next ∧ result.store.events = .binding context :: m.store.events := by
  unfold reacquire at accepted
  cases run : ReferenceMutation.reacquire m.store member place principal id key with
  | none => simp [run] at accepted
  | some next =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      obtain ⟨context,⟨old,next,kind⟩,events⟩ := ReferenceMutation.reacquire_event _ _ _ _ _ _ _ run
      exact ⟨context,old,next,kind,events⟩

theorem release_event (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : Machine p a)
    (accepted : release m member place principal id key = some result) : ∃ context old, context.change = .release old ∧ result.store.events = .binding context :: m.store.events := by
  unfold release at accepted
  cases run : ReferenceMutation.release m.store member place principal id key with
  | none => simp [run] at accepted
  | some next =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      obtain ⟨context,⟨old,kind⟩,events⟩ := ReferenceMutation.release_event _ _ _ _ _ _ _ run
      exact ⟨context,old,kind,events⟩

theorem normalize_event (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (consumed : (normalize m member place principal id key).consumed = true) :
    ∃ context old next, context.change = .degrade old next ∧
      (normalize m member place principal id key).state.store.events = .binding context :: m.store.events :=
  ReferenceMutation.normalize_consumed_event _ _ _ _ _ _ consumed

#print axioms startPlain_event
#print axioms startReference_event
#print axioms finish_event
#print axioms cancel_event
#print axioms manage_event
#print axioms acquire_event
#print axioms reacquire_event
#print axioms release_event
#print axioms normalize_event
end MirroreaProofFirst.ReferenceExecution
