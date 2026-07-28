# WRK-0034 V1/R1 - finite-sequence presentation evidence

## Role and boundary

This is **LAB** evidence for `working/WRK-0034` at its pinned authority/input
cut `1553bcc8fd140ad5ca98f5d7294fd802f776c7f1`. It reuses the exact finite
WRK-0033 state, reply, transition, translation, observation, matching,
single-use, and failure-exclusion model. `LabCorrelation` and a finite reply
list are local test labels only: they do not denote Mir request, attempt,
occurrence, delivery, scheduler, queue, history, persistence, transport, or
semantic correlation identity.

The positive result may only be a direct translation-preservation lemma and its
finite-list closure. It is not full trace equivalence, C3 completion, or source
inference. The copied WRK-0033 source below must remain byte-identical through
`failure_then_success_needs_failure_exclusion`; its source SHA-256 is
`7436c62eb3406f1e91ba7d3546ec979dfd7f2484557a941607b9f9082cac39ec`.

## Registered Lean source

```lean
inductive LabCorrelation where
  | alpha
  | beta
deriving DecidableEq, Repr

inductive LabReply where
  | success (correlation : LabCorrelation)
  | failure (correlation : LabCorrelation)
deriving DecidableEq, Repr

inductive LabObservation where
  | waiting
  | resumedOnce
  | resumedTwice
  | terminalFailure
deriving DecidableEq, Repr

inductive AdminState where
  | waiting (correlation : LabCorrelation)
  | resumed (correlation : LabCorrelation)
  | failed (correlation : LabCorrelation)
deriving DecidableEq, Repr

inductive MachineState where
  | slot (correlation : LabCorrelation)
  | resumed (correlation : LabCorrelation)
  | failed (correlation : LabCorrelation)
deriving DecidableEq, Repr

def adminObservation : AdminState -> LabObservation
  | .waiting _ => .waiting
  | .resumed _ => .resumedOnce
  | .failed _ => .terminalFailure

def machineObservation : MachineState -> LabObservation
  | .slot _ => .waiting
  | .resumed _ => .resumedOnce
  | .failed _ => .terminalFailure

def adminStep : AdminState -> LabReply -> AdminState
  | .waiting pending, .success incoming =>
      if pending = incoming then .resumed pending else .waiting pending
  | .waiting pending, .failure incoming =>
      if pending = incoming then .failed pending else .waiting pending
  | state, _ => state

def machineStep : MachineState -> LabReply -> MachineState
  | .slot pending, .success incoming =>
      if pending = incoming then .resumed pending else .slot pending
  | .slot pending, .failure incoming =>
      if pending = incoming then .failed pending else .slot pending
  | state, _ => state

def toMachine : AdminState -> MachineState
  | .waiting correlation => .slot correlation
  | .resumed correlation => .resumed correlation
  | .failed correlation => .failed correlation

theorem presentation_refinement (state : AdminState) (reply : LabReply) :
    adminObservation (adminStep state reply) =
      machineObservation (machineStep (toMachine state) reply) := by
  cases state with
  | waiting pending =>
      cases reply with
      | success incoming =>
          cases pending <;> cases incoming <;> decide
      | failure incoming =>
          cases pending <;> cases incoming <;> decide
  | resumed pending =>
      cases reply <;> rfl
  | failed pending =>
      cases reply <;> rfl

theorem administrative_single_use :
    adminStep (adminStep (.waiting .alpha) (.success .alpha)) (.success .alpha) =
      .resumed .alpha := by
  decide

theorem machine_single_use :
    machineStep (machineStep (.slot .alpha) (.success .alpha)) (.success .alpha) =
      .resumed .alpha := by
  decide

theorem administrative_failure_excludes_success :
    adminStep (adminStep (.waiting .alpha) (.failure .alpha)) (.success .alpha) =
      .failed .alpha := by
  decide

theorem machine_failure_excludes_success :
    machineStep (machineStep (.slot .alpha) (.failure .alpha)) (.success .alpha) =
      .failed .alpha := by
  decide

def weakMatchingStep : AdminState -> LabReply -> AdminState
  | .waiting pending, .success _ => .resumed pending
  | state, reply => adminStep state reply

def weakSingleUseObservation : AdminState -> LabReply -> LabObservation
  | .resumed pending, .success incoming =>
      if pending = incoming then .resumedTwice else .resumedOnce
  | state, reply => adminObservation (adminStep state reply)

def weakFailureStep : AdminState -> LabReply -> AdminState
  | .failed pending, .success incoming =>
      if pending = incoming then .resumed pending else .failed pending
  | state, reply => adminStep state reply

theorem swapped_reply_needs_matching :
    And
      (adminObservation (adminStep (.waiting .alpha) (.success .beta)) =
        LabObservation.waiting)
      (adminObservation (weakMatchingStep (.waiting .alpha) (.success .beta)) =
        LabObservation.resumedOnce) := by
  decide

theorem duplicate_reply_needs_single_use :
    And
      (adminObservation
          (adminStep (adminStep (.waiting .alpha) (.success .alpha)) (.success .alpha)) =
        LabObservation.resumedOnce)
      (weakSingleUseObservation (.resumed .alpha) (.success .alpha) =
        LabObservation.resumedTwice) := by
  decide

theorem failure_then_success_needs_failure_exclusion :
    And
      (adminObservation
          (adminStep (adminStep (.waiting .alpha) (.failure .alpha)) (.success .alpha)) =
        LabObservation.terminalFailure)
      (adminObservation
          (weakFailureStep (adminStep (.waiting .alpha) (.failure .alpha)) (.success .alpha)) =
        LabObservation.resumedOnce) := by
  decide

def adminRun (state : AdminState) (replies : List LabReply) : AdminState :=
  replies.foldl adminStep state

def machineRun (state : MachineState) (replies : List LabReply) : MachineState :=
  replies.foldl machineStep state

theorem toMachine_step (state : AdminState) (reply : LabReply) :
    toMachine (adminStep state reply) =
      machineStep (toMachine state) reply := by
  cases state with
  | waiting pending =>
      cases reply with
      | success incoming =>
          cases pending <;> cases incoming <;> decide
      | failure incoming =>
          cases pending <;> cases incoming <;> decide
  | resumed pending =>
      cases reply <;> rfl
  | failed pending =>
      cases reply <;> rfl

theorem observation_toMachine (state : AdminState) :
    adminObservation state = machineObservation (toMachine state) := by
  cases state <;> rfl

theorem finite_translation (state : AdminState) (replies : List LabReply) :
    toMachine (adminRun state replies) =
      machineRun (toMachine state) replies := by
  induction replies generalizing state with
  | nil =>
      rfl
  | cons reply replies inductionHypothesis =>
      change
        toMachine (adminRun (adminStep state reply) replies) =
          machineRun (machineStep (toMachine state) reply) replies
      rw [← toMachine_step]
      exact inductionHypothesis _

theorem finite_sequence_presentation_refinement
    (state : AdminState) (replies : List LabReply) :
    adminObservation (adminRun state replies) =
      machineObservation (machineRun (toMachine state) replies) := by
  calc
    adminObservation (adminRun state replies) =
        machineObservation (toMachine (adminRun state replies)) :=
      observation_toMachine _
    _ = machineObservation (machineRun (toMachine state) replies) := by
      rw [finite_translation]
```

## Reproduction status

The first post-registration source draft intentionally stated the smallest
translation-preservation target with `rfl`. The registered Lean command rejected
it because arbitrary opaque state/reply inputs are not definitionally reducible
without case analysis. This was the RED check for the registered proof
obligation, not an outcome or a semantic falsifier. The final source changes
only that proof term and adds the registered finite-list lemmas; it does not
change the copied model.

Observed 2026-07-28 13:27 JST: all nine registered source paths were nonempty
and their SHA-256 values matched the WRK-0034 authority cut. Before this source
was written, the registered novelty search found no retained finite-list
preservation theorem; after retention the same search naturally also finds this
artifact. The RED command failed exactly because `rfl` cannot reduce arbitrary
opaque `state` and `reply` inputs. After finite case analysis for
`toMachine_step` and structural induction for `finite_translation`, Lean 4.29.1
checked the extracted 182-line source at `--trust=0` without output. The copied
first 133 lines compare byte-for-byte equal to the WRK-0033 source. The final
source SHA-256 is
`234bb79588276c1682f25f98ce9ee7a55da9dc34a9070ce2baa56564711f354c`; no
`sorry`, `admit`, `axiom`, `unsafe`, `partial`, `implemented_by`, `Classical`,
or `Choice` token occurs, and `git diff --check` passed.

The retained result is exactly this: for every finite list of the fixed opaque
reply labels, `toMachine` commutes with each fixed step, and the fixed local
observations agree after the two corresponding `List.foldl` runs. This closes a
one-step-to-finite-list gap in the LAB presentation only. It does not model or
prove a Mir trace, delivery, scheduling, history, persistence, or any semantic
carrier. The WRK-0033 adversarial matching/single-use/failure distinctions
remain the separate reason that those assumptions cannot be silently omitted.

## Non-claims

This finite proof does not define or select a Mir request/reply/receipt/attempt/
occurrence identity, pending carrier, correlation, result payload/provenance,
continuation, syntax/elaboration or inference, failure family, authority,
transport, redaction, persistence, history, Core rule, Diagnostic, SCN, OBL,
Gate/Phase, runtime, API, or public contract. It is not full trace equivalence,
conformance, implementation readiness, or public completion.
