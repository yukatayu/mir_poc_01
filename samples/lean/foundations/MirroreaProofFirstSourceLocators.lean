import MirroreaProofFirstSourceAllocation
import MirroreaProofFirstSourceHistory

namespace MirroreaProofFirst.SourceLocators
open SourceAuthoring CompositionCore

-- Existence of referenced records is separate from current usability/authority.
-- Retired instances retain their records; no bound below asserts they are live.
def ValueValid (cfg : Config p) : Value → Prop
  | .unit => True
  | .definition key => key < cfg.definitions
  | .callable key => key < cfg.count
  | .integer value _ => InstancePrograms.Machine.lo ≤ value ∧ value ≤ InstancePrograms.Machine.hi

def ValuesValid (cfg : Config p) (values : List (String × Value)) : Prop :=
  ∀ row ∈ values, ValueValid cfg row.2

def Invariant (s : State p a) : Prop := CompositionMachine.Invariant s.machine ∧
  ValuesValid s.machine.system.configuration s.values

theorem values_extend (before after : Config p) (values : List (String × Value))
    (extension : CatalogHistory.Extends before after) (valid : ValuesValid before values) : ValuesValid after values := by
  intro row member
  have previous := valid row member
  cases h : row.2 with
  | unit => trivial
  | definition key => exact Nat.lt_of_lt_of_le (by simpa [ValueValid,h] using previous) extension.1
  | callable key => exact Nat.lt_of_lt_of_le (by simpa [ValueValid,h] using previous) extension.2.1
  | integer value mutable => simpa [ValueValid,h] using previous

def returnedValue (cfg : Config p) : Raw → Value
  | .register .. => .definition cfg.definitions
  | .instantiate .. => .callable cfg.count
  | _ => .unit

theorem outcome_value (s : InstanceState.State d p n) (cmd : Command d p n) :
    ValueValid (outcome s cmd).1 (returnedValue (config s) (erase cmd)) := by
  cases cmd <;> simp [ValueValid,outcome,returnedValue,config,erase]

theorem control_facts (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : Raw) (next : State p a × Option Nat) (valid : Invariant s)
    (accepted : SourceAuthoring.control s member place principal raw = some next) :
    Invariant next.1 ∧ ValueValid next.1.machine.system.configuration
      (returnedValue s.machine.system.configuration raw) := by
  unfold SourceAuthoring.control at accepted
  cases h : CompositionMachine.manage s.machine member place principal s.nextRequest raw with
  | none => simp [h] at accepted
  | some managed =>
      rw [h] at accepted; cases accepted
      have machineValid := CompositionMachine.manage_preserves _ _ _ _ _ _ _ valid.1 h
      obtain ⟨_,_,cfg,created,_,ran,eq⟩ := CompositionMachine.manage_parts _ _ _ _ _ _ _ h
      have extension := CatalogHistory.run_extends _ _ _ valid.1.1.1 ran
      have valuesValid := values_extend _ _ s.values extension valid.2
      rw [eq] at machineValid ⊢
      refine ⟨⟨machineValid,valuesValid⟩,?_⟩
      obtain ⟨cmd,eraseEq,_,outcomeEq⟩ := CatalogHistory.run_parts _ _ _ valid.1.1.1 ran
      have output := outcome_value s.machine.system.configuration.state cmd
      simpa only [eraseEq,← outcomeEq] using output

theorem bind_preserves (s : State p a) (name : String) (value : Value) (next : State p a)
    (valid : Invariant s) (newValue : ValueValid s.machine.system.configuration value)
    (bound : bind s name value = some next) : Invariant next := by
  unfold SourceAuthoring.bind at bound
  split at bound
  · cases bound
  · cases bound
    refine ⟨valid.1,?_⟩
    intro row member
    rcases List.mem_cons.mp member with rfl | old
    · exact newValue
    · exact valid.2 row old

theorem update_valid (cfg : Config p) (values : List (String × Value)) (name : String) (value : Value)
    (valid : ValuesValid cfg values) (newValue : ValueValid cfg value) : ValuesValid cfg (updateValue values name value) := by
  induction values with
  | nil => intro row member; cases member
  | cons head rest ih =>
      have tail : ValuesValid cfg rest := fun row member => valid row (List.mem_cons_of_mem _ member)
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

theorem bounded_value (value result : Int) (accepted : bounded value = some result) :
    InstancePrograms.Machine.lo ≤ result ∧ result ≤ InstancePrograms.Machine.hi := by
  unfold bounded at accepted
  split at accepted
  · cases accepted; assumption
  · cases accepted

theorem evaluated_value (values : List (String × Value)) (expression : Expr) (result : Int)
    (accepted : SourceAuthoring.eval values expression = some result) :
    InstancePrograms.Machine.lo ≤ result ∧ result ≤ InstancePrograms.Machine.hi := by
  cases expression with
  | integer value => exact bounded_value _ _ accepted
  | read name =>
      simp only [SourceAuthoring.eval] at accepted
      split at accepted
      · exact bounded_value _ _ accepted
      · cases accepted
  | add left right | mul left right =>
      simp only [SourceAuthoring.eval] at accepted
      cases hl : SourceAuthoring.eval values left <;> cases hr : SourceAuthoring.eval values right <;>
        simp only [hl,hr,Option.bind_eq_bind,Option.bind_none,Option.bind_some] at accepted
      all_goals try cases accepted
      exact bounded_value _ _ accepted

theorem resume_facts (m : CompositionMachine.Machine p a) (t : InvocationBoundary.Ticket)
    (next : CompositionMachine.Machine p a × Int)
    (accepted : CompositionMachine.resume m t = some next) :
    next.1.system.configuration = m.system.configuration ∧
      InstancePrograms.Machine.lo ≤ next.2 ∧ next.2 ≤ InstancePrograms.Machine.hi := by
  unfold CompositionMachine.resume at accepted
  cases he : InvocationBoundary.execute t with
  | none => simp [he] at accepted
  | some value =>
      simp only [he,Option.bind_eq_bind,Option.bind_some] at accepted
      cases hf : CompositionMachine.finish m t value with
      | none => simp [hf] at accepted
      | some completed =>
          rw [hf] at accepted; cases accepted
          obtain ⟨_,_,checked,rfl⟩ := CompositionMachine.finish_parts _ _ _ _ hf
          have result := (InvocationBoundary.result_sound _ _ _ _ checked).2.2
          exact ⟨rfl,(ContractExport.CheckedArithmetic.denotes_math result).2.2⟩


theorem control_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : Raw) (next : State p a × Option Nat) (valid : Invariant s)
    (accepted : SourceAuthoring.control s member place principal raw = some next) : Invariant next.1 :=
  (control_facts _ _ _ _ _ _ valid accepted).1

theorem invocation_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal key : Nat)
    (arg : Int) (started : CompositionMachine.Machine p a × InvocationBoundary.Ticket)
    (completed : CompositionMachine.Machine p a × Int) (valid : Invariant s)
    (hs : CompositionMachine.start s.machine member place principal s.nextRequest key arg = some started)
    (hr : CompositionMachine.resume started.1 started.2 = some completed) :
    Invariant {s with machine := completed.1,nextRequest := s.nextRequest+1} := by
  have before : started.1.system.configuration = s.machine.system.configuration := by
    rw [(CompositionMachine.start_parts _ _ _ _ _ _ _ _ hs).2.2]; rfl
  have after := (resume_facts _ _ _ hr).1
  refine ⟨SourcePreservation.resume_preserves _ _ _
    (CompositionMachine.start_preserves _ _ _ _ _ _ _ _ valid.1 hs) hr,?_⟩
  simpa only [after,before] using valid.2

theorem step_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (statement : Statement) (next : State p a) (valid : Invariant s)
    (accepted : step s member place principal statement = some next) : Invariant next := by
  cases statement with
  | register name definition predecessor =>
      simp only [step] at accepted
      cases hp : optional (definitionKey s.values) predecessor with
      | none => simp [hp] at accepted
      | some previous =>
          simp only [hp,Option.bind_eq_bind,Option.bind_some] at accepted
          cases hc : control s member place principal (.register definition previous) with
          | none => simp [hc] at accepted
          | some result =>
              rw [hc] at accepted
              obtain ⟨state,created⟩ := result
              cases hi : created with
              | none => simp [hi] at accepted
              | some key =>
                  have created := SourceCompletion.control_created _ _ _ _ _ _ valid.1 hc
                  have keyEq : key = s.machine.system.configuration.definitions := by
                    simpa [hi,SourceCompletion.expectedCreated] using created
                  exact bind_preserves state name (.definition key) next (control_preserves _ _ _ _ _ _ valid hc)
                    (by simpa [returnedValue,keyEq] using (control_facts _ _ _ _ _ _ valid hc).2)
                    (by simpa [hi] using accepted)
  | instantiate name definition places parent support =>
      simp only [step] at accepted
      cases hd : definitionKey s.values definition with
      | none => simp [hd] at accepted
      | some key =>
          simp only [hd,Option.bind_eq_bind,Option.bind_some] at accepted
          cases hp : optional (instanceKey s.values) parent with
          | none => simp [hp] at accepted
          | some parent =>
              simp only [hp,Option.bind_eq_bind,Option.bind_some] at accepted
              cases hs : dependencies s.values support with
              | none => simp [hs] at accepted
              | some support =>
                  simp only [hs,Option.bind_eq_bind,Option.bind_some] at accepted
                  cases hc : control s member place principal (.instantiate key principal places parent support) with
                  | none => simp [hc] at accepted
                  | some result =>
                      rw [hc] at accepted
                      obtain ⟨state,created⟩ := result
                      cases hi : created with
                      | none => simp [hi] at accepted
                      | some key =>
                          have created := SourceCompletion.control_created _ _ _ _ _ _ valid.1 hc
                          have keyEq : key = s.machine.system.configuration.count := by
                            simpa [hi,SourceCompletion.expectedCreated] using created
                          exact bind_preserves state name (.callable key) next (control_preserves _ _ _ _ _ _ valid hc)
                            (by simpa [returnedValue,keyEq] using (control_facts _ _ _ _ _ _ valid hc).2)
                            (by simpa [hi] using accepted)
  | retire out name =>
      simp only [step] at accepted
      cases hk : instanceKey s.values name with
      | none => simp [hk] at accepted
      | some key =>
          simp only [hk,Option.bind_eq_bind,Option.bind_some] at accepted
          cases hc : control s member place principal (.retire key) with
          | none => simp [hc] at accepted
          | some result =>
              exact bind_preserves result.1 out .unit next (control_preserves _ _ _ _ _ _ valid hc) (by trivial) (by simpa [hc] using accepted)
  | reparent out name parent =>
      simp only [step] at accepted
      cases hk : instanceKey s.values name with
      | none => simp [hk] at accepted
      | some key =>
          simp only [hk,Option.bind_eq_bind,Option.bind_some] at accepted
          cases hp : optional (instanceKey s.values) parent with
          | none => simp [hp] at accepted
          | some target =>
              simp only [hp,Option.bind_eq_bind,Option.bind_some] at accepted
              cases hc : control s member place principal (.reparent key target) with
              | none => simp [hc] at accepted
              | some result =>
                  exact bind_preserves result.1 out .unit next (control_preserves _ _ _ _ _ _ valid hc) (by trivial) (by simpa [hc] using accepted)
  | replace out name definition =>
      simp only [step] at accepted
      cases hk : instanceKey s.values name with
      | none => simp [hk] at accepted
      | some key =>
          simp only [hk,Option.bind_eq_bind,Option.bind_some] at accepted
          cases hd : definitionKey s.values definition with
          | none => simp [hd] at accepted
          | some code =>
              simp only [hd,Option.bind_eq_bind,Option.bind_some] at accepted
              cases hc : control s member place principal (.replace key code) with
              | none => simp [hc] at accepted
              | some result =>
                  exact bind_preserves result.1 out .unit next (control_preserves _ _ _ _ _ _ valid hc) (by trivial) (by simpa [hc] using accepted)
  | leave out locus =>
      simp only [step] at accepted
      cases hc : control s member place principal (.leave locus) with
      | none => simp [hc] at accepted
      | some result => exact bind_preserves result.1 out .unit next (control_preserves _ _ _ _ _ _ valid hc) (by trivial) (by simpa [hc] using accepted)
  | join out locus =>
      simp only [step] at accepted
      cases hc : control s member place principal (.join locus) with
      | none => simp [hc] at accepted
      | some result => exact bind_preserves result.1 out .unit next (control_preserves _ _ _ _ _ _ valid hc) (by trivial) (by simpa [hc] using accepted)
  | localValue name mutable expression =>
      simp only [step] at accepted
      cases hv : eval s.values expression with
      | none => simp [hv] at accepted
      | some value => exact bind_preserves s name (.integer value mutable) next valid (evaluated_value _ _ _ hv) (by simpa [hv] using accepted)
  | assign name expression =>
      simp only [step] at accepted
      cases hn : lookup s.values name with
      | none => simp [hn] at accepted
      | some value =>
          cases value <;> simp only [hn,Option.bind_eq_bind,Option.bind_some] at accepted
          all_goals try cases accepted
          rename_i old mutable
          cases mutable
          · cases accepted
          · cases hv : eval s.values expression with
            | none => simp [hv] at accepted
            | some value =>
                rw [hv] at accepted
                cases accepted
                exact ⟨valid.1,update_valid _ _ _ _ valid.2 (evaluated_value _ _ _ hv)⟩
  | invoke name target argument =>
      simp only [step] at accepted
      cases hk : instanceKey s.values target with
      | none => simp [hk] at accepted
      | some key =>
          simp only [hk,Option.bind_eq_bind,Option.bind_some] at accepted
          cases ha : eval s.values argument with
          | none => simp [ha] at accepted
          | some arg =>
              simp only [ha,Option.bind_eq_bind,Option.bind_some] at accepted
              cases hs : CompositionMachine.start s.machine member place principal s.nextRequest key arg with
              | none => simp [hs] at accepted
              | some started =>
                  simp only [hs,Option.bind_eq_bind,Option.bind_some] at accepted
                  cases hr : CompositionMachine.resume started.1 started.2 with
                  | none => simp [hr] at accepted
                  | some completed =>
                      exact bind_preserves {s with machine := completed.1,nextRequest := s.nextRequest+1}
                        name (.integer completed.2 false) next
                        (invocation_preserves _ _ _ _ _ _ _ _ valid hs hr)
                        (resume_facts _ _ _ hr).2
                        (by simpa [hr] using accepted)

theorem execute_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (next : State p a) (valid : Invariant s)
    (accepted : execute s member place principal item = some next) : Invariant next := by
  unfold execute at accepted
  cases h : step s member place principal item.statement with
  | none => simp [h] at accepted
  | some state =>
      rw [h] at accepted
      cases accepted
      exact step_preserves s member place principal item.statement state valid h

theorem run_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (program : List Located) (next : State p a) (valid : Invariant s)
    (accepted : run s member place principal program = some next) : Invariant next := by
  induction program generalizing s with
  | nil => cases accepted; exact valid
  | cons item rest ih =>
      simp only [SourceAuthoring.run] at accepted
      cases h : execute s member place principal item with
      | none => simp [h] at accepted
      | some state =>
          exact ih state (execute_preserves _ _ _ _ _ _ valid h) (by simpa [h] using accepted)


theorem execution_preserves (s : SourceExecution.Execution p a) (member : Fin a) (place : Fin p)
    (principal : Nat) (program : List Located) (valid : Invariant s.source) :
    Invariant (SourceExecution.run s member place principal program).final.source := by
  induction program generalizing s with
  | nil => exact valid
  | cons item rest ih =>
      unfold SourceExecution.run
      cases h : SourceExecution.advance s member place principal item with
      | rejected reason => exact valid
      | accepted next =>
          obtain ⟨_,state,_,_,he,_,rfl⟩ := (SourceExecution.advance_exact _ _ _ _ _ _).mp h
          exact ih _ (execute_preserves _ _ _ _ _ _ valid he)

#print axioms control_facts
#print axioms step_preserves
#print axioms execution_preserves
end MirroreaProofFirst.SourceLocators
