# 0290 — shared-space admission policy and compile-time visibility

## Objective

shared-space line の次段として、admission policy と compile-time over-approximation の接点を、

- runtime-only admission
- declared role / capability / visibility over-approximation + runtime admission
- closed-world exact admission / visibility set

の 3 案で比較し、current first practical candidate を docs-first に整理する。

## Scope and assumptions

- 今回は docs-only comparison に留める。
- actual auth protocol、consensus algorithm、exact type syntax は固定しない。
- identity core と auth stack / admission policy を分ける current line を前提に、その compile-time / runtime boundary を詰める。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/reports/0270-shared-space-activation-rule-comparison.md`
- `docs/reports/0288-shared-space-identity-auth-layering-comparison.md`

## Actions taken

1. `plan/16` に admission policy / compile-time visibility section を追加した。
2. runtime-only / over-approx + runtime admission / closed-world exact set の 3 案を比較した。
3. authoritative すごろく room と append-friendly room の pseudocode を追加し、compile-time に残すべき declaration と runtime に残すべき control-plane を分けた。
4. `plan/12` に新しい open problem row を追加した。
5. `progress.md` に current first practical candidate を mirror した。
6. `plan/90` に traceability mirror を追加した。

## Files changed

- `docs/reports/0290-shared-space-admission-policy-and-compile-time-visibility.md`
- `docs/reports/0291-review-shared-space-admission-policy-and-compile-time-visibility.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 12:45 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 291 numbered report(s).

$ git diff --check
[no output]
```

## Evidence / outputs / test results

Task start dirty state:

```text
## main...origin/main
```

Review:

- reviewer completion は current interface では取得できなかったため、`docs/reports/0291-review-shared-space-admission-policy-and-compile-time-visibility.md` に local review fallback を記録した
- local review では substantive inconsistency は見つからず、compile-time over-approximation と runtime control-plane の切り分け、traceability、progress mirror の closeout を確認した

## What changed in understanding

- current shared-space line で compile-time に残したいのは actual principal set ではなく、role / capability / visibility requirement の over-approximation である。
- fixed small room proof に寄せて closed-world exact admission を compile-time に上げるのは理論的にはきれいでも、current shared-space mainline には早すぎる。
- `authority-ack` activation と identity/auth layering の current lineとは、declared over-approx + runtime admission が最も自然に接続する。

## Open questions

- room capability declaration と visibility declaration を同じ syntax cluster に置くか。
- compile-time compare family へ上げるなら、role/capability requirement をどこまで structural floor として持てるか。
- fairness witness と admission policy witness を将来どこで接続するか。

## plan/progress update status

- `plan/` 更新あり
- `progress.md` 更新あり

## Suggested next prompt

`shared-space fairness witness と identity core / principal continuity の接点を、audit artifact line とどう切るか、current shared-space line の残課題として narrow に比較してください。`
