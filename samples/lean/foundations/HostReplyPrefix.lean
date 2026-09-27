import SourceEntryPrefix
import MirroreaProofFirstSharedReplyRetention
open MirroreaProofFirst
namespace HostReplyPrefix
open OwnerCommitJournal SharedHostCaptureReplay
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Generated/internal intent, not a public Mir communication API. Predicting an
-- admissible successor is proof information; the physical path starts at send.
-- While a residual is open the certified host is the LAST SETTLED anchor. It
-- must not be confused with the independently advancing physical native state.
inductive Intent (p a : Nat) where
  | source (input : SourceFundingQuery.CheckedInput p a)
  | entry (input : SourceFundingQuery.CheckedInput p a)
  | owner (endpoint : Fin p) (command : OwnerEndpoint.Command p a) (paid : Bool) (memory : Memory p a)

def Intent.request : Intent p a → SharedWireLifetime.Request p a
  | .source input | .entry input => .source input
  | .owner endpoint command paid _ => .owner endpoint command paid

def Intent.event (intent : Intent p a) (bytes : List UInt8) : SourceEntryPrefix.Event p a :=
  match intent with
  | .source input => .ordinary (.source input bytes)
  | .entry input => .reply input bytes
  | .owner endpoint command paid memory => .ordinary (.owner endpoint command paid bytes memory)

theorem open_known (opened : openBound history owner command paid bytes memory = some bound) :
    SharedWireLifetime.History.knownStep history (.owner owner command paid) bytes = some bound.later := by
  unfold openBound at opened
  dsimp only at opened
  split at opened
  · cases opened
  · rename_i next known
    split at opened
    · split at opened
      · cases Option.some.inj opened; exact known
      · cases opened
    · cases opened

theorem intent_known {before after : SourceEntryPrefix.State base} {intent : Intent p a}
    (step : SourceEntryPrefix.Step before (intent.event bytes) after) :
    SharedWireLifetime.History.knownStep before.joined.history intent.request bytes = some after.joined.history := by
  cases intent with
  | source input =>
    cases step with
    | ordinary live idle allowed memory ran =>
      cases SharedHostCaptureReplay.advance_sound ran with
      | source live idle known => exact known
  | entry input =>
    cases step with
    | reply live active binding localStep wireStep =>
      cases SharedHostCaptureReplay.advance_sound wireStep with
      | source live idle known => exact known
  | owner endpoint command paid memory =>
    cases step with
    | ordinary live idle allowed binding ran =>
      cases SharedHostCaptureReplay.advance_sound ran with
      | owner live idle known => exact open_known known

theorem known_prepared {history after : SharedWireLifetime.History base}
    {request : SharedWireLifetime.Request p a}
    {next : SharedFundedDriver.Next history.current (SharedFundedDriver.fundingEvent (SharedWireLifetime.event history.current request))}
    (known : SharedWireLifetime.History.knownStep history request bytes = some after)
    (prepared : SharedWireLifetime.prepare history.current request = some next) : after.current = next.val := by
  unfold SharedWireLifetime.History.knownStep at known
  split at known
  · cases known
  · rename_i result checked
    cases Option.some.inj known
    unfold SharedWireLifetime.known at checked
    split at checked
    · cases checked
    · rename_i actual admitted
      have same := Option.some.inj (admitted.symm.trans prepared)
      subst actual
      split at checked
      · cases Option.some.inj checked; rfl
      · cases checked

structure Armed (anchor : SourceEntryPrefix.Certified base) (intent : Intent p a) where
  future : SourceEntryPrefix.Certified base
  accepted : SourceEntryPrefix.Step anchor.state
    (intent.event (SharedWireLifetime.reply anchor.state.joined.history.current intent.request)) future.state
  next : SharedFundedDriver.Next anchor.state.joined.history.current
    (SharedFundedDriver.fundingEvent (SharedWireLifetime.event anchor.state.joined.history.current intent.request))
  prepared : SharedWireLifetime.prepare anchor.state.joined.history.current intent.request = some next
  future_native : future.state.joined.history.current = next.val

def arm (anchor : SourceEntryPrefix.Certified base) (intent : Intent p a) : Option (Armed anchor intent) :=
  let bytes := SharedWireLifetime.reply anchor.state.joined.history.current intent.request
  match checked : anchor.advance (intent.event bytes) with
  | none => none
  | some future =>
    match prepared : SharedWireLifetime.prepare anchor.state.joined.history.current intent.request with
    | none => none
    | some next =>
      let step := SourceEntryPrefix.Certified.advance_step checked
      some ⟨future,step,next,prepared,known_prepared (intent_known step) prepared⟩

def Armed.attempt {anchor : SourceEntryPrefix.Certified base} (armed : Armed anchor intent) :
    SharedWireLifetime.Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed :=
  ⟨anchor.state.joined.history.current,intent.request,armed.next⟩

theorem Armed.exact_future_actions (armed : Armed anchor intent) :
    armed.future.state.joined.history.actions = anchor.state.joined.history.actions ++
      [.send intent.request,.deliver,.receive (SharedWireLifetime.reply anchor.state.joined.history.current intent.request),.promote] :=
  known_actions (intent_known armed.accepted)

-- Exact executable projection of one retained-reply step. Raw retention and
-- handoff stutter on the wire; stopping after validation keeps received evidence.
def projection (slot : SharedReplyRetention.Slot p a) : SharedReplyRetention.Action → List (SharedWireLifetime.Action p a)
  | .deliver => [.deliver]
  | .retain _ | .handoff => []
  | .validate => match slot.phase with | .handed bytes => [.receive bytes] | _ => []
  | .promote => [.promote]
  | .stop => match slot.phase with | .waiting | .raw _ | .handed _ => [.loseReply] | _ => []

theorem projection_exact (step : SharedReplyRetention.Step t before action after) :
    SharedWireLifetime.Runs (SharedReplyRetention.wire t before) (projection before action)
      (SharedReplyRetention.wire t after) := by
  cases step with
  | deliver =>
    rename_i stopped physical
    cases stopped with
    | false => exact .step .nil .deliver
    | true => exact .step .nil .lateDeliver
  | retain | handoff => exact .nil
  | validate => exact .step .nil .receive
  | reject wrong => exact .step .nil (.mismatch wrong)
  | promote => exact .step .nil .promote
  | stop =>
    rename_i phase applied physical
    cases phase with
    | waiting | raw | handed => exact .step .nil .lose
    | validated | promoted => exact .nil

#print axioms intent_known
#print axioms known_prepared
#print axioms arm
#print axioms Armed.exact_future_actions
#print axioms projection_exact

structure Residual (p a : Nat) where
  slot : SharedReplyRetention.Slot p a
  actions : List (SharedWireLifetime.Action p a)

def residualStart (t : SharedWireLifetime.Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed) : Residual p a :=
  ⟨SharedReplyRetention.armed t,[.send t.request]⟩

inductive Step (t : SharedWireLifetime.Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    Residual p a → SharedReplyRetention.Action → Residual p a → Prop where
  | transition (ran : SharedReplyRetention.Step t before.slot action after) :
      Step t before action ⟨after,before.actions++projection before.slot action⟩

def advance (t : SharedWireLifetime.Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (before : Residual p a) (action : SharedReplyRetention.Action) : Option (Residual p a) :=
  (SharedReplyRetention.advance t before.slot action).map fun after =>
    ⟨after,before.actions++projection before.slot action⟩

theorem advance_complete (step : Step t before action after) : advance t before action = some after := by
  cases step with
  | transition ran => simp [advance,SharedReplyRetention.advance_complete t ran]

theorem advance_sound (checked : advance t before action = some after) : Step t before action after := by
  unfold advance at checked
  cases ran : SharedReplyRetention.advance t before.slot action with
  | none => simp [ran] at checked
  | some slot =>
    simp only [ran,Option.map_some] at checked
    cases Option.some.inj checked
    exact .transition (SharedReplyRetention.advance_sound t ran)

theorem advance_exact : advance t before action = some after ↔ Step t before action after :=
  ⟨advance_sound,advance_complete⟩

def Invariant (t : SharedWireLifetime.Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (r : Residual p a) : Prop :=
  SharedReplyRetention.invariant t r.slot ∧
  SharedWireLifetime.Runs (SharedWireLifetime.initial t.before) r.actions (SharedReplyRetention.wire t r.slot)

theorem residual_initial (prepared : SharedWireLifetime.prepare t.before t.request = some t.next) :
    Invariant t (residualStart t) := ⟨SharedReplyRetention.armed_valid t,.step .nil (.send prepared)⟩

theorem preserves (valid : Invariant t before) (step : Step t before action after) : Invariant t after := by
  cases step with
  | transition ran => exact ⟨SharedReplyRetention.preserves t valid.1 ran,valid.2.append (projection_exact ran)⟩

def liveActions (request : SharedWireLifetime.Request p a) (slot : SharedReplyRetention.Slot p a) :
    List (SharedWireLifetime.Action p a) :=
  match slot.phase with
  | .waiting => if slot.applied then [.send request,.deliver] else [.send request]
  | .raw _ | .handed _ => [.send request,.deliver]
  | .validated bytes => [.send request,.deliver,.receive bytes]
  | .promoted bytes => [.send request,.deliver,.receive bytes,.promote]

def LiveExact (t : SharedWireLifetime.Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (r : Residual p a) : Prop := r.slot.stopped = false → r.actions = liveActions t.request r.slot

theorem initial_live : LiveExact t (residualStart t) := by simp [LiveExact,residualStart,SharedReplyRetention.armed,liveActions]

theorem preserves_live (valid : LiveExact t before) (step : Step t before action after) : LiveExact t after := by
  rcases before with ⟨slot,actions⟩
  cases step with
  | transition ran =>
    cases ran with
    | deliver =>
      rename_i stopped physical
      cases stopped <;> simp_all [LiveExact,projection,liveActions]
    | retain | handoff | validate | reject | promote | stop =>
      simp_all [LiveExact,projection,liveActions]

inductive Runs (t : SharedWireLifetime.Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    Residual p a → List SharedReplyRetention.Action → Residual p a → Prop where
  | nil : Runs t before [] before
  | step : Runs t before events current → Step t current action after → Runs t before (events++[action]) after

theorem runs_preserve (valid : Invariant t before) (path : Runs t before actions after) : Invariant t after := by
  induction path with
  | nil => exact valid
  | step _ step ih => exact preserves ih step

theorem runs_live (valid : LiveExact t before) (path : Runs t before actions after) : LiveExact t after := by
  induction path with
  | nil => exact valid
  | step _ step ih => exact preserves_live ih step

structure Pending {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a} (armed : Armed anchor intent) where
  residual : Residual p a
  events : List SharedReplyRetention.Action
  path : Runs armed.attempt (residualStart armed.attempt) events residual

def Pending.start {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a} (armed : Armed anchor intent) : Pending armed :=
  ⟨residualStart armed.attempt,[],.nil⟩

def Pending.advance (before : Pending armed) (action : SharedReplyRetention.Action) : Option (Pending armed) :=
  match ran : HostReplyPrefix.advance armed.attempt before.residual action with
  | none => none
  | some after => some ⟨after,before.events++[action],.step before.path (advance_sound ran)⟩

theorem Pending.valid (pending : Pending armed) : Invariant armed.attempt pending.residual :=
  runs_preserve (residual_initial armed.prepared) pending.path

theorem Pending.live_exact (pending : Pending armed) : LiveExact armed.attempt pending.residual :=
  runs_live initial_live pending.path

theorem Pending.rooted {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a}
    {armed : Armed anchor intent} (pending : Pending armed) :
    SharedWireLifetime.Runs (SharedWireLifetime.initial base)
      (anchor.state.joined.history.actions++pending.residual.actions)
      (SharedReplyRetention.wire armed.attempt pending.residual.slot) :=
  anchor.state.joined.history.path.append pending.valid.2

-- The current local memory is the retained anchor through every residual
-- phase. It is not equated to the possibly applied physical native state.
def Pending.localWriters {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a}
    {armed : Armed anchor intent} (_ : Pending armed) : Fin p → Memory p a := anchor.state.writers

theorem Pending.local_retained {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a}
    {armed : Armed anchor intent} (before after : Pending armed) : after.localWriters = before.localWriters := rfl

#print axioms advance_exact
#print axioms preserves
#print axioms preserves_live
#print axioms Pending.rooted
#print axioms Pending.local_retained

theorem known_has_prepare {history after : SharedWireLifetime.History base}
    (known : SharedWireLifetime.History.knownStep history request bytes = some after) :
    (SharedWireLifetime.prepare history.current request).isSome = true := by
  unfold SharedWireLifetime.History.knownStep at known
  split at known
  · cases known
  · rename_i result checked
    unfold SharedWireLifetime.known at checked
    split at checked
    · cases checked
    · rename_i next prepared
      simp [prepared]

theorem certified_lift (anchor : SourceEntryPrefix.Certified base)
    (step : SourceEntryPrefix.Step anchor.state event future) :
    ∃ next, anchor.advance event = some next ∧ next.state = future := by
  have checked := SourceEntryPrefix.advance_complete step
  unfold SourceEntryPrefix.Certified.advance
  split
  · rename_i failed; rw [checked] at failed; cases failed
  · rename_i next ran
    exact ⟨_,rfl,Option.some.inj (ran.symm.trans checked)⟩

theorem arm_complete (anchor : SourceEntryPrefix.Certified base) (intent : Intent p a)
    (step : SourceEntryPrefix.Step anchor.state
      (intent.event (SharedWireLifetime.reply anchor.state.joined.history.current intent.request)) future) :
    (arm anchor intent).isSome = true := by
  obtain ⟨next,checked,same⟩ := certified_lift anchor step
  have present := known_has_prepare (intent_known step)
  unfold arm
  dsimp only
  split
  · rename_i failed; rw [checked] at failed; cases failed
  · split
    · rename_i failed; simp [failed] at present
    · rfl

theorem Pending.promoted_facts {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a}
    {armed : Armed anchor intent} (pending : Pending armed)
    (live : pending.residual.slot.stopped = false) (phase : pending.residual.slot.phase = .promoted bytes) :
    bytes = SharedWireLifetime.reply armed.attempt.before armed.attempt.request ∧
    pending.residual.slot.physical = SharedWireLifetime.native armed.future.state.joined.history.current ∧
    armed.future.state.joined.history.actions = anchor.state.joined.history.actions ++ pending.residual.actions := by
  have valid := pending.valid.1
  have facts : pending.residual.slot.applied = true ∧
      bytes = SharedWireLifetime.reply armed.attempt.before armed.attempt.request := by
    simpa only [SharedReplyRetention.invariant,phase] using valid.2
  refine ⟨facts.2,?_,?_⟩
  · have physical := valid.1
    simp only [SharedWireLifetime.pendingPhysical,facts.1,↓reduceIte] at physical
    have atNext := physical.trans (SharedWireLifetime.next_physics armed.attempt).symm
    simpa only [Armed.attempt,←armed.future_native] using atNext
  · rw [pending.live_exact live]
    simp only [liveActions,phase,facts.2,Armed.attempt]
    exact armed.exact_future_actions

-- An absorbed record contains the actual residual, including retained raw
-- bytes and stop status. It does not execute another physical round trip.
structure Absorbed {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a} (armed : Armed anchor intent) where
  receipt : Pending armed
  host : SourceEntryPrefix.Certified base
  future : host = armed.future
  exactActions : host.state.joined.history.actions = anchor.state.joined.history.actions ++ receipt.residual.actions
  exactPhysical : receipt.residual.slot.physical = SharedWireLifetime.native host.state.joined.history.current
  live : receipt.residual.slot.stopped = false
  promoted : ∃ bytes, receipt.residual.slot.phase = .promoted bytes

def absorb {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a}
    {armed : Armed anchor intent} (pending : Pending armed) : Option (Absorbed armed) :=
  if live : pending.residual.slot.stopped = false then
    match phase : pending.residual.slot.phase with
    | .promoted _bytes =>
      let facts := pending.promoted_facts live phase
      some ⟨pending,armed.future,rfl,facts.2.2,facts.2.1,live,⟨_bytes,phase⟩⟩
    | _ => none
  else none

theorem stopped_no_absorb (pending : Pending armed) (stopped : pending.residual.slot.stopped = true) :
    absorb pending = none := by simp [absorb,stopped]

theorem absorb_complete (pending : Pending armed) (live : pending.residual.slot.stopped = false)
    (phase : pending.residual.slot.phase = .promoted bytes) : (absorb pending).isSome = true := by
  unfold absorb
  simp only [live,↓reduceDIte]
  split
  · rfl
  · rename_i impossible
    exact False.elim (impossible bytes phase)

-- Even a directly constructed receipt must now carry the same live/promoted
-- facts required by the checker; data/projection equality alone is insufficient.
theorem Absorbed.acceptable (result : Absorbed armed) : (absorb result.receipt).isSome = true := by
  obtain ⟨bytes,phase⟩ := result.promoted
  exact absorb_complete result.receipt result.live phase

theorem Absorbed.not_stopped (result : Absorbed armed) : result.receipt.residual.slot.stopped ≠ true := by
  simp [result.live]
#print axioms Absorbed.acceptable
#print axioms Absorbed.not_stopped

theorem absorb_retains (checked : absorb pending = some result) : result.receipt = pending := by
  unfold absorb at checked
  split at checked
  · split at checked
    all_goals first | (cases Option.some.inj checked; rfl) | contradiction
  · cases checked

theorem normal_path (t : SharedWireLifetime.Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    Runs t (residualStart t)
      [.deliver,.retain (SharedWireLifetime.reply t.before t.request),.handoff,.validate,.promote]
      ⟨⟨.promoted (SharedWireLifetime.reply t.before t.request),false,true,
        SharedWireLifetime.execute t (SharedWireLifetime.native t.before)⟩,
       [.send t.request,.deliver,.receive (SharedWireLifetime.reply t.before t.request),.promote]⟩ :=
  .step (.step (.step (.step (.step .nil (.transition .deliver)) (.transition .retain))
    (.transition .handoff)) (.transition .validate)) (.transition .promote)

def Pending.normal {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a} (armed : Armed anchor intent) : Pending armed :=
  ⟨_,_,normal_path armed.attempt⟩

theorem normal_absorbs {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a} (armed : Armed anchor intent) :
    (absorb (Pending.normal armed)).isSome = true := absorb_complete _ rfl rfl

#print axioms arm_complete
#print axioms Pending.promoted_facts
#print axioms absorb_retains
#print axioms stopped_no_absorb
#print axioms normal_path
#print axioms normal_absorbs

def Pending.beforeDelivery {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a} (armed : Armed anchor intent) : Pending armed :=
  ⟨⟨⟨.waiting,true,false,SharedWireLifetime.native armed.attempt.before⟩,
    [.send intent.request,.loseReply]⟩,[.stop],.step .nil (.transition .stop)⟩

def Pending.afterDelivery {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a} (armed : Armed anchor intent) : Pending armed :=
  ⟨⟨⟨.waiting,true,true,SharedWireLifetime.execute armed.attempt (SharedWireLifetime.native armed.attempt.before)⟩,
    [.send intent.request,.deliver,.loseReply]⟩,[.deliver,.stop],
    .step (.step .nil (.transition .deliver)) (.transition .stop)⟩

def Pending.lateDelivery {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a} (armed : Armed anchor intent) : Pending armed :=
  ⟨⟨⟨.waiting,true,true,SharedWireLifetime.execute armed.attempt (SharedWireLifetime.native armed.attempt.before)⟩,
    [.send intent.request,.loseReply,.deliver]⟩,[.stop,.deliver],
    .step (.step .nil (.transition .stop)) (.transition .deliver)⟩

def Pending.rawStopped {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a}
    (armed : Armed anchor intent) (bytes : List UInt8) : Pending armed :=
  ⟨⟨⟨.raw bytes,true,true,SharedWireLifetime.execute armed.attempt (SharedWireLifetime.native armed.attempt.before)⟩,
    [.send intent.request,.deliver,.loseReply]⟩,[.deliver,.retain bytes,.stop],
    .step (.step (.step .nil (.transition .deliver)) (.transition .retain)) (.transition .stop)⟩

def Pending.validatedStopped {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a}
    (armed : Armed anchor intent) : Pending armed :=
  let bytes := SharedWireLifetime.reply armed.attempt.before intent.request
  ⟨⟨⟨.validated bytes,true,true,SharedWireLifetime.execute armed.attempt (SharedWireLifetime.native armed.attempt.before)⟩,
    [.send intent.request,.deliver,.receive bytes]⟩,[.deliver,.retain bytes,.handoff,.validate,.stop],
    .step (.step (.step (.step (.step .nil (.transition .deliver)) (.transition .retain))
      (.transition .handoff)) (.transition .validate)) (.transition .stop)⟩

def Pending.mismatchStopped {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a}
    (armed : Armed anchor intent) (bytes : List UInt8)
    (wrong : bytes ≠ SharedWireLifetime.reply armed.attempt.before intent.request) : Pending armed :=
  ⟨⟨⟨.handed bytes,true,true,SharedWireLifetime.execute armed.attempt (SharedWireLifetime.native armed.attempt.before)⟩,
    [.send intent.request,.deliver,.receive bytes]⟩,[.deliver,.retain bytes,.handoff,.validate],
    .step (.step (.step (.step .nil (.transition .deliver)) (.transition .retain))
      (.transition .handoff)) (.transition (.reject wrong))⟩

theorem unknown_paths_retained {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a}
    (armed : Armed anchor intent) :
    (Pending.beforeDelivery armed).localWriters = anchor.state.writers ∧
    (Pending.afterDelivery armed).localWriters = anchor.state.writers ∧
    (Pending.lateDelivery armed).localWriters = anchor.state.writers ∧
    absorb (Pending.beforeDelivery armed) = none ∧
    absorb (Pending.afterDelivery armed) = none ∧
    absorb (Pending.lateDelivery armed) = none :=
  ⟨rfl,rfl,rfl,stopped_no_absorb _ rfl,stopped_no_absorb _ rfl,stopped_no_absorb _ rfl⟩

theorem validated_stopped_received {anchor : SourceEntryPrefix.Certified base} {intent : Intent p a}
    (armed : Armed anchor intent) :
    (SharedReplyRetention.wire armed.attempt (Pending.validatedStopped armed).residual.slot).host = .received armed.attempt ∧
    absorb (Pending.validatedStopped armed) = none := ⟨rfl,stopped_no_absorb _ rfl⟩

theorem late_source_not_rollback {anchor : SourceEntryPrefix.Certified base}
    {input : SourceFundingQuery.CheckedInput p a} (armed : Armed anchor (.entry input)) :
    (Pending.lateDelivery armed).residual.slot.physical ≠ SharedWireLifetime.native anchor.state.joined.history.current :=
  SharedWireLifetime.source_rollback_false anchor.state.joined.history.current input

theorem step_stopped_monotone (step : Step t before action after) (stopped : before.slot.stopped = true) :
    after.slot.stopped = true := by
  cases step with
  | transition ran => exact SharedReplyRetention.stopped_monotone t ran stopped

theorem step_retains_bytes (step : Step t before action after)
    (retained : SharedReplyRetention.observedBytes before.slot.phase = some bytes) :
    SharedReplyRetention.observedBytes after.slot.phase = some bytes := by
  cases step with
  | transition ran => exact SharedReplyRetention.retained_bytes_frame t ran retained

#print axioms Pending.beforeDelivery
#print axioms Pending.afterDelivery
#print axioms Pending.lateDelivery
#print axioms Pending.rawStopped
#print axioms Pending.validatedStopped
#print axioms Pending.mismatchStopped
#print axioms unknown_paths_retained
#print axioms validated_stopped_received
#print axioms late_source_not_rollback
#print axioms step_stopped_monotone
#print axioms step_retains_bytes

-- A terminal capture retains the same rooted host anchor and its actual
-- residual. Stopped does not mean native rollback, public completion or EOF.
structure Interrupted (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  anchor : SourceEntryPrefix.Certified base
  intent : Intent p a
  armed : Armed anchor intent
  pending : Pending armed
  stopped : pending.residual.slot.stopped = true

theorem Interrupted.no_absorb (record : Interrupted base) : absorb record.pending = none :=
  stopped_no_absorb record.pending record.stopped

theorem Interrupted.rooted (record : Interrupted base) :
    SharedWireLifetime.Runs (SharedWireLifetime.initial base)
      (record.anchor.state.joined.history.actions++record.pending.residual.actions)
      (SharedReplyRetention.wire record.armed.attempt record.pending.residual.slot) := record.pending.rooted

#print axioms Interrupted.no_absorb
#print axioms Interrupted.rooted
end HostReplyPrefix
