import MirroreaProofFirstPublicationPayload
import MirroreaProofFirstReferenceSession

namespace MirroreaProofFirst.PublicationSession
open ReferenceContinuation

-- Reified existing W3 entries. No new source grammar or new authority policy.
-- This consumer discharges the generic evaluator premise against the separate,
-- already defined Session.Step. It does not implement a process transport.
inductive Command (p a : Nat) where
  | tick (member : Fin a) (principal : Nat)
  | cancellation (member : Fin a) (principal : Nat)
  | head (view : WorldProjection.AuthorityView a)
  | control (member : Fin a) (place : Fin p) (principal : Nat) (raw : CompositionCore.Raw)
  | replace (program : Program p)
  | continueWith (program : Program p)

def evaluate (s : Session p a) : Command p a → Option (Session p a)
  | .tick member principal => some (tick s member principal)
  | .cancellation member principal => some (cancel s member principal)
  | .head view => authorityHead s view
  | .control member place principal raw => (controlInput s member place principal raw).map Prod.fst
  | .replace program => replaceResidual s program
  | .continueWith program => ReferenceContinuation.continueWith s program

theorem evaluate_sound (s : Session p a) (command : Command p a) (next : Session p a)
    (accepted : evaluate s command = some next) : ReferenceSession.Step s next := by
  cases command with
  | tick member principal =>
      cases accepted
      exact .tick
  | cancellation member principal =>
      cases accepted
      exact .cancellation
  | head view => exact .head accepted
  | control member place principal raw =>
      cases run : controlInput s member place principal raw with
      | none => simp [evaluate,run] at accepted
      | some pair =>
          obtain ⟨result,created⟩ := pair
          simp only [evaluate,run,Option.map_some,Option.some.injEq] at accepted
          subst next
          exact .control run
  | replace program => exact .replace accepted
  | continueWith program => exact .continueWith accepted

theorem evaluate_complete (s next : Session p a) (step : ReferenceSession.Step s next) :
    ∃ command, evaluate s command = some next := by
  cases step with
  | tick => exact ⟨.tick _ _,rfl⟩
  | cancellation => exact ⟨.cancellation _ _,rfl⟩
  | head accepted => exact ⟨.head _,accepted⟩
  | control accepted =>
      rename_i member place principal raw created
      exact ⟨.control member place principal raw,by simp only [evaluate,accepted,Option.map_some]⟩
  | replace accepted => exact ⟨.replace _,accepted⟩
  | continueWith accepted => exact ⟨.continueWith _,accepted⟩

theorem evaluate_exact (s next : Session p a) :
    (∃ command, evaluate s command = some next) ↔ ReferenceSession.Step s next :=
  ⟨fun ⟨command,accepted⟩ => evaluate_sound s command next accepted,evaluate_complete _ _⟩

theorem publication_refines
    (valid : PublicationPayload.Invariant evaluate s)
    (allowed : PublicationPayload.Allowed evaluate s action) :
    (PublicationPayload.apply evaluate s action).current = s.current ∨
      ReferenceSession.Step s.current (PublicationPayload.apply evaluate s action).current :=
  PublicationPayload.action_refines ReferenceSession.Step evaluate_sound valid allowed

theorem reached_rooted (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (initialSession : Session p a)
    (root : ReferenceSession.Rooted realm view policy initialSession)
    (path : PublicationPayload.Reached evaluate n revision initialSession s) :
    ReferenceSession.Rooted realm view policy s.current := by
  induction path with
  | initial => exact root
  | step previous step ih =>
      cases step with
      | action allowed =>
          rcases publication_refines (PublicationPayload.reached_invariant previous) allowed with unchanged | advanced
          · rw [unchanged]; exact ih
          · exact .step ih advanced

theorem enabled_session_current
    (path : PublicationPayload.Reached evaluate n revision initialSession s) (i : Fin n)
    (enabled : PublicationPayload.check evaluate s (.use i) = true) : s.cached i = s.current :=
  PublicationPayload.enabled_payload_current path i enabled

theorem enabled_rooted (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (initialSession : Session p a)
    (root : ReferenceSession.Rooted realm view policy initialSession)
    (path : PublicationPayload.Reached evaluate n revision initialSession s) (i : Fin n)
    (enabled : PublicationPayload.check evaluate s (.use i) = true) :
    ReferenceSession.Rooted realm view policy (s.cached i) := by
  rw [enabled_session_current path i enabled]
  exact reached_rooted _ _ _ _ root path

-- Any existing checker reads the same full Session when the endpoint is open;
-- this does not promote successful validation to permission to act later after
-- leaving the serialization gate. The real delayed-use gap remains an obligation.
theorem enabled_check_agrees (validation : Session p a → Bool)
    (path : PublicationPayload.Reached evaluate n revision initialSession s) (i : Fin n)
    (enabled : PublicationPayload.check evaluate s (.use i) = true) :
    validation (s.cached i) = validation s.current := by
  rw [enabled_session_current path i enabled]

theorem enabled_invariants (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (initialSession : Session p a)
    (root : ReferenceSession.Rooted realm view policy initialSession)
    (path : PublicationPayload.Reached evaluate n revision initialSession s) (i : Fin n)
    (enabled : PublicationPayload.check evaluate s (.use i) = true) :
    ReferenceSession.Metadata (s.cached i) ∧
      ReferenceSourceOrigins.Invariant (s.cached i).state ∧
      ReferenceOrigins.Invariant (s.cached i).state.machine.store ∧
      ReferenceSource.PendingAgrees (s.cached i).state :=
  ReferenceSession.rooted_invariants _ _ _ _ (enabled_rooted _ _ _ _ root path i enabled)

#print axioms evaluate_exact
#print axioms publication_refines
#print axioms reached_rooted
#print axioms enabled_session_current
#print axioms enabled_check_agrees
#print axioms enabled_invariants
end MirroreaProofFirst.PublicationSession
