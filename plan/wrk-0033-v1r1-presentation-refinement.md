# WRK-0033 V1/R1 - presentation-refinement evidence

## Role and boundary

This is **LAB** evidence for `working/WRK-0033` at its pinned authority cut
`ddabd97bb3e13df51ede3ba00ead626600e1011a`. It is a finite conditional lemma
about two administrative presentations. Canon remains normative. The Lean
names below are local test labels: in particular `LabCorrelation` stands for
the opaque `LAB$Correlation` mentioned in WRK-0033 and does not denote a Mir
request, receipt, attempt, occurrence, queue item, identity, or wire token.

The positive comparison has exactly these assumptions:

1. one waiting slot carries one opaque correlation label;
2. only an equal incoming success/failure label changes that slot;
3. a matching reply is consumed once, and terminal failure cannot later resume;
4. there is no save/load, authority, redaction, multi-slot state, source
   elaboration, payload, transport, scheduler, or history semantics.

The administrative presentation and the one-slot machine presentation are
distinct Lean inductives. `toMachine` maps the former to the latter;
`presentation_refinement` proves equality of the local observation after every
finite state/reply pair. The three weakened functions are not alternative Mir
semantics. They are adversarial variants that demonstrate why matching,
single-use, and failure exclusion must remain explicit evidence if a later
surface notation proposes to omit them.

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
```

## Observation matrix

| Registered condition | Ordinary administrative and machine presentations | Weakened adversarial variant | Finite distinction |
| --- | --- | --- | --- |
| matching | a `beta` success cannot change a waiting `alpha` slot | `weakMatchingStep` accepts any success | waiting vs resumed-once |
| single use | a second matching success leaves a resumed slot resumed-once | `weakSingleUseObservation` records resumed-twice | resumed-once vs resumed-twice |
| failure exclusion | a matching failure terminally prevents later matching success | `weakFailureStep` permits it to resume | terminal-failure vs resumed-once |

The positive theorem does not prove a full operational trace equivalence. It
only proves the registered observation equality for each finite one-step state
and reply pair. The matrix gives the required adverse cases for the separate
assumptions; it is not a queue, delivery, persistence, or authority model.

## Reproduction and results

The exact WRK-0033 registered command extracts the first `lean` block above to
`${TMPDIR:-/tmp}/mir-wrk0033-v1r1-presentation-refinement.lean` and invokes:

```bash
lean --trust=0 "${TMPDIR:-/tmp}/mir-wrk0033-v1r1-presentation-refinement.lean"
```

The first uncommitted extraction exposed two model-authoring errors: `rfl` was
too strong for the cross-inductive observation mapping, and the draft logical
conjunction notation was lost during Markdown extraction. Neither is a
registered semantic falsifier. The source now enumerates the finite labels for
the explicit mapping and uses `And`.

Observed 2026-07-28 12:26 JST: every registered outcome command passed at the
WRK-0033 pinned cut. The eight source checks were nonempty and their SHA-256
values matched the registration. The source query returned P012's V1 machine-
presentation and R1 matching-receipt boundaries plus Plan 187's comparison
obligations. Lean 4.29.1 checked the 133-line extracted file at `--trust=0`
without output; its SHA-256 was
`7436c62eb3406f1e91ba7d3546ec979dfd7f2484557a941607b9f9082cac39ec`.
`git diff --check` passed.

The retained finite result is exactly this: `presentation_refinement` covers
the finite administrative states and reply labels under all registered
assumptions; the three named adverse theorems distinguish a swapped reply, a
duplicate reply, and failure followed by an attempted success once the
respective assumption is weakened. The source is not a Lean module, sample,
helper, or stable artifact; the temporary file is disposable external
validation output.

## Non-claims

The finite proof does not define a semantic correlation, source-level binding,
pending carrier, continuation, result payload/provenance, receipt transport,
failure family, persistence, history, authority, redaction, scheduler, Core
rule, grammar, scenario behavior, OBL, Gate/Phase, runtime, API, or public
contract. It does not permit surface inference by itself. A future omission
still needs a uniquely determined fact and a reconstructible elaborated basis.
