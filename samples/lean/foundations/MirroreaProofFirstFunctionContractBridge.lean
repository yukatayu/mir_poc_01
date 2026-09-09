import MirroreaProofFirstPureFunctions
import MirroreaProofFirstContractExport
namespace MirroreaProofFirst.FunctionContractBridge
open LocalContract ContractExport

def lower : Term → PureFunctions.Expr
 | .input i => .var i
 | .integer z => .integer z
 | .add a b => .add (lower a) (lower b)
 | .mul a b => .mul (lower a) (lower b)
 | .square a => .mul (lower a) (lower a)

def argumentValues (args : List Int) : List PureFunctions.Value := args.map .integer
def argumentInput (args : List Int) : Nat → Int := fun i => args[i]?.getD 0

theorem lower_typed {n : Nat} {e : Term} (h : Scoped n e) :
 PureFunctions.Typed (List.replicate n .int) (lower e) .int := by
 induction h with
 | @input i hi =>
   apply PureFunctions.Typed.var
   simp [hi]
 | integer z => exact .integer
 | add a b ha hb => exact .add ha hb
 | mul a b ha hb => exact .mul ha hb
 | square a ha => exact .mul ha ha

theorem lower_executes (args : List Int) {e : Term} (h : Scoped args.length e) :
 PureFunctions.Executes (.expression (argumentValues args) (lower e))
 (.integer (eval (argumentInput args) e)) := by
 induction h with
 | @input i hi =>
   apply PureFunctions.Executes.lookupValue
   simp [argumentValues,argumentInput,eval,List.getElem?_eq_getElem hi]
 | integer z => exact .integer
 | add a b ha hb => exact .addition ha hb
 | mul a b ha hb => exact .multiply ha hb
 | square a ha => exact .multiply ha ha

-- A closed reusable pure function, not a resource/effect continuation.
def unary (body : Term) : PureFunctions.Expr := .lambda .int (lower body)
theorem unary_typed {body : Term} (h : Scoped 1 body) :
 PureFunctions.Typed [] (unary body) (.arrow .int .int) := by
 apply PureFunctions.Typed.lambda
 exact lower_typed h

theorem unary_executes (x : Int) {body : Term} (h : Scoped 1 body) :
 PureFunctions.Executes (.expression [] (.app (unary body) (.integer x)))
 (.integer (eval (argumentInput [x]) body)) :=
 .application .lambda .integer (lower_executes [x] h)

theorem accepted_code_executes {b : Binding} {e : Envelope} {n : Nat}
 (h : accept b e = some n) :
 ∃ fuel, PureFunctions.evaluate fuel (argumentValues b.arguments) (lower b.code) = some (.integer e.result) := by
 have common := (accept_sound h).common
 have execution := lower_executes b.arguments common.scope
 have same : eval (argumentInput b.arguments) b.code = e.result := denotes_eval common.result
 rw [same] at execution
 exact (PureFunctions.execution_exact _ _).mpr execution

theorem accepted_unary_executes {b : Binding} {e : Envelope} {n : Nat} (x : Int)
 (h : accept b e = some n) (args : b.arguments = [x]) :
 ∃ fuel, PureFunctions.evaluate fuel [] (.app (unary b.code) (.integer x)) = some (.integer e.result) := by
 have common := (accept_sound h).common
 have sc : Scoped 1 b.code := by simpa [args] using common.scope
 have execution := unary_executes x sc
 have same : eval (argumentInput [x]) b.code = e.result := by
   have original := denotes_eval common.result
   change eval (argumentInput b.arguments) b.code = e.result at original
   rw [args] at original
   exact original
 rw [same] at execution
 exact (PureFunctions.execution_exact _ _).mpr execution

namespace Controls
def body : Term := positiveTerm (.square (.input 0))
example : PureFunctions.infer [] (unary body) = some (.arrow .int .int) := by decide
example : PureFunctions.evaluate 12 [] (.app (unary body) (.integer (-3))) = some (.integer 10) := by rfl
example : PureFunctions.evaluate 12 [] (.app (unary body) (.integer 4)) = some (.integer 17) := by rfl
end Controls
#print axioms lower_typed
#print axioms lower_executes
#print axioms unary_executes
#print axioms accepted_code_executes
#print axioms accepted_unary_executes
end MirroreaProofFirst.FunctionContractBridge
