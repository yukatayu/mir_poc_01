/-!
current-l2 first proof-skeleton fragment

This file captures the structural part of the theorem-side bridge: review units, emitted
Lean stubs, and the invariant that emission preserves subject and obligation identity.
-/

namespace CurrentL2

inductive ObligationKind where
  | rollbackCutNonInterference
  | noRePromotion
deriving DecidableEq, Repr

open ObligationKind

def obligationName : ObligationKind → String
  | rollbackCutNonInterference => "rollback_cut_non_interference"
  | noRePromotion => "no_re_promotion"

structure ReviewUnit where
  subjectRef : String
  obligationKind : ObligationKind
deriving Repr

structure LeanStub where
  subjectRef : String
  obligationKind : ObligationKind
  theoremName : String
deriving Repr

def mkLeanStub (unit : ReviewUnit) : LeanStub :=
  {
    subjectRef := unit.subjectRef
    obligationKind := unit.obligationKind
    theoremName := unit.subjectRef ++ "__" ++ obligationName unit.obligationKind
  }

def emitStubs (units : List ReviewUnit) : List LeanStub :=
  units.map mkLeanStub

theorem mkLeanStub_preserves_subject (unit : ReviewUnit) :
    (mkLeanStub unit).subjectRef = unit.subjectRef := by
  simp [mkLeanStub]

theorem mkLeanStub_preserves_obligation (unit : ReviewUnit) :
    (mkLeanStub unit).obligationKind = unit.obligationKind := by
  simp [mkLeanStub]

theorem mkLeanStub_names_theorem (unit : ReviewUnit) :
    (mkLeanStub unit).theoremName = unit.subjectRef ++ "__" ++ obligationName unit.obligationKind := by
  simp [mkLeanStub]

theorem emitStubs_length (units : List ReviewUnit) :
    (emitStubs units).length = units.length := by
  simp [emitStubs]

def e5ReviewUnits : List ReviewUnit :=
  [
    { subjectRef := "e5-underdeclared-lineage", obligationKind := rollbackCutNonInterference },
    { subjectRef := "e5-underdeclared-lineage", obligationKind := noRePromotion }
  ]

theorem e5_emission_preserves_count :
    (emitStubs e5ReviewUnits).length = 2 := by
  simp [emitStubs, e5ReviewUnits]

theorem e5_first_stub_subject :
    match emitStubs e5ReviewUnits with
    | stub :: _ => stub.subjectRef = "e5-underdeclared-lineage"
    | _ => False := by
  simp [emitStubs, e5ReviewUnits, mkLeanStub]

theorem e5_second_stub_obligation :
    match emitStubs e5ReviewUnits with
    | _ :: stub :: _ => stub.obligationKind = noRePromotion
    | _ => False := by
  simp [emitStubs, e5ReviewUnits, mkLeanStub]

end CurrentL2
