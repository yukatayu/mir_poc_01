import MirroreaProofFirstPublicOwnerBoundary

namespace MirroreaProofFirst.PublicOwnerReplay
open PublicOwnerBoundary

-- This checker consumes one completed continuing outer operation. Captured
-- owner commands must match its expansion; framing source/query effects are
-- checked separately. A retired handle is not accepted as a continuing one.
inductive Action (p a : Nat) where
  | frame
  | admin (i : Fin p) (command : OwnerEndpoint.Command p a)
  | work (i : Fin p) (ticket : OwnerOccurrence.Ticket) (probeRevision : Nat)

def commands : Action p a → List (Fin p × OwnerEndpoint.Command p a)
  | .frame => []
  | .admin i command => [(i,command)]
  | .work i ticket revision =>
      [(i,.owner (.reserve ticket)),(i,.freeze revision),(i,.owner .compute)]

def adminCheck : OwnerEndpoint.Command p a → Bool
  | .owner (.initialize _) | .owner (.install _ _) | .freeze _ => true
  | _ => false

theorem admin_exact : adminCheck command = true ↔ Admin command := by
  cases command with
  | freeze => simp [adminCheck,Admin]
  | owner command => cases command <;> simp [adminCheck,Admin]

abbrev Reachable {p a : Nat} (assigned : Fin p → OwnerEvaluator.Assignment p) (scopeId : Nat)
    (capacity : Fin p → Nat) (budget : Nat) :=
  {owners : Owners p a // Runs assigned scopeId capacity budget (some owners)}

def initial {p a : Nat} (assigned : Fin p → OwnerEvaluator.Assignment p) (scopeId : Nat)
    (capacity : Fin p → Nat) (budget : Nat) : Reachable (a:=a) assigned scopeId capacity budget :=
  ⟨fun _ => OwnerEndpointBudget.initial budget,.fresh⟩

def advance {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p}
    {scopeId budget : Nat} {capacity : Fin p → Nat}
    (s : Reachable (a:=a) assigned scopeId capacity budget) (action : Action p a) :
    Option (Reachable (a:=a) assigned scopeId capacity budget) := by
  cases action with
  | frame => exact some s
  | admin i command =>
    if allowed : adminCheck command = true then
      let result := OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val i) command
      exact some ⟨put s.val i result.1,.step s.property (.admin (admin_exact.mp allowed) rfl)⟩
    else exact none
  | work i ticket revision =>
    let first := OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val i) (.owner (.reserve ticket))
    if reserved : first.2 = .inl 6 then
      let second := OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) first.1 (.freeze revision)
      if probed : second.2 = .inl 2 then
        let third := OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) second.1 (.owner .compute)
        match produced : third.2 with
        | .inl _ => exact none
        | .inr envelope =>
          have ran1 : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val i) (.owner (.reserve ticket)) =
              (first.1,.inl 6) := by rw [←reserved]
          have ran2 : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) first.1 (.freeze revision) =
              (second.1,.inl 2) := by rw [←probed]
          have ran3 : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) second.1 (.owner .compute) =
              (third.1,.inr envelope) := by rw [←produced]
          exact some ⟨put s.val i third.1,.step s.property (.work (.run ran1 ran2 ran3))⟩
      else exact none
    else exact none

theorem accepted_idle {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p} {scopeId budget : Nat} {capacity : Fin p → Nat} (s : Reachable (a:=a) assigned scopeId capacity budget) : Idle s.val :=
  continuing_idle s.property

theorem frame_complete {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p} {scopeId budget : Nat} {capacity : Fin p → Nat} (s : Reachable (a:=a) assigned scopeId capacity budget) :
    (advance s .frame).map Subtype.val = some s.val := rfl

-- Admin refusal is accepted as a real transition, including its debit.
theorem admin_complete {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p} {scopeId budget : Nat} {capacity : Fin p → Nat}
    {command : OwnerEndpoint.Command p a} {i : Fin p} (s : Reachable (a:=a) assigned scopeId capacity budget)
    (allowed : Admin command) :
    (advance s (.admin i command)).map Subtype.val = some (put s.val i
      (OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val i) command).1) := by
  simp [advance,admin_exact.mpr allowed]

theorem work_complete {p a : Nat} {assigned : Fin p → OwnerEvaluator.Assignment p} {scopeId budget : Nat} {capacity : Fin p → Nat}
    {i : Fin p} {ticket : OwnerOccurrence.Ticket} {revision : Nat}
    {first second third : OwnerEndpointBudget.State p a} {envelope : OwnerReceipt.Envelope} (s : Reachable (a:=a) assigned scopeId capacity budget)
    (reserved : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.val i)
      (.owner (.reserve ticket)) = (first,.inl 6))
    (probed : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) first (.freeze revision) = (second,.inl 2))
    (produced : OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) second (.owner .compute) = (third,.inr envelope)) :
    (advance s (.work i ticket revision)).map Subtype.val = some (put s.val i third) := by
  simp [advance,reserved,probed,produced]
  split
  · rename_i value impossible
    simp [reserved,probed,produced] at impossible
  · rfl

#print axioms admin_exact
#print axioms initial
#print axioms advance
#print axioms accepted_idle
#print axioms frame_complete
#print axioms admin_complete
#print axioms work_complete

end MirroreaProofFirst.PublicOwnerReplay
