import MixedCatalogHistory
namespace MirroreaProofFirst.MixedCatalogTransitions
open MixedCatalogHistory
-- Histories use only the actual authorized machine entries. No constructor
-- imports a snapshot or arbitrary catalog as a transition. Environment heads
-- may change authority, not definition content. Start does not advance the cut;
-- accepted management, result consumption and observed head changes do.
inductive Transition : MixedRequestCore.Machine p a → MixedRequestCore.Machine p a → Prop where
  | management : MixedRequestCore.manage s member place principal requestId raw = some (next,created) → Transition s next
  | request : MixedRequestCore.startPure s member place principal requestId key argument = some (next,ticket) → Transition s next
  | ownerRequest : MixedRequestCore.startOwner s member place principal requestId key origin arguments = some (next,saved) → Transition s next
  | ownerAcknowledged : MixedRequestCore.finishOwner s saved received evidence = some next → Transition s next
  | result : MixedRequestCore.finishPure s ticket value = some next → Transition s next
  | authority : Transition s (MixedRequestCore.authorityHead s view)
  | cancellation : MixedCancelEntry.cancel s member place principal requestId ticket permit = some next → Transition s next

inductive Reached : MixedRequestCore.Machine p a → MixedRequestCore.Machine p a → Prop where
  | refl : Reached s s
  | step : Reached start s → Transition s next → Reached start next

theorem reached_trans (ab : Reached a b) (bc : Reached b c) : Reached a c := by
  induction bc with
  | refl => exact ab
  | step _ transition ih => exact .step ih transition

theorem transition_catalog (s next : MixedRequestCore.Machine p a)
    (valid : Invariant s.system.configuration) (step : Transition s next) :
    Invariant next.system.configuration ∧ Extends s.system.configuration next.system.configuration := by
  cases step with
  | management accepted =>
      obtain ⟨_,e,cfg,created,hc,hr,eq⟩ := MixedRequestCore.manage_parts accepted
      cases eq
      exact ⟨run_catalog _ _ _ valid hr,run_extends _ _ _ valid.toValid hr⟩
  | request accepted =>
      obtain ⟨_,_,eq⟩ := MixedRequestCore.startPure_parts accepted
      dsimp only at eq
      rw [eq]
      exact ⟨valid,extends_refl _⟩
  | ownerRequest accepted =>
      obtain ⟨_,eq⟩ := MixedRequestCore.startOwner_parts accepted
      dsimp only at eq
      rw [eq]
      exact ⟨valid,extends_refl _⟩
  | result accepted =>
      obtain ⟨_,_,_,eq⟩ := MixedRequestCore.finishPure_parts accepted
      subst next
      exact ⟨valid,extends_refl _⟩
  | ownerAcknowledged accepted =>
      obtain ⟨_,_,_,_,_,_,_,rfl⟩ := MixedRequestCore.finishOwner_parts accepted
      exact ⟨valid,extends_refl _⟩
  | cancellation accepted =>
      obtain ⟨_,_,_,_,rfl⟩ := MixedCancelEntry.cancel_parts _ _ _ _ _ _ _ _ accepted
      exact ⟨valid,extends_refl _⟩
  | authority => exact ⟨valid,extends_refl _⟩

theorem reached_catalog (initial next : MixedRequestCore.Machine p a)
    (valid : Invariant initial.system.configuration) (path : Reached initial next) :
    Invariant next.system.configuration ∧ Extends initial.system.configuration next.system.configuration := by
  induction path with
  | refl => exact ⟨valid,extends_refl _⟩
  | step _ transition ih =>
      have extended := transition_catalog _ _ ih.1 transition
      exact ⟨extended.1,extends_trans _ _ _ ih.2 extended.2⟩

theorem transition_cut (s next : MixedRequestCore.Machine p a) (step : Transition s next) :
    s.system.serial ≤ next.system.serial ∧ (s.system.serial = next.system.serial → s.system.configuration = next.system.configuration) := by
  cases step with
  | management accepted =>
      obtain ⟨_,e,cfg,created,hc,hr,eq⟩ := MixedRequestCore.manage_parts accepted
      cases eq
      simp [MixedManagementEntry.after]
  | request accepted =>
      obtain ⟨_,_,eq⟩ := MixedRequestCore.startPure_parts accepted
      dsimp only at eq
      rw [eq]
      exact ⟨Nat.le_refl _,fun _ => rfl⟩
  | ownerRequest accepted =>
      obtain ⟨_,eq⟩ := MixedRequestCore.startOwner_parts accepted
      dsimp only at eq
      rw [eq]
      exact ⟨Nat.le_refl _,fun _ => rfl⟩
  | result accepted =>
      obtain ⟨_,_,_,eq⟩ := MixedRequestCore.finishPure_parts accepted
      subst next
      simp [MixedRequestCore.consumePure]
  | ownerAcknowledged accepted =>
      obtain ⟨_,_,_,_,_,_,_,rfl⟩ := MixedRequestCore.finishOwner_parts accepted
      simp [MixedRequestCore.consumeOwner]
  | cancellation accepted =>
      obtain ⟨_,_,_,_,rfl⟩ := MixedCancelEntry.cancel_parts _ _ _ _ _ _ _ _ accepted
      simp [MixedCancelEntry.abandon]
  | authority => simp [MixedRequestCore.authorityHead,MixedManagementEntry.installAuthorityHead]

theorem reached_cut (s next : MixedRequestCore.Machine p a) (path : Reached s next) :
    s.system.serial ≤ next.system.serial ∧ (s.system.serial = next.system.serial → s.system.configuration = next.system.configuration) := by
  induction path with
  | refl => exact ⟨Nat.le_refl _,fun _ => rfl⟩
  | @step before after path transition ih =>
      have bound := transition_cut before after transition
      refine ⟨Nat.le_trans ih.1 bound.1,?_⟩
      intro equal
      have left : s.system.serial = before.system.serial := by omega
      have right : before.system.serial = after.system.serial := by omega
      exact (ih.2 left).trans (bound.2 right)


#print axioms transition_catalog
#print axioms reached_catalog
#print axioms transition_cut
#print axioms reached_cut
end MirroreaProofFirst.MixedCatalogTransitions
