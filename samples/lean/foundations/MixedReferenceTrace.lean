import MixedReferenceMutation
import MirroreaProofFirstReferenceTrace

namespace MirroreaProofFirst.MixedReferenceTrace
open ReferenceOwner MixedReferenceStore MixedReferenceMutation MixedReferenceCoherence

-- Raw slot status distinguishes never allocated from a released tombstone.
-- This is a preservation relation, not the full operational admissibility rule.
abbrev SlotProgress := ReferenceTrace.SlotProgress
abbrev slot_refl := ReferenceTrace.slot_refl
abbrev slot_trans := ReferenceTrace.slot_trans

private theorem same_rows (m next : MixedReferenceStore.Machine p a) (equal : next.bindings = m.bindings) (key : Nat) :
    SlotProgress m.bindings[key]? next.bindings[key]? := by rw [equal]; exact slot_refl _

private theorem slot_of_lookups (m next : MixedReferenceStore.Machine p a) (key : Nat)
    (lengths : m.bindings.length ≤ next.bindings.length)
    (vacant : m.bindings[key]? = some none → next.bindings[key]? = some none)
    (progress : ∀ before after, lookup m key = some before → lookup next key = some after → Progressed before after) :
    SlotProgress m.bindings[key]? next.bindings[key]? := by
  cases old : m.bindings[key]? with
  | none => exact .unallocated
  | some row =>
      cases row with
      | none => rw [vacant old]; exact .tombstone
      | some before =>
          have oldLookup : lookup m key = some before := by simp [lookup,old]
          have bound := lookup_bound _ _ _ oldLookup
          cases new : next.bindings[key]? with
          | none =>
              have missing : next.bindings.length ≤ key := List.getElem?_eq_none_iff.mp new
              omega
          | some row => cases row with
            | none => exact .released
            | some after => exact .active (progress before after oldLookup (by simp [lookup,new]))

inductive Transition : MixedReferenceStore.Machine p a → MixedReferenceStore.Machine p a → Prop where
  | management : MixedReferenceStore.manage m member place principal requestId raw = some (next,created) → Transition m next
  | authority : Transition m (MixedReferenceStore.authorityHead m view)
  | start : MixedReferenceStore.start m member place principal requestId key argument = some (next,ticket) → Transition m next
  | startOwner : MixedReferenceStore.startOwner m member place principal requestId key origin args = some (next,saved) → Transition m next
  | cancellation : MixedReferenceStore.cancel m member place principal requestId entry permit = some next → Transition m next
  | finishOwner : MixedReferenceStore.finishOwner m saved received evidence = some next → Transition m next
  | finishPlain : MixedReferenceStore.finishPlain m ticket value = some next → Transition m next
  | acquire : MixedReferenceMutation.acquire m member place principal requestId chain = some (next,key) → Transition m next
  | normalize : Transition m (MixedReferenceMutation.normalize m member place principal requestId key).state
  | reacquire : MixedReferenceMutation.reacquire m member place principal requestId key = some next → Transition m next
  | release : MixedReferenceMutation.release m member place principal requestId key = some next → Transition m next

theorem transition_preserves (m next : MixedReferenceStore.Machine p a) (valid : MixedReferenceCoherence.Invariant m)
    (transition : Transition m next) : MixedReferenceCoherence.Invariant next := by
  cases transition with
  | management run => exact MixedReferenceCoherence.manage_preserves _ _ _ _ _ _ _ valid run
  | authority => exact MixedReferenceCoherence.authorityHead_preserves _ _ valid
  | start run => exact MixedReferenceCoherence.start_preserves _ _ _ _ _ _ _ _ valid run
  | startOwner run => exact MixedReferenceCoherence.startOwner_preserves _ _ _ _ _ _ _ _ _ valid run
  | cancellation run => exact MixedReferenceCoherence.cancel_preserves _ _ _ _ _ _ _ _ valid run
  | finishOwner run => exact MixedReferenceCoherence.finishOwner_preserves _ valid run
  | finishPlain run => exact MixedReferenceCoherence.finishPlain_preserves _ _ _ _ valid run
  | acquire run => exact acquire_coherent _ _ _ _ _ _ _ valid run
  | normalize => exact normalize_coherent _ _ _ _ _ _ valid
  | reacquire run => exact reacquire_coherent _ _ _ _ _ _ _ valid run
  | release run => exact release_coherent _ _ _ _ _ _ _ valid run

theorem transition_slot (m next : MixedReferenceStore.Machine p a) (valid : MixedReferenceCoherence.Invariant m)
    (transition : Transition m next) (key : Nat) : SlotProgress m.bindings[key]? next.bindings[key]? := by
  cases transition with
  | management run => exact same_rows _ _ (MixedReferenceStore.manage_preserves _ _ _ _ _ _ _ valid.1 run).2.1 key
  | authority => exact same_rows _ _ rfl key
  | start run => exact same_rows _ _ (MixedReferenceStore.start_preserves _ _ _ _ _ _ _ _ valid.1 run).2.1 key
  | startOwner run => exact same_rows _ _ (MixedReferenceStore.startOwner_preserves _ _ _ _ _ _ _ _ _ valid.1 run).2.1 key
  | cancellation run => exact same_rows _ _ (MixedReferenceStore.cancel_preserves _ _ _ _ _ _ _ _ valid.1 run).2.1 key
  | finishOwner run => exact same_rows _ _ (MixedReferenceStore.finishOwner_preserves _ valid.1 run).2.1 key
  | finishPlain run => exact same_rows _ _ (MixedReferenceStore.finishPlain_preserves _ _ _ _ valid.1 run).2.1 key
  | acquire run =>
      have oldSlots := acquire_old_slots _ _ _ _ _ _ _ run
      by_cases existing : key < m.bindings.length
      · rw [oldSlots.2.2 key existing]
        exact slot_refl _
      · have absent : m.bindings[key]? = none := List.getElem?_eq_none (by omega)
        rw [absent]
        exact .unallocated
  | @normalize member place principal id target =>
      apply slot_of_lookups m _ key (by rw [normalize_length]; exact Nat.le_refl _)
      · intro vacant
        by_cases same : target = key
        · subst target
          have absent : lookup m key = none := by simp [lookup,vacant]
          simp [MixedReferenceMutation.normalize,absent,vacant]
        · rw [normalize_other_row _ _ _ _ _ _ _ valid same]
          exact vacant
      · intro before after old new
        by_cases same : target = key
        · subst target
          exact normalize_progress _ _ _ _ _ _ _ _ valid old new
        · rw [normalize_other _ _ _ _ _ _ _ valid same] at new
          have equal : before = after := Option.some.inj (old.symm.trans new)
          subst after
          exact progressed_refl _
  | @reacquire member place principal id target _ run =>
      apply slot_of_lookups m next key (by rw [reacquire_length _ _ _ _ _ _ _ run]; exact Nat.le_refl _)
      · intro vacant
        by_cases same : target = key
        · subst target
          have absent : lookup m key = none := by simp [lookup,vacant]
          simp [MixedReferenceMutation.reacquire,absent] at run
        · rw [reacquire_other_row _ _ _ _ _ _ _ _ valid same run]
          exact vacant
      · intro before after old new
        by_cases same : target = key
        · subst target
          exact (reacquire_progress _ _ _ _ _ _ _ _ _ valid old run new).1
        · rw [reacquire_other _ _ _ _ _ _ _ _ valid same run] at new
          have equal : before = after := Option.some.inj (old.symm.trans new)
          subst after
          exact progressed_refl _
  | @release member place principal id target _ run =>
      by_cases same : target = key
      · subst target
        have removed := release_removes _ _ _ _ _ _ _ valid run
        rw [removed]
        cases old : m.bindings[key]? with
        | none => exact .unallocated
        | some row => cases row with
          | none => exact .tombstone
          | some before => exact .released
      · rw [release_other _ _ _ _ _ _ _ _ valid same run]
        exact slot_refl _

inductive Reached : MixedReferenceStore.Machine p a → MixedReferenceStore.Machine p a → Prop where
  | refl : Reached m m
  | step {initial m next : MixedReferenceStore.Machine p a} :
      Reached initial m → Transition m next → Reached initial next

theorem reached_preserves (initial next : MixedReferenceStore.Machine p a) (valid : MixedReferenceCoherence.Invariant initial)
    (path : Reached initial next) : MixedReferenceCoherence.Invariant next := by
  induction path with
  | refl => exact valid
  | step _ transition ih => exact transition_preserves _ _ ih transition

theorem reached_slots (initial next : MixedReferenceStore.Machine p a) (valid : MixedReferenceCoherence.Invariant initial)
    (path : Reached initial next) (key : Nat) : SlotProgress initial.bindings[key]? next.bindings[key]? := by
  induction path with
  | refl => exact slot_refl _
  | step pathStep transition ih =>
      exact slot_trans _ _ _ ih (transition_slot _ _ (reached_preserves _ _ valid pathStep) transition key)

theorem released_never_reused (initial next : MixedReferenceStore.Machine p a) (valid : MixedReferenceCoherence.Invariant initial)
    (path : Reached initial next) (key : Nat) (released : initial.bindings[key]? = some none) :
    next.bindings[key]? = some none := by
  have progress := reached_slots _ _ valid path key
  rw [released] at progress
  generalize atKey : next.bindings[key]? = slot at progress
  cases progress
  rfl

theorem reached_progress (initial next : MixedReferenceStore.Machine p a) (valid : MixedReferenceCoherence.Invariant initial)
    (path : Reached initial next) (key : Nat) (before after : Binding)
    (old : lookup initial key = some before) (new : lookup next key = some after) : Progressed before after := by
  have progress := reached_slots _ _ valid path key
  rw [lookup_row _ _ _ old,lookup_row _ _ _ new] at progress
  cases progress with | active evidence => exact evidence

theorem reached_no_same_rank_refresh (initial next : MixedReferenceStore.Machine p a) (valid : MixedReferenceCoherence.Invariant initial)
    (path : Reached initial next) (key : Nat) (before after : Binding)
    (old : lookup initial key = some before) (new : lookup next key = some after)
    (epoch : before.request.epoch = after.request.epoch) (sameRank : rank before = rank after) : before = after :=
  no_same_rank_refresh before after (reached_progress _ _ valid path key before after old new) epoch sameRank

#print axioms slot_trans
#print axioms transition_preserves
#print axioms transition_slot
#print axioms reached_preserves
#print axioms reached_slots
#print axioms released_never_reused
#print axioms reached_progress
#print axioms reached_no_same_rank_refresh
-- Rooted only in this lower Store transition set; not full source admission.
def Rooted (realm : Nat) (view : WorldProjection.AuthorityView a) (policy : Nat → CurrentUse.Policy)
 (m : MixedReferenceStore.Machine p a) : Prop := Reached (MixedReferenceStore.initial realm view policy) m
end MirroreaProofFirst.MixedReferenceTrace
