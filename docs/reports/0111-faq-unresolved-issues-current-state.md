# 0111 — faq unresolved issues current state

## Objective

ユーザが現時点で気にしている 3 つの未解決論点、

1. stateful resource を保持した継続の multi-shot 化
2. participant churn を持つ shared-space / session 系での causal metadata と membership change
3. nested fallback / fallback stop 構文の理論的位置づけ

について、repo 内の正本に沿って current state を整理し、`faq_001.md` に日本語で丁寧にまとめる。
必要なら serious open problem として `plan/` に反映する。

## Scope and assumptions

- current L2 の core semantics、parser-free PoC、detached validation loop の既決事項は変更しない。
- `faq_001.md` は規範文書ではなく、`specs/` と `plan/` を横断した human-facing FAQ とする。
- current repo で未決のものは未決のまま書き、仮説を既決事項へ昇格させない。
- subagent / reviewer は使わず、local diff review と local validation で閉じる。

## Documents consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/04-mir-core.md`
9. `specs/05-mirrorea-fabric.md`
10. `specs/09-invariants-and-constraints.md`
11. `specs/10-open-questions.md`
12. `specs/11-roadmap-and-workstreams.md`
13. `specs/12-decision-register.md`
14. `plan/00-index.md`
15. `plan/01-status-at-a-glance.md`
16. `plan/02-system-overview-and-positioning.md`
17. `plan/03-decision-strengths-and-boundaries.md`
18. `plan/04-core-semantics-current-l2.md`
19. `plan/05-fallback-lease-and-chain-semantics.md`
20. `plan/06-surface-notation-status.md`
21. `plan/07-parser-free-poc-stack.md`
22. `plan/08-representative-programs-and-fixtures.md`
23. `plan/09-helper-stack-and-responsibility-map.md`
24. `plan/10-roadmap-overall.md`
25. `plan/11-roadmap-near-term.md`
26. `plan/12-open-problems-and-risks.md`
27. `plan/13-heavy-future-workstreams.md`
28. `plan/14-glossary-and-boundary-rules.md`
29. `plan/15-current-l2-fixture-authoring-template.md`
30. `plan/90-source-traceability.md`
31. `plan/91-maintenance-rules.md`
32. `progress.md`
33. `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
34. `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`

## Actions taken

1. current repo が 3 論点のどこまで決めているかを `specs/` / `plan/` / `docs/reports/` から再確認した。
2. 継続 / coroutine 周辺について、
   - unrestricted coroutine model は採らないこと
   - one-shot / multi-shot / capture restriction は OPEN であること
   - linear / stateful resource と unrestricted multi-shot が衝突すること
   を整理した。
3. participant churn / causal metadata について、
   - current L2 mainline では未規範であること
   - shared-space / Mirrorea future work であること
   - plain vector deletion だけでは old membership と rejoin を区別できないこと
   を整理した。
4. fallback について、
   - current L2 は guarded option chain であり outer wrapper ではないこと
   - nested fallback は canonical chain に flatten すること
   - stop 構文を fallback 本体へ入れるより selection / elaboration に分ける方が自然、という考察
   を整理した。
5. `faq_001.md` を追加し、3 論点それぞれについて
   - current repo で決まっていること
   - 未決のこと
   - 私見と小さい証明スケッチ
   を日本語でまとめた。
6. `plan/12-open-problems-and-risks.md` に、current mainline の外側にあるが serious open problem として
   - constrained continuation / multi-shot
   - dynamic membership / causal metadata
   を追加した。
7. `plan/90-source-traceability.md` を更新し、`plan/12` の根拠に `specs/04` / `specs/05` / `specs/11` を補った。

## Files changed

- `faq_001.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0111-faq-unresolved-issues-current-state.md`

## Commands run

```bash
git status --short --branch
rg -n "multi-shot|multishot|continuation|coroutine|resume|linear resource|ownership|lifetime|rollback" specs plan docs/reports crates
rg -n "vector clock|vector-clock|clock|participant|join|leave|membership|session|shared state|consistency|space" specs plan docs/reports crates
rg -n "fallback|preference chain|lease-expired|no re-promotion|outer|inner|wrapper|lineage" specs plan docs/reports crates
python3 scripts/validate_docs.py
git diff --check
git commit --no-gpg-sign -m '継続と membership と fallback の FAQ を整理する'
git commit --no-gpg-sign -m 'FAQ 整理の作業報告を追加する'
```

## Evidence / outputs / test results

- task 開始時 dirty state: `## main...origin/main [ahead 7]`、worktree は clean
- local review:
  - `faq_001.md` 後半まで通読し、3 論点の結論と proof sketch を確認した
  - `plan/12-open-problems-and-risks.md` に追加した 2 項目が `OPEN / FUTURE` として書かれていることを確認した
  - `plan/90-source-traceability.md` に `plan/12` の source anchor を補った
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 111 numbered report(s).`
- `git diff --check`
  - 無出力
- `cargo test` は未実行
  - 今回は `faq_001.md`、`plan/12`、`plan/90`、report の docs-only 変更であり、Rust code / tests / fixtures は変更していないため
- subagent / reviewer は未使用
  - user から明示的な delegation 要請がないため、本 task は local review と local validation で閉じた
- commit:
  - `fe2635a` `継続と membership と fallback の FAQ を整理する`

## What changed in understanding

- current repo は fallback についてはかなり強く決まっており、FAQ の 3 論点の中では最も settled である。
- 一方で multi-shot continuation と dynamic membership / causal metadata は、current L2 mainline の外側にある serious open problem であり、repo の一般原則から「どちらの方向が不自然か」までは言えるが、最終 formalization はまだ行っていない。
- 特に vector clock 的議論は、causal carrier と membership reconfiguration を同じものとして扱うより、epoch / incarnation を含む control-plane event に切り分ける方が repo の explicit boundary 原則と整合する。

## Open questions

- multi-shot continuation を将来入れるなら、shareable capture の formal criterion をどの layer で表すか
- participant churn を持つ shared-space / session で、membership epoch / incarnation / activation rule をどこまで current architecture に落とし込むか
- fallback の outer-condition-based chain selection を、将来 selection / elaboration construct としてどこまで formalize するか

## plan/ progress updates

- `plan/` 更新あり:
  - `plan/12-open-problems-and-risks.md`
  - `plan/90-source-traceability.md`
- `progress.md 更新不要`

## Specification-document commit hashes

- 本 task の FAQ / plan 反映 commit:
  - `fe2635a` `継続と membership と fallback の FAQ を整理する`
- report 自身の commit hash は self-reference の都合で本文に固定しない。

## Suggested next prompt

`faq_001.md` で整理した 3 論点のうち 1 つを選び、current repo でどこまで docs-only に formalize できるかを narrow scope で詰めてください。特に multi-shot continuation の capture restriction か、shared-space の membership / causal metadata 分離のどちらかを future workstream entry note として 1 本に切り出すと、次の設計判断がしやすくなります。
