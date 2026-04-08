# Report 0297 — current l2 stage3 predicate fragment reopen sequencing

- Date: 2026-04-08T13:30:14+09:00
- Author / agent: Codex
- Scope: Phase 0 / 1 / 2 closeout baseline の follow-up として late reviewer finding を吸収しつつ、Phase 3 later branch の次段を `request head + clause attachment multiline shape` ではなく `predicate fragment boundary` の reopen 条件へ寄せる current sequencing judgment を docs-only で固定する。task-start dirty state は untracked の report template 2 本（0297 / 0298）と `scripts/__pycache__/` だった。
- Decision levels touched: current parser boundary sequencing, top-level mirror hygiene, report hygiene

## 1. Objective

stage 3 later branch の next narrow step として、

- request head + clause attachment multiline shape
- predicate fragment boundary の reopen 条件

のどちらを先に扱うべきかを source-backed に比較し、
declaration-side `admit` line と request-local clause line の shared floor を崩さない sequencing judgment を与える。

併せて、0295/0296 に残っていた late reviewer finding の follow-up を current task で吸収する。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/88-current-l2-stage3-admit-next-step-sequencing.md`
- `specs/examples/89-current-l2-stage3-admit-node-handoff-comparison.md`
- `specs/examples/90-current-l2-stage3-request-local-clause-spillover-comparison.md`
- `specs/examples/91-current-l2-stage3-request-local-clause-spillover-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0295-phase012-top-level-consistency-sweep.md`
- `docs/reports/0296-review-phase012-top-level-consistency-sweep.md`
- `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json`
- `crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json`
- `scripts/current_l2_detached_loop.py`

## 3. Actions taken

1. 0295/0296 の late reviewer finding を再確認し、top-level helper help prose と report exact output placeholder を current task で補正する方針を固めた。
2. stage 3 later branch の current source anchor を読み直し、
   - declaration-side `admit` handoff deferred line
   - bare request-local clause spillover first tranche
   - minimal predicate fragment candidate
   の接続面を整理した。
3. `specs/examples/92-current-l2-stage3-predicate-fragment-reopen-sequencing.md` を追加し、request head + clause attachment multiline shape より先に predicate fragment boundary の reopen 条件を比較する current sequencing judgment を記述した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/07-parser-free-poc-stack.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` を mirror 更新した。
5. `scripts/current_l2_detached_loop.py` の top-level help prose を static gate を含む actual helper surface に合わせて補正した。

## 4. Files changed

- `scripts/current_l2_detached_loop.py`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/92-current-l2-stage3-predicate-fragment-reopen-sequencing.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0295-phase012-top-level-consistency-sweep.md`
- `docs/reports/0296-review-phase012-top-level-consistency-sweep.md`
- `docs/reports/0297-current-l2-stage3-predicate-fragment-reopen-sequencing.md`
- `docs/reports/0298-review-current-l2-stage3-predicate-fragment-reopen-sequencing.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 13:29 JST

$ python3 scripts/current_l2_detached_loop.py --help | sed -n '1,18p'
usage: current_l2_detached_loop.py [-h]
                                   {emit-fixture,emit-aggregate,emit-static-gate,compare-artifacts,compare-aggregates,compare-static-gates,...}
                                   ...

current L2 detached validation loop を回すための non-production helper。 bundle-first
/ aggregate / static gate emitter と bundle / aggregate / static gate diff
helper を薄くつなぐ。

$ rg -n "static gate artifact loop|specs/examples/92-current-l2-stage3-predicate-fragment-reopen-sequencing.md|predicate fragment boundary の reopen 条件|Phase 3 を主線" Documentation.md specs/00-document-map.md plan/07-parser-free-poc-stack.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/90-source-traceability.md progress.md
Documentation.md:179:109. stage 3 later branch の次段として request head + clause attachment multiline shape より先に predicate fragment boundary の reopen 条件を比較する current sequencing judgment は `specs/examples/92-current-l2-stage3-predicate-fragment-reopen-sequencing.md`
plan/07-parser-free-poc-stack.md:176:- stage 3 later branch の次段 sequencing では、request head + clause attachment multiline shape より先に predicate fragment boundary の reopen 条件を docs-only で比較する
plan/11-roadmap-near-term.md:10:current Phase 3 の next narrow step は、stage 3 later branch で request head + clause attachment multiline shape を先に開かず、predicate fragment boundary の reopen 条件を先に比較することである。
plan/90-source-traceability.md:610:  - `specs/examples/92-current-l2-stage3-predicate-fragment-reopen-sequencing.md`
progress.md:116:  - next docs-only sequencing としては、request head + clause attachment multiline shape より先に predicate fragment boundary の reopen 条件を比較する

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 298 numbered report(s).

$ git diff --check
[no output]
```

## 6. Evidence / findings

- stage 3 later branch の open pressure は 2 つあるが、shared floor は対称ではない。
  - request head + clause attachment multiline shape は request-local branch の structural rule を先に増やす
  - predicate fragment boundary は declaration-side `admit` line と request-local clause line の両方に共有される
- `specs/examples/89` が fixture-side `OptionDecl.admit` handoff reopen 条件を predicate fragment boundary 側に置いているため、request attachment を先に開くと handoff deferred line と sequencing がねじれる。
- `specs/examples/30` が minimal predicate fragment well-formedness を first checker cut 候補 cluster に含めているため、predicate fragment reopen は parser boundary と checker boundary の shared floor として扱いやすい。
- late reviewer finding は 2 件だけであり、
  - detached loop helper の short summary wording
  - 0295 の exact output placeholder
  の補正で閉じた。Phase wording 自体に新たな drift はなかった。
- review close-out は `docs/reports/0298-review-current-l2-stage3-predicate-fragment-reopen-sequencing.md` に記録し、substantive spec finding がなかったことを確認した。

## 7. Changes in understanding

- stage 3 later branch では、request-local clause line の bare spillover pair を actualize した後に request attachment multiline shape を先に開くのではなく、predicate payload 側の shared floor を先に切る方が declaration-side / request-local の両 branch を同じ理論線に保てる。
- predicate fragment boundary の reopen は、immediate parse actualization ではなく、shared floor comparison の reopen 条件を先に切る段階として読むのが自然である。
- Phase 0 / 1 / 2 closeout follow-up として残っていた top-level helper summary drift は current task で吸収できた。

## 8. Open questions

- minimal predicate fragment reopen の first docs-only cutを、opaque slot retention / minimal parsed fragment / shared lowered subset compare のどこに置くか。
- request head + clause attachment multiline shape を later step に残すとして、その reopening trigger を predicate fragment comparison のどの結果に結び付けるか。

## 9. Suggested next prompt

`stage 3 later branch の次段として、declaration-side admit と request-local require/ensure に共有される minimal predicate fragment boundary を current phase でどこまで reopen してよいか、opaque slot retention / minimal parsed fragment / shared compare surface の 3 案で docs-only 比較してください。`
