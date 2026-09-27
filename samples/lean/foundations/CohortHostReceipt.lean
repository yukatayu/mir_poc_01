import SourceEntryPrefix
import MirroreaProofFirstCohortCommitJournal
open MirroreaProofFirst SharedHostCaptureReplay
namespace CohortHostReceipt
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Extract the funding transition from the SAME successful known-wire step.
-- An independently supplied payment token or equal projection is insufficient.
theorem known_funding
    {before : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}
    {request : SharedWireLifetime.Request p a} {bytes : List UInt8}
    {result : SharedWireLifetime.KnownResult before request bytes}
    (checked : SharedWireLifetime.known before request bytes = some result) :
    CohortPhase.Step assigned scope bootstrap capacity
      (some (SharedFundedDriver.funding before))
      (SharedFundedDriver.fundingEvent (SharedWireLifetime.event before request))
      (some (SharedFundedDriver.funding result.val)) := by
  unfold SharedWireLifetime.known at checked
  split at checked
  · cases checked
  · rename_i next prepared
    split at checked
    · cases Option.some.inj checked; exact next.step
    · cases checked

theorem history_funding {before after : SharedWireLifetime.History base}
    (checked : SharedWireLifetime.History.knownStep before request bytes = some after) :
    CohortPhase.Step assigned scope bootstrap capacity
      (some (SharedFundedDriver.funding before.current))
      (SharedFundedDriver.fundingEvent (SharedWireLifetime.event before.current request))
      (some (SharedFundedDriver.funding after.current)) := by
  unfold SharedWireLifetime.History.knownStep at checked
  cases got : SharedWireLifetime.known before.current request bytes with
  | none => simp [got] at checked
  | some result =>
    simp only [got] at checked
    cases Option.some.inj checked
    exact known_funding got

def paymentOf : CohortPhase.Mode p → Option (PaidHeadPhase.Pending p)
  | .paid pending => some pending
  | _ => none

theorem paymentOf_exact : paymentOf mode = some pending ↔ mode = .paid pending := by
  cases mode <;> simp [paymentOf]

-- This constructor consumes a bound actual writer interval, not caller-made
-- receipt data. Its use at the owning caller's confirmation is an outer-product
-- obligation. No administrative receipt may be armed at raw IO return alone.
def ownerReceipt (bound : BoundWriter base) : CohortCommitJournal.Receipt p a :=
  .ownerReturned bound.writer.owner bound.writer.command bound.writer.reply
    (paymentOf bound.later.current.mode)

theorem owner_payment_before
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before)
      (.owner owner command) (some after))
    (paid : after.mode = .paid pending) : before.mode = .ordinary := by
  cases step <;> simp_all [CohortPhase.ownerResult]

-- The chosen payment really arises from this exact native writer operation,
-- certified source head, source ordinal and successful reply. This closes the
-- payment-metadata origin assumption of the local cohort journal, while
-- authority and truthfulness of the physical capture remain separate premises.
theorem owner_payment_origin (bound : BoundWriter base)
    (paid : paymentOf bound.later.current.mode = some pending) :
    ∃ source rest next,
      (SharedWireLifetime.native bound.earlier.current).driver.source = some source ∧
      (SharedWireLifetime.native bound.earlier.current).driver.suffix =
        PaidHeadPhase.command pending :: rest ∧
      pending.sourceOrdinal = (SharedWireLifetime.native bound.earlier.current).ordinal ∧
      bound.writer.owner = pending.target ∧
      bound.writer.command = CohortPhase.paymentRequest pending source ∧
      bound.writer.reply = CohortPhase.paymentReply pending ∧
      OwnerEndpointBudget.transition ⟨assigned.realm,pending.target⟩ scope (capacity pending.target)
        ((SharedWireLifetime.native bound.earlier.current).owners pending.target)
        (CohortPhase.paymentRequest pending source) = (next,CohortPhase.paymentReply pending) := by
  have step := history_funding bound.known
  change CohortPhase.Step assigned scope bootstrap capacity
    (some (SharedFundedDriver.funding bound.earlier.current))
    (.owner bound.writer.owner bound.writer.command)
    (some (SharedFundedDriver.funding bound.later.current)) at step
  have phase := paymentOf_exact.mp paid
  have ordinary := owner_payment_before step phase
  obtain ⟨source,rest,next,present,head,ordinal,ran,after,label⟩ :=
    CohortPhase.paid_origin step ordinary phase
  have same := CohortPhase.Event.owner.inj label
  have reply := bound.nativeReply
  rw [same.1,same.2] at reply
  have response := congrArg Prod.snd (ran.symm.trans reply)
  exact ⟨source,rest,next,present,head,ordinal,same.1,same.2,response.symm,ran⟩

-- Clearing a retained payment requires the corresponding actual successful
-- source transition at the retained ordinal; a query or other event cannot
-- substitute for notification. This follows from the operational rules.
theorem notification_origin
    (step : CohortPhase.Step assigned scope bootstrap capacity (some before) event (some after))
    (paid : before.mode = .paid pending) (external : event ≠ .local) :
    ∃ vector next,
      event = .source (vector,.step (PaidHeadPhase.command pending)) ∧
      before.ordinal = pending.sourceOrdinal ∧
      SourceFundingInput.execute assigned scope bootstrap before.driver
        (vector,.step (PaidHeadPhase.command pending)) = (next,.accepted) ∧
      after = CohortPhase.sourceResult before next .ordinary := by
  cases step <;> simp_all [CohortPhase.QueryAllowed,PaidHeadPhase.permits_exact]

def finishedEnvelope : SharedJointDriver.Running assigned scope bootstrap capacity ownerBudget seed →
    Option OwnerReceipt.Envelope
  | .closed _ => none
  | .work _ _ _ _ checked => match checked.phase with
    | .finished envelope => some envelope
    | _ => none

def FinishedProduction
    (running : SharedJointDriver.Running assigned scope bootstrap capacity ownerBudget seed)
    (envelope : OwnerReceipt.Envelope) : Prop :=
  match running with
  | .closed _ => False
  | .work _ target _ events checked =>
    checked.phase = .finished envelope ∧
    (∃ before next : OwnerEndpointBudget.State p a, OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target)
      before (.owner .compute) = (next,.inr envelope)) ∧
    ∃ earlier ordinal vector, events = earlier ++ [.source ordinal (vector,.step (.finish target))]

theorem finishedEnvelope_origin
    {running : SharedJointDriver.Running assigned scope bootstrap capacity ownerBudget seed}
    (selected : finishedEnvelope running = some envelope) : FinishedProduction running envelope := by
  cases running with
  | closed => cases selected
  | work origin target ticket events checked =>
    rcases checked with ⟨phase,state,path⟩
    cases phase <;> simp only [finishedEnvelope] at selected <;> try contradiction
    cases Option.some.inj selected
    refine ⟨rfl,?_,WorkOccurrence.finished_has_occurrence path⟩
    cases path with
    | finish prior bound ran present =>
      cases prior with
      | compute prior computed ticketBound scopeBound revisionBound =>
        exact ⟨_,_,computed⟩

-- Recipes use the complete typed snapshot of the same successor. Query replies
-- do not store a source snapshot. A completed work's produced envelope remains
-- a separate later store, following the snapshot; the outer product must retain
-- this debt and forbid normal return before it is discharged.
def sourceReceipts (before after : SharedWireLifetime.History base)
    (input : SourceFundingQuery.CheckedInput p a) : List (CohortCommitJournal.Receipt p a) :=
  match input with
  | .inr (.inr _) =>
    [.sourceReply ((SharedWireLifetime.native after.current).driver.source.map SourcePublicationWorker.project)
      (paymentOf before.current.mode).isSome] ++
    ((finishedEnvelope after.current.joint).toList.map CohortCommitJournal.Receipt.workFinished)
  | _ => []

theorem query_no_snapshot (before after : SharedWireLifetime.History base) :
    sourceReceipts before after (.inl head) = [] ∧
    sourceReceipts before after (.inr (.inl target)) = [] := ⟨rfl,rfl⟩

theorem source_notification_origin {before after : SharedWireLifetime.History base}
    (checked : SharedWireLifetime.History.knownStep before (.source input) bytes = some after)
    (paid : before.current.mode = .paid pending) :
    ∃ vector next,
      SharedFundedDriver.fundingEvent (SharedWireLifetime.event before.current (.source input)) =
        .source (vector,.step (PaidHeadPhase.command pending)) ∧
      (SharedWireLifetime.native before.current).ordinal = pending.sourceOrdinal ∧
      SourceFundingInput.execute assigned scope bootstrap
        (SharedWireLifetime.native before.current).driver
        (vector,.step (PaidHeadPhase.command pending)) = (next,.accepted) ∧
      SharedFundedDriver.funding after.current =
        CohortPhase.sourceResult (SharedFundedDriver.funding before.current) next .ordinary :=
  notification_origin (history_funding checked) paid (by
    rcases input with head | (target | request) <;>
      simp [SharedWireLifetime.event,SharedJointDriver.inputEvent,SharedFundedDriver.fundingEvent])

#print axioms known_funding
#print axioms history_funding
#print axioms paymentOf_exact
#print axioms owner_payment_before
#print axioms owner_payment_origin
#print axioms notification_origin
#print axioms source_notification_origin
#print axioms query_no_snapshot
#print axioms finishedEnvelope_origin
end CohortHostReceipt
