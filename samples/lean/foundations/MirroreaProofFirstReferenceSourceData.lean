import MirroreaProofFirstReferenceExecution
import MirroreaProofFirstSourceTyping

namespace MirroreaProofFirst.ReferenceSourceData

-- Private source carrier reuses the existing source arithmetic and plain IR.
-- The reference key names one owner binding; aliases do not duplicate that row.
inductive Value where
  | plain (value : SourceAuthoring.Value)
  | reference (key : Nat)
  deriving DecidableEq, Repr
abbrev Values := List (String × Value)

def lookup (values : Values) (name : String) : Option Value :=
  ((values.find? fun row => row.1 == name)).map Prod.snd

-- Projection is only for reusing legacy INTEGER evaluation and NONREFERENCE
-- type premises. It never supplies an executable machine or runs legacy control.
-- Reference names remain present as nonnumeric sentinels, so they cannot become
-- integer dependencies, instance management targets, or fresh shadowed names.
def plainValue : Value → SourceAuthoring.Value
  | .plain value => value | .reference _ => .unit
def plainValues (values : Values) : List (String × SourceAuthoring.Value) :=
  values.map fun (name,value) => (name,plainValue value)

theorem lookup_plainValues (values : Values) (name : String) :
    SourceAuthoring.lookup (plainValues values) name = (lookup values name).map plainValue := by
  simp [SourceAuthoring.lookup,plainValues,lookup,Function.comp_def,Option.map_map]

theorem reference_not_integer (values : Values) (name : String) (key : Nat)
    (reference : lookup values name = some (.reference key)) :
    SourceAuthoring.eval (plainValues values) (.read name) = none := by
  simp [SourceAuthoring.eval,lookup_plainValues,reference,plainValue]

def definitionKey (values : Values) (name : String) : Option Nat :=
  match lookup values name with | some (.plain (.definition key)) => some key | _ => none
def instanceKey (values : Values) (name : String) : Option Nat :=
  match lookup values name with | some (.plain (.callable key)) => some key | _ => none
def referenceKey (values : Values) (name : String) : Option Nat :=
  match lookup values name with | some (.reference key) => some key | _ => none

theorem reference_not_instance (values : Values) (name : String) (key : Nat)
    (reference : lookup values name = some (.reference key)) :
    instanceKey values name = none ∧ definitionKey values name = none := by
  simp [instanceKey,definitionKey,reference]

structure OptionSyntax where
  name : String
  target : String
  declaredAccess : Option String
  capability : FallbackStatic.Capability
  leaseUntil : Nat
  deriving DecidableEq, Repr

structure ChainSyntax where
  reader : String
  options : List OptionSyntax
  edges : List (Option FallbackStatic.EdgeDecl)
  deriving DecidableEq, Repr

def optionDecl (s : InstanceState.State d p n) (decl : OptionSyntax) (key : Fin n) : FallbackStatic.OptionDecl :=
  ⟨decl.name,key.val,decl.declaredAccess,decl.capability,(s.instances key).interface,decl.leaseUntil⟩

def resolveOption (s : InstanceState.State d p n) (values : Values) (decl : OptionSyntax) :
    Option FallbackStatic.OptionDecl := do
  let raw ← instanceKey values decl.target
  let key ← CompositionCore.index n raw
  return optionDecl s decl key

def OptionElaborates (s : InstanceState.State d p n) (values : Values) (decl : OptionSyntax)
    (option : FallbackStatic.OptionDecl) : Prop :=
  ∃ key : Fin n, instanceKey values decl.target = some key.val ∧ option = optionDecl s decl key

theorem option_exact (s : InstanceState.State d p n) (values : Values) (decl : OptionSyntax)
    (option : FallbackStatic.OptionDecl) : resolveOption s values decl = some option ↔ OptionElaborates s values decl option := by
  constructor
  · intro resolved
    unfold resolveOption at resolved
    cases named : instanceKey values decl.target with
    | none => simp [named] at resolved
    | some raw =>
        simp only [named,Option.bind_eq_bind,Option.bind_some] at resolved
        cases indexed : CompositionCore.index n raw with
        | none => simp [indexed] at resolved
        | some key =>
            have equal := CompositionCore.index_sound _ indexed
            exact ⟨key,by simpa [equal] using named,by simpa [indexed] using resolved.symm⟩
  · rintro ⟨key,named,rfl⟩
    simp [resolveOption,named,CompositionCore.index_roundtrip]

def resolveOptions (s : InstanceState.State d p n) (values : Values) : List OptionSyntax → Option (List FallbackStatic.OptionDecl)
  | [] => some []
  | item :: rest => do return (← resolveOption s values item) :: (← resolveOptions s values rest)

inductive OptionsElaborate (s : InstanceState.State d p n) (values : Values) :
    List OptionSyntax → List FallbackStatic.OptionDecl → Prop where
  | nil : OptionsElaborate s values [] []
  | cons : OptionElaborates s values item option → OptionsElaborate s values rest options →
      OptionsElaborate s values (item :: rest) (option :: options)

theorem options_exact (s : InstanceState.State d p n) (values : Values) (decl : List OptionSyntax)
    (options : List FallbackStatic.OptionDecl) :
    resolveOptions s values decl = some options ↔ OptionsElaborate s values decl options := by
  induction decl generalizing options with
  | nil => constructor <;> intro h <;> cases h <;> constructor
  | cons item rest ih =>
      constructor
      · intro resolved
        unfold resolveOptions at resolved
        cases hr : resolveOption s values item with
        | none => simp [hr] at resolved
        | some option =>
            simp only [hr,Option.bind_eq_bind,Option.bind_some] at resolved
            cases ht : resolveOptions s values rest with
            | none => simp [ht] at resolved
            | some tail =>
                simp only [ht,Option.bind_some,Option.pure_def,Option.some.injEq] at resolved
                subst options
                exact .cons ((option_exact _ _ _ _).mp hr) ((ih _).mp ht)
      · intro elaborated
        cases elaborated with
        | cons head tail => simp [resolveOptions,(option_exact _ _ _ _).mpr head,(ih _).mpr tail]

def resolveChain (s : InstanceState.State d p n) (values : Values) (decl : ChainSyntax) : Option FallbackStatic.Chain := do
  let reader ← instanceKey values decl.reader
  let _ ← CompositionCore.index n reader
  let options ← resolveOptions s values decl.options
  return ⟨reader,options,decl.edges⟩

def ChainElaborates (s : InstanceState.State d p n) (values : Values) (decl : ChainSyntax) (chain : FallbackStatic.Chain) : Prop :=
  (∃ reader : Fin n, instanceKey values decl.reader = some reader.val ∧ chain.reader = reader.val) ∧
  OptionsElaborate s values decl.options chain.options ∧ chain.edges = decl.edges

theorem chain_exact (s : InstanceState.State d p n) (values : Values) (decl : ChainSyntax) (chain : FallbackStatic.Chain) :
    resolveChain s values decl = some chain ↔ ChainElaborates s values decl chain := by
  constructor
  · intro resolved
    unfold resolveChain at resolved
    cases named : instanceKey values decl.reader with
    | none => simp [named] at resolved
    | some raw =>
        simp only [named,Option.bind_eq_bind,Option.bind_some] at resolved
        cases indexed : CompositionCore.index n raw with
        | none => simp [indexed] at resolved
        | some reader =>
            simp only [indexed,Option.bind_some] at resolved
            cases options : resolveOptions s values decl.options with
            | none => simp [options] at resolved
            | some result =>
                simp only [options,Option.bind_some,Option.pure_def,Option.some.injEq] at resolved
                subst chain
                have equal := CompositionCore.index_sound _ indexed
                exact ⟨⟨reader,by simpa [equal] using named,equal.symm⟩,(options_exact _ _ _ _).mp options,rfl⟩
  · rintro ⟨⟨reader,named,equal⟩,options,edges⟩
    have record : chain = ⟨reader.val,chain.options,decl.edges⟩ := by cases chain; simp_all
    simp only [resolveChain,named,Option.bind_eq_bind,Option.bind_some,CompositionCore.index_roundtrip,
      (options_exact _ _ _ _).mpr options,Option.pure_def]
    exact congrArg some record.symm

inductive Statement where
  | plain (statement : SourceAuthoring.Statement)
  | acquire (name : String) (chain : ChainSyntax)
  | alias (name target : String)
  | reacquire (out target : String)
  | release (out target : String)
  deriving DecidableEq, Repr

-- Full source text is the finite reference identity, not an assumed injective
-- hash. The actual Rust adapter supplies the exact decoded source bytes; its
-- parsing/span/serialization correspondence remains an explicit TCB boundary.
structure Site where
  document : String
  byteOffset : Nat
  deriving DecidableEq, Repr

structure Located where
  site : Site
  statement : Statement
  deriving DecidableEq, Repr

inductive Ty where
  | plain (type : SourceTypes.Ty)
  | reference
  deriving DecidableEq, Repr
abbrev Environment := List (String × Ty)

def typeOf : Value → Ty
  | .plain value => .plain (SourceTypes.typeOf value)
  | .reference _ => .reference
def environment (values : Values) : Environment := values.map fun (name,value) => (name,typeOf value)
def lookupType (env : Environment) (name : String) : Option Ty := ((env.find? fun row => row.1 == name)).map Prod.snd
def plainType : Ty → SourceTypes.Ty | .plain type => type | .reference => .unit
def plainEnvironment (env : Environment) : SourceTypes.Environment := env.map fun (name,type) => (name,plainType type)

theorem lookup_environment (values : Values) (name : String) :
    lookupType (environment values) name = (lookup values name).map typeOf := by
  simp [lookupType,environment,lookup,Function.comp_def,Option.map_map]

theorem lookup_plainEnvironment (env : Environment) (name : String) :
    SourceTypes.lookup (plainEnvironment env) name = (lookupType env name).map plainType := by
  simp [SourceTypes.lookup,plainEnvironment,lookupType,Function.comp_def,Option.map_map]

theorem environment_projection (values : Values) :
    SourceTyping.environment (plainValues values) = plainEnvironment (environment values) := by
  simp only [SourceTyping.environment,plainValues,plainEnvironment,environment,List.map_map]
  apply List.map_congr_left
  intro row member
  obtain ⟨name,value⟩ := row
  cases value <;> rfl

def Has (env : Environment) (name : String) (type : Ty) : Prop := lookupType env name = some type
def has (env : Environment) (name : String) (type : Ty) : Bool := decide (lookupType env name = some type)

def Premise (p : Nat) (env : Environment) : Statement → Prop
  | .plain (.invoke _ target expression) =>
      (Has env target (.plain .callable) ∨ Has env target .reference) ∧ SourceTypes.Expression (plainEnvironment env) expression
  | .plain statement => SourceTypes.Premise p (plainEnvironment env) statement
  | .acquire _ chain => Has env chain.reader (.plain .callable) ∧ ∀ option ∈ chain.options, Has env option.target (.plain .callable)
  | .alias _ target | .reacquire _ target | .release _ target => Has env target .reference

def premiseCheck (p : Nat) (env : Environment) : Statement → Bool
  | .plain (.invoke _ target expression) =>
      (has env target (.plain .callable) || has env target .reference) && SourceTypes.expressionCheck (plainEnvironment env) expression
  | .plain statement => SourceTypes.premiseCheck p (plainEnvironment env) statement
  | .acquire _ chain => has env chain.reader (.plain .callable) && chain.options.all (fun option => has env option.target (.plain .callable))
  | .alias _ target | .reacquire _ target | .release _ target => has env target .reference

theorem premise_exact (p : Nat) (env : Environment) (statement : Statement) :
    premiseCheck p env statement = true ↔ Premise p env statement := by
  cases statement with
  | plain statement => cases statement <;> simp [premiseCheck,Premise,has,Has,SourceTypes.premise_exact,SourceTypes.expression_exact]
  | acquire name chain => simp [premiseCheck,Premise,has,Has,List.all_eq_true]
  | alias name target => simp [premiseCheck,Premise,has,Has]
  | reacquire name target => simp [premiseCheck,Premise,has,Has]
  | release name target => simp [premiseCheck,Premise,has,Has]

def output : Statement → Option (String × Ty)
  | .plain statement => (SourceTypes.output statement).map fun (name,type) => (name,.plain type)
  | .acquire name _ | .alias name _ => some (name,.reference)
  | .reacquire name _ | .release name _ => some (name,.plain .unit)

def Produces (env : Environment) (binding : Option (String × Ty)) (next : Environment) : Prop :=
  match binding with | none => next = env | some row => lookupType env row.1 = none ∧ next = row :: env
def bindType (env : Environment) (binding : Option (String × Ty)) : Option Environment :=
  match binding with | none => some env | some row => if (lookupType env row.1).isSome then none else some (row :: env)

theorem bindType_exact (env : Environment) (binding : Option (String × Ty)) (next : Environment) :
    bindType env binding = some next ↔ Produces env binding next := by
  cases binding with
  | none => simp [bindType,Produces,eq_comm]
  | some row => cases found : lookupType env row.1 <;> simp [bindType,Produces,found,eq_comm]

def StatementTyped (p : Nat) (env : Environment) (statement : Statement) (next : Environment) : Prop :=
  Premise p env statement ∧ Produces env (output statement) next
def checkStatement (p : Nat) (env : Environment) (statement : Statement) : Option Environment :=
  if premiseCheck p env statement then bindType env (output statement) else none

theorem statement_exact (p : Nat) (env : Environment) (statement : Statement) (next : Environment) :
    checkStatement p env statement = some next ↔ StatementTyped p env statement next := by
  unfold checkStatement
  split
  · rename_i valid
    simp [StatementTyped,(premise_exact _ _ _).mp valid,bindType_exact]
  · rename_i invalid
    simp [StatementTyped,show ¬ Premise p env statement from fun h => invalid ((premise_exact _ _ _).mpr h)]

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
  | nil => constructor <;> intro h <;> cases h <;> constructor
  | cons item rest ih =>
      constructor
      · intro accepted
        unfold checkProgram at accepted
        cases first : checkStatement p env item.statement with
        | none => simp [first] at accepted
        | some next =>
            exact .cons ((statement_exact _ _ _ _).mp first) ((ih _).mp (by simpa [first] using accepted))
      · intro typed
        cases typed with
        | cons head tail => simp [checkProgram,(statement_exact _ _ _ _).mpr head,(ih _).mpr tail]

#print axioms option_exact
#print axioms lookup_plainValues
#print axioms reference_not_integer
#print axioms reference_not_instance
#print axioms lookup_environment
#print axioms lookup_plainEnvironment
#print axioms environment_projection
#print axioms options_exact
#print axioms chain_exact
#print axioms premise_exact
#print axioms statement_exact
#print axioms program_exact
end MirroreaProofFirst.ReferenceSourceData
