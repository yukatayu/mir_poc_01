import MirroreaProofFirstReferenceSourceTrace

namespace MirroreaProofFirst.ReferenceAuthority
open WorldProjection CurrentUse

-- Finite, conservative LAB successor profile, motivated by Canon theory/05.
-- Authentic issuer publication/currentness remains an explicit environment
-- obligation. Passing this checker never issues a claim or authenticates a head.
-- Each installation is a strictly newer immutable generation. Inventories and
-- revocation tombstones are retained; active issuer epochs cannot be erased.
-- No claim is made that an old hold survives an unrelated generation change.
def MemberSuccessor (old next : Member) : Prop :=
  old.incarnation ≤ next.incarnation ∧
  (next.incarnation = old.incarnation → next.principal = old.principal ∧ old.revision ≤ next.revision) ∧
  (old.enabled = false → next.enabled = true → old.incarnation < next.incarnation)

def memberCheck (old next : Member) : Bool :=
  decide (old.incarnation ≤ next.incarnation) &&
  (if next.incarnation = old.incarnation then decide (next.principal = old.principal ∧ old.revision ≤ next.revision) else true) &&
  (if old.enabled = false ∧ next.enabled = true then decide (old.incarnation < next.incarnation) else true)

theorem member_exact (old next : Member) : memberCheck old next = true ↔ MemberSuccessor old next := by
  simp only [memberCheck,MemberSuccessor,Bool.and_eq_true,decide_eq_true_eq]
  split <;> split <;> simp_all

def EpochSuccessor (old next : Authority) (issuer : Nat) : Prop :=
  ∀ previous, old.epochs issuer = some previous → ∃ current, next.epochs issuer = some current ∧ previous ≤ current

def epochCheck (old next : Authority) (issuer : Nat) : Bool :=
  match old.epochs issuer with
  | none => true
  | some previous => match next.epochs issuer with
    | none => false
    | some current => decide (previous ≤ current)

theorem epoch_exact (old next : Authority) (issuer : Nat) :
    epochCheck old next issuer = true ↔ EpochSuccessor old next issuer := by
  cases before : old.epochs issuer <;> cases after : next.epochs issuer <;> simp [epochCheck,EpochSuccessor,before,after]

def Successor (old next : AuthorityView a) : Prop :=
  next.realm = old.realm ∧ old.generation < next.generation ∧
  (∀ id ∈ old.authority.revoked, id ∈ next.authority.revoked) ∧
  (∀ claim ∈ old.authority.issued, claim ∈ next.authority.issued) ∧
  (next.authority.issued.map Claim.id).Nodup ∧
  (∀ claim ∈ old.authority.issued, EpochSuccessor old.authority next.authority claim.issuer) ∧
  (∀ member : Fin a, MemberSuccessor (old.members member) (next.members member))

def check (old next : AuthorityView a) : Bool :=
  decide (next.realm = old.realm) && decide (old.generation < next.generation) &&
  old.authority.revoked.all (next.authority.revoked.contains ·) &&
  old.authority.issued.all (next.authority.issued.contains ·) &&
  decide (next.authority.issued.map Claim.id).Nodup &&
  old.authority.issued.all (fun claim => epochCheck old.authority next.authority claim.issuer) &&
  (List.finRange a).all (fun member => memberCheck (old.members member) (next.members member))

theorem check_exact (old next : AuthorityView a) : check old next = true ↔ Successor old next := by
  simp [check,Successor,List.all_eq_true,epoch_exact,member_exact,and_assoc]

-- This is the checked source-facing authority input. The older unrestricted
-- Source.authorityHead remains useful for adversarial-history controls, not
-- an admitted authority installation for this finite successor profile.
def install (s : ReferenceSource.State p a) (next : AuthorityView a) : Option (ReferenceSource.State p a) :=
  if check s.machine.store.core.system.view next then some (ReferenceSource.authorityHead s next) else none

theorem install_exact (s : ReferenceSource.State p a) (next : AuthorityView a) :
    (∃ result, install s next = some result) ↔ Successor s.machine.store.core.system.view next := by
  simp [install,← check_exact]

theorem install_parts (s : ReferenceSource.State p a) (view : AuthorityView a) (next : ReferenceSource.State p a)
    (accepted : install s view = some next) :
    Successor s.machine.store.core.system.view view ∧ next = ReferenceSource.authorityHead s view := by
  unfold install at accepted
  split at accepted
  · exact ⟨(check_exact _ _).mp ‹_›,(Option.some.inj accepted).symm⟩
  · cases accepted

inductive Reached : AuthorityView a → AuthorityView a → Prop where
  | refl : Reached view view
  | step : Reached first view → Successor view next → Reached first next

theorem reached_preserves (first next : AuthorityView a) (path : Reached first next) :
    first.generation ≤ next.generation ∧
      (∀ id ∈ first.authority.revoked, id ∈ next.authority.revoked) ∧
      (∀ claim ∈ first.authority.issued, claim ∈ next.authority.issued) := by
  induction path with
  | refl => exact ⟨Nat.le_refl _,fun _ present => present,fun _ present => present⟩
  | step previous step ih =>
      exact ⟨Nat.le_trans ih.1 (Nat.le_of_lt step.2.1),
        fun id present => step.2.2.1 id (ih.2.1 id present),
        fun claim present => step.2.2.2.1 claim (ih.2.2 claim present)⟩

theorem revoked_stays_invalid (first next : AuthorityView a) (path : Reached first next)
    (claim : Claim) (revoked : claim.id ∈ first.authority.revoked)
    (context : Context) (label : Nat) (need : Need) : ¬ ValidClaim next.authority context label need claim := by
  intro valid
  exact valid.2.1 ((reached_preserves _ _ path).2.1 _ revoked)

theorem no_generation_return (first middle last : AuthorityView a)
    (changed : Successor first middle) (path : Reached middle last) : last.generation ≠ first.generation := by
  have bound := (reached_preserves _ _ path).1
  have strict := changed.2.1
  omega

theorem invocation_generation (s : InstanceState.State d p n) (view : AuthorityView a)
    (ticket : InvocationBoundary.Ticket) (accepted : InvocationBoundary.check s view ticket = true) :
    ticket.evidence.context.generation = view.generation := by
  obtain ⟨member,key,place,checked⟩ := InvocationBoundary.check_parts _ _ _ accepted
  simp only [InvocationBoundary.checkAt,Bool.and_eq_true] at checked
  have used := checked.2
  simp only [checkUse,Bool.and_eq_true] at used
  have contextAt := (revalidate_sound _ _ _ _ used.2).2.2.1
  exact congrArg Context.generation contextAt

theorem old_result_rejected (before : InstanceState.State d p n) (after : InstanceState.State d' p' n')
    (first middle last : AuthorityView a) (ticket : InvocationBoundary.Ticket) (value : Int)
    (original : InvocationBoundary.check before first ticket = true)
    (changed : Successor first middle) (path : Reached middle last) :
    InvocationBoundary.resultCheck after last ticket value = false := by
  have oldGeneration := invocation_generation _ _ _ original
  have different := no_generation_return _ _ _ changed path
  have denied : InvocationBoundary.check after last ticket = false := by
    cases checked : InvocationBoundary.check after last ticket with
    | false => rfl
    | true =>
        have newGeneration := invocation_generation _ _ _ checked
        exact False.elim (different (newGeneration.symm.trans oldGeneration))
  simp [InvocationBoundary.resultCheck,denied]

private theorem claim_id_unique (claims : List Claim) (unique : (claims.map Claim.id).Nodup)
    (left right : Claim) (lm : left ∈ claims) (rm : right ∈ claims) (same : left.id = right.id) : left = right := by
  induction claims with
  | nil => cases lm
  | cons head tail ih =>
      have nodup := List.nodup_cons.mp unique
      rcases List.mem_cons.mp lm with rfl | leftTail
      · rcases List.mem_cons.mp rm with rfl | rightTail
        · rfl
        · exact False.elim (nodup.1 (List.mem_map.mpr ⟨right,rightTail,same.symm⟩))
      · rcases List.mem_cons.mp rm with rfl | rightTail
        · exact False.elim (nodup.1 (List.mem_map.mpr ⟨left,leftTail,same⟩))
        · exact ih nodup.2 leftTail rightTail

theorem no_claim_reinterpretation (old next : AuthorityView a) (step : Successor old next)
    (before after : Claim) (issued : before ∈ old.authority.issued)
    (current : after ∈ next.authority.issued) (same : before.id = after.id) : before = after :=
  claim_id_unique _ step.2.2.2.2.1 _ _ (step.2.2.2.1 _ issued) current same

#print axioms invocation_generation
#print axioms old_result_rejected
#print axioms no_claim_reinterpretation

#print axioms member_exact
#print axioms epoch_exact
#print axioms check_exact
#print axioms install_exact
#print axioms install_parts
#print axioms reached_preserves
#print axioms revoked_stays_invalid
#print axioms no_generation_return
end MirroreaProofFirst.ReferenceAuthority

namespace MirroreaProofFirst.ReferenceAuthority
open ReferenceSource

private theorem manage_view (m : ReferenceExecution.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (raw : CompositionCore.Raw) (result : ReferenceExecution.Machine p a × Option Nat)
    (accepted : ReferenceExecution.manage m member place principal id raw = some result) :
    result.1.store.core.system.view = m.store.core.system.view := by
  have run := ReferenceExecution.manage_core _ _ _ _ _ _ _ accepted
  obtain ⟨_,_,_,_,_,_,equal⟩ := CompositionMachine.manage_parts _ _ _ _ _ _ _ run
  exact congrArg (fun pair : CompositionMachine.Machine p a × Option Nat => pair.1.system.view) equal

private theorem start_view (m next : CompositionMachine.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (argument : Int) (ticket : InvocationBoundary.Ticket)
    (accepted : CompositionMachine.start m member place principal id key argument = some (next,ticket)) :
    next.system.view = m.system.view := by
  have equal := (CompositionMachine.start_parts _ _ _ _ _ _ _ _ accepted).2.2
  change next = CompositionMachine.enqueue m ticket at equal
  rw [equal]
  rfl

private theorem finish_view (m next : ReferenceExecution.Machine p a) (entry : ReferenceExecution.Pending) (value : Int)
    (accepted : ReferenceExecution.finish m entry value = some next) :
    next.store.core.system.view = m.store.core.system.view := by
  rw [(CompositionMachine.finish_parts _ _ _ _ (ReferenceExecution.finish_core_run _ _ _ _ accepted)).2.2.2]
  rfl

private theorem cancel_view (m next : ReferenceExecution.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (entry : ReferenceExecution.Pending)
    (accepted : ReferenceExecution.cancel m member place principal id entry = some next) :
    next.store.core.system.view = m.store.core.system.view := by
  obtain ⟨permit,run⟩ := ReferenceExecution.cancel_core _ _ _ _ _ _ _ accepted
  obtain ⟨_,_,_,_,equal⟩ := CompositionMachine.cancel_parts _ _ _ _ _ _ _ _ run
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
      cases run : ReferenceExecution.startPlain s.machine member place principal s.nextRequest key argument with
      | none => rfl
      | some pair =>
          obtain ⟨machine,entry⟩ := pair
          have started := ReferenceExecution.startPlain_core _ _ _ _ _ _ _ _ run
          exact start_view _ _ _ _ _ _ _ _ _ started
  | reference key =>
      simp only [beginCall]
      cases run : ReferenceExecution.startReference s.machine member place principal s.nextRequest key argument with
      | none => rfl
      | some pair =>
          obtain ⟨machine,entry⟩ := pair
          obtain ⟨key,started⟩ := ReferenceExecution.startReference_core _ _ _ _ _ _ _ _ run
          exact start_view _ _ _ _ _ _ _ _ _ started

private theorem plan_view (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : ReferenceSourceData.Located) (plan : Plan) :
    (executePlan s member place principal item plan).state.machine.store.core.system.view = s.machine.store.core.system.view := by
  cases plan with
  | pureValue _ _ _ => rfl
  | control name command kind =>
      simp only [executePlan]
      cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest (withOwner principal command) with
      | none => rfl
      | some pair =>
          obtain ⟨machine,created⟩ := pair
          have view := manage_view _ _ _ _ _ _ _ run
          dsimp only
          cases resultValue kind created <;> exact view
  | acquire name chain =>
      simp only [executePlan]
      cases run : ReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
      | none => rfl
      | some pair =>
          obtain ⟨machine,key⟩ := pair
          have view := congrArg (fun core : CompositionMachine.Machine p a => core.system.view) (ReferenceExecution.acquire_core _ _ _ _ _ _ _ run)
          exact view
  | reacquire name key =>
      simp only [executePlan]
      cases run : ReferenceExecution.reacquire s.machine member place principal s.nextRequest key with
      | none => rfl
      | some machine =>
          have view := congrArg (fun core : CompositionMachine.Machine p a => core.system.view) (ReferenceExecution.reacquire_core _ _ _ _ _ _ _ run)
          exact view
  | release name key =>
      simp only [executePlan]
      cases run : ReferenceExecution.release s.machine member place principal s.nextRequest key with
      | none => rfl
      | some machine =>
          have view := congrArg (fun core : CompositionMachine.Machine p a => core.system.view) (ReferenceExecution.release_core _ _ _ _ _ _ _ run)
          exact view
  | call name target argument =>
      cases target with
      | «instance» key => exact begin_view _ _ _ _ _ _ _ _ _ _
      | reference key =>
          let normalized := ReferenceExecution.normalize s.machine member place principal s.nextRequest key
          let next := if normalized.consumed then mark s normalized.state (some item.site) .normalize 1 else s
          have view : next.machine.store.core.system.view = s.machine.store.core.system.view := by
            unfold next
            split
            · rename_i consumed
              have view := congrArg (fun core : CompositionMachine.Machine p a => core.system.view) (ReferenceExecution.normalize_consumed_core _ _ _ _ _ _ consumed)
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
      cases resumed : ReferenceExecution.resume s.machine saved.entry with
      | none => rfl
      | some pair =>
          obtain ⟨machine,value⟩ := pair
          unfold ReferenceExecution.resume at resumed
          cases evaluated : InvocationBoundary.execute saved.entry.ticket with
          | none => simp [evaluated] at resumed
          | some value =>
              simp only [evaluated,Option.bind_eq_bind,Option.bind_some] at resumed
              cases finished : ReferenceExecution.finish s.machine saved.entry value with
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
      cases run : ReferenceExecution.cancel s.machine member place principal s.nextRequest saved.entry with
      | none => rfl
      | some next => exact cancel_view _ _ _ _ _ _ _ run

theorem control_view (s next : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) (created : Option Nat)
    (accepted : controlInput s member place principal raw = some (next,created)) :
    next.machine.store.core.system.view = s.machine.store.core.system.view := by
  unfold controlInput at accepted
  cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest raw with
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
end MirroreaProofFirst.ReferenceAuthority
