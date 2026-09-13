import MirroreaProofFirstCompositionMachine

namespace MirroreaProofFirst.SourceAuthoring
open InstancePrograms CompositionCore CompositionMachine

-- Source-local names and values. The parser adapter exports these nodes from
-- actual ASTs; it cannot insert a World, Ticket, Evidence, request id or result.
inductive Value where
  | unit
  | definition (key : Nat)
  | callable (key : Nat)
  | integer (value : Int) (mutable : Bool)
  deriving DecidableEq, Repr

inductive Expr where
  | integer (value : Int)
  | read (name : String)
  | add (left right : Expr)
  | mul (left right : Expr)
  deriving DecidableEq, Repr

inductive Statement where
  | register (name : String) (definition : Definition) (predecessor : Option String)
  | instantiate (name definition : String) (places : List Nat) (parent : Option String)
      (dependencies : Support.Formula String)
  | retire (out name : String)
  | reparent (out name : String) (parent : Option String)
  | replace (out name definition : String)
  | leave (out : String) (place : Nat)
  | join (out : String) (place : Nat)
  | localValue (name : String) (mutable : Bool) (value : Expr)
  | assign (name : String) (value : Expr)
  | invoke (name target : String) (argument : Expr)
  deriving DecidableEq, Repr

structure Located where
  startByte : Nat
  statement : Statement
  deriving DecidableEq, Repr

structure State (p a : Nat) where
  machine : Machine p a
  values : List (String × Value)
  nextRequest : Nat
  -- Origin indexes are attached only to successful actual interpreter steps.
  origins : List (Nat × Nat)

def lookup (values : List (String × Value)) (name : String) : Option Value :=
  ((values.find? fun row => row.1 == name)).map Prod.snd

def definitionKey (values : List (String × Value)) (name : String) : Option Nat :=
  match lookup values name with | some (.definition key) => some key | _ => none
def instanceKey (values : List (String × Value)) (name : String) : Option Nat :=
  match lookup values name with | some (.callable key) => some key | _ => none

def optional (f : String → Option Nat) : Option String → Option (Option Nat)
  | none => some none
  | some name => (f name).map some
def dependencies (values : List (String × Value)) : Support.Formula String → Option (Support.Formula Nat)
  | .top => some .top
  | .bottom => some .bottom
  | .ref name => (instanceKey values name).map Support.Formula.ref
  | .both a b => do return .both (← dependencies values a) (← dependencies values b)
  | .either a b => do return .either (← dependencies values a) (← dependencies values b)

def bounded (value : Int) : Option Int :=
  if InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi then some value else none

def eval (values : List (String × Value)) : Expr → Option Int
  | .integer value => bounded value
  | .read name => match lookup values name with | some (.integer value _) => bounded value | _ => none
  | .add a b => do bounded ((← eval values a)+(← eval values b))
  | .mul a b => do bounded ((← eval values a)*(← eval values b))

def bind (s : State p a) (name : String) (value : Value) : Option (State p a) :=
  if (lookup s.values name).isSome then none else some {s with values := (name,value)::s.values}

-- Actor/locus come from the typed transition entry. Every actual operation is
-- resolved against the current value environment and the same dynamic machine.
def control (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (raw : Raw) :
    Option (State p a × Option Nat) := do
  let (machine,created) ← manage s.machine member place principal s.nextRequest raw
  return ({s with machine := machine,nextRequest := s.nextRequest+1},created)

-- Update the binding resolved by lookup. Even an untrusted duplicate-tail
-- environment cannot cause one assignment to mutate several hidden bindings.
def updateValue (values : List (String × Value)) (name : String) (value : Value) : List (String × Value) :=
  match values with
  | [] => []
  | row :: rest => if row.1 = name then (name,value) :: rest else row :: updateValue rest name value

def step (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) : Statement → Option (State p a)
  | .register name definition predecessor => do
      let previous ← optional (definitionKey s.values) predecessor
      let (next,created) ← control s member place principal (.register definition previous)
      bind next name (.definition (← created))
  | .instantiate name definition places parent support => do
      let key ← definitionKey s.values definition
      let parent ← optional (instanceKey s.values) parent
      let support ← dependencies s.values support
      let (next,created) ← control s member place principal (.instantiate key principal places parent support)
      bind next name (.callable (← created))
  | .retire out name => do
      let key ← instanceKey s.values name
      bind (← control s member place principal (.retire key)).1 out .unit
  | .reparent out name parent => do
      let key ← instanceKey s.values name
      let target ← optional (instanceKey s.values) parent
      bind (← control s member place principal (.reparent key target)).1 out .unit
  | .replace out name definition => do
      let key ← instanceKey s.values name
      let code ← definitionKey s.values definition
      bind (← control s member place principal (.replace key code)).1 out .unit
  | .leave out locus => do bind (← control s member place principal (.leave locus)).1 out .unit
  | .join out locus => do bind (← control s member place principal (.join locus)).1 out .unit
  | .localValue name mutable expression => do bind s name (.integer (← eval s.values expression) mutable)
  | .assign name expression => do
      let .integer _ true ← lookup s.values name | none
      let value ← eval s.values expression
      return {s with values := updateValue s.values name (.integer value true)}
  | .invoke name target argument => do
      let key ← instanceKey s.values target
      let arg ← eval s.values argument
      let (pending,ticket) ← CompositionMachine.start s.machine member place principal s.nextRequest key arg
      let (completed,value) ← CompositionMachine.resume pending ticket
      bind {s with machine := completed,nextRequest := s.nextRequest+1} name (.integer value false)

def execute (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) (item : Located) : Option (State p a) := do
  let next ← step s member place principal item.statement
  return {next with origins := (item.startByte,next.machine.events.length)::next.origins}

def run (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat) : List Located → Option (State p a)
  | [] => some s
  | item :: rest => do run (← execute s member place principal item) member place principal rest

-- This is a nonproduction source reference awaiting the static typing and
-- trace refinement proofs. Its imports have independent general safety proofs;
-- that alone does not prove this name/type adapter correct.
end MirroreaProofFirst.SourceAuthoring
