import MirroreaProofFirstOwnerTypedIndex
namespace MirroreaProofFirst.OwnerArgumentPresence
open OwnerTypedIndex OwnerStructuredKeys

-- Research only: supplied String captures, no nominal-value authentication.
-- The signature is complete input data retained from a particular checked
-- operation; this predicate does not establish that provenance.
def complete (signature : List Parameter) (args : String → Option String) : Bool :=
 decide (signature.map Parameter.name).Nodup &&
 signature.all (fun p => (args p.name).isSome)

def Supplied (signature : List Parameter) (args : String → Option String) : Prop :=
 (signature.map Parameter.name).Nodup ∧
 ∀ p ∈ signature, ∃ value, args p.name = some value

theorem complete_exact : complete signature args = true ↔ Supplied signature args := by
 simp [complete,Supplied,List.all_eq_true,Option.isSome_iff_exists]

theorem supplied_index (h : Supplied signature args) (p : Parameter) (member : p ∈ signature)
 (namespaceName field owner : String) :
 ∃ value, args p.name = some value ∧
 materialize args ⟨namespaceName,some p.name,some field,owner⟩ =
 some ⟨namespaceName,value,field⟩ := by
 obtain ⟨value,bound⟩ := h.2 p member
 exact ⟨value,bound,by simp [materialize,bound]⟩

theorem missing_refuses (p : Parameter) (member : p ∈ signature)
 (missing : args p.name = none) : complete signature args = false := by
 cases result : complete signature args with
 | false => rfl
 | true =>
   obtain ⟨value,bound⟩ := (complete_exact.mp result).2 p member
   simp [missing] at bound

theorem duplicate_refuses
 (duplicate : ¬ (signature.map Parameter.name).Nodup) :
 complete signature args = false := by simp [complete,duplicate]

-- Exact current invocation context is separate from completeness. Checking
-- before enqueuing only suffices when the consumed request retains that map.
theorem same_arguments (h : Supplied signature args)
 (same : ∀ p ∈ signature, changed p.name = args p.name) :
 Supplied signature changed := by
 refine ⟨h.1,?_⟩
 intro p member
 obtain ⟨value,bound⟩ := h.2 p member
 exact ⟨value,(same p member).trans bound⟩

-- A use-time guard preserves the whole underlying success/failure result for
-- complete calls, while missing/ambiguous captures cannot invoke the body.
def guarded (signature : List Parameter) (args : String → Option String)
 (body : (String → Option String) → Option α) : Option α :=
 if complete signature args then body args else none

def AdmittedResult (signature : List Parameter) (args : String → Option String)
 (body : (String → Option String) → Option α) (value : α) : Prop :=
 Supplied signature args ∧ body args = some value

theorem guarded_exact : guarded signature args body = some value ↔
 AdmittedResult signature args body value := by
 unfold guarded AdmittedResult
 by_cases h : complete signature args = true
 · simp [h,complete_exact.mp h]
 · have absent : ¬ Supplied signature args := by simpa only [← complete_exact] using h
   simp [h,absent]

theorem guarded_preserves (h : Supplied signature args) :
 guarded signature args body = body args := by simp [guarded,complete_exact.mpr h]

theorem guarded_missing (p : Parameter) (member : p ∈ signature)
 (missing : args p.name = none) : guarded signature args body = none := by
 simp [guarded,missing_refuses p member missing]

namespace Controls
def sig : List Parameter := [⟨"target","Player"⟩,⟨"delta","Int"⟩]
def args : String → Option String := fun name =>
 if name="target" then some "alice" else if name="delta" then some "2" else none
#guard complete sig args
#guard !complete sig (fun _ => none)
#guard !complete (sig++sig) args
#guard complete [] (fun _ => none)
-- Presence is weaker than scalar validity, nominal authenticity, label release,
-- current membership and source/image custody. These positive controls refute
-- those stronger interpretations, rather than silently pretending to check them.
#guard complete sig (fun _ => some "not-a-valid-Int")
#guard guarded sig args (fun _ => some (7 : Nat)) = some 7
#guard guarded sig (fun _ => none) (fun _ => some (7 : Nat)) = none
end Controls
#print axioms complete_exact
#print axioms supplied_index
#print axioms guarded_exact
#print axioms guarded_preserves
#print axioms guarded_missing
end MirroreaProofFirst.OwnerArgumentPresence
