import MirroreaProofFirstSharedNativeStep
namespace MirroreaProofFirst.SharedJointDriver

-- Joint history or one unfinished work prefix. Native state is retained by
-- the selected case; a work origin is historical context, not another current
-- native state. Transport and local host commit state remain outside this cut.
inductive Running (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (budget : Nat) (seed : QualifiedCustody.State p a) where
  | closed (state : JointDriver.Closed assigned scope bootstrap capacity budget seed)
  | work (origin : JointDriver.Closed assigned scope bootstrap capacity budget seed)
      (target : Fin p) (ticket : OwnerOccurrence.Ticket) (events : List (WorkOccurrence.Event p a))
      (checked : WorkOccurrenceReplay.Checked assigned scope bootstrap capacity origin.actual target ticket events)

variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope budget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}

def actual : Running assigned scope bootstrap capacity budget seed → WorkOccurrence.State p a
  | .closed s => s.actual
  | .work _ _ _ _ checked => checked.state

def native (s : Running assigned scope bootstrap capacity budget seed) : SharedNativeStep.State p a :=
  SharedNativeStep.fromWork (actual s)

structure Next (s : Running assigned scope bootstrap capacity budget seed) (event : WorkOccurrence.Event p a) where
  val : Running assigned scope bootstrap capacity budget seed
  physics : native val = SharedNativeStep.execute assigned scope bootstrap capacity (native s) event

def workEvent {target : Fin p} {ticket : OwnerOccurrence.Ticket} {events : List (WorkOccurrence.Event p a)}
    (origin : JointDriver.Closed assigned scope bootstrap capacity budget seed)
    (checked : WorkOccurrenceReplay.Checked assigned scope bootstrap capacity origin.actual target ticket events)
    (event : WorkOccurrence.Event p a) :
    Option (Next (.work origin target ticket events checked) event) :=
  match WorkOccurrenceReplay.advance checked event with
  | none => none
  | some next => some ⟨.work origin target ticket (events ++ [event]) next,
      SharedNativeStep.work_extension checked.path next.path⟩

def inputEvent (ordinal : Nat) : SourceFundingQuery.CheckedInput p a → WorkOccurrence.Event p a
  | .inl request => .query ordinal (.inl request)
  | .inr (.inl request) => .query ordinal (.inr request)
  | .inr (.inr request) => .source ordinal request

def source (s : Running assigned scope bootstrap capacity budget seed) (input : SourceFundingQuery.CheckedInput p a) :
    Option (Next s (inputEvent (actual s).ordinal input)) := by
  cases s with
  | work origin target ticket events checked => exact workEvent origin checked _
  | closed closed =>
    cases input with
    | inl request => exact some ⟨.closed (JointDriver.query closed),SharedNativeStep.closed_query (.inl request)⟩
    | inr input =>
      cases input with
      | inl request => exact some ⟨.closed (JointDriver.query closed),SharedNativeStep.closed_query (.inr request)⟩
      | inr request =>
        obtain ⟨vector,input⟩ := request
        cases input with
        | launch program =>
          match checked : JointDriver.refused closed vector (.launch program) with
          | none => exact none
          | some next => exact some ⟨.closed next,SharedNativeStep.closed_refused checked⟩
        | step command =>
          match command with
          | .enter target =>
            if (SourceFundingInput.execute assigned scope bootstrap closed.actual.driver (vector,.step (.enter target))).2 = .accepted then
              match closed.actual.joint.actual.source.publication.current.privateState.session.state.source.waiting with
              | none => exact none
              | some saved =>
                match workEvent closed (WorkOccurrenceReplay.initial assigned scope bootstrap capacity closed.actual target saved.entry.ticket)
                    (.source closed.actual.ordinal (vector,.step (.enter target))) with
                | none => exact none
                | some next => exact some ⟨next.val,next.physics⟩
            else
              match checked : JointDriver.source closed vector (.enter target) with
              | none => exact none
              | some next => exact some ⟨.closed next,SharedNativeStep.closed_source checked⟩
          | other =>
            match checked : JointDriver.source closed vector other with
            | none => exact none
            | some next => exact some ⟨.closed next,SharedNativeStep.closed_source checked⟩

def owner (s : Running assigned scope bootstrap capacity budget seed) (target : Fin p)
    (request : OwnerEndpoint.Command p a) (headPayment : Bool) : Option (Next s (.owner target request)) := by
  cases s with
  | closed closed =>
    match checked : JointDriver.owner closed target request headPayment with
    | none => exact none
    | some next => exact some ⟨.closed next,SharedNativeStep.closed_owner checked⟩
  | work origin target ticket events checked => exact workEvent origin checked _

-- Local work closure changes proof/history metadata after the consumed finish;
-- it does not execute a second native transition or charge a second payment.
def finish (s : Running assigned scope bootstrap capacity budget seed) :
    Option {next : Running assigned scope bootstrap capacity budget seed // native next = native s} := by
  cases s with
  | closed closed => exact some ⟨.closed closed,rfl⟩
  | work origin target ticket events checked =>
    obtain ⟨phase,state,path⟩ := checked
    cases phase with
    | finished envelope => exact some ⟨.closed (JointDriver.closeWork origin path),rfl⟩
    | _ => exact none

theorem driver_invariant (s : Running assigned scope bootstrap capacity budget seed) :
    PublicationLifecycle.Invariant assigned scope bootstrap (actual s).driver := by
  cases s with
  | closed closed => exact closed.valid
  | work origin target ticket events checked => exact WorkOccurrence.driver_invariant origin.valid checked.path

#print axioms workEvent
#print axioms source
#print axioms owner
#print axioms finish
#print axioms driver_invariant
end MirroreaProofFirst.SharedJointDriver
