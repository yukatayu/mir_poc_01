# WRK-0036 C7 cumulative-erasure countermodel evidence

This is **LAB** evidence for `working/WRK-0036` after its registration cut.
The fenced source is a disposable, artifact-local Lean check. Its local types,
constructors, functions, and theorem names are not Mir carriers, source rules,
elaboration rules, observation primitives, or public interfaces.

## Retained question

Can individual factorization checks be composed merely because each one is
valid against a more explicit representation? The fixed finite model below
answers no. `eraseA` and `eraseB` each preserve their own Boolean observation.
Both map through explicit coarsening functions to `eraseAB`, but `eraseAB`
collides on the paired observation. Thus the final common representation must
be checked directly.

The model intentionally contains no actual source transformation, omitted
fact, ground/provenance relation, elaborated artifact, semantic dependency
graph, reconstruction function, quotient, or choice. It does not prove a
general composition law or state that any Mir omission is valid or invalid.

```lean
namespace C7CumulativeLab

inductive Explicit where
  | left
  | right

inductive KeepA where
  | left
  | right

inductive KeepB where
  | left
  | right

def eraseA : Explicit -> KeepA := fun e =>
  match e with
  | .left => .left
  | .right => .right

def eraseB : Explicit -> KeepB := fun e =>
  match e with
  | .left => .left
  | .right => .right

def eraseAB : Explicit -> Unit := fun _ => ()

def coarsenA : KeepA -> Unit := fun _ => ()

def coarsenB : KeepB -> Unit := fun _ => ()

def observeA : Explicit -> Bool := fun e =>
  match e with
  | .left => false
  | .right => true

def observeB : Explicit -> Bool := fun e =>
  match e with
  | .left => false
  | .right => true

def pairedObserve : Explicit -> Bool × Bool := fun e =>
  (observeA e, observeB e)

def FiberConstant {E S O : Type} (erase : E -> S) (observe : E -> O) : Prop :=
  forall x y : E, erase x = erase y -> observe x = observe y

def Collision {E S O : Type} (erase : E -> S) (observe : E -> O) : Prop :=
  exists x y : E, erase x = erase y /\ observe x ≠ observe y

theorem commonCoarseningA :
    forall e : Explicit, eraseAB e = coarsenA (eraseA e) := by
  intro e
  cases e <;> rfl

theorem commonCoarseningB :
    forall e : Explicit, eraseAB e = coarsenB (eraseB e) := by
  intro e
  cases e <;> rfl

theorem individualA : FiberConstant eraseA observeA := by
  intro x y sameErasure
  cases x <;> cases y <;> cases sameErasure <;> rfl

theorem individualB : FiberConstant eraseB observeB := by
  intro x y sameErasure
  cases x <;> cases y <;> cases sameErasure <;> rfl

theorem cumulativeCollision : Collision eraseAB pairedObserve := by
  refine ⟨Explicit.left, Explicit.right, rfl, ?_⟩
  intro equalPair
  have firstEqual : false = true :=
    congrArg (fun pair : Bool × Bool => pair.1) equalPair
  exact Bool.false_ne_true firstEqual

theorem collisionNotFiberConstant {E S O : Type} (erase : E -> S) (observe : E -> O)
    (collision : Collision erase observe) :
    ¬ FiberConstant erase observe := by
  intro constant
  rcases collision with ⟨x, y, sameErasure, differentObservation⟩
  exact differentObservation (constant x y sameErasure)

theorem cumulativeNotFiberConstant : ¬ FiberConstant eraseAB pairedObserve :=
  collisionNotFiberConstant eraseAB pairedObserve cumulativeCollision

#print axioms commonCoarseningA
#print axioms commonCoarseningB
#print axioms individualA
#print axioms individualB
#print axioms cumulativeCollision
#print axioms cumulativeNotFiberConstant

end C7CumulativeLab
```
