# 0984 R5 docs-first package review

## Objective

`R5` runtime-crate hot-plug engine ownership cut の uncommitted docs/plan/progress diff について、
semantic overclaim、
stale status、
evidence-sync drift
がないかを review する。

## Scope and assumptions

- scope は `R5` 関連の docs / plan / snapshot diff に限る
- helper-local `hotplug_lifecycle`、`mirrorea-core` generic carrier-substrate、`mir-runtime` thin runtime/report assembly の owner split 読みが review 対象
- unrelated user-dirty files
  `crates/mir-ast/examples/current_l2_inspect_request_head_clause_bundle.rs`
  と
  `crates/mir-ast/src/current_l2.rs`
  は review scope 外とする

## Documents consulted

- `progress.md`
- `samples_progress.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `README.md`
- `Documentation.md`
- `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
- `docs/research_abstract/runtime_crate_hotplug_engine_ownership_cut_01.md`
- `docs/hands_on/runtime_crate_hotplug_engine_ownership_cut_01.md`
- `docs/reports/0982-r5-runtime-crate-hotplug-engine-ownership-cut-closeout.md`

## Actions taken

1. Ran a reviewer subagent pass over the current `R5` diff.
2. Asked the reviewer to focus on:
   helper preview vs `mirrorea-core` vs `mir-runtime` split、
   owner split vs engine actualization non-equivalence、
   stale closeout chronology、
   snapshot evidence sync.
3. Collected findings with file/line references before applying any follow-up fix.

## Evidence / outputs / test results

### Findings

1. `progress.md` still had stale `R5` status rows that contradicted the top-level snapshot:
   - top-level snapshot already said `R5` was closed and post-`R5` engine actualization was the next line
   - but a lower package-status row still marked `R5` as `active (docs-first)`
   - and the autonomy table still marked `R5` as `着手可能`
2. closeout chronology was stale after adding `0982`:
   - `plan/01-status-at-a-glance.md`
   - `plan/11-roadmap-near-term.md`
   - `progress.md`
   - `samples_progress.md` `PH14` docs column
   were still effectively stopping at `R4`
3. `samples_progress.md` `PH0` note still said validation only covered `R5` promoted-next wording, even though `0982` closeout report already existed.

### Non-findings

- reviewer did **not** find a semantic overclaim on the `R5` owner-split content itself
- reviewer confirmed the new wording consistently keeps:
  - engine actualization later
  - helper-local `hotplug_lifecycle` and sample envelope IDs as preview-only
  - `mirrorea-core` as generic carrier + logical runtime substrate
  - `mir-runtime` as thin runtime/report assembly rather than current hot-plug engine owner

### Validation note

- this review was diff-consistency review only
- the reviewer did not rerun the `R5` validation commands

## What changed in understanding

- the main remaining risk in `R5` was not semantic overclaim but **stale evidence-sync**
- package close wording is not enough; chronology, dashboard docs columns, and status rows must all be advanced together once the closeout report exists

## Open questions

- none beyond the already-known post-`R5` next-line question:
  exact package decomposition for implementation-side runtime-crate hot-plug engine actualization remains open

## Suggested next prompt

Apply the stale status / chronology / dashboard fixes from this review, rerun source-hierarchy / docs / diff checks, then request a narrow re-review that only checks whether `R5` closeout memory and report trail are internally consistent.
