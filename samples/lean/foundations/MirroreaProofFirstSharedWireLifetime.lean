import MirroreaProofFirstSharedFundedDriver
namespace MirroreaProofFirst.SharedWireLifetime

-- External LAB transport sub-relation, not a Python/OS or QUIC refinement.
-- One exclusive ordered request slot, stable deterministic endpoints, authentic
-- matching reply bytes and no replay by another writer are explicit premises.
-- Native delivery may still occur AFTER the caller has retired an unknown IO.
-- Promoting knowledge does not commit host mirrors, release the public gate,
-- acknowledge a payment or close a work interval. Those remain outer debts.
inductive Request (p a : Nat) where
  | source (input : SourceFundingQuery.CheckedInput p a)
  | owner (target : Fin p) (input : OwnerEndpoint.Command p a) (headPayment : Bool)

abbrev Certificate (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (sourceBudget ownerBudget : Nat) (seed : QualifiedCustody.State p a) :=
  SharedFundedDriver.State assigned scope bootstrap capacity sourceBudget ownerBudget seed

variable {p a : Nat} {assigned : SourceInput.Assignment p a} {scope sourceBudget ownerBudget : Nat}
  {bootstrap : SourceInput.Bootstrap a} {capacity : Fin p → Nat} {seed : QualifiedCustody.State p a}

abbrev native (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) :=
  SharedJointDriver.native s.joint

def event (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    Request p a → WorkOccurrence.Event p a
  | .source input => SharedJointDriver.inputEvent (native s).ordinal input
  | .owner target input _ => .owner target input

def prepare (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (request : Request p a) : Option (SharedFundedDriver.Next s (SharedFundedDriver.fundingEvent (event s request))) :=
  match request with
  | .source input => SharedFundedDriver.source s input
  | .owner target input headPayment => SharedFundedDriver.owner s target input headPayment

-- These bytes are computed from the SAME full native state as the certificate.
-- Equality of bytes is not authentication; endpoint/custody is still a premise.
def reply (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    Request p a → List UInt8
  | .source input => OwnerPacketCodec.encode (SourceFundingQuery.checkedReply input)
      (SourceFundingQuery.checkedExchange assigned scope bootstrap (native s).driver input).2
  | .owner target input _ => OwnerPacketCodec.encode OwnerReservationWorker.replyCodec
      (OwnerEndpointBudget.transition ⟨assigned.realm,target⟩ scope (capacity target) ((native s).owners target) input).2

structure Attempt (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (sourceBudget ownerBudget : Nat) (seed : QualifiedCustody.State p a) where
  before : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed
  request : Request p a
  next : SharedFundedDriver.Next before (SharedFundedDriver.fundingEvent (event before request))

theorem next_physics (t : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    native t.next.val = SharedNativeStep.execute assigned scope bootstrap capacity (native t.before) (event t.before t.request) := by
  rcases t with ⟨before,request,next⟩
  cases request with
  | source input =>
    cases input with
    | inl query => exact SharedNativeStep.funding_query next.step
    | inr input =>
      cases input with
      | inl query => exact SharedNativeStep.funding_query next.step
      | inr input => exact SharedNativeStep.funding_source ((SharedFundedDriver.invariant before) _ rfl).1 next.step
  | owner target input headPayment => exact SharedNativeStep.funding_owner next.step

inductive Host (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (sourceBudget ownerBudget : Nat) (seed : QualifiedCustody.State p a) where
  | settled (certificate : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
  | awaiting (attempt : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed)
  | received (attempt : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed)
  | retired (attempt : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed)

-- applied is ghost endpoint progress, NOT observable host knowledge.
structure State (assigned : SourceInput.Assignment p a) (scope : Nat)
    (bootstrap : SourceInput.Bootstrap a) (capacity : Fin p → Nat)
    (sourceBudget ownerBudget : Nat) (seed : QualifiedCustody.State p a) where
  host : Host assigned scope bootstrap capacity sourceBudget ownerBudget seed
  applied : Bool
  physical : SharedNativeStep.State p a

inductive Action (p a : Nat) where
  | send (request : Request p a)
  | deliver
  | receive (bytes : List UInt8)
  | promote
  | loseReply
  | localComplete

def initial (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    State assigned scope bootstrap capacity sourceBudget ownerBudget seed := ⟨.settled s,false,native s⟩

def execute (t : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (physical : SharedNativeStep.State p a) : SharedNativeStep.State p a :=
  SharedNativeStep.execute assigned scope bootstrap capacity physical (event t.before t.request)

-- Independent transition rules; no invariant appears as an admission guard.
inductive Step : State assigned scope bootstrap capacity sourceBudget ownerBudget seed → Action p a →
    State assigned scope bootstrap capacity sourceBudget ownerBudget seed → Prop where
  | send (checked : prepare before request = some next) :
      Step ⟨.settled before,false,physical⟩ (.send request) ⟨.awaiting ⟨before,request,next⟩,false,physical⟩
  | deliver : Step ⟨.awaiting t,false,physical⟩ .deliver ⟨.awaiting t,true,execute t physical⟩
  | lateDeliver : Step ⟨.retired t,false,physical⟩ .deliver ⟨.retired t,true,execute t physical⟩
  | receive : Step ⟨.awaiting t,true,physical⟩ (.receive (reply t.before t.request)) ⟨.received t,true,physical⟩
  | mismatch (wrong : bytes ≠ reply t.before t.request) :
      Step ⟨.awaiting t,true,physical⟩ (.receive bytes) ⟨.retired t,true,physical⟩
  | promote : Step ⟨.received t,true,physical⟩ .promote ⟨.settled t.next.val,false,physical⟩
  | lose : Step ⟨.awaiting t,applied,physical⟩ .loseReply ⟨.retired t,applied,physical⟩
  | localComplete (checked : SharedFundedDriver.finish before = some after) :
      Step ⟨.settled before,false,physical⟩ .localComplete ⟨.settled after,false,physical⟩

def advance (s : State assigned scope bootstrap capacity sourceBudget ownerBudget seed) (action : Action p a) :
    Option (State assigned scope bootstrap capacity sourceBudget ownerBudget seed) :=
  match s.host,action with
  | .settled before,.send request =>
      if s.applied then none else
        match prepare before request with
        | none => none
        | some next => some ⟨.awaiting ⟨before,request,next⟩,false,s.physical⟩
  | .settled before,.localComplete =>
      if s.applied then none else
        match SharedFundedDriver.finish before with
        | none => none
        | some after => some ⟨.settled after,false,s.physical⟩
  | .awaiting t,.deliver =>
      if s.applied then none else some ⟨.awaiting t,true,execute t s.physical⟩
  | .retired t,.deliver =>
      if s.applied then none else some ⟨.retired t,true,execute t s.physical⟩
  | .awaiting t,.receive bytes =>
      if s.applied then
        if bytes = reply t.before t.request then some ⟨.received t,true,s.physical⟩
        else some ⟨.retired t,true,s.physical⟩
      else none
  | .received t,.promote =>
      if s.applied then some ⟨.settled t.next.val,false,s.physical⟩ else none
  | .awaiting t,.loseReply => some ⟨.retired t,s.applied,s.physical⟩
  | _,_ => none

def pendingPhysical (t : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed) (applied : Bool) :=
  if applied then execute t (native t.before) else native t.before

def Invariant (s : State assigned scope bootstrap capacity sourceBudget ownerBudget seed) : Prop :=
  match s.host with
  | .settled cert => s.applied = false ∧ s.physical = native cert
  | .awaiting t | .retired t => s.physical = pendingPhysical t s.applied
  | .received t => s.applied = true ∧ s.physical = native t.next.val

theorem initial_valid (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    Invariant (initial s) := ⟨rfl,rfl⟩

theorem preserves (valid : Invariant s) (step : Step s action after) : Invariant after := by
  cases step with
  | send checked => exact valid.2
  | deliver => simpa [Invariant,pendingPhysical] using congrArg (execute _) valid
  | lateDeliver => simpa [Invariant,pendingPhysical] using congrArg (execute _) valid
  | receive =>
      refine ⟨rfl,?_⟩
      exact valid.trans (next_physics _).symm
  | mismatch wrong => exact valid
  | promote => exact ⟨rfl,valid.2⟩
  | lose => exact valid
  | localComplete checked => exact ⟨rfl,valid.2.trans (SharedFundedDriver.finish_native checked).symm⟩

inductive Runs (base : State assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    List (Action p a) → State assigned scope bootstrap capacity sourceBudget ownerBudget seed → Prop where
  | nil : Runs base [] base
  | step : Runs base actions s → Step s action next → Runs base (actions ++ [action]) next

theorem runs_preserve (valid : Invariant base) (path : Runs base actions s) : Invariant s := by
  induction path with
  | nil => exact valid
  | step prior step ih => exact preserves ih step

-- Last known knowledge is retained independently from possible native progress.
def lastKnown : Host assigned scope bootstrap capacity sourceBudget ownerBudget seed →
    Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed
  | .settled s => s
  | .awaiting t | .received t | .retired t => t.before

theorem retired_retains (t : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    lastKnown (.retired t) = t.before := rfl

theorem retired_no_send (t : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    advance ⟨.retired t,applied,physical⟩ (.send request) = none := rfl

theorem retired_no_promote (t : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    advance ⟨.retired t,applied,physical⟩ .promote = none := rfl

theorem cannot_deliver_twice (t : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed) :
    advance ⟨.awaiting t,true,physical⟩ .deliver = none ∧
    advance ⟨.retired t,true,physical⟩ .deliver = none := ⟨rfl,rfl⟩

-- An unknown response can accompany the full successor state. Claiming rollback
-- would erase the actual source ordinal (even a known refused source consumes it).
theorem retired_after_apply (t : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (valid : Invariant ⟨.retired t,true,physical⟩) : physical = native t.next.val :=
  valid.trans (next_physics t).symm

theorem source_ordinal_advances (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (input : SourceFundingQuery.CheckedInput p a) :
    (SharedNativeStep.execute assigned scope bootstrap capacity (native s) (event s (.source input))).ordinal = (native s).ordinal+1 := by
  cases input with
  | inl request => rfl
  | inr input => cases input <;> rfl

theorem source_rollback_false (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (input : SourceFundingQuery.CheckedInput p a) :
    SharedNativeStep.execute assigned scope bootstrap capacity (native s) (event s (.source input)) ≠ native s := by
  intro equal
  have ordinal := congrArg SharedNativeStep.State.ordinal equal
  rw [source_ordinal_advances] at ordinal
  omega

theorem advance_complete (step : Step s action next) : advance s action = some next := by
  cases step <;> simp_all [advance]

theorem advance_sound (checked : advance s action = some next) : Step s action next := by
  rcases s with ⟨host,applied,physical⟩
  cases host with
  | settled cert =>
    cases action with
    | send request =>
      cases applied with
      | true => simp [advance] at checked
      | false =>
        cases admitted : prepare cert request with
        | none => simp [advance,admitted] at checked
        | some after =>
          simp only [advance,Bool.false_eq_true,↓reduceIte,admitted] at checked
          cases Option.some.inj checked
          exact .send admitted
    | localComplete =>
      cases applied with
      | true => simp [advance] at checked
      | false =>
        cases finished : SharedFundedDriver.finish cert with
        | none => simp [advance,finished] at checked
        | some after =>
          simp only [advance,Bool.false_eq_true,↓reduceIte,finished] at checked
          cases Option.some.inj checked
          exact .localComplete finished
    | _ => simp [advance] at checked
  | awaiting t =>
    cases action with
    | send request => simp [advance] at checked
    | promote | localComplete => simp [advance] at checked
    | loseReply =>
      cases Option.some.inj checked
      exact .lose
    | deliver =>
      cases applied with
      | true => simp [advance] at checked
      | false =>
        cases Option.some.inj checked
        exact .deliver
    | receive bytes =>
      cases applied with
      | false => simp [advance] at checked
      | true =>
        by_cases same : bytes = reply t.before t.request
        · subst bytes
          simp only [advance,↓reduceIte] at checked
          cases Option.some.inj checked
          exact .receive
        · simp only [advance,↓reduceIte,same] at checked
          cases Option.some.inj checked
          exact .mismatch same
  | received t =>
    cases action with
    | promote =>
      cases applied with
      | false => simp [advance] at checked
      | true =>
        cases Option.some.inj checked
        exact .promote
    | _ => simp [advance] at checked
  | retired t =>
    cases action with
    | deliver =>
      cases applied with
      | true => simp [advance] at checked
      | false =>
        cases Option.some.inj checked
        exact .lateDeliver
    | _ => simp [advance] at checked

theorem advance_exact : advance s action = some next ↔ Step s action next :=
  ⟨advance_sound,advance_complete⟩

theorem advance_preserves (valid : Invariant s) (checked : advance s action = some next) :
    Invariant next := preserves valid (advance_sound checked)

-- General non-vacuity: every independently admitted concrete native operation
-- has a successful send/delivery/receipt/knowledge-promotion path. This is not
-- a promise of network fairness, endpoint availability or host-debt discharge.
theorem roundtrip
    (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (request : Request p a)
    (next : SharedFundedDriver.Next s (SharedFundedDriver.fundingEvent (event s request)))
    (admitted : prepare s request = some next) :
    Runs (initial s) [.send request,.deliver,.receive (reply s request),.promote] (initial next.val) := by
  let t : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed := ⟨s,request,next⟩
  have sent : Runs (initial s) [.send request] ⟨.awaiting t,false,native s⟩ := .step .nil (.send admitted)
  have delivered : Runs (initial s) [.send request,.deliver] ⟨.awaiting t,true,execute t (native s)⟩ := .step sent .deliver
  have received : Runs (initial s) [.send request,.deliver,.receive (reply s request)] ⟨.received t,true,execute t (native s)⟩ := .step delivered .receive
  have promoted := Runs.step received Step.promote
  have physics : execute t (native s) = native next.val := (next_physics t).symm
  simpa only [physics,initial] using promoted

structure KnownResult (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (request : Request p a) (bytes : List UInt8) where
  val : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed
  path : Runs (initial s) [.send request,.deliver,.receive bytes,.promote] (initial val)

-- Actual finite-capture consumer: reads real reply bytes, performs the admitted
-- semantic checker once, and obtains the independently established transport
-- path. A missing reply has no bytes and cannot use this function as refusal.
def known (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (request : Request p a) (bytes : List UInt8) : Option (KnownResult s request bytes) :=
  match checked : prepare s request with
  | none => none
  | some next =>
    if same : bytes = reply s request then
      some ⟨next.val,by rw [same]; exact roundtrip s request next checked⟩
    else none

theorem known_complete
    (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (request : Request p a)
    (next : SharedFundedDriver.Next s (SharedFundedDriver.fundingEvent (event s request)))
    (checked : prepare s request = some next) :
    (known s request (reply s request)).isSome = true := by
  unfold known
  split
  · rename_i failed
    rw [checked] at failed
    cases failed
  · simp

theorem known_rejects_wrong
    (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (request : Request p a) (wrong : bytes ≠ reply s request) : known s request bytes = none := by
  unfold known
  split <;> simp [wrong]

-- Completeness is chained to independent funding rules plus recognized Joint
-- operations on the same prior state, not only to a desired future reply.
theorem source_path_complete
    (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (input : SourceFundingQuery.CheckedInput p a)
    (joint : SharedJointDriver.Next s.joint (SharedJointDriver.inputEvent (SharedJointDriver.actual s.joint).ordinal input))
    (recognized : SharedJointDriver.source s.joint input = some joint)
    (step : CohortPhase.Step assigned scope bootstrap capacity (some (SharedFundedDriver.funding s))
      (SharedFundedDriver.fundingEvent (event s (.source input))) (some next)) :
    ∃ result, known s (.source input) (reply s (.source input)) = some result := by
  have complete := SharedFundedDriver.source_complete recognized step
  cases found : SharedFundedDriver.source s input with
  | none => simp [found] at complete
  | some after =>
      have checked : prepare s (.source input) = some after := found
      have yes := known_complete s (.source input) after checked
      cases got : known s (.source input) (reply s (.source input)) with
      | none => simp [got] at yes
      | some result => exact ⟨result,rfl⟩

theorem owner_path_complete
    (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (target : Fin p) (input : OwnerEndpoint.Command p a) (headPayment : Bool)
    (joint : SharedJointDriver.Next s.joint (.owner target input))
    (recognized : SharedJointDriver.owner s.joint target input headPayment = some joint)
    (step : CohortPhase.Step assigned scope bootstrap capacity (some (SharedFundedDriver.funding s))
      (.owner target input) (some next)) :
    ∃ result, known s (.owner target input headPayment) (reply s (.owner target input headPayment)) = some result := by
  have complete := SharedFundedDriver.owner_complete recognized step
  cases found : SharedFundedDriver.owner s target input headPayment with
  | none => simp [found] at complete
  | some after =>
      have checked : prepare s (.owner target input headPayment) = some after := found
      have yes := known_complete s (.owner target input headPayment) after checked
      cases got : known s (.owner target input headPayment) (reply s (.owner target input headPayment)) with
      | none => simp [got] at yes
      | some result => exact ⟨result,rfl⟩

-- Same observable retained attempt, two possible physical histories. The
-- caller's loseReply action cannot establish that delivery did not happen.
theorem unknown_paths
    (s : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed)
    (request : Request p a)
    (next : SharedFundedDriver.Next s (SharedFundedDriver.fundingEvent (event s request)))
    (admitted : prepare s request = some next) :
    let t : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed := ⟨s,request,next⟩
    Runs (initial s) [.send request,.loseReply] ⟨.retired t,false,native s⟩ ∧
    Runs (initial s) [.send request,.deliver,.loseReply] ⟨.retired t,true,native next.val⟩ ∧
    Runs (initial s) [.send request,.loseReply,.deliver] ⟨.retired t,true,native next.val⟩ := by
  dsimp only
  let t : Attempt assigned scope bootstrap capacity sourceBudget ownerBudget seed := ⟨s,request,next⟩
  have sent : Runs (initial s) [.send request] ⟨.awaiting t,false,native s⟩ := .step .nil (.send admitted)
  have lost : Runs (initial s) [.send request,.loseReply] ⟨.retired t,false,native s⟩ := .step sent .lose
  have delivered : Runs (initial s) [.send request,.deliver] ⟨.awaiting t,true,execute t (native s)⟩ := .step sent .deliver
  have physics : execute t (native s) = native next.val := (next_physics t).symm
  refine ⟨lost,?_,?_⟩
  · simpa only [physics] using (Runs.step delivered Step.lose)
  · simpa only [physics] using (Runs.step lost Step.lateDeliver)

-- Concatenate actual certified exchanges and exact metadata closures. The
-- local constructor proves no host-store/gate condition; an outer host relation
-- must bind it to discharged local commit debt and the actual return boundary.
theorem Runs.append (left : Runs base earlier middle) (right : Runs middle later after) :
    Runs base (earlier ++ later) after := by
  induction right with
  | nil => simpa using left
  | step prior step ih => simpa only [List.append_assoc] using (Runs.step ih step)

structure History (base : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) where
  current : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed
  actions : List (Action p a)
  path : Runs (initial base) actions (initial current)

def History.start (base : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed) : History base :=
  ⟨base,[],.nil⟩

variable {base : Certificate assigned scope bootstrap capacity sourceBudget ownerBudget seed}

def History.knownStep (history : History base) (request : Request p a) (bytes : List UInt8) : Option (History base) :=
  match known history.current request bytes with
  | none => none
  | some next => some ⟨next.val,history.actions ++ [.send request,.deliver,.receive bytes,.promote],
      history.path.append next.path⟩

def History.finishStep (history : History base) : Option (History base) :=
  match checked : SharedFundedDriver.finish history.current with
  | none => none
  | some next => some ⟨next,history.actions ++ [.localComplete],by
      have path : Runs (initial base) (history.actions ++ [.localComplete])
          ⟨.settled next,false,native history.current⟩ := .step history.path (.localComplete checked)
      have frame := SharedFundedDriver.finish_native checked
      simpa only [initial,←frame] using path⟩

#print axioms Runs.append
#print axioms History.knownStep
#print axioms History.finishStep
#print axioms source_path_complete
#print axioms owner_path_complete
#print axioms known_complete
#print axioms known_rejects_wrong
#print axioms advance_exact
#print axioms advance_preserves
#print axioms roundtrip
#print axioms unknown_paths
#print axioms next_physics
#print axioms initial_valid
#print axioms preserves
#print axioms runs_preserve
#print axioms retired_retains
#print axioms retired_after_apply
#print axioms cannot_deliver_twice
#print axioms source_rollback_false
end MirroreaProofFirst.SharedWireLifetime
