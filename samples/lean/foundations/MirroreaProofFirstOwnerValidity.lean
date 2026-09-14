import MirroreaProofFirstOwnerImage

namespace MirroreaProofFirst.OwnerValidity
open InstanceState InstancePrograms GraphValidation

-- Validate the declarative configuration invariant on arbitrary typed input.
-- Shape, validity, authentic current install and permission to disclose remain
-- distinct gates. Version and parent DAGs remain separate from live support.
def checkState (s : State d p n) : Bool :=
  (List.finRange d).all (fun k => InstancePrograms.check (s.definitions k)) &&
  checkAcyclic (fun a b => decide (s.predecessors a = some b)) &&
  (List.finRange n).all (fun k => refinementCheck (s.instances k).interface
    (s.definitions (s.instances k).definition).contract) &&
  (List.finRange n).all (fun k => decide ((s.instances k).placements ≠ [])) &&
  checkAcyclic (fun a b => decide ((s.instances a).parent = some b))

theorem checkState_exact (s : State d p n) : checkState s = true ↔ Valid s := by
  simp only [checkState, Bool.and_eq_true, List.all_eq_true, List.mem_finRange,
    forall_const, InstancePrograms.check_exact, refinement_exact,
    decide_eq_true_eq, checkAcyclic_exact]
  exact ⟨fun ⟨⟨⟨⟨definitions,versions⟩,interfaces⟩,placed⟩,parents⟩ =>
    ⟨definitions,versions,interfaces,placed,parents⟩,
    fun valid => ⟨⟨⟨⟨valid.definitions,valid.versions⟩,valid.interfaces⟩,valid.placed⟩,valid.parents⟩⟩

def check (image : OwnerImage.Image p a) : Bool :=
  checkState (OwnerImage.restore image).configuration.state

theorem check_exact (image : OwnerImage.Image p a) : check image = true ↔
    CompositionCore.Invariant (OwnerImage.restore image).configuration := checkState_exact _

theorem capture_accepted (input : OwnerProjection.Input p a)
    (valid : CompositionCore.Invariant input.configuration) :
    check (OwnerImage.capture input) = true := by
  apply check_exact _ |>.mpr
  simpa only [CompositionCore.Invariant,OwnerImage.restore,OwnerImage.capture,
    OwnerImage.state_roundtrip] using valid

theorem checked_admitted_completes (image : OwnerImage.Image p a)
    (assigned : Fin p) (ticket : InvocationBoundary.Ticket)
    (checked : check image = true)
    (allowed : OwnerProjection.Allowed (OwnerImage.restore image) assigned ticket) :
    ∃ value, OwnerProjection.serve (OwnerImage.restore image) assigned ticket = some value ∧
      Output ticket.definition.contract value :=
  OwnerProjection.admitted_serve_exists _ _ _ ((check_exact image).mp checked) allowed

theorem protected_source_capture (s : ReferenceSource.State p a)
    (valid : ReferenceExecution.Invariant s.machine) :
    check (OwnerImage.capture (OwnerProjection.project s)) = true :=
  capture_accepted _ valid.1.1.1.1.1

#print axioms checkState_exact
#print axioms check_exact
#print axioms capture_accepted
#print axioms checked_admitted_completes
#print axioms protected_source_capture
end MirroreaProofFirst.OwnerValidity
