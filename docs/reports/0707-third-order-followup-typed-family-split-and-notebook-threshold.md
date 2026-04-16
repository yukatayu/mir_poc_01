# 0707 — third-order follow-up typed family split and notebook threshold

## Objective

Close the first two theory-lab third-order follow-up packages:

1. `typed-surface family unification keep/drop note`
2. `notebook-consumer threshold and discharge reserve note`

and update snapshot / plan / FAQ mirrors so the theory-lab lane advances from
`reserve checkpoint close`
to
`third-order follow-up active`
without leaving stale wording behind.

## Scope and assumptions

- This task is docs-first research integration, not code broadening.
- `specs/` remain normative; `plan/` stays repository memory; `progress.md` and `tasks.md` stay thin snapshots.
- No final typed syntax, no final public theorem contract, and no concrete theorem prover binding are fixed here.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `specs/examples/413`
- `specs/examples/418`
- `specs/examples/419`
- `specs/examples/425`
- `specs/examples/426`
- `specs/examples/433`
- `specs/examples/434`

## Actions taken

1. Added `specs/examples/439...440` for the first two third-order follow-up packages.
2. Fixed the typed line at a two-tier split:
   `principal checker attachment` above
   `source-visible structural marker family`
   and
   `reserve cluster family`.
3. Fixed the theorem line at an `abstract discharge-entry row` threshold, while keeping notebook-first review and symbolic-ref-only admissible evidence intact.
4. Advanced current snapshot wording from `reserve checkpoint close` to `third-order follow-up active`.
5. Updated `Documentation.md`, `progress.md`, `tasks.md`, relevant `plan/`, FAQ, abstract, traceability, and document map mirrors.

## Files changed

- `specs/examples/439-current-l2-typed-surface-family-unification-keep-drop-note.md`
- `specs/examples/440-current-l2-notebook-consumer-threshold-and-discharge-reserve-note.md`
- `docs/reports/0707-third-order-followup-typed-family-split-and-notebook-threshold.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `faq_004.md`
- `faq_005.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `samples/current-l2/README.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M JST'
cargo test -p mir-semantics --test current_l2_proof_notebook_review_unit_support
cargo test -p mir-semantics --test current_l2_formal_hook_support
python3 scripts/validate_docs.py
git diff --check
git status --short
```

## Evidence / outputs / test results

- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-17 00:28 JST`
- `cargo test -p mir-semantics --test current_l2_proof_notebook_review_unit_support`
  - `4 passed`
- `cargo test -p mir-semantics --test current_l2_formal_hook_support`
  - `5 passed`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 706 numbered report(s).`
- `git diff --check`
  - no output
- `git status --short`
  - modified snapshot / plan / FAQ / abstract files plus new `0707` / `439` / `440` before commit

## What changed in understanding

- Typed work no longer needs an immediate “shared attachment shape” decision; the current stable cut is a two-tier family split under checker attachment principal.
- Theorem-side notebook pressure can be made more precise without collapsing review and discharge: an abstract discharge-entry reserve row is enough for the current threshold.
- The theory-lab lane is no longer best described as a checkpoint hold; it has moved into third-order follow-up with the model-check projection keep/drop refresh as the next main reopen.

## Open questions

- exact namespace for `source_visible_structural_marker_family_refs[]` / `reserve_cluster_family_refs[]`
- exact namespace for `abstract_discharge_entry_reserve`
- model-check small-cluster projection keep/drop cut
- order / handoff source-surface wording reserve
- modality internalization trigger

## Update necessity notes

- `plan/` updated
- `progress.md` updated
- `tasks.md` updated
- `Documentation.md` updated
- `AGENTS.md` updated unnecessary
- `.docs/` updated unnecessary

## Suggested next prompt

`では引き続き、tasks.md 先頭の model-check small-cluster projection keep/drop refresh と order / handoff source-surface wording reserve note を自走でお願いします。`
