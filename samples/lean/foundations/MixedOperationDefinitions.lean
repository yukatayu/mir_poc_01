import OwnerNumericBoundary
namespace MirroreaProofFirst.MixedOperationDefinitions
open OwnerEffectService OwnerCheckedArithmetic

-- Concrete finite candidate for the existing definition-catalog boundary.
-- These are checked/materialized field coordinates, NOT final source syntax or
-- transferable authority. Current field ownership/labels, key materialization,
-- resource admission, and authorized installation are distinct later checks.
structure OwnerContract where
 fields : List (Nat × (Nat × Nat)) -- field -> (owner, confidentiality rank)
 arguments : List Nat             -- rank of each captured integer parameter
 control : Nat
 deriving DecidableEq, Repr
structure OwnerDefinition where
 body : Body
 contract : OwnerContract
 deriving DecidableEq, Repr

def ownerAt (c : OwnerContract) (k : Nat) : Nat := ((c.fields.lookup k).getD (0,0)).1
def labelAt (c : OwnerContract) (k : Nat) : Nat := ((c.fields.lookup k).getD (0,0)).2
def capturedAt (c : OwnerContract) (j : Nat) : Nat := (c.arguments[j]?).getD 0

def Scoped (c : OwnerContract) : Checked Nat Nat → Prop
 | .integer _ => True
 | .state k => (c.fields.lookup k).isSome = true
 | .parameter j => j < c.arguments.length
 | .add a b | .sub a b => Scoped c a ∧ Scoped c b

def scopeCheck (c : OwnerContract) : Checked Nat Nat → Bool
 | .integer _ => true
 | .state k => (c.fields.lookup k).isSome
 | .parameter j => decide (j < c.arguments.length)
 | .add a b | .sub a b => scopeCheck c a && scopeCheck c b

theorem scope_exact (tree : Checked Nat Nat) : scopeCheck c tree = true ↔ Scoped c tree := by
 induction tree <;> simp_all [scopeCheck,Scoped]

-- Finite rule-relative definition judgment; may fail at service for absent
-- dynamic values or overflow. It promises no pure-function totality for writes.
def OwnerSatisfies (d : OwnerDefinition) : Prop :=
 (d.contract.fields.map Prod.fst).Nodup ∧
 (d.contract.fields.lookup d.body.target).isSome = true ∧
 Scoped d.contract d.body.tree ∧ OwnerNumericBoundary.Literals 63 d.body.tree ∧
 OwnerSourceFlow.Admissible (ownerAt d.contract) (labelAt d.contract)
   (capturedAt d.contract) d.contract.control [d.body]

def ownerCheck (d : OwnerDefinition) : Bool :=
 decide (d.contract.fields.map Prod.fst).Nodup &&
 (d.contract.fields.lookup d.body.target).isSome && scopeCheck d.contract d.body.tree &&
 OwnerNumericBoundary.literalCheck 63 d.body.tree &&
 OwnerSourceFlow.check (ownerAt d.contract) (labelAt d.contract)
   (capturedAt d.contract) d.contract.control [d.body]

theorem owner_check_exact : ownerCheck d = true ↔ OwnerSatisfies d := by
 simp only [ownerCheck,OwnerSatisfies,Bool.and_eq_true,decide_eq_true_eq,
   scope_exact,OwnerNumericBoundary.literals_exact,OwnerSourceFlow.check_exact,and_assoc]

inductive Contract where
 | pure (contract : InstancePrograms.Contract)
 | owner (contract : OwnerContract)
 deriving DecidableEq, Repr
inductive Definition where
 | pure (definition : InstancePrograms.Definition)
 | owner (definition : OwnerDefinition)
 deriving DecidableEq, Repr

def Definition.contract : Definition → Contract
 | .pure d => .pure d.contract
 | .owner d => .owner d.contract

def Satisfies : Definition → Prop
 | .pure d => InstancePrograms.Satisfies d
 | .owner d => OwnerSatisfies d

def check : Definition → Bool
 | .pure d => InstancePrograms.check d
 | .owner d => ownerCheck d

theorem check_exact (d : Definition) : check d = true ↔ Satisfies d := by
 cases d with
 | pure old => exact InstancePrograms.check_exact old
 | owner op => exact owner_check_exact

-- Provisional narrow replacement: retain the complete declared owner interface;
-- body changes still need independent checks AND current edit/use authorization.
-- A broader effect refinement or changed footprints is not chosen here.
def Refines : Contract → Contract → Prop
 | .pure old,.pure new => InstancePrograms.Refines old new
 | .owner old,.owner new => old = new
 | _,_ => False

def refinementCheck : Contract → Contract → Bool
 | .pure old,.pure new => InstancePrograms.refinementCheck old new
 | .owner old,.owner new => decide (old = new)
 | _,_ => false

theorem refinement_exact (old new : Contract) : refinementCheck old new = true ↔ Refines old new := by
 cases old <;> cases new <;> simp [refinementCheck,Refines,InstancePrograms.refinement_exact]

theorem refines_self (c : Contract) : Refines c c := by
 cases c with
 | pure c => exact InstanceState.refines_self c
 | owner c => rfl

theorem refines_trans (a b c : Contract) (ab : Refines a b) (bc : Refines b c) : Refines a c := by
 cases a <;> cases b <;> cases c <;> simp_all [Refines]
 exact InstanceState.refines_trans _ _ _ ab bc

def exchangeCheck (old : Contract) (new : Definition) : Bool :=
 refinementCheck old new.contract && check new

theorem exchange_exact (old : Contract) (new : Definition) : exchangeCheck old new = true ↔ Refines old new.contract ∧ Satisfies new := by
 simp [exchangeCheck,refinement_exact,check_exact]

def ownerLookup (catalog : List Definition) (key : Nat) : Option OwnerDefinition :=
 match catalog[key]? with | some (.owner d) => some d | _ => none

def bodyLookup (catalog : List Definition) (key : Nat) : Option Body :=
 (ownerLookup catalog key).map OwnerDefinition.body

def pureLookup (catalog : List Definition) (key : Nat) : Option InstancePrograms.Definition :=
 match catalog[key]? with | some (.pure d) => some d | _ => none

theorem owner_lookup_exact : ownerLookup catalog key = some d ↔ catalog[key]? = some (.owner d) := by
 unfold ownerLookup
 cases catalog[key]? with
 | none => simp
 | some entry => cases entry <;> simp

theorem pure_cannot_be_owner (found : catalog[key]? = some (.pure d)) :
 ownerLookup catalog key = none ∧ bodyLookup catalog key = none := by
 simp [ownerLookup,bodyLookup,found]

theorem pure_embedding (old : List InstancePrograms.Definition) (key : Nat) :
 pureLookup (old.map Definition.pure) key = old[key]? ∧
 ownerLookup (old.map Definition.pure) key = none := by
 simp only [pureLookup,ownerLookup,List.getElem?_map]
 cases old[key]? <;> exact ⟨rfl,rfl⟩

-- Structural append only. This helper is NOT an authorized registration entry;
-- use its exact rule under the existing full-payload management boundary later.
def appendChecked (catalog : List Definition) (new : Definition) : Option (List Definition) :=
 if check new then some (catalog ++ [new]) else none

theorem append_exact : appendChecked catalog new = some next ↔ Satisfies new ∧ next = catalog ++ [new] := by
 unfold appendChecked
 split
 · simp [(check_exact new).mp ‹_›,eq_comm]
 · have no : ¬ Satisfies new := fun yes => ‹check new ≠ true› ((check_exact new).mpr yes)
   simp [no]

theorem append_old (accepted : appendChecked catalog new = some next) (old : key < catalog.length) :
 next[key]? = catalog[key]? := by
 obtain ⟨_,rfl⟩ := append_exact.mp accepted
 simp [List.getElem?_append_left,old]

theorem append_owner (accepted : appendChecked catalog (.owner d) = some next) :
 ownerLookup next catalog.length = some d := by
 obtain ⟨_,rfl⟩ := append_exact.mp accepted
 apply owner_lookup_exact.mpr
 simp

namespace Controls
def contract : OwnerContract := ⟨[(0,(7,0))],[0],0⟩
def add : OwnerDefinition := ⟨⟨0,.add (.state 0) (.parameter 0)⟩,contract⟩
#guard ownerCheck add
#guard !ownerCheck {add with contract := {contract with fields := []}}
#guard !ownerCheck {add with contract := {contract with fields := [(0,(7,0)),(0,(8,1))]}}
#guard !ownerCheck {add with contract := {contract with arguments := [1]}}
#guard !ownerCheck {add with body := ⟨0,.parameter 1⟩}
#guard !exchangeCheck (.pure InstancePrograms.Controls.contract) (.owner add)
#guard !exchangeCheck (.owner contract) (.pure InstancePrograms.Controls.original)
def catalog := [Definition.pure InstancePrograms.Controls.original]
def registered := appendChecked catalog (.owner add)
#guard registered.isSome
#guard (registered.bind fun c => pureLookup c 0) = some InstancePrograms.Controls.original
#guard (registered.bind fun c => ownerLookup c 0) = none
#guard (registered.bind fun c => ownerLookup c 1) = some add
#guard (registered.bind fun c => bodyLookup c 1).bind (fun b =>
 evaluate (FallibleFlow.signed 63) (fun _ => some 10) (fun _ => some 5) b.tree) = some 15
-- Arithmetic outcome only; no service or authority inferred from this control.
end Controls
#print axioms owner_check_exact
#print axioms check_exact
#print axioms exchange_exact
#print axioms owner_lookup_exact
#print axioms pure_embedding
#print axioms append_exact
#print axioms append_old
#print axioms append_owner
end MirroreaProofFirst.MixedOperationDefinitions
