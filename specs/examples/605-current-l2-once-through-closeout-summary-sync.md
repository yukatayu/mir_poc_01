# 605. current-l2 once-through closeout summary sync

## 位置づけ

- current Phase 6 / Package 132 closeout。
- `specs/examples/596`、`597`、`598`、`599`、`603`、`604` の次段として、
  Package 127...131 executable loop 後の current first line / mixed-gate lane / true user-spec residual / next self-driven queue を
  `closeout`
  helper へ再同期する。
- final public parser / checker / runtime API、final public verifier contract、final public witness/provider/artifact contract、installed-binary / packaging / FFI / engine adapter adoption、exhaustive shared-space catalog adoption を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py closeout`
   は Package 127...131 executable loop 後の repo-local once-through reading を
   helper-local summary として示してよい。
2. `closeout` summary は
   - current first line
   - mixed-gate lanes
   - true user-spec residuals
   - next self-driven packages
   を 1 枚に圧縮してよい。
3. current live queue は
   `133 reserve integration entrypoint sync`
   `134 parser-side residual closeout sync`
   `135 true user-spec residual freeze sync`
   に進めてよい。

## current recommendation

- after-loop closeout sync は docs-only rewrite に留めず、helper command で current reading を短く出せる形にしてよい。
- current cut は repo-local near-end completion reading の drift suppression に留める。
- final public parser / checker / runtime API、final public verifier contract、final public witness/provider/artifact contract、installed-binary / packaging / FFI / engine adapter adoption、exhaustive shared-space catalog adoption には上げない。

## actualized evidence

- helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_once_through_closeout_summary_text_mentions_first_lines_and_residual_boundaries scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_once_through_closeout_summary_json_contains_next_packages scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_main_closeout_command_uses_renderer`
- helper commands:
  - `python3 scripts/current_l2_guided_samples.py closeout`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

## stop line

- final public parser / checker / runtime API
- final public verifier contract
- final public witness/provider/artifact contract
- installed-binary / packaging / FFI / engine adapter adoption
- exhaustive shared-space catalog adoption

## retained-later line

- reserve integration lane tightening
- parser-side residual closeout sync
- true user-spec residual freeze sync
