/-!
LAB-only OBL-020 well-formedness preservation statement-shape draft.

This file checks that the step-rule preservation postcondition can be expressed
as Lean propositions without importing final MirCore runtime datatypes, concrete
step rules, scheduler semantics, conformance evidence, or proof-obligation
status changes.
-/

namespace MirCore.Lab.OBL020.StatementDraft

universe u

structure Vocab where
  Config : Type u
  StepLabel : Type u
  StepFamily : Type u

structure Pred (V : Vocab.{u}) where
  WellFormed : V.Config -> Prop
  Step : V.Config -> V.StepLabel -> V.Config -> Prop
  CanonStepFamily : V.StepFamily -> Prop
  StepHasFamily : V.StepLabel -> V.StepFamily -> Prop

def PreservesWF
    {V : Vocab.{u}}
    (P : Pred V)
    (before : V.Config)
    (label : V.StepLabel)
    (after : V.Config) : Prop :=
  P.WellFormed before ->
  P.Step before label after ->
    P.WellFormed after

def FamilyStepPreservesWF
    {V : Vocab.{u}}
    (P : Pred V)
    (before : V.Config)
    (label : V.StepLabel)
    (after : V.Config)
    (family : V.StepFamily) : Prop :=
  P.CanonStepFamily family ->
  P.StepHasFamily label family ->
    PreservesWF P before label after

def OBL020StatementDraft
    (V : Vocab.{u})
    (P : Pred V) : Prop :=
  forall
    (before : V.Config)
    (label : V.StepLabel)
    (after : V.Config),
      PreservesWF P before label after

end MirCore.Lab.OBL020.StatementDraft
