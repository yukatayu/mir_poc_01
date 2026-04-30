# 606. current-l2 reserve integration entrypoint summary sync

## 位置づけ

- historical Phase 6 / Package 133 closeout memory。
- `specs/examples/470`、`476`、`477`、`478`、`571`、`605` の次段として、
  theorem-first external pilot / `auditable_authority_witness` / `delegated_rng_service` / model-check second-line reserve を
  once-through closeout summary 後の next reopen line として
  historical `reserve`
  helper へ再同期した package memory を記録する。
- final public theorem contract、final public witness/provider/artifact contract、concrete theorem/model-check production binding、installed-binary / packaging / FFI / engine adapter adoption、exhaustive shared-space catalog adoption を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py reserve`
   は 2026-04-22 clean-sample migration 前の historical helper-local summary entrypoint として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. historical `reserve` summary は
   - reserve package id
   - entry commands
   - anchor refs
   - next queue
   - stop line
   を 1 枚に圧縮してよい。
4. historical closeout queue memory では
   `134 parser-side residual closeout sync`
   `135 true user-spec residual freeze sync`
   `later mixed / user-spec residual`
   に進んでいた。
   current queue authority は `progress.md` / `tasks.md` に残す。

## current recommendation

- reserve integration lane は compare-floor を増やさず、once-through closeout summary の次に reopen する historical helper-local summary memory として保持してよい。
- current cut は reserve package を final public contract 群から切り離した reopen order の drift suppression に留める。
- final public theorem contract、final public witness/provider/artifact contract、concrete theorem/model-check production binding、installed-binary / packaging / FFI / engine adapter adoption、exhaustive shared-space catalog adoption には上げない。

## actualized evidence

- historical helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_once_through_closeout_summary_json_contains_next_packages scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_reserve_integration_summary_text_mentions_four_reserve_lanes scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_reserve_integration_summary_json_contains_package_ids_and_next_queue scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_main_reserve_command_uses_renderer`
- historical helper commands:
  - `python3 scripts/current_l2_guided_samples.py reserve`
  - `python3 scripts/current_l2_guided_samples.py reserve --format json`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final public theorem contract
- final public witness/provider/artifact contract
- concrete theorem/model-check production binding
- installed-binary / packaging / FFI / engine adapter adoption
- exhaustive shared-space catalog adoption

## retained-later line

- parser-side residual closeout sync
- true user-spec residual freeze sync
- later mixed / user-spec residual reopen
