import LeaseCohortCorrespondence
import CohortRetiredProbe
open MirroreaProofFirst
namespace RetiredLeaseCorrespondence
open CohortCommitJournal
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- A post-retirement public probe changes only the physical guard. Its actual
-- memory and unfinished debt refer to the SAME retained source/owner history.
-- The target after pending stores is a logical target, not a claim that those
-- stores happened or that the retired endpoint may resume.
theorem probe_target (probe : CohortRetiredProbe.Certified base) :
    target probe.journal = target probe.origin.current.state.cohort := by
  have kept := probe.keeps
  simp only [target,kept.1,kept.2.1]

theorem probe_correspondence (probe : CohortRetiredProbe.Certified base)
    (fresh : base.mode = .prelude owed) :
    LeaseCohortCorrespondence.Correspondence probe.origin ∧
    target probe.journal = target probe.origin.current.state.cohort ∧
    probe.journal.retired = true :=
  ⟨LeaseCohortCorrespondence.live_correspondence probe.origin fresh,probe_target probe,probe.keeps.2.2⟩

theorem observed_discharged (probe : CohortRetiredProbe.Certified base)
    (fresh : base.mode = .prelude owed) (empty : probe.journal.pending = [])
    (accepted : CohortHostObservation.memoryEq probe.journal.memory observed = true) :
    observed.bootstrapped = true ∧
    observed.snapshot = CohortSnapshotCorrespondence.nativeSnapshot probe.origin.current.state.inner ∧
    CohortAdministrativeCorrespondence.Corresponds observed probe.origin.current.state.inner.joined.closed ∧
    (∀ envelope ∈ observed.produced, CohortProductionCorrespondence.ProducedAt probe.origin.current.state.inner envelope) ∧
    observed.pendingPayment = CohortPaymentCorrespondence.expected probe.origin.current.state.inner := by
  have kept := probe.keeps
  rw [kept.1] at accepted
  rw [kept.2.1] at empty
  exact CohortStateCorrespondence.observed_discharged probe.origin
    (by simp [fresh,CohortHostReceipt.paymentOf]) empty accepted

-- Observation transfer never discharges pending debt. In particular normal
-- equality with a current native snapshot is NOT asserted at an interrupted
-- source snapshot store; probe_target retains the precise delayed relation.
#print axioms probe_target
#print axioms probe_correspondence
#print axioms observed_discharged
end RetiredLeaseCorrespondence
