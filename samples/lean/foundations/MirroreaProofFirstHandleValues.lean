import MirroreaProofFirstModuleContractBoundary
namespace MirroreaProofFirst.HandleValues
-- Opaque module interface reference, freely carried but never an authority grant.
structure Interface (n : Nat) where
 moduleHandle : CurrentUse.Handle n
 operation : CurrentUse.Handle n
 deriving DecidableEq, Repr
-- A single-sort, finite binding fragment. No source parser or distributed closure.
inductive Expr (n : Nat) where
 | reference (h : Interface n)
 | var (index : Nat)
 | bind (value body : Expr n)
 deriving Repr
inductive Scoped : Nat → Expr n → Prop where
 | reference : Scoped depth (.reference h)
 | var : i < depth → Scoped depth (.var i)
 | bind : Scoped depth value → Scoped (depth+1) body → Scoped depth (.bind value body)
def check (depth : Nat) : Expr n → Bool
 | .reference _ => true
 | .var i => decide (i < depth)
 | .bind v b => check depth v && check (depth+1) b
 theorem check_exact (depth : Nat) (e : Expr n) : check depth e = true ↔ Scoped depth e := by
 induction e generalizing depth with
 | reference h => exact ⟨fun _ => .reference,fun _ => rfl⟩
 | var i =>
   simp only [check,decide_eq_true_eq]
   exact ⟨Scoped.var,fun h => by cases h; assumption⟩
 | bind v b hv hb =>
   simp only [check,Bool.and_eq_true,hv,hb]
   exact ⟨fun ⟨a,b⟩ => .bind a b,fun h => by cases h; constructor <;> assumption⟩
def evaluate (env : List (Interface n)) : Expr n → Option (Interface n)
 | .reference h => some h
 | .var i => env[i]?
 | .bind v b => (evaluate env v).bind (fun h => evaluate (h::env) b)
-- Independent execution relation; binding passes the actual value without refresh.
inductive Evaluates : List (Interface n) → Expr n → Interface n → Prop where
 | reference : Evaluates env (.reference h) h
 | var : env[i]? = some h → Evaluates env (.var i) h
 | bind : Evaluates env v h → Evaluates (h::env) b out → Evaluates env (.bind v b) out
 theorem evaluate_exact (env : List (Interface n)) (e : Expr n) (out : Interface n) :
 evaluate env e = some out ↔ Evaluates env e out := by
 constructor
 · induction e generalizing env out with
   | reference h => intro eq; cases eq; exact .reference
   | var i => exact Evaluates.var
   | bind v b hv hb =>
     intro eq
     cases h : evaluate env v with
     | none => simp [evaluate,h] at eq
     | some value => exact .bind (hv _ _ h) (hb _ _ (by simpa [evaluate,h] using eq))
 · intro h; induction h with
   | reference => rfl
   | var h => exact h
   | bind hv hb iv ib => simp [evaluate,iv,ib]
 theorem scoped_terminates {depth : Nat} {e : Expr n} (h : Scoped depth e)
 (env : List (Interface n)) (length : env.length = depth) : ∃ out, evaluate env e = some out := by
 induction h generalizing env with
 | reference => exact ⟨_,rfl⟩
 | @var i depth hi =>
   have bound : i < env.length := by simpa [length] using hi
   exact ⟨env[i],by simp [evaluate,bound]⟩
 | bind hv hb iv ib =>
   obtain ⟨v,ev⟩ := iv env length
   obtain ⟨out,eo⟩ := ib (v::env) (by simp [length])
   exact ⟨out,by simp [evaluate,ev,eo]⟩
def references : Expr n → List (Interface n)
 | .reference h => [h]
 | .var _ => []
 | .bind v b => references v ++ references b
 theorem no_new_reference {env : List (Interface n)} {e : Expr n} {out : Interface n}
 (h : Evaluates env e out) : out ∈ env ∨ out ∈ references e := by
 induction h with
 | reference => exact .inr (by simp [references])
 | var h => exact .inl (List.mem_of_getElem? h)
 | bind hv hb iv ib =>
   rcases ib with member | literal
   · simp only [List.mem_cons] at member
     rcases member with eq | old
     · subst eq
       rcases iv with old | literal
       · exact .inl old
       · exact .inr (by simp [references,literal])
     · exact .inl old
   · exact .inr (by simp [references,literal])
-- Invocation uses the caller's member/principal/request/arguments and the carried
-- interface's exact stamps. Neither caller evidence nor a reference reissues it.
def request (caller : CurrentUse.UseRequest n) (h : Interface n) : CurrentUse.UseRequest n :=
 {caller with moduleHandle := h.moduleHandle, operation := h.operation}
def invoke (s : CurrentUse.World n) (registry : ModuleContractBoundary.Registry n)
 (caller : CurrentUse.UseRequest n) (args : List Int) (auth : CurrentUse.Evidence)
 (proof : ModuleContractBoundary.CallProof n) (env : List (Interface n)) (e : Expr n) : Option Nat :=
 (evaluate env e).bind (fun h => ModuleContractBoundary.call s registry (request caller h) args auth proof)
 theorem invoke_sound {s : CurrentUse.World n} {registry : ModuleContractBoundary.Registry n}
 {caller : CurrentUse.UseRequest n} {args : List Int} {auth : CurrentUse.Evidence}
 {proof : ModuleContractBoundary.CallProof n} {env : List (Interface n)} {e : Expr n} {out : Nat}
 (ok : invoke s registry caller args auth proof env e = some out) :
 ∃ h, Evaluates env e h ∧ ModuleContractBoundary.Successful s registry (request caller h) args proof out := by
 cases he : evaluate env e with
 | none => simp [invoke,he] at ok
 | some h => exact ⟨h,(evaluate_exact _ _ _).mp he,
     ModuleContractBoundary.call_sound (by simpa [invoke,he] using ok)⟩
 theorem no_authority (s : CurrentUse.World n) (registry : ModuleContractBoundary.Registry n)
 (caller : CurrentUse.UseRequest n) (args : List Int) (auth : CurrentUse.Evidence)
 (proof : ModuleContractBoundary.CallProof n) (env : List (Interface n)) (e : Expr n)
 (empty : s.authority.issued = []) : invoke s registry caller args auth proof env e = none := by
 unfold invoke
 cases he : evaluate env e with
 | none => rfl
 | some h => exact ModuleContractBoundary.no_authority_no_call _ _ _ _ _ _ empty
 theorem stale_module (s : CurrentUse.World n) (registry : ModuleContractBoundary.Registry n)
 (caller : CurrentUse.UseRequest n) (args : List Int) (auth : CurrentUse.Evidence)
 (proof : ModuleContractBoundary.CallProof n) (env : List (Interface n)) (e : Expr n)
 (stale : ∀ h, Evaluates env e h → ¬ CurrentUse.CurrentHandle s .module h.moduleHandle) :
 invoke s registry caller args auth proof env e = none := by
 cases eq : invoke s registry caller args auth proof env e with
 | none => rfl
 | some out =>
   obtain ⟨h,ev,success⟩ := invoke_sound eq
   exact False.elim (stale h ev success.current.2.2.1)
namespace Controls
open CurrentUse.Controls ModuleContractBoundary.Controls
 def token : Interface 4 := ⟨CurrentUse.Controls.request.moduleHandle,CurrentUse.Controls.request.operation⟩
 def pass : Expr 4 := .bind (.var 0) (.var 0)
 example : check 1 pass = true := by decide
 example : evaluate [token] pass = some token := by decide
 example : invoke world registry CurrentUse.Controls.request [41] evidence proof [token] pass = some 42 := by decide
 example : invoke retiredWorld registry CurrentUse.Controls.request [41] evidence proof [token] pass = none := by decide
 example : invoke {world with authority := {auth with issued := []}} registry
   CurrentUse.Controls.request [41] evidence proof [token] pass = none := by decide
 def staleToken : Interface 4 := {token with moduleHandle :=
   {token.moduleHandle with identity := {token.moduleHandle.identity with revision := token.moduleHandle.identity.revision+1}}}
 -- Even fresh caller proof cannot replace a carried stale stamp with caller's live one.
 example : invoke world registry CurrentUse.Controls.request [41] evidence proof [staleToken] pass = none := by decide
 example : check 0 pass = false := by decide
end Controls
#print axioms check_exact
#print axioms evaluate_exact
#print axioms scoped_terminates
#print axioms no_new_reference
#print axioms invoke_sound
#print axioms no_authority
#print axioms stale_module
end MirroreaProofFirst.HandleValues

-- Later unreviewed registered-entry extension; old invoke remains explicit.
namespace MirroreaProofFirst.HandleValues
-- Explicit catalog profile; existing invoke remains the weaker retained candidate.
-- Reuse the actual handle evaluator, with no second evaluation or refreshed stamp.
def invokeRegistered (s : CurrentUse.World n) (registry : ModuleContractBoundary.Registry n)
 (catalog : ModuleContractBoundary.Catalog) (caller : CurrentUse.UseRequest n)
 (args : List Int) (auth : CurrentUse.Evidence) (proof : ModuleContractBoundary.CallProof n)
 (env : List (Interface n)) (e : Expr n) : Option Nat :=
 (evaluate env e).bind (fun h => ModuleContractBoundary.catalogCall s registry catalog
   (request caller h) args auth proof)

theorem registered_sound {s : CurrentUse.World n} {registry : ModuleContractBoundary.Registry n}
 {catalog : ModuleContractBoundary.Catalog} {caller : CurrentUse.UseRequest n}
 {args : List Int} {auth : CurrentUse.Evidence} {proof : ModuleContractBoundary.CallProof n}
 {env : List (Interface n)} {e : Expr n} {out : Nat}
 (ok : invokeRegistered s registry catalog caller args auth proof env e = some out) :
 ∃ h, Evaluates env e h ∧
 ModuleContractBoundary.Successful s registry (request caller h) args proof out ∧
 ∃ d, registry h.operation.key = some d ∧ catalog (s.records h.operation.key).code = some d := by
 cases he : evaluate env e with
 | none => simp [invokeRegistered,he] at ok
 | some h =>
   have checked := ModuleContractBoundary.catalog_sound (by simpa [invokeRegistered,he] using ok)
   exact ⟨h,(evaluate_exact _ _ _).mp he,checked⟩

theorem registered_stale (s : CurrentUse.World n) (registry : ModuleContractBoundary.Registry n)
 (catalog : ModuleContractBoundary.Catalog) (caller : CurrentUse.UseRequest n)
 (args : List Int) (auth : CurrentUse.Evidence) (proof : ModuleContractBoundary.CallProof n)
 (env : List (Interface n)) (e : Expr n)
 (stale : ∀ h, Evaluates env e h → ¬ CurrentUse.CurrentHandle s .module h.moduleHandle) :
 invokeRegistered s registry catalog caller args auth proof env e = none := by
 cases eq : invokeRegistered s registry catalog caller args auth proof env e with
 | none => rfl
 | some out =>
   obtain ⟨h,ev,success,_⟩ := registered_sound eq
   exact False.elim (stale h ev success.current.2.2.1)

theorem registered_no_authority (s : CurrentUse.World n) (registry : ModuleContractBoundary.Registry n)
 (catalog : ModuleContractBoundary.Catalog) (caller : CurrentUse.UseRequest n)
 (args : List Int) (auth : CurrentUse.Evidence) (proof : ModuleContractBoundary.CallProof n)
 (env : List (Interface n)) (e : Expr n) (empty : s.authority.issued = []) :
 invokeRegistered s registry catalog caller args auth proof env e = none := by
 cases eq : invokeRegistered s registry catalog caller args auth proof env e with
 | none => rfl
 | some out =>
   obtain ⟨h,_,success,_⟩ := registered_sound eq
   rcases success.current with ⟨_,_,_,_,_,_,_,allowed⟩
   exact False.elim (CurrentUse.no_claim_no_authorization _ _ _ _ empty allowed)

namespace RegisteredControls
open CurrentUse.Controls ModuleContractBoundary.Controls ModuleContractBoundary.DescriptorControl Controls
example : invokeRegistered world registry catalog CurrentUse.Controls.request [41] evidence proof [token] pass = some 42 := by decide
example : invokeRegistered world registry catalog CurrentUse.Controls.request [41] evidence proof [staleToken] pass = none := by decide
example : invokeRegistered world (fun _ => some sameValue) catalog CurrentUse.Controls.request [41] evidence refreshed [token] pass = none := by decide
end RegisteredControls
#print axioms registered_no_authority
#print axioms registered_sound
#print axioms registered_stale
end MirroreaProofFirst.HandleValues
