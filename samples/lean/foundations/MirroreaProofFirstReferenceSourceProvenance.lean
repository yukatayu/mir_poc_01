import MirroreaProofFirstReferenceSourceLocators

namespace MirroreaProofFirst.ReferenceSourceProvenance
open ReferenceSourceData ReferenceSource

def Aligned (s : State p a) : Prop :=
  ∀ name, (s.writes.find? (fun w => w.name == name)).map Write.value = lookup s.values name

inductive Numbered : List Write → Prop where
  | nil : Numbered []
  | cons : w.ordinal = rest.length → Numbered rest → Numbered (w :: rest)

def ReadFrom (writes : List Write) (ceiling : Nat) (r : Read) : Prop :=
  match r.producer with
  | none => r.value = none
  | some ordinal => ∃ producer ∈ writes, producer.ordinal = ordinal ∧ producer.name = r.name ∧
      r.value = some producer.value ∧ producer.ordinal < ceiling

def InputsFrom (writes : List Write) (ceiling : Nat) (deps : List Read) : Prop :=
  ∀ r ∈ deps, ReadFrom writes ceiling r

structure Invariant (s : State p a) : Prop where
  aligned : Aligned s
  numbered : Numbered s.writes
  referenced : ∀ w ∈ s.writes, InputsFrom s.writes w.ordinal w.inputs
  waiting : ∀ saved, s.waiting = some saved → InputsFrom s.writes s.writes.length saved.inputs

theorem ordinal_lt (writes : List Write) (numbered : Numbered writes) (w : Write) (member : w ∈ writes) :
    w.ordinal < writes.length := by
  induction numbered with
  | nil => cases member
  | @cons head rest equal numbered ih =>
      rcases List.mem_cons.mp member with rfl | member
      · simp [equal]
      · exact Nat.lt_trans (ih member) (by simp)

theorem ordinal_unique (writes : List Write) (numbered : Numbered writes)
    (left right : Write) (hl : left ∈ writes) (hr : right ∈ writes)
    (same : left.ordinal = right.ordinal) : left = right := by
  induction numbered with
  | nil => cases hl
  | @cons rest head equal numbered ih =>
      rcases List.mem_cons.mp hl with leftHead | leftTail
      · rcases List.mem_cons.mp hr with rightHead | rightTail
        · exact leftHead.trans rightHead.symm
        · have less := ordinal_lt rest numbered right rightTail
          have atHead : left.ordinal = rest.length := leftHead ▸ equal
          omega
      · rcases List.mem_cons.mp hr with rightHead | rightTail
        · have less := ordinal_lt rest numbered left leftTail
          have atHead : right.ordinal = rest.length := rightHead ▸ equal
          omega
        · exact ih leftTail rightTail

theorem read_from (s : State p a) (valid : Invariant s) (name : String) :
    ReadFrom s.writes s.writes.length (ReferenceSource.read s name) := by
  unfold ReadFrom ReferenceSource.read
  cases found : s.writes.find? (fun w => w.name == name) with
  | none => simpa [found] using (valid.aligned name).symm
  | some producer =>
      have member := List.mem_of_find?_eq_some found
      have key : producer.name = name := by simpa using List.find?_some found
      exact ⟨producer,member,rfl,key,by simpa [found] using (valid.aligned name).symm,
        ordinal_lt s.writes valid.numbered producer member⟩

theorem inputs_from (s : State p a) (valid : Invariant s) (item : Located) :
    InputsFrom s.writes s.writes.length (inputs s item) := by
  intro r member
  obtain ⟨name,_,rfl⟩ := List.mem_map.mp member
  exact read_from _ valid _

theorem read_weaken (before after : List Write) (lo hi : Nat) (r : Read)
    (included : ∀ w ∈ before, w ∈ after) (bound : lo ≤ hi)
    (valid : ReadFrom before lo r) : ReadFrom after hi r := by
  cases producer : r.producer with
  | none => simpa [ReadFrom,producer] using valid
  | some ordinal =>
      obtain ⟨w,member,ordinalAt,nameAt,valueAt,less⟩ :=
        (show ∃ w ∈ before, w.ordinal = ordinal ∧ w.name = r.name ∧ r.value = some w.value ∧ w.ordinal < lo
          from by simpa [ReadFrom,producer] using valid)
      simp only [ReadFrom,producer]
      exact ⟨w,included _ member,ordinalAt,nameAt,valueAt,Nat.lt_of_lt_of_le less bound⟩

theorem update_other (values : Values) (name other : String) (value : Value) (different : other ≠ name) :
    lookup (updateValue values name value) other = lookup values other := by
  induction values with
  | nil => rfl
  | cons row rest ih =>
      by_cases key : row.1 = name
      · simp [updateValue,key,lookup,Ne.symm different]
      · simp only [updateValue,if_neg key]
        by_cases selected : row.1 = other
        · simp [lookup,selected]
        · simpa [lookup,selected] using ih

theorem update_self (values : Values) (name : String) (old value : Value)
    (found : lookup values name = some old) : lookup (updateValue values name value) name = some value := by
  induction values with
  | nil => simp [lookup] at found
  | cons row rest ih =>
      by_cases key : row.1 = name
      · simp [updateValue,key,lookup]
      · have tail : lookup rest name = some old := by simpa [lookup,key] using found
        simpa [updateValue,key,lookup] using ih tail

theorem write_aligned (s : State p a) (site : Site) (beforeCount : Nat) (deps : List Read)
    (name : String) (value : Value) (assign : Bool) (aligned : Aligned s)
    (existing : assign = true → ∃ old, lookup s.values name = some old) :
    Aligned (write s site beforeCount deps name value assign) := by
  intro other
  by_cases same : other = name
  · subst other
    cases assign with
    | false => simp [write,lookup]
    | true =>
        obtain ⟨old,found⟩ := existing rfl
        simpa [write] using (update_self _ _ _ _ found).symm
  · cases assign with
    | false => simpa [write,lookup,Ne.symm same] using aligned other
    | true => simpa [write,Ne.symm same,update_other _ _ _ _ same] using aligned other

theorem write_invariant (s : State p a) (site : Site) (beforeCount : Nat) (deps : List Read)
    (name : String) (value : Value) (assign : Bool) (valid : Invariant s)
    (existing : assign = true → ∃ old, lookup s.values name = some old)
    (dependencies : InputsFrom s.writes s.writes.length deps) :
    Invariant (write s site beforeCount deps name value assign) := by
  refine ⟨write_aligned _ _ _ _ _ _ _ valid.aligned existing,.cons rfl valid.numbered,?_,?_⟩
  · intro w member r input
    rcases List.mem_cons.mp member with rfl | old
    · exact read_weaken _ _ _ _ _ (fun w member => List.mem_cons_of_mem _ member) (Nat.le_refl _)
        (dependencies r input)
    · exact read_weaken _ _ _ _ _ (fun w member => List.mem_cons_of_mem _ member) (Nat.le_refl _)
        (valid.referenced w old r input)
  · intro saved waiting r input
    exact read_weaken _ _ _ _ _ (fun w member => List.mem_cons_of_mem _ member) (by simp [write])
      (valid.waiting saved waiting r input)

theorem mark_invariant (s : State p a) (machine : ReferenceExecution.Machine p a)
    (origin : Option Site) (kind : MicroKind) (consumed : Nat) (valid : Invariant s) :
    Invariant (mark s machine origin kind consumed) :=
  ⟨valid.aligned,valid.numbered,valid.referenced,valid.waiting⟩

theorem executePlan_invariant (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (plan : Plan) (valid : Invariant s)
    (assignment : ReferenceSourceTyping.AssignmentTyped s.values plan) :
    Invariant (executePlan s member place principal item plan).state := by
  have deps := inputs_from s valid item
  cases plan with
  | pureValue name value assign =>
      apply write_invariant _ _ _ _ _ _ _ valid _ deps
      intro assigned
      subst assign
      obtain ⟨old,found,_⟩ := assignment
      exact ⟨old,found⟩
  | control name command kind =>
      cases managed : ReferenceExecution.manage s.machine member place principal s.nextRequest (withOwner principal command) with
      | none => simpa [executePlan,managed] using valid
      | some pair =>
          obtain ⟨machine,created⟩ := pair
          have next := mark_invariant s machine (some item.site) .control 1 valid
          cases returned : resultValue kind created with
          | none => simpa [executePlan,managed,returned] using next
          | some value =>
              simp only [executePlan,managed,returned]
              exact write_invariant _ _ _ _ _ _ _ next (by simp) deps
  | acquire name chain =>
      cases acquired : ReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
      | none => simpa [executePlan,acquired] using valid
      | some pair =>
          obtain ⟨machine,key⟩ := pair
          simp only [executePlan,acquired]
          exact write_invariant _ _ _ _ _ _ _ (mark_invariant _ _ _ _ _ valid) (by simp) deps
  | reacquire name key =>
      cases acquired : ReferenceExecution.reacquire s.machine member place principal s.nextRequest key with
      | none => simpa [executePlan,acquired] using valid
      | some machine =>
          simp only [executePlan,acquired]
          exact write_invariant _ _ _ _ _ _ _ (mark_invariant _ _ _ _ _ valid) (by simp) deps
  | release name key =>
      cases released : ReferenceExecution.release s.machine member place principal s.nextRequest key with
      | none => simpa [executePlan,released] using valid
      | some machine =>
          simp only [executePlan,released]
          exact write_invariant _ _ _ _ _ _ _ (mark_invariant _ _ _ _ _ valid) (by simp) deps
  | call name target argument =>
      have valuesAt := (executeCall_shape s member place principal item name target argument).1
      have evidence := executeCall_evidence s member place principal item name target argument
      refine ⟨?_,?_,?_,?_⟩
      · intro queried
        simpa only [evidence.1,valuesAt] using valid.aligned queried
      · simpa only [evidence.1] using valid.numbered
      · simpa only [evidence.1] using valid.referenced
      · intro saved savedAt
        rw [evidence.1]
        rcases evidence.2 saved savedAt with old | new
        · exact valid.waiting saved old
        · rw [new.1]; exact deps

theorem advance_invariant (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (valid : Invariant s) : Invariant (advance s member place principal item).state := by
  unfold advance
  split
  · exact valid
  · cases typed : checkStatement p (environment s.values) item.statement with
    | none => exact valid
    | some env =>
        cases compiled : elaborate s.machine.store.core.system.configuration.state s.values item.statement with
        | none => exact valid
        | some plan =>
            exact executePlan_invariant _ _ _ _ _ _ valid
              (ReferenceSourceTyping.assignment_corresponds _ _ _ _ _
                ((statement_exact _ _ _ _).mp typed)
                ((ReferenceSourceElaboration.elaborate_exact _ _ _ _).mp compiled))

theorem complete_invariant (s : State p a) (valid : Invariant s) : Invariant (complete s).state := by
  cases waiting : s.waiting with
  | none => simpa [complete,waiting] using valid
  | some saved =>
      cases resumed : ReferenceExecution.resume s.machine saved.entry with
      | none => simpa [complete,waiting,resumed] using valid
      | some pair =>
          obtain ⟨machine,value⟩ := pair
          have deps := valid.waiting saved waiting
          have written := write_invariant
            (mark s machine (some saved.site) .result 0) saved.site saved.machineBefore saved.inputs
            saved.name (.plain (.integer value false)) false (mark_invariant _ _ _ _ _ valid) (by simp) deps
          simp only [complete,waiting,resumed]
          exact ⟨written.aligned,written.numbered,written.referenced,by intro _ absent; cases absent⟩

theorem controlInput_invariant (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) (result : State p a × Option Nat) (valid : Invariant s)
    (accepted : controlInput s member place principal raw = some result) : Invariant result.1 := by
  unfold controlInput at accepted
  cases managed : ReferenceExecution.manage s.machine member place principal s.nextRequest raw with
  | none => simp [managed] at accepted
  | some pair =>
      obtain ⟨machine,created⟩ := pair
      simp only [managed,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      exact mark_invariant _ _ _ _ _ valid

theorem cancel_invariant (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (valid : Invariant s) : Invariant (cancel s member place principal).state := by
  obtain ⟨values,writes,waiting⟩ := cancel_values_writes s member place principal
  refine ⟨?_,?_,?_,?_⟩
  · simpa only [Aligned,values,writes] using valid.aligned
  · simpa only [writes] using valid.numbered
  · simpa only [writes] using valid.referenced
  · rcases waiting with unchanged | cleared
    · simpa only [writes,unchanged] using valid.waiting
    · simp [cleared]

theorem step_invariant (s next : State p a) (valid : Invariant s) (step : ReferenceSourceTrace.Step s next) :
    Invariant next := by
  cases step with
  | advance => exact advance_invariant _ _ _ _ _ valid
  | cancellation => exact cancel_invariant _ _ _ _ valid
  | complete => exact complete_invariant _ valid
  | head => exact mark_invariant _ _ _ _ _ valid
  | control run => exact controlInput_invariant _ _ _ _ _ _ valid run

-- Historical reads identify actual earlier successful writes. Waiting stores
-- the original reads, so an interleaved head/control cannot substitute a new
-- value or producer at completion. No result is manufactured on failure.
theorem rooted_invariant (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (s : State p a)
    (path : ReferenceSourceTrace.Rooted realm view policy s) : Invariant s := by
  induction path with
  | refl =>
      refine ⟨?_,.nil,?_,?_⟩
      · intro name; rfl
      · intro w member; cases member
      · intro saved absent; cases absent
  | step previous step ih => exact step_invariant _ _ ih step

#print axioms rooted_invariant
#print axioms executePlan_invariant
#print axioms complete_invariant
#print axioms write_invariant
#print axioms inputs_from
#print axioms ordinal_unique
end MirroreaProofFirst.ReferenceSourceProvenance
