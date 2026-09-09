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


-- Unreviewed reference composition. World, registry and allocation grant are
-- independent trusted inputs; no distributed atomic snapshot or issuer is derived.
namespace MirroreaProofFirst.ModuleContractBoundary.CurrentAllocation
open ContractExport ResourceBoundary

abbrev Grant (n : Nat) := CurrentUse.World n → CurrentUse.UseRequest n →
 Descriptor → State → Action → Bool

def run (limits : BoundedIdentifiers.Limits) (lo hi : Int)
 (world : CurrentUse.World n) (registry : Registry n) (catalog : Catalog)
 (u : CurrentUse.UseRequest n) (args : List Int) (auth : CurrentUse.Evidence)
 (p : CallProof n) (grant : Grant n) (state : State) : Option (State × List Handle) :=
 match registry u.operation.key with
 | none => none
 | some d =>
   if catalog d.codeId = some d ∧ CurrentUse.checkUse world u auth = true ∧
       linkCheck world u d args p = true then
     CheckedAllocation.run limits lo hi
       (fun s _ a => grant world u d s a) state (expected world u d args) p.payload
   else none

theorem exact (limits : BoundedIdentifiers.Limits) (lo hi : Int)
 (world : CurrentUse.World n) (registry : Registry n) (catalog : Catalog)
 (u : CurrentUse.UseRequest n) (args : List Int) (auth : CurrentUse.Evidence)
 (p : CallProof n) (grant : Grant n) (state : State) (out : State × List Handle) :
 run limits lo hi world registry catalog u args auth p grant state = some out ↔
 ∃ d value, registry u.operation.key = some d ∧ catalog d.codeId = some d ∧
 CurrentUse.checkUse world u auth = true ∧ Linked world u d args p ∧
 CheckedArithmetic.Denotes lo hi args d.code p.payload.result ∧
 accept (expected world u d args) p.payload = some value ∧
 grant world u d state (.allocate u.principal value) = true ∧
 BoundedIdentifiers.Capacity limits state (.allocate u.principal value) ∧
 out = raw state (.allocate u.principal value) := by
 unfold run
 cases lookup : registry u.operation.key with
 | none => simp
 | some d =>
   have guarded (condition : Prop) [Decidable condition]
       (value : Option (State × List Handle)) :
       (if condition then value else none) = some out ↔
       condition ∧ value = some out := by
     by_cases h : condition <;> simp [h]
   simp only [Option.some.injEq]
   rw [guarded]
   simp only [CheckedAllocation.run_exact, expected, CheckedArithmetic.exact,
     link_exact]
   constructor
   · rintro ⟨⟨registered,current,linked⟩,machine,value,accepted,granted,capacity,result⟩
     exact ⟨d,value,rfl,registered,current,linked,machine,accepted,granted,capacity,result⟩
   · rintro ⟨found,value,hfound,registered,current,linked,machine,accepted,granted,capacity,result⟩
     subst found
     exact ⟨⟨registered,current,linked⟩,machine,value,accepted,granted,capacity,result⟩

-- This conclusion does not equate either authorization layer with a certificate.
theorem sound {limits : BoundedIdentifiers.Limits} {lo hi : Int}
 {world : CurrentUse.World n} {registry : Registry n} {catalog : Catalog}
 {u : CurrentUse.UseRequest n} {args : List Int} {auth : CurrentUse.Evidence}
 {p : CallProof n} {grant : Grant n} {state : State} (wf : WF state)
 {out : State × List Handle}
 (ok : run limits lo hi world registry catalog u args auth p grant state = some out) :
 CurrentUse.CurrentUse world u ∧ ∃ d value,
 registry u.operation.key = some d ∧ catalog d.codeId = some d ∧
 Linked world u d args p ∧ PositiveExport (expected world u d args) p.payload value ∧
 CheckedArithmetic.Denotes lo hi args d.code p.payload.result ∧
 grant world u d state (.allocate u.principal value) = true ∧
 WF out.1 ∧ BoundedIdentifiers.Within limits out.1 := by
 obtain ⟨d,value,lookup,registered,current,linked,machine,accepted,granted,capacity,rfl⟩ :=
   (exact _ _ _ _ _ _ _ _ _ _ _ _ _).mp ok
 exact ⟨CurrentUse.checkUse_sound _ _ _ current,d,value,lookup,registered,linked,
   accept_sound accepted,machine,granted,
   allocate_wf wf _ _ (accept_bound accepted).1,
   (BoundedIdentifiers.raw_within _ _ _).mpr capacity⟩

theorem lawful_completes {limits : BoundedIdentifiers.Limits} {lo hi : Int}
 {world : CurrentUse.World n} {registry : Registry n} {catalog : Catalog}
 {u : CurrentUse.UseRequest n} {args : List Int} {p : CallProof n} {grant : Grant n}
 {state : State} {d : Descriptor} {value : Nat}
 (live : CurrentUse.CurrentUse world u) (lookup : registry u.operation.key = some d)
 (registered : catalog d.codeId = some d) (linked : Linked world u d args p)
 (accepted : accept (expected world u d args) p.payload = some value)
 (bounds : CheckedArithmetic.Bounds lo hi args d.code)
 (granted : grant world u d state (.allocate u.principal value) = true)
 (capacity : BoundedIdentifiers.Capacity limits state (.allocate u.principal value)) :
 ∃ auth, run limits lo hi world registry catalog u args auth p grant state =
   some (raw state (.allocate u.principal value)) := by
 obtain ⟨auth,current⟩ := CurrentUse.checkUse_complete _ _ live
 have machine := CheckedArithmetic.accepted_machine_value accepted bounds
 rw [CheckedAllocation.accepted_result accepted] at machine
 refine ⟨auth,(exact _ _ _ _ _ _ _ _ _ _ _ _ _).mpr ?_⟩
 exact ⟨d,value,lookup,registered,current,linked,
   (CheckedArithmetic.exact _ _ _ _ _).mp machine,accepted,granted,capacity,rfl⟩

theorem current_denied (limits : BoundedIdentifiers.Limits) (lo hi : Int)
 (world : CurrentUse.World n) (registry : Registry n) (catalog : Catalog)
 (u : CurrentUse.UseRequest n) (args : List Int) (auth : CurrentUse.Evidence)
 (p : CallProof n) (grant : Grant n) (state : State)
 (denied : ¬ CurrentUse.CurrentUse world u) :
 run limits lo hi world registry catalog u args auth p grant state = none := by
 cases h : run limits lo hi world registry catalog u args auth p grant state with
 | none => rfl
 | some out =>
   obtain ⟨_,_,_,_,current,_⟩ := (exact _ _ _ _ _ _ _ _ _ _ _ _ _).mp h
   exact False.elim (denied (CurrentUse.checkUse_sound _ _ _ current))

theorem resource_denied (limits : BoundedIdentifiers.Limits) (lo hi : Int)
 (world : CurrentUse.World n) (registry : Registry n) (catalog : Catalog)
 (u : CurrentUse.UseRequest n) (args : List Int) (auth : CurrentUse.Evidence)
 (p : CallProof n) (grant : Grant n) (state : State)
 (denied : ∀ d value, grant world u d state (.allocate u.principal value) = false) :
 run limits lo hi world registry catalog u args auth p grant state = none := by
 cases h : run limits lo hi world registry catalog u args auth p grant state with
 | none => rfl
 | some out =>
   obtain ⟨d,value,_,_,_,_,_,_,granted,_⟩ := (exact _ _ _ _ _ _ _ _ _ _ _ _ _).mp h
   rw [denied d value] at granted
   contradiction

namespace Controls
open ModuleContractBoundary.Controls CurrentUse.Controls
-- This finite test grant explicitly sees the context, complete carried stamps,
-- descriptor and requested allocation. It is not a cryptographic issuer model.
def grant : Grant 4 := fun w u d _ action =>
 decide (CurrentUse.currentContext w u = CurrentUse.currentContext world request ∧
   u.moduleHandle = request.moduleHandle ∧ u.operation = request.operation ∧
   d = descriptor ∧ action = .allocate 3 42)
def result := (run ⟨1,1⟩ (-100) 100 world registry DescriptorControl.catalog
 request [41] evidence proof grant empty).map
 (fun out => out.2.map (fun h => (h.id,h.region.hi,h.region.holder)))
example : result = some [(0,42,3)] := by decide
example : run ⟨1,1⟩ (-100) 100 world registry DescriptorControl.catalog
 request [41] evidence proof (fun _ _ _ _ _ => false) empty = none := by decide
example : run ⟨1,1⟩ (-100) 100 {world with authority := {auth with revoked := [2]}}
 registry DescriptorControl.catalog request [41] evidence proof
 (fun _ _ _ _ _ => true) empty = none := by decide
example : run ⟨1,1⟩ (-100) 100 retiredWorld registry DescriptorControl.catalog
 request [41] evidence proof (fun _ _ _ _ _ => true) empty = none := by decide
example : run ⟨1,1⟩ (-100) 100 world registry (fun _ => none)
 request [41] evidence proof grant empty = none := by decide
example : run ⟨0,1⟩ (-100) 100 world registry DescriptorControl.catalog
 request [41] evidence proof grant empty = none := by decide
-- Refresh the valid module-layer evidence for a different invocation; that
-- does not refresh the independent resource grant for the original invocation.
def other : CurrentUse.UseRequest 4 := {request with request := 10}
def otherEvidence : CurrentUse.Evidence :=
 {evidence with context := CurrentUse.currentContext world other}
def otherProof : CallProof 4 :=
 {proof with
  context := CurrentUse.currentContext world other
  payload := {proof.payload with binding := expected world other descriptor [41]}}
example : catalogCall world registry DescriptorControl.catalog other [41] otherEvidence otherProof = some 42 := by decide
example : run ⟨1,1⟩ (-100) 100 world registry DescriptorControl.catalog
 other [41] otherEvidence otherProof grant empty = none := by decide
-- Current authorization and fresh resource IDs do not supply request deduplication.
def repeated := (run ⟨2,2⟩ (-100) 100 world registry DescriptorControl.catalog
 request [41] evidence proof grant empty).bind fun first =>
 (run ⟨2,2⟩ (-100) 100 world registry DescriptorControl.catalog
 request [41] evidence proof grant first.1).map fun second =>
 (first.2.map Handle.id,second.2.map Handle.id)
example : repeated = some ([0],[1]) := by decide
end Controls
#print axioms exact
#print axioms sound
#print axioms lawful_completes
#print axioms current_denied
#print axioms resource_denied
end MirroreaProofFirst.ModuleContractBoundary.CurrentAllocation

-- Unreviewed atomic reference step; crash-prefix mechanisms remain unproved.

namespace MirroreaProofFirst.ModuleContractBoundary.CurrentAllocation.Once
open ResourceBoundary
-- Reference inputs, not a wire format or trusted-head acquisition mechanism.
structure Invocation (n : Nat) where
 limits : BoundedIdentifiers.Limits
 lo : Int
 hi : Int
 world : CurrentUse.World n
 registry : Registry n
 catalog : Catalog
 request : CurrentUse.UseRequest n
 arguments : List Int
 evidence : CurrentUse.Evidence
 proof : CallProof n
 grant : Grant n

def Invocation.key (i : Invocation n) : CurrentUse.UseId :=
 (CurrentUse.currentContext i.world i.request).useId

def Invocation.allocate (i : Invocation n) (s : State) : Option (State × List Handle) :=
 run i.limits i.lo i.hi i.world i.registry i.catalog i.request i.arguments
 i.evidence i.proof i.grant s

structure StateWithHistory where
 resources : State
 committed : List CurrentUse.UseId

def step (i : Invocation n) (s : StateWithHistory) : Option (StateWithHistory × List Handle) :=
 if i.key ∈ s.committed then none else
 (i.allocate s.resources).map fun out => (⟨out.1,i.key :: s.committed⟩,out.2)

theorem step_exact (i : Invocation n) (s : StateWithHistory)
 (out : StateWithHistory × List Handle) :
 step i s = some out ↔ i.key ∉ s.committed ∧
 ∃ result, i.allocate s.resources = some result ∧
 out = (⟨result.1,i.key :: s.committed⟩,result.2) := by
 by_cases h : i.key ∈ s.committed
 · simp [step,h]
 · cases allocated : i.allocate s.resources <;> simp [step,h,allocated,eq_comm]

def advance (i : Invocation n) (s : StateWithHistory) : StateWithHistory :=
 ((step i s).map Prod.fst).getD s

theorem committed_retained (i : Invocation n) (s : StateWithHistory)
 (key : CurrentUse.UseId) (present : key ∈ s.committed) :
 key ∈ (advance i s).committed := by
 cases h : step i s with
 | none => simpa [advance,h] using present
 | some out =>
   obtain ⟨_,result,_,rfl⟩ := (step_exact _ _ _).mp h
   simpa [advance,h] using List.mem_cons_of_mem i.key present

theorem advance_wf (i : Invocation n) (s : StateWithHistory) (wf : WF s.resources)
 (unique : s.committed.Nodup) :
 WF (advance i s).resources ∧ (advance i s).committed.Nodup := by
 cases h : step i s with
 | none => simpa [advance,h] using And.intro wf unique
 | some out =>
   obtain ⟨fresh,result,allocated,rfl⟩ := (step_exact _ _ _).mp h
   have preserved := (CurrentAllocation.sound wf allocated).2
   obtain ⟨_,_,_,_,_,_,_,_,wf',_⟩ := preserved
   simpa [advance,h,List.nodup_cons] using And.intro wf' (And.intro fresh unique)

def schedule : StateWithHistory → List (Invocation n) → StateWithHistory
 | s,[] => s
 | s,i::rest => schedule (advance i s) rest

theorem schedule_retains (s : StateWithHistory) (is : List (Invocation n))
 (key : CurrentUse.UseId) (present : key ∈ s.committed) :
 key ∈ (schedule s is).committed := by
 induction is generalizing s with
 | nil => exact present
 | cons i is ih => exact ih _ (committed_retained i s key present)

theorem schedule_wf (s : StateWithHistory) (is : List (Invocation n))
 (wf : WF s.resources) (unique : s.committed.Nodup) :
 WF (schedule s is).resources ∧ (schedule s is).committed.Nodup := by
 induction is generalizing s with
 | nil => exact ⟨wf,unique⟩
 | cons i is ih =>
   obtain ⟨wf',unique'⟩ := advance_wf i s wf unique
   exact ih _ wf' unique'

theorem no_second_effect {i : Invocation n} {s : StateWithHistory}
 {out : StateWithHistory × List Handle} (success : step i s = some out)
 (later : List (Invocation n)) (again : Invocation n) (same : again.key = i.key) :
 step again (schedule out.1 later) = none := by
 obtain ⟨_,result,_,rfl⟩ := (step_exact _ _ _).mp success
 have retained := schedule_retains
   (⟨result.1,i.key :: s.committed⟩ : StateWithHistory) later i.key (by simp)
 simp [step,same,retained]

theorem failure_unchanged (i : Invocation n) (s : StateWithHistory)
 (failure : step i s = none) : advance i s = s := by simp [advance,failure]

theorem fresh_success (i : Invocation n) (s : StateWithHistory)
 (fresh : i.key ∉ s.committed) (allocated : i.allocate s.resources = some result) :
 step i s = some (⟨result.1,i.key :: s.committed⟩,result.2) :=
 (step_exact _ _ _).mpr ⟨fresh,result,allocated,rfl⟩

namespace Controls
open ModuleContractBoundary.Controls CurrentUse.Controls

def invocation : Invocation 4 :=
 {limits := ⟨3,3⟩, lo := -100, hi := 100, world := world,
  registry := registry, catalog := DescriptorControl.catalog, request := request,
  arguments := [41], evidence := evidence, proof := proof,
  grant := CurrentAllocation.Controls.grant}
def initial : StateWithHistory := ⟨empty,[]⟩
def first := advance invocation initial
example : first.resources.nextId = 1 ∧ first.committed = [invocation.key] := by decide
example : step invocation first = none := by decide
example : (schedule initial [invocation,invocation]).resources.nextId = 1 := by decide
-- Different requests can succeed; this is not an all-rejection safety argument.
def other : Invocation 4 :=
 {invocation with
  request := CurrentAllocation.Controls.other
  evidence := CurrentAllocation.Controls.otherEvidence
  proof := CurrentAllocation.Controls.otherProof
  grant := fun _ _ _ _ _ => true}
example : (schedule initial [invocation,other]).resources.nextId = 2 := by decide
example : (schedule initial [invocation,other]).committed.length = 2 := by decide
-- An unsuccessful attempt does not consume the logical key.
def denied : Invocation 4 := {invocation with grant := fun _ _ _ _ _ => false}
example : (schedule initial [denied,invocation]).resources.nextId = 1 := by decide
example : (schedule initial [denied,invocation]).committed.length = 1 := by decide
-- Crash-prefix models are deliberately outside the atomic step relation.
-- Resource write surviving while the history write is lost permits repetition.
def lostHistory : StateWithHistory := ⟨first.resources,[]⟩
example : (advance invocation lostHistory).resources.nextId = 2 := by decide
-- Conversely, history surviving without the effect blocks the missing allocation.
def lostEffect : StateWithHistory := ⟨empty,first.committed⟩
example : step invocation lostEffect = none := by decide
example : lostEffect.resources.nextId = 0 := by decide
end Controls
#print axioms step_exact
#print axioms committed_retained
#print axioms advance_wf
#print axioms schedule_retains
#print axioms schedule_wf
#print axioms no_second_effect
#print axioms failure_unchanged
#print axioms fresh_success
end MirroreaProofFirst.ModuleContractBoundary.CurrentAllocation.Once
