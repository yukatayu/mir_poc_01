# Report 0434 — phase5 failure-body threshold package

- Date: 2026-04-10 02:58 JST
- Author / agent: Codex GPT-5
- Scope: Phase 5 theorem-line later reopen package for the failure-body threshold and its mirror updates
- Decision levels touched: L2 docs-first threshold judgment and repository-memory mirror maintenance

## 1. Objective

Phase 5 theorem-line later reopen の current next package として、
`transport_protocol_ref` まで届いた retained bridge に
`failure_body_ref` をどこまで足してよいかを docs-first で比較し、
actual runtime invocation protocol / host binding / failure wording は still later に残す current threshold を固定する。

## 2. Scope and assumptions

- `proof_notebook` first bridge だけを扱う。
- current theorem-line later package の current first choice を additive に 1 step だけ進める。
- actual runtime invocation protocol、consumer-host-specific binding、concrete failure wordingは固定しない。

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
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 4. Actions taken

1. `failure-body threshold` を
   - transport-ready retained bridge を terminal cut にする案
   - `failure_body_ref` だけを足す案
   - `failure_body_ref + actual_invocation_protocol_ref` をまとめて足す案
   の 3 案で比較した。
2. `specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md` を追加し、current first choice を **`failure_body_ref` のみ追加** に固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を 159 状態へ更新した。
4. local docs validation と diff check を通した後、reviewer を 1 回だけ起動して findings を回収した。
5. reviewer の findings に従い、report 0434 に exact evidence を補い、scoped mirror の stale line を補正した。

## 5. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0434-phase5-failure-body-threshold-package.md`
- `docs/reports/0435-review-phase5-failure-body-threshold-package.md`

## 6. Normative statement changed

- `specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md` を追加し、theorem-line retained bridge の current first choice として `failure_body_ref` までを足してよいという L2 docs-first threshold judgment を新たに固定した。

## 7. Commands run and exact outputs

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
?? docs/reports/0434-phase5-failure-body-threshold-package.md
?? docs/reports/0435-review-phase5-failure-body-threshold-package.md
?? specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md
```

```text
$ reviewer wait (agent 019d735d-9f69-7901-be45-e9af40fcdbda)
completed with findings
```

## 8. Evidence / outputs / test results

- transport-ready retained bridge の次段として、`failure_body_ref` だけを symbolic ref のまま narrow に吸収してよい。
- ただし actual runtime invocation protocol / host binding / failure wording はまだ theorem-line bridge に入れない方が自然である。
- したがって current next later reopen は **`actual-invocation-protocol threshold`** に移った。
- reviewer は `progress.md` summary / work log と `plan/17` の stale sequencing line、report 0434 の evidence gap を指摘したため、この 3 点を補正した。

## 9. What changed in understanding

- transport protocol と failure body は theorem-line retained bridge の symbolic line として分離して持てるが、actual runtime invocation protocol まで同時に上げるのは premature である。
- `proof_notebook` first bridge では failure body も symbolic anchor として切っておく方が、actual invocation protocol family を後段で narrow に reopen しやすい。

## 10. Open questions

- `actual_invocation_protocol_ref` をどこまで theorem-line retained bridge に寄せてよいか。
- consumer-host-specific binding と failure wording を still later reopen に保てるか。
- `proof_assistant_adapter` pressure を second practical candidate のまま維持できる条件は何か。

## 11. Suggested next prompt

`actual-invocation-protocol threshold` を docs-first で比較し、`failure_body_ref` の次段として `actual_invocation_protocol_ref` をどこまで retained bridge に寄せてよいかを source-backed に整理してください。
