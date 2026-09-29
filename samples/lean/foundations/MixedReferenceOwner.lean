import MixedReferenceSelection
import MixedReferenceHolding
import MirroreaProofFirstReferenceOwner
namespace MirroreaProofFirst.MixedReferenceOwner
open CurrentUse WorldProjection
open ReferenceAccess (WitnessMeaning witness_exact)
open ReferenceOwner (Binding Change previous after subject action)
abbrev Context := ReferenceOwner.Context
abbrev Permit := ReferenceOwner.Permit
-- Authorizes reference BINDING mutation, not owner-effect operation execution.
-- Saved payloads remain the existing Binding/Choice/Stamp types.
def current (s : MixedManagementEntry.System p a) (frontier : Nat) (change : Change)
    (member : Fin a) (place : Fin p) (principal id : Nat) : Context :=
  let data := s.configuration.state
  {scope :=
    {principal := principal, member := member.val, memberIncarnation := (s.view.members member).incarnation,
     locus := place.val, locusIncarnation := data.placeIncarnation place,
     moduleKey := (subject change).chain.reader, moduleIncarnation := 0,
     action := action change, target := (subject change).binding, targetIncarnation := (subject change).epoch,
     targetRevision := (subject change).lineage, instanceId := data.realm,
     request := id, arguments := [],code := 0,contract := 0,generation := s.view.generation},
   change := change,cut := s.serial,occurrenceFrontier := frontier,
   memberIdentity := ⟨.member,(s.view.members member).incarnation,(s.view.members member).revision⟩,
   locusIdentity := ⟨.locus,data.placeIncarnation place,0⟩}

-- Ownership designation alone is insufficient. The current actor and an
-- independently issued action/target claim are both required below. Captured
-- old actor stamps do not authorize rejoin; fresh current claims may do so.
def OwnerActor (s : MixedManagementEntry.System p a) (change : Change) (member : Fin a)
    (place : Fin p) (principal : Nat) : Prop :=
  MixedManagementEntry.CurrentActor s member place principal ∧
  (subject change).member = member.val ∧ (subject change).place = place.val ∧
  (subject change).principal = principal

def actorCheck (s : MixedManagementEntry.System p a) (change : Change) (member : Fin a)
    (place : Fin p) (principal : Nat) : Bool :=
  MixedManagementEntry.actorCheck s member place principal &&
    decide ((subject change).member = member.val ∧ (subject change).place = place.val ∧
      (subject change).principal = principal)

theorem actor_exact (s : MixedManagementEntry.System p a) (change : Change) (member : Fin a)
    (place : Fin p) (principal : Nat) : actorCheck s change member place principal = true ↔
      OwnerActor s change member place principal := by
  simp only [actorCheck,Bool.and_eq_true,MixedManagementEntry.actor_exact,decide_eq_true_eq,OwnerActor]

def check (s : MixedManagementEntry.System p a) (frontier : Nat) (change : Change) (member : Fin a)
    (place : Fin p) (principal id : Nat) (permit : Permit) : Bool :=
  let ctx := current s frontier change member place principal id
  let policy := s.controlPolicy (action change)
  actorCheck s change member place principal && decide (permit.context = ctx) &&
    decide (permit.policy = policy.id) && decide (permit.version = policy.version) &&
    checkWitness s.view.authority ctx.scope policy.label policy.expression permit.witness

def Allowed (s : MixedManagementEntry.System p a) (frontier : Nat) (change : Change) (member : Fin a)
    (place : Fin p) (principal id : Nat) : Prop :=
  OwnerActor s change member place principal ∧
  Authorized s.view.authority (current s frontier change member place principal id).scope
    (s.controlPolicy (action change)).label (s.controlPolicy (action change)).expression

def Submitted (s : MixedManagementEntry.System p a) (frontier : Nat) (change : Change) (member : Fin a)
    (place : Fin p) (principal id : Nat) (permit : Permit) : Prop :=
  OwnerActor s change member place principal ∧
  permit.context = current s frontier change member place principal id ∧
  permit.policy = (s.controlPolicy (action change)).id ∧ permit.version = (s.controlPolicy (action change)).version ∧
  WitnessMeaning s.view.authority (current s frontier change member place principal id).scope
    (s.controlPolicy (action change)).label (s.controlPolicy (action change)).expression permit.witness

theorem check_exact (s : MixedManagementEntry.System p a) (frontier : Nat) (change : Change) (member : Fin a)
    (place : Fin p) (principal id : Nat) (permit : Permit) :
    check s frontier change member place principal id permit = true ↔
      Submitted s frontier change member place principal id permit := by
  simp only [check,Bool.and_eq_true,actor_exact,decide_eq_true_eq,witness_exact,Submitted,and_assoc]

def authorize (s : MixedManagementEntry.System p a) (frontier : Nat) (change : Change) (member : Fin a)
    (place : Fin p) (principal id : Nat) : Option Permit := do
  if !actorCheck s change member place principal then none else do
    let ctx := current s frontier change member place principal id
    let policy := s.controlPolicy (action change)
    let witness ← produce s.view.authority ctx.scope policy.label policy.expression
    return ⟨ctx,policy.id,policy.version,witness⟩

theorem authorize_checked (s : MixedManagementEntry.System p a) (frontier : Nat) (change : Change) (member : Fin a)
    (place : Fin p) (principal id : Nat) (permit : Permit)
    (authorized : authorize s frontier change member place principal id = some permit) :
    check s frontier change member place principal id permit = true := by
  unfold authorize at authorized
  split at authorized
  · cases authorized
  · rename_i allowed
    have actor : actorCheck s change member place principal = true := by simpa using allowed
    cases hp : produce s.view.authority (current s frontier change member place principal id).scope
        (s.controlPolicy (action change)).label (s.controlPolicy (action change)).expression with
    | none => simp [hp] at authorized
    | some witness =>
        simp only [hp,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at authorized
        subst permit
        simp [check,actor,produce_checked _ _ _ _ _ hp]

theorem authorize_complete (s : MixedManagementEntry.System p a) (frontier : Nat) (change : Change) (member : Fin a)
    (place : Fin p) (principal id : Nat) (allowed : Allowed s frontier change member place principal id) :
    ∃ permit, authorize s frontier change member place principal id = some permit := by
  obtain ⟨w,hw⟩ := produce_complete _ _ _ _ allowed.2
  exact ⟨⟨current s frontier change member place principal id,
      (s.controlPolicy (action change)).id,(s.controlPolicy (action change)).version,w⟩,
    by simp [authorize,(actor_exact _ _ _ _ _).mpr allowed.1,hw]⟩

theorem changed_payload_rejected (s : MixedManagementEntry.System p a) (frontier : Nat) (change : Change) (member : Fin a)
    (place : Fin p) (principal id : Nat) (permit : Permit) (different : permit.context.change ≠ change) :
    check s frontier change member place principal id permit = false := by
  have unequal : permit.context ≠ current s frontier change member place principal id := by
    intro equal; exact different (congrArg ReferenceOwner.Context.change equal)
  simp [check,unequal]

theorem changed_frontier_rejected (s : MixedManagementEntry.System p a) (frontier : Nat) (change : Change) (member : Fin a)
    (place : Fin p) (principal id : Nat) (permit : Permit) (different : permit.context.occurrenceFrontier ≠ frontier) :
    check s frontier change member place principal id permit = false := by
  have unequal : permit.context ≠ current s frontier change member place principal id := by
    intro equal; exact different (congrArg ReferenceOwner.Context.occurrenceFrontier equal)
  simp [check,unequal]

theorem pure_current (s : ManagementEntry.System p a) (frontier : Nat) (change : Change)
 (member : Fin a) (place : Fin p) (principal id : Nat) :
 current (MixedManagementEmbedding.system s) frontier change member place principal id =
 ReferenceOwner.current s frontier change member place principal id := rfl

theorem pure_check (s : ManagementEntry.System p a) (frontier : Nat) (change : Change)
 (member : Fin a) (place : Fin p) (principal id : Nat) (permit : Permit) :
 check (MixedManagementEmbedding.system s) frontier change member place principal id permit =
 ReferenceOwner.check s frontier change member place principal id permit := rfl

theorem pure_authorize (s : ManagementEntry.System p a) (frontier : Nat) (change : Change)
 (member : Fin a) (place : Fin p) (principal id : Nat) :
 authorize (MixedManagementEmbedding.system s) frontier change member place principal id =
 ReferenceOwner.authorize s frontier change member place principal id := rfl

#print axioms pure_current
#print axioms pure_check
#print axioms pure_authorize
-- This module ONLY authorizes a proposed full payload. The consumer must still
-- check old-store equality, CatalogValid/static floors, selection history,
-- post-commit lease, request freshness and the actual allowed state transition.
-- No mutation or preservation claim follows from authorize alone.
#print axioms actor_exact
#print axioms check_exact
#print axioms authorize_checked
#print axioms authorize_complete
#print axioms changed_payload_rejected
#print axioms changed_frontier_rejected
end MirroreaProofFirst.MixedReferenceOwner
