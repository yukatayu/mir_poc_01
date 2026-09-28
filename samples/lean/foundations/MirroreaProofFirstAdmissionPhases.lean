import MirroreaProofFirstReferenceAccess
import MirroreaProofFirstModuleContractBoundary

/- Authority component of the existing I3 Awaiting -> ServeReserved distinction.
   A staged request is NOT a permit/reservation. Resolving it rechecks the same
   submitted derivation at the current context; the result binds that context.
   Neither this record nor Evidence is a linear capability. The model makes no
   resource, namespace, custody, peer or physical atomic-use claim. -/
namespace MirroreaProofFirst.AdmissionPhases
open CurrentUse

-- Reuse W3's submitted derivation and W2's revocation frame; no new policy calculus.
def Bound (a : Authority) (p : Policy) (ctx : Context) (e : Evidence) : Prop :=
  e.policy = p.id ∧ e.version = p.version ∧ e.context = ctx ∧
    ReferenceAccess.WitnessMeaning a ctx p.label p.expression e.witness

theorem revalidate_exact : revalidate a p ctx e = true ↔ Bound a p ctx e := by
  simp only [revalidate, Bool.and_eq_true, decide_eq_true_eq,
    ReferenceAccess.witness_exact, Bound, and_assoc]

/-- Every request field except the authority generation must be unchanged. -/
def SameRequest (staged current : Context) : Prop :=
  { staged with generation := current.generation } = current

def sameRequestCheck (staged current : Context) : Bool :=
  decide ({ staged with generation := current.generation } = current)

theorem same_request_exact : sameRequestCheck staged current = true ↔ SameRequest staged current := by
  simp [sameRequestCheck, SameRequest]

/-- Rebind the decision context explicitly; retain the entire witness and policy
    identity/version. This operation alone is not acceptance. -/
def atCurrent (staged : Evidence) (current : Context) : Evidence :=
  { staged with context := current }

def Resolves (a : Authority) (p : Policy) (current : Context) (staged : Evidence) : Prop :=
  SameRequest staged.context current ∧ staged.policy = p.id ∧ staged.version = p.version ∧
    ReferenceAccess.WitnessMeaning a current p.label p.expression staged.witness

def resolve (a : Authority) (p : Policy) (current : Context) (staged : Evidence) : Option Evidence :=
  if sameRequestCheck staged.context current && revalidate a p current (atCurrent staged current)
  then some (atCurrent staged current) else none

theorem resolve_exact : resolve a p current staged = some result ↔
    Resolves a p current staged ∧ result = atCurrent staged current := by
  unfold resolve
  split
  · rename_i checked
    have parts : Resolves a p current staged := by
      simpa [Bool.and_eq_true, same_request_exact, revalidate_exact, Bound,
        atCurrent, Resolves, and_assoc] using checked
    simp [parts, eq_comm]
  · rename_i refused
    have denied : ¬ Resolves a p current staged := by
      simpa [Bool.and_eq_true, same_request_exact, revalidate_exact, Bound,
        atCurrent, Resolves, and_assoc] using refused
    simp [denied]

theorem resolve_relative_complete (valid : Resolves a p current staged) :
    resolve a p current staged = some (atCurrent staged current) :=
  resolve_exact.mpr ⟨valid, rfl⟩

theorem resolved_current (accepted : resolve a p current staged = some result) :
    Bound a p current result ∧ result.witness = staged.witness := by
  obtain ⟨⟨_,policy,version,supported⟩,rfl⟩ := resolve_exact.mp accepted
  exact ⟨⟨policy, version, rfl, supported⟩, rfl⟩

theorem resolved_stale_handoff_rejected
    (accepted : resolve a p current staged = some result)
    (changed : current.generation ≠ next.generation) :
    revalidate later policy next result = false := by
  apply context_change_rejected
  intro same
  have ctx := (resolved_current accepted).1.2.2.1
  exact changed (congrArg Context.generation (ctx.symm.trans same))

theorem revoked_staged_rejected (claimId : Nat)
    (present : claimId ∈ ModuleContractBoundary.CurrentPolicyFrame.used staged.witness)
    (revoked : claimId ∈ a.revoked) : resolve a p current staged = none := by
  have no : checkWitness a current p.label p.expression staged.witness = false := by
    cases checked : checkWitness a current p.label p.expression staged.witness with
    | false => rfl
    | true =>
        exact False.elim (ModuleContractBoundary.CurrentPolicyFrame.used_claims_current
          _ _ _ _ _ checked claimId present revoked)
  simp [resolve, revalidate, atCurrent, no]

theorem changed_request_rejected (changed : ¬ SameRequest staged.context current) :
    resolve a p current staged = none := by
  have denied : sameRequestCheck staged.context current = false := by
    cases h : sameRequestCheck staged.context current with
    | false => rfl
    | true => exact False.elim (changed (same_request_exact.mp h))
  simp [resolve, denied]

namespace Controls
open CurrentUse.Controls

def eitherPolicy : Policy := { policy with expression := .either (.leaf ⟨10,11⟩) (.leaf ⟨20,22⟩) }
def old : Evidence := { evidence with witness := .left (.leaf claimA) }
def revokedA : Authority := { auth with revoked := [claimA.id] }

def g2 : Context := { old.context with generation := old.context.generation + 1 }
-- A still-current selected witness can resolve an Awaiting request under G2.
#guard resolve auth eitherPolicy g2 old = some (atCurrent old g2)
-- A G1 decision is not a G2 handoff; no automatic generation rewrite there.
#guard !revalidate auth eitherPolicy g2 old
#guard revalidate auth eitherPolicy g2 (atCurrent old g2)
-- Revocation cannot be hidden by creating a new decision context.
#guard resolve revokedA eitherPolicy g2 old = none
#guard resolve auth eitherPolicy { g2 with arguments := [.integer 99] } old = none
#guard resolve auth eitherPolicy { g2 with principal := 9 } old = none
#guard resolve auth eitherPolicy { g2 with code := 999 } old = none
#guard resolve auth eitherPolicy { g2 with instanceId := 999 } old = none
end Controls

#print axioms revalidate_exact
#print axioms same_request_exact
#print axioms resolve_exact
#print axioms resolve_relative_complete
#print axioms resolved_current
#print axioms resolved_stale_handoff_rejected
#print axioms revoked_staged_rejected
#print axioms changed_request_rejected
end MirroreaProofFirst.AdmissionPhases


namespace MirroreaProofFirst.AdmissionPhases.FullUse
open CurrentUse
-- This conditional boundary receives the exact retained request; it does not
-- prove provenance of that request or prevent an upstream caller from replacing
-- it. Concrete snapshot/custody and one-use mechanisms are separate obligations.
-- The immutable request retains all four stamped handles. Authority generation
-- rebinding alone cannot refresh member/locus/module revisions or eligibility.
def resolveUse (s : World n) (request : UseRequest n) (staged : Evidence) : Option Evidence :=
 match resolve s.authority (s.policies request.operation.key) (currentContext s request) staged with
 | none => none
 | some current => if checkUse s request current then some current else none

-- CurrentUse is an independent structural/support/authority judgment. Resolves
-- additionally constrains the original witness; an existential new witness
-- cannot replace a revoked selected branch.
def Allowed (s : World n) (request : UseRequest n) (staged : Evidence) : Prop :=
 CurrentUse.CurrentUse s request ∧
 Resolves s.authority (s.policies request.operation.key) (currentContext s request) staged

theorem current_checks (live : CurrentUse.CurrentUse s request)
 (bound : Bound s.authority (s.policies request.operation.key) (currentContext s request) evidence) :
 checkUse s request evidence = true := by
 obtain ⟨hm,hl,hmod,hop,subject,home,moduleLink,_⟩ := live
 simp only [checkUse,Bool.and_eq_true,checkHandle_exact,decide_eq_true_eq]
 exact ⟨⟨⟨⟨⟨⟨⟨hm,hl⟩,hmod⟩,hop⟩,subject⟩,home⟩,moduleLink⟩,revalidate_exact.mpr bound⟩

theorem resolveUse_exact : resolveUse s request staged = some result ↔
 Allowed s request staged ∧ result = atCurrent staged (currentContext s request) := by
 unfold resolveUse
 cases resolved : resolve s.authority (s.policies request.operation.key) (currentContext s request) staged with
 | none =>
   change (none : Option Evidence) = some result ↔ _
   simp only [reduceCtorEq,false_iff,not_and]
   intro h
   have produced :=  resolve_relative_complete h.2
   rw [resolved] at produced
   contradiction
 | some actual =>
   have parts := resolve_exact.mp resolved
   change (if checkUse s request actual then some actual else none) = some result ↔ _
   constructor
   · intro accepted
     split at accepted
     · rename_i checked
       cases accepted
       exact ⟨⟨checkUse_sound _ _ _ checked,parts.1⟩,parts.2⟩
     · contradiction
   · rintro ⟨allowed,equal⟩
     have checked := current_checks allowed.1 (resolved_current resolved).1
     rw [if_pos checked]
     exact congrArg some (parts.2.trans equal.symm)

theorem resolveUse_complete (allowed : Allowed s request staged) :
 resolveUse s request staged = some (atCurrent staged (currentContext s request)) :=
 resolveUse_exact.mpr ⟨allowed,rfl⟩

theorem resolveUse_checked (ok : resolveUse s request staged = some result) :
 checkUse s request result = true ∧ result.witness = staged.witness := by
 obtain ⟨allowed,rfl⟩ := resolveUse_exact.mp ok
 have resolved := resolve_relative_complete allowed.2
 exact ⟨current_checks allowed.1 (resolved_current resolved).1,rfl⟩

theorem stale_member_revision_rejected
 (stale : request.member.identity.revision ≠ (s.records request.member.key).identity.revision) :
 resolveUse s request staged = none := by
 cases h : resolveUse s request staged with
 | none => rfl
 | some result =>
   have live := (resolveUse_exact.mp h).1.1
   have same := live.1.2.1
   exact False.elim (stale (congrArg RecordIdentity.revision same))

namespace Controls
open CurrentUse.Controls
-- Earlier authority-only six-guard countermodel remains valid and is not erased.
def revised : World 4 :=
 {world with records := fun k => if k = 0 then
   {world.records k with identity := ⟨.member,1,1⟩} else world.records k}
#guard (resolveUse world request evidence).isSome
#guard (resolveUse {world with generation := 1} request evidence).isSome
#guard resolveUse revised request evidence = none
#guard resolveUse {world with authority := {auth with revoked := [claimA.id]}} request evidence = none
#guard resolveUse {world with support := {world.support with eligible := fun k => k != 1}} request evidence = none
end Controls
#print axioms current_checks
#print axioms resolveUse_exact
#print axioms resolveUse_complete
#print axioms resolveUse_checked
#print axioms stale_member_revision_rejected
end MirroreaProofFirst.AdmissionPhases.FullUse
