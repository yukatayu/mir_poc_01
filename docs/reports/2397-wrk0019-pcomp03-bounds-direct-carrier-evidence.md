# Report 2397 - WRK-0019 P-COMP-03 bounds direct-carrier evidence

- Date: 2026-07-23 08:48 JST
- Author / agent: Codex
- Scope: one pre-registered L3 existing-lane observation and LAB evidence
  retention
- Decision levels touched: L3 only; no Canon theory, ledger, Gate, Phase,
  grammar, scenario, implementation, or OBL decision

## Objective

Create exactly the pre-registered non-production arrays-bounds sidecar, execute
the pinned command after registration push, and retain only permitted evidence.

## Scope and assumptions

WRK-0019 permits one fixed Product Alpha package-path observation. The
observation is not a public diagnostic contract, failure phase, conformance
class, workflow-ready claim, or general direct-carrier result.

## Start state / dirty state

Started from pushed registration closeout `2e1871ed`, with the working record's
result fields empty. The only new source artifact before outcome execution was
the exact declared sidecar.

## Documents consulted

Read Canon README, MAP, ADR-0014, architecture/02, theory/11, the working
annex, WRK-0019, and the pinned post-WRK-0013 disposition, plan 167, Product
Alpha computational matrix, negative fixture, and direct-world manifest shape.

## Actions taken

1. Created only the exact JSON sidecar declared by WRK-0019.
2. Ran the registered digest guards, matrix, `check-all`, two focused Rust
   tests, Product Alpha `check`, `run-local`, exit-status assertion, and JSON
   assertions without adapting any input.
3. Retained only the declared sidecar, LAB memo/index, and this direct report.
   Generated `/tmp` output remains disposable.

## Files changed

- `samples/product-alpha1/computational/arrays-bounds/negative/direct-world/package.mir.json`
- `plan/wrk-0019-pcomp03-bounds-direct-carrier.md`
- `plan/00-index.md`
- this report

## Commands run

- the exact registered `WRK-0019` command, including four SHA-256 guards
- `python3 scripts/mir_computational_samples.py matrix --format json`
- `python3 scripts/mir_computational_samples.py check-all --format json`
- the registered focused `mir-semantics` and `mir-runtime` Cargo tests
- Product Alpha `check` and `run-local` against the exact sidecar

## Evidence / outputs / test results

All four pinned input digests matched. The 15-row matrix reported 7 accepted,
5 expected runtime rejections, and 3 expected check rejections; `check-all`
reported all 15 rows passed. Both focused Rust tests passed (one test each).

The target Product Alpha `check` verdict was `accepted`. Its `run-local`
command returned exit 2 and JSON `status: error`, `command: run-local`,
`diagnostic_code: MirCompute`, and the registered out-of-bounds message. The
sidecar SHA-256 is
`7d833a2c2e41a5dfc695246716e1a8343a5a3d1dca26bc29d9b6d0d86370d8f3`.

## What changed in understanding

The existing Product Alpha package route can carry this one fixed
arrays-bounds negative manifest through schema acceptance to the registered
`MirCompute` error. That establishes a narrow route observation only; it does
not reconcile the helper classification, closed registry evaluation phase, or
public diagnostic semantics.

## Open questions

- No general direct P-COMP-03 carrier or textual `.mir` input support follows.
- Core/result correspondence, global-step coverage, and outcome-totality
  placement remain independent formal boundaries.
- A later candidate must be distinct, pre-registrable, and have a current
  decision consumer; this result is not a repair or coverage-widening basis.

## Suggested next prompt

Treat this as bounded LAB evidence only. Seek a distinct qualified target or
request an owner/canon decision on a formal-interface boundary; do not widen
this observation into a public workflow claim.

## Plan update status

`plan/` 更新済み: the declared unnumbered evidence memo and its index entry
record the command result and non-effects without changing a normative plan.

## Documentation.md update status

`Documentation.md` 更新不要: it is intentionally outside this evidence
commit; a separate normal status-sync commit follows.

## docs/project-status.md update status

更新不要: it is intentionally outside this evidence commit; a separate normal
status-sync commit follows.

## progress.md update status

`progress.md` 更新不要: it is intentionally outside this evidence commit; a
separate normal status-sync commit follows.

## tasks.md update status

`tasks.md` 更新不要: it is intentionally outside this evidence commit; a
separate normal status-sync commit follows.

## samples_progress.md update status

`samples_progress.md` 更新不要: it is intentionally outside this evidence
commit; a separate normal status-sync commit follows.

## Reviewer findings and follow-up

Pre-registration planner and temporary Oracle review required an
existing-lane-only target, explicit phase/public non-claims, and a freeze on
reserved-surface pressure; the executed procedure satisfied those constraints.
L3 does not require a reviewer-bound admission.

## Skipped validations and reasons

No broad release workflow, network transport test, or public/product claim
validation was run: none is implied by this one sidecar. Generated `/tmp` JSON
is intentionally not committed.

## Commit / push status

Pending at report write. This evidence package will be committed and pushed
before a separate metadata-only result annotation and normal status-sync commit.

## Sub-agent session close status

The pre-registration planner and temporary Oracle consult are complete. No
sub-agent edited repository files for this evidence package.
