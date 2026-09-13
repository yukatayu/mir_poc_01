import MirroreaProofFirstSourceExecution

namespace MirroreaProofFirst.SourceFrame
open SourceAuthoring SourceTypes SourceTyping

def Frame (before after : List (String × Value)) (changed : String) : Prop :=
  ∀ other, other ≠ changed → SourceAuthoring.lookup after other = SourceAuthoring.lookup before other

theorem bind_frame (s : State p a) (name : String) (value : Value) (next : State p a)
    (accepted : SourceAuthoring.bind s name value = some next) : Frame s.values next.values name := by
  unfold SourceAuthoring.bind at accepted
  split at accepted
  · cases accepted
  · cases accepted
    intro other different
    simp [SourceAuthoring.lookup,Ne.symm different]

theorem update_frame (values : List (String × Value)) (name : String) (value : Value) :
    Frame values (updateValue values name value) name := by
  intro other different
  induction values with
  | nil => rfl
  | cons row rest ih =>
      by_cases key : row.1 = name
      · simp [updateValue,key,SourceAuthoring.lookup,Ne.symm different]
      · simp only [updateValue,if_neg key]
        by_cases selected : row.1 = other
        · simp [SourceAuthoring.lookup,selected]
        · simpa [SourceAuthoring.lookup,selected] using ih

theorem step_frame (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (statement : Statement) (next : State p a)
    (accepted : step s member place principal statement = some next) :
    Frame s.values next.values (SourceExecution.outputName statement) := by
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
                  have hb := bind_frame state name (.definition key) next (by simpa [hi] using accepted)
                  have hv : state.values = s.values := control_values s member place principal _ (state,created) hc
                  simpa only [hv] using hb
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
                          have hb := bind_frame state name (.callable key) next (by simpa [hi] using accepted)
                          have hv : state.values = s.values := control_values s member place principal _ (state,created) hc
                          simpa only [hv] using hb
  | retire out name =>
      simp only [step] at accepted
      cases hk : instanceKey s.values name with
      | none => simp [hk] at accepted
      | some key =>
          simp only [hk,Option.bind_eq_bind,Option.bind_some] at accepted
          cases hc : control s member place principal (.retire key) with
          | none => simp [hc] at accepted
          | some result =>
              have hb := bind_frame result.1 out .unit next (by simpa [hc] using accepted)
              simpa only [control_values _ _ _ _ _ _ hc] using hb
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
                  have hb := bind_frame result.1 out .unit next (by simpa [hc] using accepted)
                  simpa only [control_values _ _ _ _ _ _ hc] using hb
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
                  have hb := bind_frame result.1 out .unit next (by simpa [hc] using accepted)
                  simpa only [control_values _ _ _ _ _ _ hc] using hb
  | leave out locus =>
      simp only [step] at accepted
      cases hc : control s member place principal (.leave locus) with
      | none => simp [hc] at accepted
      | some result =>
          have hb := bind_frame result.1 out .unit next (by simpa [hc] using accepted)
          simpa only [control_values _ _ _ _ _ _ hc] using hb
  | join out locus =>
      simp only [step] at accepted
      cases hc : control s member place principal (.join locus) with
      | none => simp [hc] at accepted
      | some result =>
          have hb := bind_frame result.1 out .unit next (by simpa [hc] using accepted)
          simpa only [control_values _ _ _ _ _ _ hc] using hb
  | localValue name mutable expression =>
      simp only [step] at accepted
      cases hv : eval s.values expression with
      | none => simp [hv] at accepted
      | some value => exact bind_frame s name (.integer value mutable) next (by simpa [hv] using accepted)
  | assign name expression =>
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
                exact update_frame s.values name (.integer value true)
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
                      exact bind_frame {s with machine := completed.1,nextRequest := s.nextRequest+1}
                        name (.integer completed.2 false) next (by simpa [hr] using accepted)

theorem execute_frame (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (next : State p a) (accepted : execute s member place principal item = some next) :
    Frame s.values next.values (SourceExecution.outputName item.statement) := by
  unfold execute at accepted
  cases h : step s member place principal item.statement with
  | none => simp [h] at accepted
  | some state => rw [h] at accepted; cases accepted; exact step_frame s member place principal item.statement state h

#print axioms step_frame
#print axioms execute_frame
end MirroreaProofFirst.SourceFrame
