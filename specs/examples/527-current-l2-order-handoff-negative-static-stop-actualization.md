# 527 — current L2 order-handoff negative static-stop actualization

## 位置づけ

- historical Package 58 negative-corpus tightening memory。
- `p13-dice-late-join-missing-publication-witness` と
  `p14-dice-late-join-handoff-before-publication`
  は、2026-04-22 clean-sample migration 前の
  archived late-join visibility negative compare anchors として保ってよい。
- current active sample-side compare floor は
  `samples/clean-near-end/order-handoff/02_missing_witness_rejected.mir`
  と
  `samples/clean-near-end/order-handoff/03_handoff_before_publication_rejected.mir`
  に置く。
- final parser grammar、final source-surface handoff wording、
  final emitted-handoff contract、final public witness/provider/artifact contract、
  final shared-space exhaustive catalog を fixed する task ではない。

## current recommendation

- archived `p13 / p14` は
  helper-local static-stop historical compare anchors として読む。
- current execution / inspection front door は
  clean-near-end order-handoff `02 / 03`
  に置く。
- negative pair visibility は current helper-local reading に残すが、
  archived prototype path は active surface に戻さない。

## actualized negative-pair memory

| anchor | historical role | current compare floor |
|---|---|---|
| `p13-dice-late-join-missing-publication-witness` | missing publication witness prototype | `02_missing_witness_rejected` |
| `p14-dice-late-join-handoff-before-publication` | handoff-before-publish prototype | `03_handoff_before_publication_rejected` |

## actualized evidence

- runner side helper-local static gate
  - `crates/mir-runtime/src/current_l2.rs`
- regression
  - `crates/mir-runtime/tests/current_l2_order_handoff_negative_static_stop.rs`
- archived compare anchors
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-order-handoff/p13-dice-late-join-missing-publication-witness.txt`
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-order-handoff/p14-dice-late-join-handoff-before-publication.txt`
- current clean-near-end compare floor
  - `samples/clean-near-end/order-handoff/02_missing_witness_rejected.mir`
  - `samples/clean-near-end/order-handoff/03_handoff_before_publication_rejected.mir`
- prior positive/default floor
  - `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md`
  - `specs/examples/471-current-l2-authoritative-room-vertical-slice-emitted-artifact-ratchet.md`
  - `specs/examples/526-current-l2-order-handoff-helper-cli-surface-preview-actualization.md`

## stop line

- archived compare anchor の active path 復帰
- final parser grammar
- final source wording
- final emitted-artifact / emitted-handoff schema
- final public witness/provider/artifact contract
- stronger fairness / replay operational profile
