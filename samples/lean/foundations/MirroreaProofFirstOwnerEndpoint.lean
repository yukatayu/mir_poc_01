import OwnerReservationWorkerSupport

namespace MirroreaProofFirst.OwnerEndpoint

-- Local endpoint fence for a single fresh owner namespace. Authenticated
-- publisher origin and physical exclusivity are NOT derived from this record.
structure State (p a : Nat) where
  owner : OwnerReservation.State p a
  fence : Nat

inductive Command (p a : Nat) where
  | owner (command : OwnerReservationWorker.Command p a)
  | freeze (revision : Nat)

def permitted (s : State p a) : OwnerReservationWorker.Command p a → Bool
  | .reserve _ | .compute => decide (s.owner.core.revision = s.fence)
  | .install revision _ => decide (revision = s.fence)
  | _ => true

-- Exact canonical-byte equality of the complete stored image, not a hash or a
-- caller-declared id. General codec injectivity supplies the equality meaning.
def sameImage (left right : OwnerImage.Image p a) : Bool :=
  decide (OwnerPacketCodec.encode (OwnerFullCodec.image p a) left = OwnerPacketCodec.encode (OwnerFullCodec.image p a) right)

theorem sameImage_exact {left right : OwnerImage.Image p a} : sameImage left right = true ↔ left = right := by
  constructor
  · intro checked
    have encoded : OwnerPacketCodec.encode (OwnerFullCodec.image p a) left = OwnerPacketCodec.encode (OwnerFullCodec.image p a) right := by
      simpa [sameImage] using checked
    have decoded := congrArg (OwnerPacketCodec.decode (OwnerFullCodec.image p a)) encoded
    rw [OwnerPacketCodec.roundtrip,OwnerPacketCodec.roundtrip] at decoded
    exact Option.some.inj decoded
  · intro equal; cases equal; simp [sameImage]

-- Idempotent confirmation of a completed installation. This is not a second
-- install, freeze/quiescence ack, new image admission or current-use authority.
def confirms (s : State p a) : OwnerReservationWorker.Command p a → Bool
  | .install revision image => decide (revision = s.owner.core.revision ∧ revision = s.fence) &&
      sameImage s.owner.core.image image
  | _ => false

theorem confirms_exact : confirms s (.install revision image) = true ↔
    revision = s.owner.core.revision ∧ revision = s.fence ∧ s.owner.core.image = image := by
  simp [confirms,sameImage_exact,and_assoc]

def transition (assigned : OwnerEvaluator.Assignment p) (scopeId capacity : Nat)
    (state : Option (State p a)) (command : Command p a) :
    Option (State p a) × Sum Nat OwnerReceipt.Envelope :=
  match state,command with
  | none,.owner command =>
      let (next,reply) := OwnerReservationWorker.transition assigned scopeId capacity none command
      (next.map (fun owner => ⟨owner,0⟩),reply)
  | none,.freeze _ => (none,.inl 1)
  | some s,.owner command =>
      if confirms s command then (state,.inl 7) else if permitted s command then
        let (next,reply) := OwnerReservationWorker.transition assigned scopeId capacity (some s.owner) command
        (next.map (fun owner => ⟨owner,s.fence⟩),reply)
      else (state,.inl 14)
  | some s,.freeze revision =>
      if s.owner.active.isSome then (state,.inl 2)
      else (some {s with fence := max s.fence revision},.inl 12)

-- All existing owner updates retain the reservation/production histories.
-- Freezing is local: its ack can only follow an idle owner transition.
theorem owner_step
    (ran : transition assigned scopeId capacity (some s) command = (next,reply)) :
    ∃ value, next = some value ∧ (value.owner = s.owner ∨ OwnerReservation.Step s.owner value.owner) ∧
      s.fence ≤ value.fence := by
  cases command with
  | owner command =>
    simp only [transition] at ran
    split at ran
    · exact ⟨s,(Prod.mk.inj ran).1.symm,Or.inl rfl,Nat.le_refl _⟩
    · split at ran
      · cases previous : OwnerReservationWorker.transition assigned scopeId capacity (some s.owner) command with
        | mk nextOwner response =>
          simp only [previous] at ran
          rcases OwnerReservationWorker.transition_step previous with unchanged | ⟨value,same,step⟩
          · rw [unchanged] at ran
            simp only [Option.map_some,Prod.mk.injEq] at ran
            exact ⟨_,ran.1.symm,Or.inl rfl,Nat.le_refl _⟩
          · rw [same] at ran
            simp only [Option.map_some,Prod.mk.injEq] at ran
            exact ⟨_,ran.1.symm,Or.inr step,Nat.le_refl _⟩
      · exact ⟨s,(Prod.mk.inj ran).1.symm,Or.inl rfl,Nat.le_refl _⟩
  | freeze revision =>
    simp only [transition] at ran
    split at ran
    · exact ⟨s,(Prod.mk.inj ran).1.symm,Or.inl rfl,Nat.le_refl _⟩
    · exact ⟨_,(Prod.mk.inj ran).1.symm,Or.inl rfl,Nat.le_max_left _ _⟩

theorem frozen_refuses_reserve (frozen : s.owner.core.revision ≠ s.fence) :
    transition assigned scopeId capacity (some s) (.owner (.reserve ticket)) = (some s,.inl 14) := by
  simp [transition,confirms,permitted,frozen]

theorem frozen_refuses_compute (frozen : s.owner.core.revision ≠ s.fence) :
    transition assigned scopeId capacity (some s) (.owner .compute) = (some s,.inl 14) := by
  simp [transition,confirms,permitted,frozen]

theorem freeze_ack_idle
    (ran : transition assigned scopeId capacity (some s) (.freeze revision) = (next,.inl 12)) :
    s.owner.active = none ∧ ∃ value, next = some value ∧ revision ≤ value.fence ∧ value.owner = s.owner := by
  simp only [transition] at ran
  split at ran
  · cases ran
  · rename_i idle
    exact ⟨by simpa using idle,_,(Prod.mk.inj ran).1.symm,Nat.le_max_right _ _,rfl⟩

theorem freeze_active_refused (active : s.owner.active = some ticket) :
    transition assigned scopeId capacity (some s) (.freeze revision) = (some s,.inl 2) := by
  simp [transition,active]

theorem installation_at_fence
    (ran : transition assigned scopeId capacity (some s) (.owner (.install revision image)) = (next,.inl 7)) :
    revision = s.fence := by
  simp only [transition] at ran
  split at ran
  · rename_i confirmation
    exact (confirms_exact.mp confirmation).2.1
  · split at ran
    · rename_i admitted
      simpa [permitted] using admitted
    · cases ran

theorem exact_installation_reconfirms
    (revisionAt : revision = s.owner.core.revision) (fenceAt : revision = s.fence)
    (imageAt : image = s.owner.core.image) :
    transition assigned scopeId capacity (some s) (.owner (.install revision image)) = (some s,.inl 7) := by
  have confirmed := confirms_exact.mpr ⟨revisionAt,fenceAt,imageAt.symm⟩
  simp [transition,confirmed]

-- No completed-install acknowledgement is possible for a different image at
-- the same revision. A higher floor also prevents confirming an older image.
theorem same_revision_different_image_refused
    (revisionAt : revision = s.owner.core.revision) (different : s.owner.core.image ≠ image) :
    (transition assigned scopeId capacity (some s) (.owner (.install revision image))).2 ≠ .inl 7 := by
  have noConfirm : confirms s (.install revision image) = false := by
    cases h : confirms s (.install revision image) with
    | false => rfl
    | true => exact False.elim (different (confirms_exact.mp h).2.2)
  simp only [transition,noConfirm,Bool.false_eq_true,ite_false]
  split
  · simp [OwnerReservationWorker.transition,revisionAt]
  · simp

-- Independent positive reservation assumptions, with the local admission fence
-- open. This theorem does not promise a command/CPU deadline or recovery.
theorem enabled_reservation_progress
    (current : s.owner.core.revision = s.fence)
    (scope : scopeId = s.owner.core.scopeId)
    (admitted : OwnerEvaluator.Admitted s.owner.core.assigned s.owner.core.image ticket)
    (idle : s.owner.active = none) (fresh : OwnerReservation.hasKey s.owner ticket = false)
    (room : s.owner.reserved.length < s.owner.core.capacity) :
    ∃ next, transition assigned scopeId capacity (some s) (.owner (.reserve ticket)) = (some next,.inl 6) := by
  obtain ⟨owner,reserved⟩ := OwnerReservation.admissible_reservation_progress scope admitted idle fresh room
  exact ⟨⟨owner,s.fence⟩,by simp [transition,confirms,permitted,current,OwnerReservationWorker.transition,reserved,OwnerReservationWorker.responseCode]⟩

#print axioms sameImage_exact
#print axioms confirms_exact
#print axioms exact_installation_reconfirms
#print axioms same_revision_different_image_refused
#print axioms owner_step
#print axioms frozen_refuses_reserve
#print axioms frozen_refuses_compute
#print axioms freeze_ack_idle
#print axioms freeze_active_refused
#print axioms installation_at_fence
#print axioms enabled_reservation_progress
end MirroreaProofFirst.OwnerEndpoint
