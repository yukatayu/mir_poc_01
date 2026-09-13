import MirroreaProofFirstReferenceMutation

namespace MirroreaProofFirst.ReferenceResult
open ReferenceOwner ReferenceSelection ReferenceAccessHistory

-- The argument-bearing invocation and argument-free reference permission are
-- checked at the SAME state. Neither permission is manufactured from the other.
-- A saved binding includes the epoch, lineage, selected guard and activation
-- frontier. Completion never reselects, refreshes, or retargets that binding.
def Corresponds (binding : Binding) (choice : Choice) (option : FallbackStatic.OptionDecl)
    (ticket : InvocationBoundary.Ticket) : Prop :=
  ticket.member = binding.request.member ∧ ticket.place = binding.request.place ∧
  ticket.principal = binding.request.principal ∧ ticket.key = option.target ∧
  ticket.realm = choice.guard.context.targetCapture.realm ∧
  ticket.memberIdentity = choice.guard.context.memberIdentity ∧
  ticket.definition = choice.guard.context.definition ∧
  ticket.arithmeticProfile = choice.guard.context.arithmeticProfile ∧
  ticket.contractTheoryVersion = choice.guard.context.contractTheoryVersion

instance (binding : Binding) (choice : Choice) (option : FallbackStatic.OptionDecl)
    (ticket : InvocationBoundary.Ticket) : Decidable (Corresponds binding choice option ticket) :=
  inferInstanceAs (Decidable (_ ∧ _ ∧ _ ∧ _ ∧ _ ∧ _ ∧ _ ∧ _ ∧ _))

def Current (m : ReferenceStore.Machine p a) (binding : Binding) (ticket : InvocationBoundary.Ticket) : Prop :=
  ReferenceMutation.lookup m binding.request.binding = some binding ∧
  ReferenceHolding.Live .separate binding.request binding.holding.guard binding.holding.activation m.history ∧
  ReferenceHolding.Saved .separate m.core.system binding.request binding.holding.guard ∧
  ∃ choice option, binding.selected = some choice ∧
    binding.request.chain.options[choice.index]? = some option ∧
    Corresponds binding choice option ticket ∧
    Continuous (atIndex binding.request choice.index) choice.guard
      (m.history.drop binding.activation ++ [ReferenceStore.frame m.core])

def check (m : ReferenceStore.Machine p a) (binding : Binding) (ticket : InvocationBoundary.Ticket) : Bool :=
  decide (ReferenceMutation.lookup m binding.request.binding = some binding) &&
  ReferenceMutation.holdingLive m binding &&
  match binding.selected with
  | none => false
  | some choice => match binding.request.chain.options[choice.index]? with
    | none => false
    | some option => decide (Corresponds binding choice option ticket) &&
        ReferenceMutation.continuouslyLive m binding choice

theorem check_exact (m : ReferenceStore.Machine p a) (binding : Binding) (ticket : InvocationBoundary.Ticket) :
    check m binding ticket = true ↔ Current m binding ticket := by
  cases chosen : binding.selected with
  | none => simp [check,Current,chosen]
  | some choice =>
      cases selected : binding.request.chain.options[choice.index]? with
      | none => simp [check,Current,chosen,selected]
      | some option =>
          simp [check,Current,chosen,selected,ReferenceMutation.continuouslyLive,usable,continuous_exact,
            ReferenceMutation.holdingLive,ReferenceHolding.live_exact,ReferenceHolding.check_exact,and_assoc]

theorem changed_binding_rejected (m : ReferenceStore.Machine p a) (binding : Binding)
    (ticket : InvocationBoundary.Ticket)
    (changed : ReferenceMutation.lookup m binding.request.binding ≠ some binding) : check m binding ticket = false := by
  simp [check,changed]

theorem lost_guard_rejected (m : ReferenceStore.Machine p a) (binding : Binding)
    (ticket : InvocationBoundary.Ticket) (choice : Choice) (selected : binding.selected = some choice)
    (lost : ReferenceMutation.continuouslyLive m binding choice = false) : check m binding ticket = false := by
  simp only [check,selected]
  cases binding.request.chain.options[choice.index]? <;> simp [lost]

theorem historical_invalidation_rejected (m : ReferenceStore.Machine p a) (binding : Binding)
    (ticket : InvocationBoundary.Ticket) (choice : Choice) (selected : binding.selected = some choice)
    (lost : continuousCheck (m.history.drop binding.activation) (atIndex binding.request choice.index) choice.guard = false) :
    check m binding ticket = false := by
  exact lost_guard_rejected _ _ _ _ selected (usable_no_resurrection _ _ _ _ lost)

theorem checked_current_permission (m : ReferenceStore.Machine p a) (binding : Binding)
    (ticket : InvocationBoundary.Ticket) (checked : check m binding ticket = true) :
    ∃ choice option, binding.selected = some choice ∧
      binding.request.chain.options[choice.index]? = some option ∧ Corresponds binding choice option ticket ∧
      ReferenceAccess.Saved m.core.system (m.core.system.controlPolicy 13)
        (atIndex binding.request choice.index) choice.guard := by
  obtain ⟨_,_,_,choice,option,selected,atOption,corresponds,history⟩ := (check_exact _ _ _).mp checked
  have live := (continuous_exact _ _ _).mpr history
  exact ⟨choice,option,selected,atOption,corresponds,usable_current _ _ _ _ live⟩

theorem checked_holding (m : ReferenceStore.Machine p a) (binding : Binding)
    (ticket : InvocationBoundary.Ticket) (checked : check m binding ticket = true) :
    ReferenceHolding.Live .separate binding.request binding.holding.guard binding.holding.activation m.history ∧
      ReferenceHolding.Saved .separate m.core.system binding.request binding.holding.guard := by
  have meaning := (check_exact _ _ _).mp checked
  exact ⟨meaning.2.1,meaning.2.2.1⟩

theorem lost_holding_rejected (m : ReferenceStore.Machine p a) (binding : Binding)
    (ticket : InvocationBoundary.Ticket) (lost : ReferenceMutation.holdingLive m binding = false) :
    check m binding ticket = false := by simp [check,lost]

#print axioms checked_holding
#print axioms lost_holding_rejected
#print axioms check_exact
#print axioms changed_binding_rejected
#print axioms lost_guard_rejected
#print axioms historical_invalidation_rejected
#print axioms checked_current_permission
end MirroreaProofFirst.ReferenceResult
