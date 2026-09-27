import CohortHostExecution
import HostReplyPrefix
open MirroreaProofFirst
namespace CohortHostFailure
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- An outstanding physical request is rooted in the SAME full live host, not
-- in a separately reconstructed native projection. Before confirmation the
-- physical native state may advance while this host is its last settled cut.
def ready (live : CohortHostExecution.Live base) : Bool :=
  !live.current.state.cohort.retired && live.current.state.cohort.gate &&
    live.current.state.cohort.call.isSome && live.current.state.cohort.pending.isEmpty

structure Armed (live : CohortHostExecution.Live base) (intent : HostReplyPrefix.Intent p a) where
  admitted : ready live = true
  wire : HostReplyPrefix.Armed live.source intent

def arm (live : CohortHostExecution.Live base) (intent : HostReplyPrefix.Intent p a) : Option (Armed live intent) :=
  if admitted : ready live then
    (HostReplyPrefix.arm live.source intent).map fun wire => ⟨admitted,wire⟩
  else none

variable {live : CohortHostExecution.Live base} {intent : HostReplyPrefix.Intent p a}

theorem arm_complete (admitted : ready live = true)
    (wire : HostReplyPrefix.Armed live.source intent)
    (checked : HostReplyPrefix.arm live.source intent = some wire) :
    (arm live intent).isSome = true := by simp [arm,admitted,checked]

theorem armed_gate (armed : Armed live intent) :
    live.current.state.cohort.retired = false ∧ live.current.state.cohort.gate = true ∧
    live.current.state.cohort.call.isSome = true ∧ live.current.state.cohort.pending = [] := by
  simpa [ready,and_assoc] using armed.admitted

variable {armed : Armed live intent}

structure Stopped (armed : Armed live intent) where
  pending : HostReplyPrefix.Pending armed.wire
  stopped : pending.residual.slot.stopped = true

variable {stopped : Stopped armed}

def Stopped.beforeDelivery (armed : Armed live intent) : Stopped armed :=
  ⟨.beforeDelivery armed.wire,rfl⟩
def Stopped.afterDelivery (armed : Armed live intent) : Stopped armed :=
  ⟨.afterDelivery armed.wire,rfl⟩
def Stopped.raw (armed : Armed live intent) (bytes : List UInt8) : Stopped armed :=
  ⟨.rawStopped armed.wire bytes,rfl⟩

theorem Stopped.no_absorb (stopped : Stopped armed) : HostReplyPrefix.absorb stopped.pending = none :=
  HostReplyPrefix.stopped_no_absorb stopped.pending stopped.stopped

theorem Stopped.rooted (stopped : Stopped (live:=live) armed) :
    SharedWireLifetime.Runs (SharedWireLifetime.initial base)
      (live.source.state.joined.history.actions++stopped.pending.residual.actions)
      (SharedReplyRetention.wire armed.wire.attempt stopped.pending.residual.slot) :=
  stopped.pending.rooted

-- General retirement frame: current local memory/debt and the entire settled
-- known-wire history are retained. Retirement is not an application rollback.
theorem source_retire_history (step : SourceEntryPrefix.Step before .retire after) :
    after.joined.history = before.joined.history := by
  cases step with
  | retireIdle live idle wireStep =>
    cases SharedHostCaptureReplay.advance_sound wireStep; rfl
  | retireActive live opened wireStep localStep =>
    cases SharedHostCaptureReplay.advance_sound wireStep; rfl

theorem retire_frames (step : CohortHostPrefix.Step before .retire after) :
    after.cohort.memory = before.cohort.memory ∧ after.cohort.pending = before.cohort.pending ∧
    after.inner.joined.history = before.inner.joined.history ∧
    after.inner.writers = before.inner.writers ∧ after.inner.stored = before.inner.stored := by
  cases step with
  | retire native localStep =>
    have memory := CohortCommitJournal.retire_retains localStep
    have writers := SourceEntryPrefix.retire_keeps_writers native
    exact ⟨memory.1,memory.2.1,source_retire_history native,writers.1,writers.2⟩

-- Keep the residual as runtime DATA, not merely as an index of a returned
-- certificate: erased proof/type parameters cannot be the custody mechanism.
structure Outstanding (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  anchor : CohortHostExecution.Live base
  intent : HostReplyPrefix.Intent p a
  armed : Armed anchor intent
  stopped : Stopped armed

structure Retired (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  origin : Outstanding base
  current : CohortHostExecution.Live base
  step : CohortHostPrefix.Step origin.anchor.current.state .retire current.current.state

def Outstanding.retire (origin : Outstanding base) : Option (Retired base) :=
  match checked : origin.anchor.advance .retire with
  | none => none
  | some current => some ⟨origin,current,CohortHostExecution.Live.advance_step checked⟩

theorem Outstanding.retire_origin (checked : Outstanding.retire origin = some retired) :
    retired.origin = origin := by
  unfold Outstanding.retire at checked
  split at checked
  · cases checked
  · cases Option.some.inj checked; rfl

theorem Retired.keeps (retired : Retired base) :
    retired.current.current.state.cohort.memory = retired.origin.anchor.current.state.cohort.memory ∧
    retired.current.current.state.cohort.pending = retired.origin.anchor.current.state.cohort.pending ∧
    retired.current.source.state.joined.history = retired.origin.anchor.source.state.joined.history := by
  have facts := retire_frames retired.step
  exact ⟨facts.1,facts.2.1,facts.2.2.1⟩

structure Released (base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  retired : Retired base
  current : CohortHostExecution.Live base
  step : CohortHostPrefix.Step retired.current.current.state .releaseRetired current.current.state

def Retired.release (retired : Retired base) : Option (Released base) :=
  match checked : retired.current.advance .releaseRetired with
  | none => none
  | some current => some ⟨retired,current,CohortHostExecution.Live.advance_step checked⟩

theorem Retired.release_origin (checked : Retired.release retired = some released) :
    released.retired = retired := by
  unfold Retired.release at checked
  split at checked
  · cases checked
  · cases Option.some.inj checked; rfl

theorem Released.keeps (released : Released base) :
    released.current.current.state.cohort.memory = released.retired.origin.anchor.current.state.cohort.memory ∧
    released.current.current.state.cohort.pending = released.retired.origin.anchor.current.state.cohort.pending ∧
    released.current.source.state.joined.history = released.retired.origin.anchor.source.state.joined.history := by
  have cleanup := CohortHostPrefix.cleanup_keeps_debt released.step
  have old := released.retired.keeps
  exact ⟨cleanup.2.1.trans old.1,cleanup.2.2.1.trans old.2.1,
    (congrArg (fun s => s.joined.history) cleanup.1).trans old.2.2⟩

def Outstanding.rawBytes (origin : Outstanding base) : Option (List UInt8) :=
  SharedReplyRetention.observedBytes origin.stopped.pending.residual.slot.phase

def Retired.rawBytes (retired : Retired base) : Option (List UInt8) := retired.origin.rawBytes

def Released.rawBytes (released : Released base) : Option (List UInt8) := released.retired.rawBytes

theorem retired_raw_kept (checked : Outstanding.retire origin = some retired) :
    retired.rawBytes = origin.rawBytes := by rw [Retired.rawBytes,Outstanding.retire_origin checked]

theorem released_raw_kept (checked : Retired.release retired = some released) :
    released.rawBytes = retired.rawBytes := by rw [Released.rawBytes,Retired.release_origin checked]

theorem Released.no_absorb (released : Released base) :
    HostReplyPrefix.absorb released.retired.origin.stopped.pending = none :=
  released.retired.origin.stopped.no_absorb

def Outstanding.slot (origin : Outstanding base) : SharedReplyRetention.Slot p a :=
  origin.stopped.pending.residual.slot

def Retired.slot (retired : Retired base) : SharedReplyRetention.Slot p a := retired.origin.slot

def Released.slot (released : Released base) : SharedReplyRetention.Slot p a := released.retired.slot

theorem retired_slot_kept (checked : Outstanding.retire origin = some retired) :
    retired.slot = origin.slot := by rw [Retired.slot,Outstanding.retire_origin checked]

theorem released_slot_kept (checked : Retired.release retired = some released) :
    released.slot = retired.slot := by rw [Released.slot,Retired.release_origin checked]

theorem Released.rooted (released : Released base) :
    SharedWireLifetime.Runs (SharedWireLifetime.initial base)
      (released.current.source.state.joined.history.actions++released.retired.origin.stopped.pending.residual.actions)
      (SharedReplyRetention.wire released.retired.origin.armed.wire.attempt released.slot) := by
  rw [released.keeps.2.2]
  exact released.retired.origin.stopped.rooted

#print axioms retired_slot_kept
#print axioms released_slot_kept
#print axioms Released.rooted

theorem arm_from_rule (admitted : ready live = true)
    (step : SourceEntryPrefix.Step live.source.state
      (intent.event (SharedWireLifetime.reply live.source.state.joined.history.current intent.request)) future) :
    (arm live intent).isSome = true := by
  have present := HostReplyPrefix.arm_complete live.source intent step
  cases result : HostReplyPrefix.arm live.source intent with
  | none => simp [result] at present
  | some wire => exact arm_complete admitted wire result

theorem source_local_live (step : SourceEntryJournal.Step context before action after) : before.retired = false := by
  cases step <;> assumption

theorem cohort_retirement_flags (step : CohortCommitJournal.Step before .retire after) :
    after.retired = true ∧ after.gate = before.gate ∧ after.call = before.call := by
  cases step; exact ⟨rfl,rfl,rfl⟩

theorem retirement_flags (step : CohortHostPrefix.Step before .retire after) :
    after.cohort.retired = true ∧ after.cohort.gate = before.cohort.gate ∧ after.cohort.call = before.cohort.call := by
  cases step with
  | retire native localStep => exact cohort_retirement_flags localStep

theorem cohort_can_release {before : CohortCommitJournal.State p a}
    (flag : before.retired = true) (held : before.gate = true) (called : before.call.isSome = true) :
    ∃ next, CohortCommitJournal.Step before .releaseRetired next := by
  rcases before with ⟨memory,pending,call,gate,stopped⟩
  simp only at flag held called
  subst stopped; subst gate
  cases call with
  | none => cases called
  | some callId => exact ⟨_,.releaseRetired⟩

theorem request_can_retire (intent : HostReplyPrefix.Intent p a)
    (step : SourceEntryPrefix.Step before (intent.event bytes) after) :
    ∃ next, SourceEntryPrefix.Step before .retire next := by
  cases intent with
  | source input | owner endpoint command paid memory =>
    cases step with
    | ordinary live idle allowed binding ran =>
      exact ⟨_,.retireIdle live idle (SharedHostCaptureReplay.advance_complete (.retire live))⟩
  | entry input =>
    cases step with
    | reply live active inputBound localStep wireStep =>
      have localRule := (SourceEntryCapture.Open.advance_sound localStep).2
      have entryLive := source_local_live localRule
      obtain ⟨next,checked,equal⟩ := SourceEntryCapture.Open.advance_complete _ (.retire entryLive)
      exact ⟨_,.retireActive live active (SharedHostCaptureReplay.advance_complete (.retire live)) checked⟩

theorem live_lift (live : CohortHostExecution.Live base)
    (step : CohortHostPrefix.Step live.current.state event next) :
    (live.advance event).isSome = true := by
  have present := CohortHostPrefix.Certified.advance_complete (before:=live.current) step
  cases checked : live.current.advance event with
  | none => simp [checked] at present
  | some after => simp [CohortHostExecution.Live.advance,checked]

theorem cohort_can_retire (live : before.retired = false) :
    ∃ next : CohortCommitJournal.State p a, CohortCommitJournal.Step before .retire next := by
  rcases before with ⟨memory,pending,call,gate,retired⟩
  simp only at live
  subst retired
  exact ⟨_,.retire⟩

theorem Outstanding.retire_total (origin : Outstanding base) : origin.retire.isSome = true := by
  obtain ⟨inner,native⟩ := request_can_retire origin.intent origin.armed.wire.accepted
  obtain ⟨cohort,localStep⟩ := cohort_can_retire (armed_gate origin.armed).1
  have step : CohortHostPrefix.Step origin.anchor.current.state .retire ⟨inner,cohort⟩ :=
    .retire native localStep
  have present := live_lift origin.anchor step
  unfold Outstanding.retire
  split
  · rename_i absent; simp [absent] at present
  · rfl

theorem Retired.release_total (retired : Retired base) : retired.release.isSome = true := by
  have flags := retirement_flags retired.step
  have prior := armed_gate retired.origin.armed
  have called : retired.current.current.state.cohort.call.isSome = true := by
    rw [flags.2.2]; exact prior.2.2.1
  obtain ⟨cohort,localStep⟩ := cohort_can_release flags.1 (flags.2.1.trans prior.2.1) called
  have step : CohortHostPrefix.Step retired.current.current.state .releaseRetired
      ⟨retired.current.current.state.inner,cohort⟩ := .release localStep
  have present := live_lift retired.current step
  unfold Retired.release
  split
  · rename_i absent; simp [absent] at present
  · rfl

#print axioms arm_from_rule
#print axioms request_can_retire
#print axioms live_lift
#print axioms Outstanding.retire_total
#print axioms Retired.release_total

#print axioms arm_complete
#print axioms armed_gate
#print axioms Stopped.beforeDelivery
#print axioms Stopped.afterDelivery
#print axioms Stopped.raw
#print axioms Stopped.no_absorb
#print axioms Stopped.rooted
#print axioms source_retire_history
#print axioms retire_frames
#print axioms Outstanding.retire_origin
#print axioms Retired.release_origin
#print axioms retired_raw_kept
#print axioms released_raw_kept
#print axioms Retired.keeps
#print axioms Released.keeps
#print axioms Released.no_absorb
end CohortHostFailure
