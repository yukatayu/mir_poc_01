import CohortMemoryProvenance
open MirroreaProofFirst SharedHostCaptureReplay
namespace CohortSnapshotCorrespondence
open CohortCommitJournal
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

def nativeSnapshot (state : SourceEntryPrefix.State base) :=
  (SharedWireLifetime.native state.joined.history.current).driver.source.map SourcePublicationWorker.project

def framesSnapshot : Store p a → Bool
  | .snapshot _ => false
  | _ => true

theorem write_snapshot (frame : framesSnapshot store = true) :
    (write memory store).snapshot = memory.snapshot := by
  cases store <;> simp_all [framesSnapshot,write]

theorem fold_snapshot (stores : List (Store p a))
    (frame : ∀ store ∈ stores, framesSnapshot store = true) (memory : Memory p a) :
    (stores.foldl write memory).snapshot = memory.snapshot := by
  induction stores generalizing memory with
  | nil => rfl
  | cons first rest ih =>
    have firstFrame := frame first List.mem_cons_self
    have restFrame : ∀ store ∈ rest, framesSnapshot store = true :=
      fun store member => frame store (List.mem_cons_of_mem first member)
    exact (ih restFrame (write memory first)).trans (write_snapshot firstFrame)

theorem owner_store_frames (member : store ∈ ownerStores memory owner command reply payment) :
    framesSnapshot store = true := by
  unfold ownerStores at member
  simp only [List.mem_append] at member
  rcases member with (initialized | paid) | tagged
  · split at initialized
    · split at initialized <;> simp_all [framesSnapshot]
    · simp at initialized
  · obtain ⟨value,_,same⟩ := List.mem_map.mp paid
    subst store
    rfl
  · split at tagged <;> simp_all [framesSnapshot]

theorem owner_recipe_snapshot (memory : Memory p a) (owner : Fin p)
    (command : OwnerEndpoint.Command p a) (reply : Sum Nat OwnerReceipt.Envelope)
    (payment : Option (PaidHeadPhase.Pending p)) :
    ((recipe memory (.ownerReturned owner command reply payment)).foldl write memory).snapshot = memory.snapshot :=
  fold_snapshot _ (fun _ member => owner_store_frames member) memory

theorem finished_snapshot (memory : Memory p a) (envelope : Option OwnerReceipt.Envelope) :
    (CohortHostDebt.receiptResult memory (envelope.toList.map Receipt.workFinished)).snapshot = memory.snapshot := by
  cases envelope <;> rfl

theorem source_receipts_snapshot
    (ran : SharedWireLifetime.History.knownStep before (.source input) bytes = some after)
    (current : memory.snapshot =
      (SharedWireLifetime.native before.current).driver.source.map SourcePublicationWorker.project) :
    (CohortHostDebt.receiptResult memory (CohortHostReceipt.sourceReceipts before after input)).snapshot =
      (SharedWireLifetime.native after.current).driver.source.map SourcePublicationWorker.project := by
  cases input with
  | inl head =>
    have native := SharedHostJoin.history_native ran
    simpa [CohortHostReceipt.sourceReceipts,CohortHostDebt.receiptResult,native,
      SharedNativeStep.execute,SharedWireLifetime.event,SharedJointDriver.inputEvent] using current
  | inr input =>
    cases input with
    | inl owner =>
      have native := SharedHostJoin.history_native ran
      simpa [CohortHostReceipt.sourceReceipts,CohortHostDebt.receiptResult,native,
        SharedNativeStep.execute,SharedWireLifetime.event,SharedJointDriver.inputEvent] using current
    | inr request =>
      simp only [CohortHostReceipt.sourceReceipts,List.singleton_append,CohortHostDebt.receiptResult]
      rw [finished_snapshot]
      cases paid : CohortHostReceipt.paymentOf before.current.mode <;>
        simp [recipe,write]

theorem known_owner_snapshot
    (ran : SharedWireLifetime.History.knownStep before (.owner owner command paid) bytes = some after) :
    (SharedWireLifetime.native after.current).driver.source.map SourcePublicationWorker.project =
      (SharedWireLifetime.native before.current).driver.source.map SourcePublicationWorker.project := by
  rw [SharedHostJoin.history_native ran]
  rfl

theorem opened_snapshot
    (ran : openBound history owner command paid bytes memory = some opened) :
    (SharedWireLifetime.native opened.later.current).driver.source.map SourcePublicationWorker.project =
      (SharedWireLifetime.native history.current).driver.source.map SourcePublicationWorker.project := by
  have frame := known_owner_snapshot opened.known
  simpa only [open_earlier ran] using frame

theorem finished_native (ran : SharedWireLifetime.History.finishStep before = some after) :
    SharedWireLifetime.native after.current = SharedWireLifetime.native before.current := by
  unfold SharedWireLifetime.History.finishStep at ran
  split at ran
  · cases ran
  · rename_i next finished
    cases Option.some.inj ran
    exact SharedFundedDriver.finish_native finished

-- Correspondence to the actual native source, not merely to a second fold of
-- receipt recipes. It holds at every known prefix for the DELAYED store target.
theorem interpret_snapshot (step : SourceEntryPrefix.Step before event after)
    (current : memory.snapshot = nativeSnapshot before) :
    (CohortMemoryProvenance.interpret before after event memory).snapshot = nativeSnapshot after := by
  cases step with
  | ordinary live idle allowed bound joinedRun =>
    cases SharedHostCaptureReplay.advance_sound joinedRun with
    | source lowerLive lowerIdle wireRun => exact source_receipts_snapshot wireRun current
    | owner lowerLive lowerIdle openRun =>
      simpa [CohortMemoryProvenance.interpret,CohortHostDebt.receipts,CohortHostDebt.receiptResult,
        nativeSnapshot,opened_snapshot openRun] using current
    | observe => exact current
    | confirmed live active same ran =>
      simpa [CohortMemoryProvenance.interpret,CohortHostDebt.receipts,active,
        CohortHostDebt.receiptResult,CohortHostReceipt.ownerReceipt,owner_recipe_snapshot,nativeSnapshot] using current
    | localComplete lowerLive lowerIdle finishRun =>
      simpa [CohortMemoryProvenance.interpret,CohortHostDebt.receipts,CohortHostDebt.receiptResult,
        nativeSnapshot,finished_native finishRun] using current
    | retire => exact current
  | claim => exact current
  | reply live active bound localStep wireStep =>
    cases SharedHostCaptureReplay.advance_sound wireStep with
    | source _ _ ran => exact source_receipts_snapshot ran current
  | store => exact current
  | retireIdle live idle wireStep => cases SharedHostCaptureReplay.advance_sound wireStep; exact current
  | retireActive live active wireStep localStep => cases SharedHostCaptureReplay.advance_sound wireStep; exact current

theorem history_snapshot
    (path : CohortMemoryProvenance.NativeHistory first initial last result)
    (initialSnapshot : initial.snapshot = nativeSnapshot first) :
    result.snapshot = nativeSnapshot last := by
  induction path with
  | nil => exact initialSnapshot
  | step prior native ih => exact interpret_snapshot native ih

theorem paired_snapshot (path : CohortHostPrefix.Runs first last)
    (initialSnapshot : (target first.cohort).snapshot = nativeSnapshot first.inner) :
    (target last.cohort).snapshot = nativeSnapshot last.inner :=
  history_snapshot (CohortMemoryProvenance.runs_refine path) initialSnapshot

theorem live_snapshot (live : CohortHostExecution.Live base) :
    (target live.current.state.cohort).snapshot = nativeSnapshot live.current.state.inner :=
  history_snapshot (CohortMemoryProvenance.live_origin live) rfl

-- This describes actual saved memory only after its corresponding store debt
-- has been discharged. In an unknown IO prefix this remains the LAST KNOWN
-- native state; it is not knowledge of an unreceived endpoint result.
theorem discharged_snapshot (live : CohortHostExecution.Live base)
    (empty : live.current.state.cohort.pending = []) :
    live.current.state.cohort.memory.snapshot = nativeSnapshot live.current.state.inner := by
  simpa [target,empty] using live_snapshot live

theorem closed_snapshot (live : CohortHostExecution.Live base)
    (returned : CohortHostPrefix.Step live.current.state .close after) :
    after.cohort.memory.snapshot = nativeSnapshot after.inner :=
  history_snapshot (CohortMemoryProvenance.returned_memory live.current.path returned) rfl

#print axioms owner_recipe_snapshot
#print axioms source_receipts_snapshot
#print axioms interpret_snapshot
#print axioms history_snapshot
#print axioms paired_snapshot
#print axioms live_snapshot
#print axioms discharged_snapshot
#print axioms closed_snapshot
end CohortSnapshotCorrespondence
