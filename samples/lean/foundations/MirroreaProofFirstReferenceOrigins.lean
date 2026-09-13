import MirroreaProofFirstReferenceChronology

namespace MirroreaProofFirst.ReferenceOrigins
open ReferenceOwner ReferenceStore ReferenceMutation

def Selected (events : List ReferenceStore.Occurrence) (binding : Binding) : Prop :=
  ∃ context, events.reverse[binding.frontier]? = some (.binding context) ∧ after context.change = some binding ∧
    binding.activation = binding.frontier + 1

def Held (events : List ReferenceStore.Occurrence) (binding : Binding) : Prop :=
  ∃ context issued, events.reverse[binding.holding.frontier]? = some (.binding context) ∧
    (context.change = .acquire issued ∨ ∃ previous, context.change = .reacquire previous issued) ∧
    issued.request = binding.request ∧ issued.holding = binding.holding ∧
    binding.holding.activation = binding.holding.frontier + 1

def Invariant (m : ReferenceStore.Machine p a) : Prop :=
  ∀ binding, some binding ∈ m.bindings → Selected m.events binding ∧ Held m.events binding

theorem indexed_extend (events : List ReferenceStore.Occurrence) (event existing : ReferenceStore.Occurrence)
    (key : Nat) (present : events.reverse[key]? = some existing) :
    (event :: events).reverse[key]? = some existing := by
  obtain ⟨bound,_⟩ := List.getElem?_eq_some_iff.mp present
  simpa only [List.reverse_cons,List.getElem?_append_left bound] using present

theorem selected_extend (events : List ReferenceStore.Occurrence) (event : ReferenceStore.Occurrence)
    (binding : Binding) (valid : Selected events binding) : Selected (event :: events) binding := by
  obtain ⟨context,present,produced,clock⟩ := valid
  exact ⟨context,indexed_extend _ _ _ _ present,produced,clock⟩

theorem held_extend (events : List ReferenceStore.Occurrence) (event : ReferenceStore.Occurrence)
    (binding : Binding) (valid : Held events binding) : Held (event :: events) binding := by
  obtain ⟨context,issued,present,kind,request,stamp,clock⟩ := valid
  exact ⟨context,issued,indexed_extend _ _ _ _ present,kind,request,stamp,clock⟩

theorem held_same (events : List ReferenceStore.Occurrence) (old next : Binding)
    (request : next.request = old.request) (stamp : next.holding = old.holding) (valid : Held events old) : Held events next := by
  obtain ⟨context,issued,present,kind,requestAt,stampAt,clock⟩ := valid
  exact ⟨context,issued,by rw [stamp]; exact present,kind,requestAt.trans request.symm,stampAt.trans stamp.symm,by simpa [stamp] using clock⟩

theorem indexed_new (events : List ReferenceStore.Occurrence) (context : ReferenceOwner.Context) :
    (ReferenceStore.Occurrence.binding context :: events).reverse[events.length]? = some (.binding context) := by
  simpa [List.reverse_cons] using (List.getElem?_concat_length (l:=events.reverse) (a:=.binding context))

theorem selected_new (events : List ReferenceStore.Occurrence) (context : ReferenceOwner.Context) (binding : Binding)
    (frontier : binding.frontier = events.length) (clock : binding.activation = binding.frontier + 1) (produced : after context.change = some binding) :
    Selected (.binding context :: events) binding := by
  exact ⟨context,by rw [frontier]; exact indexed_new _ _,produced,clock⟩

theorem held_new (events : List ReferenceStore.Occurrence) (context : ReferenceOwner.Context) (binding : Binding)
    (frontier : binding.holding.frontier = events.length)
    (clock : binding.holding.activation = binding.holding.frontier + 1)
    (kind : context.change = .acquire binding ∨ ∃ old, context.change = .reacquire old binding) :
    Held (.binding context :: events) binding :=
  ⟨context,binding,by rw [frontier]; exact indexed_new _ _,kind,rfl,rfl,clock⟩

theorem same_rows (m next : ReferenceStore.Machine p a) (event : ReferenceStore.Occurrence)
    (rows : next.bindings = m.bindings) (events : next.events = event :: m.events) (valid : Invariant m) : Invariant next := by
  intro binding present
  obtain ⟨selected,held⟩ := valid binding (by simpa [rows] using present)
  rw [events]
  exact ⟨selected_extend _ _ _ selected,held_extend _ _ _ held⟩

private theorem lookup_member (m : ReferenceStore.Machine p a) (key : Nat) (binding : Binding)
    (found : lookup m key = some binding) : some binding ∈ m.bindings :=
  List.mem_of_getElem? (lookup_row _ _ _ found)

theorem change_preserves (m next : ReferenceStore.Machine p a) (valid : Invariant m)
    (count : m.history.length = m.events.length + 1) (record : ChangeRecord m next) : Invariant next := by
  obtain ⟨change,old,rows,metadata,context,exactChange,_,events⟩ := record
  intro binding present
  rw [rows] at present
  rw [events]
  have older : ∀ oldBinding, some oldBinding ∈ m.bindings →
      Selected (.binding context :: m.events) oldBinding ∧ Held (.binding context :: m.events) oldBinding := by
    intro oldBinding present
    have known := valid oldBinding present
    exact ⟨selected_extend _ _ _ known.1,held_extend _ _ _ known.2⟩
  cases change with
  | acquire new =>
      rcases List.mem_append.mp present with oldMem | freshMem
      · exact older binding oldMem
      · have equal : binding = new := by simpa using freshMem
        subst binding
        exact ⟨selected_new _ _ _ metadata.1.1 (by rw [metadata.1.2,metadata.1.1,count]) (by rw [exactChange]; rfl),held_new _ _ _ metadata.2.1 (by rw [metadata.2.2,metadata.2.1,count]) (Or.inl exactChange)⟩
  | reacquire previous new =>
      rcases List.mem_or_eq_of_mem_set present with oldMem | freshEq
      · exact older binding oldMem
      · cases freshEq
        exact ⟨selected_new _ _ _ metadata.1.1 (by rw [metadata.1.2,metadata.1.1,count]) (by rw [exactChange]; rfl),held_new _ _ _ metadata.2.1 (by rw [metadata.2.2,metadata.2.1,count]) (Or.inr ⟨previous,exactChange⟩)⟩
  | degrade previous new =>
      rcases List.mem_or_eq_of_mem_set present with oldMem | freshEq
      · exact older binding oldMem
      · cases freshEq
        have stored : lookup m previous.request.binding = some previous := by simpa [oldStoreCheck] using old
        have held := (older previous (lookup_member _ _ _ stored)).2
        exact ⟨selected_new _ _ _ metadata.1.1 (by rw [metadata.1.2,metadata.1.1,count]) (by rw [exactChange]; rfl),held_same _ _ _ metadata.2.1 metadata.2.2 held⟩
  | release previous =>
      rcases List.mem_or_eq_of_mem_set present with oldMem | impossible
      · exact older binding oldMem
      · cases impossible

theorem transition_preserves (m next : ReferenceStore.Machine p a) (valid : Invariant m)
    (count : m.history.length = m.events.length + 1) (step : ReferenceTrace.Transition m next) : Invariant next := by
  have fixed : next.bindings = m.bindings → Invariant next := by
    intro rows
    rcases ReferenceChronology.transition_recorded _ _ step with unchanged | recorded
    · simpa [unchanged] using valid
    · obtain ⟨_,event,events⟩ := recorded
      exact same_rows _ _ _ rows events valid
  cases step with
  | management run =>
      obtain ⟨_,_,_,_,equal⟩ := ReferenceStore.manage_parts _ _ _ _ _ _ _ run
      have rows := congrArg (fun pair : ReferenceStore.Machine p a × Option Nat => pair.1.bindings) equal
      exact fixed rows
  | authority => exact fixed rfl
  | start run =>
      simp only [ReferenceStore.start,Option.bind_eq_bind,Option.bind,Option.pure_def] at run
      split at run
      · cases run
      · rename_i pair started
        obtain ⟨core,ticket⟩ := pair
        simp only [Option.some.injEq,Prod.mk.injEq] at run
        obtain ⟨rfl,rfl⟩ := run
        exact fixed rfl
  | finishPlain run =>
      simp only [ReferenceStore.finishPlain,Option.bind_eq_bind,Option.bind,Option.pure_def] at run
      split at run
      · cases run
      · cases run
        exact fixed rfl
  | cancellation run =>
      obtain ⟨_,_,_,rfl⟩ := ReferenceStore.cancel_parts _ _ _ _ _ _ _ _ run
      exact fixed rfl
  | acquire run => exact change_preserves _ _ valid count (acquire_change_record _ _ _ _ _ _ _ run)
  | reacquire run => exact change_preserves _ _ valid count (reacquire_change_record _ _ _ _ _ _ _ run)
  | release run => exact change_preserves _ _ valid count (release_change_record _ _ _ _ _ _ _ run)
  | @normalize member place principal requestId key =>
      rcases normalize_change_record m member place principal requestId key with unchanged | record
      · simpa [unchanged] using valid
      · exact change_preserves _ _ valid count record

theorem reached_preserves (first next : ReferenceStore.Machine p a) (valid : Invariant first)
    (count : first.history.length = first.events.length + 1) (path : ReferenceTrace.Reached first next) : Invariant next := by
  induction path with
  | refl => exact valid
  | @step m next previous step ih =>
      obtain ⟨frames,_,historyAt,eventAt⟩ := ReferenceChronology.reached_history _ _ previous
      have countAt : m.history.length = m.events.length + 1 := by
        rw [historyAt,List.length_append,eventAt,count]
        omega
      exact transition_preserves _ _ ih countAt step

theorem rooted_origins (realm : Nat) (view : WorldProjection.AuthorityView a)
    (policy : Nat → CurrentUse.Policy) (s : ReferenceSource.State p a)
    (path : ReferenceSourceTrace.Rooted realm view policy s) : Invariant s.machine.store := by
  apply reached_preserves (ReferenceStore.initial realm view policy) s.machine.store
  · intro binding present; cases present
  · rfl
  · exact ReferenceChronology.source_reached_store _ _ path

#print axioms transition_preserves
#print axioms reached_preserves
#print axioms rooted_origins
#print axioms indexed_extend
#print axioms change_preserves
end MirroreaProofFirst.ReferenceOrigins
