import CohortMemoryProvenance
open MirroreaProofFirst SharedHostCaptureReplay
namespace CohortBootstrappedCorrespondence
open CohortCommitJournal
set_option maxHeartbeats 1600000
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base : SharedWireLifetime.Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

theorem write_preserves (present : memory.bootstrapped = true) (store : Store p a) :
    (write memory store).bootstrapped = true := by cases store <;> simp_all [write]

theorem fold_preserves (present : memory.bootstrapped = true) (stores : List (Store p a)) :
    (stores.foldl write memory).bootstrapped = true := by
  induction stores generalizing memory with
  | nil => exact present
  | cons store rest ih => exact ih (write_preserves present store)

theorem receipts_preserve (present : memory.bootstrapped = true) (receipts : List (Receipt p a)) :
    (CohortHostDebt.receiptResult memory receipts).bootstrapped = true := by
  induction receipts generalizing memory with
  | nil => exact present
  | cons receipt rest ih => exact ih (fold_preserves present _)

theorem history_preserves
    (path : CohortMemoryProvenance.NativeHistory first initial last result)
    (present : initial.bootstrapped = true) : result.bootstrapped = true := by
  induction path with
  | nil => exact present
  | step prior native ih => exact receipts_preserve ih _

-- The bit is rooted in the real checked startup handoff retained by Live.
-- It says the bootstrap write completed, not that arbitrary raw IO is trusted.
theorem live_target (live : CohortHostExecution.Live base) :
    (target live.current.state.cohort).bootstrapped = true :=
  history_preserves (CohortMemoryProvenance.live_origin live) rfl

theorem step_memory {before after : CohortHostPrefix.State base}
    (step : CohortHostPrefix.Step before event after) (present : before.cohort.memory.bootstrapped = true) :
    after.cohort.memory.bootstrapped = true := by
  rcases before with ⟨inner,journal⟩
  cases step with
  | inner native localStores =>
    cases localStores with
    | plain => exact present
    | notify => exact present
    | snapshot => exact present
  | enter noInner localStores | commit noInner localStores | close localStores native
  | retire native localStores | release localStores =>
    cases localStores <;> first | exact present | exact write_preserves present _

theorem memory_preserves {first last : CohortHostPrefix.State base}
    (path : CohortHostPrefix.Runs first last) (anchored : first.cohort.memory.bootstrapped = true) :
    last.cohort.memory.bootstrapped = true := by
  induction path with
  | nil => exact anchored
  | step prior step ih => exact step_memory step ih

theorem live_memory (live : CohortHostExecution.Live base) :
    live.current.state.cohort.memory.bootstrapped = true :=
  memory_preserves live.current.path rfl

#print axioms write_preserves
#print axioms fold_preserves
#print axioms receipts_preserve
#print axioms history_preserves
#print axioms live_target
#print axioms live_memory
end CohortBootstrappedCorrespondence
