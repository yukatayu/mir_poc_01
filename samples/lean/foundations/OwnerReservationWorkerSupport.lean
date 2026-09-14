-- External factoring of the existing transition/codec, without an IO entrypoint.
import MirroreaProofFirstOwnerReservation
import MirroreaProofFirstOwnerReceipt

-- External nonproduction process experiment. These commands are private test
-- framing, not Mir syntax, accepted transport authorization or a public wire.
-- A sole trusted launcher owns the pipe and fresh namespace. No populated
-- restart, external effects, child spawn or independent origin grant exists.
open MirroreaProofFirst
namespace OwnerReservationWorker
open OwnerCodecTree

inductive Command (p a : Nat) where
  | initialize (image : OwnerImage.Image p a)
  | reserve (ticket : InvocationBoundary.Ticket)
  | compute
  | install (revision : Nat) (image : OwnerImage.Image p a)
  | abandon

def commandCodec (p a : Nat) : Codec (Command p a) := iso
  (sum (OwnerFullCodec.image p a)
    (sum OwnerFullCodec.ticket (sum unit (sum (product natural (OwnerFullCodec.image p a)) unit))))
  (fun command => match command with
    | .initialize image => .inl image
    | .reserve ticket => .inr (.inl ticket)
    | .compute => .inr (.inr (.inl ()))
    | .install revision image => .inr (.inr (.inr (.inl (revision,image))))
    | .abandon => .inr (.inr (.inr (.inr ()))))
  (fun raw => match raw with
    | .inl image => .initialize image
    | .inr (.inl ticket) => .reserve ticket
    | .inr (.inr (.inl _)) => .compute
    | .inr (.inr (.inr (.inl (revision,image)))) => .install revision image
    | .inr (.inr (.inr (.inr _))) => .abandon)
  (by intro command; cases command <;> rfl)
  (by intro raw; rcases raw with image | ticket | ⟨⟩ | ⟨revision,image⟩ | ⟨⟩ <;> rfl)

def replyCodec := sum natural OwnerReceipt.codec

def responseCode : OwnerReservation.Response → Nat
  | .denied => 1 | .busy => 2 | .duplicate => 3 | .conflict => 4 | .exhausted => 5 | .reserved => 6

def imageCheck (assigned : OwnerEvaluator.Assignment p) (image : OwnerImage.Image p a) : Bool :=
  decide (image.definitions ≤ OwnerPayload.maxRecords ∧ image.count ≤ OwnerPayload.maxRecords ∧
    image.state.realm = assigned.realm ∧ image.view.realm = assigned.realm) && OwnerTableEvaluator.validity image

def transition (assigned : OwnerEvaluator.Assignment p) (scopeId capacity : Nat)
    (s : Option (OwnerReservation.State p a)) (command : Command p a) :
    Option (OwnerReservation.State p a) × Sum Nat OwnerReceipt.Envelope :=
  match s,command with
  | none,.initialize image => if imageCheck assigned image then
      (some (OwnerReservation.initial assigned scopeId image capacity),.inl 10) else (s,.inl 1)
  | none,_ => (s,.inl 1)
  | some _,.initialize _ => (s,.inl 1)
  | some state,.reserve ticket =>
      let (next,response) := OwnerReservation.reserve state scopeId ticket
      (some next,.inl (responseCode response))
  | some state,.compute =>
      match OwnerReservation.compute state with
      | none => (s,.inl 11)
      | some (next,reply) =>
          (some next,match reply with
            | .produced record => .inr (OwnerReceipt.project record)
            | .denied => .inl 1 | .duplicate => .inl 3 | .conflict => .inl 4 | .exhausted => .inl 5)
  | some state,.install revision image =>
      if imageCheck assigned image && decide (state.core.revision < revision) then
        match OwnerReservation.install state revision image with
        | some next => (some next,.inl 7)
        | none => (s,.inl 2)
      else (s,.inl 1)
  | some state,.abandon => (some (OwnerReservation.abandon state),.inl 8)

-- Every admitted update of an existing native-driver value either frames it or
-- is one of the exact reservation-model steps. No command can reset history.
theorem transition_step
    (ran : transition assigned scopeId capacity (some state) command = (next,reply)) :
    next = some state ∨ ∃ value, next = some value ∧ OwnerReservation.Step state value := by
  cases command with
  | «initialize» image => exact Or.inl (Prod.mk.inj ran).1.symm
  | reserve ticket =>
      cases reserved : OwnerReservation.reserve state scopeId ticket with
      | mk value response =>
          simp only [transition,reserved,Prod.mk.injEq] at ran
          exact Or.inr ⟨value,ran.1.symm,.reservation reserved⟩
  | compute =>
      cases computed : OwnerReservation.compute state with
      | none => simp only [transition,computed,Prod.mk.injEq] at ran; exact Or.inl ran.1.symm
      | some pair =>
          obtain ⟨value,response⟩ := pair
          simp only [transition,computed,Prod.mk.injEq] at ran
          exact Or.inr ⟨value,ran.1.symm,.computation computed⟩
  | install revision image =>
      simp only [transition] at ran
      split at ran
      · cases installed : OwnerReservation.install state revision image with
        | none => simp only [installed,Prod.mk.injEq] at ran; exact Or.inl ran.1.symm
        | some value =>
            simp only [installed,Prod.mk.injEq] at ran
            exact Or.inr ⟨value,ran.1.symm,.installation installed⟩
      · exact Or.inl (Prod.mk.inj ran).1.symm
  | abandon => exact Or.inr ⟨_,(Prod.mk.inj ran).1.symm,.abandonment⟩

theorem command_bytes (command : Command p a) :
    OwnerPacketCodec.decode (commandCodec p a) (OwnerPacketCodec.encode (commandCodec p a) command) = some command :=
  OwnerPacketCodec.roundtrip _ _

#print axioms transition_step
#print axioms command_bytes


end OwnerReservationWorker
