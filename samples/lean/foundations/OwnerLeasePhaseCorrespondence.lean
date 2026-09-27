import OwnerLeasePhaseEntry
open MirroreaProofFirst SharedHostCaptureReplay
namespace OwnerLeasePhaseCorrespondence
open OwnerLeasePhaseRelation OwnerLeasePhaseFunding OwnerLeasePhaseEntry CohortHostReceipt
set_option maxHeartbeats 2000000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

theorem source_frame
    (allowed : SourceEntryPrefix.ordinaryAllowed (.source input bytes) = true)
    (ran : SharedWireLifetime.History.knownStep before (.source input) bytes = some after) :
    expected after.current.mode owner = expected before.current.mode owner := by
  rcases input with head | (index | request)
  · exact query_frame (history_funding ran)
  · exact query_frame (history_funding ran)
  · exact source_ordinary_frame allowed (history_funding ran)

theorem finish_frame (ran : SharedFundedDriver.finish before = some after) :
    expected after.mode owner = expected before.mode owner := by
  unfold SharedFundedDriver.finish at ran
  split at ran
  · cases ran
  · split at ran
    · cases Option.some.inj ran; rfl
    · rename_i next completed
      cases Option.some.inj ran
      exact local_frame next.property

theorem history_finish_frame
    (ran : SharedWireLifetime.History.finishStep before = some after) :
    expected after.current.mode owner = expected before.current.mode owner := by
  unfold SharedWireLifetime.History.finishStep at ran
  split at ran
  · cases ran
  · rename_i next finished
    cases Option.some.inj ran
    exact finish_frame finished

theorem snapshot_phase (step : SourceEntryJournal.Step context before .storeSnapshot after) :
    after.phase = .acceptedStored ∨ after.phase = .refusedStored := by
  cases step <;> simp

theorem preserves {before after : SourceEntryPrefix.State base}
    (step : SourceEntryPrefix.Step before event after)
    (entryValid : SourceEntryPrefix.Invariant before)
    (historyValid : HistoryInvariant before.joined) (prior : Aligned before) : Aligned after := by
  cases step with
  | ordinary live idle allowed binding joinedRun =>
    cases SharedHostCaptureReplay.advance_sound joinedRun with
    | source lowerLive noWriter ran =>
      intro owner
      simpa [AtOwner,noWriter,idle,SourceEntryPrefix.observedWriters,source_frame allowed ran]
        using prior owner
    | owner lowerLive noWriter ran =>
      rename_i target command paid bytes observed bound
      have fields := OwnerCurrentCorrespondence.open_fields ran
      intro other different
      have meaning := prior other
      simp only [AtOwner,noWriter,idle] at meaning
      have releasedAt := owner_other_before (history_funding bound.known)
        (show other ≠ bound.writer.owner from different)
      change expected bound.earlier.current.mode other = released at releasedAt
      rw [open_earlier ran] at releasedAt
      simpa [AtOwner,SourceEntryPrefix.observedWriters,releasedAt] using meaning
    | observe lowerLive active same ran =>
      rename_i opened target bound observed
      have fields := OwnerCurrentCorrespondence.observe_fields ran
      simp only [Aligned,AtOwner,active] at prior
      intro other different
      have differentTarget : other ≠ target := by simpa [fields.1,same] using different
      have meaning := prior other (by simpa [AtOwner,active,fields.1] using different)
      simpa [AtOwner,SourceEntryPrefix.observedWriters,SourceEntryPrefix.put,differentTarget] using meaning
    | confirmed lowerLive active same ran =>
      rename_i opened target closed observed
      simp only [Aligned,AtOwner,active] at prior
      have phase := owner_after (owner:=target) (history_funding opened.bound.known)
      change expected opened.bound.later.current.mode target = released at phase
      rw [historyValid.1 opened active] at phase
      intro other
      by_cases equal : other = target
      · subst other
        have lease := closed.leaseReleased
        have entered := closed.enteredCleared
        rw [finish_observed ran] at lease entered
        simp [AtOwner,idle,SourceEntryPrefix.observedWriters,SourceEntryPrefix.put,phase,leasePair,lease,entered,released]
      · have meaning := prior other (by simpa [AtOwner,active,same] using equal)
        have phaseOther := owner_after (owner:=other) (history_funding opened.bound.known)
        change expected opened.bound.later.current.mode other = released at phaseOther
        rw [historyValid.1 opened active] at phaseOther
        simpa [AtOwner,idle,SourceEntryPrefix.observedWriters,SourceEntryPrefix.put,equal,phaseOther] using meaning
    | localComplete lowerLive noWriter ran =>
      intro owner
      simpa [AtOwner,noWriter,idle,SourceEntryPrefix.observedWriters,history_finish_frame ran] using prior owner
    | retire => exact prior
  | claim live idle writerIdle binding vectorBound ran =>
    rename_i observed vector endpoint staged snapshot opened
    have fields := SourceEntryCapture.start_sound ran
    simp only [Aligned,AtOwner,writerIdle]
    intro other different
    have otherEndpoint : other ≠ endpoint := by simpa [fields.2.2.1] using different
    have meaning := prior other
    simp only [AtOwner,writerIdle,idle] at meaning
    have releasedAt := started_released (owner:=other) ran
    simpa [AtOwner,writerIdle,SourceEntryPrefix.put,otherEndpoint,releasedAt] using meaning
  | reply live active binding localRun wireRun =>
    have context := (SourceEntryCapture.Open.advance_sound localRun).1
    cases SharedHostCaptureReplay.advance_sound wireRun with
    | source lowerLive noWriter ran =>
      simp only [Aligned,AtOwner,noWriter,active] at prior
      simp only [Aligned,AtOwner,noWriter]
      intro other different
      have meaning := prior other (by simpa [AtOwner,noWriter,active,context] using different)
      simpa [AtOwner] using meaning
  | store live active localRun =>
    rename_i opened next kind observed snapshot
    obtain ⟨sameHistory,sameWriter,noWriter⟩ := entryValid.1 opened active
    have checked := SourceEntryCapture.Open.observe_sound localRun
    have history := SourceEntryPrefix.local_history checked.2.1
    simp only [Aligned,AtOwner,noWriter,active] at prior
    cases kind with
    | notify =>
      simp only [Aligned,AtOwner,SourceEntryPrefix.afterStore,noWriter]
      intro other different
      have meaning := prior other (by simpa [AtOwner,noWriter,active,checked.1] using different)
      have distinct : other ≠ opened.entry.context.endpoint := by simpa [checked.1] using different
      simpa [AtOwner,noWriter,SourceEntryPrefix.afterStore,SourceEntryPrefix.put,distinct] using meaning
    | cancel =>
      simp only [Aligned,AtOwner,SourceEntryPrefix.afterStore,noWriter]
      intro other different
      have meaning := prior other (by simpa [AtOwner,noWriter,active,checked.1] using different)
      have distinct : other ≠ opened.entry.context.endpoint := by simpa [checked.1] using different
      simpa [AtOwner,noWriter,SourceEntryPrefix.afterStore,SourceEntryPrefix.put,distinct] using meaning
    | snapshot =>
      have stored := snapshot_phase checked.2.1
      intro other
      have nativeMeaning := stored_expected next stored other
      rw [history,sameHistory,checked.1,checked.2.2.1] at nativeMeaning
      by_cases equal : other = opened.entry.context.endpoint
      · simpa [AtOwner,noWriter,SourceEntryPrefix.afterStore,SourceEntryPrefix.put,equal] using nativeMeaning.symm
      · have meaning := prior other (by simpa [AtOwner,noWriter,active] using equal)
        simpa [AtOwner,noWriter,SourceEntryPrefix.afterStore,SourceEntryPrefix.put,equal,nativeMeaning] using meaning
  | retireIdle live idle wireRun => cases SharedHostCaptureReplay.advance_sound wireRun; exact prior
  | retireActive live active wireRun localRun =>
    have noWriter := (entryValid.1 _ active).2.2
    have context := (SourceEntryCapture.Open.advance_sound localRun).1
    cases SharedHostCaptureReplay.advance_sound wireRun
    simpa [Aligned,AtOwner,noWriter,active,context] using prior

theorem runs_aligned {before after : SourceEntryPrefix.State base}
    (path : SourceEntryPrefix.Runs before events after)
    (entryValid : SourceEntryPrefix.Invariant before)
    (historyValid : HistoryInvariant before.joined) (prior : Aligned before) : Aligned after := by
  induction path with
  | nil => exact prior
  | step path step ih =>
    exact preserves step (SourceEntryPrefix.runs_preserves entryValid path)
      (runs_history historyValid (SourceEntryPrefix.runs_projection path)) ih

theorem live_aligned (live : CohortHostExecution.Live base) (fresh : base.mode = .prelude owed) :
    Aligned live.current.state.inner :=
  runs_aligned live.source.path SourceEntryPrefix.initial_valid SharedHostCaptureReplay.initial_valid
    (fresh_aligned fresh)

#print axioms source_frame
#print axioms history_finish_frame
#print axioms preserves
#print axioms runs_aligned
#print axioms live_aligned
end OwnerLeasePhaseCorrespondence
