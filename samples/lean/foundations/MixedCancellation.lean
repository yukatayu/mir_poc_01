import MixedRequestEmbedding
namespace MirroreaProofFirst.MixedCancellation
open CurrentUse WorldProjection
open ReferenceCancellationBoundary (WitnessMeaning witness_exact)
abbrev Permit := ReferenceCancellationBoundary.Permit
abbrev Context := ReferenceCancellationBoundary.Context
-- Exact pure cancellation boundary lifted to the common catalog. Permit and
-- witness meaning are reused, not weakened. No owner-effect cancellation policy.
-- Original source SHA256 748a5ea45f14fcc32169e3898f6133de7f37efff730e34b9e8104941fcd54363
def current (s : MixedManagementEntry.System p a) (frontier : Nat) (ticket : InvocationBoundary.Ticket)
    (member : Fin a) (place : Fin p) (principal id : Nat) : Context :=
  let data := s.configuration.state
  {scope :=
    {principal := principal,member := member.val,memberIncarnation := (s.view.members member).incarnation,
     locus := place.val,locusIncarnation := data.placeIncarnation place,
     moduleKey := ticket.key,moduleIncarnation := ticket.moduleIdentity.incarnation,
     action := 15,target := ticket.id,targetIncarnation := ticket.memberIdentity.incarnation,
     targetRevision := ticket.memberIdentity.revision,instanceId := data.realm,
     request := id,arguments := [.integer ticket.argument],code := ticket.evidence.context.code,
     contract := ticket.evidence.context.contract,generation := s.view.generation},
   ticket := ticket,cut := s.serial,occurrenceFrontier := frontier,
   memberIdentity := ⟨.member,(s.view.members member).incarnation,(s.view.members member).revision⟩,
   locusIdentity := ⟨.locus,data.placeIncarnation place,0⟩}

-- Full ticket equality binds definition, arithmetic/theory versions, all saved
-- identities, argument and invocation evidence. The issuer leaf in this finite
-- profile grants action/target scope; it is not a predicate on every field.
def Caller (s : MixedManagementEntry.System p a) (ticket : InvocationBoundary.Ticket)
    (member : Fin a) (place : Fin p) (principal : Nat) : Prop :=
  MixedManagementEntry.CurrentActor s member place principal ∧
  ticket.realm = s.configuration.state.realm ∧ ticket.member = member.val ∧
  ticket.place = place.val ∧ ticket.principal = principal

def actorCheck (s : MixedManagementEntry.System p a) (ticket : InvocationBoundary.Ticket)
    (member : Fin a) (place : Fin p) (principal : Nat) : Bool :=
  MixedManagementEntry.actorCheck s member place principal &&
    decide (ticket.realm = s.configuration.state.realm ∧ ticket.member = member.val ∧
      ticket.place = place.val ∧ ticket.principal = principal)

theorem actor_exact (s : MixedManagementEntry.System p a) (ticket : InvocationBoundary.Ticket)
    (member : Fin a) (place : Fin p) (principal : Nat) :
    actorCheck s ticket member place principal = true ↔ Caller s ticket member place principal := by
  simp only [actorCheck,Bool.and_eq_true,MixedManagementEntry.actor_exact,decide_eq_true_eq,Caller]

def check (s : MixedManagementEntry.System p a) (frontier : Nat) (ticket : InvocationBoundary.Ticket) (member : Fin a)
    (place : Fin p) (principal id : Nat) (permit : Permit) : Bool :=
  let ctx := current s frontier ticket member place principal id
  let policy := s.controlPolicy 15
  actorCheck s ticket member place principal && decide (permit.context = ctx) &&
    decide (permit.policy = policy.id) && decide (permit.version = policy.version) &&
    checkWitness s.view.authority ctx.scope policy.label policy.expression permit.witness

def Allowed (s : MixedManagementEntry.System p a) (frontier : Nat) (ticket : InvocationBoundary.Ticket) (member : Fin a)
    (place : Fin p) (principal id : Nat) : Prop :=
  Caller s ticket member place principal ∧
  Authorized s.view.authority (current s frontier ticket member place principal id).scope
    (s.controlPolicy 15).label (s.controlPolicy 15).expression

def Submitted (s : MixedManagementEntry.System p a) (frontier : Nat) (ticket : InvocationBoundary.Ticket) (member : Fin a)
    (place : Fin p) (principal id : Nat) (permit : Permit) : Prop :=
  Caller s ticket member place principal ∧
  permit.context = current s frontier ticket member place principal id ∧
  permit.policy = (s.controlPolicy 15).id ∧ permit.version = (s.controlPolicy 15).version ∧
  WitnessMeaning s.view.authority (current s frontier ticket member place principal id).scope
    (s.controlPolicy 15).label (s.controlPolicy 15).expression permit.witness

theorem check_exact (s : MixedManagementEntry.System p a) (frontier : Nat) (ticket : InvocationBoundary.Ticket) (member : Fin a)
    (place : Fin p) (principal id : Nat) (permit : Permit) :
    check s frontier ticket member place principal id permit = true ↔
      Submitted s frontier ticket member place principal id permit := by
  simp only [check,Bool.and_eq_true,actor_exact,decide_eq_true_eq,witness_exact,Submitted,and_assoc]

def authorize (s : MixedManagementEntry.System p a) (frontier : Nat) (ticket : InvocationBoundary.Ticket) (member : Fin a)
    (place : Fin p) (principal id : Nat) : Option Permit := do
  if !actorCheck s ticket member place principal then none else do
    let ctx := current s frontier ticket member place principal id
    let policy := s.controlPolicy 15
    let witness ← produce s.view.authority ctx.scope policy.label policy.expression
    return ⟨ctx,policy.id,policy.version,witness⟩

theorem authorize_checked (s : MixedManagementEntry.System p a) (frontier : Nat) (ticket : InvocationBoundary.Ticket) (member : Fin a)
    (place : Fin p) (principal id : Nat) (permit : Permit)
    (authorized : authorize s frontier ticket member place principal id = some permit) :
    check s frontier ticket member place principal id permit = true := by
  unfold authorize at authorized
  split at authorized
  · cases authorized
  · rename_i allowed
    have actor : actorCheck s ticket member place principal = true := by simpa using allowed
    cases hp : produce s.view.authority (current s frontier ticket member place principal id).scope
        (s.controlPolicy 15).label (s.controlPolicy 15).expression with
    | none => simp [hp] at authorized
    | some witness =>
        simp only [hp,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at authorized
        subst permit
        simp [check,actor,produce_checked _ _ _ _ _ hp]

theorem authorize_complete (s : MixedManagementEntry.System p a) (frontier : Nat) (ticket : InvocationBoundary.Ticket) (member : Fin a)
    (place : Fin p) (principal id : Nat) (allowed : Allowed s frontier ticket member place principal id) :
    ∃ permit, authorize s frontier ticket member place principal id = some permit := by
  obtain ⟨w,hw⟩ := produce_complete _ _ _ _ allowed.2
  exact ⟨⟨current s frontier ticket member place principal id,
      (s.controlPolicy 15).id,(s.controlPolicy 15).version,w⟩,
    by simp [authorize,(actor_exact _ _ _ _ _).mpr allowed.1,hw]⟩

theorem changed_ticket_rejected (s : MixedManagementEntry.System p a) (frontier : Nat)
    (ticket : InvocationBoundary.Ticket) (member : Fin a) (place : Fin p) (principal id : Nat)
    (permit : Permit) (different : permit.context.ticket ≠ ticket) :
    check s frontier ticket member place principal id permit = false := by
  have unequal : permit.context ≠ current s frontier ticket member place principal id := by
    intro equal; exact different (congrArg ReferenceCancellationBoundary.Context.ticket equal)
  simp [check,unequal]

theorem changed_cut_rejected (s : MixedManagementEntry.System p a) (frontier : Nat)
    (ticket : InvocationBoundary.Ticket) (member : Fin a) (place : Fin p) (principal id : Nat)
    (permit : Permit) (different : permit.context.cut ≠ s.serial) :
    check s frontier ticket member place principal id permit = false := by
  have unequal : permit.context ≠ current s frontier ticket member place principal id := by
    intro equal; exact different (congrArg ReferenceCancellationBoundary.Context.cut equal)
  simp [check,unequal]


theorem pure_current (s : ManagementEntry.System p a) (frontier : Nat) (ticket : InvocationBoundary.Ticket)
 (member : Fin a) (place : Fin p) (principal id : Nat) :
 current (MixedManagementEmbedding.system s) frontier ticket member place principal id =
 ReferenceCancellationBoundary.current s frontier ticket member place principal id := rfl

theorem pure_check (s : ManagementEntry.System p a) (frontier : Nat) (ticket : InvocationBoundary.Ticket)
 (member : Fin a) (place : Fin p) (principal id : Nat) (permit : Permit) :
 check (MixedManagementEmbedding.system s) frontier ticket member place principal id permit =
 ReferenceCancellationBoundary.check s frontier ticket member place principal id permit := rfl

theorem pure_authorize (s : ManagementEntry.System p a) (frontier : Nat) (ticket : InvocationBoundary.Ticket)
 (member : Fin a) (place : Fin p) (principal id : Nat) :
 authorize (MixedManagementEmbedding.system s) frontier ticket member place principal id =
 ReferenceCancellationBoundary.authorize s frontier ticket member place principal id := rfl

#print axioms check_exact
#print axioms authorize_checked
#print axioms authorize_complete
#print axioms changed_ticket_rejected
#print axioms changed_cut_rejected
#print axioms pure_check
#print axioms pure_authorize
end MirroreaProofFirst.MixedCancellation
