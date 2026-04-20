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

inductive Capability where
  | roomHistory
  | ephemeralToken
deriving DecidableEq, Repr

open Capability

abbrev CaptureSet := Capability → Bool

def captureSubset (lhs rhs : CaptureSet) : Prop :=
  ∀ capability, lhs capability = true → rhs capability = true

def emptyCapture : CaptureSet := fun _ => false

def ephemeralOnly : CaptureSet
  | ephemeralToken => true
  | roomHistory => false

theorem capture_subset_refl (captures : CaptureSet) : captureSubset captures captures := by
  intro capability h
  exact h

theorem ephemeral_only_not_subset_of_empty :
    ¬ captureSubset ephemeralOnly emptyCapture := by
  intro h
  have hToken := h ephemeralToken rfl
  simp [emptyCapture] at hToken

def remoteCallAllowed (remainingCalls : Nat) : Prop :=
  0 < remainingCalls

theorem zero_budget_rejects_remote_call :
    ¬ remoteCallAllowed 0 := by
  simp [remoteCallAllowed]

theorem positive_budget_allows_remote_call :
    remoteCallAllowed 1 := by
  simp [remoteCallAllowed]

end CurrentL2FiniteIndexFirstLayer
