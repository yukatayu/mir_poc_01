/-!
current-l2 finite-index first-layer fragment

This file keeps the smallest self-contained Lean facts for the finite-index first layer:
capture-set inclusion, lifetime preorder, and simple remote-call budget.
It is not the final public typed calculus.
-/

namespace CurrentL2FiniteIndexFirstLayer

inductive Lifetime where
  | step
  | session
deriving DecidableEq, Repr

open Lifetime

def outlives : Lifetime → Lifetime → Prop
  | session, _ => True
  | step, step => True
  | step, session => False

theorem outlives_refl (lifetime : Lifetime) : outlives lifetime lifetime := by
  cases lifetime <;> simp [outlives]

theorem session_outlives_step : outlives session step := by
  simp [outlives]

theorem step_does_not_outlive_session : ¬ outlives step session := by
  simp [outlives]

theorem outlives_trans
    {a b c : Lifetime}
    (hab : outlives a b)
    (hbc : outlives b c) :
    outlives a c := by
  cases a <;> cases b <;> cases c <;> simp [outlives] at hab hbc ⊢

inductive Capability where
  | roomHistory
  | ephemeralToken
deriving DecidableEq, Repr

open Capability

abbrev CaptureSet := Capability → Bool

def captureSubset (lhs rhs : CaptureSet) : Prop :=
  ∀ capability, lhs capability = true → rhs capability = true

def emptyCapture : CaptureSet := fun _ => false

def fullCapture : CaptureSet := fun _ => true

def ephemeralOnly : CaptureSet
  | ephemeralToken => true
  | roomHistory => false

def roomHistoryOnly : CaptureSet
  | roomHistory => true
  | ephemeralToken => false

theorem capture_subset_refl (captures : CaptureSet) : captureSubset captures captures := by
  intro capability h
  exact h

theorem empty_capture_subset (captures : CaptureSet) :
    captureSubset emptyCapture captures := by
  intro capability h
  simp [emptyCapture] at h

theorem capture_subset_trans
    {capturesA capturesB capturesC : CaptureSet}
    (hab : captureSubset capturesA capturesB)
    (hbc : captureSubset capturesB capturesC) :
    captureSubset capturesA capturesC := by
  intro capability h
  exact hbc capability (hab capability h)

theorem ephemeral_only_subset_of_full_capture :
    captureSubset ephemeralOnly fullCapture := by
  intro capability h
  simp [fullCapture]

theorem ephemeral_only_not_subset_of_empty :
    ¬ captureSubset ephemeralOnly emptyCapture := by
  intro h
  have hToken := h ephemeralToken rfl
  simp [emptyCapture] at hToken

theorem room_history_only_not_subset_of_ephemeral_only :
    ¬ captureSubset roomHistoryOnly ephemeralOnly := by
  intro h
  have hHistory := h roomHistory rfl
  simp [ephemeralOnly] at hHistory

def remoteCallAllowed (remainingCalls : Nat) : Prop :=
  0 < remainingCalls

def spendRemoteCall : Nat → Nat
  | 0 => 0
  | remainingCalls + 1 => remainingCalls

theorem zero_budget_rejects_remote_call :
    ¬ remoteCallAllowed 0 := by
  simp [remoteCallAllowed]

theorem positive_budget_allows_remote_call :
    remoteCallAllowed 1 := by
  simp [remoteCallAllowed]

theorem succ_budget_allows_remote_call (remainingCalls : Nat) :
    remoteCallAllowed (Nat.succ remainingCalls) := by
  simp [remoteCallAllowed]

theorem single_budget_is_exhausted_after_one_call :
    ¬ remoteCallAllowed (spendRemoteCall 1) := by
  simp [spendRemoteCall, remoteCallAllowed]

theorem two_budget_still_allows_after_one_call :
    remoteCallAllowed (spendRemoteCall 2) := by
  simp [spendRemoteCall, remoteCallAllowed]

end CurrentL2FiniteIndexFirstLayer
