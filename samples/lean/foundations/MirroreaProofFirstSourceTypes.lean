import MirroreaProofFirstSourceAuthoring

namespace MirroreaProofFirst.SourceTypes
open SourceAuthoring

inductive Ty where
  | unit | definition | callable | integer (mutable : Bool)
  deriving DecidableEq, Repr
abbrev Environment := List (String × Ty)

def typeOf : Value → Ty
  | .unit => .unit
  | .definition _ => .definition
  | .callable _ => .callable
  | .integer _ mutable => .integer mutable

def lookup (env : Environment) (name : String) : Option Ty :=
  ((env.find? fun row => row.1 == name)).map Prod.snd

def Has (env : Environment) (name : String) (type : Ty) : Prop := lookup env name = some type
def has (env : Environment) (name : String) (type : Ty) : Bool := decide (lookup env name = some type)

-- Static declaration does not assert current authority, current liveness or a
-- successful call. Instance calls retain the explicit runtime rejection path.
def Expression (env : Environment) : Expr → Prop
  | .integer value => InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi
  | .read name => ∃ mutable, Has env name (.integer mutable)
  | .add a b | .mul a b => Expression env a ∧ Expression env b

def expressionCheck (env : Environment) : Expr → Bool
  | .integer value => decide (InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi)
  | .read name => match lookup env name with | some (.integer _) => true | _ => false
  | .add a b | .mul a b => expressionCheck env a && expressionCheck env b

theorem expression_exact (env : Environment) (e : Expr) :
    expressionCheck env e = true ↔ Expression env e := by
  induction e with
  | integer value => simp [expressionCheck,Expression]
  | read name =>
      simp only [expressionCheck,Expression,Has]
      cases h : lookup env name with
      | none => simp
      | some type => cases type <;> simp
  | add a b ia ib => simp [expressionCheck,Expression,ia,ib]
  | mul a b ia ib => simp [expressionCheck,Expression,ia,ib]

def Optional (env : Environment) (type : Ty) : Option String → Prop
  | none => True
  | some name => Has env name type
def optionalCheck (env : Environment) (type : Ty) : Option String → Bool
  | none => true
  | some name => has env name type
theorem optional_exact (env : Environment) (type : Ty) (name : Option String) :
    optionalCheck env type name = true ↔ Optional env type name := by cases name <;> simp [optionalCheck,Optional,has,Has]

def References (env : Environment) : Support.Formula String → Prop
  | .top | .bottom => True
  | .ref name => Has env name .callable
  | .both a b | .either a b => References env a ∧ References env b
def referencesCheck (env : Environment) : Support.Formula String → Bool
  | .top | .bottom => true
  | .ref name => has env name .callable
  | .both a b | .either a b => referencesCheck env a && referencesCheck env b
theorem references_exact (env : Environment) (f : Support.Formula String) :
    referencesCheck env f = true ↔ References env f := by
  induction f <;> simp_all [referencesCheck,References,has,Has]

def Premise (p : Nat) (env : Environment) : Statement → Prop
  | .register _ definition predecessor => InstancePrograms.Satisfies definition ∧ Optional env .definition predecessor
  | .instantiate _ definition places parent support => Has env definition .definition ∧
      (∀ place ∈ places,place < p) ∧ places ≠ [] ∧ Optional env .callable parent ∧ References env support
  | .retire _ name => Has env name .callable
  | .reparent _ name parent => Has env name .callable ∧ Optional env .callable parent
  | .replace _ name definition => Has env name .callable ∧ Has env definition .definition
  | .leave _ place | .join _ place => place < p
  | .localValue _ _ value => Expression env value
  | .assign name value => Has env name (.integer true) ∧ Expression env value
  | .invoke _ name value => Has env name .callable ∧ Expression env value

def premiseCheck (p : Nat) (env : Environment) : Statement → Bool
  | .register _ definition predecessor => InstancePrograms.check definition && optionalCheck env .definition predecessor
  | .instantiate _ definition places parent support => has env definition .definition &&
      places.all (fun place => decide (place < p)) && !places.isEmpty &&
      optionalCheck env .callable parent && referencesCheck env support
  | .retire _ name => has env name .callable
  | .reparent _ name parent => has env name .callable && optionalCheck env .callable parent
  | .replace _ name definition => has env name .callable && has env definition .definition
  | .leave _ place | .join _ place => decide (place < p)
  | .localValue _ _ value => expressionCheck env value
  | .assign name value => has env name (.integer true) && expressionCheck env value
  | .invoke _ name value => has env name .callable && expressionCheck env value

theorem premise_exact (p : Nat) (env : Environment) (s : Statement) :
    premiseCheck p env s = true ↔ Premise p env s := by
  cases s <;> simp [premiseCheck,Premise,InstancePrograms.check_exact,optional_exact,expression_exact,
    references_exact,has,Has,List.all_eq_true,and_assoc]

def output : Statement → Option (String × Ty)
  | .register name .. => some (name,.definition)
  | .instantiate name .. => some (name,.callable)
  | .localValue name mutable _ => some (name,.integer mutable)
  | .invoke name .. => some (name,.integer false)
  | .assign .. => none
  | .retire out _ | .reparent out .. | .replace out .. | .leave out _ | .join out _ => some (out,.unit)

def Produces (env : Environment) (binding : Option (String × Ty)) (next : Environment) : Prop :=
  match binding with
  | none => next = env
  | some row => lookup env row.1 = none ∧ next = row :: env

def bindType (env : Environment) (binding : Option (String × Ty)) : Option Environment :=
  match binding with
  | none => some env
  | some row => if (lookup env row.1).isSome then none else some (row :: env)

theorem bindType_exact (env : Environment) (binding : Option (String × Ty)) (next : Environment) :
    bindType env binding = some next ↔ Produces env binding next := by
  cases binding with
  | none => simp [bindType,Produces,eq_comm]
  | some row => cases h : lookup env row.1 <;> simp [bindType,Produces,h,eq_comm]

def StatementTyped (p : Nat) (env : Environment) (s : Statement) (next : Environment) : Prop :=
  Premise p env s ∧ Produces env (output s) next

def checkStatement (p : Nat) (env : Environment) (s : Statement) : Option Environment :=
  if premiseCheck p env s then bindType env (output s) else none

theorem statement_exact (p : Nat) (env : Environment) (s : Statement) (next : Environment) :
    checkStatement p env s = some next ↔ StatementTyped p env s next := by
  unfold checkStatement
  split
  · rename_i checked
    simp [StatementTyped,(premise_exact p env s).mp checked,bindType_exact]
  · rename_i failed
    simp [StatementTyped,show ¬ Premise p env s from fun h => failed ((premise_exact p env s).mpr h)]

inductive ProgramTyped (p : Nat) : Environment → List Located → Environment → Prop where
  | nil : ProgramTyped p env [] env
  | cons : StatementTyped p env item.statement next → ProgramTyped p next rest finalEnv →
      ProgramTyped p env (item :: rest) finalEnv

def checkProgram (p : Nat) (env : Environment) : List Located → Option Environment
  | [] => some env
  | item :: rest => do checkProgram p (← checkStatement p env item.statement) rest

theorem program_exact (p : Nat) (env : Environment) (program : List Located) (finalEnv : Environment) :
    checkProgram p env program = some finalEnv ↔ ProgramTyped p env program finalEnv := by
  induction program generalizing env with
  | nil =>
      constructor
      · intro h; cases h; exact .nil
      · intro h; cases h; rfl
  | cons item rest ih =>
      constructor
      · intro h
        unfold checkProgram at h
        cases hs : checkStatement p env item.statement with
        | none => simp [hs] at h
        | some next =>
            exact .cons ((statement_exact _ _ _ _).mp hs) ((ih next).mp (by simpa [hs] using h))
      · intro typed
        cases typed with
        | cons hs restTyped =>
            simp [checkProgram,(statement_exact _ _ _ _).mpr hs,(ih _).mpr restTyped]

namespace Controls
def env : Environment := [("code",.definition),("base",.callable),("count",.integer false),("changing",.integer true)]
#guard (checkStatement 3 env (.invoke "out" "base" (.read "count"))).isSome
#guard (checkStatement 3 env (.invoke "out" "code" (.read "count"))).isNone
#guard (checkStatement 3 env (.invoke "out" "base" (.read "missing"))).isNone
#guard (checkStatement 3 env (.invoke "count" "base" (.read "count"))).isNone
#guard (checkStatement 3 env (.assign "count" (.integer 3))).isNone
#guard (checkStatement 3 env (.assign "changing" (.integer 3))).isSome
#guard (checkStatement 3 env (.instantiate "new" "code" [0,2] none .top)).isSome
#guard (checkStatement 3 env (.instantiate "new" "base" [0,2] none .top)).isNone
#guard (checkStatement 3 env (.instantiate "new" "code" [0,3] none .top)).isNone
#guard (checkStatement 3 env (.instantiate "new" "code" [] none .top)).isNone
#guard (checkStatement 3 env (.instantiate "new" "code" [0] none (.either (.ref "missing") .top))).isNone
#guard !expressionCheck env (.integer (InstancePrograms.Machine.hi+1))
end Controls

#print axioms expression_exact
#print axioms premise_exact
#print axioms statement_exact
#print axioms program_exact
end MirroreaProofFirst.SourceTypes
