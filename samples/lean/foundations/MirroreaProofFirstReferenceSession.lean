import MirroreaProofFirstReferenceContinuation
import MirroreaProofFirstReferenceSourceOrigins

namespace MirroreaProofFirst.ReferenceSession
open ReferenceContinuation ReferenceSourceData

-- Reconstructed source partition, independent of the driver implementation.
def Partition (program : Program p) (completed : List Located) (stopped : Option Located)
    (remaining : List Located) : Prop := program.items = completed ++ stopped.toList ++ remaining

def ArchiveValid (old : Superseded p) : Prop :=
  Partition old.program old.completed old.stopped old.remaining ∧
    (old.status = .ready → old.stopped = none)

def Metadata (s : Session p a) : Prop :=
  Partition s.program s.completed s.stopped s.remaining ∧
    (s.status = .ready → s.stopped = none) ∧ ∀ old ∈ s.superseded, ArchiveValid old

theorem launch_metadata (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
    (program : Program p) (s : Session p a) (accepted : launch realm view policy program = some s) : Metadata s := by
  rw [launch_state _ _ _ _ _ accepted]
  simp [Metadata,Partition]

theorem tick_metadata (s : Session p a) (member : Fin a) (principal : Nat) (valid : Metadata s) :
    Metadata (tick s member principal) := by
  obtain ⟨partition,ready,archived⟩ := valid
  cases phase : s.status with
  | failed reason => simpa [tick,phase] using (show Metadata s from ⟨partition,ready,archived⟩)
  | waiting =>
      simp only [tick,phase]
      by_cases finished : (ReferenceSource.complete s.state).status = .ready
      · simp only [finished,ite_true]
        exact ⟨by simpa [Partition,List.append_assoc] using partition,fun _ => rfl,archived⟩
      · simp only [finished,ite_false]
        exact ⟨partition,fun impossible => False.elim (finished impossible),archived⟩
  | ready =>
      have noStopped := ready phase
      cases remaining : s.remaining with
      | nil => simpa [tick,phase,remaining] using (show Metadata s from ⟨partition,ready,archived⟩)
      | cons item rest =>
          simp only [tick,phase,remaining]
          by_cases finished : (ReferenceSource.advance s.state member s.program.place principal item).status = .ready
          · simp only [finished,ite_true]
            refine ⟨?_,fun _ => rfl,archived⟩
            simpa [Partition,noStopped,remaining,List.append_assoc] using partition
          · simp only [finished,ite_false]
            refine ⟨?_,fun impossible => False.elim (finished impossible),archived⟩
            simpa [Partition,noStopped,remaining,List.append_assoc] using partition

theorem cancel_metadata (s : Session p a) (member : Fin a) (principal : Nat) (valid : Metadata s) :
    Metadata (cancel s member principal) := by
  simp only [cancel]
  split
  · refine ⟨valid.1,?_,valid.2.2⟩
    intro impossible
    cases impossible
  · exact valid

theorem adopted_metadata (s next : Session p a) (program : Program p) (valid : Metadata s)
    (accepted : Adopted s next program) : Metadata next := by
  obtain ⟨_,programAt,completed,remaining,stopped,status,history⟩ := accepted
  refine ⟨?_,?_,?_⟩
  · simp [Partition,programAt,completed,remaining,stopped]
  · intro _; exact stopped
  · intro old present
    rw [history] at present
    rcases List.mem_cons.mp present with rfl | past
    · exact ⟨valid.1,valid.2.1⟩
    · exact valid.2.2 old past

theorem head_parts (s next : Session p a) (view : WorldProjection.AuthorityView a)
    (accepted : authorityHead s view = some next) :
    ReferenceAuthority.Successor s.state.machine.store.core.system.view view ∧
      next = {s with state := ReferenceSource.authorityHead s.state view} := by
  unfold authorityHead at accepted
  cases run : ReferenceAuthority.install s.state view with
  | none => simp [run] at accepted
  | some state =>
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      obtain ⟨admissible,rfl⟩ := ReferenceAuthority.install_parts _ _ _ run
      exact ⟨admissible,accepted.symm⟩

theorem control_parts (s next : Session p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) (created : Option Nat)
    (accepted : controlInput s member place principal raw = some (next,created)) :
    ∃ state, ReferenceSource.controlInput s.state member place principal raw = some (state,created) ∧
      next = {s with state := state} := by
  unfold controlInput at accepted
  cases run : ReferenceSource.controlInput s.state member place principal raw with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨state,key⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at accepted
      obtain ⟨rfl,rfl⟩ := accepted
      exact ⟨state,rfl,rfl⟩

-- Actual session entry closure: no raw record replacement or populated-state
-- relaunch. New source admission retains its parsed Program.place and all prior
-- archival records. Authority install is the checked successor profile.
inductive Step : Session p a → Session p a → Prop where
  | tick : Step s (ReferenceContinuation.tick s member principal)
  | cancellation : Step s (ReferenceContinuation.cancel s member principal)
  | head : authorityHead s view = some next → Step s next
  | control : controlInput s member place principal raw = some (next,created) → Step s next
  | replace : replaceResidual s program = some next → Step s next
  | continueWith : continueWith s program = some next → Step s next

inductive Rooted (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy) :
    Session p a → Prop where
  | launch : ReferenceContinuation.launch realm view policy program = some s → Rooted realm view policy s
  | step : Rooted realm view policy s → Step s next → Rooted realm view policy next

theorem step_metadata (s next : Session p a) (valid : Metadata s) (step : Step s next) : Metadata next := by
  cases step with
  | tick => exact tick_metadata _ _ _ valid
  | cancellation => exact cancel_metadata _ _ _ valid
  | head run => rw [(head_parts _ _ _ run).2]; exact valid
  | control run => obtain ⟨_,_,rfl⟩ := control_parts _ _ _ _ _ _ _ run; exact valid
  | replace run => exact adopted_metadata _ _ _ valid (replace_state _ _ _ run)
  | continueWith run => exact adopted_metadata _ _ _ valid (continue_state _ _ _ run)

theorem step_source (s next : Session p a) (step : Step s next) : ReferenceSourceTrace.Reached s.state next.state := by
  cases step with
  | tick => exact tick_source _ _ _
  | cancellation => exact cancel_source _ _ _
  | head run => rw [(head_parts _ _ _ run).2]; exact .step .refl .head
  | control run => obtain ⟨_,committed,rfl⟩ := control_parts _ _ _ _ _ _ _ run; exact .step .refl (.control committed)
  | replace run => rw [(replace_state _ _ _ run).1]; exact .refl
  | continueWith run => rw [(continue_state _ _ _ run).1]; exact .refl

theorem rooted_metadata (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
    (s : Session p a) (path : Rooted realm view policy s) : Metadata s := by
  induction path with
  | launch run => exact launch_metadata _ _ _ _ _ run
  | step previous step ih => exact step_metadata _ _ ih step

theorem rooted_source (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
    (s : Session p a) (path : Rooted realm view policy s) : ReferenceSourceTrace.Rooted realm view policy s.state := by
  induction path with
  | launch run => rw [launch_state _ _ _ _ _ run]; exact .refl
  | step previous step ih => exact ReferenceSourceTrace.trans _ _ _ ih (step_source _ _ step)

theorem rooted_invariants (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
    (s : Session p a) (path : Rooted realm view policy s) :
    Metadata s ∧ ReferenceSourceOrigins.Invariant s.state ∧ ReferenceOrigins.Invariant s.state.machine.store ∧
      ReferenceSource.PendingAgrees s.state := by
  have source := rooted_source _ _ _ _ path
  exact ⟨rooted_metadata _ _ _ _ path,ReferenceSourceOrigins.rooted_aligned _ _ _ _ source,
    ReferenceOrigins.rooted_origins _ _ _ _ source,ReferenceSourceTrace.rooted_pending _ _ _ _ source⟩

theorem drive_rooted (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
    (fuel : Nat) (s : Session p a) (member : Fin a) (principal : Nat) (path : Rooted realm view policy s) :
    Rooted realm view policy (drive fuel s member principal) := by
  induction fuel generalizing s with
  | zero => exact path
  | succ fuel ih => exact ih _ (.step path .tick)

#print axioms drive_rooted

#print axioms launch_metadata
#print axioms tick_metadata
#print axioms adopted_metadata
#print axioms step_metadata
#print axioms rooted_metadata
#print axioms rooted_source
#print axioms rooted_invariants
end MirroreaProofFirst.ReferenceSession

namespace MirroreaProofFirst.ReferenceSession
open ReferenceContinuation ReferenceSourceData

private def OutcomeAt (site : Site) (result : ReferenceSource.Outcome p a) : Prop :=
  (result.status = .ready → result.state.waiting = none) ∧
  (result.status = .waiting → result.state.waiting.isSome = true) ∧
  ∀ saved, result.state.waiting = some saved → saved.site = site

private theorem idle_outcome (s : ReferenceSource.State p a) (site : Site) (reason : ReferenceSource.Failure)
    (idle : s.waiting = none) : OutcomeAt site ⟨s,.failed reason⟩ := by simp [OutcomeAt,idle]

private theorem begin_at (s : ReferenceSource.State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (site : Site) (beforeCount : Nat) (deps : List ReferenceSource.Read) (name : String)
    (target : ReferenceSource.Target) (argument : Int) (idle : s.waiting = none) :
    OutcomeAt site (ReferenceSource.beginCall s member place principal site beforeCount deps name target argument) := by
  cases target with
  | «instance» key =>
      simp only [ReferenceSource.beginCall]
      cases ReferenceExecution.startPlain s.machine member place principal s.nextRequest key argument with
      | none => exact idle_outcome _ _ _ idle
      | some pair => obtain ⟨machine,entry⟩ := pair; simp [OutcomeAt,ReferenceSource.mark]
  | reference key =>
      simp only [ReferenceSource.beginCall]
      cases ReferenceExecution.startReference s.machine member place principal s.nextRequest key argument with
      | none => exact idle_outcome _ _ _ idle
      | some pair => obtain ⟨machine,entry⟩ := pair; simp [OutcomeAt,ReferenceSource.mark]

private theorem plan_at (s : ReferenceSource.State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (plan : ReferenceSource.Plan) (idle : s.waiting = none) :
    OutcomeAt item.site (ReferenceSource.executePlan s member place principal item plan) := by
  cases plan with
  | pureValue name value assign => simp [ReferenceSource.executePlan,OutcomeAt,ReferenceSource.write,idle]
  | control name command kind =>
      simp only [ReferenceSource.executePlan]
      cases ReferenceExecution.manage s.machine member place principal s.nextRequest (ReferenceSource.withOwner principal command) with
      | none => exact idle_outcome _ _ _ idle
      | some pair =>
          obtain ⟨machine,created⟩ := pair
          dsimp only
          cases ReferenceSource.resultValue kind created <;> simp [OutcomeAt,ReferenceSource.write,ReferenceSource.mark,idle]
  | acquire name chain =>
      simp only [ReferenceSource.executePlan]
      cases ReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
      | none => exact idle_outcome _ _ _ idle
      | some pair => obtain ⟨machine,key⟩ := pair; simp [OutcomeAt,ReferenceSource.write,ReferenceSource.mark,idle]
  | reacquire name key =>
      simp only [ReferenceSource.executePlan]
      cases ReferenceExecution.reacquire s.machine member place principal s.nextRequest key with
      | none => exact idle_outcome _ _ _ idle
      | some machine => simp [OutcomeAt,ReferenceSource.write,ReferenceSource.mark,idle]
  | release name key =>
      simp only [ReferenceSource.executePlan]
      cases ReferenceExecution.release s.machine member place principal s.nextRequest key with
      | none => exact idle_outcome _ _ _ idle
      | some machine => simp [OutcomeAt,ReferenceSource.write,ReferenceSource.mark,idle]
  | call name target argument =>
      cases target with
      | «instance» key => exact begin_at _ _ _ _ _ _ _ _ _ _ idle
      | reference key =>
          let normalized := ReferenceExecution.normalize s.machine member place principal s.nextRequest key
          let next := if normalized.consumed then ReferenceSource.mark s normalized.state (some item.site) .normalize 1 else s
          have nextIdle : next.waiting = none := by unfold next; split <;> exact idle
          change OutcomeAt item.site
            (match normalized.result with
            | .error reason => (⟨next,.failed (.normalization reason)⟩ : ReferenceSource.Outcome p a)
            | .ok _ => ReferenceSource.beginCall next member place principal item.site s.machine.store.events.length (ReferenceSource.inputs s item) name (.reference key) argument)
          cases normalized.result with
          | error reason => exact idle_outcome _ _ _ nextIdle
          | ok _ => exact begin_at _ _ _ _ _ _ _ _ _ _ nextIdle

private theorem advance_at (s : ReferenceSource.State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (idle : s.waiting = none) :
    OutcomeAt item.site (ReferenceSource.advance s member place principal item) := by
  simp only [ReferenceSource.advance,idle,Option.isSome_none,Bool.false_eq_true,ite_false]
  cases checkStatement p (environment s.values) item.statement with
  | none => exact idle_outcome _ _ _ idle
  | some env =>
      cases ReferenceSource.elaborate s.machine.store.core.system.configuration.state s.values item.statement with
      | none => exact idle_outcome _ _ _ idle
      | some plan => exact plan_at _ _ _ _ _ _ idle

-- The retained statement matches the actual pending continuation, including
-- complete source-content identity and byte position. Source pending coverage
-- separately proves that there is no hidden second classified source request.
def Continuing (s : Session p a) : Prop :=
  (s.status = .ready → s.state.waiting = none) ∧
  (s.status = .waiting → s.state.waiting.isSome = true) ∧
  ∀ saved, s.state.waiting = some saved → ∃ item, s.stopped = some item ∧ saved.site = item.site

theorem tick_continuing (s : Session p a) (member : Fin a) (principal : Nat) (valid : Continuing s) :
    Continuing (tick s member principal) := by
  cases phase : s.status with
  | failed reason => simpa [tick,phase] using valid
  | waiting =>
      simp only [tick,phase]
      cases waiting : s.state.waiting with
      | none =>
          simp [ReferenceSource.complete,waiting,Continuing]
      | some saved =>
          cases resumed : ReferenceExecution.resume s.state.machine saved.entry with
          | none =>
              simp only [ReferenceSource.complete,waiting,resumed]
              refine ⟨?_,?_,?_⟩
              · intro impossible; cases impossible
              · intro impossible; cases impossible
              · exact valid.2.2
          | some pair =>
              obtain ⟨machine,value⟩ := pair
              simp [ReferenceSource.complete,waiting,resumed,Continuing,ReferenceSource.write,ReferenceSource.mark]
  | ready =>
      have idle := valid.1 phase
      cases remaining : s.remaining with
      | nil => simpa [tick,phase,remaining] using valid
      | cons item rest =>
          have result := advance_at s.state member s.program.place principal item idle
          simp only [tick,phase,remaining]
          refine ⟨result.1,result.2.1,?_⟩
          intro saved waiting
          have site := result.2.2 saved waiting
          have blocked : (ReferenceSource.advance s.state member s.program.place principal item).status ≠ .ready := by
            intro ready
            have noWaiting := result.1 ready
            rw [waiting] at noWaiting
            cases noWaiting
          exact ⟨item,by simp [blocked],site⟩

#print axioms advance_at
#print axioms tick_continuing
end MirroreaProofFirst.ReferenceSession

namespace MirroreaProofFirst.ReferenceSession
open ReferenceContinuation ReferenceSourceData

theorem cancel_continuing (s : Session p a) (member : Fin a) (principal : Nat) (valid : Continuing s) :
    Continuing (cancel s member principal) := by
  simp only [cancel]
  split
  · rename_i ready
    obtain ⟨_,_,_,_,equal⟩ := ReferenceSource.cancel_success _ _ _ _ ready
    refine ⟨?_,?_,?_⟩
    · intro impossible; cases impossible
    · intro impossible; cases impossible
    · intro saved present
      simp only [equal] at present
      cases present
  · exact valid

theorem adopted_continuing (s next : Session p a) (program : Program p)
    (idle : s.state.waiting = none) (accepted : Adopted s next program) : Continuing next := by
  obtain ⟨state,_,_,_,stopped,status,_⟩ := accepted
  simp [Continuing,state,status,idle]

theorem step_continuing (s next : Session p a) (valid : Continuing s) (step : Step s next) : Continuing next := by
  cases step with
  | tick => exact tick_continuing _ _ _ valid
  | cancellation => exact cancel_continuing _ _ _ valid
  | head run =>
      rw [(head_parts _ _ _ run).2]
      exact valid
  | control run =>
      obtain ⟨state,committed,rfl⟩ := control_parts _ _ _ _ _ _ _ run
      have pending : state.waiting = s.state.waiting := (ReferenceSource.controlInput_projects _ _ _ _ _ _ committed).2.1
      simpa only [Continuing,pending] using valid
  | replace run =>
      exact adopted_continuing _ _ _ ((replace_exact _ _).mp ⟨_,run⟩).2.1 (replace_state _ _ _ run)
  | continueWith run =>
      exact adopted_continuing _ _ _ ((continue_exact _ _).mp ⟨_,run⟩).2.2.1 (continue_state _ _ _ run)

theorem rooted_continuing (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
    (s : Session p a) (path : Rooted realm view policy s) : Continuing s := by
  induction path with
  | launch run => rw [launch_state _ _ _ _ _ run]; simp [Continuing,ReferenceSource.initial]
  | step previous step ih => exact step_continuing _ _ ih step

-- Every old archive remains, in order, after every admitted operation.
theorem step_archive (s next : Session p a) (step : Step s next) :
    ∃ added, next.superseded = added ++ s.superseded := by
  cases step with
  | @tick member principal =>
      refine ⟨[],?_⟩
      cases phase : s.status with
      | failed _ => simp [tick,phase]
      | waiting => simp [tick,phase]
      | ready => cases remaining : s.remaining <;> simp [tick,phase,remaining]
  | cancellation => exact ⟨[],by simp [(cancel_retains _ _ _).2.2.2.2]⟩
  | head run => rw [(head_parts _ _ _ run).2]; exact ⟨[],rfl⟩
  | control run => obtain ⟨_,_,rfl⟩ := control_parts _ _ _ _ _ _ _ run; exact ⟨[],rfl⟩
  | replace run => exact ⟨[archive s],(replace_state _ _ _ run).2.2.2.2.2.2⟩
  | continueWith run => exact ⟨[archive s],(continue_state _ _ _ run).2.2.2.2.2.2⟩

theorem rooted_waiting_statement (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
    (s : Session p a) (path : Rooted realm view policy s) (saved : ReferenceSource.Awaiting)
    (waiting : s.state.waiting = some saved) :
    ∃ item, s.stopped = some item ∧ saved.site = item.site ∧
      item ∈ s.program.items ∧ s.state.machine.pending = [saved.entry] := by
  obtain ⟨item,stopped,site⟩ := (rooted_continuing _ _ _ _ path).2.2 saved waiting
  refine ⟨item,stopped,site,?_,?_⟩
  · have partition := (rooted_metadata _ _ _ _ path).1
    rw [partition,stopped]
    simp [List.mem_append]
  · exact (ReferenceSourceTrace.rooted_single_pending _ _ _ _ (rooted_source _ _ _ _ path)).2 saved waiting

#print axioms cancel_continuing
#print axioms step_continuing
#print axioms rooted_continuing
#print axioms step_archive
#print axioms rooted_waiting_statement
end MirroreaProofFirst.ReferenceSession

namespace MirroreaProofFirst.ReferenceSession
open ReferenceContinuation

theorem tick_view (s : Session p a) (member : Fin a) (principal : Nat) :
    (tick s member principal).state.machine.store.core.system.view = s.state.machine.store.core.system.view := by
  cases phase : s.status with
  | failed _ => simp [tick,phase]
  | waiting => simpa only [tick,phase] using (ReferenceAuthority.complete_view s.state)
  | ready =>
      cases remaining : s.remaining with
      | nil => simp [tick,phase,remaining]
      | cons item rest => simpa only [tick,phase,remaining] using (ReferenceAuthority.advance_view s.state member s.program.place principal item)

theorem drive_view (fuel : Nat) (s : Session p a) (member : Fin a) (principal : Nat) :
    (drive fuel s member principal).state.machine.store.core.system.view = s.state.machine.store.core.system.view := by
  induction fuel generalizing s with
  | zero => rfl
  | succ fuel ih => exact (ih _).trans (tick_view _ _ _)

theorem head_checked (s : Session p a) (view : WorldProjection.AuthorityView a)
    (checked : ReferenceAuthority.check s.state.machine.store.core.system.view view = true) :
    authorityHead s view = some {s with state := ReferenceSource.authorityHead s.state view} := by
  simp [authorityHead,ReferenceAuthority.install,checked]

theorem authority_trans (first middle last : WorldProjection.AuthorityView a)
    (left : ReferenceAuthority.Reached first middle) (right : ReferenceAuthority.Reached middle last) :
    ReferenceAuthority.Reached first last := by
  induction right with
  | refl => exact left
  | step previous step ih => exact .step ih step

theorem step_authority (s next : Session p a) (step : Step s next) :
    ReferenceAuthority.Reached s.state.machine.store.core.system.view next.state.machine.store.core.system.view := by
  cases step with
  | tick => rw [tick_view]; exact .refl
  | cancellation =>
      simp only [cancel]
      split
      · rw [ReferenceAuthority.cancellation_view]; exact .refl
      · exact .refl
  | head run =>
      obtain ⟨admitted,rfl⟩ := head_parts _ _ _ run
      exact .step .refl admitted
  | control run =>
      obtain ⟨state,committed,rfl⟩ := control_parts _ _ _ _ _ _ _ run
      rw [ReferenceAuthority.control_view _ _ _ _ _ _ _ committed]
      exact .refl
  | replace run => rw [(replace_state _ _ _ run).1]; exact .refl
  | continueWith run => rw [(continue_state _ _ _ run).1]; exact .refl

inductive Reached : Session p a → Session p a → Prop where
  | refl : Reached s s
  | step : Reached first s → Step s next → Reached first next

theorem reached_authority (first next : Session p a) (path : Reached first next) :
    ReferenceAuthority.Reached first.state.machine.store.core.system.view next.state.machine.store.core.system.view := by
  induction path with
  | refl => exact .refl
  | step previous step ih => exact authority_trans _ _ _ ih (step_authority _ _ step)

theorem reached_archive (first next : Session p a) (path : Reached first next) :
    ∃ added, next.superseded = added ++ first.superseded := by
  induction path with
  | refl => exact ⟨[],rfl⟩
  | step previous step ih =>
      obtain ⟨past,pastAt⟩ := ih
      obtain ⟨added,addedAt⟩ := step_archive _ _ step
      exact ⟨added ++ past,by rw [addedAt,pastAt,List.append_assoc]⟩

-- After an admitted new head, no finite same-session extension can accept
-- the old ticket's result. This composes the executable authority boundary
-- with ALL other session entries, rather than postulating a head-only trace.
theorem old_finish_rejected (s installed next : Session p a) (view : WorldProjection.AuthorityView a)
    (entry : ReferenceExecution.Pending) (value : Int)
    (original : InvocationBoundary.check s.state.machine.store.core.system.configuration.state
      s.state.machine.store.core.system.view entry.ticket = true)
    (changed : authorityHead s view = some installed) (path : Reached installed next) :
    ReferenceExecution.finish next.state.machine entry value = none := by
  obtain ⟨admitted,installedAt⟩ := head_parts _ _ _ changed
  have future := reached_authority _ _ path
  rw [installedAt] at future
  have denied := ReferenceAuthority.old_result_rejected _ next.state.machine.store.core.system.configuration.state
    _ view _ entry.ticket value original admitted future
  cases finished : ReferenceExecution.finish next.state.machine entry value with
  | none => rfl
  | some result =>
      have core := ReferenceExecution.finish_core_run _ _ _ _ finished
      have checked := (CompositionMachine.finish_parts _ _ _ _ core).2.2.1
      rw [denied] at checked
      cases checked

#print axioms tick_view
#print axioms drive_view
#print axioms head_checked
#print axioms step_authority
#print axioms reached_authority
#print axioms reached_archive
#print axioms old_finish_rejected
end MirroreaProofFirst.ReferenceSession
