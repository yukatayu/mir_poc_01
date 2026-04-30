# Report 1023 — Problem 2 actual-package evidence refresh

- Date: 2026-04-30
- Author / agent: Codex
- Scope: active `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md` の evidence refresh。supplemental reviewer traces `1021` / `1022` を同梱
- Decision levels touched: none; active docs authority の evidence surface と historical/current boundary の補正のみ

## 1. Objective

historical `p07` / `p08` reading と old current-L2 runner/CLI labels を、current Sugoroku / network canary / compatibility front door / CLI-shaped clean-sample surface に寄せ直す。あわせて、`p08` が single current sample ではなく partial replacement であること、`NET-03` が stale reconnect / membership-epoch guard canary であることを明示する。

## 2. Scope and assumptions

- scope は `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md` のみ。
- `p07` / `p08` は historical package-reading anchors として残し、current runnable target にはしない。
- current theorem/model-check line は adjacent evidence として触れるが、`467` で theorem/model-check docs 本体を更新しない。
- reviewer-generated reports `1021` / `1022` は this package の supplemental review trace として扱う。

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
- `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md`
- `scripts/current_l2_guided_samples.py`
- `scripts/network_transport_samples.py`
- `scripts/sugoroku_world_samples.py`
- `scripts/clean_near_end_samples.py`
- `scripts/README.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `samples/clean-near-end/network-transport/README.md`
- `samples/README.md`
- `samples_progress.md`
- `crates/mir-runtime/src/current_l2_cli.rs`

## 4. Actions taken

1. `467` の active evidence rows と `p07` / `p08` integration note を読み、historical/current 混同箇所を特定した。
2. current front door candidates を実行し、Sugoroku handoff core / late-join visibility、network stale-reconnect canary、compatibility front door、CLI-shaped clean sample、model-check adjacent floor、Lean sync anchor を確認した。
3. `467` の evidence rows を current command surface へ置換し、old `current_l2_source_sample_runner` / `current_l2_operational_cli` labels を current evidence から外した。
4. `p07` / `p08` integration note を historical reading と current replacement surfaces の split へ書き換えた。
5. reviewer 指摘で `NET-03` の wording を stale reconnect / membership-epoch guard canary に narrowing した。
6. `progress.md` と `tasks.md` を package closeout に同期した。

## 5. Files changed

- `progress.md`
- `tasks.md`
- `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md`
- `docs/reports/1021-problem2-actual-package-evidence-refresh-review.md`
- `docs/reports/1022-problem2-actual-package-evidence-refresh-rereview.md`

## 6. Commands run and exact outputs

- `git status --short`
  - package start時点では working tree clean
- `git branch --show-current`
  - `main`
- `git log -1 --oneline`
  - `ce9afe7 Refresh model-check active evidence`
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json`
  - pass; `terminal_outcome = success`
- `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership --format json`
  - pass; `published_history_visible = true`
- `python3 scripts/network_transport_samples.py run NET-03 --debug reconnect --format json`
  - pass; `static_or_runtime_verdict = reject`, `reason_family = stale_membership_epoch`
- `python3 scripts/network_transport_samples.py check-all --format json`
  - pass; `NET-02..NET-05` all passed
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - pass; compatibility front door から active clean-near-end corpus 全体が runnable
- `cargo run -q -p mir-runtime --bin mir-current-l2 -- check-source-sample samples/clean-near-end/order-handoff/01_authorized_roll_publish_handoff.mir --format json`
  - pass; clean order-handoff sample を CLI-shaped current surface から読める
- `python3 scripts/clean_near_end_samples.py run model-check --format json`
  - pass; current model-check family `01_peterson_sc_pass / 02_peterson_relaxed_counterexample / 03_broken_mutex_counterexample`
- `python3 scripts/current_l2_lean_sample_sync.py`
  - pass; `/home/yukatayu/dev/mir_poc_01/samples/lean/manifest.json` を出力
- `python3 scripts/check_source_hierarchy.py`
  - pass; `all required paths present`
- `python3 scripts/validate_docs.py`
  - pass; `Documentation scaffold looks complete.`
- `git diff --check`
  - pass; no diff-format errors

## 7. Evidence / findings

- historical `p07` is best read through current Sugoroku handoff core and late-join history visibility slices, not via a single old package target.
- historical `p08` no longer has a single current active sample; the safe current reading is split across `NET-03` plus the network canary family check.
- `current_l2_source_sample_runner` and `current_l2_operational_cli` are still live surfaces, but not direct proof surfaces for historical `p07/p08`.
- reviewer supplemental trace `1021` caught the only meaningful overclaim:
  - `NET-03` was initially described too broadly as replay/reconnect coverage
- reviewer supplemental trace `1022` confirmed the narrowed wording removed that overclaim.

## 8. What changed in understanding

- the safe current replacement for `467` is a decomposition across multiple active surfaces rather than a single renamed runner.
- `NET-03` is narrower than the old `p08` story and should stay framed as a stale reconnect / membership-epoch guard canary.
- the current package can still preserve `p07/p08` as historical archive anchors without pretending those archive names are active runnable targets.

## 9. Open questions

- should nearby Problem 2 docs beyond `467` receive the same historical-to-current split treatment, especially where archived `p07/p08` language still appears outside active evidence tables?
- should the repo keep committing reviewer-generated supplemental traces like `1021` / `1022`, or should future packages fold those findings only into the main closeout report?

## 10. Suggested next prompt

nearby active Problem 2 docs を scan し、archived `p07/p08` language が current runnable target のように読める箇所を docs-only で冷やす。必要なら Sugoroku / network canary / compatibility front door への current mapping を mirror する。

## 11. `plan/` update

更新不要。repository memory や subsystem boundary の変更ではなく、active docs authority の evidence refresh のみを行った。

## 12. `progress.md` update

更新した。`467` package closeout と validation anchor を recent log に追記した。

## 13. `tasks.md` update

更新した。current task-level status に `467` closeout を反映した。

## 14. `samples_progress.md` update

更新不要。sample dashboard の root、progress%、blocker は変えていない。

## 15. Skipped validations and reasons

- full validation floor は未実行。
  - reason: docs-only package であり、`467` に directly 対応する Sugoroku / network / compatibility-front-door / model-check adjacent floor と docs floor の focused validation を優先した。

## 16. Commit / push status

report 作成時点では未実施。package closeout で同じ変更群とともに commit / push する。

## 17. Sub-agent session close status

- `Schrodinger` (`019dddc9-0956-7a51-8863-980f59edece0`): closed after re-review returned no findings
