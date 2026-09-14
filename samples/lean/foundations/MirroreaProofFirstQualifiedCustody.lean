import MirroreaProofFirstQualifiedPublication

namespace MirroreaProofFirst.QualifiedCustody
open QualifiedSession

-- Reversible finite activation profile, not a grant or a final rejoin policy.
-- An existing private activation keeps its original member/locus incarnation.
-- Current operation authorization remains in the original entry checker.
structure Stamp where
  realm : Nat
  memberIncarnation : Nat
  locusIncarnation : Nat
  deriving DecidableEq, Repr

def system (s : Session p a) := s.state.source.machine.store.core.system
def capture (s : Session p a) : Stamp :=
  ⟨(system s).configuration.state.realm,(system s).view.members s.member |>.incarnation,
    (system s).configuration.state.placeIncarnation s.program.caller⟩

def Active (stamp : Stamp) (s : Session p a) : Prop :=
  stamp = capture s ∧ ManagementEntry.CurrentActor (system s) s.member s.program.caller s.principal
def activeCheck (stamp : Stamp) (s : Session p a) : Bool :=
  decide (stamp = capture s) && ManagementEntry.actorCheck (system s) s.member s.program.caller s.principal
theorem active_exact : activeCheck stamp s = true ↔ Active stamp s := by
  simp [activeCheck,Active,ManagementEntry.actor_exact]

-- Only source initiation/adoption uses this activation guard. Received values,
-- cancellation, observed authority heads and separately authorized control
-- still have their own original entry conditions. No lost-invocation grant is
-- manufactured for cancellation and no global generation is frozen by Stamp.
def initiates : QualifiedPublication.Command p a → Bool
  | .tick | .replace _ | .continueWith _ => true
  | _ => false

structure State (p a : Nat) where
  stamp : Stamp
  privateState : QualifiedPublication.Snapshot p a

def start (s : Session p a) : State p a := ⟨capture s,QualifiedPublication.start s⟩

-- No populated import, binding edit, arbitrary continuation or stamp refresh.
def evaluate (s : State p a) (command : QualifiedPublication.Command p a) : Option (State p a) :=
  if initiates command && !activeCheck s.stamp s.privateState.session then none else
    (QualifiedPublication.evaluate s.privateState command).map (fun next => ⟨s.stamp,next⟩)

-- Declarative source rule plus retained history, separately from its checker.
def PrivateEntry (s : QualifiedPublication.Snapshot p a) (command : QualifiedPublication.Command p a)
    (next : QualifiedPublication.Snapshot p a) : Prop :=
  ∃ result, QualifiedPublication.Entry s.session command next.session result ∧
    next.replies = ⟨s.replies.length,command,result⟩ :: s.replies

theorem private_exact : QualifiedPublication.evaluate s command = some next ↔ PrivateEntry s command next := by
  constructor
  · exact QualifiedPublication.evaluated_entry
  · intro ⟨result,entry,replies⟩
    have ran := QualifiedPublication.evaluate_complete entry
    cases next with
    | mk session records =>
        dsimp only at ran replies
        cases replies
        simp [QualifiedPublication.evaluate,ran]

def Entry (s : State p a) (command : QualifiedPublication.Command p a) (next : State p a) : Prop :=
  (initiates command = true → Active s.stamp s.privateState.session) ∧
  next.stamp = s.stamp ∧ PrivateEntry s.privateState command next.privateState

theorem evaluate_exact : evaluate s command = some next ↔ Entry s command next := by
  by_cases permitted : initiates command = true → Active s.stamp s.privateState.session
  · have gate : (initiates command && !activeCheck s.stamp s.privateState.session) = false := by
      cases hi : initiates command with
      | false => simp
      | true => simp [active_exact.mpr (permitted hi)]
    cases ran : QualifiedPublication.evaluate s.privateState command with
    | none => simp [evaluate,gate,Entry,← private_exact,ran]
    | some value =>
        simp only [evaluate,gate,Bool.false_eq_true,ite_false,ran,Option.map_some,Option.some.injEq]
        constructor
        · intro same; subst next; exact ⟨permitted,rfl,private_exact.mp ran⟩
        · intro admitted
          cases next with
          | mk stamp privateState =>
              obtain ⟨_,same,result⟩ := admitted
              have atValue : value = privateState := Option.some.inj (ran.symm.trans (private_exact.mpr result))
              cases same
              cases atValue
              rfl
  · have gate : (initiates command && !activeCheck s.stamp s.privateState.session) = true := by
      cases hi : initiates command with
      | false => exact False.elim (permitted (by simp [hi]))
      | true =>
          have no : activeCheck s.stamp s.privateState.session = false := by
            cases check : activeCheck s.stamp s.privateState.session with
            | false => rfl
            | true => exact False.elim (permitted (fun _ => active_exact.mp check))
          simp [no]
    simp [evaluate,gate,Entry,permitted]

theorem evaluate_complete (active : initiates command = true → Active s.stamp s.privateState.session)
    (computed : QualifiedPublication.evaluate s.privateState command = some snapshot) :
    evaluate s command = some ⟨s.stamp,snapshot⟩ :=
  evaluate_exact.mpr ⟨active,rfl,private_exact.mp computed⟩

theorem evaluation_step (accepted : evaluate s command = some next) :
    QualifiedSession.Step s.privateState.session next.privateState.session :=
  QualifiedPublication.entry_step (evaluate_exact.mp accepted).2.2.choose_spec.1

inductive Rooted (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (identity : QualifiedSource.Actor p a) : State p a → Prop where
  | initial : QualifiedSession.launch realm view policy member principal program = some session →
      identity = ⟨program.caller,member,principal⟩ → Rooted realm view policy identity (start session)
  | step : Rooted realm view policy identity s → evaluate s command = some next →
      Rooted realm view policy identity next

theorem rooted_private (root : Rooted realm view policy identity s) :
    QualifiedSession.Rooted realm view policy identity s.privateState.session ∧
    QualifiedPublication.Ordinals s.privateState := by
  induction root with
  | initial launched same => exact ⟨.launch launched same,QualifiedPublication.start_ordinals⟩
  | step previous accepted ih =>
      exact ⟨.step ih.1 (evaluation_step accepted),
        QualifiedPublication.evaluated_ordinals ih.2 (private_exact.mpr (evaluate_exact.mp accepted).2.2)⟩

theorem initiation_current (accepted : evaluate s command = some next) (fresh : initiates command = true) :
    Active s.stamp s.privateState.session := (evaluate_exact.mp accepted).1 fresh

theorem changed_incarnation_refused
    (changed : s.stamp ≠ capture s.privateState.session) (fresh : initiates command = true) :
    evaluate s command = none := by
  simp [evaluate,fresh,activeCheck,changed]

theorem departed_refused
    (departed : (system s.privateState.session).configuration.state.participating
      s.privateState.session.program.caller = false) (fresh : initiates command = true) :
    evaluate s command = none := by
  simp [evaluate,fresh,activeCheck,ManagementEntry.actorCheck,departed]

-- Every admitted command frames the activation stamp, including head/control,
-- cancellation and source replacement. A new enabled incarnation cannot silently
-- renew this activation merely by passing current actorCheck again.
theorem stamp_retained (accepted : evaluate s command = some next) : next.stamp = s.stamp :=
  (evaluate_exact.mp accepted).2.1

-- A pending private continuation is retained byte-for-byte as mathematical
-- data, or removed by the existing receive/cancel path. In particular no
-- admitted head/control can erase binding or retarget name/site/inputs.
theorem entry_pending_retained
    (continuing : QualifiedSession.Continuing before)
    (waiting : before.state.source.waiting = some saved)
    (entry : QualifiedPublication.Entry before command after result) :
    after.state.source.waiting = some saved ∨ after.state.source.waiting = none := by
  cases entry with
  | tick ready =>
      have empty := continuing.1 ready
      rw [waiting] at empty
      cases empty
  | receive received =>
      obtain ⟨_,state,ran,equal⟩ := QualifiedSession.receive_parts received
      rw [equal]
      exact Or.inr (QualifiedSource.received_clears ran)
  | cancel =>
      simp only [QualifiedSession.cancel]
      split
      · exact Or.inr (QualifiedSource.cancel_clears _ _ ‹_›)
      · exact Or.inl waiting
  | head installed =>
      rw [(QualifiedSession.head_parts installed).2]
      exact Or.inl waiting
  | control committed =>
      obtain ⟨state,ran,equal⟩ := QualifiedSession.control_parts committed
      rw [equal]
      exact Or.inl ((ReferenceSource.controlInput_projects _ _ _ _ _ _ ran).2.1.trans waiting)
  | replace adopted =>
      have empty := (QualifiedSession.adopt_parts (QualifiedSession.replacement_adopts adopted)).2.1
      rw [waiting] at empty
      cases empty
  | continueWith adopted =>
      have empty := (QualifiedSession.adopt_parts (QualifiedSession.continuation_adopts adopted)).2.1
      rw [waiting] at empty
      cases empty


theorem pending_retained
    (root : Rooted realm view policy identity s)
    (waiting : s.privateState.session.state.source.waiting = some saved)
    (accepted : evaluate s command = some next) :
    next.privateState.session.state.source.waiting = some saved ∨
    next.privateState.session.state.source.waiting = none :=
  entry_pending_retained (QualifiedSession.rooted_continuing (rooted_private root).1) waiting
    (evaluate_exact.mp accepted).2.2.choose_spec.1

theorem live_pending_exact
    (root : Rooted realm view policy identity s)
    (waiting : s.privateState.session.state.source.waiting = some saved)
    (accepted : evaluate s command = some next)
    (retained : next.privateState.session.state.source.waiting = some other) : other = saved := by
  rcases pending_retained root waiting accepted with same | cleared
  · exact Option.some.inj (retained.symm.trans same)
  · rw [retained] at cleared
    cases cleared

#print axioms private_exact
#print axioms evaluate_exact
#print axioms evaluate_complete
#print axioms rooted_private
#print axioms initiation_current
#print axioms changed_incarnation_refused
#print axioms departed_refused
#print axioms stamp_retained
#print axioms pending_retained
#print axioms live_pending_exact
end MirroreaProofFirst.QualifiedCustody
