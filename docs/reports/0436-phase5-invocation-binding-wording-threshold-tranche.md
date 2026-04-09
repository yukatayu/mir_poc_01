# Report 0436 — phase5 invocation / binding / wording threshold tranche

- Date: 2026-04-10 03:18 JST
- Author / agent: Codex GPT-5
- Scope: Phase 5 theorem-line later tranche for actual-invocation-protocol / consumer-host-binding / failure-wording thresholds and mirror updates
- Decision levels touched: L2 docs-first threshold judgments and repository-memory mirror maintenance

## 1. Objective

Phase 5 theorem-line later reopen の current next tranche として、
`failure_body_ref` まで届いた retained bridge に

- `actual_invocation_protocol_ref`
- `consumer_host_binding_ref`
- `failure_wording_ref`

をどこまで additive に足してよいかを docs-first で比較し、
actual notebook runtime handoff actualization / emitted invocation receipt / runtime transcript family は still later に残す current threshold を固定する。

## 2. Scope and assumptions

- `proof_notebook` first bridge だけを扱う。
- current theorem-line later package の current first choice を additive に 3 step だけ進める。
- actual notebook runtime handoff actualization、actual emitted invocation receipt、runtime transcript family は固定しない。
- `proof_assistant_adapter` pressure は second practical candidate のまま維持する。

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
- `specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 4. Actions taken

1. `actual_invocation_protocol_ref`、`consumer_host_binding_ref`、`failure_wording_ref` をそれぞれ
   - current bridge を terminal cut にする案
   - additive に ref 1 個だけ足す案
   - next concrete actualization とまとめて足す案
   で比較した。
2. `specs/examples/160-current-l2-theorem-line-failure-ready-actual-invocation-protocol-threshold.md` を追加し、current first choice を **`actual_invocation_protocol_ref` のみ追加** に固定した。
3. `specs/examples/161-current-l2-theorem-line-invocation-ready-host-binding-threshold.md` を追加し、current first choice を **`consumer_host_binding_ref` のみ追加** に固定した。
4. `specs/examples/162-current-l2-theorem-line-binding-ready-failure-wording-threshold.md` を追加し、current first choice を **`failure_wording_ref` のみ追加** に固定した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を 162 状態へ更新した。
6. local docs validation と diff check を通した後、reviewer を 1 回だけ起動して findings を回収した。
7. reviewer findings を反映し、report / mirror の stale wording や evidence gap を補正した。

## 5. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/160-current-l2-theorem-line-failure-ready-actual-invocation-protocol-threshold.md`
- `specs/examples/161-current-l2-theorem-line-invocation-ready-host-binding-threshold.md`
- `specs/examples/162-current-l2-theorem-line-binding-ready-failure-wording-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0436-phase5-invocation-binding-wording-threshold-tranche.md`
- `docs/reports/0437-review-phase5-invocation-binding-wording-threshold-tranche.md`

## 6. Normative statement changed

- `specs/examples/160-current-l2-theorem-line-failure-ready-actual-invocation-protocol-threshold.md` を追加し、theorem-line retained bridge の current first choice として `actual_invocation_protocol_ref` までを足してよいという L2 docs-first threshold judgment を新たに固定した。
- `specs/examples/161-current-l2-theorem-line-invocation-ready-host-binding-threshold.md` を追加し、theorem-line retained bridge の current first choice として `consumer_host_binding_ref` までを足してよいという L2 docs-first threshold judgment を新たに固定した。
- `specs/examples/162-current-l2-theorem-line-binding-ready-failure-wording-threshold.md` を追加し、theorem-line retained bridge の current first choice として `failure_wording_ref` までを足してよいという L2 docs-first threshold judgment を新たに固定した。

## 7. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-10 03:18 JST
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 437 numbered report(s).
```

```text
$ git diff --check
[no output]
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
?? docs/reports/0436-phase5-invocation-binding-wording-threshold-tranche.md
?? docs/reports/0437-review-phase5-invocation-binding-wording-threshold-tranche.md
?? specs/examples/160-current-l2-theorem-line-failure-ready-actual-invocation-protocol-threshold.md
?? specs/examples/161-current-l2-theorem-line-invocation-ready-host-binding-threshold.md
?? specs/examples/162-current-l2-theorem-line-binding-ready-failure-wording-threshold.md
```

```text
$ reviewer wait (agent 019d7372-2374-77d1-b27e-2903de9e5419)
completed with findings
```

## 8. Evidence / outputs / test results

- failure-ready retained bridge の次段として、`actual_invocation_protocol_ref` だけを symbolic ref のまま narrow に吸収してよい。
- invocation-ready retained bridge の次段として、`consumer_host_binding_ref` だけを symbolic ref のまま narrow に吸収してよい。
- binding-ready retained bridge の次段として、`failure_wording_ref` だけを symbolic ref のまま narrow に吸収してよい。
- ただし actual notebook runtime handoff actualization / emitted invocation receipt / runtime transcript family はまだ theorem-line bridge に入れない方が自然である。
- したがって current next later reopen は **actual notebook runtime handoff actualization comparison** に移った。
- reviewer は report evidence placeholder、`progress.md` timestamp / work-log drift、`plan/11` の naming slip を指摘したため、この 3 点を補正した。

## 9. What changed in understanding

- theorem-line retained bridge は `failure_body_ref` の後も、actual invocation protocol / host binding / failure wording を 1 段ずつ symbolic ref family として narrow に切り分けられる。
- ただし wording の次段からは concrete runtime handoff actualization pressure が強くなり、docs-first retained bridge と actual runtime handoff line の境界を改めて比較する必要がある。

## 10. Open questions

- actual notebook runtime handoff actualization をどこまで theorem-line retained bridge に寄せてよいか。
- actual emitted invocation receipt / runtime transcript family を retained bridge に入れずに later reopen に保てるか。
- `proof_assistant_adapter` pressure を second practical candidate のまま維持できる条件は何か。

## 11. Suggested next prompt

`actual notebook runtime handoff actualization comparison` を docs-first で比較し、`failure_wording_ref` の次段として actual runtime handoff actualization / emitted invocation receipt / runtime transcript family をどこまで retained bridge に寄せてよいかを source-backed に整理してください。
