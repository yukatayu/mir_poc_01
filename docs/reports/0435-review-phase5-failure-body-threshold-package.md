# Report 0435 — review Phase 5 failure-body threshold package

- Date: 2026-04-10 02:58 JST
- Author / agent: Codex GPT-5
- Scope: Review of the uncommitted Phase 5 failure-body threshold package and listed mirror files
- Decision levels touched: No normative change; review of current L2 docs-first threshold package and mirror hygiene

## 1. Objective

Uncommitted な Phase 5 failure-body threshold package を review し、

- current first choice が `failure_body_ref` のみ追加に正しく留まっているか
- actual invocation protocol / host binding / failure wording が deferred のまま保たれているか
- mirror files の next reopen が `actual-invocation-protocol threshold` に揃っているか
- report 0434 の evidence / hygiene に問題がないか

を source-backed に確認する。

## 2. Scope and assumptions

- scoped files 以外には立ち入らず、failure-body threshold package の additive change と report hygiene だけを見る。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/158-current-l2-theorem-line-runtime-coupling-ready-transport-protocol-threshold.md`
- `specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0434-phase5-failure-body-threshold-package.md`

## 4. Actions taken

1. AGENTS の必読順に沿って baseline docs と normative specs を読んだ。
2. scoped files の uncommitted diff を確認した。
3. theorem-line chain の直前 158 と新規 159 を line-by-line で比較した。
4. mirror files 内の next reopen wording が `actual-invocation-protocol threshold` に揃っているか検索した。
5. report 0434 の recorded evidence を、current repo state (`python3 scripts/validate_docs.py`、`git status --short --branch`) と照合した。

## 5. Files changed

- `docs/reports/0435-review-phase5-failure-body-threshold-package.md`

`plan/ 更新不要`

`progress.md 更新不要`

`tasks.md 更新不要`

## 6. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-10 02:58 JST
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 435 numbered report(s).
```

```text
$ git status --short --branch
## main...origin/main
 M Documentation.md
 M docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md
 M plan/11-roadmap-near-term.md
 M plan/12-open-problems-and-risks.md
 M plan/13-heavy-future-workstreams.md
 M plan/17-research-phases-and-autonomy-gates.md
 M plan/90-source-traceability.md
 M progress.md
 M specs/00-document-map.md
 M tasks.md
?? docs/reports/0434-phase5-failure-body-threshold-package.md
?? docs/reports/0435-review-phase5-failure-body-threshold-package.md
?? specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md
```

## 7. Evidence / findings

- Finding 1: `progress.md` の summary paragraph と work log が 159 state と整合していなかった。
- Finding 2: `plan/17` の sequencing line が旧 reopen target のままだった。
- Finding 3: report 0434 は validation / review evidence を exact outputs として残していなかった。
- No semantic finding: 159 の minimal sketch と practical examples は 158 から additive であり、追加 field は `failure_body_ref` のみである。
- No defer-boundary finding: 159 本文は actual invocation protocol / host binding / failure wording を deferred のまま保っている。

## 8. What changed in understanding

- current issue は package semantics ではなく snapshot / report hygiene に限られていた。
- 158 から 159 への change は additive のまま保てており、failure body line を actual invocation protocol から narrow に切れる。

## 9. Open questions

- `actual_invocation_protocol_ref` をどこまで theorem-line retained bridge に寄せてよいか。
- consumer-host-specific binding と failure wording を still later reopen に保てるか。

## 10. Suggested next prompt

`actual-invocation-protocol threshold` を docs-first で比較し、`failure_body_ref` の次段として `actual_invocation_protocol_ref` をどこまで retained bridge に寄せてよいかを source-backed に整理してください。
