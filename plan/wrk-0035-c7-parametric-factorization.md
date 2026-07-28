# WRK-0035 C7 parametric factorization evidence

This is **LAB** evidence for `working/WRK-0035` after its registration cut.
The source below is a disposable, artifact-local Lean check. Its local types
and functions are not Mir carriers, source rules, elaboration rules, or public
interfaces.

## Retained question

For arbitrary `E`, `S`, `O`, `erase`, and `observe`, does equality of erased
representations imply equality of observations exactly when every represented
image point has one realized observation? The source deliberately expands
pointwise uniqueness as existence plus a uniqueness implication: bare Lean 4.29
in this lane provides neither `exists!` syntax nor `ExistsUnique` without an
import. This is notation/dependency avoidance, not a change to the registered
mathematical question.

The artifact also includes an explicit collision refutation and a `Unit`/`Bool`/
`Bool` countermodel against unique reconstruction over all of the codomain.
Neither result constructs a recovery function, uses choice or quotients, or
licenses omission/desugaring for any Mir source fact.

```lean
universe u v w

namespace C7FactorizationLab

variable {E : Type u} {S : Type v} {O : Type w}

def FiberConstant (erase : E -> S) (observe : E -> O) : Prop :=
  forall x y : E, erase x = erase y -> observe x = observe y

def ObservedAt (erase : E -> S) (observe : E -> O) (s : S) (o : O) : Prop :=
  exists e : E, erase e = s /\ observe e = o

def UniqueObservedOnImage (erase : E -> S) (observe : E -> O) : Prop :=
  forall s : S, (exists e : E, erase e = s) ->
    exists o : O, ObservedAt erase observe s o /\
      forall candidate : O, ObservedAt erase observe s candidate -> candidate = o

def Collision (erase : E -> S) (observe : E -> O) : Prop :=
  exists x y : E, erase x = erase y /\ observe x ≠ observe y

theorem fiberConstant_iff_uniqueObservedOnImage (erase : E -> S) (observe : E -> O) :
    FiberConstant erase observe <-> UniqueObservedOnImage erase observe := by
  constructor
  · intro constant s hasPreimage
    rcases hasPreimage with ⟨e0, h0⟩
    refine ⟨observe e0, ⟨e0, h0, rfl⟩, ?_⟩
    intro candidate candidateObserved
    rcases candidateObserved with ⟨e, he, hObserve⟩
    exact hObserve.symm.trans (constant e e0 (he.trans h0.symm))
  · intro unique x y sameErasure
    rcases unique (erase x) ⟨x, rfl⟩ with ⟨o, _, uniqueObserved⟩
    have xObserved : ObservedAt erase observe (erase x) (observe x) := ⟨x, rfl, rfl⟩
    have yObserved : ObservedAt erase observe (erase x) (observe y) := ⟨y, sameErasure.symm, rfl⟩
    exact (uniqueObserved (observe x) xObserved).trans
      (uniqueObserved (observe y) yObserved).symm

theorem collision_not_fiberConstant (erase : E -> S) (observe : E -> O)
    (collision : Collision erase observe) :
    ¬ FiberConstant erase observe := by
  intro constant
  rcases collision with ⟨x, y, sameErasure, differentObservation⟩
  exact differentObservation (constant x y sameErasure)

theorem collision_not_uniqueObservedOnImage (erase : E -> S) (observe : E -> O)
    (collision : Collision erase observe) :
    ¬ UniqueObservedOnImage erase observe := by
  intro unique
  exact collision_not_fiberConstant erase observe collision
    ((fiberConstant_iff_uniqueObservedOnImage erase observe).mpr unique)

def collisionErase : Bool -> Unit := fun _ => ()

def collisionObserve : Bool -> Bool := fun value => value

theorem collisionModel : Collision collisionErase collisionObserve := by
  refine ⟨false, true, rfl, ?_⟩
  exact Bool.false_ne_true

theorem collisionModelRejectsFiberConstant :
    ¬ FiberConstant collisionErase collisionObserve :=
  collision_not_fiberConstant collisionErase collisionObserve collisionModel

theorem collisionModelRejectsUniqueObservation :
    ¬ UniqueObservedOnImage collisionErase collisionObserve :=
  collision_not_uniqueObservedOnImage collisionErase collisionObserve collisionModel

def fullErase : Unit -> Bool := fun _ => false

def fullObserve : Unit -> Bool := fun _ => false

def FullReconstructs (reconstruct : Bool -> Bool) : Prop :=
  forall e : Unit, reconstruct (fullErase e) = fullObserve e

def UniqueFullReconstructor : Prop :=
  exists reconstruct : Bool -> Bool, FullReconstructs reconstruct /\
    forall candidate : Bool -> Bool, FullReconstructs candidate -> candidate = reconstruct

def alwaysFalse : Bool -> Bool := fun _ => false

def identityBool : Bool -> Bool := fun value => value

theorem fullModelFiberConstant : FiberConstant fullErase fullObserve := by
  intro x y _
  rfl

theorem fullModelUniqueObservedOnImage : UniqueObservedOnImage fullErase fullObserve :=
  (fiberConstant_iff_uniqueObservedOnImage fullErase fullObserve).mp fullModelFiberConstant

theorem alwaysFalseReconstructs : FullReconstructs alwaysFalse := by
  intro e
  cases e
  rfl

theorem identityBoolReconstructs : FullReconstructs identityBool := by
  intro e
  cases e
  rfl

theorem alwaysFalse_ne_identityBool : alwaysFalse ≠ identityBool := by
  intro equalFunctions
  have equalAtTrue : alwaysFalse true = identityBool true := congrFun equalFunctions true
  exact Bool.false_ne_true equalAtTrue

theorem noUniqueFullReconstructor : ¬ UniqueFullReconstructor := by
  intro unique
  rcases unique with ⟨reconstruct, _, uniqueReconstructs⟩
  have falseMatches : alwaysFalse = reconstruct :=
    uniqueReconstructs alwaysFalse alwaysFalseReconstructs
  have identityMatches : identityBool = reconstruct :=
    uniqueReconstructs identityBool identityBoolReconstructs
  exact alwaysFalse_ne_identityBool (falseMatches.trans identityMatches.symm)

#print axioms fiberConstant_iff_uniqueObservedOnImage
#print axioms collision_not_fiberConstant
#print axioms collision_not_uniqueObservedOnImage
#print axioms noUniqueFullReconstructor

end C7FactorizationLab
```
