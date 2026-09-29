import MixedCatalogEmbedding
namespace MirroreaProofFirst.MixedCatalogService
open MixedInstanceState WorldProjection
open MixedOperationDefinitions (Definition OwnerDefinition)

-- Private research discriminator, not public action numbering. Existing pure
-- invocation is 5; this candidate must receive explicit owner-use claims for 16.
def invocationAction : Definition → Nat
 | .pure _ => 5
 | .owner _ => 16

def operationDefinition (s : State d p n) (key : Fin n) : Definition :=
 s.definitions (s.instances key).definition

def located (definition : Definition) (place : Fin p) : Bool :=
 match definition with
 | .pure _ => true
 | .owner d => decide (MixedOperationDefinitions.ownerAt d.contract d.body.target = place.val)

def slotRecord (s : State d p n) (v : AuthorityView a) : Slot a p n → CurrentUse.Record (size a p n)
 | .member key =>
  {identity := ⟨.member,(v.members key).incarnation,(v.members key).revision⟩,
   principal := (v.members key).principal,home := encode (.member key),moduleKey := encode (.member key),
   action := 0,code := 0,contract := 0}
 | .locus place =>
  {identity := ⟨.locus,s.placeIncarnation place,0⟩,
   principal := 0,home := encode (.locus place),moduleKey := encode (.locus place),
   action := 0,code := 0,contract := 0}
 | .moduleSlot key =>
  {identity := ⟨.module,1,(s.instances key).revision⟩,
   principal := (s.instances key).owner,home := encode (.moduleSlot key),moduleKey := encode (.moduleSlot key),
   action := 0,code := (s.instances key).definition.val,contract := (s.instances key).definition.val}
 | .operation key place =>
  {identity := ⟨.operation,1,(s.instances key).revision⟩,
   principal := (s.instances key).owner,home := encode (.locus place),moduleKey := encode (.moduleSlot key),
   action := invocationAction (operationDefinition s key),
   code := (s.instances key).definition.val,contract := (s.instances key).definition.val}

def slotForm (s : State d p n) : Slot a p n → Support.Formula (Fin (size a p n))
 | .member _ | .locus _ => .top
 | .moduleSlot key => Growth.mapFormula (fun k => encode (.moduleSlot k)) ((snapshot s).forms key)
 | .operation key place => .both (.ref (encode (.moduleSlot key))) (.ref (encode (.locus place)))
def slotEligible (s : State d p n) (v : AuthorityView a) : Slot a p n → Bool
 | .member key => (v.members key).enabled && decide (s.realm = v.realm)
 | .locus place => s.participating place
 | .moduleSlot key => (snapshot s).eligible key
 | .operation key place => (s.instances key).placements.contains place && s.participating place &&
     located (operationDefinition s key) place

def support (s : State d p n) (v : AuthorityView a) : Support.Snapshot (size a p n) :=
 ⟨fun key => slotForm s (decode key),fun key => slotEligible s v (decode key)⟩
def world (s : State d p n) (v : AuthorityView a) : CurrentUse.World (size a p n) :=
 {instanceId := s.realm,generation := v.generation,support := support s v,
  records := fun key => slotRecord s v (decode key),
  authority := if s.realm = v.realm then v.authority else emptyAuthority,
  policies := fun key => match decode key with
   | .operation unit place => v.policies unit.val place.val
   | _ => inactivePolicy}

def lookupOwner (s : State d p n) (code : Nat) : Option OwnerDefinition := do
 let key ← CompositionCore.index d code
 match s.definitions key with | .owner op => some op | .pure _ => none
def registry (s : State d p n) (code : Nat) : Option OwnerEffectService.Body :=
 (lookupOwner s code).map OwnerDefinition.body

theorem lookup_operation (s : State d p n) (v : AuthorityView a) (key : Fin n) (place : Fin p) :
 lookupOwner s (slotRecord s v (.operation key place)).code =
 match operationDefinition s key with | .owner op => some op | .pure _ => none := by
 simp [lookupOwner,slotRecord,CompositionCore.index_roundtrip,operationDefinition]

theorem pure_record (s : InstanceState.State d p n) (v : AuthorityView a) (slot : Slot a p n) :
 slotRecord (MixedCatalogEmbedding.state s) v slot = WorldProjection.slotRecord s v slot := by
 cases slot <;> rfl

theorem pure_form (s : InstanceState.State d p n) (slot : Slot a p n) :
 slotForm (MixedCatalogEmbedding.state s) slot = WorldProjection.slotForm s slot := by
 cases slot <;> rfl

theorem pure_eligible (s : InstanceState.State d p n) (v : AuthorityView a) (slot : Slot a p n) :
 slotEligible (MixedCatalogEmbedding.state s) v slot = WorldProjection.slotEligible s v slot := by
 cases slot <;> simp [slotEligible,WorldProjection.slotEligible,located,operationDefinition,
   MixedCatalogEmbedding.state,MixedCatalogEmbedding.embedInstance] <;> rfl

theorem pure_world (s : InstanceState.State d p n) (v : AuthorityView a) :
 world (MixedCatalogEmbedding.state s) v = WorldProjection.world s v := by
 simp only [world,WorldProjection.world,support,WorldProjection.support,pure_record,pure_form,pure_eligible]
 rfl

-- No independently supplied registry: the exact immutable shared catalog is the
-- ONLY source of body lookup. This still does not implement source issue, resources
-- or authentic physical transport. Field coordinates/labels must match actual store.
def serve (ops : FallibleFlow.Arithmetic) (s : State d p n) (v : AuthorityView a)
 (owner : OwnerEffectService.Owner (size a p n)) (pending : OwnerEffectService.Pending (size a p n)) :=
 match decode pending.request.operation.key with
 | .operation key place =>
   match operationDefinition s key with
   | .pure _ => (owner,OwnerEffectService.Outcome.refused .code)
   | .owner _ => OwnerEffectService.serve ops (world s v) (registry s) owner pending
 | _ => (owner,OwnerEffectService.Outcome.refused .code)

theorem committed_catalog (committed : (serve ops s v owner pending).2 = .committed out) :
 ∃ key place definition,
 decode pending.request.operation.key = .operation key place ∧
 operationDefinition s key = .owner definition ∧ pending.body = definition.body ∧
 OwnerEffectService.Commits ops (world s v) (registry s) owner pending out := by
 unfold serve at committed
 cases slot : decode pending.request.operation.key with
 | member key => simp [slot] at committed
 | locus place => simp [slot] at committed
 | moduleSlot key => simp [slot] at committed
 | operation key place =>
   cases kind : operationDefinition s key with
   | pure _ => simp [slot,kind] at committed
   | owner definition =>
     simp only [slot,kind] at committed
     have meaning := OwnerEffectService.committed_exact.mp committed
     have code := meaning.2.1
     have encoded : pending.request.operation.key = encode (.operation key place) := by
       have eq := congrArg encode slot
       simpa [encode_decode] using eq
     have matchBody : pending.body = definition.body := by
       change registry s ((world s v).records pending.request.operation.key).code = some pending.body at code
       simp only [encoded,world,decode_encode] at code
       simp only [registry,lookup_operation,kind,Option.map_some,Option.some.injEq] at code
       exact code.symm
     exact ⟨key,place,definition,rfl,kind,matchBody,meaning⟩

theorem pure_operation_no_write (slot : decode pending.request.operation.key = .operation key place)
 (kind : operationDefinition s key = .pure definition) :
 serve ops s v owner pending = (owner,.refused .code) := by simp [serve,slot,kind]

#print axioms lookup_operation
#print axioms pure_world
#print axioms committed_catalog
#print axioms pure_operation_no_write
end MirroreaProofFirst.MixedCatalogService
