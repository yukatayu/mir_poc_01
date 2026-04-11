# Report 0549 — phase5 higher level async control family comparison

- Date: 2026-04-11T01:44:34.752468Z
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen として、checker-verdict-transport-channel-body-ready higher-level async-control family の current first cut を比較し、続けて authority-serial transition family の symbolic retained bridge threshold まで docs-first で固定する。
- Decision levels touched: normative `specs/examples/` judgment, mirror and snapshot updates

## 1. Objective

`specs/examples/218-current-l2-theorem-line-checker-verdict-transport-channel-body-ready-low-level-memory-order-family-threshold.md`
を前提に、low-level memory-order family を still 外に残したまま

1. `event_tree_execution_view` / `authority_serial_transition_family` / `witness_aware_commit_family`
   のどれを higher-level async-control family の current first cut に置くかを比較し、
2. current first cut になった family を theorem-line retained bridge にどこまで近づけるか

を source-backed に整理する。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/218-current-l2-theorem-line-checker-verdict-transport-channel-body-ready-low-level-memory-order-family-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0358-async-control-memory-boundary-inventory.md`

## 3. Actions taken

1. `specs/examples/219-...higher-level-async-control-family-comparison.md` を追加し、3 候補比較を整理した。
2. `specs/examples/220-...authority-serial-transition-family-threshold.md` を追加し、current first cut になった `authority_serial_transition_family` の symbolic retained bridge threshold を整理した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、Phase 5 research abstract を current snapshot に更新した。

## 4. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/219-current-l2-theorem-line-checker-verdict-transport-channel-body-ready-higher-level-async-control-family-comparison.md`
- `specs/examples/220-current-l2-theorem-line-higher-level-async-control-family-ready-authority-serial-transition-family-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0549-phase5-higher-level-async-control-family-comparison.md`
- `docs/reports/0550-phase5-higher-level-async-control-authority-serial-threshold.md`
- `docs/reports/0551-review-phase5-higher-level-async-control-family-comparison-package.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-11 10:43 JST

$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   92G  2.1G  98% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       787Mi        93Mi       1.2Mi       233Mi       172Mi
Swap:           19Gi       2.1Gi        17Gi

$ python3 scripts/new_report.py --slug phase5-higher-level-async-control-family-comparison
/home/yukatayu/dev/mir_poc_01/docs/reports/0549-phase5-higher-level-async-control-family-comparison.md

$ python3 scripts/new_report.py --slug phase5-higher-level-async-control-authority-serial-threshold
/home/yukatayu/dev/mir_poc_01/docs/reports/0550-phase5-higher-level-async-control-authority-serial-threshold.md
```

検証コマンドは task close 前に別途実行する。

## 6. Evidence / findings

- `event_tree_execution_view` は current repo では derived execution / debug / explanation view として読む方が自然であり、theorem-line retained bridge の first async-control family には強すぎる。
- `witness_aware_commit_family` は authority ordering より一段強く、witness / receipt / replay attachment を premature に押し込みやすい。
- `authority_serial_transition_family` は Phase 4 authoritative room baseline と最も直接つながり、source-of-truth ordering family として first cut に置くのが最も narrow である。
- current threshold では actual authority protocol row までは入れず、`family_kind = authority_serial_transition_family` を持つ symbolic retained bridge に留めるのが最小である。

## 7. Changes in understanding

- higher-level async-control family の first cut は、derived view や witness attachment より先に source-of-truth ordering family を切る方が理論的にきれいである。
- theorem-line retained bridge の async-control side は、low-level memory-order family を still 外に残したまま、`authority_serial_transition_family` を symbolic retained bridge として足すところまでは source-backed に進めてよい。

## 8. Open questions

- `authority_serial_transition_contract` の minimal row core をどこまで narrow に切るか
- `witness_aware_commit_family` を second candidate として reopen するとき、authority-serial contract row とどこまで shared row にするか
- `event_tree_execution_view` を derived execution / debug view として theorem-line へどう接続するか

## 9. Suggested next prompt

Phase 5 の next self-driven step として、`authority-serial-transition-family-ready authority-serial-transition-contract comparison` を 3 案で比較し、minimal `authority_serial_transition_contract` row を theorem-line retained bridge にどこまで近づけるかを docs-first で整理してください。
