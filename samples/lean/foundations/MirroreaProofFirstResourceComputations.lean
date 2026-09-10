import MirroreaProofFirstPureHandleFunctions
import MirroreaProofFirstProfileGuarantees
import MirroreaProofFirstOwnerAssignment
namespace MirroreaProofFirst.ResourceComputations
open PureHandleFunctions

structure Source (n : Nat) where
 interface : Expr n
 argument : Expr n
inductive Typed (G : List Ty) : Source n → Prop where
 | call : PureHandleFunctions.Typed G s.interface .handle →
     PureHandleFunctions.Typed G s.argument .int → Typed G s

def check (G : List Ty) (s : Source n) : Bool :=
 decide (infer G s.interface = some .handle ∧ infer G s.argument = some .int)
theorem check_exact (G : List Ty) (s : Source n) : check G s = true ↔ Typed G s := by
 simp only [check,decide_eq_true_eq,infer_exact]
 exact ⟨fun ⟨h,a⟩ => .call h a,fun h => by cases h; constructor <;> assumption⟩

inductive Selects (env : List (Value n)) (s : Source n) : HandleValues.Interface n → Int → Prop where
 | selected : Executes (.expression env s.interface) (.handle h) →
     Executes (.expression env s.argument) (.integer x) → Selects env s h x

def select (fuel : Nat) (env : List (Value n)) (s : Source n) : Option (HandleValues.Interface n × Int) :=
 match evaluate fuel env s.interface, evaluate fuel env s.argument with
 | some (.handle h), some (.integer x) => some (h,x)
 | _,_ => none

theorem select_sound {fuel : Nat} {env : List (Value n)} {s : Source n}
 {h : HandleValues.Interface n} {x : Int} (ok : select fuel env s = some (h,x)) : Selects env s h x := by
 unfold select at ok
 split at ok
 · cases ok
   rename_i eh ex
   exact .selected ((execution_exact _ _).mp ⟨fuel,eh⟩) ((execution_exact _ _).mp ⟨fuel,ex⟩)
 · contradiction

theorem select_complete {env : List (Value n)} {s : Source n}
 {h : HandleValues.Interface n} {x : Int} (selected : Selects env s h x) :
 ∃ minimum, ∀ fuel, minimum ≤ fuel → select fuel env s = some (h,x) := by
 cases selected with
 | selected eh ex =>
   obtain ⟨a,ha⟩ := execution_complete eh
   obtain ⟨b,hb⟩ := execution_complete ex
   refine ⟨max a b,?_⟩
   intro fuel enough
   exact (by simp only [runTask] at ha hb; simp [select,ha fuel (by omega),hb fuel (by omega)])

-- Meaning comes from evaluated source, while authority is independently supplied.
def request (caller : CurrentUse.UseRequest n) (h : HandleValues.Interface n) (x : Int) : CurrentUse.UseRequest n :=
 {caller with moduleHandle := h.moduleHandle,operation := h.operation,arguments := [.integer x]}

def allocate (fuel : Nat) (G : List Ty) (env : List (Value n)) (source : Source n)
 (current : ModuleContractBoundary.CurrentAllocation.RestoredCall.ExecutionContext n) (caller : CurrentUse.UseRequest n)
 (auth : CurrentUse.Evidence) (proof : ModuleContractBoundary.CallProof n) (state : ResourceBoundary.State) : Option (ResourceBoundary.State × List ResourceBoundary.Handle) :=
 if check G source then
   (select fuel env source).bind fun chosen =>
     ModuleContractBoundary.CurrentAllocation.run current.limits current.lo current.hi current.world current.registry current.catalog
       (request caller chosen.1 chosen.2) [chosen.2] auth proof current.grant state
 else none

theorem allocate_selected {fuel : Nat} {G : List Ty} {env : List (Value n)} {source : Source n}
 {current : ModuleContractBoundary.CurrentAllocation.RestoredCall.ExecutionContext n} {caller : CurrentUse.UseRequest n}
 {auth : CurrentUse.Evidence} {proof : ModuleContractBoundary.CallProof n} {state : ResourceBoundary.State} {out : ResourceBoundary.State × List ResourceBoundary.Handle}
 (ok : allocate fuel G env source current caller auth proof state = some out) :
 Typed G source ∧ ∃ h x, Selects env source h x ∧
 ModuleContractBoundary.CurrentAllocation.run current.limits current.lo current.hi current.world current.registry current.catalog
   (request caller h x) [x] auth proof current.grant state = some out := by
 unfold allocate at ok
 split at ok
 · rename_i checked
   refine ⟨(check_exact _ _).mp checked,?_⟩
   cases selected : select fuel env source with
   | none => simp [selected] at ok
   | some pair =>
     exact ⟨pair.1,pair.2,select_sound selected,by simpa [selected] using ok⟩
 · contradiction

theorem allocate_preserves {fuel : Nat} {G : List Ty} {env : List (Value n)} {source : Source n}
 {current : ModuleContractBoundary.CurrentAllocation.RestoredCall.ExecutionContext n} {caller : CurrentUse.UseRequest n}
 {auth : CurrentUse.Evidence} {proof : ModuleContractBoundary.CallProof n} {state : ResourceBoundary.State} {out : ResourceBoundary.State × List ResourceBoundary.Handle}
 (wf : ResourceBoundary.WF state) (ok : allocate fuel G env source current caller auth proof state = some out) :
 ResourceBoundary.WF out.1 ∧ ResourceBoundary.BoundedIdentifiers.Within current.limits out.1 ∧
 ∃ h x, Selects env source h x ∧ CurrentUse.CurrentUse current.world (request caller h x) := by
 obtain ⟨_,h,x,selected,executed⟩ := allocate_selected ok
 obtain ⟨live,d,value,lookup,registered,linked,exported,bounds,grant,wf',within⟩ := ModuleContractBoundary.CurrentAllocation.sound wf executed
 exact ⟨wf',within,h,x,selected,live⟩

-- This is explicitly relative to admission of the exact selected call, not
-- a claim that every typed program is authorized or every submitted proof valid.
theorem selected_call_eventually {G : List Ty} {env : List (Value n)} {source : Source n}
 {current : ModuleContractBoundary.CurrentAllocation.RestoredCall.ExecutionContext n} {caller : CurrentUse.UseRequest n}
 {auth : CurrentUse.Evidence} {proof : ModuleContractBoundary.CallProof n} {state : ResourceBoundary.State}
 {h : HandleValues.Interface n} {x : Int} {out : ResourceBoundary.State × List ResourceBoundary.Handle}
 (typed : Typed G source) (selected : Selects env source h x)
 (admitted : ModuleContractBoundary.CurrentAllocation.run current.limits current.lo current.hi current.world current.registry current.catalog
   (request caller h x) [x] auth proof current.grant state = some out) :
 ∃ minimum, ∀ fuel, minimum ≤ fuel → allocate fuel G env source current caller auth proof state = some out := by
 obtain ⟨minimum,enough⟩ := select_complete selected
 exact ⟨minimum,fun fuel bound => by simp [allocate,(check_exact _ _).mpr typed,enough fuel bound,admitted]⟩

#print axioms check_exact
#print axioms select_sound
#print axioms select_complete
#print axioms allocate_selected
#print axioms allocate_preserves
#print axioms selected_call_eventually
end MirroreaProofFirst.ResourceComputations

namespace MirroreaProofFirst.ResourceComputations.ResourceContext
open ResourceBoundary
-- Dynamic affine interpretation: aliases name one semantic resource identity,
-- not separate permissions. The pure Value syntax contains no resource handles.
abbrev Delta := Nat → Option Region
def Realized (state : State) (delta : Delta) : Prop :=
 ∀ id region, delta id = some region → state.live id = some region

def erase (delta : Delta) (id : Nat) : Delta := fun key => if key = id then none else delta key
def consume (delta : Delta) (action : Action) : Delta :=
 match consumed action with | none => delta | some handle => erase delta handle.id
def add (delta : Delta) (h : Handle) : Delta := fun key => if key = h.id then some h.region else delta key
def extend : Delta → List Handle → Delta
 | delta,[] => delta
 | delta,h::tail => extend (add delta h) tail
def update (delta : Delta) (action : Action) (returned : List Handle) : Delta :=
 extend (consume delta action) returned

def Owned (delta : Delta) (action : Action) : Prop :=
 ∀ h, consumed action = some h → delta h.id = some h.region
def ownedCheck (delta : Delta) (action : Action) : Bool :=
 match consumed action with | none => true | some h => decide (delta h.id = some h.region)

theorem owned_exact (delta : Delta) (action : Action) : ownedCheck delta action = true ↔ Owned delta action := by
 cases h : consumed action <;> simp [ownedCheck,Owned,h]

theorem consume_frame {delta : Delta} {action : Action} {id : Nat} {region : Region}
 (present : consume delta action id = some region) :
 delta id = some region ∧ ∀ h, consumed action = some h → id ≠ h.id := by
 cases selected : consumed action with
 | none => exact ⟨by simpa [consume,selected] using present,by simp⟩
 | some handle =>
   simp only [consume,selected,erase] at present
   split at present
   · contradiction
   · rename_i different
     exact ⟨present,by intro h equal; simp only [Option.some.injEq] at equal; subst h; exact different⟩

theorem add_realized {state : State} {delta : Delta} {h : Handle}
 (old : Realized state delta) (current : state.live h.id = some h.region) : Realized state (add delta h) := by
 intro id region present
 unfold add at present
 split at present
 · rename_i same; simp_all
 · exact old _ _ present

theorem extend_realized {state : State} {delta : Delta} {returned : List Handle}
 (old : Realized state delta) (current : ∀ h ∈ returned, state.live h.id = some h.region) :
 Realized state (extend delta returned) := by
 induction returned generalizing delta with
 | nil => exact old
 | cons h tail ih =>
   apply ih (add_realized old (current h (by simp)))
   intro h' member; exact current h' (by simp [member])

theorem raw_realized {state : State} {delta : Delta} (wf : WF state)
 (realized : Realized state delta) (action : Action) :
 Realized (raw state action).1 (update delta action (raw state action).2) := by
 apply extend_realized
 · intro id region present
   obtain ⟨old,untouched⟩ := consume_frame present
   have live := realized _ _ old
   rw [raw_frame state action id (wf.bounded _ _ live).1 untouched]
   exact live
 · intro h member; exact (returned_current state action h member).1

-- This checker enforces the caller's current resource context in addition to
-- the existing independent policy/current-handle/cut checks, not instead of them.
def execute (policy : Nat → Bool) (state : State) (delta : Delta) (action : Action) :
 Option (State × Delta × List Handle) :=
 if ownedCheck delta action then
   (ResourceBoundary.execute policy state action).map fun out =>
     (out.1,update delta action out.2,out.2)
 else none

theorem execute_exact (policy : Nat → Bool) (state : State) (delta : Delta) (action : Action)
 (out : State × Delta × List Handle) : execute policy state delta action = some out ↔
 Owned delta action ∧ Allowed policy state action ∧
 out = ((raw state action).1,update delta action (raw state action).2,(raw state action).2) := by
 unfold execute ResourceBoundary.execute
 rw [← owned_exact,← ResourceBoundary.check_exact]
 by_cases owned : ownedCheck delta action = true <;>
   by_cases allowed : ResourceBoundary.check policy state action = true <;>
   simp [owned,allowed,eq_comm]

theorem execute_preserves {policy : Nat → Bool} {state : State} {delta : Delta} {action : Action}
 {out : State × Delta × List Handle} (wf : WF state) (realized : Realized state delta)
 (ok : execute policy state delta action = some out) : WF out.1 ∧ Realized out.1 out.2.1 := by
 obtain ⟨_,allowed,rfl⟩ := (execute_exact _ _ _ _ _).mp ok
 exact ⟨raw_wf wf allowed,raw_realized wf realized action⟩

#print axioms owned_exact
#print axioms raw_realized
#print axioms execute_exact
#print axioms execute_preserves
end MirroreaProofFirst.ResourceComputations.ResourceContext

namespace MirroreaProofFirst.ResourceComputations
open PureHandleFunctions ResourceBoundary ResourceContext

def allocateWithDelta (fuel : Nat) (G : List Ty) (env : List (Value n)) (source : Source n)
 (current : ModuleContractBoundary.CurrentAllocation.RestoredCall.ExecutionContext n)
 (caller : CurrentUse.UseRequest n) (auth : CurrentUse.Evidence)
 (proof : ModuleContractBoundary.CallProof n) (state : State) (delta : Delta) :
 Option (State × Delta × List Handle) :=
 (allocate fuel G env source current caller auth proof state).map fun out =>
   (out.1,extend delta out.2,out.2)

theorem allocation_delta_preserved {fuel : Nat} {G : List Ty} {env : List (Value n)} {source : Source n}
 {current : ModuleContractBoundary.CurrentAllocation.RestoredCall.ExecutionContext n}
 {caller : CurrentUse.UseRequest n} {auth : CurrentUse.Evidence}
 {proof : ModuleContractBoundary.CallProof n} {state : State} {delta : Delta}
 {out : State × Delta × List Handle} (wf : WF state) (realized : Realized state delta)
 (ok : allocateWithDelta fuel G env source current caller auth proof state delta = some out) :
 WF out.1 ∧ Realized out.1 out.2.1 := by
 cases allocated : allocate fuel G env source current caller auth proof state with
 | none => simp [allocateWithDelta,allocated] at ok
 | some result =>
   simp [allocateWithDelta,allocated] at ok; subst out
   obtain ⟨_,h,x,selected,executed⟩ := allocate_selected allocated
   obtain ⟨d,value,lookup,registered,live,linked,machine,accepted,granted,capacity,resultShape⟩ :=
     (ModuleContractBoundary.CurrentAllocation.exact _ _ _ _ _ _ _ _ _ _ _ _ _).mp executed
   have good := (allocate_preserves wf allocated).1
   subst result
   exact ⟨good,by simpa [ResourceContext.update,ResourceContext.consume,consumed]
     using ResourceContext.raw_realized wf realized (.allocate caller.principal value)⟩

-- Returned identities extend Delta; pure metadata aliases never create rights.
theorem updated_returned (state : State) (delta : Delta) (action : Action) :
 ∀ h ∈ (raw state action).2,
 ResourceContext.update delta action (raw state action).2 h.id = some h.region := by
 cases action <;> simp [raw,allocateRaw,moveRaw,splitRaw,newHandle,ResourceBoundary.erase,push,
   ResourceContext.update,extend,ResourceContext.add,ResourceContext.consume,consumed]

-- Exact source-selected invocation uses the existing shared semantic request key.
def invocation (current : ModuleContractBoundary.CurrentAllocation.RestoredCall.ExecutionContext n)
 (caller : CurrentUse.UseRequest n) (auth : CurrentUse.Evidence)
 (proof : ModuleContractBoundary.CallProof n) (h : HandleValues.Interface n) (x : Int) :
 ModuleContractBoundary.CurrentAllocation.Once.Invocation n :=
 {limits := current.limits,lo := current.lo,hi := current.hi,world := current.world,
  registry := current.registry,catalog := current.catalog,request := request caller h x,
  arguments := [x],evidence := auth,proof := proof,grant := current.grant}

def allocateOnce (fuel : Nat) (G : List Ty) (env : List (Value n)) (source : Source n)
 (current : ModuleContractBoundary.CurrentAllocation.RestoredCall.ExecutionContext n)
 (caller : CurrentUse.UseRequest n) (auth : CurrentUse.Evidence)
 (proof : ModuleContractBoundary.CallProof n)
 (state : ModuleContractBoundary.CurrentAllocation.Once.StateWithHistory) (delta : Delta) :
 Option (ModuleContractBoundary.CurrentAllocation.Once.StateWithHistory × Delta × List Handle) :=
 if check G source then (select fuel env source).bind fun chosen =>
   (ModuleContractBoundary.CurrentAllocation.Once.step
     (invocation current caller auth proof chosen.1 chosen.2) state).map fun out =>
       (out.1,extend delta out.2,out.2)
 else none

theorem invocation_exact (current : ModuleContractBoundary.CurrentAllocation.RestoredCall.ExecutionContext n)
 (caller : CurrentUse.UseRequest n) (auth : CurrentUse.Evidence)
 (proof : ModuleContractBoundary.CallProof n) (h : HandleValues.Interface n) (x : Int) (state : State) :
 (invocation current caller auth proof h x).allocate state =
 ModuleContractBoundary.CurrentAllocation.run current.limits current.lo current.hi current.world
 current.registry current.catalog (request caller h x) [x] auth proof current.grant state := rfl

#print axioms allocation_delta_preserved
#print axioms updated_returned
end MirroreaProofFirst.ResourceComputations

namespace MirroreaProofFirst.ResourceComputations
open PureHandleFunctions ResourceBoundary ResourceContext
open ModuleContractBoundary.CurrentAllocation

theorem allocateOnce_selected {fuel : Nat} {G : List Ty} {env : List (Value n)} {source : Source n}
 {current : RestoredCall.ExecutionContext n} {caller : CurrentUse.UseRequest n}
 {auth : CurrentUse.Evidence} {proof : ModuleContractBoundary.CallProof n}
 {state : Once.StateWithHistory} {delta : Delta}
 {out : Once.StateWithHistory × Delta × List Handle}
 (ok : allocateOnce fuel G env source current caller auth proof state delta = some out) :
 Typed G source ∧ ∃ h x result, Selects env source h x ∧
 Once.step (invocation current caller auth proof h x) state = some result ∧
 out = (result.1,extend delta result.2,result.2) := by
 unfold allocateOnce at ok
 split at ok
 · rename_i checked
   refine ⟨(check_exact _ _).mp checked,?_⟩
   cases selected : select fuel env source with
   | none => simp [selected] at ok
   | some pair =>
     simp only [selected,Option.bind_some] at ok
     cases executed : Once.step (invocation current caller auth proof pair.1 pair.2) state with
     | none => simp [executed] at ok
     | some result =>
       exact ⟨pair.1,pair.2,result,select_sound selected,executed,by simpa [executed] using ok.symm⟩
 · contradiction

theorem allocateOnce_preserves {fuel : Nat} {G : List Ty} {env : List (Value n)} {source : Source n}
 {current : RestoredCall.ExecutionContext n} {caller : CurrentUse.UseRequest n}
 {auth : CurrentUse.Evidence} {proof : ModuleContractBoundary.CallProof n}
 {state : Once.StateWithHistory} {delta : Delta}
 {out : Once.StateWithHistory × Delta × List Handle}
 (wf : WF state.resources) (realized : Realized state.resources delta)
 (unique : state.committed.Nodup)
 (ok : allocateOnce fuel G env source current caller auth proof state delta = some out) :
 WF out.1.resources ∧ Realized out.1.resources out.2.1 ∧ out.1.committed.Nodup := by
 obtain ⟨typed,h,x,result,selected,stepped,rfl⟩ := allocateOnce_selected ok
 obtain ⟨fresh,allocated,executed,rfl⟩ := (Once.step_exact _ _ _).mp stepped
 have sourceAllocated : allocate fuel G env source current caller auth proof state.resources = some allocated := by
   unfold allocateOnce at ok
   split at ok
   · cases selection : select fuel env source with
     | none => simp [selection] at ok
     | some pair =>
       -- Use deterministic source evaluation to recover the exact selected pair.
       have hx := select_sound selection
       cases selected with
       | selected eh ex =>
         cases hx with
         | selected eh' ex' =>
           have heq := IterationBudget.execution_result_unique eh eh'
           have xeq := IterationBudget.execution_result_unique ex ex'
           cases heq; cases xeq
           simpa [allocate,(check_exact _ _).mpr typed,selection,invocation_exact] using executed
   · contradiction
 have kept := allocation_delta_preserved wf realized
   (show allocateWithDelta fuel G env source current caller auth proof state.resources delta =
      some (allocated.1,extend delta allocated.2,allocated.2) from by simp [allocateWithDelta,sourceAllocated])
 exact ⟨kept.1,kept.2,by simpa [List.nodup_cons] using And.intro fresh unique⟩

#print axioms allocateOnce_selected
#print axioms allocateOnce_preserves
end MirroreaProofFirst.ResourceComputations

namespace MirroreaProofFirst.ResourceComputations.Sequence
open PureHandleFunctions ResourceBoundary ResourceContext
open ModuleContractBoundary.CurrentAllocation

-- Finite checked Core reference, with pure binders and separate resource names.
-- Resource names carry metadata; only Delta grants a consumable identity.
inductive Instruction (n : Nat) where
 | pureLet (value : Expr n)
 | allocate (source : Source n)
 | release (reference : Nat)
 | move (reference receiver : Nat)
 | split (reference cut : Nat)

structure Local (n : Nat) where
 types : List Ty
 values : List (Value n)
 references : List Handle
 rights : Delta
structure Store (n : Nat) where
 shared : Once.StateWithHistory
 frame : Local n

-- Supplied independently at EACH step, including after suspension. This record
-- is not a stored authority or an authorization consequence of typing.
structure Context (n : Nat) where
 current : RestoredCall.ExecutionContext n
 caller : CurrentUse.UseRequest n
 evidence : CurrentUse.Evidence
 proof : ModuleContractBoundary.CallProof n
 resourcePolicy : Nat → Bool

inductive Failure where
 | sourceType | referenceBudget | missingResource | boundaryDenied
 deriving DecidableEq, Repr
inductive Event where
 | allocation (returned : List Handle)
 | resource (action : Action) (returned : List Handle)
 deriving DecidableEq, Repr
structure Result (n : Nat) where
 store : Store n
 failure : Option Failure
 events : List Event

def unchanged (s : Store n) (failure : Failure) : Result n := ⟨s,some failure,[]⟩
def resourceStep (ctx : Context n) (s : Store n) (a : Action) : Result n :=
 match ResourceContext.execute ctx.resourcePolicy s.shared.resources s.frame.rights a with
 | none => unchanged s .boundaryDenied
 | some out =>
   ⟨{shared := {s.shared with resources := out.1},
      frame := {s.frame with rights := out.2.1,references := out.2.2 ++ s.frame.references}},
    none,[.resource a out.2.2]⟩

def namedResource (ctx : Context n) (s : Store n) (index : Nat) (make : Handle → Action) : Result n :=
 match s.frame.references[index]? with
 | none => unchanged s .missingResource
 | some h => resourceStep ctx s (make h)

def step (fuel : Nat) (ctx : Context n) (s : Store n) : Instruction n → Result n
 | .pureLet expression =>
   match infer s.frame.types expression with
   | none => unchanged s .sourceType
   | some type => match evaluate fuel s.frame.values expression with
     | none => unchanged s .referenceBudget
     | some value => ⟨{s with frame := {s.frame with types := (type :: s.frame.types), values := (value :: s.frame.values)}},none,[]⟩
 | .allocate source =>
   if check s.frame.types source then
     match select fuel s.frame.values source with
     | none => unchanged s .referenceBudget
     | some _ =>
       match allocateOnce fuel s.frame.types s.frame.values source ctx.current ctx.caller
         ctx.evidence ctx.proof s.shared s.frame.rights with
       | none => unchanged s .boundaryDenied
       | some out =>
         ⟨{shared := out.1,frame := {s.frame with rights := out.2.1, references := out.2.2 ++ s.frame.references}},none,[.allocation out.2.2]⟩
   else unchanged s .sourceType
 | .release index => namedResource ctx s index (.release ctx.caller.principal)
 | .move index receiver => namedResource ctx s index (fun h => .move ctx.caller.principal h receiver)
 | .split index cut => namedResource ctx s index (fun h => .split ctx.caller.principal h cut)

-- A failed later instruction retains every earlier successful state transition
-- and event. Fuel is a reference evaluator status, never a source-language effect.
def run (fuel : Nat) (contexts : Nat → Context n) (position : Nat)
 (s : Store n) : List (Instruction n) → Result n
 | [] => ⟨s,none,[]⟩
 | command :: rest =>
   let first := step fuel (contexts position) s command
   match first.failure with
   | some _ => first
   | none =>
     let later := run fuel contexts (position+1) first.store rest
     {later with events := first.events ++ later.events}

structure Good (s : Store n) : Prop where
 resources : WF s.shared.resources
 realized : Realized s.shared.resources s.frame.rights
 unique : s.shared.committed.Nodup
 environment : EnvTyped s.frame.values s.frame.types

theorem resourceStep_good {ctx : Context n} {s : Store n} (good : Good s) (a : Action) :
 Good (resourceStep ctx s a).store := by
 unfold resourceStep
 split
 · exact good
 · rename_i out executed
   have kept := ResourceContext.execute_preserves good.resources good.realized executed
   exact ⟨kept.1,kept.2,good.unique,good.environment⟩

theorem namedResource_good {ctx : Context n} {s : Store n} (good : Good s)
 (index : Nat) (make : Handle → Action) : Good (namedResource ctx s index make).store := by
 unfold namedResource
 split
 · exact good
 · exact resourceStep_good good _

theorem step_good {ctx : Context n} {s : Store n} (good : Good s)
 (fuel : Nat) (command : Instruction n) : Good (step fuel ctx s command).store := by
 cases command with
 | pureLet expression =>
   simp only [step]
   split
   · exact good
   · rename_i type inferred
     split
     · exact good
     · rename_i value evaluated
       refine ⟨good.resources,good.realized,good.unique,?_⟩
       exact .cons ((evaluator_typed fuel).1 good.environment ((infer_exact _ _ _).mp inferred) evaluated) good.environment
 | allocate source =>
   simp only [step]
   split
   · split
     · exact good
     · split
       · exact good
       · rename_i out allocated
         have kept := allocateOnce_preserves good.resources good.realized good.unique allocated
         exact ⟨kept.1,kept.2.1,kept.2.2,good.environment⟩
   · exact good
 | release index => exact namedResource_good good _ _
 | move index receiver => exact namedResource_good good _ _
 | split index cut => exact namedResource_good good _ _

theorem run_good {s : Store n} (good : Good s) (fuel : Nat)
 (contexts : Nat → Context n) (position : Nat) (commands : List (Instruction n)) :
 Good (run fuel contexts position s commands).store := by
 induction commands generalizing position s with
 | nil => exact good
 | cons command rest ih =>
   simp only [run]
   split
   · exact step_good good fuel command
   · exact ih (step_good good fuel command) (position+1)

#print axioms resourceStep_good
#print axioms step_good
#print axioms run_good
end MirroreaProofFirst.ResourceComputations.Sequence

namespace MirroreaProofFirst.ResourceComputations.Sequence
open PureHandleFunctions ResourceBoundary ResourceContext
open ModuleContractBoundary.CurrentAllocation

def eventRights (delta : Delta) : Event → Delta
 | .allocation returned => extend delta returned
 | .resource action returned => ResourceContext.update delta action returned
def replayRights (delta : Delta) (events : List Event) : Delta := events.foldl eventRights delta

theorem resourceStep_rights (ctx : Context n) (s : Store n) (action : Action) :
 (resourceStep ctx s action).store.frame.rights = replayRights s.frame.rights (resourceStep ctx s action).events := by
 unfold resourceStep
 split
 · rfl
 · rename_i out accepted
   obtain ⟨_,_,rfl⟩ := (ResourceContext.execute_exact _ _ _ _ _).mp accepted
   rfl

theorem namedResource_rights (ctx : Context n) (s : Store n) (index : Nat) (make : Handle → Action) :
 (namedResource ctx s index make).store.frame.rights = replayRights s.frame.rights (namedResource ctx s index make).events := by
 unfold namedResource
 split
 · rfl
 · exact resourceStep_rights _ _ _

theorem step_rights (fuel : Nat) (ctx : Context n) (s : Store n) (command : Instruction n) :
 (step fuel ctx s command).store.frame.rights = replayRights s.frame.rights (step fuel ctx s command).events := by
 cases command with
 | pureLet expression => simp only [step]; split; rfl; split <;> rfl
 | allocate source =>
   simp only [step]
   split
   · split
     · rfl
     · split
       · rfl
       · rename_i out accepted
         obtain ⟨_,h,x,result,_,_,rfl⟩ := allocateOnce_selected accepted
         rfl
   · rfl
 | release index => exact namedResource_rights _ _ _ _
 | move index receiver => exact namedResource_rights _ _ _ _
 | split index cut => exact namedResource_rights _ _ _ _

theorem run_rights (fuel : Nat) (contexts : Nat → Context n) (position : Nat)
 (s : Store n) (commands : List (Instruction n)) :
 (run fuel contexts position s commands).store.frame.rights =
 replayRights s.frame.rights (run fuel contexts position s commands).events := by
 induction commands generalizing position s with
 | nil => rfl
 | cons command rest ih =>
   simp only [run]
   split
   · exact step_rights _ _ _ _
   · simp only [replayRights,List.foldl_append]
     have one := step_rights fuel (contexts position) s command
     simp only [replayRights] at one
     rw [← one]
     exact ih _ _

theorem resourceStep_failure {ctx : Context n} {s : Store n} {a : Action} {f : Failure}
 (failed : (resourceStep ctx s a).failure = some f) :
 (resourceStep ctx s a).store = s ∧ (resourceStep ctx s a).events = [] := by
 unfold resourceStep at failed ⊢
 split at failed <;> simp_all [unchanged]

theorem namedResource_failure {ctx : Context n} {s : Store n} {index : Nat} {make : Handle → Action} {f : Failure}
 (failed : (namedResource ctx s index make).failure = some f) :
 (namedResource ctx s index make).store = s ∧ (namedResource ctx s index make).events = [] := by
 unfold namedResource at failed ⊢
 split at failed
 · simp_all [unchanged]
 · rename_i h found
   simpa [found] using resourceStep_failure failed

theorem step_failure {fuel : Nat} {ctx : Context n} {s : Store n} {command : Instruction n} {f : Failure}
 (failed : (step fuel ctx s command).failure = some f) :
 (step fuel ctx s command).store = s ∧ (step fuel ctx s command).events = [] := by
 cases command with
 | pureLet expression =>
   simp only [step] at failed ⊢
   split at failed
   · simp_all [unchanged]
   · split at failed <;> simp_all [unchanged]
 | allocate source =>
   simp only [step] at failed ⊢
   split at failed
   · split at failed
     · simp_all [unchanged]
     · split at failed <;> simp_all [unchanged]
   · simp_all [unchanged]
 | release index => exact namedResource_failure failed
 | move index receiver => exact namedResource_failure failed
 | split index cut => exact namedResource_failure failed

-- This law is intentionally failure-aware: no claim that the whole sequence
-- rolls back. The tail runs from the committed pre's actual output store.
theorem run_append (fuel : Nat) (contexts : Nat → Context n) (position : Nat)
 (s : Store n) (pre suffix : List (Instruction n)) :
 run fuel contexts position s (pre ++ suffix) =
 let first := run fuel contexts position s pre
 match first.failure with
 | some _ => first
 | none =>
   let later := run fuel contexts (position + pre.length) first.store suffix
   {later with events := first.events ++ later.events} := by
 induction pre generalizing position s with
 | nil => simp [run]
 | cons command rest ih =>
   simp only [List.cons_append,run]
   split
   · simp_all
   · rw [ih]
     split <;> simp_all [List.length_cons,List.append_assoc,Nat.add_assoc,Nat.add_comm ]

#print axioms step_rights
#print axioms run_rights
#print axioms step_failure
#print axioms run_append
end MirroreaProofFirst.ResourceComputations.Sequence

namespace MirroreaProofFirst.ResourceComputations.Sequence.Controls
open PureHandleFunctions ResourceBoundary ResourceContext
open ModuleContractBoundary ModuleContractBoundary.CurrentAllocation
-- One reusable square-plus-one module and one source program, with varying
-- input values. The context below is test authority, never generated by a proof.
def source : Source 4 := ⟨.handle HandleValues.Controls.token,.app (.var 0) (.var 1)⟩
def program : List (Instruction 4) :=
 [.pureLet (.lambda .int (.add (.var 0) (.integer 1))),.allocate source,
  .split 0 1,.move 0 4,.release 0,.release 2]
def initial (x : Int) : Store 4 :=
 ⟨Once.Controls.initial,⟨[.int],[.integer x],[],fun _ => none⟩⟩
def useFor (x : Int) : CurrentUse.UseRequest 4 :=
 {CurrentUse.Controls.request with arguments := [.integer (x+1)]}
def proofFor (x : Int) : CallProof 4 :=
 {ModuleContractBoundary.Controls.symbolicProof with
  context := CurrentUse.currentContext CurrentUse.Controls.world (useFor x)
  payload := ⟨expected CurrentUse.Controls.world (useFor x)
    ModuleContractBoundary.Controls.symbolicDescriptor [x+1],(x+1)*(x+1)+1,
    .square (.input 0),.square (.input 0)⟩}
def context (x : Int) (position : Nat) : Context 4 :=
 {current := {RestoredCall.Controls.current with limits := ⟨16,16⟩,lo := -10000,hi := 10000, registry := fun _ => some ModuleContractBoundary.Controls.symbolicDescriptor, catalog := fun _ => some ModuleContractBoundary.Controls.symbolicDescriptor, grant := fun _ _ _ _ _ => true}, caller := if position = 4 then {useFor x with principal := 4} else useFor x, evidence := {CurrentUse.Controls.evidence with context := CurrentUse.currentContext CurrentUse.Controls.world (useFor x)}, proof := proofFor x,resourcePolicy := fun _ => true}
def result (x : Int) := run 20 (context x) 0 (initial x) program

example : (result 3).failure = none ∧ (result 4).failure = none := by decide
example : (result 3).events.length = 5 ∧ (result 3).store.shared.resources.nextId = 4 := by decide
example : ((result 3).events.head?).map (fun e => match e with
 | .allocation hs => (hs.head?).map (fun h => h.region.hi)
 | _ => none) = some (some 17) := by decide
example : ((result 4).events.head?).map (fun e => match e with
 | .allocation hs => (hs.head?).map (fun h => h.region.hi)
 | _ => none) = some (some 26) := by decide
example : (result 3).store.frame.rights 0 = none ∧ (result 3).store.frame.rights 1 = none ∧
 (result 3).store.frame.rights 2 = none ∧ (result 3).store.frame.rights 3 = none := by decide

theorem initial_good (x : Int) : Good (initial x) :=
 ⟨empty_wf,by intro id region h; contradiction,by simp [initial,Once.Controls.initial],.cons .integer .nil⟩
theorem program_good (x : Int) : Good (result x).store := run_good (initial_good x) _ _ _ _

-- The first successful allocation and its rights survive a later stale alias.
def stale := run 20 (context 3) 0 (initial 3)
 [.pureLet (.lambda .int (.add (.var 0) (.integer 1))), .allocate source,.split 0 1,.release 2]
example : stale.failure = some .boundaryDenied ∧ stale.events.length = 2 := by decide
example : stale.store.frame.rights 1 = some ⟨0,0,1,3⟩ ∧
 stale.store.frame.rights 2 = some ⟨0,1,17,3⟩ ∧ stale.store.frame.rights 0 = none := by decide
-- Repeated source calls through an aliased interface share one actual UseId.
def repeated := run 20 (context 3) 0 (initial 3)
 [.pureLet (.lambda .int (.add (.var 0) (.integer 1))),.allocate source,.allocate source]
example : repeated.failure = some .boundaryDenied ∧ repeated.events.length = 1 ∧
 repeated.store.shared.resources.nextId = 1 := by decide
-- An old proof is not transferable to a newly computed argument.
def changedArgument := run 20 (context 3) 0 (initial 4) program
example : changedArgument.failure = some .boundaryDenied ∧ changedArgument.events = [] := by decide
-- Source rejection and reference budget have distinct observable INTERNAL tags.
example : (step 20 (context 3 0) (initial 3) (.pureLet (.var 9))).failure = some .sourceType := by decide
example : (step 0 (context 3 0) (initial 3) (.pureLet (.integer 2))).failure = some .referenceBudget := by decide
#print axioms program_good
end MirroreaProofFirst.ResourceComputations.Sequence.Controls

namespace MirroreaProofFirst.ResourceComputations.Sequence.Semantics
open PureHandleFunctions ResourceBoundary ResourceContext
open ModuleContractBoundary ModuleContractBoundary.CurrentAllocation

-- Independent admission relation for the selected invocation. Submitted
-- certificate acceptance stays explicit: semantic validity alone does not imply
-- acceptance of every certificate, nor does it grant authority.
def Admission (i : Once.Invocation n) (s : Once.StateWithHistory)
 (out : Once.StateWithHistory × List Handle) : Prop :=
 i.key ∉ s.committed ∧ ∃ d value,
 i.registry i.request.operation.key = some d ∧ i.catalog d.codeId = some d ∧
 CurrentUse.checkUse i.world i.request i.evidence = true ∧ Linked i.world i.request d i.arguments i.proof ∧
 ContractExport.CheckedArithmetic.Denotes i.lo i.hi i.arguments d.code i.proof.payload.result ∧
 ContractExport.accept (expected i.world i.request d i.arguments) i.proof.payload = some value ∧
 i.grant i.world i.request d s.resources (.allocate i.request.principal value) = true ∧
 BoundedIdentifiers.Capacity i.limits s.resources (.allocate i.request.principal value) ∧
 out = (⟨(raw s.resources (.allocate i.request.principal value)).1,i.key :: s.committed⟩,
   (raw s.resources (.allocate i.request.principal value)).2)

theorem admission_exact (i : Once.Invocation n) (s : Once.StateWithHistory)
 (out : Once.StateWithHistory × List Handle) : Once.step i s = some out ↔ Admission i s out := by
 rw [Once.step_exact]
 constructor
 · rintro ⟨fresh,result,executed,rfl⟩
   obtain ⟨d,value,lookup,catalog,current,linked,machine,accepted,grant,capacity,rfl⟩ :=
     (CurrentAllocation.exact _ _ _ _ _ _ _ _ _ _ _ _ _).mp executed
   exact ⟨fresh,d,value,lookup,catalog,current,linked,machine,accepted,grant,capacity,rfl⟩
 · rintro ⟨fresh,d,value,lookup,catalog,current,linked,machine,accepted,grant,capacity,rfl⟩
   exact ⟨fresh,_,(CurrentAllocation.exact _ _ _ _ _ _ _ _ _ _ _ _ _).mpr
     ⟨d,value,lookup,catalog,current,linked,machine,accepted,grant,capacity,rfl⟩,rfl⟩

def resourceResult (s : Store n) (a : Action) : Result n :=
 let out := raw s.shared.resources a
 ⟨⟨{s.shared with resources := out.1},{s.frame with rights := ResourceContext.update s.frame.rights a out.2, references := out.2 ++ s.frame.references}⟩,
 none,[.resource a out.2]⟩
inductive ResourceTransition (ctx : Context n) (s : Store n) (a : Action) : Result n → Prop where
 | accepted : Owned s.frame.rights a → Allowed ctx.resourcePolicy s.shared.resources a →
     ResourceTransition ctx s a (resourceResult s a)
 | denied : (¬ Owned s.frame.rights a ∨ ¬ Allowed ctx.resourcePolicy s.shared.resources a) →
     ResourceTransition ctx s a (unchanged s .boundaryDenied)

theorem resource_transition_exact (ctx : Context n) (s : Store n) (a : Action) (out : Result n) :
 resourceStep ctx s a = out ↔ ResourceTransition ctx s a out := by
 constructor
 · intro equal; subst out
   unfold resourceStep
   cases executed : ResourceContext.execute ctx.resourcePolicy s.shared.resources s.frame.rights a with
   | none =>
     apply ResourceTransition.denied
     by_cases owned : Owned s.frame.rights a
     · right; intro allowed
       have yes := (ResourceContext.execute_exact _ _ _ _ _).mpr ⟨owned,allowed,rfl⟩
       rw [executed] at yes; contradiction
     · exact Or.inl owned
   | some result =>
     obtain ⟨owned,allowed,rfl⟩ := (ResourceContext.execute_exact _ _ _ _ _).mp executed
     exact .accepted owned allowed
 · intro transition
   cases transition with
   | accepted owned allowed =>
     have yes := (ResourceContext.execute_exact _ _ _ _ _).mpr ⟨owned,allowed,rfl⟩
     simp [resourceStep,yes,resourceResult]
   | denied denied =>
     have no : ResourceContext.execute ctx.resourcePolicy s.shared.resources s.frame.rights a = none := by
       cases executed : ResourceContext.execute ctx.resourcePolicy s.shared.resources s.frame.rights a with
       | none => rfl
       | some result =>
         obtain ⟨owned,allowed,_⟩ := (ResourceContext.execute_exact _ _ _ _ _).mp executed
         exact False.elim (denied.elim (fun h => h owned) (fun h => h allowed))
     simp [resourceStep,no]

inductive NamedTransition (ctx : Context n) (s : Store n) (index : Nat) (make : Handle → Action) : Result n → Prop where
 | missing : s.frame.references[index]? = none → NamedTransition ctx s index make (unchanged s .missingResource)
 | found : s.frame.references[index]? = some h → ResourceTransition ctx s (make h) out → NamedTransition ctx s index make out

theorem named_transition_exact (ctx : Context n) (s : Store n) (index : Nat)
 (make : Handle → Action) (out : Result n) : namedResource ctx s index make = out ↔ NamedTransition ctx s index make out := by
 constructor
 · intro eq; subst out
   cases found : s.frame.references[index]? with
   | none => simpa [namedResource,found] using NamedTransition.missing (ctx := ctx) (make := make) found
   | some h =>
     simpa [namedResource,found] using (NamedTransition.found found ((resource_transition_exact ctx s (make h) _).mp rfl))
 · intro transition
   cases transition with
   | missing absent => simp [namedResource,absent]
   | found present transition => simpa [namedResource,present] using (resource_transition_exact _ _ _ _).mpr transition

#print axioms admission_exact
#print axioms resource_transition_exact
#print axioms named_transition_exact
end MirroreaProofFirst.ResourceComputations.Sequence.Semantics

namespace MirroreaProofFirst.ResourceComputations.Sequence.Semantics
open PureHandleFunctions ResourceBoundary ResourceContext
open ModuleContractBoundary.CurrentAllocation

def pureResult (s : Store n) (t : Ty) (v : Value n) : Result n :=
 ⟨{s with frame := {s.frame with types := t :: s.frame.types, values := v :: s.frame.values}},none,[]⟩
def allocationResult (s : Store n) (out : Once.StateWithHistory × List Handle) : Result n :=
 ⟨⟨out.1,{s.frame with rights := extend s.frame.rights out.2, references := out.2 ++ s.frame.references}⟩,none,[.allocation out.2]⟩

-- No reference fuel constructor: budget exhaustion is NOT a language outcome.
inductive Transition (ctx : Context n) (s : Store n) : Instruction n → Result n → Prop where
 | pureRejected : (∀ t, ¬ PureHandleFunctions.Typed s.frame.types e t) →
     Transition ctx s (.pureLet e) (unchanged s .sourceType)
 | pureValue : PureHandleFunctions.Typed s.frame.types e t →
     Executes (.expression s.frame.values e) v → Transition ctx s (.pureLet e) (pureResult s t v)
 | callRejected : ¬ Typed s.frame.types source →
     Transition ctx s (.allocate source) (unchanged s .sourceType)
 | callDenied : Typed s.frame.types source → Selects s.frame.values source h x →
     (∀ out, ¬ Admission (invocation ctx.current ctx.caller ctx.evidence ctx.proof h x) s.shared out) →
     Transition ctx s (.allocate source) (unchanged s .boundaryDenied)
 | callValue : Typed s.frame.types source → Selects s.frame.values source h x →
     Admission (invocation ctx.current ctx.caller ctx.evidence ctx.proof h x) s.shared out →
     Transition ctx s (.allocate source) (allocationResult s out)
 | release : NamedTransition ctx s index (.release ctx.caller.principal) out → Transition ctx s (.release index) out
 | move : NamedTransition ctx s index (fun h => .move ctx.caller.principal h receiver) out → Transition ctx s (.move index receiver) out
 | split : NamedTransition ctx s index (fun h => .split ctx.caller.principal h cut) out → Transition ctx s (.split index cut) out

theorem no_type_exact (G : List Ty) (e : Expr n) :
 infer G e = none ↔ ∀ t, ¬ PureHandleFunctions.Typed G e t := by
 constructor
 · intro no t typed; rw [typed_infer typed] at no; contradiction
 · intro no
   cases inferred : infer G e with
   | none => rfl
   | some t => exact False.elim (no t ((infer_exact _ _ _).mp inferred))

theorem selected_step {fuel : Nat} {ctx : Context n} {s : Store n} {source : Source n}
 {h : HandleValues.Interface n} {x : Int}
 (typed : Typed s.frame.types source) (selected : select fuel s.frame.values source = some (h,x)) :
 step fuel ctx s (.allocate source) =
 match Once.step (invocation ctx.current ctx.caller ctx.evidence ctx.proof h x) s.shared with
 | none => unchanged s .boundaryDenied
 | some out => allocationResult s out := by
 simp only [step,(check_exact _ _).mpr typed,ite_true,selected,allocateOnce,Option.bind_some]
 cases executed : Once.step (invocation ctx.current ctx.caller ctx.evidence ctx.proof h x) s.shared <;> rfl

theorem transition_eventually {ctx : Context n} {s : Store n} {command : Instruction n} {out : Result n}
 (transition : Transition ctx s command out) :
 ∃ minimum, ∀ fuel, minimum ≤ fuel → step fuel ctx s command = out := by
 cases transition with
 | pureRejected no => exact ⟨0,fun _ _ => by simp [step,(no_type_exact _ _).mpr no]⟩
 | pureValue typed executed =>
   obtain ⟨minimum,enough⟩ := execution_complete executed
   refine ⟨minimum,fun fuel bound => ?_⟩
   have value := enough fuel bound
   simp only [runTask] at value
   simp [step,typed_infer typed,value,pureResult]
 | callRejected no => exact ⟨0,fun _ _ => by simp [step,show check s.frame.types _ = false from Bool.eq_false_iff.mpr (fun h => no ((check_exact _ _).mp h))]⟩
 | callDenied typed selected no =>
   rename_i source h x
   obtain ⟨minimum,enough⟩ := select_complete selected
   have denied : Once.step (invocation ctx.current ctx.caller ctx.evidence ctx.proof h x) s.shared = none := by
     cases executed : Once.step (invocation ctx.current ctx.caller ctx.evidence ctx.proof h x) s.shared with
     | none => rfl
     | some out => exact False.elim (no out ((admission_exact _ _ _).mp executed))
   exact ⟨minimum,fun fuel bound => by rw [selected_step typed (enough fuel bound),denied]⟩
 | callValue typed selected admitted =>
   obtain ⟨minimum,enough⟩ := select_complete selected
   exact ⟨minimum,fun fuel bound => by rw [selected_step typed (enough fuel bound),(admission_exact _ _ _).mpr admitted]⟩
 | release transition => exact ⟨0,fun _ _ => (named_transition_exact _ _ _ _ _).mpr transition⟩
 | move transition => exact ⟨0,fun _ _ => (named_transition_exact _ _ _ _ _).mpr transition⟩
 | split transition => exact ⟨0,fun _ _ => (named_transition_exact _ _ _ _ _).mpr transition⟩

#print axioms selected_step
#print axioms transition_eventually
end MirroreaProofFirst.ResourceComputations.Sequence.Semantics

namespace MirroreaProofFirst.ResourceComputations.Sequence.Semantics
open PureHandleFunctions ResourceBoundary ResourceContext
open ModuleContractBoundary.CurrentAllocation

theorem completed_step_sound {fuel : Nat} {ctx : Context n} {s : Store n} {command : Instruction n}
 (completed : (step fuel ctx s command).failure ≠ some .referenceBudget) :
 Transition ctx s command (step fuel ctx s command) := by
 cases command with
 | pureLet expression =>
   cases inferred : infer s.frame.types expression with
   | none => simpa [step,inferred] using Transition.pureRejected (ctx := ctx) (s := s) ((no_type_exact _ _).mp inferred)
   | some t =>
     cases evaluated : evaluate fuel s.frame.values expression with
     | none => exact False.elim (completed (by simp [step,inferred,evaluated,unchanged]))
     | some v =>
       have semantic := (execution_exact (.expression s.frame.values expression) v).mp ⟨fuel,evaluated⟩
       simpa [step,inferred,evaluated,pureResult] using (Transition.pureValue (ctx := ctx) (s := s) ((infer_exact _ _ _).mp inferred) semantic)
 | allocate source =>
   by_cases checked : check s.frame.types source = true
   · have typed := (check_exact _ _).mp checked
     cases selected : select fuel s.frame.values source with
     | none => exact False.elim (completed (by simp [step,checked,selected,unchanged]))
     | some pair =>
       rw [selected_step typed selected]
       cases executed : Once.step (invocation ctx.current ctx.caller ctx.evidence ctx.proof pair.1 pair.2) s.shared with
       | none =>
         apply Transition.callDenied typed (select_sound selected)
         intro out admitted
         have yes := (admission_exact _ _ _).mpr admitted
         rw [executed] at yes; contradiction
       | some out => exact .callValue typed (select_sound selected) ((admission_exact _ _ _).mp executed)
   · have no : ¬ Typed s.frame.types source := fun h => checked ((check_exact _ _).mpr h)
     simpa [step,checked] using Transition.callRejected (ctx := ctx) (s := s) no
 | release index => exact .release ((named_transition_exact _ _ _ _ _).mp rfl)
 | move index receiver => exact .move ((named_transition_exact _ _ _ _ _).mp rfl)
 | split index cut => exact .split ((named_transition_exact _ _ _ _ _).mp rfl)

theorem transition_not_budget {ctx : Context n} {s : Store n} {command : Instruction n} {out : Result n}
 (transition : Transition ctx s command out) : out.failure ≠ some .referenceBudget := by
 cases transition with
 | pureRejected _ => simp [unchanged]
 | pureValue _ _ => simp [pureResult]
 | callRejected _ => simp [unchanged]
 | callDenied _ _ _ => simp [unchanged]
 | callValue _ _ _ => simp [allocationResult]
 | release h | move h | split h =>
   cases h with
   | missing _ => simp [unchanged]
   | found _ h => cases h <;> simp [unchanged,resourceResult]

-- Declarative sequential composition preserves the actual failed result and the
-- preceding event prefix. It neither calls run nor contains a preservation axiom.
inductive Runs (contexts : Nat → Context n) : Nat → Store n → List (Instruction n) → Result n → Prop where
 | done : Runs contexts position s [] ⟨s,none,[]⟩
 | failed {f : Failure} : Transition (contexts position) s command first → first.failure = some f →
     Runs contexts position s (command :: rest) first
 | next : Transition (contexts position) s command first → first.failure = none →
     Runs contexts (position+1) first.store rest later →
     Runs contexts position s (command :: rest) {later with events := first.events ++ later.events}

theorem runs_eventually {contexts : Nat → Context n} {position : Nat} {s : Store n}
 {commands : List (Instruction n)} {out : Result n} (semantic : Runs contexts position s commands out) :
 ∃ minimum, ∀ fuel, minimum ≤ fuel → run fuel contexts position s commands = out := by
 induction semantic with
 | done => exact ⟨0,fun _ _ => rfl⟩
 | failed transition failed =>
   obtain ⟨minimum,enough⟩ := transition_eventually transition
   exact ⟨minimum,fun fuel bound => by simp [run,enough fuel bound,failed]⟩
 | next transition success semantic ih =>
   obtain ⟨a,ha⟩ := transition_eventually transition
   obtain ⟨b,hb⟩ := ih
   exact ⟨max a b,fun fuel bound => by simp [run,ha fuel (by omega),success,hb fuel (by omega)]⟩

theorem completed_run_sound {fuel : Nat} {contexts : Nat → Context n} {position : Nat}
 {s : Store n} {commands : List (Instruction n)}
 (completed : (run fuel contexts position s commands).failure ≠ some .referenceBudget) :
 Runs contexts position s commands (run fuel contexts position s commands) := by
 induction commands generalizing position s with
 | nil => exact .done
 | cons command rest ih =>
   cases firstStatus : (step fuel (contexts position) s command).failure with
   | some failure =>
     have firstCompleted : (step fuel (contexts position) s command).failure ≠ some .referenceBudget := by
       simpa [run,firstStatus] using completed
     simpa [run,firstStatus] using Runs.failed (rest := rest) (completed_step_sound firstCompleted) firstStatus
   | none =>
     have firstCompleted : (step fuel (contexts position) s command).failure ≠ some .referenceBudget := by simp [firstStatus]
     have laterCompleted : (run fuel contexts (position+1) (step fuel (contexts position) s command).store rest).failure ≠ some .referenceBudget := by
       simpa [run,firstStatus] using completed
     simpa [run,firstStatus] using Runs.next (completed_step_sound firstCompleted) firstStatus (ih laterCompleted)

#print axioms completed_step_sound
#print axioms transition_not_budget
#print axioms runs_eventually
#print axioms completed_run_sound
end MirroreaProofFirst.ResourceComputations.Sequence.Semantics

namespace MirroreaProofFirst.ResourceComputations.Continuation
open PureHandleFunctions ResourceBoundary ResourceContext Sequence
open ModuleContractBoundary.CurrentAllocation

-- Bounded scheduler continuation: one active or suspended frame for this owner.
-- It captures the ACTUAL residual program and lexical/resource environment.
-- Pure Value has no constructor for this frame or its resumable token.
structure Frame (n : Nat) where
 lexical : Sequence.Local n
 pending : List (Instruction n)
 position : Nat
inductive Phase (n : Nat) where
 | active (frame : Frame n)
 | suspended (identity : Nat) (frame : Frame n)
 | halted (frame : Frame n) (failure : Option Failure)
def Phase.frame : Phase n → Frame n
 | .active f | .suspended _ f | .halted f _ => f
structure Configuration (n : Nat) where
 shared : Once.StateWithHistory
 phase : Phase n
 nextIdentity : Nat
 events : List Event

def store (c : Configuration n) : Store n := ⟨c.shared,c.phase.frame.lexical⟩
def Good (c : Configuration n) : Prop := Sequence.Good (store c)
def start (s : Store n) (body : List (Instruction n)) : Configuration n :=
 ⟨s.shared,.active ⟨s.frame,body,0⟩,0,[]⟩

-- Capturing removes the runnable phase. A copied numeric token is not a second
-- frame; resume removes the suspended phase before any effect can run.
def capture (c : Configuration n) : Option (Configuration n × Nat) :=
 match c.phase with
 | .active frame => some ({c with phase := .suspended c.nextIdentity frame,nextIdentity := c.nextIdentity+1},c.nextIdentity)
 | _ => none
def resume (identity : Nat) (c : Configuration n) : Option (Configuration n) :=
 match c.phase with
 | .suspended key frame => if identity = key then some {c with phase := .active frame} else none
 | _ => none

def tick (fuel : Nat) (ctx : Context n) (c : Configuration n) : Configuration n :=
 match c.phase with
 | .active frame =>
   match frame.pending with
   | [] => {c with phase := .halted frame none}
   | command :: rest =>
     let first := Sequence.step fuel ctx ⟨c.shared,frame.lexical⟩ command
     let nextFrame : Frame n := ⟨first.store.frame,rest,frame.position+1⟩
     let phase := match first.failure with
       | none => Phase.active nextFrame
       | some failure => Phase.halted {nextFrame with pending := frame.pending,position := frame.position} (some failure)
     {c with shared := first.store.shared,phase := phase,events := c.events ++ first.events}
 | _ => c

def drive (fuel : Nat) (contexts : Nat → Context n) : Nat → Configuration n → Configuration n
 | 0,c => c
 | ticks+1,c => drive fuel contexts ticks (tick fuel (contexts c.phase.frame.position) c)

theorem capture_frame {c out : Configuration n} {identity : Nat}
 (captured : capture c = some (out,identity)) :
 out.shared = c.shared ∧ out.phase.frame = c.phase.frame ∧ out.events = c.events ∧
 identity = c.nextIdentity ∧ out.nextIdentity = c.nextIdentity+1 := by
 unfold capture at captured
 split at captured
 · cases captured; simp_all [Phase.frame]
 · contradiction

theorem resume_frame {identity : Nat} {c out : Configuration n}
 (resumed : resume identity c = some out) :
 out.shared = c.shared ∧ out.phase.frame = c.phase.frame ∧ out.events = c.events ∧ out.nextIdentity = c.nextIdentity := by
 unfold resume at resumed
 split at resumed
 · split at resumed
   · cases resumed; simp_all [Phase.frame]
   · contradiction
 · contradiction

theorem capture_good {c out : Configuration n} {identity : Nat} (good : Good c)
 (captured : capture c = some (out,identity)) : Good out := by
 obtain ⟨shared,frame,_,_,_⟩ := capture_frame captured
 simpa [Good,store,shared,frame] using good

theorem resume_good {identity : Nat} {c out : Configuration n} (good : Good c)
 (resumed : resume identity c = some out) : Good out := by
 obtain ⟨shared,frame,_,_⟩ := resume_frame resumed
 simpa [Good,store,shared,frame] using good

theorem tick_good {c : Configuration n} (good : Good c) (fuel : Nat) (ctx : Context n) : Good (tick fuel ctx c) := by
 unfold tick
 split
 · rename_i frame active
   have before : Sequence.Good (⟨c.shared,frame.lexical⟩ : Store n) := by simpa [Good,store,active,Phase.frame] using good
   split
   · simpa [Good,store,Phase.frame] using before
   · rename_i command rest pending
     have after := Sequence.step_good (ctx := ctx) before fuel command
     cases status : (Sequence.step fuel ctx ⟨c.shared,frame.lexical⟩ command).failure <;>
       simpa [Good,store,Phase.frame,status] using after
 · exact good

theorem drive_good {c : Configuration n} (good : Good c) (fuel : Nat) (contexts : Nat → Context n) (ticks : Nat) :
 Good (drive fuel contexts ticks c) := by
 induction ticks generalizing c with
 | zero => exact good
 | succ ticks ih => exact ih (tick_good good fuel _)

-- Freshly captured computation resumes with exactly its prior frame. Contexts
-- are not captured: tick/drive still require the current independent provider.
theorem capture_resume {c out : Configuration n} {identity : Nat}
 (captured : capture c = some (out,identity)) :
 resume identity out = some {c with nextIdentity := c.nextIdentity+1} := by
 unfold capture at captured
 split at captured
 · cases captured; simp_all [resume]
 · contradiction

theorem resume_once {c out : Configuration n} {identity : Nat}
 (resumed : resume identity c = some out) (alias : Nat) : resume alias out = none := by
 unfold resume at resumed
 split at resumed
 · split at resumed
   · cases resumed; rfl
   · contradiction
 · contradiction

#print axioms capture_frame
#print axioms resume_frame
#print axioms capture_good
#print axioms resume_good
#print axioms tick_good
#print axioms drive_good
#print axioms capture_resume
#print axioms resume_once
end MirroreaProofFirst.ResourceComputations.Continuation

namespace MirroreaProofFirst.ResourceComputations.Continuation
open PureHandleFunctions ResourceBoundary ResourceContext Sequence
open ModuleContractBoundary.CurrentAllocation

-- Retry only the reference-budget pause, from its saved residual frame. A
-- semantic denial is terminal here and is not silently retried/re-authorized.
def retryBudget (c : Configuration n) : Configuration n :=
 match c.phase with
 | .halted frame (some .referenceBudget) => {c with phase := .active frame}
 | _ => c

def available (identity : Nat) (c : Configuration n) : Bool :=
 match c.phase with | .suspended key _ => decide (identity = key) | _ => false
def IdentityWF (c : Configuration n) : Prop :=
 match c.phase with | .suspended key _ => key < c.nextIdentity | _ => True
def Retired (identity : Nat) (c : Configuration n) : Prop :=
 identity < c.nextIdentity ∧ available identity c = false

theorem retry_frame (c : Configuration n) :
 (retryBudget c).shared = c.shared ∧ (retryBudget c).phase.frame = c.phase.frame ∧
 (retryBudget c).events = c.events ∧ (retryBudget c).nextIdentity = c.nextIdentity := by
 unfold retryBudget; split <;> simp_all [Phase.frame]

theorem retry_good {c : Configuration n} (good : Good c) : Good (retryBudget c) := by
 obtain ⟨shared,frame,_,_⟩ := retry_frame c
 simpa [Good,store,shared,frame] using good

theorem resumed_retired {identity : Nat} {c out : Configuration n} (valid : IdentityWF c)
 (resumed : resume identity c = some out) : Retired identity out := by
 unfold resume at resumed
 split at resumed
 · rename_i key frame suspended
   split at resumed
   · rename_i same; cases resumed
     simp_all [IdentityWF,Retired,available]
   · contradiction
 · contradiction

theorem tick_retired {identity : Nat} {c : Configuration n} (retired : Retired identity c)
 (fuel : Nat) (ctx : Context n) : Retired identity (tick fuel ctx c) := by
 unfold tick
 split
 · split
   · exact ⟨retired.1,rfl⟩
   · dsimp
     split <;> exact ⟨retired.1,rfl⟩
 · exact retired

theorem capture_retired {identity token : Nat} {c out : Configuration n}
 (retired : Retired identity c) (captured : capture c = some (out,token)) : Retired identity out := by
 unfold capture at captured
 split at captured
 · cases captured
   exact ⟨Nat.lt_trans retired.1 (Nat.lt_succ_self _),by simp [available,Nat.ne_of_lt retired.1]⟩
 · contradiction

theorem resume_retired {identity token : Nat} {c out : Configuration n}
 (retired : Retired identity c) (resumed : resume token c = some out) : Retired identity out := by
 unfold resume at resumed
 split at resumed
 · split at resumed
   · cases resumed; exact ⟨retired.1,rfl⟩
   · contradiction
 · contradiction

theorem retry_retired {identity : Nat} {c : Configuration n} (retired : Retired identity c) :
 Retired identity (retryBudget c) := by
 unfold retryBudget; split
 · exact ⟨retired.1,rfl⟩
 · exact retired

inductive Control (n : Nat) where
 | tick (fuel : Nat) (ctx : Context n)
 | capture
 | resume (identity : Nat)
 | retryBudget

def act (c : Configuration n) : Control n → Configuration n
 | .tick fuel ctx => tick fuel ctx c
 | .capture => ((capture c).map Prod.fst).getD c
 | .resume id => (resume id c).getD c
 | .retryBudget => retryBudget c

def schedule : Configuration n → List (Control n) → Configuration n
 | c,[] => c
 | c,a::rest => schedule (act c a) rest

theorem act_retired {identity : Nat} {c : Configuration n} (retired : Retired identity c)
 (action : Control n) : Retired identity (act c action) := by
 cases action with
 | tick fuel ctx => exact tick_retired retired _ _
 | retryBudget => exact retry_retired retired
 | capture =>
   cases captured : capture c with
   | none => simpa [act,captured] using retired
   | some out => simpa [act,captured] using capture_retired retired captured
 | resume token =>
   cases resumed : resume token c with
   | none => simpa [act,resumed] using retired
   | some out => simpa [act,resumed] using resume_retired retired resumed

theorem schedule_retired {identity : Nat} {c : Configuration n} (retired : Retired identity c)
 (actions : List (Control n)) : Retired identity (schedule c actions) := by
 induction actions generalizing c with
 | nil => exact retired
 | cons action rest ih => exact ih (act_retired retired action)

theorem unavailable_rejected {identity : Nat} {c : Configuration n}
 (unavailable : available identity c = false) : resume identity c = none := by
 unfold resume
 split
 · rename_i key frame phase
   have different : identity ≠ key := by simpa [available,phase] using unavailable
   simp [different]
 · rfl

theorem no_revival {identity : Nat} {c out : Configuration n} (valid : IdentityWF c)
 (resumed : resume identity c = some out) (later : List (Control n)) :
 resume identity (schedule out later) = none :=
 unavailable_rejected (schedule_retired (resumed_retired valid resumed) later).2

#print axioms retry_frame
#print axioms retry_good
#print axioms resumed_retired
#print axioms tick_retired
#print axioms capture_retired
#print axioms schedule_retired
#print axioms no_revival
end MirroreaProofFirst.ResourceComputations.Continuation

namespace MirroreaProofFirst.ResourceComputations.Continuation.Controls
open PureHandleFunctions ResourceBoundary Sequence
open ModuleContractBoundary.CurrentAllocation

def begun := start (Sequence.Controls.initial 3) Sequence.Controls.program
def prepared := tick 20 (Sequence.Controls.context 3 0) begun
def suspended := (capture prepared).map Prod.fst
def resumed := suspended.bind (resume 0)
def finished := resumed.map (drive 20 (Sequence.Controls.context 3) 6)
def outcome (c : Configuration n) : Option Failure :=
 match c.phase with | .halted _ status => status | _ => none

example : suspended.isSome = true ∧ resumed.isSome = true := by decide
example : suspended.map (fun c => c.phase.frame.lexical.values[1]?) = some (some (.integer 3)) := by rfl
example : resumed.map (fun c => c.phase.frame.lexical.values[1]?) = some (some (.integer 3)) := by rfl
example : finished.map (fun c => (outcome c,c.events.length,c.shared.resources.nextId)) = some (none,5,4) := by decide
example : (resumed.bind (resume 0)).isNone = true := by decide
-- The scheduler cannot execute a suspended frame before consuming its token.
example : suspended.map (fun c => (tick 20 (Sequence.Controls.context 3 1) c).events.length) = some 0 := by decide

-- Current revocation/independent grant/descriptor inputs are acquired at resume
-- time by the provider; the captured frame has no World or authorization grant.
def revoked (position : Nat) : Context 4 :=
 let base := Sequence.Controls.context 3 position
 {base with current := {base.current with world := (CurrentUse.revoke ⟨base.current.world,[]⟩ 1).world}}
def denied (position : Nat) : Context 4 :=
 let base := Sequence.Controls.context 3 position
 {base with current := {base.current with grant := fun _ _ _ _ _ => false}}
def replaced (position : Nat) : Context 4 :=
 let base := Sequence.Controls.context 3 position
 {base with current := {base.current with registry := fun _ => some {ModuleContractBoundary.Controls.symbolicDescriptor with code := .integer 17}}}
example : (resumed.map (drive 20 revoked 6)).map (fun c => (outcome c,c.events.length,c.shared.resources.nextId)) =
 some (some .boundaryDenied,0,0) := by decide
example : (resumed.map (drive 20 denied 6)).map (fun c => (outcome c,c.events.length)) =
 some (some .boundaryDenied,0) := by decide
example : (resumed.map (drive 20 replaced 6)).map (fun c => (outcome c,c.events.length)) =
 some (some .boundaryDenied,0) := by decide
-- Replacing the lexical value by 100 is a different computation and fails the
-- proof for captured input3; ordinary resume preserves input3 (frame laws above).
example : (drive 20 (Sequence.Controls.context 3) 6
 (start (Sequence.Controls.initial 100) Sequence.Controls.program)).shared.resources.nextId = 0 := by decide

def allocated := drive 20 (Sequence.Controls.context 3) 2 begun
def held := (capture allocated).map Prod.fst
example : held.map (fun c => c.phase.frame.lexical.rights 0) = some (some ⟨0,0,17,3⟩) := by decide
example : (held.bind (resume 0)).map (fun c => c.phase.frame.lexical.rights 0) = some (some ⟨0,0,17,3⟩) := by decide

-- Exhaustion AFTER an effect keeps the residual instruction and its rights.
-- Retrying the original whole program would duplicate or deny the first call;
-- this retry runs only the saved residual and does not allocate a second time.
def budgetProgram : List (Instruction 4) :=
 [.pureLet (.lambda .int (.add (.var 0) (.integer 1))),.allocate Sequence.Controls.source,
  .pureLet (.app (.lambda .int (.var 0)) (.integer 7)),.release 0]
def beforeBudget := drive 20 (Sequence.Controls.context 3) 2 (start (Sequence.Controls.initial 3) budgetProgram)
def budgetPaused := tick 0 (Sequence.Controls.context 3 2) beforeBudget
def budgetRetried := drive 20 (Sequence.Controls.context 3) 3 (retryBudget budgetPaused)
example : outcome budgetPaused = some .referenceBudget ∧ budgetPaused.phase.frame.pending.length = 2 ∧
 budgetPaused.events.length = 1 ∧ budgetPaused.shared.resources.nextId = 1 := by decide
example : budgetPaused.phase.frame.lexical.rights 0 = some ⟨0,0,17,3⟩ := by decide
example : outcome budgetRetried = none ∧ budgetRetried.events.length = 2 ∧
 budgetRetried.shared.resources.nextId = 1 ∧ budgetRetried.phase.frame.lexical.rights 0 = none := by decide
end MirroreaProofFirst.ResourceComputations.Continuation.Controls

namespace MirroreaProofFirst.ResourceComputations.Continuation
open PureHandleFunctions ResourceBoundary ResourceContext Sequence
open ModuleContractBoundary.CurrentAllocation

theorem start_identity (s : Store n) (body : List (Instruction n)) : IdentityWF (start s body) := True.intro

theorem tick_identity {c : Configuration n} (valid : IdentityWF c) (fuel : Nat) (ctx : Context n) :
 IdentityWF (tick fuel ctx c) := by
 unfold tick; split
 · split
   · exact True.intro
   · dsimp; split <;> exact True.intro
 · exact valid

theorem capture_identity {c out : Configuration n} {token : Nat}
 (captured : capture c = some (out,token)) : IdentityWF out := by
 unfold capture at captured; split at captured
 · cases captured; exact Nat.lt_succ_self _
 · contradiction

theorem resume_identity {c out : Configuration n} {token : Nat}
 (resumed : resume token c = some out) : IdentityWF out := by
 unfold resume at resumed; split at resumed
 · split at resumed
   · cases resumed; exact True.intro
   · contradiction
 · contradiction

theorem act_good {c : Configuration n} (good : Good c) (action : Control n) : Good (act c action) := by
 cases action with
 | tick fuel ctx => exact tick_good good _ _
 | retryBudget => exact retry_good good
 | capture =>
   cases captured : capture c with
   | none => simpa [act,captured] using good
   | some out => simpa [act,captured] using capture_good good captured
 | resume token =>
   cases resumed : resume token c with
   | none => simpa [act,resumed] using good
   | some out => simpa [act,resumed] using resume_good good resumed

theorem act_identity {c : Configuration n} (valid : IdentityWF c) (action : Control n) : IdentityWF (act c action) := by
 cases action with
 | tick fuel ctx => exact tick_identity valid _ _
 | retryBudget =>
   simp only [act,retryBudget]; split
   · exact True.intro
   · exact valid
 | capture =>
   cases captured : capture c with
   | none => simpa [act,captured] using valid
   | some out => simpa [act,captured] using capture_identity captured
 | resume token =>
   cases resumed : resume token c with
   | none => simpa [act,resumed] using valid
   | some out => simpa [act,resumed] using resume_identity resumed

theorem schedule_good {c : Configuration n} (good : Good c) (valid : IdentityWF c)
 (actions : List (Control n)) : Good (schedule c actions) ∧ IdentityWF (schedule c actions) := by
 induction actions generalizing c with
 | nil => exact ⟨good,valid⟩
 | cons action rest ih => exact ih (act_good good action) (act_identity valid action)

-- A real scheduler step shares the very same operational transition and event
-- record as Sequence. No expected JSON or synthetic observer event is inserted.
def observe (c : Configuration n) : Option (Result n) :=
 match c.phase with
 | .halted frame status => some ⟨⟨c.shared,frame.lexical⟩,status,c.events⟩
 | _ => none

theorem drive_halted (fuel : Nat) (contexts : Nat → Context n) (ticks : Nat)
 (c : Configuration n) {frame : Frame n} {status : Option Failure}
 (halted : c.phase = .halted frame status) : drive fuel contexts ticks c = c := by
 induction ticks with
 | zero => rfl
 | succ ticks ih => simpa [drive,tick,halted] using ih

theorem drive_sequence (fuel : Nat) (contexts : Nat → Context n)
 (body : List (Instruction n)) (s : Store n) (position serial : Nat) (past : List Event) :
 observe (drive fuel contexts (body.length+1)
   ⟨s.shared,.active ⟨s.frame,body,position⟩,serial,past⟩) =
 some {Sequence.run fuel contexts position s body with events := past ++ (Sequence.run fuel contexts position s body).events} := by
 induction body generalizing s position past with
 | nil => simp [drive,tick,observe,Sequence.run]
 | cons command rest ih =>
   simp only [List.length_cons]
   rw [drive]
   simp only [tick,Phase.frame]
   cases status : (Sequence.step fuel (contexts position) s command).failure with
   | some failure =>
     rw [drive_halted _ _ _ _ rfl]
     simp [observe,Sequence.run,status]
   | none =>
     rw [ih]
     simp [Sequence.run,status,List.append_assoc]

#print axioms schedule_good
#print axioms drive_sequence
end MirroreaProofFirst.ResourceComputations.Continuation

namespace MirroreaProofFirst.ResourceComputations.Sequence.Typing
open PureHandleFunctions ResourceBoundary ResourceContext

structure Shape where
 values : List Ty
 references : Nat
 deriving DecidableEq, Repr

def shape (frame : Sequence.Local n) : Shape := ⟨frame.types,frame.references.length⟩
def addValue (s : Shape) (t : Ty) : Shape := {s with values := t :: s.values}
def addReferences (s : Shape) (count : Nat) : Shape := {s with references := s.references+count}

-- Static reference names do not claim that the named resource remains usable.
-- Dynamic affine Delta is checked by the resource transition at actual use.
inductive CommandTyped : Shape → Instruction n → Shape → Prop where
 | pureLet : PureHandleFunctions.Typed s.values e t → CommandTyped s (.pureLet e) (addValue s t)
 | allocate : Typed s.values source → CommandTyped s (.allocate source) (addReferences s 1)
 | release : index < s.references → CommandTyped s (.release index) s
 | move : index < s.references → CommandTyped s (.move index receiver) (addReferences s 1)
 | split : index < s.references → CommandTyped s (.split index cut) (addReferences s 2)

def inferCommand (s : Shape) : Instruction n → Option Shape
 | .pureLet e => (infer s.values e).map (addValue s)
 | .allocate source => if check s.values source then some (addReferences s 1) else none
 | .release index => if index < s.references then some s else none
 | .move index _ => if index < s.references then some (addReferences s 1) else none
 | .split index _ => if index < s.references then some (addReferences s 2) else none

theorem command_exact (s : Shape) (command : Instruction n) (out : Shape) :
 inferCommand s command = some out ↔ CommandTyped s command out := by
 constructor
 · intro inferred
   cases command with
   | pureLet e =>
     cases found : infer s.values e with
     | none => simp [inferCommand,found] at inferred
     | some t =>
       simp [inferCommand,found] at inferred; subst out
       exact .pureLet ((infer_exact _ _ _).mp found)
   | allocate source =>
     simp only [inferCommand] at inferred
     split at inferred
     · rename_i checked; cases inferred; exact .allocate ((check_exact _ _).mp checked)
     · contradiction
   | release index =>
     simp only [inferCommand] at inferred; split at inferred
     · rename_i bound; cases inferred; exact .release bound
     · contradiction
   | move index receiver =>
     simp only [inferCommand] at inferred; split at inferred
     · rename_i bound; cases inferred; exact .move bound
     · contradiction
   | split index cut =>
     simp only [inferCommand] at inferred; split at inferred
     · rename_i bound; cases inferred; exact .split bound
     · contradiction
 · intro typed
   cases typed with
   | pureLet typed => simp [inferCommand,typed_infer typed]
   | allocate typed => simp [inferCommand,(check_exact _ _).mpr typed]
   | release bound => simp [inferCommand,bound]
   | move bound => simp [inferCommand,bound]
   | split bound => simp [inferCommand,bound]

inductive ProgramTyped : Shape → List (Instruction n) → Shape → Prop where
 | done : ProgramTyped s [] s
 | next : CommandTyped s command mid → ProgramTyped mid rest out → ProgramTyped s (command :: rest) out

def inferProgram : Shape → List (Instruction n) → Option Shape
 | s,[] => some s
 | s,command::rest => (inferCommand s command).bind fun mid => inferProgram mid rest

theorem program_exact (s : Shape) (commands : List (Instruction n)) (out : Shape) :
 inferProgram s commands = some out ↔ ProgramTyped s commands out := by
 constructor
 · intro inferred
   induction commands generalizing s with
   | nil => cases inferred; exact .done
   | cons command rest ih =>
     cases first : inferCommand s command with
     | none => simp [inferProgram,first] at inferred
     | some mid =>
       apply ProgramTyped.next ((command_exact _ _ _).mp first)
       exact ih mid (by simpa [inferProgram,first] using inferred)
 · intro typed
   induction typed with
   | done => rfl
   | next first rest ih => simp [inferProgram,(command_exact _ _ _).mpr first,ih]

-- A suspended frame carries its lexical input shape and its pending computation
-- judgment, independently of current authority. No resource permission is derived
-- from a successful static checker, or moved into a duplicable pure closure.
def FrameTyped (frame : Continuation.Frame n) (out : Shape) : Prop :=
 EnvTyped frame.lexical.values frame.lexical.types ∧ ProgramTyped (shape frame.lexical) frame.pending out

theorem capture_typed {c captured : Continuation.Configuration n} {identity : Nat} {out : Shape}
 (typed : FrameTyped c.phase.frame out)
 (success : Continuation.capture c = some (captured,identity)) : FrameTyped captured.phase.frame out := by
 rw [(Continuation.capture_frame success).2.1]
 exact typed

theorem resume_typed {c resumed : Continuation.Configuration n} {identity : Nat} {out : Shape}
 (typed : FrameTyped c.phase.frame out)
 (success : Continuation.resume identity c = some resumed) : FrameTyped resumed.phase.frame out := by
 rw [(Continuation.resume_frame success).2.1]
 exact typed

example : inferProgram ⟨[.int],0⟩ Controls.program =
 some ⟨[.arrow .int .int,.int],4⟩ := by decide
example : inferProgram ⟨[],0⟩ ([.release 0] : List (Instruction 4)) = none := by decide
example : inferProgram ⟨[.int],0⟩ ([.pureLet (.app (.var 0) (.integer 1))] : List (Instruction 4)) = none := by decide
-- A stale metadata alias may be well typed but must still fail dynamic ownership.
example : (inferProgram ⟨[.int],0⟩ ([.pureLet (.lambda .int (.add (.var 0) (.integer 1))),
 .allocate Controls.source,.split 0 1,.release 2] : List (Instruction 4))).isSome = true := by decide

#print axioms command_exact
#print axioms program_exact
#print axioms capture_typed
#print axioms resume_typed
end MirroreaProofFirst.ResourceComputations.Sequence.Typing

namespace MirroreaProofFirst.ResourceComputations.Sequence.Typing
open PureHandleFunctions ResourceBoundary ResourceContext
open ModuleContractBoundary.CurrentAllocation

theorem allocation_arity {fuel : Nat} {G : List Ty} {env : List (Value n)} {source : Source n}
 {current : RestoredCall.ExecutionContext n} {caller : CurrentUse.UseRequest n}
 {auth : CurrentUse.Evidence} {proof : ModuleContractBoundary.CallProof n}
 {state : Once.StateWithHistory} {delta : Delta} {out : Once.StateWithHistory × Delta × List Handle}
 (ok : allocateOnce fuel G env source current caller auth proof state delta = some out) : out.2.2.length = 1 := by
 obtain ⟨_,h,x,result,_,stepped,rfl⟩ := allocateOnce_selected ok
 obtain ⟨_,allocated,executed,rfl⟩ := (Once.step_exact _ _ _).mp stepped
 obtain ⟨_,_,_,_,_,_,_,_,_,_,rfl⟩ := (ModuleContractBoundary.CurrentAllocation.exact _ _ _ _ _ _ _ _ _ _ _ _ _).mp executed
 rfl

theorem resource_shape {ctx : Context n} {s : Store n} {a : Action}
 (ok : (resourceStep ctx s a).failure = none) :
 shape (resourceStep ctx s a).store.frame =
 addReferences (shape s.frame) (raw s.shared.resources a).2.length := by
 unfold resourceStep at ok ⊢
 cases executed : ResourceContext.execute ctx.resourcePolicy s.shared.resources s.frame.rights a with
 | none => simp [executed,unchanged] at ok
 | some out =>
   obtain ⟨_,_,rfl⟩ := (ResourceContext.execute_exact _ _ _ _ _).mp executed
   simp [shape,addReferences,List.length_append,Nat.add_comm]

theorem named_shape {ctx : Context n} {s : Store n} {index count : Nat} {make : Handle → Action}
 (arity : ∀ h, (raw s.shared.resources (make h)).2.length = count)
 (ok : (namedResource ctx s index make).failure = none) :
 shape (namedResource ctx s index make).store.frame = addReferences (shape s.frame) count := by
 unfold namedResource at ok ⊢
 cases found : s.frame.references[index]? with
 | none => simp [found,unchanged] at ok
 | some h =>
   have result := resource_shape (ctx := ctx) (s := s) (a := make h) (by simpa [found] using ok)
   rw [arity h] at result
   exact result

theorem step_shape {fuel : Nat} {ctx : Context n} {s : Store n} {command : Instruction n} {out : Shape}
 (typed : CommandTyped (shape s.frame) command out) (ok : (Sequence.step fuel ctx s command).failure = none) :
 shape (Sequence.step fuel ctx s command).store.frame = out := by
 cases typed with
 | pureLet typed =>
   have inferred := typed_infer typed
   simp only [shape] at inferred
   simp only [Sequence.step,inferred] at ok ⊢
   cases evaluated : evaluate fuel s.frame.values _ with
   | none => simp [evaluated,unchanged] at ok
   | some value => rfl
 | allocate typed =>
   have checked := (check_exact _ _).mpr typed
   simp only [shape] at checked
   simp only [Sequence.step,checked,ite_true] at ok ⊢
   split at ok
   · simp [unchanged] at ok
   · rename_i pair selected
     cases allocated : allocateOnce fuel s.frame.types s.frame.values _ ctx.current ctx.caller ctx.evidence ctx.proof s.shared s.frame.rights with
     | none => simp [allocated,unchanged] at ok
     | some result =>
       simp [shape,addReferences,List.length_append,allocation_arity allocated,Nat.add_comm]
 | release bound =>
   have result := named_shape (fun h => (show (raw s.shared.resources (.release ctx.caller.principal h)).2.length = 0 from rfl)) ok
   simpa [addReferences] using result
 | move bound => exact named_shape (fun h => rfl) ok
 | split bound => exact named_shape (fun h => rfl) ok

theorem program_shape {fuel : Nat} {contexts : Nat → Context n} {position : Nat} {s : Store n}
 {commands : List (Instruction n)} {out : Shape}
 (typed : ProgramTyped (shape s.frame) commands out)
 (ok : (Sequence.run fuel contexts position s commands).failure = none) :
 shape (Sequence.run fuel contexts position s commands).store.frame = out := by
 induction commands generalizing position s with
 | nil => cases typed; rfl
 | cons command rest ih =>
   cases typed with
   | next first tail =>
     cases status : (Sequence.step fuel (contexts position) s command).failure with
     | some failure => simp [Sequence.run,status] at ok
     | none =>
       have changed := step_shape first status
       have nextTyped := changed ▸ tail
       have nextOk : (Sequence.run fuel contexts (position+1) (Sequence.step fuel (contexts position) s command).store rest).failure = none := by simpa [Sequence.run,status] using ok
       simpa [Sequence.run,status] using ih nextTyped nextOk

#print axioms allocation_arity
#print axioms step_shape
#print axioms program_shape
end MirroreaProofFirst.ResourceComputations.Sequence.Typing

namespace MirroreaProofFirst.ResourceComputations.Sequence.Typing
open PureHandleFunctions ResourceBoundary ResourceContext
open Continuation

theorem tick_typed {c : Configuration n} {out : Shape}
 (good : Continuation.Good c) (typed : FrameTyped c.phase.frame out)
 (fuel : Nat) (ctx : Context n) : FrameTyped (tick fuel ctx c).phase.frame out := by
 unfold tick
 split
 · rename_i frame active
   have before : Sequence.Good (⟨c.shared,frame.lexical⟩ : Store n) := by
     simpa [Continuation.Good,Continuation.store,active,Phase.frame] using good
   have frameTyped : FrameTyped frame out := by simpa [active,Phase.frame] using typed
   cases pending : frame.pending with
   | nil => simpa [pending,Phase.frame] using frameTyped
   | cons command rest =>
     have program : ProgramTyped (shape frame.lexical) (command::rest) out := by simpa [pending] using frameTyped.2
     cases program with
     | next first tail =>
       have after := Sequence.step_good (ctx := ctx) before fuel command
       cases status : (Sequence.step fuel ctx ⟨c.shared,frame.lexical⟩ command).failure with
       | none =>
         have shapeChanged := step_shape first status
         have nextTyped := shapeChanged ▸ tail
         simpa [pending,status,Phase.frame,FrameTyped] using And.intro after.environment nextTyped
       | some failure =>
         have same := (Sequence.step_failure status).1
         simpa [pending,status,Phase.frame,FrameTyped,same] using frameTyped
 · exact typed

theorem retry_typed {c : Configuration n} {out : Shape} (typed : FrameTyped c.phase.frame out) :
 FrameTyped (retryBudget c).phase.frame out := by rw [(retry_frame c).2.1]; exact typed

theorem act_typed {c : Configuration n} {out : Shape}
 (good : Continuation.Good c) (typed : FrameTyped c.phase.frame out) (action : Control n) :
 FrameTyped (act c action).phase.frame out := by
 cases action with
 | tick fuel ctx => exact tick_typed good typed _ _
 | retryBudget => exact retry_typed typed
 | capture =>
   cases captured : capture c with
   | none => simpa [act,captured] using typed
   | some result => simpa [act,captured] using capture_typed typed captured
 | resume token =>
   cases resumed : resume token c with
   | none => simpa [act,resumed] using typed
   | some result => simpa [act,resumed] using resume_typed typed resumed

theorem schedule_typed {c : Configuration n} {out : Shape}
 (good : Continuation.Good c) (typed : FrameTyped c.phase.frame out) (actions : List (Control n)) :
 Continuation.Good (Continuation.schedule c actions) ∧ FrameTyped (Continuation.schedule c actions).phase.frame out := by
 induction actions generalizing c with
 | nil => exact ⟨good,typed⟩
 | cons action rest ih => exact ih (act_good good action) (act_typed good typed action)

#print axioms tick_typed
#print axioms schedule_typed
end MirroreaProofFirst.ResourceComputations.Sequence.Typing

namespace MirroreaProofFirst.ResourceComputations.Sequence
open PureHandleFunctions ResourceBoundary ResourceContext
open ModuleContractBoundary.CurrentAllocation Semantics

def Evolves (before after : Once.StateWithHistory) : Prop :=
 (after.resources = before.resources ∨ ∃ a, after.resources = (raw before.resources a).1) ∧
 before.committed ⊆ after.committed

theorem evolves_refl (s : Once.StateWithHistory) : Evolves s s := ⟨Or.inl rfl,fun _ h => h⟩
theorem resource_evolves {ctx : Context n} {s : Store n} {a : Action} {out : Result n}
 (semantic : ResourceTransition ctx s a out) : Evolves s.shared out.store.shared := by
 cases semantic with
 | accepted _ _ => exact ⟨Or.inr ⟨a,rfl⟩,fun _ h => h⟩
 | denied _ => exact evolves_refl _
theorem named_evolves {ctx : Context n} {s : Store n} {index : Nat} {make : Handle → Action} {out : Result n}
 (semantic : NamedTransition ctx s index make out) : Evolves s.shared out.store.shared := by
 cases semantic with
 | missing _ => exact evolves_refl _
 | found _ transition => exact resource_evolves transition

theorem transition_evolves {ctx : Context n} {s : Store n} {command : Instruction n} {out : Result n}
 (semantic : Transition ctx s command out) : Evolves s.shared out.store.shared := by
 cases semantic with
 | pureRejected _ => exact evolves_refl _
 | pureValue _ _ => exact evolves_refl _
 | callRejected _ => exact evolves_refl _
 | callDenied _ _ _ => exact evolves_refl _
 | callValue _ _ admitted =>
   obtain ⟨_,d,value,_,_,_,_,_,_,_,_,rfl⟩ := admitted
   exact ⟨Or.inr ⟨_,rfl⟩,fun _ h => List.mem_cons_of_mem _ h⟩
 | release semantic | move semantic | split semantic => exact named_evolves semantic

theorem step_evolves (fuel : Nat) (ctx : Context n) (s : Store n) (command : Instruction n) :
 Evolves s.shared (step fuel ctx s command).store.shared := by
 by_cases budget : (step fuel ctx s command).failure = some .referenceBudget
 · rw [(step_failure budget).1]; exact evolves_refl _
 · exact transition_evolves (completed_step_sound budget)

def RetiredResource (identity : Nat) (s : ResourceBoundary.State) : Prop :=
 identity < s.nextId ∧ s.live identity = none

theorem evolution_retired {before after : Once.StateWithHistory} {identity : Nat}
 (evolved : Evolves before after) (retired : RetiredResource identity before.resources) :
 RetiredResource identity after.resources := by
 rcases evolved.1 with same | ⟨a,changed⟩
 · simpa [same] using retired
 · rw [changed]
   exact ⟨Nat.lt_of_lt_of_le retired.1 (raw_monotone _ _).1,raw_old_absent _ _ _ retired.1 retired.2⟩

theorem run_retired {identity : Nat} {s : Store n} (retired : RetiredResource identity s.shared.resources)
 (fuel : Nat) (contexts : Nat → Context n) (position : Nat) (commands : List (Instruction n)) :
 RetiredResource identity (run fuel contexts position s commands).store.shared.resources := by
 induction commands generalizing position s with
 | nil => exact retired
 | cons command rest ih =>
   have kept := evolution_retired (step_evolves fuel (contexts position) s command) retired
   simp only [run]; split
   · exact kept
   · exact ih kept _

theorem run_history (fuel : Nat) (contexts : Nat → Context n) (position : Nat)
 (s : Store n) (commands : List (Instruction n)) :
 s.shared.committed ⊆ (run fuel contexts position s commands).store.shared.committed := by
 induction commands generalizing position s with
 | nil => exact fun _ h => h
 | cons command rest ih =>
   have kept := (step_evolves fuel (contexts position) s command).2
   simp only [run]; split
   · exact kept
   · exact List.Subset.trans kept (ih _ _)

-- These range over mixed pure/allocation/split/move/release programs, not just
-- the old resource-only or invocation-only schedules.
theorem retired_not_owned {identity : Nat} {s : Store n} (good : Good s)
 (retired : RetiredResource identity s.shared.resources) : s.frame.rights identity = none := by
 cases present : s.frame.rights identity with
 | none => rfl
 | some region =>
   have live := good.realized _ _ present
   rw [retired.2] at live; contradiction

theorem committed_call_rejected {fuel : Nat} {G : List Ty} {env : List (Value n)} {source : Source n}
 {current : RestoredCall.ExecutionContext n} {caller : CurrentUse.UseRequest n}
 {auth : CurrentUse.Evidence} {proof : ModuleContractBoundary.CallProof n}
 {s : Store n} (contexts : Nat → Context n) (position : Nat) (later : List (Instruction n))
 (already : ∀ h x, (invocation current caller auth proof h x).key ∈ s.shared.committed) :
 allocateOnce fuel G env source current caller auth proof
 (run fuel contexts position s later).store.shared (run fuel contexts position s later).store.frame.rights = none := by
 unfold allocateOnce
 split
 · cases selected : select fuel env source with
   | none => rfl
   | some pair =>
     have present := run_history fuel contexts position s later (already pair.1 pair.2)
     simp [Once.step,present]
 · rfl

#print axioms step_evolves
#print axioms run_retired
#print axioms run_history
#print axioms retired_not_owned
#print axioms committed_call_rejected
end MirroreaProofFirst.ResourceComputations.Sequence

namespace MirroreaProofFirst.ResourceComputations.Sequence.Typing
open PureHandleFunctions ResourceBoundary ResourceContext Semantics
open ModuleContractBoundary.CurrentAllocation

theorem typed_selection_exists {env : List (Value n)} {G : List Ty} {source : Source n}
 (environment : EnvTyped env G) (typed : Typed G source) : ∃ h x, Selects env source h x := by
 cases typed with
 | call ht xt =>
   obtain ⟨hv,hf,hh,he⟩ := Normalization.typed_environment_total environment ht
   obtain ⟨xv,xf,hx,xe⟩ := Normalization.typed_environment_total environment xt
   cases hh with
   | handle =>
     cases hx with
     | integer =>
       exact ⟨_,_,.selected ((execution_exact _ _).mp ⟨hf,he hf (Nat.le_refl _)⟩)
         ((execution_exact _ _).mp ⟨xf,xe xf (Nat.le_refl _)⟩)⟩

theorem typed_transition_exists {ctx : Context n} {s : Store n} {command : Instruction n} {outShape : Shape}
 (environment : EnvTyped s.frame.values s.frame.types) (typed : CommandTyped (shape s.frame) command outShape) :
 ∃ out, Transition ctx s command out := by
 cases typed with
 | pureLet typed =>
   obtain ⟨v,fuel,_,enough⟩ := Normalization.typed_environment_total environment typed
   exact ⟨_,.pureValue typed ((execution_exact _ _).mp ⟨fuel,enough fuel (Nat.le_refl _)⟩)⟩
 | allocate typed =>
   obtain ⟨h,x,selected⟩ := typed_selection_exists environment typed
   cases executed : Once.step (invocation ctx.current ctx.caller ctx.evidence ctx.proof h x) s.shared with
   | none =>
     refine ⟨_,.callDenied typed selected ?_⟩
     intro out accepted
     have yes := (admission_exact _ _ _).mpr accepted
     rw [executed] at yes; contradiction
   | some out => exact ⟨_,.callValue typed selected ((admission_exact _ _ _).mp executed)⟩
 | release _ => exact ⟨_,.release ((named_transition_exact _ _ _ _ _).mp rfl)⟩
 | move _ => exact ⟨_,.move ((named_transition_exact _ _ _ _ _).mp rfl)⟩
 | split _ => exact ⟨_,.split ((named_transition_exact _ _ _ _ _).mp rfl)⟩

-- An independently well-typed finite program cannot remain permanently stuck
-- behind reference fuel. Authorization/resource denial is still a valid result;
-- neither type validity nor this theorem promises an authorized successful effect.
theorem typed_run_exists {contexts : Nat → Context n} {position : Nat} {s : Store n}
 {commands : List (Instruction n)} {outShape : Shape}
 (good : Good s) (typed : ProgramTyped (shape s.frame) commands outShape) :
 ∃ out, Runs contexts position s commands out := by
 induction commands generalizing position s with
 | nil => exact ⟨_,.done⟩
 | cons command rest ih =>
   cases typed with
   | next first tail =>
     obtain ⟨result,transition⟩ := typed_transition_exists (ctx := contexts position) good.environment first
     cases status : result.failure with
     | some failure => exact ⟨_,.failed transition status⟩
     | none =>
       obtain ⟨minimum,enough⟩ := transition_eventually transition
       have executed := enough minimum (Nat.le_refl _)
       have after : Good result.store := by simpa [executed] using step_good (ctx := contexts position) good minimum command
       have changed := step_shape (fuel := minimum) (ctx := contexts position) first (by simpa [executed] using status)
       rw [executed] at changed
       have nextTyped := changed ▸ tail
       obtain ⟨out,later⟩ := ih after nextTyped (position := position+1)
       exact ⟨_,.next transition status later⟩

theorem checked_program_eventually {contexts : Nat → Context n} {position : Nat} {s : Store n}
 {commands : List (Instruction n)} {outShape : Shape}
 (good : Good s) (checked : inferProgram (shape s.frame) commands = some outShape) :
 ∃ out minimum, Runs contexts position s commands out ∧
   ∀ fuel, minimum ≤ fuel → Sequence.run fuel contexts position s commands = out := by
 obtain ⟨out,semantic⟩ := typed_run_exists good ((program_exact _ _ _).mp checked)
 obtain ⟨minimum,enough⟩ := runs_eventually semantic
 exact ⟨out,minimum,semantic,enough⟩

#print axioms typed_selection_exists
#print axioms typed_transition_exists
#print axioms typed_run_exists
#print axioms checked_program_eventually
end MirroreaProofFirst.ResourceComputations.Sequence.Typing

namespace MirroreaProofFirst.ResourceComputations.Sequence.Rows
open PureHandleFunctions ResourceBoundary Semantics
inductive Effect where
 | allocate | release | move | split
 deriving DecidableEq, Repr

def instructionEffect : Instruction n → List Effect
 | .pureLet _ => []
 | .allocate _ => [.allocate]
 | .release _ => [.release]
 | .move _ _ => [.move]
 | .split _ _ => [.split]
def effectRow (commands : List (Instruction n)) : List Effect := commands.flatMap instructionEffect

def eventEffect : Event → Effect
 | .allocation _ => .allocate
 | .resource (.allocate _ _) _ => .allocate
 | .resource (.release _ _) _ => .release
 | .resource (.move _ _ _) _ => .move
 | .resource (.split _ _ _) _ => .split

theorem transition_effects {ctx : Context n} {s : Store n} {command : Instruction n} {out : Result n}
 (transition : Transition ctx s command out) :
 out.events.map eventEffect = if out.failure.isNone then instructionEffect command else [] := by
 cases transition with
 | pureRejected _ | pureValue _ _ | callRejected _ | callDenied _ _ _ | callValue _ _ _ => rfl
 | release h | move h | split h =>
   cases h with
   | missing _ => rfl
   | found _ h => cases h <;> rfl

theorem step_effects (fuel : Nat) (ctx : Context n) (s : Store n) (command : Instruction n) :
 (step fuel ctx s command).events.map eventEffect =
 if (step fuel ctx s command).failure.isNone then instructionEffect command else [] := by
 by_cases budget : (step fuel ctx s command).failure = some .referenceBudget
 · rw [(step_failure budget).2,budget]; rfl
 · exact transition_effects (completed_step_sound budget)

-- The actual successful effects are precisely a prefix of the declared row;
-- completion consumes the whole row. Failed attempts do not invent an effect.
theorem run_effect_prefix (fuel : Nat) (contexts : Nat → Context n) (position : Nat)
 (s : Store n) (commands : List (Instruction n)) :
 ∃ remaining, (run fuel contexts position s commands).events.map eventEffect ++ remaining = effectRow commands ∧
 ((run fuel contexts position s commands).failure = none → remaining = []) := by
 induction commands generalizing position s with
 | nil => exact ⟨[],rfl,fun _ => rfl⟩
 | cons command rest ih =>
   cases status : (step fuel (contexts position) s command).failure with
   | some failure =>
     refine ⟨effectRow (command::rest),?_,?_⟩
     · simp [run,status,(step_failure status).2]
     · simp [run,status]
   | none =>
     obtain ⟨remaining,events,finished⟩ := ih (position+1) (step fuel (contexts position) s command).store
     have first := step_effects fuel (contexts position) s command
     simp only [status,Option.isNone_none,ite_true] at first
     refine ⟨remaining,?_,?_⟩
     · simp only [run,status,List.map_append,effectRow,List.flatMap_cons]
       rw [List.append_assoc,events,first]
       rfl
     · simpa [run,status] using finished

-- Residual obligations are the actual suspended instructions, including their
-- source expressions. They are not a second hand-authored service/ID table.
def residualObligations (frame : Continuation.Frame n) : List (Instruction n) := frame.pending

theorem capture_rows {c out : Continuation.Configuration n} {id : Nat}
 (captured : Continuation.capture c = some (out,id)) :
 effectRow (residualObligations out.phase.frame) = effectRow (residualObligations c.phase.frame) := by
 rw [(Continuation.capture_frame captured).2.1]

theorem resume_rows {c out : Continuation.Configuration n} {id : Nat}
 (resumed : Continuation.resume id c = some out) :
 residualObligations out.phase.frame = residualObligations c.phase.frame := by
 rw [(Continuation.resume_frame resumed).2.1]

#print axioms step_effects
#print axioms run_effect_prefix
#print axioms capture_rows
#print axioms resume_rows
end MirroreaProofFirst.ResourceComputations.Sequence.Rows

namespace MirroreaProofFirst.ResourceComputations.Sequence.Rows
open PureHandleFunctions ResourceBoundary ResourceContext Typing

def instructionFailures : Instruction n → List Failure
 | .pureLet _ => []
 | _ => [.boundaryDenied]
def failureRow (commands : List (Instruction n)) : List Failure := commands.flatMap instructionFailures

theorem named_typed_failure {ctx : Context n} {s : Store n} {index : Nat}
 {make : Handle → Action} {failure : Failure}
 (bound : index < s.frame.references.length)
 (failed : (namedResource ctx s index make).failure = some failure) : failure = .boundaryDenied := by
 unfold namedResource at failed
 cases found : s.frame.references[index]? with
 | none =>
   have outside := List.getElem?_eq_none_iff.mp found
   omega
 | some h =>
   simp only [found] at failed
   unfold resourceStep at failed
   split at failed
   · simpa [unchanged] using failed.symm
   · contradiction

theorem command_failure_row {fuel : Nat} {ctx : Context n} {s : Store n}
 {command : Instruction n} {outShape : Shape} {failure : Failure}
 (typed : CommandTyped (shape s.frame) command outShape)
 (failed : (step fuel ctx s command).failure = some failure)
 (semantic : failure ≠ .referenceBudget) : failure ∈ instructionFailures command := by
 cases typed with
 | pureLet typed =>
   have inferred := typed_infer typed
   simp only [shape] at inferred
   simp only [step,inferred] at failed
   split at failed
   · simp [unchanged] at failed; exact False.elim (semantic failed.symm)
   · contradiction
 | allocate typed =>
   have checked := (check_exact _ _).mpr typed
   simp only [shape] at checked
   simp only [step,checked,ite_true] at failed
   split at failed
   · simp [unchanged] at failed; exact False.elim (semantic failed.symm)
   · split at failed
     · simpa [instructionFailures,unchanged] using failed.symm
     · contradiction
 | release bound => simpa [instructionFailures] using named_typed_failure bound failed
 | move bound => simpa [instructionFailures] using named_typed_failure bound failed
 | split bound => simpa [instructionFailures] using named_typed_failure bound failed

theorem program_failure_row {fuel : Nat} {contexts : Nat → Context n} {position : Nat} {s : Store n}
 {commands : List (Instruction n)} {outShape : Shape} {failure : Failure}
 (typed : ProgramTyped (shape s.frame) commands outShape)
 (failed : (run fuel contexts position s commands).failure = some failure)
 (semantic : failure ≠ .referenceBudget) : failure ∈ failureRow commands := by
 induction commands generalizing position s with
 | nil => contradiction
 | cons command rest ih =>
   cases typed with
   | next first tail =>
     cases status : (step fuel (contexts position) s command).failure with
     | some firstFailure =>
       have same : firstFailure = failure := by simpa [run,status] using failed
       subst firstFailure
       have member := command_failure_row first status semantic
       exact List.mem_append.mpr (Or.inl member)
     | none =>
       have changed := step_shape first status
       have nextTyped := changed ▸ tail
       have nextFailed : (run fuel contexts (position+1) (step fuel (contexts position) s command).store rest).failure = some failure := by
         simpa [run,status] using failed
       exact List.mem_append.mpr (Or.inr (ih nextTyped nextFailed))

-- The ordinary successful sample has actual effects and a checked failure row.
-- This counter-control prevents declaring [] for a resource computation that
-- can be denied; reference-budget status is deliberately not in this row.
example : failureRow (Controls.program : List (Instruction 4)) =
 [.boundaryDenied,.boundaryDenied,.boundaryDenied,.boundaryDenied,.boundaryDenied] := by rfl
example : instructionFailures (Instruction.pureLet (n := 4) (.integer 3)) = [] := by rfl
#print axioms command_failure_row
#print axioms program_failure_row
end MirroreaProofFirst.ResourceComputations.Sequence.Rows

namespace MirroreaProofFirst.ResourceComputations
-- Independently stated field equations prevent an implementation of request
-- from silently ignoring the source-selected interface or computed argument.
theorem request_fields (caller : CurrentUse.UseRequest n) (h : HandleValues.Interface n) (x : Int) :
 (request caller h x).moduleHandle = h.moduleHandle ∧
 (request caller h x).operation = h.operation ∧
 (request caller h x).arguments = [.integer x] ∧
 (request caller h x).principal = caller.principal ∧
 (request caller h x).member = caller.member ∧
 (request caller h x).locus = caller.locus ∧
 (request caller h x).request = caller.request := ⟨rfl,rfl,rfl,rfl,rfl,rfl,rfl⟩
#print axioms request_fields
end MirroreaProofFirst.ResourceComputations

-- Derived scheduler integrity, added after the frozen Oracle definition cut.
namespace MirroreaProofFirst.ResourceComputations.Continuation
open Sequence
open ModuleContractBoundary.CurrentAllocation

theorem tick_evolves (fuel : Nat) (ctx : Context n) (c : Configuration n) :
 Sequence.Evolves c.shared (tick fuel ctx c).shared := by
 unfold tick
 split
 · rename_i frame active
   split
   · exact Sequence.evolves_refl _
   · exact Sequence.step_evolves fuel ctx ⟨c.shared,frame.lexical⟩ _
 · exact Sequence.evolves_refl _

theorem act_evolves (c : Configuration n) (control : Control n) :
 Sequence.Evolves c.shared (act c control).shared := by
 cases control with
 | tick fuel ctx => exact tick_evolves fuel ctx c
 | retryBudget => rw [show (act c .retryBudget).shared = c.shared from (retry_frame c).1]; exact Sequence.evolves_refl _
 | capture =>
   cases captured : capture c with
   | none => simpa [act,captured] using Sequence.evolves_refl c.shared
   | some out =>
     have same := (capture_frame captured).1
     simpa [act,captured,same] using Sequence.evolves_refl c.shared
 | resume token =>
   cases resumed : resume token c with
   | none => simpa [act,resumed] using Sequence.evolves_refl c.shared
   | some out =>
     have same := (resume_frame resumed).1
     simpa [act,resumed,same] using Sequence.evolves_refl c.shared

theorem schedule_history (c : Configuration n) (controls : List (Control n)) :
 c.shared.committed ⊆ (schedule c controls).shared.committed := by
 induction controls generalizing c with
 | nil => exact fun _ h => h
 | cons first rest ih => exact List.Subset.trans (act_evolves c first).2 (ih _)

theorem schedule_resource_retired {identity : Nat} {c : Configuration n}
 (retired : Sequence.RetiredResource identity c.shared.resources) (controls : List (Control n)) :
 Sequence.RetiredResource identity (schedule c controls).shared.resources := by
 induction controls generalizing c with
 | nil => exact retired
 | cons first rest ih => exact ih (Sequence.evolution_retired (act_evolves c first) retired)

def RightsTrace (initial : ResourceContext.Delta) (c : Configuration n) : Prop :=
 c.phase.frame.lexical.rights = Sequence.replayRights initial c.events

theorem tick_rights_trace {initial : ResourceContext.Delta} {c : Configuration n}
 (valid : RightsTrace initial c) (fuel : Nat) (ctx : Context n) :
 RightsTrace initial (tick fuel ctx c) := by
 unfold tick
 split
 · rename_i frame active
   have before : frame.lexical.rights = Sequence.replayRights initial c.events := by
     simpa [RightsTrace,active,Phase.frame] using valid
   split
   · simpa [RightsTrace,Phase.frame] using before
   · rename_i command rest pending
     have after := Sequence.step_rights fuel ctx ⟨c.shared,frame.lexical⟩ command
     cases status : (Sequence.step fuel ctx ⟨c.shared,frame.lexical⟩ command).failure <;>
       simp only [RightsTrace,status,Phase.frame]
     all_goals
       simp only [Sequence.replayRights,List.foldl_append] at before after ⊢
       rw [← before]
       exact after
 · exact valid

theorem act_rights_trace {initial : ResourceContext.Delta} {c : Configuration n}
 (valid : RightsTrace initial c) (control : Control n) : RightsTrace initial (act c control) := by
 cases control with
 | tick fuel ctx => exact tick_rights_trace valid fuel ctx
 | retryBudget => simpa [RightsTrace,act,(retry_frame c).2.1,(retry_frame c).2.2.1] using valid
 | capture =>
   cases captured : capture c with
   | none => simpa [act,captured] using valid
   | some out =>
     obtain ⟨_,frame,events,_,_⟩ := capture_frame captured
     simpa [act,captured,RightsTrace,frame,events] using valid
 | resume token =>
   cases resumed : resume token c with
   | none => simpa [act,resumed] using valid
   | some out =>
     obtain ⟨_,frame,events,_⟩ := resume_frame resumed
     simpa [act,resumed,RightsTrace,frame,events] using valid

theorem schedule_rights_trace {initial : ResourceContext.Delta} {c : Configuration n}
 (valid : RightsTrace initial c) (controls : List (Control n)) : RightsTrace initial (schedule c controls) := by
 induction controls generalizing c with
 | nil => exact valid
 | cons first rest ih => exact ih (act_rights_trace valid first)

theorem from_start_rights_trace (s : Sequence.Store n) (body : List (Sequence.Instruction n))
 (controls : List (Control n)) : RightsTrace s.frame.rights (schedule (start s body) controls) :=
 schedule_rights_trace (initial := s.frame.rights) (c := start s body) (by rfl) controls

#print axioms from_start_rights_trace
#print axioms schedule_history
#print axioms schedule_resource_retired
#print axioms schedule_rights_trace
end MirroreaProofFirst.ResourceComputations.Continuation

namespace MirroreaProofFirst.ResourceComputations.ResourceContext
open ResourceBoundary

theorem extend_untouched (delta : Delta) (returned : List Handle) (identity : Nat)
 (fresh : ∀ h ∈ returned, identity ≠ h.id) : extend delta returned identity = delta identity := by
 induction returned generalizing delta with
 | nil => rfl
 | cons h tail ih =>
   rw [extend,ih (add delta h) (fun h member => fresh h (List.mem_cons_of_mem _ member))]
   simp [add,fresh h (by simp)]

theorem update_untouched (delta : Delta) (action : Action) (returned : List Handle) (identity : Nat)
 (notConsumed : ∀ h, consumed action = some h → identity ≠ h.id)
 (notReturned : ∀ h ∈ returned, identity ≠ h.id) :
 update delta action returned identity = delta identity := by
 rw [update,extend_untouched _ _ _ notReturned]
 cases selected : consumed action with
 | none => simp [consume,selected]
 | some h => simp [consume,selected,erase,notConsumed h selected]

-- Pointwise postconditions do not contain update/replay in their conclusions.
theorem execute_untouched {policy : Nat → Bool} {s : State} {delta : Delta} {action : Action}
 {out : State × Delta × List Handle} (executed : execute policy s delta action = some out)
 (identity : Nat) (notConsumed : ∀ h, consumed action = some h → identity ≠ h.id)
 (notReturned : ∀ h ∈ out.2.2, identity ≠ h.id) : out.2.1 identity = delta identity := by
 obtain ⟨_,_,rfl⟩ := (execute_exact _ _ _ _ _).mp executed
 exact update_untouched _ _ _ _ notConsumed notReturned

theorem execute_returned {policy : Nat → Bool} {s : State} {delta : Delta} {action : Action}
 {out : State × Delta × List Handle} (executed : execute policy s delta action = some out) :
 ∀ h ∈ out.2.2, out.2.1 h.id = some h.region := by
 obtain ⟨_,_,rfl⟩ := (execute_exact _ _ _ _ _).mp executed
 exact updated_returned _ _ _

theorem execute_consumed {policy : Nat → Bool} {s : State} {delta : Delta} {action : Action}
 {out : State × Delta × List Handle} (wf : WF s) (realized : Realized s delta)
 (executed : execute policy s delta action = some out) {h : Handle}
 (consumedHere : consumed action = some h) : out.2.1 h.id = none := by
 have preserved := execute_preserves wf realized executed
 obtain ⟨_,allowed,shape⟩ := (execute_exact _ _ _ _ _).mp executed
 have absent := ResourceBoundary.consumed_absent wf allowed consumedHere
 cases present : out.2.1 h.id with
 | none => rfl
 | some r =>
   have live := preserved.2 _ _ present
   rw [shape] at live
   rw [absent.1] at live
   contradiction

#print axioms execute_untouched
#print axioms execute_returned
#print axioms execute_consumed
end MirroreaProofFirst.ResourceComputations.ResourceContext

namespace MirroreaProofFirst.ResourceComputations.Continuation.Prefix
open Sequence Sequence.Semantics

-- Actual progressing ticks, each with its own supplied context. No evaluator,
-- capture/resume control or fuel status occurs in this independent relation.
inductive Executed (initial : Store n) : List (Instruction n) → Store n → List Event → Prop where
 | empty : Executed initial [] initial []
 | next {ctx : Context n} : Executed initial done before events →
     Transition ctx before command result → result.failure = none →
     Executed initial (done ++ [command]) result.store (events ++ result.events)

def Realizes (initial : Store n) (program : List (Instruction n)) (c : Configuration n) : Prop :=
 ∃ done, program = done ++ c.phase.frame.pending ∧ c.phase.frame.position = done.length ∧
 Executed initial done (Continuation.store c) c.events

theorem start_realizes (initial : Store n) (program : List (Instruction n)) :
 Realizes initial program (start initial program) := ⟨[],rfl,rfl,.empty⟩

theorem tick_realizes {initial : Store n} {program : List (Instruction n)} {c : Configuration n}
 (valid : Realizes initial program c) (fuel : Nat) (ctx : Context n) :
 Realizes initial program (tick fuel ctx c) := by
 unfold tick
 split
 · rename_i frame active
   obtain ⟨done,body,position,executed⟩ := valid
   simp only [active,Phase.frame,Continuation.store] at body position executed
   cases pending : frame.pending with
   | nil =>
     exact ⟨done,by simpa [pending,Phase.frame] using body,position,executed⟩
   | cons command rest =>
     cases status : (step fuel ctx ⟨c.shared,frame.lexical⟩ command).failure with
     | none =>
       have semantic := completed_step_sound (show (step fuel ctx ⟨c.shared,frame.lexical⟩ command).failure ≠ some .referenceBudget by simp [status])
       refine ⟨done ++ [command],?_,?_,?_⟩
       · simpa [pending,status,Phase.frame,List.append_assoc] using body
       · simp [status,Phase.frame,position,List.length_append]
       · simpa [status,Phase.frame,Continuation.store] using Executed.next executed semantic status
     | some failure =>
       have unchanged := step_failure status
       refine ⟨done,?_,?_,?_⟩
       · simpa [pending,status,Phase.frame] using body
       · simpa [status,Phase.frame] using position
       · simpa [status,Phase.frame,Continuation.store,unchanged.1,unchanged.2] using executed
 · exact valid

theorem act_realizes {initial : Store n} {program : List (Instruction n)} {c : Configuration n}
 (valid : Realizes initial program c) (control : Control n) : Realizes initial program (act c control) := by
 cases control with
 | tick fuel ctx => exact tick_realizes valid fuel ctx
 | retryBudget =>
   obtain ⟨_,frame,events,_⟩ := retry_frame c
   simpa [Realizes,Continuation.store,act,frame,events,(retry_frame c).1] using valid
 | capture =>
   cases captured : capture c with
   | none => simpa [act,captured] using valid
   | some out =>
     obtain ⟨shared,frame,events,_,_⟩ := capture_frame captured
     simpa [Realizes,Continuation.store,act,captured,shared,frame,events] using valid
 | resume token =>
   cases resumed : resume token c with
   | none => simpa [act,resumed] using valid
   | some out =>
     obtain ⟨shared,frame,events,_⟩ := resume_frame resumed
     simpa [Realizes,Continuation.store,act,resumed,shared,frame,events] using valid

theorem schedule_realizes {initial : Store n} {program : List (Instruction n)} {c : Configuration n}
 (valid : Realizes initial program c) (controls : List (Control n)) :
 Realizes initial program (schedule c controls) := by
 induction controls generalizing c with
 | nil => exact valid
 | cons first rest ih => exact ih (act_realizes valid first)

theorem from_start (initial : Store n) (program : List (Instruction n)) (controls : List (Control n)) :
 Realizes initial program (schedule (start initial program) controls) :=
 schedule_realizes (start_realizes initial program) controls

theorem executed_effects {initial : Store n} {done : List (Instruction n)} {out : Store n} {events : List Event}
 (executed : Executed initial done out events) : events.map Rows.eventEffect = Rows.effectRow done := by
 induction executed with
 | empty => rfl
 | next previous transition success ih =>
   have effects := Rows.transition_effects transition
   simp only [success,Option.isNone_none,ite_true] at effects
   simp [List.map_append,ih,effects,Rows.effectRow,List.flatMap_append]

theorem prefix_shape {initial : Store n} {program : List (Instruction n)} {c : Configuration n}
 (valid : Realizes initial program c) :
 c.phase.frame.position ≤ program.length ∧
 c.phase.frame.pending = program.drop c.phase.frame.position ∧
 c.events.map Rows.eventEffect = Rows.effectRow (program.take c.phase.frame.position) := by
 obtain ⟨done,body,position,executed⟩ := valid
 have effects := executed_effects executed
 simp [body,position,effects]

theorem from_start_prefix (initial : Store n) (program : List (Instruction n)) (controls : List (Control n)) :
 let out := schedule (start initial program) controls
 out.phase.frame.position ≤ program.length ∧
 out.phase.frame.pending = program.drop out.phase.frame.position ∧
 out.events.map Rows.eventEffect = Rows.effectRow (program.take out.phase.frame.position) :=
 prefix_shape (from_start initial program controls)

-- A terminal semantic denial is justified by a separate Transition at the
-- actual denying tick. Budget pauses carry no semantic-denial claim.
def Status (c : Configuration n) : Prop :=
 match c.phase with
 | .halted frame none => frame.pending = []
 | .halted _ (some .referenceBudget) => True
 | .halted frame (some failure) =>
   ∃ ctx command rest, frame.pending = command :: rest ∧
     Transition ctx (Continuation.store c) command (Sequence.unchanged (Continuation.store c) failure)
 | _ => True

theorem failed_step_shape {fuel : Nat} {ctx : Context n} {s : Store n} {command : Instruction n} {failure : Failure}
 (failed : (step fuel ctx s command).failure = some failure) :
 step fuel ctx s command = Sequence.unchanged s failure := by
 have fields := step_failure failed
 cases result : step fuel ctx s command
 simp_all [Sequence.unchanged]

theorem tick_status {c : Configuration n} (valid : Status c) (fuel : Nat) (ctx : Context n) :
 Status (tick fuel ctx c) := by
 unfold tick
 split
 · rename_i frame active
   cases pending : frame.pending with
   | nil => simp [Status,pending]
   | cons command rest =>
     cases status : (step fuel ctx ⟨c.shared,frame.lexical⟩ command).failure with
     | none => simp [Status,status]
     | some failure =>
       have shape := failed_step_shape status
       by_cases budget : failure = .referenceBudget
       · simp [Status,status,budget]
       · have completed : (step fuel ctx ⟨c.shared,frame.lexical⟩ command).failure ≠ some .referenceBudget := by simp [status,budget]
         have semantic := completed_step_sound completed
         rw [shape] at semantic
         cases failure <;> simp_all [Status,Continuation.store,Phase.frame,Sequence.unchanged]
         all_goals exact ⟨ctx,semantic⟩
 · exact valid

theorem act_status {c : Configuration n} (valid : Status c) (control : Control n) : Status (act c control) := by
 cases control with
 | tick fuel ctx => exact tick_status valid fuel ctx
 | retryBudget =>
   simp only [act,retryBudget]
   split
   · simp [Status]
   · exact valid
 | capture =>
   cases phase : c.phase <;> simp [act,capture,phase,Status] at valid ⊢
   exact valid
 | resume token =>
   cases phase : c.phase with
   | active frame => simpa [act,resume,phase] using valid
   | halted frame failure => simpa [act,resume,phase] using valid
   | suspended key frame =>
     by_cases same : token = key <;> simp [act,resume,phase,same,Status]

theorem schedule_status {c : Configuration n} (valid : Status c) (controls : List (Control n)) :
 Status (schedule c controls) := by
 induction controls generalizing c with
 | nil => exact valid
 | cons first rest ih => exact ih (act_status valid first)

theorem from_start_status (initial : Store n) (program : List (Instruction n)) (controls : List (Control n)) :
 Status (schedule (start initial program) controls) :=
 schedule_status (c := start initial program) True.intro controls

#print axioms from_start_status
#print axioms from_start_prefix
#print axioms tick_status
#print axioms from_start
end MirroreaProofFirst.ResourceComputations.Continuation.Prefix

namespace MirroreaProofFirst.ResourceComputations.ProfileConsumer
open PureHandleFunctions ResourceBoundary LocalContract ContractExport
open ModuleContractBoundary ModuleContractBoundary.CurrentAllocation Sequence

-- Independent profile derivations; neither branch assumes checker acceptance.
def Derivable (b : Binding) (e : Envelope) : Prop :=
 match b.profile with
 | .checkedValue => 0 < e.result
 | .symbolicNonnegative => b.code = positiveTerm e.base ∧ Nonnegative b.assumptions e.base

def Guarantee (b : Binding) (e : Envelope) : Prop :=
 match b.profile with
 | .checkedValue => 0 < e.result
 | .symbolicNonnegative => ProfileGuarantees.UniformPositive b ∧
     ∃ input, AssumptionsHold input b.assumptions

theorem export_exists {b : Binding} {e : Envelope} (common : Common b e)
 (derived : Derivable b e) :
 ∃ certificate, accept b {e with certificate := certificate} = some e.result.toNat := by
 cases profile : b.profile with
 | checkedValue =>
   exact ⟨e.certificate,checked_profile_complete common profile (by simpa [Derivable,profile] using derived)⟩
 | symbolicNonnegative =>
   have d : b.code = positiveTerm e.base ∧ Nonnegative b.assumptions e.base := by simpa [Derivable,profile] using derived
   exact symbolic_profile_complete common profile d.1 d.2

theorem guarantee {b : Binding} {e : Envelope} (common : Common b e)
 (derived : Derivable b e) : Guarantee b e := by
 cases profile : b.profile with
 | checkedValue => simpa [Guarantee,Derivable,profile] using derived
 | symbolicNonnegative =>
   obtain ⟨certificate,accepted⟩ := export_exists common derived
   simpa [Guarantee,profile] using ProfileGuarantees.accepted_symbolic_inhabited accepted profile

def withCertificate (proof : CallProof n) (certificate : Certificate) : CallProof n :=
 {proof with payload := {proof.payload with certificate := certificate}}

theorem certificate_linked {world : CurrentUse.World n} {u : CurrentUse.UseRequest n}
 {d : Descriptor} {args : List Int} {proof : CallProof n}
 (linked : Linked world u d args proof) (certificate : Certificate) :
 Linked world u d args (withCertificate proof certificate) :=
 ⟨linked.context,linked.moduleStamp,linked.operationStamp,linked.code,linked.contract,linked.arity,linked.arguments⟩

-- This package states independent INPUT obligations, not output invariants or
-- successful calls. The authority witness and symbolic certificate are existential.
structure Premises (current : RestoredCall.ExecutionContext n) (caller : CurrentUse.UseRequest n)
 (proof : CallProof n) (s : Store n) (source : Source n)
 (h : HandleValues.Interface n) (x : Int) (d : Descriptor) : Prop where
 typed : Typed s.frame.types source
 selected : Selects s.frame.values source h x
 currentUse : CurrentUse.CurrentUse current.world (request caller h x)
 descriptor : current.registry h.operation.key = some d
 catalog : current.catalog d.codeId = some d
 linked : Linked current.world (request caller h x) d [x] proof
 common : Common (expected current.world (request caller h x) d [x]) proof.payload
 profile : Derivable (expected current.world (request caller h x) d [x]) proof.payload
 bounds : CheckedArithmetic.Bounds current.lo current.hi [x] d.code
 grant : current.grant current.world (request caller h x) d s.shared.resources
   (.allocate caller.principal proof.payload.result.toNat) = true
 capacity : BoundedIdentifiers.Capacity current.limits s.shared.resources
   (.allocate caller.principal proof.payload.result.toNat)
 fresh : (CurrentUse.currentContext current.world (request caller h x)).useId ∉ s.shared.committed

def outcome (current : RestoredCall.ExecutionContext n) (caller : CurrentUse.UseRequest n)
 (s : Store n) (h : HandleValues.Interface n) (x : Int) (length : Nat) : Result n :=
 let allocated := raw s.shared.resources (.allocate caller.principal length)
 Semantics.allocationResult s (⟨allocated.1,(CurrentUse.currentContext current.world (request caller h x)).useId :: s.shared.committed⟩,allocated.2)

theorem executes {current : RestoredCall.ExecutionContext n} {caller : CurrentUse.UseRequest n}
 {proof : CallProof n} {s : Store n} {source : Source n}
 {h : HandleValues.Interface n} {x : Int} {d : Descriptor}
 (premises : Premises current caller proof s source h x d) (resourcePolicy : Nat → Bool) :
 Guarantee (expected current.world (request caller h x) d [x]) proof.payload ∧
 ∃ auth certificate minimum, ∀ fuel, minimum ≤ fuel →
 step fuel ⟨current,caller,auth,withCertificate proof certificate,resourcePolicy⟩ s (.allocate source) =
 outcome current caller s h x proof.payload.result.toNat := by
 refine ⟨guarantee premises.common premises.profile,?_⟩
 obtain ⟨certificate,accepted⟩ := export_exists premises.common premises.profile
 have linked := certificate_linked premises.linked certificate
 obtain ⟨auth,allocated⟩ := CurrentAllocation.lawful_completes premises.currentUse
   premises.descriptor premises.catalog linked accepted premises.bounds premises.grant premises.capacity
 obtain ⟨minimum,enough⟩ := select_complete premises.selected
 refine ⟨auth,certificate,minimum,fun fuel bound => ?_⟩
 rw [Semantics.selected_step premises.typed (enough fuel bound)]
 have once : Once.step (invocation current caller auth (withCertificate proof certificate) h x) s.shared =
 some (⟨(raw s.shared.resources (.allocate caller.principal proof.payload.result.toNat)).1,
 (CurrentUse.currentContext current.world (request caller h x)).useId :: s.shared.committed⟩,
 (raw s.shared.resources (.allocate caller.principal proof.payload.result.toNat)).2) := by
   apply (Once.step_exact _ _ _).mpr
   exact ⟨premises.fresh,_,allocated,rfl⟩
 rw [once]
 rfl

#print axioms export_exists
#print axioms guarantee
#print axioms executes
end MirroreaProofFirst.ResourceComputations.ProfileConsumer

namespace MirroreaProofFirst.ResourceComputations.CaptureAdapter

-- Metadata is already-supplied source information, not inferred labels or
-- authenticated provenance. It remains attached to every result and continuation.
structure Declaration (I : Type) where
 source : ProducerFlow.Expr I
 types : I → ProducerFlow.Ty
 labels : I → Nat
 control : Nat
 captureLabel : Nat
 consumerLabel : Nat

def Admissible (d : Declaration I) : Prop :=
 OwnerAssignment.Capture.Allowed d.types d.labels d.control d.source .int d.captureLabel ∧
 d.captureLabel ≤ d.consumerLabel

def check (d : Declaration I) : Bool :=
 OwnerAssignment.Capture.check d.types d.labels d.control d.source .int d.captureLabel &&
 decide (d.captureLabel ≤ d.consumerLabel)

theorem check_exact (d : Declaration I) : check d = true ↔ Admissible d := by
 simp only [check,Bool.and_eq_true,OwnerAssignment.Capture.check_exact,decide_eq_true_eq,Admissible]

structure Tagged (I : Type) (n : Nat) where
 declaration : Declaration I
 computation : Continuation.Configuration n

inductive Failure where | metadata | capture
 deriving DecidableEq, Repr
inductive Outcome (n : Nat) where
 | rejected (state : Sequence.Store n) (failure : Failure)
 | ready (computation : Continuation.Configuration n)
structure Entry (I : Type) (n : Nat) where
 declaration : Declaration I
 outcome : Outcome n

def bindCaptured (s : Sequence.Store n) (x : Int) : Sequence.Store n :=
 {s with frame := {s.frame with types := .int :: s.frame.types, values := .integer x :: s.frame.values}}

def enter (ops : FallibleFlow.Arithmetic) (input : I → ProducerFlow.Value)
 (d : Declaration I) (s : Sequence.Store n) (program : List (Sequence.Instruction n)) : Entry I n :=
 ⟨d,if check d then
   match FallibleFlow.eval ops input d.source with
   | some (.int x) => .ready (Continuation.start (bindCaptured s x) program)
   | _ => .rejected s .capture
 else .rejected s .metadata⟩

inductive Enters (ops : FallibleFlow.Arithmetic) (input : I → ProducerFlow.Value)
 (d : Declaration I) (s : Sequence.Store n) (program : List (Sequence.Instruction n)) : Outcome n → Prop where
 | metadata : ¬ Admissible d → Enters ops input d s program (.rejected s .metadata)
 | failed : Admissible d → (∀ x, FallibleFlow.eval ops input d.source ≠ some (.int x)) →
     Enters ops input d s program (.rejected s .capture)
 | captured : Admissible d → FallibleFlow.eval ops input d.source = some (.int x) →
     Enters ops input d s program (.ready (Continuation.start (bindCaptured s x) program))

theorem enter_exact (ops : FallibleFlow.Arithmetic) (input : I → ProducerFlow.Value)
 (d : Declaration I) (s : Sequence.Store n) (program : List (Sequence.Instruction n)) (out : Outcome n) :
 (enter ops input d s program).outcome = out ↔ Enters ops input d s program out := by
 constructor
 · intro eq; subst out
   unfold enter
   by_cases accepted : check d = true
   · simp only [accepted,ite_true]
     cases actual : FallibleFlow.eval ops input d.source with
     | none => exact .failed ((check_exact d).mp accepted) (by simp [actual])
     | some value =>
       cases value with
       | int x => exact .captured ((check_exact d).mp accepted) actual
       | bool b => exact .failed ((check_exact d).mp accepted) (by simp [actual])
   · simp only [accepted]
     exact .metadata (fun h => accepted ((check_exact d).mpr h))
 · intro semantic
   cases semantic with
   | metadata no => simp [enter,show check d = false from Bool.eq_false_iff.mpr (fun h => no ((check_exact d).mp h))]
   | failed admitted absent =>
     simp only [enter,(check_exact d).mpr admitted,ite_true]
   | captured admitted actual => simp [enter,(check_exact d).mpr admitted,actual]

theorem bind_good {s : Sequence.Store n} (good : Sequence.Good s) (x : Int) :
 Sequence.Good (bindCaptured s x) :=
 ⟨good.resources,good.realized,good.unique,.cons .integer good.environment⟩

theorem value_retained (s : Sequence.Store n) (x : Int) :
 (bindCaptured s x).frame.values[0]? = some (.integer x) ∧
 (bindCaptured s x).frame.values.drop 1 = s.frame.values ∧
 (bindCaptured s x).frame.rights = s.frame.rights ∧ (bindCaptured s x).shared = s.shared :=
 ⟨rfl,rfl,rfl,rfl⟩

theorem metadata_retained (ops : FallibleFlow.Arithmetic) (input : I → ProducerFlow.Value)
 (d : Declaration I) (s : Sequence.Store n) (program : List (Sequence.Instruction n)) :
 (enter ops input d s program).declaration = d := rfl

theorem ready_origin {ops : FallibleFlow.Arithmetic} {input : I → ProducerFlow.Value}
 {d : Declaration I} {s : Sequence.Store n} {program : List (Sequence.Instruction n)}
 {out : Continuation.Configuration n}
 (ready : (enter ops input d s program).outcome = .ready out) :
 Admissible d ∧ ∃ x, FallibleFlow.eval ops input d.source = some (.int x) ∧
 out = Continuation.start (bindCaptured s x) program := by
 have semantic := (enter_exact _ _ _ _ _ _).mp ready
 cases semantic with
 | captured allowed actual => exact ⟨allowed,_,actual,rfl⟩

-- Guarded wrapper is the selected adapter entry; raw constructors are not a
-- provenance service. No operation may silently lower the stored completion label.
def act (state : Tagged I n) (control : Continuation.Control n) : Tagged I n :=
 if check state.declaration then
 {state with computation := Continuation.act state.computation control}
 else state

def schedule : Tagged I n → List (Continuation.Control n) → Tagged I n
 | s,[] => s
 | s,control :: rest => schedule (act s control) rest

theorem act_metadata (s : Tagged I n) (control : Continuation.Control n) :
 (act s control).declaration = s.declaration := by unfold act; split <;> rfl

theorem schedule_metadata (s : Tagged I n) (controls : List (Continuation.Control n)) :
 (schedule s controls).declaration = s.declaration := by
 induction controls generalizing s with
 | nil => rfl
 | cons first rest ih => exact (ih _).trans (act_metadata s first)

theorem schedule_computation {s : Tagged I n} (admitted : Admissible s.declaration)
 (controls : List (Continuation.Control n)) :
 (schedule s controls).computation = Continuation.schedule s.computation controls := by
 induction controls generalizing s with
 | nil => rfl
 | cons first rest ih =>
   simp only [schedule,Continuation.schedule]
   have next : Admissible (act s first).declaration := by simpa [act_metadata] using admitted
   rw [ih next]
   simp [act,(check_exact s.declaration).mpr admitted]

def tagged (entry : Entry I n) : Option (Tagged I n) :=
 match entry.outcome with
 | .ready c => some ⟨entry.declaration,c⟩
 | .rejected _ _ => none

theorem ready_good {ops : FallibleFlow.Arithmetic} {input : I → ProducerFlow.Value}
 {d : Declaration I} {s : Sequence.Store n} {program : List (Sequence.Instruction n)}
 {out : Continuation.Configuration n} (good : Sequence.Good s)
 (ready : (enter ops input d s program).outcome = .ready out) : Continuation.Good out := by
 obtain ⟨_,x,_,rfl⟩ := ready_origin ready
 exact bind_good good x

theorem completion_retained {ops : FallibleFlow.Arithmetic} {input : I → ProducerFlow.Value}
 {d : Declaration I} {s : Sequence.Store n} {program : List (Sequence.Instruction n)}
 {out : Continuation.Configuration n}
 (ready : (enter ops input d s program).outcome = .ready out) (controls : List (Continuation.Control n)) :
 (schedule (⟨d,out⟩ : Tagged I n) controls).declaration.captureLabel ≤
 (schedule (⟨d,out⟩ : Tagged I n) controls).declaration.consumerLabel := by
 rw [schedule_metadata]
 exact (ready_origin ready).1.2

theorem act_denied {s : Tagged I n} (denied : ¬ Admissible s.declaration)
 (control : Continuation.Control n) : act s control = s := by
 have rejected : check s.declaration = false := Bool.eq_false_iff.mpr (fun h => denied ((check_exact _).mp h))
 simp [act,rejected]

theorem schedule_denied {s : Tagged I n} (denied : ¬ Admissible s.declaration)
 (controls : List (Continuation.Control n)) : schedule s controls = s := by
 induction controls with
 | nil => rfl
 | cons first rest ih => simpa [schedule,act_denied denied first] using ih

namespace Controls
abbrev Key := Fin 1
def declaration (label consumer : Nat) : Declaration Key :=
 ⟨.read 0,fun _ => .int,fun _ => label,0,label,consumer⟩
def input (x : Int) (_ : Key) : ProducerFlow.Value := .int x
def initial : Sequence.Store 4 := ⟨ModuleContractBoundary.CurrentAllocation.Once.Controls.initial,⟨[],[],[],fun _ => none⟩⟩
def entry (label consumer : Nat) := enter (FallibleFlow.signed 63) (input 3)
 (declaration label consumer) initial Sequence.Controls.program
def controls : List (Continuation.Control 4) :=
 (List.range 7).map (fun position => .tick 20 (Sequence.Controls.context 3 position))
def completed (label consumer : Nat) := (tagged (entry label consumer)).map (fun s => schedule s controls)

example : (completed 0 0).map (fun s => (Continuation.Controls.outcome s.computation,s.computation.events.length)) = some (none,5) := by decide
example : (completed 1 1).map (fun s => s.computation.events.length) = some 5 := by decide
example : (completed 1 0).isNone = true := by decide
example : (completed 1 1).map (fun s => (s.declaration.captureLabel,s.declaration.consumerLabel)) = some (1,1) := by decide
example : (tagged (entry 0 0)).map (fun s => s.computation.phase.frame.lexical.values[0]?) = some (some (.integer 3)) := by rfl
example : (tagged (entry 1 1)).map (fun s => s.computation.phase.frame.lexical.values[0]?) = some (some (.integer 3)) := by rfl

-- Even an unused secret capture has a completion dependency. Overflow does not
-- start the consumer and the result keeps its high declaration.
def overflowDeclaration : Declaration Key :=
 {declaration 1 1 with source := .add (.read 0) (.lit (.int 1))}
def failed := enter (FallibleFlow.signed 63) (input 9223372036854775807)
 overflowDeclaration initial Sequence.Controls.program
example : (tagged failed).isNone = true ∧ failed.declaration.captureLabel = 1 := by decide
end Controls

#print axioms ready_good
#print axioms completion_retained
#print axioms schedule_denied
#print axioms enter_exact
#print axioms bind_good
#print axioms value_retained
#print axioms ready_origin
#print axioms schedule_metadata
#print axioms schedule_computation
end MirroreaProofFirst.ResourceComputations.CaptureAdapter

namespace MirroreaProofFirst.ResourceComputations
open ResourceBoundary ResourceContext ModuleContractBoundary.CurrentAllocation

theorem allocateOnce_untouched {fuel : Nat} {G : List PureHandleFunctions.Ty}
 {env : List (PureHandleFunctions.Value n)} {source : Source n}
 {current : RestoredCall.ExecutionContext n} {caller : CurrentUse.UseRequest n}
 {auth : CurrentUse.Evidence} {proof : ModuleContractBoundary.CallProof n}
 {state : Once.StateWithHistory} {delta : Delta} {out : Once.StateWithHistory × Delta × List Handle}
 (executed : allocateOnce fuel G env source current caller auth proof state delta = some out)
 (identity : Nat) (notReturned : ∀ h ∈ out.2.2, identity ≠ h.id) : out.2.1 identity = delta identity := by
 obtain ⟨_,h,x,result,_,_,rfl⟩ := allocateOnce_selected executed
 exact extend_untouched _ _ _ notReturned

theorem allocateOnce_returned {fuel : Nat} {G : List PureHandleFunctions.Ty}
 {env : List (PureHandleFunctions.Value n)} {source : Source n}
 {current : RestoredCall.ExecutionContext n} {caller : CurrentUse.UseRequest n}
 {auth : CurrentUse.Evidence} {proof : ModuleContractBoundary.CallProof n}
 {state : Once.StateWithHistory} {delta : Delta} {out : Once.StateWithHistory × Delta × List Handle}
 (executed : allocateOnce fuel G env source current caller auth proof state delta = some out) :
 ∀ h ∈ out.2.2, out.2.1 h.id = some h.region := by
 obtain ⟨_,h,x,result,_,stepped,rfl⟩ := allocateOnce_selected executed
 obtain ⟨_,allocated,accepted,rfl⟩ := (Once.step_exact _ _ _).mp stepped
 obtain ⟨d,value,_,_,_,_,_,_,_,_,rfl⟩ := (ModuleContractBoundary.CurrentAllocation.exact _ _ _ _ _ _ _ _ _ _ _ _ _).mp accepted
 simpa [ResourceContext.update,ResourceContext.consume,consumed] using
   updated_returned state.resources delta (.allocate caller.principal value)

#print axioms allocateOnce_untouched
#print axioms allocateOnce_returned
end MirroreaProofFirst.ResourceComputations

namespace MirroreaProofFirst.ResourceComputations.Continuation
open Sequence Sequence.Typing

theorem checked_schedule {initial : Store n} {program : List (Instruction n)} {output : Shape}
 (good : Sequence.Good initial) (typed : ProgramTyped (shape initial.frame) program output)
 (controls : List (Control n)) :
 let out := schedule (start initial program) controls
 Good out ∧ FrameTyped out.phase.frame output ∧ IdentityWF out ∧
 Prefix.Realizes initial program out ∧ Prefix.Status out ∧ RightsTrace initial.frame.rights out ∧
 initial.shared.committed ⊆ out.shared.committed ∧
 (∀ identity, RetiredResource identity initial.shared.resources → RetiredResource identity out.shared.resources) := by
 have goodStart : Good (start initial program) := good
 have typedStart : FrameTyped (start initial program).phase.frame output := ⟨good.environment,typed⟩
 have typeResult := schedule_typed goodStart typedStart controls
 have validResult := schedule_good goodStart (start_identity initial program) controls
 exact ⟨typeResult.1,typeResult.2,validResult.2,Prefix.from_start initial program controls,
   Prefix.from_start_status initial program controls,from_start_rights_trace initial program controls,
   schedule_history (start initial program) controls,
   fun _ retired => schedule_resource_retired retired controls⟩

#print axioms checked_schedule
end MirroreaProofFirst.ResourceComputations.Continuation

namespace MirroreaProofFirst.ResourceComputations.CaptureAdapter
-- The metadata adapter and actual arbitrary-control scheduler share this one
-- realization. Value-only erasure cannot satisfy the retained declaration field.
theorem captured_schedule {ops : FallibleFlow.Arithmetic} {input : I → ProducerFlow.Value}
 {d : Declaration I} {initial : Sequence.Store n} {program : List (Sequence.Instruction n)}
 {captured : Continuation.Configuration n}
 (ready : (enter ops input d initial program).outcome = .ready captured)
 (controls : List (Continuation.Control n)) :
 Admissible d ∧ ∃ x,
 FallibleFlow.eval ops input d.source = some (.int x) ∧
 (schedule (⟨d,captured⟩ : Tagged I n) controls).declaration = d ∧
 Continuation.Prefix.Realizes (bindCaptured initial x) program
   (schedule (⟨d,captured⟩ : Tagged I n) controls).computation ∧
 Continuation.Prefix.Status (schedule (⟨d,captured⟩ : Tagged I n) controls).computation ∧
 Continuation.RightsTrace initial.frame.rights (schedule (⟨d,captured⟩ : Tagged I n) controls).computation := by
 obtain ⟨admitted,x,actual,rfl⟩ := ready_origin ready
 refine ⟨admitted,x,actual,schedule_metadata _ _,?_,?_,?_⟩
 all_goals rw [schedule_computation admitted]
 · exact Continuation.Prefix.from_start _ _ _
 · exact Continuation.Prefix.from_start_status _ _ _
 · exact Continuation.from_start_rights_trace _ _ _

#print axioms captured_schedule
end MirroreaProofFirst.ResourceComputations.CaptureAdapter

namespace MirroreaProofFirst.ResourceComputations.Continuation.ControlIndexed
open Sequence Sequence.Semantics

-- Independent control-indexed transitions: the context in a semantic transition
-- is exactly the one in THIS control, not an existential past/future context.
-- Fuel is abstracted for semantic outcomes. Only budget exhaustion uses the
-- operational budget predicate; it is not a semantic authorization judgment.
inductive Step : Configuration n → Control n → Configuration n → Prop where
 | tickSuccess {c : Configuration n} {frame : Frame n}
     (active : c.phase = .active frame) (pending : frame.pending = command :: rest)
     (meaning : Transition ctx ⟨c.shared,frame.lexical⟩ command result)
     (success : result.failure = none) :
     Step c (.tick fuel ctx)
       {c with shared := result.store.shared, phase := .active ⟨result.store.frame,rest,frame.position+1⟩, events := c.events ++ result.events}
 | tickDenied {c : Configuration n} {frame : Frame n} {failure : Failure}
     (active : c.phase = .active frame) (pending : frame.pending = command :: rest)
     (meaning : Transition ctx ⟨c.shared,frame.lexical⟩ command
       (Sequence.unchanged ⟨c.shared,frame.lexical⟩ failure)) :
     Step c (.tick fuel ctx) {c with phase := .halted frame (some failure)}
 | tickBudget {c : Configuration n} {frame : Frame n}
     (active : c.phase = .active frame) (pending : frame.pending = command :: rest)
     (budget : (Sequence.step fuel ctx ⟨c.shared,frame.lexical⟩ command).failure = some .referenceBudget) :
     Step c (.tick fuel ctx) {c with phase := .halted frame (some .referenceBudget)}
 | tickDone {c : Configuration n} {frame : Frame n}
     (active : c.phase = .active frame) (pending : frame.pending = []) :
     Step c (.tick fuel ctx) {c with phase := .halted frame none}
 | tickInactive {c : Configuration n} (inactive : ∀ frame, c.phase ≠ .active frame) :
     Step c (.tick fuel ctx) c
 | captureActive {c : Configuration n} {frame : Frame n}
     (active : c.phase = .active frame) :
     Step c .capture {c with phase := .suspended c.nextIdentity frame,nextIdentity := c.nextIdentity+1}
 | captureInactive {c : Configuration n} (inactive : ∀ frame, c.phase ≠ .active frame) :
     Step c .capture c
 | resumeMatching {c : Configuration n} {frame : Frame n}
     (suspended : c.phase = .suspended identity frame) :
     Step c (.resume identity) {c with phase := .active frame}
 | resumeUnavailable {c : Configuration n}
     (unavailable : ∀ frame, c.phase ≠ .suspended identity frame) :
     Step c (.resume identity) c
 | retryPaused {c : Configuration n} {frame : Frame n}
     (paused : c.phase = .halted frame (some .referenceBudget)) :
     Step c .retryBudget {c with phase := .active frame}
 | retryOther {c : Configuration n}
     (other : ∀ frame, c.phase ≠ .halted frame (some .referenceBudget)) :
     Step c .retryBudget c

inductive Scheduled : Configuration n → List (Control n) → Configuration n → Prop where
 | empty : Scheduled c [] c
 | next {control : Control n} : Step c control middle → Scheduled middle controls out →
     Scheduled c (control :: controls) out

theorem tick_sound (c : Configuration n) (fuel : Nat) (ctx : Context n) :
 Step c (.tick fuel ctx) (tick fuel ctx c) := by
 unfold tick
 split
 · rename_i frame active
   cases pending : frame.pending with
   | nil => simpa [pending] using Step.tickDone (fuel := fuel) (ctx := ctx) active pending
   | cons command rest =>
     cases status : (Sequence.step fuel ctx ⟨c.shared,frame.lexical⟩ command).failure with
     | none =>
       have semantic := completed_step_sound (show (Sequence.step fuel ctx ⟨c.shared,frame.lexical⟩ command).failure ≠ some .referenceBudget by simp [status])
       simpa [status] using Step.tickSuccess (fuel := fuel) active pending semantic status
     | some failure =>
       have shape := Prefix.failed_step_shape status
       by_cases budget : failure = .referenceBudget
       · subst failure
         simpa [shape,Sequence.unchanged,←pending] using Step.tickBudget active pending status
       · have semantic := completed_step_sound (show (Sequence.step fuel ctx ⟨c.shared,frame.lexical⟩ command).failure ≠ some .referenceBudget by simp [status,budget])
         rw [shape] at semantic
         simpa [shape,Sequence.unchanged,←pending] using Step.tickDenied (fuel := fuel) active pending semantic
 · exact Step.tickInactive (by assumption)

theorem act_sound (c : Configuration n) (control : Control n) :
 Step c control (act c control) := by
 cases control with
 | tick fuel ctx => exact tick_sound c fuel ctx
 | capture =>
   cases phase : c.phase with
   | active frame => simpa [act,capture,phase] using Step.captureActive phase
   | suspended key frame => simpa [act,capture,phase] using Step.captureInactive (c := c) (by simp [phase])
   | halted frame failure => simpa [act,capture,phase] using Step.captureInactive (c := c) (by simp [phase])
 | resume identity =>
   cases phase : c.phase with
   | active frame => simpa [act,resume,phase] using Step.resumeUnavailable (c := c) (identity := identity) (by simp [phase])
   | halted frame failure => simpa [act,resume,phase] using Step.resumeUnavailable (c := c) (identity := identity) (by simp [phase])
   | suspended key frame =>
     by_cases same : identity = key
     · subst identity; simpa [act,resume,phase] using Step.resumeMatching phase
     · simpa [act,resume,phase,same] using Step.resumeUnavailable (c := c) (identity := identity) (by simp [phase,Ne.symm same])
 | retryBudget =>
   cases phase : c.phase with
   | active frame => simpa [act,retryBudget,phase] using Step.retryOther (c := c) (by simp [phase])
   | suspended key frame => simpa [act,retryBudget,phase] using Step.retryOther (c := c) (by simp [phase])
   | halted frame failure =>
     cases failure with
     | none => simpa [act,retryBudget,phase] using Step.retryOther (c := c) (by simp [phase])
     | some failure =>
       cases failure <;> simp only [act,retryBudget,phase]
       · exact Step.retryOther (by simp [phase])
       · exact Step.retryPaused phase
       · exact Step.retryOther (by simp [phase])
       · exact Step.retryOther (by simp [phase])

theorem schedule_sound (c : Configuration n) (controls : List (Control n)) :
 Scheduled c controls (schedule c controls) := by
 induction controls generalizing c with
 | nil => exact .empty
 | cons first rest ih => exact .next (act_sound c first) (ih _)

#print axioms tick_sound
#print axioms act_sound
#print axioms schedule_sound
end MirroreaProofFirst.ResourceComputations.Continuation.ControlIndexed

namespace MirroreaProofFirst.ResourceComputations.Continuation.ControlIndexed
open Sequence Sequence.Semantics

theorem step_realizes {initial : Store n} {program : List (Instruction n)}
 {c out : Configuration n} {control : Control n}
 (valid : Prefix.Realizes initial program c) (rule : Step c control out) :
 Prefix.Realizes initial program out := by
 cases rule with
 | tickSuccess active pending meaning success =>
   rename_i command rest ctx result fuel frame
   obtain ⟨done,body,position,executed⟩ := valid
   simp only [active,Phase.frame,Continuation.store] at body position executed
   refine ⟨done ++ [command],?_,?_,?_⟩
   · simpa [pending,Phase.frame,List.append_assoc] using body
   · simp [Phase.frame,position,List.length_append]
   · simpa [Phase.frame,Continuation.store] using Prefix.Executed.next executed meaning success
 | tickDenied active pending meaning => simpa [Prefix.Realizes,Continuation.store,Phase.frame,active] using valid
 | tickBudget active pending budget => simpa [Prefix.Realizes,Continuation.store,Phase.frame,active] using valid
 | tickDone active pending => simpa [Prefix.Realizes,Continuation.store,Phase.frame,active] using valid
 | tickInactive inactive => exact valid
 | captureActive active => simpa [Prefix.Realizes,Continuation.store,Phase.frame,active] using valid
 | captureInactive inactive => exact valid
 | resumeMatching suspended => simpa [Prefix.Realizes,Continuation.store,Phase.frame,suspended] using valid
 | resumeUnavailable unavailable => exact valid
 | retryPaused paused => simpa [Prefix.Realizes,Continuation.store,Phase.frame,paused] using valid
 | retryOther other => exact valid

theorem step_status {c out : Configuration n} {control : Control n}
 (valid : Prefix.Status c) (rule : Step c control out) : Prefix.Status out := by
 cases rule with
 | tickSuccess active pending meaning success => simp [Prefix.Status]
 | tickDenied active pending meaning =>
   rename_i command rest ctx fuel frame failure
   cases failure <;> simp_all [Prefix.Status,Continuation.store,Phase.frame]
   all_goals exact ⟨ctx,meaning⟩
 | tickBudget active pending budget => simp [Prefix.Status]
 | tickDone active pending => exact pending
 | tickInactive inactive => exact valid
 | captureActive active => trivial
 | captureInactive inactive => exact valid
 | resumeMatching suspended => trivial
 | resumeUnavailable unavailable => exact valid
 | retryPaused paused => trivial
 | retryOther other => exact valid

theorem scheduled_forget {initial : Store n} {program : List (Instruction n)}
 {c out : Configuration n} {controls : List (Control n)}
 (valid : Prefix.Realizes initial program c) (status : Prefix.Status c)
 (execution : Scheduled c controls out) :
 Prefix.Realizes initial program out ∧ Prefix.Status out := by
 induction execution with
 | empty => exact ⟨valid,status⟩
 | next first later ih => exact ih (step_realizes valid first) (step_status status first)

#print axioms step_realizes
#print axioms step_status
#print axioms scheduled_forget
end MirroreaProofFirst.ResourceComputations.Continuation.ControlIndexed

namespace MirroreaProofFirst.ResourceComputations.Continuation.ControlIndexed
open Sequence Sequence.Semantics

theorem allocation_denied {ctx : Context n} {s : Store n} {source : Source n} {result : Result n}
 (denied : ctx.current.grant = fun _ _ _ _ _ => false)
 (meaning : Transition ctx s (.allocate source) result) : result.failure ≠ none := by
 cases meaning with
 | callRejected rejected => simp [Sequence.unchanged]
 | callDenied typed selected absent => simp [Sequence.unchanged]
 | callValue typed selected admitted =>
   obtain ⟨_,d,value,_,_,_,_,_,_,grant,_,_⟩ := admitted
   simp [invocation,denied] at grant

theorem denied_tick_position {ctx : Context n} {c out : Configuration n}
 {frame : Frame n} {source : Source n} {rest : List (Instruction n)} {fuel : Nat}
 (active : c.phase = .active frame) (pending : frame.pending = .allocate source :: rest)
 (denied : ctx.current.grant = fun _ _ _ _ _ => false)
 (rule : Step c (.tick fuel ctx) out) : out.phase.frame.position = frame.position := by
 cases rule with
 | tickSuccess before body meaning success =>
   simp_all only [Phase.active.injEq,List.cons.injEq]
   rw [←body.1] at meaning
   exact False.elim ((allocation_denied denied meaning) success)
 | tickDenied before body meaning => simp_all [Phase.frame]
 | tickBudget before body budget => simp_all [Phase.frame]
 | tickDone before empty => simp_all
 | tickInactive inactive => exact False.elim (inactive frame active)

-- The strengthened conjunction refers to one and the same actual schedule.
theorem checked_control_schedule {initial : Store n} {program : List (Instruction n)} {output : Typing.Shape}
 (good : Sequence.Good initial) (typed : Typing.ProgramTyped (Typing.shape initial.frame) program output)
 (controls : List (Control n)) :
 let out := schedule (start initial program) controls
 Scheduled (start initial program) controls out ∧
 Good out ∧ Typing.FrameTyped out.phase.frame output ∧ IdentityWF out ∧
 Prefix.Realizes initial program out ∧ Prefix.Status out ∧ RightsTrace initial.frame.rights out ∧
 initial.shared.committed ⊆ out.shared.committed ∧
 (∀ identity, RetiredResource identity initial.shared.resources → RetiredResource identity out.shared.resources) :=
 ⟨schedule_sound _ _,checked_schedule good typed controls⟩

#print axioms allocation_denied
#print axioms denied_tick_position
#print axioms checked_control_schedule
end MirroreaProofFirst.ResourceComputations.Continuation.ControlIndexed

namespace MirroreaProofFirst.ResourceComputations.CaptureAdapter
theorem captured_control_schedule {ops : FallibleFlow.Arithmetic} {input : I → ProducerFlow.Value}
 {d : Declaration I} {initial : Sequence.Store n} {program : List (Sequence.Instruction n)}
 {captured : Continuation.Configuration n}
 (ready : (enter ops input d initial program).outcome = .ready captured)
 (controls : List (Continuation.Control n)) :
 Admissible d ∧ ∃ x,
 FallibleFlow.eval ops input d.source = some (.int x) ∧
 Continuation.ControlIndexed.Scheduled (Continuation.start (bindCaptured initial x) program) controls
   (schedule (⟨d,captured⟩ : Tagged I n) controls).computation ∧
 (schedule (⟨d,captured⟩ : Tagged I n) controls).declaration = d := by
 obtain ⟨admitted,x,actual,rfl⟩ := ready_origin ready
 refine ⟨admitted,x,actual,?_,schedule_metadata _ _⟩
 rw [schedule_computation admitted]
 exact Continuation.ControlIndexed.schedule_sound _ _
#print axioms captured_control_schedule
end MirroreaProofFirst.ResourceComputations.CaptureAdapter

namespace MirroreaProofFirst.ResourceComputations.Continuation.ControlIndexed.Controls
open Sequence

def source : Source 4 := ⟨.handle HandleValues.Controls.token,.integer 4⟩
def initial := Sequence.Controls.initial 3
def body : List (Instruction 4) := [.allocate source]
def allowing := Sequence.Controls.context 3 0
def denying : Context 4 := {allowing with current := {allowing.current with grant := fun _ _ _ _ _ => false}}
def first := start initial body
def accepted := schedule first [.tick 20 allowing]
def rejected := schedule first [.tick 20 denying]
example : accepted.events.length = 1 ∧ accepted.phase.frame.position = 1 := by decide
example : rejected.events = [] ∧ rejected.phase.frame.position = 0 := by decide
-- The older existential-context predicate admits this candidate endpoint,
-- regardless of what intended controls are supplied outside that predicate.
example : Prefix.Realizes initial body accepted ∧ Prefix.Status accepted :=
 ⟨Prefix.from_start _ _ _,Prefix.from_start_status _ _ _⟩
-- The independent CONTROL-INDEXED specification excludes precisely that endpoint
-- for the denying control, without invoking the actual scheduler in the proof.
theorem context_substitution_rejected : ¬ Scheduled first [.tick 20 denying] accepted := by
 intro execution
 cases execution with
 | next rule later =>
   cases later
   have stopped := denied_tick_position (frame := ⟨initial.frame,body,0⟩) (source := source)
     (rest := []) rfl rfl rfl rule
   have advanced : accepted.phase.frame.position = 1 := by decide
   rw [advanced] at stopped
   contradiction

#print axioms context_substitution_rejected
end MirroreaProofFirst.ResourceComputations.Continuation.ControlIndexed.Controls
