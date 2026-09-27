import MirroreaProofFirstSharedWireLifetime
import MirroreaProofFirstOwnerCommitJournal
namespace MirroreaProofFirst.SharedHostJoin
open SharedWireLifetime
variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}
  {base s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

-- Equality is derived from the actual known-step computation, not from matching
-- writer projections/digests. This binds the complete native state (including
-- endpoint fields deliberately erased by OwnerCommitJournal.project).
theorem known_native (checked : known s request bytes = some result) :
    native result.val = SharedNativeStep.execute assigned scope bootstrap capacity (native s) (event s request) := by
  unfold known at checked
  split at checked
  · cases checked
  · rename_i next prepared
    split at checked
    · cases Option.some.inj checked
      exact next_physics ⟨s,request,next⟩
    · cases checked

theorem history_native {history after : History base}
    (checked : History.knownStep history request bytes = some after) :
    native after.current = SharedNativeStep.execute assigned scope bootstrap capacity
      (native history.current) (event history.current request) := by
  unfold History.knownStep at checked
  cases got : known history.current request bytes with
  | none => simp [got] at checked
  | some result =>
    simp only [got] at checked
    cases Option.some.inj checked
    exact known_native got

theorem history_owner {history after : History base}
    (checked : History.knownStep history (.owner owner command headPayment) bytes = some after) :
    (native after.current).owners owner =
      (OwnerEndpointBudget.transition ⟨assigned.realm,owner⟩ scope (capacity owner)
        ((native history.current).owners owner) command).1 := by
  have whole := history_native checked
  simpa [SharedNativeStep.execute,event,PublicOwnerBoundary.put] using congrArg (fun s => s.owners owner) whole

#print axioms known_native
#print axioms history_native
#print axioms history_owner
end MirroreaProofFirst.SharedHostJoin
