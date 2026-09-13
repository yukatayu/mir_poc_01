import MirroreaProofFirstSourceCompletion

namespace MirroreaProofFirst.SourceAllocation
open SourceAuthoring CompositionMachine

def Below (m : Machine p a) (next : Nat) : Prop :=
  (∀ id ∈ m.system.used,id.request < next) ∧ (∀ t ∈ m.pending,t.id < next)

def Invariant (s : State p a) : Prop := Below s.machine s.nextRequest

theorem fresh_next (s : State p a) (valid : Invariant s) (realm principal : Nat) :
    fresh s.machine ⟨realm,principal,s.nextRequest⟩ = true := by
  apply (fresh_exact _ _).mpr
  constructor
  · intro member
    have impossible := valid.1 _ member
    exact Nat.lt_irrefl _ impossible
  · intro member
    obtain ⟨t,ht,eq⟩ := List.mem_map.mp member
    have same : t.id = s.nextRequest := congrArg CurrentUse.UseId.request eq
    have less := valid.2 t ht
    omega

theorem empty (s : State p a) (used : s.machine.system.used = []) (pending : s.machine.pending = []) : Invariant s := by
  constructor <;> intro x hx <;> simp_all

theorem manage_below (m : Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : CompositionCore.Raw) (next : Machine p a × Option Nat) (valid : Below m id)
    (accepted : manage m member place principal id raw = some next) : Below next.1 (id+1) := by
  obtain ⟨_,_,cfg,created,_,_,rfl⟩ := manage_parts _ _ _ _ _ _ _ accepted
  constructor
  · intro request member
    rcases List.mem_cons.mp member with rfl | old
    · exact Nat.lt_succ_self id
    · exact Nat.lt_trans (valid.1 request old) (Nat.lt_succ_self id)
  · intro t member
    exact Nat.lt_trans (valid.2 t member) (Nat.lt_succ_self id)

theorem prepare_id (s : InstanceState.State d p n) (v : WorldProjection.AuthorityView a)
    (member : Fin a) (key : Fin n) (place : Fin p) (principal id : Nat) (arg : Int)
    (t : InvocationBoundary.Ticket)
    (accepted : InvocationBoundary.prepare s v member key place principal id arg = some t) : t.id = id := by
  unfold InvocationBoundary.prepare at accepted
  cases e : CurrentUse.authorize (WorldProjection.world s v).authority
      ((WorldProjection.world s v).policies (WorldProjection.request s v member key place principal id arg).operation.key)
      (CurrentUse.currentContext (WorldProjection.world s v) (WorldProjection.request s v member key place principal id arg)) with
  | none => simp [e] at accepted
  | some evidence =>
      simp only [e,Option.bind_eq_bind,Option.bind_some] at accepted
      split at accepted
      · cases accepted; rfl
      · cases accepted

theorem start_id (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) (arg : Int)
    (next : Machine p a × InvocationBoundary.Ticket)
    (accepted : start m member place principal id key arg = some next) : next.2.id = id := by
  unfold start at accepted
  cases hi : CompositionCore.index m.system.configuration.count key with
  | none => simp [hi] at accepted
  | some unit =>
      simp only [hi,Option.bind_eq_bind,Option.bind_some] at accepted
      cases ht : InvocationBoundary.prepare m.system.configuration.state m.system.view member unit place principal id arg with
      | none => simp [ht] at accepted
      | some t =>
          simp only [ht,Option.bind_eq_bind,Option.bind_some] at accepted
          split at accepted
          · cases accepted; exact prepare_id _ _ _ _ _ _ _ _ _ ht
          · cases accepted

theorem start_below (m : Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) (arg : Int)
    (next : Machine p a × InvocationBoundary.Ticket) (valid : Below m id)
    (accepted : start m member place principal id key arg = some next) : Below next.1 (id+1) := by
  have exactId := start_id _ _ _ _ _ _ _ _ accepted
  obtain ⟨_,_,eq⟩ := start_parts _ _ _ _ _ _ _ _ accepted
  rw [eq]
  constructor
  · intro request member
    exact Nat.lt_trans (valid.1 request member) (Nat.lt_succ_self id)
  · intro t member
    rcases List.mem_cons.mp member with rfl | old
    · rw [exactId]; exact Nat.lt_succ_self id
    · exact Nat.lt_trans (valid.2 t old) (Nat.lt_succ_self id)

theorem finish_below (m : Machine p a) (t : InvocationBoundary.Ticket) (value : Int)
    (next : Machine p a) (bound : Nat) (valid : Below m bound)
    (accepted : finish m t value = some next) : Below next bound := by
  obtain ⟨pending,_,_,rfl⟩ := finish_parts _ _ _ _ accepted
  constructor
  · intro id member
    rcases List.mem_cons.mp member with rfl | old
    · exact valid.2 t pending
    · exact valid.1 id old
  · intro ticket member
    exact valid.2 ticket (List.mem_filter.mp member).1

theorem bind_preserves (s : State p a) (name : String) (value : Value) (next : State p a)
    (valid : Invariant s) (bound : bind s name value = some next) : Invariant next := by
  unfold SourceAuthoring.bind at bound
  split at bound
  · cases bound
  · cases bound; exact valid

theorem control_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) (next : State p a × Option Nat)
    (valid : Invariant s) (accepted : control s member place principal raw = some next) : Invariant next.1 := by
  unfold SourceAuthoring.control at accepted
  cases h : manage s.machine member place principal s.nextRequest raw with
  | none => simp [h] at accepted
  | some result => rw [h] at accepted; cases accepted; exact manage_below _ _ _ _ _ _ _ valid h

theorem resume_below (m : Machine p a) (t : InvocationBoundary.Ticket)
    (next : Machine p a × Int) (bound : Nat) (valid : Below m bound)
    (accepted : resume m t = some next) : Below next.1 bound := by
  unfold resume at accepted
  cases he : InvocationBoundary.execute t with
  | none => simp [he] at accepted
  | some value =>
      simp only [he,Option.bind_eq_bind,Option.bind_some] at accepted
      cases hf : finish m t value with
      | none => simp [hf] at accepted
      | some result => rw [hf] at accepted; cases accepted; exact finish_below _ _ _ _ _ valid hf

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
                        (resume_below _ _ _ _ (start_below _ _ _ _ _ _ _ _ valid hs) hr)
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
      simp only [run] at accepted
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

#print axioms fresh_next
#print axioms start_id
#print axioms step_preserves
#print axioms execution_preserves
end MirroreaProofFirst.SourceAllocation
