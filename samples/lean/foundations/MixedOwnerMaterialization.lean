import MixedNamedOwnerSourceEvidence
import MixedCatalogService
namespace MirroreaProofFirst.MixedOwnerMaterialization
open OwnerEffectService MixedOperationDefinitions

-- Current metadata is a separate owner-side input, not inferred from the body
-- or from a transport identity. A physical table/lock must implement this cut.
-- No statement here authenticates that input or its issuer.
def FieldsAt (d : OwnerDefinition) (current : Nat → Option (Nat × Nat)) : Prop :=
 ∀ key info, (key,info) ∈ d.contract.fields → current key = some info

def fieldsCheck (d : OwnerDefinition) (current : Nat → Option (Nat × Nat)) : Bool :=
 d.contract.fields.all fun (key,info) => decide (current key = some info)

theorem fields_exact : fieldsCheck d current = true ↔ FieldsAt d current := by
 simp [fieldsCheck,FieldsAt,List.all_eq_true]

def IntegerArgument : CurrentUse.Scalar → Prop
 | .integer v => OwnerNumericBoundary.Range 63 v
 | .boolean _ => False

def argumentCheck : CurrentUse.Scalar → Bool
 | .integer v => decide (OwnerNumericBoundary.Range 63 v)
 | .boolean _ => false

theorem argument_exact : argumentCheck arg = true ↔ IntegerArgument arg := by
 cases arg <;> simp [argumentCheck,IntegerArgument,OwnerNumericBoundary.Range]

def Payload (d : OwnerDefinition) (e : Pending n) : Prop :=
 e.body = d.body ∧ e.request.arguments.length = d.contract.arguments.length ∧
 ∀ arg ∈ e.request.arguments, IntegerArgument arg

def payloadCheck (d : OwnerDefinition) (e : Pending n) : Bool :=
 decide (e.body = d.body ∧ e.request.arguments.length = d.contract.arguments.length) &&
 e.request.arguments.all argumentCheck

theorem payload_exact : payloadCheck d e = true ↔ Payload d e := by
 simp [payloadCheck,Payload,List.all_eq_true,argument_exact,and_assoc]

def Ready (d : OwnerDefinition) (current : Nat → Option (Nat × Nat)) (s : Owner n) (e : Pending n) : Prop :=
 FieldsAt d current ∧ (∃ value, s.store d.body.target = some value) ∧ Payload d e

def readyCheck (d : OwnerDefinition) (current : Nat → Option (Nat × Nat)) (s : Owner n) (e : Pending n) : Bool :=
 fieldsCheck d current && (s.store d.body.target).isSome && payloadCheck d e

theorem ready_exact : readyCheck d current s e = true ↔ Ready d current s e := by
 cases hx : s.store d.body.target <;> simp [readyCheck,Ready,fields_exact,payload_exact,and_assoc,hx]

-- A refused boundary returns the exact owner, and no synthetic write/receipt.
-- This research wrapper retains Option inside a state/result pair; failure can
-- never erase an earlier committed state. It is not yet a shared-queue executor.
def serve (ops : FallibleFlow.Arithmetic) (catalog : MixedInstanceState.State d p m)
 (view : WorldProjection.AuthorityView a) (current : Nat → Option (Nat × Nat))
 (s : Owner (WorldProjection.size a p m)) (e : Pending (WorldProjection.size a p m)) :
 Owner (WorldProjection.size a p m) × Option (Outcome (WorldProjection.size a p m)) :=
 match WorldProjection.decode e.request.operation.key with
 | .operation key _ =>
   match MixedCatalogService.operationDefinition catalog key with
   | .owner definition =>
     if readyCheck definition current s e then
       let result := MixedCatalogService.serve ops catalog view s e
       (result.1,some result.2)
     else (s,none)
   | .pure _ => (s,none)
 | _ => (s,none)

def Admitted (catalog : MixedInstanceState.State d p m)
 (current : Nat → Option (Nat × Nat)) (s : Owner (WorldProjection.size a p m))
 (e : Pending (WorldProjection.size a p m)) : Prop :=
 ∃ key place definition, WorldProjection.decode e.request.operation.key = .operation key place ∧
 MixedCatalogService.operationDefinition catalog key = .owner definition ∧ Ready definition current s e

theorem serve_admission : (serve ops catalog view current s e).2 = some result ↔
 Admitted catalog current s e ∧ (MixedCatalogService.serve ops catalog view s e).2 = result := by
 unfold serve Admitted
 cases slot : WorldProjection.decode e.request.operation.key <;> simp only [slot]
 all_goals try simp
 rename_i key place
 cases kind : MixedCatalogService.operationDefinition catalog key <;> simp only [kind]
 · simp
 · rename_i definition
   by_cases ready : Ready definition current s e
   · simp [ready_exact.mpr ready,slot,kind,ready]
   · have no : readyCheck definition current s e = false := by
       cases h : readyCheck definition current s e <;> simp_all [ready_exact]
     simp [no,slot,kind,ready]

theorem admitted_state (accepted : (serve ops catalog view current s e).2 = some result) :
 (serve ops catalog view current s e).1 = (MixedCatalogService.serve ops catalog view s e).1 := by
 unfold serve at *
 split at accepted
 · split at accepted
   · split at accepted
     · simp_all
     · cases accepted
   · cases accepted
 · cases accepted

theorem rejected_retains (rejected : (serve ops catalog view current s e).2 = none) :
 (serve ops catalog view current s e).1 = s := by
 unfold serve at *
 split at rejected
 · split at rejected
   · split at rejected
     · cases rejected
     · simp_all
   · rfl
 · rfl

theorem committed_existing_target
 (accepted : (serve ops catalog view current s e).2 = some (.committed out)) :
 ∃ old, s.store e.body.target = some old ∧ out.oldValue = some old := by
 obtain ⟨⟨key,place,definition,slot,kind,ready⟩,committed⟩ := serve_admission.mp accepted
 -- Catalog service already provides its independent CurrentUse/code/evaluation
 -- judgment; exact body in Payload ties current materialization to the request.
 obtain ⟨k,l,op,hs,hk,hbody,commits⟩ := MixedCatalogService.committed_catalog committed
 obtain ⟨fields,⟨old,present⟩,body,args⟩ := ready
 refine ⟨old,by simpa [body] using present,?_⟩
 have oldAt := congrArg Write.oldValue commits.2.2.2
 simpa [body,present] using oldAt

-- A source capture payload has exact arity/labels and is never filled by zero.
def sourceArguments (plan : MixedNamedOwnerSource.Plan) : List CurrentUse.Scalar :=
 plan.captures.map fun row => .integer row.2.value

def sourcePayloadCheck (plan : MixedNamedOwnerSource.Plan) : Bool :=
 plan.captures.all fun row => decide (OwnerNumericBoundary.Range 63 row.2.value)

theorem source_arguments_range (checked : sourcePayloadCheck plan = true) :
 ∀ arg ∈ sourceArguments plan, IntegerArgument arg := by
 intro arg member
 obtain ⟨row,mem,rfl⟩ := List.mem_map.mp member
 change OwnerNumericBoundary.Range 63 row.2.value
 exact of_decide_eq_true (List.all_eq_true.mp checked row mem)

theorem payload_arguments_bounded (valid : Payload d e) :
 OwnerNumericBoundary.Bounded 63 (OwnerEffectService.args e) := by
 intro j value found
 unfold OwnerEffectService.args at found
 cases item : e.request.arguments[j]? with
 | none => simp [item] at found
 | some arg =>
   have bounded := valid.2.2 arg (List.mem_of_getElem? item)
   cases arg with
   | boolean v => exact False.elim bounded
   | integer v =>
     have same : v = value := by simpa [item] using found
     simpa [same] using bounded

theorem committed_bounded (catalogValid : MixedInstanceState.Valid catalog)
 (stateBound : OwnerNumericBoundary.Bounded 63 s.store)
 (accepted : (serve (FallibleFlow.signed 63) catalog view current s e).2 = some (.committed out)) :
 OwnerNumericBoundary.Range 63 out.value ∧
 OwnerNumericBoundary.Bounded 63 (serve (FallibleFlow.signed 63) catalog view current s e).1.store := by
 obtain ⟨⟨key,place,definition,slot,kind,ready⟩,committed⟩ := serve_admission.mp accepted
 obtain ⟨k,l,op,hs,hk,body,meaning⟩ := MixedCatalogService.committed_catalog committed
 have valid := catalogValid.definitions (catalog.instances k).definition
 change MixedOperationDefinitions.Satisfies (MixedCatalogService.operationDefinition catalog k) at valid
 rw [hk] at valid
 have literals : OwnerNumericBoundary.Literals 63 e.body.tree := by
   rw [body]
   exact valid.2.2.2.1
 have range := OwnerNumericBoundary.evaluate_bounded s.store (OwnerEffectService.args e)
   e.body.tree stateBound (payload_arguments_bounded ready.2.2) literals meaning.2.2.1
 refine ⟨range,?_⟩
 rw [admitted_state accepted]
 have oldCommit := OwnerEffectService.committed_exact.mpr meaning
 have stores := (OwnerEffectService.committed_state oldCommit).1
 have same : MixedCatalogService.serve (FallibleFlow.signed 63) catalog view s e =
   OwnerEffectService.serve (FallibleFlow.signed 63) (MixedCatalogService.world catalog view)
     (MixedCatalogService.registry catalog) s e := by simp [MixedCatalogService.serve,hs,hk]
 rw [same,stores]
 exact OwnerNumericBoundary.put_bounded stateBound range

#print axioms payload_arguments_bounded
#print axioms committed_bounded

#print axioms ready_exact
#print axioms serve_admission
#print axioms admitted_state
#print axioms rejected_retains
#print axioms committed_existing_target
#print axioms source_arguments_range
end MirroreaProofFirst.MixedOwnerMaterialization
