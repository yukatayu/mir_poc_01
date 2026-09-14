import MirroreaProofFirstPublicationCapacity

namespace MirroreaProofFirst.PublicationCapacityDriver
open OwnerCodecTree

-- Private outcome and successful-transition quota. Refused wire requests do
-- not consume a reserved source transition slot. This is deliberately NOT a
-- bound on request count, CPU time or network traffic. The native consumer must
-- read to EOF rather than impose the old unconditional 512-iteration loop.
inductive Status where
  | accepted | semanticRefused | profileRefused
  deriving DecidableEq, Repr

structure State (p a : Nat) where
  source : Option (PublicationInput.State p a)
  remaining : Nat
  suffix : List (PublicationInput.Command p a)

def initial (capacity : Nat) : State p a := ⟨none,capacity,[]⟩

def startsRound : PublicationInput.Input p a → Bool
  | .step (.stage _) | .step (.arrival _) => true
  | _ => false

def sameCommand (left right : PublicationInput.Command p a) : Bool :=
  decide (OwnerPacketCodec.encode (PublicationInput.command p a) left =
    OwnerPacketCodec.encode (PublicationInput.command p a) right)

theorem sameCommand_exact {p a : Nat} {left right : PublicationInput.Command p a} :
    sameCommand left right = true ↔ left = right := by
  constructor
  · intro checked
    have encoded := of_decide_eq_true checked
    have decoded := congrArg (OwnerPacketCodec.decode (PublicationInput.command p a)) encoded
    simpa only [OwnerPacketCodec.roundtrip,Option.some.injEq] using decoded
  · intro equal; subst right; simp [sameCommand]

-- Consume the matching planned command even if independent freeze/ack entries
-- arrive in another order. The whole resulting suffix is checked again, so
-- removal alone never certifies commutativity or physical acknowledgement.
def nextSuffix (old : State p a) (input : PublicationInput.Input p a)
    (candidate : PublicationInput.State p a) : List (PublicationInput.Command p a) :=
  if startsRound input && old.suffix.isEmpty then
    PublicationCapacity.completion p candidate.publication.barrier.announced
  else match input with
    | .step command => old.suffix.eraseP (sameCommand command)
    | .launch _ => old.suffix

def adoptionFuel : PublicationInput.Input p a → Option Nat
  | .launch program | .step (.stage (.replace program)) | .step (.stage (.continueWith program)) =>
      some program.items.length
  | _ => none

-- Reject an oversized ordinary local prefix BEFORE adopting it. Existing
-- source semantics permit replacement after failure, not arbitrary replacement
-- of a ready residual. Refusal must not strand an already adopted local write.
def schedule (realm scopeId : Nat) (old : State p a) (input : PublicationInput.Input p a)
    (candidate : PublicationInput.State p a) : Option (List (PublicationInput.Command p a)) := do
  let base := nextSuffix old input candidate
  match adoptionFuel input with
  | none => return base
  | some fuel =>
    let completed ← PublicationCapacity.checkBoundPath realm scopeId candidate base
    let (localSteps,_) ← PublicationCapacity.planPrefix realm scopeId (fuel+1) completed
    return base ++ localSteps

def passes (realm scopeId remaining : Nat) (candidate : PublicationInput.State p a)
    (suffix : List (PublicationInput.Command p a)) : Bool :=
  decide (suffix.length ≤ remaining) &&
    (PublicationCapacity.checkBoundPath realm scopeId candidate suffix).isSome

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
      state.suffix.length ≤ state.remaining ∧
      ∃ next, PublicationCapacity.BoundSequence assigned.realm scopeId source state.suffix next

theorem initial_invariant {p a : Nat} (assigned : SourceInput.Assignment p a)
    (scopeId : Nat) (seed : SourceInput.Bootstrap a) (capacity : Nat) :
    Invariant assigned scopeId seed (initial (p:=p) (a:=a) capacity) := by
  simp [Invariant,initial,PublicationInput.Invariant]

theorem passes_exact : passes realm scopeId remaining candidate suffix = true ↔
    suffix.length ≤ remaining ∧
    ∃ next, PublicationCapacity.BoundSequence realm scopeId candidate suffix next := by
  simp only [passes,Bool.and_eq_true,decide_eq_true_eq]
  cases checked : PublicationCapacity.checkBoundPath realm scopeId candidate suffix with
  | none => simp [← PublicationCapacity.checkBoundPath_exact,checked]
  | some next => simp [← PublicationCapacity.checkBoundPath_exact,checked]

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

-- Relative completeness: an actual semantic successor with a bounded,
-- assignment-bound remaining completion is admitted. This quantifies over
-- arbitrary source states/programs/commands; finite positive controls follow.
theorem accepted_of_completion {p a : Nat} {old : State p a}
    {input : PublicationInput.Input p a} {candidate next : PublicationInput.State p a}
    {assigned : SourceInput.Assignment p a} {seed : SourceInput.Bootstrap a}
    (budget : old.remaining = remaining+1)
    (step : PublicationInput.transition assigned scopeId seed old.source input = (some candidate,true))
    {suffix : List (PublicationInput.Command p a)}
    (planned : schedule assigned.realm scopeId old input candidate = some suffix)
    (space : suffix.length ≤ remaining)
    (path : PublicationCapacity.BoundSequence assigned.realm scopeId candidate suffix next) :
    transition assigned scopeId seed old input =
      (⟨some candidate,remaining,suffix⟩,.accepted) := by
  have checked := passes_exact.mpr (And.intro space (Exists.intro next path))
  simp [transition,budget,step,planned,checked]

theorem retained_completion {p a : Nat} {state : State p a} {source : PublicationInput.State p a}
    {assigned : SourceInput.Assignment p a} {seed : SourceInput.Bootstrap a}
    (valid : Invariant assigned scopeId seed state) (present : state.source = some source) :
    state.suffix.length ≤ state.remaining ∧
    ∃ next, PublicationCapacity.BoundSequence assigned.realm scopeId source state.suffix next :=
  valid.2 source present

theorem adoption_schedule_suspends {p a : Nat} {old : State p a}
    {input : PublicationInput.Input p a} {candidate : PublicationInput.State p a}
    {commands : List (PublicationInput.Command p a)}
    (adoption : adoptionFuel input = some fuel)
    (scheduled : schedule realm scopeId old input candidate = some commands) :
    ∃ final, PublicationCapacity.BoundSequence realm scopeId candidate commands final ∧
      PublicationCapacity.suspended final = true := by
  cases base : PublicationCapacity.checkBoundPath realm scopeId candidate (nextSuffix old input candidate) with
  | none => simp [schedule,adoption,base] at scheduled
  | some completed =>
    cases localPlan : PublicationCapacity.planPrefix realm scopeId (fuel+1) completed with
    | none => simp [schedule,adoption,base,localPlan] at scheduled
    | some pair =>
      obtain ⟨localSteps,final⟩ := pair
      have same : nextSuffix old input candidate ++ localSteps = commands := by
        simpa [schedule,adoption,base,localPlan] using scheduled
      obtain ⟨path,stopped⟩ := PublicationCapacity.planPrefix_sound localPlan
      subst commands
      exact ⟨final,PublicationCapacity.boundSequence_append (PublicationCapacity.checkBoundPath_exact.mp base) path,stopped⟩

-- The next scheduled command has an executable successor and remains admitted
-- with the reserved quota. Wire retries cannot spend this quota. Actual owner
-- acknowledgements/terminal outcomes are still required to supply these inputs.
theorem reserved_head_admitted {p a : Nat} {old : State p a}
    {assigned : SourceInput.Assignment p a} {seed : SourceInput.Bootstrap a}
    {source : PublicationInput.State p a} {command : PublicationInput.Command p a}
    {rest : List (PublicationInput.Command p a)}
    (valid : Invariant assigned scopeId seed old) (present : old.source = some source)
    (pending : old.suffix = command::rest)
    (notAdoption : adoptionFuel (.step command) = none) :
    (transition assigned scopeId seed old (.step command)).2 = .accepted := by
  obtain ⟨room,last,path⟩ := valid.2 source present
  rw [pending] at path room
  cases path with
  | cons fit executed tail =>
    rename_i middle
    cases budget : old.remaining with
    | zero => simp [budget] at room
    | succ remaining =>
      have space : rest.length ≤ remaining := by simpa [budget] using room
      have planned : schedule assigned.realm scopeId old (.step command) middle = some rest := by
        simp [schedule,notAdoption,nextSuffix,pending,sameCommand]
      have semantic : PublicationInput.transition assigned scopeId seed old.source (.step command) = (some middle,true) := by
        simp [PublicationInput.transition,present,executed]
      have accepted := accepted_of_completion budget semantic planned space tail
      rw [accepted]

-- A scheduled head already has a retained certificate. Avoid recomputing its
-- entire future path. All off-path inputs still use the full checker.
def fastCandidate (scopeId : Nat) (old : State p a) (input : PublicationInput.Input p a) : Option (State p a) :=
  match old.source,input,old.suffix,old.remaining with
  | some source,.step command,expected::rest,remaining+1 =>
    if sameCommand command expected && (adoptionFuel (.step command)).isNone then
      (PublicationInput.execute scopeId source command).map (fun next => ⟨some next,remaining,rest⟩)
    else none
  | _,_,_,_ => none

theorem scheduled_successor {p a : Nat} {old : State p a}
    {assigned : SourceInput.Assignment p a} {seed : SourceInput.Bootstrap a}
    {source next : PublicationInput.State p a} {command : PublicationInput.Command p a}
    {rest : List (PublicationInput.Command p a)}
    (valid : Invariant assigned scopeId seed old) (present : old.source = some source)
    (pending : old.suffix = command::rest) (budget : old.remaining = remaining+1)
    (notAdoption : adoptionFuel (.step command) = none)
    (executed : PublicationInput.execute scopeId source command = some next) :
    transition assigned scopeId seed old (.step command) = (⟨some next,remaining,rest⟩,.accepted) := by
  obtain ⟨room,last,path⟩ := valid.2 source present
  rw [pending] at room path
  cases path with
  | cons fit ran tail =>
    rw [executed] at ran
    cases ran
    have space : rest.length ≤ remaining := by simpa [budget] using room
    have planned : schedule assigned.realm scopeId old (.step command) next = some rest := by
      simp [schedule,notAdoption,nextSuffix,pending,sameCommand]
    exact accepted_of_completion budget (by simp [PublicationInput.transition,present,executed]) planned space tail

theorem fastCandidate_correct {p a : Nat} {old next : State p a}
    {assigned : SourceInput.Assignment p a} {seed : SourceInput.Bootstrap a}
    {input : PublicationInput.Input p a}
    (valid : Invariant assigned scopeId seed old)
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
            cases notAdoption : adoptionFuel (.step expected) with
            | some _ => simp [fastCandidate,present,pending,budget,sameCommand,notAdoption] at made
            | none =>
              cases executed : PublicationInput.execute scopeId source expected with
              | none => simp [fastCandidate,present,pending,budget,sameCommand,notAdoption,executed] at made
              | some actual =>
                have equal : (⟨some actual,remaining,rest⟩ : State p a) = next := by
                  simpa [fastCandidate,present,pending,budget,sameCommand,notAdoption,executed] using made
                subst next
                exact scheduled_successor valid present pending budget notAdoption executed
          · simp [fastCandidate,present,pending,budget,same] at made

def transitionFast (assigned : SourceInput.Assignment p a) (scopeId : Nat)
    (seed : SourceInput.Bootstrap a) (old : State p a) (input : PublicationInput.Input p a) : State p a × Status :=
  match fastCandidate scopeId old input with
  | some next => (next,.accepted)
  | none => transition assigned scopeId seed old input

theorem transitionFast_exact {p a : Nat} {old : State p a}
    {assigned : SourceInput.Assignment p a} {seed : SourceInput.Bootstrap a}
    {input : PublicationInput.Input p a}
    (valid : Invariant assigned scopeId seed old) :
    transitionFast assigned scopeId seed old input = transition assigned scopeId seed old input := by
  cases made : fastCandidate scopeId old input with
  | none => simp [transitionFast,made]
  | some next => simp [transitionFast,made,fastCandidate_correct valid made]

theorem transitionFast_preserves {p a : Nat} {old : State p a}
    {assigned : SourceInput.Assignment p a} {seed : SourceInput.Bootstrap a}
    {input : PublicationInput.Input p a}
    (valid : Invariant assigned scopeId seed old) :
    Invariant assigned scopeId seed (transitionFast assigned scopeId seed old input).1 := by
  rw [transitionFast_exact valid]
  exact transition_preserves assigned scopeId seed old input valid

def statusCodec : Codec Status where
  encode := fun status => .natural (match status with
    | .accepted => 0 | .semanticRefused => 1 | .profileRefused => 2)
  decode := fun tree => match tree with
    | .natural 0 => some .accepted | .natural 1 => some .semanticRefused
    | .natural 2 => some .profileRefused | _ => none
  roundtrip := by intro status; cases status <;> rfl
  canonical := by intro tree value decoded; split at decoded <;> cases decoded <;> rfl

def reply (p a : Nat) := product statusCodec (optional (SourcePublicationWorker.output p a))

theorem reply_roundtrip (response : Status × Option (SourcePublicationWorker.PrivateOutput p a)) :
    OwnerPacketCodec.decode (reply p a) (OwnerPacketCodec.encode (reply p a) response) = some response :=
  OwnerPacketCodec.roundtrip _ _

-- The three explicit status tags are smaller than the legacy true/accepted
-- wrapper whose exact byte length was checked by PublicationCapacity. This is
-- for arbitrary output values, not a fixed example or assumed size inequality.
theorem status_reply_smaller {p a : Nat} (status : Status)
    (output : Option (SourcePublicationWorker.PrivateOutput p a)) :
    (OwnerPacketCodec.encode (reply p a) (status,output)).length ≤
      (OwnerPacketCodec.encode (SourcePublicationWorker.reply p a) (true,output)).length := by
  cases status <;>
    simp [reply,statusCodec,SourcePublicationWorker.reply,OwnerPacketCodec.encode,
      OwnerTreeBytes.encode,OwnerCodecTree.product,OwnerCodecTree.boolean,OwnerCodecTree.iso,
      OwnerCodecTree.sum,OwnerCodecTree.unit,OwnerTreeBytes.writeTree,OwnerTreeBytes.writeTrees,
      OwnerByteCodec.writeNat]

theorem bounded_reply {p a : Nat} {state : State p a}
    {assigned : SourceInput.Assignment p a} {scopeId : Nat} {seed : SourceInput.Bootstrap a}
    (valid : Invariant assigned scopeId seed state) (status : Status) :
    (OwnerPacketCodec.encode (reply p a) (status,state.source.map SourcePublicationWorker.project)).length ≤ 65536 := by
  apply Nat.le_trans (status_reply_smaller status _)
  cases present : state.source with
  | none =>
    simp [SourcePublicationWorker.reply,OwnerPacketCodec.encode,OwnerTreeBytes.encode,
      OwnerCodecTree.product,OwnerCodecTree.boolean,OwnerCodecTree.optional,OwnerCodecTree.iso,
      OwnerCodecTree.sum,OwnerCodecTree.unit,OwnerTreeBytes.writeTree,OwnerTreeBytes.writeTrees,
      OwnerByteCodec.writeNat]
  | some source =>
    obtain ⟨_,next,path⟩ := valid.2 source present
    have fits := (PublicationCapacity.boundSequence_first path).1.1.2
    simpa [present,PublicationCapacity.ReplyFits] using fits

-- The private recipient profile is part of the retained sequence, including
-- prospective held replies. Byte length alone did not cover scope2^128.
theorem retained_payload_fits {p a : Nat} {state : State p a}
    {assigned : SourceInput.Assignment p a} {scopeId : Nat} {seed : SourceInput.Bootstrap a}
    (valid : Invariant assigned scopeId seed state) :
    PublicationReadability.PayloadFits (state.source.map SourcePublicationWorker.project) := by
  cases present : state.source with
  | none => simpa [present] using (PublicationReadability.none_fits (p:=p) (a:=a))
  | some source =>
    obtain ⟨_,final,path⟩ := valid.2 source present
    simpa [present] using PublicationCapacity.state_payload_fits
      (PublicationCapacity.boundSequence_first path).1

theorem readable_reply {p a : Nat} {state : State p a}
    {assigned : SourceInput.Assignment p a} {scopeId : Nat} {seed : SourceInput.Bootstrap a}
    (valid : Invariant assigned scopeId seed state) (status : Status) :
    SourceCodec.compactFits (OwnerPacketCodec.encode (reply p a)
      (status,state.source.map SourcePublicationWorker.project)) = true ∧
    OwnerPacketCodec.decodeAt (reply p a) 256
      (OwnerPacketCodec.encode (reply p a) (status,state.source.map SourcePublicationWorker.project)) =
        some (status,state.source.map SourcePublicationWorker.project) := by
  let output := state.source.map SourcePublicationWorker.project
  let tag := match status with | .accepted => 0 | .semanticRefused => 1 | .profileRefused => 2
  have small : tag < 3 := by cases status <;> decide
  have fit : PublicationReadability.PayloadFits output := retained_payload_fits valid
  have readable := PublicationReadability.payload_readable fit small
  have sameTree : (reply p a).encode (status,output) =
      (PublicationReadability.wire p a).encode (tag,output) := by cases status <;> rfl
  have sameBytes : OwnerPacketCodec.encode (reply p a) (status,output) =
      OwnerPacketCodec.encode (PublicationReadability.wire p a) (tag,output) := by
    simp only [OwnerPacketCodec.encode,sameTree]
  constructor
  · unfold SourceCodec.compactFits
    simp only [Bool.and_eq_true,decide_eq_true_eq]
    constructor
    · refine ⟨bounded_reply valid status,?_⟩
      change OwnerPayload.digitCheck 128 0 (OwnerPacketCodec.encode (reply p a) (status,output)) = true
      rw [sameBytes]
      exact (OwnerPayload.digits_exact _ _ _).mpr readable.1
    · change OwnerPayload.textCheck 4096 (OwnerPacketCodec.encode (reply p a) (status,output)) = true
      rw [sameBytes]
      exact (OwnerPayload.text_exact _ _).mpr readable.2.1
  · apply OwnerPacketCodec.bounded_roundtrip
    change OwnerTreeBytes.treeCost ((reply p a).encode (status,output)) ≤ 256
    rw [sameTree]
    exact Nat.le_trans (PublicationReadability.wrapper_cost _ _) fit.2.2

#print axioms retained_payload_fits
#print axioms readable_reply

#print axioms sameCommand_exact
#print axioms scheduled_successor
#print axioms fastCandidate_correct
#print axioms transitionFast_exact
#print axioms transitionFast_preserves
#print axioms adoption_schedule_suspends
#print axioms reserved_head_admitted
#print axioms initial_invariant
#print axioms passes_exact
#print axioms transition_preserves
#print axioms refusal_frames_state
#print axioms accepted_refines
#print axioms accepted_of_completion
#print axioms retained_completion
#print axioms statusCodec
#print axioms reply_roundtrip
#print axioms status_reply_smaller
#print axioms bounded_reply
theorem retained_dispatch_profile (valid : Invariant assigned scopeId seed state)
    (present : state.source = some source) (dispatched : source.dispatch = some request) :
    OwnerResponseProfile.ResponseFits request.context.scopeId request.context.revision request.ticket := by
  obtain ⟨_,next,path⟩ := valid.2 source present
  exact PublicationCapacity.state_dispatch_fits (PublicationCapacity.boundSequence_first path).1 request dispatched

#print axioms retained_dispatch_profile
end MirroreaProofFirst.PublicationCapacityDriver
