# 600. current-l2 typed-checker first executable slice actualization

## 位置づけ

- current compatibility-front-door adjacent floor。
- `mir-current-l2 check-source-sample`
  は archived Problem 1 bundle quickstart の続きではなく、
  active clean-near-end `.mir` sample set に対する
  repo-local checker-shaped inspection command として残してよい。
- final public verifier contract、final typed calculus、
  final public checker artifact を fixed する task ではない。

## この package で固定する current cut

1. `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample <clean-near-end-sample> --format pretty|json`
   は active clean-near-end sample family
   `typing / order-handoff / model-check / modal`
   に対して動き、
   `sample` / `family` / `source_path` / `static_verdict` /
   `entered_evaluation` / `terminal_outcome` / `reason_family` /
   `constraints_solved` / `constraints_failed`
   を repo-local summary として返してよい。
2. archived `samples/current-l2/*.txt` fixture mode は still present だが、
   `--host-plan <path>` か adjacent host plan を要する runtime-private compatibility path に留め、
   current sample-side entrypoint には置かない。
3. current docs / validation anchor は archived Problem 1 bundle quickstart ではなく、
   active clean-near-end `.mir` examplesに置く。

## current recommendation

- `check-source-sample` は clean-near-end accepted sample set を
  CLI-shaped current surface から inspect する repo-local command として扱ってよい。
- current cut は
  active clean-near-end runtime / static corpus floor を
  checker-shaped summary から読み直すところに留める。
- archived prototype / bundle / parser-companion quickstart と混ぜず、
  current active sample root を優先する。

## actualized evidence

- runtime command:
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/typing/01_authorized_declassification.mir --format json`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/typing/02_unauthorized_declassification_rejected.mir --format json`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/order-handoff/05_delegated_rng_service.mir --format json`
- tests:
  - `cargo test -p mir-runtime --test current_l2_operational_cli`

## stop line

- final typed source principal
- final typed calculus
- final public verifier contract
- final public checker artifact

## retained-later line

- stronger typed surface principal promotion
- final public theorem/model-check/verifier seam
- final public parser / checker / runtime API
