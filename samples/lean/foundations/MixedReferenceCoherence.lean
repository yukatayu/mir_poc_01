import MixedReferenceStore
import MirroreaProofFirstReferenceCoherence

namespace MirroreaProofFirst.MixedReferenceCoherence
open ReferenceOwner MixedReferenceStore MixedReferenceAccess MixedReferenceHistory ReferenceSelection

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
  holding : MixedReferenceHolding.Admitted history eventCount binding.request binding.holding

def RowsStored (m : MixedReferenceStore.Machine p a) : Prop :=
  ∀ key binding, m.bindings[key]? = some (some binding) → Stored m.history m.events.length key binding
def Invariant (m : MixedReferenceStore.Machine p a) : Prop := MixedReferenceStore.Invariant m ∧ RowsStored m

theorem initial_invariant (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy) :
    Invariant (MixedReferenceStore.initial (p:=p) realm view policy) := by
  refine ⟨MixedReferenceStore.initial_invariant _ _ _,?_⟩
  intro key binding atKey
  simp [MixedReferenceStore.initial] at atKey

-- Binding payload and progress ordering are exactly the existing pure data.
abbrev rank := ReferenceCoherence.rank
abbrev stableRequest := ReferenceCoherence.stableRequest
abbrev Progressed := ReferenceCoherence.Progressed
abbrev progressed_refl := @ReferenceCoherence.progressed_refl
abbrev progressed_trans := @ReferenceCoherence.progressed_trans
abbrev no_same_rank_refresh := @ReferenceCoherence.no_same_rank_refresh
abbrev same_epoch_nondecreasing := @ReferenceCoherence.same_epoch_nondecreasing

theorem stored_extend (history extra : List (Frame p a)) (before after key : Nat) (binding : Binding)
    (stored : Stored history before key binding) (events : before ≤ after) :
    Stored (history ++ extra) after key binding := by
  refine ⟨stored.key_matches,stored.canonical_request,?_,Nat.lt_of_lt_of_le stored.frontier_bound events,?_,
    MixedReferenceHolding.admitted_extend _ _ _ _ _ _ stored.holding events⟩
  · have bound := stored.activation_bound
    simp only [List.length_append]; omega
  · intro choice selected
    obtain ⟨bound,atActivation,atIndexEq,meaning⟩ := stored.selected choice selected
    exact ⟨bound,atActivation,by simpa [List.getElem?_append_left stored.activation_bound] using atIndexEq,meaning⟩

theorem advanceCore_rows (m : MixedReferenceStore.Machine p a) (next : MixedRequestCore.Machine p a)
    (event : MixedRequestCore.Occurrence) (rows : RowsStored m) : RowsStored (advanceCore m next event) := by
  intro key binding atKey
  exact stored_extend _ _ _ _ _ _ (rows key binding atKey) (by simp [advanceCore])

theorem manage_preserves (m : MixedReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (raw : MixedCompositionCore.Raw) (result : MixedReferenceStore.Machine p a × Option Nat) (valid : Invariant m)
    (accepted : MixedReferenceStore.manage m member place principal id raw = some result) : Invariant result.1 := by
  refine ⟨(MixedReferenceStore.manage_preserves _ _ _ _ _ _ _ valid.1 accepted).1,?_⟩
  obtain ⟨next,created,_,_,rfl⟩ := MixedReferenceStore.manage_parts _ _ _ _ _ _ _ accepted
  exact advanceCore_rows _ _ _ valid.2

theorem authorityHead_preserves (m : MixedReferenceStore.Machine p a) (view : WorldProjection.AuthorityView a)
    (valid : Invariant m) : Invariant (MixedReferenceStore.authorityHead m view) :=
  ⟨(MixedReferenceStore.authorityHead_preserves _ _ valid.1).1,advanceCore_rows _ _ _ valid.2⟩

theorem start_preserves (m : MixedReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id key : Nat)
    (argument : Int) (result : MixedReferenceStore.Machine p a × InvocationBoundary.Ticket) (valid : Invariant m)
    (accepted : MixedReferenceStore.start m member place principal id key argument = some result) : Invariant result.1 := by
  refine ⟨(MixedReferenceStore.start_preserves _ _ _ _ _ _ _ _ valid.1 accepted).1,?_⟩
  unfold MixedReferenceStore.start at accepted
  cases hr : MixedRequestCore.startPure m.core member place principal id key argument with
  | none => simp [hr] at accepted
  | some pair =>
      obtain ⟨next,ticket⟩ := pair
      simp only [hr,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
      subst result
      exact advanceCore_rows _ _ _ valid.2

theorem finishPlain_preserves (m : MixedReferenceStore.Machine p a) (ticket : InvocationBoundary.Ticket) (value : Int)
    (result : MixedReferenceStore.Machine p a) (valid : Invariant m)
    (accepted : MixedReferenceStore.finishPlain m ticket value = some result) : Invariant result := by
  refine ⟨(MixedReferenceStore.finishPlain_preserves _ _ _ _ valid.1 accepted).1,?_⟩
  unfold MixedReferenceStore.finishPlain at accepted
  cases hr : MixedRequestCore.finishPure m.core ticket value with
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

theorem cancel_preserves (m : MixedReferenceStore.Machine p a) (member : Fin a) (place : Fin p) (principal id : Nat)
    (entry : ReferenceExecution.Pending) (permit : ReferenceCancellation.Permit) (result : MixedReferenceStore.Machine p a)
    (valid : Invariant m) (accepted : MixedReferenceStore.cancel m member place principal id entry permit = some result) : Invariant result := by
  refine ⟨(MixedReferenceStore.cancel_preserves _ _ _ _ _ _ _ _ valid.1 accepted).1,?_⟩
  obtain ⟨_,next,_,rfl⟩ := MixedReferenceStore.cancel_parts _ _ _ _ _ _ _ _ accepted
  intro key binding atKey
  exact stored_extend _ _ _ _ _ _ (valid.2 key binding atKey) (by simp [MixedReferenceStore.cancelled])

theorem startOwner_preserves (m : MixedReferenceStore.Machine p a) (member : Fin a) (place : Fin p)
 (principal id key : Nat) (origin : OwnerEffectService.Origin) (args : List CurrentUse.Scalar)
 (result : MixedReferenceStore.Machine p a × OwnerSavedPending.Saved) (valid : Invariant m)
 (accepted : MixedReferenceStore.startOwner m member place principal id key origin args = some result) :
 Invariant result.1 := by
 refine ⟨(MixedReferenceStore.startOwner_preserves _ _ _ _ _ _ _ _ _ valid.1 accepted).1,?_⟩
 unfold MixedReferenceStore.startOwner at accepted
 cases run : MixedRequestCore.startOwner m.core member place principal id key origin args with
 | none => simp [run] at accepted
 | some pair =>
   obtain ⟨next,saved⟩ := pair
   simp only [run,Option.bind_eq_bind,Option.bind_some,Option.pure_def,Option.some.injEq] at accepted
   subst result
   exact advanceCore_rows _ _ _ valid.2

theorem finishOwner_preserves (m : MixedReferenceStore.Machine p a) (valid : Invariant m)
 (accepted : MixedReferenceStore.finishOwner m saved received evidence = some next) : Invariant next := by
 refine ⟨(MixedReferenceStore.finishOwner_preserves _ valid.1 accepted).1,?_⟩
 obtain ⟨_,_,rfl⟩ := MixedReferenceStore.finishOwner_parts m accepted
 exact advanceCore_rows _ _ _ valid.2
#print axioms finishOwner_preserves
#print axioms startOwner_preserves
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
end MirroreaProofFirst.MixedReferenceCoherence
