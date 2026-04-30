# 579 — current L2 parser-side request-head / clause-bundle inspector actualization

## 位置づけ

- current repo-local archived-compare floor。
- representative parser-companion sample `p06 / p07 / p08`
  は active sample root からは外れたが、
  `Stage3RequestHeadClauseBundle` parse result を
  archived compare sample に対して inspect する command surface は
  repo-local current command として残してよい。
- final grammar、
  final public parser / checker / runtime API、
  full `Program` lowering、
  final diagnostics / span-rich contract を fixed する task ではない。

## この package で固定する current cut

1. `cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- <archived-path> --format pretty|json`
   は、archived parser-companion sample set に対する
   repo-local compare-floor inspection command として使ってよい。
2. inspection target は
   `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/`
   配下の representative slice
   `p06 / p07 / p08`
   に留める。
3. archived parser-companion inspection は
   active clean-near-end sample root や
   current `scripts/current_l2_guided_samples.py` front door と混ぜない。

## current recommendation

- parser-side carrier inspection command は current repo-local command として残してよいが、
  inspection target は archived compare sample に限定する。
- inspection summary は parse result の readable cut であり、
  final public parser API や active parser-companion sample family を意味しない。

## actualized evidence

- current parser-side compare-floor command:
  - `cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt --format json`
  - `cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/p07-dice-late-join-visible-history.request.txt --format pretty`
- tests:
  - `cargo test -p mir-ast --test current_l2_request_head_clause_bundle_manifest`
  - `cargo test -p mir-ast --test current_l2_stage3_request_head_clause_bundle_spike`
- archived compare-floor docs:
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/README.md`

## stop line

- final public parser / checker / runtime API
- full `Program` lowering
- final diagnostics / span-rich contract
- final public verifier contract

## retained-later line

- parser-side representative mapping memory
- final parser grammar
