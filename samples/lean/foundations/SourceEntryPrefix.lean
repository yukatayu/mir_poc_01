import SharedHostPrefix
import SourceEntryCapture
open MirroreaProofFirst
namespace SourceEntryPrefix
open OwnerCommitJournal SharedHostCaptureReplay
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- One normalized prefix owns current complete writer memories. Historical
-- records never substitute for those memories. The wire substate is the SAME
-- existing rooted joined state; known reply and local stores remain separate.
-- This first cut covers known replies. Raw/unresolved physical IO and actual
-- abort-only continuation are further obligations, not fictional guards here.
structure Opened (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  openedAt : Nat
  entry : SourceEntryCapture.Open base
structure Stored (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  openedAt : Nat
  storedAt : Nat
  entry : SourceEntryCapture.Open base

inductive StoreKind where
  | notify | cancel | snapshot
  deriving DecidableEq, BEq
def StoreKind.action : StoreKind → SourceEntryJournal.Action
  | .notify => .notify
  | .cancel => .cancel
  | .snapshot => .storeSnapshot

inductive Event (p a : Nat) where
  | ordinary (event : JoinedEvent p a)
  | claim (endpoint : Fin p) (vector : Vector (Fin 513) p) (before staged : Memory p a)
      (snapshot : Option (SourcePublicationWorker.PrivateOutput p a))
  | reply (input : SourceFundingQuery.CheckedInput p a) (bytes : List UInt8)
  | store (kind : StoreKind) (memory : Memory p a) (snapshot : Option (SourcePublicationWorker.PrivateOutput p a))
  | retire

structure State (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  joined : JoinedState base
  writers : Fin p → Memory p a
  active : Option (Opened base)
  stored : List (Stored base)
  events : List (Event p a)

def State.start (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) : State base :=
  let joined := JoinedState.start base
  ⟨joined,fun i => ⟨project ((SharedWireLifetime.native joined.history.current).owners i),none,false⟩,none,[],[]⟩

def put (writers : Fin p → Memory p a) (endpoint : Fin p) (memory : Memory p a) : Fin p → Memory p a :=
  fun i => if i = endpoint then memory else writers i

def ordinaryAllowed : JoinedEvent p a → Bool
  | .source (.inr (.inr (_,.step (.enter _)))) _ => false
  | .retire => false
  | _ => true

def openingMatches (writers : Fin p → Memory p a) : JoinedEvent p a → Bool
  | .owner endpoint _ _ _ before => memoryEq before (writers endpoint)
  | _ => true

def observedWriters (writers : Fin p → Memory p a) : JoinedEvent p a → Fin p → Memory p a
  | .observe endpoint memory | .confirmed endpoint memory => put writers endpoint memory
  | _ => writers

def inputEq (left right : SourceFundingQuery.CheckedInput p a) : Bool :=
  OwnerPacketCodec.encode (SourceFundingQuery.checkedInput p a) left ==
    OwnerPacketCodec.encode (SourceFundingQuery.checkedInput p a) right

theorem inputEq_exact {left right : SourceFundingQuery.CheckedInput p a} : inputEq left right = true ↔ left = right := by
  constructor
  · intro same
    have bytes : OwnerPacketCodec.encode (SourceFundingQuery.checkedInput p a) left =
        OwnerPacketCodec.encode (SourceFundingQuery.checkedInput p a) right := by simpa [inputEq] using same
    have decoded := congrArg (OwnerPacketCodec.decode (SourceFundingQuery.checkedInput p a)) bytes
    simpa only [OwnerPacketCodec.roundtrip,Option.some.injEq] using decoded
  · intro same; subst right; simp [inputEq]

def vectorMatches (writers : Fin p → Memory p a) (vector : Vector (Fin 513) p) : Bool :=
  (List.finRange p).all (fun i => decide ((vector[i.val]).val = (writers i).data.credits))

def afterStore (s : State base) (opened : Opened base) (next : SourceEntryCapture.Open base)
    (kind : StoreKind) (memory : Memory p a) (snapshot : Option (SourcePublicationWorker.PrivateOutput p a)) : State base :=
  {s with
    writers:=put s.writers opened.entry.context.endpoint memory,
    active:=(match kind with | .snapshot => none | _ => some ⟨opened.openedAt,next⟩),
    stored:=(match kind with | .snapshot => ⟨opened.openedAt,s.events.length,next⟩::s.stored | _ => s.stored),
    events:=s.events++[.store kind memory snapshot]}

def advance (s : State base) (event : Event p a) : Option (State base) :=
  if s.joined.stopped then none else
  match event with
  | .ordinary e =>
    match s.active with
    | some _ => none
    | none =>
      if ordinaryAllowed e && openingMatches s.writers e then
        (advanceJoined s.joined e).map fun joined =>
          {s with joined:=joined,writers:=observedWriters s.writers e,events:=s.events++[event]}
      else none
  | .claim endpoint vector before staged snapshot =>
    match s.active,s.joined.active with
    | none,none =>
      if memoryEq before (s.writers endpoint) && vectorMatches s.writers vector then
        (SourceEntryCapture.start s.joined.history endpoint vector before staged snapshot).map fun opened =>
          {s with writers:=put s.writers endpoint staged,active:=some ⟨s.events.length,opened⟩,events:=s.events++[event]}
      else none
    | _,_ => none
  | .reply input bytes =>
    match s.active with
    | none => none
    | some opened =>
      if inputEq input (.inr (.inr opened.entry.context.request)) then
        match opened.entry.advance (.reply bytes),advanceJoined s.joined (.source input bytes) with
        | some next,some joined => some {s with joined:=joined,active:=some ⟨opened.openedAt,next⟩,events:=s.events++[event]}
        | _,_ => none
      else none
  | .store kind memory snapshot =>
    match s.active with
    | none => none
    | some opened =>
      (opened.entry.observe kind.action memory snapshot).map fun next => afterStore s opened next kind memory snapshot
  | .retire =>
    match advanceJoined s.joined .retire with
    | none => none
    | some joined =>
      match s.active with
      | none => some {s with joined:=joined,events:=s.events++[event]}
      | some opened =>
        (opened.entry.advance .retire).map fun next =>
          {s with joined:=joined,active:=some ⟨opened.openedAt,next⟩,events:=s.events++[event]}

-- These rules use the established lower-layer checks plus operational input
-- equality. They do not ask the desired history/memory invariant as a guard.
inductive Step : State base → Event p a → State base → Prop where
  | ordinary (live : s.joined.stopped = false) (idle : s.active = none)
      (allowed : ordinaryAllowed e = true) (memory : openingMatches s.writers e = true)
      (ran : advanceJoined s.joined e = some joined) :
      Step s (.ordinary e) {s with joined:=joined,writers:=observedWriters s.writers e,events:=s.events++[.ordinary e]}
  | claim (live : s.joined.stopped = false) (idle : s.active = none) (writerIdle : s.joined.active = none)
      (memory : before = s.writers endpoint) (vectorBound : vectorMatches s.writers vector = true)
      (ran : SourceEntryCapture.start s.joined.history endpoint vector before staged snapshot = some opened) :
      Step s (.claim endpoint vector before staged snapshot)
        {s with writers:=put s.writers endpoint staged,active:=some ⟨s.events.length,opened⟩,events:=s.events++[.claim endpoint vector before staged snapshot]}
  | reply (live : s.joined.stopped = false) (active : s.active = some opened)
      (inputBound : input = .inr (.inr opened.entry.context.request))
      (localStep : opened.entry.advance (.reply bytes) = some next)
      (wireStep : advanceJoined s.joined (.source input bytes) = some joined) :
      Step s (.reply input bytes) {s with joined:=joined,active:=some ⟨opened.openedAt,next⟩,events:=s.events++[.reply input bytes]}
  | store (live : s.joined.stopped = false) (active : s.active = some opened)
      (localStep : opened.entry.observe kind.action memory snapshot = some next) :
      Step s (.store kind memory snapshot) (afterStore s opened next kind memory snapshot)
  | retireIdle (live : s.joined.stopped = false) (idle : s.active = none)
      (wireStep : advanceJoined s.joined .retire = some joined) :
      Step s .retire {s with joined:=joined,events:=s.events++[.retire]}
  | retireActive (live : s.joined.stopped = false) (active : s.active = some opened)
      (wireStep : advanceJoined s.joined .retire = some joined)
      (localStep : opened.entry.advance .retire = some next) :
      Step s .retire {s with joined:=joined,active:=some ⟨opened.openedAt,next⟩,events:=s.events++[.retire]}

theorem advance_complete (step : Step s event next) : advance s event = some next := by
  cases step <;> simp_all [advance,memoryEq_exact,inputEq_exact]

theorem advance_sound (checked : advance s event = some next) : Step s event next := by
  unfold advance at checked
  split at checked
  · cases checked
  · rename_i live
    have live : s.joined.stopped = false := by simpa using live
    cases event with
    | ordinary e =>
      cases active : s.active with
      | some opened => simp [active] at checked
      | none =>
        simp only [active] at checked
        split at checked
        · rename_i guard
          have guard : ordinaryAllowed e = true ∧ openingMatches s.writers e = true := by simpa using guard
          cases ran : advanceJoined s.joined e with
          | none => simp [ran] at checked
          | some joined =>
            simp only [ran,Option.map_some] at checked
            cases Option.some.inj checked
            simpa only [active] using Step.ordinary live active guard.1 guard.2 ran
        · cases checked
    | claim endpoint vector before staged snapshot =>
      cases active : s.active <;> cases writerActive : s.joined.active <;> simp only [active,writerActive] at checked
      all_goals try contradiction
      split at checked
      · rename_i guard
        have guard : before = s.writers endpoint ∧ vectorMatches s.writers vector = true := by simpa [memoryEq_exact] using guard
        cases ran : SourceEntryCapture.start s.joined.history endpoint vector before staged snapshot with
        | none => simp [ran] at checked
        | some opened =>
          simp only [ran,Option.map_some] at checked
          cases Option.some.inj checked
          exact .claim live active writerActive guard.1 guard.2 ran
      · cases checked
    | reply input bytes =>
      cases active : s.active with
      | none => simp [active] at checked
      | some opened =>
        simp only [active] at checked
        split at checked
        · rename_i bound
          have bound := inputEq_exact.mp bound
          cases localStep : opened.entry.advance (.reply bytes) <;>
            cases wireStep : advanceJoined s.joined (.source input bytes) <;>
            simp only [localStep,wireStep] at checked
          all_goals try contradiction
          cases Option.some.inj checked
          exact .reply live active bound localStep wireStep
        · cases checked
    | store kind memory snapshot =>
      cases active : s.active with
      | none => simp [active] at checked
      | some opened =>
        simp only [active] at checked
        cases localStep : opened.entry.observe kind.action memory snapshot with
        | none => simp [localStep] at checked
        | some after =>
          simp only [localStep,Option.map_some] at checked
          cases Option.some.inj checked
          exact .store live active localStep
    | retire =>
      cases wireStep : advanceJoined s.joined .retire with
      | none => simp [wireStep] at checked
      | some joined =>
        simp only [wireStep] at checked
        cases active : s.active with
        | none =>
          simp only [active] at checked
          cases Option.some.inj checked
          simpa only [active] using Step.retireIdle live active wireStep
        | some opened =>
          simp only [active] at checked
          cases localStep : opened.entry.advance .retire with
          | none => simp [localStep] at checked
          | some after =>
            simp only [localStep,Option.map_some] at checked
            cases Option.some.inj checked
            exact .retireActive live active wireStep localStep

theorem advance_exact : advance s event = some next ↔ Step s event next := ⟨advance_sound,advance_complete⟩

#print axioms advance_exact

def Event.projection : Event p a → List (JoinedEvent p a)
  | .ordinary e => [e]
  | .reply input bytes => [.source input bytes]
  | .retire => [.retire]
  | _ => []

theorem step_projection (step : Step s event next) : JoinedRuns s.joined event.projection next.joined := by
  cases step with
  | ordinary live idle allowed memory ran => exact .step .nil (SharedHostCaptureReplay.advance_sound ran)
  | claim => exact .nil
  | reply live active bound localStep wireStep => exact .step .nil (SharedHostCaptureReplay.advance_sound wireStep)
  | store => exact .nil
  | retireIdle live idle wireStep => exact .step .nil (SharedHostCaptureReplay.advance_sound wireStep)
  | retireActive live active wireStep localStep => exact .step .nil (SharedHostCaptureReplay.advance_sound wireStep)

theorem step_events (step : Step s event next) : next.events = s.events ++ [event] := by cases step <;> rfl

theorem joined_extends (step : JoinedStep before event after) : Extends before.history after.history := by
  cases step with
  | source live idle ran => exact known_extends ran
  | owner live idle ran => exact open_extends ran
  | observe => exact .refl _
  | confirmed => exact .refl _
  | localComplete live idle ran => exact finish_extends ran
  | retire => exact .refl _

theorem step_extends (step : Step s event next) : Extends s.joined.history next.joined.history := by
  cases step with
  | ordinary live idle allowed memory ran => exact joined_extends (SharedHostCaptureReplay.advance_sound ran)
  | claim => exact .refl _
  | reply live active bound localStep wireStep => exact joined_extends (SharedHostCaptureReplay.advance_sound wireStep)
  | store => exact .refl _
  | retireIdle live idle wireStep => exact joined_extends (SharedHostCaptureReplay.advance_sound wireStep)
  | retireActive live active wireStep localStep => exact joined_extends (SharedHostCaptureReplay.advance_sound wireStep)

def ActiveValid (s : State base) : Prop :=
  ∀ opened, s.active = some opened →
    opened.entry.state.history = s.joined.history ∧
    s.writers opened.entry.context.endpoint = opened.entry.state.writer ∧ s.joined.active = none
def StoredValid (s : State base) : Prop :=
  ∀ stored ∈ s.stored, Extends stored.entry.state.history s.joined.history
def Invariant (s : State base) : Prop := ActiveValid s ∧ StoredValid s

theorem initial_valid : Invariant (State.start base) := by simp [Invariant,ActiveValid,StoredValid,State.start]

theorem local_history {kind : StoreKind} (step : SourceEntryJournal.Step context before kind.action after) : after.history = before.history := by
  cases kind <;> cases step <;> rfl

theorem reply_facts (step : SourceEntryJournal.Step context before (.reply bytes) after) :
    SharedWireLifetime.History.knownStep before.history (.source (.inr (.inr context.request))) bytes = some after.history ∧
    after.writer = before.writer := by
  cases step <;> exact ⟨by assumption,rfl⟩

theorem reply_binding {opened : Opened base} {s : State base} {entry : SourceEntryCapture.Open base}
    {joined : JoinedState base} (history : opened.entry.state.history = s.joined.history)
    (inputBound : input = .inr (.inr opened.entry.context.request))
    (localStep : opened.entry.advance (.reply bytes) = some entry)
    (wireStep : advanceJoined s.joined (.source input bytes) = some joined) :
    entry.state.history = joined.history ∧ entry.state.writer = opened.entry.state.writer ∧
    entry.context = opened.entry.context ∧ joined.active = none := by
  obtain ⟨context,localStep⟩ := SourceEntryCapture.Open.advance_sound localStep
  have wireStep := SharedHostCaptureReplay.advance_sound wireStep
  cases wireStep with
  | source live idle known =>
    obtain ⟨ran,memory⟩ := reply_facts localStep
    rw [history,←inputBound] at ran
    exact ⟨Option.some.inj (ran.symm.trans known),memory,context,idle⟩

theorem preserves_active (valid : ActiveValid s) (step : Step s event next) : ActiveValid next := by
  cases step with
  | ordinary live idle allowed memory ran => simp [ActiveValid,idle]
  | claim live idle writerIdle memory vectorBound ran =>
    obtain ⟨allowed,atHistory,endpoint,vector,initial,staged,before,snapshot⟩ := SourceEntryCapture.start_sound ran
    intro op equal; cases Option.some.inj equal
    simp only [initial,SourceEntryJournal.initial]
    exact ⟨atHistory,by simp [put,endpoint,staged],writerIdle⟩
  | reply live active inputBound localStep wireStep =>
    obtain ⟨history,memory,idle⟩ := valid _ active
    obtain ⟨nextHistory,nextMemory,context,nextIdle⟩ := reply_binding history inputBound localStep wireStep
    intro op equal; cases Option.some.inj equal
    exact ⟨nextHistory,by rw [context,memory,nextMemory],nextIdle⟩
  | store live active localStep =>
    rename_i opened next kind observed snapshot
    obtain ⟨history,memory,idle⟩ := valid _ active
    obtain ⟨context,localStep,afterMemory,afterSnapshot⟩ := SourceEntryCapture.Open.observe_sound localStep
    have nextHistory := (local_history localStep).trans history
    cases kind with
    | snapshot => simp [ActiveValid,afterStore]
    | notify | cancel =>
      intro op equal; cases Option.some.inj equal
      exact ⟨nextHistory,by simp [afterStore,put,context,afterMemory],idle⟩
  | retireIdle live idle wireStep => simp [ActiveValid,idle]
  | retireActive live active wireStep localStep =>
    obtain ⟨history,memory,idle⟩ := valid _ active
    obtain ⟨context,localStep⟩ := SourceEntryCapture.Open.advance_sound localStep
    have fields := SourceEntryJournal.retire_retains localStep
    have wireStep := SharedHostCaptureReplay.advance_sound wireStep
    cases wireStep
    intro op equal; cases Option.some.inj equal
    exact ⟨fields.1.trans history,by rw [context,fields.2.1]; exact memory,idle⟩

theorem preserves_stored (activeValid : ActiveValid s) (valid : StoredValid s)
    (step : Step s event next) : StoredValid next := by
  have extension := step_extends step
  have carry : ∀ old ∈ s.stored, Extends old.entry.state.history next.joined.history :=
    fun old member => (valid old member).trans extension
  cases step with
  | ordinary => exact carry
  | claim => exact carry
  | reply => exact carry
  | retireIdle => exact carry
  | retireActive => exact carry
  | store live active localStep =>
    rename_i opened next kind observed snapshot
    obtain ⟨history,memory,idle⟩ := activeValid _ active
    obtain ⟨context,localStep,afterMemory,afterSnapshot⟩ := SourceEntryCapture.Open.observe_sound localStep
    have nextHistory := (local_history localStep).trans history
    cases kind with
    | notify | cancel => exact carry
    | snapshot =>
      intro old member
      rcases List.mem_cons.mp member with equal | oldMember
      · subst old
        rw [nextHistory]
        exact .refl _
      · exact carry _ oldMember

theorem preserves (valid : Invariant s) (step : Step s event next) : Invariant next :=
  ⟨preserves_active valid.1 step,preserves_stored valid.1 valid.2 step⟩

-- Full memory equality is required at owner opening, including lease/entered.
-- Equal native Data or an equal-valued unrelated historical record cannot
-- replace the current memory retained by the preceding source-entry stores.
theorem owner_open_memory (step : Step s (.ordinary (.owner endpoint command paid bytes memory)) next) :
    memory = s.writers endpoint := by
  cases step with
  | ordinary live idle allowed binding ran => exact memoryEq_exact.mp binding

theorem source_frames_writers (step : Step s (.ordinary (.source input bytes)) next) : next.writers = s.writers := by
  cases step; rfl

theorem owner_rejects_lease_erasure (held : (s.writers endpoint).lease = some ticket)
    (erased : memory.lease = none) :
    advance s (.ordinary (.owner endpoint command paid bytes memory)) = none := by
  cases checked : advance s (.ordinary (.owner endpoint command paid bytes memory)) with
  | none => rfl
  | some next =>
    have equal := owner_open_memory (advance_sound checked)
    rw [equal,held] at erased
    cases erased

#print axioms step_projection
#print axioms preserves
#print axioms owner_rejects_lease_erasure

theorem step_retains_stored (step : Step s event next) : ∃ fresh, next.stored = fresh ++ s.stored := by
  cases step
  all_goals try exact ⟨[],rfl⟩
  rename_i opened after kind memory snapshot live active ran
  cases kind
  all_goals first | exact ⟨[],rfl⟩ | exact ⟨[_],rfl⟩

theorem retire_keeps_writers (step : Step s .retire next) : next.writers = s.writers ∧ next.stored = s.stored := by
  cases step <;> exact ⟨rfl,rfl⟩

inductive Runs : State base → List (Event p a) → State base → Prop where
  | nil : Runs s [] s
  | step : Runs first earlier s → Step s event next → Runs first (earlier++[event]) next

theorem Runs.append (left : Runs before earlier middle) (right : Runs middle later after) :
    Runs before (earlier++later) after := by
  induction right with
  | nil => simpa using left
  | step prior step ih => simpa only [List.append_assoc] using Runs.step ih step

theorem runs_events (path : Runs before events after) : after.events = before.events ++ events := by
  induction path with
  | nil => simp
  | step prior step ih => rw [step_events step,ih,List.append_assoc]

theorem runs_projection (path : Runs before events after) :
    JoinedRuns before.joined (events.flatMap Event.projection) after.joined := by
  induction path with
  | nil => exact .nil
  | step prior step ih =>
    simpa only [List.flatMap_append,List.flatMap_cons,List.flatMap_nil,List.append_nil] using
      ih.append (step_projection step)

theorem runs_preserves (valid : Invariant before) (path : Runs before events after) : Invariant after := by
  induction path with
  | nil => exact valid
  | step prior step ih => exact preserves ih step

theorem runs_retains_stored (path : Runs before events after) : ∃ fresh, after.stored = fresh ++ before.stored := by
  induction path with
  | nil => exact ⟨[],rfl⟩
  | step prior step ih =>
    obtain ⟨old,left⟩ := ih
    obtain ⟨new,right⟩ := step_retains_stored step
    exact ⟨new++old,by rw [right,left,List.append_assoc]⟩

structure Certified (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  state : State base
  path : Runs (.start base) state.events state

def Certified.start (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) : Certified base :=
  ⟨.start base,.nil⟩

def Certified.advance (before : Certified base) (event : Event p a) : Option (Certified base) :=
  match checked : SourceEntryPrefix.advance before.state event with
  | none => none
  | some next =>
    let step := advance_sound checked
    some ⟨next,by rw [step_events step]; exact .step before.path step⟩

theorem Certified.valid (checked : Certified base) : Invariant checked.state := runs_preserves initial_valid checked.path

theorem Certified.joined_path (checked : Certified base) :
    JoinedRuns (JoinedState.start base) (checked.state.events.flatMap Event.projection) checked.state.joined :=
  runs_projection checked.path

theorem Certified.joined_valid (checked : Certified base) : HistoryInvariant checked.state.joined :=
  runs_history SharedHostCaptureReplay.initial_valid checked.joined_path

theorem Certified.exact_actions (checked : Certified base) :
    checked.state.joined.history.actions = (checked.state.events.flatMap Event.projection).flatMap JoinedEvent.actions := by
  simpa only [JoinedState.start,SharedWireLifetime.History.start,List.nil_append] using runs_actions checked.joined_path

theorem Certified.advance_step (checked : Certified.advance before event = some after) :
    Step before.state event after.state := by
  unfold Certified.advance at checked
  split at checked
  · cases checked
  · rename_i next ran
    cases Option.some.inj checked
    exact advance_sound ran

def Certified.consume (before : Certified base) : List (Event p a) → Option (Certified base)
  | [] => some before
  | event::rest => do
    let next ← before.advance event
    next.consume rest

theorem Certified.consume_sound (checked : Certified.consume before events = some after) :
    after.state.events = before.state.events ++ events ∧ Runs before.state events after.state := by
  induction events generalizing before after with
  | nil =>
    simp only [Certified.consume,Option.some.injEq] at checked
    subst after
    exact ⟨by simp,.nil⟩
  | cons event rest ih =>
    cases ran : before.advance event with
    | none => simp [Certified.consume,ran] at checked
    | some next =>
      simp only [Certified.consume,ran] at checked
      obtain ⟨tailEvents,tailPath⟩ := ih checked
      have step := Certified.advance_step ran
      refine ⟨?_,?_⟩
      · rw [tailEvents,step_events step]
        simp only [List.append_assoc,List.singleton_append]
      · simpa only [List.nil_append,List.singleton_append] using (Runs.step Runs.nil step).append tailPath

#print axioms Certified.valid
#print axioms Certified.joined_valid
#print axioms Certified.exact_actions
#print axioms Certified.consume_sound
#print axioms runs_retains_stored
#print axioms retire_keeps_writers

theorem vectorMatches_exact : vectorMatches writers vector = true ↔
    ∀ i : Fin p, (vector[i.val]).val = (writers i).data.credits := by
  simp [vectorMatches,List.all_eq_true]

theorem claim_frames_others (different : other ≠ endpoint)
    (step : Step s (.claim endpoint vector before staged snapshot) next) :
    next.writers other = s.writers other := by
  cases step
  simp [put,different]

theorem snapshot_recorded (step : Step s (.store .snapshot memory snapshot) next) :
    ∃ opened stored, s.active = some opened ∧ next.stored = stored::s.stored ∧
      stored.openedAt = opened.openedAt ∧ stored.storedAt = s.events.length ∧
      stored.entry.context = opened.entry.context ∧
      next.writers opened.entry.context.endpoint = stored.entry.state.writer := by
  cases step with
  | store live active ran =>
    have facts := SourceEntryCapture.Open.observe_sound ran
    refine ⟨_,⟨_,_,_⟩,active,rfl,rfl,rfl,facts.1,?_⟩
    simpa only [afterStore,put,↓reduceIte] using facts.2.2.1.symm

#print axioms vectorMatches_exact
#print axioms claim_frames_others
#print axioms snapshot_recorded

-- Parameter-general lifting has no product admission predicate in its
-- premises. It rules out co-restricting the checker and Step to a fixture's
-- number of owners while retaining apparent relative exactness.
theorem owner_open_lift {s : State base} (live : s.joined.stopped = false)
    (idle : s.active = none) (memory : observed = s.writers endpoint)
    (lower : JoinedStep s.joined (.owner endpoint command paid bytes observed) joined) :
    ∃ next, advance s (.ordinary (.owner endpoint command paid bytes observed)) = some next := by
  have ran := SharedHostCaptureReplay.advance_complete lower
  refine ⟨_,advance_complete (Step.ordinary live idle ?_ ?_ ran)⟩
  · rfl
  · exact memoryEq_exact.mpr memory

theorem claim_lift {s : State base} (live : s.joined.stopped = false)
    (idle : s.active = none) (writerIdle : s.joined.active = none)
    (memory : before = s.writers endpoint)
    (vectorBound : ∀ i : Fin p, (vector[i.val]).val = (s.writers i).data.credits)
    (allowed : SourceEntryCapture.StartAllowed s.joined.history endpoint before staged snapshot) :
    ∃ next, advance s (.claim endpoint vector before staged snapshot) = some next := by
  have present := SourceEntryCapture.start_complete (vector:=vector) allowed
  cases ran : SourceEntryCapture.start s.joined.history endpoint vector before staged snapshot with
  | none => simp [ran] at present
  | some opened =>
    exact ⟨_,advance_complete (.claim live idle writerIdle memory (vectorMatches_exact.mpr vectorBound) ran)⟩

#print axioms owner_open_lift
#print axioms claim_lift

-- Origin is historical: idle dispatch and the pending ticket's endpoint hold
-- at checked opening. Neither bare Open nor the local Invariant confers it.
def CheckedOrigin (context : SourceEntryJournal.Context base) : Prop :=
  ∃ before, SourceEntryCapture.StartAllowed context.before context.endpoint before context.staged context.oldSnapshot
def Origins (s : State base) : Prop :=
  (∀ opened, s.active = some opened → CheckedOrigin opened.entry.context) ∧
  (∀ stored ∈ s.stored, CheckedOrigin stored.entry.context)

theorem initial_origins : Origins (State.start base) := by simp [Origins,State.start]

theorem preserves_origins (valid : Origins s) (step : Step s event next) : Origins next := by
  cases step with
  | ordinary live idle allowed memory ran => exact ⟨by simp [idle],valid.2⟩
  | claim live idle writerIdle memory vectorBound ran =>
    rename_i before vector endpoint staged snapshot opened
    have facts := SourceEntryCapture.start_sound ran
    refine ⟨?_,valid.2⟩
    intro op equal; cases Option.some.inj equal
    refine ⟨before,?_⟩
    simpa only [facts.2.1,facts.2.2.1,facts.2.2.2.2.2.1,facts.2.2.2.2.2.2.2] using facts.1
  | reply live active bound localStep wireStep =>
    refine ⟨?_,valid.2⟩
    intro op equal; cases Option.some.inj equal
    rw [(SourceEntryCapture.Open.advance_sound localStep).1]
    exact valid.1 _ active
  | store live active localStep =>
    rename_i opened after kind observed snapshot
    have same := (SourceEntryCapture.Open.observe_sound localStep).1
    have origin := valid.1 _ active
    cases kind with
    | notify | cancel =>
      refine ⟨?_,valid.2⟩
      intro op equal; cases Option.some.inj equal
      rw [same]; exact origin
    | snapshot =>
      refine ⟨by simp [afterStore],?_⟩
      intro stored member
      rcases List.mem_cons.mp member with equal | old
      · subst stored; change CheckedOrigin after.context; rw [same]; exact origin
      · exact valid.2 _ old
  | retireIdle live idle wireStep => exact ⟨by simp [idle],valid.2⟩
  | retireActive live active wireStep localStep =>
    refine ⟨?_,valid.2⟩
    intro op equal; cases Option.some.inj equal
    rw [(SourceEntryCapture.Open.advance_sound localStep).1]
    exact valid.1 _ active

theorem runs_origins (valid : Origins before) (path : Runs before events after) : Origins after := by
  induction path with
  | nil => exact valid
  | step _ step ih => exact preserves_origins ih step

theorem Certified.checked_origins (checked : Certified base) : Origins checked.state :=
  runs_origins initial_origins checked.path

#print axioms Certified.checked_origins
end SourceEntryPrefix
