import MirroreaProofFirstSourceExecution
import MirroreaProofFirstCatalogHistory

namespace MirroreaProofFirst.SourceHistory
open SourceAuthoring

variable {initial : CompositionMachine.Machine p a}

def Invariant (s : State p a) : Prop := CatalogHistory.Reached initial s.machine

theorem bind_preserves (s : State p a) (name : String) (value : Value) (next : State p a)
    (valid : Invariant (initial := initial) s) (bound : bind s name value = some next) : Invariant (initial := initial) next := by
  unfold SourceAuthoring.bind at bound
  split at bound
  · cases bound
  · cases bound; exact valid

theorem control_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) (next : State p a × Option Nat)
    (valid : Invariant (initial := initial) s) (accepted : control s member place principal raw = some next) : Invariant (initial := initial) next.1 := by
  unfold SourceAuthoring.control at accepted
  cases h : CompositionMachine.manage s.machine member place principal s.nextRequest raw with
  | none => simp [h] at accepted
  | some result =>
      rw [h] at accepted
      cases accepted
      exact CatalogHistory.management_reaches _ _ _ _ _ _ _ valid h

theorem resume_preserves (m : CompositionMachine.Machine p a) (t : InvocationBoundary.Ticket)
    (next : CompositionMachine.Machine p a × Int) (valid : CatalogHistory.Reached initial m)
    (accepted : CompositionMachine.resume m t = some next) : CatalogHistory.Reached initial next.1 := by
  unfold CompositionMachine.resume at accepted
  cases he : InvocationBoundary.execute t with
  | none => simp [he] at accepted
  | some value =>
      simp only [he,Option.bind_eq_bind,Option.bind_some] at accepted
      cases hf : CompositionMachine.finish m t value with
      | none => simp [hf] at accepted
      | some result =>
          rw [hf] at accepted
          cases accepted
          exact CatalogHistory.finish_reaches _ _ _ _ valid hf

theorem step_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (statement : Statement) (next : State p a) (valid : Invariant (initial := initial) s)
    (accepted : step s member place principal statement = some next) : Invariant (initial := initial) next := by
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
                  exact bind_preserves state name (.definition key) next (control_preserves _ _ _ _ _ _ valid hc)
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
                          exact bind_preserves state name (.callable key) next (control_preserves _ _ _ _ _ _ valid hc)
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
              exact bind_preserves _ _ _ _ (control_preserves _ _ _ _ _ _ valid hc) (by simpa [hc] using accepted)
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
                  exact bind_preserves _ _ _ _ (control_preserves _ _ _ _ _ _ valid hc) (by simpa [hc] using accepted)
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
                  exact bind_preserves _ _ _ _ (control_preserves _ _ _ _ _ _ valid hc) (by simpa [hc] using accepted)
  | leave out locus =>
      simp only [step] at accepted
      cases hc : control s member place principal (.leave locus) with
      | none => simp [hc] at accepted
      | some result => exact bind_preserves _ _ _ _ (control_preserves _ _ _ _ _ _ valid hc) (by simpa [hc] using accepted)
  | join out locus =>
      simp only [step] at accepted
      cases hc : control s member place principal (.join locus) with
      | none => simp [hc] at accepted
      | some result => exact bind_preserves _ _ _ _ (control_preserves _ _ _ _ _ _ valid hc) (by simpa [hc] using accepted)
  | localValue name mutable expression =>
      simp only [step] at accepted
      cases hv : eval s.values expression with
      | none => simp [hv] at accepted
      | some value => exact bind_preserves _ _ _ _ valid (by simpa [hv] using accepted)
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
            | some value => rw [hv] at accepted; cases accepted; exact valid
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
                        (resume_preserves _ _ _ (CatalogHistory.start_reaches _ _ _ _ _ _ _ _ valid hs) hr)
                        (by simpa [hr] using accepted)

theorem execute_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (next : State p a) (valid : Invariant (initial := initial) s)
    (accepted : execute s member place principal item = some next) : Invariant (initial := initial) next := by
  unfold execute at accepted
  cases h : step s member place principal item.statement with
  | none => simp [h] at accepted
  | some state =>
      rw [h] at accepted
      cases accepted
      exact step_preserves s member place principal item.statement state valid h

theorem run_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (program : List Located) (next : State p a) (valid : Invariant (initial := initial) s)
    (accepted : run s member place principal program = some next) : Invariant (initial := initial) next := by
  induction program generalizing s with
  | nil => cases accepted; exact valid
  | cons item rest ih =>
      simp only [run] at accepted
      cases h : execute s member place principal item with
      | none => simp [h] at accepted
      | some state =>
          exact ih state (execute_preserves _ _ _ _ _ _ valid h) (by simpa [h] using accepted)

#print axioms step_preserves
#print axioms run_preserves

-- The exported execution path, including a rejected suffix, stays in the actual
-- machine history. Thus its catalog/head claims are not an unrelated model.
theorem execution_reaches (s : SourceExecution.Execution p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (program : List Located) (path : CatalogHistory.Reached initial s.source.machine) :
    CatalogHistory.Reached initial (SourceExecution.run s member place principal program).final.source.machine := by
  induction program generalizing s with
  | nil => exact path
  | cons item rest ih =>
      unfold SourceExecution.run
      cases h : SourceExecution.advance s member place principal item with
      | rejected reason => exact path
      | accepted next =>
          obtain ⟨_,state,value,_,he,_,rfl⟩ := (SourceExecution.advance_exact _ _ _ _ _ _).mp h
          exact ih (SourceExecution.appendWrite s item state value)
            (execute_preserves s.source member place principal item state path he)

theorem execution_catalog (s : SourceExecution.Execution p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (program : List Located) (valid : CatalogHistory.Invariant s.source.machine.system.configuration) :
    CatalogHistory.Invariant (SourceExecution.run s member place principal program).final.source.machine.system.configuration ∧
    CatalogHistory.Extends s.source.machine.system.configuration
      (SourceExecution.run s member place principal program).final.source.machine.system.configuration :=
  CatalogHistory.reached_catalog _ _ valid (execution_reaches s member place principal program .refl)

#print axioms execution_reaches
#print axioms execution_catalog
end MirroreaProofFirst.SourceHistory
