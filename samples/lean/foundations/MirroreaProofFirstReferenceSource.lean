import MirroreaProofFirstReferenceSourceData
import MirroreaProofFirstReferenceAllocation

namespace MirroreaProofFirst.ReferenceSource
open ReferenceSourceData

inductive ResultKind where
  | unit | definition | instance
  deriving DecidableEq, Repr
inductive Target where
  | instance (key : Nat) | reference (key : Nat)
  deriving DecidableEq, Repr
inductive Plan where
  | pureValue (name : String) (value : Value) (assign : Bool)
  | control (name : String) (command : CompositionCore.Raw) (kind : ResultKind)
  | call (name : String) (target : Target) (argument : Int)
  | acquire (name : String) (chain : FallbackStatic.Chain)
  | reacquire (name : String) (key : Nat)
  | release (name : String) (key : Nat)
  deriving DecidableEq, Repr

-- Pure elaboration finishes BEFORE any semantic mutation. Legacy integer
-- evaluation is reused, but legacy source execute/control are never invoked.
def elaboratePlain (values : Values) : SourceAuthoring.Statement → Option Plan
  | .register name definition predecessor => do
      let previous ← SourceAuthoring.optional (definitionKey values) predecessor
      return .control name (.register definition previous) .definition
  | .instantiate name definition places parent support => do
      let key ← definitionKey values definition
      let parent ← SourceAuthoring.optional (instanceKey values) parent
      let support ← SourceAuthoring.dependencies (plainValues values) support
      -- Owner principal is supplied at execution, not compiled from a name.
      return .control name (.instantiate key 0 places parent support) .instance
  | .retire out name => do return .control out (.retire (← instanceKey values name)) .unit
  | .reparent out name parent => do
      return .control out (.reparent (← instanceKey values name) (← SourceAuthoring.optional (instanceKey values) parent)) .unit
  | .replace out name definition => do
      return .control out (.replace (← instanceKey values name) (← definitionKey values definition)) .unit
  | .leave out place => some (.control out (.leave place) .unit)
  | .join out place => some (.control out (.join place) .unit)
  | .localValue name mutable expression => do
      return .pureValue name (.plain (.integer (← SourceAuthoring.eval (plainValues values) expression) mutable)) false
  | .assign name expression => do
      return .pureValue name (.plain (.integer (← SourceAuthoring.eval (plainValues values) expression) true)) true
  | .invoke name target expression => do
      let arg ← SourceAuthoring.eval (plainValues values) expression
      let target ← match lookup values target with
        | some (.plain (.callable key)) => some (.instance key)
        | some (.reference key) => some (.reference key)
        | _ => none
      return .call name target arg

def elaborate (s : InstanceState.State d p n) (values : Values) : Statement → Option Plan
  | .plain statement => elaboratePlain values statement
  | .acquire name decl => do
      let chain ← resolveChain s values decl
      match FallbackStatic.check s chain with
      | .error _ => none
      | .ok _ => some (.acquire name chain)
  | .alias name target => do return .pureValue name (.reference (← referenceKey values target)) false
  | .reacquire name target => do return .reacquire name (← referenceKey values target)
  | .release name target => do return .release name (← referenceKey values target)

def expressionReads : SourceAuthoring.Expr → List String
  | .integer _ => [] | .read name => [name]
  | .add left right | .mul left right => expressionReads left ++ expressionReads right
def supportReads : Support.Formula String → List String
  | .top | .bottom => [] | .ref name => [name]
  | .both left right | .either left right => supportReads left ++ supportReads right
def plainReads : SourceAuthoring.Statement → List String
  | .register _ _ predecessor => predecessor.toList
  | .instantiate _ definition _ parent support => definition :: parent.toList ++ supportReads support
  | .retire _ target => [target]
  | .reparent _ target parent => target :: parent.toList
  | .replace _ target definition => [target,definition]
  | .leave .. | .join .. => []
  | .localValue _ _ expression | .assign _ expression => expressionReads expression
  | .invoke _ target expression => target :: expressionReads expression
def reads : Statement → List String
  | .plain statement => plainReads statement
  | .acquire _ chain => chain.reader :: chain.options.map OptionSyntax.target
  | .alias _ target | .reacquire _ target | .release _ target => [target]

structure Read where
  name : String
  value : Option Value
  producer : Option Nat
  deriving DecidableEq, Repr
structure Write where
  ordinal : Nat
  site : Site
  name : String
  value : Value
  inputs : List Read
  machineBefore : Nat
  machineAfter : Nat
  deriving DecidableEq, Repr
inductive MicroKind where
  | control | acquire | normalize | reacquire | release | request | result | authorityHead | cancellation
  deriving DecidableEq, Repr
structure Origin where
  site : Option Site
  kind : MicroKind
  beforeCount : Nat
  afterCount : Nat
  deriving DecidableEq, Repr
structure Awaiting where
  entry : ReferenceExecution.Pending
  site : Site
  name : String
  inputs : List Read
  machineBefore : Nat
  deriving DecidableEq, Repr

structure State (p a : Nat) where
  machine : ReferenceExecution.Machine p a
  values : Values
  nextRequest : Nat
  writes : List Write
  origins : List Origin
  waiting : Option Awaiting

def initial (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy) : State p a :=
  ⟨ReferenceExecution.initial realm view policy,[],0,[],[],none⟩

def read (s : State p a) (name : String) : Read :=
  ⟨name,lookup s.values name,(s.writes.find? fun write => write.name == name).map Write.ordinal⟩
def inputs (s : State p a) (item : Located) : List Read := (reads item.statement).map (read s)

def updateValue (values : Values) (name : String) (value : Value) : Values :=
  match values with
  | [] => []
  | row :: rest => if row.1 = name then (name,value) :: rest else row :: updateValue rest name value

-- These records annotate actual state transitions. They do not add core or
-- binding events, and cannot manufacture an absent result or source write.
def mark (s : State p a) (next : ReferenceExecution.Machine p a) (origin : Option Site)
    (kind : MicroKind) (consumed : Nat) : State p a :=
  {s with
    machine := next,nextRequest := s.nextRequest+consumed,
    origins := ⟨origin,kind,s.machine.store.events.length,next.store.events.length⟩ :: s.origins}

def write (s : State p a) (site : Site) (beforeCount : Nat) (deps : List Read)
    (name : String) (value : Value) (assign : Bool) : State p a :=
  {s with
    values := if assign then updateValue s.values name value else (name,value) :: s.values,
    writes := ⟨s.writes.length,site,name,value,deps,beforeCount,s.machine.store.events.length⟩ :: s.writes}

inductive Failure where
  | awaiting | staticType | elaboration | rejected | missingCreated | cancelled
  | normalization (reason : ReferenceMutation.Error)
  deriving DecidableEq, Repr
inductive Status where
  | ready | waiting | failed (reason : Failure)
  deriving DecidableEq, Repr
structure Outcome (p a : Nat) where
  state : State p a
  status : Status

def resultValue : ResultKind → Option Nat → Option Value
  | .unit,_ => some (.plain .unit)
  | .definition,some key => some (.plain (.definition key))
  | .instance,some key => some (.plain (.callable key))
  | _,none => none

def withOwner (principal : Nat) : CompositionCore.Raw → CompositionCore.Raw
  | .instantiate definition _ places parent support => .instantiate definition principal places parent support
  | command => command

-- Exposed to the LAB origin proof; only SourceTrace entries are admitted.
def beginCall (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (site : Site) (beforeCount : Nat) (deps : List Read) (name : String) (target : Target) (argument : Int) : Outcome p a :=
  let pending := match target with
    | .instance key => ReferenceExecution.startPlain s.machine member place principal s.nextRequest key argument
    | .reference key => ReferenceExecution.startReference s.machine member place principal s.nextRequest key argument
  match pending with
  | none => ⟨s,.failed .rejected⟩
  | some (machine,entry) =>
      let next := mark s machine (some site) .request 1
      ⟨{next with waiting := some ⟨entry,site,name,deps,beforeCount⟩},.waiting⟩

def executePlan (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located) : Plan → Outcome p a
  | .pureValue name value assign =>
      ⟨write s item.site s.machine.store.events.length (inputs s item) name value assign,.ready⟩
  | .control name command kind =>
      match ReferenceExecution.manage s.machine member place principal s.nextRequest (withOwner principal command) with
      | none => ⟨s,.failed .rejected⟩
      | some (machine,created) =>
          let next := mark s machine (some item.site) .control 1
          match resultValue kind created with
          | none => ⟨next,.failed .missingCreated⟩
          | some value => ⟨write next item.site s.machine.store.events.length (inputs s item) name value false,.ready⟩
  | .acquire name chain =>
      match ReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
      | none => ⟨s,.failed .rejected⟩
      | some (machine,key) =>
          let next := mark s machine (some item.site) .acquire 1
          ⟨write next item.site s.machine.store.events.length (inputs s item) name (.reference key) false,.ready⟩
  | .reacquire name key =>
      match ReferenceExecution.reacquire s.machine member place principal s.nextRequest key with
      | none => ⟨s,.failed .rejected⟩
      | some machine =>
          let next := mark s machine (some item.site) .reacquire 1
          ⟨write next item.site s.machine.store.events.length (inputs s item) name (.plain .unit) false,.ready⟩
  | .release name key =>
      match ReferenceExecution.release s.machine member place principal s.nextRequest key with
      | none => ⟨s,.failed .rejected⟩
      | some machine =>
          let next := mark s machine (some item.site) .release 1
          ⟨write next item.site s.machine.store.events.length (inputs s item) name (.plain .unit) false,.ready⟩
  | .call name (.instance key) argument =>
      beginCall s member place principal item.site s.machine.store.events.length (inputs s item) name (.instance key) argument
  | .call name (.reference key) argument =>
      let normalized := ReferenceExecution.normalize s.machine member place principal s.nextRequest key
      let next := if normalized.consumed then mark s normalized.state (some item.site) .normalize 1 else s
      match normalized.result with
      | .error reason => ⟨next,.failed (.normalization reason)⟩
      | .ok _ => beginCall next member place principal item.site s.machine.store.events.length (inputs s item) name (.reference key) argument

-- The whole current statement is type-checked before even normalization.
-- An error carries its actual state; there is no outer Option to roll it back.
def advance (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located) : Outcome p a :=
  if s.waiting.isSome then ⟨s,.failed .awaiting⟩ else
  match checkStatement p (environment s.values) item.statement with
  | none => ⟨s,.failed .staticType⟩
  | some _ => match elaborate s.machine.store.core.system.configuration.state s.values item.statement with
    | none => ⟨s,.failed .elaboration⟩
    | some plan => executePlan s member place principal item plan

-- A real source continuation admits interleaved authority/control transitions.
-- Completion executes the saved checked Mir function and uses the protected
-- finish entry. Failure retains the request and adds no successful source write.
def complete (s : State p a) : Outcome p a :=
  match s.waiting with
  | none => ⟨s,.failed .awaiting⟩
  | some saved => match ReferenceExecution.resume s.machine saved.entry with
    | none => ⟨s,.failed .rejected⟩
    | some (machine,value) =>
        let next := mark s machine (some saved.site) .result 0
        ⟨{write next saved.site saved.machineBefore saved.inputs saved.name (.plain (.integer value false)) false with waiting := none},.ready⟩

-- Control-plane cancellation acts on the saved originating source request.
-- The programmer supplies neither event nor request id; the allocator advances
-- only after the real authorized cancellation commits. Failed cancellation
-- preserves the pending continuation; success does not invent its output value.
def cancel (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) : Outcome p a :=
  match s.waiting with
  | none => ⟨s,.failed .awaiting⟩
  | some saved => match ReferenceExecution.cancel s.machine member place principal s.nextRequest saved.entry with
    | none => ⟨s,.failed .rejected⟩
    | some machine =>
        ⟨{mark s machine (some saved.site) .cancellation 1 with waiting := none},.ready⟩

def authorityHead (s : State p a) (view : WorldProjection.AuthorityView a) : State p a :=
  mark s (ReferenceExecution.authorityHead s.machine view) none .authorityHead 0

-- Typed environment control can interleave with an outstanding source request.
-- It uses the SAME allocator/checked management entry and preserves the source
-- continuation. This input is not counted as source-authored construction.
def controlInput (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) : Option (State p a × Option Nat) := do
  let (machine,created) ← ReferenceExecution.manage s.machine member place principal s.nextRequest raw
  return (mark s machine none .control 1,created)

-- Synchronous driver is merely a consumer of the same exposed continuation.
-- Tests can pause after advance, apply an actual control/head, then complete.
def run (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) : List Located → Outcome p a
  | [] => ⟨s,if s.waiting.isSome then .waiting else .ready⟩
  | item :: rest =>
      let result := advance s member place principal item
      match result.status with
      | .failed _ => result
      | .ready => run result.state member place principal rest
      | .waiting =>
          let resumed := complete result.state
          match resumed.status with
          | .ready => run resumed.state member place principal rest
          | _ => resumed

private theorem machine_trans (first middle last : ReferenceExecution.Machine p a)
    (left : ReferenceExecution.Reached first middle) (right : ReferenceExecution.Reached middle last) :
    ReferenceExecution.Reached first last := by
  induction right with
  | refl => exact left
  | step _ step ih => exact .step ih step

theorem beginCall_projects (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (site : Site) (beforeCount : Nat) (deps : List Read) (name : String) (target : Target) (argument : Int) :
    ReferenceExecution.Reached s.machine
      (beginCall s member place principal site beforeCount deps name target argument).state.machine := by
  cases target with
  | «instance» key =>
      simp only [beginCall]
      cases run : ReferenceExecution.startPlain s.machine member place principal s.nextRequest key argument with
      | none => exact .refl
      | some pair => obtain ⟨machine,entry⟩ := pair; exact .step .refl (.startPlain run)
  | reference key =>
      simp only [beginCall]
      cases run : ReferenceExecution.startReference s.machine member place principal s.nextRequest key argument with
      | none => exact .refl
      | some pair => obtain ⟨machine,entry⟩ := pair; exact .step .refl (.startReference run)

theorem executePlan_projects (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located) (plan : Plan) :
    ReferenceExecution.Reached s.machine (executePlan s member place principal item plan).state.machine := by
  cases plan with
  | pureValue name value assign => exact .refl
  | control name command kind =>
      simp only [executePlan]
      cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest (withOwner principal command) with
      | none => exact .refl
      | some pair =>
          obtain ⟨machine,created⟩ := pair
          dsimp only
          cases resultValue kind created <;> exact .step .refl (.management run)
  | acquire name chain =>
      simp only [executePlan]
      cases run : ReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
      | none => exact .refl
      | some pair => obtain ⟨machine,key⟩ := pair; exact .step .refl (.acquire run)
  | reacquire name key =>
      simp only [executePlan]
      cases run : ReferenceExecution.reacquire s.machine member place principal s.nextRequest key with
      | none => exact .refl
      | some machine => exact .step .refl (.reacquire run)
  | release name key =>
      simp only [executePlan]
      cases run : ReferenceExecution.release s.machine member place principal s.nextRequest key with
      | none => exact .refl
      | some machine => exact .step .refl (.release run)
  | call name target argument =>
      cases target with
      | «instance» key => exact beginCall_projects _ _ _ _ _ _ _ _ _ _
      | reference key =>
          let normalized := ReferenceExecution.normalize s.machine member place principal s.nextRequest key
          let next := if normalized.consumed then mark s normalized.state (some item.site) .normalize 1 else s
          have priorSteps : ReferenceExecution.Reached s.machine next.machine := by
            unfold next
            split
            · exact .step .refl .normalize
            · exact .refl
          change ReferenceExecution.Reached s.machine
            (match normalized.result with
            | .error reason => (⟨next,.failed (.normalization reason)⟩ : Outcome p a)
            | .ok _ => beginCall next member place principal item.site s.machine.store.events.length (inputs s item) name (.reference key) argument).state.machine
          cases normalized.result with
          | error _ => exact priorSteps
          | ok _ => exact machine_trans _ _ _ priorSteps (beginCall_projects _ _ _ _ _ _ _ _ _ _)

theorem advance_projects (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located) :
    ReferenceExecution.Reached s.machine (advance s member place principal item).state.machine := by
  unfold advance
  split
  · exact .refl
  · cases checkStatement p (environment s.values) item.statement with
    | none => exact .refl
    | some env =>
        cases elaborate s.machine.store.core.system.configuration.state s.values item.statement with
        | none => exact .refl
        | some plan => exact executePlan_projects _ _ _ _ _ _

theorem complete_projects (s : State p a) : ReferenceExecution.Reached s.machine (complete s).state.machine := by
  cases waiting : s.waiting with
  | none => simpa [complete,waiting] using (ReferenceExecution.Reached.refl (m:=s.machine))
  | some saved =>
      simp only [complete,waiting]
      cases resumed : ReferenceExecution.resume s.machine saved.entry with
      | none => exact .refl
      | some pair =>
          obtain ⟨machine,value⟩ := pair
          unfold ReferenceExecution.resume at resumed
          cases evaluated : InvocationBoundary.execute saved.entry.ticket with
          | none => simp [evaluated] at resumed
          | some result =>
              simp only [evaluated,Option.bind_eq_bind,Option.bind_some] at resumed
              cases finished : ReferenceExecution.finish s.machine saved.entry result with
              | none => simp [finished] at resumed
              | some next =>
                  simp only [finished,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at resumed
                  obtain ⟨rfl,rfl⟩ := resumed
                  exact .step .refl (.finish finished)

theorem run_projects (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (program : List Located) :
    ReferenceExecution.Reached s.machine (run s member place principal program).state.machine := by
  induction program generalizing s with
  | nil => exact .refl
  | cons item rest ih =>
      simp only [run]
      cases advanced : (advance s member place principal item).status with
      | failed reason => exact advance_projects _ _ _ _ _
      | ready => exact machine_trans _ _ _ (advance_projects _ _ _ _ _) (ih _)
      | waiting =>
          dsimp only
          cases resumed : (complete (advance s member place principal item).state).status with
          | ready =>
              exact machine_trans _ _ _ (advance_projects _ _ _ _ _)
                (machine_trans _ _ _ (complete_projects _) (ih _))
          | waiting => exact machine_trans _ _ _ (advance_projects _ _ _ _ _) (complete_projects _)
          | failed _ => exact machine_trans _ _ _ (advance_projects _ _ _ _ _) (complete_projects _)

theorem run_preserves_machine (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (program : List Located)
    (valid : ReferenceExecution.Invariant s.machine) :
    ReferenceExecution.Invariant (run s member place principal program).state.machine :=
  ReferenceExecution.reached_preserves _ _ valid (run_projects _ _ _ _ _)

theorem static_rejection_unchanged (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located)
    (idle : s.waiting = none) (illTyped : checkStatement p (environment s.values) item.statement = none) :
    advance s member place principal item = ⟨s,.failed .staticType⟩ := by simp [advance,idle,illTyped]

theorem completion_rejection_unchanged (s : State p a) (reason : Failure)
    (rejected : (complete s).status = .failed reason) : (complete s).state = s := by
  cases waiting : s.waiting with
  | none => simp [complete,waiting]
  | some saved =>
      cases resumed : ReferenceExecution.resume s.machine saved.entry with
      | none => simp [complete,waiting,resumed]
      | some pair => simp [complete,waiting,resumed] at rejected

theorem head_projects (s : State p a) (view : WorldProjection.AuthorityView a) :
    ReferenceExecution.Reached s.machine (authorityHead s view).machine := .step .refl .authority

theorem controlInput_projects (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) (result : State p a × Option Nat)
    (accepted : controlInput s member place principal raw = some result) :
    ReferenceExecution.Reached s.machine result.1.machine ∧ result.1.waiting = s.waiting ∧
      result.1.values = s.values ∧ result.1.writes = s.writes := by
  unfold controlInput at accepted
  cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest raw with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨machine,created⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      exact ⟨.step .refl (.management run),rfl,rfl,rfl⟩

theorem complete_parts (s : State p a) (result : State p a)
    (completed : complete s = ⟨result,.ready⟩) :
    ∃ saved machine value, s.waiting = some saved ∧
      InvocationBoundary.execute saved.entry.ticket = some value ∧
      ReferenceExecution.finish s.machine saved.entry value = some machine ∧
      result = {write (mark s machine (some saved.site) .result 0) saved.site saved.machineBefore
        saved.inputs saved.name (.plain (.integer value false)) false with waiting := none} := by
  unfold complete at completed
  cases waiting : s.waiting with
  | none => simp [waiting] at completed
  | some saved =>
      simp only [waiting] at completed
      cases resumed : ReferenceExecution.resume s.machine saved.entry with
      | none => simp [resumed] at completed
      | some pair =>
          obtain ⟨machine,value⟩ := pair
          simp only [resumed,Outcome.mk.injEq,and_true] at completed
          unfold ReferenceExecution.resume at resumed
          cases evaluated : InvocationBoundary.execute saved.entry.ticket with
          | none => simp [evaluated] at resumed
          | some actual =>
              simp only [evaluated,Option.bind_eq_bind,Option.bind_some] at resumed
              cases finished : ReferenceExecution.finish s.machine saved.entry actual with
              | none => simp [finished] at resumed
              | some next =>
                  simp only [finished,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at resumed
                  obtain ⟨rfl,rfl⟩ := resumed
                  exact ⟨saved,next,actual,rfl,evaluated,finished,completed.symm⟩

-- The successful SOURCE write comes from execution of the exact saved Mir
-- code and acceptance by the binding-aware completion entry. This judgment
-- quantifies over arbitrary states/tickets, not a fixed expected result JSON.
theorem complete_write_meaning (s : State p a) (result : State p a)
    (completed : complete s = ⟨result,.ready⟩) :
    ∃ saved value, s.waiting = some saved ∧
      ReferenceExecution.Protected s.machine.store saved.entry ∧
      InvocationBoundary.ResultMeaning s.machine.store.core.system.configuration.state
        s.machine.store.core.system.view saved.entry.ticket value ∧
      result.waiting = none ∧ result.values = (saved.name,.plain (.integer value false)) :: s.values ∧
      result.writes = ⟨s.writes.length,saved.site,saved.name,.plain (.integer value false),saved.inputs,
        saved.machineBefore,result.machine.store.events.length⟩ :: s.writes := by
  obtain ⟨saved,machine,value,waiting,_,finished,rfl⟩ := complete_parts _ _ completed
  obtain ⟨guarded,meaning⟩ := ReferenceExecution.finish_meaning _ _ _ _ finished
  exact ⟨saved,value,waiting,guarded,meaning,rfl,rfl,rfl⟩

def Allocated (s : State p a) : Prop := ReferenceAllocation.Below s.machine s.nextRequest

theorem initial_allocated (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy) :
    Allocated (initial (p:=p) realm view policy) := ReferenceAllocation.initial_below _ _ _

theorem beginCall_allocated (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (site : Site) (beforeCount : Nat) (deps : List Read) (name : String) (target : Target) (argument : Int)
    (valid : Allocated s) : Allocated (beginCall s member place principal site beforeCount deps name target argument).state := by
  cases target with
  | «instance» key =>
      simp only [beginCall]
      cases run : ReferenceExecution.startPlain s.machine member place principal s.nextRequest key argument with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,entry⟩ := pair
          exact ReferenceAllocation.startPlain_below _ _ _ _ _ _ _ _ valid run
  | reference key =>
      simp only [beginCall]
      cases run : ReferenceExecution.startReference s.machine member place principal s.nextRequest key argument with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,entry⟩ := pair
          exact ReferenceAllocation.startReference_below _ _ _ _ _ _ _ _ valid run

theorem executePlan_allocated (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (plan : Plan) (valid : Allocated s) : Allocated (executePlan s member place principal item plan).state := by
  cases plan with
  | pureValue name value assign => exact valid
  | control name command kind =>
      simp only [executePlan]
      cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest (withOwner principal command) with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,created⟩ := pair
          have bounds := ReferenceAllocation.manage_below _ _ _ _ _ _ _ valid run
          dsimp only
          cases resultValue kind created <;> exact bounds
  | acquire name chain =>
      simp only [executePlan]
      cases run : ReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,key⟩ := pair
          exact ReferenceAllocation.acquire_below _ _ _ _ _ _ _ valid run
  | reacquire name key =>
      simp only [executePlan]
      cases run : ReferenceExecution.reacquire s.machine member place principal s.nextRequest key with
      | none => exact valid
      | some machine => exact ReferenceAllocation.reacquire_below _ _ _ _ _ _ _ valid run
  | release name key =>
      simp only [executePlan]
      cases run : ReferenceExecution.release s.machine member place principal s.nextRequest key with
      | none => exact valid
      | some machine => exact ReferenceAllocation.release_below _ _ _ _ _ _ _ valid run
  | call name target argument =>
      cases target with
      | «instance» key => exact beginCall_allocated _ _ _ _ _ _ _ _ _ _ valid
      | reference key =>
          let normalized := ReferenceExecution.normalize s.machine member place principal s.nextRequest key
          let next := if normalized.consumed then mark s normalized.state (some item.site) .normalize 1 else s
          have nextValid : Allocated next := by
            unfold next
            split
            · rename_i consumed
              exact ReferenceAllocation.normalize_consumed_below _ _ _ _ _ _ valid consumed
            · exact valid
          change Allocated
            (match normalized.result with
            | .error reason => (⟨next,.failed (.normalization reason)⟩ : Outcome p a)
            | .ok _ => beginCall next member place principal item.site s.machine.store.events.length (inputs s item) name (.reference key) argument).state
          cases normalized.result with
          | error _ => exact nextValid
          | ok _ => exact beginCall_allocated _ _ _ _ _ _ _ _ _ _ nextValid

theorem advance_allocated (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located)
    (valid : Allocated s) : Allocated (advance s member place principal item).state := by
  unfold advance
  split
  · exact valid
  · cases checkStatement p (environment s.values) item.statement with
    | none => exact valid
    | some env =>
        cases elaborate s.machine.store.core.system.configuration.state s.values item.statement with
        | none => exact valid
        | some plan => exact executePlan_allocated _ _ _ _ _ _ valid

theorem complete_allocated (s : State p a) (valid : Allocated s) : Allocated (complete s).state := by
  cases waiting : s.waiting with
  | none => simpa [complete,waiting] using valid
  | some saved =>
      simp only [complete,waiting]
      cases resumed : ReferenceExecution.resume s.machine saved.entry with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,value⟩ := pair
          exact ReferenceAllocation.resume_below _ _ _ _ valid resumed

theorem run_allocated (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (program : List Located)
    (valid : Allocated s) : Allocated (run s member place principal program).state := by
  induction program generalizing s with
  | nil => exact valid
  | cons item rest ih =>
      simp only [run]
      cases advanced : (advance s member place principal item).status with
      | failed reason => exact advance_allocated _ _ _ _ _ valid
      | ready => exact ih _ (advance_allocated _ _ _ _ _ valid)
      | waiting =>
          dsimp only
          have bounds := complete_allocated (advance s member place principal item).state
            (advance_allocated s member place principal item valid)
          cases resumed : (complete (advance s member place principal item).state).status with
          | ready => exact ih _ bounds
          | waiting => exact bounds
          | failed _ => exact bounds

theorem head_allocated (s : State p a) (view : WorldProjection.AuthorityView a) (valid : Allocated s) :
    Allocated (authorityHead s view) := ReferenceAllocation.head_below _ _ _ valid

theorem controlInput_allocated (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) (result : State p a × Option Nat) (valid : Allocated s)
    (accepted : controlInput s member place principal raw = some result) : Allocated result.1 := by
  unfold controlInput at accepted
  cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest raw with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨machine,created⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      exact ReferenceAllocation.manage_below _ _ _ _ _ _ _ valid run

theorem fresh_next (s : State p a) (realm principal : Nat) (valid : Allocated s) :
    CompositionMachine.fresh s.machine.store.core ⟨realm,principal,s.nextRequest⟩ = true :=
  ReferenceAllocation.fresh_bound _ _ _ _ valid

private theorem beginCall_shape (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (site : Site) (beforeCount : Nat) (deps : List Read) (name : String) (target : Target) (argument : Int) :
    let result := beginCall s member place principal site beforeCount deps name target argument
    result.state.values = s.values ∧ result.status ≠ .ready ∧
      (result.status = .waiting → ∃ saved, result.state.waiting = some saved ∧ saved.name = name) ∧
      (result.status ≠ .waiting → result.state.waiting = s.waiting) := by
  cases target with
  | «instance» key =>
      simp only [beginCall]
      cases ReferenceExecution.startPlain s.machine member place principal s.nextRequest key argument with
      | none => simp
      | some pair => obtain ⟨machine,entry⟩ := pair; simp [mark]
  | reference key =>
      simp only [beginCall]
      cases ReferenceExecution.startReference s.machine member place principal s.nextRequest key argument with
      | none => simp
      | some pair => obtain ⟨machine,entry⟩ := pair; simp [mark]

theorem executeCall_shape (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (name : String) (target : Target) (argument : Int) :
    let result := executePlan s member place principal item (.call name target argument)
    result.state.values = s.values ∧ result.status ≠ .ready ∧
      (result.status = .waiting → ∃ saved, result.state.waiting = some saved ∧ saved.name = name) ∧
      (result.status ≠ .waiting → result.state.waiting = s.waiting) := by
  cases target with
  | «instance» key => exact beginCall_shape _ _ _ _ _ _ _ _ _ _
  | reference key =>
      let normalized := ReferenceExecution.normalize s.machine member place principal s.nextRequest key
      let next := if normalized.consumed then mark s normalized.state (some item.site) .normalize 1 else s
      have same : next.values = s.values := by unfold next; split <;> rfl
      have sameWaiting : next.waiting = s.waiting := by unfold next; split <;> rfl
      change (let result := match normalized.result with
        | .error reason => (⟨next,.failed (.normalization reason)⟩ : Outcome p a)
        | .ok _ => beginCall next member place principal item.site s.machine.store.events.length (inputs s item) name (.reference key) argument
        result.state.values = s.values ∧ result.status ≠ .ready ∧
          (result.status = .waiting → ∃ saved, result.state.waiting = some saved ∧ saved.name = name) ∧
          (result.status ≠ .waiting → result.state.waiting = s.waiting))
      cases normalized.result with
      | error reason => simp [same,sameWaiting]
      | ok choice =>
          simpa only [same,sameWaiting] using (beginCall_shape next member place principal item.site
            s.machine.store.events.length (inputs s item) name (.reference key) argument)

private theorem beginCall_evidence (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (site : Site) (beforeCount : Nat) (deps : List Read) (name : String) (target : Target) (argument : Int) :
    let result := beginCall s member place principal site beforeCount deps name target argument
    result.state.writes = s.writes ∧
      (∀ saved, result.state.waiting = some saved →
        s.waiting = some saved ∨ (saved.inputs = deps ∧ saved.site = site ∧ saved.machineBefore = beforeCount)) := by
  cases target with
  | «instance» key =>
      simp only [beginCall]
      cases ReferenceExecution.startPlain s.machine member place principal s.nextRequest key argument with
      | none => exact ⟨rfl,fun _ savedAt => Or.inl savedAt⟩
      | some pair => obtain ⟨machine,entry⟩ := pair; simp [mark]
  | reference key =>
      simp only [beginCall]
      cases ReferenceExecution.startReference s.machine member place principal s.nextRequest key argument with
      | none => exact ⟨rfl,fun _ savedAt => Or.inl savedAt⟩
      | some pair => obtain ⟨machine,entry⟩ := pair; simp [mark]

theorem executeCall_evidence (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (name : String) (target : Target) (argument : Int) :
    let result := executePlan s member place principal item (.call name target argument)
    result.state.writes = s.writes ∧
      (∀ saved, result.state.waiting = some saved →
        s.waiting = some saved ∨ (saved.inputs = inputs s item ∧ saved.site = item.site ∧
          saved.machineBefore = s.machine.store.events.length)) := by
  cases target with
  | «instance» key => exact beginCall_evidence _ _ _ _ _ _ _ _ _ _
  | reference key =>
      let normalized := ReferenceExecution.normalize s.machine member place principal s.nextRequest key
      let next := if normalized.consumed then mark s normalized.state (some item.site) .normalize 1 else s
      have same : next.writes = s.writes := by unfold next; split <;> rfl
      have sameWaiting : next.waiting = s.waiting := by unfold next; split <;> rfl
      change (let result := match normalized.result with
        | .error reason => (⟨next,.failed (.normalization reason)⟩ : Outcome p a)
        | .ok _ => beginCall next member place principal item.site s.machine.store.events.length (inputs s item) name (.reference key) argument
        result.state.writes = s.writes ∧
          (∀ saved, result.state.waiting = some saved → s.waiting = some saved ∨
            (saved.inputs = inputs s item ∧ saved.site = item.site ∧ saved.machineBefore = s.machine.store.events.length)))
      cases normalized.result with
      | error reason => exact ⟨same,fun _ savedAt => Or.inl (sameWaiting.symm.trans savedAt)⟩
      | ok choice =>
          simpa only [same,sameWaiting] using (beginCall_evidence next member place principal item.site
            s.machine.store.events.length (inputs s item) name (.reference key) argument)

#print axioms executeCall_evidence
#print axioms executeCall_shape
-- Exact source-owned continuation coverage, including rejected completion.
-- This profile has at most one pending SOURCE request; the engine itself does
-- not conflate that profile with a general network-concurrency guarantee.
def PendingAgrees (s : State p a) : Prop :=
  s.machine.pending = s.waiting.toList.map Awaiting.entry

theorem initial_pending (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy) :
    PendingAgrees (initial (p:=p) realm view policy) := rfl

theorem beginCall_pending (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (site : Site) (beforeCount : Nat) (deps : List Read) (name : String) (target : Target) (argument : Int)
    (idle : s.waiting = none) (valid : PendingAgrees s) :
    PendingAgrees (beginCall s member place principal site beforeCount deps name target argument).state := by
  have empty : s.machine.pending = [] := by simpa [PendingAgrees,idle] using valid
  cases target with
  | «instance» key =>
      simp only [beginCall]
      cases run : ReferenceExecution.startPlain s.machine member place principal s.nextRequest key argument with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,entry⟩ := pair
          have shape := (ReferenceExecution.startPlain_pending _ _ _ _ _ _ _ _ run).1
          simpa [PendingAgrees,mark,empty] using shape
  | reference key =>
      simp only [beginCall]
      cases run : ReferenceExecution.startReference s.machine member place principal s.nextRequest key argument with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,entry⟩ := pair
          have shape := (ReferenceExecution.startReference_pending _ _ _ _ _ _ _ _ run).1
          simpa [PendingAgrees,mark,empty] using shape

theorem executePlan_pending (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (plan : Plan) (idle : s.waiting = none) (valid : PendingAgrees s) :
    PendingAgrees (executePlan s member place principal item plan).state := by
  cases plan with
  | pureValue name value assign => exact valid
  | control name command kind =>
      simp only [executePlan]
      cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest (withOwner principal command) with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,created⟩ := pair
          have pending : machine.pending = s.machine.pending := ReferenceExecution.manage_pending _ _ _ _ _ _ _ run
          dsimp only
          cases resultValue kind created <;> simpa [PendingAgrees,write,mark,pending] using valid
  | acquire name chain =>
      simp only [executePlan]
      cases run : ReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,key⟩ := pair
          have pending : machine.pending = s.machine.pending := ReferenceExecution.acquire_pending _ _ _ _ _ _ _ run
          simpa [PendingAgrees,write,mark,pending] using valid
  | reacquire name key =>
      simp only [executePlan]
      cases run : ReferenceExecution.reacquire s.machine member place principal s.nextRequest key with
      | none => exact valid
      | some machine =>
          have pending : machine.pending = s.machine.pending := ReferenceExecution.reacquire_pending _ _ _ _ _ _ _ run
          simpa [PendingAgrees,write,mark,pending] using valid
  | release name key =>
      simp only [executePlan]
      cases run : ReferenceExecution.release s.machine member place principal s.nextRequest key with
      | none => exact valid
      | some machine =>
          have pending : machine.pending = s.machine.pending := ReferenceExecution.release_pending _ _ _ _ _ _ _ run
          simpa [PendingAgrees,write,mark,pending] using valid
  | call name target argument =>
      cases target with
      | «instance» key => exact beginCall_pending _ _ _ _ _ _ _ _ _ _ idle valid
      | reference key =>
          let normalized := ReferenceExecution.normalize s.machine member place principal s.nextRequest key
          let next := if normalized.consumed then mark s normalized.state (some item.site) .normalize 1 else s
          have nextIdle : next.waiting = none := by unfold next; split <;> exact idle
          have nextValid : PendingAgrees next := by
            unfold next
            split
            · exact valid
            · exact valid
          change PendingAgrees
            (match normalized.result with
            | .error reason => (⟨next,.failed (.normalization reason)⟩ : Outcome p a)
            | .ok _ => beginCall next member place principal item.site s.machine.store.events.length (inputs s item) name (.reference key) argument).state
          cases normalized.result with
          | error _ => exact nextValid
          | ok _ => exact beginCall_pending _ _ _ _ _ _ _ _ _ _ nextIdle nextValid

theorem advance_pending (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located)
    (valid : PendingAgrees s) : PendingAgrees (advance s member place principal item).state := by
  cases waiting : s.waiting with
  | some saved => simpa [advance,waiting] using valid
  | none =>
      simp only [advance,waiting,Option.isSome_none,Bool.false_eq_true,ite_false]
      cases checkStatement p (environment s.values) item.statement with
      | none => exact valid
      | some env =>
          cases elaborate s.machine.store.core.system.configuration.state s.values item.statement with
          | none => exact valid
          | some plan => exact executePlan_pending _ _ _ _ _ _ waiting valid

theorem complete_pending (s : State p a) (valid : PendingAgrees s) : PendingAgrees (complete s).state := by
  cases status : (complete s).status with
  | failed reason => simpa [completion_rejection_unchanged _ _ status] using valid
  | waiting =>
      cases waiting : s.waiting with
      | none => simp [complete,waiting] at status
      | some saved =>
          cases resumed : ReferenceExecution.resume s.machine saved.entry <;> simp [complete,waiting,resumed] at status
  | ready =>
      obtain ⟨saved,machine,value,waiting,_,finished,equal⟩ := complete_parts s (complete s).state
        (by cases h : complete s; simp_all)
      have only : s.machine.pending = [saved.entry] := by simpa [PendingAgrees,waiting] using valid
      obtain ⟨_,_,next,_,shape⟩ := ReferenceExecution.finish_parts _ _ _ _ finished
      rw [equal,shape]
      simp [PendingAgrees,write,mark,only,ReferenceExecution.keep]

theorem head_pending (s : State p a) (view : WorldProjection.AuthorityView a) (valid : PendingAgrees s) :
    PendingAgrees (authorityHead s view) := valid

theorem controlInput_pending (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) (result : State p a × Option Nat) (valid : PendingAgrees s)
    (accepted : controlInput s member place principal raw = some result) : PendingAgrees result.1 := by
  unfold controlInput at accepted
  cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest raw with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨machine,created⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      have pending : machine.pending = s.machine.pending := ReferenceExecution.manage_pending _ _ _ _ _ _ _ run
      simpa [PendingAgrees,mark,pending] using valid

#print axioms initial_pending
#print axioms beginCall_pending
#print axioms executePlan_pending
#print axioms advance_pending
#print axioms complete_pending
#print axioms head_pending
#print axioms controlInput_pending
#print axioms initial_allocated
#print axioms beginCall_allocated
#print axioms executePlan_allocated
#print axioms advance_allocated
#print axioms complete_allocated
#print axioms run_allocated
#print axioms head_allocated
#print axioms controlInput_allocated
#print axioms fresh_next
#print axioms beginCall_projects
#print axioms executePlan_projects
#print axioms advance_projects
#print axioms complete_projects
#print axioms run_projects
#print axioms run_preserves_machine
#print axioms static_rejection_unchanged
#print axioms completion_rejection_unchanged
#print axioms head_projects
#print axioms controlInput_projects
#print axioms complete_parts
#print axioms complete_write_meaning
-- Further source name/type preservation, source-write/current-result meaning,
-- allocator and provenance completeness remain consumer obligations. This is
-- nonproduction research, not W3 acceptance by imported-layer proof alone.
theorem cancel_projects (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) :
    ReferenceExecution.Reached s.machine (cancel s member place principal).state.machine := by
  cases waiting : s.waiting with
  | none => simpa [cancel,waiting] using (ReferenceExecution.Reached.refl (m:=s.machine))
  | some saved =>
      simp only [cancel,waiting]
      cases run : ReferenceExecution.cancel s.machine member place principal s.nextRequest saved.entry with
      | none => exact .refl
      | some next => exact .step .refl (.cancellation run)

theorem cancel_allocated (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (valid : Allocated s) :
    Allocated (cancel s member place principal).state := by
  cases waiting : s.waiting with
  | none => simpa [cancel,waiting] using valid
  | some saved =>
      simp only [cancel,waiting]
      cases run : ReferenceExecution.cancel s.machine member place principal s.nextRequest saved.entry with
      | none => exact valid
      | some next => exact ReferenceAllocation.cancel_below _ _ _ _ _ _ _ valid run

theorem cancel_pending (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (valid : PendingAgrees s) :
    PendingAgrees (cancel s member place principal).state := by
  cases waiting : s.waiting with
  | none => simpa [cancel,waiting] using valid
  | some saved =>
      simp only [cancel,waiting]
      cases run : ReferenceExecution.cancel s.machine member place principal s.nextRequest saved.entry with
      | none => exact valid
      | some next =>
          have only : s.machine.pending = [saved.entry] := by simpa [PendingAgrees,waiting] using valid
          obtain ⟨_,permit,store,_,rfl⟩ := ReferenceExecution.cancel_parts _ _ _ _ _ _ _ run
          simp [PendingAgrees,mark,only,ReferenceExecution.keep]

theorem cancel_values_writes (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) :
    (cancel s member place principal).state.values = s.values ∧
      (cancel s member place principal).state.writes = s.writes ∧
      ((cancel s member place principal).state.waiting = s.waiting ∨ (cancel s member place principal).state.waiting = none) := by
  cases waiting : s.waiting with
  | none => simp [cancel,waiting]
  | some saved =>
      cases run : ReferenceExecution.cancel s.machine member place principal s.nextRequest saved.entry <;>
        simp [cancel,waiting,run,mark]

theorem cancel_success (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (ready : (cancel s member place principal).status = .ready) :
    ∃ saved next, s.waiting = some saved ∧
      ReferenceExecution.cancel s.machine member place principal s.nextRequest saved.entry = some next ∧
      (cancel s member place principal).state = {mark s next (some saved.site) .cancellation 1 with waiting := none} := by
  cases waiting : s.waiting with
  | none => simp [cancel,waiting] at ready
  | some saved =>
      cases run : ReferenceExecution.cancel s.machine member place principal s.nextRequest saved.entry with
      | none => simp [cancel,waiting,run] at ready
      | some next => exact ⟨saved,next,rfl,run,by simp [cancel,waiting,run]⟩

theorem cancel_failure_unchanged (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (failed : (cancel s member place principal).status ≠ .ready) : (cancel s member place principal).state = s := by
  cases waiting : s.waiting with
  | none => simp [cancel,waiting]
  | some saved =>
      cases run : ReferenceExecution.cancel s.machine member place principal s.nextRequest saved.entry with
      | none => simp [cancel,waiting,run]
      | some next => simp [cancel,waiting,run] at failed

#print axioms cancel_projects
#print axioms cancel_allocated
#print axioms cancel_pending
#print axioms cancel_values_writes
#print axioms cancel_success
#print axioms cancel_failure_unchanged
end MirroreaProofFirst.ReferenceSource
