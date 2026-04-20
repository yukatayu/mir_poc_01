# 609. current-l2 theorem-first external pilot summary index actualization

## 位置づけ

- current Phase 6 / reserve package closeout。
- `specs/examples/470`、`575`、`601`、`603`、`606`、`608` の次段として、
  theorem-first external pilot を
  `emit-theorem problem1`
  の output dir 内 summary index まで再同期する。
- final public theorem contract、proof object public schema、
  concrete theorem prover production binding、final public verifier contract を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
   は representative theorem line `p06 / p07 / p08` の emitted bundle JSON に加えて、
   `pilot-summary.md` / `pilot-summary.json`
   を output dir に materialize してよい。
2. emitted summary index は
   - current reading
   - sample rows
   - stop line
   - output dir
   を repo-local notebook-first pilot index として圧縮してよい。
3. theorem-first external pilot は reserve package として close してよく、
   next reopen line は
   `auditable_authority_witness`
   `delegated_rng_service`
   model-check second-line
   に移してよい。

## current recommendation

- theorem-first external pilot は new public theorem contract を足さず、
  emitted artifact loop の output dir に human-readable / machine-readable index を置く current cut まで actualize してよい。
- current cut は notebook-first theorem pilot の repo-local external-consumer 入口を整えるところに留め、
  final public theorem contract、proof object public schema、concrete theorem prover brand には上げない。
- representative sample は `p06`、support pair は `p07 / p08` のまま保つ。

## actualized evidence

- helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_problem1_theorem_emit_text_mentions_output_dir_command_and_samples scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_problem1_theorem_emit_manifest_writes_pilot_index_files scripts.tests.test_problem_sample_bundles.ProblemSampleBundleDocsTests.test_problem1_bundle_doc_mentions_representative_paths_and_commands`
- helper commands:
  - `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
  - `python3 scripts/current_l2_guided_samples.py emit-theorem problem1 --format json`
  - `cat target/current-l2-guided/problem1-theorem-pilot/pilot-summary.md`
  - `cat target/current-l2-guided/problem1-theorem-pilot/pilot-summary.json`

## stop line

- final public theorem contract
- proof object public schema
- concrete theorem prover production binding
- final public verifier contract

## retained-later line

- `auditable_authority_witness`
- `delegated_rng_service`
- model-check second-line
- later mixed gate reopen
