import MirroreaProofFirstSourceFundingInput

namespace MirroreaProofFirst.SourceFundingQuery
open PublicationCapacityDriver (State Status)
open PublicationOwnerBudget

-- Read-only privileged plan metadata for one assigned owner. A consumer can
-- query all owners under one retained source-call custody scope. This is not
-- an ordinary Mir read, owner balance, authenticated permission or public
-- observation: plan cost can depend on private source values.
def amount (state : State p a) (owner : Fin p) : Nat :=
  if state.source.isSome then demand state.suffix owner else 0

def inspect (state : State p a) (owner : Fin p) : State p a × Nat :=
  (state,amount state owner)

theorem faithful (present : state.source = some source) :
    (inspect state owner).2 = demand state.suffix owner := by
  simp [inspect,amount,present]

theorem passive_state : (inspect state owner).1 = state := rfl

theorem before_execute :
    SourceFundingInput.execute assigned scopeId seed (inspect state owner).1 value =
      SourceFundingInput.execute assigned scopeId seed state value := rfl

theorem command_cost_bound (command : PublicationInput.Command p a) (owner : Fin p) :
    commandCost command owner ≤ 3 := by
  cases command <;> simp [commandCost] <;> split <;> omega

theorem demand_bound (commands : List (PublicationInput.Command p a)) (owner : Fin p) :
    demand commands owner ≤ 3*commands.length := by
  induction commands with
  | nil => simp [demand]
  | cons command rest ih =>
    rw [demand_cons]
    have head := command_cost_bound command owner
    simp only [List.length_cons]
    omega

theorem amount_bound {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a}
    {state : State p a} (valid : PublicationLifecycle.Invariant assigned scopeId seed state)
    (capacity : state.remaining ≤ 512) (owner : Fin p) : amount state owner ≤ 1536 := by
  cases present : state.source with
  | none => simp [amount,present]
  | some source =>
    have lengthBound := (valid.2 source present).1
    have costs := demand_bound state.suffix owner
    simp only [amount,present,Option.isSome_some,ite_true]
    omega

-- Whole query reply: one bounded natural, independent of owner-count. It
-- does not append a field to a source reply already at its framing limit.
theorem reply_fits {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a}
    {state : State p a} (valid : PublicationLifecycle.Invariant assigned scopeId seed state)
    (capacity : state.remaining ≤ 512) (owner : Fin p) :
    OwnerResponseProfile.Fits OwnerCodecTree.natural 256 13 1 (amount state owner) := by
  have bound := amount_bound valid capacity owner
  exact OwnerResponseProfile.fits_natural (digits:=11) (by omega) (by decide)

theorem reply_readable {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a}
    {state : State p a} (valid : PublicationLifecycle.Invariant assigned scopeId seed state)
    (capacity : state.remaining ≤ 512) (owner : Fin p) :
    OwnerResponseProfile.WireFits OwnerCodecTree.natural 256 (amount state owner) :=
  OwnerResponseProfile.fits_wire (reply_fits valid capacity owner) (by decide) (by decide)

-- Source transitions never manufacture semantic source quota. This maintains
-- the query bound through the actual source-funding wrapper from a <=512 start.
theorem source_remaining_mono {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a}
    {old : State p a} {input : PublicationInput.Input p a}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed old) :
    (PublicationLifecycle.transitionFast assigned scopeId seed old input).1.remaining ≤ old.remaining := by
  rw [PublicationLifecycle.transitionFast_exact valid]
  by_cases accepted : (PublicationLifecycle.transition assigned scopeId seed old input).2 = .accepted
  · have spent := (PublicationLifecycle.accepted_refines assigned scopeId seed old input accepted).2
    omega
  · rw [PublicationLifecycle.refusal_frames_state assigned scopeId seed old input accepted]
    exact Nat.le_refl _

theorem funded_remaining_mono {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a}
    {old : State p a} {value : SourceFundingInput.Request p a}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed old) :
    (SourceFundingInput.execute assigned scopeId seed old value).1.remaining ≤ old.remaining := by
  have original := source_remaining_mono valid (input:=value.2)
  cases ran : PublicationLifecycle.transitionFast assigned scopeId seed old value.2 with
  | mk next status =>
    rw [ran] at original
    cases status <;> simp [SourceFundingInput.execute,ran] <;> try exact original
    split
    · exact original
    · exact Nat.le_refl _

#print axioms faithful
#print axioms passive_state
#print axioms before_execute
#print axioms command_cost_bound
#print axioms demand_bound
#print axioms amount_bound
#print axioms reply_fits
#print axioms reply_readable
#print axioms source_remaining_mono
#print axioms funded_remaining_mono

-- A reply codec is selected by the actual request constructor. Query replies
-- do not enlarge the existing source reply. Exclusive ordered IO must retain
-- that constructor through reply receipt; a raw caller-supplied status cannot
-- select a different reply decoder.
abbrev Input (p a : Nat) := Sum (Fin p) (SourceFundingInput.Request p a)
def input (p a : Nat) : OwnerCodecTree.Codec (Input p a) :=
  OwnerCodecTree.sum (OwnerCodecTree.finite p) (SourceFundingInput.request p a)

def Response : Input p a → Type
  | .inl _ => Nat
  | .inr _ => Status × Option (SourcePublicationWorker.PrivateOutput p a)

def reply (command : Input p a) : OwnerCodecTree.Codec (Response command) :=
  match command with
  | .inl _ => OwnerCodecTree.natural
  | .inr _ => PublicationCapacityDriver.reply p a

def exchange (assigned : SourceInput.Assignment p a) (scopeId : Nat)
    (seed : SourceInput.Bootstrap a) (state : State p a) (command : Input p a) :
    State p a × Response command :=
  match command with
  | .inl owner => inspect state owner
  | .inr value =>
    let (next,status) := SourceFundingInput.execute assigned scopeId seed state value
    (next,(status,next.source.map SourcePublicationWorker.project))

theorem query_exact {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a} {state : State p a} {owner : Fin p} :
    exchange assigned scopeId seed state (.inl owner) = (state,amount state owner) := rfl

theorem exchange_preserves {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a} {state : State p a} {command : Input p a}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed state)
    (capacity : state.remaining ≤ 512) :
    PublicationLifecycle.Invariant assigned scopeId seed (exchange assigned scopeId seed state command).1 ∧
    (exchange assigned scopeId seed state command).1.remaining ≤ 512 := by
  cases command with
  | inl owner => exact ⟨valid,capacity⟩
  | inr value =>
    exact ⟨SourceFundingInput.preserves valid,Nat.le_trans (funded_remaining_mono valid) capacity⟩

theorem exchange_reply_readable {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a} {state : State p a} {command : Input p a}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed state)
    (capacity : state.remaining ≤ 512) :
    let result := exchange assigned scopeId seed state command
    (OwnerPacketCodec.encode (reply command) result.2).length ≤ 65536 ∧
    OwnerPacketCodec.decodeAt (reply command) 256 (OwnerPacketCodec.encode (reply command) result.2) = some result.2 := by
  cases command with
  | inl owner =>
    have fits := reply_fits valid capacity owner
    have readable := reply_readable valid capacity owner
    exact ⟨Nat.le_trans fits.1 (by decide),readable.2.2.2⟩
  | inr value =>
    have ready := SourceFundingInput.reply_readable valid (value:=value)
    exact ⟨ready.1,ready.2.2⟩

theorem input_exact {p a : Nat} (bytes : List UInt8) (value : Input p a) :
    OwnerPacketCodec.decode (input p a) bytes = some value ↔
      bytes = OwnerPacketCodec.encode (input p a) value := OwnerPacketCodec.exact _ _ _

#print axioms query_exact
#print axioms exchange_preserves
#print axioms exchange_reply_readable
#print axioms input_exact


-- A separate next-obligation question, restricted to the actual owner recipes.
-- The host must retain its exact request position and match a later paid event;
-- a True answer alone never authorizes, performs or acknowledges that event.
inductive HeadRequest (p : Nat) where
  | freeze (owner : Fin p) (revision : Nat)
  | install (owner : Fin p) (revision : Nat)
  | finish (owner : Fin p)

def headRequest (p : Nat) : OwnerCodecTree.Codec (HeadRequest p) :=
  OwnerCodecTree.iso
    (OwnerCodecTree.sum (OwnerCodecTree.product (OwnerCodecTree.finite p) OwnerCodecTree.natural)
      (OwnerCodecTree.sum (OwnerCodecTree.product (OwnerCodecTree.finite p) OwnerCodecTree.natural)
        (OwnerCodecTree.finite p)))
    (fun value => match value with
      | .freeze i r => .inl (i,r)
      | .install i r => .inr (.inl (i,r))
      | .finish i => .inr (.inr i))
    (fun value => match value with
      | .inl (i,r) => .freeze i r
      | .inr (.inl (i,r)) => .install i r
      | .inr (.inr i) => .finish i)
    (by intro value; cases value <;> rfl)
    (by intro value; rcases value with ⟨i,r⟩ | ⟨i,r⟩ | i <;> rfl)

def headCommand : HeadRequest p → PublicationInput.Command p a
  | .freeze i r => .freeze i r
  | .install i r => .install i r
  | .finish i => .finish i

def matchesHead (state : State p a) (request : HeadRequest p) : Bool :=
  match state.suffix with
  | [] => false
  | command::_ => PublicationCapacityDriver.sameCommand (headCommand request) command

theorem matches_head_exact : matchesHead state request = true ↔
    ∃ rest, state.suffix = headCommand request :: rest := by
  cases pending : state.suffix with
  | nil => simp [matchesHead,pending]
  | cons command rest =>
    rw [show matchesHead state request = PublicationCapacityDriver.sameCommand (headCommand request) command by simp [matchesHead,pending],
      PublicationCapacityDriver.sameCommand_exact]
    constructor
    · intro same; exact ⟨rest,by rw [same]⟩
    · rintro ⟨tail,equal⟩
      exact (List.cons.inj equal).1.symm

-- Keep the already proved amount/execute exchange as a branch and add one
-- read-only Boolean response. Its frame is selected by the retained request.
abbrev CheckedInput (p a : Nat) := Sum (HeadRequest p) (Input p a)
def checkedInput (p a : Nat) : OwnerCodecTree.Codec (CheckedInput p a) :=
  OwnerCodecTree.sum (headRequest p) (input p a)
def CheckedResponse : CheckedInput p a → Type
  | .inl _ => Bool
  | .inr original => Response original

def checkedReply (request : CheckedInput p a) : OwnerCodecTree.Codec (CheckedResponse request) :=
  match request with
  | .inl _ => OwnerCodecTree.boolean
  | .inr original => reply original

def checkedExchange (assigned : SourceInput.Assignment p a) (scopeId : Nat)
    (seed : SourceInput.Bootstrap a) (state : State p a) (request : CheckedInput p a) :
    State p a × CheckedResponse request :=
  match request with
  | .inl head => (state,matchesHead state head)
  | .inr original => exchange assigned scopeId seed state original

theorem head_passive : (checkedExchange assigned scopeId seed state (.inl request)).1 = state := rfl

theorem checked_preserves {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a} {state : State p a} {request : CheckedInput p a}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed state)
    (capacity : state.remaining ≤ 512) :
    PublicationLifecycle.Invariant assigned scopeId seed (checkedExchange assigned scopeId seed state request).1 ∧
    (checkedExchange assigned scopeId seed state request).1.remaining ≤ 512 := by
  cases request with
  | inl head => exact ⟨valid,capacity⟩
  | inr original => exact exchange_preserves valid capacity

theorem boolean_readable (value : Bool) :
    (OwnerPacketCodec.encode OwnerCodecTree.boolean value).length ≤ 65536 ∧
    SourceCodec.compactFits (OwnerPacketCodec.encode OwnerCodecTree.boolean value) = true ∧
    OwnerPacketCodec.decodeAt OwnerCodecTree.boolean 256
      (OwnerPacketCodec.encode OwnerCodecTree.boolean value) = some value := by
  have unitFit : OwnerResponseProfile.Fits OwnerCodecTree.unit 4096 3 1 () := by
    simp [OwnerResponseProfile.Fits,OwnerPacketCodec.encode,OwnerCodecTree.unit,
      OwnerTreeBytes.encode,OwnerTreeBytes.writeTree,OwnerTreeBytes.writeTrees,
      OwnerTreeBytes.treeCost,OwnerTreeBytes.treesCost,OwnerByteCodec.writeNat,
      OwnerPayload.digitCheck,OwnerPayload.nextDigits,OwnerPayload.textCheck,OwnerPayload.textHeadCheck]
  have fit : OwnerResponseProfile.Fits OwnerCodecTree.boolean 4096 8 3 value := by
    cases value with
    | false =>
      have first := OwnerResponseProfile.fits_inl unitFit OwnerCodecTree.unit
      refine ⟨?_,first.2⟩
      have := first.1
      change (OwnerPacketCodec.encode (OwnerCodecTree.sum OwnerCodecTree.unit OwnerCodecTree.unit) (.inl ())).length ≤ 8
      omega
    | true => exact OwnerResponseProfile.fits_inr unitFit OwnerCodecTree.unit
  have readable := OwnerResponseProfile.fits_wire fit (by decide) (by decide)
  refine ⟨readable.1,?_,readable.2.2.2⟩
  simp [SourceCodec.compactFits,readable.1,fit.2.1,fit.2.2.1]

theorem checked_reply_readable {p a : Nat} {assigned : SourceInput.Assignment p a}
    {scopeId : Nat} {seed : SourceInput.Bootstrap a} {state : State p a} {request : CheckedInput p a}
    (valid : PublicationLifecycle.Invariant assigned scopeId seed state)
    (capacity : state.remaining ≤ 512) :
    let result := checkedExchange assigned scopeId seed state request
    (OwnerPacketCodec.encode (checkedReply request) result.2).length ≤ 65536 ∧
    SourceCodec.compactFits (OwnerPacketCodec.encode (checkedReply request) result.2) = true ∧
    OwnerPacketCodec.decodeAt (checkedReply request) 256
      (OwnerPacketCodec.encode (checkedReply request) result.2) = some result.2 := by
  cases request with
  | inl head => exact boolean_readable _
  | inr original =>
    cases original with
    | inl owner =>
      have fit := reply_fits valid capacity owner
      have readable := reply_readable valid capacity owner
      refine ⟨Nat.le_trans fit.1 (by decide),?_,readable.2.2.2⟩
      have textFit := OwnerResponseProfile.text_bound_monotone (by decide : 256 ≤ 4096) readable.2.2.1
      have textCheck := (OwnerPayload.text_exact _ _).mpr textFit
      change SourceCodec.compactFits (OwnerPacketCodec.encode OwnerCodecTree.natural (amount state owner)) = true
      simp [SourceCodec.compactFits,readable.1,fit.2.1,textCheck]
    | inr value => exact SourceFundingInput.reply_readable valid

#print axioms headRequest
#print axioms matches_head_exact
#print axioms head_passive
#print axioms checked_preserves
#print axioms boolean_readable
#print axioms checked_reply_readable

end MirroreaProofFirst.SourceFundingQuery
