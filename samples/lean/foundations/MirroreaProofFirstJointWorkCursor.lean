import MirroreaProofFirstCohortPhase
import MirroreaProofFirstOwnerReservationMonitor

namespace MirroreaProofFirst.JointWorkCursor
open PublicJointHistory (sourceResult)
open SourceRegistration (ownerResult)

inductive Phase where
  | entered | reserved | probed
  | computed (envelope : OwnerReceipt.Envelope)

-- Retain the LAST closed joint state. Every interior edge is one actual
-- source/owner operation on the same state. The list retains its exact inputs;
-- read-only native queries belong to the separate CohortPhase event projection. In particular reservation may
-- make the physical owner busy; no idle closed-history premise is asserted
-- for that interior. Host observation/commit gaps are a separate projection.
inductive Cursor (assigned : Fin p → OwnerEvaluator.Assignment p) (scope : Nat)
    (capacity : Fin p → Nat) (base : SourceRegistration.State p a)
    (target : Fin p) (ticket : OwnerOccurrence.Ticket) (revision : Nat) :
    List (CohortPhase.Event p a) → Phase → SourceRegistration.State p a → Prop where
  | enter : base.actual.pending = none →
      PublicationInput.execute scope base.actual.source (.enter target) = some entered →
      entered.dispatch = some ⟨target,⟨scope,base.actual.source.publication.barrier.installed target⟩,ticket⟩ →
      Cursor assigned scope capacity base target ticket revision [.source (vector,.step (.enter target))] .entered (sourceResult base entered)
  | reserve : Cursor assigned scope capacity base target ticket revision events .entered state →
      OwnerEndpointBudget.transition (assigned target) scope (capacity target) (state.actual.owners target)
        (.owner (.reserve ticket)) = (next,.inl 6) →
      Cursor assigned scope capacity base target ticket revision (events ++ [.owner target (.owner (.reserve ticket))]) .reserved
        (ownerResult state target (.owner (.reserve ticket)) next (.inl 6))
  | probe : Cursor assigned scope capacity base target ticket revision events .reserved state →
      OwnerEndpointBudget.transition (assigned target) scope (capacity target) (state.actual.owners target)
        (.freeze revision) = (next,.inl 2) →
      Cursor assigned scope capacity base target ticket revision (events ++ [.owner target (.freeze revision)]) .probed
        (ownerResult state target (.freeze revision) next (.inl 2))
  | compute : Cursor assigned scope capacity base target ticket revision events .probed state →
      OwnerEndpointBudget.transition (assigned target) scope (capacity target) (state.actual.owners target)
        (.owner .compute) = (next,.inr envelope) →
      envelope.ticket = ticket → envelope.scopeId = scope →
      envelope.revision = base.actual.source.publication.barrier.installed target →
      Cursor assigned scope capacity base target ticket revision (events ++ [.owner target (.owner .compute)]) (.computed envelope)
        (ownerResult state target (.owner .compute) next (.inr envelope))

theorem pending_clear (cursor : Cursor assigned scope capacity base target ticket revision events phase state) :
    state.actual.pending = none := by
  induction cursor with
  | enter clear ran dispatch => exact clear
  | reserve prior ran ih => exact ih
  | probe prior ran ih => exact ih
  | compute prior ran ticket scope revision ih => exact ih

theorem registration_inside
    (history : PublicJointHistory.Runs assigned scope capacity budget seed base)
    (cursor : Cursor assigned scope capacity base target ticket revision events phase state) :
    SourceRegistration.Runs assigned scope capacity budget seed state := by
  induction cursor with
  | enter clear ran dispatch =>
    exact .source (command:=.enter target) (PublicJointHistory.registration_path history) clear trivial trivial ran
  | reserve prior ran ih => exact .owner ih (pending_clear prior) trivial ran
  | probe prior ran ih => exact .failedFreeze ih (pending_clear prior) (by decide) ran
  | compute prior ran ticket scope revision ih => exact .owner ih (pending_clear prior) trivial ran

-- Semantic closure after an enabled matching finish. This theorem alone
-- does not assert a physical finish occurrence. The trace consumer must consume
-- that exact source event before advancing its last physically closed state.
theorem finish_closes
    (history : PublicJointHistory.Runs assigned scope capacity budget seed base)
    (cursor : Cursor assigned scope capacity base target ticket revision events (.computed envelope) state)
    (finish : PublicationInput.execute scope state.actual.source (.finish target) = some after) :
    PublicJointHistory.Runs assigned scope capacity budget seed (sourceResult state after) := by
  cases cursor with
  | compute probed compute ticketAt scopeAt revisionAt =>
    cases probed with
    | probe reserved probe =>
      cases reserved with
      | reserve entered reserve =>
        cases entered with
        | enter clear enter dispatch =>
          apply PublicJointHistory.Runs.step history
          apply PublicJointHistory.Step.work clear enter dispatch reserve
          · simpa [ownerResult,SourceOwnerFloor.ownerResult,PublicationPayload.put] using probe
          · simpa [ownerResult,SourceOwnerFloor.ownerResult,PublicationPayload.put] using compute
          · exact ticketAt
          · exact scopeAt
          · exact revisionAt
          · exact finish

#print axioms pending_clear
#print axioms registration_inside
#print axioms finish_closes
end MirroreaProofFirst.JointWorkCursor

namespace MirroreaProofFirst.OwnerObservationGap

-- One physical owner reply may precede its host monitor update. Retain both
-- sides and the actual request/reply occurrence under exclusive custody.
-- This local relation is not a whole-host/OS interleaving proof.
inductive State (p a : Nat) where
  | ready (physical : OwnerEndpointBudget.State p a) (known : Option OwnerReservationMonitor.Known)
  | replied (before : OwnerEndpointBudget.State p a) (known : Option OwnerReservationMonitor.Known)
      (command : OwnerEndpoint.Command p a) (after : OwnerEndpointBudget.State p a)
      (reply : Sum Nat OwnerReceipt.Envelope)
  | retired (physical : OwnerEndpointBudget.State p a)

def physical : State p a → OwnerEndpointBudget.State p a
  | .ready s _ | .retired s => s
  | .replied _ _ _ s _ => s

def synchronized : State p a → Bool
  | .ready _ _ => true
  | _ => false

def Coherent (assigned : OwnerEvaluator.Assignment p) (scope capacity : Nat) : State p a → Prop
  | .ready s known => OwnerReservationMonitor.project s.owner = known
  | .replied before known command after reply =>
      OwnerReservationMonitor.project before.owner = known ∧
      OwnerEndpointBudget.transition assigned scope capacity before command = (after,reply)
  | .retired _ => True

inductive Event (p a : Nat) where
  | response (command : OwnerEndpoint.Command p a) (reply : Sum Nat OwnerReceipt.Envelope)
  | commit | retire | local

inductive Step (assigned : OwnerEvaluator.Assignment p) (scope capacity : Nat) :
    State p a → Event p a → State p a → Prop where
  | frame : Step assigned scope capacity s .local s
  | response : OwnerEndpointBudget.transition assigned scope capacity before command = (after,reply) →
      Step assigned scope capacity (.ready before known) (.response command reply)
        (.replied before known command after reply)
  | commit : Step assigned scope capacity (.replied before known command after reply) .commit
      (.ready after (OwnerReservationMonitor.observe capacity known command reply))
  | retire : Step assigned scope capacity s .retire (.retired (physical s))

theorem initial_coherent {p a : Nat} (assigned : OwnerEvaluator.Assignment p) (scope capacity budget : Nat) :
    Coherent assigned scope capacity (.ready (OwnerEndpointBudget.initial (a:=a) budget) none) := rfl

theorem preserves (coherent : Coherent assigned scope capacity s)
    (step : Step assigned scope capacity s event next) : Coherent assigned scope capacity next := by
  cases step with
  | frame => exact coherent
  | response ran => exact ⟨coherent,ran⟩
  | commit =>
    change OwnerReservationMonitor.project _ = _
    rw [OwnerReservationMonitor.budget_exact coherent.2,coherent.1]
  | retire => trivial

theorem response_excludes_synchronization
    (step : Step assigned scope capacity s (.response command reply) next) : synchronized next = false := by
  cases step
  rfl

theorem commit_preserves_physical
    (step : Step assigned scope capacity s .commit next) : physical next = physical s := by
  cases step
  rfl

theorem retirement_preserves_physical
    (step : Step assigned scope capacity s .retire next) : physical next = physical s ∧ synchronized next = false := by
  cases step
  exact ⟨rfl,rfl⟩

theorem retired_absorbing
    (step : Step assigned scope capacity (.retired owner) event next) : next = .retired owner := by
  cases step <;> rfl

-- Every typed transition has a path in this local relation. This establishes
-- relative progress of the relation, not implementation progress. Synchronized
-- observation alone does not imply an idle owner or an open public-entry gate.
theorem actual_transition_progress
    (coherent : Coherent assigned scope capacity (.ready before known))
    (ran : OwnerEndpointBudget.transition assigned scope capacity before command = (after,reply)) :
    ∃ interior final,
      Step assigned scope capacity (.ready before known) (.response command reply) interior ∧
      Step assigned scope capacity interior .commit final ∧
      Coherent assigned scope capacity final ∧ physical final = after ∧ synchronized final = true := by
  let middle := State.replied before known command after reply
  let final := State.ready after (OwnerReservationMonitor.observe capacity known command reply)
  have first : Step assigned scope capacity (.ready before known) (.response command reply) middle := .response ran
  have second : Step assigned scope capacity middle .commit final := .commit
  exact ⟨middle,final,first,second,preserves (preserves coherent first) second,rfl,rfl⟩

#print axioms initial_coherent
#print axioms preserves
#print axioms response_excludes_synchronization
#print axioms commit_preserves_physical
#print axioms retirement_preserves_physical
#print axioms retired_absorbing
#print axioms actual_transition_progress
end MirroreaProofFirst.OwnerObservationGap
