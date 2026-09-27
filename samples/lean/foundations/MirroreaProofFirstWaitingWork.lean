import MirroreaProofFirstPublicJointWork

namespace MirroreaProofFirst.WaitingWork
open PublicationCapacity PublicationLifecycle PublicationOwnerBudget

-- A settled waiting source has no predicted work in its old suffix. Entry
-- creates a release obligation. This lemma uses the unchanged actual driver;
-- it does not replace the driver with a separately certified scheduled one.
theorem empty_release_schedule
    (empty : old.suffix = [])
    (release : releasePlan entered = [PublicationInput.Command.finish target])
    (held : entered.publication.held target ≠ none)
    (path : BoundSequence realm scope entered [.finish target] finished)
    (quiet : Quiet finished) :
    schedule realm scope old (.step (.enter target)) entered = some [.finish target] := by
  have fit := boundFits_exact.mpr (boundSequence_first path)
  have active : quietCheck entered = false := by
    apply Bool.eq_false_iff.mpr
    intro yes
    exact held ((quiet_exact.mp yes).1.2.2 target)
  have checked := checkBoundPath_exact.mpr path
  simp [schedule,scheduledTail,empty,residualPlan,eraseCommand,checkBoundPath,fit,
    active,releasedPlan,release,checked,quiet_exact.mpr quiet]

-- Independent declarative entry/release path, two source credits and three
-- actual target-owner credits suffice. No successful funded entry is assumed.
-- The resulting finish-head driver feeds PublicJointWork.funded_entered_work.
theorem funded_waiting_entry {p a : Nat} {assigned : SourceInput.Assignment p a}
    {old : PublicationCapacityDriver.State p a} {vector : Vector (Fin 513) p}
    {scope remaining : Nat} {bootstrap : SourceInput.Bootstrap a}
    {source entered finished : PublicationInput.State p a} {target : Fin p}
    (valid : Invariant assigned scope bootstrap old)
    (present : old.source = some source) (empty : old.suffix = [])
    (quota : old.remaining = remaining+2)
    (enter : PublicationInput.execute scope source (.enter target) = some entered)
    (release : releasePlan entered = [.finish target])
    (held : entered.publication.held target ≠ none)
    (path : BoundSequence assigned.realm scope entered [.finish target] finished)
    (quiet : Quiet finished)
    (funded : 3 ≤ (vector[target.val]).val) :
    let next : PublicationCapacityDriver.State p a := ⟨some entered,remaining+1,[.finish target]⟩
    SourceFundingInput.execute assigned scope bootstrap old (vector,.step (.enter target)) = (next,.accepted) ∧
    Invariant assigned scope bootstrap next ∧ Covered next.suffix (fun i => (vector[i.val]).val) := by
  let next : PublicationCapacityDriver.State p a := ⟨some entered,remaining+1,[.finish target]⟩
  have scheduled := empty_release_schedule empty release held path quiet
  have semantic : PublicationInput.transition assigned scope bootstrap old.source (.step (.enter target)) = (some entered,true) := by
    simp [PublicationInput.transition,present,enter]
  have accepted : PublicationLifecycle.transition assigned scope bootstrap old (.step (.enter target)) = (next,.accepted) :=
    accepted_of_completion (by omega) semantic scheduled (by simp) ⟨finished,path,quiet⟩
  have covered : Covered next.suffix (fun i => (vector[i.val]).val) := by
    intro i
    change demand [.finish target] i ≤ (vector[i.val]).val
    by_cases same : i=target
    · subst i; simpa [demand,commandCost] using funded
    · simp [demand,commandCost,same]
  have actual : SourceFundingInput.execute assigned scope bootstrap old (vector,.step (.enter target)) = (next,.accepted) := by
    apply SourceFundingInput.accepted_exact.mpr
    refine ⟨?_,?_⟩
    · simpa [transitionFast_exact valid] using accepted
    · simpa [SourceFundingInput.initialDebit,present,SourceFundingInput.credits] using covered
  refine ⟨actual,?_,covered⟩
  have preserved := SourceFundingInput.preserves valid (value:=(vector,.step (.enter target)))
  simpa [actual] using preserved

-- Same semantic entry with only the last source credit is refused because
-- there is no source credit left to discharge its newly created finish.
theorem last_source_credit_refused
    (valid : Invariant assigned scope bootstrap old)
    (present : old.source = some source) (empty : old.suffix = [])
    (quota : old.remaining = 1)
    (enter : PublicationInput.execute scope source (.enter target) = some entered)
    (release : releasePlan entered = [.finish target])
    (held : entered.publication.held target ≠ none)
    (path : BoundSequence assigned.realm scope entered [.finish target] finished)
    (quiet : Quiet finished) :
    SourceFundingInput.execute assigned scope bootstrap old (vector,.step (.enter target)) = (old,.profileRefused) := by
  have scheduled := empty_release_schedule empty release held path quiet
  have semantic : PublicationInput.transition assigned scope bootstrap old.source (.step (.enter target)) = (some entered,true) := by
    simp [PublicationInput.transition,present,enter]
  have refused : PublicationLifecycle.transition assigned scope bootstrap old (.step (.enter target)) = (old,.profileRefused) := by
    simp [PublicationLifecycle.transition,quota,semantic,scheduled,passes]
  simp [SourceFundingInput.execute,transitionFast_exact valid,refused]

#print axioms empty_release_schedule
#print axioms funded_waiting_entry
#print axioms last_source_credit_refused
end MirroreaProofFirst.WaitingWork
