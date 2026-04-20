# 606. current-l2 reserve integration entrypoint summary sync

## 位置づけ

- current Phase 6 / Package 133 closeout。
- `specs/examples/470`、`476`、`477`、`478`、`571`、`605` の次段として、
  theorem-first external pilot / `auditable_authority_witness` / `delegated_rng_service` / model-check second-line reserve を
  once-through closeout summary 後の next reopen line として
  `reserve`
  helper へ再同期する。
- final public theorem contract、final public witness/provider/artifact contract、concrete theorem/model-check production binding、installed-binary / packaging / FFI / engine adapter adoption、exhaustive shared-space catalog adoption を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py reserve`
   は theorem-first external pilot / `auditable_authority_witness` / `delegated_rng_service` / model-check second-line reserve を
   current reopen order と stop line 付きの helper-local summary として示してよい。
2. `reserve` summary は
   - reserve package id
   - entry commands
   - anchor refs
   - next queue
   - stop line
   を 1 枚に圧縮してよい。
3. current live queue は
   `134 parser-side residual closeout sync`
   `135 true user-spec residual freeze sync`
   `later mixed / user-spec residual`
   に進めてよい。

## current recommendation

- reserve integration lane は compare-floor を増やさず、once-through closeout summary の次に reopen する narrow helper-local summary として actualize してよい。
- current cut は reserve package を final public contract 群から切り離した reopen order の drift suppression に留める。
- final public theorem contract、final public witness/provider/artifact contract、concrete theorem/model-check production binding、installed-binary / packaging / FFI / engine adapter adoption、exhaustive shared-space catalog adoption には上げない。

## actualized evidence

- helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_once_through_closeout_summary_json_contains_next_packages scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_reserve_integration_summary_text_mentions_four_reserve_lanes scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_reserve_integration_summary_json_contains_package_ids_and_next_queue scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_main_reserve_command_uses_renderer`
- helper commands:
  - `python3 scripts/current_l2_guided_samples.py reserve`
  - `python3 scripts/current_l2_guided_samples.py reserve --format json`
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
