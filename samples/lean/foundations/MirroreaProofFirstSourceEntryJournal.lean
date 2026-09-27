import MirroreaProofFirstOwnerLeasePrefix
namespace MirroreaProofFirst.SourceEntryJournal
open OwnerCommitJournal
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- One source_send entry interval, under the existing outer call gate. This
-- journal starts AFTER claim returns and follows a known typed reply through
-- the local writer notification/cancellation and source snapshot assignment.
-- Raw/unknown IO and physical lock/control flow remain the separate wire/host
-- refinement obligations. Neither a source-visible API nor an authority grant.
structure Context (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  before : SharedWireLifetime.History base
  endpoint : Fin p
  ticket : InvocationBoundary.Ticket
  vector : Vector (Fin 513) p
  staged : Memory p a
  oldSnapshot : Option (SourcePublicationWorker.PrivateOutput p a)

def Context.request (c : Context base) : SourceFundingInput.Request p a :=
  (c.vector,.step (.enter c.endpoint))

def Context.status (c : Context base) (history : SharedWireLifetime.History base) : PublicationCapacityDriver.Status :=
  (SourceFundingInput.execute assigned scope bootstrap (SharedWireLifetime.native history.current).driver c.request).2

def snapshot (history : SharedWireLifetime.History base) : Option (SourcePublicationWorker.PrivateOutput p a) :=
  (SharedWireLifetime.native history.current).driver.source.map SourcePublicationWorker.project

inductive Phase where
  | awaitingReply | acceptedUnnotified | refusedUncancelled
  | acceptedNotified | refusedCancelled | acceptedStored | refusedStored
  deriving DecidableEq, BEq
structure State (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  history : SharedWireLifetime.History base
  writer : Memory p a
  sourceSnapshot : Option (SourcePublicationWorker.PrivateOutput p a)
  phase : Phase
  retired : Bool

def initial (c : Context base) : State base := ⟨c.before,c.staged,c.oldSnapshot,.awaitingReply,false⟩
inductive Action where
  | reply (bytes : List UInt8) | notify | cancel | storeSnapshot | retire

-- Branches are selected from the actual known native source transition; an
-- incoming success flag or entered=false cannot select the cancel branch.
def advance (c : Context base) (s : State base) (action : Action) : Option (State base) :=
  if s.retired then none else
  match action,s.phase with
  | .reply bytes,.awaitingReply =>
    match SharedWireLifetime.History.knownStep s.history (.source (.inr (.inr c.request))) bytes with
    | none => none
    | some later => some {s with history:=later,phase:=if c.status s.history = .accepted then .acceptedUnnotified else .refusedUncancelled}
  | .notify,.acceptedUnnotified =>
    (OwnerLeasePrefix.entered s.writer c.ticket).map fun writer => {s with writer:=writer,phase:=.acceptedNotified}
  | .cancel,.refusedUncancelled =>
    (OwnerLeasePrefix.cancel s.writer).map fun writer => {s with writer:=writer,phase:=.refusedCancelled}
  | .storeSnapshot,.acceptedNotified => some {s with sourceSnapshot:=snapshot s.history,phase:=.acceptedStored}
  | .storeSnapshot,.refusedCancelled => some {s with sourceSnapshot:=snapshot s.history,phase:=.refusedStored}
  | .retire,_ => some {s with retired:=true}
  | _,_ => none

inductive Step (c : Context base) : State base → Action → State base → Prop where
  | replyAccepted (live : s.retired = false) (phase : s.phase = .awaitingReply)
      (known : SharedWireLifetime.History.knownStep s.history (.source (.inr (.inr c.request))) bytes = some later)
      (accepted : c.status s.history = .accepted) :
      Step c s (.reply bytes) {s with history:=later,phase:=.acceptedUnnotified}
  | replyRefused (live : s.retired = false) (phase : s.phase = .awaitingReply)
      (known : SharedWireLifetime.History.knownStep s.history (.source (.inr (.inr c.request))) bytes = some later)
      (refused : c.status s.history ≠ .accepted) :
      Step c s (.reply bytes) {s with history:=later,phase:=.refusedUncancelled}
  | notify (live : s.retired = false) (phase : s.phase = .acceptedUnnotified)
      (notified : OwnerLeasePrefix.entered s.writer c.ticket = some writer) :
      Step c s .notify {s with writer:=writer,phase:=.acceptedNotified}
  | cancel (live : s.retired = false) (phase : s.phase = .refusedUncancelled)
      (cancelled : OwnerLeasePrefix.cancel s.writer = some writer) :
      Step c s .cancel {s with writer:=writer,phase:=.refusedCancelled}
  | storeAccepted (live : s.retired = false) (phase : s.phase = .acceptedNotified) :
      Step c s .storeSnapshot {s with sourceSnapshot:=snapshot s.history,phase:=.acceptedStored}
  | storeRefused (live : s.retired = false) (phase : s.phase = .refusedCancelled) :
      Step c s .storeSnapshot {s with sourceSnapshot:=snapshot s.history,phase:=.refusedStored}
  | retire (live : s.retired = false) : Step c s .retire {s with retired:=true}

theorem advance_complete (step : Step c s action next) : advance c s action = some next := by
  cases step <;> simp_all [advance]

theorem advance_sound (checked : advance c s action = some next) : Step c s action next := by
  by_cases live : s.retired = false
  · cases action with
    | reply bytes =>
      cases phase : s.phase <;> simp only [advance,live,phase,Bool.false_eq_true,↓reduceIte] at checked
      all_goals try contradiction
      cases known : SharedWireLifetime.History.knownStep s.history (.source (.inr (.inr c.request))) bytes with
      | none => simp [known] at checked
      | some later =>
        simp only [known] at checked
        by_cases accepted : c.status s.history = .accepted
        · simp only [accepted,↓reduceIte] at checked
          cases Option.some.inj checked; simpa only [live] using Step.replyAccepted live phase known accepted
        · simp only [accepted,↓reduceIte] at checked
          cases Option.some.inj checked; simpa only [live] using Step.replyRefused live phase known accepted
    | notify =>
      cases phase : s.phase <;> simp only [advance,live,phase,Bool.false_eq_true,↓reduceIte] at checked
      all_goals try contradiction
      cases got : OwnerLeasePrefix.entered s.writer c.ticket with
      | none => simp [got] at checked
      | some writer =>
        simp only [got,Option.map_some] at checked
        cases Option.some.inj checked; simpa only [live] using Step.notify live phase got
    | cancel =>
      cases phase : s.phase <;> simp only [advance,live,phase,Bool.false_eq_true,↓reduceIte] at checked
      all_goals try contradiction
      cases got : OwnerLeasePrefix.cancel s.writer with
      | none => simp [got] at checked
      | some writer =>
        simp only [got,Option.map_some] at checked
        cases Option.some.inj checked; simpa only [live] using Step.cancel (c:=c) live phase got
    | storeSnapshot =>
      cases phase : s.phase <;> simp only [advance,live,phase,Bool.false_eq_true,↓reduceIte] at checked
      all_goals try contradiction
      all_goals cases Option.some.inj checked
      · simpa only [live] using Step.storeAccepted (c:=c) live phase
      · simpa only [live] using Step.storeRefused (c:=c) live phase
    | retire =>
      cases phase : s.phase <;> simp only [advance,live,phase,Bool.false_eq_true,↓reduceIte] at checked
      all_goals cases Option.some.inj checked; simpa only [phase] using Step.retire (c:=c) live
  · have stopped : s.retired = true := by cases flag : s.retired <;> simp_all
    simp [advance,stopped] at checked

theorem advance_exact : advance c s action = some next ↔ Step c s action next :=
  ⟨advance_sound,advance_complete⟩

-- Independent provenance carried by each post-reply phase. The history is
-- the exact knownStep from this interval's pre-entry history, not a merely
-- equal snapshot or a new unrelated state with the same projection.
def Known (c : Context base) (s : State base) : Prop :=
  ∃ bytes, SharedWireLifetime.History.knownStep c.before (.source (.inr (.inr c.request))) bytes = some s.history

def Invariant (c : Context base) (s : State base) : Prop :=
  s.writer.data = c.staged.data ∧
  match s.phase with
  | .awaitingReply => s.history = c.before ∧ s.writer.lease = some c.ticket ∧ s.writer.entered = false ∧ s.sourceSnapshot = c.oldSnapshot
  | .acceptedUnnotified => Known c s ∧ c.status c.before = .accepted ∧ s.writer.lease = some c.ticket ∧ s.writer.entered = false ∧ s.sourceSnapshot = c.oldSnapshot
  | .refusedUncancelled => Known c s ∧ c.status c.before ≠ .accepted ∧ s.writer.lease = some c.ticket ∧ s.writer.entered = false ∧ s.sourceSnapshot = c.oldSnapshot
  | .acceptedNotified => Known c s ∧ c.status c.before = .accepted ∧ s.writer.lease = some c.ticket ∧ s.writer.entered = true ∧ s.sourceSnapshot = c.oldSnapshot
  | .refusedCancelled => Known c s ∧ c.status c.before ≠ .accepted ∧ s.writer.lease = none ∧ s.writer.entered = false ∧ s.sourceSnapshot = c.oldSnapshot
  | .acceptedStored => Known c s ∧ c.status c.before = .accepted ∧ s.writer.lease = some c.ticket ∧ s.writer.entered = true ∧ s.sourceSnapshot = snapshot s.history
  | .refusedStored => Known c s ∧ c.status c.before ≠ .accepted ∧ s.writer.lease = none ∧ s.writer.entered = false ∧ s.sourceSnapshot = snapshot s.history

theorem initial_valid (held : c.staged.lease = some c.ticket) (inactive : c.staged.entered = false) :
    Invariant c (initial c) := by simp [Invariant,initial,held,inactive]

theorem preserves (valid : Invariant c s) (step : Step c s action next) : Invariant c next := by
  obtain ⟨data,bound⟩ := valid
  cases step with
  | replyAccepted live phase known accepted =>
    simp only [Invariant,phase] at bound ⊢
    obtain ⟨history,held,inactive,old⟩ := bound
    exact ⟨data,⟨_,by simpa only [history] using known⟩,by simpa only [history] using accepted,held,inactive,old⟩
  | replyRefused live phase known refused =>
    simp only [Invariant,phase] at bound ⊢
    obtain ⟨history,held,inactive,old⟩ := bound
    exact ⟨data,⟨_,by simpa only [history] using known⟩,by simpa only [history] using refused,held,inactive,old⟩
  | notify live phase notified =>
    obtain ⟨_,_,rfl⟩ := OwnerLeasePrefix.entered_exact.mp notified
    simp only [phase] at bound
    exact ⟨data,bound.1,bound.2.1,bound.2.2.1,rfl,bound.2.2.2.2⟩
  | cancel live phase cancelled =>
    simp only [Invariant,phase] at bound
    unfold OwnerLeasePrefix.cancel at cancelled
    rw [bound.2.2.2.1] at cancelled
    cases Option.some.inj cancelled
    exact ⟨data,bound.1,bound.2.1,rfl,rfl,bound.2.2.2.2⟩
  | storeAccepted live phase =>
    simp only [Invariant,phase] at bound ⊢
    exact ⟨data,bound.1,bound.2.1,bound.2.2.1,bound.2.2.2.1,True.intro⟩
  | storeRefused live phase =>
    simp only [Invariant,phase] at bound ⊢
    exact ⟨data,bound.1,bound.2.1,bound.2.2.1,bound.2.2.2.1,True.intro⟩
  | retire => exact ⟨data,bound⟩

#print axioms advance_exact
#print axioms initial_valid
#print axioms preserves

-- A real successful claim establishes the initial journal obligations. Claim
-- admission still includes the retained keys, capacity and credits predicate.
theorem claimed_initial (made : OwnerLeasePrefix.claim before cap c.ticket = some c.staged) :
    Invariant c (initial c) := by
  have memory := (OwnerLeasePrefix.claim_exact.mp made).2
  apply initial_valid <;> rw [memory] <;> rfl

inductive Runs (c : Context base) (start : State base) : State base → Prop where
  | nil : Runs c start start
  | step : Runs c start s → Step c s action next → Runs c start next

theorem runs_preserves (valid : Invariant c start) (path : Runs c start after) : Invariant c after := by
  induction path with
  | nil => exact valid
  | step prior step ih => exact preserves ih step

theorem stopped_no_action (stopped : s.retired = true) : advance c s action = none := by
  simp [advance,stopped]

theorem retire_retains (step : Step c s .retire next) :
    next.history = s.history ∧ next.writer = s.writer ∧ next.sourceSnapshot = s.sourceSnapshot ∧
    next.phase = s.phase ∧ next.retired = true := by
  cases step; exact ⟨rfl,rfl,rfl,rfl,rfl⟩

theorem accepted_unnotified_no_cancel (phase : s.phase = .acceptedUnnotified) : advance c s .cancel = none := by
  simp [advance,phase]

theorem awaiting_reply_no_cancel (phase : s.phase = .awaitingReply) : advance c s .cancel = none := by
  simp [advance,phase]

theorem cancel_requires_known_refusal (valid : Invariant c s) (step : Step c s .cancel next) :
    Known c s ∧ c.status c.before ≠ .accepted := by
  cases step with
  | cancel live phase cancelled =>
    have fields := valid.2
    simp only [phase] at fields
    exact ⟨fields.1,fields.2.1⟩

theorem cancel_frames_native (valid : Invariant c s) (step : Step c s .cancel next) :
    (SharedWireLifetime.native next.history.current).driver = (SharedWireLifetime.native c.before.current).driver ∧
    (SharedWireLifetime.native next.history.current).owners = (SharedWireLifetime.native c.before.current).owners := by
  obtain ⟨⟨bytes,known⟩,refused⟩ := cancel_requires_known_refusal valid step
  have framed := OwnerLeasePrefix.source_refused_frames known refused
  cases step
  exact framed

theorem snapshot_requires_notification (valid : Invariant c s)
    (step : Step c s .storeSnapshot next) (accepted : c.status c.before = .accepted) :
    s.writer.lease = some c.ticket ∧ s.writer.entered = true := by
  cases step with
  | storeAccepted live phase =>
    have fields := valid.2
    simp only [phase] at fields
    exact ⟨fields.2.2.1,fields.2.2.2.1⟩
  | storeRefused live phase =>
    have fields := valid.2
    simp only [phase] at fields
    exact False.elim (fields.2.1 accepted)

theorem no_premature_snapshot (phase : s.phase = .awaitingReply ∨ s.phase = .acceptedUnnotified ∨ s.phase = .refusedUncancelled) :
    advance c s .storeSnapshot = none := by
  rcases phase with phase | phase | phase <;> simp [advance,phase]

theorem accepted_wire_progress (c : Context base)
    (known : SharedWireLifetime.History.knownStep c.before (.source (.inr (.inr c.request))) bytes = some later)
    (accepted : c.status c.before = .accepted) :
    advance c (initial c) (.reply bytes) = some ⟨later,c.staged,c.oldSnapshot,.acceptedUnnotified,false⟩ := by
  exact advance_complete (.replyAccepted rfl rfl known accepted)

theorem refused_wire_progress (c : Context base)
    (known : SharedWireLifetime.History.knownStep c.before (.source (.inr (.inr c.request))) bytes = some later)
    (refused : c.status c.before ≠ .accepted) :
    advance c (initial c) (.reply bytes) = some ⟨later,c.staged,c.oldSnapshot,.refusedUncancelled,false⟩ := by
  exact advance_complete (.replyRefused rfl rfl known refused)

-- An accepted reply changes no local lease field. In particular entered=false
-- STILL HOLDS in the native-accepted/unnotified state. The phase/known receipt,
-- not that boolean, prevents cancellation and premature snapshot publication.
theorem accepted_unnotified_prefix (c : Context base)
    (inactive : c.staged.entered = false)
    (known : SharedWireLifetime.History.knownStep c.before (.source (.inr (.inr c.request))) bytes = some later)
    (accepted : c.status c.before = .accepted) :
    ∃ s, advance c (initial c) (.reply bytes) = some s ∧ s.writer.entered = false ∧
      advance c s .cancel = none ∧ advance c s .storeSnapshot = none := by
  refine ⟨⟨later,c.staged,c.oldSnapshot,.acceptedUnnotified,false⟩,
    accepted_wire_progress c known accepted,inactive,?_,?_⟩ <;> rfl

theorem notify_then_store_progress (c : Context base) (later : SharedWireLifetime.History base)
    (held : c.staged.lease = some c.ticket) (inactive : c.staged.entered = false) :
    Runs c ⟨later,c.staged,c.oldSnapshot,.acceptedUnnotified,false⟩
      ⟨later,{c.staged with entered:=true},snapshot later,.acceptedStored,false⟩ := by
  have checkedLocal : OwnerLeasePrefix.entered c.staged c.ticket = some {c.staged with entered:=true} :=
    OwnerLeasePrefix.entered_exact.mpr ⟨held,inactive,rfl⟩
  exact (Runs.step Runs.nil (.notify rfl rfl checkedLocal)).step (.storeAccepted rfl rfl)

theorem cancel_then_store_progress (c : Context base) (later : SharedWireLifetime.History base)
    (inactive : c.staged.entered = false) :
    Runs c ⟨later,c.staged,c.oldSnapshot,.refusedUncancelled,false⟩
      ⟨later,{c.staged with lease:=none},snapshot later,.refusedStored,false⟩ := by
  have checkedLocal : OwnerLeasePrefix.cancel c.staged = some {c.staged with lease:=none} := by
    simp [OwnerLeasePrefix.cancel,inactive]
  exact (Runs.step Runs.nil (.cancel rfl rfl checkedLocal)).step (.storeRefused rfl rfl)

#print axioms claimed_initial
#print axioms runs_preserves
#print axioms cancel_frames_native
#print axioms snapshot_requires_notification
#print axioms accepted_unnotified_prefix
#print axioms notify_then_store_progress
#print axioms cancel_then_store_progress

def AcceptedPhase : Phase → Prop
  | .acceptedUnnotified | .acceptedNotified | .acceptedStored => True
  | _ => False

theorem accepted_known (valid : Invariant c s) (phase : AcceptedPhase s.phase) :
    Known c s ∧ c.status c.before = .accepted := by
  have fields := valid.2
  cases phaseAt : s.phase <;> simp only [AcceptedPhase,phaseAt] at phase fields
  all_goals first | contradiction | exact ⟨fields.1,fields.2.1⟩

theorem accepted_dispatch {c : Context base} {s : State base} (valid : Invariant c s) (phase : AcceptedPhase s.phase)
    (present : (SharedWireLifetime.native c.before.current).driver.source = some source)
    (waiting : (SourcePublicationWorker.project source).source.pending = some c.ticket) :
    ∃ enteredSource,
      (SharedWireLifetime.native s.history.current).driver.source = some enteredSource ∧
      enteredSource.dispatch = some ⟨c.endpoint,⟨scope,source.publication.barrier.installed c.endpoint⟩,c.ticket⟩ := by
  obtain ⟨⟨bytes,known⟩,accepted⟩ := accepted_known valid phase
  exact OwnerLeasePrefix.source_accepted_dispatch known present waiting accepted

theorem known_owner_binding (valid : Invariant c s) (known : Known c s)
    (bound : c.staged.data = project ((SharedWireLifetime.native c.before.current).owners c.endpoint)) :
    s.writer.data = project ((SharedWireLifetime.native s.history.current).owners c.endpoint) := by
  obtain ⟨bytes,checked⟩ := known
  rw [valid.1,bound,(OwnerLeasePrefix.source_physics checked).2.1]

#print axioms accepted_dispatch
#print axioms known_owner_binding
end MirroreaProofFirst.SourceEntryJournal
