# 0270 — shared-space activation rule comparison

## Objective

shared-space / membership line の次段として、participant active 化規則を

- `authority-ack`
- `full-coverage-like activation`
- `quorum-like activation`

の 3 案で比較し、authoritative room と append-friendly room で何が最小 operational candidate かを docs-first に整理する。

## Scope and assumptions

- 今回は docs-only の comparison に留める。
- final activation rule を固定しない。
- compile-time では visibility role と required path の over-approximation までを扱い、actual active set / acknowledgement frontier は runtime control-plane に残す。
- `Raft` / `Paxos` / `VRR` 等の algorithm 名は operational realization に残し、language / shared-space spec の必須語彙にしない。

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
- `plan/10-roadmap-overall.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `docs/reports/0264-shared-space-membership-boundary-and-example.md`
- `docs/reports/0266-shared-space-membership-research-line-refresh.md`
- `docs/reports/0268-shared-space-authority-ownership-and-consistency-comparison.md`
- `docs/reports/0269-review-shared-space-authority-doc-change.md`

## Actions taken

1. `plan/16` の activation compile-time section を拡張し、`authority-ack` / `full-coverage-like activation` / `quorum-like activation` の 3 案を分解した。
2. 各案について、
   - operational reading
   - 利点
   - 欠点
   - 向く room
   を書き分けた。
3. authoritative room と append-friendly room での current working judgment を追記した。
4. `plan/12` の dynamic membership / causal metadata row に、authoritative room の最小候補は `authority-ack`、他 2 案は policy option comparison に残すことを追記した。
5. `plan/10` と `progress.md` を mirror 更新した。
6. `plan/90` を更新し、`plan/10` / `plan/12` / `plan/16` の今回変更が `0270` / `0271` に依拠することを traceability に反映した。

## Files changed

- `docs/reports/0270-shared-space-activation-rule-comparison.md`
- `docs/reports/0271-review-shared-space-activation-rule-comparison.md`
- `plan/10-roadmap-overall.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Evidence / outputs / test results

Task start dirty state:

```text
## main...origin/main
```

Timestamp used for progress update:

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 11:05 JST
```

Validation after review:

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 271 numbered report(s).

$ git diff --check
[no output]
```

Reviewer:

- reviewer 1 回実施
- substantive semantic finding なし
- process finding 2 件
  - `plan/90` の traceability mirror に `0270` / `0271` を追加する必要
  - report の consulted-doc / evidence section を close out する必要
- いずれも task 内で反映済み

## What changed in understanding

- authoritative room に対しては、active 化規則の最小 operational candidate を `authority-ack` に置くのが最も自然だと整理できた。
- `full-coverage-like activation` は fixed small room には直感的だが、open join / leave / reconnect を含む shared-space では成立条件そのものが揺れやすい。
- `quorum-like activation` は replicated authority と相性があるが、activation rule だけで consensus flavor が前面に出やすく、current phase では重い。
- append-friendly room では membership activation と data-plane visibility を同じ acknowledgement rule で縛らない方が、participant carrier / consistency mode / authority placement の分離が保ちやすい。

## Open questions

- authoritative room の active 化を authority 1 点受理で十分とするか、それとも一部 room だけ full-coverage-like を選べるようにするか。
- replicated authority room で `quorum-like activation` を comparison option 以上に上げる threshold は何か。
- late join / reconnect / in-flight action collision を activation rule とどこまで同じ carrier に入れるか。
- active 化規則と fairness / RNG trust model をどこで接続するか。

## plan/progress update status

- `plan/` 更新あり
- `progress.md` 更新あり

## Suggested next prompt

`shared-space authoritative room の authority placement について、single room authority / replicated authority / relaxed room projection の 3 案を、activation rule と consistency mode の相性つきで narrow に比較してください。`
