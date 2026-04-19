# 0800 — FAQ 009 current status and self-drive bound

## Objective

`faq_009.md` を追加し、

- 現状どこまで何が終わっているか
- 二大問題を完全に解決し、言語側の実装まで終わっていると読めるか
- 全体像の中で今どこにいるか
- そこへ持っていくために何がまだ必要か
- それらに答えればどこまで自走できるか

を、current docs / plan / report / validation evidence に即して整理する。

## Scope and assumptions

- `specs/` を規範正本、`plan/` を repository memory、`docs/reports/` を時系列証跡として扱う。
- `faq_009.md` は current explanation であり、新しい規範判断を作る文書ではない。
- current status の再説明を目的とし、final public parser/checker/runtime API、final public verifier contract、final public theorem/model-check contract、final public witness/provider contract は adoption しない。
- current snapshot と整合している限り、`progress.md` と `tasks.md` は今回書き換えない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `tasks.md`
- `faq_007.md`
- `faq_008.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `docs/reports/0799-actual-lean-prototype-widening-and-snapshot-sync.md`

## Actions taken

1. AGENTS の current-status 読書順に沿って、README / Documentation / progress / task axes / specs 00..03 / specs 09 を再読した。
2. current snapshot と normative / memory docs の整合を確認し、current completion level と remaining gate を棚卸しした。
3. representative validation を再実行し、runner / CLI / authored inventory / theorem actual Lean execution floor が維持されていることを確認した。
4. `faq_009.md` を新規作成し、`faq_008.md` 以後の genuine progress、done / not done、overall ladder、mixed gate / user-spec residual / evidence-dependent question を current reading に沿って整理した。
5. `specs/00-document-map.md`、`Documentation.md`、`plan/90-source-traceability.md` に `faq_009.md` の導線を追加した。
6. `progress.md` 更新不要、`tasks.md` 更新不要と判断した。
   - 理由: current snapshot の核心 reading は既に `2026-04-19 13:09 JST` 更新分と一致しており、今回の task は status explanation delta の追加が主で、repo status そのものの切り替えではないため。

## Evidence / outputs / test results

- resource / baseline
  - `df -h .`
  - `free -h`
  - `git status --short --branch`
  - result: worktree clean (`## main...origin/main`)
- representative validation
  - `source "$HOME/.elan/env" && cargo test -p mir-runtime --test current_l2_operational_cli --test current_l2_source_sample_runner --test current_l2_theorem_actual_lean_execution_prototype_widening`
  - result:
    - `current_l2_operational_cli`: `12 passed`
    - `current_l2_source_sample_runner`: `22 passed`
    - `current_l2_theorem_actual_lean_execution_prototype_widening`: `3 passed`
  - reading:
    - current CLI / runner floor is still intact
    - representative prototype theorem actual Lean execution floor is still intact
- representative static theorem actual execution
  - `source "$HOME/.elan/env" && cargo test -p mir-semantics --test current_l2_lean_theorem_stub_actual_probe`
  - result: `1 passed`
- authored corpus inventory
  - `python3 scripts/current_l2_source_sample_regression.py inventory`
  - result: authored sixteen present

## What changed in understanding

- current repo は docs-only theory comparison phase をすでに抜けており、two big problems の current first line / actual adoption / helper-local actualization / residual compression まで source-backed に進んでいる。
- theorem-side actual execution は `e5` only の読みではなく、representative theorem quartet `e5 / p06 / p07 / p08` actual Lean execution floor に達している。
- remaining work の中心は foundational theory construction ではなく、actual execution helper/CLI hardening、broader coverage、later mixed gate、true user-spec residual に移っている。
- ただし、final public language implementation complete と読むのは still overclaim であり、public seams と target decisions は残る。

## Open questions

- theorem actual execution floor を Python helper / CLI orchestration まで統一するか、それとも runtime regression を principal regression surface に保つか。
- representative theorem quartet の次に、どの theorem-relevant corpus を widening するか。
- final public theorem/model-check/order-handoff/shared-space seams のうち、どれを actual implementation pressure で next reopen するか。
- packaging / FFI / broader app target をいつどの acceptance criteria で reopen するか。

## Suggested next prompt

`faq_009.md` を current explanation delta として保ったまま、theorem actual execution helper/CLI hardening と broader theorem-relevant coverage を next package として進め、必要なら progress/tasks/plan/spec snapshot を current reopen line に再同期してください。
