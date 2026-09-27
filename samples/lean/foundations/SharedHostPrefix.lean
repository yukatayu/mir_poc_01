import SharedHostJournal
open MirroreaProofFirst
namespace SharedHostCaptureReplay
open OwnerCommitJournal
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Complete wire-state extension, not only a list/hash/projection prefix.
def Extends (earlier later : SharedWireLifetime.History base) : Prop :=
  ∃ suffix, later.actions = earlier.actions ++ suffix ∧
    SharedWireLifetime.Runs (SharedWireLifetime.initial earlier.current) suffix
      (SharedWireLifetime.initial later.current)

theorem Extends.refl (history : SharedWireLifetime.History base) : Extends history history :=
  ⟨[],by simp,.nil⟩
theorem Extends.trans (left : Extends before middle) (right : Extends middle after) : Extends before after := by
  obtain ⟨x,hx,px⟩ := left
  obtain ⟨y,hy,py⟩ := right
  exact ⟨x++y,by rw [hy,hx,List.append_assoc],px.append py⟩

theorem known_extends (checked : SharedWireLifetime.History.knownStep before request bytes = some after) :
    Extends before after := by
  unfold SharedWireLifetime.History.knownStep at checked
  cases got : SharedWireLifetime.known before.current request bytes with
  | none => simp [got] at checked
  | some result =>
    simp only [got] at checked
    cases Option.some.inj checked
    exact ⟨_,rfl,result.path⟩

theorem finish_extends (checked : SharedWireLifetime.History.finishStep before = some after) :
    Extends before after := by
  unfold SharedWireLifetime.History.finishStep at checked
  split at checked
  · cases checked
  · rename_i next finished
    cases Option.some.inj checked
    refine ⟨[.localComplete],rfl,?_⟩
    have path : SharedWireLifetime.Runs (SharedWireLifetime.initial before.current) [.localComplete]
        ⟨.settled next,false,SharedWireLifetime.native before.current⟩ :=
      .step .nil (.localComplete finished)
    have frame := SharedFundedDriver.finish_native finished
    simpa only [SharedWireLifetime.initial,←frame] using path

def openBound (history : SharedWireLifetime.History base) (owner : Fin p)
    (command : OwnerEndpoint.Command p a) (paid : Bool) (bytes : List UInt8)
    (before : Memory p a) : Option (BoundWriter base) :=
  let native := (SharedWireLifetime.native history.current).owners owner
  let result := OwnerEndpointBudget.transition ⟨assigned.realm,owner⟩ scope (capacity owner) native command
  match checked : SharedWireLifetime.History.knownStep history (.owner owner command paid) bytes with
  | none => none
  | some later =>
    if maySend before command then
      if matching : dataEq before.data (project native) = true then
        let binding := dataEq_exact.mp matching
        let opened : WriterJournalCaptureReplay.OpenWriter p a :=
          ⟨owner,result.1,before,command,result.2,before,stores before command result.2,.nil,by
            rw [stores_effects,binding,effects_project (show OwnerEndpointBudget.transition
              ⟨assigned.realm,owner⟩ scope (capacity owner) native command = (result.1,result.2) from rfl)]⟩
        some ⟨history,later,opened,paid,bytes,checked,rfl,binding,(SharedHostJoin.history_owner checked).symm⟩
      else none
    else none

theorem open_earlier (opened : openBound history owner command paid bytes memory = some bound) :
    bound.earlier = history := by
  unfold openBound at opened
  dsimp only at opened
  split at opened
  · cases opened
  · split at opened
    · split at opened
      · cases Option.some.inj opened; rfl
      · cases opened
    · cases opened

theorem open_extends (opened : openBound history owner command paid bytes memory = some bound) :
    Extends history bound.later := by
  have result := known_extends bound.known
  rwa [open_earlier opened] at result

theorem observe_history (observed : BoundWriter.observe before memory = some after) :
    after.earlier = before.earlier ∧ after.later = before.later := by
  unfold BoundWriter.observe at observed
  dsimp only at observed
  split at observed
  · cases observed
  · cases Option.some.inj observed; exact ⟨rfl,rfl⟩

theorem finish_bound (finished : BoundWriter.finish before memory = some closed) : closed.bound = before := by
  unfold BoundWriter.finish at finished
  dsimp only at finished
  split at finished
  · cases Option.some.inj finished; rfl
  · cases finished

-- Decoded observations are normalized from one pinned physical event stream.
-- `confirmed` means its caller provenance has been checked externally. The
-- constructor is not a source-level API, authority, or proof of physical truth.
inductive JoinedEvent (p a : Nat) where
  | source (input : SourceFundingQuery.CheckedInput p a) (bytes : List UInt8)
  | owner (owner : Fin p) (command : OwnerEndpoint.Command p a) (paid : Bool)
      (bytes : List UInt8) (before : Memory p a)
  | observe (owner : Fin p) (memory : Memory p a)
  | confirmed (owner : Fin p) (memory : Memory p a)
  | localComplete
  | retire

structure RecordedOpen (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  position : Nat
  bound : BoundWriter base
structure RecordedClosed (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  openedAt : Nat
  confirmedAt : Nat
  writer : ClosedWriter base
structure JoinedState (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  history : SharedWireLifetime.History base
  active : Option (RecordedOpen base)
  closed : List (RecordedClosed base)
  events : List (JoinedEvent p a)
  stopped : Bool

def JoinedState.start (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) : JoinedState base :=
  ⟨.start base,none,[],[],false⟩

-- Operational cases prescribe the updates. No desired prefix invariant is an
-- admission guard; in particular records are not accepted by testing it.
def advanceJoined (s : JoinedState base) (event : JoinedEvent p a) : Option (JoinedState base) :=
  if s.stopped then none else
  let eventsAfter := s.events ++ [event]
  match event with
  | .retire => some {s with events:=eventsAfter,stopped:=true}
  | .source input bytes =>
    match s.active with
    | some _ => none
    | none => match SharedWireLifetime.History.knownStep s.history (.source input) bytes with
      | none => none
      | some next => some {s with history:=next,events:=eventsAfter}
  | .owner owner command paid bytes before =>
    match s.active with
    | some _ => none
    | none => match openBound s.history owner command paid bytes before with
      | none => none
      | some bound => some {s with history:=bound.later,active:=some ⟨s.events.length,bound⟩,events:=eventsAfter}
  | .observe owner memory =>
    match s.active with
    | none => none
    | some current =>
      if current.bound.writer.owner == owner then
        match current.bound.observe memory with
        | none => none
        | some bound => some {s with active:=some ⟨current.position,bound⟩,events:=eventsAfter}
      else none
  | .confirmed owner memory =>
    match s.active with
    | none => none
    | some current =>
      if current.bound.writer.owner == owner then
        match current.bound.finish memory with
        | none => none
        | some closed => some {s with active:=none,closed:=⟨current.position,s.events.length,closed⟩::s.closed,events:=eventsAfter}
      else none
  | .localComplete =>
    match s.active with
    | some _ => none
    | none => match SharedWireLifetime.History.finishStep s.history with
      | none => none
      | some next => some {s with history:=next,events:=eventsAfter}

-- Independent higher-layer rules consume checks at the already proved lower
-- wire/journal boundaries; they do not contain the target JoinedInvariant.
inductive JoinedStep : JoinedState base → JoinedEvent p a → JoinedState base → Prop where
  | source (live : s.stopped = false) (idle : s.active = none)
      (ran : SharedWireLifetime.History.knownStep s.history (.source input) bytes = some next) :
      JoinedStep s (.source input bytes) {s with history:=next,events:=s.events++[.source input bytes]}
  | owner (live : s.stopped = false) (idle : s.active = none)
      (ran : openBound s.history owner command paid bytes before = some bound) :
      JoinedStep s (.owner owner command paid bytes before)
        {s with history:=bound.later,active:=some ⟨s.events.length,bound⟩,events:=s.events++[.owner owner command paid bytes before]}
  | observe (live : s.stopped = false) (active : s.active = some current)
      (same : current.bound.writer.owner = owner) (ran : current.bound.observe memory = some bound) :
      JoinedStep s (.observe owner memory) {s with active:=some ⟨current.position,bound⟩,events:=s.events++[.observe owner memory]}
  | confirmed (live : s.stopped = false) (active : s.active = some current)
      (same : current.bound.writer.owner = owner) (ran : current.bound.finish memory = some closed) :
      JoinedStep s (.confirmed owner memory) {s with active:=none,closed:=⟨current.position,s.events.length,closed⟩::s.closed,events:=s.events++[.confirmed owner memory]}
  | localComplete (live : s.stopped = false) (idle : s.active = none)
      (ran : SharedWireLifetime.History.finishStep s.history = some next) :
      JoinedStep s .localComplete {s with history:=next,events:=s.events++[.localComplete]}
  | retire (live : s.stopped = false) :
      JoinedStep s .retire {s with stopped:=true,events:=s.events++[.retire]}

theorem advance_complete (step : JoinedStep s event next) : advanceJoined s event = some next := by
  cases step <;> simp_all [advanceJoined]

theorem advance_sound (checked : advanceJoined s event = some next) : JoinedStep s event next := by
  unfold advanceJoined at checked
  split at checked
  · cases checked
  · rename_i live
    have live : s.stopped = false := by simpa using live
    dsimp only at checked
    cases event with
    | retire => cases Option.some.inj checked; exact .retire live
    | source input bytes =>
      cases active : s.active with
      | some current => simp [active] at checked
      | none =>
        simp only [active] at checked
        split at checked
        · cases checked
        · rename_i after ran; cases Option.some.inj checked; simpa only [active] using (JoinedStep.source live active ran)
    | owner owner command paid bytes before =>
      cases active : s.active with
      | some current => simp [active] at checked
      | none =>
        simp only [active] at checked
        split at checked
        · cases checked
        · rename_i bound ran; cases Option.some.inj checked; exact .owner live active ran
    | observe owner memory =>
      cases active : s.active with
      | none => simp [active] at checked
      | some current =>
        simp only [active] at checked
        split at checked
        · rename_i same
          split at checked
          · cases checked
          · rename_i bound ran; cases Option.some.inj checked
            exact .observe live active (by simpa using same) ran
        · cases checked
    | confirmed owner memory =>
      cases active : s.active with
      | none => simp [active] at checked
      | some current =>
        simp only [active] at checked
        split at checked
        · rename_i same
          split at checked
          · cases checked
          · rename_i bound ran; cases Option.some.inj checked
            exact .confirmed live active (by simpa using same) ran
        · cases checked
    | localComplete =>
      cases active : s.active with
      | some current => simp [active] at checked
      | none =>
        simp only [active] at checked
        split at checked
        · cases checked
        · rename_i after ran; cases Option.some.inj checked; simpa only [active] using (JoinedStep.localComplete live active ran)

theorem advance_exact : advanceJoined s event = some next ↔ JoinedStep s event next :=
  ⟨advance_sound,advance_complete⟩

-- A historical writer keeps its historical nativeData; subsequent operations
-- extend the history, rather than equating that data to the new owner state.
def HistoryInvariant (s : JoinedState base) : Prop :=
  (∀ opened, s.active = some opened → opened.bound.later = s.history) ∧
  ∀ closed ∈ s.closed, Extends closed.writer.bound.later s.history

theorem initial_valid : HistoryInvariant (JoinedState.start base) := by
  simp [HistoryInvariant,JoinedState.start]

theorem preserves_history (valid : HistoryInvariant s) (step : JoinedStep s event next) :
    HistoryInvariant next := by
  rcases valid with ⟨opened,closed⟩
  cases step with
  | source live idle ran =>
    exact ⟨by simpa [idle],fun cw member => (closed cw member).trans (known_extends ran)⟩
  | owner live idle ran =>
    refine ⟨?_,fun cw member => (closed cw member).trans ?_⟩
    · intro op same; cases Option.some.inj same; rfl
    · exact open_extends ran

  | observe live active same ran =>
    refine ⟨?_,closed⟩
    intro op equal; cases Option.some.inj equal
    exact (observe_history ran).2.trans (opened _ active)
  | confirmed live active same ran =>
    refine ⟨by simp,?_⟩
    intro cw member
    rcases List.mem_cons.mp member with equal | old
    · subst cw
      rw [finish_bound ran,opened _ active]
      exact Extends.refl _
    · exact closed cw old
  | localComplete live idle ran =>
    exact ⟨by simpa [idle],fun cw member => (closed cw member).trans (finish_extends ran)⟩
  | retire live => exact ⟨opened,closed⟩


def JoinedEvent.actions : JoinedEvent p a → List (SharedWireLifetime.Action p a)
  | .source input bytes => [.send (.source input),.deliver,.receive bytes,.promote]
  | .owner target command paid bytes _ => [.send (.owner target command paid),.deliver,.receive bytes,.promote]
  | .localComplete => [.localComplete]
  | _ => []

theorem known_actions (checked : SharedWireLifetime.History.knownStep before request bytes = some after) :
    after.actions = before.actions ++ [.send request,.deliver,.receive bytes,.promote] := by
  unfold SharedWireLifetime.History.knownStep at checked
  split at checked
  · cases checked
  · cases Option.some.inj checked; rfl

theorem finish_actions (checked : SharedWireLifetime.History.finishStep before = some after) :
    after.actions = before.actions ++ [.localComplete] := by
  unfold SharedWireLifetime.History.finishStep at checked
  split at checked
  · cases checked
  · cases Option.some.inj checked; rfl

theorem open_actions (opened : openBound history owner command paid bytes memory = some bound) :
    bound.later.actions = history.actions ++ [.send (.owner owner command paid),.deliver,.receive bytes,.promote] := by
  unfold openBound at opened
  dsimp only at opened
  split at opened
  · cases opened
  · rename_i later checked
    split at opened
    · split at opened
      · cases Option.some.inj opened; exact known_actions checked
      · cases opened
    · cases opened

theorem step_events (step : JoinedStep s event next) : next.events = s.events ++ [event] := by
  cases step <;> rfl

theorem step_actions (step : JoinedStep s event next) :
    next.history.actions = s.history.actions ++ event.actions := by
  cases step with
  | source live idle ran => exact known_actions ran
  | owner live idle ran => exact open_actions ran
  | localComplete live idle ran => exact finish_actions ran
  | observe => simp [JoinedEvent.actions]
  | confirmed => simp [JoinedEvent.actions]
  | retire => simp [JoinedEvent.actions]

theorem step_retains_closed (step : JoinedStep s event next) :
    ∃ fresh, next.closed = fresh ++ s.closed := by
  cases step
  all_goals first | exact ⟨[],rfl⟩ | exact ⟨[_],rfl⟩

theorem stopped_no_step (stopped : s.stopped = true) : advanceJoined s event = none := by
  simp [advanceJoined,stopped]

theorem active_no_source (active : s.active = some current) :
    advanceJoined s (.source input bytes) = none := by simp [advanceJoined,active]
theorem active_no_owner (active : s.active = some current) :
    advanceJoined s (.owner owner command paid bytes memory) = none := by simp [advanceJoined,active]
theorem active_no_finish (active : s.active = some current) :
    advanceJoined s .localComplete = none := by simp [advanceJoined,active]

inductive JoinedRuns : JoinedState base → List (JoinedEvent p a) → JoinedState base → Prop where
  | nil : JoinedRuns s [] s
  | step : JoinedRuns origin earlier middle → JoinedStep middle event after → JoinedRuns origin (earlier++[event]) after

theorem runs_history (valid : HistoryInvariant before) (path : JoinedRuns before events after) :
    HistoryInvariant after := by
  induction path with
  | nil => exact valid
  | step prior step ih => exact preserves_history ih step

theorem runs_events (path : JoinedRuns before events after) : after.events = before.events ++ events := by
  induction path with
  | nil => simp
  | step prior step ih => rw [step_events step,ih,List.append_assoc]

theorem runs_actions (path : JoinedRuns before events after) :
    after.history.actions = before.history.actions ++ events.flatMap JoinedEvent.actions := by
  induction path with
  | nil => simp
  | step prior step ih => rw [step_actions step,ih]; simp [List.flatMap_append,List.append_assoc]

theorem runs_retains_closed (path : JoinedRuns before events after) :
    ∃ fresh, after.closed = fresh ++ before.closed := by
  induction path with
  | nil => exact ⟨[],rfl⟩
  | step prior step ih =>
    obtain ⟨priorFresh,old⟩ := ih
    obtain ⟨newFresh,next⟩ := step_retains_closed step
    exact ⟨newFresh++priorFresh,by rw [next,old,List.append_assoc]⟩

-- The actual consumer carries this certificate through every decoded native,
-- observation, confirmation and local-completion event. No separate mutable
-- history or unindexed closed-writer container is admitted at that boundary.
structure CertifiedJoined (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  state : JoinedState base
  path : JoinedRuns (.start base) state.events state

def CertifiedJoined.start (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) : CertifiedJoined base :=
  ⟨.start base,.nil⟩

def CertifiedJoined.advance (before : CertifiedJoined base) (event : JoinedEvent p a) : Option (CertifiedJoined base) :=
  match checked : advanceJoined before.state event with
  | none => none
  | some next =>
    let step := advance_sound checked
    some ⟨next,by rw [step_events step]; exact .step before.path step⟩

theorem CertifiedJoined.valid (checked : CertifiedJoined base) : HistoryInvariant checked.state :=
  runs_history initial_valid checked.path

theorem CertifiedJoined.exact_actions (checked : CertifiedJoined base) :
    checked.state.history.actions = checked.state.events.flatMap JoinedEvent.actions := by
  simpa [JoinedState.start,SharedWireLifetime.History.start] using runs_actions checked.path

#print axioms advance_exact
#print axioms preserves_history
#print axioms runs_actions
#print axioms runs_history
#print axioms runs_retains_closed
#print axioms CertifiedJoined.valid
#print axioms CertifiedJoined.exact_actions

-- Positions are in the normalized consumed stream, not reusable memory IDs.
-- The physical binder separately binds those events to actual caller frames.
def ClosedAt (events : List (JoinedEvent p a)) (closed : RecordedClosed base) : Prop :=
  closed.openedAt < closed.confirmedAt ∧ closed.confirmedAt < events.length ∧
  events[closed.confirmedAt]? = some (.confirmed closed.writer.bound.writer.owner closed.writer.observed)

def PositionInvariant (s : JoinedState base) : Prop :=
  (∀ op, s.active = some op → op.position < s.events.length) ∧
  (∀ cw ∈ s.closed, ClosedAt s.events cw) ∧
  (∀ op, s.active = some op → ∀ cw ∈ s.closed, cw.confirmedAt < op.position) ∧
  s.closed.Pairwise (fun recent older => older.confirmedAt < recent.openedAt)

theorem ClosedAt.append (valid : ClosedAt events closed) : ClosedAt (events++[event]) closed := by
  rcases valid with ⟨order,bounded,atEvent⟩
  refine ⟨order,by simpa using Nat.lt_succ_of_lt bounded,?_⟩
  rw [List.getElem?_append_left bounded]
  exact atEvent

theorem finish_observed (finished : BoundWriter.finish before memory = some closed) : closed.observed = memory := by
  unfold BoundWriter.finish at finished
  dsimp only at finished
  split at finished
  · cases Option.some.inj finished; rfl
  · cases finished

theorem initial_positions : PositionInvariant (JoinedState.start base) := by
  simp [PositionInvariant,JoinedState.start]

theorem preserves_positions (valid : PositionInvariant s) (step : JoinedStep s event next) :
    PositionInvariant next := by
  rcases valid with ⟨activeBound,closedBound,newest,ordered⟩
  cases step with
  | source live idle ran =>
    exact ⟨by simp [idle],fun cw member => (closedBound cw member).append,by simp [idle],ordered⟩
  | owner live idle ran =>
    refine ⟨?_,fun cw member => (closedBound cw member).append,?_,ordered⟩
    · intro op equal; cases Option.some.inj equal; simp
    · intro op equal cw member; cases Option.some.inj equal; exact (closedBound cw member).2.1
  | observe live active same ran =>
    refine ⟨?_,fun cw member => (closedBound cw member).append,?_,ordered⟩
    · intro op equal; cases Option.some.inj equal
      simpa using Nat.lt_succ_of_lt (activeBound _ active)
    · intro op equal cw member; cases Option.some.inj equal; exact newest _ active cw member
  | confirmed live active same ran =>
    refine ⟨by simp,?_,by simp,?_⟩
    · intro cw member
      rcases List.mem_cons.mp member with equal | old
      · subst cw
        refine ⟨activeBound _ active,by simp,?_⟩
        simp only [List.getElem?_append_right (Nat.le_refl _),Nat.sub_self,List.getElem?_cons_zero]
        rw [finish_bound ran,finish_observed ran,same]
      · exact (closedBound cw old).append
    · exact List.Pairwise.cons (fun cw member => newest _ active cw member) ordered
  | localComplete live idle ran =>
    exact ⟨by simp [idle],fun cw member => (closedBound cw member).append,by simp [idle],ordered⟩
  | retire live =>
    refine ⟨?_,fun cw member => (closedBound cw member).append,newest,ordered⟩
    intro op equal; simpa using Nat.lt_succ_of_lt (activeBound op equal)

theorem runs_positions (valid : PositionInvariant before) (path : JoinedRuns before events after) :
    PositionInvariant after := by
  induction path with
  | nil => exact valid
  | step prior step ih => exact preserves_positions ih step

theorem CertifiedJoined.positions (checked : CertifiedJoined base) : PositionInvariant checked.state :=
  runs_positions initial_positions checked.path

#print axioms preserves_positions
#print axioms CertifiedJoined.positions

def BoundWriter.origin (bound : BoundWriter base) : JoinedEvent p a :=
  .owner bound.writer.owner bound.writer.command bound.headPayment bound.bytes bound.writer.before

theorem open_origin (opened : openBound history owner command paid bytes memory = some bound) :
    bound.origin = .owner owner command paid bytes memory := by
  unfold openBound at opened
  dsimp only at opened
  split at opened
  · cases opened
  · split at opened
    · split at opened
      · cases Option.some.inj opened; rfl
      · cases opened
    · cases opened

theorem observe_origin (observed : BoundWriter.observe before memory = some after) : after.origin = before.origin := by
  unfold BoundWriter.observe at observed
  dsimp only at observed
  split at observed
  · cases observed
  · cases Option.some.inj observed; rfl

-- Both the exact opening event and the native prehistory at that opening are
-- retained. Later confirmations never substitute an equal-valued other writer.
def OriginAt (events : List (JoinedEvent p a)) (position : Nat) (bound : BoundWriter base) : Prop :=
  events[position]? = some bound.origin ∧
  bound.earlier.actions = (events.take position).flatMap JoinedEvent.actions

def OriginInvariant (s : JoinedState base) : Prop :=
  (∀ op, s.active = some op → OriginAt s.events op.position op.bound) ∧
  ∀ cw ∈ s.closed, OriginAt s.events cw.openedAt cw.writer.bound

theorem OriginAt.append (valid : OriginAt events position bound) (bounded : position < events.length) :
    OriginAt (events++[event]) position bound := by
  simpa only [OriginAt,List.getElem?_append_left bounded,
    List.take_append_of_le_length (Nat.le_of_lt bounded)] using valid

theorem OriginAt.observe (valid : OriginAt events position before)
    (observed : BoundWriter.observe before memory = some after) : OriginAt events position after := by
  simpa only [OriginAt,observe_origin observed,(observe_history observed).1] using valid

theorem initial_origins : OriginInvariant (JoinedState.start base) := by
  simp [OriginInvariant,JoinedState.start]

theorem preserves_origins (valid : OriginInvariant s) (positions : PositionInvariant s)
    (actions : s.history.actions = s.events.flatMap JoinedEvent.actions)
    (step : JoinedStep s event next) : OriginInvariant next := by
  rcases valid with ⟨activeOrigin,closedOrigin⟩
  rcases positions with ⟨activeBound,closedBound,newest,ordered⟩
  have carry : ∀ cw ∈ s.closed, OriginAt (s.events++[event]) cw.openedAt cw.writer.bound := by
    intro cw member
    have bounded := closedBound cw member
    exact (closedOrigin cw member).append (Nat.lt_trans bounded.1 bounded.2.1)
  cases step with
  | source live idle ran => exact ⟨by simp [idle],carry⟩
  | owner live idle ran =>
    refine ⟨?_,carry⟩
    intro op equal; cases Option.some.inj equal
    constructor
    · simp only [List.getElem?_append_right (Nat.le_refl _),Nat.sub_self,List.getElem?_cons_zero]
      rw [open_origin ran]
    · rw [open_earlier ran]
      simpa only [List.take_append_of_le_length (Nat.le_refl _),List.take_length] using actions
  | observe live active same ran =>
    refine ⟨?_,carry⟩
    intro op equal; cases Option.some.inj equal
    exact ((activeOrigin _ active).observe ran).append (activeBound _ active)
  | confirmed live active same ran =>
    refine ⟨by simp,?_⟩
    intro cw member
    rcases List.mem_cons.mp member with equal | old
    · subst cw
      rw [finish_bound ran]
      exact (activeOrigin _ active).append (activeBound _ active)
    · exact carry cw old
  | localComplete live idle ran => exact ⟨by simp [idle],carry⟩
  | retire live =>
    exact ⟨fun op active => (activeOrigin op active).append (activeBound op active),carry⟩

theorem runs_origins (path : JoinedRuns (JoinedState.start base) events after) : OriginInvariant after := by
  induction path with
  | nil => exact initial_origins
  | step prior step ih =>
    apply preserves_origins ih (runs_positions initial_positions prior) _ step
    have actionEquality := runs_actions prior
    have eventEquality := runs_events prior
    simp only [JoinedState.start,SharedWireLifetime.History.start,List.nil_append] at actionEquality eventEquality
    simpa only [eventEquality] using actionEquality

theorem CertifiedJoined.origins (checked : CertifiedJoined base) : OriginInvariant checked.state :=
  runs_origins checked.path

#print axioms preserves_origins
#print axioms CertifiedJoined.origins

theorem JoinedRuns.append (left : JoinedRuns before earlier middle) (right : JoinedRuns middle later after) :
    JoinedRuns before (earlier++later) after := by
  induction right with
  | nil => simpa using left
  | step prior step ih => simpa only [List.append_assoc] using JoinedRuns.step ih step

theorem CertifiedJoined.advance_step (checked : CertifiedJoined.advance before event = some after) :
    JoinedStep before.state event after.state := by
  unfold CertifiedJoined.advance at checked
  split at checked
  · cases checked
  · rename_i next got
    cases Option.some.inj checked
    exact advance_sound got

-- Pure input-list consumer. The list is still supplied by the physical binder;
-- this theorem establishes no missing/reordered normalized inputs INSIDE this
-- consumer, and makes no assertion of external observation authenticity.
def CertifiedJoined.consume (before : CertifiedJoined base) : List (JoinedEvent p a) → Option (CertifiedJoined base)
  | [] => some before
  | event::rest => do
    let next ← before.advance event
    next.consume rest

theorem CertifiedJoined.consume_sound (checked : CertifiedJoined.consume before events = some after) :
    after.state.events = before.state.events ++ events ∧ JoinedRuns before.state events after.state := by
  induction events generalizing before after with
  | nil =>
    simp only [CertifiedJoined.consume,Option.some.injEq] at checked
    subst after
    exact ⟨by simp,.nil⟩
  | cons event rest ih =>
    cases got : before.advance event with
    | none => simp [CertifiedJoined.consume,got] at checked
    | some next =>
      simp only [CertifiedJoined.consume,got] at checked
      obtain ⟨tailEvents,tailPath⟩ := ih checked
      have step := CertifiedJoined.advance_step got
      refine ⟨?_,?_⟩
      · rw [tailEvents,step_events step]
        simp only [List.append_assoc,List.singleton_append]
      · simpa only [List.nil_append,List.singleton_append] using
          (JoinedRuns.step JoinedRuns.nil step).append tailPath

theorem CertifiedJoined.consume_stopped (stopped : before.state.stopped = true) :
    CertifiedJoined.consume before (event::rest) = none := by
  have noStep : advanceJoined before.state event = none := stopped_no_step stopped
  have noAdvance : before.advance event = none := by
    unfold CertifiedJoined.advance
    split
    · rfl
    · rename_i next checked
      rw [noStep] at checked
      cases checked
  simp [CertifiedJoined.consume,noAdvance]

#print axioms CertifiedJoined.consume_sound
#print axioms CertifiedJoined.consume_stopped
end SharedHostCaptureReplay
