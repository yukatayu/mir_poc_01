# Report 0350 — review task snapshot and maintenance rule

- Date: 2026-04-09 09:48 JST
- Author / agent: Codex
- Scope: `tasks.md` 導入 task の review record と local fallback
- Decision levels touched: none

## 1. Objective

`tasks.md` の追加と maintenance rule 更新が、current phase reading や非規範文書の境界を壊していないかを確認し、その review evidence を記録する。

## 2. Inputs consulted

- `tasks.md`
- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `progress.md`
- `docs/reports/0349-task-snapshot-and-maintenance-rule.md`

## 3. Actions taken

1. reviewer subagent を 1 回起動して completion を待つ運用を試みた。
2. しかし usable handle が返らず継続待機できなかったため、AGENTS の fallback に従って local diff review に切り替えた。
3. `tasks.md` の blocker recommendation と current phase reading が、`progress.md` / `plan/11` / `plan/12` / `plan/17` と一致しているかを照合した。
4. `tasks.md` が規範判断の正本に見えないよう、`specs/00-document-map.md`、`plan/90-source-traceability.md`、`AGENTS.md`、`plan/91-maintenance-rules.md` の wording を再確認した。
5. wording hygiene として `tasks.md` の英語混じり 1 箇所を日本語へ寄せた。

## 4. Files changed

- `tasks.md`
- `docs/reports/0350-review-task-snapshot-and-maintenance-rule.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M JST'
2026-04-09 09:48 JST

$ rg -n "authority-ack|single room authority|append-friendly room|authority_rng|delegated_rng_service|Phase 3 self-driven portion|reserve path|Phase 2 maintenance tail|Phase 4 side line|Phase 5 inventory line|shared-space / membership|static analysis / type / theorem prover|detached validation loop" progress.md plan/11-roadmap-near-term.md plan/12-open-problems-and-risks.md plan/16-shared-space-membership-and-example-boundary.md plan/17-research-phases-and-autonomy-gates.md
[matches confirming current recommendation and phase reading across the consulted files]
```

## 6. Evidence / findings

- substantive finding は無い。
- `tasks.md` の current recommendation は、少なくとも次と整合している。
  - Phase 3 を reserve path に戻した current phase reading
  - detached validation loop を Phase 2 maintenance tail の主線候補に置く reading
  - shared-space / membership を Phase 4 side line として進める reading
  - static analysis / type / theorem prover boundary を Phase 5 inventory line として進める reading
- `tasks.md` を規範文書と誤読しないための guard も、top-level doc map と maintenance rule に入っている。

## 7. Changes in understanding

- `tasks.md` は progress の代替ではなく、execution-facing snapshot として分ける方が current repo には合っている。
- reviewer handle 不達時の fallback でも、source-backed な local diff review を report 化すれば運用上の穴は小さく保てる。

## 8. Open questions

- reviewer subagent / handle が返らないケースを repository rule 上どう扱うかは、必要なら後で maintenance note に切り出してもよい。

## 9. Suggested next prompt

`tasks.md` の最優先 package から 1 本を選び、docs-first comparison と PoC evidence を伴う current checkpoint まで自走で進めてください。
