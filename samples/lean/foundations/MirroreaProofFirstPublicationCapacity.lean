import MirroreaProofFirstPublicationReadability
import OwnerEndpointCodec
import MirroreaProofFirstOwnerResponseProfile

namespace MirroreaProofFirst.PublicationCapacity

-- Reversible finite executor profile. Its certificate is a pure prediction;
-- no physical freeze/ack/installation or private commit happens in preflight.
-- The checked path bounds publisher replies, prospective held replies and
-- endpoint install decoding/image limits. Actual CPU/physical ownership, owner
-- command quotas and authenticated delivery remain separate obligations.
def ReplyFits (s : PublicationInput.State p a) : Prop :=
  (OwnerPacketCodec.encode (SourcePublicationWorker.reply p a)
    (false,some (SourcePublicationWorker.project s))).length ≤ 65536 ∧
  (OwnerPacketCodec.encode (SourcePublicationWorker.reply p a)
    (true,some (SourcePublicationWorker.project s))).length ≤ 65536

theorem boolean_reply_smaller (output : Option (SourcePublicationWorker.PrivateOutput p a)) :
    (OwnerPacketCodec.encode (SourcePublicationWorker.reply p a) (false,output)).length ≤
    (OwnerPacketCodec.encode (SourcePublicationWorker.reply p a) (true,output)).length := by
  simp [SourcePublicationWorker.reply,OwnerPacketCodec.encode,OwnerTreeBytes.encode,
    OwnerCodecTree.product,OwnerCodecTree.boolean,OwnerCodecTree.iso,OwnerCodecTree.sum,
    OwnerCodecTree.unit,OwnerTreeBytes.writeTree,OwnerTreeBytes.writeTrees,OwnerByteCodec.writeNat]

-- The true wrapper bounds both legacy status alternatives generally; encode
-- the retained private projection once, not twice at every path state.
def replyFits (s : PublicationInput.State p a) : Bool :=
  decide ((OwnerPacketCodec.encode (SourcePublicationWorker.reply p a)
    (true,some (SourcePublicationWorker.project s))).length ≤ 65536)

theorem replyFits_exact : replyFits s = true ↔ ReplyFits s := by
  constructor
  · intro checked
    have bound := of_decide_eq_true checked
    exact ⟨Nat.le_trans (boolean_reply_smaller _) bound,bound⟩
  · intro fit
    exact decide_eq_true fit.2

-- Match the existing native decoder preflight, including its actual codec fuel.
def inputFits (codec : OwnerCodecTree.Codec α) (textLimit : Nat) (value : α) : Bool :=
  let bytes := OwnerPacketCodec.encode codec value
  decide (bytes.length ≤ 65536) && OwnerPayload.digitCheck 128 0 bytes &&
    OwnerPayload.textCheck textLimit bytes && (OwnerPacketCodec.decodeAt codec 256 bytes).isSome

-- For every fixed-cohort endpoint, test the actual outgoing install shape and
-- its existing bounded image admission; the publisher does not mint authority.
def endpointFits (s : PublicationInput.State p a) : Bool :=
  let image := CustodyPublication.image s.publication.current
  (List.finRange p).all (fun endpoint =>
    OwnerReservationWorker.imageCheck ⟨image.state.realm,endpoint⟩ image &&
    inputFits (OwnerEndpointWorker.command p a) 256
      (.owner (.install s.publication.barrier.announced image)) &&
    inputFits (OwnerEndpointWorker.command p a) 256 (.owner (.initialize image)))

def DispatchFits (s : PublicationInput.State p a) : Prop :=
  ∀ dispatched, s.dispatch = some dispatched →
    OwnerResponseProfile.ResponseFits dispatched.context.scopeId dispatched.context.revision dispatched.ticket

def dispatchFits (s : PublicationInput.State p a) : Bool :=
  match s.dispatch with
  | none => true
  | some dispatched => OwnerResponseProfile.responseCheck dispatched.context.scopeId dispatched.context.revision dispatched.ticket

theorem dispatchFits_exact : dispatchFits s = true ↔ DispatchFits s := by
  cases present : s.dispatch <;> simp [dispatchFits,DispatchFits,present,OwnerResponseProfile.responseCheck_exact]

-- A future held interval adds another image to the private response. Check
-- each possible admitted enter, without executing it in the actual process.
def heldFits (scopeId : Nat) (s : PublicationInput.State p a) : Bool :=
  (List.finRange p).all (fun endpoint =>
    match PublicationInput.execute scopeId s (.enter endpoint) with
    | none => true
    | some entered => replyFits entered && PublicationReadability.payloadCheck (some (SourcePublicationWorker.project entered)) && dispatchFits entered)

def StateFits (scopeId : Nat) (s : PublicationInput.State p a) : Prop :=
  ReplyFits s ∧ endpointFits s = true ∧ heldFits scopeId s = true ∧
    PublicationReadability.PayloadFits (some (SourcePublicationWorker.project s)) ∧ DispatchFits s

def stateFits (scopeId : Nat) (s : PublicationInput.State p a) : Bool :=
  replyFits s && endpointFits s && heldFits scopeId s &&
    PublicationReadability.payloadCheck (some (SourcePublicationWorker.project s)) && dispatchFits s

theorem stateFits_exact : stateFits scopeId s = true ↔ StateFits scopeId s := by
  simp [stateFits,StateFits,replyFits_exact,PublicationReadability.payloadCheck_exact,dispatchFits_exact,and_assoc]

theorem stateFits_reply (fit : StateFits scopeId s) : ReplyFits s := fit.1

theorem endpoint_install_fits {p a : Nat} {s : PublicationInput.State p a}
    (fit : StateFits scopeId s) (endpoint : Fin p) :
    let image := CustodyPublication.image s.publication.current
    OwnerReservationWorker.imageCheck ⟨image.state.realm,endpoint⟩ image = true ∧
    inputFits (OwnerEndpointWorker.command p a) 256
      (.owner (.install s.publication.barrier.announced image)) = true := by
  have all := fit.2.1
  simp only [endpointFits,List.all_eq_true] at all
  have selected := all endpoint (List.mem_finRange endpoint)
  simp only [Bool.and_eq_true] at selected
  exact selected.1

theorem endpoint_initialize_fits {p a : Nat} {s : PublicationInput.State p a}
    (fit : StateFits scopeId s) (endpoint : Fin p) :
    OwnerResponseProfile.WireFits (OwnerEndpointWorker.command p a) 256
      (.owner (.initialize (CustodyPublication.image s.publication.current))) := by
  have all := fit.2.1
  simp only [endpointFits,List.all_eq_true] at all
  have selected := all endpoint (List.mem_finRange endpoint)
  simp only [Bool.and_eq_true] at selected
  exact OwnerResponseProfile.wireCheck_exact.mp selected.2

theorem held_reply_fits {p a : Nat} {s entered : PublicationInput.State p a} {endpoint : Fin p}
    (fit : StateFits scopeId s)
    (enter : PublicationInput.execute scopeId s (.enter endpoint) = some entered) :
    ReplyFits entered := by
  have all := fit.2.2.1
  simp only [heldFits,List.all_eq_true] at all
  have selected := all endpoint (List.mem_finRange endpoint)
  rw [enter] at selected
  simp only [Bool.and_eq_true] at selected
  exact replyFits_exact.mp selected.1.1

theorem state_payload_fits (fit : StateFits scopeId s) :
    PublicationReadability.PayloadFits (some (SourcePublicationWorker.project s)) := fit.2.2.2.1

theorem held_payload_fits {p a : Nat} {s entered : PublicationInput.State p a} {endpoint : Fin p}
    (fit : StateFits scopeId s)
    (enter : PublicationInput.execute scopeId s (.enter endpoint) = some entered) :
    PublicationReadability.PayloadFits (some (SourcePublicationWorker.project entered)) := by
  have all := fit.2.2.1
  simp only [heldFits,List.all_eq_true] at all
  have selected := all endpoint (List.mem_finRange endpoint)
  rw [enter] at selected
  simp only [Bool.and_eq_true] at selected
  exact PublicationReadability.payloadCheck_exact.mp selected.1.2

theorem state_dispatch_fits (fit : StateFits scopeId s) : DispatchFits s := fit.2.2.2.2

theorem held_dispatch_fits {p a : Nat} {s entered : PublicationInput.State p a} {endpoint : Fin p}
    (fit : StateFits scopeId s)
    (enter : PublicationInput.execute scopeId s (.enter endpoint) = some entered) : DispatchFits entered := by
  have all := fit.2.2.1
  simp only [heldFits,List.all_eq_true] at all
  have selected := all endpoint (List.mem_finRange endpoint)
  rw [enter] at selected
  simp only [Bool.and_eq_true] at selected
  exact dispatchFits_exact.mp selected.2

#print axioms dispatchFits_exact
#print axioms state_dispatch_fits
#print axioms held_dispatch_fits
#print axioms endpoint_initialize_fits

#print axioms state_payload_fits
#print axioms held_payload_fits

inductive Sequence (scopeId : Nat) : PublicationInput.State p a →
    List (PublicationInput.Command p a) → PublicationInput.State p a → Prop where
  | nil : StateFits scopeId s → Sequence scopeId s [] s
  | cons : StateFits scopeId s → PublicationInput.execute scopeId s command = some middle →
      Sequence scopeId middle rest next → Sequence scopeId s (command::rest) next

-- Every executed prediction checks the actual reply encodings at both ends.
-- No supplied invariant/result/accepted receive is an input to this checker.
def checkPath (scopeId : Nat) : PublicationInput.State p a →
    List (PublicationInput.Command p a) → Option (PublicationInput.State p a)
  | s,[] => if stateFits scopeId s then some s else none
  | s,command::rest => if stateFits scopeId s then
      PublicationInput.execute scopeId s command >>= fun next => checkPath scopeId next rest
    else none

theorem checkPath_exact : checkPath scopeId s commands = some next ↔
    Sequence scopeId s commands next := by
  induction commands generalizing s next with
  | nil =>
    constructor
    · intro checked
      simp only [checkPath] at checked
      split at checked
      · rename_i fit; cases checked; exact .nil (stateFits_exact.mp fit)
      · cases checked
    · intro sequence; cases sequence with
      | nil fit => simp [checkPath,stateFits_exact.mpr fit]
  | cons command rest ih =>
    constructor
    · intro checked
      simp only [checkPath] at checked
      split at checked
      · rename_i fit
        cases step : PublicationInput.execute scopeId s command with
        | none => simp [step] at checked
        | some middle => exact .cons (stateFits_exact.mp fit) step (ih.mp (by simpa [step] using checked))
      · cases checked
    · intro sequence; cases sequence with
      | cons fit step tail => simp [checkPath,stateFits_exact.mpr fit,step,ih.mpr tail]

theorem sequence_reached (path : PublicationInput.Reached scopeId source s)
    (steps : Sequence scopeId s commands next) :
    PublicationInput.Reached scopeId source next := by
  induction steps with
  | nil _ => exact path
  | cons _ step _ ih => exact ih (.step path step)

theorem sequence_last_fits (steps : Sequence scopeId s commands next) : ReplyFits next := by
  induction steps with
  | nil fit => exact fit.1
  | cons _ _ _ ih => exact ih

def completion (p revision : Nat) : List (PublicationInput.Command p a) :=
  (List.finRange p).map (fun i => .freeze i revision) ++
  (List.finRange p).map (fun i => .acknowledge i revision) ++ [.publish] ++
  (List.finRange p).map (fun i => .install i revision)

-- Staging/arrival can prepare a private successor without publishing it. Check
-- the corresponding finite completion before even announcing that stage.
def preflight (scopeId remaining : Nat) (s : PublicationInput.State p a)
    (command : PublicationInput.Command p a) : Option (PublicationInput.State p a) := do
  let staged ← PublicationInput.execute scopeId s command
  let tail := completion p staged.publication.barrier.announced
  if 1 + tail.length ≤ remaining then do
    let _ ← checkPath scopeId staged tail
    return staged
  else none

theorem preflight_sound {p a : Nat} {s staged : PublicationInput.State p a}
    {command : PublicationInput.Command p a} (checked : preflight scopeId remaining s command = some staged) :
    PublicationInput.execute scopeId s command = some staged ∧
    ∃ next, Sequence scopeId staged (completion p staged.publication.barrier.announced) next ∧
      1 + (completion (a:=a) p staged.publication.barrier.announced).length ≤ remaining := by
  cases step : PublicationInput.execute scopeId s command with
  | none => simp [preflight,step] at checked
  | some candidate =>
    by_cases room : 1 + (completion (a:=a) p candidate.publication.barrier.announced).length ≤ remaining
    · cases predicted : checkPath scopeId candidate (completion p candidate.publication.barrier.announced) with
      | none => simp [preflight,step,room,predicted] at checked
      | some next =>
        have same : candidate = staged := by simpa [preflight,step,room,predicted] using checked
        subst staged
        exact ⟨rfl,next,checkPath_exact.mp predicted,room⟩
    · simp [preflight,step,room] at checked

theorem preflight_complete {p a : Nat} {s staged next : PublicationInput.State p a}
    {command : PublicationInput.Command p a}
    (step : PublicationInput.execute scopeId s command = some staged)
    (sequence : Sequence scopeId staged (completion p staged.publication.barrier.announced) next)
    (room : 1 + (completion (a:=a) p staged.publication.barrier.announced).length ≤ remaining) :
    preflight scopeId remaining s command = some staged := by
  simp [preflight,step,room,checkPath_exact.mpr sequence]

-- Actual source/authority semantics are preserved by this stricter, reversible
-- profile. No wire/CPU deadline, arbitrary interleaving, or owner availability
-- follows from existence of this encoded source-side completion schedule.
theorem preflight_rooted {p a realm : Nat} {source : QualifiedCustody.State p a}
    {view : WorldProjection.AuthorityView a} {policy : Nat → CurrentUse.Policy}
    {identity : QualifiedSource.Actor p a}
    {s staged : PublicationInput.State p a} {command : PublicationInput.Command p a}
    (root : QualifiedCustody.Rooted realm view policy identity source)
    (path : PublicationInput.Reached scopeId source s)
    (checked : preflight scopeId remaining s command = some staged) :
    QualifiedCustody.Rooted realm view policy identity staged.publication.current ∧
    ∃ next : PublicationInput.State p a, QualifiedCustody.Rooted realm view policy identity next.publication.current ∧ ReplyFits next := by
  obtain ⟨step,next,sequence,_⟩ := preflight_sound checked
  have reached := PublicationInput.Reached.step path step
  exact ⟨PublicationInput.rooted_source root reached,next,
    PublicationInput.rooted_source root (sequence_reached reached sequence),sequence_last_fits sequence⟩

-- Bind the profile to the immutable realm actually passed to each fixed-cohort
-- endpoint. The older StateFits predicate alone used the image's own realm.
def BoundFits (realm scopeId : Nat) (s : PublicationInput.State p a) : Prop :=
  StateFits scopeId s ∧ (CustodyPublication.image s.publication.current).state.realm = realm

def boundFits (realm scopeId : Nat) (s : PublicationInput.State p a) : Bool :=
  stateFits scopeId s && decide ((CustodyPublication.image s.publication.current).state.realm = realm)

theorem boundFits_exact : boundFits realm scopeId s = true ↔ BoundFits realm scopeId s := by
  simp [boundFits,BoundFits,stateFits_exact]

inductive BoundSequence (realm scopeId : Nat) : PublicationInput.State p a →
    List (PublicationInput.Command p a) → PublicationInput.State p a → Prop where
  | nil : BoundFits realm scopeId s → BoundSequence realm scopeId s [] s
  | cons : BoundFits realm scopeId s → PublicationInput.execute scopeId s command = some middle →
      BoundSequence realm scopeId middle rest next → BoundSequence realm scopeId s (command::rest) next

def checkBoundPath (realm scopeId : Nat) : PublicationInput.State p a →
    List (PublicationInput.Command p a) → Option (PublicationInput.State p a)
  | s,[] => if boundFits realm scopeId s then some s else none
  | s,command::rest => if boundFits realm scopeId s then
      PublicationInput.execute scopeId s command >>= fun next => checkBoundPath realm scopeId next rest
    else none

theorem checkBoundPath_exact : checkBoundPath realm scopeId s commands = some next ↔
    BoundSequence realm scopeId s commands next := by
  induction commands generalizing s next with
  | nil =>
    constructor
    · intro checked
      simp only [checkBoundPath] at checked
      split at checked
      · rename_i fit; cases checked; exact .nil (boundFits_exact.mp fit)
      · cases checked
    · intro sequence; cases sequence with
      | nil fit => simp [checkBoundPath,boundFits_exact.mpr fit]
  | cons command rest ih =>
    constructor
    · intro checked
      simp only [checkBoundPath] at checked
      split at checked
      · rename_i fit
        cases step : PublicationInput.execute scopeId s command with
        | none => simp [step] at checked
        | some middle => exact .cons (boundFits_exact.mp fit) step (ih.mp (by simpa [step] using checked))
      · cases checked
    · intro sequence; cases sequence with
      | cons fit step tail => simp [checkBoundPath,boundFits_exact.mpr fit,step,ih.mpr tail]

theorem boundSequence_first (path : BoundSequence realm scopeId s commands next) : BoundFits realm scopeId s := by
  cases path with
  | nil fit => exact fit
  | cons fit _ _ => exact fit

theorem boundSequence_erases (path : BoundSequence realm scopeId s commands next) : Sequence scopeId s commands next := by
  induction path with
  | nil fit => exact .nil fit.1
  | cons fit step _ ih => exact .cons fit.1 step ih

theorem assigned_endpoint_fits {p a : Nat} {s : PublicationInput.State p a}
    (fit : BoundFits realm scopeId s) (endpoint : Fin p) :
    OwnerReservationWorker.imageCheck ⟨realm,endpoint⟩ (CustodyPublication.image s.publication.current) = true := by
  have checked := (endpoint_install_fits fit.1 endpoint).1
  simpa only [fit.2] using checked

theorem boundSequence_append (first : BoundSequence realm scopeId s left middle)
    (second : BoundSequence realm scopeId middle right next) :
    BoundSequence realm scopeId s (left ++ right) next := by
  induction first with
  | nil _ => exact second
  | cons fit step _ ih => exact .cons fit step (ih second)

def suspended (s : PublicationInput.State p a) : Bool :=
  let session := s.publication.current.privateState.session
  decide (session.status ≠ .ready) || session.remaining.isEmpty

-- Predict only this already checked finite source's local prefix. Stop at
-- actual waiting/failure/end; never manufacture an owner response or execute
-- its computation here. A caller supplies structural fuel from source size.
def planPrefix (realm scopeId : Nat) : Nat → PublicationInput.State p a →
    Option (List (PublicationInput.Command p a) × PublicationInput.State p a)
  | fuel,s =>
    if boundFits realm scopeId s then
      if suspended s then some ([],s) else
        match fuel with
        | 0 => none
        | n+1 => do
          let staged ← PublicationInput.execute scopeId s (.stage .tick)
          let commands := PublicationInput.Command.stage SourceInput.Command.tick ::
            completion p staged.publication.barrier.announced
          let next ← checkBoundPath realm scopeId s commands
          let (tail,final) ← planPrefix realm scopeId n next
          return (commands ++ tail,final)
    else none

theorem planPrefix_sound {p a : Nat} {s final : PublicationInput.State p a}
    {commands : List (PublicationInput.Command p a)}
    (computed : planPrefix realm scopeId fuel s = some (commands,final)) :
    BoundSequence realm scopeId s commands final ∧ suspended final = true := by
  induction fuel generalizing s commands final with
  | zero =>
    simp only [planPrefix] at computed
    split at computed
    · rename_i fits
      split at computed
      · rename_i stopped
        cases computed
        exact ⟨.nil (boundFits_exact.mp fits),stopped⟩
      · cases computed
    · cases computed
  | succ fuel ih =>
    simp only [planPrefix] at computed
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
            cases rest : planPrefix realm scopeId fuel after with
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

#print axioms planPrefix_sound
#print axioms boolean_reply_smaller
#print axioms boundSequence_append
#print axioms boundFits_exact
#print axioms checkBoundPath_exact
#print axioms boundSequence_first
#print axioms boundSequence_erases
#print axioms assigned_endpoint_fits
#print axioms endpoint_install_fits
#print axioms held_reply_fits
#print axioms stateFits_exact
#print axioms stateFits_reply
#print axioms replyFits_exact
#print axioms checkPath_exact
#print axioms sequence_reached
#print axioms preflight_sound
#print axioms preflight_complete
#print axioms preflight_rooted
end MirroreaProofFirst.PublicationCapacity
