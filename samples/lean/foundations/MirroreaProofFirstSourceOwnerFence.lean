import MirroreaProofFirstSourceOwnerGate

namespace MirroreaProofFirst.SourceOwnerFence

def actionFences (old : Fin p → Nat) :
    PublicationUse.Action p (QualifiedPublication.Command p a) → Fin p → Nat
  | .administrative (.freeze endpoint revision) => Publication.put old endpoint (max (old endpoint) revision)
  | _ => old

def commandFences (old : Fin p → Nat) : PublicationInput.Command p a → Fin p → Nat
  | .freeze endpoint revision => Publication.put old endpoint (max (old endpoint) revision)
  | _ => old

theorem image_fences
    (ran : PublicationImage.execute CustodyPublication.image QualifiedCustody.evaluate source action = some next) :
    next.barrier.fence = actionFences source.barrier.fence action := by
  unfold PublicationImage.execute at ran
  split at ran
  · cases ran
    cases action with
    | enter endpoint => rfl
    | finish endpoint => rfl
    | administrative action => cases action <;> rfl
  · cases ran

theorem selected_fences
    (selected : PublicationInput.select source command = some action) :
    actionFences source.publication.barrier.fence action = commandFences source.publication.barrier.fence command := by
  cases command with
  | stage input => cases input <;> cases selected <;> rfl
  | freeze endpoint revision => cases selected; rfl
  | acknowledge endpoint revision => cases selected; rfl
  | publish => cases selected; rfl
  | install endpoint revision => cases selected; rfl
  | finish endpoint => cases selected; rfl
  | enter endpoint =>
    simp only [PublicationInput.select] at selected
    split at selected
    · cases selected
    · cases waiting : source.publication.current.privateState.session.state.source.waiting with
      | none => simp [waiting] at selected
      | some saved =>
        simp only [waiting,Option.bind_eq_bind,Option.bind_some] at selected
        split at selected <;> cases selected <;> rfl
  | arrival envelope =>
    cases dispatch : source.dispatch with
    | none => simp [PublicationInput.select,dispatch] at selected
    | some dispatched =>
      by_cases ticket : envelope.ticket = dispatched.ticket
      · cases accepted : QualifiedReceipt.accept dispatched.context source.publication.current envelope with
        | none => simp [PublicationInput.select,dispatch,ticket,accepted] at selected
        | some privateState =>
          cases result : envelope.result with
          | rejected reason => simp [PublicationInput.select,dispatch,ticket,result] at selected
          | value value =>
            have equal : action = .administrative (.stage (.receive envelope.ticket value)) := by
              simpa [PublicationInput.select,dispatch,ticket,accepted,result] using selected.symm
            rw [equal]; rfl
      · simp [PublicationInput.select,dispatch,ticket] at selected

theorem source_fences (ran : PublicationInput.execute scopeId source command = some next) :
    next.publication.barrier.fence = commandFences source.publication.barrier.fence command := by
  obtain ⟨action,selected,executed,_⟩ := PublicationInput.executed_action ran
  exact (image_fences executed).trans (selected_fences selected)

-- Extra current/older freezes cannot outrun the source gate. This theorem
-- includes successful and refused/busy/profile/budget replies, not only12.
theorem bounded_owner_freeze_frames
    (bounded : revision ≤ OwnerFenceMonitor.floor owner.owner)
    (ran : OwnerEndpointBudget.transition assigned scopeId capacity owner (.freeze revision) = (next,reply)) :
    OwnerFenceMonitor.floor next.owner = OwnerFenceMonitor.floor owner.owner := by
  rw [OwnerFenceMonitor.budget_exact ran]
  cases reply with
  | inr envelope => rfl
  | inl code =>
    by_cases success : code = 12
    · subst code; simp [OwnerFenceMonitor.advance,Nat.max_eq_left bounded]
    · simp [OwnerFenceMonitor.advance,success]

-- A future freeze uses the certified head and retains its matching pending
-- payment until the actual source notification. The interstitial state may
-- differ; this law restores equality at the next continuing public boundary.
theorem paid_freeze_restores
    (before : OwnerFenceMonitor.floor owner.owner = source.publication.barrier.fence endpoint)
    (ownerRan : OwnerEndpointBudget.transition assigned scopeId capacity owner (.freeze revision) = (nextOwner,.inl 12))
    (sourceRan : PublicationInput.execute scopeId source (.freeze endpoint revision) = some nextSource) :
    OwnerFenceMonitor.floor nextOwner.owner = nextSource.publication.barrier.fence endpoint := by
  rw [OwnerFenceMonitor.budget_exact ownerRan,source_fences sourceRan]
  simp [OwnerFenceMonitor.advance,commandFences,Publication.put,before]

-- A head query means one exact retained source occurrence, not permission to
-- use the owner. The selected host permits future freezes only on that branch.
def futureFreezeCheck (state : PublicationCapacityDriver.State p a) (published : Nat)
    (endpoint : Fin p) (revision : Nat) : Bool :=
  decide (revision ≤ published) || SourceFundingQuery.matchesHead state (.freeze endpoint revision)

def FutureFreezeAllowed (state : PublicationCapacityDriver.State p a) (published : Nat)
    (endpoint : Fin p) (revision : Nat) : Prop :=
  revision ≤ published ∨ ∃ rest, state.suffix = .freeze endpoint revision :: rest

theorem future_freeze_exact : futureFreezeCheck state published endpoint revision = true ↔
    FutureFreezeAllowed state published endpoint revision := by
  simp [futureFreezeCheck,FutureFreezeAllowed,SourceFundingQuery.matches_head_exact,SourceFundingQuery.headCommand]

theorem future_freeze_is_head (checked : futureFreezeCheck state published endpoint revision = true)
    (future : published < revision) : ∃ rest, state.suffix = .freeze endpoint revision :: rest := by
  rcases future_freeze_exact.mp checked with old | head
  · omega
  · exact head

#print axioms image_fences
#print axioms selected_fences
#print axioms source_fences
#print axioms bounded_owner_freeze_frames
#print axioms paid_freeze_restores
#print axioms future_freeze_exact
#print axioms future_freeze_is_head
end MirroreaProofFirst.SourceOwnerFence
