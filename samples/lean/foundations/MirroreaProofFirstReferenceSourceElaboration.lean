import MirroreaProofFirstReferenceSource

namespace MirroreaProofFirst.ReferenceSourceElaboration
open ReferenceSourceData ReferenceSource

-- Declarative arithmetic over the FULL values. No reference/Unit erasure is
-- present in this judgment, and every intermediate result must fit Int64.
def InRange (value : Int) : Prop :=
  InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi
def Evaluates (values : Values) : SourceAuthoring.Expr → Int → Prop
  | .integer literal,result => result = literal ∧ InRange literal
  | .read name,result => (∃ mutable, lookup values name = some (.plain (.integer result mutable))) ∧ InRange result
  | .add left right,result => ∃ x y, Evaluates values left x ∧ Evaluates values right y ∧ result = x+y ∧ InRange result
  | .mul left right,result => ∃ x y, Evaluates values left x ∧ Evaluates values right y ∧ result = x*y ∧ InRange result

theorem bounded_exact (value result : Int) :
    SourceAuthoring.bounded value = some result ↔ result = value ∧ InRange value := by
  unfold SourceAuthoring.bounded
  split <;> simp_all [InRange,eq_comm] <;> grind

theorem eval_exact (values : Values) (expression : SourceAuthoring.Expr) (result : Int) :
    SourceAuthoring.eval (plainValues values) expression = some result ↔ Evaluates values expression result := by
  induction expression generalizing result with
  | integer value => exact bounded_exact _ _
  | read name =>
      simp only [SourceAuthoring.eval,lookup_plainValues,Evaluates]
      cases found : lookup values name with
      | none => simp
      | some value =>
          cases value with
          | reference key => simp [plainValue]
          | plain value =>
              cases value <;> simp [plainValue,bounded_exact] <;> grind
  | add left right ihl ihr =>
      simp only [SourceAuthoring.eval,Option.bind_eq_bind,Option.bind_eq_some_iff,bounded_exact,ihl,ihr,Evaluates]
      grind
  | mul left right ihl ihr =>
      simp only [SourceAuthoring.eval,Option.bind_eq_bind,Option.bind_eq_some_iff,bounded_exact,ihl,ihr,Evaluates]
      grind

theorem reference_not_numeric (values : Values) (name : String) (key : Nat)
    (found : lookup values name = some (.reference key)) (result : Int) :
    ¬ Evaluates values (.read name) result := by simp [Evaluates,found]

theorem definition_exact (values : Values) (name : String) (key : Nat) :
    definitionKey values name = some key ↔ lookup values name = some (.plain (.definition key)) := by
  unfold definitionKey
  cases lookup values name with
  | none => simp
  | some value => cases value with
    | reference key => simp
    | plain value => cases value <;> simp

theorem instance_exact (values : Values) (name : String) (key : Nat) :
    instanceKey values name = some key ↔ lookup values name = some (.plain (.callable key)) := by
  unfold instanceKey
  cases lookup values name with
  | none => simp
  | some value => cases value with
    | reference key => simp
    | plain value => cases value <;> simp

theorem reference_exact (values : Values) (name : String) (key : Nat) :
    referenceKey values name = some key ↔ lookup values name = some (.reference key) := by
  unfold referenceKey
  cases lookup values name with
  | none => simp
  | some value => cases value <;> simp

def OptionalName (meaning : String → Nat → Prop) : Option String → Option Nat → Prop
  | none,key => key = none
  | some name,key => ∃ raw, meaning name raw ∧ key = some raw

theorem optional_exact (resolve : String → Option Nat) (meaning : String → Nat → Prop)
    (exact : ∀ name key, resolve name = some key ↔ meaning name key)
    (name : Option String) (key : Option Nat) :
    SourceAuthoring.optional resolve name = some key ↔ OptionalName meaning name key := by
  cases name <;> simp [SourceAuthoring.optional,OptionalName,exact,eq_comm]

def DependencyNames (values : Values) : Support.Formula String → Support.Formula Nat → Prop
  | .top,result => result = .top
  | .bottom,result => result = .bottom
  | .ref name,result => ∃ key, lookup values name = some (.plain (.callable key)) ∧ result = .ref key
  | .both left right,result => ∃ a b, DependencyNames values left a ∧ DependencyNames values right b ∧ result = .both a b
  | .either left right,result => ∃ a b, DependencyNames values left a ∧ DependencyNames values right b ∧ result = .either a b

theorem instance_projection (values : Values) (name : String) :
    SourceAuthoring.instanceKey (plainValues values) name = instanceKey values name := by
  simp only [SourceAuthoring.instanceKey,lookup_plainValues,instanceKey]
  cases lookup values name with
  | none => rfl
  | some value => cases value with
    | reference key => rfl
    | plain value => cases value <;> rfl

theorem dependencies_exact (values : Values) (names : Support.Formula String) (result : Support.Formula Nat) :
    SourceAuthoring.dependencies (plainValues values) names = some result ↔ DependencyNames values names result := by
  induction names generalizing result with
  | top => simp [SourceAuthoring.dependencies,DependencyNames,eq_comm]
  | bottom => simp [SourceAuthoring.dependencies,DependencyNames,eq_comm]
  | ref name =>
      simp only [SourceAuthoring.dependencies,instance_projection,Option.map_eq_some_iff,instance_exact,DependencyNames]
      grind
  | both left right ihl ihr =>
      simp only [SourceAuthoring.dependencies,Option.bind_eq_bind,Option.bind_eq_some_iff,
        Option.pure_def,Option.some.injEq,ihl,ihr,DependencyNames]
      grind
  | either left right ihl ihr =>
      simp only [SourceAuthoring.dependencies,Option.bind_eq_bind,Option.bind_eq_some_iff,
        Option.pure_def,Option.some.injEq,ihl,ihr,DependencyNames]
      grind

def TargetMeans (values : Values) (name : String) : Target → Prop
  | .instance key => lookup values name = some (.plain (.callable key))
  | .reference key => lookup values name = some (.reference key)

theorem target_exact (values : Values) (name : String) (target : Target) :
    (match lookup values name with
    | some (.plain (.callable key)) => some (Target.instance key)
    | some (.reference key) => some (Target.reference key)
    | _ => none) = some target ↔ TargetMeans values name target := by
  cases target <;> cases found : lookup values name with
  | none => simp [TargetMeans,found]
  | some value => cases value with
    | reference key => simp [TargetMeans,found]
    | plain value => cases value <;> simp [TargetMeans,found]

-- The plan's output sort and COMPLETE command payload are determined by the
-- source constructor. This is separate from executable elaboration/checking.
def PlainFor (values : Values) : SourceAuthoring.Statement → Plan → Prop
  | .register name definition previous,plan => ∃ key,
      OptionalName (fun x k => lookup values x = some (.plain (.definition k))) previous key ∧
      plan = .control name (.register definition key) .definition
  | .instantiate name definition places parent support,plan => ∃ key ancestor formula,
      lookup values definition = some (.plain (.definition key)) ∧
      OptionalName (fun x k => lookup values x = some (.plain (.callable k))) parent ancestor ∧
      DependencyNames values support formula ∧
      plan = .control name (.instantiate key 0 places ancestor formula) .instance
  | .retire out name,plan => ∃ key, lookup values name = some (.plain (.callable key)) ∧ plan = .control out (.retire key) .unit
  | .reparent out name parent,plan => ∃ key ancestor, lookup values name = some (.plain (.callable key)) ∧
      OptionalName (fun x k => lookup values x = some (.plain (.callable k))) parent ancestor ∧
      plan = .control out (.reparent key ancestor) .unit
  | .replace out name definition,plan => ∃ key code, lookup values name = some (.plain (.callable key)) ∧
      lookup values definition = some (.plain (.definition code)) ∧ plan = .control out (.replace key code) .unit
  | .leave out place,plan => plan = .control out (.leave place) .unit
  | .join out place,plan => plan = .control out (.join place) .unit
  | .localValue name mutable expression,plan => ∃ value, Evaluates values expression value ∧
      plan = .pureValue name (.plain (.integer value mutable)) false
  | .assign name expression,plan => ∃ value, Evaluates values expression value ∧
      plan = .pureValue name (.plain (.integer value true)) true
  | .invoke name target expression,plan => ∃ argument resolved, Evaluates values expression argument ∧
      TargetMeans values target resolved ∧ plan = .call name resolved argument

theorem plain_exact (values : Values) (statement : SourceAuthoring.Statement) (plan : Plan) :
    elaboratePlain values statement = some plan ↔ PlainFor values statement plan := by
  cases statement with
  | invoke name target expression =>
      cases found : lookup values target with
      | none => simp [elaboratePlain,found,PlainFor,TargetMeans]; grind
      | some value =>
          cases value with
          | reference key =>
              simp [elaboratePlain,found,Option.bind_eq_some_iff,PlainFor,TargetMeans,eval_exact]
              grind
          | plain value =>
              cases value <;> simp [elaboratePlain,found,Option.bind_eq_some_iff,PlainFor,TargetMeans,eval_exact] <;> grind
  | _ => simp only [elaboratePlain,Option.bind_eq_bind,Option.bind_eq_some_iff,Option.pure_def,
      Option.some.injEq,definition_exact,instance_exact,dependencies_exact,eval_exact,
      optional_exact _ _ (definition_exact values),optional_exact _ _ (instance_exact values),
      PlainFor] <;> grind

def PlanFor (s : InstanceState.State d p n) (values : Values) : Statement → Plan → Prop
  | .plain statement,plan => PlainFor values statement plan
  | .acquire name decl,plan => ∃ chain, ChainElaborates s values decl chain ∧
      FallbackStatic.Admissible s chain ∧ plan = .acquire name chain
  | .alias name target,plan => ∃ key, lookup values target = some (.reference key) ∧ plan = .pureValue name (.reference key) false
  | .reacquire name target,plan => ∃ key, lookup values target = some (.reference key) ∧ plan = .reacquire name key
  | .release name target,plan => ∃ key, lookup values target = some (.reference key) ∧ plan = .release name key

theorem elaborate_exact (s : InstanceState.State d p n) (values : Values) (statement : Statement) (plan : Plan) :
    elaborate s values statement = some plan ↔ PlanFor s values statement plan := by
  cases statement with
  | plain statement => exact plain_exact _ _ _
  | acquire name decl =>
      simp only [elaborate,Option.bind_eq_bind,Option.bind_eq_some_iff,chain_exact,PlanFor]
      apply exists_congr
      intro chain
      cases checked : FallbackStatic.check s chain with
      | error reason =>
          have bad : ¬ FallbackStatic.Admissible s chain := by
            intro valid; have := (FallbackStatic.check_exact _ _).mpr valid; simp [checked] at this
          simp [bad]
      | ok token =>
          cases token
          simp [eq_comm,(FallbackStatic.check_exact _ _).mp checked]
  | alias name target => simp only [elaborate,Option.bind_eq_bind,Option.bind_eq_some_iff,Option.pure_def,Option.some.injEq,reference_exact,PlanFor]; grind
  | reacquire name target => simp only [elaborate,Option.bind_eq_bind,Option.bind_eq_some_iff,Option.pure_def,Option.some.injEq,reference_exact,PlanFor]; grind
  | release name target => simp only [elaborate,Option.bind_eq_bind,Option.bind_eq_some_iff,Option.pure_def,Option.some.injEq,reference_exact,PlanFor]; grind

-- An existing reference name cannot elaborate to an independently authorized
-- ordinary target, even though raw helper execution can run such a forged plan.
theorem no_reference_coercion (s : InstanceState.State d p n) (values : Values)
    (out name : String) (expression : SourceAuthoring.Expr) (binding target : Nat) (argument : Int)
    (reference : lookup values name = some (.reference binding)) :
    elaborate s values (.plain (.invoke out name expression)) = some (.call out (.instance target) argument) → False := by
  intro compiled
  have meaning := (elaborate_exact _ _ _ _).mp compiled
  obtain ⟨arg,resolved,_,targetMeaning,equal⟩ := meaning
  cases equal
  simp [TargetMeans,reference] at targetMeaning

#print axioms eval_exact
#print axioms dependencies_exact
#print axioms plain_exact
#print axioms elaborate_exact
#print axioms no_reference_coercion
end MirroreaProofFirst.ReferenceSourceElaboration
