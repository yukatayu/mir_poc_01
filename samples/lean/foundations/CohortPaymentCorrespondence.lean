import CohortMemoryProvenance
open MirroreaProofFirst SharedHostCaptureReplay
namespace CohortPaymentCorrespondence
open CohortCommitJournal CohortHostReceipt
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Native payment is created at a known owner reply; cohort payment is stored
-- only after this SAME writer returns. During that interval use its earlier
-- phase, not the already advanced native phase. Local store debt is separate.
def expected (state : SourceEntryPrefix.State base) : Option (PaidHeadPhase.Pending p) :=
  match state.joined.active with
  | none => paymentOf state.joined.history.current.mode
  | some opened => paymentOf opened.bound.earlier.current.mode

def framesPayment : Store p a → Bool
  | .payment _ | .clearPayment => false
  | _ => true

theorem write_payment (frame : framesPayment store = true) :
    (write memory store).pendingPayment = memory.pendingPayment := by
  cases store <;> simp_all [framesPayment,write]

theorem fold_payment (stores : List (Store p a))
    (frame : ∀ store ∈ stores, framesPayment store = true) (memory : Memory p a) :
    (stores.foldl write memory).pendingPayment = memory.pendingPayment := by
  induction stores generalizing memory with
  | nil => rfl
  | cons first rest ih =>
    exact (ih (fun store member => frame store (List.mem_cons_of_mem first member))
      (write memory first)).trans (write_payment (frame first List.mem_cons_self))

theorem owner_before_unpaid
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before)
      (.owner owner command) (some after)) : paymentOf before.mode = none := by
  cases step <;> simp_all [paymentOf]

theorem query_frames
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before)
      (.query request) (some after)) : paymentOf after.mode = paymentOf before.mode := by
  cases step; rfl

theorem local_frames
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before)
      .local (some after)) : paymentOf after.mode = paymentOf before.mode := by
  cases step <;> simp_all [paymentOf]

theorem source_after_unpaid
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before)
      (.source request) (some after)) : paymentOf after.mode = none := by
  cases step <;> simp_all [paymentOf,CohortPhase.sourceResult]
  case refused modes _ _ =>
    rcases modes with ordinary | ⟨owed,prelude⟩ <;> simp [*]

-- This local algebraic lemma does not authenticate the optional payment.
-- owner_payment_origin separately derives it from the actual successful step.
theorem owner_recipe_payment (memory : Memory p a) (owner : Fin p)
    (command : OwnerEndpoint.Command p a) (reply : Sum Nat OwnerReceipt.Envelope)
    (payment : Option (PaidHeadPhase.Pending p)) :
    ((recipe memory (.ownerReturned owner command reply payment)).foldl write memory).pendingPayment =
      payment.or memory.pendingPayment := by
  cases command with
  | freeze revision =>
    cases reply with
    | inr envelope => cases payment <;> simp [recipe,ownerStores,write]
    | inl code =>
      by_cases success : code = 12
      · subst code; cases payment <;> simp [recipe,ownerStores,write]
      · cases payment <;> simp [recipe,ownerStores,write,success]
  | owner command =>
    cases reply with
    | inr envelope => cases command <;> cases payment <;> simp [recipe,ownerStores,write]
    | inl code =>
      by_cases initialized : code = 10
      · subst code
        by_cases present : owner ∈ memory.initialized
        all_goals cases command <;> cases payment <;> simp [recipe,ownerStores,write,present]
      · by_cases installed : code = 7
        · subst code; cases command <;> cases payment <;> simp [recipe,ownerStores,write]
        · cases command <;> cases payment <;> simp [recipe,ownerStores,write,initialized,installed]

theorem finished_frames (memory : Memory p a) (envelope : Option OwnerReceipt.Envelope) :
    (CohortHostDebt.receiptResult memory (envelope.toList.map Receipt.workFinished)).pendingPayment =
      memory.pendingPayment := by cases envelope <;> rfl

theorem source_payment
    (ran : SharedWireLifetime.History.knownStep before (.source input) bytes = some after)
    (prior : memory.pendingPayment = paymentOf before.current.mode) :
    (CohortHostDebt.receiptResult memory (sourceReceipts before after input)).pendingPayment =
      paymentOf after.current.mode := by
  rcases input with head | (owner | request)
  · exact prior.trans (query_frames (history_funding ran)).symm
  · exact prior.trans (query_frames (history_funding ran)).symm
  · have afterUnpaid := source_after_unpaid (history_funding ran)
    change paymentOf after.current.mode = none at afterUnpaid
    rw [afterUnpaid]
    simp only [sourceReceipts,List.singleton_append,CohortHostDebt.receiptResult]
    rw [finished_frames]
    cases paid : paymentOf before.current.mode <;> simp [recipe,write,prior,paid]

theorem finish_payment (ran : SharedFundedDriver.finish before = some after) :
    paymentOf after.mode = paymentOf before.mode := by
  unfold SharedFundedDriver.finish at ran
  split at ran
  · cases ran
  · split at ran
    · cases Option.some.inj ran; rfl
    · rename_i next completed
      cases Option.some.inj ran
      exact local_frames next.property

theorem history_finish_payment
    (ran : SharedWireLifetime.History.finishStep before = some after) :
    paymentOf after.current.mode = paymentOf before.current.mode := by
  unfold SharedWireLifetime.History.finishStep at ran
  split at ran
  · cases ran
  · rename_i next finished
    cases Option.some.inj ran
    exact finish_payment finished

theorem interpret_payment (step : SourceEntryPrefix.Step before event after)
    (valid : HistoryInvariant before.joined)
    (prior : memory.pendingPayment = expected before) :
    (CohortMemoryProvenance.interpret before after event memory).pendingPayment = expected after := by
  cases step with
  | ordinary live idle allowed binding joinedRun =>
    cases SharedHostCaptureReplay.advance_sound joinedRun with
    | source lowerLive noWriter ran =>
      simpa [CohortMemoryProvenance.interpret,CohortHostDebt.receipts,expected,noWriter]
        using source_payment ran (by simpa [expected,noWriter] using prior)
    | owner lowerLive noWriter ran =>
      simpa [CohortMemoryProvenance.interpret,CohortHostDebt.receipts,CohortHostDebt.receiptResult,
        expected,open_earlier ran,noWriter] using prior
    | observe lowerLive active same ran =>
      simpa [CohortMemoryProvenance.interpret,CohortHostDebt.receipts,CohortHostDebt.receiptResult,
        expected,(observe_history ran).1,active] using prior
    | confirmed lowerLive active same ran =>
      rename_i opened owner closed observed
      have unpaid := owner_before_unpaid (history_funding opened.bound.known)
      change paymentOf opened.bound.earlier.current.mode = none at unpaid
      have currentEmpty : memory.pendingPayment = none := by
        simpa [expected,active,unpaid] using prior
      have sameHistory := valid.1 opened active
      simp [CohortMemoryProvenance.interpret,CohortHostDebt.receipts,active,
        CohortHostDebt.receiptResult,ownerReceipt,owner_recipe_payment,currentEmpty,expected,sameHistory]
    | localComplete lowerLive noWriter ran =>
      simpa [CohortMemoryProvenance.interpret,CohortHostDebt.receipts,CohortHostDebt.receiptResult,
        expected,noWriter,history_finish_payment ran] using prior
    | retire => exact prior
  | claim => exact prior
  | reply live active inputBinding localRun wireRun =>
    cases SharedHostCaptureReplay.advance_sound wireRun with
    | source lowerLive noWriter ran =>
      simpa [CohortMemoryProvenance.interpret,CohortHostDebt.receipts,expected,noWriter]
        using source_payment ran (by simpa [expected,noWriter] using prior)
  | store => exact prior
  | retireIdle live idle wireRun => cases SharedHostCaptureReplay.advance_sound wireRun; exact prior
  | retireActive live active wireRun localRun => cases SharedHostCaptureReplay.advance_sound wireRun; exact prior

theorem history_payment
    (path : CohortMemoryProvenance.NativeHistory first initial last result)
    (valid : HistoryInvariant first.joined)
    (initialPayment : initial.pendingPayment = expected first) :
    result.pendingPayment = expected last := by
  induction path with
  | nil => exact initialPayment
  | step prior native ih =>
    obtain ⟨events,path⟩ := prior.source_path
    exact interpret_payment native (runs_history valid (SourceEntryPrefix.runs_projection path)) ih

-- A fresh launch has a prelude phase and no pre-existing payment. A generic
-- arbitrary mid-history base is not silently treated as fresh startup.
theorem live_payment (live : CohortHostExecution.Live base)
    (fresh : paymentOf base.mode = none) :
    (target live.current.state.cohort).pendingPayment = expected live.current.state.inner := by
  apply history_payment (CohortMemoryProvenance.live_origin live) SharedHostCaptureReplay.initial_valid
  simpa [target,CohortHostStartup.anchor,CohortHostStartup.empty,write,expected,
    SourceEntryPrefix.State.start,JoinedState.start,SharedWireLifetime.History.start] using fresh.symm

theorem discharged_payment (live : CohortHostExecution.Live base)
    (fresh : paymentOf base.mode = none) (empty : live.current.state.cohort.pending = []) :
    live.current.state.cohort.memory.pendingPayment = expected live.current.state.inner := by
  simpa [target,empty] using live_payment live fresh

#print axioms fold_payment
#print axioms owner_before_unpaid
#print axioms query_frames
#print axioms local_frames
#print axioms source_after_unpaid
#print axioms owner_recipe_payment
#print axioms source_payment
#print axioms finish_payment
#print axioms interpret_payment
#print axioms history_payment
#print axioms live_payment
#print axioms discharged_payment
end CohortPaymentCorrespondence
