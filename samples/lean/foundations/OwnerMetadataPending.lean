import OwnerMetadataManagement
import MixedOwnerAttemptQueue
namespace MirroreaProofFirst.OwnerMetadataPending
open OwnerStructuredKeys OwnerSourceContext OwnerMetadataRegistry

-- This retained envelope is a compiler/service candidate over the EXISTING
-- saved request. It is not a credential, independent runtime, wire or image.
-- Metadata/module anchors exclude serials so unrelated additions remain usable.
structure Anchor where
 instanceId : Nat
 ownerName : String
 ownerKey : Nat
 ownerIdentity : CurrentUse.RecordIdentity
 moduleKey : Nat
 moduleIdentity : CurrentUse.RecordIdentity
 code : Nat
 contract : Nat
 schema : Schema
 deriving DecidableEq

def anchor (r : Registry) : Anchor :=
 ⟨r.instanceId,r.ownerName,r.ownerKey,r.ownerIdentity,r.moduleKey,r.moduleIdentity,
 r.code,r.contract,r.schema⟩

abbrev Row := Nat × Binding

def row (addresses : Addresses) (r : Registry) (owner : Nat)
 (field : Nat × (Nat × Nat)) : Option Row := do
 let key ← decode addresses field.1
 let metadata ← current r key
 let bound : Binding := ⟨key,metadata⟩
 if OwnerSourceContext.usable r.schema (current r) bound &&
   decide (field.2 = (owner,metadata.label)) then some (field.1,bound) else none

def RowBinds (addresses : Addresses) (r : Registry) (owner : Nat)
 (field : Nat × (Nat × Nat)) (out : Row) : Prop :=
 out.1 = field.1 ∧ decode addresses field.1 = some out.2.key ∧
 OwnerSourceContext.Usable r.schema (current r) out.2 ∧
 field.2 = (owner,out.2.metadata.label)

theorem row_exact : row addresses r owner field = some out ↔ RowBinds addresses r owner field out := by
 constructor
 · intro accepted
   unfold row at accepted
   cases decoded : decode addresses field.1 with
   | none => simp [decoded] at accepted
   | some key =>
     cases found : current r key with
     | none => simp [decoded,found] at accepted
     | some metadata =>
       simp only [decoded,found,Option.bind_eq_bind,Option.bind_some] at accepted
       split at accepted
       · rename_i checked
         cases accepted
         have facts : OwnerSourceContext.Usable r.schema (current r) ⟨key,metadata⟩ ∧
           field.2 = (owner,metadata.label) := by
           simpa only [Bool.and_eq_true,OwnerSourceContext.usable_exact,decide_eq_true_eq] using checked
         exact ⟨rfl,decoded,facts⟩
       · cases accepted
 · rintro ⟨same,decoded,usable,info⟩
   have found := usable.2
   cases out with
   | mk index bound =>
     cases bound with
     | mk key metadata =>
       simp_all [row,OwnerSourceContext.usable_exact.mpr usable]

def rows (addresses : Addresses) (r : Registry) (owner : Nat) :
 List (Nat × (Nat × Nat)) → Option (List Row)
 | [] => some []
 | field::rest => do
   let first ← row addresses r owner field
   let later ← rows addresses r owner rest
   return first::later

inductive RowsBind (addresses : Addresses) (r : Registry) (owner : Nat) :
 List (Nat × (Nat × Nat)) → List Row → Prop where
 | nil : RowsBind addresses r owner [] []
 | cons : RowBinds addresses r owner field first → RowsBind addresses r owner rest later →
   RowsBind addresses r owner (field::rest) (first::later)

theorem rows_sound (accepted : rows addresses r owner fields = some out) :
 RowsBind addresses r owner fields out := by
 induction fields generalizing out with
 | nil => cases accepted; exact .nil
 | cons field rest ih =>
   cases first : row addresses r owner field with
   | none => simp [rows,first] at accepted
   | some bound =>
     cases later : rows addresses r owner rest with
     | none => simp [rows,first,later] at accepted
     | some tail =>
       simp only [rows,first,later,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
       subst out
       exact .cons (row_exact.mp first) (ih later)

theorem rows_complete (meaning : RowsBind addresses r owner fields out) :
 rows addresses r owner fields = some out := by
 induction meaning with
 | nil => rfl
 | cons first _ ih => simp [rows,row_exact.mpr first,ih]

theorem rows_exact : rows addresses r owner fields = some out ↔ RowsBind addresses r owner fields out :=
 ⟨rows_sound,rows_complete⟩

def currentFields (addresses : Addresses) (r : Registry) (owner : Nat) (index : Nat) : Option (Nat × Nat) := do
 let key ← decode addresses index
 let metadata ← current r key
 return (owner,metadata.label)

theorem rows_fields (meaning : RowsBind addresses r owner fields out) :
 ∀ field ∈ fields, currentFields addresses r owner field.1 = some field.2 := by
 induction meaning with
 | nil => simp
 | cons first _ ih =>
   intro field present
   rcases List.mem_cons.mp present with rfl | later
   · simp [currentFields,first.2.1,first.2.2.1.2,first.2.2.2]
   · exact ih field later

theorem rows_current (meaning : RowsBind addresses r owner fields out) :
 ∀ bound ∈ out, decode addresses bound.1 = some bound.2.key ∧
 OwnerSourceContext.Usable r.schema (current r) bound.2 := by
 induction meaning with
 | nil => simp
 | cons first _ ih =>
   intro bound present
   rcases List.mem_cons.mp present with rfl | later
   · exact ⟨first.1 ▸ first.2.1,first.2.2.1⟩
   · exact ih bound later

structure Packet where
 saved : OwnerSavedPending.Saved
 anchor : Anchor
 owner : Nat
 fields : List Row
 deriving DecidableEq

def bind (addresses : Addresses) (s : OwnerMetadataManagement.State p a) (place : Fin p)
 (definition : MixedOperationDefinitions.OwnerDefinition) (saved : OwnerSavedPending.Saved) : Option Packet := do
 if !OwnerMetadataManagement.attachmentCheck s place || saved.body != definition.body then none else do
 let fields ← rows addresses s.metadata place.val definition.contract.fields
 return ⟨saved,anchor s.metadata,place.val,fields⟩

def Binds (addresses : Addresses) (s : OwnerMetadataManagement.State p a) (place : Fin p)
 (definition : MixedOperationDefinitions.OwnerDefinition) (saved : OwnerSavedPending.Saved) (packet : Packet) : Prop :=
 OwnerMetadataManagement.Attached s place ∧ saved.body = definition.body ∧
 ∃ fields, RowsBind addresses s.metadata place.val definition.contract.fields fields ∧
 packet = ⟨saved,anchor s.metadata,place.val,fields⟩

theorem bind_exact : bind addresses s place definition saved = some packet ↔
 Binds addresses s place definition saved packet := by
 constructor
 · intro accepted
   unfold bind at accepted
   split at accepted
   · cases accepted
   · rename_i gate
     have facts : OwnerMetadataManagement.Attached s place ∧ saved.body = definition.body := by
       simpa [OwnerMetadataManagement.attachment_exact] using gate
     cases bound : rows addresses s.metadata place.val definition.contract.fields with
     | none => simp [bound] at accepted
     | some fields =>
       simp only [bound,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
       exact ⟨facts.1,facts.2,fields,rows_sound bound,accepted.symm⟩
 · rintro ⟨attached,body,fields,meaning,rfl⟩
   simp [bind,OwnerMetadataManagement.attachment_exact.mpr attached,body,rows_complete meaning]

-- Every retained field is rechecked at use. Exact saved equality binds the
-- fields to the actual queued request, including source origin/body/arguments.
def check (addresses : Addresses) (s : OwnerMetadataManagement.State p a) (place : Fin p)
 (saved : OwnerSavedPending.Saved) (packet : Packet) : Bool :=
 OwnerMetadataManagement.attachmentCheck s place &&
 decide (packet.saved = saved ∧ packet.anchor = anchor s.metadata ∧ packet.owner = place.val) &&
 packet.fields.all fun bound => decide (decode addresses bound.1 = some bound.2.key) &&
   OwnerSourceContext.usable s.metadata.schema (current s.metadata) bound.2

def Current (addresses : Addresses) (s : OwnerMetadataManagement.State p a) (place : Fin p)
 (saved : OwnerSavedPending.Saved) (packet : Packet) : Prop :=
 OwnerMetadataManagement.Attached s place ∧
 packet.saved = saved ∧ packet.anchor = anchor s.metadata ∧ packet.owner = place.val ∧
 ∀ bound ∈ packet.fields, decode addresses bound.1 = some bound.2.key ∧
   OwnerSourceContext.Usable s.metadata.schema (current s.metadata) bound.2

theorem check_exact : check addresses s place saved packet = true ↔ Current addresses s place saved packet := by
 simp [check,Current,OwnerMetadataManagement.attachment_exact,List.all_eq_true,
   OwnerSourceContext.usable_exact,and_assoc]

theorem fresh_check (accepted : bind addresses s place definition saved = some packet) :
 check addresses s place saved packet = true := by
 obtain ⟨attached,_,fields,meaning,rfl⟩ := bind_exact.mp accepted
 exact check_exact.mpr ⟨attached,rfl,rfl,rfl,rows_current meaning⟩

theorem bound_fields (accepted : bind addresses s place definition saved = some packet) :
 MixedOwnerMaterialization.FieldsAt definition (currentFields addresses s.metadata place.val) := by
 obtain ⟨_,_,_,meaning,_⟩ := bind_exact.mp accepted
 intro key info present
 exact rows_fields meaning (key,info) present

theorem changed_field_refuses (present : bound ∈ packet.fields)
 (changed : current s.metadata bound.2.key ≠ some bound.2.metadata) :
 check addresses s place saved packet = false := by
 cases checked : check addresses s place saved packet with
 | false => rfl
 | true => exact False.elim (changed (((check_exact.mp checked).2.2.2.2 bound present).2.2))

theorem substituted_payload_refuses (different : packet.saved ≠ saved) :
 check addresses s place saved packet = false := by simp [check,different]

-- Gate the existing owner queue directly. No reconstructed expected write or
-- synthetic result is supplied. Raw Packet construction is not an admitted
-- enclosing entry: source/transfer provenance must establish bind origin.
def advance (ops : FallibleFlow.Arithmetic) (addresses : Addresses)
 (s : OwnerMetadataManagement.State p a) (place : Fin p) (packet : Packet)
 (owner : MixedOwnerAttemptQueue.State) :
 MixedOwnerAttemptQueue.State × Option MixedOwnerAttemptQueue.Result :=
 match owner.queued with
 | none => (owner,none)
 | some saved =>
   if check addresses s place saved packet then
     MixedOwnerAttemptQueue.advance ops s.management.configuration.state s.management.view
       (currentFields addresses s.metadata place.val) owner
   else (owner,none)

theorem advance_parts (completed : (advance ops addresses s place packet owner).2 = some result) :
 ∃ saved, owner.queued = some saved ∧ Current addresses s place saved packet ∧
 advance ops addresses s place packet owner =
 MixedOwnerAttemptQueue.advance ops s.management.configuration.state s.management.view
   (currentFields addresses s.metadata place.val) owner := by
 unfold advance at completed ⊢
 cases queued : owner.queued with
 | none => simp [queued] at completed
 | some saved =>
   simp only [queued] at completed ⊢
   split at completed
   · rename_i checked
     exact ⟨saved,rfl,check_exact.mp checked,by simp [checked]⟩
   · cases completed

theorem advance_refused_retains
 (queued : owner.queued = some saved) (refused : check addresses s place saved packet = false) :
 advance ops addresses s place packet owner = (owner,none) := by simp [advance,queued,refused]

theorem advance_preserves (valid : MixedOwnerAttemptQueue.Invariant owner) :
 MixedOwnerAttemptQueue.Invariant (advance ops addresses s place packet owner).1 := by
 unfold advance
 split
 · exact valid
 · split
   · exact MixedOwnerAttemptQueue.advance_preserves valid
   · exact valid

-- Relative completeness keeps the independent current binding, payload
-- materialization, existing service admission and declarative write semantics.
-- Neither successful gate evaluation nor a successful write is assumed.
theorem advance_complete {p a : Nat} {s : OwnerMetadataManagement.State p a} {place : Fin p}
 {pending : OwnerEffectService.Pending (WorldProjection.size a p s.management.configuration.count)}
 {written : OwnerEffectService.Write (WorldProjection.size a p s.management.configuration.count)}
 (queued : owner.queued = some saved)
 (metadata : Current addresses s place saved packet)
 (materialized : OwnerSavedPending.materialize (WorldProjection.size a p s.management.configuration.count) saved = some pending)
 (admitted : MixedOwnerMaterialization.Admitted s.management.configuration.state
   (currentFields addresses s.metadata place.val) ⟨owner.store,[]⟩ pending)
 (meaning : OwnerEffectService.Commits ops
   (MixedCatalogService.world s.management.configuration.state s.management.view)
   (MixedCatalogService.registry s.management.configuration.state) ⟨owner.store,[]⟩ pending written) :
 (advance ops addresses s place packet owner).2 = some (.committed (MixedOwnerAttemptQueue.record written)) := by
 simp only [advance,queued,check_exact.mpr metadata,ite_true]
 exact MixedOwnerAttemptQueue.committed_complete queued materialized admitted meaning

#print axioms bind_exact
#print axioms fresh_check
#print axioms bound_fields
#print axioms advance_parts
#print axioms advance_refused_retains
#print axioms advance_preserves
#print axioms advance_complete

#print axioms row_exact
#print axioms rows_exact
#print axioms rows_fields
#print axioms rows_current
#print axioms check_exact
#print axioms changed_field_refuses
#print axioms substituted_payload_refuses
end MirroreaProofFirst.OwnerMetadataPending
