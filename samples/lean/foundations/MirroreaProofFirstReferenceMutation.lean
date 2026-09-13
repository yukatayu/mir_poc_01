import MirroreaProofFirstReferenceCoherence

namespace MirroreaProofFirst.ReferenceMutation
open ReferenceOwner ReferenceStore ReferenceAccess ReferenceAccessHistory ReferenceSelection

def lookup (m : ReferenceStore.Machine p a) (key : Nat) : Option Binding := (m.bindings[key]?).join

def postCore (m : ReferenceStore.Machine p a) (principal id : Nat) : CompositionMachine.Machine p a :=
  {m.core with system := {m.core.system with
    serial := m.core.system.serial+1
    used := ManagementEntry.useId m.core.system principal id :: m.core.system.used}}

def updatedRows (m : ReferenceStore.Machine p a) : Change → List (Option Binding)
  | .acquire next => m.bindings ++ [some next]
  | .degrade old next | .reacquire old next => m.bindings.set old.request.binding (some next)
  | .release old => m.bindings.set old.request.binding none

def oldStoreCheck (m : ReferenceStore.Machine p a) : Change → Bool
  | .acquire next => decide (next.request.binding = m.bindings.length)
  | .degrade old _ | .reacquire old _ | .release old => decide (lookup m old.request.binding = some old)

-- All state changes below use this checked, non-exported commit path. Pure
-- proposal computation can fail without changing binding, history or allocator.
private def commit (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (change : Change) : Option (ReferenceStore.Machine p a) := do
  if !CompositionMachine.fresh m.core (ManagementEntry.useId m.core.system principal id) || !oldStoreCheck m change then none else do
    let permit ← ReferenceOwner.authorize m.core.system m.events.length change member place principal id
    if !ReferenceOwner.check m.core.system m.events.length change member place principal id permit then none else do
      let next := postCore m principal id
      let rows := updatedRows m change
      if !floorsCheck next.system.configuration.state rows then none else
        some {core := next,bindings := rows,history := m.history ++ [frame next],events := .binding permit.context :: m.events}

theorem postCore_invariant (m : ReferenceStore.Machine p a) (principal id : Nat)
    (valid : CompositionMachine.Invariant m.core)
    (fresh : CompositionMachine.fresh m.core (ManagementEntry.useId m.core.system principal id) = true) :
    CompositionMachine.Invariant (postCore m principal id) := by
  obtain ⟨unused,unpending⟩ := (CompositionMachine.fresh_exact _ _).mp fresh
  refine ⟨⟨valid.1.1,List.nodup_cons.mpr ⟨unused,valid.1.2⟩⟩,valid.2.1,?_⟩
  intro ticket member
  simp only [postCore,List.mem_cons,not_or]
  refine ⟨?_,valid.2.2 ticket member⟩
  intro equal
  exact unpending (equal ▸ List.mem_map.mpr ⟨ticket,member,rfl⟩)

private theorem commit_parts (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (change : Change) (next : ReferenceStore.Machine p a)
    (accepted : commit m member place principal id change = some next) :
    CompositionMachine.fresh m.core (ManagementEntry.useId m.core.system principal id) = true ∧
    ∃ permit, ReferenceOwner.Submitted m.core.system m.events.length change member place principal id permit ∧
      Floors (postCore m principal id).system.configuration.state (updatedRows m change) ∧
      next = {core := postCore m principal id,bindings := updatedRows m change,history := m.history ++ [frame (postCore m principal id)],events := .binding permit.context :: m.events} := by
  unfold commit at accepted
  split at accepted
  · cases accepted
  · rename_i ready
    have fresh : CompositionMachine.fresh m.core (ManagementEntry.useId m.core.system principal id) = true := by
      have both : CompositionMachine.fresh m.core (ManagementEntry.useId m.core.system principal id) = true ∧
          oldStoreCheck m change = true := by simpa using ready
      exact both.1
    cases hp : ReferenceOwner.authorize m.core.system m.events.length change member place principal id with
    | none => simp [hp] at accepted
    | some permit =>
        simp only [hp,Option.bind_eq_bind,Option.bind_some] at accepted
        split at accepted
        · cases accepted
        · rename_i owner
          have checked : ReferenceOwner.check m.core.system m.events.length change member place principal id permit = true := by simpa using owner
          split at accepted
          · cases accepted
          · rename_i floor
            have floorChecked : floorsCheck (postCore m principal id).system.configuration.state (updatedRows m change) = true := by simpa using floor
            exact ⟨fresh,permit,(ReferenceOwner.check_exact _ _ _ _ _ _ _ _).mp checked,
              (floors_exact _ _).mp floorChecked,(Option.some.inj accepted).symm⟩

private theorem commit_preserves (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (change : Change) (next : ReferenceStore.Machine p a)
    (valid : ReferenceStore.Invariant m) (accepted : commit m member place principal id change = some next) :
    ReferenceStore.Invariant next ∧ next.core.system.serial = m.core.system.serial+1 ∧
      next.core.system.view = m.core.system.view ∧ next.core.system.configuration = m.core.system.configuration := by
  obtain ⟨fresh,permit,_,floor,rfl⟩ := commit_parts _ _ _ _ _ _ _ accepted
  exact ⟨⟨postCore_invariant _ _ _ valid.1 fresh,valid.2.1,floor,⟨m.history,rfl⟩,
    by simpa [coreEvents,postCore] using valid.2.2.2.2⟩,rfl,rfl,rfl⟩

-- The window is grounded in the stored activation position. The store entry
-- invariant must establish that it begins at the actual guard-acquisition frame.
def continuouslyLive (m : ReferenceStore.Machine p a) (binding : Binding) (choice : Choice) : Bool :=
  usable (frame m.core) (m.history.drop binding.activation)
    (atIndex binding.request choice.index) choice.guard

def newBinding (m : ReferenceStore.Machine p a) (request : Request) (choice : Option Choice)
    (holding : ReferenceHolding.Stamp) : Binding :=
  ⟨request,choice,m.history.length,m.events.length,holding⟩

def holdingLive (m : ReferenceStore.Machine p a) (binding : Binding) : Bool :=
  ReferenceHolding.liveCheck .separate binding.request binding.holding.guard binding.holding.activation m.history &&
    ReferenceHolding.check .separate m.core.system binding.request binding.holding.guard

def acquire (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (chain : FallbackStatic.Chain) : Option (ReferenceStore.Machine p a × Nat) := do
  let request : Request := ⟨m.bindings.length,0,0,chain,0,member.val,place.val,principal⟩
  let post := postCore m principal id
  let choice ← search post.system (post.system.controlPolicy 13) request 0
  let holding ← ReferenceHolding.prepare .separate post.system request
  let next ← commit m member place principal id (.acquire (newBinding m request (some choice)
    ⟨holding,m.history.length,m.events.length⟩))
  return (next,request.binding)

def reacquire (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) :
    Option (ReferenceStore.Machine p a) := do
  let old ← lookup m key
  let request := {old.request with epoch := old.request.epoch+1,lineage := old.request.lineage+1}
  let post := postCore m principal id
  let choice ← search post.system (post.system.controlPolicy 13) request 0
  let holding ← ReferenceHolding.prepare .separate post.system request
  commit m member place principal id (.reacquire old (newBinding m request (some choice)
    ⟨holding,m.history.length,m.events.length⟩))

def release (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) :
    Option (ReferenceStore.Machine p a) := do
  let old ← lookup m key
  commit m member place principal id (.release old)

inductive Error where
  | absent | exhausted | ownerDenied | ownerNotLive | mutationRejected
  deriving DecidableEq, Repr
structure Outcome (p a : Nat) where
  state : ReferenceStore.Machine p a
  result : Except Error Choice
  consumed : Bool

-- No outer Except/Option can roll back an owner degradation already committed.
-- An exhausted result can therefore contain an updated state and consumed id.
def normalize (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) : Outcome p a :=
  match lookup m key with
  | none => ⟨m,.error .absent,false⟩
  | some old =>
      if !ReferenceOwner.actorCheck m.core.system (.release old) member place principal then ⟨m,.error .ownerDenied,false⟩ else
      if !holdingLive m old then ⟨m,.error .ownerNotLive,false⟩ else
      match old.selected with
      | none => ⟨m,.error .exhausted,false⟩
      | some choice =>
          if continuouslyLive m old choice then ⟨m,.ok choice,false⟩ else
          let post := postCore m principal id
          let selected := search post.system (post.system.controlPolicy 13) old.request (choice.index+1)
          let change := Change.degrade old (newBinding m old.request selected old.holding)
          match commit m member place principal id change with
          | none => ⟨m,.error .mutationRejected,false⟩
          | some next => ⟨next,match selected with | none => .error .exhausted | some chosen => .ok chosen,true⟩

theorem acquire_preserves (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (chain : FallbackStatic.Chain) (result : ReferenceStore.Machine p a × Nat) (valid : ReferenceStore.Invariant m)
    (accepted : acquire m member place principal id chain = some result) : ReferenceStore.Invariant result.1 := by
  unfold acquire at accepted
  dsimp only at accepted
  simp only [Option.bind_eq_bind,Option.bind,Option.pure_def] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i holding held
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i next committed
        cases accepted
        exact (commit_preserves _ _ _ _ _ _ _ valid committed).1

theorem reacquire_preserves (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : ReferenceStore.Machine p a) (valid : ReferenceStore.Invariant m)
    (accepted : reacquire m member place principal id key = some result) : ReferenceStore.Invariant result := by
  unfold reacquire at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i choice selected
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i holding held
        exact (commit_preserves _ _ _ _ _ _ _ valid accepted).1

theorem release_preserves (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : ReferenceStore.Machine p a) (valid : ReferenceStore.Invariant m)
    (accepted : release m member place principal id key = some result) : ReferenceStore.Invariant result := by
  unfold release at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · exact (commit_preserves _ _ _ _ _ _ _ valid accepted).1

theorem normalize_preserves (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (valid : ReferenceStore.Invariant m) : ReferenceStore.Invariant (normalize m member place principal id key).state := by
  unfold normalize
  split
  · exact valid
  · split
    · exact valid
    · split
      · exact valid
      · split
        · exact valid
        · split
          · exact valid
          · dsimp only
            split
            · exact valid
            · rename_i next committed
              exact (commit_preserves _ _ _ _ _ _ _ valid committed).1

theorem normalize_unconsumed_unchanged (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (unconsumed : (normalize m member place principal id key).consumed = false) :
    (normalize m member place principal id key).state = m := by
  have alternatives : (normalize m member place principal id key).state = m ∨
      (normalize m member place principal id key).consumed = true := by
    unfold normalize
    split
    · exact Or.inl rfl
    · split
      · exact Or.inl rfl
      · split
        · exact Or.inl rfl
        · split
          · exact Or.inl rfl
          · split
            · exact Or.inl rfl
            · dsimp only
              split
              · exact Or.inl rfl
              · exact Or.inr rfl
  rcases alternatives with unchanged | consumed
  · exact unchanged
  · simp [unconsumed] at consumed

theorem lookup_row (m : ReferenceStore.Machine p a) (key : Nat) (binding : Binding)
    (found : lookup m key = some binding) : m.bindings[key]? = some (some binding) := by
  unfold lookup at found
  cases atKey : m.bindings[key]? with
  | none => simp [atKey] at found
  | some row =>
      cases row with
      | none => simp [atKey] at found
      | some value =>
          have equal : value = binding := by simpa [atKey] using found
          simp [equal]

theorem newBinding_stored (m : ReferenceStore.Machine p a) (principal id key : Nat) (request : Request)
    (selected : Option Choice) (holding : ReferenceHolding.Stamp)
    (held : ReferenceHolding.Admitted (m.history ++ [frame (postCore m principal id)]) (m.events.length+1) request holding)
    (sameKey : request.binding = key) (canonical : request.optionIndex = 0)
    (admitted : ∀ choice, selected = some choice → choice.index < request.chain.options.length ∧
      Saved (postCore m principal id).system ((postCore m principal id).system.controlPolicy 13)
        (atIndex request choice.index) choice.guard) :
    ReferenceCoherence.Stored (m.history ++ [frame (postCore m principal id)]) (m.events.length+1)
      key (newBinding m request selected holding) := by
  refine ⟨sameKey,canonical,?_,?_,?_,held⟩
  · simp [newBinding]
  · simp [newBinding]
  · intro choice chosen
    have evidence := admitted choice chosen
    exact ⟨evidence.1,frame (postCore m principal id),by simp [newBinding],evidence.2⟩

theorem acquire_coherent (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (chain : FallbackStatic.Chain) (result : ReferenceStore.Machine p a × Nat) (valid : ReferenceCoherence.Invariant m)
    (accepted : acquire m member place principal id chain = some result) : ReferenceCoherence.Invariant result.1 := by
  refine ⟨acquire_preserves _ _ _ _ _ _ _ valid.1 accepted,?_⟩
  unfold acquire at accepted
  dsimp only at accepted
  simp only [Option.bind_eq_bind,Option.bind,Option.pure_def] at accepted
  split at accepted
  · cases accepted
  · rename_i choice selected
    dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i holding held
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i next committed
        cases accepted
        obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ committed
        rw [eq]
        apply ReferenceCoherence.append_rows m.bindings m.history [frame (postCore m principal id)]
          m.events.length (m.events.length+1) valid.2 (by omega)
        apply newBinding_stored m principal id m.bindings.length _ (some choice) _
          (ReferenceHolding.fresh_admitted _ _ _ _ _
            ((ReferenceHolding.check_exact _ _ _ _).mp (ReferenceHolding.prepare_checked _ _ _ _ held))) rfl rfl
        intro chosen isChoice
        cases isChoice
        exact (search_bounds _ _ _ _ _ selected).2

theorem release_coherent (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : ReferenceStore.Machine p a) (valid : ReferenceCoherence.Invariant m)
    (accepted : release m member place principal id key = some result) : ReferenceCoherence.Invariant result := by
  refine ⟨release_preserves _ _ _ _ _ _ _ valid.1 accepted,?_⟩
  unfold release at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · rename_i old found
    obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ accepted
    rw [eq]
    apply ReferenceCoherence.set_rows m.bindings m.history [frame (postCore m principal id)]
      m.events.length (m.events.length+1) valid.2 (by omega) old.request.binding none
    intro binding impossible
    cases impossible

theorem reacquire_coherent (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : ReferenceStore.Machine p a) (valid : ReferenceCoherence.Invariant m)
    (accepted : reacquire m member place principal id key = some result) : ReferenceCoherence.Invariant result := by
  refine ⟨reacquire_preserves _ _ _ _ _ _ _ valid.1 accepted,?_⟩
  unfold reacquire at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · rename_i old found
    have stored := valid.2 key old (lookup_row _ _ _ found)
    dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i choice selected
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i holding held
        obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ accepted
        rw [eq]
        apply ReferenceCoherence.set_rows m.bindings m.history [frame (postCore m principal id)]
          m.events.length (m.events.length+1) valid.2 (by omega) old.request.binding _
        intro binding isNew
        cases isNew
        apply newBinding_stored m principal id old.request.binding
          {old.request with epoch := old.request.epoch+1,lineage := old.request.lineage+1}
          (some choice) _
          (ReferenceHolding.fresh_admitted _ _ _ _ _
            ((ReferenceHolding.check_exact _ _ _ _).mp (ReferenceHolding.prepare_checked _ _ _ _ held))) rfl stored.canonical_request
        intro chosen isChoice
        cases isChoice
        exact (search_bounds _ _ _ _ _ selected).2

theorem normalize_coherent (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (valid : ReferenceCoherence.Invariant m) : ReferenceCoherence.Invariant (normalize m member place principal id key).state := by
  refine ⟨normalize_preserves _ _ _ _ _ _ valid.1,?_⟩
  unfold normalize
  split
  · exact valid.2
  · rename_i old found
    have stored := valid.2 key old (lookup_row _ _ _ found)
    split
    · exact valid.2
    · split
      · exact valid.2
      · split
        · exact valid.2
        · rename_i choice chosen
          split
          · exact valid.2
          · dsimp only
            split
            · exact valid.2
            · rename_i next committed
              obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ committed
              rw [eq]
              apply ReferenceCoherence.set_rows m.bindings m.history [frame (postCore m principal id)]
                m.events.length (m.events.length+1) valid.2 (by omega) old.request.binding _
              intro binding isNew
              cases isNew
              apply newBinding_stored m principal id old.request.binding old.request _ old.holding
                (ReferenceHolding.admitted_extend _ _ _ _ _ _ stored.holding (by omega)) rfl stored.canonical_request
              intro selected success
              exact (search_bounds _ _ _ _ _ success).2

theorem lookup_bound (m : ReferenceStore.Machine p a) (key : Nat) (binding : Binding)
    (found : lookup m key = some binding) : key < m.bindings.length := by
  by_cases inside : key < m.bindings.length
  · exact inside
  · have absent : m.bindings[key]? = none := List.getElem?_eq_none (by omega)
    simp [lookup,absent] at found

theorem degradation_progress (m : ReferenceStore.Machine p a) (principal id : Nat) (old : Binding)
    (choice : Choice) (selected : Option Choice)
    (stored : ReferenceCoherence.Stored m.history m.events.length old.request.binding old)
    (chosen : old.selected = some choice)
    (searched : search (postCore m principal id).system ((postCore m principal id).system.controlPolicy 13)
      old.request (choice.index+1) = selected) :
    ReferenceCoherence.Progressed old (newBinding m old.request selected old.holding) := by
  refine ⟨rfl,Nat.le_refl _,Nat.le_refl _,fun _ => rfl,?_,fun _ => rfl⟩
  intro _
  right
  cases selected with
  | none =>
      simpa [ReferenceCoherence.rank,newBinding,chosen] using (stored.selected choice chosen).1
  | some next =>
      have strict := search_after_strict _ _ _ _ _ searched
      simpa [ReferenceCoherence.rank,newBinding,chosen] using strict

theorem reacquisition_progress (m : ReferenceStore.Machine p a) (old : Binding) (selected : Option Choice)
    (holding : ReferenceHolding.Stamp) :
    ReferenceCoherence.Progressed old (newBinding m
      {old.request with epoch := old.request.epoch+1,lineage := old.request.lineage+1} selected holding) := by
  refine ⟨rfl,by simp [newBinding],by simp [newBinding],?_,?_,?_⟩
  · intro equal
    change old.request.epoch = old.request.epoch+1 at equal
    omega
  · intro equal
    change old.request.epoch = old.request.epoch+1 at equal
    omega
  · intro equal
    change old.request.epoch = old.request.epoch+1 at equal
    omega

theorem normalize_progress (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (before after : Binding) (valid : ReferenceCoherence.Invariant m)
    (old : lookup m key = some before)
    (new : lookup (normalize m member place principal id key).state key = some after) :
    ReferenceCoherence.Progressed before after := by
  have unchanged (same : lookup m key = some after) : ReferenceCoherence.Progressed before after := by
    have equal : before = after := Option.some.inj (old.symm.trans same)
    subst after
    exact ReferenceCoherence.progressed_refl _
  have stored := valid.2 key before (lookup_row _ _ _ old)
  have bound := lookup_bound _ _ _ old
  simp only [normalize,old] at new
  split at new
  · exact unchanged new
  · split at new
    · exact unchanged new
    · split at new
      · exact unchanged new
      · rename_i choice chosen
        split at new
        · exact unchanged new
        · split at new
          · exact unchanged new
          · rename_i next committed
            obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ committed
            change lookup next key = some after at new
            rw [eq] at new
            have resultEq : newBinding m before.request
                (search (postCore m principal id).system ((postCore m principal id).system.controlPolicy 13)
                  before.request (choice.index+1)) before.holding = after := by
              simpa [lookup,updatedRows,stored.key_matches,List.getElem?_set_self bound] using new
            rw [← resultEq]
            exact degradation_progress _ _ _ _ _ _ (stored.key_matches ▸ stored) chosen rfl

theorem normalize_other (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key other : Nat) (valid : ReferenceCoherence.Invariant m) (different : key ≠ other) :
    lookup (normalize m member place principal id key).state other = lookup m other := by
  unfold normalize
  split
  · rfl
  · rename_i old found
    have stored := valid.2 key old (lookup_row _ _ _ found)
    split
    · rfl
    · split
      · rfl
      · split
        · rfl
        · split
          · rfl
          · dsimp only
            split
            · rfl
            · rename_i next committed
              obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ committed
              change lookup next other = lookup m other
              rw [eq]
              simp [lookup,updatedRows,stored.key_matches,List.getElem?_set_ne different]

theorem reacquire_progress (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (before after : Binding) (next : ReferenceStore.Machine p a)
    (valid : ReferenceCoherence.Invariant m) (old : lookup m key = some before)
    (accepted : reacquire m member place principal id key = some next) (new : lookup next key = some after) :
    ReferenceCoherence.Progressed before after ∧ after.request.epoch = before.request.epoch+1 ∧
      after.request.lineage = before.request.lineage+1 := by
  have stored := valid.2 key before (lookup_row _ _ _ old)
  have bound := lookup_bound _ _ _ old
  simp only [reacquire,old,Option.bind_eq_bind,Option.bind_some] at accepted
  simp only [Option.bind] at accepted
  split at accepted
  · cases accepted
  · rename_i choice selected
    dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i holding held
      obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ accepted
      rw [eq] at new
      have resultEq : newBinding m {before.request with epoch := before.request.epoch+1,lineage := before.request.lineage+1}
          (some choice) ⟨holding,m.history.length,m.events.length⟩ = after := by
        simpa [lookup,updatedRows,stored.key_matches,List.getElem?_set_self bound] using new
      rw [← resultEq]
      exact ⟨reacquisition_progress _ _ _ _,rfl,rfl⟩

theorem reacquire_other (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key other : Nat) (next : ReferenceStore.Machine p a) (valid : ReferenceCoherence.Invariant m)
    (different : key ≠ other) (accepted : reacquire m member place principal id key = some next) :
    lookup next other = lookup m other := by
  unfold reacquire at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · rename_i old found
    have stored := valid.2 key old (lookup_row _ _ _ found)
    dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i choice selected
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i holding held
        obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ accepted
        rw [eq]
        simp [lookup,updatedRows,stored.key_matches,List.getElem?_set_ne different]

theorem release_removes (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (next : ReferenceStore.Machine p a) (valid : ReferenceCoherence.Invariant m)
    (accepted : release m member place principal id key = some next) : next.bindings[key]? = some none := by
  unfold release at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · rename_i old found
    have stored := valid.2 key old (lookup_row _ _ _ found)
    have bound := lookup_bound _ _ _ found
    obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ accepted
    rw [eq]
    simpa [updatedRows,stored.key_matches] using (List.getElem?_set_self (a := (none : Option Binding)) bound)

theorem release_other (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key other : Nat) (next : ReferenceStore.Machine p a) (valid : ReferenceCoherence.Invariant m)
    (different : key ≠ other) (accepted : release m member place principal id key = some next) :
    next.bindings[other]? = m.bindings[other]? := by
  unfold release at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · rename_i old found
    have stored := valid.2 key old (lookup_row _ _ _ found)
    obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ accepted
    rw [eq]
    simp [updatedRows,stored.key_matches,List.getElem?_set_ne different]

theorem acquire_old_slots (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (chain : FallbackStatic.Chain) (result : ReferenceStore.Machine p a × Nat)
    (accepted : acquire m member place principal id chain = some result) :
    result.2 = m.bindings.length ∧ result.1.bindings.length = m.bindings.length+1 ∧
      ∀ key, key < m.bindings.length → result.1.bindings[key]? = m.bindings[key]? := by
  unfold acquire at accepted
  dsimp only at accepted
  simp only [Option.bind_eq_bind,Option.bind,Option.pure_def] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i holding held
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i next committed
        cases accepted
        obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ committed
        rw [eq]
        exact ⟨rfl,by simp [updatedRows],fun _ inside => List.getElem?_append_left inside⟩

theorem normalize_other_row (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key other : Nat) (valid : ReferenceCoherence.Invariant m) (different : key ≠ other) :
    (normalize m member place principal id key).state.bindings[other]? = m.bindings[other]? := by
  unfold normalize
  split
  · rfl
  · rename_i old found
    have stored := valid.2 key old (lookup_row _ _ _ found)
    split
    · rfl
    · split
      · rfl
      · split
        · rfl
        · split
          · rfl
          · dsimp only
            split
            · rfl
            · rename_i next committed
              obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ committed
              change next.bindings[other]? = m.bindings[other]?
              rw [eq]
              simp [updatedRows,stored.key_matches,List.getElem?_set_ne different]

theorem reacquire_other_row (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key other : Nat) (next : ReferenceStore.Machine p a) (valid : ReferenceCoherence.Invariant m)
    (different : key ≠ other) (accepted : reacquire m member place principal id key = some next) :
    next.bindings[other]? = m.bindings[other]? := by
  unfold reacquire at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · rename_i old found
    have stored := valid.2 key old (lookup_row _ _ _ found)
    dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i choice selected
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i holding held
        obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ accepted
        rw [eq]
        simp [updatedRows,stored.key_matches,List.getElem?_set_ne different]

theorem normalize_length (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) : (normalize m member place principal id key).state.bindings.length = m.bindings.length := by
  unfold normalize
  split
  · rfl
  · split
    · rfl
    · split
      · rfl
      · split
        · rfl
        · split
          · rfl
          · dsimp only
            split
            · rfl
            · rename_i next committed
              obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ committed
              change next.bindings.length = m.bindings.length
              rw [eq]
              simp [updatedRows]

theorem reacquire_length (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (next : ReferenceStore.Machine p a)
    (accepted : reacquire m member place principal id key = some next) : next.bindings.length = m.bindings.length := by
  unfold reacquire at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i choice selected
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i holding held
        obtain ⟨_,permit,_,_,eq⟩ := commit_parts _ _ _ _ _ _ _ accepted
        rw [eq]
        simp [updatedRows]

private theorem commit_pending (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (change : Change) (next : ReferenceStore.Machine p a)
    (accepted : commit m member place principal id change = some next) : next.core.pending = m.core.pending := by
  obtain ⟨_,_,_,_,rfl⟩ := commit_parts _ _ _ _ _ _ _ accepted
  rfl

theorem acquire_pending (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (chain : FallbackStatic.Chain) (result : ReferenceStore.Machine p a × Nat)
    (accepted : acquire m member place principal id chain = some result) : result.1.core.pending = m.core.pending := by
  unfold acquire at accepted
  dsimp only at accepted
  simp only [Option.bind_eq_bind,Option.bind,Option.pure_def] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i holding held
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i next committed
        cases accepted
        exact commit_pending _ _ _ _ _ _ _ committed

theorem reacquire_pending (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : ReferenceStore.Machine p a) (accepted : reacquire m member place principal id key = some result) :
    result.core.pending = m.core.pending := by
  unfold reacquire at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i choice selected
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i holding held
        exact commit_pending _ _ _ _ _ _ _ accepted

theorem release_pending (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : ReferenceStore.Machine p a) (accepted : release m member place principal id key = some result) :
    result.core.pending = m.core.pending := by
  unfold release at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · exact commit_pending _ _ _ _ _ _ _ accepted

theorem normalize_pending (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat) :
    (normalize m member place principal id key).state.core.pending = m.core.pending := by
  unfold normalize
  split
  · rfl
  · split
    · rfl
    · split
      · rfl
      · split
        · rfl
        · split
          · rfl
          · dsimp only
            split
            · rfl
            · rename_i next committed
              exact commit_pending _ _ _ _ _ _ _ committed

private theorem commit_core (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (change : Change) (next : ReferenceStore.Machine p a)
    (accepted : commit m member place principal id change = some next) : next.core = postCore m principal id := by
  obtain ⟨_,_,_,_,rfl⟩ := commit_parts _ _ _ _ _ _ _ accepted
  rfl

theorem acquire_core (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (chain : FallbackStatic.Chain) (result : ReferenceStore.Machine p a × Nat)
    (accepted : acquire m member place principal id chain = some result) : result.1.core = postCore m principal id := by
  unfold acquire at accepted
  dsimp only at accepted
  simp only [Option.bind_eq_bind,Option.bind,Option.pure_def] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i holding held
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i next committed
        cases accepted
        exact commit_core _ _ _ _ _ _ _ committed

theorem reacquire_core (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : ReferenceStore.Machine p a) (accepted : reacquire m member place principal id key = some result) :
    result.core = postCore m principal id := by
  unfold reacquire at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i choice selected
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i holding held
        exact commit_core _ _ _ _ _ _ _ accepted

theorem release_core (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (result : ReferenceStore.Machine p a) (accepted : release m member place principal id key = some result) :
    result.core = postCore m principal id := by
  unfold release at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · exact commit_core _ _ _ _ _ _ _ accepted

theorem normalize_consumed_core (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (consumed : (normalize m member place principal id key).consumed = true) :
    (normalize m member place principal id key).state.core = postCore m principal id := by
  have casesOfRun : (normalize m member place principal id key).consumed = false ∨
      (normalize m member place principal id key).state.core = postCore m principal id := by
    unfold normalize
    split
    · exact Or.inl rfl
    · split
      · exact Or.inl rfl
      · split
        · exact Or.inl rfl
        · split
          · exact Or.inl rfl
          · split
            · exact Or.inl rfl
            · dsimp only
              split
              · exact Or.inl rfl
              · rename_i next committed
                exact Or.inr (commit_core _ _ _ _ _ _ _ committed)
  rcases casesOfRun with unused | changed
  · simp [consumed] at unused
  · exact changed

#print axioms acquire_core
#print axioms reacquire_core
#print axioms release_core
#print axioms normalize_consumed_core
#print axioms acquire_pending
#print axioms reacquire_pending
#print axioms release_pending
#print axioms normalize_pending
#print axioms postCore_invariant
#print axioms commit_parts
#print axioms commit_preserves
private theorem commit_recorded (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (change : Change) (next : ReferenceStore.Machine p a)
    (accepted : commit m member place principal id change = some next) :
    next.history = m.history ++ [frame next.core] ∧ ∃ event, next.events = event :: m.events := by
  obtain ⟨_,permit,_,_,rfl⟩ := commit_parts _ _ _ _ _ _ _ accepted
  exact ⟨rfl,_,rfl⟩

theorem acquire_recorded (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (chain : FallbackStatic.Chain) (result : ReferenceStore.Machine p a × Nat)
    (accepted : acquire m member place principal id chain = some result) :
    result.1.history = m.history ++ [frame result.1.core] ∧ ∃ event, result.1.events = event :: m.events := by
  unfold acquire at accepted
  dsimp only at accepted
  simp only [Option.bind_eq_bind,Option.bind,Option.pure_def] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i holding held
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i next committed
        cases accepted
        exact commit_recorded _ _ _ _ _ _ _ committed

theorem reacquire_recorded (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (result : ReferenceStore.Machine p a)
    (accepted : reacquire m member place principal id key = some result) :
    result.history = m.history ++ [frame result.core] ∧ ∃ event, result.events = event :: m.events := by
  unfold reacquire at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i choice selected
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i holding held
        exact commit_recorded _ _ _ _ _ _ _ accepted

theorem release_recorded (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (result : ReferenceStore.Machine p a)
    (accepted : release m member place principal id key = some result) :
    result.history = m.history ++ [frame result.core] ∧ ∃ event, result.events = event :: m.events := by
  unfold release at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · exact commit_recorded _ _ _ _ _ _ _ accepted

theorem normalize_recorded (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) :
    (normalize m member place principal id key).state = m ∨
      ((normalize m member place principal id key).state.history = m.history ++ [frame (normalize m member place principal id key).state.core] ∧
        ∃ event, (normalize m member place principal id key).state.events = event :: m.events) := by
  unfold normalize
  split
  · exact Or.inl rfl
  · split
    · exact Or.inl rfl
    · split
      · exact Or.inl rfl
      · split
        · exact Or.inl rfl
        · split
          · exact Or.inl rfl
          · dsimp only
            split
            · exact Or.inl rfl
            · rename_i next committed
              exact Or.inr (commit_recorded _ _ _ _ _ _ _ committed)

#print axioms acquire_recorded
#print axioms reacquire_recorded
#print axioms release_recorded
#print axioms normalize_recorded
#print axioms acquire_preserves
#print axioms reacquire_preserves
#print axioms release_preserves
#print axioms normalize_preserves
#print axioms normalize_unconsumed_unchanged
#print axioms lookup_row
#print axioms newBinding_stored
#print axioms acquire_coherent
#print axioms release_coherent
#print axioms reacquire_coherent
#print axioms normalize_coherent
#print axioms lookup_bound
#print axioms degradation_progress
#print axioms reacquisition_progress
#print axioms normalize_progress
#print axioms normalize_other
#print axioms reacquire_progress
#print axioms reacquire_other
#print axioms release_removes
#print axioms release_other
#print axioms acquire_old_slots
#print axioms normalize_other_row
#print axioms reacquire_other_row
#print axioms normalize_length
#print axioms reacquire_length
-- Stronger event-origin facts are derived from actual mutators, separately
-- from historical guard validity. The metadata rule distinguishes acquisition
-- of a hold from a degradation which must retain the original hold stamp.
def FreshMetadata (m : ReferenceStore.Machine p a) (binding : Binding) : Prop :=
  binding.frontier = m.events.length ∧ binding.activation = m.history.length

def Metadata (m : ReferenceStore.Machine p a) : Change → Prop
  | .acquire next | .reacquire _ next =>
      FreshMetadata m next ∧ next.holding.frontier = m.events.length ∧ next.holding.activation = m.history.length
  | .degrade old next => FreshMetadata m next ∧ next.request = old.request ∧ next.holding = old.holding
  | .release _ => True

def ChangeRecord (m next : ReferenceStore.Machine p a) : Prop :=
  ∃ change, oldStoreCheck m change = true ∧ next.bindings = updatedRows m change ∧ Metadata m change ∧
    ∃ context, context.change = change ∧ context.occurrenceFrontier = m.events.length ∧
      next.events = .binding context :: m.events

private theorem commit_old_checked (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (change : Change) (next : ReferenceStore.Machine p a)
    (accepted : commit m member place principal id change = some next) : oldStoreCheck m change = true := by
  unfold commit at accepted
  split at accepted
  · cases accepted
  · rename_i ready
    have both : CompositionMachine.fresh m.core (ManagementEntry.useId m.core.system principal id) = true ∧
        oldStoreCheck m change = true := by simpa using ready
    exact both.2

private theorem commit_change_record (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (change : Change) (next : ReferenceStore.Machine p a)
    (metadata : Metadata m change) (accepted : commit m member place principal id change = some next) : ChangeRecord m next := by
  have old := commit_old_checked _ _ _ _ _ _ _ accepted
  obtain ⟨_,permit,submitted,_,rfl⟩ := commit_parts _ _ _ _ _ _ _ accepted
  have exactContext := submitted.2.1
  exact ⟨change,old,rfl,metadata,permit.context,by rw [exactContext]; rfl,by rw [exactContext]; rfl,rfl⟩

theorem acquire_change_record (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (chain : FallbackStatic.Chain) (result : ReferenceStore.Machine p a × Nat)
    (accepted : acquire m member place principal id chain = some result) :
    ChangeRecord m result.1 := by
  unfold acquire at accepted
  dsimp only at accepted
  simp only [Option.bind_eq_bind,Option.bind,Option.pure_def] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i holding held
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i next committed
        cases accepted
        exact commit_change_record _ _ _ _ _ _ _ (by simp [Metadata,FreshMetadata,newBinding]) committed

theorem reacquire_change_record (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (result : ReferenceStore.Machine p a)
    (accepted : reacquire m member place principal id key = some result) :
    ChangeRecord m result := by
  unfold reacquire at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i choice selected
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i holding held
        exact commit_change_record _ _ _ _ _ _ _ (by simp [Metadata,FreshMetadata,newBinding]) accepted

theorem release_change_record (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (result : ReferenceStore.Machine p a)
    (accepted : release m member place principal id key = some result) :
    ChangeRecord m result := by
  unfold release at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · exact commit_change_record _ _ _ _ _ _ _ (by simp [Metadata,FreshMetadata,newBinding]) accepted

theorem normalize_change_record (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) :
    (normalize m member place principal id key).state = m ∨
      ChangeRecord m (normalize m member place principal id key).state := by
  unfold normalize
  split
  · exact Or.inl rfl
  · split
    · exact Or.inl rfl
    · split
      · exact Or.inl rfl
      · split
        · exact Or.inl rfl
        · split
          · exact Or.inl rfl
          · dsimp only
            split
            · exact Or.inl rfl
            · rename_i next committed
              exact Or.inr (commit_change_record _ _ _ _ _ _ _ (by simp [Metadata,FreshMetadata,newBinding]) committed)

#print axioms acquire_change_record
#print axioms reacquire_change_record
#print axioms release_change_record
#print axioms normalize_change_record
-- Exact occurrence kinds, derived from the mutator and its checked context.
private theorem commit_event (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (change : Change) (next : ReferenceStore.Machine p a)
    (accepted : commit m member place principal id change = some next) :
    ∃ context, context.change = change ∧ next.events = .binding context :: m.events := by
  obtain ⟨_,permit,submitted,_,rfl⟩ := commit_parts _ _ _ _ _ _ _ accepted
  exact ⟨permit.context,by rw [submitted.2.1]; rfl,rfl⟩

theorem acquire_event (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id : Nat) (chain : FallbackStatic.Chain) (result : ReferenceStore.Machine p a × Nat)
    (accepted : acquire m member place principal id chain = some result) :
    ∃ context, (∃ next, context.change = .acquire next) ∧ result.1.events = .binding context :: m.events := by
  unfold acquire at accepted
  dsimp only at accepted
  simp only [Option.bind_eq_bind,Option.bind,Option.pure_def] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i holding held
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i next committed
        cases accepted
        obtain ⟨context,kind,events⟩ := commit_event _ _ _ _ _ _ _ committed
        exact ⟨context,⟨_,kind⟩,events⟩

theorem reacquire_event (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (result : ReferenceStore.Machine p a)
    (accepted : reacquire m member place principal id key = some result) :
    ∃ context, (∃ old next, context.change = .reacquire old next) ∧ result.events = .binding context :: m.events := by
  unfold reacquire at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · dsimp only at accepted
    split at accepted
    · cases accepted
    · rename_i choice selected
      dsimp only at accepted
      split at accepted
      · cases accepted
      · rename_i holding held
        obtain ⟨context,kind,events⟩ := commit_event _ _ _ _ _ _ _ accepted
        exact ⟨context,⟨_,_,kind⟩,events⟩

theorem release_event (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
    (principal id key : Nat) (result : ReferenceStore.Machine p a)
    (accepted : release m member place principal id key = some result) :
    ∃ context, (∃ old, context.change = .release old) ∧ result.events = .binding context :: m.events := by
  unfold release at accepted
  simp only [Option.bind_eq_bind,Option.bind] at accepted
  split at accepted
  · cases accepted
  · obtain ⟨context,kind,events⟩ := commit_event _ _ _ _ _ _ _ accepted
    exact ⟨context,⟨_,kind⟩,events⟩

theorem normalize_consumed_event (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (consumed : (normalize m member place principal id key).consumed = true) :
    ∃ context old next, context.change = .degrade old next ∧
      (normalize m member place principal id key).state.events = .binding context :: m.events := by
  have casesOfRun : (normalize m member place principal id key).consumed = false ∨
      ∃ context old next, context.change = .degrade old next ∧
      (normalize m member place principal id key).state.events = .binding context :: m.events := by
    unfold normalize
    split
    · exact Or.inl rfl
    · split
      · exact Or.inl rfl
      · split
        · exact Or.inl rfl
        · split
          · exact Or.inl rfl
          · split
            · exact Or.inl rfl
            · dsimp only
              split
              · exact Or.inl rfl
              · rename_i next committed
                obtain ⟨context,kind,events⟩ := commit_event _ _ _ _ _ _ _ committed
                exact Or.inr ⟨context,_,_,kind,events⟩
  rcases casesOfRun with unused | changed
  · simp [consumed] at unused
  · exact changed

#print axioms acquire_event
#print axioms reacquire_event
#print axioms release_event
#print axioms normalize_consumed_event
end MirroreaProofFirst.ReferenceMutation
