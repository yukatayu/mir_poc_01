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
