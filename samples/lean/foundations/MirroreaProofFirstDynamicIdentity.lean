import MirroreaProofFirstDynamicSupport
import MirroreaProofFirstCurrentUse

namespace MirroreaProofFirst.IdentityGrowth
open CurrentUse Growth

-- Enumeration is append-only in this reference cut. Retirement retains the
-- slot; re-incarnation must change the identity. No external identity format.
def extend (old : Fin n → A) (fresh : Fin m → A) (k : Fin (n+m)) : A :=
  if h : k.val < n then old ⟨k.val,h⟩ else fresh ⟨k.val-n,by omega⟩

@[simp] theorem extend_old (old : Fin n → A) (fresh : Fin m → A) (k : Fin n) :
    extend old fresh (left n m k) = old k := by
  simp [extend,left,k.isLt]

def mapRecord (m : Nat) (r : Record n) : Record (n+m) :=
  { identity := r.identity, principal := r.principal, home := left n m r.home,
    moduleKey := left n m r.moduleKey, action := r.action, code := r.code,
    contract := r.contract }

def mapHandle (m : Nat) (h : Handle n) : Handle (n+m) :=
  ⟨h.instanceId,left n m h.key,h.identity⟩

def mapRequest (m : Nat) (u : UseRequest n) : UseRequest (n+m) :=
  ⟨u.principal,mapHandle m u.member,mapHandle m u.locus,
    mapHandle m u.moduleHandle,mapHandle m u.operation,u.request,u.arguments⟩

structure Addition (n m : Nat) where
  records : Fin m → Record (n+m)
  policies : Fin m → Policy
  forms : Fin m → Support.Formula (Fin (n+m))
  eligible : Fin m → Bool

def grow (s : World n) (a : Addition n m) : World (n+m) :=
  { instanceId := s.instanceId, generation := s.generation,
    support := append s.support a.forms a.eligible,
    records := extend (fun k => mapRecord m (s.records k)) a.records,
    authority := s.authority, policies := extend s.policies a.policies }

@[simp] theorem grow_record (s : World n) (a : Addition n m) (k : Fin n) :
    (grow s a).records (left n m k) = mapRecord m (s.records k) :=
  extend_old (fun j => mapRecord m (s.records j)) a.records k

@[simp] theorem grow_policy (s : World n) (a : Addition n m) (k : Fin n) :
    (grow s a).policies (left n m k) = s.policies k := extend_old _ _ k

theorem grow_checkHandle (s : World n) (a : Addition n m)
    (kind : RecordKind) (h : Handle n) :
    checkHandle (grow s a) kind (mapHandle m h) = checkHandle s kind h := by
  simp only [checkHandle,mapHandle,grow_record,mapRecord]
  rw [show (grow s a).instanceId = s.instanceId from rfl]
  exact congrArg (fun b => decide (h.instanceId = s.instanceId) &&
    decide (h.identity = (s.records h.key).identity) && decide (h.identity.kind = kind) && b)
    (append_preserves_computed_live s.support a.forms a.eligible h.key)

theorem grow_currentHandle (s : World n) (a : Addition n m)
    (kind : RecordKind) (h : Handle n) :
    CurrentHandle (grow s a) kind (mapHandle m h) ↔ CurrentHandle s kind h := by
  rw [←checkHandle_exact,←checkHandle_exact,grow_checkHandle]

theorem grow_context (s : World n) (a : Addition n m) (u : UseRequest n) :
    currentContext (grow s a) (mapRequest m u) = currentContext s u := by
  simp [currentContext,mapRequest,mapHandle,mapRecord,left,grow,extend]

theorem left_eq_iff (x y : Fin n) : left n m x = left n m y ↔ x = y :=
  ⟨fun h => left_injective n m h,congrArg (left n m)⟩

theorem grow_checkUse (s : World n) (a : Addition n m)
    (u : UseRequest n) (e : Evidence) :
    checkUse (grow s a) (mapRequest m u) e = checkUse s u e := by
  unfold checkUse
  rw [grow_context]
  simp only [mapRequest,grow_checkHandle]
  simp only [mapHandle,grow_record,mapRecord,left_eq_iff,grow_policy]
  rfl

-- A transport-neutral historical locator, not authority: every component of
-- its old identity is retained, including instance and slot, not just kind.
def locator (h : Handle n) : Nat × Nat × RecordIdentity :=
  (h.instanceId,h.key.val,h.identity)

theorem grow_locator (h : Handle n) : locator (mapHandle m h) = locator h := rfl

theorem mapHandle_injective (m : Nat) : Function.Injective (mapHandle (n:=n) m) := by
  intro a b eq
  cases a with
  | mk ai ak ar =>
    cases b with
    | mk bi bk br =>
      simp only [mapHandle,Handle.mk.injEq,left_eq_iff] at eq
      rcases eq with ⟨rfl,rfl,rfl⟩
      rfl

-- Addition itself never issues/revokes authority or silently changes a policy.
theorem grow_authority (s : World n) (a : Addition n m) :
    (grow s a).authority = s.authority := rfl

namespace Controls
open CurrentUse.Controls
def more : Addition 4 2 :=
  ⟨fun _ => mapRecord 2 (world.records 3), fun _ => policy,
    fun _ => .ref 3,fun _ => true⟩
def added := grow world more
def oldRequest := mapRequest 2 request
def freshRequest : UseRequest 6 :=
  { oldRequest with operation := ⟨7,4,(added.records 4).identity⟩ }
#guard checkUse added oldRequest evidence
#guard checkHandle added .operation freshRequest.operation
-- Creating a live record does not issue permission to use its new target.
#guard (authorize added.authority (added.policies 4)
  (currentContext added freshRequest)).isNone
#guard !checkUse added freshRequest evidence
#guard !checkUse {added with authority := {added.authority with revoked := [2]}}
  oldRequest evidence
#guard locator oldRequest.operation = locator request.operation
end Controls

#print axioms grow_checkHandle
#print axioms grow_currentHandle
#print axioms grow_context
#print axioms grow_checkUse
#print axioms grow_locator
#print axioms mapHandle_injective
#print axioms grow_authority
end MirroreaProofFirst.IdentityGrowth
