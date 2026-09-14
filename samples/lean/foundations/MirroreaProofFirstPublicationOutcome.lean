import MirroreaProofFirstPublicationSession

namespace MirroreaProofFirst.PublicationOutcome
open ReferenceContinuation PublicationSession

-- Results of the existing entries, before any transport encoding. A returned
-- cancellation status is the actual cancellation attempt's status, not an
-- inference that Option.some or a publication means domain success.
inductive Result where
  | tick (status : ReferenceSource.Status)
  | cancellation (status : ReferenceSource.Status)
  | head
  | control (created : Option Nat)
  | replace
  | continueWith
  deriving DecidableEq, Repr

inductive Entry : Session p a → Command p a → Session p a → Result → Prop where
  | tick : Entry s (.tick member principal) (tick s member principal)
      (.tick (tick s member principal).status)
  | cancellation : Entry s (.cancellation member principal) (cancel s member principal)
      (.cancellation (ReferenceSource.cancel s.state member s.program.place principal).status)
  | head : authorityHead s view = some next → Entry s (.head view) next .head
  | control : controlInput s member place principal raw = some (next,created) →
      Entry s (.control member place principal raw) next (.control created)
  | replace : replaceResidual s program = some next → Entry s (.replace program) next .replace
  | continueWith : ReferenceContinuation.continueWith s program = some next →
      Entry s (.continueWith program) next .continueWith

def evaluateResult (s : Session p a) : Command p a → Option (Session p a × Result)
  | .tick member principal =>
      let next := tick s member principal
      some (next,.tick next.status)
  | .cancellation member principal =>
      some (cancel s member principal,
        .cancellation (ReferenceSource.cancel s.state member s.program.place principal).status)
  | .head view => (authorityHead s view).map (fun next => (next,.head))
  | .control member place principal raw =>
      (controlInput s member place principal raw).map (fun pair => (pair.1,.control pair.2))
  | .replace program => (replaceResidual s program).map (fun next => (next,.replace))
  | .continueWith program =>
      (ReferenceContinuation.continueWith s program).map (fun next => (next,.continueWith))

theorem evaluateResult_sound (accepted : evaluateResult s command = some (next,result)) :
    Entry s command next result := by
  cases command with
  | tick member principal => cases accepted; exact .tick
  | cancellation member principal => cases accepted; exact .cancellation
  | head view =>
      cases run : authorityHead s view <;> simp [evaluateResult,run] at accepted
      obtain ⟨rfl,rfl⟩ := accepted
      exact .head run
  | control member place principal raw =>
      cases run : controlInput s member place principal raw <;> simp [evaluateResult,run] at accepted
      obtain ⟨rfl,rfl⟩ := accepted
      exact .control run
  | replace program =>
      cases run : replaceResidual s program <;> simp [evaluateResult,run] at accepted
      obtain ⟨rfl,rfl⟩ := accepted
      exact .replace run
  | continueWith program =>
      cases run : ReferenceContinuation.continueWith s program <;> simp [evaluateResult,run] at accepted
      obtain ⟨rfl,rfl⟩ := accepted
      exact .continueWith run

theorem evaluateResult_complete (entry : Entry s command next result) :
    evaluateResult s command = some (next,result) := by
  cases entry with
  | tick => rfl
  | cancellation => rfl
  | head accepted => simp [evaluateResult,accepted]
  | control accepted => simp [evaluateResult,accepted]
  | replace accepted => simp [evaluateResult,accepted]
  | continueWith accepted => simp [evaluateResult,accepted]

theorem evaluateResult_exact :
    evaluateResult s command = some (next,result) ↔ Entry s command next result :=
  ⟨evaluateResult_sound,evaluateResult_complete⟩

theorem result_erasure (s : Session p a) (command : Command p a) :
    (evaluateResult s command).map Prod.fst = PublicationSession.evaluate s command := by
  cases command <;> simp [evaluateResult,PublicationSession.evaluate,Option.map_map,Function.comp_def]

theorem entry_erasure (entry : Entry s command next result) : ReferenceSession.Step s next := by
  apply PublicationSession.evaluate_sound s command next
  rw [← result_erasure,evaluateResult_complete entry]
  rfl

-- Ordinals are generated from retained reply history, never supplied by an
-- ordinary source programmer. This is a reference history, not a codec or a
-- distributed inbox. Namespace ownership and finite allocation are separate.
structure Reply (p a : Nat) where
  ordinal : Nat
  command : Command p a
  result : Result

structure Snapshot (p a : Nat) where
  session : Session p a
  replies : List (Reply p a)

def evaluate (s : Snapshot p a) (command : Command p a) : Option (Snapshot p a) :=
  (evaluateResult s.session command).map fun pair =>
    ⟨pair.1,⟨s.replies.length,command,pair.2⟩ :: s.replies⟩

inductive Transition : Snapshot p a → Command p a → Snapshot p a → Prop where
  | entry : Entry s.session command next result →
      Transition s command ⟨next,⟨s.replies.length,command,result⟩ :: s.replies⟩

theorem evaluate_exact : evaluate s command = some next ↔ Transition s command next := by
  constructor
  · intro accepted
    cases run : evaluateResult s.session command with
    | none => simp [evaluate,run] at accepted
    | some pair =>
        obtain ⟨session,result⟩ := pair
        simp only [evaluate,run,Option.map_some,Option.some.injEq] at accepted
        subst next
        exact .entry (evaluateResult_sound run)
  · intro step
    cases step with
    | entry accepted => simp [evaluate,evaluateResult_complete accepted]

theorem transition_outcome (step : Transition s command next) :
    ∃ result, Entry s.session command next.session result ∧
      next.replies = ⟨s.replies.length,command,result⟩ :: s.replies := by
  cases step with
  | entry accepted => exact ⟨_,accepted,rfl⟩

theorem publication_outcome
    (path : PublicationPayload.Reached evaluate n revision initial s)
    (allowed : PublicationPayload.Allowed evaluate s .publish) :
    ∃ command result,
      Entry s.current.session command (PublicationPayload.apply evaluate s .publish).current.session result ∧
      (PublicationPayload.apply evaluate s .publish).current.replies =
        ⟨s.current.replies.length,command,result⟩ :: s.current.replies := by
  obtain ⟨pair,prepared⟩ := allowed.2
  have accepted := (PublicationPayload.reached_invariant path).preparedFromCurrent pair prepared
  obtain ⟨result,entry,history⟩ := transition_outcome (evaluate_exact.mp accepted)
  exact ⟨pair.2,result,by simpa [PublicationPayload.apply,prepared] using entry,
    by simpa [PublicationPayload.apply,prepared] using history⟩

theorem cancellation_denied_is_unchanged (s : Session p a) (member : Fin a) (principal : Nat)
    (denied : (ReferenceSource.cancel s.state member s.program.place principal).status ≠ .ready) :
    evaluateResult s (.cancellation member principal) =
      some (s,.cancellation (ReferenceSource.cancel s.state member s.program.place principal).status) := by
  simp [evaluateResult,ReferenceContinuation.cancel,denied]

theorem control_result_preserved
    (entry : Entry s (.control member place principal raw) next result) :
    ∃ created, result = .control created ∧
      controlInput s member place principal raw = some (next,created) := by
  cases entry with
  | control accepted => exact ⟨_,rfl,accepted⟩

#print axioms evaluateResult_exact
#print axioms result_erasure
#print axioms entry_erasure
#print axioms evaluate_exact
#print axioms publication_outcome
#print axioms cancellation_denied_is_unchanged
#print axioms control_result_preserved
end MirroreaProofFirst.PublicationOutcome
