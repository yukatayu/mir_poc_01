import MixedCatalogService
import MixedCatalogControls
namespace MirroreaProofFirst.MixedCatalogUse
open MixedCatalogService OwnerEffectService OwnerCheckedArithmetic CurrentUse WorldProjection

-- Declarative rule contains original/current authority and independent value
-- evaluation, not a premise equating the implementation outcome with its result.
def AdmittedWrite (ops : FallibleFlow.Arithmetic) (s : MixedInstanceState.State d p n)
 (v : AuthorityView a) (owner : Owner (size a p n)) (pending : Pending (size a p n))
 (out : Write (size a p n)) : Prop :=
 ∃ key place definition,
 decode pending.request.operation.key = .operation key place ∧
 operationDefinition s key = .owner definition ∧ pending.body = definition.body ∧
 AdmissionPhases.FullUse.Allowed (world s v) pending.request pending.original ∧
 evaluate ops owner.store (args pending) pending.body.tree = some out.value ∧
 out = ⟨pending,AdmissionPhases.atCurrent pending.original (currentContext (world s v) pending.request),
   owner.store pending.body.target,out.value,Recorded.actualReads owner.store pending.body.tree⟩

theorem committed_exact :
 (MixedCatalogService.serve ops s v owner pending).2 = .committed out ↔ AdmittedWrite ops s v owner pending out := by
 constructor
 · intro h
   obtain ⟨key,place,definition,slot,kind,body,meaning⟩ := committed_catalog h
   exact ⟨key,place,definition,slot,kind,body,meaning.1,meaning.2.2⟩
 · rintro ⟨key,place,definition,slot,kind,body,auth,value,record⟩
   have encoded : pending.request.operation.key = encode (.operation key place) := by
     simpa [encode_decode] using congrArg encode slot
   have code : CodeAt (world s v) (registry s) pending := by
     simp [CodeAt,MixedCatalogService.world,encoded,decode_encode,registry,lookup_operation,kind,body]
   simp only [MixedCatalogService.serve,slot,kind]
   exact OwnerEffectService.committed_exact.mpr ⟨auth,code,value,record⟩

theorem committed_state (h : (MixedCatalogService.serve ops s v owner pending).2 = .committed out) :
 (MixedCatalogService.serve ops s v owner pending).1.store = put owner.store pending.body.target out.value ∧
 (MixedCatalogService.serve ops s v owner pending).1.history = owner.history ++ [out] := by
 obtain ⟨key,place,definition,slot,kind,_,meaning⟩ := committed_catalog h
 simp only [MixedCatalogService.serve,slot,kind]
 exact OwnerEffectService.committed_state (OwnerEffectService.committed_exact.mpr meaning)

theorem refused_retains (h : (MixedCatalogService.serve ops s v owner pending).2 = .refused why) :
 (MixedCatalogService.serve ops s v owner pending).1 = owner := by
 unfold MixedCatalogService.serve at *
 split at h
 · split at h
   · rfl
   · exact OwnerEffectService.refused_retains h
 · rfl

-- Valid shared catalog discharges the literal premise. No unchecked registry or
-- caller assertion supplies the body. Store/parsed capture bounds remain inputs.
theorem committed_literals (valid : MixedInstanceState.Valid s)
 (h : (MixedCatalogService.serve ops s v owner pending).2 = .committed out) :
 OwnerNumericBoundary.Literals 63 pending.body.tree := by
 obtain ⟨key,place,definition,slot,kind,body,_⟩ := committed_catalog h
 have checked := valid.definitions (s.instances key).definition
 change MixedOperationDefinitions.Satisfies (operationDefinition s key) at checked
 rw [kind] at checked
 rw [body]
 exact checked.2.2.2.1

theorem store_bounded (valid : MixedInstanceState.Valid s)
 (state : OwnerNumericBoundary.Bounded 63 owner.store)
 (captured : OwnerNumericBoundary.Bounded 63 (args pending)) :
 OwnerNumericBoundary.Bounded 63 (MixedCatalogService.serve (FallibleFlow.signed 63) s v owner pending).1.store := by
 cases h : (MixedCatalogService.serve (FallibleFlow.signed 63) s v owner pending).2 with
 | refused why => rw [refused_retains h]; exact state
 | committed out =>
   rw [(committed_state h).1]
   apply OwnerNumericBoundary.put_bounded state
   obtain ⟨_,_,_,_,_,_,meaning⟩ := committed_catalog h
   exact OwnerNumericBoundary.evaluate_bounded owner.store (args pending) pending.body.tree
     state captured (committed_literals valid h) meaning.2.2.1

namespace Controls
open MixedCatalogControls
-- Deliberately NOT a replay boundary. Catalog kind+authority is necessary but
-- cannot replace one shared queue/used-ID ledger. A second raw call writes again.
def twice := (pending view 2 add.body).map fun request =>
 let first := MixedCatalogService.serve (FallibleFlow.signed 63) installed view initial request
 MixedCatalogService.serve (FallibleFlow.signed 63) installed view first.1 request
#guard (twice.map fun r => r.1.store 0) = some (some 20)
#guard (twice.map fun r => r.1.history.length) = some 2
end Controls

#print axioms committed_exact
#print axioms committed_state
#print axioms refused_retains
#print axioms committed_literals
#print axioms store_bounded
end MirroreaProofFirst.MixedCatalogUse
