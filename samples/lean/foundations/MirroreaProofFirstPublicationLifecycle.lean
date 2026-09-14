import MirroreaProofFirstPublicationCapacityDriver

namespace MirroreaProofFirst.PublicationLifecycle
open PublicationCapacity
open PublicationCapacityDriver (State Status initial sameCommand sameCommand_exact)

-- Completion of private resource obligations is distinct from completion of
-- the user's program. A waiting/failed/retired activation may be parked, but
-- an active ready residual and any held interval must retain further credits.
def stopped (s : PublicationInput.State p a) : Bool :=
  let privateState := s.publication.current
  let session := privateState.privateState.session
  decide (session.status ≠ .ready) || session.remaining.isEmpty ||
    !QualifiedCustody.activeCheck privateState.stamp session

def Settled (s : PublicationInput.State p a) : Prop :=
  s.publication.barrier.announced = s.publication.barrier.published ∧
  (∀ i, s.publication.barrier.installed i = s.publication.barrier.published) ∧
  (∀ i, s.publication.held i = none)

def settledCheck (s : PublicationInput.State p a) : Bool :=
  decide (s.publication.barrier.announced = s.publication.barrier.published) &&
  (List.finRange p).all (fun i =>
    decide (s.publication.barrier.installed i = s.publication.barrier.published) &&
    (s.publication.held i).isNone)

theorem settled_exact : settledCheck s = true ↔ Settled s := by
  simp [settledCheck,Settled,List.all_eq_true,forall_and]

def Quiet (s : PublicationInput.State p a) : Prop := Settled s ∧ stopped s = true
def quietCheck (s : PublicationInput.State p a) : Bool := settledCheck s && stopped s

theorem quiet_exact : quietCheck s = true ↔ Quiet s := by
  simp [quietCheck,Quiet,settled_exact]

-- Predict release only as a future obligation. A physical consumer still needs
-- an actual matching owner terminal event before issuing the finish input.
def releasePlan (s : PublicationInput.State p a) : List (PublicationInput.Command p a) :=
  (List.finRange p).filterMap (fun i => if (s.publication.held i).isSome then some (.finish i) else none)

def settlePlan (s : PublicationInput.State p a) : List (PublicationInput.Command p a) :=
  releasePlan s ++
    if s.publication.barrier.published < s.publication.barrier.announced then
      completion p s.publication.barrier.announced
    else (List.finRange p).filterMap (fun i =>
      if s.publication.barrier.installed i ≠ s.publication.barrier.published then
        some (.install i s.publication.barrier.published) else none)

-- Recompute a finite local prefix from the actual resulting state. In
-- particular an actual arrival determines values before this prediction, and
-- legitimate authority controls can replace an obsolete predicted prefix.
def localPrefix (realm scopeId : Nat) : Nat → PublicationInput.State p a →
    Option (List (PublicationInput.Command p a) × PublicationInput.State p a)
  | fuel,s =>
    if boundFits realm scopeId s then
      if stopped s then some ([],s) else
        match fuel with
        | 0 => none
        | n+1 => do
          let staged ← PublicationInput.execute scopeId s (.stage .tick)
          let commands := PublicationInput.Command.stage SourceInput.Command.tick ::
            completion p staged.publication.barrier.announced
          let next ← checkBoundPath realm scopeId s commands
          let (tail,final) ← localPrefix realm scopeId n next
          return (commands ++ tail,final)
    else none

theorem localPrefix_sound {p a : Nat} {s final : PublicationInput.State p a}
    {commands : List (PublicationInput.Command p a)}
    (computed : localPrefix realm scopeId fuel s = some (commands,final)) :
    BoundSequence realm scopeId s commands final ∧ stopped final = true := by
  induction fuel generalizing s commands final with
  | zero =>
    simp only [localPrefix] at computed
    split at computed
    · rename_i fits
      split at computed
      · rename_i stopped
        cases computed
        exact ⟨.nil (boundFits_exact.mp fits),stopped⟩
      · cases computed
    · cases computed
  | succ fuel ih =>
    simp only [localPrefix] at computed
    split at computed
    · rename_i fits
      split at computed
      · rename_i stopped
        cases computed
        exact ⟨.nil (boundFits_exact.mp fits),stopped⟩
      · cases staged : PublicationInput.execute scopeId s (.stage .tick) with
        | none => simp [staged] at computed
        | some middle =>
          cases first : checkBoundPath realm scopeId s
              (.stage .tick :: completion p middle.publication.barrier.announced) with
          | none => simp [staged,first] at computed
          | some after =>
            cases rest : localPrefix realm scopeId fuel after with
            | none => simp [staged,first,rest] at computed
            | some pair =>
              obtain ⟨tail,last⟩ := pair
              have equal : (PublicationInput.Command.stage SourceInput.Command.tick ::
                  completion p middle.publication.barrier.announced) ++ tail = commands ∧ last = final := by
                simpa [staged,first,rest] using computed
              obtain ⟨path,stopped⟩ := ih rest
              rcases equal with ⟨rfl,rfl⟩
              exact ⟨boundSequence_append (checkBoundPath_exact.mp first) path,stopped⟩
    · cases computed

def plan (realm scopeId : Nat) (candidate : PublicationInput.State p a) :
    Option (List (PublicationInput.Command p a)) := do
  let firstSteps := settlePlan candidate
  let settled ← checkBoundPath realm scopeId candidate firstSteps
  let fuel := settled.publication.current.privateState.session.remaining.length + 1
  let (rest,final) ← localPrefix realm scopeId fuel settled
  if quietCheck final then some (firstSteps ++ rest) else none

def scheduledTail (old : State p a) : PublicationInput.Input p a →
    Option (List (PublicationInput.Command p a))
  | .step command => match old.suffix with
    | expected :: rest => if sameCommand command expected then some rest else none
    | [] => none
  | .launch _ => none

-- Remove only one occurrence of the command actually performed. This is a
-- proposed residual, not a commutativity assumption: recheck every remaining
-- step against the actual successor, including the Quiet endpoint.
def eraseCommand (command : PublicationInput.Command p a) :
    List (PublicationInput.Command p a) → List (PublicationInput.Command p a)
  | [] => []
  | expected :: rest => if sameCommand command expected then rest
      else expected :: eraseCommand command rest

-- If an independently admitted enter added a held interval, retain completed
-- round notifications and prepend only the necessary release obligations.
-- Every step is rechecked; a planned release is never a physical terminal event.
def releasedPlan (realm scopeId : Nat) (candidate : PublicationInput.State p a)
    (rest : List (PublicationInput.Command p a)) : Option (List (PublicationInput.Command p a)) :=
  let commands := releasePlan candidate ++ rest
  match checkBoundPath realm scopeId candidate commands with
  | some final => if quietCheck final then some commands else plan realm scopeId candidate
  | none => plan realm scopeId candidate

def residualPlan (realm scopeId : Nat) (old : State p a)
    (input : PublicationInput.Input p a) (candidate : PublicationInput.State p a) :
    Option (List (PublicationInput.Command p a)) :=
  match input with
  | .launch _ => plan realm scopeId candidate
  | .step command =>
    let rest := eraseCommand command old.suffix
    match checkBoundPath realm scopeId candidate rest with
    | some final => if quietCheck final then some rest else releasedPlan realm scopeId candidate rest
    | none => releasedPlan realm scopeId candidate rest

def schedule (realm scopeId : Nat) (old : State p a) (input : PublicationInput.Input p a)
    (candidate : PublicationInput.State p a) : Option (List (PublicationInput.Command p a)) :=
  match scheduledTail old input with
  | some rest => some rest
  | none => residualPlan realm scopeId old input candidate

def Certified (realm scopeId : Nat) (source : PublicationInput.State p a)
    (commands : List (PublicationInput.Command p a)) : Prop :=
  ∃ final, BoundSequence realm scopeId source commands final ∧ Quiet final

theorem plan_certified (computed : plan realm scopeId source = some commands) :
    Certified realm scopeId source commands := by
  unfold plan at computed
  cases first : checkBoundPath realm scopeId source (settlePlan source) with
  | none => simp [first] at computed
  | some settled =>
    cases rest : localPrefix realm scopeId
        (settled.publication.current.privateState.session.remaining.length + 1) settled with
    | none => simp [first,rest] at computed
    | some pair =>
      obtain ⟨tail,final⟩ := pair
      by_cases quiet : quietCheck final = true
      · have equal : settlePlan source ++ tail = commands := by
          simpa [first,rest,quiet] using computed
        rw [← equal]
        exact ⟨final,boundSequence_append (checkBoundPath_exact.mp first)
          (localPrefix_sound rest).1,quiet_exact.mp quiet⟩
      · simp [first,rest,quiet] at computed

def checkSuffix (realm scopeId : Nat) (source : PublicationInput.State p a)
    (commands : List (PublicationInput.Command p a)) : Bool :=
  match checkBoundPath realm scopeId source commands with
  | none => false
  | some final => quietCheck final

theorem checkSuffix_exact : checkSuffix realm scopeId source commands = true ↔
    Certified realm scopeId source commands := by
  unfold checkSuffix Certified
  cases checked : checkBoundPath realm scopeId source commands with
  | none => simp [← checkBoundPath_exact,checked]
  | some final => simp [← checkBoundPath_exact,checked,quiet_exact]

theorem releasedPlan_certified
    (computed : releasedPlan realm scopeId source rest = some commands) :
    Certified realm scopeId source commands := by
  unfold releasedPlan at computed
  cases ran : checkBoundPath realm scopeId source (releasePlan source ++ rest) with
  | none => exact plan_certified (by simpa [ran] using computed)
  | some final =>
      by_cases quiet : quietCheck final = true
      · have equal : releasePlan source ++ rest = commands := by simpa [ran,quiet] using computed
        rw [← equal]
        exact ⟨final,checkBoundPath_exact.mp ran,quiet_exact.mp quiet⟩
      · exact plan_certified (by simpa [ran,quiet] using computed)

theorem residualPlan_certified
    (computed : residualPlan realm scopeId old input source = some commands) :
    Certified realm scopeId source commands := by
  cases input with
  | launch program => exact plan_certified computed
  | step command =>
    unfold residualPlan at computed
    cases ran : checkBoundPath realm scopeId source (eraseCommand command old.suffix) with
    | none => exact releasedPlan_certified (by simpa [ran] using computed)
    | some final =>
      by_cases quiet : quietCheck final = true
      · have equal : eraseCommand command old.suffix = commands := by simpa [ran,quiet] using computed
        rw [← equal]
        exact ⟨final,checkBoundPath_exact.mp ran,quiet_exact.mp quiet⟩
      · exact releasedPlan_certified (by simpa [ran,quiet] using computed)

-- Relative completeness for a feasible residual starts from declarative
-- execution and Quiet, without assuming success of schedule or residualPlan.
theorem schedule_of_residual
    (certified : Certified realm scopeId candidate (eraseCommand command old.suffix)) :
    schedule realm scopeId old (.step command) candidate = some (eraseCommand command old.suffix) := by
  obtain ⟨final,path,quiet⟩ := certified
  have ran := checkBoundPath_exact.mpr path
  have stopped := quiet_exact.mpr quiet
  have residual : residualPlan realm scopeId old (.step command) candidate =
      some (eraseCommand command old.suffix) := by simp [residualPlan,ran,stopped]
  cases pending : old.suffix with
  | nil => simpa [schedule,scheduledTail,pending] using residual
  | cons expected rest =>
    by_cases same : sameCommand command expected = true
    · simp [schedule,scheduledTail,eraseCommand,pending,same]
    · simpa [schedule,scheduledTail,pending,same] using residual

def passes (realm scopeId remaining : Nat) (source : PublicationInput.State p a)
    (commands : List (PublicationInput.Command p a)) : Bool :=
  decide (commands.length ≤ remaining) && checkSuffix realm scopeId source commands

theorem passes_exact : passes realm scopeId remaining source commands = true ↔
    commands.length ≤ remaining ∧ Certified realm scopeId source commands := by
  simp [passes,checkSuffix_exact]

def transition (assigned : SourceInput.Assignment p a) (scopeId : Nat)
    (seed : SourceInput.Bootstrap a) (old : State p a) (input : PublicationInput.Input p a) : State p a × Status :=
  match old.remaining with
  | 0 => (old,.profileRefused)
  | remaining+1 =>
    let (candidate,accepted) := PublicationInput.transition assigned scopeId seed old.source input
    if accepted then
      match candidate with
      | none => (old,.profileRefused)
      | some source =>
        match schedule assigned.realm scopeId old input source with
        | none => (old,.profileRefused)
        | some suffix =>
          if passes assigned.realm scopeId remaining source suffix then
            (⟨some source,remaining,suffix⟩,.accepted)
          else (old,.profileRefused)
    else (old,.semanticRefused)

def Invariant (assigned : SourceInput.Assignment p a) (scopeId : Nat)
    (seed : SourceInput.Bootstrap a) (state : State p a) : Prop :=
  PublicationInput.Invariant assigned scopeId seed state.source ∧
    ∀ source, state.source = some source →
      state.suffix.length ≤ state.remaining ∧ Certified assigned.realm scopeId source state.suffix

theorem initial_invariant (assigned : SourceInput.Assignment p a)
    (scopeId : Nat) (seed : SourceInput.Bootstrap a) (capacity : Nat) :
    Invariant assigned scopeId seed (initial (p:=p) (a:=a) capacity) := by
  simp [Invariant,initial,PublicationInput.Invariant]

theorem transition_preserves {p a : Nat} (assigned : SourceInput.Assignment p a)
    (scopeId : Nat) (seed : SourceInput.Bootstrap a) (old : State p a)
    (input : PublicationInput.Input p a) (valid : Invariant assigned scopeId seed old) :
    Invariant assigned scopeId seed (transition assigned scopeId seed old input).1 := by
  cases budget : old.remaining with
  | zero => simpa [transition,budget] using valid
  | succ remaining =>
    cases step : PublicationInput.transition assigned scopeId seed old.source input with
    | mk candidate accepted =>
      cases accepted with
      | false => simpa [transition,budget,step] using valid
      | true =>
        cases candidate with
        | none => simpa [transition,budget,step] using valid
        | some source =>
          cases planned : schedule assigned.realm scopeId old input source with
          | none => simpa [transition,budget,step,planned] using valid
          | some suffix =>
            by_cases checked : passes assigned.realm scopeId remaining source suffix = true
            · have semantic := PublicationInput.transition_preserves valid.1 (entry:=input)
              rw [step] at semantic
              simp only [transition,budget,step,ite_true,planned,checked]
              refine ⟨semantic,?_⟩
              intro actual equal
              cases equal
              exact passes_exact.mp checked
            · simpa [transition,budget,step,planned,checked] using valid

theorem refusal_frames_state {p a : Nat} (assigned : SourceInput.Assignment p a)
    (scopeId : Nat) (seed : SourceInput.Bootstrap a) (old : State p a)
    (input : PublicationInput.Input p a)
    (refused : (transition assigned scopeId seed old input).2 ≠ .accepted) :
    (transition assigned scopeId seed old input).1 = old := by
  cases budget : old.remaining with
  | zero => simp [transition,budget]
  | succ remaining =>
    cases step : PublicationInput.transition assigned scopeId seed old.source input with
    | mk candidate accepted =>
      cases accepted with
      | false => simp [transition,budget,step]
      | true =>
        cases candidate with
        | none => simp [transition,budget,step]
        | some source =>
          cases planned : schedule assigned.realm scopeId old input source with
          | none => simp [transition,budget,step,planned]
          | some suffix =>
            by_cases checked : passes assigned.realm scopeId remaining source suffix = true
            · simp [transition,budget,step,planned,checked] at refused
            · simp [transition,budget,step,planned,checked]

theorem accepted_refines {p a : Nat} (assigned : SourceInput.Assignment p a)
    (scopeId : Nat) (seed : SourceInput.Bootstrap a) (old : State p a)
    (input : PublicationInput.Input p a)
    (accepted : (transition assigned scopeId seed old input).2 = .accepted) :
    PublicationInput.transition assigned scopeId seed old.source input =
      ((transition assigned scopeId seed old input).1.source,true) ∧
    (transition assigned scopeId seed old input).1.remaining + 1 = old.remaining := by
  cases budget : old.remaining with
  | zero => simp [transition,budget] at accepted
  | succ remaining =>
    cases step : PublicationInput.transition assigned scopeId seed old.source input with
    | mk candidate ok =>
      cases ok with
      | false => simp [transition,budget,step] at accepted
      | true =>
        cases candidate with
        | none => simp [transition,budget,step] at accepted
        | some source =>
          cases planned : schedule assigned.realm scopeId old input source with
          | none => simp [transition,budget,step,planned] at accepted
          | some suffix =>
            by_cases checked : passes assigned.realm scopeId remaining source suffix = true
            · simp [transition,budget,step,planned,checked]
            · simp [transition,budget,step,planned,checked] at accepted

-- Relative admission starts from an actual semantic successor and a concrete
-- declarative completion ending Quiet; it is not merely preservation on acceptance.
theorem accepted_of_completion
    (budget : old.remaining = remaining+1)
    (step : PublicationInput.transition assigned scopeId seed old.source input = (some candidate,true))
    (planned : schedule assigned.realm scopeId old input candidate = some suffix)
    (space : suffix.length ≤ remaining)
    (certified : Certified assigned.realm scopeId candidate suffix) :
    transition assigned scopeId seed old input =
      (⟨some candidate,remaining,suffix⟩,.accepted) := by
  have checked := passes_exact.mpr ⟨space,certified⟩
  simp [transition,budget,step,planned,checked]

theorem residual_admitted
    (budget : old.remaining = remaining+1)
    (step : PublicationInput.transition assigned scopeId seed old.source (.step command) = (some candidate,true))
    (space : (eraseCommand command old.suffix).length ≤ remaining)
    (certified : Certified assigned.realm scopeId candidate (eraseCommand command old.suffix)) :
    transition assigned scopeId seed old (.step command) =
      (⟨some candidate,remaining,eraseCommand command old.suffix⟩,.accepted) :=
  accepted_of_completion budget step (schedule_of_residual certified) space certified

theorem scheduled_successor (valid : Invariant assigned scopeId seed old)
    (present : old.source = some source) (pending : old.suffix = command :: rest)
    (budget : old.remaining = remaining+1)
    (executed : PublicationInput.execute scopeId source command = some next) :
    transition assigned scopeId seed old (.step command) = (⟨some next,remaining,rest⟩,.accepted) := by
  obtain ⟨room,final,path,quiet⟩ := valid.2 source present
  rw [pending] at room path
  cases path with
  | cons fit ran tail =>
    rw [executed] at ran
    cases ran
    have space : rest.length ≤ remaining := by simpa [budget] using room
    have planned : schedule assigned.realm scopeId old (.step command) next = some rest := by
      simp [schedule,scheduledTail,pending,sameCommand]
    exact accepted_of_completion budget (by simp [PublicationInput.transition,present,executed])
      planned space ⟨final,tail,quiet⟩

theorem reserved_head_admitted (valid : Invariant assigned scopeId seed old)
    (present : old.source = some source) (pending : old.suffix = command :: rest) :
    ∃ next, transition assigned scopeId seed old (.step command) = (next,.accepted) := by
  obtain ⟨room,final,path,quiet⟩ := valid.2 source present
  rw [pending] at room path
  cases path with
  | cons fit executed tail =>
    cases budget : old.remaining with
    | zero => simp [budget] at room
    | succ remaining => exact ⟨_,scheduled_successor valid present pending budget executed⟩

def fastCandidate (scopeId : Nat) (old : State p a) (input : PublicationInput.Input p a) : Option (State p a) :=
  match old.source,input,old.suffix,old.remaining with
  | some source,.step command,expected::rest,remaining+1 =>
    if sameCommand command expected then
      (PublicationInput.execute scopeId source command).map (fun next => ⟨some next,remaining,rest⟩)
    else none
  | _,_,_,_ => none

theorem fastCandidate_correct (valid : Invariant assigned scopeId seed old)
    (made : fastCandidate scopeId old input = some next) :
    transition assigned scopeId seed old input = (next,.accepted) := by
  cases present : old.source with
  | none => simp [fastCandidate,present] at made
  | some source =>
    cases input with
    | launch program => simp [fastCandidate,present] at made
    | step command =>
      cases pending : old.suffix with
      | nil => simp [fastCandidate,present,pending] at made
      | cons expected rest =>
        cases budget : old.remaining with
        | zero => simp [fastCandidate,present,pending,budget] at made
        | succ remaining =>
          by_cases same : sameCommand command expected = true
          · have equal := sameCommand_exact.mp same
            subst command
            cases executed : PublicationInput.execute scopeId source expected with
            | none => simp [fastCandidate,present,pending,budget,sameCommand,executed] at made
            | some actual =>
              have equal : (⟨some actual,remaining,rest⟩ : State _ _) = next := by
                simpa [fastCandidate,present,pending,budget,sameCommand,executed] using made
              subst next
              exact scheduled_successor valid present pending budget executed
          · simp [fastCandidate,present,pending,budget,same] at made

-- A newly produced plan has already checked its complete BoundSequence and
-- Quiet endpoint. Rechecking that same sequence cannot add admission evidence.
-- Retained/off-order tails still need their usual check against the successor.
def checkedSchedule (realm scopeId remaining : Nat) (old : State p a)
    (input : PublicationInput.Input p a) (source : PublicationInput.State p a) :
    Option (List (PublicationInput.Command p a)) :=
  match scheduledTail old input with
  | some suffix => if passes realm scopeId remaining source suffix then some suffix else none
  | none => do
    let suffix ← residualPlan realm scopeId old input source
    if suffix.length ≤ remaining then some suffix else none

theorem checkedSchedule_exact : checkedSchedule realm scopeId remaining old input source =
    (schedule realm scopeId old input source >>= fun suffix =>
      if passes realm scopeId remaining source suffix then some suffix else none) := by
  cases tail : scheduledTail old input with
  | some suffix => simp [checkedSchedule,schedule,tail]
  | none =>
    cases planned : residualPlan realm scopeId old input source with
    | none => simp [checkedSchedule,schedule,tail,planned]
    | some suffix =>
      have certified := residualPlan_certified planned
      have checked : checkSuffix realm scopeId source suffix = true := checkSuffix_exact.mpr certified
      simp [checkedSchedule,schedule,tail,planned,passes,checked]

def transitionPlanned (assigned : SourceInput.Assignment p a) (scopeId : Nat)
    (seed : SourceInput.Bootstrap a) (old : State p a) (input : PublicationInput.Input p a) : State p a × Status :=
  match old.remaining with
  | 0 => (old,.profileRefused)
  | remaining+1 =>
    let (candidate,accepted) := PublicationInput.transition assigned scopeId seed old.source input
    if accepted then
      match candidate with
      | none => (old,.profileRefused)
      | some source =>
        match checkedSchedule assigned.realm scopeId remaining old input source with
        | none => (old,.profileRefused)
        | some suffix => (⟨some source,remaining,suffix⟩,.accepted)
    else (old,.semanticRefused)

-- This equality needs no invariant premise: the only skipped work is a
-- duplicate check of the plan which was just constructed and certified.
theorem transitionPlanned_exact : transitionPlanned assigned scopeId seed old input =
    transition assigned scopeId seed old input := by
  cases budget : old.remaining with
  | zero => simp [transitionPlanned,transition,budget]
  | succ remaining =>
    cases step : PublicationInput.transition assigned scopeId seed old.source input with
    | mk candidate accepted =>
      cases accepted with
      | false => simp [transitionPlanned,transition,budget,step]
      | true =>
        cases candidate with
        | none => simp [transitionPlanned,transition,budget,step]
        | some source =>
          rw [transitionPlanned,transition,budget,step]
          simp only [ite_true,checkedSchedule_exact]
          cases planned : schedule assigned.realm scopeId old input source with
          | none => simp
          | some suffix =>
            by_cases checked : passes assigned.realm scopeId remaining source suffix = true
            · simp [checked]
            · simp [checked]

#print axioms checkedSchedule_exact
#print axioms transitionPlanned_exact

def transitionFast (assigned : SourceInput.Assignment p a) (scopeId : Nat)
    (seed : SourceInput.Bootstrap a) (old : State p a) (input : PublicationInput.Input p a) : State p a × Status :=
  match fastCandidate scopeId old input with
  | some next => (next,.accepted)
  | none => transitionPlanned assigned scopeId seed old input

theorem transitionFast_exact (valid : Invariant assigned scopeId seed old) :
    transitionFast assigned scopeId seed old input = transition assigned scopeId seed old input := by
  cases made : fastCandidate scopeId old input with
  | none => simp [transitionFast,made,transitionPlanned_exact]
  | some next => simp [transitionFast,made,fastCandidate_correct valid made]

theorem transitionFast_preserves (valid : Invariant assigned scopeId seed old) :
    Invariant assigned scopeId seed (transitionFast assigned scopeId seed old input).1 := by
  rw [transitionFast_exact valid]
  exact transition_preserves assigned scopeId seed old input valid

theorem erases (valid : Invariant assigned scopeId seed state) :
    PublicationCapacityDriver.Invariant assigned scopeId seed state := by
  refine ⟨valid.1,?_⟩
  intro source present
  obtain ⟨space,final,path,_⟩ := valid.2 source present
  exact ⟨space,final,path⟩

theorem bounded_reply {p a : Nat} {assigned : SourceInput.Assignment p a} {state : State p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a}
    (valid : Invariant assigned scopeId seed state) (status : Status) :
    (OwnerPacketCodec.encode (PublicationCapacityDriver.reply p a)
      (status,state.source.map SourcePublicationWorker.project)).length ≤ 65536 :=
  PublicationCapacityDriver.bounded_reply (erases valid) status

theorem exhausted_quiet (valid : Invariant assigned scopeId seed state)
    (present : state.source = some source) (exhausted : state.remaining = 0) : Quiet source := by
  obtain ⟨space,final,path,quiet⟩ := valid.2 source present
  have empty : state.suffix = [] := by
    have zero : state.suffix.length = 0 := Nat.eq_zero_of_le_zero (by simpa [exhausted] using space)
    simpa using zero
  rw [empty] at path
  cases path
  exact quiet

theorem exhausted_no_held (valid : Invariant assigned scopeId seed state)
    (present : state.source = some source) (exhausted : state.remaining = 0) :
    ∀ i, source.publication.held i = none :=
  (exhausted_quiet valid present exhausted).1.2.2

theorem exhausted_active_ready_finished (valid : Invariant assigned scopeId seed state)
    (present : state.source = some source) (exhausted : state.remaining = 0)
    (ready : source.publication.current.privateState.session.status = .ready)
    (active : QualifiedCustody.activeCheck source.publication.current.stamp
      source.publication.current.privateState.session = true) :
    source.publication.current.privateState.session.remaining = [] := by
  have stop := (exhausted_quiet valid present exhausted).2
  simpa [stopped,ready,active] using stop

theorem readable_reply {p a : Nat} {assigned : SourceInput.Assignment p a} {state : State p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a}
    (valid : Invariant assigned scopeId seed state) (status : Status) :
    SourceCodec.compactFits (OwnerPacketCodec.encode (PublicationCapacityDriver.reply p a)
      (status,state.source.map SourcePublicationWorker.project)) = true ∧
    OwnerPacketCodec.decodeAt (PublicationCapacityDriver.reply p a) 256
      (OwnerPacketCodec.encode (PublicationCapacityDriver.reply p a)
        (status,state.source.map SourcePublicationWorker.project)) =
      some (status,state.source.map SourcePublicationWorker.project) :=
  PublicationCapacityDriver.readable_reply (erases valid) status

theorem residualPlan_released_bound
    (certified : Certified realm scopeId candidate (releasePlan candidate ++ eraseCommand command old.suffix)) :
    ∃ commands, residualPlan realm scopeId old (.step command) candidate = some commands ∧
      commands.length ≤ (releasePlan candidate ++ eraseCommand command old.suffix).length ∧
      Certified realm scopeId candidate commands := by
  obtain ⟨final,path,quiet⟩ := certified
  have made : releasedPlan realm scopeId candidate (eraseCommand command old.suffix) =
      some (releasePlan candidate ++ eraseCommand command old.suffix) := by
    simp [releasedPlan,checkBoundPath_exact.mpr path,quiet_exact.mpr quiet]
  cases ran : checkBoundPath realm scopeId candidate (eraseCommand command old.suffix) with
  | none =>
      refine ⟨_,?_,Nat.le_refl _,final,path,quiet⟩
      simpa [residualPlan,ran] using made
  | some finish =>
      by_cases stopped : quietCheck finish = true
      · refine ⟨eraseCommand command old.suffix,?_,?_,finish,checkBoundPath_exact.mp ran,quiet_exact.mp stopped⟩
        · simp [residualPlan,ran,stopped]
        · simp only [List.length_append]; omega
      · refine ⟨_,?_,Nat.le_refl _,final,path,quiet⟩
        simpa [residualPlan,ran,stopped] using made

-- Arbitrary finite participant sets and all rooted driver states. The
-- independent declarative paths include real semantic steps and profile
-- bounds at every state; no schedule/checker success is a premise. This also
-- covers a newly entered owner while another participant already froze.
theorem released_path_admitted
    (valid : Invariant assigned scopeId seed old)
    (budget : old.remaining = remaining+1)
    (step : PublicationInput.transition assigned scopeId seed old.source (.step command) = (some candidate,true))
    (release : BoundSequence assigned.realm scopeId candidate (releasePlan candidate) middle)
    (tail : BoundSequence assigned.realm scopeId middle (eraseCommand command old.suffix) final)
    (quiet : Quiet final)
    (space : (releasePlan candidate ++ eraseCommand command old.suffix).length ≤ remaining) :
    ∃ next, transition assigned scopeId seed old (.step command) = (next,.accepted) := by
  have certified : Certified assigned.realm scopeId candidate
      (releasePlan candidate ++ eraseCommand command old.suffix) :=
    ⟨final,boundSequence_append release tail,quiet⟩
  have fallback (offhead : scheduledTail old (.step command) = none) :
      ∃ next, transition assigned scopeId seed old (.step command) = (next,.accepted) := by
    obtain ⟨commands,planned,length,certified⟩ := residualPlan_released_bound certified
    exact ⟨_,accepted_of_completion budget step (by simpa [schedule,offhead] using planned)
      (Nat.le_trans length space) certified⟩
  cases pending : old.suffix with
  | nil => exact fallback (by simp [scheduledTail,pending])
  | cons expected rest =>
      by_cases same : sameCommand command expected = true
      · have equal := sameCommand_exact.mp same
        subst command
        cases present : old.source with
        | none => simp [PublicationInput.transition,present] at step
        | some source => exact reserved_head_admitted valid present pending
      · exact fallback (by simp [scheduledTail,pending,same])

#print axioms releasedPlan_certified
#print axioms residualPlan_released_bound
#print axioms released_path_admitted
#print axioms residualPlan_certified
#print axioms schedule_of_residual
#print axioms residual_admitted
#print axioms readable_reply

#print axioms transition_preserves
#print axioms localPrefix_sound
#print axioms plan_certified
#print axioms scheduled_successor
#print axioms reserved_head_admitted
#print axioms fastCandidate_correct
#print axioms transitionFast_exact
#print axioms transitionFast_preserves
#print axioms erases
#print axioms bounded_reply
#print axioms refusal_frames_state
#print axioms accepted_refines
#print axioms accepted_of_completion
#print axioms settled_exact
#print axioms quiet_exact
#print axioms checkSuffix_exact
#print axioms passes_exact
#print axioms initial_invariant
#print axioms exhausted_quiet
#print axioms exhausted_no_held
#print axioms exhausted_active_ready_finished
end MirroreaProofFirst.PublicationLifecycle
