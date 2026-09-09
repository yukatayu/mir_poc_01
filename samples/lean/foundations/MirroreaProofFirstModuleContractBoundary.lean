import MirroreaProofFirstCurrentUse
import MirroreaProofFirstFunctionContractBridge

namespace MirroreaProofFirst.ModuleContractBoundary
open LocalContract ContractExport

-- Reference call boundary, not a source module syntax or a registry update protocol.
structure Descriptor where
 codeId : Nat
 contractId : Nat
 arity : Nat
 profile : Profile
 theoryVersion : Nat
 contractVersion : Nat
 code : Term
 assumptions : List Term
 deriving DecidableEq, Repr

abbrev Registry (n : Nat) := Fin n → Option Descriptor

-- Full stamps supplement Context: non-operation revisions are not all in Context.
structure CallProof (n : Nat) where
 context : CurrentUse.Context
 moduleStamp : CurrentUse.Handle n
 operationStamp : CurrentUse.Handle n
 payload : Envelope

def expected (s : CurrentUse.World n) (u : CurrentUse.UseRequest n)
 (d : Descriptor) (args : List Int) : Binding :=
 {profile := d.profile, theoryVersion := d.theoryVersion,
  contractVersion := d.contractVersion, instanceId := s.instanceId,
  generation := s.generation, principal := u.principal, request := u.request,
  code := d.code, arguments := args, assumptions := d.assumptions}

structure Linked (s : CurrentUse.World n) (u : CurrentUse.UseRequest n)
 (d : Descriptor) (args : List Int) (p : CallProof n) : Prop where
 context : p.context = CurrentUse.currentContext s u
 moduleStamp : p.moduleStamp = u.moduleHandle
 operationStamp : p.operationStamp = u.operation
 code : d.codeId = (s.records u.operation.key).code
 contract : d.contractId = (s.records u.operation.key).contract
 arity : args.length = d.arity
 arguments : u.arguments = args.map CurrentUse.Scalar.integer

def linkCheck (s : CurrentUse.World n) (u : CurrentUse.UseRequest n)
 (d : Descriptor) (args : List Int) (p : CallProof n) : Bool :=
 decide (p.context = CurrentUse.currentContext s u ∧
  p.moduleStamp = u.moduleHandle ∧ p.operationStamp = u.operation ∧
  d.codeId = (s.records u.operation.key).code ∧
  d.contractId = (s.records u.operation.key).contract ∧
  args.length = d.arity ∧ u.arguments = args.map CurrentUse.Scalar.integer)

theorem link_exact (s : CurrentUse.World n) (u : CurrentUse.UseRequest n)
 (d : Descriptor) (args : List Int) (p : CallProof n) :
 linkCheck s u d args p = true ↔ Linked s u d args p := by
 simp only [linkCheck,decide_eq_true_eq]
 constructor
 · rintro ⟨hc,hm,ho,hcode,hcontract,hn,ha⟩
   exact ⟨hc,hm,ho,hcode,hcontract,hn,ha⟩
 · intro h
   exact ⟨h.context,h.moduleStamp,h.operationStamp,h.code,h.contract,h.arity,h.arguments⟩

def call (s : CurrentUse.World n) (registry : Registry n)
 (u : CurrentUse.UseRequest n) (args : List Int) (auth : CurrentUse.Evidence)
 (p : CallProof n) : Option Nat :=
 match registry u.operation.key with
 | none => none
 | some d =>
   if CurrentUse.checkUse s u auth && linkCheck s u d args p then
     accept (expected s u d args) p.payload
   else none

-- Declarative result contains current-use and a semantic export property, not call=true.
structure Successful (s : CurrentUse.World n) (registry : Registry n)
 (u : CurrentUse.UseRequest n) (args : List Int) (p : CallProof n) (value : Nat) : Prop where
 current : CurrentUse.CurrentUse s u
 descriptor : ∃ d, registry u.operation.key = some d ∧ Linked s u d args p ∧
   PositiveExport (expected s u d args) p.payload value

theorem call_sound {s : CurrentUse.World n} {registry : Registry n}
 {u : CurrentUse.UseRequest n} {args : List Int} {auth : CurrentUse.Evidence}
 {p : CallProof n} {value : Nat} (ok : call s registry u args auth p = some value) :
 Successful s registry u args p value := by
 unfold call at ok
 split at ok
 · contradiction
 · rename_i d hd
   split at ok
   · rename_i h
     have checks : CurrentUse.checkUse s u auth = true ∧ linkCheck s u d args p = true := by
       simpa only [Bool.and_eq_true] using h
     exact ⟨CurrentUse.checkUse_sound s u auth checks.1,
       d,hd,(link_exact s u d args p).mp checks.2,accept_sound ok⟩
   · contradiction

theorem call_checked_complete {s : CurrentUse.World n} {registry : Registry n}
 {u : CurrentUse.UseRequest n} {args : List Int} {p : CallProof n} {d : Descriptor}
 (live : CurrentUse.CurrentUse s u) (lookup : registry u.operation.key = some d)
 (linked : Linked s u d args p) (common : Common (expected s u d args) p.payload)
 (profile : d.profile = .checkedValue) (positive : 0 < p.payload.result) :
 ∃ auth, call s registry u args auth p = some p.payload.result.toNat := by
 obtain ⟨auth,checked⟩ := CurrentUse.checkUse_complete s u live
 have exported := checked_profile_complete common (by simpa [expected] using profile) positive
 exact ⟨auth,by simp [call,lookup,checked,(link_exact s u d args p).mpr linked,exported]⟩

-- Relative completeness for the independently derived symbolic profile also
-- preserves the same current module, typed arguments and authorization context.
theorem call_symbolic_complete {s : CurrentUse.World n} {registry : Registry n}
 {u : CurrentUse.UseRequest n} {args : List Int} {p : CallProof n} {d : Descriptor}
 (live : CurrentUse.CurrentUse s u) (lookup : registry u.operation.key = some d)
 (linked : Linked s u d args p) (common : Common (expected s u d args) p.payload)
 (profile : d.profile = .symbolicNonnegative)
 (code : d.code = positiveTerm p.payload.base)
 (derivation : Nonnegative d.assumptions p.payload.base) :
 ∃ auth c, call s registry u args auth
   {p with payload := {p.payload with certificate := c}} = some p.payload.result.toNat := by
 obtain ⟨auth,checked⟩ := CurrentUse.checkUse_complete s u live
 obtain ⟨c,exported⟩ := symbolic_profile_complete common
   (by simpa [expected] using profile) code derivation
 have linked' : Linked s u d args {p with payload := {p.payload with certificate := c}} :=
   ⟨linked.context,linked.moduleStamp,linked.operationStamp,linked.code,
    linked.contract,linked.arity,linked.arguments⟩
 exact ⟨auth,c,by simp [call,lookup,checked,(link_exact _ _ _ _ _).mpr linked',exported]⟩

theorem no_authority_no_call (s : CurrentUse.World n) (registry : Registry n)
 (u : CurrentUse.UseRequest n) (args : List Int) (auth : CurrentUse.Evidence)
 (p : CallProof n) (empty : s.authority.issued = []) : call s registry u args auth p = none := by
 cases h : call s registry u args auth p with
 | none => rfl
 | some value =>
   have current := (call_sound h).current
   rcases current with ⟨_,_,_,_,_,_,_,allowed⟩
   exact False.elim (CurrentUse.no_claim_no_authorization _ _ _ _ empty allowed)

theorem call_actual_execution {s : CurrentUse.World n} {registry : Registry n}
 {u : CurrentUse.UseRequest n} {args : List Int} {auth : CurrentUse.Evidence}
 {p : CallProof n} {value : Nat} (ok : call s registry u args auth p = some value) :
 ∃ d, registry u.operation.key = some d ∧
   (Int.ofNat value = eval (FunctionContractBridge.argumentInput args) d.code) ∧
   ∃ fuel, PureFunctions.evaluate fuel (FunctionContractBridge.argumentValues args)
     (FunctionContractBridge.lower d.code) = some (.integer p.payload.result) := by
 unfold call at ok
 split at ok
 · contradiction
 · rename_i d hd
   split at ok
   · refine ⟨d,hd,?_,?_⟩
     · exact accepted_integer_length ok
     · exact FunctionContractBridge.accepted_code_executes ok
   · contradiction

theorem changed_context_rejected (s : CurrentUse.World n) (registry : Registry n)
 (u : CurrentUse.UseRequest n) (args : List Int) (auth : CurrentUse.Evidence)
 (p : CallProof n) (different : p.context ≠ CurrentUse.currentContext s u) :
 call s registry u args auth p = none := by
 unfold call; split
 · rfl
 · simp [linkCheck,different]

theorem changed_module_stamp_rejected (s : CurrentUse.World n) (registry : Registry n)
 (u : CurrentUse.UseRequest n) (args : List Int) (auth : CurrentUse.Evidence)
 (p : CallProof n) (different : p.moduleStamp ≠ u.moduleHandle) :
 call s registry u args auth p = none := by
 unfold call; split
 · rfl
 · simp [linkCheck,different]

namespace Controls
open CurrentUse.Controls

def descriptor : Descriptor := ⟨100,200,1,.checkedValue,1,1,
 positiveTerm (.input 0),[]⟩
def registry : Registry 4 := fun i => if i = 3 then some descriptor else none
def proof : CallProof 4 := ⟨CurrentUse.currentContext world request,
 request.moduleHandle,request.operation,
 ⟨expected world request descriptor [41],42,.input 0,.hypothesis (.input 0)⟩⟩
example : call world registry request [41] evidence proof = some 42 := by decide
example : call world registry request [40] evidence proof = none := by decide
example : call world registry request [41,0] evidence proof = none := by decide
example : call world (fun _ => none) request [41] evidence proof = none := by decide
example : call {world with authority := {auth with issued := []}}
 registry request [41] evidence proof = none := by decide
example : call {world with authority := {auth with revoked := [2]}}
 registry request [41] evidence proof = none := by decide
example : call retiredWorld registry request [41] evidence proof = none := by decide
-- Scalar tagging is checked even when fresh auth evidence is supplied for the wrong type.
def boolRequest : CurrentUse.UseRequest 4 := {request with arguments := [.boolean true]}
def boolEvidence : CurrentUse.Evidence := {evidence with context := CurrentUse.currentContext world boolRequest}
example : CurrentUse.checkUse world boolRequest boolEvidence = true := by decide
example : call world registry boolRequest [41] boolEvidence
 {proof with context := CurrentUse.currentContext world boolRequest} = none := by decide
-- No borrowed old proof binding when the operation's actual descriptor changes.
example : call world (fun _ => some {descriptor with code := positiveTerm (.square (.input 0))})
 request [41] evidence proof = none := by decide
def symbolicDescriptor : Descriptor :=
 {descriptor with profile := .symbolicNonnegative, code := positiveTerm (.square (.input 0))}
def symbolicProof : CallProof 4 := {proof with payload :=
 ⟨expected world request symbolicDescriptor [41],1682,.square (.input 0),.square (.input 0)⟩}
example : call world (fun _ => some symbolicDescriptor) request [41] evidence symbolicProof = some 1682 := by decide
example : call world (fun _ => some symbolicDescriptor) request [41] evidence
 {symbolicProof with payload := {symbolicProof.payload with certificate := .integer 0}} = none := by decide
end Controls
#print axioms link_exact
#print axioms call_sound
#print axioms call_checked_complete
#print axioms call_symbolic_complete
#print axioms no_authority_no_call
#print axioms call_actual_execution
#print axioms changed_context_rejected
#print axioms changed_module_stamp_rejected
end MirroreaProofFirst.ModuleContractBoundary

-- Later unreviewed descriptor/catalog extension; excluded from frozen Oracle input.
namespace MirroreaProofFirst.ModuleContractBoundary
-- Descriptor uniqueness for one fixed use and proof, not authentic registry issuance.
theorem linked_descriptor_unique {s : CurrentUse.World n} {u : CurrentUse.UseRequest n}
 {args : List Int} {p : CallProof n} {a b : Descriptor}
 (la : Linked s u a args p) (lb : Linked s u b args p)
 (binding : expected s u a args = expected s u b args) : a = b := by
 have hc := la.code.trans lb.code.symm
 have ht := la.contract.trans lb.contract.symm
 have hn := la.arity.symm.trans lb.arity
 cases a; cases b
 simp_all only [expected,ContractExport.Binding.mk.injEq]

-- A previously successful proof cannot be rebound to another descriptor even if
-- nominal IDs and the actual result happen to remain equal. A freshly constructed
-- proof is a different question; no registry mutation authorization is inferred.
theorem changed_descriptor_rejected {s : CurrentUse.World n} {oldRegistry newRegistry : Registry n}
 {u : CurrentUse.UseRequest n} {args : List Int} {auth newAuth : CurrentUse.Evidence}
 {p : CallProof n} {value : Nat} {old new : Descriptor}
 (accepted : call s oldRegistry u args auth p = some value)
 (oldLookup : oldRegistry u.operation.key = some old)
 (newLookup : newRegistry u.operation.key = some new) (different : old ≠ new) :
 call s newRegistry u args newAuth p = none := by
 cases h : call s newRegistry u args newAuth p with
 | none => rfl
 | some v =>
   obtain ⟨a,ha,la,ea⟩ := (call_sound accepted).descriptor
   obtain ⟨b,hb,lb,eb⟩ := (call_sound h).descriptor
   have ae : a = old := Option.some.inj (ha.symm.trans oldLookup)
   have be : b = new := Option.some.inj (hb.symm.trans newLookup)
   subst a; subst b
   exact False.elim (different (linked_descriptor_unique la lb
     (ea.common.binding.symm.trans eb.common.binding)))

-- A finite-capacity immutable descriptor catalog is a reversible candidate for
-- the registry integrity premise. It is not an authenticated head or installer.
abbrev Catalog := Nat → Option Descriptor
def CatalogWF (capacity : Nat) (c : Catalog) : Prop :=
 ∀ key d, c key = some d → key < capacity ∧ d.codeId = key

def InsertAllowed (capacity : Nat) (c : Catalog) (d : Descriptor) : Prop :=
 d.codeId < capacity ∧ c d.codeId = none

def insert (capacity : Nat) (c : Catalog) (d : Descriptor) : Option Catalog :=
 if d.codeId < capacity ∧ c d.codeId = none then
   some (fun key => if key = d.codeId then some d else c key)
 else none

def Extends (c c' : Catalog) : Prop := ∀ key d, c key = some d → c' key = some d

theorem insert_exists (capacity : Nat) (c : Catalog) (d : Descriptor) :
 (∃ c', insert capacity c d = some c') ↔ InsertAllowed capacity c d := by
 simp only [insert,InsertAllowed]; split <;> simp_all

theorem insert_extends {capacity : Nat} {c c' : Catalog} {d : Descriptor}
 (ok : insert capacity c d = some c') : Extends c c' := by
 unfold insert at ok; split at ok
 · rename_i h; cases ok
   intro key old present
   have different : key ≠ d.codeId := by
     intro eq; subst key; rw [h.2] at present; contradiction
   simp [different,present]
 · contradiction

theorem insert_wf {capacity : Nat} {c c' : Catalog} {d : Descriptor}
 (wf : CatalogWF capacity c) (ok : insert capacity c d = some c') : CatalogWF capacity c' := by
 unfold insert at ok; split at ok
 · rename_i h; cases ok
   intro key found present; dsimp at present; split at present
   · rename_i eq; cases present; exact ⟨eq ▸ h.1,eq.symm⟩
   · exact wf key found present
 · contradiction

def runInserts (capacity : Nat) : Catalog → List Descriptor → Catalog
 | c,[] => c
 | c,d::rest => runInserts capacity ((insert capacity c d).getD c) rest

theorem inserts_preserve (capacity : Nat) (c : Catalog) (ds : List Descriptor)
 (wf : CatalogWF capacity c) :
 CatalogWF capacity (runInserts capacity c ds) ∧ Extends c (runInserts capacity c ds) := by
 induction ds generalizing c with
 | nil => exact ⟨wf,fun _ _ h => h⟩
 | cons d ds ih =>
   cases step : insert capacity c d with
   | none => simpa [runInserts,step] using ih c wf
   | some c' =>
     obtain ⟨wf',extension⟩ := ih c' (insert_wf wf step)
     refine ⟨by simpa [runInserts,step] using wf',?_⟩
     intro key old present
     simpa [runInserts,step] using extension key old (insert_extends step key old present)

-- This selected entry checks the immutable catalog entry; a fresh envelope
-- cannot substitute code under the same nominal identity at this boundary.
-- Raw call and existing handle consumers remain the explicitly weaker path.
def catalogCall (s : CurrentUse.World n) (registry : Registry n) (catalog : Catalog)
 (u : CurrentUse.UseRequest n) (args : List Int) (auth : CurrentUse.Evidence) (p : CallProof n) : Option Nat :=
 match registry u.operation.key with
 | none => none
 | some d => if catalog d.codeId = some d then call s registry u args auth p else none

theorem catalog_agrees {s : CurrentUse.World n} {registry : Registry n} {catalog : Catalog}
 {u : CurrentUse.UseRequest n} {args : List Int} {auth : CurrentUse.Evidence}
 {p : CallProof n} {d : Descriptor} (lookup : registry u.operation.key = some d)
 (registered : catalog d.codeId = some d) :
 catalogCall s registry catalog u args auth p = call s registry u args auth p := by
 simp [catalogCall,lookup,registered]

theorem catalog_sound {s : CurrentUse.World n} {registry : Registry n} {catalog : Catalog}
 {u : CurrentUse.UseRequest n} {args : List Int} {auth : CurrentUse.Evidence}
 {p : CallProof n} {value : Nat} (ok : catalogCall s registry catalog u args auth p = some value) :
 Successful s registry u args p value ∧
 ∃ d, registry u.operation.key = some d ∧ catalog (s.records u.operation.key).code = some d := by
 unfold catalogCall at ok; split at ok
 · contradiction
 · rename_i d hd; split at ok
   · rename_i hc
     have success := call_sound ok
     obtain ⟨found,hfound,linked,exported⟩ := success.descriptor
     have eq : found = d := Option.some.inj (hfound.symm.trans hd)
     subst found
     exact ⟨success,d,hd,linked.code ▸ hc⟩
   · contradiction

theorem fresh_proof_cannot_rebind {s : CurrentUse.World n} {registry : Registry n} {catalog : Catalog}
 {u : CurrentUse.UseRequest n} {args : List Int} {auth : CurrentUse.Evidence}
 {p : CallProof n} {old new : Descriptor}
 (oldBinding : catalog (s.records u.operation.key).code = some old)
 (lookup : registry u.operation.key = some new) (different : old ≠ new) :
 catalogCall s registry catalog u args auth p = none := by
 cases h : catalogCall s registry catalog u args auth p with
 | none => rfl
 | some value =>
   obtain ⟨_,d,hd,hc⟩ := catalog_sound h
   have eq : d = new := Option.some.inj (hd.symm.trans lookup)
   subst d
   exact False.elim (different (Option.some.inj (oldBinding.symm.trans hc)))

namespace DescriptorControl
open Controls CurrentUse.Controls
-- Different code with the same value at this invocation still cannot borrow the
-- old proof. Refreshing the envelope under the same trusted registry/auth inputs
-- is deliberately not prohibited by this limited boundary model.
def sameValue : Descriptor := {descriptor with code := .add (.integer 1) (.input 0)}
example : sameValue ≠ descriptor := by decide
example : LocalContract.eval (FunctionContractBridge.argumentInput [41]) sameValue.code = 42 := by decide
example : call world (fun _ => some sameValue) request [41] evidence proof = none := by decide
def refreshed : CallProof 4 := {proof with payload :=
 {proof.payload with binding := expected world request sameValue [41]}}
example : call world (fun _ => some sameValue) request [41] evidence refreshed = some 42 := by decide
def catalog : Catalog := fun id => if id = 100 then some descriptor else none
example : catalogCall world registry catalog request [41] evidence proof = some 42 := by decide
example : catalogCall world (fun _ => some sameValue) catalog request [41] evidence refreshed = none := by decide
example : insert 101 catalog sameValue = none := by
 simp [insert,catalog,sameValue,descriptor]
example : (insert 101 (fun _ => none) descriptor).isSome = true := by decide
example : (insert 100 (fun _ => none) descriptor).isSome = false := by decide
end DescriptorControl
#print axioms insert_exists
#print axioms insert_extends
#print axioms insert_wf
#print axioms inserts_preserve
#print axioms catalog_agrees
#print axioms catalog_sound
#print axioms fresh_proof_cannot_rebind
#print axioms linked_descriptor_unique
#print axioms changed_descriptor_rejected
end MirroreaProofFirst.ModuleContractBoundary

-- Later unreviewed downstream-addition and stale-catalog consequences.
namespace MirroreaProofFirst.ModuleContractBoundary
-- Safe downstream catalog addition for a fixed current World/use/registry.
-- This does not freeze unrelated runtime state or assert concurrent atomicity.
theorem catalog_call_preserved {s : CurrentUse.World n} {registry : Registry n}
 {catalog newer : Catalog} {u : CurrentUse.UseRequest n} {args : List Int}
 {auth : CurrentUse.Evidence} {p : CallProof n} {value : Nat}
 (extension : Extends catalog newer)
 (ok : catalogCall s registry catalog u args auth p = some value) :
 catalogCall s registry newer u args auth p = some value := by
 unfold catalogCall at ok ⊢
 split at ok
 · rename_i h; simp_all
 · rename_i d hd
   split at ok
   · rename_i registered
     simpa [hd,extension d.codeId d registered] using ok
   · contradiction

theorem insert_sequence_preserves_call (capacity : Nat) {s : CurrentUse.World n}
 {registry : Registry n} {catalog : Catalog} {u : CurrentUse.UseRequest n}
 {args : List Int} {auth : CurrentUse.Evidence} {p : CallProof n} {value : Nat}
 (ds : List Descriptor) (wf : CatalogWF capacity catalog)
 (ok : catalogCall s registry catalog u args auth p = some value) :
 catalogCall s registry (runInserts capacity catalog ds) u args auth p = some value :=
 catalog_call_preserved (inserts_preserve capacity catalog ds wf).2 ok

-- A catalog image cannot supply current authority/lifetime. Even arbitrary
-- catalog contents do not revive a module rejected by the supplied current World.
theorem no_catalog_revives_module (s : CurrentUse.World n) (registry : Registry n)
 (catalog : Catalog) (u : CurrentUse.UseRequest n) (args : List Int)
 (auth : CurrentUse.Evidence) (p : CallProof n)
 (dead : ¬ CurrentUse.CurrentHandle s .module u.moduleHandle) :
 catalogCall s registry catalog u args auth p = none := by
 cases h : catalogCall s registry catalog u args auth p with
 | none => rfl
 | some value => exact False.elim (dead (catalog_sound h).1.current.2.2.1)

-- Reading an older genuinely missing catalog entry fails; it never substitutes
-- another descriptor. Authenticating that image and current World remains open.
theorem missing_catalog_entry_rejected (s : CurrentUse.World n) (registry : Registry n)
 (catalog : Catalog) (u : CurrentUse.UseRequest n) (args : List Int)
 (auth : CurrentUse.Evidence) (p : CallProof n)
 (missing : catalog (s.records u.operation.key).code = none) :
 catalogCall s registry catalog u args auth p = none := by
 cases h : catalogCall s registry catalog u args auth p with
 | none => rfl
 | some value =>
   obtain ⟨_,d,_,found⟩ := catalog_sound h
   rw [missing] at found
   contradiction

#print axioms catalog_call_preserved
#print axioms insert_sequence_preserves_call
#print axioms no_catalog_revives_module
#print axioms missing_catalog_entry_rejected
end MirroreaProofFirst.ModuleContractBoundary
