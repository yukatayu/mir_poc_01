/-!
LAB-only THM-001 / OBL-001 statement-shape draft.

This file checks that the ordinary-assignment elaboration soundness postcondition
can be expressed as Lean propositions without importing final MirCore datatypes,
runtime dispatch, conformance evidence, or proof-obligation status changes.
-/

namespace MirCore.Lab.OBL001.StatementDraft

universe u

structure Vocab where
  Env : Type u
  Ctx : Type u
  Locus : Type u
  Assign : Type u
  Result : Type u
  Write : Type u
  Request : Type u

structure Pred (V : Vocab.{u}) where
  SurfaceAssignment : V.Assign -> Prop
  SimpleAssign : V.Assign -> Prop
  ElaboratesAssignment :
    V.Env -> V.Ctx -> V.Locus -> V.Assign -> V.Result -> Prop
  GeneratedWrite : V.Result -> V.Write -> Prop
  OwnerLocalWriteAt :
    V.Env -> V.Locus -> V.Assign -> V.Write -> Prop
  RequestForWrite :
    V.Result -> V.Write -> V.Request -> Prop
  OwnerDirectedRequest :
    V.Env -> V.Locus -> V.Assign -> V.Request -> Prop
  RequestCarriesFailureContainment :
    V.Assign -> V.Request -> Prop
  RequestCarriesAuthorityObligations :
    V.Env -> V.Locus -> V.Assign -> V.Request -> Prop
  RequestCarriesDependencyEvidence :
    V.Assign -> V.Request -> Prop
  RequestCarriesSpanEvidence :
    V.Assign -> V.Request -> Prop
  AllRhsReadsRecorded :
    V.Assign -> V.Result -> Prop
  GeneratedFailuresContained :
    V.Assign -> V.Result -> Prop
  AuthorityObligationsRepresented :
    V.Env -> V.Locus -> V.Assign -> V.Result -> Prop
  SourceSpansPreserved :
    V.Assign -> V.Result -> Prop
  VisibleWriteConsequencesExplicit :
    V.Env -> V.Assign -> V.Result -> Prop
  NoAmbientAuthorityFromNestedLocus :
    V.Env -> V.Locus -> V.Assign -> V.Result -> Prop

def RequestEvidenceSound
    {V : Vocab.{u}}
    (P : Pred V)
    (env : V.Env)
    (locus : V.Locus)
    (assign : V.Assign)
    (result : V.Result)
    (write : V.Write)
    (request : V.Request) : Prop :=
  P.RequestForWrite result write request /\
  P.OwnerDirectedRequest env locus assign request /\
  P.RequestCarriesAuthorityObligations env locus assign request /\
  P.RequestCarriesFailureContainment assign request /\
  P.RequestCarriesDependencyEvidence assign request /\
  P.RequestCarriesSpanEvidence assign request

def GeneratedWriteSound
    {V : Vocab.{u}}
    (P : Pred V)
    (env : V.Env)
    (locus : V.Locus)
    (assign : V.Assign)
    (result : V.Result)
    (write : V.Write) : Prop :=
  P.OwnerLocalWriteAt env locus assign write \/
    exists request,
      RequestEvidenceSound P env locus assign result write request

def AllGeneratedWritesSound
    {V : Vocab.{u}}
    (P : Pred V)
    (env : V.Env)
    (locus : V.Locus)
    (assign : V.Assign)
    (result : V.Result) : Prop :=
  forall write,
    P.GeneratedWrite result write ->
      GeneratedWriteSound P env locus assign result write

def AssignmentElabSoundnessPost
    {V : Vocab.{u}}
    (P : Pred V)
    (env : V.Env)
    (locus : V.Locus)
    (assign : V.Assign)
    (result : V.Result) : Prop :=
  AllGeneratedWritesSound P env locus assign result /\
  P.AllRhsReadsRecorded assign result /\
  P.GeneratedFailuresContained assign result /\
  P.AuthorityObligationsRepresented env locus assign result /\
  P.SourceSpansPreserved assign result /\
  P.VisibleWriteConsequencesExplicit env assign result /\
  P.NoAmbientAuthorityFromNestedLocus env locus assign result

def THM001StatementDraft
    (V : Vocab.{u})
    (P : Pred V) : Prop :=
  forall
    (env : V.Env)
    (ctx : V.Ctx)
    (locus : V.Locus)
    (assign : V.Assign)
    (result : V.Result),
      P.SurfaceAssignment assign ->
      P.SimpleAssign assign ->
      P.ElaboratesAssignment env ctx locus assign result ->
        AssignmentElabSoundnessPost P env locus assign result

end MirCore.Lab.OBL001.StatementDraft
