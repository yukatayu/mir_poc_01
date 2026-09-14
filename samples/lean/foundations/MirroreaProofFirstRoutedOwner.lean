import MirroreaProofFirstOwnerEndpointBudget
import MirroreaProofFirstPublicationCapacity

namespace MirroreaProofFirst.RoutedOwner

-- Only the response context relation: this does NOT authenticate a process,
-- establish its image/authorization or reserve its remaining credits.
def ReplyContextMatches (dispatch : PublicationInput.Dispatch p) (owner : OwnerEndpoint.State p a) : Prop :=
  owner.owner.core.scopeId = dispatch.context.scopeId ∧
  owner.owner.core.revision = dispatch.context.revision

-- A dispatch produced by an admitted source enter inherits its checked output
-- profile. Equality to the recipient's STORED context is a separate premise;
-- a caller-chosen startup number alone does not prove that premise physically.
theorem generated_profile {source entered : PublicationInput.State p a}
    (fit : PublicationCapacity.StateFits scope source)
    (enter : PublicationInput.execute scope source (.enter endpoint) = some entered)
    (dispatchAt : entered.dispatch = some dispatch)
    (bound : ReplyContextMatches dispatch owner) :
    OwnerResponseProfile.ResponseFits owner.owner.core.scopeId owner.owner.core.revision dispatch.ticket := by
  have ready := PublicationCapacity.held_dispatch_fits fit enter dispatch dispatchAt
  simpa [bound.1,bound.2] using ready

theorem generated_guard {source entered : PublicationInput.State p a}
    (fit : PublicationCapacity.StateFits scope source)
    (enter : PublicationInput.execute scope source (.enter endpoint) = some entered)
    (dispatchAt : entered.dispatch = some dispatch)
    (bound : ReplyContextMatches dispatch owner) (small : owner.owner.core.capacity ≤ 64) :
    OwnerEndpointProfile.check capacity (some owner) (.owner (.reserve dispatch.ticket)) = true :=
  OwnerEndpointProfile.check_exact.mpr ⟨small,generated_profile fit enter dispatchAt bound⟩

-- Independent existing semantic admission plus exact context and funded room
-- gives a reserve AND computation. No successful reserve/reply is a premise.
-- Source continuation consumption and physical origin are not claimed here.
theorem generated_funded_roundtrip {source entered : PublicationInput.State p a}
    (fit : PublicationCapacity.StateFits scope source)
    (enter : PublicationInput.execute scope source (.enter endpoint) = some entered)
    (dispatchAt : entered.dispatch = some dispatch)
    (bound : ReplyContextMatches dispatch owner)
    (path : OwnerReservation.Steps (OwnerReservation.initial initialAssigned initialScope initialImage initialCapacity) owner.owner)
    (small : owner.owner.core.capacity ≤ 64)
    (current : owner.owner.core.revision = owner.fence)
    (admitted : OwnerEvaluator.Admitted owner.owner.core.assigned owner.owner.core.image dispatch.ticket)
    (idle : owner.owner.active = none) (fresh : OwnerReservation.hasKey owner.owner dispatch.ticket = false)
    (room : owner.owner.reserved.length < owner.owner.core.capacity) (funded : 2 ≤ credits) :
    ∃ (reserved computed : OwnerEndpointBudget.State p a) (record : OwnerOccurrence.Record p a),
      OwnerEndpointBudget.transition assigned dispatch.context.scopeId capacity ⟨some owner,credits⟩
        (.owner (.reserve dispatch.ticket)) = (reserved,.inl 6) ∧
      OwnerEndpointBudget.transition assigned dispatch.context.scopeId capacity reserved (.owner .compute) =
        (computed,.inr (OwnerReceipt.project record)) ∧
      reserved.remaining = credits-1 ∧ computed.remaining = credits-2 ∧
      OwnerResponseProfile.WireFits OwnerReservationWorker.replyCodec 256 (.inr (OwnerReceipt.project record)) ∧
      OwnerResponseProfile.WireFits (PublicationInput.input p a) 256 (.step (.arrival (OwnerReceipt.project record))) :=
  OwnerEndpointBudget.enabled_roundtrip path small (generated_profile fit enter dispatchAt bound)
    current bound.1.symm admitted idle fresh room funded

#print axioms generated_profile
#print axioms generated_guard
#print axioms generated_funded_roundtrip
end MirroreaProofFirst.RoutedOwner
