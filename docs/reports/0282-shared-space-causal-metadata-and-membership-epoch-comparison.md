# 0282 — shared-space causal metadata and membership epoch comparison

## Objective

shared-space / membership research line の次段として、participant churn と causal metadata の接続を

- plain vector deletion
- epoch / incarnation split
- control-plane separated carrier

の 3 案で比較し、current phase の first practical candidate を docs-first に整理する。

## Scope and assumptions

- 今回は docs-only comparison に留める。
- final distributed clock algorithm や exact serialization は固定しない。
- authoritative room / append-friendly room をまたぐ general boundary comparison として扱う。
- current `member_incarnation` working line を前提に、その causal-carrier 側の位置づけを詰める。

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
- `docs/reports/0264-shared-space-membership-boundary-and-example.md`
- `docs/reports/0280-shared-space-reconnect-and-inflight-policy-cut.md`

## Actions taken

1. `plan/16` の vector-clock / membership section を、plain vector deletion / epoch-incarnation split / control-plane separated carrier の 3 案比較へ整理し直した。
2. leave / rejoin と old message collision を使って、何が防げて何が still OPEN かを書き分けた。
3. `plan/12` に current first practical candidate を mirror した。
4. `progress.md` を更新し、shared-space line の causal metadata cut を current snapshot に反映した。
5. `plan/90` に traceability mirror を追加した。

## Files changed

- `docs/reports/0282-shared-space-causal-metadata-and-membership-epoch-comparison.md`
- `docs/reports/0283-review-shared-space-causal-metadata-and-membership-epoch-comparison.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 12:13 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 283 numbered report(s).

$ git diff --check
[no output]
```

## Evidence / outputs / test results

Task start dirty state:

```text
## main...origin/main
```

Reviewer:

- reviewer を 1 回起動し、さらに 1 回 retry したが、今回の wait window では completion を取得できなかった
- そのため AGENTS / repo 運用に従い、local evidence fallback で close out した
- local review では次だけを確認した
  - plain vector deletion を current first choice にしない line になっている
  - epoch / incarnation split は first practical candidate だが final clock algorithm へ過剰 commit していない
  - control-plane separated carrier は current default ではなく next stronger candidate に留まっている
  - `plan/90` に `0282` / `0283` の traceability mirror を追加済み

## What changed in understanding

- leave / rejoin と old message collision を扱うとき、plain vector deletion は current first choice にしない方が自然だと整理できた。
- `member_incarnation` を reconnect policy 側だけでなく causal carrier 側にも接続するなら、epoch / incarnation split が first practical candidate になる。
- control-plane separated carrier はさらに clean だが、current phase では operational realization 寄りなので next stronger candidate に残すのが妥当である。

## Open questions

- config epoch を data-plane artifact にどこまで mirror するか。
- control-plane separated carrier を reopen する threshold は何か。
- append-friendly room で actor incarnation をどこまで strict に持たせるか。

## plan/progress update status

- `plan/` 更新あり
- `progress.md` 更新あり

## Suggested next prompt

`shared-space identity / auth layering と fairness trust model のどちらを先に詰めるべきかを、current shared-space line の残課題として narrow に比較してください。`
