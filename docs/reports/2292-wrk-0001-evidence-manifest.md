# Report 2292 - WRK-0001 evidence manifest

- Date: 2026-07-21 17:22 JST
- Author / agent: Codex
- Scope: Append the committed finite-index evidence to WRK-0001 and preserve its L3 non-promotion boundary.
- Decision levels touched: L3 evidence only. No L0/L1, theory/11, contract, SCN, Gate, Phase, proof, implementation, or public-state movement.

## Objective

Bind the already-committed reproduction evidence to the exact committed WRK record using the append-only provenance rule.

## Scope and assumptions

The evidence commit is `887a0f6cd2de57443f4508c14fbadf4a88f25992`. Its plan artifact is fixed by SHA-256 `5155ce3de994cc92975a797a2d7ee1b7b79453ff6739a125546c26f000d23972`. This task changes no evidence content and adds no new LAB source, helper, runner, schema, or runtime surface.

## Start state / dirty state

Started from clean, pushed `main` at `887a0f6c`, where the evidence commit was reachable but not yet listed in WRK-0001's `Evidence commits:` field.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, ADR-0014, theory/02, `working/README.md`, and WRK-0001.
- `plan/00-index.md`, `plan/158`, `plan/159`, and `plan/wrk-0001-finite-index-reproduction.md` as LAB evidence and operating memory.
- `docs/project-status.md`, `progress.md`, `tasks.md`, and the completed Oracle advisory result.

## Actions taken

- Resolved the evidence commit and artifact SHA-256 directly from Git.
- Added the exact full commit ID and immutable plan artifact reference to WRK-0001's existing Results and review section.
- Regenerated canonical `INDEX.json` after the working record byte change.
- Recorded the completed Oracle advisory only as a future candidate comparison, without adopting its proposed target or changing this pilot.

## Files changed

- `mirrorea_canon/working/WRK-0001-finite-index-boundaries.md`
- `mirrorea_canon/INDEX.json`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2292-wrk-0001-evidence-manifest.md`

## Commands run

- `git show HEAD:plan/wrk-0001-finite-index-reproduction.md | sha256sum`
- `python3 meta/build-index.py` from `mirrorea_canon/`
- Focused documentation and provenance validation commands listed below.

## Evidence / outputs / test results

- The artifact hash resolves to `5155ce3de994cc92975a797a2d7ee1b7b79453ff6739a125546c26f000d23972` at evidence commit `887a0f6c`.
- The manifest states only the previously observed Lean 4.29.1 compile and registered source audit results.
- A browser-backed Oracle advisory compared candidates and recommended a later, separately pre-registered OBL-021 single-projection adequacy countermodel. It was not used to choose, modify, or strengthen WRK-0001.

## What changed in understanding

The explicit evidence-commit mechanism can bind a narrow result without implying that unrelated later work belongs to the pilot. The Oracle recommendation is useful only as a candidate for a future independent WRK decision.

## Open questions

- Authoritative clean-worktree validation must still confirm the committed manifest in isolation.
- A cross-cut checkpoint must decide whether the Oracle-suggested OBL-021 candidate is eligible without treating the advisory as canon or a committed experiment.
- L2 remains fail-closed pending an owner-authenticated trust anchor.

## Suggested next prompt

Run authoritative WRK validation in a clean detached worktree, conduct one cross-cut review, then stop at the planned checkpoint with a clear next-candidate recommendation and no promotion.

## Plan update status

`plan/` 更新不要: the immutable evidence document was already committed in the permitted `plan/` lane; this task only manifests its commit and hash in the WRK record.

## Documentation.md update status

`Documentation.md` 更新不要: the top-level reader route and project boundaries did not change.

## docs/project-status.md update status

更新済み: the human view now distinguishes manifested L3 evidence from promotion.

## progress.md update status

更新済み: the recent log records exact evidence manifestation and the remaining checkpoint work.

## tasks.md update status

更新済み: the pilot experiment is closed and the checkpoint is current.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample path, runner, command catalog, or readiness classification changed.

## Reviewer findings and follow-up

The Oracle advisory is recorded as an independent, non-normative candidate comparison. It recommends isolating `CoreTermOf` projection adequacy in an OBL-021 finite countermodel, but that proposal remains unadopted and requires a separate eligibility and pre-registration review.

## Skipped validations and reasons

Authoritative working-annex validation is intentionally run only after this manifest commit exists; it cannot validate an uncommitted manifest as authoritative evidence.

## Commit / push status

Pending focused diff review, commit, push, and then clean detached-worktree authoritative validation.

## Sub-agent session close status

No new sub-agent result was used. The Oracle consultation completed; its advisory conclusion is mirrored above without normative force.
