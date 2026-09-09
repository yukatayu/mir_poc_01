import Std
namespace MirroreaProofFirst.PureFunctions
inductive Ty where
 | int | nat
 | arrow (domain codomain : Ty)
 deriving DecidableEq, Repr
inductive Expr where
 | integer (z : Int)
 | natural (n : Nat)
 | var (index : Nat)
 | add (a b : Expr)
 | mul (a b : Expr)
 | lambda (parameter : Ty) (body : Expr)
 | app (fn argument : Expr)
 -- The iteration body binds its current value at index0; the outer environment
 -- remains available. No mutable region ownership or effect continuation here.
 | iterate (type : Ty) (count initial body : Expr)
 deriving DecidableEq, Repr
inductive Typed : List Ty → Expr → Ty → Prop where
 | integer {G z} : Typed G (.integer z) .int
 | natural {G n} : Typed G (.natural n) .nat
 | var {G i t} : G[i]? = some t → Typed G (.var i) t
 | add {G a b} : Typed G a .int → Typed G b .int → Typed G (.add a b) .int
 | mul {G a b} : Typed G a .int → Typed G b .int → Typed G (.mul a b) .int
 | lambda {G a b body} : Typed (a::G) body b → Typed G (.lambda a body) (.arrow a b)
 | app {G fn arg a b} : Typed G fn (.arrow a b) → Typed G arg a → Typed G (.app fn arg) b
 | iterate {G count initial body t} : Typed G count .nat → Typed G initial t →
     Typed (t::G) body t → Typed G (.iterate t count initial body) t

def infer (G : List Ty) : Expr → Option Ty
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

theorem typed_infer {G : List Ty} {e : Expr} {t : Ty} (h : Typed G e t) : infer G e = some t := by
 induction h <;> simp_all [infer]

theorem infer_typed (G : List Ty) (e : Expr) (t : Ty) (h : infer G e = some t) : Typed G e t := by
 induction e generalizing G t with
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

theorem infer_exact (G : List Ty) (e : Expr) (t : Ty) : infer G e = some t ↔ Typed G e t :=
 ⟨infer_typed G e t,typed_infer⟩

inductive Value where
 | integer (z : Int)
 | natural (n : Nat)
 | closure (parameter : Ty) (body : Expr) (environment : List Value)
 deriving Repr

-- Captured environment typing is explicit, independent of interpreter success.
mutual
inductive HasType : Value → Ty → Prop where
 | integer {z} : HasType (.integer z) .int
 | natural {n} : HasType (.natural n) .nat
 | closure {a b body env G} : EnvTyped env G → Typed (a::G) body b →
     HasType (.closure a body env) (.arrow a b)
inductive EnvTyped : List Value → List Ty → Prop where
 | nil : EnvTyped [] []
 | cons {v t env G} : HasType v t → EnvTyped env G → EnvTyped (v::env) (t::G)
end

mutual
 def evaluate : Nat → List Value → Expr → Option Value
 | 0,_,_ => none
 | fuel+1,env,e => match e with
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
 def iterateValues : Nat → Nat → List Value → Expr → Value → Option Value
 | 0,_,_,_,_ => none
 | _+1,0,_,_,v => some v
 | fuel+1,n+1,env,body,v => do
   let next ← evaluate fuel (v::env) body
   iterateValues fuel n env body next
end

theorem environment_lookup {env : List Value} {G : List Ty} (typed : EnvTyped env G)
 (i : Nat) {t : Ty} {v : Value} (ht : G[i]? = some t) (hv : env[i]? = some v) : HasType v t := by
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
 (∀ {env G e t v}, EnvTyped env G → Typed G e t → evaluate fuel env e = some v → HasType v t) ∧
 (∀ {n env G body t initial v}, EnvTyped env G → Typed (t::G) body t → HasType initial t →
   iterateValues fuel n env body initial = some v → HasType v t) := by
 induction fuel with
 | zero => constructor <;> intros <;> contradiction
 | succ fuel ih =>
   constructor
   · intro env G e t v he ht hv
     cases ht with
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

inductive Task where
 | expression (env : List Value) (e : Expr)
 | iteration (n : Nat) (env : List Value) (body : Expr) (initial : Value)
inductive Executes : Task → Value → Prop where
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

def runTask (fuel : Nat) : Task → Option Value
 | .expression env e => evaluate fuel env e
 | .iteration n env body initial => iterateValues fuel n env body initial

-- Completeness is relative to finite declarative executions. It neither asserts
-- a global fixed fuel bound nor treats exhausted fuel as a source type error.
theorem execution_complete {task : Task} {v : Value} (h : Executes task v) :
 ∃ minimum, ∀ fuel, minimum ≤ fuel → runTask fuel task = some v := by
 induction h with
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

theorem execution_sound (fuel : Nat) (task : Task) (v : Value)
 (h : runTask fuel task = some v) : Executes task v := by
 induction fuel generalizing task v with
 | zero => cases task <;> contradiction
 | succ fuel ih =>
   cases task with
   | expression env e =>
     cases e with
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

theorem execution_exact (task : Task) (v : Value) :
 (∃ fuel, runTask fuel task = some v) ↔ Executes task v := by
 constructor
 · rintro ⟨fuel,h⟩; exact execution_sound fuel task v h
 · intro h; obtain ⟨fuel,hf⟩ := execution_complete h; exact ⟨fuel,hf fuel (Nat.le_refl _)⟩

def counterBody (delta : Int) : Expr := .add (.var 0) (.integer delta)

theorem counter_iteration_executes (n : Nat) (initial delta : Int) :
 Executes (.iteration n [] (counterBody delta) (.integer initial))
 (.integer (initial + delta * (n : Int))) := by
 induction n generalizing initial with
 | zero => simpa using (Executes.zero (env := []) (body := counterBody delta) (v := .integer initial))
 | succ n ih =>
   have body : Executes (.expression [.integer initial] (counterBody delta)) (.integer (initial+delta)) :=
     .addition (.lookupValue rfl) .integer
   have rest := ih (initial+delta)
   have step := Executes.step body rest
   simpa [Int.natCast_add,Int.mul_add,Int.add_assoc,Int.add_comm,Int.add_left_comm] using step

theorem counter_has_fuel (n : Nat) (initial delta : Int) :
 ∃ fuel, evaluate fuel [] (.iterate .int (.natural n) (.integer initial) (counterBody delta)) =
 some (.integer (initial + delta * (n : Int))) := by
 exact (execution_exact (.expression [] (.iterate .int (.natural n) (.integer initial) (counterBody delta))) _).mpr
   (.iteration .natural .integer (counter_iteration_executes n initial delta))

namespace Controls
-- A higher-order function applies its argument twice; this is ordinary pure reuse.
def twice : Expr := .lambda (.arrow .int .int)
 (.lambda .int (.app (.var 1) (.app (.var 1) (.var 0))))
def increment : Expr := .lambda .int (.add (.var 0) (.integer 1))
def program : Expr := .app (.app twice increment) (.integer 40)
example : infer [] program = some .int := by decide
example : evaluate 10 [] program = some (.integer 42) := by rfl
-- Iteration has the same general body evaluator; there is no example-name branch.
def iteration : Expr := .iterate .int (.natural 7) (.integer 0)
 (.add (.var 0) (.integer 3))
example : infer [] iteration = some .int := by decide
example : evaluate 20 [] iteration = some (.integer 21) := by rfl
example : infer [] (.app increment (.natural 4)) = none := by decide
example : infer [] (.var 0) = none := by decide
example : infer [] (.iterate .int (.integer 7) (.integer 0) (.var 0)) = none := by decide
example : evaluate 0 [] program = none := by decide
end Controls
#print axioms infer_exact
#print axioms environment_lookup
#print axioms evaluator_typed
#print axioms execution_complete
#print axioms execution_sound
#print axioms execution_exact
#print axioms counter_has_fuel
end MirroreaProofFirst.PureFunctions
