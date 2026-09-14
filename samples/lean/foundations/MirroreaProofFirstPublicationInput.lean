import MirroreaProofFirstCustodyPublication
import MirroreaProofFirstSourceInput

namespace MirroreaProofFirst.PublicationInput

-- Private fixed-cohort coordinator carrier. It does not authenticate control
-- events, reserve a physical namespace or prove that an endpoint has quiesced.
-- Those events must come from the actual admitted process/transport manager.
structure Dispatch (p : Nat) where
  endpoint : Fin p
  context : QualifiedReceipt.Context
  ticket : InvocationBoundary.Ticket
  deriving DecidableEq, Repr

structure State (p a : Nat) where
  publication : CustodyPublication.State p p a
  dispatch : Option (Dispatch p)

inductive Command (p a : Nat) where
  | stage (input : SourceInput.Command p a)
  | arrival (envelope : OwnerReceipt.Envelope)
  | freeze (endpoint : Fin p) (revision : Nat)
  | acknowledge (endpoint : Fin p) (revision : Nat)
  | publish
  | install (endpoint : Fin p) (revision : Nat)
  | enter (endpoint : Fin p)
  | finish (endpoint : Fin p)

def select (s : State p a) : Command p a → Option (PublicationUse.Action p (QualifiedPublication.Command p a))
  | .stage (.receive _ _) => none
  | .stage input => some (.administrative (.stage (SourceInput.elaborate input)))
  | .arrival envelope => do
      let dispatched ← s.dispatch
      if envelope.ticket ≠ dispatched.ticket then none else do
        let _ ← QualifiedReceipt.accept dispatched.context s.publication.current envelope
        match envelope.result with
        | .value value => some (.administrative (.stage (.receive envelope.ticket value)))
        | .rejected _ => none
  | .freeze endpoint revision => some (.administrative (.freeze endpoint revision))
  | .acknowledge endpoint revision => some (.administrative (.acknowledge endpoint revision))
  | .publish => some (.administrative .publish)
  | .install endpoint revision => some (.administrative (.install endpoint revision))
  | .enter endpoint => do
      if s.dispatch.isSome then none else do
        let saved ← s.publication.current.privateState.session.state.source.waiting
        if saved.entry.ticket.place = endpoint.val then some (.enter endpoint) else none
  | .finish endpoint => some (.finish endpoint)

-- Dispatch correlation is created by this coordinator's actual enter event;
-- arrival bytes cannot supply or replace it. A retained cancelled/received
-- continuation is cleared only by the committed publication.
def retain (scopeId : Nat) (s : State p a) (next : CustodyPublication.State p p a) :
    Command p a → Option (Dispatch p)
  | .enter endpoint => s.publication.current.privateState.session.state.source.waiting.map
      (fun saved => ⟨endpoint,⟨scopeId,s.publication.barrier.installed endpoint⟩,saved.entry.ticket⟩)
  | .publish => if next.current.privateState.session.state.source.waiting.isNone then none else s.dispatch
  | _ => s.dispatch

def execute (scopeId : Nat) (s : State p a) (command : Command p a) : Option (State p a) := do
  let action ← select s command
  let next ← PublicationImage.execute CustodyPublication.image QualifiedCustody.evaluate s.publication action
  return ⟨next,retain scopeId s next command⟩

def initial (source : QualifiedCustody.State p a) : State p a :=
  ⟨PublicationImage.initial CustodyPublication.image p 0 source,none⟩

-- This path is a real sequence of commands, not a caller-supplied invariant.
inductive Reached (scopeId : Nat) (source : QualifiedCustody.State p a) : State p a → Prop where
  | initial : Reached scopeId source (PublicationInput.initial source)
  | step : Reached scopeId source s → execute scopeId s command = some next → Reached scopeId source next

theorem executed_action (accepted : execute scopeId s command = some next) :
    ∃ action, select s command = some action ∧
      PublicationImage.execute CustodyPublication.image QualifiedCustody.evaluate s.publication action =
        some next.publication ∧ next.dispatch = retain scopeId s next.publication command := by
  cases selected : select s command with
  | none => simp [execute,selected] at accepted
  | some action =>
    cases ran : PublicationImage.execute CustodyPublication.image QualifiedCustody.evaluate s.publication action with
    | none => simp [execute,selected,ran] at accepted
    | some publication =>
      simp [execute,selected,ran] at accepted
      cases accepted
      exact ⟨action,rfl,ran,rfl⟩

theorem reached_publication {p a : Nat} {source : QualifiedCustody.State p a} {s : State p a}
    (path : Reached scopeId source s) :
    CustodyPublication.Reached p 0 source s.publication := by
  induction path with
  | initial => exact .initial
  | step prior accepted ih => exact .step ih (executed_action accepted).choose_spec.2.1

theorem rooted_source (root : QualifiedCustody.Rooted realm view policy identity source)
    (path : Reached scopeId source s) :
    QualifiedCustody.Rooted realm view policy identity s.publication.current :=
  CustodyPublication.current_rooted root (reached_publication path)

theorem raw_receive_refused : execute scopeId s (.stage (.receive ticket value)) = none := rfl

theorem arrival_correlated (accepted : execute scopeId s (.arrival envelope) = some next) :
    ∃ dispatched value received, s.dispatch = some dispatched ∧ envelope.ticket = dispatched.ticket ∧
      envelope.result = .value value ∧
      QualifiedReceipt.accept dispatched.context s.publication.current envelope = some received ∧
      next.publication.current = s.publication.current ∧ next.dispatch = s.dispatch := by
  obtain ⟨action,selected,ran,retained⟩ := executed_action accepted
  cases dispatch : s.dispatch with
  | none => simp [select,dispatch] at selected
  | some dispatched =>
    by_cases ticket : envelope.ticket = dispatched.ticket
    · cases received : QualifiedReceipt.accept dispatched.context s.publication.current envelope with
      | none => simp [select,dispatch,ticket,received] at selected
      | some privateState =>
        cases result : envelope.result with
        | rejected reason => simp [select,dispatch,ticket,result] at selected
        | value value =>
          have same : action = .administrative (.stage (.receive envelope.ticket value)) := by
            simpa [select,dispatch,ticket,received,result] using selected.symm
          subst action
          unfold PublicationImage.execute at ran
          split at ran
          · have samePublication := Option.some.inj ran
            have sameCurrent := congrArg PublicationImage.State.current samePublication
            exact ⟨dispatched,value,privateState,rfl,ticket,rfl,received,sameCurrent.symm,by simpa [retain,dispatch] using retained⟩
          · cases ran
    · simp [select,dispatch,ticket] at selected

theorem dispatch_created (accepted : execute scopeId s (.enter endpoint) = some next) :
    s.dispatch = none ∧ ∃ saved, s.publication.current.privateState.session.state.source.waiting = some saved ∧
      saved.entry.ticket.place = endpoint.val ∧
      next.dispatch = some ⟨endpoint,⟨scopeId,s.publication.barrier.installed endpoint⟩,saved.entry.ticket⟩ := by
  obtain ⟨action,selected,_,retained⟩ := executed_action accepted
  cases dispatch : s.dispatch with
  | some prior => simp [select,dispatch] at selected
  | none =>
    cases waiting : s.publication.current.privateState.session.state.source.waiting with
    | none => simp [select,dispatch,waiting] at selected
    | some saved =>
      by_cases place : saved.entry.ticket.place = endpoint.val
      · exact ⟨rfl,saved,rfl,place,by simpa [retain,waiting] using retained⟩
      · simp [select,dispatch,waiting,place] at selected

#print axioms executed_action
#print axioms reached_publication
#print axioms rooted_source
#print axioms raw_receive_refused
#print axioms arrival_correlated
#print axioms dispatch_created
end MirroreaProofFirst.PublicationInput

namespace MirroreaProofFirst.PublicationInput
open OwnerCodecTree

def command (p a : Nat) : Codec (Command p a) := iso
  (sum (SourceInput.command p a) (sum OwnerReceipt.codec
    (sum (product (finite p) natural) (sum (product (finite p) natural)
      (sum unit (sum (product (finite p) natural) (sum (finite p) (finite p))))))))
  (fun c => match c with
    | .stage s => .inl s
    | .arrival e => .inr (.inl e)
    | .freeze i r => .inr (.inr (.inl (i,r)))
    | .acknowledge i r => .inr (.inr (.inr (.inl (i,r))))
    | .publish => .inr (.inr (.inr (.inr (.inl ()))))
    | .install i r => .inr (.inr (.inr (.inr (.inr (.inl (i,r))))))
    | .enter i => .inr (.inr (.inr (.inr (.inr (.inr (.inl i))))))
    | .finish i => .inr (.inr (.inr (.inr (.inr (.inr (.inr i)))))))
  (fun c => match c with
    | .inl s => .stage s
    | .inr (.inl e) => .arrival e
    | .inr (.inr (.inl (i,r))) => .freeze i r
    | .inr (.inr (.inr (.inl (i,r)))) => .acknowledge i r
    | .inr (.inr (.inr (.inr (.inl _)))) => .publish
    | .inr (.inr (.inr (.inr (.inr (.inl (i,r)))))) => .install i r
    | .inr (.inr (.inr (.inr (.inr (.inr (.inl i)))))) => .enter i
    | .inr (.inr (.inr (.inr (.inr (.inr (.inr i)))))) => .finish i)
  (by intro c; cases c <;> rfl)
  (by intro c; rcases c with s | e | ⟨i,r⟩ | ⟨i,r⟩ | ⟨⟩ | ⟨i,r⟩ | i | i <;> rfl)

inductive Input (p a : Nat) where
  | launch (program : QualifiedSession.Program p)
  | step (command : Command p a)

def input (p a : Nat) : Codec (Input p a) := iso (sum (SourceCodec.compact p) (command p a))
  (fun c => match c with | .launch program => .inl program | .step command => .inr command)
  (fun c => match c with | .inl program => .launch program | .inr command => .step command)
  (by intro c; cases c <;> rfl) (by intro c; cases c <;> rfl)

def transition (assigned : SourceInput.Assignment p a) (scopeId : Nat) (seed : SourceInput.Bootstrap a)
    (state : Option (State p a)) (input : Input p a) : Option (State p a) × Bool :=
  match state,input with
  | none,.launch program => match SourceInput.launch assigned seed program with
    | none => (none,false) | some source => (some (initial source),true)
  | some state,.step command => match execute scopeId state command with
    | none => (some state,false) | some next => (some next,true)
  | _,_ => (state,false)

def Invariant (assigned : SourceInput.Assignment p a) (scopeId : Nat) (seed : SourceInput.Bootstrap a)
    (state : Option (State p a)) : Prop :=
  ∀ actual, state = some actual → ∃ program source,
    SourceInput.launch assigned seed program = some source ∧ Reached scopeId source actual

theorem transition_preserves (valid : Invariant assigned scopeId seed state) :
    Invariant assigned scopeId seed (transition assigned scopeId seed state entry).1 := by
  cases state with
  | none =>
    cases entry with
    | launch program =>
      cases started : SourceInput.launch assigned seed program with
      | none => simp [transition,started,Invariant]
      | some source =>
        intro actual equal
        have same : initial source = actual := by simpa [transition,started] using equal
        cases same
        exact ⟨program,source,started,.initial⟩
    | step _ => simp [transition,Invariant]
  | some old =>
    cases entry with
    | launch _ => exact valid
    | step command =>
      cases computed : execute scopeId old command with
      | none => simpa [transition,computed] using valid
      | some next =>
        intro actual equal
        have same : next = actual := by simpa [transition,computed] using equal
        cases same
        obtain ⟨program,source,started,path⟩ := valid old rfl
        exact ⟨program,source,started,.step path computed⟩

theorem invariant_source (valid : Invariant assigned scopeId seed (some state)) :
    QualifiedCustody.Rooted assigned.realm (SourceInput.view seed.head)
      (SourceInput.lookupRules seed.control) assigned.identity state.publication.current := by
  obtain ⟨program,source,started,path⟩ := valid state rfl
  exact rooted_source (SourceInput.launch_rooted started) path

theorem no_reinitialize : transition assigned scopeId seed (some state) (.launch program) = (some state,false) := rfl

theorem decoded_execution (decoded : OwnerPacketCodec.decode (command p a) bytes = some cmd)
    (accepted : execute scopeId s cmd = some next) :
    bytes = OwnerPacketCodec.encode (command p a) cmd ∧
      ∃ action, select s cmd = some action ∧
        PublicationImage.execute CustodyPublication.image QualifiedCustody.evaluate s.publication action = some next.publication := by
  obtain ⟨action,selected,ran,_⟩ := executed_action accepted
  exact ⟨OwnerPacketCodec.canonical _ _ _ decoded,action,selected,ran⟩

#print axioms command
#print axioms input
#print axioms transition_preserves
#print axioms invariant_source
#print axioms no_reinitialize
#print axioms decoded_execution
end MirroreaProofFirst.PublicationInput
