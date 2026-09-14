import MirroreaProofFirstPublicationOutcome
import MirroreaProofFirstReceivedResult
import MirroreaProofFirstPublicationProgress

namespace MirroreaProofFirst.PublicationExecution
open ReferenceContinuation

-- One command language for source driving AND publication preparation. The
-- reference-only waiting tick is absent from this executable boundary.
-- `received` is explicit mathematical input, not a proof of owner provenance.
inductive Command (p a : Nat) where
  | source (command : PublicationSession.Command p a)
  | received (entry : ReferenceExecution.Pending) (value : Int)

inductive Result where
  | source (result : PublicationOutcome.Result)
  | received (value : Int)
  deriving DecidableEq, Repr

def ReadyFor (s : Session p a) : PublicationSession.Command p a → Prop
  | .tick .. => s.status = .ready
  | _ => True

def readyCheck (s : Session p a) : PublicationSession.Command p a → Bool
  | .tick .. => decide (s.status = .ready)
  | _ => true

theorem ready_exact (s : Session p a) (command : PublicationSession.Command p a) :
    readyCheck s command = true ↔ ReadyFor s command := by
  cases command <;> simp [readyCheck,ReadyFor]

inductive Entry : Session p a → Command p a → Session p a → Result → Prop where
  | source : ReadyFor s command → PublicationOutcome.Entry s command next result →
      Entry s (.source command) next (.source result)
  | received : ReceivedResult.arrive s entry value = some next →
      Entry s (.received entry value) next (.received value)

def evaluateResult (s : Session p a) : Command p a → Option (Session p a × Result)
  | .source command =>
      if readyCheck s command then
        (PublicationOutcome.evaluateResult s command).map fun pair => (pair.1,.source pair.2)
      else none
  | .received entry value => (ReceivedResult.arrive s entry value).map fun next => (next,.received value)

theorem evaluateResult_sound (accepted : evaluateResult s command = some (next,result)) :
    Entry s command next result := by
  cases command with
  | source command =>
      simp only [evaluateResult] at accepted
      split at accepted
      · rename_i ready
        cases run : PublicationOutcome.evaluateResult s command with
        | none => simp [run] at accepted
        | some pair =>
            obtain ⟨state,value⟩ := pair
            simp only [run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
            obtain ⟨rfl,rfl⟩ := accepted
            exact .source ((ready_exact _ _).mp ready) (PublicationOutcome.evaluateResult_sound run)
      · cases accepted
  | received entry value =>
      cases run : ReceivedResult.arrive s entry value with
      | none => simp [evaluateResult,run] at accepted
      | some state =>
          simp only [evaluateResult,run,Option.map_some,Option.some.injEq,Prod.mk.injEq] at accepted
          obtain ⟨rfl,rfl⟩ := accepted
          exact .received run

theorem evaluateResult_complete (entry : Entry s command next result) :
    evaluateResult s command = some (next,result) := by
  cases entry with
  | source ready entry =>
      simp [evaluateResult,(ready_exact _ _).mpr ready,PublicationOutcome.evaluateResult_complete entry]
  | received accepted => simp [evaluateResult,accepted]

theorem evaluateResult_exact :
    evaluateResult s command = some (next,result) ↔ Entry s command next result :=
  ⟨evaluateResult_sound,evaluateResult_complete⟩

theorem entry_erasure {s next : Session p a} (entry : Entry s command next result) (member : Fin a) (principal : Nat) :
    ReferenceSession.Step s next := by
  cases entry with
  | source _ entry => exact PublicationOutcome.entry_erasure entry
  | received accepted => rw [ReceivedResult.arrive_tick accepted member principal]; exact .tick

theorem waiting_tick_rejected (waiting : s.status = .waiting) (member : Fin a) (principal : Nat) :
    evaluateResult s (.source (.tick member principal)) = none := by
  simp [evaluateResult,readyCheck,waiting]

private theorem cancellation_writes (s : Session p a) (member : Fin a) (principal : Nat) :
    (cancel s member principal).state.writes = s.state.writes := by
  dsimp only [cancel]
  split
  · cases waiting : s.state.waiting with
    | none => simp [ReferenceSource.cancel,waiting]
    | some saved =>
        cases cancelled : ReferenceExecution.cancel s.state.machine member s.program.place principal
          s.state.nextRequest saved.entry <;> simp [ReferenceSource.cancel,waiting,cancelled,ReferenceSource.mark]
  · rfl

-- An arbitrary waiting state cannot acquire a source write through any source
-- command in this closed language. Cancellation and metadata controls remain
-- admitted where W3 admits them; a successful cancellation is not a result.
theorem waiting_source_frames_writes (waiting : s.status = .waiting)
    (entry : Entry s (.source command) next result) : next.state.writes = s.state.writes := by
  cases entry with
  | source ready entry =>
      cases entry with
      | tick => simp [ReadyFor,waiting] at ready
      | cancellation => exact cancellation_writes _ _ _
      | head accepted => rw [(ReferenceSession.head_parts _ _ _ accepted).2]; rfl
      | control accepted =>
          obtain ⟨state,run,rfl⟩ := ReferenceSession.control_parts _ _ _ _ _ _ _ accepted
          exact (ReferenceSource.controlInput_projects _ _ _ _ _ _ run).2.2.2
      | replace accepted => rw [(replace_state _ _ _ accepted).1]
      | continueWith accepted => rw [(continue_state _ _ _ accepted).1]

theorem waiting_write_requires_received (waiting : s.status = .waiting)
    (entry : Entry s command next result) (changed : next.state.writes ≠ s.state.writes) :
    ∃ pending value, command = .received pending value ∧
      ReceivedResult.arrive s pending value = some next := by
  cases command with
  | source command => exact False.elim (changed (waiting_source_frames_writes waiting entry))
  | received pending value =>
      cases entry with
      | received accepted => exact ⟨pending,value,rfl,accepted⟩

-- This total immutable model wrapper records a coarse refused-input outcome.
-- It cannot infer a particular wire failure reason from Option.none. A later
-- mutable implementation must prove correspondence before using this frame.
inductive Attempt where
  | refused
  | accepted (result : Result)
  deriving DecidableEq, Repr

def attempt (s : Session p a) (command : Command p a) : Session p a × Attempt :=
  match evaluateResult s command with
  | none => (s,.refused)
  | some (next,result) => (next,.accepted result)

theorem refused_frame (refused : evaluateResult s command = none) :
    attempt s command = (s,.refused) := by simp [attempt,refused]

-- This is the same evaluator used by every stage below, not a separate
-- unrestricted PublicationOutcome / ReferenceContinuation tick fallback.
def evaluate (s : Session p a) (command : Command p a) : Option (Session p a) :=
  (evaluateResult s command).map Prod.fst

theorem evaluate_exact : evaluate s command = some next ↔ ∃ result, Entry s command next result := by
  constructor
  · intro accepted
    cases run : evaluateResult s command with
    | none => simp [evaluate,run] at accepted
    | some pair =>
        obtain ⟨state,result⟩ := pair
        simp only [evaluate,run,Option.map_some,Option.some.injEq] at accepted
        subst next
        exact ⟨result,evaluateResult_sound run⟩
  · rintro ⟨result,entry⟩
    simp [evaluate,evaluateResult_complete entry]

theorem publication_entry
    (path : PublicationPayload.Reached evaluate n revision initial s)
    (allowed : PublicationPayload.Allowed evaluate s .publish) :
    ∃ command result, Entry s.current command (PublicationPayload.apply evaluate s .publish).current result := by
  obtain ⟨pair,prepared⟩ := allowed.2
  have accepted := (PublicationPayload.reached_invariant path).preparedFromCurrent pair prepared
  obtain ⟨result,entry⟩ := evaluate_exact.mp accepted
  exact ⟨pair.2,result,by simpa [PublicationPayload.apply,prepared] using entry⟩

theorem publication_waiting_write_requires_received
    (path : PublicationPayload.Reached evaluate n revision initial s)
    (allowed : PublicationPayload.Allowed evaluate s .publish)
    (waiting : s.current.status = .waiting)
    (changed : (PublicationPayload.apply evaluate s .publish).current.state.writes ≠ s.current.state.writes) :
    ∃ pending value, ReceivedResult.arrive s.current pending value =
      some (PublicationPayload.apply evaluate s .publish).current := by
  obtain ⟨command,result,entry⟩ := publication_entry path allowed
  obtain ⟨pending,value,_,received⟩ := waiting_write_requires_received waiting entry changed
  exact ⟨pending,value,received⟩

#print axioms evaluateResult_exact
#print axioms entry_erasure
#print axioms waiting_tick_rejected
#print axioms waiting_source_frames_writes
#print axioms waiting_write_requires_received
#print axioms refused_frame
#print axioms evaluate_exact
#print axioms publication_entry
#print axioms publication_waiting_write_requires_received
end MirroreaProofFirst.PublicationExecution

namespace MirroreaProofFirst.PublicationExecution
open ReferenceContinuation

-- The result-retaining publication interface. The session-only evaluator
-- above is its erasure, not the eventual physical completion interface.
structure Reply (p a : Nat) where
  ordinal : Nat
  command : Command p a
  result : Result

structure Snapshot (p a : Nat) where
  session : Session p a
  replies : List (Reply p a)

def snapshotEvaluate (s : Snapshot p a) (command : Command p a) : Option (Snapshot p a) :=
  (evaluateResult s.session command).map fun pair =>
    ⟨pair.1,⟨s.replies.length,command,pair.2⟩ :: s.replies⟩

inductive Transition : Snapshot p a → Command p a → Snapshot p a → Prop where
  | entry : Entry s.session command next result →
      Transition s command ⟨next,⟨s.replies.length,command,result⟩ :: s.replies⟩

theorem snapshot_exact : snapshotEvaluate s command = some next ↔ Transition s command next := by
  constructor
  · intro accepted
    cases run : evaluateResult s.session command with
    | none => simp [snapshotEvaluate,run] at accepted
    | some pair =>
        obtain ⟨session,result⟩ := pair
        simp only [snapshotEvaluate,run,Option.map_some,Option.some.injEq] at accepted
        subst next
        exact .entry (evaluateResult_sound run)
  · intro step
    cases step with
    | entry accepted => simp [snapshotEvaluate,evaluateResult_complete accepted]

theorem transition_outcome (step : Transition s command next) :
    ∃ result, Entry s.session command next.session result ∧
      next.replies = ⟨s.replies.length,command,result⟩ :: s.replies := by
  cases step with
  | entry accepted => exact ⟨_,accepted,rfl⟩

def Ordinals (s : Snapshot p a) : Prop :=
  s.replies.map Reply.ordinal = (List.range s.replies.length).reverse

theorem empty_ordinals (s : Session p a) : Ordinals (Snapshot.mk s []) := rfl

theorem transition_ordinals (valid : Ordinals s) (step : Transition s command next) : Ordinals next := by
  cases step with
  | entry accepted =>
      simp only [Ordinals,List.map_cons,List.length_cons]
      rw [List.range_succ,List.reverse_append]
      simpa only [List.reverse_singleton,List.singleton_append] using congrArg (List.cons s.replies.length) valid

theorem ordinals_unique (valid : Ordinals s) : (s.replies.map Reply.ordinal).Nodup := by
  rw [valid]
  rw [List.nodup_iff_pairwise_ne,List.pairwise_reverse]
  exact List.nodup_range.imp (fun different => Ne.symm different)

theorem publication_outcome
    (path : PublicationPayload.Reached snapshotEvaluate n revision initial s)
    (allowed : PublicationPayload.Allowed snapshotEvaluate s .publish) :
    ∃ command result,
      Entry s.current.session command (PublicationPayload.apply snapshotEvaluate s .publish).current.session result ∧
      (PublicationPayload.apply snapshotEvaluate s .publish).current.replies =
        ⟨s.current.replies.length,command,result⟩ :: s.current.replies := by
  obtain ⟨pair,prepared⟩ := allowed.2
  have accepted := (PublicationPayload.reached_invariant path).preparedFromCurrent pair prepared
  obtain ⟨result,entry,history⟩ := transition_outcome (snapshot_exact.mp accepted)
  exact ⟨pair.2,result,by simpa [PublicationPayload.apply,prepared] using entry,
    by simpa [PublicationPayload.apply,prepared] using history⟩

theorem publication_received_outcome
    (path : PublicationPayload.Reached snapshotEvaluate n revision initial s)
    (allowed : PublicationPayload.Allowed snapshotEvaluate s .publish)
    (waiting : s.current.session.status = .waiting)
    (changed : (PublicationPayload.apply snapshotEvaluate s .publish).current.session.state.writes ≠
      s.current.session.state.writes) :
    ∃ pending value,
      ReceivedResult.arrive s.current.session pending value =
        some (PublicationPayload.apply snapshotEvaluate s .publish).current.session ∧
      (PublicationPayload.apply snapshotEvaluate s .publish).current.replies =
        ⟨s.current.replies.length,.received pending value,.received value⟩ :: s.current.replies := by
  obtain ⟨command,result,entry,history⟩ := publication_outcome path allowed
  obtain ⟨pending,value,equal,received⟩ := waiting_write_requires_received waiting entry changed
  subst command
  cases entry with
  | received accepted => exact ⟨pending,value,received,history⟩

#print axioms snapshot_exact
#print axioms transition_ordinals
#print axioms ordinals_unique
#print axioms publication_outcome
#print axioms publication_received_outcome
end MirroreaProofFirst.PublicationExecution

namespace MirroreaProofFirst.PublicationExecution
open ReferenceContinuation

theorem reached_rooted_ordinals {initialSession : Session p a}
    (root : ReferenceSession.Rooted realm view policy initialSession)
    (member : Fin a) (principal : Nat)
    (path : PublicationPayload.Reached snapshotEvaluate n revision (Snapshot.mk initialSession []) s) :
    ReferenceSession.Rooted realm view policy s.current.session ∧ Ordinals s.current := by
  induction path with
  | initial => exact ⟨root,empty_ordinals _⟩
  | step previous step ih =>
      cases step with
      | action allowed =>
          have refined := PublicationPayload.action_refines
            (fun old next => ∃ command, Transition old command next)
            (fun _ command _ accepted => ⟨command,snapshot_exact.mp accepted⟩)
            (PublicationPayload.reached_invariant previous) allowed
          rcases refined with unchanged | ⟨command,transition⟩
          · rw [unchanged]; exact ih
          · obtain ⟨result,entry,_⟩ := transition_outcome transition
            exact ⟨.step ih.1 (entry_erasure entry member principal),transition_ordinals ih.2 transition⟩

-- Relative completeness from an independent admitted Entry, not a premise
-- that the publication implementation already accepted the requested round.
-- This is a finite possible schedule, not a fairness or availability promise.
theorem entry_has_normal_publication {s : PublicationUse.State n (Snapshot p a) (Command p a)}
    (nonempty : 0 < n) (path : PublicationUse.Reached snapshotEvaluate n revision initial s)
    (settled : s.base.barrier.announced = s.base.barrier.published)
    (free : ∀ i, s.held i = none) (entry : Entry s.base.current.session command session result) :
    ∃ next,
      PublicationProgress.useRun snapshotEvaluate s
        ((PublicationProgress.normal n (s.base.barrier.published+1)).map (PublicationProgress.lift command)) = some next ∧
      PublicationUse.Reached snapshotEvaluate n revision initial next ∧
      next.base.current.session = session ∧
      next.base.current.replies = ⟨s.base.current.replies.length,command,result⟩ :: s.base.current.replies ∧
      ∀ i, next.base.cached i = next.base.current ∧
        PublicationUse.check snapshotEvaluate next (.enter i) = true := by
  let outcome : Snapshot p a := ⟨session,⟨s.base.current.replies.length,command,result⟩ :: s.base.current.replies⟩
  have computed : snapshotEvaluate s.base.current command = some outcome :=
    snapshot_exact.mpr (.entry entry)
  obtain ⟨next,run,reached,current,_,_,all,_⟩ :=
    PublicationProgress.normal_payload_succeeds nonempty path settled free computed
  exact ⟨next,run,reached,congrArg Snapshot.session current,congrArg Snapshot.replies current,
    fun i => ⟨(all i).1.trans current.symm,(all i).2.2⟩⟩

#print axioms reached_rooted_ordinals
#print axioms entry_has_normal_publication
end MirroreaProofFirst.PublicationExecution
