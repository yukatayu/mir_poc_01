import MirroreaProofFirstOwnerImageMonitor
import MirroreaProofFirstSourceFundingCheckedWork

namespace MirroreaProofFirst.SourceOwnerGate

-- Known native source refusal remains eligible for the source's own framed
-- diagnostic. Only an open logical gate requires a physically usable owner.
def check (source : PublicationInput.State p a) (endpoint : Fin p)
    (known : Option (OwnerImageMonitor.Current p a)) (stoppedAt : Nat) : Bool :=
  if source.publication.barrier.installed endpoint = source.publication.barrier.fence endpoint then
    OwnerImageMonitor.usableCheck known stoppedAt source.publication.barrier.published
      (CustodyPublication.image source.publication.current)
  else true

def Allowed (source : PublicationInput.State p a) (endpoint : Fin p)
    (owner : OwnerEndpointBudget.State p a) : Prop :=
  source.publication.barrier.installed endpoint = source.publication.barrier.fence endpoint →
    OwnerImageMonitor.Usable owner source.publication.barrier.published
      (CustodyPublication.image source.publication.current)

theorem check_exact
    (path : OwnerImageMonitor.JointRuns assigned scopeId capacity budget owner known stoppedAt) :
    check source endpoint known stoppedAt = true ↔ Allowed source endpoint owner := by
  unfold check Allowed
  split
  · rename_i equal
    simp only [equal,true_implies]
    exact OwnerImageMonitor.usable_check_exact path
  · rename_i unequal
    simp [unequal]

theorem entered_logical_gate
    (accepted : PublicationInput.execute scopeId source (.enter endpoint) = some next) :
    source.publication.barrier.installed endpoint = source.publication.barrier.fence endpoint := by
  obtain ⟨action,selected,ran,_⟩ := PublicationInput.executed_action accepted
  have actionAt : action = .enter endpoint := by
    simp only [PublicationInput.select] at selected
    split at selected
    · cases selected
    · cases waiting : source.publication.current.privateState.session.state.source.waiting with
      | none => simp [waiting] at selected
      | some saved =>
        simp only [waiting,Option.bind_eq_bind,Option.bind_some] at selected
        split at selected
        · exact (Option.some.inj selected).symm
        · cases selected
  subst action
  unfold PublicationImage.execute at ran
  split at ran
  · rename_i allowed
    have checks : Publication.check source.publication.barrier (.use endpoint) = true ∧
        (source.publication.held endpoint).isNone = true := by
      simpa only [PublicationImage.check,Bool.and_eq_true] using allowed
    exact (Publication.check_exact _ _).mp checks.1
  · cases ran

theorem entered_actual_owner
    (path : OwnerImageMonitor.JointRuns assigned scopeId capacity budget owner known stoppedAt)
    (checked : check source endpoint known stoppedAt = true)
    (accepted : PublicationInput.execute scopeId source (.enter endpoint) = some next) :
    OwnerImageMonitor.Usable owner source.publication.barrier.published
      (CustodyPublication.image source.publication.current) :=
  (check_exact path).mp checked (entered_logical_gate accepted)

-- The preflight does not suppress known source refusal diagnostics. This is
-- independent of the actual owner; no work is dispatched on the refused branch.
theorem closed_source_passes (closed : source.publication.barrier.installed endpoint ≠
    source.publication.barrier.fence endpoint) : check source endpoint known stoppedAt = true := by
  simp [check,closed]

theorem usable_source_passes
    (path : OwnerImageMonitor.JointRuns assigned scopeId capacity budget owner known stoppedAt)
    (usable : OwnerImageMonitor.Usable owner source.publication.barrier.published
      (CustodyPublication.image source.publication.current)) : check source endpoint known stoppedAt = true :=
  (check_exact path).mpr (fun _ => usable)

-- The actual metered source worker may also reject for lifecycle/funding.
-- If it accepted entry, its original publication transition really ran.
theorem funded_entered_actual
    (path : OwnerImageMonitor.JointRuns ownerAssigned scopeId capacity budget owner known stoppedAt)
    (valid : PublicationLifecycle.Invariant assigned scopeId seed old)
    (present : old.source = some source)
    (checked : check source endpoint known stoppedAt = true)
    (accepted : SourceFundingInput.execute assigned scopeId seed old (credits,.step (.enter endpoint)) =
      (next,.accepted)) :
    OwnerImageMonitor.Usable owner source.publication.barrier.published
      (CustodyPublication.image source.publication.current) := by
  have native := (SourceFundingInput.accepted_exact.mp accepted).1
  rw [PublicationLifecycle.transitionFast_exact valid] at native
  have sem := (PublicationLifecycle.accepted_refines assigned scopeId seed old
    (.step (.enter endpoint)) (by rw [native])).1
  rw [native,present] at sem
  cases step : PublicationInput.execute scopeId source (.enter endpoint) with
  | none => simp [PublicationInput.transition,step] at sem
  | some value => exact entered_actual_owner path checked step

#print axioms funded_entered_actual
#print axioms check_exact
#print axioms entered_logical_gate
#print axioms entered_actual_owner
#print axioms closed_source_passes
#print axioms usable_source_passes
end MirroreaProofFirst.SourceOwnerGate
