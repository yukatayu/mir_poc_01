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
