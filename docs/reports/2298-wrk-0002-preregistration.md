# Report 2298 - WRK-0002 OBL-021 projection-vacuity pre-registration

- Date: 2026-07-21 19:42 JST
- Author / agent: Codex
- Scope: Select and commit a bounded L3 countermodel candidate before running its outcome checks.
- Decision levels touched: L3 only. No L0/L1, theory/11, contract, SCN, Gate, Phase, proof, implementation, or public-state movement.

## Objective

Pre-register a small Lean countermodel that tests whether the existing LAB OBL-021 statement draft is vacuous without result-projection non-vacuity, while leaving final equality and all status decisions open.

## Scope and assumptions

The initially considered Full System V1 textual-pipeline reproduction is ineligible as a retained WRK candidate: its source lane is outside the validator's current permitted LAB roots. This task does not broaden that authority. The selected OBL-021 candidate uses only existing `plan` and `samples/lean` lanes and creates no new helper family, schema, CI/Make target, or public interface.

## Start state / dirty state

Started from pushed, clean `main` at `f8fb02a8`. Ignored local `target/` output exists from prior focused validation and remains outside the evidence set. No OBL-021 countermodel outcome command has run in this package.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, ADR-0014, `working/README.md`, and `plan/01-phases.md` / `02-operating-model.md`.
- `mirrorea_canon/theory/03-elaboration.md`, `10-diagnostics.md`, and `11-metatheory-ledger.md`.
- `plan/126`, `plan/140`, `plan/143`, `plan/158`, `plan/159`, `tasks.md`, and `progress.md`.
- `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`, its explanation, and `samples/lean/manifest.json`.
- `docs/reports/2297-minimal-runnable-mir-core-feasibility.md`.

## Actions taken

- Audited the proposed textual-core integration against the WRK validator's permitted LAB roots and classified it as an escalation candidate rather than widening the roots.
- Compared OBL-021's canonical determinism clause with the existing abstract LAB statement draft and the recorded unresolved projection/equality boundaries.
- Created WRK-0002 with pinned Canon/LAB inputs, an alternative, a falsifier, rollback, exact commands, and explicit non-claims.

## Files changed

- `mirrorea_canon/working/WRK-0002-obl021-projection-vacuity.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/INDEX.json`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2298-wrk-0002-preregistration.md`

## Commands run

- Read-only `rg` / `sed` inspection of Canon theory, existing LAB statement, historical OBL-021 boundary records, and validator eligibility rules.
- `git rev-parse HEAD` and SHA-256 snapshots of the pinned Canon/LAB inputs.
- No Lean compile, missing-file red check, source audit, or countermodel outcome command.

## Evidence / outputs / test results

- Canon theory/03 names OBL-021 as elaboration determinism; canon theory/11 keeps it open.
- The existing LAB statement draft compares projected components through universal predicates but does not itself state result-projection existence, uniqueness, or result identity.
- The existing validator permits only `plan`, `samples/clean-near-end`, `samples/current-l2`, and `samples/lean` as WRK retained-evidence roots. Full System V1 is therefore deliberately not used as this record's evidence lane.
- WRK-0002 is only pre-registered. Its candidate Lean source does not yet exist and no positive or negative outcome is claimed.

## What changed in understanding

The research governance boundary is materially narrower than the repository's broad LAB implementation inventory. That limitation prevents using the runnable Full System V1 source-first line as L3 evidence without an explicit governance change, but it does not block permitted Lean countermodel research. OBL-021's existing abstract draft provides a focused way to test an unchosen projection/non-vacuity premise rather than prematurely selecting final equality.

## Open questions

- Whether the registered countermodel compiles and demonstrates the planned vacuity remains untested until after this commit.
- Whether a future owner/canon action should expand retained-evidence roots for source-first implementation research remains an escalation question.
- Final equality, diagnostic equivalence, result-projection totality/uniqueness, artifact identity, and all OBL-021 status decisions remain unresolved.

## Suggested next prompt

Run WRK-0002's registered red/green Lean commands, record the countermodel evidence in the existing `plan` and `samples/lean` lanes, and preserve L3-only non-claims.

## Plan update status

更新不要: `plan/158` and `plan/159` already govern this lifecycle; an evidence artifact belongs only after the registration commit.

## Documentation.md update status

更新不要: the high-level reader route did not change.

## docs/project-status.md update status

更新済み: the human-facing status now distinguishes WRK-0002 pre-registration from unrun evidence.

## progress.md update status

更新済み: the recent log records the L3 registration and non-result status.

## tasks.md update status

更新済み: the Full System V1 lane boundary is explicit, and WRK-0002 evidence becomes the current package.

## samples_progress.md update status

更新不要: sample status, command catalog, and blocker classification did not change before evidence execution.

## Reviewer findings and follow-up

The immediately preceding Oracle feasibility review was advisory input for the textual-core candidate. Local validator evidence showed that candidate is out of the standing retained-evidence roots, so it is not relied on here. No new reviewer result is needed before L3 pre-registration.

## Skipped validations and reasons

The countermodel's Lean compile, source audit, and sync test are intentionally deferred until after this registration commit. Running them first would violate the committed outcome order. Documentation/index/source-hierarchy validation is run for the registration itself.

## Commit / push status

Pending validation of the committed pre-registration, then commit and push.

## Sub-agent session close status

No new sub-agent result was used. The prior temporary Oracle review remains advisory and is already distilled in Report 2297.
