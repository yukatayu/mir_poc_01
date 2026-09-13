import MirroreaProofFirstReferenceStore

namespace MirroreaProofFirst.ReferenceCoherence
open ReferenceOwner ReferenceStore ReferenceAccess ReferenceAccessHistory ReferenceSelection

-- Existence/coherence of a stored binding is independent of current usability.
-- Its saved selection was admitted at its actual activation frame; subsequent
-- semantic invalidation does not erase that historical fact or grant mutation.
structure Stored (history : List (Frame p a)) (eventCount key : Nat) (binding : Binding) : Prop where
  key_matches : binding.request.binding = key
  canonical_request : binding.request.optionIndex = 0
  activation_bound : binding.activation < history.length
  frontier_bound : binding.frontier < eventCount
  selected : ∀ choice, binding.selected = some choice →
    choice.index < binding.request.chain.options.length ∧
    ∃ atActivation, history[binding.activation]? = some atActivation ∧
      Saved atActivation.system atActivation.policy (atIndex binding.request choice.index) choice.guard
  holding : ReferenceHolding.Admitted history eventCount binding.request binding.holding

def RowsStored (m : ReferenceStore.Machine p a) : Prop :=
  ∀ key binding, m.bindings[key]? = some (some binding) → Stored m.history m.events.length key binding
def Invariant (m : ReferenceStore.Machine p a) : Prop := ReferenceStore.Invariant m ∧ RowsStored m

theorem initial_invariant (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy) :
    Invariant (ReferenceStore.initial (p:=p) realm view policy) := by
  refine ⟨ReferenceStore.initial_invariant _ _ _,?_⟩
  intro key binding atKey
  simp [ReferenceStore.initial] at atKey

def rank (binding : Binding) : Nat :=
  match binding.selected with | none => binding.request.chain.options.length | some choice => choice.index

-- This erasure is ONLY a relational comparison of stable ownership/definition;
-- authorization and saved evidence continue to bind the full un-erased request.
def stableRequest (request : Request) : Request := {request with epoch := 0,lineage := 0}

structure Progressed (before after : Binding) : Prop where
  stable : stableRequest before.request = stableRequest after.request
  epoch : before.request.epoch ≤ after.request.epoch
  lineage : before.request.lineage ≤ after.request.lineage
  same_epoch_lineage : before.request.epoch = after.request.epoch → before.request.lineage = after.request.lineage
  -- Stronger than nondecreasing index: at the same epoch/rank the ENTIRE saved
  -- binding is unchanged, excluding implicit replacement of its witness/clock.
  same_epoch : before.request.epoch = after.request.epoch → before = after ∨ rank before < rank after
  same_hold : before.request.epoch = after.request.epoch → before.holding = after.holding

theorem progressed_refl (binding : Binding) : Progressed binding binding :=
  ⟨rfl,Nat.le_refl _,Nat.le_refl _,fun _ => rfl,fun _ => Or.inl rfl,fun _ => rfl⟩

theorem progressed_trans (first middle last : Binding) (left : Progressed first middle) (right : Progressed middle last) :
    Progressed first last := by
  refine ⟨left.stable.trans right.stable,Nat.le_trans left.epoch right.epoch,
    Nat.le_trans left.lineage right.lineage,?_,?_,?_⟩
  · intro equal
    have firstEq : first.request.epoch = middle.request.epoch := by have := left.epoch; have := right.epoch; omega
    have lastEq : middle.request.epoch = last.request.epoch := by have := left.epoch; have := right.epoch; omega
    exact (left.same_epoch_lineage firstEq).trans (right.same_epoch_lineage lastEq)
  · intro equal
    have firstEq : first.request.epoch = middle.request.epoch := by have := left.epoch; have := right.epoch; omega
    have lastEq : middle.request.epoch = last.request.epoch := by have := left.epoch; have := right.epoch; omega
    rcases left.same_epoch firstEq with same | smaller
    · subst middle; exact right.same_epoch lastEq
    · rcases right.same_epoch lastEq with same | later
      · subst last; exact Or.inr smaller
      · exact Or.inr (Nat.lt_trans smaller later)
  · intro equal
    have firstEq : first.request.epoch = middle.request.epoch := by have := left.epoch; have := right.epoch; omega
    have lastEq : middle.request.epoch = last.request.epoch := by have := left.epoch; have := right.epoch; omega
    exact (left.same_hold firstEq).trans (right.same_hold lastEq)

theorem no_same_rank_refresh (before after : Binding) (progress : Progressed before after)
    (epoch : before.request.epoch = after.request.epoch) (sameRank : rank before = rank after) : before = after := by
  rcases progress.same_epoch epoch with same | smaller
  · exact same
  · omega

theorem same_epoch_nondecreasing (before after : Binding) (progress : Progressed before after)
    (epoch : before.request.epoch = after.request.epoch) : rank before ≤ rank after := by
  rcases progress.same_epoch epoch with same | smaller
  · subst after; exact Nat.le_refl _
  · exact Nat.le_of_lt smaller

theorem stored_extend (history extra : List (Frame p a)) (before after key : Nat) (binding : Binding)
    (stored : Stored history before key binding) (events : before ≤ after) :
    Stored (history ++ extra) after key binding := by
  refine ⟨stored.key_matches,stored.canonical_request,?_,Nat.lt_of_lt_of_le stored.frontier_bound events,?_,
    ReferenceHolding.admitted_extend _ _ _ _ _ _ stored.holding events⟩
  · have bound := stored.activation_bound
    simp only [List.length_append]; omega
  · intro choice selected
    obtain ⟨bound,atActivation,atIndexEq,meaning⟩ := stored.selected choice selected
    exact ⟨bound,atActivation,by simpa [List.getElem?_append_left stored.activation_bound] using atIndexEq,meaning⟩

theorem advanceCore_rows (m : ReferenceStore.Machine p a) (next : CompositionMachine.Machine p a)
    (event : CompositionMachine.Occurrence) (rows : RowsStored m) : RowsStored (advanceCore m next event) := by
  intro key binding atKey
  exact stored_extend _ _ _ _ _ _ (rows key binding atKey) (by simp [advanceCore])

theorem manage_preserves (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : CompositionCore.Raw) (result : ReferenceStore.Machine p a × Option Nat) (valid : Invariant m)
    (accepted : ReferenceStore.manage m member place principal id raw = some result) : Invariant result.1 := by
  refine ⟨(ReferenceStore.manage_preserves _ _ _ _ _ _ _ valid.1 accepted).1,?_⟩
  obtain ⟨next,created,_,_,rfl⟩ := ReferenceStore.manage_parts _ _ _ _ _ _ _ accepted
  exact advanceCore_rows _ _ _ valid.2

theorem authorityHead_preserves (m : ReferenceStore.Machine p a) (view : WorldProjection.AuthorityView a)
    (valid : Invariant m) : Invariant (ReferenceStore.authorityHead m view) :=
  ⟨(ReferenceStore.authorityHead_preserves _ _ valid.1).1,advanceCore_rows _ _ _ valid.2⟩

theorem start_preserves (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : ReferenceStore.Machine p a × InvocationBoundary.Ticket) (valid : Invariant m)
    (accepted : ReferenceStore.start m member place principal id key argument = some result) : Invariant result.1 := by
  refine ⟨(ReferenceStore.start_preserves _ _ _ _ _ _ _ _ valid.1 accepted).1,?_⟩
  unfold ReferenceStore.start at accepted
  cases hr : CompositionMachine.start m.core member place principal id key argument with
  | none => simp [hr] at accepted
  | some pair =>
      obtain ⟨next,ticket⟩ := pair
      simp only [hr,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      exact advanceCore_rows _ _ _ valid.2

theorem finishPlain_preserves (m : ReferenceStore.Machine p a) (ticket : InvocationBoundary.Ticket) (value : Int)
    (result : ReferenceStore.Machine p a) (valid : Invariant m)
    (accepted : ReferenceStore.finishPlain m ticket value = some result) : Invariant result := by
  refine ⟨(ReferenceStore.finishPlain_preserves _ _ _ _ valid.1 accepted).1,?_⟩
  unfold ReferenceStore.finishPlain at accepted
  cases hr : CompositionMachine.finish m.core ticket value with
  | none => simp [hr] at accepted
  | some next =>
      simp only [hr,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      exact advanceCore_rows _ _ _ valid.2

-- Inserting a new current row never remaps the older rows. Releasing a row
-- writes none at its existing slot; that slot is not recycled by append.
theorem append_rows (rows : List (Option Binding)) (history extra : List (Frame p a)) (before after : Nat)
    (old : ∀ key binding, rows[key]? = some (some binding) → Stored history before key binding)
    (events : before ≤ after) (new : Binding) (validNew : Stored (history ++ extra) after rows.length new) :
    ∀ key binding, (rows ++ [some new])[key]? = some (some binding) →
      Stored (history ++ extra) after key binding := by
  intro key binding atKey
  by_cases earlier : key < rows.length
  · rw [List.getElem?_append_left earlier] at atKey
    exact stored_extend _ _ _ _ _ _ (old key binding atKey) events
  · rw [List.getElem?_append_right (by omega)] at atKey
    have equal : key = rows.length := by
      by_cases same : key = rows.length
      · exact same
      · have positive : 0 < key-rows.length := by omega
        have missing : ([some new] : List (Option Binding))[key-rows.length]? = none :=
          List.getElem?_eq_none (by simp; omega)
        rw [missing] at atKey
        cases atKey
    subst key
    simp only [Nat.sub_self,List.getElem?_cons_zero,Option.some.injEq] at atKey
    subst binding
    exact validNew

theorem set_rows (rows : List (Option Binding)) (history extra : List (Frame p a)) (before after : Nat)
    (old : ∀ key binding, rows[key]? = some (some binding) → Stored history before key binding)
    (events : before ≤ after) (target : Nat) (new : Option Binding)
    (validNew : ∀ binding, new = some binding → Stored (history ++ extra) after target binding) :
    ∀ key binding, (rows.set target new)[key]? = some (some binding) →
      Stored (history ++ extra) after key binding := by
  intro key binding atKey
  by_cases same : target = key
  · subst key
    rw [List.getElem?_set] at atKey
    simp only [ite_true] at atKey
    split at atKey
    · exact validNew binding (Option.some.inj atKey)
    · cases atKey
  · rw [List.getElem?_set_ne same] at atKey
    exact stored_extend _ _ _ _ _ _ (old key binding atKey) events

theorem cancel_preserves (m : ReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (entry : ReferenceExecution.Pending) (permit : ReferenceCancellation.Permit) (result : ReferenceStore.Machine p a)
    (valid : Invariant m) (accepted : ReferenceStore.cancel m member place principal id entry permit = some result) : Invariant result := by
  refine ⟨(ReferenceStore.cancel_preserves _ _ _ _ _ _ _ _ valid.1 accepted).1,?_⟩
  obtain ⟨_,next,_,rfl⟩ := ReferenceStore.cancel_parts _ _ _ _ _ _ _ _ accepted
  intro key binding atKey
  exact stored_extend _ _ _ _ _ _ (valid.2 key binding atKey) (by simp [ReferenceStore.cancelled])

#print axioms cancel_preserves
#print axioms stored_extend
#print axioms advanceCore_rows
#print axioms manage_preserves
#print axioms authorityHead_preserves
#print axioms start_preserves
#print axioms finishPlain_preserves
#print axioms append_rows
#print axioms set_rows
#print axioms progressed_refl
#print axioms progressed_trans
#print axioms no_same_rank_refresh
#print axioms same_epoch_nondecreasing
#print axioms initial_invariant
end MirroreaProofFirst.ReferenceCoherence
