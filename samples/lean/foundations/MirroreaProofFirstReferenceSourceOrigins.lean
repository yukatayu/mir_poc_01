import MirroreaProofFirstReferenceOrigins

namespace MirroreaProofFirst.ReferenceSourceOrigins
open ReferenceSource ReferenceSourceData

-- A source label describes the actual stored occurrence. This definition is
-- independent of mark; marking alone cannot establish that an event occurred.
def Matches : MicroKind → ReferenceStore.Occurrence → Prop
  | .control, .core (.management ..) => True
  | .request, .core (.requested ..) => True
  | .result, .core (.result ..) => True
  | .authorityHead, .core (.authorityHead ..) => True
  | .cancellation, .cancellation _ => True
  | .acquire, .binding context => ∃ next, context.change = .acquire next
  | .reacquire, .binding context => ∃ old next, context.change = .reacquire old next
  | .release, .binding context => ∃ old, context.change = .release old
  | .normalize, .binding context => ∃ old next, context.change = .degrade old next
  | _, _ => False

-- Exact ordered correspondence: no invented, omitted, duplicated, or
-- misclassified occurrences. Source-only value writes do not create events.
inductive Aligned : List ReferenceStore.Occurrence → List Origin → Prop where
  | nil : Aligned [] []
  | step : Aligned events origins → Matches origin.kind event →
      origin.beforeCount = events.length → origin.afterCount = events.length + 1 →
      Aligned (event :: events) (origin :: origins)

def Invariant (s : State p a) : Prop := Aligned s.machine.store.events s.origins

theorem aligned_length (events : List ReferenceStore.Occurrence) (origins : List Origin)
    (valid : Aligned events origins) : events.length = origins.length := by
  induction valid <;> simp_all

theorem aligned_index (events : List ReferenceStore.Occurrence) (origins : List Origin)
    (valid : Aligned events origins) (origin : Origin) (present : origin ∈ origins) :
    origin.afterCount = origin.beforeCount + 1 ∧ origin.afterCount ≤ events.length ∧
      ∃ event, events.reverse[origin.beforeCount]? = some event ∧ Matches origin.kind event := by
  induction valid with
  | nil => cases present
  | @step priorEvents priorOrigins event latest previous kind beforeCount afterCount ih =>
      rcases List.mem_cons.mp present with same | older
      · subst origin
        refine ⟨by omega,by simp [afterCount],event,?_,kind⟩
        rw [beforeCount,List.reverse_cons]
        simpa using (List.getElem?_concat_length (l:=priorEvents.reverse) (a:=event))
      · obtain ⟨increment,bound,old,indexed,matched⟩ := ih older
        exact ⟨increment,by simp only [List.length_cons]; omega,old,
          ReferenceOrigins.indexed_extend _ _ _ _ indexed,matched⟩

theorem mark_preserves (s : State p a) (next : ReferenceExecution.Machine p a)
    (site : Option Site) (kind : MicroKind) (consumed : Nat)
    (valid : Invariant s) (event : ReferenceStore.Occurrence)
    (recorded : next.store.events = event :: s.machine.store.events) (matched : Matches kind event) :
    Invariant (mark s next site kind consumed) := by
  unfold Invariant mark
  dsimp only
  rw [recorded]
  exact .step valid matched rfl rfl

theorem write_preserves (s : State p a) (site : Site) (beforeCount : Nat) (deps : List Read)
    (name : String) (value : Value) (assign : Bool) (valid : Invariant s) :
    Invariant (write s site beforeCount deps name value assign) := valid

theorem beginCall_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (site : Site) (beforeCount : Nat) (deps : List Read) (name : String) (target : Target) (argument : Int)
    (valid : Invariant s) :
    Invariant (beginCall s member place principal site beforeCount deps name target argument).state := by
  cases target with
  | «instance» key =>
      simp only [beginCall]
      cases run : ReferenceExecution.startPlain s.machine member place principal s.nextRequest key argument with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,entry⟩ := pair
          obtain ⟨ticket,recorded⟩ := ReferenceExecution.startPlain_event _ _ _ _ _ _ _ _ run
          exact mark_preserves _ _ _ _ 1 valid _ recorded True.intro
  | reference key =>
      simp only [beginCall]
      cases run : ReferenceExecution.startReference s.machine member place principal s.nextRequest key argument with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,entry⟩ := pair
          obtain ⟨ticket,recorded⟩ := ReferenceExecution.startReference_event _ _ _ _ _ _ _ _ run
          exact mark_preserves _ _ _ _ 1 valid _ recorded True.intro

theorem executePlan_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (plan : Plan) (valid : Invariant s) :
    Invariant (executePlan s member place principal item plan).state := by
  cases plan with
  | pureValue name value assign => exact valid
  | control name command kind =>
      simp only [executePlan]
      cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest (withOwner principal command) with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,created⟩ := pair
          obtain ⟨context,created',recorded⟩ := ReferenceExecution.manage_event _ _ _ _ _ _ _ run
          have marked := mark_preserves s machine (some item.site) .control 1 valid _ recorded True.intro
          dsimp only
          cases resultValue kind created <;> exact marked
  | acquire name chain =>
      simp only [executePlan]
      cases run : ReferenceExecution.acquire s.machine member place principal s.nextRequest chain with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,key⟩ := pair
          obtain ⟨context,next,kind,recorded⟩ := ReferenceExecution.acquire_event _ _ _ _ _ _ _ run
          exact mark_preserves _ _ _ _ 1 valid _ recorded ⟨next,kind⟩
  | reacquire name key =>
      simp only [executePlan]
      cases run : ReferenceExecution.reacquire s.machine member place principal s.nextRequest key with
      | none => exact valid
      | some machine =>
          obtain ⟨context,old,next,kind,recorded⟩ := ReferenceExecution.reacquire_event _ _ _ _ _ _ _ run
          exact mark_preserves _ _ _ _ 1 valid _ recorded ⟨old,next,kind⟩
  | release name key =>
      simp only [executePlan]
      cases run : ReferenceExecution.release s.machine member place principal s.nextRequest key with
      | none => exact valid
      | some machine =>
          obtain ⟨context,old,kind,recorded⟩ := ReferenceExecution.release_event _ _ _ _ _ _ _ run
          exact mark_preserves _ _ _ _ 1 valid _ recorded ⟨old,kind⟩
  | call name target argument =>
      cases target with
      | «instance» key => exact beginCall_preserves _ _ _ _ _ _ _ _ _ _ valid
      | reference key =>
          let normalized := ReferenceExecution.normalize s.machine member place principal s.nextRequest key
          let next := if normalized.consumed then mark s normalized.state (some item.site) .normalize 1 else s
          have marked : Invariant next := by
            unfold next
            split
            · rename_i consumed
              obtain ⟨context,old,next,kind,recorded⟩ := ReferenceExecution.normalize_event _ _ _ _ _ _ consumed
              exact mark_preserves _ _ _ _ 1 valid _ recorded ⟨old,next,kind⟩
            · exact valid
          change Invariant
            (match normalized.result with
            | .error reason => (⟨next,.failed (.normalization reason)⟩ : Outcome p a)
            | .ok _ => beginCall next member place principal item.site s.machine.store.events.length (inputs s item) name (.reference key) argument).state
          cases normalized.result with
          | error _ => exact marked
          | ok _ => exact beginCall_preserves _ _ _ _ _ _ _ _ _ _ marked

theorem advance_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (item : Located) (valid : Invariant s) : Invariant (advance s member place principal item).state := by
  unfold advance
  split
  · exact valid
  · cases checkStatement p (environment s.values) item.statement with
    | none => exact valid
    | some env =>
        cases elaborate s.machine.store.core.system.configuration.state s.values item.statement with
        | none => exact valid
        | some plan => exact executePlan_preserves _ _ _ _ _ _ valid

theorem complete_preserves (s : State p a) (valid : Invariant s) : Invariant (complete s).state := by
  cases waiting : s.waiting with
  | none => simpa [complete,waiting] using valid
  | some saved =>
      simp only [complete,waiting]
      cases resumed : ReferenceExecution.resume s.machine saved.entry with
      | none => exact valid
      | some pair =>
          obtain ⟨machine,value⟩ := pair
          unfold ReferenceExecution.resume at resumed
          cases evaluated : InvocationBoundary.execute saved.entry.ticket with
          | none => simp [evaluated] at resumed
          | some result =>
              simp only [evaluated,Option.bind_eq_bind,Option.bind_some] at resumed
              cases finished : ReferenceExecution.finish s.machine saved.entry result with
              | none => simp [finished] at resumed
              | some next =>
                  simp only [finished,Option.bind_some,Option.pure_def,Option.some.injEq,Prod.mk.injEq] at resumed
                  obtain ⟨rfl,rfl⟩ := resumed
                  exact mark_preserves _ _ _ _ 0 valid _ (ReferenceExecution.finish_event _ _ _ _ finished) True.intro

theorem cancel_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (valid : Invariant s) : Invariant (cancel s member place principal).state := by
  cases waiting : s.waiting with
  | none => simpa [cancel,waiting] using valid
  | some saved =>
      simp only [cancel,waiting]
      cases run : ReferenceExecution.cancel s.machine member place principal s.nextRequest saved.entry with
      | none => exact valid
      | some next =>
          obtain ⟨permit,recorded⟩ := ReferenceExecution.cancel_event _ _ _ _ _ _ _ run
          exact mark_preserves _ _ _ _ 1 valid _ recorded True.intro

theorem head_preserves (s : State p a) (view : WorldProjection.AuthorityView a) (valid : Invariant s) :
    Invariant (authorityHead s view) :=
  mark_preserves _ _ _ _ 0 valid _ rfl True.intro

theorem control_preserves (s : State p a) (member : Fin a) (place : Fin p) (principal : Nat)
    (raw : CompositionCore.Raw) (result : State p a × Option Nat) (valid : Invariant s)
    (accepted : controlInput s member place principal raw = some result) : Invariant result.1 := by
  unfold controlInput at accepted
  cases run : ReferenceExecution.manage s.machine member place principal s.nextRequest raw with
  | none => simp [run] at accepted
  | some pair =>
      obtain ⟨machine,created⟩ := pair
      simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      obtain ⟨context,created,recorded⟩ := ReferenceExecution.manage_event _ _ _ _ _ _ _ run
      exact mark_preserves _ _ _ _ 1 valid _ recorded True.intro

theorem step_preserves (s next : State p a) (valid : Invariant s) (step : ReferenceSourceTrace.Step s next) :
    Invariant next := by
  cases step with
  | advance => exact advance_preserves _ _ _ _ _ valid
  | cancellation => exact cancel_preserves _ _ _ _ valid
  | complete => exact complete_preserves _ valid
  | head => exact head_preserves _ _ valid
  | control run => exact control_preserves _ _ _ _ _ _ valid run

theorem reached_preserves (s next : State p a) (valid : Invariant s) (path : ReferenceSourceTrace.Reached s next) :
    Invariant next := by
  induction path with
  | refl => exact valid
  | step previous step ih => exact step_preserves _ _ ih step

theorem rooted_aligned (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (s : State p a) (path : ReferenceSourceTrace.Rooted realm view policy s) :
    Invariant s := reached_preserves (initial realm view policy) s .nil path

theorem rooted_no_fabrication (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (s : State p a) (path : ReferenceSourceTrace.Rooted realm view policy s) :
    s.machine.store.events.length = s.origins.length ∧ ∀ origin ∈ s.origins,
      origin.afterCount = origin.beforeCount + 1 ∧ origin.afterCount ≤ s.machine.store.events.length ∧
      ∃ event, s.machine.store.events.reverse[origin.beforeCount]? = some event ∧ Matches origin.kind event := by
  have valid := rooted_aligned _ _ _ _ path
  exact ⟨aligned_length _ _ valid,fun origin present => aligned_index _ _ valid origin present⟩

#print axioms step_preserves
#print axioms rooted_aligned
#print axioms rooted_no_fabrication

#print axioms aligned_index
#print axioms mark_preserves
#print axioms executePlan_preserves
end MirroreaProofFirst.ReferenceSourceOrigins
