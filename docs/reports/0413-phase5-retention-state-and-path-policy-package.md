# 0413 — Phase 5 retention state and path policy package

- Date: 2026-04-10 00:16 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line の retention-state / path-policy package (`148...` / `149...`) と mirror 更新
- Decision levels touched: L1 / L2

## 1. Objective

Phase 5 theorem-line の later reopen package として、

- lifecycle-ready retained bridge に retention state をどこまで足すか
- retention-ready retained bridge に retained path policy ref をどこまで足すか
- actual emitted notebook artifact をどこまで後段に残すか

を docs-first に比較し、current first choice を固定する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `specs/examples/145-current-l2-theorem-line-retained-bridge-review-session-link-threshold.md`
- `specs/examples/146-current-l2-theorem-line-session-linked-retained-bridge-review-observation-threshold.md`
- `specs/examples/147-current-l2-theorem-line-observed-session-lifecycle-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0410-phase5-observed-session-lifecycle-package.md`
- `docs/reports/0411-review-phase5-observed-session-lifecycle-package.md`
- `docs/reports/0412-review-current-uncommitted-phase5-package.md`

## 3. Actions taken

1. `specs/examples/148-current-l2-theorem-line-lifecycle-ready-retention-state-threshold.md` を追加し、
   - lifecycle-ready retained bridge を terminal cut にする
   - `retention_state` だけを足す retention-ready retained bridge
   - retention state と path policy / emitted artifact をまとめて入れる
   を比較した。
2. current first choice を
   - lifecycle-ready retained bridge の次段では `retention_state` までは足してよい
   - actual retained path policy / emitted artifact pressure は second step
   に固定した。
3. `specs/examples/149-current-l2-theorem-line-retention-ready-path-policy-threshold.md` を追加し、
   - retention-ready retained bridge を terminal cut にする
   - `retained_path_policy_ref` だけを足す path-ready retained bridge
   - path policy と emitted artifact をまとめて入れる
   を比較した。
4. current first choice を
   - retention-ready retained bridge の次段では `retained_path_policy_ref` までは足してよい
   - actual emitted notebook artifact は second step
   に固定した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot に揃えた。
6. reviewer subagent を最後に 1 回だけ起動し、completion を長めに待つ closeout 方針を取る。

## 4. Files changed

- `specs/examples/148-current-l2-theorem-line-lifecycle-ready-retention-state-threshold.md`
- `specs/examples/149-current-l2-theorem-line-retention-ready-path-policy-threshold.md`
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

## 5. Commands run

```bash
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/validate_docs.py
git diff --check
git status --short --branch
```

## 6. Evidence / findings

- `specs/examples/148...` で、lifecycle-ready retained bridge の次段は path policy ではなく `retention_state` に留めるのが最小だと固定した。
- `specs/examples/149...` で、そのさらに次段は actual emitted artifact ではなく `retained_path_policy_ref` に留めるのが最小だと固定した。
- current theorem-line chain では、
  - `bridge_subject_ref + review_units + bridge_goal_text`
  - `+ comparison_basis_refs`
  - `+ bless_decision_state`
  - `+ review_note_refs`
  - `+ retained_notebook_ref`
  - `+ review_session_ref`
  - `+ reviewed_by_ref + reviewed_at_ref`
  - `+ review_session_state`
  - `+ retention_state`
  - `+ retained_path_policy_ref`
  の順で narrow に強くする ratchet が current first choice になった。
- actual emitted notebook artifact と `proof_assistant_adapter` specific schema は still later reopen に残る。
- local validation と review closeout は `0414` に記録する。

## 7. Changes in understanding

- review session progression の次段には、path policy を入れるより先に retention progression tag を一段挟むのが最も narrow である。
- retention progression の次段でも actual artifact をすぐ入れず、まず `retained_path_policy_ref` という symbolic policy ref に留めると theorem-line bridge の docs-only discipline を保ちやすい。
- actual emitted notebook artifact pressure は retention progression / path-policy anchor と別 reopen axis に分離できる。

## 8. Open questions

- actual emitted notebook artifact threshold をどこまで足すか
- actual theorem handoff emitter を later reopen に保てるか
- typed symbolic `evidence_refs` family を boundary-specific handoff artifact へ昇格させる concrete pressure を何とみなすか
- `proof_assistant_adapter` consumer pressure を second practical candidate のまま維持する条件がいつ崩れるか

## 9. Suggested next prompt

`Phase 5 の next later reopen candidate として、path-ready retained bridge の上に actual emitted notebook artifact をどこまで足すのが最小かを docs-first で比較してください。`
