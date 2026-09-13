import MirroreaProofFirstReferenceSourceTrace
import MirroreaProofFirstSourceLocators

namespace MirroreaProofFirst.ReferenceSourceLocators
open ReferenceSourceData ReferenceSource

-- A locator denotes a retained record, not present usability/authority.
-- Released reference aliases denote tombstones; retired instances remain typed.
def ValueValid (m : ReferenceExecution.Machine p a) : Value → Prop
  | .plain value => SourceLocators.ValueValid m.store.core.system.configuration value
  | .reference key => key < m.store.bindings.length
def ValuesValid (m : ReferenceExecution.Machine p a) (values : Values) : Prop :=
  ∀ row ∈ values, ValueValid m row.2

theorem slots_length (before after : List (Option ReferenceOwner.Binding))
    (slots : ∀ key : Nat, ReferenceTrace.SlotProgress before[key]? after[key]?) : before.length ≤ after.length := by
  apply Classical.byContradiction
  intro smaller
  have bound : after.length < before.length := by omega
  have nextNone : after[after.length]? = none := List.getElem?_eq_none (by omega)
  have progress := slots after.length
  rw [nextNone] at progress
  cases old : before[after.length]? with
  | none => have missing := List.getElem?_eq_none_iff.mp old; omega
  | some row => rw [old] at progress; cases row <;> cases progress

theorem transition_catalog (m next : ReferenceExecution.Machine p a)
    (valid : ReferenceExecution.Invariant m) (step : ReferenceExecution.Transition m next) :
    CatalogHistory.Extends m.store.core.system.configuration next.store.core.system.configuration := by
  have catalog := valid.1.1.2.1
  cases step with
  | management run =>
      exact (CatalogHistory.transition_catalog _ _ catalog (.management (ReferenceExecution.manage_core _ _ _ _ _ _ _ run))).2
  | authority => exact CatalogHistory.extends_refl _
  | startPlain run =>
      exact (CatalogHistory.transition_catalog _ _ catalog (.request (ReferenceExecution.startPlain_core _ _ _ _ _ _ _ _ run))).2
  | startReference run =>
      obtain ⟨target,coreRun⟩ := ReferenceExecution.startReference_core _ _ _ _ _ _ _ _ run
      exact (CatalogHistory.transition_catalog _ _ catalog (.request coreRun)).2
  | cancellation run =>
      obtain ⟨_,coreRun⟩ := ReferenceExecution.cancel_core _ _ _ _ _ _ _ run
      exact (CatalogHistory.transition_catalog _ _ catalog (.cancellation coreRun)).2
  | finish run =>
      exact (CatalogHistory.transition_catalog _ _ catalog (.result (ReferenceExecution.finish_core_run _ _ _ _ run))).2
  | acquire run => rw [ReferenceExecution.acquire_core _ _ _ _ _ _ _ run]; exact CatalogHistory.extends_refl _
  | reacquire run => rw [ReferenceExecution.reacquire_core _ _ _ _ _ _ _ run]; exact CatalogHistory.extends_refl _
  | release run => rw [ReferenceExecution.release_core _ _ _ _ _ _ _ run]; exact CatalogHistory.extends_refl _
  | @normalize member place principal id key =>
      cases consumed : (ReferenceExecution.normalize m member place principal id key).consumed with
      | false => rw [ReferenceExecution.normalize_unconsumed _ _ _ _ _ _ consumed]; exact CatalogHistory.extends_refl _
      | true => rw [ReferenceExecution.normalize_consumed_core _ _ _ _ _ _ consumed]; exact CatalogHistory.extends_refl _

theorem reached_growth (m next : ReferenceExecution.Machine p a)
    (valid : ReferenceExecution.Invariant m) (path : ReferenceExecution.Reached m next) :
    CatalogHistory.Extends m.store.core.system.configuration next.store.core.system.configuration ∧
      m.store.bindings.length ≤ next.store.bindings.length := by
  have length := slots_length _ _ (ReferenceExecution.reached_slots _ _ valid path)
  refine ⟨?_,length⟩
  clear length
  induction path with
  | refl => exact CatalogHistory.extends_refl _
  | @step middle next previous step ih =>
      have midValid := ReferenceExecution.reached_preserves _ _ valid previous
      exact CatalogHistory.extends_trans _ _ _ ih (transition_catalog _ _ midValid step)

theorem value_grows (m next : ReferenceExecution.Machine p a) (value : Value)
    (growth : CatalogHistory.Extends m.store.core.system.configuration next.store.core.system.configuration ∧
      m.store.bindings.length ≤ next.store.bindings.length) (valid : ValueValid m value) : ValueValid next value := by
  cases value with
  | reference key => exact Nat.lt_of_lt_of_le valid growth.2
  | plain value =>
      cases value with
      | unit => trivial
      | definition key => exact Nat.lt_of_lt_of_le valid growth.1.1
      | callable key => exact Nat.lt_of_lt_of_le valid growth.1.2.1
      | integer value mutable => exact valid

theorem values_grow (m next : ReferenceExecution.Machine p a) (values : Values)
    (valid : ReferenceExecution.Invariant m) (path : ReferenceExecution.Reached m next)
    (valuesValid : ValuesValid m values) : ValuesValid next values :=
  fun row member => value_grows _ _ _ (reached_growth _ _ valid path) (valuesValid row member)

theorem lookup_valid (m : ReferenceExecution.Machine p a) (values : Values) (name : String) (value : Value)
    (valid : ValuesValid m values) (found : lookup values name = some value) : ValueValid m value := by
  unfold lookup at found
  cases row : values.find? (fun row => row.1 == name) with
  | none => simp [row] at found
  | some entry =>
      have equal : entry.2 = value := by simpa [row] using found
      rw [← equal]
      exact valid entry (List.mem_of_find?_eq_some row)

theorem update_valid (m : ReferenceExecution.Machine p a) (values : Values) (name : String) (value : Value)
    (valid : ValuesValid m values) (newValue : ValueValid m value) : ValuesValid m (updateValue values name value) := by
  induction values with
  | nil => intro row member; cases member
  | cons head rest ih =>
      have tail : ValuesValid m rest := fun row member => valid row (List.mem_cons_of_mem _ member)
      unfold updateValue
      split
      · intro row member
        rcases List.mem_cons.mp member with rfl | old
        · exact newValue
        · exact tail row old
      · intro row member
        rcases List.mem_cons.mp member with rfl | old
        · exact valid row (by simp)
        · exact ih tail row old

def kindOfRaw : CompositionCore.Raw → ResultKind
  | .register .. => .definition | .instantiate .. => .instance | _ => .unit

theorem owner_kind (raw : CompositionCore.Raw) (principal : Nat) :
    kindOfRaw (withOwner principal raw) = kindOfRaw raw := by cases raw <;> rfl

theorem result_expected (cfg : CompositionCore.Config p) (raw : CompositionCore.Raw) :
    resultValue (kindOfRaw raw) (SourceCompletion.expectedCreated cfg raw) =
      some (.plain (SourceLocators.returnedValue cfg raw)) := by cases raw <;> rfl

theorem manage_returned (m : ReferenceExecution.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (raw : CompositionCore.Raw) (result : ReferenceExecution.Machine p a × Option Nat)
    (valid : ReferenceExecution.Invariant m)
    (accepted : ReferenceExecution.manage m member place principal id raw = some result) :
    ValueValid result.1 (.plain (SourceLocators.returnedValue m.store.core.system.configuration raw)) ∧
      result.2 = SourceCompletion.expectedCreated m.store.core.system.configuration raw := by
  have coreRun := ReferenceExecution.manage_core _ _ _ _ _ _ _ accepted
  obtain ⟨_,_,cfg,created,_,ran,equal⟩ := CompositionMachine.manage_parts _ _ _ _ _ _ _ coreRun
  have cfgAt : result.1.store.core.system.configuration = cfg :=
    congrArg (fun pair : CompositionMachine.Machine p a × Option Nat => pair.1.system.configuration) equal
  have createdAt : result.2 = created := congrArg Prod.snd equal
  refine ⟨?_,createdAt.trans (SourceCompletion.run_created _ _ _ valid.1.1.1.1.1 ran)⟩
  change SourceLocators.ValueValid result.1.store.core.system.configuration _
  rw [cfgAt]
  obtain ⟨command,erased,_,outcome⟩ := CatalogHistory.run_parts _ _ _ valid.1.1.1.1.1 ran
  have output := SourceLocators.outcome_value m.store.core.system.configuration.state command
  simpa only [erased,← outcome] using output

theorem acquire_value (m : ReferenceExecution.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (chain : FallbackStatic.Chain) (result : ReferenceExecution.Machine p a × Nat)
    (accepted : ReferenceExecution.acquire m member place principal id chain = some result) :
    ValueValid result.1 (.reference result.2) := by
  unfold ReferenceExecution.acquire at accepted
  cases run : ReferenceMutation.acquire m.store member place principal id chain with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨next,key⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      obtain ⟨keyAt,lengthAt,_⟩ := ReferenceMutation.acquire_old_slots _ _ _ _ _ _ _ run
      change key < next.bindings.length
      have keyEq : key = m.store.bindings.length := keyAt
      have lengthEq : next.bindings.length = m.store.bindings.length + 1 := lengthAt
      omega

def PlanValues (m : ReferenceExecution.Machine p a) : Plan → Prop
  | .pureValue _ value _ => ValueValid m value
  | .control _ raw kind => kind = kindOfRaw raw
  | _ => True

theorem evaluates_range (values : Values) (expression : SourceAuthoring.Expr) (result : Int)
    (evaluated : ReferenceSourceElaboration.Evaluates values expression result) :
    ReferenceSourceElaboration.InRange result := by
  cases expression with
  | integer value => obtain ⟨rfl,range⟩ := evaluated; exact range
  | read name => exact evaluated.2
  | add left right | mul left right => obtain ⟨_,_,_,_,_,range⟩ := evaluated; exact range

theorem plan_values (m : ReferenceExecution.Machine p a) (values : Values) (statement : Statement) (plan : Plan)
    (valid : ValuesValid m values)
    (meaning : ReferenceSourceElaboration.PlanFor m.store.core.system.configuration.state values statement plan) :
    PlanValues m plan := by
  cases statement with
  | plain statement =>
      cases statement with
      | localValue name mutable expression =>
          obtain ⟨value,evaluated,rfl⟩ := meaning
          exact evaluates_range _ _ _ evaluated
      | assign name expression =>
          obtain ⟨value,evaluated,rfl⟩ := meaning
          exact evaluates_range _ _ _ evaluated
      | _ => simp only [ReferenceSourceElaboration.PlanFor,ReferenceSourceElaboration.PlainFor] at meaning
             grind [PlanValues,kindOfRaw]
  | acquire name chain => obtain ⟨_,_,_,rfl⟩ := meaning; trivial
  | alias name target => obtain ⟨key,found,rfl⟩ := meaning; exact lookup_valid _ _ _ _ valid found
  | reacquire name target => obtain ⟨_,_,rfl⟩ := meaning; trivial
  | release name target => obtain ⟨_,_,rfl⟩ := meaning; trivial

theorem write_valid (s : State p a) (site : Site) (beforeCount : Nat) (deps : List Read)
    (name : String) (value : Value) (assign : Bool)
    (valid : ValuesValid s.machine s.values) (newValue : ValueValid s.machine value) :
    ValuesValid (write s site beforeCount deps name value assign).machine
      (write s site beforeCount deps name value assign).values := by
  cases assign with
  | false =>
      intro row member
      rcases List.mem_cons.mp member with rfl | old
      · exact newValue
      · exact valid row old
  | true => exact update_valid _ _ _ _ valid newValue

theorem executePlan_values (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (plan : Plan) (machineValid : ReferenceExecution.Invariant s.machine)
    (valid : ValuesValid s.machine s.values) (planValid : PlanValues s.machine plan) :
    ValuesValid (executePlan s member place principal item plan).state.machine
      (executePlan s member place principal item plan).state.values := by
  have growth := values_grow _ _ _ machineValid (executePlan_projects s member place principal item plan) valid
  by_cases ready : (executePlan s member place principal item plan).status = .ready
  · cases plan with
    | pureValue name value assign => exact write_valid _ _ _ _ _ _ _ valid planValid
    | control name command kind =>
        cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest (withOwner principal command) with
        | none => simp [executePlan,run] at ready
        | some pair =>
            obtain ⟨machine,created⟩ := pair
            have returned := manage_returned _ _ _ _ _ _ _ machineValid run
            dsimp only at returned
            have kindAt : kind = kindOfRaw (withOwner principal command) := planValid.trans (owner_kind _ _).symm
            have valueAt : resultValue kind created = some (.plain (SourceLocators.returnedValue s.machine.store.core.system.configuration (withOwner principal command))) := by
              rw [kindAt,returned.2]; exact result_expected _ _
            simp only [executePlan,run,valueAt] at growth ⊢
            exact write_valid _ _ _ _ _ _ _ growth returned.1
    | acquire name chain =>
        cases run : ReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
        | none => simp [executePlan,run] at ready
        | some pair =>
            obtain ⟨machine,key⟩ := pair
            simp only [executePlan,run] at growth ⊢
            exact write_valid _ _ _ _ _ _ _ growth (acquire_value _ _ _ _ _ _ _ run)
    | reacquire name key =>
        cases run : ReferenceExecution.reacquire s.machine member place principal s.nextRequest key with
        | none => simp [executePlan,run] at ready
        | some machine =>
            simp only [executePlan,run] at growth ⊢
            exact write_valid _ _ _ _ _ _ _ growth trivial
    | release name key =>
        cases run : ReferenceExecution.release s.machine member place principal s.nextRequest key with
        | none => simp [executePlan,run] at ready
        | some machine =>
            simp only [executePlan,run] at growth ⊢
            exact write_valid _ _ _ _ _ _ _ growth trivial
    | call name target argument => exact False.elim ((executeCall_shape _ _ _ _ _ _ _ _).2.1 ready)
  · rw [ReferenceSourceTyping.executePlan_nonready _ _ _ _ _ _ ready]
    exact growth

theorem advance_values (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (machineValid : ReferenceExecution.Invariant s.machine)
    (valid : ValuesValid s.machine s.values) :
    ValuesValid (advance s member place principal item).state.machine
      (advance s member place principal item).state.values := by
  unfold advance
  split
  · exact valid
  · cases typed : checkStatement p (environment s.values) item.statement with
    | none => exact valid
    | some env =>
        cases compiled : elaborate s.machine.store.core.system.configuration.state s.values item.statement with
        | none => exact valid
        | some plan =>
            exact executePlan_values _ _ _ _ _ _ machineValid valid
              (plan_values _ _ _ _ valid ((ReferenceSourceElaboration.elaborate_exact _ _ _ _).mp compiled))

theorem complete_values (s : State p a) (machineValid : ReferenceExecution.Invariant s.machine)
    (valid : ValuesValid s.machine s.values) :
    ValuesValid (complete s).state.machine (complete s).state.values := by
  have growth := values_grow _ _ _ machineValid (complete_projects s) valid
  by_cases ready : (complete s).status = .ready
  · have completed : complete s = ⟨(complete s).state,.ready⟩ := by
      cases h : complete s; simp_all
    obtain ⟨saved,value,_,_,meaning,_,valuesAt,_⟩ := complete_write_meaning _ _ completed
    rw [valuesAt]
    intro row member
    rcases List.mem_cons.mp member with rfl | old
    · exact (ContractExport.CheckedArithmetic.denotes_math meaning.2.2).2.2
    · exact growth row old
  · cases waiting : s.waiting with
    | none => simpa [complete,waiting] using valid
    | some saved =>
        cases resumed : ReferenceExecution.resume s.machine saved.entry with
        | none => simpa [complete,waiting,resumed] using valid
        | some pair => obtain ⟨machine,value⟩ := pair; simp [complete,waiting,resumed] at ready

theorem controlInput_values (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) (result : State p a × Option Nat)
    (machineValid : ReferenceExecution.Invariant s.machine) (valid : ValuesValid s.machine s.values)
    (accepted : controlInput s member place principal raw = some result) :
    ValuesValid result.1.machine result.1.values := by
  have growth := values_grow _ _ _ machineValid (controlInput_projects _ _ _ _ _ _ accepted).1 valid
  unfold controlInput at accepted
  cases managed : ReferenceExecution.manage s.machine member place principal s.nextRequest raw with
  | none => simp [managed] at accepted
  | some pair =>
      obtain ⟨machine,created⟩ := pair
      simp only [managed,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      exact growth

theorem step_values (s next : State p a) (machineValid : ReferenceExecution.Invariant s.machine)
    (valid : ValuesValid s.machine s.values) (step : ReferenceSourceTrace.Step s next) :
    ValuesValid next.machine next.values := by
  cases step with
  | advance => exact advance_values _ _ _ _ _ machineValid valid
  | @cancellation member place principal =>
      rw [(cancel_values_writes s member place principal).1]
      exact values_grow _ _ _ machineValid (cancel_projects _ _ _ _) valid
  | complete => exact complete_values _ machineValid valid
  | head => exact values_grow _ _ _ machineValid (head_projects _ _) valid
  | control run => exact controlInput_values _ _ _ _ _ _ machineValid valid run

-- A finite actual source execution cannot create dangling source locators or
-- an out-of-range integer. This does not confer liveness or authority on a
-- retained target or on a released reference's tombstone.
theorem rooted_values (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (s : State p a)
    (path : ReferenceSourceTrace.Rooted realm view policy s) : ValuesValid s.machine s.values := by
  induction path with
  | refl => intro row member; cases member
  | step previous step ih => exact step_values _ _ (ReferenceSourceTrace.rooted_invariants _ _ _ _ previous).1 ih step

#print axioms rooted_values
#print axioms step_values
#print axioms complete_values
#print axioms controlInput_values
#print axioms advance_values
#print axioms executePlan_values
#print axioms write_valid
#print axioms manage_returned
#print axioms acquire_value
#print axioms plan_values
#print axioms slots_length
#print axioms transition_catalog
#print axioms reached_growth
#print axioms values_grow
#print axioms lookup_valid
#print axioms update_valid
end MirroreaProofFirst.ReferenceSourceLocators
