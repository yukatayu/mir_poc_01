# Report 1020 — model-check active evidence refresh

- Date: 2026-04-30
- Author / agent: Codex
- Scope: active model-check chain `specs/examples/478/480/482/488/492/495` の evidence-row refresh。`467` と theorem-side docs 本体はこの package の外
- Decision levels touched: none; active docs authority と current runnable evidence surface の対応整理のみ

## 1. Objective

retired model-check helper/test names を参照していた active `specs/examples/478/480/482/488/492/495` を、current clean-near-end / compatibility front door / CLI-shaped surface へ寄せ直す。あわせて、historical `e5 / p05 / p06 / p07 / p08 / p09` labels を current runnable floor と混同しない wording に補正する。

## 2. Scope and assumptions

- scope は active model-check chain `478/480/482/488/492/495` に限定した。
- `specs/` の normative meaning や package recommendation 自体は変更しない。
- `467` の Problem 2 actual-package evidence refresh は、Sugoroku / network canary を含む別 package として残す。
- `current_l2_lean_sample_sync.py` は theorem-side adjacent evidence anchor としてだけ使い、1:1 theorem-discharge target とは扱わない。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `AGENTS.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/478-current-l2-model-check-second-line-concretization.md`
- `specs/examples/480-current-l2-model-check-property-language-and-tool-seam-probe.md`
- `specs/examples/482-current-l2-model-check-property-tool-threshold-default.md`
- `specs/examples/488-current-l2-model-check-row-local-property-and-checker-boundary-actual-adoption.md`
- `specs/examples/492-current-l2-model-check-public-checker-artifact-preview-and-verifier-handoff-reserve-actualization.md`
- `specs/examples/495-current-l2-model-check-tool-brand-and-verifier-handoff-coupled-later-gate.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/clean_near_end_samples.py`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `samples/README.md`
- `scripts/README.md`
- `samples_progress.md`

## 4. Actions taken

1. active model-check docs に残っていた retired helper/test names を抽出した。
2. current runnable front door が `clean_near_end_samples.py run model-check`、`current_l2_guided_samples.py smoke-all`、`mir-current-l2 check-source-sample` であることを確認した。
3. `478/480/482/488/492/495` の evidence rows を current surface に置換した。
4. reviewer 指摘で、`e5 / p05 / p06 / p07 / p08 / p09` を current evidence と誤読しうる prose を `historical package reading` に下げた。
5. reviewer 指摘で、`clean_near_end_samples.py closeout` rows を canonical inventory / current emitted rows までに限定し、probe / threshold / reserve judgments 自体は helper-local / doc-level interpretation に戻した。
6. `progress.md` と `tasks.md` を narrow package closeout に同期した。

## 5. Files changed

- `progress.md`
- `tasks.md`
- `specs/examples/478-current-l2-model-check-second-line-concretization.md`
- `specs/examples/480-current-l2-model-check-property-language-and-tool-seam-probe.md`
- `specs/examples/482-current-l2-model-check-property-tool-threshold-default.md`
- `specs/examples/488-current-l2-model-check-row-local-property-and-checker-boundary-actual-adoption.md`
- `specs/examples/492-current-l2-model-check-public-checker-artifact-preview-and-verifier-handoff-reserve-actualization.md`
- `specs/examples/495-current-l2-model-check-tool-brand-and-verifier-handoff-coupled-later-gate.md`

## 6. Commands run and exact outputs

- `git status --short`
  - package start時点では working tree clean
- `git branch --show-current`
  - `main`
- `git log -1 --oneline`
  - `09a23b4 Refresh order-handoff witness-provider active evidence`
- `python3 scripts/clean_near_end_samples.py run model-check --format json`
  - pass; returned current model-check family `01_peterson_sc_pass / 02_peterson_relaxed_counterexample / 03_broken_mutex_counterexample`
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - pass; compatibility front door から active clean-near-end corpus 全体が runnable
- `cargo run -q -p mir-runtime --bin mir-current-l2 -- check-source-sample samples/clean-near-end/model-check/01_peterson_sc_pass.mir --format json`
  - pass; clean model-check sample を CLI-shaped current surface から読める
- `python3 scripts/current_l2_lean_sample_sync.py`
  - pass; `/home/yukatayu/dev/mir_poc_01/samples/lean/manifest.json` を出力
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - pass; adjacent provider-boundary evidence を current runtime surface から読める
- `python3 scripts/check_source_hierarchy.py`
  - pass; `all required paths present`
- `python3 scripts/validate_docs.py`
  - pass; `Documentation scaffold looks complete.`
- `git diff --check`
  - pass; no diff-format errors

## 7. Evidence / findings

- current runnable model-check front door is the clean-near-end family, not the old `current_l2_model_check_*` helper/test labels.
- `current_l2_source_sample_runner` と `current_l2_operational_cli` は live surface ではあるが、old model-check manifest を直接証明する named proof surface ではなくなっていた。
- `clean_near_end_samples.py closeout` は canonical inventory / current emitted rows を返すが、doc-local probe / threshold / reserve / verifier-handoff candidate refs を directly emit しない。
- reviewer `Plato` first pass caught:
  - current-evidence と historical `e5/p...` anchors の混同
  - `closeout` が doc-local judgments を直接出力するように読める wording
- reviewer second pass reported no findings in scope.

## 8. What changed in understanding

- model-check chain の current docs refresh は、single command replacement では足りず、clean-near-end family / compatibility wrapper / CLI-shaped check / theorem-side adjacent sync の役割分解が必要だった。
- `historical package reading` を残すこと自体は許容されるが、current runnable floor と明確に分離しないと active docs authority が stale helper/test names を current evidence と誤読させる。
- `closeout` の価値は canonical inventory anchor にあり、doc-local helper refs の direct emission claim ではない。

## 9. Open questions

- `467` の actual package evidence refresh を次 package でどう切るか。Sugoroku current p07 replacement と network canary current p08 partial replacement を同じ docs-only package にまとめてよいか。
- theorem-side docs (`474/479/481/485/486/487/491/494/508`) でも同じ helper/test-name drift refresh を続けるべきか。

## 10. Suggested next prompt

`467` を中心に、Problem 2 actual-package evidence refresh を別 package として続け、Sugoroku / network canary / compatibility-front-door の current surface へ寄せつつ、historical `p07/p08` labels を current runnable floor と混同しない wording に補正する。

## 11. `plan/` update

更新不要。long-lived repository memory や subsystem boundary ではなく、active docs authority の evidence refresh のみを行った。

## 12. `progress.md` update

更新した。model-check active evidence refresh と validation anchor を recent log に追記した。

## 13. `tasks.md` update

更新した。current task-level status に narrow model-check package closeout を反映した。

## 14. `samples_progress.md` update

更新不要。sample dashboard の runnable root、progress%、blocker は変えていない。

## 15. Skipped validations and reasons

- full validation floor は未実行。
  - reason: docs-only package であり、current runnable model-check front door と docs floor に紐づく focused validation を優先した。

## 16. Commit / push status

report 作成時点では未実施。package closeout で同じ変更群とともに commit / push する。

## 17. Sub-agent session close status

- `Cicero` (`019dddb5-d962-7660-a4de-93a7264b7ba9`): closed after mapping handoff was incorporated
- `Plato` (`019dddbc-664c-72e1-83b7-44332a8b4739`): closed after second-pass review returned no findings
