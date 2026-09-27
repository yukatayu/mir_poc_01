import CohortHostExecution
open MirroreaProofFirst SharedHostCaptureReplay
namespace CohortMemoryProvenance
open CohortCommitJournal
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Logical target includes the exact UNEXECUTED store suffix. It is not present
-- physical memory and is not a promise that retired work will ever execute.
-- The atomic receipt interpretation is independent of host store scheduling.
-- Receipts themselves are derived from the SAME native/source transition.
def interpret (before after : SourceEntryPrefix.State base)
    (event : SourceEntryPrefix.Event p a) (memory : Memory p a) : Memory p a :=
  CohortHostDebt.receiptResult memory (CohortHostDebt.receipts before after event)

theorem notify_receipts_empty (classified : CohortHostDebt.kind event = .notify) :
    CohortHostDebt.receipts before after event = [] := by
  cases event <;> simp only [CohortHostDebt.kind] at classified <;> try contradiction
  case ordinary event => cases event <;> cases classified
  case store kind writer value => cases kind <;> rfl

theorem debt_refines (step : CohortHostDebt.Step before after s event next) :
    target next = interpret before after event (target s) := by
  cases step with
  | plain live gate call empty classified =>
    simp [target,empty,interpret,CohortHostDebt.compileReceipts_exact]
  | notify live gate call owed classified =>
    simp [interpret,notify_receipts_empty classified,CohortHostDebt.receiptResult]
  | snapshot live gate call owed =>
    simp [target,owed,interpret,CohortHostDebt.receipts,CohortHostDebt.receiptResult]

-- Native receipt history has no cohort gate, local Store action, pending list,
-- or equality-to-the-desired-result premise. It admits actual inner transitions
-- and applies their atomic receipt interpretation. Local commits stutter here.
inductive NativeHistory : SourceEntryPrefix.State base → Memory p a →
    SourceEntryPrefix.State base → Memory p a → Prop where
  | nil : NativeHistory inner memory inner memory
  | step (prior : NativeHistory first initial before memory)
      (native : SourceEntryPrefix.Step before event after) :
      NativeHistory first initial after (interpret before after event memory)

theorem NativeHistory.trans (left : NativeHistory first initial middle memory)
    (right : NativeHistory middle memory last result) : NativeHistory first initial last result := by
  induction right with
  | nil => exact left
  | step prior native ih => exact .step ih native

theorem NativeHistory.source_path (path : NativeHistory first initial last result) :
    ∃ events, SourceEntryPrefix.Runs first events last := by
  induction path with
  | nil => exact ⟨[],.nil⟩
  | step prior native ih =>
    obtain ⟨events,path⟩ := ih
    exact ⟨_,.step path native⟩

theorem step_refines (step : CohortHostPrefix.Step s event next) :
    NativeHistory s.inner (target s.cohort) next.inner (target next.cohort) := by
  rcases s with ⟨inner,cohort⟩
  cases step with
  | inner native localStores =>
    rw [debt_refines localStores]
    exact .step .nil native
  | enter noInner localStores => cases localStores; exact .nil
  | commit noInner localStores =>
    rw [commit_target localStores]
    exact .nil
  | close localStores native =>
    cases localStores
    cases native with
    | completed noInner ran =>
      simpa [interpret,CohortHostDebt.receipts,CohortHostDebt.receiptResult,target]
        using NativeHistory.step NativeHistory.nil ran
  | retire native localStores =>
    cases localStores
    simpa [interpret,CohortHostDebt.receipts,CohortHostDebt.receiptResult]
      using NativeHistory.step NativeHistory.nil native
  | release localStores => cases localStores; exact .nil

theorem runs_refine (path : CohortHostPrefix.Runs first last) :
    NativeHistory first.inner (target first.cohort) last.inner (target last.cohort) := by
  induction path with
  | nil => exact .nil
  | step prior step ih => exact ih.trans (step_refines step)

-- A normal return has actually discharged its local stores. Only at that
-- boundary can the logical receipt target be replaced by current full memory.
theorem returned_memory (path : CohortHostPrefix.Runs first before)
    (returned : CohortHostPrefix.Step before .close after) :
    NativeHistory first.inner (target first.cohort) after.inner after.cohort.memory := by
  rcases before with ⟨inner,cohort⟩
  have whole := runs_refine (CohortHostPrefix.Runs.step path returned)
  cases returned with
  | close localStores native => cases localStores; exact whole

-- This theorem applies to the current SAME paired certificate. No independently
-- replayed projection, supplied output equality, or intended invariant is input.
theorem certified_origin (checked : CohortHostPrefix.Certified anchor) :
    NativeHistory anchor.inner (target anchor.cohort)
      checked.state.inner (target checked.state.cohort) := runs_refine checked.path

theorem live_origin (live : CohortHostExecution.Live base) :
    NativeHistory (.start base)
      (target (CohortHostStartup.anchor base live.callId).cohort)
      live.current.state.inner (target live.current.state.cohort) := certified_origin live.current

-- Non-vacuity: every lower transition of the selected plain class, from an
-- actual outer entry with no local debt, advances to its entire receipt target.
-- Memory is arbitrary here: no correspondence conclusion is an input guard.
theorem admitted_native (native : SourceEntryPrefix.Step before event after)
    (plain : CohortHostDebt.kind event = .plain) (memory : Memory p a) (callId : Nat) :
    ∃ next, CohortHostPrefix.advance ⟨before,⟨memory,[],some callId,true,false⟩⟩ (.inner event) = some next ∧
      next.inner = after ∧ target next.cohort = interpret before after event memory := by
  have step := CohortHostPrefix.inner_lift native plain memory callId
  refine ⟨_,CohortHostPrefix.advance_complete step,rfl,?_⟩
  simp [target,interpret,CohortHostDebt.compileReceipts_exact]

-- Decisive counterexample to equating physical memory with the receipt target
-- at every prefix: a pending snapshot has not yet been physically stored.
theorem pending_snapshot_is_not_current (memory : Memory p a)
    (snapshot : Option (SourcePublicationWorker.PrivateOutput p a))
    (different : snapshot ≠ memory.snapshot) (callId : Nat) :
    target (⟨memory,[.snapshot snapshot],some callId,true,false⟩ : State p a) ≠ memory := by
  intro equal
  exact different (congrArg Memory.snapshot equal)

-- The same discrepancy actually appears at the proved startup/launch cut.
-- Its pending snapshot is the projection of the actual native certificate.
theorem launch_not_current (actual : CohortHostExecution.Snapshot base ≠ none) (callId : Nat) :
    target (CohortHostStartup.anchor base callId).cohort ≠
      (CohortHostStartup.anchor base callId).cohort.memory := by
  exact pending_snapshot_is_not_current (write CohortHostStartup.empty .bootstrap)
    (CohortHostExecution.Snapshot base) actual callId

#print axioms launch_not_current
#print axioms debt_refines
#print axioms step_refines
#print axioms runs_refine
#print axioms returned_memory
#print axioms certified_origin
#print axioms live_origin
#print axioms admitted_native
#print axioms pending_snapshot_is_not_current
end CohortMemoryProvenance
