# 0276 — shared-space RNG / fairness source placement

## Objective

shared-space / membership research line の次段として、RNG / fairness source placement を

- `authority_rng`
- `delegated_rng_service`
- `distributed_randomness_provider`

の 3 案で比較し、authoritative room と append-heavy room でどの placement が current phase に自然かを docs-first に整理する。

## Scope and assumptions

- 今回は docs-only comparison に留める。
- final fairness proof model や distributed randomness protocol は固定しない。
- participant carrier / authority placement / consistency mode と RNG provider placement を意図的に別軸で扱う。
- auth / identity / reconnect policy の finalization には進まない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `plan/00-index.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `docs/reports/0268-shared-space-authority-ownership-and-consistency-comparison.md`
- `docs/reports/0270-shared-space-activation-rule-comparison.md`
- `docs/reports/0272-shared-space-authority-placement-comparison.md`
- `docs/reports/0274-shared-space-consistency-mode-comparison.md`

## Actions taken

1. `plan/16` に RNG / fairness source placement の候補比較を詳細化した。
2. `authority_rng` / `delegated_rng_service` / `distributed_randomness_provider` を、
   - 読み
   - 利点
   - 欠点
   - room type ごとの向き
   で書き分けた。
3. authoritative room と append-heavy room の current working judgment を追記した。
4. `plan/12` の shared-space authority / resource ownership risk row に、RNG / fairness source の最小候補と next practical candidate を反映した。
5. `progress.md` を更新し、shared-space line の RNG / fairness source placement を current snapshot に反映した。
6. `plan/90` に traceability mirror を追加した。

## Files changed

- `docs/reports/0276-shared-space-rng-fairness-source-placement.md`
- `docs/reports/0277-review-shared-space-rng-fairness-source-placement.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 11:44 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 277 numbered report(s).

$ git diff --check
[no output]
```

## Evidence / outputs / test results

Task start dirty state:

```text
## main...origin/main
 M plan/12-open-problems-and-risks.md
 M plan/16-shared-space-membership-and-example-boundary.md
 M progress.md
```

Reviewer:

- reviewer 1 回実施
- substantive semantic finding なし
- hygiene finding 2 件
  - `plan/90` の traceability mirror に `0276` / `0277` を追加する必要
  - report の command trace / final validation evidence を close out する必要
- いずれも task 内で反映済み

## What changed in understanding

- authoritative room の current first choice は `authority_rng` が最も自然だと整理できた。
- fairness source を participant tree topology に埋め込むより、authority placement と切り離した explicit provider capability として持つ方が、audit / replacement / HW hook の観点で自然である。
- append-heavy room では shared RNG 自体が不要なことも多く、必要な場合でも `delegated_rng_service` を next practical candidate に置く方が room rule と randomness provider を混線させにくい。
- `distributed_randomness_provider` は fairness claim を強めうるが、membership churn / timeout / reconnect policy と一体化しやすく、current phase では future comparison に残すのが妥当である。

## Open questions

- `delegated_rng_service` の provider identity / audit reference / failure policy をどこで切るか。
- `distributed_randomness_provider` を reopen する threshold は何か。
- fairness trust model と reconnect / late-join policy をどこで接続するか。

## plan/progress update status

- `plan/` 更新あり
- `progress.md` 更新あり

## Suggested next prompt

`shared-space authoritative game room について、activation / authority placement / consistency mode / RNG-fairness source の 4 軸を 1 つの concrete profile に束ね、すごろく風の room 例で実践的に比較してください。`
