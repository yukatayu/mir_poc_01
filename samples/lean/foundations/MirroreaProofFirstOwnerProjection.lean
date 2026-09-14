import MirroreaProofFirstReferenceSource

namespace MirroreaProofFirst.OwnerProjection
open InstanceState InstancePrograms WorldProjection InvocationBoundary CurrentUse

-- Sufficient mathematical admission input for the unary invocation boundary.
-- Excludes caller values, pending continuation and source/event histories.
-- This is NOT a wire format or permission to disclose all policy/claim data;
-- authorized finite representation and authentic current publication remain owed.
structure Input (p a : Nat) where
  configuration : CompositionCore.Config p
  view : AuthorityView a

def project (s : ReferenceSource.State p a) : Input p a :=
  ⟨s.machine.store.core.system.configuration,s.machine.store.core.system.view⟩

def EvidenceValid (authority : Authority) (policy : Policy) (context : CurrentUse.Context)
    (evidence : Evidence) : Prop :=
  evidence.policy = policy.id ∧ evidence.version = policy.version ∧ evidence.context = context ∧
    ReferenceAccess.WitnessMeaning authority context policy.label policy.expression evidence.witness

theorem evidence_exact : CurrentUse.revalidate authority policy context evidence = true ↔
    EvidenceValid authority policy context evidence := by
  simp [CurrentUse.revalidate,EvidenceValid,ReferenceAccess.witness_exact,and_assoc]

def SubmittedAt (s : State d p n) (v : AuthorityView a) (t : Ticket)
    (member : Fin a) (key : Fin n) (place : Fin p) : Prop :=
  Current s v t member key place ∧
    EvidenceValid (world s v).authority ((world s v).policies (savedRequest t member key place).operation.key)
      (currentContext (world s v) (savedRequest t member key place)) t.evidence

-- Relative completeness for the ACTUALLY submitted witness. Mere existential
-- authorization could permit replacing a stale disjunction branch and is weaker.
theorem checkAt_exact (s : State d p n) (v : AuthorityView a) (t : Ticket)
    (member : Fin a) (key : Fin n) (place : Fin p) :
    InvocationBoundary.checkAt s v t member key place = true ↔ SubmittedAt s v t member key place := by
  constructor
  · intro checked
    refine ⟨checkAt_sound _ _ _ _ _ _ checked,?_⟩
    simp only [InvocationBoundary.checkAt,Bool.and_eq_true] at checked
    have use := checked.2
    simp only [checkUse,Bool.and_eq_true] at use
    exact evidence_exact.mp use.2
  · rintro ⟨⟨hm,hk,hl,hprofile,htheory,hdefinition,hdomain,
      memberLive,locusLive,moduleLive,operationLive,subject,home,moduleKey,authorized⟩,evidence⟩
    simp [InvocationBoundary.checkAt,hm,hk,hl,hprofile,htheory,hdefinition,hdomain,
      checkUse,(checkHandle_exact _ _ _).mpr memberLive,(checkHandle_exact _ _ _).mpr locusLive,
      (checkHandle_exact _ _ _).mpr moduleLive,(checkHandle_exact _ _ _).mpr operationLive,
      subject,home,moduleKey,evidence_exact.mpr evidence]

def Allowed (input : Input p a) (assigned : Fin p) (ticket : Ticket) : Prop :=
  ∃ member key, SubmittedAt input.configuration.state input.view ticket member key assigned

def check (input : Input p a) (assigned : Fin p) (ticket : Ticket) : Bool :=
  decide (ticket.place = assigned.val) && InvocationBoundary.check input.configuration.state input.view ticket

theorem check_exact (input : Input p a) (assigned : Fin p) (ticket : Ticket) :
    check input assigned ticket = true ↔ Allowed input assigned ticket := by
  constructor
  · intro checked
    simp only [check,Bool.and_eq_true,decide_eq_true_eq] at checked
    obtain ⟨member,key,place,accepted⟩ := check_parts _ _ _ checked.2
    have submitted := (checkAt_exact _ _ _ _ _ _).mp accepted
    have same : place = assigned := by
      apply Fin.ext
      exact submitted.1.2.2.1.symm.trans checked.1
    subst place
    exact ⟨member,key,submitted⟩
  · rintro ⟨member,key,submitted⟩
    have accepted := (checkAt_exact _ _ _ _ _ _).mpr submitted
    have hm := submitted.1.1
    have hk := submitted.1.2.1
    have hl := submitted.1.2.2.1
    simp [check,InvocationBoundary.check,hm,hk,hl,CompositionCore.index_roundtrip,accepted]

def serve (input : Input p a) (assigned : Fin p) (ticket : Ticket) : Option Int :=
  if check input assigned ticket then InvocationBoundary.execute ticket else none

theorem serve_exact {input : Input p a} {assigned : Fin p} {ticket : Ticket} {value : Int} : serve input assigned ticket = some value ↔
    Allowed input assigned ticket ∧ Machine.Executes ticket.definition.code ticket.argument value := by
  unfold serve
  split
  · rename_i admitted
    simp only [(check_exact _ _ _).mp admitted,true_and]
    exact Machine.run_exact _ _ _
  · rename_i refused
    simp [show ¬ Allowed input assigned ticket from fun allowed => refused ((check_exact _ _ _).mpr allowed)]

theorem admitted_serve_exists (input : Input p a) (assigned : Fin p) (ticket : Ticket)
    (valid : CompositionCore.Invariant input.configuration) (allowed : Allowed input assigned ticket) :
    ∃ value, serve input assigned ticket = some value ∧ Output ticket.definition.contract value := by
  have checked := (check_exact _ _ _).mpr allowed
  have eligible : InvocationBoundary.check input.configuration.state input.view ticket = true := by
    simp only [check,Bool.and_eq_true] at checked
    exact checked.2
  obtain ⟨value,execution,output⟩ := current_execution _ _ _ valid eligible
  exact ⟨value,serve_exact.mpr ⟨allowed,execution⟩,output⟩

theorem wrong_locus_rejected {input : Input p a} {assigned : Fin p} {ticket : Ticket}
    (wrong : ticket.place ≠ assigned.val) : serve input assigned ticket = none := by
  simp [serve,check,wrong]

theorem projection_factors (s t : ReferenceSource.State p a) (same : project s = project t)
    (assigned : Fin p) (ticket : Ticket) : serve (project s) assigned ticket = serve (project t) assigned ticket := by
  rw [same]

theorem source_write_frames_projection (s : ReferenceSource.State p a) (site : ReferenceSourceData.Site)
    (before : Nat) (deps : List ReferenceSource.Read) (name : String) (value : ReferenceSourceData.Value)
    (assign : Bool) : project (ReferenceSource.write s site before deps name value assign) = project s := rfl

#print axioms evidence_exact
#print axioms checkAt_exact
#print axioms check_exact
#print axioms serve_exact
#print axioms admitted_serve_exists
#print axioms wrong_locus_rejected
#print axioms projection_factors
#print axioms source_write_frames_projection
end MirroreaProofFirst.OwnerProjection
