import MixedReferenceMutation
import MirroreaProofFirstReferenceResult

namespace MirroreaProofFirst.MixedReferenceResult
open ReferenceOwner ReferenceSelection MixedReferenceHistory

-- The argument-bearing invocation and argument-free reference permission are
-- checked at the SAME state. Neither permission is manufactured from the other.
-- A saved binding includes the epoch, lineage, selected guard and activation
-- frontier. Completion never reselects, refreshes, or retargets that binding.
abbrev Corresponds := ReferenceResult.Corresponds

def Current (m : MixedReferenceStore.Machine p a) (binding : Binding) (ticket : InvocationBoundary.Ticket) : Prop :=
  MixedReferenceMutation.lookup m binding.request.binding = some binding ∧
  MixedReferenceHolding.Live .separate binding.request binding.holding.guard binding.holding.activation m.history ∧
  MixedReferenceHolding.Saved .separate m.core.system binding.request binding.holding.guard ∧
  ∃ choice option, binding.selected = some choice ∧
    binding.request.chain.options[choice.index]? = some option ∧
    Corresponds binding choice option ticket ∧
    Continuous (atIndex binding.request choice.index) choice.guard
      (m.history.drop binding.activation ++ [MixedReferenceStore.frame m.core])

def check (m : MixedReferenceStore.Machine p a) (binding : Binding) (ticket : InvocationBoundary.Ticket) : Bool :=
  decide (MixedReferenceMutation.lookup m binding.request.binding = some binding) &&
  MixedReferenceMutation.holdingLive m binding &&
  match binding.selected with
  | none => false
  | some choice => match binding.request.chain.options[choice.index]? with
    | none => false
    | some option => decide (Corresponds binding choice option ticket) &&
        MixedReferenceMutation.continuouslyLive m binding choice

theorem check_exact (m : MixedReferenceStore.Machine p a) (binding : Binding) (ticket : InvocationBoundary.Ticket) :
    check m binding ticket = true ↔ Current m binding ticket := by
  cases chosen : binding.selected with
  | none => simp [check,Current,chosen]
  | some choice =>
      cases selected : binding.request.chain.options[choice.index]? with
      | none => simp [check,Current,chosen,selected]
      | some option =>
          simp [check,Current,chosen,selected,MixedReferenceMutation.continuouslyLive,usable,continuous_exact,
            MixedReferenceMutation.holdingLive,MixedReferenceHolding.live_exact,MixedReferenceHolding.check_exact,and_assoc]

theorem changed_binding_rejected (m : MixedReferenceStore.Machine p a) (binding : Binding)
    (ticket : InvocationBoundary.Ticket)
    (changed : MixedReferenceMutation.lookup m binding.request.binding ≠ some binding) : check m binding ticket = false := by
  simp [check,changed]

theorem lost_guard_rejected (m : MixedReferenceStore.Machine p a) (binding : Binding)
    (ticket : InvocationBoundary.Ticket) (choice : Choice) (selected : binding.selected = some choice)
    (lost : MixedReferenceMutation.continuouslyLive m binding choice = false) : check m binding ticket = false := by
  simp only [check,selected]
  cases binding.request.chain.options[choice.index]? <;> simp [lost]

theorem historical_invalidation_rejected (m : MixedReferenceStore.Machine p a) (binding : Binding)
    (ticket : InvocationBoundary.Ticket) (choice : Choice) (selected : binding.selected = some choice)
    (lost : continuousCheck (m.history.drop binding.activation) (atIndex binding.request choice.index) choice.guard = false) :
    check m binding ticket = false := by
  exact lost_guard_rejected _ _ _ _ selected (usable_no_resurrection _ _ _ _ lost)

theorem checked_current_permission (m : MixedReferenceStore.Machine p a) (binding : Binding)
    (ticket : InvocationBoundary.Ticket) (checked : check m binding ticket = true) :
    ∃ choice option, binding.selected = some choice ∧
      binding.request.chain.options[choice.index]? = some option ∧ Corresponds binding choice option ticket ∧
      MixedReferenceAccess.Saved m.core.system (m.core.system.controlPolicy 13)
        (atIndex binding.request choice.index) choice.guard := by
  obtain ⟨_,_,_,choice,option,selected,atOption,corresponds,history⟩ := (check_exact _ _ _).mp checked
  have live := (continuous_exact _ _ _).mpr history
  exact ⟨choice,option,selected,atOption,corresponds,usable_current _ _ _ _ live⟩

theorem checked_holding (m : MixedReferenceStore.Machine p a) (binding : Binding)
    (ticket : InvocationBoundary.Ticket) (checked : check m binding ticket = true) :
    MixedReferenceHolding.Live .separate binding.request binding.holding.guard binding.holding.activation m.history ∧
      MixedReferenceHolding.Saved .separate m.core.system binding.request binding.holding.guard := by
  have meaning := (check_exact _ _ _).mp checked
  exact ⟨meaning.2.1,meaning.2.2.1⟩

theorem lost_holding_rejected (m : MixedReferenceStore.Machine p a) (binding : Binding)
    (ticket : InvocationBoundary.Ticket) (lost : MixedReferenceMutation.holdingLive m binding = false) :
    check m binding ticket = false := by simp [check,lost]

#print axioms checked_holding
#print axioms lost_holding_rejected
#print axioms check_exact
#print axioms changed_binding_rejected
#print axioms lost_guard_rejected
#print axioms historical_invalidation_rejected
#print axioms checked_current_permission
end MirroreaProofFirst.MixedReferenceResult
