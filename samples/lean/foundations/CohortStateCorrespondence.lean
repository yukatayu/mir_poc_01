import CohortSnapshotCorrespondence
import CohortAdministrativeCorrespondence
import CohortPaymentCorrespondence
import CohortProductionCorrespondence
import CohortBootstrappedCorrespondence
import OwnerCurrentCorrespondence
open MirroreaProofFirst SharedHostCaptureReplay
namespace CohortStateCorrespondence
open CohortCommitJournal
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- All fields refer to this ONE current cursor. The delayed receipt target is
-- not current physical memory while stores remain; unknown IO is last-known.
-- Historical facts confer no authority/currentness, and produced is not sent.
structure Correspondence (live : CohortHostExecution.Live base) : Prop where
  receiptHistory : CohortMemoryProvenance.NativeHistory (.start base)
    (target (CohortHostStartup.anchor base live.callId).cohort)
    live.current.state.inner (target live.current.state.cohort)
  bootstrapped : live.current.state.cohort.memory.bootstrapped = true
  snapshot : (target live.current.state.cohort).snapshot =
    CohortSnapshotCorrespondence.nativeSnapshot live.current.state.inner
  administrative : CohortAdministrativeCorrespondence.Corresponds
    (target live.current.state.cohort) live.current.state.inner.joined.closed
  production : ∀ envelope ∈ (target live.current.state.cohort).produced,
    CohortProductionCorrespondence.ProducedAt live.current.state.inner envelope
  payment : (target live.current.state.cohort).pendingPayment =
    CohortPaymentCorrespondence.expected live.current.state.inner
  owners : OwnerCurrentCorrespondence.Aligned live.current.state.inner
  lifecycle : WellFormed live.current.state.cohort
  history : HistoryInvariant live.current.state.inner.joined
  entry : SourceEntryPrefix.Invariant live.current.state.inner
  funding : CohortPhase.Invariant assigned scope bootstrap
    (some (SharedFundedDriver.funding live.current.state.inner.joined.history.current))

theorem live_correspondence (live : CohortHostExecution.Live base)
    (fresh : CohortHostReceipt.paymentOf base.mode = none) : Correspondence live :=
  ⟨CohortMemoryProvenance.live_origin live,
    CohortBootstrappedCorrespondence.live_memory live,
    CohortSnapshotCorrespondence.live_snapshot live,
    CohortAdministrativeCorrespondence.live_corresponds live,
    CohortProductionCorrespondence.live_origin live,
    CohortPaymentCorrespondence.live_payment live fresh,
    OwnerCurrentCorrespondence.live_aligned live,
    live.valid.2.2.1,live.source.joined_valid,live.source.valid,
    SharedFundedDriver.invariant _⟩

-- The private captured-value gate compares sets extensionally. Transfer the exact
-- field meaning through that independently proved checker, never by assuming
-- observed memory already satisfies the requested correspondence.
theorem observed_discharged (live : CohortHostExecution.Live base)
    (fresh : CohortHostReceipt.paymentOf base.mode = none)
    (empty : live.current.state.cohort.pending = [])
    (accepted : CohortHostObservation.memoryEq live.current.state.cohort.memory observed = true) :
    observed.bootstrapped = true ∧
    observed.snapshot = CohortSnapshotCorrespondence.nativeSnapshot live.current.state.inner ∧
    CohortAdministrativeCorrespondence.Corresponds observed live.current.state.inner.joined.closed ∧
    (∀ envelope ∈ observed.produced, CohortProductionCorrespondence.ProducedAt live.current.state.inner envelope) ∧
    observed.pendingPayment = CohortPaymentCorrespondence.expected live.current.state.inner := by
  have relation := live_correspondence live fresh
  have matched :=  CohortHostObservation.memoryEq_exact.mp accepted
  rcases matched with ⟨boot,snapshot,initialized,freezes,installs,produced,payment⟩
  refine ⟨boot.symm.trans relation.bootstrapped,
    snapshot.symm.trans (CohortSnapshotCorrespondence.discharged_snapshot live empty),?_,?_,
    payment.symm.trans (CohortPaymentCorrespondence.discharged_payment live fresh empty)⟩
  · intro fact
    have current := CohortAdministrativeCorrespondence.discharged_corresponds live empty fact
    cases fact with
    | initialized owner => exact (initialized.membership owner).symm.trans current
    | frozen owner revision => exact (freezes.membership (owner,revision)).symm.trans current
    | installed owner revision => exact (installs.membership (owner,revision)).symm.trans current
  · intro envelope member
    exact CohortProductionCorrespondence.discharged_origin live empty envelope (produced.2 member)

-- Every ordinary local transition preserves the theorem without rechecking the
-- target assertion as a guard. This includes entry, commit, close and retire.
theorem advance_correspondence (before after : CohortHostExecution.Live base)
    (fresh : CohortHostReceipt.paymentOf base.mode = none)
    (_checked : before.advance event = some after) : Correspondence after :=
  live_correspondence after fresh

#print axioms live_correspondence
#print axioms observed_discharged
#print axioms advance_correspondence
end CohortStateCorrespondence
