import MixedReferenceSource
import MirroreaProofFirstReferenceSourceElaboration

namespace MirroreaProofFirst.MixedReferenceSourceElaboration
open ReferenceSourceData MixedReferenceSource

-- Reuse the exact declarative full-value arithmetic and name judgments.
open ReferenceSourceElaboration (InRange Evaluates bounded_exact eval_exact
 reference_not_numeric definition_exact instance_exact reference_exact OptionalName
 optional_exact DependencyNames instance_projection dependencies_exact TargetMeans target_exact)

-- The plan's output sort and COMPLETE command payload are determined by the
-- source constructor. This is separate from executable elaboration/checking.
def PlainFor (values : Values) : SourceAuthoring.Statement → Plan → Prop
  | .register name definition previous,plan => ∃ key,
      OptionalName (fun x k => lookup values x = some (.plain (.definition k))) previous key ∧
      plan = .control name (.register (.pure definition) key) .definition
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

def PlanFor (s : MixedInstanceState.State d p n) (values : Values) : Statement → Plan → Prop
  | .plain statement,plan => PlainFor values statement plan
  | .acquire name decl,plan => ∃ chain, MixedReferenceSourceData.ChainElaborates s values decl chain ∧
      MixedFallbackStatic.Admissible s chain ∧ plan = .acquire name chain
  | .alias name target,plan => ∃ key, lookup values target = some (.reference key) ∧ plan = .pureValue name (.reference key) false
  | .reacquire name target,plan => ∃ key, lookup values target = some (.reference key) ∧ plan = .reacquire name key
  | .release name target,plan => ∃ key, lookup values target = some (.reference key) ∧ plan = .release name key

theorem elaborate_exact (s : MixedInstanceState.State d p n) (values : Values) (statement : Statement) (plan : Plan) :
    elaborate s values statement = some plan ↔ PlanFor s values statement plan := by
  cases statement with
  | plain statement => exact plain_exact _ _ _
  | acquire name decl =>
      simp only [elaborate,Option.bind_eq_bind,Option.bind_eq_some_iff,MixedReferenceSourceData.chain_exact,PlanFor]
      apply exists_congr
      intro chain
      cases checked : MixedFallbackStatic.check s chain with
      | error reason =>
          have bad : ¬ MixedFallbackStatic.Admissible s chain := by
            intro valid; have := (MixedFallbackStatic.check_exact _ _).mpr valid; simp [checked] at this
          simp [bad]
      | ok token =>
          cases token
          simp [eq_comm,(MixedFallbackStatic.check_exact _ _).mp checked]
  | alias name target => simp only [elaborate,Option.bind_eq_bind,Option.bind_eq_some_iff,Option.pure_def,Option.some.injEq,reference_exact,PlanFor]; grind
  | reacquire name target => simp only [elaborate,Option.bind_eq_bind,Option.bind_eq_some_iff,Option.pure_def,Option.some.injEq,reference_exact,PlanFor]; grind
  | release name target => simp only [elaborate,Option.bind_eq_bind,Option.bind_eq_some_iff,Option.pure_def,Option.some.injEq,reference_exact,PlanFor]; grind

-- An existing reference name cannot elaborate to an independently authorized
-- ordinary target, even though raw helper execution can run such a forged plan.
theorem no_reference_coercion (s : MixedInstanceState.State d p n) (values : Values)
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
end MirroreaProofFirst.MixedReferenceSourceElaboration
