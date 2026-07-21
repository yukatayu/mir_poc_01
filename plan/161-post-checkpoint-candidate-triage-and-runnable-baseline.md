# plan/161 - post-checkpoint candidate triage and runnable baseline

## Role and authority

This file is LAB repository memory. `mirrorea_canon/` remains the normative
source for theory, obligation status, Gates, phases, contracts, and process.
This triage does not create a Canon decision, choose a `plan/143` axis,
promote or freeze a working record, move the theory ledger, or relabel any
runnable workflow.

It records two bounded facts after `plan/160`:

1. no new standing-eligible L3 proposition is selected immediately; and
2. the already-documented Full System V1 release-check remains reproducible.

The two facts are intentionally separate. A passing runnable LAB workflow is
not a semantic proof or authorization to widen the `working/` evidence lanes.

## Selection question

After WRK-0002 through WRK-0005, is there an existing-carrier,
existing-lane proposition for which both plausible outcomes change a live,
already-recorded decision branch, without choosing equality, adequacy,
totality placement, Diagnostic semantics, input identity, an external
contract, or another reserved boundary?

The answer at this checkpoint is **no**. No `WRK-0006` is opened.

The selection predicate used here is:

```text
standing eligible
and not already resolved or audited
and positive and falsifying outcomes both remain plausible
and those outcomes lead to distinct live downstream branches
```

It is a LAB triage criterion, not a new Canon rule.

## Evidence read

`plan/160` separates the OBL-021 LAB statement draft into outcome existence,
pairwise coherence on a fixed actual-outcome fiber, and adequacy to a selected
observation/equality. It explicitly rejects a fifth relation-law restatement.

The read-only source map independently found no candidate that survives this
test: OBL-020 needs concrete transition/frame/history/authority carriers;
OBL-001's concrete-evidence bridge remains owner-facing; OBL-024/025 repeats
the recorded association/replay and repair-realization audits unless it starts
choosing Diagnostic or repair semantics; and existing foundations would risk
selecting final label/declassification semantics.

A planner proposed one apparent exception: test whether two individually
sound Diagnostics for a fixed rejection must share an experiment-local blame
observation. A second, temporary Oracle review and local source reading reject
it as an immediate candidate. The current OBL-024 draft has no pairwise
comparison, uniqueness, canonicalization, ordering, or `EquivalentDiagnostic`
bridge; a countermodel would only repeat that an unstated bridge is unstated.
It would not eliminate Axis B1, B2, or B3 in
`plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md`.

This disagreement was useful: the planner's proposal was narrow enough to
audit, but it failed the distinct-live-branch test after the independent
review. It is not a rejected Canon option and does not record an owner
disposition.

## Candidate disposition

| Candidate class | Result | Reason |
| --- | --- | --- |
| Fifth OBL-021 relation-law lemma | Do not open | It only repackages fiberwise all-pairs coherence already recorded by WRK-0005. |
| Joint adequacy / Result extensionality | Do not open | It either assumes the desired bridge or selects the reserved observation/equality boundary. |
| OBL-024 soundness to diagnostic comparison | Do not open now | Current source and draft already leave the bridge open; a new countermodel does not select or prune a `plan/143` Axis B option. |
| OBL-025 repair realization/coverage | Do not open | T-RESEARCH-027 already isolates the carrier-to-realization gap; further work either duplicates it or selects repair semantics. |
| Existing Full System V1 workflow | Validation only | It is runnable LAB evidence outside the current `working/` permitted roots, so it is not retained L3 theory evidence. |

## Runnable baseline

On 2026-07-21, the following existing commands passed from a clean worktree:

```text
make check
python3 scripts/full_system_v1_release_check.py --format json check-all \
  --out /tmp/mirrorea-full-v1-release-20260721
```

`make check` passed Canon index, source hierarchy, documentation validation,
and `cargo check`. The Full System V1 release-check passed all 29 planned
commands, including tests, source-first helpers, Product Alpha compatibility,
and representative projection, same-binary local role-split, provider-admission,
and renderer-pose CLI flows. It reported a bounded release-check surface, not
real transport, arbitrary provider execution, final packet/FFI semantics, or
distributed durable save/load.

The subsequent maintenance audits make this baseline fail closed: parser,
checker/runtime, PoseGraph, projection/local-split, provider, and renderer rows
require accepted/rejected exit-code agreement. Provider and renderer helpers
also compare fresh report projections with their committed generated evidence
without rewriting it, and a provider/renderer failure returns exit status 2.
The release viewer additionally names C-distributed conformance and real
transport / multi-process execution as explicit non-claims. This is
evidence-integrity hardening only; it does not change the workflow's bounded
LAB classification or any Canon claim.

## Post-repair baseline attestation

On 2026-07-22, clean commit
`4a52dd3ee26488005859fbaab6dd845c5a3ee74d` matched the upstream tracking ref after the
bounded host-adapter semantic-invariant repair. Existing commands reproduced:

```text
make check
cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture
python3 scripts/full_system_v1_samples.py checker-check-all --format json
python3 scripts/full_system_v1_samples.py check-all --format json
python3 scripts/full_system_v1_release_check.py --format json check-all --out <temporary-empty-directory>
```

`make check` retained the Canon index (84 files), source hierarchy (711/711),
documentation scaffold, and `cargo check`. The focused typed-IR suite passed
20 tests. The checker corpus passed 3 positive and 18 expected-negative rows;
the aggregate executable partition passed 21 checker + 17 runtime + 12
operational = 50 rows. The isolated release workflow accepted all 29 planned
commands. The temporary generated bundles are disposable LAB evidence and are
not repository artifacts.

This attests reproducibility of the current bounded LAB surface only. It does
not reopen L3 selection, make Full System V1 an admitted `working/` evidence
lane, alter a Canon claim, select pending semantics, or change workflow,
conformance, Gate, Phase, OBL, or public-product status.

## Reopen rule

Keep no active new working record. Reopen autonomous L3 selection only when an
exact existing proposition is identified whose positive and falsifying outcomes
both alter a recorded live branch while excluding every ADR-0014 reserved
surface. Otherwise continue ordinary maintenance/reproducibility validation or
prepare a clearly labeled escalation bundle when a reserved decision is needed.

## Non-claims

- No Axis A--D choice in `plan/143`.
- No Diagnostic equality, ordering, ABI, association, replay, repair, or
  outcome-totality placement selection.
- No OBL status, proof, theory-ledger, Gate/Phase, conformance, implementation,
  or public completion claim.
- No new Lean file, helper, runner, schema, CI surface, evidence lane, or
  Full System V1 working-annex admission.
