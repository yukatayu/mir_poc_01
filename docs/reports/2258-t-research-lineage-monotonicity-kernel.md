# Report 2258 — T research lineage monotonicity kernel

- Date: 2026-07-17
- Author / agent: Codex
- Scope: bounded LAB theory research under `plan/156`
- Decision levels touched: no canon decision level changed

## Objective

Resume one existing-lane theory investigation without selecting the optional OBL-001 concrete-evidence bridge. Test a smallest source-grounded `[E-DEGRADE]` / `[E-REACQ]` monotonicity shape under explicit, adversarially audited assumptions.

## Scope and assumptions

The canon remains `T0/G0 rebaseline`, and `mirrorea_canon/theory/11-metatheory-ledger.md` remains the sole proof-status source. The later direct theory objective authorizes unrelated existing-lane LAB selection, but does not record an OBL-001 bridge defer, authorize bridge design, or waive the pre-T1 moratorium. Scratch Lean files are disposable and untracked under `/tmp/mirrorea-t-research-005/`.

Lineage-as-`(witness, epoch)`, `Option Nat` positions, `seen`, target-only updates, persistence of older entries, and a two-rule `Run` are experiment-local sufficient conditions. They are not definitions of MirCore `Config`, canonical lineage, history maximum, active selection, or the full small-step relation.

## Start state / dirty state

The worktree was clean at `54139db8 Complete OBL-001 bridge decision bundle`. `T-RESEARCH-001` through `T-RESEARCH-003` were LAB `research-complete`; `T-RESEARCH-004` was not selected. The bridge bundle was owner-record pending.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, `CANON.md`, and `mirrorea_canon/NORTH-STAR.md`
- `mirrorea_canon/plan/00-gates.md`, `plan/01-phases.md`, `plan/02-operating-model.md`, and `meta/agent-instructions.md`
- `mirrorea_canon/theory/00-overview.md`, `01-mircore-v0.md`, `02-types-effects-failures.md`, `04-ordering-and-cuts.md`, `06-existence-fallback.md`, and `11-metatheory-ledger.md`
- `mirrorea_canon/adr/ADR-0004.md`, `mirrorea_canon/scenarios/SCN-08-avatar-fallback.md`, `plan/76-g1-obl020-021-dependency-inventory.md`, and `plan/156`
- Reports 2253, 2254, and 2257; `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, `samples_progress.md`, and `.docs/progress-task-axes.md`

## Actions taken

- Rebased the research boundary against canon, the proof ledger, and the autonomy envelope.
- Obtained an Oracle review of resuming bridge-independent research; it permitted work selection only, not a bridge disposition.
- Built disposable positive and negative Lean models from an explicit source cut.
- Corrected the model after exact-file review: renamed the local support predicate, narrowed the theorem, added a named restricted run, and strengthened two negative models.
- Recompiled, audited placeholders and theorem axioms, obtained a second Oracle PASS, and reproduced the repository’s current Lean, Surface, docs, hierarchy, and Cargo anchors.

## Files changed

- `docs/project-status.md`
- `docs/reports/2258-t-research-lineage-monotonicity-kernel.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `progress.md`
- `tasks.md`

## Commands run

- `df -h .`, `free -h`, `du -sh .`, and `du -sh target .git .cargo .lake`
- `lean --version`; `lean --trust=0` for both scratch files and the existing OBL-001/020/021 statement drafts
- `rg` placeholder / unsafe / explicit-axiom scans and `sha256sum` for scratch evidence
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- `python3 scripts/surface_mir_samples.py check-all --format json`
- `make check`
- two exact-file Oracle reviews

## Evidence / outputs / test results

- Lean was `4.29.1`; both corrected scratch files compiled with `--trust=0` and exit status 0.
- `#print axioms` reported exactly `[propext]` for `run_prior_entry_monotone` and `reacq_preserves_old_position`. The result is not axiom-free.
- The scratch scan found no `sorry`, `admit`, declared `axiom`, `opaque`, `unsafe`, `partial`, or `implemented_by`.
- SHA-256: `3d39b7eb997900676dcab67b252787953f5f80ad273318c3b233fe5ec7919733` for `LineageKernel.lean`; `2a8d797491c2efbfb02a66c861f886921e4ef67e385fc0bf94b4aef1b0812b69` for `LineageCountermodels.lean`. Scratch size: 24 KiB.
- The positive kernel proves only that entries defined at the start of the restricted two-rule run remain defined and pointwise nondecreasing. A named two-step run degrades an old entry and initializes a distinct local key at a lower index without changing prior entries.
- Negative evidence shows direct decrease breaks the property; arbitrary unrelated decrease can break the global property; a deliberately weakened reacquire can reset an existing key; pair inequality is weaker than separate component inequality; and two state-local maximum invariants do not imply transition nondecrease. None is a counterexample to canonical MirCore rules.
- Existing OBL statement drafts compiled; Lean sync passed 21 tests; Surface accepted all 53 samples with no failures and `workflow_ready: false`; `make check` passed hierarchy 704/704, docs validation, and `cargo check`.

## What changed in understanding

The direct theory objective resumes bridge-independent research while preserving the bridge’s stronger owner-provenance requirement. THM-002 supplies a clear transition direction, but canon intentionally does not fix an extensional lease-store update or mathematical lineage identity. The kernel is therefore feasibility evidence, not a reduced proof obligation.

## Open questions

- A source-faithful representation and frame discipline for canonical `L`, history, and chain instances remain open.
- The next OBL-020 rule × invariant clause needs an exact source cut and new falsifier; this kernel must not be reused as a canonical step definition.
- The OBL-001 bridge remains owner-record pending, and OBL-021 still has result-equivalence / OPEN-014 boundaries.

## Suggested next prompt

Select the next smallest canon-grounded OBL-020 research unit under `plan/156` with a new source cut and falsifier; do not generalize this kernel to OBL-020.

## Plan update status

`plan/` 更新済み: `plan/156` records the direct-objective selection reading, T-RESEARCH-005’s conditional result, binding boundary, evidence, review, and next selection guard.

## Documentation.md update status

`Documentation.md` 更新不要: reader entry points and source hierarchy did not change.

## docs/project-status.md update status

更新済み: the concise control view now distinguishes bridge-independent research selection from a bridge-specific owner defer and states the restricted result without a proof-status claim.

## progress.md update status

`progress.md` 更新済み: the current research evidence, decision queue, Macro 1 / Macro 5 readings, and dated recent log now include T-RESEARCH-005.

## tasks.md update status

`tasks.md` 更新済み: the completed lineage kernel replaces the stale no-successor reading, while the bridge remains a separate owner decision and further research remains subject to the selection rule.

## samples_progress.md update status

`samples_progress.md` 更新不要: no active sample, validation command, debug surface, or runnable workflow classification changed.

## Reviewer findings and follow-up

The first Oracle review found scope defects: the local support predicate could be mistaken for canonical well-formedness; target framing and old-entry persistence were definitionally assumed; freshness was not used by the main proof; the two-rule run was not a MirCore trace; and two negative examples needed stronger forms. The scratch work was corrected before a second exact-file review returned PASS for a conditional LAB kernel only.

Oracle advice is advisory. The wrapper requested GPT-5.6 Sol but did not verify model selection. Its reviewer environment lacked Lean, so this report separately records local tool version, compile results, axiom output, and hashes.

## Skipped validations and reasons

No canonical Lean package, runtime, conformance profile, transport, product, or public API changed, so no claim-specific runtime or distributed suite was appropriate. Existing front-door checks were reproduced but remain bounded LAB evidence, not validation of a MirCore formalization.

## Commit / push status

Pending at report write. The synchronized LAB record will be committed with `--no-gpg-sign` and pushed after focused documentation validation.

## Sub-agent session close status

Both Oracle reviews completed and their findings were critically distilled. No local sub-agent session was available or remains active.
