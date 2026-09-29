import MirroreaProofFirstOwnerEffectService
import MirroreaProofFirstCompositionCore
namespace MirroreaProofFirst.OwnerSavedPending
open CurrentUse OwnerEffectService

-- Durable mathematical payload for a pending source operation while the finite
-- configuration grows. Only Fin's bound proof is removed. This is NOT a wire
-- codec, persisted image, authenticated import, or a new authority producer.
structure SavedHandle where
 instanceId : Nat
 key : Nat
 identity : RecordIdentity
 deriving DecidableEq

structure SavedRequest where
 principal : Nat
 member : SavedHandle
 locus : SavedHandle
 moduleHandle : SavedHandle
 operation : SavedHandle
 request : Nat
 arguments : List Scalar
 deriving DecidableEq

structure Saved where
 origin : Origin
 request : SavedRequest
 original : Evidence
 body : Body
 deriving DecidableEq

def saveHandle (h : Handle n) : SavedHandle := ⟨h.instanceId,h.key.val,h.identity⟩
def loadHandle (n : Nat) (h : SavedHandle) : Option (Handle n) :=
 (CompositionCore.index n h.key).map fun key => ⟨h.instanceId,key,h.identity⟩
def saveRequest (r : UseRequest n) : SavedRequest :=
 ⟨r.principal,saveHandle r.member,saveHandle r.locus,saveHandle r.moduleHandle,
  saveHandle r.operation,r.request,r.arguments⟩
def loadRequest (n : Nat) (r : SavedRequest) : Option (UseRequest n) := do
 let member ← loadHandle n r.member
 let locus ← loadHandle n r.locus
 let moduleHandle ← loadHandle n r.moduleHandle
 let operation ← loadHandle n r.operation
 return ⟨r.principal,member,locus,moduleHandle,operation,r.request,r.arguments⟩
def save (p : Pending n) : Saved := ⟨p.origin,saveRequest p.request,p.original,p.body⟩
def materialize (n : Nat) (s : Saved) : Option (Pending n) :=
 (loadRequest n s.request).map fun request => ⟨s.origin,request,s.original,s.body⟩

theorem handle_roundtrip (h : Handle n) : loadHandle n (saveHandle h) = some h := by
 cases h
 simp [loadHandle,saveHandle,CompositionCore.index_roundtrip]

theorem handle_exact : loadHandle n saved = some h ↔ saveHandle h = saved := by
 constructor
 · intro accepted
   unfold loadHandle at accepted
   cases indexed : CompositionCore.index n saved.key with
   | none => simp [indexed] at accepted
   | some key =>
     simp only [indexed,Option.map_some,Option.some.injEq] at accepted
     subst h
     cases saved
     simp only [saveHandle]
     rw [CompositionCore.index_sound _ indexed]
 · intro eq
   rw [←eq]
   exact handle_roundtrip h

theorem request_roundtrip (r : UseRequest n) : loadRequest n (saveRequest r) = some r := by
 cases r
 simp [loadRequest,saveRequest,handle_roundtrip]

theorem request_exact : loadRequest n saved = some request ↔ saveRequest request = saved := by
 constructor
 · intro accepted
   unfold loadRequest at accepted
   cases hm : loadHandle n saved.member with
   | none => simp [hm] at accepted
   | some member =>
    cases hl : loadHandle n saved.locus with
    | none => simp [hm,hl] at accepted
    | some locus =>
     cases hb : loadHandle n saved.moduleHandle with
     | none => simp [hm,hl,hb] at accepted
     | some moduleHandle =>
      cases ho : loadHandle n saved.operation with
      | none => simp [hm,hl,hb,ho] at accepted
      | some operation =>
       simp only [hm,hl,hb,ho,Option.bind_eq_bind,Option.bind_some,Option.pure_def,
         Option.some.injEq] at accepted
       subst request
       cases saved
       simp only [saveRequest]
       rw [handle_exact.mp hm,handle_exact.mp hl,handle_exact.mp hb,handle_exact.mp ho]
 · intro eq
   rw [←eq]
   exact request_roundtrip request

theorem roundtrip (p : Pending n) : materialize n (save p) = some p := by
 cases p
 simp [materialize,save,request_roundtrip]

-- Exact equality includes original witness/version/context, full body, source
-- activation/site/ordinal/control label, every identity and all arguments.
theorem materialize_exact : materialize n saved = some pending ↔ save pending = saved := by
 constructor
 · intro accepted
   unfold materialize at accepted
   cases hr : loadRequest n saved.request with
   | none => simp [hr] at accepted
   | some request =>
     simp only [hr,Option.map_some,Option.some.injEq] at accepted
     subst pending
     cases saved
     simp only [save]
     rw [request_exact.mp hr]
 · intro equal
   rw [←equal]
   exact roundtrip pending

def Coordinates (n : Nat) (s : Saved) : Prop :=
 s.request.member.key < n ∧ s.request.locus.key < n ∧
 s.request.moduleHandle.key < n ∧ s.request.operation.key < n

theorem saved_coordinates (p : Pending n) : Coordinates n (save p) :=
 ⟨p.request.member.key.isLt,p.request.locus.key.isLt,
  p.request.moduleHandle.key.isLt,p.request.operation.key.isLt⟩

theorem materialize_exists : (∃ pending, materialize n saved = some pending) ↔ Coordinates n saved := by
 constructor
 · rintro ⟨pending,loaded⟩
   rw [←materialize_exact.mp loaded]
   exact saved_coordinates pending
 · intro h
   let handle (s : SavedHandle) (bound : s.key < n) : Handle n := ⟨s.instanceId,⟨s.key,bound⟩,s.identity⟩
   let pending : Pending n := ⟨saved.origin,
     ⟨saved.request.principal,handle saved.request.member h.1,handle saved.request.locus h.2.1,
      handle saved.request.moduleHandle h.2.2.1,handle saved.request.operation h.2.2.2,
      saved.request.request,saved.request.arguments⟩,saved.original,saved.body⟩
   exact ⟨pending,materialize_exact.mpr rfl⟩

theorem growth_retains (loaded : materialize n saved = some pending) (growth : n ≤ m) :
 ∃ next, materialize m saved = some next ∧ save next = save pending := by
 have old := materialize_exists.mp ⟨pending,loaded⟩
 have bounds : Coordinates m saved := ⟨Nat.lt_of_lt_of_le old.1 growth,
   Nat.lt_of_lt_of_le old.2.1 growth,Nat.lt_of_lt_of_le old.2.2.1 growth,
   Nat.lt_of_lt_of_le old.2.2.2 growth⟩
 obtain ⟨next,atNew⟩ := materialize_exists.mpr bounds
 exact ⟨next,atNew,(materialize_exact.mp atNew).trans (materialize_exact.mp loaded).symm⟩

-- Materialization cannot reauthorize. Service still revalidates original
-- evidence through the existing current full-use checker, never produce().
def resolve (world : World n) (saved : Saved) : Option (Pending n × Evidence) := do
 let pending ← materialize n saved
 let current ← AdmissionPhases.FullUse.resolveUse world pending.request pending.original
 return (pending,current)

theorem resolve_exact : resolve world saved = some (pending,current) ↔
 save pending = saved ∧ AdmissionPhases.FullUse.Allowed world pending.request pending.original ∧
 current = AdmissionPhases.atCurrent pending.original (currentContext world pending.request) := by
 unfold resolve
 cases hm : materialize _ saved with
 | none =>
   simp only [Option.bind_eq_bind,Option.bind_none,reduceCtorEq,false_iff]
   rintro ⟨same,_,_⟩
   have yes := materialize_exact.mpr same
   rw [hm] at yes
   contradiction
 | some entry =>
   simp only [Option.bind_eq_bind,Option.bind_some]
   cases ha : AdmissionPhases.FullUse.resolveUse world entry.request entry.original with
   | none =>
     simp only [Option.bind_none,reduceCtorEq,false_iff]
     rintro ⟨same,allowed,_⟩
     have eq : entry = pending := Option.some.inj (hm.symm.trans (materialize_exact.mpr same))
     subst entry
     rw [AdmissionPhases.FullUse.resolveUse_complete allowed] at ha
     contradiction
   | some evidence =>
     obtain ⟨allowed,rfl⟩ := AdmissionPhases.FullUse.resolveUse_exact.mp ha
     simp only [Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq]
     constructor
     · rintro ⟨rfl,rfl⟩
       exact ⟨materialize_exact.mp hm,allowed,rfl⟩
     · rintro ⟨same,_,currentAt⟩
       have eq : entry = pending := Option.some.inj (hm.symm.trans (materialize_exact.mpr same))
       subst entry
       exact ⟨rfl,currentAt.symm⟩

#print axioms materialize_exact
#print axioms materialize_exists
#print axioms growth_retains
#print axioms resolve_exact
end MirroreaProofFirst.OwnerSavedPending
