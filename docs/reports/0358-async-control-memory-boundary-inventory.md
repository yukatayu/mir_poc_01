# Report 0358 — async control / memory-model boundary inventory

- Date: 2026-04-09 12:29 JST
- Author / agent: Codex
- Scope: `atomic_cut` の位置づけと、higher-level async control / memory-model boundary を current plan / task snapshot に追加する
- Decision levels touched: non-normative planning and repository memory wording only

## 1. Objective

user からの「`atomic_cut` だけで並列制御を頑張るのか」「tree-like room view や Elm 的 event flow を活かした、より分かりやすく理論的にきれいな async-control theory を研究できないか」という問題提起を受けて、current repo の計画に次を明示する。

- current core で source-backed に固定されているのは `atomic_cut` の local cut までであること
- higher-level async control は別 line として研究する余地があること
- C++ 的 low-level memory-order family を current immediate candidate にしないこと
- この論点が Phase 4 / 5 の self-driven research line に入ること

## 2. Scope and assumptions

- 規範判断の正本は引き続き `specs/` であり、今回の変更は `plan/` / `tasks.md` / `progress.md` の inventory / blocker / phase snapshot を更新するものに留める。
- `atomic_cut` 自体の意味論や current L2 runtime semantics は変更しない。
- public syntax や actual scheduler API の finalization は行わない。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`

## 4. Actions taken

1. `tasks.md` の current task map を更新し、Task C を `static analysis / type / theorem prover / async-control boundary` inventory として拡張した。
2. `tasks.md` に blocker 6「非同期制御 / memory-model boundary」を追加し、選択肢と current recommendation を明文化した。
3. `plan/12-open-problems-and-risks.md` に risk register row と補足 section を追加し、`atomic_cut` の local cut と higher-level ordering family の境界を明示した。
4. `plan/13-heavy-future-workstreams.md` に workstream 6「非同期制御 / scheduler / memory-model boundary」を追加した。
5. `plan/16-shared-space-membership-and-example-boundary.md` に、tree-like room view 上の leaf-to-root event bubbling を source-of-truth ではなく derived execution / debug / explanation view として扱う current reading を追記した。
6. `plan/90-source-traceability.md` に provenance addendum を追加した。
7. `progress.md` を更新し、current summary / bottleneck / 作業ログに async-control boundary inventory を反映した。

## 5. Files changed

- `tasks.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0358-async-control-memory-boundary-inventory.md`

## 6. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M JST'
2026-04-09 12:29 JST

$ python3 scripts/new_report.py --slug async-control-memory-boundary-inventory
/home/yukatayu/dev/mir_poc_01/docs/reports/0358-async-control-memory-boundary-inventory.md
```

検証コマンドは task close 前に別途実行する。

## 7. Evidence / outputs / test results

- current repo の source-backed line では、`atomic_cut` は Mir-0 / current L2 の place-local finalizing cut であり、global ordering / fairness / scheduler semantics までは fixed していない。
- user の問題提起は current line と衝突せず、むしろ Phase 4 / 5 の inventory line に不足していた open problem を明示するものだった。
- current recommendation は「`atomic_cut` だけで全非同期制御を背負わせる」でも「low-level memory-order を直ちに言語コアへ入れる」でもなく、higher-level async-control family を docs-first に比較する line である。
- `plan/ 更新済み`
- `tasks.md 更新済み`
- `progress.md 更新済み`

## 8. What changed in understanding

- shared-space / consistency / fairness / proof boundary を考えると、async control は独立した inventory line として明示した方が current roadmap が読みやすい。
- tree-like room view と leaf-to-root event bubbling は、source-of-truth carrier ではなく derived execution / debug / explanation view として扱う方が current repo の architecture separation と整合する。

## 9. Open questions

- higher-level async-control family を consistency mode catalog とどう直交化するか。
- event-tree execution view を theorem prover / model checker に送るとき、どの relation を core とし、どこからを derived view とみなすか。
- low-level memory-order 相当を将来まったく入れないのか、lowering target や external verifier vocabulary としてだけ残すのか。

## 10. Suggested next prompt

Phase 4 / 5 の next self-driven step として、authoritative room の `authority-serial transition` を leaf-to-root event bubbling な derived execution view とどう対応付けるかを、read-mostly / fan-out / delegated capability の例つきで narrow に比較してください。
