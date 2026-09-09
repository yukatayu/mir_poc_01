import MirroreaProofFirstHandleValues
namespace MirroreaProofFirst.PureHandleFunctions
variable {size : Nat}
inductive Ty where
 | int | nat | handle
 | arrow (domain codomain : Ty)
 deriving DecidableEq, Repr
inductive Expr (size : Nat) where
 | handle (h : HandleValues.Interface size)
 | integer (z : Int)
 | natural (n : Nat)
 | var (index : Nat)
 | add (a b : Expr size)
 | mul (a b : Expr size)
 | lambda (parameter : Ty) (body : Expr size)
 | app (fn argument : Expr size)
 -- The iteration body binds its current value at index0; the outer environment
 -- remains available. No mutable region ownership or effect continuation here.
 | iterate (type : Ty) (count initial body : Expr size)
 deriving DecidableEq, Repr
inductive Typed : List Ty → Expr size → Ty → Prop where
 | handle {G h} : Typed G (.handle h) .handle
 | integer {G z} : Typed G (.integer z) .int
 | natural {G n} : Typed G (.natural n) .nat
 | var {G i t} : G[i]? = some t → Typed G (.var i) t
 | add {G a b} : Typed G a .int → Typed G b .int → Typed G (.add a b) .int
 | mul {G a b} : Typed G a .int → Typed G b .int → Typed G (.mul a b) .int
 | lambda {G a b body} : Typed (a::G) body b → Typed G (.lambda a body) (.arrow a b)
 | app {G fn arg a b} : Typed G fn (.arrow a b) → Typed G arg a → Typed G (.app fn arg) b
 | iterate {G count initial body t} : Typed G count .nat → Typed G initial t →
     Typed (t::G) body t → Typed G (.iterate t count initial body) t

def infer (G : List Ty) : Expr size → Option Ty
 | .handle _ => some .handle
 | .integer _ => some .int
 | .natural _ => some .nat
 | .var i => G[i]?
 | .add a b | .mul a b => if infer G a = some .int ∧ infer G b = some .int then some .int else none
 | .lambda a body => (infer (a::G) body).map (.arrow a)
 | .app fn arg => match infer G fn with
   | some (.arrow a b) => if infer G arg = some a then some b else none
   | _ => none
 | .iterate t count initial body =>
   if infer G count = some .nat ∧ infer G initial = some t ∧ infer (t::G) body = some t then some t else none

theorem typed_infer {G : List Ty} {e : Expr size} {t : Ty} (h : Typed G e t) : infer G e = some t := by
 induction h <;> simp_all [infer]

theorem infer_typed (G : List Ty) (e : Expr size) (t : Ty) (h : infer G e = some t) : Typed G e t := by
 induction e generalizing G t with
 | handle hval => cases h; exact .handle
 | integer z => cases h; exact .integer
 | natural n => cases h; exact .natural
 | var i => exact .var h
 | add a b ha hb =>
   simp only [infer] at h; split at h
   · rename_i inputs; cases h; exact .add (ha _ _ inputs.1) (hb _ _ inputs.2)
   · contradiction
 | mul a b ha hb =>
   simp only [infer] at h; split at h
   · rename_i inputs; cases h; exact .mul (ha _ _ inputs.1) (hb _ _ inputs.2)
   · contradiction
 | lambda a body ih =>
   cases hb : infer (a::G) body <;> simp [infer,hb] at h
   cases h; exact .lambda (ih _ _ hb)
 | app fn arg hf ha =>
   cases hfn : infer G fn with
   | none => simp [infer,hfn] at h
   | some ty =>
     cases ty with
     | handle => simp [infer,hfn] at h
     | int => simp [infer,hfn] at h
     | nat => simp [infer,hfn] at h
     | arrow a b =>
       simp only [infer,hfn] at h; split at h
       · rename_i harr; cases h; exact .app (hf _ _ hfn) (ha _ _ harr)
       · contradiction
 | iterate ty count initial body hc hi hb =>
   simp only [infer] at h; split at h
   · rename_i all; cases h
     exact .iterate (hc _ _ all.1) (hi _ _ all.2.1) (hb _ _ all.2.2)
   · contradiction

theorem infer_exact (G : List Ty) (e : Expr size) (t : Ty) : infer G e = some t ↔ Typed G e t :=
 ⟨infer_typed G e t,typed_infer⟩

inductive Value (size : Nat) where
 | handle (h : HandleValues.Interface size)
 | integer (z : Int)
 | natural (n : Nat)
 | closure (parameter : Ty) (body : Expr size) (environment : List (Value size))
 deriving Repr

-- Captured environment typing is explicit, independent of interpreter success.
mutual
inductive HasType : Value size → Ty → Prop where
 | handle {h} : HasType (.handle h) .handle
 | integer {z} : HasType (.integer z) .int
 | natural {n} : HasType (.natural n) .nat
 | closure {a b body env G} : EnvTyped env G → Typed (a::G) body b →
     HasType (.closure a body env) (.arrow a b)
inductive EnvTyped : List (Value size) → List Ty → Prop where
 | nil : EnvTyped [] []
 | cons {v t env G} : HasType v t → EnvTyped env G → EnvTyped (v::env) (t::G)
end

mutual
 def evaluate : Nat → List (Value size) → Expr size → Option (Value size)
 | 0,_,_ => none
 | fuel+1,env,e => match e with
   | .handle h => some (.handle h)
   | .integer z => some (.integer z)
   | .natural n => some (.natural n)
   | .var i => env[i]?
   | .add a b => match evaluate fuel env a, evaluate fuel env b with
     | some (.integer x), some (.integer y) => some (.integer (x+y))
     | _,_ => none
   | .mul a b => match evaluate fuel env a, evaluate fuel env b with
     | some (.integer x), some (.integer y) => some (.integer (x*y))
     | _,_ => none
   | .lambda a body => some (.closure a body env)
   | .app fn arg => match evaluate fuel env fn, evaluate fuel env arg with
     | some (.closure _ body captured), some value => evaluate fuel (value::captured) body
     | _,_ => none
   | .iterate _ count initial body => match evaluate fuel env count, evaluate fuel env initial with
     | some (.natural n), some value => iterateValues fuel n env body value
     | _,_ => none
 def iterateValues : Nat → Nat → List (Value size) → Expr size → Value size → Option (Value size)
 | 0,_,_,_,_ => none
 | _+1,0,_,_,v => some v
 | fuel+1,n+1,env,body,v => do
   let next ← evaluate fuel (v::env) body
   iterateValues fuel n env body next
end

theorem environment_lookup {env : List (Value size)} {G : List Ty} (typed : EnvTyped env G)
 (i : Nat) {t : Ty} {v : Value size} (ht : G[i]? = some t) (hv : env[i]? = some v) : HasType v t := by
 induction i generalizing env G with
 | zero =>
   cases typed with
   | nil => simp at ht
   | cons head tail => simp at ht hv; subst t; subst v; exact head
 | succ i ih =>
   cases typed with
   | nil => simp at ht
   | cons head tail => exact ih tail (by simpa using ht) (by simpa using hv)

theorem evaluator_typed (fuel : Nat) :
 (∀ {env : List (Value size)} {G e t v}, EnvTyped env G → Typed G e t → evaluate fuel env e = some v → HasType v t) ∧
 (∀ {n} {env : List (Value size)} {G body t initial v}, EnvTyped env G → Typed (t::G) body t → HasType initial t →
   iterateValues fuel n env body initial = some v → HasType v t) := by
 induction fuel with
 | zero => constructor <;> intros <;> contradiction
 | succ fuel ih =>
   constructor
   · intro env G e t v he ht hv
     cases ht with
     | handle => cases hv; exact .handle
     | integer => cases hv; exact .integer
     | natural => cases hv; exact .natural
     | var lookup => exact environment_lookup he _ lookup hv
     | add ha hb =>
       simp only [evaluate] at hv; split at hv
       · cases hv; exact .integer
       · contradiction
     | mul ha hb =>
       simp only [evaluate] at hv; split at hv
       · cases hv; exact .integer
       · contradiction
     | lambda bodyType => cases hv; exact .closure he bodyType
     | @app G fn arg a b fnType argType =>
       cases ef : evaluate fuel env fn with
       | none => simp [evaluate,ef] at hv
       | some fnValue =>
         cases ea : evaluate fuel env arg with
         | none => simp [evaluate,ef,ea] at hv
         | some argValue =>
           have fnTyped := ih.1 he fnType ef
           have argTyped := ih.1 he argType ea
           cases fnTyped with
           | closure capturedType bodyType =>
             exact ih.1 (.cons argTyped capturedType) bodyType (by simpa [evaluate,ef,ea] using hv)
     | @iterate G count initial body ty countType initType bodyType =>
       cases ec : evaluate fuel env count with
       | none => simp [evaluate,ec] at hv
       | some countValue =>
         have countTyped := ih.1 he countType ec
         cases countTyped with
         | natural =>
           cases ei : evaluate fuel env initial with
           | none => simp [evaluate,ec,ei] at hv
           | some initialValue =>
             exact ih.2 he bodyType (ih.1 he initType ei) (by simpa [evaluate,ec,ei] using hv)
   · intro n env G body t initial v he hb hi hv
     cases n with
     | zero => cases hv; exact hi
     | succ n =>
       cases eb : evaluate fuel (initial::env) body with
       | none => simp [iterateValues,eb] at hv
       | some next =>
         have hn := ih.1 (.cons hi he) hb eb
         exact ih.2 he hb hn (by simpa [iterateValues,eb] using hv)

inductive Task (size : Nat) where
 | expression (env : List (Value size)) (e : Expr size)
 | iteration (n : Nat) (env : List (Value size)) (body : Expr size) (initial : Value size)
inductive Executes : Task size → Value size → Prop where
 | handle {env h} : Executes (.expression env (.handle h)) (.handle h)
 | integer {env z} : Executes (.expression env (.integer z)) (.integer z)
 | natural {env n} : Executes (.expression env (.natural n)) (.natural n)
 | lookupValue {env i v} : env[i]? = some v → Executes (.expression env (.var i)) v
 | addition {env a b x y} : Executes (.expression env a) (.integer x) →
     Executes (.expression env b) (.integer y) → Executes (.expression env (.add a b)) (.integer (x+y))
 | multiply {env a b x y} : Executes (.expression env a) (.integer x) →
     Executes (.expression env b) (.integer y) → Executes (.expression env (.mul a b)) (.integer (x*y))
 | lambda {env a body} : Executes (.expression env (.lambda a body)) (.closure a body env)
 | application {env fn arg a body captured value out} :
     Executes (.expression env fn) (.closure a body captured) →
     Executes (.expression env arg) value → Executes (.expression (value::captured) body) out →
     Executes (.expression env (.app fn arg)) out
 | iteration {env ty count initial body n v out} :
     Executes (.expression env count) (.natural n) → Executes (.expression env initial) v →
     Executes (.iteration n env body v) out → Executes (.expression env (.iterate ty count initial body)) out
 | zero {env body v} : Executes (.iteration 0 env body v) v
 | step {env body n v next out} : Executes (.expression (v::env) body) next →
     Executes (.iteration n env body next) out → Executes (.iteration (n+1) env body v) out

def runTask (fuel : Nat) : Task size → Option (Value size)
 | .expression env e => evaluate fuel env e
 | .iteration n env body initial => iterateValues fuel n env body initial

-- Completeness is relative to finite declarative executions. It neither asserts
-- a global fixed fuel bound nor treats exhausted fuel as a source type error.
theorem execution_complete {task : Task size} {v : Value size} (h : Executes task v) :
 ∃ minimum, ∀ fuel, minimum ≤ fuel → runTask fuel task = some v := by
 induction h with
 | handle => exact ⟨1,by intro fuel enough; cases fuel <;> simp_all [runTask,evaluate]⟩
 | integer => exact ⟨1,by intro fuel enough; cases fuel <;> simp_all [runTask,evaluate]⟩
 | natural => exact ⟨1,by intro fuel enough; cases fuel <;> simp_all [runTask,evaluate]⟩
 | lookupValue lookup => exact ⟨1,by intro fuel enough; cases fuel <;> simp_all [runTask,evaluate]⟩
 | lambda => exact ⟨1,by intro fuel enough; cases fuel <;> simp_all [runTask,evaluate]⟩
 | addition left right il ir =>
   obtain ⟨l,hl⟩ := il; obtain ⟨r,hr⟩ := ir
   refine ⟨max l r+1,?_⟩
   intro fuel enough; cases fuel with
   | zero => omega
   | succ fuel =>
     have al := hl fuel (by omega); have ar := hr fuel (by omega)
     simp only [runTask] at al ar
     simp [runTask,evaluate,al,ar]
 | multiply left right il ir =>
   obtain ⟨l,hl⟩ := il; obtain ⟨r,hr⟩ := ir
   refine ⟨max l r+1,?_⟩
   intro fuel enough; cases fuel with
   | zero => omega
   | succ fuel =>
     have al := hl fuel (by omega); have ar := hr fuel (by omega)
     simp only [runTask] at al ar
     simp [runTask,evaluate,al,ar]
 | application f a body fi ai bi =>
   obtain ⟨x,hx⟩ := fi; obtain ⟨y,hy⟩ := ai; obtain ⟨z,hz⟩ := bi
   refine ⟨max x (max y z)+1,?_⟩
   intro fuel enough; cases fuel with
   | zero => omega
   | succ fuel =>
     have ax := hx fuel (by omega); have ay := hy fuel (by omega); have az := hz fuel (by omega)
     simp only [runTask] at ax ay az
     simp [runTask,evaluate,ax,ay,az]
 | iteration c i loop ci ii li =>
   obtain ⟨x,hx⟩ := ci; obtain ⟨y,hy⟩ := ii; obtain ⟨z,hz⟩ := li
   refine ⟨max x (max y z)+1,?_⟩
   intro fuel enough; cases fuel with
   | zero => omega
   | succ fuel =>
     have ax := hx fuel (by omega); have ay := hy fuel (by omega); have az := hz fuel (by omega)
     simp only [runTask] at ax ay az
     simp [runTask,evaluate,ax,ay,az]
 | zero => exact ⟨1,by intro fuel enough; cases fuel <;> simp_all [runTask,iterateValues]⟩
 | step body loop bi li =>
   obtain ⟨x,hx⟩ := bi; obtain ⟨y,hy⟩ := li
   refine ⟨max x y+1,?_⟩
   intro fuel enough; cases fuel with
   | zero => omega
   | succ fuel =>
     have ax := hx fuel (by omega); have ay := hy fuel (by omega)
     simp only [runTask] at ax ay
     simp [runTask,iterateValues,ax,ay]

theorem execution_sound (fuel : Nat) (task : Task size) (v : Value size)
 (h : runTask fuel task = some v) : Executes task v := by
 induction fuel generalizing task v with
 | zero => cases task <;> contradiction
 | succ fuel ih =>
   cases task with
   | expression env e =>
     cases e with
     | handle hval => cases h; exact .handle
     | integer z => cases h; exact .integer
     | natural n => cases h; exact .natural
     | var i => exact .lookupValue h
     | lambda a body => cases h; exact .lambda
     | add a b =>
       simp only [runTask,evaluate] at h
       split at h
       · rename_i x y ea eb
         cases h
         exact .addition (ih _ _ ea) (ih _ _ eb)
       · contradiction
     | mul a b =>
       simp only [runTask,evaluate] at h
       split at h
       · rename_i x y ea eb
         cases h
         exact .multiply (ih _ _ ea) (ih _ _ eb)
       · contradiction
     | app fn arg =>
       simp only [runTask,evaluate] at h
       split at h
       · rename_i a body captured value ef ea
         exact .application (ih _ _ ef) (ih _ _ ea) (ih _ _ h)
       · contradiction
     | iterate ty count initial body =>
       simp only [runTask,evaluate] at h
       split at h
       · rename_i n value ec ei
         exact .iteration (ih _ _ ec) (ih _ _ ei) (ih _ _ h)
       · contradiction
   | iteration n env body initial =>
     cases n with
     | zero => cases h; exact .zero
     | succ n =>
       cases eb : evaluate fuel (initial::env) body with
       | none => simp [runTask,iterateValues,eb] at h
       | some next =>
         exact .step (ih _ _ eb) (ih _ _ (by simpa [runTask,iterateValues,eb] using h))

theorem execution_exact (task : Task size) (v : Value size) :
 (∃ fuel, runTask fuel task = some v) ↔ Executes task v := by
 constructor
 · rintro ⟨fuel,h⟩; exact execution_sound fuel task v h
 · intro h; obtain ⟨fuel,hf⟩ := execution_complete h; exact ⟨fuel,hf fuel (Nat.le_refl _)⟩


-- Same closure evaluator accepts an interface reference as ordinary argument.
def identity : Expr size := .lambda .handle (.var 0)
def twice : Expr size := .lambda (.arrow .handle .handle)
 (.lambda .handle (.app (.var 1) (.app (.var 1) (.var 0))))
def carried (h : HandleValues.Interface size) : Expr size := .app (.app twice identity) (.handle h)
theorem carried_typed (h : HandleValues.Interface size) : Typed [] (carried h) .handle := by
 apply (infer_exact _ _ _).mp
 rfl
theorem carried_evaluates (h : HandleValues.Interface size) :
 evaluate 10 [] (carried h) = some (.handle h) := by rfl
-- Pure evaluation may carry/duplicate a reference, never a linear resource or grant.
def invoke (fuel : Nat) (s : CurrentUse.World size)
 (registry : ModuleContractBoundary.Registry size) (caller : CurrentUse.UseRequest size)
 (args : List Int) (auth : CurrentUse.Evidence) (proof : ModuleContractBoundary.CallProof size)
 (env : List (Value size)) (e : Expr size) : Option Nat :=
 match evaluate fuel env e with
 | some (.handle h) => ModuleContractBoundary.call s registry (HandleValues.request caller h) args auth proof
 | _ => none
 theorem invoke_sound {fuel : Nat} {s : CurrentUse.World size}
 {registry : ModuleContractBoundary.Registry size} {caller : CurrentUse.UseRequest size}
 {args : List Int} {auth : CurrentUse.Evidence} {proof : ModuleContractBoundary.CallProof size}
 {env : List (Value size)} {e : Expr size} {out : Nat}
 (ok : invoke fuel s registry caller args auth proof env e = some out) :
 ∃ h, Executes (.expression env e) (.handle h) ∧
 ModuleContractBoundary.Successful s registry (HandleValues.request caller h) args proof out := by
 unfold invoke at ok
 split at ok
 · rename_i h he
   exact ⟨h,execution_sound fuel _ _ he,ModuleContractBoundary.call_sound ok⟩
 · contradiction
 theorem no_authority (fuel : Nat) (s : CurrentUse.World size)
 (registry : ModuleContractBoundary.Registry size) (caller : CurrentUse.UseRequest size)
 (args : List Int) (auth : CurrentUse.Evidence) (proof : ModuleContractBoundary.CallProof size)
 (env : List (Value size)) (e : Expr size) (empty : s.authority.issued = []) :
 invoke fuel s registry caller args auth proof env e = none := by
 unfold invoke; split
 · exact ModuleContractBoundary.no_authority_no_call _ _ _ _ _ _ empty
 · rfl
 theorem stale_module (fuel : Nat) (s : CurrentUse.World size)
 (registry : ModuleContractBoundary.Registry size) (caller : CurrentUse.UseRequest size)
 (args : List Int) (auth : CurrentUse.Evidence) (proof : ModuleContractBoundary.CallProof size)
 (env : List (Value size)) (e : Expr size)
 (stale : ∀ h, Executes (.expression env e) (.handle h) →
   ¬ CurrentUse.CurrentHandle s .module h.moduleHandle) :
 invoke fuel s registry caller args auth proof env e = none := by
 cases eq : invoke fuel s registry caller args auth proof env e with
 | none => rfl
 | some out =>
   obtain ⟨h,ev,success⟩ := invoke_sound eq
   exact False.elim (stale h ev success.current.2.2.1)
 theorem carried_invokes (s : CurrentUse.World size)
 (registry : ModuleContractBoundary.Registry size) (caller : CurrentUse.UseRequest size)
 (args : List Int) (auth : CurrentUse.Evidence) (proof : ModuleContractBoundary.CallProof size)
 (h : HandleValues.Interface size) :
 invoke 10 s registry caller args auth proof [] (carried h) =
 ModuleContractBoundary.call s registry (HandleValues.request caller h) args auth proof := by
 simp [invoke,carried_evaluates]
namespace Controls
open CurrentUse.Controls ModuleContractBoundary.Controls HandleValues.Controls
 example : invoke 10 world registry CurrentUse.Controls.request [41] evidence proof [] (carried token) = some 42 := by decide
 example : invoke 10 world registry CurrentUse.Controls.request [41] evidence proof [] (carried staleToken) = none := by decide
 example : invoke 10 retiredWorld registry CurrentUse.Controls.request [41] evidence proof [] (carried token) = none := by decide
 example : infer [] (.add (.handle token) (.integer 1)) = none := by decide
 example : evaluate 30 [] (.iterate .handle (.natural 10) (.handle token) (.var 0)) = some (.handle token) := by rfl
end Controls
#print axioms invoke_sound
#print axioms no_authority
#print axioms stale_module
#print axioms carried_invokes
#print axioms infer_exact
#print axioms evaluator_typed
#print axioms execution_complete
#print axioms execution_sound
#print axioms execution_exact
#print axioms carried_typed
#print axioms carried_evaluates
end MirroreaProofFirst.PureHandleFunctions

-- Later unreviewed registered-entry extension; old invoke remains explicit.
namespace MirroreaProofFirst.PureHandleFunctions
-- Pure closures/iteration are unchanged; the selected profile passes the actual
-- returned interface to the same catalog/currentness/contract boundary.
def invokeRegistered (fuel : Nat) (s : CurrentUse.World size)
 (registry : ModuleContractBoundary.Registry size) (catalog : ModuleContractBoundary.Catalog)
 (caller : CurrentUse.UseRequest size) (args : List Int) (auth : CurrentUse.Evidence)
 (proof : ModuleContractBoundary.CallProof size) (env : List (Value size)) (e : Expr size) : Option Nat :=
 match evaluate fuel env e with
 | some (.handle h) => ModuleContractBoundary.catalogCall s registry catalog
     (HandleValues.request caller h) args auth proof
 | _ => none

theorem registered_sound {fuel : Nat} {s : CurrentUse.World size}
 {registry : ModuleContractBoundary.Registry size} {catalog : ModuleContractBoundary.Catalog}
 {caller : CurrentUse.UseRequest size} {args : List Int} {auth : CurrentUse.Evidence}
 {proof : ModuleContractBoundary.CallProof size} {env : List (Value size)} {e : Expr size} {out : Nat}
 (ok : invokeRegistered fuel s registry catalog caller args auth proof env e = some out) :
 ∃ h, Executes (.expression env e) (.handle h) ∧
 ModuleContractBoundary.Successful s registry (HandleValues.request caller h) args proof out ∧
 ∃ d, registry h.operation.key = some d ∧ catalog (s.records h.operation.key).code = some d := by
 unfold invokeRegistered at ok; split at ok
 · rename_i h he
   exact ⟨h,execution_sound fuel _ _ he,ModuleContractBoundary.catalog_sound ok⟩
 · contradiction

theorem registered_carried (s : CurrentUse.World size) (registry : ModuleContractBoundary.Registry size)
 (catalog : ModuleContractBoundary.Catalog) (caller : CurrentUse.UseRequest size)
 (args : List Int) (auth : CurrentUse.Evidence) (proof : ModuleContractBoundary.CallProof size)
 (h : HandleValues.Interface size) :
 invokeRegistered 10 s registry catalog caller args auth proof [] (carried h) =
 ModuleContractBoundary.catalogCall s registry catalog (HandleValues.request caller h) args auth proof := by
 simp [invokeRegistered,carried_evaluates]

theorem registered_stale (fuel : Nat) (s : CurrentUse.World size)
 (registry : ModuleContractBoundary.Registry size) (catalog : ModuleContractBoundary.Catalog)
 (caller : CurrentUse.UseRequest size) (args : List Int) (auth : CurrentUse.Evidence)
 (proof : ModuleContractBoundary.CallProof size) (env : List (Value size)) (e : Expr size)
 (stale : ∀ h, Executes (.expression env e) (.handle h) →
   ¬ CurrentUse.CurrentHandle s .module h.moduleHandle) :
 invokeRegistered fuel s registry catalog caller args auth proof env e = none := by
 cases eq : invokeRegistered fuel s registry catalog caller args auth proof env e with
 | none => rfl
 | some out =>
   obtain ⟨h,ev,success,_⟩ := registered_sound eq
   exact False.elim (stale h ev success.current.2.2.1)

theorem registered_no_authority (fuel : Nat) (s : CurrentUse.World size)
 (registry : ModuleContractBoundary.Registry size) (catalog : ModuleContractBoundary.Catalog)
 (caller : CurrentUse.UseRequest size) (args : List Int) (auth : CurrentUse.Evidence)
 (proof : ModuleContractBoundary.CallProof size) (env : List (Value size)) (e : Expr size)
 (empty : s.authority.issued = []) :
 invokeRegistered fuel s registry catalog caller args auth proof env e = none := by
 cases eq : invokeRegistered fuel s registry catalog caller args auth proof env e with
 | none => rfl
 | some out =>
   obtain ⟨h,_,success,_⟩ := registered_sound eq
   rcases success.current with ⟨_,_,_,_,_,_,_,allowed⟩
   exact False.elim (CurrentUse.no_claim_no_authorization _ _ _ _ empty allowed)

-- Closed checked-expression entry: arguments are explicit expression values,
-- not an externally supplied untyped closure environment. No parser is claimed.
def invokeClosedRegistered (fuel : Nat) (s : CurrentUse.World size)
 (registry : ModuleContractBoundary.Registry size) (catalog : ModuleContractBoundary.Catalog)
 (caller : CurrentUse.UseRequest size) (args : List Int) (auth : CurrentUse.Evidence)
 (proof : ModuleContractBoundary.CallProof size) (e : Expr size) : Option Nat :=
 if infer [] e = some .handle then invokeRegistered fuel s registry catalog caller args auth proof [] e else none

theorem closed_registered_sound {fuel : Nat} {s : CurrentUse.World size}
 {registry : ModuleContractBoundary.Registry size} {catalog : ModuleContractBoundary.Catalog}
 {caller : CurrentUse.UseRequest size} {args : List Int} {auth : CurrentUse.Evidence}
 {proof : ModuleContractBoundary.CallProof size} {e : Expr size} {out : Nat}
 (ok : invokeClosedRegistered fuel s registry catalog caller args auth proof e = some out) :
 Typed [] e .handle ∧ ∃ h, Executes (.expression [] e) (.handle h) ∧
 ModuleContractBoundary.Successful s registry (HandleValues.request caller h) args proof out ∧
 ∃ d, registry h.operation.key = some d ∧ catalog (s.records h.operation.key).code = some d := by
 unfold invokeClosedRegistered at ok; split at ok
 · rename_i typed
   exact ⟨(infer_exact _ _ _).mp typed,registered_sound ok⟩
 · contradiction

theorem closed_registered_complete {fuel : Nat} {s : CurrentUse.World size}
 {registry : ModuleContractBoundary.Registry size} {catalog : ModuleContractBoundary.Catalog}
 {caller : CurrentUse.UseRequest size} {args : List Int} {auth : CurrentUse.Evidence}
 {proof : ModuleContractBoundary.CallProof size} {e : Expr size} {out : Nat}
 (typed : Typed [] e .handle)
 (exec : invokeRegistered fuel s registry catalog caller args auth proof [] e = some out) :
 invokeClosedRegistered fuel s registry catalog caller args auth proof e = some out := by
 simp [invokeClosedRegistered,(infer_exact _ _ _).mpr typed,exec]

-- Relative completeness uses an independent finite execution derivation, not a
-- successful invocation as the expression evaluator's premise.
theorem closed_registered_derivation_complete {s : CurrentUse.World size}
 {registry : ModuleContractBoundary.Registry size} {catalog : ModuleContractBoundary.Catalog}
 {caller : CurrentUse.UseRequest size} {args : List Int} {auth : CurrentUse.Evidence}
 {proof : ModuleContractBoundary.CallProof size} {e : Expr size} {out : Nat}
 {h : HandleValues.Interface size} (typed : Typed [] e .handle)
 (exec : Executes (.expression [] e) (.handle h))
 (accepted : ModuleContractBoundary.catalogCall s registry catalog
   (HandleValues.request caller h) args auth proof = some out) :
 ∃ fuel, invokeClosedRegistered fuel s registry catalog caller args auth proof e = some out := by
 obtain ⟨fuel,ev⟩ := (execution_exact _ _).mpr exec
 have evaluated : evaluate fuel [] e = some (.handle h) := ev
 exact ⟨fuel,closed_registered_complete typed (by simp [invokeRegistered,evaluated,accepted])⟩

theorem closed_registered_carried (s : CurrentUse.World size)
 (registry : ModuleContractBoundary.Registry size) (catalog : ModuleContractBoundary.Catalog)
 (caller : CurrentUse.UseRequest size) (args : List Int) (auth : CurrentUse.Evidence)
 (proof : ModuleContractBoundary.CallProof size) (h : HandleValues.Interface size) :
 invokeClosedRegistered 10 s registry catalog caller args auth proof (carried h) =
 ModuleContractBoundary.catalogCall s registry catalog (HandleValues.request caller h) args auth proof := by
 simp [invokeClosedRegistered,(infer_exact _ _ _).mpr (carried_typed h),registered_carried]

namespace RegisteredControls
open CurrentUse.Controls ModuleContractBoundary.Controls ModuleContractBoundary.DescriptorControl HandleValues.Controls
example : invokeRegistered 10 world registry catalog CurrentUse.Controls.request [41] evidence proof [] (carried token) = some 42 := by decide
example : invokeRegistered 10 world registry catalog CurrentUse.Controls.request [41] evidence proof [] (carried staleToken) = none := by decide
example : invokeRegistered 10 world (fun _ => some sameValue) catalog CurrentUse.Controls.request [41] evidence refreshed [] (carried token) = none := by decide
-- Operational invocation alone is not source typing admission.
def illTyped : Expr 4 := .app (.lambda .int (.handle token)) (.handle token)
example : infer [] illTyped = none := by decide
example : invokeRegistered 10 world registry catalog CurrentUse.Controls.request [41] evidence proof [] illTyped = some 42 := by decide
example : invokeClosedRegistered 10 world registry catalog CurrentUse.Controls.request [41] evidence proof illTyped = none := by decide
example : invokeClosedRegistered 10 world registry catalog CurrentUse.Controls.request [41] evidence proof (carried token) = some 42 := by decide
example : invokeClosedRegistered 10 world (fun _ => some sameValue) catalog CurrentUse.Controls.request [41] evidence refreshed (carried token) = none := by decide
end RegisteredControls
#print axioms registered_no_authority
#print axioms registered_sound
#print axioms registered_stale
#print axioms closed_registered_sound
#print axioms closed_registered_complete
#print axioms closed_registered_derivation_complete
#print axioms closed_registered_carried
#print axioms registered_carried
end MirroreaProofFirst.PureHandleFunctions

namespace MirroreaProofFirst.PureHandleFunctions.ReferencePreservation
variable {size : Nat}
-- P bounds existing references, not permission, currentness, or confidentiality.
def ExprWithin (P : HandleValues.Interface size → Prop) : Expr size → Prop
 | .handle h => P h
 | .integer _ | .natural _ | .var _ => True
 | .add a b | .mul a b | .app a b => ExprWithin P a ∧ ExprWithin P b
 | .lambda _ body => ExprWithin P body
 | .iterate _ count initial body => ExprWithin P count ∧ ExprWithin P initial ∧ ExprWithin P body
mutual
inductive ValueWithin (P : HandleValues.Interface size → Prop) : Value size → Prop where
 | handle : P h → ValueWithin P (.handle h)
 | integer : ValueWithin P (.integer z)
 | natural : ValueWithin P (.natural n)
 | closure : ExprWithin P body → EnvWithin P env → ValueWithin P (.closure a body env)
inductive EnvWithin (P : HandleValues.Interface size → Prop) : List (Value size) → Prop where
 | nil : EnvWithin P []
 | cons : ValueWithin P v → EnvWithin P env → EnvWithin P (v::env)
end
def TaskWithin (P : HandleValues.Interface size → Prop) : Task size → Prop
 | .expression env e => EnvWithin P env ∧ ExprWithin P e
 | .iteration _ env body initial => EnvWithin P env ∧ ExprWithin P body ∧ ValueWithin P initial
theorem lookup_within {P : HandleValues.Interface size → Prop} {env : List (Value size)}
 (bounded : EnvWithin P env) (i : Nat) {v : Value size} (lookup : env[i]? = some v) : ValueWithin P v := by
 induction i generalizing env with
 | zero =>
   cases bounded with
   | nil => simp at lookup
   | cons head tail => simp at lookup; subst v; exact head
 | succ i ih =>
   cases bounded with
   | nil => simp at lookup
   | cons head tail => exact ih tail (by simpa using lookup)
theorem execution_preserves {P : HandleValues.Interface size → Prop} {task : Task size} {v : Value size}
 (exec : Executes task v) : TaskWithin P task → ValueWithin P v := by
 induction exec with
 | handle => intro h; exact .handle h.2
 | integer => intro _; exact .integer
 | natural => intro _; exact .natural
 | lookupValue lookup => intro h; exact lookup_within h.1 _ lookup
 | addition => intro _; exact .integer
 | multiply => intro _; exact .integer
 | lambda => intro h; exact .closure h.2 h.1
 | application fn arg body ihf iha ihb =>
   intro h
   have hf := ihf ⟨h.1,h.2.1⟩
   have ha := iha ⟨h.1,h.2.2⟩
   cases hf with
   | closure hb he => exact ihb ⟨.cons ha he,hb⟩
 | iteration count initial steps ihc ihi ihs =>
   intro h
   exact ihs ⟨h.1,h.2.2.2,ihi ⟨h.1,h.2.2.1⟩⟩
 | zero => intro h; exact h.2.2
 | step body steps ihb ihs =>
   intro h
   exact ihs ⟨h.1,h.2.1,ihb ⟨.cons h.2.2 h.1,h.2.1⟩⟩
theorem evaluator_preserves {P : HandleValues.Interface size → Prop} {fuel : Nat}
 {env : List (Value size)} {e : Expr size} {v : Value size}
 (bounded : EnvWithin P env ∧ ExprWithin P e) (exec : evaluate fuel env e = some v) : ValueWithin P v :=
 execution_preserves (execution_sound fuel (.expression env e) v exec) bounded
theorem no_new_handle {P : HandleValues.Interface size → Prop} {fuel : Nat}
 {env : List (Value size)} {e : Expr size} {h : HandleValues.Interface size}
 (bounded : EnvWithin P env ∧ ExprWithin P e) (exec : evaluate fuel env e = some (.handle h)) : P h := by
 have result := evaluator_preserves bounded exec
 cases result with
 | handle existing => exact existing
theorem carried_within {P : HandleValues.Interface size → Prop} {h : HandleValues.Interface size}
 (member : P h) : ExprWithin P (carried h) := by
 simp [carried,twice,identity,ExprWithin,member]
theorem no_refresh {fuel : Nat} {env : List (Value size)} {e : Expr size}
 {original out : HandleValues.Interface size}
 (bounded : EnvWithin (fun h => h = original) env ∧ ExprWithin (fun h => h = original) e)
 (exec : evaluate fuel env e = some (.handle out)) : out = original := no_new_handle (P := fun h => h = original) bounded exec
namespace Controls
open HandleValues.Controls
-- Neither hidden literal code nor a captured environment may be omitted from
-- the reference bound. These are finite controls, not the general proof.
def hidden : Expr 4 := .lambda .int (.handle staleToken)
example : infer [] hidden = some (.arrow .int .handle) := by decide
example : evaluate 4 [] (.app hidden (.integer 0)) = some (.handle staleToken) := by rfl
example : ¬ ExprWithin (fun h => h = token) hidden := by
 simp only [hidden,ExprWithin]
 decide
def captured : Value 4 := .closure .int (.var 1) [.handle staleToken]
example : evaluate 4 [captured] (.app (.var 0) (.integer 0)) = some (.handle staleToken) := by rfl
example : ¬ ValueWithin (fun h => h = token) captured := by
 intro h
 cases h with
 | closure body env =>
   cases env with
   | cons head tail =>
     cases head with
     | handle eq => exact (by decide : staleToken ≠ token) eq
theorem capture_bound_necessary : ∃ env e h, ExprWithin (fun h => h = token) e ∧
 evaluate 4 env e = some (.handle h) ∧ h ≠ token := by
 refine ⟨[captured],.app (.var 0) (.integer 0),staleToken,⟨trivial,trivial⟩,rfl,?_⟩
 decide
-- Same arbitrary reference survives genuine higher-order application and iteration.
theorem repeated (h : HandleValues.Interface size) :
 evaluate 12 [] (.iterate .handle (.natural 3) (carried h) (.var 0)) = some (.handle h) := by rfl
end Controls
theorem registered_uses_existing {P : HandleValues.Interface size → Prop} {fuel : Nat}
 {s : CurrentUse.World size} {registry : ModuleContractBoundary.Registry size}
 {catalog : ModuleContractBoundary.Catalog} {caller : CurrentUse.UseRequest size}
 {args : List Int} {auth : CurrentUse.Evidence} {proof : ModuleContractBoundary.CallProof size}
 {env : List (Value size)} {e : Expr size} {out : Nat}
 (bounded : EnvWithin P env ∧ ExprWithin P e)
 (ok : invokeRegistered fuel s registry catalog caller args auth proof env e = some out) :
 ∃ h, P h ∧ Executes (.expression env e) (.handle h) ∧
 ModuleContractBoundary.Successful s registry (HandleValues.request caller h) args proof out ∧
 ∃ d, registry h.operation.key = some d ∧ catalog (s.records h.operation.key).code = some d := by
 obtain ⟨h,exec,success,entry⟩ := registered_sound ok
 have within := execution_preserves exec bounded
 cases within with
 | handle old => exact ⟨h,old,exec,success,entry⟩
#print axioms registered_uses_existing
#print axioms Controls.capture_bound_necessary
#print axioms carried_within
#print axioms no_refresh
#print axioms Controls.repeated
#print axioms execution_preserves
#print axioms evaluator_preserves
#print axioms no_new_handle
end MirroreaProofFirst.PureHandleFunctions.ReferencePreservation

namespace MirroreaProofFirst.PureHandleFunctions.IterationSelection
open CurrentUse.Controls ModuleContractBoundary.Controls ModuleContractBoundary.DescriptorControl HandleValues.Controls
-- Same expression and world; only the ordinary natural argument changes.
-- Calling that input secret and the success/failure observation public is an
-- explicit conditional scenario, not an adopted label or observer policy.
def selection : Expr 4 := .iterate .handle (.var 0) (.handle token) (.handle staleToken)
theorem typed : Typed [.nat] selection .handle := (infer_exact _ _ _).mp (by decide)
theorem zero_selects : evaluate 4 [.natural 0] selection = some (.handle token) := by rfl
theorem one_selects : evaluate 4 [.natural 1] selection = some (.handle staleToken) := by rfl
theorem same_reference_bound (n : Nat) :
 ReferencePreservation.EnvWithin (fun h => h = token ∨ h = staleToken) [.natural n] ∧
 ReferencePreservation.ExprWithin (fun h => h = token ∨ h = staleToken) selection := by
 refine ⟨.cons .natural .nil,?_⟩
 simp [ReferencePreservation.ExprWithin,selection]
theorem actual_zero_call : invokeRegistered 4 world registry catalog CurrentUse.Controls.request
 [41] evidence proof [.natural 0] selection = some 42 := by decide
theorem actual_one_call : invokeRegistered 4 world registry catalog CurrentUse.Controls.request
 [41] evidence proof [.natural 1] selection = none := by decide
theorem different_call_outcomes :
 invokeRegistered 4 world registry catalog CurrentUse.Controls.request [41] evidence proof [.natural 0] selection ≠
 invokeRegistered 4 world registry catalog CurrentUse.Controls.request [41] evidence proof [.natural 1] selection := by
 rw [actual_zero_call,actual_one_call]
 decide
-- A fixed public count does not create this two-run variation. A future label
-- checker must retain a secret count's control/completion dependency; reference
-- preservation and ordinary typing alone establish no such information-flow law.
#print axioms typed
#print axioms same_reference_bound
#print axioms different_call_outcomes
end MirroreaProofFirst.PureHandleFunctions.IterationSelection

namespace MirroreaProofFirst.PureHandleFunctions.IterationBudget
open CurrentUse.Controls ModuleContractBoundary.Controls ModuleContractBoundary.DescriptorControl HandleValues.Controls
-- Separate budget channel: every finite declarative run returns the SAME token,
-- yet a fixed evaluator budget may expose count-dependent completion.
def quiet : Expr 4 := .iterate .handle (.var 0) (.handle token) (.var 0)
theorem identity_iteration (n : Nat) (env : List (Value size)) (v : Value size) :
 Executes (.iteration n env (.var 0) v) v := by
 induction n with
 | zero => exact .zero
 | succ n ih => exact .step (.lookupValue rfl) ih
theorem quiet_executes (n : Nat) : Executes (.expression [.natural n] quiet) (.handle token) :=
 .iteration (.lookupValue rfl) .handle (identity_iteration n _ _)
theorem execution_result_unique {task : Task size} {v w : Value size}
 (a : Executes task v) (b : Executes task w) : v = w := by
 obtain ⟨f,hf⟩ := execution_complete a
 obtain ⟨g,hg⟩ := execution_complete b
 exact Option.some.inj ((hf (max f g) (Nat.le_max_left _ _)).symm.trans
   (hg (max f g) (Nat.le_max_right _ _)))
theorem quiet_result_unique {n : Nat} {v : Value 4}
 (exec : Executes (.expression [.natural n] quiet) v) : v = .handle token :=
 execution_result_unique exec (quiet_executes n)
theorem quiet_typed : Typed [.nat] quiet .handle := (infer_exact _ _ _).mp (by decide)
example : invokeRegistered 4 world registry catalog CurrentUse.Controls.request
 [41] evidence proof [.natural 0] quiet = some 42 := by decide
example : invokeRegistered 4 world registry catalog CurrentUse.Controls.request
 [41] evidence proof [.natural 3] quiet = none := by decide
#print axioms identity_iteration
#print axioms quiet_executes
#print axioms execution_result_unique
#print axioms quiet_result_unique
#print axioms quiet_typed
end MirroreaProofFirst.PureHandleFunctions.IterationBudget

-- Later unreviewed totality extension; no source, fixed-budget or authority claim.
namespace MirroreaProofFirst.PureHandleFunctions.Normalization
variable {size : Nat}
-- Logical relation over the type. This defines evidence of computation, not an
-- axiom that every typeable expression computes.
def Computable : Ty → Value size → Prop
 | .handle, v => ∃ h, v = .handle h
 | .int, v => ∃ z, v = .integer z
 | .nat, v => ∃ n, v = .natural n
 | .arrow a b, v => ∃ body captured,
   v = .closure a body captured ∧
   ∀ arg, Computable a arg → ∃ out,
     Executes (.expression (arg::captured) body) out ∧ Computable b out

def GoodEnv (G : List Ty) (env : List (Value size)) : Prop :=
 ∀ (i : Nat) (t : Ty), G[i]? = some t → ∃ v, env[i]? = some v ∧ Computable t v

theorem good_nil : GoodEnv (size := size) [] [] := by
 intro i t h
 simp at h

theorem good_cons {G : List Ty} {env : List (Value size)} {t : Ty} {v : Value size} (head : Computable t v) (tail : GoodEnv G env) :
 GoodEnv (t::G) (v::env) := by
 intro i ty lookup
 cases i with
 | zero => simp at lookup; subst ty; exact ⟨v,rfl,head⟩
 | succ i => exact tail i ty (by simpa using lookup)

theorem iteration_computable {t : Ty} {env : List (Value size)} {body : Expr size}
 (bodyTotal : ∀ v, Computable t v → ∃ out,
   Executes (.expression (v::env) body) out ∧ Computable t out)
 (count : Nat) : ∀ v, Computable t v → ∃ out,
   Executes (.iteration count env body v) out ∧ Computable t out := by
 induction count with
 | zero => intro v hv; exact ⟨v,.zero,hv⟩
 | succ n ih =>
   intro v hv
   obtain ⟨next,step,hn⟩ := bodyTotal v hv
   obtain ⟨out,rest,ho⟩ := ih next hn
   exact ⟨out,.step step rest,ho⟩

theorem fundamental {G : List Ty} {e : Expr size} {t : Ty} (typed : Typed G e t) :
 ∀ env, GoodEnv G env → ∃ out,
   Executes (.expression env e) out ∧ Computable t out := by
 induction typed with
 | handle => intro env he; exact ⟨_,.handle,_,rfl⟩
 | integer => intro env he; exact ⟨_,.integer,_,rfl⟩
 | natural => intro env he; exact ⟨_,.natural,_,rfl⟩
 | var lookup =>
   intro env he
   obtain ⟨v,hv,cv⟩ := he _ _ lookup
   exact ⟨v,.lookupValue hv,cv⟩
 | add ha hb ia ib =>
   intro env he
   obtain ⟨va,ea,za,rfl⟩ := ia env he
   obtain ⟨vb,eb,zb,rfl⟩ := ib env he
   exact ⟨_,.addition ea eb,_,rfl⟩
 | mul ha hb ia ib =>
   intro env he
   obtain ⟨va,ea,za,rfl⟩ := ia env he
   obtain ⟨vb,eb,zb,rfl⟩ := ib env he
   exact ⟨_,.multiply ea eb,_,rfl⟩
 | @lambda G a b body ht ih =>
   intro env he
   refine ⟨.closure a body env,.lambda,body,env,rfl,?_⟩
   intro arg ca
   exact ih (arg::env) (good_cons ca he)
 | app hf ha fi ai =>
   intro env he
   obtain ⟨vf,ef,body,captured,rfl,total⟩ := fi env he
   obtain ⟨arg,ea,ca⟩ := ai env he
   obtain ⟨out,eb,cb⟩ := total arg ca
   exact ⟨out,.application ef ea eb,cb⟩
 | iterate hc hi hb ci ii bi =>
   intro env he
   obtain ⟨vc,ec,count,rfl⟩ := ci env he
   obtain ⟨initial,ei,cv⟩ := ii env he
   have bt := fun v hv => bi (v::env) (good_cons hv he)
   obtain ⟨out,loop,co⟩ := iteration_computable bt count initial cv
   exact ⟨out,.iteration ec ei loop,co⟩

theorem closed_executes {e : Expr size} {t : Ty} (typed : Typed [] e t) :
 ∃ out, Executes (.expression [] e) out ∧ Computable t out :=
 fundamental typed [] good_nil

theorem closed_evaluator_total {e : Expr size} {t : Ty} (typed : Typed [] e t) :
 ∃ out minimum, ∀ fuel, minimum ≤ fuel → evaluate fuel [] e = some out := by
 obtain ⟨out,executes,_⟩ := closed_executes typed
 obtain ⟨minimum,enough⟩ := execution_complete executes
 exact ⟨out,minimum,enough⟩

#print axioms fundamental
#print axioms iteration_computable
#print axioms closed_executes
#print axioms closed_evaluator_total

-- Type admission alone supplies a finite pure handle-selection derivation.
-- Currentness, invocation authority and contract acceptance remain separate.
theorem checked_handle_total {e : Expr size} (checked : infer [] e = some .handle) :
 ∃ h minimum, Executes (.expression [] e) (.handle h) ∧
   ∀ fuel, minimum ≤ fuel → evaluate fuel [] e = some (.handle h) := by
 obtain ⟨out,run,h,rfl⟩ := closed_executes ((infer_exact _ _ _).mp checked)
 obtain ⟨minimum,enough⟩ := execution_complete run
 exact ⟨h,minimum,run,enough⟩

theorem checked_call_eventually_exact {e : Expr size}
 (checked : infer [] e = some .handle)
 (s : CurrentUse.World size) (registry : ModuleContractBoundary.Registry size)
 (catalog : ModuleContractBoundary.Catalog) (caller : CurrentUse.UseRequest size)
 (args : List Int) (auth : CurrentUse.Evidence) (proof : ModuleContractBoundary.CallProof size) :
 ∃ h minimum, Executes (.expression [] e) (.handle h) ∧
   ∀ fuel, minimum ≤ fuel →
     invokeClosedRegistered fuel s registry catalog caller args auth proof e =
       ModuleContractBoundary.catalogCall s registry catalog
         (HandleValues.request caller h) args auth proof := by
 obtain ⟨h,minimum,run,enough⟩ := checked_handle_total checked
 refine ⟨h,minimum,run,?_⟩
 intro fuel hf
 simp [invokeClosedRegistered,checked,invokeRegistered,enough fuel hf]

-- Dropping closed typing would assert totality even for a missing variable.
theorem unbound_never_returns (fuel : Nat) :
 evaluate fuel [] (.var 0 : Expr size) = none := by
 cases fuel <;> rfl

example : infer [] (.app (.integer 1) (.integer 2) : Expr size) = none := by rfl
example (h : HandleValues.Interface size) :
 ∃ out minimum, ∀ fuel, minimum ≤ fuel → evaluate fuel [] (carried h) = some out :=
 closed_evaluator_total (carried_typed h)

#print axioms checked_handle_total
#print axioms checked_call_eventually_exact
#print axioms unbound_never_returns
end MirroreaProofFirst.PureHandleFunctions.Normalization

-- Typed closure environments discharge the logical-relation premise.
namespace MirroreaProofFirst.PureHandleFunctions.Normalization
variable {size : Nat}
theorem typed_value_computable {v : Value size} {t : Ty} (typed : HasType v t) :
 Computable t v := by
 induction typed using HasType.rec (motive_2 := fun env G _ => GoodEnv G env) with
 | handle => exact ⟨_,rfl⟩
 | integer => exact ⟨_,rfl⟩
 | natural => exact ⟨_,rfl⟩
 | @closure a b body env G he hb ih =>
   refine ⟨body,env,rfl,?_⟩
   intro arg ca
   exact fundamental hb (arg::env) (good_cons ca ih)
 | nil => exact good_nil
 | cons hv he cv ce => exact good_cons cv ce

theorem typed_environment_good {env : List (Value size)} {G : List Ty}
 (typed : EnvTyped env G) : GoodEnv G env := by
 induction env generalizing G with
 | nil => cases typed; exact good_nil
 | cons v env ih =>
   cases typed with
   | cons hv he => exact good_cons (typed_value_computable hv) (ih he)

theorem typed_environment_total {env : List (Value size)} {G : List Ty}
 {e : Expr size} {t : Ty} (he : EnvTyped env G) (ht : Typed G e t) :
 ∃ out minimum, HasType out t ∧
   ∀ fuel, minimum ≤ fuel → evaluate fuel env e = some out := by
 obtain ⟨out,executes,_⟩ := fundamental ht env (typed_environment_good he)
 obtain ⟨minimum,enough⟩ := execution_complete executes
 refine ⟨out,minimum,?_,enough⟩
 exact (evaluator_typed minimum).1 he ht (enough minimum (Nat.le_refl _))
#print axioms typed_value_computable
#print axioms typed_environment_good
#print axioms typed_environment_total
end MirroreaProofFirst.PureHandleFunctions.Normalization
