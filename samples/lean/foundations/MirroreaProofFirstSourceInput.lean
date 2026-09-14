import MirroreaProofFirstSourceCodec
import MirroreaProofFirstQualifiedCustody

namespace MirroreaProofFirst.SourceInput
open OwnerCodecTree

-- Finite explicit rules for a private source activation, including future keys.
-- Unlike captureView over existing instances, this data defines a total policy
-- function. Its default is explicit supplied policy, never an implicit grant.
-- No claim is made that arbitrary function-valued policy can be captured here.
structure Rules (K : Type) where
  fallback : CurrentUse.Policy
  overrides : List (K × CurrentUse.Policy)

def lookupRules [DecidableEq K] (rules : Rules K) (key : K) : CurrentUse.Policy :=
  ((rules.overrides.find? fun row => decide (row.1 = key)).map Prod.snd).getD rules.fallback

def rules (key : Codec K) : Codec (Rules K) := iso
  (product OwnerFullCodec.policy (list (product key OwnerFullCodec.policy)))
  (fun value => (value.fallback,value.overrides)) (fun (fallback,overrides) => ⟨fallback,overrides⟩)
  (by intro value; cases value; rfl) (by intro value; rfl)

structure Head (a : Nat) where
  realm : Nat
  generation : Nat
  members : Vector WorldProjection.Member a
  authority : AuthorityImage.Image
  access : Rules (Nat × Nat)

def head (a : Nat) : Codec (Head a) := iso
  (product natural (product natural (product (vector OwnerRecordCodecs.member a)
    (product OwnerRecordCodecs.authority (rules (product natural natural))))))
  (fun value => (value.realm,value.generation,value.members,value.authority,value.access))
  (fun (realm,generation,members,authority,access) => ⟨realm,generation,members,authority,access⟩)
  (by intro value; cases value; rfl) (by intro value; rfl)

def view (head : Head a) : WorldProjection.AuthorityView a :=
  ⟨head.realm,head.generation,(fun key => head.members[key.val]),AuthorityImage.restore head.authority,
    fun key place => lookupRules head.access (key,place)⟩

structure Bootstrap (a : Nat) where
  head : Head a
  control : Rules Nat

def bootstrap (a : Nat) : Codec (Bootstrap a) := iso (product (head a) (rules natural))
  (fun value => (value.head,value.control)) (fun (head,control) => ⟨head,control⟩)
  (by intro value; cases value; rfl) (by intro value; rfl)

def raw : Codec CompositionCore.Raw := iso
  (sum (product OwnerCodeCodec.definition (optional natural))
    (sum (product natural (product natural (product (list natural)
      (product (optional natural) (OwnerTreeCodecs.formula natural)))))
    (sum natural (sum (product natural (optional natural))
      (sum (product natural natural) (sum natural natural))))))
  (fun value => match value with
    | .register definition predecessor => .inl (definition,predecessor)
    | .instantiate definition owner places parent dependencies => .inr (.inl (definition,owner,places,parent,dependencies))
    | .retire key => .inr (.inr (.inl key))
    | .reparent key parent => .inr (.inr (.inr (.inl (key,parent))))
    | .replace key definition => .inr (.inr (.inr (.inr (.inl (key,definition)))))
    | .leave place => .inr (.inr (.inr (.inr (.inr (.inl place)))))
    | .join place => .inr (.inr (.inr (.inr (.inr (.inr place))))))
  (fun value => match value with
    | .inl (definition,predecessor) => .register definition predecessor
    | .inr (.inl (definition,owner,places,parent,dependencies)) => .instantiate definition owner places parent dependencies
    | .inr (.inr (.inl key)) => .retire key
    | .inr (.inr (.inr (.inl (key,parent)))) => .reparent key parent
    | .inr (.inr (.inr (.inr (.inl (key,definition))))) => .replace key definition
    | .inr (.inr (.inr (.inr (.inr (.inl place))))) => .leave place
    | .inr (.inr (.inr (.inr (.inr (.inr place))))) => .join place)
  (by intro value; cases value <;> rfl)
  (by intro value; rcases value with ⟨definition,predecessor⟩ | ⟨definition,owner,places,parent,dependencies⟩ |
      key | ⟨key,parent⟩ | ⟨key,definition⟩ | place | place <;> rfl)

-- Private typed entry carrier. No populated Session, saved binding, Stamp,
-- observer record or history is an input. Ordinary source needs only ticks;
-- host arrival/control events are distinct entries, not handwritten Mir calls.
inductive Command (p a : Nat) where
  | tick
  | receive (ticket : InvocationBoundary.Ticket) (value : Int)
  | cancel
  | head (head : Head a)
  | control (member : Fin a) (place : Fin p) (principal : Nat) (raw : CompositionCore.Raw)
  | replace (program : QualifiedSession.Program p)
  | continueWith (program : QualifiedSession.Program p)

def command (p a : Nat) : Codec (Command p a) := iso
  (sum unit (sum (product OwnerFullCodec.ticket integer) (sum unit (sum (head a)
    (sum (product (finite a) (product (finite p) (product natural raw)))
      (sum (SourceCodec.compact p) (SourceCodec.compact p)))))))
  (fun value => match value with
    | .tick => .inl ()
    | .receive ticket value => .inr (.inl (ticket,value))
    | .cancel => .inr (.inr (.inl ()))
    | .head view => .inr (.inr (.inr (.inl view)))
    | .control member place principal raw => .inr (.inr (.inr (.inr (.inl (member,place,principal,raw)))))
    | .replace source => .inr (.inr (.inr (.inr (.inr (.inl source)))))
    | .continueWith source => .inr (.inr (.inr (.inr (.inr (.inr source))))))
  (fun value => match value with
    | .inl _ => .tick
    | .inr (.inl (ticket,value)) => .receive ticket value
    | .inr (.inr (.inl _)) => .cancel
    | .inr (.inr (.inr (.inl view))) => .head view
    | .inr (.inr (.inr (.inr (.inl (member,place,principal,raw))))) => .control member place principal raw
    | .inr (.inr (.inr (.inr (.inr (.inl source))))) => .replace source
    | .inr (.inr (.inr (.inr (.inr (.inr source))))) => .continueWith source)
  (by intro value; cases value <;> rfl)
  (by intro value; rcases value with ⟨⟩ | ⟨ticket,value⟩ | ⟨⟩ | view |
      ⟨member,place,principal,raw⟩ | source | source <;> rfl)

def elaborate : Command p a → QualifiedPublication.Command p a
  | .tick => .tick
  | .receive ticket value => .receive ticket value
  | .cancel => .cancel
  | .head head => .head (view head)
  | .control member place principal raw => .control member place principal raw
  | .replace source => .replace source
  | .continueWith source => .continueWith source

def evaluate (s : QualifiedCustody.State p a) (input : Command p a) :=
  QualifiedCustody.evaluate s (elaborate input)

theorem entry_exact : evaluate s input = some next ↔ QualifiedCustody.Entry s (elaborate input) next :=
  QualifiedCustody.evaluate_exact

theorem decoded_entry
    (decoded : OwnerPacketCodec.decode (command p a) bytes = some input)
    (accepted : evaluate s input = some next) :
    bytes = OwnerPacketCodec.encode (command p a) input ∧
    QualifiedCustody.Entry s (elaborate input) next :=
  ⟨OwnerPacketCodec.canonical _ _ _ decoded,entry_exact.mp accepted⟩

theorem encoded_evaluation (input : Command p a) :
    (OwnerPacketCodec.decode (command p a) (OwnerPacketCodec.encode (command p a) input) >>= evaluate s) =
      evaluate s input := by rw [OwnerPacketCodec.roundtrip]; rfl

#print axioms rules
#print axioms head
#print axioms bootstrap
#print axioms raw
#print axioms command
#print axioms entry_exact
#print axioms decoded_entry
#print axioms encoded_evaluation

structure Assignment (p a : Nat) where
  realm : Nat
  identity : QualifiedSource.Actor p a

-- Bootstrap is owned by the launcher, separate from source program bytes.
-- This constructor does not authenticate that launcher or issue its claims.
def launch (assigned : Assignment p a) (seed : Bootstrap a) (source : QualifiedSession.Program p) :
    Option (QualifiedCustody.State p a) := do
  if source.caller ≠ assigned.identity.caller || seed.head.realm ≠ assigned.realm then none else do
    let session ← QualifiedSession.launch assigned.realm (view seed.head) (lookupRules seed.control)
      assigned.identity.member assigned.identity.principal source
    if QualifiedCustody.activeCheck (QualifiedCustody.capture session) session then
      some (QualifiedCustody.start session) else none

theorem launch_rooted (accepted : launch assigned seed source = some state) :
    QualifiedCustody.Rooted assigned.realm (view seed.head) (lookupRules seed.control) assigned.identity state := by
  unfold launch at accepted
  split at accepted
  · cases accepted
  · rename_i allowed
    have constraints : source.caller = assigned.identity.caller ∧ seed.head.realm = assigned.realm := by
      simpa using allowed
    have caller := constraints.1
    cases started : QualifiedSession.launch assigned.realm (view seed.head) (lookupRules seed.control)
        assigned.identity.member assigned.identity.principal source with
    | none => simp [started] at accepted
    | some session =>
      simp only [started,Option.bind_eq_bind,Option.bind_some] at accepted
      split at accepted
      · cases accepted
        exact .initial started (by rw [caller])
      · cases accepted

-- Positive entry theorem starts from independent source typing, assigned
-- caller/realm and current actor admission of the original launch.
theorem launch_complete
    (caller : source.caller = assigned.identity.caller) (realm : seed.head.realm = assigned.realm)
    (started : QualifiedSession.launch assigned.realm (view seed.head) (lookupRules seed.control)
      assigned.identity.member assigned.identity.principal source = some session)
    (active : QualifiedCustody.Active (QualifiedCustody.capture session) session) :
    launch assigned seed source = some (QualifiedCustody.start session) := by
  simp [launch,caller,realm,started,QualifiedCustody.active_exact.mpr active]

inductive Input (p a : Nat) where
  | launch (source : QualifiedSession.Program p)
  | step (command : Command p a)

def input (p a : Nat) : Codec (Input p a) := iso (sum (SourceCodec.compact p) (command p a))
  (fun value => match value with | .launch source => .inl source | .step command => .inr command)
  (fun value => match value with | .inl source => .launch source | .inr command => .step command)
  (by intro value; cases value <;> rfl) (by intro value; cases value <;> rfl)

def transition (assigned : Assignment p a) (seed : Bootstrap a)
    (state : Option (QualifiedCustody.State p a)) (input : Input p a) :
    Option (QualifiedCustody.State p a) × Bool :=
  match state,input with
  | none,.launch source => match launch assigned seed source with
    | none => (none,false) | some next => (some next,true)
  | some state,.step command => match evaluate state command with
    | none => (some state,false) | some next => (some next,true)
  | _,_ => (state,false)

def Invariant (assigned : Assignment p a) (seed : Bootstrap a)
    (state : Option (QualifiedCustody.State p a)) : Prop :=
  ∀ value, state = some value →
    QualifiedCustody.Rooted assigned.realm (view seed.head) (lookupRules seed.control) assigned.identity value

theorem transition_preserves (valid : Invariant assigned seed state) :
    Invariant assigned seed (transition assigned seed state entry).1 := by
  cases state with
  | none =>
    cases entry with
    | launch source =>
      cases started : launch assigned seed source with
      | none => simp [transition,started,Invariant]
      | some next =>
        intro value equal
        have same : next = value := by simpa [transition,started] using equal
        cases same
        exact launch_rooted started
    | step command => simp [transition,Invariant]
  | some current =>
    cases entry with
    | launch _ => exact valid
    | step command =>
      cases computed : evaluate current command with
      | none => simpa [transition,computed] using valid
      | some next =>
        intro value equal
        have same : next = value := by simpa [transition,computed] using equal
        cases same
        exact .step (valid current rfl) computed

theorem no_reinitialize : transition assigned seed (some state) (.launch source) = (some state,false) := rfl

#print axioms launch_rooted
#print axioms launch_complete
#print axioms input
#print axioms transition_preserves
#print axioms no_reinitialize
end MirroreaProofFirst.SourceInput
