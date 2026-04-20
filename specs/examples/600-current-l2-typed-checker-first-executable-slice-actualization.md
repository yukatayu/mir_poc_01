# 600. current-l2 typed-checker first executable slice actualization

## 位置づけ

- current Phase 6 / Package 127 closeout。
- `specs/examples/557` の finite-index first strong typing layer default と、
  `run-source-sample` helper-local typed checker preview 群の次段として、
  `mir-current-l2 check-source-sample`
  を actualize し、typed sample set に対する focused checker slice を repo-local command に下ろす。
- final public verifier contract、final typed calculus、final public checker artifact を fixed する task ではない。

## この package で固定する current cut

1. `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample <sample> --format pretty|json`
   は first strong typing sample set
   `p10 / p11 / p12 / p15 / p16`
   に対してだけ動き、次を focused summary として返してよい。
   - typed sample manifest
   - static gate verdict
   - terminal outcome
   - typed checker hint preview
   - actual checker payload family threshold
   - actual checker payload row detail / row body threshold
2. non-typed sample に対しては、
   `check-source-sample` は
   `first strong typing sample set only`
   の execution error で止めてよい。
3. Problem 1 sample bundle doc は
   `run-source-sample`
   に加えて
   `check-source-sample`
   を案内し、typed checker current first line を focused checker slice でも追ってよい。

## current recommendation

- first strong typing layer は helper-local preview のままにせず、
  typed sample set に限った focused checker command として repo-local actualization してよい。
- current cut は
  - checker-adjacent principal
  - finite decidable index fragment
  - IFC / capture / lifetime / simple cost
  を executable slice まで下ろすところに留める。
- `check-source-sample` は repo-local command であり、
  final public verifier contract、
  final typed source principal、
  final typed calculus、
  final public checker artifact
  には上げない。

## actualized evidence

- runtime command:
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt --format pretty`
- tests:
  - `cargo test -p mir-runtime --test current_l2_operational_cli`
  - `python3 -m unittest scripts.tests.test_problem_sample_bundles`

## stop line

- final typed source principal
- final typed calculus
- final public verifier contract
- final public checker artifact

## retained-later line

- stronger typed surface principal promotion
- final public theorem/model-check/verifier seam
- final public parser / checker / runtime API
