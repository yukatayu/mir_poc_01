import Std
namespace MirroreaProofFirst.OwnerStatementIdentity

-- Structured source occurrence identity. Program identity is a namespace,
-- not an authenticity credential. Actor/owner/code/signature/labels/site are
-- retained in Code; no projection erases any field of that payload.
structure Scope where
 program : String
 handler : String
 deriving DecidableEq
structure Key where
 scope : Scope
 ordinal : Nat
 deriving DecidableEq
structure Entry (Code : Type) where
 key : Key
 code : Code
 deriving DecidableEq

def lower (scope : Scope) (start : Nat) : List Code → List (Entry Code)
 | [] => []
 | code :: rest => ⟨⟨scope,start⟩,code⟩ :: lower scope (start+1) rest

-- Positional declarative elaboration, with distinct equal-looking occurrences.
inductive Elaborates (scope : Scope) : Nat → List Code → List (Entry Code) → Prop where
 | nil : Elaborates scope start [] []
 | cons : Elaborates scope (start+1) source tail →
   Elaborates scope start (code::source) (⟨⟨scope,start⟩,code⟩::tail)

variable {Code : Type} {source : List Code} {rows raw : List (Entry Code)} {entry : Entry Code}

theorem lower_sound : Elaborates scope start source (lower scope start source) := by
 induction source generalizing start with
 | nil => exact .nil
 | cons code source ih => exact .cons ih

theorem lower_complete (typed : Elaborates scope start source rows) :
 rows = lower scope start source := by
 induction typed with
 | nil => rfl
 | cons _ ih => simp [lower,ih]

theorem lower_exact : lower scope start source = rows ↔ Elaborates scope start source rows := by
 constructor
 · rintro rfl; exact lower_sound
 · intro typed; exact (lower_complete typed).symm

-- Ordered authentic-source comparison; equality includes complete Code.
-- Independent expected-source custody is a caller obligation, not this check.
def check [DecidableEq Code] (scope : Scope) (source : List Code) (rows : List (Entry Code)) : Bool :=
 decide (rows = lower scope 0 source)

theorem check_exact [DecidableEq Code] : check scope source rows = true ↔ Elaborates scope 0 source rows := by
 simp only [check,decide_eq_true_eq]
 exact ⟨fun h => (lower_exact).mp h.symm,fun h => (lower_complete h)⟩

theorem erase_lower : (lower scope start source).map Entry.code = source := by
 induction source generalizing start with
 | nil => rfl
 | cons code source ih => simp [lower,ih]

theorem lower_length : (lower scope start source).length = source.length := by
 induction source generalizing start with
 | nil => rfl
 | cons code source ih => simp [lower,ih]

theorem member_bound (present : entry ∈ lower scope start source) :
 entry.key.scope = scope ∧ start ≤ entry.key.ordinal ∧ entry.key.ordinal < start+source.length := by
 induction source generalizing start with
 | nil => simp [lower] at present
 | cons code source ih =>
   simp only [lower,List.mem_cons] at present
   rcases present with rfl | present
   · simp
   · have h := ih present
     exact ⟨h.1,by omega,by simpa only [List.length_cons] using (show entry.key.ordinal < start+(source.length+1) by omega)⟩

theorem keys_nodup : ((lower scope start source).map Entry.key).Nodup := by
 induction source generalizing start with
 | nil => simp [lower]
 | cons code source ih =>
   simp only [lower,List.map_cons,List.nodup_cons]
   refine ⟨?_,ih⟩
   intro present
   obtain ⟨entry,member,equal⟩ := List.mem_map.mp present
   have bound := (member_bound member).2.1
   have ord : entry.key.ordinal = start := congrArg Key.ordinal equal
   omega

theorem indexed_payload (i : Nat) :
 (lower scope start source)[i]? = (source[i]?).map (fun code => ⟨⟨scope,start+i⟩,code⟩) := by
 induction source generalizing start i with
 | nil => simp [lower]
 | cons code source ih =>
   cases i with
   | zero => simp [lower]
   | succ i => simpa [lower,Nat.add_assoc,Nat.add_comm,Nat.add_left_comm] using (ih (start:=start+1) i)

theorem different_ordinals (different : i ≠ j) :
 (Key.mk scope i) ≠ (Key.mk scope j) := by
 intro same; exact different (congrArg Key.ordinal same)

theorem different_scopes (different : scope ≠ other) :
 (Key.mk scope i) ≠ (Key.mk other j) := by
 intro same; exact different (congrArg Key.scope same)

-- Rechecking after raw image decode requires the same independently held
-- source. Self-consistent replacement does not establish authenticity.
def restore [DecidableEq Code] (scope : Scope) (source : List Code) (raw : List (Entry Code)) : Option (List (Entry Code)) :=
 if check scope source raw then some raw else none

theorem restore_exact [DecidableEq Code] :
 restore scope source raw = some rows ↔ raw = rows ∧ Elaborates scope 0 source rows := by
 unfold restore
 by_cases h : check scope source raw = true
 · simp [h]
   intro equal
   subst rows
   exact (check_exact).mp h
 · simp [h]
   intro equal typed
   subst rows
   exact h ((check_exact).mpr typed)

#print axioms lower_exact
#print axioms check_exact
#print axioms erase_lower
#print axioms lower_length
#print axioms member_bound
#print axioms keys_nodup
#print axioms indexed_payload
#print axioms different_ordinals
#print axioms different_scopes
#print axioms restore_exact
end MirroreaProofFirst.OwnerStatementIdentity
