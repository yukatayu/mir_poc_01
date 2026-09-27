import MirroreaProofFirstOwnerActualRoot

namespace MirroreaProofFirst.PublicJointHistory

-- One closed-operation history over the SAME source and owner states.
-- This is the logical registration/idle projection, not yet the physical
-- driver, pending INSTALL payment/ordinal, or whole funding invariant.
-- Queries and known refusals frame this projection; unknown IO retires in
-- the outer physical relation and has no continuing state here.
abbrev State := SourceRegistration.State
open SourceRegistration (ownerResult)

def sourceResult (s : State p a) (next : PublicationInput.State p a) : State p a :=
  {s with actual.source := next}

def publicSource : PublicationInput.Command p a → Prop
  | .enter _ | .finish _ => False
  | _ => True

def workResult (s : State p a) (i : Fin p) (ticket : OwnerOccurrence.Ticket)
    (revision : Nat) (reserved probed computed : OwnerEndpointBudget.State p a)
    (envelope : OwnerReceipt.Envelope) (after : PublicationInput.State p a) : State p a :=
  sourceResult (ownerResult (ownerResult (ownerResult s i (.owner (.reserve ticket)) reserved (.inl 6))
    i (.freeze revision) probed (.inl 2)) i (.owner .compute) computed (.inr envelope)) after

theorem work_owners :
    (workResult s i ticket revision reserved probed computed envelope after).actual.owners =
      PublicOwnerBoundary.put s.actual.owners i computed := by
  funext j
  by_cases equal : j = i <;>
    simp [workResult,sourceResult,ownerResult,SourceOwnerFloor.ownerResult,
      PublicationPayload.put,PublicOwnerBoundary.put,equal]

-- Exact constituent transitions, including the actual source finish. No
-- constructor assumes the desired idle/current/root conclusion.
inductive Step (assigned : Fin p → OwnerEvaluator.Assignment p) (scopeId : Nat)
    (capacity : Fin p → Nat) : State p a → State p a → Prop where
  | frame : Step assigned scopeId capacity s s
  | source : s.actual.pending = none → publicSource command →
      SourceOwnerFloor.sourceAllowed s.actual command → SourceRegistration.sourceAllowed s command →
      PublicationInput.execute scopeId s.actual.source command = some next →
      Step assigned scopeId capacity s (sourceResult s next)
  | notify : s.actual.pending = some (i,revision) →
      PublicationInput.execute scopeId s.actual.source (.freeze i revision) = some next →
      Step assigned scopeId capacity s {s with actual.source := next,actual.pending := none}
  | owner : s.actual.pending = none → PublicOwnerBoundary.Admin (.owner command) →
      SourceOwnerImage.ImageAllowed s.actual.source command →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.actual.owners i) (.owner command) = (next,reply) →
      Step assigned scopeId capacity s (ownerResult s i (.owner command) next reply)
  | extraFreeze : s.actual.pending = none → revision ≤ s.actual.source.publication.barrier.published →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.actual.owners i) (.freeze revision) = (next,reply) →
      Step assigned scopeId capacity s (ownerResult s i (.freeze revision) next reply)
  | failedFreeze : s.actual.pending = none → reply ≠ .inl 12 →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.actual.owners i) (.freeze revision) = (next,reply) →
      Step assigned scopeId capacity s (ownerResult s i (.freeze revision) next reply)
  | paidFreeze : s.actual.pending = none →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.actual.owners i) (.freeze revision) = (next,.inl 12) →
      Step assigned scopeId capacity s
        {ownerResult s i (.freeze revision) next (.inl 12) with actual.pending := some (i,revision)}
  | work : s.actual.pending = none →
      PublicationInput.execute scopeId s.actual.source (.enter i) = some entered →
      entered.dispatch = some ⟨i,⟨scopeId,s.actual.source.publication.barrier.installed i⟩,ticket⟩ →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) (s.actual.owners i)
        (.owner (.reserve ticket)) = (reserved,.inl 6) →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) reserved
        (.freeze revision) = (probed,.inl 2) →
      OwnerEndpointBudget.transition (assigned i) scopeId (capacity i) probed
        (.owner .compute) = (computed,.inr envelope) →
      envelope.ticket = ticket → envelope.scopeId = scopeId →
      envelope.revision = s.actual.source.publication.barrier.installed i →
      PublicationInput.execute scopeId entered (.finish i) = some after →
      Step assigned scopeId capacity s (workResult s i ticket revision reserved probed computed envelope after)

theorem registration_step
    (path : SourceRegistration.Runs assigned scopeId capacity budget seed s)
    (step : Step assigned scopeId capacity s next) :
    SourceRegistration.Runs assigned scopeId capacity budget seed next := by
  cases step with
  | frame => exact path
  | source clear allowedPublic floor installed ran => exact .source path clear floor installed ran
  | notify pending ran => exact .notify path pending ran
  | owner clear admin image ran => exact .owner path clear image ran
  | extraFreeze clear bounded ran => exact .extraFreeze path clear bounded ran
  | failedFreeze clear failed ran => exact .failedFreeze path clear failed ran
  | paidFreeze clear ran => exact .paidFreeze path clear ran
  | @work i arity entered ticket reserved revision probed computed envelope after state clear enter dispatch reserve probe compute correlated scopeMatches version finish =>
    have sourcePath := SourceRegistration.Runs.source (command:=.enter i) path clear trivial trivial enter
    have reservedPath := SourceRegistration.Runs.owner (i:=i) (command:=.reserve ticket)
      sourcePath clear trivial reserve
    have probedPath := SourceRegistration.Runs.failedFreeze (i:=i) (revision:=revision) (reply:=.inl 2)
      reservedPath clear (by decide) (by simpa [ownerResult,SourceOwnerFloor.ownerResult,PublicationPayload.put] using probe)
    have computedPath := SourceRegistration.Runs.owner (i:=i) (command:=.compute)
      probedPath clear trivial (by simpa [ownerResult,SourceOwnerFloor.ownerResult,PublicationPayload.put] using compute)
    have finalPath := SourceRegistration.Runs.source (command:=.finish i)
      computedPath clear trivial trivial finish
    exact finalPath

theorem owner_step (step : Step assigned scopeId capacity s next) :
    PublicOwnerBoundary.Step assigned scopeId capacity (some s.actual.owners) (some next.actual.owners) := by
  cases step with
  | frame => exact .frame
  | source clear allowedPublic floor installed ran => exact .frame
  | notify pending ran => exact .frame
  | owner clear admin image ran => exact .admin admin ran
  | extraFreeze clear bounded ran => exact .admin (command:=.freeze _) trivial ran
  | failedFreeze clear failed ran => exact .admin (command:=.freeze _) trivial ran
  | paidFreeze clear ran => exact .admin (command:=.freeze _) trivial ran
  | work clear enter dispatch reserve probe compute correlated scopeMatches version finish =>
    rw [work_owners]
    exact .work (.run reserve probe compute)

inductive Runs (assigned : Fin p → OwnerEvaluator.Assignment p) (scopeId : Nat)
    (capacity : Fin p → Nat) (budget : Nat) (seed : QualifiedCustody.State p a) : State p a → Prop where
  | fresh : Runs assigned scopeId capacity budget seed (SourceRegistration.initial seed budget)
  | step : Runs assigned scopeId capacity budget seed s → Step assigned scopeId capacity s next →
      Runs assigned scopeId capacity budget seed next

theorem registration_path (path : Runs assigned scopeId capacity budget seed s) :
    SourceRegistration.Runs assigned scopeId capacity budget seed s := by
  induction path with
  | fresh => exact .fresh
  | step prior step ih => exact registration_step ih step

theorem owner_path (path : Runs assigned scopeId capacity budget seed s) :
    PublicOwnerBoundary.Runs assigned scopeId capacity budget (some s.actual.owners) := by
  induction path with
  | fresh => exact .fresh
  | step prior step ih => exact .step ih (owner_step step)

theorem continuing_idle (path : Runs assigned scopeId capacity budget seed s) :
    PublicOwnerBoundary.Idle s.actual.owners := PublicOwnerBoundary.continuing_idle (owner_path path)

-- No independently chosen owner history, idle assumption or current-image
-- preflight remains in this consumer. Presence and a clear registration gate
-- remain real premises; freshness/room/funding/progress are not claimed.
theorem entered_ready
    (path : Runs assigned scopeId capacity budget seed s)
    (clear : s.actual.pending = none)
    (present : (s.actual.owners i).owner = some owner)
    (entered : PublicationInput.execute scopeId s.actual.source (.enter i) = some next) :
    OwnerImageMonitor.Usable (s.actual.owners i) s.actual.source.publication.barrier.published
      (CustodyPublication.image s.actual.source.publication.current) ∧
    owner.owner.active = none ∧
    ∃ initialImage, OwnerReservation.Steps
      (OwnerReservation.initial (assigned i) scopeId initialImage (capacity i)) owner.owner := by
  exact ⟨SourceRegistration.entered_current (registration_path path) clear present entered,
    PublicOwnerBoundary.actual_present_idle (owner_path path) present,
    OwnerActualRoot.public_root (owner_path path) present⟩

#print axioms work_owners
#print axioms registration_step
#print axioms owner_step
#print axioms registration_path
#print axioms owner_path
#print axioms continuing_idle
#print axioms entered_ready

-- Construct the actual owner interval at an accepted logical entry using the
-- same joint history. These independent resource/profile/admission premises
-- remain; no successful owner reservation or computation is assumed.
theorem entered_enabled_work
    (path : Runs assigned scopeId capacity budget seed s)
    (clear : s.actual.pending = none)
    (present : (s.actual.owners i).owner = some owner)
    (entered : PublicationInput.execute scopeId s.actual.source (.enter i) = some next)
    (_dispatched : next.dispatch = some ⟨i,⟨scopeId,s.actual.source.publication.barrier.installed i⟩,ticket⟩)
    (small : capacity i ≤ 64)
    (profile : OwnerResponseProfile.ResponseFits owner.owner.core.scopeId owner.owner.core.revision ticket)
    (admitted : OwnerEvaluator.Admitted owner.owner.core.assigned owner.owner.core.image ticket)
    (fresh : OwnerReservation.hasKey owner.owner ticket = false)
    (room : owner.owner.reserved.length < owner.owner.core.capacity)
    (funded : 3 ≤ (s.actual.owners i).remaining) :
    ∃ after, PublicOwnerBoundary.Work (assigned i) scopeId (capacity i) ticket revision
        (s.actual.owners i) after ∧ OwnerEndpointBudget.held after.owner = false := by
  obtain ⟨actual,atOwner,atRevision,atImage,current⟩ :=
    SourceRegistration.entered_current (registration_path path) clear present entered
  have same : actual = owner := Option.some.inj (atOwner.symm.trans present)
  subst actual
  exact OwnerActualRoot.public_enabled_work (owner_path path) present small profile
    (atRevision.trans current.symm) admitted fresh room funded

#print axioms entered_enabled_work

end MirroreaProofFirst.PublicJointHistory
