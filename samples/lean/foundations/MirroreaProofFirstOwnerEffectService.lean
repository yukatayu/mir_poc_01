import MirroreaProofFirstOwnerRecordedArithmetic
import MirroreaProofFirstAdmissionPhases
namespace MirroreaProofFirst.OwnerEffectService
open CurrentUse OwnerCheckedArithmetic

-- Research cut only. These are checked/materialized keys and parsed arguments.
-- Source elaboration, key ownership, IFC/resource contracts, authenticated
-- registry installation and real atomic service remain explicit obligations.
deriving instance DecidableEq for OwnerCheckedArithmetic.Checked
deriving instance Repr for OwnerCheckedArithmetic.Checked
deriving instance DecidableEq for CurrentUse.UseRequest

structure Body where
 target : Nat
 tree : Checked Nat Nat
 deriving DecidableEq, Repr
structure Origin where
 document : String
 byteOffset : Nat
 activation : Nat
 ordinal : Nat
 controlLabel : Nat
 deriving DecidableEq, Repr
structure Pending (n : Nat) where
 origin : Origin
 request : UseRequest n
 original : Evidence
 body : Body
 deriving DecidableEq

-- Code identifiers are looked up at service time; equal numeric identifiers
-- alone cannot hide a different body. This registry is an explicit trusted input.
def boundCode (w : World n) (registry : Nat → Option Body) (e : Pending n) : Bool :=
 decide (registry (w.records e.request.operation.key).code = some e.body)
def CodeAt (w : World n) (registry : Nat → Option Body) (e : Pending n) : Prop :=
 registry (w.records e.request.operation.key).code = some e.body

def args (e : Pending n) (j : Nat) : Option Int :=
 match e.request.arguments[j]? with
 | some (.integer value) => some value
 | _ => none

def put (store : Nat → Option Int) (target : Nat) (value : Int) : Nat → Option Int :=
 fun key => if key = target then some value else store key

structure Write (n : Nat) where
 pending : Pending n
 serviceEvidence : Evidence
 oldValue : Option Int
 value : Int
 reads : List (Nat × Int)
 deriving DecidableEq
inductive Failure where
 | authority | code | arithmetic
 deriving DecidableEq, Repr
inductive Outcome (n : Nat) where
 | refused (reason : Failure)
 | committed (write : Write n)
 deriving DecidableEq
structure Owner (n : Nat) where
 store : Nat → Option Int
 history : List (Write n)

-- A service outcome always carries its actual resulting store/history. A later
-- requester refusal cannot turn a committed Owner into Option.none or its input.
def serve (ops : FallibleFlow.Arithmetic) (world : World n)
 (registry : Nat → Option Body) (s : Owner n) (e : Pending n) : Owner n × Outcome n :=
 match AdmissionPhases.FullUse.resolveUse world e.request e.original with
 | none => (s,.refused .authority)
 | some current =>
   if boundCode world registry e then
    match Recorded.evaluateRecorded ops s.store (args e) e.body.tree with
    | none => (s,.refused .arithmetic)
    | some (v,reads) =>
      let write : Write n := ⟨e,current,s.store e.body.target,v,reads⟩
      (⟨put s.store e.body.target v,s.history++[write]⟩,.committed write)
   else (s,.refused .code)

-- Declarative successful service: original request/witness is currently usable,
-- exact installed code matches, independent value evaluator succeeds, and every
-- retained read is retrieved from the original service-time state. No assumption
-- here states that serve is correct.
def Commits (ops : FallibleFlow.Arithmetic) (world : World n)
 (registry : Nat → Option Body) (s : Owner n) (e : Pending n) (out : Write n) : Prop :=
 AdmissionPhases.FullUse.Allowed world e.request e.original ∧ CodeAt world registry e ∧
 evaluate ops s.store (args e) e.body.tree = some out.value ∧
 out = ⟨e,AdmissionPhases.atCurrent e.original (currentContext world e.request),
   s.store e.body.target,out.value,Recorded.actualReads s.store e.body.tree⟩

theorem committed_exact : (serve ops world registry s e).2 = .committed out ↔
 Commits ops world registry s e out := by
 unfold serve
 cases admitted : AdmissionPhases.FullUse.resolveUse world e.request e.original with
 | none =>
  simp only [reduceCtorEq,false_iff]
  intro h
  have yes := AdmissionPhases.FullUse.resolveUse_complete h.1
  rw [admitted] at yes
  contradiction
 | some current =>
  obtain ⟨allowed,rfl⟩ := AdmissionPhases.FullUse.resolveUse_exact.mp admitted
  by_cases code : CodeAt world registry e
  · simp only [CodeAt] at code
    simp only [boundCode,code,decide_true,ite_true]
    rw [Recorded.exact_recording]
    cases value : evaluate ops s.store (args e) e.body.tree with
    | none => simp [Commits,allowed,CodeAt,code,value]
    | some v =>
      simp only [Option.map_some,Outcome.committed.injEq]
      constructor
      · intro equal
        subst out
        exact ⟨allowed,code,value,rfl⟩
      · intro h
        have same : v = out.value := Option.some.inj (value.symm.trans h.2.2.1)
        simpa [same] using h.2.2.2.symm
  · have denied : boundCode world registry e = false := by simp [boundCode,CodeAt] at *; assumption
    simp [denied,Commits,code]

theorem committed_state (committed : (serve ops world registry s e).2 = .committed out) :
 (serve ops world registry s e).1.store = put s.store e.body.target out.value ∧
 (serve ops world registry s e).1.history = s.history ++ [out] := by
 unfold serve at *
 split at committed
 · cases committed
 · split at committed
   · split at committed
     · cases committed
     · cases committed; simp_all
   · cases committed

theorem refused_retains (refused : (serve ops world registry s e).2 = .refused why) :
 (serve ops world registry s e).1 = s := by
 unfold serve at *
 split at refused
 · rfl
 · split at refused
   · split at refused
     · simp_all
     · cases refused
   · simp_all

theorem all_service_retains_history : ∃ suffix,
 (serve ops world registry s e).1.history = s.history ++ suffix := by
 cases result : (serve ops world registry s e).2 with
 | refused why => exact ⟨[],by rw [refused_retains result]; simp⟩
 | committed out => exact ⟨[out],(committed_state result).2⟩

theorem commit_reads_exact (committed : (serve ops world registry s e).2 = .committed out) :
 ∀ k v, (k,v) ∈ out.reads ↔ k ∈ Recorded.keys e.body.tree ∧ s.store k = some v := by
 have meaning := committed_exact.mp committed
 have recorded := congrArg Write.reads meaning.2.2.2
 intro k v
 rw [recorded,Recorded.actualReads_exact]

#print axioms committed_exact
#print axioms committed_state
#print axioms refused_retains
#print axioms all_service_retains_history
#print axioms commit_reads_exact
end MirroreaProofFirst.OwnerEffectService
