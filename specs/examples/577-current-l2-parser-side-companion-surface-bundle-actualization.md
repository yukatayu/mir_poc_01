# 577 — current L2 parser-side companion surface bundle actualization

## 位置づけ

- historical Phase 6 / Package 103 closeout memory。
- representative parser-companion sample set
  `p06 / p07 / p08`
  を、
  2026-04-22 clean-sample migration 前の
  archived parser-companion bundle memory として記録する。
- final grammar、
  final public parser / checker / runtime API、
  final public theorem contract、
  final public witness/provider/artifact contract を fixed する task ではない。

## この package で固定する current cut

1. parser-companion sample set は
   `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/`
   の archived compare floor として残してよい。
2. `p06 / p07 / p08`
   の parser-companion sample は
   `Stage3RequestHeadClauseBundle` compare floor の historical sample set として読み、
   current active sample root には戻さない。
3. current repo の representative active sample root は
   `samples/clean-near-end/`
   `samples/current-l2/`
   `samples/lean/`
   に置く。

## current recommendation

- parser-companion sample set は、
  representative theorem-first / authoritative-room slice を
  parser-side carrier へ戻していた historical compare floor として保ってよい。
- archived parser-companion sample を current active sample root と混ぜず、
  helper-local / non-production reader aid だったことを明示する。

## actualized evidence

- historical docs:
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/README.md`
- archived compare samples:
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt`
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/p07-dice-late-join-visible-history.request.txt`
  - `samples/old/2026-04-22-pre-clean-near-end/prototype/current-l2-parser-companion/p08-dice-stale-reconnect-refresh.request.txt`
- parser-side compare floor:
  - `cargo test -p mir-ast --test current_l2_request_head_clause_bundle_manifest`
  - `cargo test -p mir-ast --test current_l2_stage3_request_head_clause_bundle_spike`

## stop line

- final grammar
- final public parser / checker / runtime API
- final public theorem contract
- final public witness/provider/artifact contract

## retained-later line

- parser-side archived compare-floor inspection
- parser-side mapping memory
