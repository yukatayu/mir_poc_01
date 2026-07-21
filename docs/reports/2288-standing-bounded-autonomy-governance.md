# Report 2288 - Standing bounded autonomy governance

- Date: 2026-07-21 16:58 JST
- Author / agent: Codex; independent planner, code-mapper, and reviewer sub-agents; one GPT-5.6 Sol Pro Oracle advisory review
- Scope: Establish the owner-approved standing bounded-autonomy route, then recut its WRK evidence validator before package close.
- Decision levels touched: L0 process amendment and L1 operating documentation already authorized by the owner's standing direction; L2 operating refinement in PROPOSAL-007. No theory/11, contract, SCN, Gate, Phase, proof, or public-state movement.

## Objective

Make reversible, non-reserved theory research and existing-lane validation self-directed without allowing routine work to mutate settled canon, proof status, contracts, lifecycle state, or public claims.

## Scope and assumptions

The owner's 2026-07-21 direction authorizes agents to select and advance bounded candidates while preserving the canon process for L0/L1 and reserved surfaces. This package implements only that operating boundary and its evidence integrity. It neither selects the first WRK pilot nor enables L2, which remains fail-closed.

## Start state / dirty state

Started from `main...origin/main` after the preceding governance integration, with no committed standing route that let a routine candidate enter canonical current research state without a per-target table. The initial uncommitted Package A patch was intentionally re-reviewed before commit; no unrelated user changes were reverted.

## Documents consulted

- Canon entry path: `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, ADR-0014, `working/README.md`, `plan/02-operating-model.md`, and meta process documents.
- LAB status and memory: `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `plan/154`, `plan/156` through `plan/159`.
- Validation sources: `scripts/validate_docs.py`, `scripts/check_source_hierarchy.py`, `mirrorea_canon/meta/build-index.py`, and their tests.
- Advisory input: an independent planner, local reviewers, and a browser-backed GPT-5.6 Sol Pro consultation distilled into `plan/159`; the external chat itself is not repository state.

## Actions taken

- Replaced the exact routine target-table approach with ADR-0014's standing negative-list L3 boundary and the canonical `working/WRK-####` annex; L0/L1, contracts, lifecycle, `theory/11`, proof, and public claims remain reserved.
- Added L3 pre-registration fields, immutable path and pre-registration projection, L2 frozen-material review checks, and an unresolved owner trust registry that deliberately fail-closes every L2 promotion.
- Replaced the unsound scan that attributed every descendant commit to every active WRK. A graph audit now checks every HEAD-reachable working-annex tree, registration ancestry, history identity, append-only evidence-commit ownership, artifact binding, and merge-local deltas.
- Added `Evidence commits:` as an L2 operating refinement: evidence is explicit, full-length, reachable, post-registration, globally exclusive, and confined to existing declared LAB locations plus exact metadata.
- Added authoritative clean-worktree validation, while explicitly not treating ordinary dirty files as proof or evidence in normal local validation.
- Updated canon/LAB navigation, status views, workflow, validator registries, and the finite A/B/C research horizon.

## Files changed

- `docs/project-status.md`
- Canon governance and indexing: `mirrorea_canon/adr/ADR-0014.md`, `mirrorea_canon/working/README.md`, PROPOSAL-006, PROPOSAL-007, review-key placeholder, operating/meta documents, changelog, MAP, and regenerated `mirrorea_canon/INDEX.json`.
- LAB plans and reader views: `plan/154`, `plan/156` through `plan/159`, `Documentation.md`, `README.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, and `docs/diagrams/workflow.mmd`.
- Validation: `Makefile`, `scripts/check_source_hierarchy.py`, `scripts/validate_docs.py`, `scripts/tests/test_validate_docs.py`, and `scripts/tests/test_build_index.py`.
- This immutable task evidence: `docs/reports/2288-standing-bounded-autonomy-governance.md`.

## Commands run

- `df -h .` and `free -h`
- `python3 -m compileall -q scripts/validate_docs.py scripts/tests/test_validate_docs.py`
- focused WRK red/green unit selections and `python3 -m unittest discover -s scripts/tests -q`
- `python3 meta/build-index.py` and `python3 meta/build-index.py --check` from `mirrorea_canon/`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `make docs` and `make check` (including `cargo check`)
- `git diff --check`

## Evidence / outputs / test results

- Pre-heavy-work resource check found 35 GiB free root-disk space and 9.6 GiB available memory; no new global tool installation or large external build artifact was needed for this package.
- The focused adversarial tests first demonstrated the predecessor defects: uncommitted deletion, malformed transient identity, ignored source, and merge attribution gaps. The replacement tests now cover registration contamination, immutable DAG history, merge of pre-registration work, manifested in-lane/out-of-lane evidence, pre-registration evidence, removal, duplicate ownership, and authoritative dirty state.
- The full `scripts/tests` unit suite exited successfully after the recut. Canon index generation currently reports 79 indexed files; documentation validation reports 1,442 numbered reports; source hierarchy reports 709 required paths and zero missing.
- `make docs` and `make check` completed successfully; `cargo check` finished the dev profile successfully. `git diff --check` was clean. No Gate/Phase, OBL, conformance, implementation, or public claim follows from these checks.

## What changed in understanding

An exact owner target list was too restrictive, but a descendant-wide Git lane scan was also incorrect: it serialized independent work and mishandled merge imports. The smallest reliable boundary is a canonical per-WRK evidence-commit list plus a DAG audit. It validates retained evidence provenance without claiming to prove external execution, helper intent, or erased history.

## Open questions

- Package B still needs to select the first eligible `WRK-####` pilot from existing evidence and pre-register its alternative/falsifier before running it.
- Owner-authenticated review-key activation remains an owner/canon action; L2 stays unavailable until then.
- Git provenance cannot establish unlisted experiment non-use, external tool/network provenance, or the intent of arbitrary files. These remain review/escalation limits.

## Suggested next prompt

Continue Package B: choose and pre-register one non-reserved candidate, run its smallest existing-lane positive and negative validation, then retain or falsify it as L3 evidence without attempting L2 promotion.

## Plan update status

`plan/` 更新済み: `plan/158` now states the explicit evidence-commit route; `plan/159` records the recut, invariants, limitations, and validation order; `plan/00-index.md` registers both current plans.

## Documentation.md update status

`Documentation.md` 更新済み: the reader route distinguishes the current autonomous horizon from the WRK provenance refinement.

## docs/project-status.md update status

更新済み: the human-facing current-state view now states committed pre-registration and manifested existing-lane evidence, with canonical and LAB sources.

## progress.md update status

`progress.md` 更新済み: the validation floor and recent log now distinguish explicit evidence ownership from the former over-broad descendant scan.

## tasks.md update status

`tasks.md` 更新済み: Package A is marked as final-review/validation/commit/push closeout, and Package B remains the next autonomous research package.

## samples_progress.md update status

`samples_progress.md 更新不要`: sample paths, runnable classifications, validation commands, and sample blockers did not change in this governance/validator package.

## Reviewer findings and follow-up

Earlier independent reviews found ambiguous L3 review timing, stale governance pointers, missing index freshness, weak structural WRK validation, and unsafe missing-section handling. A subsequent adversarial review found that the first descendant-wide lane check could miss malformed/deleted history and falsely attribute unrelated merge work. The planner and Oracle independently recommended explicit append-only evidence ownership, a graph-wide history audit, exact metadata exceptions, and a clean authoritative worktree. This package implements that recut. The final reviewer and its one permitted retry did not return after extended waits and were closed; local review then found and fixed the committed-deletion/absent-annex history-audit bypass, with a dedicated regression test.

## Skipped validations and reasons

No validation was intentionally skipped. Two final independent-review attempts did not return and are recorded above; local focused review and full validation were used instead. The authoritative clean-worktree command runs immediately after the package commit because this dirty development worktree cannot be authoritative evidence. No global installation was needed, so new-disk consumption is 0 bytes apart from small source/docs/test changes inside the repository.

## Commit / push status

This report is committed and pushed with its package; the authoritative clean-worktree check follows from the committed `HEAD` and is recorded in the package completion notification.

## Sub-agent session close status

The planner completed and was closed. Earlier mapper/review agents are closed. Two final reviewer attempts were closed after no completion response; their non-return and the local fallback are recorded above.
