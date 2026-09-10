import MirroreaProofFirstResourceComputations
namespace MirroreaProofFirst.ResourceComputations.ScopeControls
open ResourceBoundary Sequence
-- Counterexample to inferring whole-sequence capacity from allocation bounds.
def limited (position : Nat) : Context 4 :=
 let base := Sequence.Controls.context 3 position
 {base with current := {base.current with limits := ⟨1,1⟩}}
def afterSplit := run 20 limited 0 (Sequence.Controls.initial 3)
 (Sequence.Controls.program.take 3)
example : afterSplit.failure = none ∧ afterSplit.events.length = 2 ∧
 afterSplit.store.shared.resources.nextId = 3 := by decide
example : ¬ BoundedIdentifiers.Within ⟨1,1⟩ afterSplit.store.shared.resources := by unfold BoundedIdentifiers.Within; decide
-- No theorem authenticates a newly supplied context: the former still permits
-- the resumed call, while a supplied revoked context denies it (existing controls).
example : Continuation.Controls.finished.map (fun c => c.events.length) = some 5 := by decide
-- A saved whole configuration is a separate state history, not another control
-- action in the same history. One-shot proof does not certify rollback safety.
example : (Continuation.Controls.suspended.bind (Continuation.resume 0)).isSome = true := by decide
end MirroreaProofFirst.ResourceComputations.ScopeControls

namespace MirroreaProofFirst.ResourceComputations.Sequence.DistinctOccurrences
open ModuleContractBoundary
-- Test authority issues fresh bindings for distinct occurrences, not distinct
-- values. This helper constructs fixtures only, never production credentials.
def context (position : Nat) : Sequence.Context 4 :=
 let base := Controls.context 3 position
 let request := {base.caller with request := base.caller.request + position}
 {base with caller := request, evidence := {base.evidence with context := CurrentUse.currentContext base.current.world request}, proof := {base.proof with context := CurrentUse.currentContext base.current.world request, payload := {base.proof.payload with binding := expected base.current.world request ModuleContractBoundary.Controls.symbolicDescriptor [4]}}}
def result := run 20 context 0 (Controls.initial 3)
 [.pureLet (.lambda .int (.add (.var 0) (.integer 1))),.allocate Controls.source,.allocate Controls.source]
example : result.failure = none ∧ result.events.length = 2 ∧ result.store.shared.committed.length = 2 := by decide
example : result.store.shared.resources.nextId = 2 := by decide
end MirroreaProofFirst.ResourceComputations.Sequence.DistinctOccurrences

namespace MirroreaProofFirst.ResourceComputations.Continuation.CapturedShadow
open Sequence
-- The closure captures the outer3. After resumption an ordinary let binds100;
-- calling that closure still computes3+1 for the actual module allocation.
def source : Source 4 := ⟨.handle HandleValues.Controls.token,.app (.var 1) (.integer 1)⟩
def body : List (Instruction 4) :=
 [.pureLet (.lambda .int (.add (.var 1) (.var 0))),.pureLet (.integer 100),.allocate source]
def prepared := tick 20 (Sequence.Controls.context 3 0) (start (Sequence.Controls.initial 3) body)
def held := (capture prepared).map Prod.fst
def completed := (held.bind (resume 0)).map (drive 20 (Sequence.Controls.context 3) 4)
example : completed.map (fun c => (Controls.outcome c,c.events.length,c.phase.frame.lexical.values[0]?)) =
 some (none,1,some (.integer 100)) := by rfl
example : (completed.bind (fun c => c.events.head?)).map (fun event => match event with
 | .allocation hs => (hs.head?).map (fun h => h.region.hi)
 | _ => none) = some (some 17) := by decide
example : (Typing.inferProgram ⟨[.int],0⟩ body).isSome = true := by decide
end MirroreaProofFirst.ResourceComputations.Continuation.CapturedShadow

namespace MirroreaProofFirst.ResourceComputations.SelectionControls
open PureHandleFunctions ResourceBoundary ContractExport LocalContract
open ModuleContractBoundary ModuleContractBoundary.CurrentAllocation Sequence

def claimA : CurrentUse.Claim := {CurrentUse.Controls.claimA with targets := [3,4]}
def claimB : CurrentUse.Claim := {CurrentUse.Controls.claimB with targets := [3,4]}
def authority : CurrentUse.Authority :=
 ⟨[claimA,claimB],[],fun i => if i = 10 ∨ i = 20 then some 0 else none⟩
def world : CurrentUse.World 5 := {instanceId := 7,generation := 0,support := ⟨fun _ => .top,fun _ => true⟩, records := fun k => {identity := ⟨match k.val with | 0 => .member | 1 => .locus | 2 => .module | _ => .operation,1,0⟩, principal := 3,home := 1,moduleKey := 2,action := 5,code := 100,contract := 200}, authority := authority,policies := fun _ => CurrentUse.Controls.policy}
def handle (k : Fin 5) : CurrentUse.Handle 5 := ⟨7,k,(world.records k).identity⟩
def interface (k : Fin 5) : HandleValues.Interface 5 := ⟨handle 2,handle k⟩
def source : Source 5 :=
 ⟨.iterate .handle (.var 0) (.handle (interface 3)) (.handle (interface 4)),.integer 4⟩
def initial (count : Nat) : Store 5 :=
 ⟨Once.Controls.initial,⟨[.nat],[.natural count],[],fun _ => none⟩⟩
def use (k : Fin 5) : CurrentUse.UseRequest 5 :=
 ⟨3,handle 0,handle 1,handle 2,handle k,19,[.integer 4]⟩
def descriptor (profile : Profile) : Descriptor :=
 {codeId := 100,contractId := 200,arity := 1,profile := profile,theoryVersion := 1,contractVersion := 1,
  code := match profile with | .checkedValue => positiveTerm (.input 0) | .symbolicNonnegative => positiveTerm (.square (.input 0)),assumptions := []}
def proof (profile : Profile) (k : Fin 5) : CallProof 5 :=
 {context := CurrentUse.currentContext world (use k),moduleStamp := handle 2,operationStamp := handle k,
  payload := ⟨expected world (use k) (descriptor profile) [4],eval (fun _ => 4) (descriptor profile).code,
    .square (.input 0),.square (.input 0)⟩}
def context (profile : Profile) (k : Fin 5) : Context 5 := {current := {world := world,registry := fun _ => some (descriptor profile),catalog := fun _ => some (descriptor profile), grant := fun _ _ _ _ _ => true,limits := ⟨16,16⟩,lo := -10000,hi := 10000}, caller := use k,evidence := ⟨4,0,CurrentUse.currentContext world (use k),.both (.leaf claimA) (.leaf claimB)⟩, proof := proof profile k,resourcePolicy := fun _ => true}
def result (profile : Profile) (count : Nat) (k : Fin 5) := step 20 (context profile k) (initial count) (.allocate source)

example : interface 3 ≠ interface 4 := by decide
example : select 20 (initial 0).frame.values source = some (interface 3,4) := by decide
example : select 20 (initial 1).frame.values source = some (interface 4,4) := by decide
example : (result .symbolicNonnegative 0 3).failure = none ∧
 (result .symbolicNonnegative 1 4).failure = none := by decide
example : (result .checkedValue 0 3).failure = none ∧
 (result .checkedValue 1 4).failure = none := by decide
example : (result .symbolicNonnegative 1 3).failure = some .boundaryDenied := by decide
example : (result .symbolicNonnegative 1 4).store.frame.rights 0 = some ⟨0,0,17,3⟩ := by decide
example : (result .checkedValue 1 4).store.frame.rights 0 = some ⟨0,0,5,3⟩ := by decide
-- A true mathematical result with the wrong submitted symbolic certificate fails.
def wrongCertificate : Context 5 :=
 let good := context .symbolicNonnegative 4
 {good with proof := {good.proof with payload := {good.proof.payload with certificate := .integer 0}}}
example : (step 20 wrongCertificate (initial 1) (.allocate source)).failure = some .boundaryDenied := by decide
theorem premises (profile : Profile) :
 ProfileConsumer.Premises (context profile 4).current (use 4) (proof profile 4)
 (initial 1) source (interface 4) 4 (descriptor profile) := by
 constructor
 · exact (ResourceComputations.check_exact _ _).mp (by decide)
 · exact select_sound (show select 20 (initial 1).frame.values source = some (interface 4,4) by decide)
 · apply CurrentUse.checkUse_sound _ _ (context profile 4).evidence
   cases profile <;> decide
 · rfl
 · rfl
 · constructor <;> rfl
 · apply (common_exact _ _).mp
   cases profile <;> decide
 · cases profile with
   | checkedValue => change 0 < (5 : Int); decide
   | symbolicNonnegative => exact ⟨rfl,.square (.input 0)⟩
 · cases profile <;> simp [descriptor,positiveTerm,context,CheckedArithmetic.Bounds,CheckedArithmetic.InRange,eval]
 · rfl
 · unfold BoundedIdentifiers.Capacity
   cases profile <;> decide
 · simp [CurrentUse.Context.useId,initial,Once.Controls.initial]

#print axioms premises
end MirroreaProofFirst.ResourceComputations.SelectionControls

namespace MirroreaProofFirst.ResourceComputations.Sequence.MixedScheduleControls
open Continuation
-- A successful allocation precedes a fuel pause. Capture/resume occurs after
-- reactivating the residual; the final allocation uses a different current grant.
def secondSource : Source 4 :=
 ⟨.handle HandleValues.Controls.token,.app (.var 1) (.var 2)⟩
def body : List (Instruction 4) :=
 [.pureLet (.lambda .int (.add (.var 0) (.integer 1))),.allocate Sequence.Controls.source,
  .pureLet (.app (.lambda .int (.var 0)) (.integer 7)),.allocate secondSource]
def beforePause := drive 20 DistinctOccurrences.context 2 (start (Sequence.Controls.initial 3) body)
def paused := tick 0 (DistinctOccurrences.context 2) beforePause
def context (allow : Bool) : Sequence.Context 4 :=
 let base := DistinctOccurrences.context 3
 {base with current := {base.current with grant := fun _ _ _ _ _ => allow}}
def controls (allow : Bool) : List (Control 4) :=
 [.retryBudget,.capture,.resume 0,.tick 20 (DistinctOccurrences.context 2),
  .tick 20 (context allow),.tick 20 (DistinctOccurrences.context 4)]
def finished (allow : Bool) := Continuation.schedule paused (controls allow)
example : Continuation.Controls.outcome paused = some .referenceBudget ∧
 paused.events.length = 1 ∧ paused.phase.frame.position = 2 := by decide
example : Continuation.Controls.outcome (finished true) = none ∧
 (finished true).events.length = 2 ∧ (finished true).phase.frame.pending.length = 0 := by decide
example : Continuation.Controls.outcome (finished false) = some .boundaryDenied ∧
 (finished false).events.length = 1 ∧ (finished false).phase.frame.position = 3 ∧
 (finished false).phase.frame.pending.length = 1 := by decide
example : (finished false).phase.frame.lexical.rights 0 = some ⟨0,0,17,3⟩ ∧
 (finished false).shared.committed.length = 1 := by decide
-- Independent pointwise frame control: releasing one of two rights keeps the other.
def released := step 20 (DistinctOccurrences.context 3) DistinctOccurrences.result.store (.release 0)
example : released.failure = none ∧ released.store.frame.rights 1 = none ∧
 released.store.frame.rights 0 = some ⟨0,0,17,3⟩ := by decide
end MirroreaProofFirst.ResourceComputations.Sequence.MixedScheduleControls
