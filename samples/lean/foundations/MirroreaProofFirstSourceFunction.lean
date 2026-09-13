import MirroreaProofFirstSourcePureLowering
import MirroreaProofFirstSourceAuthoring

namespace MirroreaProofFirst.SourceFunction
open InstancePrograms SourceAuthoring
open ContractExport.CheckedArithmetic

-- Named strict source AST. Immutable let evaluation is retained even when the
-- return expression does not use it. This has no effects or cost-equivalence claim.
inductive Body where
  | result (expression : Expr)
  | letValue (name : String) (expression : Expr) (rest : Body)
  deriving DecidableEq, Repr

def evaluateExpr (values : String → Option Int) : Expr → Option Int
  | .integer n => checked Machine.lo Machine.hi n
  | .read name => values name
  | .add left right => do checked Machine.lo Machine.hi ((← evaluateExpr values left)+(← evaluateExpr values right))
  | .mul left right => do checked Machine.lo Machine.hi ((← evaluateExpr values left)*(← evaluateExpr values right))

def compileExpr (terms : String → LocalContract.Term) : Expr → LocalContract.Term
  | .integer n => .integer n
  | .read name => terms name
  | .add left right => .add (compileExpr terms left) (compileExpr terms right)
  | .mul left right => .mul (compileExpr terms left) (compileExpr terms right)

def evaluateBody (values : String → Option Int) : Body → Option Int
  | .result expression => evaluateExpr values expression
  | .letValue name expression rest => do
      let value ← evaluateExpr values expression
      evaluateBody (fun other => if other = name then some value else values other) rest

def compileBody (terms : String → LocalContract.Term) : Body → LocalContract.Term
  | .result expression => compileExpr terms expression
  | .letValue name expression rest =>
      let term := compileExpr terms expression
      SourcePureLowering.sequence term
        (compileBody (fun other => if other = name then term else terms other) rest)

def evaluate (parameter : String) (body : Body) (input : Int) : Option Int :=
  if Machine.lo ≤ input ∧ input ≤ Machine.hi then
    evaluateBody (fun name => if name = parameter then some input else none) body
  else none

-- A missing lexical name is an out-of-scope input, rejected by the independent
-- scope checker and evaluator. It never defaults to a usable integer.
def compile (parameter : String) (body : Body) : LocalContract.Term :=
  compileBody (fun name => if name = parameter then .input 0 else .input 1) body

def Aligned (terms : String → LocalContract.Term) (values : String → Option Int) (input : Int) : Prop :=
  ∀ name, Machine.run (terms name) input = values name

theorem expression_preserves (terms : String → LocalContract.Term) (values : String → Option Int)
    (input : Int) (bounded : Machine.lo ≤ input ∧ input ≤ Machine.hi) (aligned : Aligned terms values input)
    (expression : Expr) : Machine.run (compileExpr terms expression) input = evaluateExpr values expression := by
  induction expression with
  | integer n => simp [compileExpr,evaluateExpr,Machine.run,bounded,ContractExport.CheckedArithmetic.evaluate]
  | read name => exact aligned name
  | add left right ihl ihr =>
      simp only [Machine.run,if_pos bounded] at ihl ihr ⊢
      simp [compileExpr,evaluateExpr,ContractExport.CheckedArithmetic.evaluate,ihl,ihr]
  | mul left right ihl ihr =>
      simp only [Machine.run,if_pos bounded] at ihl ihr ⊢
      simp [compileExpr,evaluateExpr,ContractExport.CheckedArithmetic.evaluate,ihl,ihr]

theorem body_preserves (terms : String → LocalContract.Term) (values : String → Option Int)
    (input : Int) (bounded : Machine.lo ≤ input ∧ input ≤ Machine.hi) (aligned : Aligned terms values input)
    (body : Body) : Machine.run (compileBody terms body) input = evaluateBody values body := by
  induction body generalizing terms values with
  | result expression => exact expression_preserves terms values input bounded aligned expression
  | letValue name expression rest ih =>
      simp only [compileBody,SourcePureLowering.sequence_run,evaluateBody]
      have eval := expression_preserves terms values input bounded aligned expression
      rw [eval]
      cases h : evaluateExpr values expression with
      | none => simp
      | some value =>
          simp only [Option.bind_eq_bind,Option.bind_some]
          apply ih
          intro other
          by_cases key : other = name
          · simp only [if_pos key]; exact eval.trans h
          · simpa only [if_neg key] using aligned other

-- General expression/strict-let substitution correctness, including rejection
-- for unused overflowing initializers, unknown names, and out-of-bounds input.
theorem compile_exact (parameter : String) (body : Body) (input : Int) :
    Machine.run (compile parameter body) input = evaluate parameter body input := by
  by_cases bounded : Machine.lo ≤ input ∧ input ≤ Machine.hi
  · unfold compile evaluate
    rw [if_pos bounded]
    apply body_preserves _ _ input bounded
    intro name
    by_cases key : name = parameter
    · simp [key,Machine.run,bounded,ContractExport.CheckedArithmetic.evaluate,checked]
    · simp [key,Machine.run,bounded,ContractExport.CheckedArithmetic.evaluate]
  · simp [Machine.run,evaluate,bounded]

namespace Controls
def original : Body := .letValue "square" (.mul (.read "x") (.read "x"))
  (.result (.add (.read "square") (.integer 1)))
#guard Machine.run (compile "x" original) 2 = some 5
#guard Machine.run (compile "x" (.letValue "unused" (.mul (.integer Machine.hi) (.integer 2))
  (.result (.integer 1)))) 0 = none
#guard Machine.run (compile "x" (.letValue "unused" (.read "missing") (.result (.integer 1)))) 0 = none
#guard Machine.run (compile "x" (.result (.integer 1))) (Machine.hi+1) = none
#guard Machine.run (compile "x" (.letValue "x" (.integer 3) (.result (.read "x")))) 0 = some 3
end Controls
#print axioms expression_preserves
#print axioms body_preserves
#print axioms compile_exact
end MirroreaProofFirst.SourceFunction
