# plan/132 - G1 status evidence readiness dry-run

## Purpose

This file is LAB repository memory.

It records a docs/validation-only dry-run for the evidence checks that a future
G1 OBL-001 / OBL-020 / OBL-021 status proposal packet would need to cite.

This file does not edit canon, does not close G0 or G1, does not choose
`stated` vs `lean-stated`, does not submit a status proposal, does not move
metatheory ledger status, does not complete OBL-001 / OBL-020 / OBL-021, does
not prove OBL-002, does not claim conformance, does not add an executable row,
does not refine a Lean predicate, and does not change runtime, transport,
diagnostic, repair, Core IR, public API, grammar, or sample status.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: legacy `specs/`, `plan/`, samples,
  helpers, tests, reports, Rust code, and Lean statement drafts outside
  `mirrorea_canon/`
- Snapshot status: `progress.md` and `tasks.md`
- Runnable dashboard: `samples_progress.md`

If LAB evidence conflicts with canon, canon wins. This file is evidence
readiness only; the canon metatheory ledger remains the only proof/status
authority.

## Dry-run target

`plan/131` says a future proposal packet should cite fresh validation for:

- Lean compile-check of the OBL-001 / OBL-020 / OBL-021 statement drafts;
- sync guards for body links and obvious vacuity / drift;
- scans excluding admitted stubs and placeholder bodies.

P79 runs those checks against the current LAB statement drafts:

| OBL | Lean draft | Current statement constant | Dry-run result |
|---|---|---|---|
| OBL-001 | `samples/lean/lab-statements/obl001/THM001StatementDraft.lean` | `THM001StatementDraft` | pass |
| OBL-020 | `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean` | `OBL020StatementDraft` | pass |
| OBL-021 | `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean` | `OBL021StatementDraft` | pass |

## Commands and results

| Check | Command | Result |
|---|---|---|
| Lean toolchain availability | `lean --version && elan --version && lake --version` | pass: Lean 4.29.1, elan 4.2.3, Lake 5.0.0-src |
| OBL-001 compile-check | `lean samples/lean/lab-statements/obl001/THM001StatementDraft.lean` | pass |
| OBL-020 compile-check | `lean samples/lean/lab-statements/obl020/StepWFStatementDraft.lean` | pass |
| OBL-021 compile-check | `lean samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean` | pass |
| OBL-001/020/021 sync guards | `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` | pass: 21 tests |
| Admitted-stub / placeholder scan | `rg -n "^\\s*(axiom\|constant\|theorem)\\b\|\\bsorry\\b\|:=\\s*(by\\s+)?trivial\\b\|:=\\s*(\\(\\s*)?True(\\s*\\))?\\b" <three OBL drafts>` | pass: no matches |

The sync script `python3 scripts/current_l2_lean_sample_sync.py` was not run in
this dry-run because it rewrites `samples/lean/manifest.json` and generated
clean-near-end stubs. The future proposal packet needs fresh evidence for the
three statement drafts and body-link guards; it does not need a generated
manifest refresh unless that package explicitly chooses to cite manifest
verification.

## Evidence classification

| Criterion from `plan/130` / `plan/131` | P79 evidence | Status after P79 |
|---|---|---|
| Fresh compile-check evidence | Three target Lean files compile directly with `lean`. | `current LAB support` strengthened for proposal preparation. |
| No admitted stubs | Targeted scan found no `axiom`, `constant`, `theorem`, `sorry`, or placeholder bodies in the three drafts. | `current LAB support` strengthened; not canon acceptance. |
| Non-vacuity / drift guard | Sync guard unit tests passed and include OBL-001 / OBL-020 / OBL-021 body-link checks plus vacuity checks. | `current LAB support` strengthened; not proof. |
| Artifact identity | Paths and constants are stable enough for a future packet to cite as LAB candidates. | `proposal criterion` still open for human/canon acceptance. |
| Ledger target mapping | Drafts still live under `MirCore.Lab...`; they do not silently claim canon target namespaces. | `human/canon decision` still open: LAB namespace vs canon-facing wrapper. |

## Remaining blockers

P79 removes no canon blocker. These remain:

- requested status vocabulary is still unchosen for each OBL;
- OBL-020 still has a full-vs-G1-supporting scope question;
- OBL-021 still has final equality / diagnostic equivalence abstraction
  questions;
- OPEN-014 remains unresolved or explicitly deferrable only by a later packet;
- LAB namespace vs canon-facing wrapper remains a human/canon decision;
- proof, conformance, runtime, and final ABI claims remain outside G1 status
  evidence readiness.

## Non-claims

- No canon edit.
- No G0 exit.
- No T0 -> T1 transition.
- No G1 exit.
- No G2..G7 exit.
- No requested status selection.
- No draft proposal submission.
- No OBL status movement.
- No OBL completion.
- No proof skeleton completion.
- No proof discharge.
- No C-static, C-runtime, or C-distributed conformance claim.
- No new executable row.
- No Lean predicate refinement.
- No runtime dispatch, request serving, store mutation, occurrence ordering,
  admission lifecycle, stale-membership runtime failure, runtime scheduling
  determinism, or distributed transport claim.
- No final Core IR, Diagnostic, repair, runtime, transport, projection,
  telemetry, public API, grammar ABI, equality relation, diagnostic equivalence
  contract, or step-family taxonomy freeze.
- No sample status relabel.
- No exact executable negative evidence claim for SCN-02 negative (b).
- No OPEN-014 resolution.
- No G3 / THM-004 authority proof or production auth claim.

## Next allowed move

The next autonomous package can stay docs-only and prepare a requested-status
options matrix for OBL-001 / OBL-020 / OBL-021:

- compare `stated` vs `lean-stated` per OBL;
- state which option is most natural for a later proposal and why;
- keep the choice advisory until human/canon review;
- preserve the no-ledger-movement boundary.

If that matrix exposes a concrete blocker, the fallback is a narrow
OBL-specific criteria refinement or wrapper-statement preflight, not canon
ledger movement.

## Close condition

This file is closed when `plan/00-index.md`, `plan/90-source-traceability.md`,
the docs validators, current snapshot docs, `samples_progress.md`, and the
package report are synchronized.

Close condition is evidence-readiness-only: no canon edit, no gate exit, no
requested status choice, no OBL status movement, no proof, no conformance
claim, no implementation change, and no runnable sample status change.
