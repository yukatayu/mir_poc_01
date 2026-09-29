import MirroreaProofFirstOwnerArgumentDomain
namespace MirroreaProofFirst.OwnerInvocationContext
open OwnerTypedIndex OwnerArgumentDomain

-- A field-for-field finite bridge, not a parser or authentic-source issuer.
-- Code includes the full ordered expression/target/budget/context retained by
-- the consumer. A source reference or artifact string is an identifier only.
structure SourceParameter where
 parameter : Parameter
 sourceRef : String
 deriving DecidableEq, Repr

structure Declaration (Code : Type) where
 artifact : String
 operation : String
 actor : String
 owner : String
 sourceRef : String
 signature : List SourceParameter
 code : Code
 deriving DecidableEq, Repr

structure Plan (Code : Type) where
 artifact : String
 operation : String
 actor : String
 owner : String
 sourceRef : String
 signature : List SourceParameter
 code : Code
 deriving DecidableEq, Repr

def lower (source : Declaration Code) : Plan Code :=
 ⟨source.artifact,source.operation,source.actor,source.owner,source.sourceRef,source.signature,source.code⟩

-- The independent per-occurrence relation names the original source fields,
-- not a checker success or a desired postcondition as a premise.
def Corresponds (source : Declaration Code) (plan : Plan Code) : Prop :=
 plan.artifact = source.artifact ∧ plan.operation = source.operation ∧
 plan.actor = source.actor ∧ plan.owner = source.owner ∧
 plan.sourceRef = source.sourceRef ∧ plan.signature = source.signature ∧ plan.code = source.code

theorem lower_exact : lower source = plan ↔ Corresponds source plan := by
 cases source; cases plan
 simp [lower,Corresponds,eq_comm]

def InventoryCorresponds : List (Declaration Code) → List (Plan Code) → Prop
 | [],[] => True
 | source::tail,plan::rest => Corresponds source plan ∧ InventoryCorresponds tail rest
 | _,_ => False

theorem inventory_exact : source.map lower = plans ↔ InventoryCorresponds source plans := by
 induction source generalizing plans with
 | nil => cases plans <;> simp [InventoryCorresponds]
 | cons source tail ih =>
   cases plans with
   | nil => simp [InventoryCorresponds]
   | cons plan rest => simp [InventoryCorresponds,lower_exact,ih]

def check [DecidableEq Code] (source : List (Declaration Code)) (plans : List (Plan Code)) : Bool :=
 decide (source.map lower = plans)

theorem check_exact [DecidableEq Code] {source : List (Declaration Code)} {plans : List (Plan Code)} : check source plans = true ↔ InventoryCorresponds source plans := by
 simp [check,inventory_exact]

theorem lowered : InventoryCorresponds source (source.map lower) := inventory_exact.mp rfl

-- Restriction retains exact parameters, including unused/duplicate names and
-- source spans. It neither re-elaborates from expression reads nor deduplicates.
theorem restrict (related : InventoryCorresponds source plans) (owners : List String) :
 InventoryCorresponds (source.filter (fun d => owners.contains d.owner))
 (plans.filter (fun p => owners.contains p.owner)) := by
 rw [← inventory_exact] at related ⊢
 rw [← related,List.filter_map]
 rfl

theorem retained_signature (related : Corresponds source plan) :
 plan.signature = source.signature := related.2.2.2.2.2.1

theorem signature_erasure_refused [DecidableEq Code] {source : Declaration Code}
 (nonempty : source.signature ≠ []) :
 check [source] [{lower source with signature := []}] = false := by
 simp [check,lower,nonempty]

-- The expected inventory is passed separately from candidate decoding.
-- This model authenticates neither input. Real source custody/expected-image
-- custody is a TCB obligation, and comparing two altered decoded copies is not
-- a substitute. Old missing fields are not represented by an empty signature.
def restore [DecidableEq Code] (expected : List (Declaration Code))
 (candidate : List (Plan Code)) : Option (List (Plan Code)) :=
 if check expected candidate then some candidate else none

theorem restore_exact [DecidableEq Code] {expected : List (Declaration Code)} {candidate plans : List (Plan Code)} : restore expected candidate = some plans ↔
 candidate = plans ∧ InventoryCorresponds expected candidate := by
 simp only [restore]
 by_cases accepted : check expected candidate = true
 · simp [accepted,check_exact.mp accepted]
 · have wrong : ¬ InventoryCorresponds expected candidate := by
     simpa only [← check_exact] using accepted
   simp [accepted,wrong]

theorem restored_roundtrip [DecidableEq Code] {source : List (Declaration Code)} :
 restore source (source.map lower) = some (source.map lower) := by simp [restore,check]

-- The guard is bound to the checked declaration paired with this actual plan;
-- full signature correspondence makes use-time admission the source judgment.
def signatureOf (source : Declaration Code) : List Parameter := source.signature.map SourceParameter.parameter
def planSignature (plan : Plan Code) : List Parameter := plan.signature.map SourceParameter.parameter

theorem invocation_domain (related : Corresponds source plan) :
 OwnerArgumentDomain.check (planSignature plan) args = true ↔
 Matches (signatureOf source) args := by
 simpa [planSignature,signatureOf,retained_signature related] using
   (OwnerArgumentDomain.check_exact (signature := signatureOf source) (args := args))

theorem invocation_result (related : Corresponds source plan) :
 invoke (planSignature plan) args state body = invoke (signatureOf source) args state body := by
 simp only [planSignature,signatureOf,retained_signature related]

-- A shared-body equation alone does not use the retained Code field. This
-- equation interprets the two original code values separately. The concrete
-- Rust interpreter remains an implementation obligation, not an axiom here.
theorem invocation_code_result {source : Declaration Code} {plan : Plan Code}
 (related : Corresponds source plan)
 (eval : Code → Arguments → State → Outcome State Event Result) :
 invoke (planSignature plan) args state (eval plan.code) =
 invoke (signatureOf source) args state (eval source.code) := by
 simp only [planSignature,signatureOf,retained_signature related,related.2.2.2.2.2.2]

-- Rust duplicates these signature coordinates in its outer plan. Keep that
-- duplication explicit in the representation instead of forgetting it. Rows
-- are already retained in Plan.signature; this record holds only coordinates.
structure SignatureCoordinates where
 operation : String
 ownerKind : Bool
 actor : Option String
 owner : Option String
 sourceRef : String
 deriving DecidableEq, Repr

def CoordinatesAgree (plan : Plan Code) (stored : SignatureCoordinates) : Prop :=
 stored.operation = plan.operation ∧ stored.ownerKind = true ∧
 stored.actor = some plan.actor ∧ stored.owner = some plan.owner ∧
 stored.sourceRef = plan.sourceRef

instance (plan : Plan Code) (stored : SignatureCoordinates) :
 Decidable (CoordinatesAgree plan stored) := inferInstanceAs (Decidable
 (stored.operation = plan.operation ∧ stored.ownerKind = true ∧
 stored.actor = some plan.actor ∧ stored.owner = some plan.owner ∧
 stored.sourceRef = plan.sourceRef))

def checkCoordinates (plan : Plan Code) (stored : SignatureCoordinates) : Bool :=
 decide (CoordinatesAgree plan stored)

def checkRetained (plan : Plan Code) (stored : SignatureCoordinates) (args : Arguments) : Bool :=
 checkCoordinates plan stored && OwnerArgumentDomain.check (planSignature plan) args

theorem retained_domain_exact (related : Corresponds source plan) :
 checkRetained plan stored args = true ↔
 CoordinatesAgree plan stored ∧ Matches (signatureOf source) args := by
 simp [checkRetained,checkCoordinates,invocation_domain related]

-- Replacing both expected/source context and plans with a freshly lowered
-- inventory preserves correspondence. Whether the update is authorized,
-- compatible with pending requests or current is a separate premise/check.
theorem replacement (source : List (Declaration Code)) :
 InventoryCorresponds source (source.map lower) := lowered

-- SYS3 presents operation inventories sorted by name, while M8 retains
-- source order. This comparison forgets only inventory presentation order;
-- it retains full parameter order, code and source occurrence. It does not
-- establish multi-statement continuation or change execution scheduling.
def sourceKey (source : Declaration Code) := (source.operation,source.owner)
def planKey (plan : Plan Code) := (plan.operation,plan.owner)
def checkInventory [DecidableEq Code] (source : List (Declaration Code)) (plans : List (Plan Code)) : Bool :=
 decide (source.map sourceKey).Nodup && decide (plans.map planKey).Nodup &&
 source.all (fun d => plans.any (fun p => decide (lower d = p))) &&
 plans.all (fun p => source.any (fun d => decide (lower d = p)))

def InventoryMatches (source : List (Declaration Code)) (plans : List (Plan Code)) : Prop :=
 (source.map sourceKey).Nodup ∧ (plans.map planKey).Nodup ∧
 (∀ d ∈ source, ∃ p ∈ plans, Corresponds d p) ∧
 (∀ p ∈ plans, ∃ d ∈ source, Corresponds d p)

theorem inventory_match_exact [DecidableEq Code]
 {source : List (Declaration Code)} {plans : List (Plan Code)} :
 checkInventory source plans = true ↔ InventoryMatches source plans := by
 simp [checkInventory,InventoryMatches,List.all_eq_true,List.any_eq_true,lower_exact,and_assoc]

-- The actual factory preserves source order. The structural comparator may
-- forget that presentation order only after key uniqueness is established.
theorem ordered_implies_inventory
 (related : InventoryCorresponds source plans)
 (distinct : (source.map sourceKey).Nodup) : InventoryMatches source plans := by
 have equal := inventory_exact.mpr related
 subst plans
 refine ⟨distinct,?_,?_,?_⟩
 · simpa [List.map_map,planKey,sourceKey,lower] using distinct
 · intro d member
   exact ⟨lower d,List.mem_map.mpr ⟨d,member,rfl⟩,lower_exact.mp rfl⟩
 · intro p member
   obtain ⟨d,member,same⟩ := List.mem_map.mp member
   exact ⟨d,member,lower_exact.mp same⟩

theorem inventory_matching_restrict (related : InventoryMatches source plans) (owners : List String) :
 InventoryMatches (source.filter (fun d => owners.contains d.owner))
 (plans.filter (fun p => owners.contains p.owner)) := by
 refine ⟨?_,?_,?_,?_⟩
 · exact ((List.filter_sublist).map sourceKey).nodup related.1
 · exact ((List.filter_sublist).map planKey).nodup related.2.1
 · intro d present
   obtain ⟨member,kept⟩ := List.mem_filter.mp present
   obtain ⟨p,held,matching⟩ := related.2.2.1 d member
   exact ⟨p,List.mem_filter.mpr ⟨held,by simpa [matching.2.2.2.1] using kept⟩,matching⟩
 · intro p present
   obtain ⟨member,kept⟩ := List.mem_filter.mp present
   obtain ⟨d,held,matching⟩ := related.2.2.2 p member
   exact ⟨d,List.mem_filter.mpr ⟨held,by simpa [← matching.2.2.2.1] using kept⟩,matching⟩

theorem matched_plan_source (related : InventoryMatches source plans) (member : plan ∈ plans) :
 ∃ declaration ∈ source, Corresponds declaration plan := related.2.2.2 plan member

def restoreInventory [DecidableEq Code] (expected : List (Declaration Code))
 (candidate : List (Plan Code)) : Option (List (Plan Code)) :=
 if checkInventory expected candidate then some candidate else none

theorem restore_inventory_exact [DecidableEq Code] {expected : List (Declaration Code)}
 {candidate plans : List (Plan Code)} : restoreInventory expected candidate = some plans ↔
 candidate = plans ∧ InventoryMatches expected candidate := by
 unfold restoreInventory
 by_cases accepted : checkInventory expected candidate = true
 · simp [accepted,inventory_match_exact.mp accepted]
 · have wrong : ¬ InventoryMatches expected candidate := by
     simpa only [← inventory_match_exact] using accepted
   simp [accepted,wrong]

namespace Controls
def param : SourceParameter := ⟨⟨"target","Player"⟩,"declaration-span"⟩
def source : Declaration String := ⟨"artifact-A","attack","self","S","statement-1",[param],"ordered full Core + budget"⟩
def plan := lower source
#guard check [source] [plan]
#guard !check [source] [{plan with signature := []}]
#guard !check [source] [{plan with signature := [{param with sourceRef := "different-span"}]}]
#guard !check [source] [{plan with code := "different Core"}]
#guard !check [source] [{plan with owner := "T"}]
#guard !check [source] [plan,plan]
#guard restore [source] [plan] = some [plan]
-- Two altered inventories agreeing with each other does not authenticate them.
#guard check [{source with signature := []}] [{plan with signature := []}]
def source2 := {source with operation := "z-operation",sourceRef := "statement-2"}
#guard checkInventory [source,source2] [lower source2,plan]
#guard !checkInventory [source] [plan,plan]
#guard !checkInventory [source,source] [plan]
#guard !checkInventory [source,source2] [plan]
#guard !checkInventory [source] [plan,lower source2]
#guard !checkInventory [source] [{plan with signature := []}]
#guard restoreInventory [source,source2] [lower source2,plan] = some [lower source2,plan]
def coords : SignatureCoordinates := ⟨"attack",true,some "self",some "S","statement-1"⟩
#guard checkRetained plan coords [("target","self")]
#guard !checkRetained plan {coords with ownerKind := false} [("target","self")]
#guard !checkRetained plan {coords with actor := some "other"} [("target","self")]
#guard !checkRetained plan {coords with sourceRef := "other-span"} [("target","self")]
#guard !checkRetained plan coords []
end Controls
#print axioms invocation_code_result
#print axioms retained_domain_exact
#print axioms ordered_implies_inventory
#print axioms inventory_match_exact
#print axioms inventory_matching_restrict
#print axioms restore_inventory_exact
#print axioms inventory_exact
#print axioms check_exact
#print axioms restrict
#print axioms restore_exact
#print axioms invocation_domain
end MirroreaProofFirst.OwnerInvocationContext
