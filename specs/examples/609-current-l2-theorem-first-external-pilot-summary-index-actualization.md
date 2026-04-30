# 609. current-l2 theorem-first external pilot summary index actualization

## 位置づけ

- historical Phase 6 / reserve package closeout memory。
- `specs/examples/470`、`575`、`601`、`603`、`606`、`608` の次段として、
  theorem-first external pilot を
  historical `emit-theorem problem1`
  output dir 内 summary index まで再同期した package memory を記録する。
- final public theorem contract、proof object public schema、
  concrete theorem prover production binding、final public verifier contract を fixed する task ではない。

## この package で固定する current cut

1. `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
   は 2026-04-22 clean-sample migration 前の historical summary-index entrypoint として扱い、
   current active compatibility front door には戻さない。
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く。
3. historical emitted summary index は
   - current reading
   - sample rows
   - stop line
   - output dir
   を repo-local notebook-first pilot index として圧縮してよい。
4. theorem-first external pilot の historical closeout queue memory では、
   next reopen line を
   `auditable_authority_witness`
   `delegated_rng_service`
   model-check second-line
   に置いていた。current queue authority は `progress.md` / `tasks.md` に残す。

## current recommendation

- theorem-first external pilot は new public theorem contract を足さず、
  historical emitted artifact loop output dir に human-readable / machine-readable index を置いていた current cut memory として保持してよい。
- current cut は notebook-first theorem pilot の repo-local external-consumer 入口を整えるところに留め、
  final public theorem contract、proof object public schema、concrete theorem prover brand には上げない。
- representative sample は `p06`、support pair は `p07 / p08` のまま保つ。

## actualized evidence

- historical helper tests:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_problem1_theorem_emit_text_mentions_output_dir_command_and_samples scripts.tests.test_current_l2_guided_samples.CurrentL2GuidedSamplesTests.test_problem1_theorem_emit_manifest_writes_pilot_index_files scripts.tests.test_problem_sample_bundles.ProblemSampleBundleDocsTests.test_problem1_bundle_doc_mentions_representative_paths_and_commands`
- historical helper commands:
  - `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
  - `python3 scripts/current_l2_guided_samples.py emit-theorem problem1 --format json`
  - `cat target/current-l2-guided/problem1-theorem-pilot/pilot-summary.md`
  - `cat target/current-l2-guided/problem1-theorem-pilot/pilot-summary.json`
- current compatibility commands:
  - `python3 scripts/current_l2_guided_samples.py list`
  - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - `python3 scripts/current_l2_guided_samples.py closeout --format json`

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
