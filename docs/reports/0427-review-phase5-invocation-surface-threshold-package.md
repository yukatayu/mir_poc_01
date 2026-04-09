# 0427 — review phase5 invocation-surface threshold package

## Objective

`docs/reports/0426-phase5-invocation-surface-threshold-package.md` と 155 package の mirror について、threshold semantics、mirror drift、provenance、report hygiene が current repo の task close contract を満たしているかを確認する。

## Scope and assumptions

- 対象は `specs/examples/155-current-l2-theorem-line-validation-ready-invocation-surface-threshold.md` とその関連 mirror に限る。
- reviewer subagent は 1 回だけ起動し、180 秒 wait を行う。
- finding があれば package 内で吸収する。

## Documents consulted

- `docs/reports/0426-phase5-invocation-surface-threshold-package.md`
- `specs/examples/155-current-l2-theorem-line-validation-ready-invocation-surface-threshold.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. reviewer subagent `019d7325-6fa3-7be2-be60-1ed582611515` を起動した。
2. `wait_agent` を 180 秒で 1 回行い、completion を受け取った。
3. reviewer finding 2 件を受け、`progress.md` の Phase 5 top summary を 155 package に追従させ、`docs/reports/0426-phase5-invocation-surface-threshold-package.md` の consulted list に `plan/00-index.md` を追加した。
4. `python3 scripts/validate_docs.py`、`git diff --check`、`git status --short --branch` で closeout evidence を確認した。
5. reviewer subagent は close 済みである。

## Files changed

- `progress.md`
- `docs/reports/0426-phase5-invocation-surface-threshold-package.md`
- `docs/reports/0427-review-phase5-invocation-surface-threshold-package.md`

## Commands run

- `wait_agent 019d7325-6fa3-7be2-be60-1ed582611515 (180s x 1)`
- `nl -ba progress.md | sed -n '34,50p'`
- `nl -ba docs/reports/0426-phase5-invocation-surface-threshold-package.md | sed -n '1,80p'`
- `python3 scripts/validate_docs.py`
- `git diff --check`
- `git status --short --branch`

## Evidence / outputs / test results

- reviewer finding: 2 件
  - `progress.md` の Phase 5 top-level progress mirror が stale で、155 package 後の状態と矛盾していた
  - 0426 report の consulted list に `plan/00-index.md` が欠けていた
- reviewer completion では、semantic cut 自体には substantive finding が無かった
  - `consumer_invocation_surface_ref` だけを current first choice に含めること
  - next later reopen が `exchange-body threshold` に揃っていること
  は 155 spec / mirror / tasks / detailed progress row で整合している
- post-fix validation:
  - `python3 scripts/validate_docs.py` → `Documentation scaffold looks complete.` / `Found 427 numbered report(s).`
  - `git diff --check` → 無出力
  - `git status --short --branch` → 155 package 差分のみが残る状態

## What changed in understanding

- 155 package の semantics 自体は安定しており、closeout で吸収すべき論点は top-level progress mirror と consulted provenance の completeness である。
- `progress.md` は detailed row だけでなく phase summary も current package close に追従している必要がある。
- roadmap / current L2 task では `plan/00-index.md` の consulted provenance を report に残しておく方が repo rule と整合する。

## Open questions

- exchange-body threshold comparison の row split は次 task に残る。
- `plan/` 更新不要。review record task では新しい規範判断は作っていない。
- `progress.md` 更新不要。review record task では phase 読みそのものは package fix の一部として吸収済みであり、新しい方針は作っていない。
- `tasks.md` 更新不要。review record task では task map を変えていない。

## Suggested next prompt

Phase 5 theorem-line later reopen の次段として、`consumer_invocation_surface_ref` の上に `exchange_rule_body_ref` をどこまで足してよいか、concrete file-blob exchange protocol / environment-specific runtime coupling / failure body をどう分けるべきかを narrow に比較してください。
