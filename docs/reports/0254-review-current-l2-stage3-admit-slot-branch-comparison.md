# Report 0254 — review current L2 stage 3 admit-slot branch comparison

- Date: 2026-04-06
- Author / agent: Codex
- Scope: current dirty diff review for the docs-only task around `specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md`, where the reviewed diff still used `stage 2` wording and had to be corrected
- Decision levels touched: L2

## 1. Objective

Review the current dirty diff for the admit-slot branch comparison task, with focus on:

1. whether the then-current `stage 2` wording around `specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md` contradicts specs 29 / 30 / 73 / 74 / 80 / 82
2. whether the current judgment that reads `e3-option-admit-chain` as a contrast reference rather than a full-program parse target is source-backed
3. whether Documentation / plan / progress / report mirrors are factually aligned

## 2. Scope and assumptions

- Review only. No normative spec edits were made in this task.
- The repository reading order in `AGENTS.md` was followed for the relevant materials.
- Normative judgments are taken from `specs/`; `plan/` and `progress.md` are treated as mirrors.

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/29-current-l2-first-parser-subset-inventory.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md`
- `specs/examples/80-current-l2-stage1-parser-spike-first-tranche-actualization.md`
- `specs/examples/82-current-l2-stage1-parser-spike-malformed-source-first-tranche-actualization.md`
- `specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0253-current-l2-stage3-admit-slot-branch-comparison.md`
- `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`

## 4. Actions taken

1. Inspected the dirty diff and enumerated the changed files.
2. Re-read specs 29 / 30 / 73 / 74 / 77 / 80 / 82 / 83 and the `e3-option-admit-chain` fixture.
3. Compared the new stage-numbering and `e3` reading against the already-set staged parser order.
4. Checked mirror documents and report files for factual alignment and required progress-log/report content.

## 5. Files changed

- Updated `docs/reports/0254-review-current-l2-stage2-parser-spike-admit-slot-comparison.md`

plan/ 更新不要
progress.md 更新不要

## 6. Commands run and exact outputs

### `git status --short`

```text
 M Documentation.md
 M plan/11-roadmap-near-term.md
 M plan/12-open-problems-and-risks.md
 M plan/90-source-traceability.md
 M progress.md
 M specs/00-document-map.md
?? docs/reports/0253-current-l2-stage2-parser-spike-admit-slot-comparison.md
?? docs/reports/0254-review-current-l2-stage2-parser-spike-admit-slot-comparison.md
?? specs/examples/83-current-l2-stage2-parser-spike-admit-slot-comparison.md
```

### `git diff -- ...`

Key exact added lines:

```text
+100. stage 2 parser spike の next narrow step として、`e3` を丸ごと送るのではなく declaration-side `admit` attached slot を先に切るのが自然だという current docs-only judgment は `specs/examples/83-current-l2-stage2-parser-spike-admit-slot-comparison.md`
+- current docs-only next step としては、`e3` を full-program parse へ送る前に declaration-side `admit` attached slot を stage 2 の最小 cut として比較し、`PerformVia` / request-local clause は still later stage に残すのが自然である
+- stage 2 parser spike では、declaration-side `admit` attached slot と `PerformVia` / request-local clause を一気に同時 actualize すると lexical freeze pressure が急に上がる。
```

### `tail -n 8 progress.md`

```text
- 2026-04-06 07:23 JST — stage 1 actual parser spike の smoke family を比較し、`e4-malformed-lineage` と `e7-write-fallback-after-expiry` の two-fixture pair を最小 working set にし、`e3-option-admit-chain` は later-stage contrast reference に残す current judgment を spec / plan / progress に反映した。次はこの working set を actual private parser spike module / helper 配置へどこまで narrow に落とすかを比べる段階。
- 2026-04-06 07:34 JST — actual stage 1 parser spike の private helper 配置と compare surface を比較し、`crates/mir-ast/tests/support/` 配置の private helper + lowered fixture-subset compare を最小 cut とする current judgment を spec / plan / progress に反映した。次はこの配置で non-production の actual stage 1 parser spike を実装に入れてよいか、input surface と test inline string 方針を narrow に詰める段階。
- 2026-04-06 07:43 JST — actual stage 1 parser spike の input surface / internal carrier / private helper naming を比較し、test inline string、dedicated wrapper + owned `surface_text`、`current_l2_stage1_parser_spike_support` を第一候補とする current judgment を spec / plan / progress に反映した。次は non-production の actual stage 1 parser spike を `crates/mir-ast/tests/support/` で実装し、`e4` / `e7` の lowered fixture-subset compare smoke を通す段階。
- 2026-04-06 08:08 JST — stage 1 parser spike の first tranche を actualize し、`crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs` と `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` で `e4` / `e7` の lowered fixture-subset compare と guard-slot retention smoke を通した。`cargo test -p mir-ast`、`cargo test -p mir-semantics`、docs validation まで揃ったので、次は malformed-source smoke か stage 2 contrast (`e3`) のどちらを先に narrow に切るかを比べる段階。
- 2026-04-06 08:31 JST — stage 1 parser spike の malformed-source first tranche を actualize し、accepted-cluster malformed と option-local `admit` spillover の 2 本だけを helper-local substring smoke として固定した。`cargo test -p mir-ast --test current_l2_stage1_parser_spike` と docs validation が通ったので、次は `perform` / request-local clause spillover と `e3` stage 2 contrast のどちらを先に切るかを比べる段階。
```

## 7. Evidence / findings

### Finding 1 — High

`specs/examples/83-current-l2-stage2-parser-spike-admit-slot-comparison.md` renumbers the staged parser roadmap without reconciling the already-set stage order, and the mirror files propagate that contradiction.

- Existing staged order is explicit in `specs/examples/73-current-l2-first-parser-spike-staging.md:118-123,219-237`:
  - stage 1 = chain / declaration structural floor
  - stage 2 = `try` / rollback structural floor
  - stage 3 = request / admissibility cluster
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md:25-29` says that stage order is to be maintained.
- `specs/examples/83-current-l2-stage2-parser-spike-admit-slot-comparison.md:8,35,54,109-115,133-137` labels declaration-side `admit` as the next `stage 2` parser spike.
- The same `stage 2 = admit-slot` claim is mirrored in `Documentation.md:169`, `plan/11-roadmap-near-term.md:221-224`, `plan/12-open-problems-and-risks.md:201`, `progress.md:3,33,116`, and `docs/reports/0253-current-l2-stage2-parser-spike-admit-slot-comparison.md:9-17,41-43,87-89,104`.

Why this matters:

- `admit` belongs to the request / admissibility cluster in spec 73, not to the already-defined stage 2.
- As written, the diff silently changes sequencing semantics from `73/74` without an explicit “we are revising the stage order” step.
- That makes the new judgment not source-backed as a `stage 2` claim, even if the narrower “use `e3` as a contrast reference and split declaration-side `admit` from request parsing” idea is otherwise plausible.

### Finding 2 — Medium

`progress.md` claims the stage 2 admit-slot comparison is reflected, but it does not append the required dated work-log entry for this non-trivial task.

- `progress.md:3` changes the header to “stage 2 parser spike admit-slot comparison まで反映”.
- `progress.md:33` and `progress.md:116` add summary text for the new judgment.
- But the dated task log still ends at `2026-04-06 08:31 JST` in `progress.md:239-240`, with no entry for the admit-slot comparison review/addition.

Why this matters:

- `AGENTS.md` requires a dated concise work log at task close for non-trivial tasks affecting current status.
- This leaves `progress.md` internally inconsistent: the snapshot says the task is reflected, but the chronological log does not show it.

## 8. Changes in understanding

- The narrow substantive idea in spec 83 is not obviously wrong by itself: treating `e3` as a contrast anchor and splitting declaration-side `admit` from request parsing is defensible.
- The break is in stage numbering and mirror propagation. The repo already fixed a three-stage order, and this diff changes the meaning of “stage 2” without saying so.
- The mirror set is therefore not factually aligned yet, chiefly because the new comparison is phrased as a settled `stage 2` step rather than as either `stage 3` work or an explicit revision to spec 73 / 74.

## 9. Open questions

- Should declaration-side `admit` be re-labeled as part of stage 3, while still keeping the narrower “attached-slot before full request parse” judgment?
- If the intended change is to reorder the staged spike, should spec 73 and every downstream mirror be updated explicitly rather than only adding spec 83?
- After the staging issue is resolved, should `plan/90-source-traceability.md` add an explicit pointer to spec 73 in the same addendum for clearer sequencing provenance?

## 10. Suggested next prompt

Reconcile the staged parser order first: either rewrite `specs/examples/83-current-l2-stage2-parser-spike-admit-slot-comparison.md` and its mirrors so the admit-slot work is clearly stage 3, or explicitly revise `specs/examples/73-current-l2-first-parser-spike-staging.md` / `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md` and then realign Documentation / plan / progress / report mirrors with that new stage numbering.
