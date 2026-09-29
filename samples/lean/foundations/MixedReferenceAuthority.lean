import MirroreaProofFirstReferenceAuthority
import MixedReferenceSourceTrace
namespace MirroreaProofFirst.MixedReferenceAuthority
open WorldProjection CurrentUse
-- The existing monotone head profile is retained exactly; its issuer/publication
-- authenticity remains an external premise. This checker never issues authority.
open ReferenceAuthority (Successor check check_exact Reached no_generation_return)
def install (s : MixedReferenceSource.State p a) (next : AuthorityView a) : Option (MixedReferenceSource.State p a) :=
  if check s.machine.store.core.system.view next then some (MixedReferenceSource.authorityHead s next) else none

theorem install_exact (s : MixedReferenceSource.State p a) (next : AuthorityView a) :
    (∃ result, install s next = some result) ↔ Successor s.machine.store.core.system.view next := by
  simp [install,← check_exact]

theorem install_parts (s : MixedReferenceSource.State p a) (view : AuthorityView a) (next : MixedReferenceSource.State p a)
    (accepted : install s view = some next) :
    Successor s.machine.store.core.system.view view ∧ next = MixedReferenceSource.authorityHead s view := by
  unfold install at accepted
  split at accepted
  · exact ⟨(check_exact _ _).mp ‹_›,(Option.some.inj accepted).symm⟩
  · cases accepted

theorem invocation_generation (s : MixedInstanceState.State d p n) (view : AuthorityView a)
    (ticket : InvocationBoundary.Ticket) (accepted : MixedPureInvocation.check s view ticket = true) :
    ticket.evidence.context.generation = view.generation := by
  obtain ⟨member,key,place,checked⟩ := MixedPureInvocation.check_parts accepted
  simp only [MixedPureInvocation.checkAt,Bool.and_eq_true] at checked
  have used := checked.2
  simp only [checkUse,Bool.and_eq_true] at used
  have contextAt := (revalidate_sound _ _ _ _ used.2).2.2.1
  exact congrArg Context.generation contextAt

theorem old_result_rejected (before : MixedInstanceState.State d p n) (after : MixedInstanceState.State d' p' n')
    (first middle last : AuthorityView a) (ticket : InvocationBoundary.Ticket) (value : Int)
    (original : MixedPureInvocation.check before first ticket = true)
    (changed : Successor first middle) (path : Reached middle last) :
    MixedPureInvocation.resultCheck after last ticket value = false := by
  have oldGeneration := invocation_generation _ _ _ original
  have different := no_generation_return _ _ _ changed path
  have denied : MixedPureInvocation.check after last ticket = false := by
    cases checked : MixedPureInvocation.check after last ticket with
    | false => rfl
    | true =>
        have newGeneration := invocation_generation _ _ _ checked
        exact False.elim (different (newGeneration.symm.trans oldGeneration))
  simp [MixedPureInvocation.resultCheck,denied]


#print axioms install_exact
#print axioms install_parts
#print axioms old_result_rejected
end MirroreaProofFirst.MixedReferenceAuthority

namespace MirroreaProofFirst.MixedReferenceAuthority
open MixedReferenceSource

private theorem manage_view (m : MixedReferenceExecution.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (raw : MixedCompositionCore.Raw) (result : MixedReferenceExecution.Machine p a × Option Nat)
    (accepted : MixedReferenceExecution.manage m member place principal id raw = some result) :
    result.1.store.core.system.view = m.store.core.system.view := by
  have run := MixedReferenceExecution.manage_core _ _ _ _ _ _ _ accepted
  obtain ⟨_,_,_,_,_,_,equal⟩ := MixedRequestCore.manage_parts run
  exact congrArg (fun pair : MixedRequestCore.Machine p a × Option Nat => pair.1.system.view) equal

private theorem start_view (m next : MixedRequestCore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (argument : Int) (ticket : InvocationBoundary.Ticket)
    (accepted : MixedRequestCore.startPure m member place principal id key argument = some (next,ticket)) :
    next.system.view = m.system.view := by
  have equal := (MixedRequestCore.startPure_parts accepted).2.2
  change next = MixedRequestCore.enqueue m (.pure ticket) at equal
  rw [equal]
  rfl

private theorem finish_view (m next : MixedReferenceExecution.Machine p a) (entry : ReferenceExecution.Pending) (value : Int)
    (accepted : MixedReferenceExecution.finish m entry value = some next) :
    next.store.core.system.view = m.store.core.system.view := by
  rw [(MixedRequestCore.finishPure_parts (MixedReferenceExecution.finish_core_run _ _ _ _ accepted)).2.2.2]
  rfl

private theorem cancel_view (m next : MixedReferenceExecution.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (entry : ReferenceExecution.Pending)
    (accepted : MixedReferenceExecution.cancel m member place principal id entry = some next) :
    next.store.core.system.view = m.store.core.system.view := by
  obtain ⟨permit,run⟩ := MixedReferenceExecution.cancel_core _ _ _ _ _ _ _ accepted
  obtain ⟨_,_,_,_,equal⟩ := MixedCancelEntry.cancel_parts _ _ _ _ _ _ _ _ run
  rw [equal]
  rfl

private theorem begin_view (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (site : ReferenceSourceData.Site) (beforeCount : Nat) (deps : List Read) (name : String)
    (target : Target) (argument : Int) :
    (beginCall s member place principal site beforeCount deps name target argument).state.machine.store.core.system.view =
      s.machine.store.core.system.view := by
  cases target with
  | «instance» key =>
      simp only [beginCall]
      cases run : MixedReferenceExecution.startPlain s.machine member place principal s.nextRequest key argument with
      | none => rfl
      | some pair =>
          obtain ⟨machine,entry⟩ := pair
          have started := MixedReferenceExecution.startPlain_core _ _ _ _ _ _ _ _ run
          exact start_view _ _ _ _ _ _ _ _ _ started
  | reference key =>
      simp only [beginCall]
      cases run : MixedReferenceExecution.startReference s.machine member place principal s.nextRequest key argument with
      | none => rfl
      | some pair =>
          obtain ⟨machine,entry⟩ := pair
          obtain ⟨key,started⟩ := MixedReferenceExecution.startReference_core _ _ _ _ _ _ _ _ run
          exact start_view _ _ _ _ _ _ _ _ _ started

private theorem plan_view (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : ReferenceSourceData.Located) (plan : Plan) :
    (executePlan s member place principal item plan).state.machine.store.core.system.view = s.machine.store.core.system.view := by
  cases plan with
  | pureValue _ _ _ => rfl
  | control name command kind =>
      simp only [executePlan]
      cases run : MixedReferenceExecution.manage s.machine member place principal s.nextRequest (withOwner principal command) with
      | none => rfl
      | some pair =>
          obtain ⟨machine,created⟩ := pair
          have view := manage_view _ _ _ _ _ _ _ run
          dsimp only
          cases resultValue kind created <;> exact view
  | acquire name chain =>
      simp only [executePlan]
      cases run : MixedReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
      | none => rfl
      | some pair =>
          obtain ⟨machine,key⟩ := pair
          have view := congrArg (fun core : MixedRequestCore.Machine p a => core.system.view) (MixedReferenceExecution.acquire_core _ _ _ _ _ _ _ run)
          exact view
  | reacquire name key =>
      simp only [executePlan]
      cases run : MixedReferenceExecution.reacquire s.machine member place principal s.nextRequest key with
      | none => rfl
      | some machine =>
          have view := congrArg (fun core : MixedRequestCore.Machine p a => core.system.view) (MixedReferenceExecution.reacquire_core _ _ _ _ _ _ _ run)
          exact view
  | release name key =>
      simp only [executePlan]
      cases run : MixedReferenceExecution.release s.machine member place principal s.nextRequest key with
      | none => rfl
      | some machine =>
          have view := congrArg (fun core : MixedRequestCore.Machine p a => core.system.view) (MixedReferenceExecution.release_core _ _ _ _ _ _ _ run)
          exact view
  | call name target argument =>
      cases target with
      | «instance» key => exact begin_view _ _ _ _ _ _ _ _ _ _
      | reference key =>
          let normalized := MixedReferenceExecution.normalize s.machine member place principal s.nextRequest key
          let next := if normalized.consumed then mark s normalized.state (some item.site) .normalize 1 else s
          have view : next.machine.store.core.system.view = s.machine.store.core.system.view := by
            unfold next
            split
            · rename_i consumed
              have view := congrArg (fun core : MixedRequestCore.Machine p a => core.system.view) (MixedReferenceExecution.normalize_consumed_core _ _ _ _ _ _ consumed)
              exact view
            · rfl
          change (match normalized.result with
            | .error reason => (⟨next,.failed (.normalization reason)⟩ : Outcome p a)
            | .ok _ => beginCall next member place principal item.site s.machine.store.events.length (inputs s item) name (.reference key) argument).state.machine.store.core.system.view = s.machine.store.core.system.view
          cases normalized.result with
          | error _ => exact view
          | ok _ => exact (begin_view _ _ _ _ _ _ _ _ _ _).trans view

theorem advance_view (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : ReferenceSourceData.Located) :
    (advance s member place principal item).state.machine.store.core.system.view = s.machine.store.core.system.view := by
  unfold advance
  split
  · rfl
  · cases ReferenceSourceData.checkStatement p (ReferenceSourceData.environment s.values) item.statement with
    | none => rfl
    | some env =>
        cases elaborate s.machine.store.core.system.configuration.state s.values item.statement with
        | none => rfl
        | some plan => exact plan_view _ _ _ _ _ _

theorem complete_view (s : State p a) : (complete s).state.machine.store.core.system.view = s.machine.store.core.system.view := by
  cases waiting : s.waiting with
  | none => simp [complete,waiting]
  | some saved =>
      simp only [complete,waiting]
      cases resumed : MixedReferenceExecution.resume s.machine saved.entry with
      | none => rfl
      | some pair =>
          obtain ⟨machine,value⟩ := pair
          unfold MixedReferenceExecution.resume at resumed
          cases evaluated : InvocationBoundary.execute saved.entry.ticket with
          | none => simp [evaluated] at resumed
          | some value =>
              simp only [evaluated,Option.bind_eq_bind,Option.bind_some] at resumed
              cases finished : MixedReferenceExecution.finish s.machine saved.entry value with
              | none => simp [finished] at resumed
              | some next =>
                  simp only [finished,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at resumed
                  obtain ⟨rfl,rfl⟩ := resumed
                  exact finish_view _ _ _ _ finished

theorem cancellation_view (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) :
    (cancel s member place principal).state.machine.store.core.system.view = s.machine.store.core.system.view := by
  cases waiting : s.waiting with
  | none => simp [cancel,waiting]
  | some saved =>
      simp only [cancel,waiting]
      cases run : MixedReferenceExecution.cancel s.machine member place principal s.nextRequest saved.entry with
      | none => rfl
      | some next => exact cancel_view _ _ _ _ _ _ _ run

theorem control_view (s next : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : MixedCompositionCore.Raw) (created : Option Nat)
    (accepted : controlInput s member place principal raw = some (next,created)) :
    next.machine.store.core.system.view = s.machine.store.core.system.view := by
  unfold controlInput at accepted
  cases run : MixedReferenceExecution.manage s.machine member place principal s.nextRequest raw with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨machine,key⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at accepted
      obtain ⟨rfl,rfl⟩ := accepted
      exact manage_view _ _ _ _ _ _ _ run

#print axioms advance_view
#print axioms complete_view
#print axioms cancellation_view
#print axioms control_view
end MirroreaProofFirst.MixedReferenceAuthority
