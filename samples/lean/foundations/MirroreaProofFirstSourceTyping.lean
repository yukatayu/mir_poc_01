import MirroreaProofFirstSourcePreservation

namespace MirroreaProofFirst.SourceTyping
open SourceAuthoring SourceTypes

def environment (values : List (String × Value)) : Environment :=
  values.map (fun row => (row.1,typeOf row.2))

theorem lookup_environment (values : List (String × Value)) (name : String) :
    SourceTypes.lookup (environment values) name = (SourceAuthoring.lookup values name).map typeOf := by
  simp [SourceTypes.lookup,SourceAuthoring.lookup,environment,Function.comp_def,Option.map_map]

theorem update_environment (values : List (String × Value)) (name : String) (old value : Value)
    (found : SourceAuthoring.lookup values name = some old) (same : typeOf value = typeOf old) :
    environment (updateValue values name value) = environment values := by
  induction values with
  | nil => simp [SourceAuthoring.lookup] at found
  | cons row rest ih =>
      by_cases key : row.1 = name
      · have eq : row.2 = old := by simpa [SourceAuthoring.lookup,key] using found
        simp [updateValue,key,environment,eq,same]
      · have tail : SourceAuthoring.lookup rest name = some old := by
          simpa [SourceAuthoring.lookup,key] using found
        simp only [updateValue,if_neg key]
        change (row.1,typeOf row.2) :: environment (updateValue rest name value) =
          (row.1,typeOf row.2) :: environment rest
        rw [ih tail]

theorem bind_environment (s : State p a) (name : String) (value : Value) (next : State p a)
    (accepted : SourceAuthoring.bind s name value = some next) :
    environment next.values = (name,typeOf value) :: environment s.values := by
  unfold SourceAuthoring.bind at accepted
  split at accepted
  · cases accepted
  · cases accepted; rfl

theorem control_values (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) (next : State p a × Option Nat)
    (accepted : control s member place principal raw = some next) : next.1.values = s.values := by
  unfold SourceAuthoring.control at accepted
  cases h : CompositionMachine.manage s.machine member place principal s.nextRequest raw with
  | none => simp [h] at accepted
  | some result => rw [h] at accepted; cases accepted; rfl

-- This theorem states actual value-environment preservation. It neither asserts
-- current authorization nor promises that a well-typed dynamic operation succeeds.
theorem step_environment (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (statement : Statement) (next : State p a) (env : Environment)
    (typed : StatementTyped p (environment s.values) statement env)
    (accepted : step s member place principal statement = some next) : environment next.values = env := by
  have outputs := typed.2
  cases statement with
  | register name definition predecessor =>
      obtain ⟨_,rfl⟩ := outputs
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
                  have hb := bind_environment state name (.definition key) next (by simpa [hi] using accepted)
                  have hv : state.values = s.values := control_values s member place principal _ (state,created) hc
                  simpa only [typeOf,hv] using hb
  | instantiate name definition places parent support =>
      obtain ⟨_,rfl⟩ := outputs
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
                          have hb := bind_environment state name (.callable key) next (by simpa [hi] using accepted)
                          have hv : state.values = s.values := control_values s member place principal _ (state,created) hc
                          simpa only [typeOf,hv] using hb
  | retire out name =>
      obtain ⟨_,rfl⟩ := outputs
      simp only [step] at accepted
      cases hk : instanceKey s.values name with
      | none => simp [hk] at accepted
      | some key =>
          simp only [hk,Option.bind_eq_bind,Option.bind_some] at accepted
          cases hc : control s member place principal (.retire key) with
          | none => simp [hc] at accepted
          | some result =>
              have hb := bind_environment result.1 out .unit next (by simpa [hc] using accepted)
              simpa only [typeOf,control_values _ _ _ _ _ _ hc] using hb
  | reparent out name parent =>
      obtain ⟨_,rfl⟩ := outputs
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
                  have hb := bind_environment result.1 out .unit next (by simpa [hc] using accepted)
                  simpa only [typeOf,control_values _ _ _ _ _ _ hc] using hb
  | replace out name definition =>
      obtain ⟨_,rfl⟩ := outputs
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
                  have hb := bind_environment result.1 out .unit next (by simpa [hc] using accepted)
                  simpa only [typeOf,control_values _ _ _ _ _ _ hc] using hb
  | leave out locus =>
      obtain ⟨_,rfl⟩ := outputs
      simp only [step] at accepted
      cases hc : control s member place principal (.leave locus) with
      | none => simp [hc] at accepted
      | some result =>
          have hb := bind_environment result.1 out .unit next (by simpa [hc] using accepted)
          simpa only [typeOf,control_values _ _ _ _ _ _ hc] using hb
  | join out locus =>
      obtain ⟨_,rfl⟩ := outputs
      simp only [step] at accepted
      cases hc : control s member place principal (.join locus) with
      | none => simp [hc] at accepted
      | some result =>
          have hb := bind_environment result.1 out .unit next (by simpa [hc] using accepted)
          simpa only [typeOf,control_values _ _ _ _ _ _ hc] using hb
  | localValue name mutable expression =>
      obtain ⟨_,rfl⟩ := outputs
      simp only [step] at accepted
      cases hv : eval s.values expression with
      | none => simp [hv] at accepted
      | some value => exact bind_environment s name (.integer value mutable) next (by simpa [hv] using accepted)
  | assign name expression =>
      have eq : env = environment s.values := outputs
      subst env
      simp only [step] at accepted
      cases hn : SourceAuthoring.lookup s.values name with
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
                rw [hv] at accepted; cases accepted
                exact update_environment s.values name (.integer old true) (.integer value true) hn rfl
  | invoke name target argument =>
      obtain ⟨_,rfl⟩ := outputs
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
                      exact bind_environment {s with machine := completed.1,nextRequest := s.nextRequest+1}
                        name (.integer completed.2 false) next (by simpa [hr] using accepted)

theorem execute_environment (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (next : State p a) (env : Environment)
    (typed : StatementTyped p (environment s.values) item.statement env)
    (accepted : execute s member place principal item = some next) : environment next.values = env := by
  unfold execute at accepted
  cases h : step s member place principal item.statement with
  | none => simp [h] at accepted
  | some state => rw [h] at accepted; cases accepted; exact step_environment s member place principal item.statement state env typed h

theorem run_environment (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (program : List Located) (next : State p a) (env : Environment)
    (typed : ProgramTyped p (environment s.values) program env)
    (accepted : run s member place principal program = some next) : environment next.values = env := by
  induction program generalizing s with
  | nil => cases typed; cases accepted; rfl
  | cons item rest ih =>
      cases typed with
      | cons hs tail =>
          simp only [run] at accepted
          cases h : execute s member place principal item with
          | none => simp [h] at accepted
          | some state =>
              have eq := execute_environment s member place principal item state _ hs h
              exact ih state (by simpa [eq] using tail) (by simpa [h] using accepted)

#print axioms lookup_environment
#print axioms update_environment
#print axioms step_environment
#print axioms run_environment
end MirroreaProofFirst.SourceTyping
