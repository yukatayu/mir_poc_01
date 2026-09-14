import MirroreaProofFirstReceivedResult
import MirroreaProofFirstReferenceSourceProvenance

namespace MirroreaProofFirst.SourcePlacement
open ReferenceSourceData

-- Reversible finite source/Core candidate, not adopted syntax. An optional
-- execution-locus annotation applies ONLY to a callable invocation. It neither
-- relocates the caller's environment/continuation nor issues authority. A block
-- of remote private-state statements needs a separate owner-body semantics.
structure Raw where
  source : Located
  operation : Option Nat
  deriving DecidableEq, Repr

structure Core (p : Nat) where
  source : Located
  caller : Fin p
  operation : Fin p
  deriving DecidableEq, Repr

def IsCall : Statement → Prop
  | .plain (.invoke ..) => True
  | _ => False

def isCall : Statement → Bool
  | .plain (.invoke ..) => true
  | _ => false

theorem isCall_exact (s : Statement) : isCall s = true ↔ IsCall s := by
  cases s with
  | plain s => cases s <;> simp [isCall,IsCall]
  | _ => simp [isCall,IsCall]

def Placement (caller : Fin p) (raw : Raw) (operation : Fin p) : Prop :=
  match raw.operation with
  | none => operation = caller
  | some n => IsCall raw.source.statement ∧ operation.val = n

def resolve (caller : Fin p) (raw : Raw) : Option (Fin p) :=
  match raw.operation with
  | none => some caller
  | some n => if isCall raw.source.statement then CompositionCore.index p n else none

theorem resolve_exact (caller operation : Fin p) (raw : Raw) :
    resolve caller raw = some operation ↔ Placement caller raw operation := by
  cases destination : raw.operation with
  | none => simp [resolve,Placement,destination,eq_comm]
  | some n =>
      by_cases call : isCall raw.source.statement = true
      · have meaning := (isCall_exact _).mp call
        simp only [resolve,Placement,destination,call,ite_true,meaning,true_and]
        constructor
        · intro indexed
          exact CompositionCore.index_sound _ indexed
        · intro equal
          rw [← equal]
          exact CompositionCore.index_roundtrip operation
      · simp [resolve,Placement,destination,call,
          show ¬ IsCall raw.source.statement from fun h => call ((isCall_exact _).mpr h)]

-- Independent declarative relation; it mentions neither compile nor resolve.
def Elaborates (env : Environment) (caller : Fin p) (raw : Raw)
    (core : Core p) (next : Environment) : Prop :=
  core.source = raw.source ∧ core.caller = caller ∧ Placement caller raw core.operation ∧
    StatementTyped p env raw.source.statement next

def compile (env : Environment) (caller : Fin p) (raw : Raw) : Option (Core p × Environment) := do
  let operation ← resolve caller raw
  let next ← checkStatement p env raw.source.statement
  return (⟨raw.source,caller,operation⟩,next)

theorem compile_exact (env : Environment) (caller : Fin p) (raw : Raw)
    (core : Core p) (next : Environment) :
    compile env caller raw = some (core,next) ↔ Elaborates env caller raw core next := by
  constructor
  · intro checked
    unfold compile at checked
    cases selected : resolve caller raw with
    | none => simp [selected] at checked
    | some operation =>
        simp only [selected,Option.bind_eq_bind,Option.bind_some] at checked
        cases typed : checkStatement p env raw.source.statement with
        | none => simp [typed] at checked
        | some output =>
            simp only [typed,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at checked
            obtain ⟨rfl,rfl⟩ := checked
            exact ⟨rfl,rfl,(resolve_exact _ _ _).mp selected,(statement_exact _ _ _ _).mp typed⟩
  · rintro ⟨sourceAt,callerAt,placed,typed⟩
    have selected := (resolve_exact _ _ _).mpr placed
    have checked := (statement_exact _ _ _ _).mpr typed
    cases core
    simp_all [compile]

def execute (state : ReferenceSource.State p a) (member : Fin a) (principal : Nat)
    (core : Core p) : ReferenceSource.Outcome p a :=
  ReferenceSource.advance state member core.operation principal core.source

theorem local_compatibility (state : ReferenceSource.State p a) (member : Fin a)
    (principal : Nat) (caller : Fin p) (item : Located) :
    execute state member principal ⟨item,caller,caller⟩ =
      ReferenceSource.advance state member caller principal item := rfl

theorem explicit_target (compiled : compile env caller raw = some (core,next))
    (explicit : raw.operation = some target) :
    core.caller = caller ∧ core.operation.val = target ∧ IsCall core.source.statement := by
  obtain ⟨sourceAt,callerAt,placed,_⟩ := (compile_exact _ _ _ _ _).mp compiled
  have chosen : IsCall raw.source.statement ∧ core.operation.val = target := by
    simpa [Placement,explicit] using placed
  exact ⟨callerAt,chosen.2,sourceAt ▸ chosen.1⟩

theorem execute_rooted {state : ReferenceSource.State p a}
    (root : ReferenceSourceTrace.Rooted realm view policy state)
    (member : Fin a) (principal : Nat) (core : Core p) :
    ReferenceSourceTrace.Rooted realm view policy (execute state member principal core).state :=
  .step root .advance

theorem execute_provenance {state : ReferenceSource.State p a}
    (valid : ReferenceSourceProvenance.Invariant state)
    (member : Fin a) (principal : Nat) (core : Core p) :
    ReferenceSourceProvenance.Invariant (execute state member principal core).state :=
  ReferenceSourceProvenance.advance_invariant _ _ _ _ _ valid

-- Explicit result input is the same protected entry; it cannot be replaced by
-- an ordinary tick of the reference evaluator. This preserves all committed
-- source/machine prefixes, including normalization before a later refusal.
theorem received_rooted (root : ReferenceSourceTrace.Rooted realm view policy state)
    (accepted : ReceivedResult.accept state entry value = some next) :
    ReferenceSourceTrace.Rooted realm view policy next := by
  have complete := ReceivedResult.accept_sound accepted
  have path : ReferenceSourceTrace.Rooted realm view policy (ReferenceSource.complete state).state :=
    .step root .complete
  simpa [complete] using path

theorem prepare_coordinates (s : InstanceState.State d p n) (view : WorldProjection.AuthorityView a)
    (member : Fin a) (key : Fin n) (place : Fin p) (principal id : Nat) (argument : Int)
    (ticket : InvocationBoundary.Ticket)
    (prepared : InvocationBoundary.prepare s view member key place principal id argument = some ticket) :
    ticket.place = place.val ∧ ticket.argument = argument ∧ ticket.principal = principal ∧ ticket.id = id := by
  unfold InvocationBoundary.prepare at prepared
  simp only [Option.bind_eq_bind,Option.bind] at prepared
  split at prepared
  · cases prepared
  · dsimp only at prepared
    split at prepared
    · cases prepared
      exact ⟨rfl,rfl,rfl,rfl⟩
    · cases prepared

theorem start_coordinates (m : CompositionMachine.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (argument : Int) (next : CompositionMachine.Machine p a)
    (ticket : InvocationBoundary.Ticket)
    (started : CompositionMachine.start m member place principal id key argument = some (next,ticket)) :
    ticket.place = place.val ∧ ticket.argument = argument ∧ ticket.principal = principal ∧ ticket.id = id := by
  unfold CompositionMachine.start at started
  cases indexed : CompositionCore.index m.system.configuration.count key with
  | none => simp [indexed] at started
  | some unit =>
      simp only [indexed,Option.bind_eq_bind,Option.bind_some] at started
      cases prepared : InvocationBoundary.prepare m.system.configuration.state m.system.view member unit place principal id argument with
      | none => simp [prepared] at started
      | some captured =>
          simp only [prepared,Option.bind_some] at started
          split at started
          · have equal : captured = ticket := (Prod.mk.inj (Option.some.inj started)).2
            subst ticket
            exact prepare_coordinates _ _ _ _ _ _ _ _ _ prepared
          · cases started

theorem beginCall_coordinates (s : ReferenceSource.State p a) (member : Fin a) (place : Fin p)
    (principal : Nat) (site : Site) (before : Nat) (deps : List ReferenceSource.Read)
    (name : String) (target : ReferenceSource.Target) (argument : Int)
    (waiting : (ReferenceSource.beginCall s member place principal site before deps name target argument).status = .waiting) :
    ∃ saved, (ReferenceSource.beginCall s member place principal site before deps name target argument).state.waiting = some saved ∧
      saved.entry.ticket.place = place.val ∧ saved.entry.ticket.argument = argument ∧
      saved.site = site ∧ saved.name = name ∧ saved.inputs = deps := by
  cases target with
  | «instance» key =>
      cases started : ReferenceExecution.startPlain s.machine member place principal s.nextRequest key argument with
      | none => simp [ReferenceSource.beginCall,started] at waiting
      | some pair =>
          obtain ⟨machine,entry⟩ := pair
          have coordinates := start_coordinates _ _ _ _ _ _ _ _ _
            (ReferenceExecution.startPlain_core _ _ _ _ _ _ _ _ started)
          exact ⟨⟨entry,site,name,deps,before⟩,by simp [ReferenceSource.beginCall,started],coordinates.1,coordinates.2.1,rfl,rfl,rfl⟩
  | reference key =>
      cases started : ReferenceExecution.startReference s.machine member place principal s.nextRequest key argument with
      | none => simp [ReferenceSource.beginCall,started] at waiting
      | some pair =>
          obtain ⟨machine,entry⟩ := pair
          obtain ⟨target,startedCore⟩ := ReferenceExecution.startReference_core _ _ _ _ _ _ _ _ started
          have coordinates := start_coordinates _ _ _ _ _ _ _ _ _ startedCore
          exact ⟨⟨entry,site,name,deps,before⟩,by simp [ReferenceSource.beginCall,started],coordinates.1,coordinates.2.1,rfl,rfl,rfl⟩

theorem plan_waiting_target (s : ReferenceSource.State p a) (member : Fin a) (place : Fin p)
    (principal : Nat) (item : Located) (plan : ReferenceSource.Plan)
    (waiting : (ReferenceSource.executePlan s member place principal item plan).status = .waiting) :
    ∃ saved, (ReferenceSource.executePlan s member place principal item plan).state.waiting = some saved ∧
      saved.entry.ticket.place = place.val ∧ saved.site = item.site ∧
      saved.inputs = ReferenceSource.inputs s item := by
  cases plan with
  | pureValue name value assign => cases waiting
  | control name command kind =>
      simp only [ReferenceSource.executePlan] at waiting
      split at waiting
      · cases waiting
      · split at waiting <;> cases waiting
  | acquire name chain => simp only [ReferenceSource.executePlan] at waiting; split at waiting <;> cases waiting
  | reacquire name key => simp only [ReferenceSource.executePlan] at waiting; split at waiting <;> cases waiting
  | release name key => simp only [ReferenceSource.executePlan] at waiting; split at waiting <;> cases waiting
  | call name target argument =>
      cases target with
      | «instance» key =>
          obtain ⟨saved,savedAt,placeAt,_,siteAt,_,depsAt⟩ := beginCall_coordinates _ _ _ _ _ _ _ _ _ _ waiting
          exact ⟨saved,savedAt,placeAt,siteAt,depsAt⟩
      | reference key =>
          simp only [ReferenceSource.executePlan] at waiting ⊢
          split at waiting
          · cases waiting
          · rename_i success
            obtain ⟨saved,savedAt,placeAt,_,siteAt,_,depsAt⟩ := beginCall_coordinates _ _ _ _ _ _ _ _ _ _ waiting
            exact ⟨saved,savedAt,placeAt,siteAt,depsAt⟩

theorem execute_waiting_target (s : ReferenceSource.State p a) (member : Fin a) (principal : Nat)
    (core : Core p) (waiting : (execute s member principal core).status = .waiting) :
    ∃ saved, (execute s member principal core).state.waiting = some saved ∧
      saved.entry.ticket.place = core.operation.val ∧ saved.site = core.source.site ∧
      saved.inputs = ReferenceSource.inputs s core.source := by
  cases pending : s.waiting with
  | some saved => simp [execute,ReferenceSource.advance,pending] at waiting
  | none =>
      cases typed : checkStatement p (environment s.values) core.source.statement with
      | none => simp [execute,ReferenceSource.advance,pending,typed] at waiting
      | some env =>
          cases compiled : ReferenceSource.elaborate s.machine.store.core.system.configuration.state s.values core.source.statement with
          | none => simp [execute,ReferenceSource.advance,pending,typed,compiled] at waiting
          | some plan =>
              have atPlan : (ReferenceSource.executePlan s member core.operation principal core.source plan).status = .waiting := by
                simpa [execute,ReferenceSource.advance,pending,typed,compiled] using waiting
              simpa [execute,ReferenceSource.advance,pending,typed,compiled] using
                (plan_waiting_target s member core.operation principal core.source plan atPlan)

#print axioms resolve_exact
#print axioms compile_exact
#print axioms explicit_target
#print axioms execute_rooted
#print axioms execute_provenance
#print axioms received_rooted
#print axioms prepare_coordinates
#print axioms start_coordinates
#print axioms beginCall_coordinates
#print axioms execute_waiting_target
end MirroreaProofFirst.SourcePlacement
