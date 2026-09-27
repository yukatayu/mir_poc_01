import MirroreaProofFirstJointWorkCursor

namespace MirroreaProofFirst.WorkOccurrence
open PublicJointHistory (sourceResult)
open SourceRegistration (ownerResult)

-- Successful work-interval projection only. An event is a consumed typed
-- request, with its actual pre-send ordinal. Startup, literal API decoding,
-- local host commits and unknown physical outcomes remain separate obligations.
-- All source events use the SAME retained complete capacity driver.
inductive Phase where
  | waiting | entered | reserved | probed
  | computed (envelope : OwnerReceipt.Envelope)
  | finished (envelope : OwnerReceipt.Envelope)

structure State (p a : Nat) where
  driver : PublicationCapacityDriver.State p a
  joint : SourceRegistration.State p a
  ordinal : Nat

inductive Event (p a : Nat) where
  | source (preOrdinal : Nat) (request : SourceFundingInput.Request p a)
  | query (preOrdinal : Nat) (request : Sum (SourceFundingQuery.HeadRequest p) (Fin p))
  | owner (target : Fin p) (request : OwnerEndpoint.Command p a)

def vectorAt (s : State p a) (v : Vector (Fin 513) p) : Prop :=
  ∀ i, (s.joint.actual.owners i).remaining = (v[i.val]).val

def onSource (s : State p a) (next : PublicationCapacityDriver.State p a)
    (source : PublicationInput.State p a) : State p a :=
  ⟨next,sourceResult s.joint source,s.ordinal+1⟩

def onOwner (s : State p a) (i : Fin p) (command : OwnerEndpoint.Command p a)
    (next : OwnerEndpointBudget.State p a) (reply : Sum Nat OwnerReceipt.Envelope) : State p a :=
  ⟨s.driver,ownerResult s.joint i command next reply,s.ordinal⟩

-- General erasure fact for a successful funded STEP on an already-present
-- source. No equality of projected output can replace the complete old driver.
theorem accepted_semantic
    (valid : PublicationLifecycle.Invariant assigned scope bootstrap old)
    (present : old.source = some source)
    (ran : SourceFundingInput.execute assigned scope bootstrap old (vector,.step command) = (next,.accepted)) :
    ∃ after, next.source = some after ∧ PublicationInput.execute scope source command = some after := by
  have fast := (SourceFundingInput.accepted_exact.mp ran).1
  have full : PublicationLifecycle.transition assigned scope bootstrap old (.step command) = (next,.accepted) := by
    rw [← PublicationLifecycle.transitionFast_exact valid]
    exact fast
  have erased := (PublicationLifecycle.accepted_refines assigned scope bootstrap old (.step command)
    (by rw [full])).1
  rw [full,present] at erased
  simp only [PublicationInput.transition] at erased
  cases executed : PublicationInput.execute scope source command with
  | none => simp [executed] at erased
  | some after =>
    refine ⟨after,?_,rfl⟩
    simpa [executed] using (congrArg Prod.fst erased).symm

-- A rooted interval, not an independent choice of a driver or semantic cursor
-- at each operation. Query events count physical ordinals, including queries
-- which do not change the semantic driver. The initial ordinal is the actual
-- post-bootstrap/pre-entry value, supplied by the surrounding lifetime fold.
inductive Trace (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (base : State p a) (target : Fin p) (ticket : OwnerOccurrence.Ticket) :
    List (Event p a) → Phase → State p a → Prop where
  | start : Trace assigned scope bootstrap capacity base target ticket [] .waiting base
  | enter : base.joint.actual.pending = none → vectorAt base vector →
      SourceFundingInput.execute assigned scope bootstrap base.driver (vector,.step (.enter target)) = (next,.accepted) →
      next.source = some entered →
      entered.dispatch = some ⟨target,⟨scope,base.joint.actual.source.publication.barrier.installed target⟩,ticket⟩ →
      Trace assigned scope bootstrap capacity base target ticket
        [.source base.ordinal (vector,.step (.enter target))] .entered (onSource base next entered)
  | query : Trace assigned scope bootstrap capacity base target ticket events .entered s →
      Trace assigned scope bootstrap capacity base target ticket
        (events ++ [.query s.ordinal request]) .entered {s with ordinal := s.ordinal+1}
  | reserve : Trace assigned scope bootstrap capacity base target ticket events .entered s →
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.joint.actual.owners target)
        (.owner (.reserve ticket)) = (next,.inl 6) →
      Trace assigned scope bootstrap capacity base target ticket
        (events ++ [.owner target (.owner (.reserve ticket))]) .reserved
        (onOwner s target (.owner (.reserve ticket)) next (.inl 6))
  | probe : Trace assigned scope bootstrap capacity base target ticket events .reserved s →
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.joint.actual.owners target)
        (.freeze (base.joint.actual.source.publication.barrier.published+1)) = (next,.inl 2) →
      Trace assigned scope bootstrap capacity base target ticket
        (events ++ [.owner target (.freeze (base.joint.actual.source.publication.barrier.published+1))]) .probed
        (onOwner s target (.freeze (base.joint.actual.source.publication.barrier.published+1)) next (.inl 2))
  | compute : Trace assigned scope bootstrap capacity base target ticket events .probed s →
      OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) (s.joint.actual.owners target)
        (.owner .compute) = (next,.inr envelope) →
      envelope.ticket = ticket → envelope.scopeId = scope →
      envelope.revision = base.joint.actual.source.publication.barrier.installed target →
      Trace assigned scope bootstrap capacity base target ticket
        (events ++ [.owner target (.owner .compute)]) (.computed envelope)
        (onOwner s target (.owner .compute) next (.inr envelope))
  | finish : Trace assigned scope bootstrap capacity base target ticket events (.computed envelope) s →
      vectorAt s vector →
      SourceFundingInput.execute assigned scope bootstrap s.driver (vector,.step (.finish target)) = (next,.accepted) →
      next.source = some after →
      Trace assigned scope bootstrap capacity base target ticket
        (events ++ [.source s.ordinal (vector,.step (.finish target))]) (.finished envelope) (onSource s next after)

def semanticEvent : Event p a → List (CohortPhase.Event p a)
  | .source _ request => [.source request]
  | .owner i command => [.owner i command]
  | .query _ _ => []

def cursorPhase : Phase → Option JointWorkCursor.Phase
  | .entered => some .entered
  | .reserved => some .reserved
  | .probed => some .probed
  | .computed e => some (.computed e)
  | _ => none

theorem driver_invariant
    (initial : PublicationLifecycle.Invariant assigned scope bootstrap base.driver)
    (trace : Trace assigned scope bootstrap capacity base target ticket events phase s) :
    PublicationLifecycle.Invariant assigned scope bootstrap s.driver := by
  induction trace with
  | start => exact initial
  | @enter vector next entered clear bound ran present dispatched =>
    have preserved := SourceFundingInput.preserves initial (value:=(vector,.step (.enter target)))
    simpa [ran,onSource] using preserved
  | query prior ih => exact ih
  | reserve prior ran ih => exact ih
  | probe prior ran ih => exact ih
  | compute prior ran t c r ih => exact ih
  | @finish events envelope s vector next after prior bound ran present ih =>
    have preserved := SourceFundingInput.preserves ih (value:=(vector,.step (.finish target)))
    simpa [ran,onSource] using preserved

theorem source_coupled
    (initial : base.driver.source = some base.joint.actual.source)
    (trace : Trace assigned scope bootstrap capacity base target ticket events phase s) :
    s.driver.source = some s.joint.actual.source := by
  induction trace with
  | start => exact initial
  | enter clear bound ran present dispatched => exact present
  | query prior ih => exact ih
  | reserve prior ran ih => exact ih
  | probe prior ran ih => exact ih
  | compute prior ran t c r ih => exact ih
  | finish prior bound ran present ih => exact present

-- Semantic cursor is DERIVED from the consumed full-driver event trace.
-- Query erasure is explicit and cannot change or fabricate an owner/source edge.
theorem cursor_from_trace
    (valid : PublicationLifecycle.Invariant assigned scope bootstrap base.driver)
    (initial : base.driver.source = some base.joint.actual.source)
    (trace : Trace assigned scope bootstrap capacity base target ticket events phase s)
    (inside : cursorPhase phase = some cursor) :
    JointWorkCursor.Cursor (fun i => ⟨assigned.realm,i⟩) scope capacity base.joint target ticket
      (base.joint.actual.source.publication.barrier.published+1)
      (events.flatMap semanticEvent) cursor s.joint := by
  induction trace generalizing cursor with
  | start => cases inside
  | enter clear bound ran present dispatched =>
    cases inside
    obtain ⟨actual,presentAt,semantic⟩ := accepted_semantic valid initial ran
    have same := Option.some.inj (presentAt.symm.trans present)
    subst actual
    exact .enter clear semantic dispatched
  | query prior ih =>
    simpa [semanticEvent] using ih inside
  | reserve prior ran ih =>
    cases inside
    simpa [semanticEvent,onOwner] using JointWorkCursor.Cursor.reserve (ih rfl) ran
  | probe prior ran ih =>
    cases inside
    simpa [semanticEvent,onOwner] using JointWorkCursor.Cursor.probe (ih rfl) ran
  | compute prior ran t c r ih =>
    cases inside
    simpa [semanticEvent,onOwner] using JointWorkCursor.Cursor.compute (ih rfl) ran t c r
  | finish prior bound ran present ih => cases inside

-- The final closed Joint path is obtained ONLY for a trace ending in an
-- actual consumed funded finish event. Enabledness alone cannot inhabit it.
theorem finished_history {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scope budget : Nat} {capacity : Fin p → Nat} {bootstrap : SourceInput.Bootstrap a}
    {base s : State p a} {target : Fin p} {ticket : OwnerOccurrence.Ticket}
    {events : List (Event p a)} {envelope : OwnerReceipt.Envelope} {seed : QualifiedCustody.State p a}
    (history : PublicJointHistory.Runs (fun i => ⟨assigned.realm,i⟩) scope capacity budget seed base.joint)
    (valid : PublicationLifecycle.Invariant assigned scope bootstrap base.driver)
    (initial : base.driver.source = some base.joint.actual.source)
    (trace : Trace assigned scope bootstrap capacity base target ticket events (.finished envelope) s) :
    PublicJointHistory.Runs (fun i => ⟨assigned.realm,i⟩) scope capacity budget seed s.joint := by
  cases trace with
  | finish prior bound ran present =>
    obtain ⟨after,presentAt,semantic⟩ := accepted_semantic (driver_invariant valid prior) (source_coupled initial prior) ran
    have same := Option.some.inj (presentAt.symm.trans present)
    subst after
    exact JointWorkCursor.finish_closes history (cursor_from_trace valid initial prior rfl) semantic

theorem finished_has_occurrence {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scope : Nat} {capacity : Fin p → Nat} {bootstrap : SourceInput.Bootstrap a}
    {base s : State p a} {target : Fin p} {ticket : OwnerOccurrence.Ticket}
    {events : List (Event p a)} {envelope : OwnerReceipt.Envelope}
    (trace : Trace assigned scope bootstrap capacity base target ticket events (.finished envelope) s) :
    ∃ priorEvents ordinal vector, events = priorEvents ++ [.source ordinal (vector,.step (.finish target))] := by
  cases trace with
  | finish => exact ⟨_,_,_,rfl⟩

theorem no_finish_before
    (trace : Trace assigned scope bootstrap capacity base target ticket events phase s)
    (unfinished : ∀ e, phase ≠ .finished e) :
    ∀ ordinal vector, Event.source ordinal (vector,.step (.finish target)) ∉ events := by
  induction trace with
  | start => simp
  | enter => simp
  | query prior ih => simpa using ih (by intro e; intro bad; cases bad)
  | reserve prior ran ih => simpa using ih (by intro e; intro bad; cases bad)
  | probe prior ran ih => simpa using ih (by intro e; intro bad; cases bad)
  | compute prior ran t c r ih => simpa using ih (by intro e; intro bad; cases bad)
  | finish => exact False.elim (unfinished _ rfl)

theorem computed_no_finish
    (trace : Trace assigned scope bootstrap capacity base target ticket events (.computed envelope) s) :
    ∀ ordinal vector, Event.source ordinal (vector,.step (.finish target)) ∉ events :=
  no_finish_before trace (by intro e; intro bad; cases bad)

#print axioms accepted_semantic
#print axioms driver_invariant
#print axioms source_coupled
#print axioms cursor_from_trace
#print axioms finished_history
#print axioms finished_has_occurrence
#print axioms computed_no_finish
end MirroreaProofFirst.WorkOccurrence
