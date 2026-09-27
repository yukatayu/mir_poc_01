import CohortMemoryProvenance
import CohortSnapshotCorrespondence
open MirroreaProofFirst SharedHostCaptureReplay
namespace OwnerCurrentCorrespondence
open OwnerCommitJournal
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

theorem source_owners
    (ran : SharedWireLifetime.History.knownStep before (.source input) bytes = some after) :
    (SharedWireLifetime.native after.current).owners = (SharedWireLifetime.native before.current).owners := by
  rw [SharedHostJoin.history_native ran]
  rcases input with head | (owner | request) <;> rfl

theorem owner_other
    (ran : SharedWireLifetime.History.knownStep before (.owner owner command paid) bytes = some after)
    (different : other ≠ owner) :
    (SharedWireLifetime.native after.current).owners other =
      (SharedWireLifetime.native before.current).owners other := by
  rw [SharedHostJoin.history_native ran]
  simp [SharedNativeStep.execute,SharedWireLifetime.event,PublicOwnerBoundary.put,different]

theorem entry_step_owners (step : SourceEntryJournal.Step context before action after) :
    (SharedWireLifetime.native after.history.current).owners =
      (SharedWireLifetime.native before.history.current).owners := by
  cases step with
  | replyAccepted live phase known accepted => exact source_owners known
  | replyRefused live phase known refused => exact source_owners known
  | notify | cancel | storeAccepted | storeRefused | retire => rfl

theorem entry_path_owners (path : SourceEntryJournal.Runs context before after) :
    (SharedWireLifetime.native after.history.current).owners =
      (SharedWireLifetime.native before.history.current).owners := by
  induction path with
  | nil => rfl
  | step prior step ih => exact (entry_step_owners step).trans ih

theorem entry_data (opened : SourceEntryCapture.Open base) :
    opened.state.writer.data =
      project ((SharedWireLifetime.native opened.state.history.current).owners opened.context.endpoint) := by
  rw [entry_path_owners opened.path]
  exact opened.valid.1.trans opened.beforeData

theorem open_fields
    (ran : openBound history owner command paid bytes memory = some bound) :
    bound.writer.owner = owner ∧ bound.writer.current = memory := by
  unfold openBound at ran
  dsimp only at ran
  split at ran
  · cases ran
  · split at ran
    · split at ran
      · cases Option.some.inj ran; exact ⟨rfl,rfl⟩
      · cases ran
    · cases ran

theorem observe_fields (ran : BoundWriter.observe before memory = some after) :
    after.writer.owner = before.writer.owner ∧ after.writer.current = memory := by
  unfold BoundWriter.observe at ran
  dsimp only at ran
  split at ran
  · cases ran
  · cases Option.some.inj ran; exact ⟨rfl,rfl⟩

-- Current memory is equal to native data for idle owners. The one active writer
-- instead retains its actual sampled current memory and an ordered local suffix;
-- it is not prematurely equated to the native state after the reply.
def Aligned (state : SourceEntryPrefix.State base) : Prop :=
  match state.joined.active with
  | none => ∀ owner, (state.writers owner).data =
      project ((SharedWireLifetime.native state.joined.history.current).owners owner)
  | some opened => state.writers opened.bound.writer.owner = opened.bound.writer.current ∧
      ∀ owner, owner ≠ opened.bound.writer.owner → (state.writers owner).data =
        project ((SharedWireLifetime.native state.joined.history.current).owners owner)

theorem initial_aligned : Aligned (SourceEntryPrefix.State.start base) := by intro owner; rfl

-- Full delayed writer target comes from its own native operation. Combining
-- this with Aligned ties it to actual current memory, not to a second model.
theorem active_target (bound : BoundWriter base) :
    bound.writer.pending.foldl write bound.writer.current =
      ⟨project ((SharedWireLifetime.native bound.later.current).owners bound.writer.owner),none,false⟩ := by
  exact (runs_target bound.writer.path).trans (by simpa [bound.afterBinding] using bound.writer.nativeTarget)

theorem preserves {before after : SourceEntryPrefix.State base}
    (step : SourceEntryPrefix.Step before event after)
    (entryValid : SourceEntryPrefix.Invariant before)
    (historyValid : HistoryInvariant before.joined) (prior : Aligned before) : Aligned after := by
  cases step with
  | ordinary live idle allowed binding joinedRun =>
    cases SharedHostCaptureReplay.advance_sound joinedRun with
    | source lowerLive noWriter ran =>
      simpa [Aligned,noWriter,SourceEntryPrefix.observedWriters,source_owners ran] using prior
    | owner lowerLive noWriter ran =>
      rename_i owner command paid bytes observed bound
      have fields := open_fields ran
      have observedAt : observed = before.writers owner := memoryEq_exact.mp binding
      simp only [Aligned,noWriter] at prior
      constructor
      · simpa [fields.1,fields.2] using observedAt.symm
      · intro other different
        change (before.writers other).data = project ((SharedWireLifetime.native bound.later.current).owners other)
        rw [owner_other bound.known (by simpa [fields.1] using different),open_earlier ran]
        exact prior other
    | observe lowerLive active same ran =>
      rename_i opened owner bound observed
      have fields := observe_fields ran
      simp only [Aligned,active] at prior
      constructor
      · simp [SourceEntryPrefix.observedWriters,SourceEntryPrefix.put,fields.1,fields.2,same]
      · intro other different
        have differentOwner : other ≠ owner := by simpa [fields.1,same] using different
        simpa [SourceEntryPrefix.observedWriters,SourceEntryPrefix.put,differentOwner] using
          prior.2 other (by simpa [fields.1] using different)
    | confirmed lowerLive active same ran =>
      rename_i opened owner closed observed
      simp only [Aligned,active] at prior
      intro other
      by_cases equal : other = owner
      · subst other
        have data := closed.nativeData
        rw [finish_observed ran,finish_bound ran,historyValid.1 opened active,same] at data
        simpa [SourceEntryPrefix.observedWriters,SourceEntryPrefix.put] using data
      · simpa [SourceEntryPrefix.observedWriters,SourceEntryPrefix.put,equal] using
          prior.2 other (by simpa [same] using equal)
    | localComplete lowerLive noWriter ran =>
      simpa [Aligned,noWriter,SourceEntryPrefix.observedWriters,
        CohortSnapshotCorrespondence.finished_native ran] using prior
    | retire => exact prior
  | claim live idle writerIdle binding vectorBound ran =>
    rename_i observed vector endpoint staged snapshot opened
    have fields := SourceEntryCapture.start_sound ran
    have data : staged.data = project ((SharedWireLifetime.native before.joined.history.current).owners endpoint) := by
      simpa only [fields.2.1,fields.2.2.1,fields.2.2.2.2.2.1] using opened.beforeData
    simp only [Aligned,writerIdle] at prior ⊢
    intro other
    by_cases equal : other = endpoint
    · subst other; simpa [SourceEntryPrefix.put] using data
    · simpa [SourceEntryPrefix.put,equal] using prior other
  | reply live active binding localRun wireRun =>
    cases SharedHostCaptureReplay.advance_sound wireRun with
    | source lowerLive noWriter ran =>
      simpa [Aligned,noWriter,source_owners ran] using prior
  | store live active localRun =>
    rename_i opened next kind observed snapshot
    obtain ⟨sameHistory,sameWriter,noWriter⟩ := entryValid.1 opened active
    have checked := SourceEntryCapture.Open.observe_sound localRun
    have data := entry_data next
    have history := SourceEntryPrefix.local_history checked.2.1
    rw [checked.2.2.1,history,sameHistory,checked.1] at data
    simp only [Aligned,noWriter,SourceEntryPrefix.afterStore] at prior ⊢
    intro other
    by_cases equal : other = opened.entry.context.endpoint
    · subst other; simpa [SourceEntryPrefix.put] using data
    · simpa [SourceEntryPrefix.put,equal] using prior other
  | retireIdle live idle wireRun => cases SharedHostCaptureReplay.advance_sound wireRun; exact prior
  | retireActive live active wireRun localRun => cases SharedHostCaptureReplay.advance_sound wireRun; exact prior

theorem runs_aligned {before after : SourceEntryPrefix.State base}
    (path : SourceEntryPrefix.Runs before events after)
    (entryValid : SourceEntryPrefix.Invariant before)
    (historyValid : HistoryInvariant before.joined) (prior : Aligned before) : Aligned after := by
  induction path with
  | nil => exact prior
  | step path step ih =>
    exact preserves step (SourceEntryPrefix.runs_preserves entryValid path)
      (runs_history historyValid (SourceEntryPrefix.runs_projection path)) ih

theorem certified_aligned (checked : SourceEntryPrefix.Certified base) : Aligned checked.state :=
  runs_aligned checked.path SourceEntryPrefix.initial_valid SharedHostCaptureReplay.initial_valid initial_aligned

theorem live_aligned (live : CohortHostExecution.Live base) : Aligned live.current.state.inner :=
  certified_aligned live.source

#print axioms source_owners
#print axioms owner_other
#print axioms entry_step_owners
#print axioms entry_path_owners
#print axioms entry_data
#print axioms open_fields
#print axioms observe_fields
#print axioms initial_aligned
#print axioms active_target
#print axioms preserves
#print axioms runs_aligned
#print axioms certified_aligned
#print axioms live_aligned
end OwnerCurrentCorrespondence
