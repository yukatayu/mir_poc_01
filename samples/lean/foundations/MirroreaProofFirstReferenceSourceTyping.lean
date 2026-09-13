import MirroreaProofFirstReferenceSourceElaboration

namespace MirroreaProofFirst.ReferenceSourceTyping
open ReferenceSourceData ReferenceSource ReferenceSourceElaboration

def kindType : ResultKind → Ty
  | .unit => .plain .unit | .definition => .plain .definition | .instance => .plain .callable
def planOutput : Plan → Option (String × Ty)
  | .pureValue name value false => some (name,typeOf value)
  | .pureValue _ _ true => none
  | .control name _ kind => some (name,kindType kind)
  | .call name _ _ => some (name,.plain (.integer false))
  | .acquire name _ => some (name,.reference)
  | .reacquire name _ | .release name _ => some (name,.plain .unit)

-- Only an assignment updates an existing binding; its full type must match.
def AssignmentTyped (values : Values) : Plan → Prop
  | .pureValue name value true => ∃ old, lookup values name = some old ∧ typeOf value = typeOf old
  | _ => True

theorem output_corresponds (s : InstanceState.State d p n) (values : Values) (statement : Statement)
    (plan : Plan) (meaning : PlanFor s values statement plan) : planOutput plan = output statement := by
  cases statement with
  | plain statement =>
      cases statement <;> simp only [PlanFor,PlainFor] at meaning <;>
        grind [planOutput,output,SourceTypes.output,kindType,typeOf,SourceTypes.typeOf]
  | acquire name chain => obtain ⟨_,_,_,rfl⟩ := meaning; rfl
  | alias name target => obtain ⟨_,_,rfl⟩ := meaning; rfl
  | reacquire name target => obtain ⟨_,_,rfl⟩ := meaning; rfl
  | release name target => obtain ⟨_,_,rfl⟩ := meaning; rfl

theorem mutable_projection (values : Values) (name : String)
    (hasType : SourceTypes.Has (plainEnvironment (environment values)) name (.integer true)) :
    ∃ value, lookup values name = some (.plain (.integer value true)) := by
  simp only [SourceTypes.Has,lookup_plainEnvironment,lookup_environment,Option.map_map] at hasType
  cases found : lookup values name with
  | none => simp [found] at hasType
  | some value =>
      cases value with
      | reference key => simp [found,typeOf,plainType] at hasType
      | plain value =>
          cases value with
          | unit => simp [found,typeOf,plainType,SourceTypes.typeOf] at hasType
          | definition key => simp [found,typeOf,plainType,SourceTypes.typeOf] at hasType
          | callable key => simp [found,typeOf,plainType,SourceTypes.typeOf] at hasType
          | integer number mutable =>
              have yes : mutable = true := by simpa [found,typeOf,plainType,SourceTypes.typeOf] using hasType
              subst mutable
              exact ⟨number,rfl⟩

theorem assignment_corresponds (s : InstanceState.State d p n) (values : Values) (statement : Statement)
    (plan : Plan) (env : Environment) (typed : StatementTyped p (environment values) statement env)
    (meaning : PlanFor s values statement plan) : AssignmentTyped values plan := by
  cases statement with
  | plain statement =>
      cases statement with
      | assign name expression =>
          obtain ⟨value,_,rfl⟩ := meaning
          have hasType := typed.1.1
          obtain ⟨old,found⟩ := mutable_projection values name hasType
          exact ⟨.plain (.integer old true),found,rfl⟩
      | _ => simp only [PlanFor,PlainFor] at meaning; grind [AssignmentTyped]
  | acquire name chain => obtain ⟨_,_,_,rfl⟩ := meaning; trivial
  | alias name target => obtain ⟨_,_,rfl⟩ := meaning; trivial
  | reacquire name target => obtain ⟨_,_,rfl⟩ := meaning; trivial
  | release name target => obtain ⟨_,_,rfl⟩ := meaning; trivial

theorem update_environment (values : Values) (name : String) (old value : Value)
    (found : lookup values name = some old) (same : typeOf value = typeOf old) :
    environment (updateValue values name value) = environment values := by
  induction values with
  | nil => simp [lookup] at found
  | cons row rest ih =>
      by_cases key : row.1 = name
      · have eq : row.2 = old := by simpa [lookup,key] using found
        simp [updateValue,key,environment,eq,same]
      · have tail : lookup rest name = some old := by simpa [lookup,key] using found
        simp only [updateValue,if_neg key]
        change (row.1,typeOf row.2) :: environment (updateValue rest name value) =
          (row.1,typeOf row.2) :: environment rest
        rw [ih tail]

theorem resultValue_type (kind : ResultKind) (created : Option Nat) (value : Value)
    (resolved : resultValue kind created = some value) : typeOf value = kindType kind := by
  cases kind <;> cases created <;> simp [resultValue] at resolved <;> subst value <;> rfl

theorem executePlan_environment (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (plan : Plan) (env : Environment)
    (outputs : Produces (environment s.values) (planOutput plan) env)
    (assignment : AssignmentTyped s.values plan)
    (ready : (executePlan s member place principal item plan).status = .ready) :
    environment (executePlan s member place principal item plan).state.values = env := by
  cases plan with
  | pureValue name value assign =>
      cases assign with
      | false => obtain ⟨_,rfl⟩ := outputs; rfl
      | true =>
          obtain ⟨old,found,same⟩ := assignment
          have equal : env = environment s.values := outputs
          subst env
          exact update_environment _ _ _ _ found same
  | control name command kind =>
      obtain ⟨_,rfl⟩ := outputs
      cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest (withOwner principal command) with
      | none => simp [executePlan,run] at ready
      | some pair =>
          obtain ⟨machine,created⟩ := pair
          cases value : resultValue kind created with
          | none => simp [executePlan,run,value] at ready
          | some result => simp [executePlan,run,value,write,mark,environment,resultValue_type _ _ _ value]
  | acquire name chain =>
      obtain ⟨_,rfl⟩ := outputs
      cases run : ReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
      | none => simp [executePlan,run] at ready
      | some pair => obtain ⟨machine,key⟩ := pair; simp [executePlan,run,write,mark,environment,typeOf]
  | reacquire name key =>
      obtain ⟨_,rfl⟩ := outputs
      cases run : ReferenceExecution.reacquire s.machine member place principal s.nextRequest key with
      | none => simp [executePlan,run] at ready
      | some machine => simp [executePlan,run,write,mark,environment,typeOf,SourceTypes.typeOf]
  | release name key =>
      obtain ⟨_,rfl⟩ := outputs
      cases run : ReferenceExecution.release s.machine member place principal s.nextRequest key with
      | none => simp [executePlan,run] at ready
      | some machine => simp [executePlan,run,write,mark,environment,typeOf,SourceTypes.typeOf]
  | call name target argument => exact False.elim ((executeCall_shape _ _ _ _ _ _ _ _).2.1 ready)

-- Immediate ready is a phase-specific claim; a started call has no result name
-- yet and must instead satisfy the saved-continuation invariant.
theorem advance_environment (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (env : Environment)
    (typed : StatementTyped p (environment s.values) item.statement env)
    (ready : (advance s member place principal item).status = .ready) :
    environment (advance s member place principal item).state.values = env := by
  have checked := (statement_exact _ _ _ _).mpr typed
  cases waiting : s.waiting with
  | some saved => simp [advance,waiting] at ready
  | none =>
      cases compiled : elaborate s.machine.store.core.system.configuration.state s.values item.statement with
      | none => simp [advance,waiting,checked,compiled] at ready
      | some plan =>
          have meaning := (elaborate_exact _ _ _ _).mp compiled
          have outputs : Produces (environment s.values) (planOutput plan) env := by
            rw [output_corresponds _ _ _ _ meaning]; exact typed.2
          have assignment := assignment_corresponds _ _ _ _ _ typed meaning
          have readyPlan : (executePlan s member place principal item plan).status = .ready := by
            simpa [advance,waiting,checked,compiled] using ready
          simpa [advance,waiting,checked,compiled] using executePlan_environment _ _ _ _ _ _ _ outputs assignment readyPlan

theorem executePlan_waiting (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (plan : Plan)
    (waiting : (executePlan s member place principal item plan).status = .waiting) :
    (executePlan s member place principal item plan).state.values = s.values ∧
      ∃ saved, (executePlan s member place principal item plan).state.waiting = some saved ∧
        planOutput plan = some (saved.name,.plain (.integer false)) := by
  cases plan with
  | pureValue name value assign => simp [executePlan] at waiting
  | control name command kind =>
      cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest (withOwner principal command) with
      | none => simp [executePlan,run] at waiting
      | some pair =>
          obtain ⟨machine,created⟩ := pair
          cases value : resultValue kind created <;> simp [executePlan,run,value] at waiting
  | acquire name chain =>
      cases run : ReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
      | none => simp [executePlan,run] at waiting
      | some pair => obtain ⟨machine,key⟩ := pair; simp [executePlan,run] at waiting
  | reacquire name key =>
      cases run : ReferenceExecution.reacquire s.machine member place principal s.nextRequest key <;>
        simp [executePlan,run] at waiting
  | release name key =>
      cases run : ReferenceExecution.release s.machine member place principal s.nextRequest key <;>
        simp [executePlan,run] at waiting
  | call name target argument =>
      have shape := executeCall_shape s member place principal item name target argument
      obtain ⟨saved,savedAt,nameAt⟩ := shape.2.2.1 waiting
      exact ⟨shape.1,saved,savedAt,by simp [planOutput,nameAt]⟩

theorem executePlan_nonready (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (plan : Plan)
    (nonready : (executePlan s member place principal item plan).status ≠ .ready) :
    (executePlan s member place principal item plan).state.values = s.values := by
  cases plan with
  | pureValue name value assign => simp [executePlan] at nonready
  | control name command kind =>
      cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest (withOwner principal command) with
      | none => simp [executePlan,run]
      | some pair =>
          obtain ⟨machine,created⟩ := pair
          cases value : resultValue kind created with
          | none => simp [executePlan,run,value,mark]
          | some result => simp [executePlan,run,value] at nonready
  | acquire name chain =>
      cases run : ReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
      | none => simp [executePlan,run]
      | some pair => obtain ⟨machine,key⟩ := pair; simp [executePlan,run] at nonready
  | reacquire name key =>
      cases run : ReferenceExecution.reacquire s.machine member place principal s.nextRequest key with
      | none => simp [executePlan,run]
      | some machine => simp [executePlan,run] at nonready
  | release name key =>
      cases run : ReferenceExecution.release s.machine member place principal s.nextRequest key with
      | none => simp [executePlan,run]
      | some machine => simp [executePlan,run] at nonready
  | call name target argument => exact (executeCall_shape _ _ _ _ _ _ _ _).1

theorem executePlan_nonwaiting (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (plan : Plan)
    (nonwaiting : (executePlan s member place principal item plan).status ≠ .waiting) :
    (executePlan s member place principal item plan).state.waiting = s.waiting := by
  cases plan with
  | pureValue name value assign => rfl
  | control name command kind =>
      cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest (withOwner principal command) with
      | none => simp [executePlan,run]
      | some pair =>
          obtain ⟨machine,created⟩ := pair
          cases value : resultValue kind created <;> simp [executePlan,run,value,mark,write]
  | acquire name chain =>
      cases run : ReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
      | none => simp [executePlan,run]
      | some pair => obtain ⟨machine,key⟩ := pair; simp [executePlan,run,mark,write]
  | reacquire name key =>
      cases run : ReferenceExecution.reacquire s.machine member place principal s.nextRequest key <;>
        simp [executePlan,run,mark,write]
  | release name key =>
      cases run : ReferenceExecution.release s.machine member place principal s.nextRequest key <;>
        simp [executePlan,run,mark,write]
  | call name target argument => exact (executeCall_shape _ _ _ _ _ _ _ _).2.2.2 nonwaiting

theorem advance_waiting (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (env : Environment) (typed : StatementTyped p (environment s.values) item.statement env)
    (waiting : (advance s member place principal item).status = .waiting) :
    (advance s member place principal item).state.values = s.values ∧
      ∃ saved, (advance s member place principal item).state.waiting = some saved ∧
        lookupType (environment s.values) saved.name = none ∧
        env = (saved.name,.plain (.integer false)) :: environment s.values := by
  have checked := (statement_exact _ _ _ _).mpr typed
  cases idle : s.waiting with
  | some saved => simp [advance,idle] at waiting
  | none =>
      cases compiled : elaborate s.machine.store.core.system.configuration.state s.values item.statement with
      | none => simp [advance,idle,checked,compiled] at waiting
      | some plan =>
          have atPlan : (executePlan s member place principal item plan).status = .waiting := by
            simpa [advance,idle,checked,compiled] using waiting
          obtain ⟨same,saved,savedAt,outputAt⟩ := executePlan_waiting _ _ _ _ _ _ atPlan
          have meaning := (elaborate_exact _ _ _ _).mp compiled
          have outputs := typed.2
          rw [← output_corresponds _ _ _ _ meaning,outputAt] at outputs
          simpa [advance,idle,checked,compiled] using And.intro same ⟨saved,savedAt,outputs⟩

theorem advance_nonready (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (nonready : (advance s member place principal item).status ≠ .ready) :
    (advance s member place principal item).state.values = s.values := by
  unfold advance at *
  split
  · rfl
  · rename_i idle
    cases checked : checkStatement p (environment s.values) item.statement with
    | none => rfl
    | some env =>
        cases compiled : elaborate s.machine.store.core.system.configuration.state s.values item.statement with
        | none => rfl
        | some plan =>
            apply executePlan_nonready
            simpa [idle,checked,compiled] using nonready

theorem advance_nonwaiting (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (nonwaiting : (advance s member place principal item).status ≠ .waiting) :
    (advance s member place principal item).state.waiting = s.waiting := by
  unfold advance at *
  split
  · rfl
  · rename_i idle
    cases checked : checkStatement p (environment s.values) item.statement with
    | none => rfl
    | some env =>
        cases compiled : elaborate s.machine.store.core.system.configuration.state s.values item.statement with
        | none => rfl
        | some plan =>
            apply executePlan_nonwaiting
            simpa [idle,checked,compiled] using nonwaiting

-- The full output environment is installed only at successful completion of
-- this exact saved invocation. Interleavings are covered separately by the
-- source continuation invariant, not by assuming atomic source execution.
theorem invoke_environment (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (env : Environment) (typed : StatementTyped p (environment s.values) item.statement env)
    (waiting : (advance s member place principal item).status = .waiting)
    (ready : (complete (advance s member place principal item).state).status = .ready) :
    environment (complete (advance s member place principal item).state).state.values = env := by
  obtain ⟨same,saved,savedAt,_,envAt⟩ := advance_waiting _ _ _ _ _ _ typed waiting
  obtain ⟨entry,value,entryAt,_,_,_,valuesAt,_⟩ := complete_write_meaning
    (advance s member place principal item).state (complete (advance s member place principal item).state).state
    (by cases h : complete (advance s member place principal item).state; simp_all)
  have eq : entry = saved := by rw [savedAt] at entryAt; exact (Option.some.inj entryAt).symm
  subst entry
  simp [valuesAt,environment,typeOf,SourceTypes.typeOf,same,envAt]

def Unique (s : State p a) : Prop := (environment s.values |>.map Prod.fst).Nodup
def WaitingFresh (s : State p a) : Prop :=
  ∀ saved, s.waiting = some saved → lookupType (environment s.values) saved.name = none

theorem lookup_none (env : Environment) (name : String) :
    lookupType env name = none ↔ name ∉ env.map Prod.fst := by
  induction env with
  | nil => simp [lookupType]
  | cons row rest ih =>
      by_cases key : row.1 = name
      · simp [lookupType,key]
      · have left : lookupType (row :: rest) name = lookupType rest name := by simp [lookupType,key]
        rw [left]
        simpa only [List.map_cons,List.mem_cons,not_or,ne_eq,show name ≠ row.1 from Ne.symm key,not_false_eq_true,true_and] using ih

theorem produces_unique (env next : Environment) (binding : Option (String × Ty))
    (unique : (env.map Prod.fst).Nodup) (outputs : Produces env binding next) :
    (next.map Prod.fst).Nodup := by
  cases binding with
  | none => subst next; exact unique
  | some row =>
      obtain ⟨fresh,rfl⟩ := outputs
      exact List.nodup_cons.mpr ⟨(lookup_none _ _).mp fresh,unique⟩

theorem initial_names (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy) :
    Unique (initial (p:=p) realm view policy) ∧ WaitingFresh (initial (p:=p) realm view policy) := by
  simp [Unique,WaitingFresh,initial,environment]

theorem advance_unique (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (unique : Unique s) : Unique (advance s member place principal item).state := by
  by_cases ready : (advance s member place principal item).status = .ready
  · cases idle : s.waiting with
    | some saved => simp [advance,idle] at ready
    | none =>
        cases checked : checkStatement p (environment s.values) item.statement with
        | none => simp [advance,idle,checked] at ready
        | some env =>
            have typed := (statement_exact _ _ _ _).mp checked
            unfold Unique
            rw [advance_environment _ _ _ _ _ _ typed ready]
            exact produces_unique _ _ _ unique typed.2
  · simpa [Unique,advance_nonready _ _ _ _ _ ready] using unique

theorem advance_fresh (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (fresh : WaitingFresh s) : WaitingFresh (advance s member place principal item).state := by
  by_cases waiting : (advance s member place principal item).status = .waiting
  · cases idle : s.waiting with
    | some saved => simp [advance,idle] at waiting
    | none =>
        cases checked : checkStatement p (environment s.values) item.statement with
        | none => simp [advance,idle,checked] at waiting
        | some env =>
            have typed := (statement_exact _ _ _ _).mp checked
            obtain ⟨same,saved,atSaved,freshName,_⟩ := advance_waiting _ _ _ _ _ _ typed waiting
            intro other atOther
            have equal : other = saved := by rw [atSaved] at atOther; exact (Option.some.inj atOther).symm
            subst other
            simpa only [same] using freshName
  · have sameWaiting := advance_nonwaiting s member place principal item waiting
    cases idle : s.waiting with
    | none => simp [WaitingFresh,sameWaiting,idle]
    | some saved => simpa [advance,idle] using fresh

theorem complete_unique (s : State p a) (unique : Unique s) (fresh : WaitingFresh s) : Unique (complete s).state := by
  cases waiting : s.waiting with
  | none => simpa [complete,waiting] using unique
  | some saved =>
      cases resumed : ReferenceExecution.resume s.machine saved.entry with
      | none => simpa [complete,waiting,resumed] using unique
      | some pair =>
          obtain ⟨machine,value⟩ := pair
          have absent := (lookup_none _ _).mp (fresh saved waiting)
          simpa [Unique,complete,waiting,resumed,write,mark,environment] using List.nodup_cons.mpr ⟨absent,unique⟩

theorem complete_fresh (s : State p a) (fresh : WaitingFresh s) : WaitingFresh (complete s).state := by
  cases waiting : s.waiting with
  | none => simpa [complete,waiting] using fresh
  | some saved =>
      cases resumed : ReferenceExecution.resume s.machine saved.entry with
      | none => simpa [complete,waiting,resumed] using fresh
      | some pair => obtain ⟨machine,value⟩ := pair; simp [WaitingFresh,complete,waiting,resumed]

theorem head_names (s : State p a) (view : WorldProjection.AuthorityView a)
    (unique : Unique s) (fresh : WaitingFresh s) : Unique (authorityHead s view) ∧ WaitingFresh (authorityHead s view) :=
  ⟨unique,fresh⟩

theorem controlInput_names (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) (result : State p a × Option Nat) (unique : Unique s) (fresh : WaitingFresh s)
    (accepted : controlInput s member place principal raw = some result) : Unique result.1 ∧ WaitingFresh result.1 := by
  obtain ⟨_,sameWaiting,sameValues,_⟩ := controlInput_projects _ _ _ _ _ _ accepted
  simpa only [Unique,WaitingFresh,sameWaiting,sameValues] using And.intro unique fresh

theorem cancel_names (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (unique : Unique s) (fresh : WaitingFresh s) :
    Unique (cancel s member place principal).state ∧ WaitingFresh (cancel s member place principal).state := by
  obtain ⟨values,_,waiting⟩ := cancel_values_writes s member place principal
  refine ⟨by simpa only [Unique,values] using unique,?_⟩
  rcases waiting with unchanged | cleared
  · simpa only [WaitingFresh,values,unchanged] using fresh
  · simp [WaitingFresh,cleared]

theorem run_environment (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (program : List Located) (env : Environment) (typed : ProgramTyped p (environment s.values) program env)
    (ready : (run s member place principal program).status = .ready) :
    environment (run s member place principal program).state.values = env := by
  induction program generalizing s with
  | nil => cases typed; rfl
  | cons item rest ih =>
      cases typed with
      | cons head tail =>
          simp only [run] at *
          cases advanced : (advance s member place principal item).status with
          | failed reason => simp [advanced] at ready
          | ready =>
              have envAt := advance_environment _ _ _ _ _ _ head advanced
              simpa [advanced] using ih (advance s member place principal item).state
                (by simpa [envAt] using tail) (by simpa [advanced] using ready)
          | waiting =>
              cases completed : (complete (advance s member place principal item).state).status with
              | failed reason => simp [advanced,completed] at ready
              | waiting => simp [advanced,completed] at ready
              | ready =>
                  have envAt := invoke_environment _ _ _ _ _ _ head advanced completed
                  simpa [advanced,completed] using ih (complete (advance s member place principal item).state).state
                    (by simpa [envAt] using tail) (by simpa [advanced,completed] using ready)

#print axioms lookup_none
#print axioms produces_unique
#print axioms advance_unique
#print axioms advance_fresh
#print axioms complete_unique
#print axioms complete_fresh
#print axioms cancel_names
#print axioms controlInput_names
#print axioms run_environment
#print axioms executePlan_waiting
#print axioms executePlan_nonready
#print axioms executePlan_nonwaiting
#print axioms advance_waiting
#print axioms advance_nonready
#print axioms advance_nonwaiting
#print axioms invoke_environment
#print axioms output_corresponds
#print axioms mutable_projection
#print axioms assignment_corresponds
#print axioms update_environment
#print axioms executePlan_environment
#print axioms advance_environment
end MirroreaProofFirst.ReferenceSourceTyping
