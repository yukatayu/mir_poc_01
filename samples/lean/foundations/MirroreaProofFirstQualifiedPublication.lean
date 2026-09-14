import MirroreaProofFirstPublicationImage
import MirroreaProofFirstQualifiedSession
import MirroreaProofFirstOwnerEvaluator

namespace MirroreaProofFirst.QualifiedPublication
open QualifiedSession

-- All seven private source entries are represented here. Ordinary source
-- execution supplies tick; no caller changes or populated relaunch is added.
-- Remote transport still has to authenticate entry origin and actor custody.
inductive Command (p a : Nat) where
  | tick
  | receive (ticket : InvocationBoundary.Ticket) (value : Int)
  | cancel
  | head (view : WorldProjection.AuthorityView a)
  | control (member : Fin a) (place : Fin p) (principal : Nat) (raw : CompositionCore.Raw)
  | replace (program : Program p)
  | continueWith (program : Program p)

inductive Result where
  | tick (status : ReferenceSource.Status)
  | received (value : Int)
  | cancellation (status : ReferenceSource.Status)
  | head
  | control (created : Option Nat)
  | replace
  | continued
  deriving DecidableEq, Repr

inductive Entry : Session p a → Command p a → Session p a → Result → Prop where
  | tick : s.status = .ready → Entry s .tick (tick s) (.tick (tick s).status)
  | receive : receive s ticket value = some next → Entry s (.receive ticket value) next (.received value)
  | cancel : Entry s .cancel (cancel s) (.cancellation (QualifiedSource.cancel s.state (actor s)).status)
  | head : authorityHead s view = some next → Entry s (.head view) next .head
  | control : controlInput s member place principal raw = some (next,created) →
      Entry s (.control member place principal raw) next (.control created)
  | replace : replaceResidual s program = some next → Entry s (.replace program) next .replace
  | continueWith : QualifiedSession.continueWith s program = some next →
      Entry s (.continueWith program) next .continued

def evaluateResult (s : Session p a) : Command p a → Option (Session p a × Result)
  | .tick => if s.status = .ready then some (tick s,.tick (tick s).status) else none
  | .receive ticket value => (receive s ticket value).map (fun next => (next,.received value))
  | .cancel => some (cancel s,.cancellation (QualifiedSource.cancel s.state (actor s)).status)
  | .head view => (authorityHead s view).map (fun next => (next,.head))
  | .control member place principal raw => (controlInput s member place principal raw).map
      (fun pair => (pair.1,.control pair.2))
  | .replace program => (replaceResidual s program).map (fun next => (next,.replace))
  | .continueWith program => (QualifiedSession.continueWith s program).map (fun next => (next,.continued))

theorem evaluate_sound (accepted : evaluateResult s command = some (next,result)) :
    Entry s command next result := by
  cases command with
  | tick =>
      simp only [evaluateResult] at accepted
      split at accepted
      · rename_i ready
        cases accepted
        exact .tick ready
      · cases accepted
  | receive ticket value =>
      cases run : receive s ticket value <;> simp [evaluateResult,run] at accepted
      obtain ⟨rfl,rfl⟩ := accepted
      exact .receive run
  | cancel => cases accepted; exact .cancel
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
      cases run : QualifiedSession.continueWith s program <;> simp [evaluateResult,run] at accepted
      obtain ⟨rfl,rfl⟩ := accepted
      exact .continueWith run

theorem evaluate_complete (entry : Entry s command next result) :
    evaluateResult s command = some (next,result) := by
  cases entry with
  | tick ready => simp [evaluateResult,ready]
  | receive accepted => simp [evaluateResult,accepted]
  | cancel => rfl
  | head accepted => simp [evaluateResult,accepted]
  | control accepted => simp [evaluateResult,accepted]
  | replace accepted => simp [evaluateResult,accepted]
  | continueWith accepted => simp [evaluateResult,accepted]

theorem evaluate_exact : evaluateResult s command = some (next,result) ↔ Entry s command next result :=
  ⟨evaluate_sound,evaluate_complete⟩

theorem entry_step (entry : Entry s command next result) : QualifiedSession.Step s next := by
  cases entry with
  | tick _ => exact .tick
  | receive accepted => exact .receive accepted
  | cancel => exact .cancel
  | head accepted => exact .head accepted
  | control accepted => exact .control accepted
  | replace accepted => exact .replace accepted
  | continueWith accepted => exact .continueWith accepted

structure Reply (p a : Nat) where
  ordinal : Nat
  command : Command p a
  result : Result
structure Snapshot (p a : Nat) where
  session : Session p a
  replies : List (Reply p a)

def evaluate (s : Snapshot p a) (command : Command p a) : Option (Snapshot p a) :=
  (evaluateResult s.session command).map (fun pair => ⟨pair.1,⟨s.replies.length,command,pair.2⟩ :: s.replies⟩)

def image (s : Snapshot p a) : OwnerImage.Image p a :=
  OwnerImage.capture (OwnerProjection.project s.session.state.source)

theorem evaluated_entry (accepted : evaluate s command = some next) :
    ∃ result, Entry s.session command next.session result ∧
      next.replies = ⟨s.replies.length,command,result⟩ :: s.replies := by
  cases run : evaluateResult s.session command with
  | none => simp [evaluate,run] at accepted
  | some pair =>
      obtain ⟨state,result⟩ := pair
      simp only [evaluate,run,Option.map_some,Option.some.injEq] at accepted
      subst next
      exact ⟨result,evaluate_sound run,rfl⟩

theorem payload_rooted
    (root : QualifiedSession.Rooted realm view policy identity initial.session)
    (path : PublicationPayload.Reached evaluate n revision initial s) :
    QualifiedSession.Rooted realm view policy identity s.current.session := by
  induction path with
  | initial => exact root
  | step previous step ih =>
      cases step with
      | action allowed =>
          have refined := PublicationPayload.action_refines
            (fun old next => QualifiedSession.Step old.session next.session)
            (fun _ _ _ accepted => entry_step (evaluated_entry accepted).choose_spec.1)
            (PublicationPayload.reached_invariant previous) allowed
          rcases refined with unchanged | advanced
          · rw [unchanged]; exact ih
          · exact .step ih advanced

theorem abstract_rooted
    (root : QualifiedSession.Rooted realm view policy identity initial.session)
    (path : PublicationUse.Reached evaluate n revision initial s) :
    QualifiedSession.Rooted realm view policy identity s.base.current.session :=
  payload_rooted root (PublicationUse.reached_payload path)

theorem current_rooted
    (root : QualifiedSession.Rooted realm view policy identity initial.session)
    (path : PublicationImage.Reached image evaluate n revision initial s) :
    QualifiedSession.Rooted realm view policy identity s.current.session := by
  obtain ⟨abstract,reached,rfl⟩ := PublicationImage.reached_lifts path
  exact abstract_rooted root reached

-- This conclusion is about actual image-only transition data. The abstract
-- private Session representation is constructed by reached_lifts, never sent
-- to an owner and never used as an untrusted invariant certificate.
theorem held_evaluation
    {s : PublicationImage.State n (Snapshot p a) (Command p a) (OwnerImage.Image p a)}
    (path : PublicationImage.Reached image evaluate n revision initial s)
    (endpoint : Fin n) (pair : Nat × OwnerImage.Image p a)
    (held : s.held endpoint = some pair) (assigned : OwnerEvaluator.Assignment p)
    (ticket : InvocationBoundary.Ticket) (value : Int) :
    OwnerEvaluator.run assigned pair.2 ticket = .value value ↔
      OwnerEvaluator.Admitted assigned (image s.current) ticket ∧
        InstancePrograms.Machine.Executes ticket.definition.code ticket.argument value := by
  obtain ⟨_,current⟩ := PublicationImage.held_current path endpoint pair held
  rw [current]
  exact OwnerEvaluator.result_exact _ _ _ _

theorem entry_has_publication
    {s : PublicationImage.State n (Snapshot p a) (Command p a) (OwnerImage.Image p a)}
    (nonempty : 0 < n) (path : PublicationImage.Reached image evaluate n revision initial s)
    (settled : s.barrier.announced = s.barrier.published) (free : ∀ i, s.held i = none)
    (entry : Entry s.current.session command session result) :
    ∃ next, PublicationImage.run image evaluate s
        ((PublicationProgress.normal n (s.barrier.published+1)).map (PublicationProgress.lift command)) = some next ∧
      PublicationImage.Reached image evaluate n revision initial next ∧
      next.current.session = session ∧
      next.current.replies = ⟨s.current.replies.length,command,result⟩ :: s.current.replies ∧
      ∀ i, next.cached i = image next.current ∧ PublicationImage.check evaluate next (.enter i) = true := by
  let value : Snapshot p a := ⟨session,⟨s.current.replies.length,command,result⟩ :: s.current.replies⟩
  have computed : evaluate s.current command = some value := by
    simp [evaluate,evaluate_complete entry,value]
  obtain ⟨next,ran,reached,current,_,all⟩ := PublicationImage.normal_succeeds nonempty path settled free computed
  exact ⟨next,ran,reached,congrArg (fun x => x.session) current,congrArg (fun x => x.replies) current,
    fun i => ⟨(all i).1.trans (congrArg image current).symm,(all i).2.2⟩⟩

-- Reply ordinals are derived from retained length only on this empty-rooted
-- history. Session.Rooted alone says nothing about an arbitrary supplied log.
def start (session : Session p a) : Snapshot p a := ⟨session,[]⟩
def Ordinals (s : Snapshot p a) : Prop :=
  s.replies.map Reply.ordinal = (List.range s.replies.length).reverse

theorem start_ordinals : Ordinals (start session) := rfl

theorem evaluated_ordinals (valid : Ordinals s) (computed : evaluate s command = some next) :
    Ordinals next := by
  obtain ⟨result,_,replies⟩ := evaluated_entry computed
  simp only [Ordinals,replies,List.map_cons,List.length_cons]
  rw [List.range_succ,List.reverse_append]
  simpa only [List.reverse_singleton,List.singleton_append] using congrArg (List.cons s.replies.length) valid

theorem payload_ordinals
    (path : PublicationPayload.Reached evaluate n revision (start session) s) : Ordinals s.current := by
  induction path with
  | initial => exact start_ordinals
  | step previous step ih =>
      cases step with
      | action allowed =>
          have refined := PublicationPayload.action_refines
            (fun old next => ∃ command, evaluate old command = some next)
            (fun _ command _ accepted => ⟨command,accepted⟩)
            (PublicationPayload.reached_invariant previous) allowed
          rcases refined with unchanged | ⟨command,computed⟩
          · rw [unchanged]; exact ih
          · exact evaluated_ordinals ih computed

theorem reached_ordinals
    (path : PublicationImage.Reached image evaluate n revision (start session) s) : Ordinals s.current := by
  obtain ⟨abstract,reached,rfl⟩ := PublicationImage.reached_lifts path
  exact payload_ordinals (PublicationUse.reached_payload reached)

theorem ordinals_unique (valid : Ordinals s) : (s.replies.map Reply.ordinal).Nodup := by
  rw [valid]
  rw [List.nodup_iff_pairwise_ne,List.pairwise_reverse]
  exact List.nodup_range.imp (fun different => Ne.symm different)

#print axioms evaluated_ordinals
#print axioms payload_ordinals
#print axioms reached_ordinals
#print axioms ordinals_unique

#print axioms evaluate_exact
#print axioms entry_step
#print axioms evaluated_entry
#print axioms abstract_rooted
#print axioms current_rooted
#print axioms held_evaluation
#print axioms entry_has_publication
end MirroreaProofFirst.QualifiedPublication
