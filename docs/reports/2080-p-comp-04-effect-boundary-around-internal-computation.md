# Report 2080 — P-COMP-04 effect boundary around internal computation

- Date: 2026-05-21 21:45:16 +0900
- Author / agent: Codex
- Scope: `P-COMP-04` direct host read/write boundary rows, schema/runtime admission checks, helper/sample actualization, snapshot-doc sync
- Decision levels touched: `L1` existing computational-core direction, `L2` executable sample/helper realization

## Objective

Close `P-COMP-04` by proving the bounded pure/effect split around internal computation without overclaim:

- one accepted direct Product Alpha-1 host read -> Mir transform -> host write row
- one rejected undeclared host effect row
- one rejected undeclared failure-row row
- one rejected missing capability row

while preserving the existing Product Alpha-1 operational floor and keeping legacy adapter-owned `typed_host_io.add_one` as host-boundary evidence only.

## Scope and assumptions

- Keep `samples/product-alpha1/computational/add-one-pure-mir/` and the `P-COMP-03` helper rows intact.
- Realize `P-COMP-04` through real Product Alpha-1 package roots under `samples/product-alpha1/computational/host-io-internal-transform/`.
- Treat `required_capabilities` and `failure_tag` as bounded checker-admission boundary declarations for current computational product rows; do not claim broad effectful runtime semantics, final grammar, final public ABI/SDK, PoseGraph runtime evidence, or backend realization.

## Start state / dirty state

- Start point was `main` after `P-COMP-03` (`03354c43`) had already been committed and pushed.
- Work began from the local `P-COMP-04` in-progress tree; no unrelated user changes were reverted.
- Temporary `.mirrorea-alpha/` runtime output was generated during validation and removed before closeout.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/28-mir-computational-core.md`
- `specs/32-autonomous-execution-and-completion-contract.md`
- `plan/00-index.md`
- `plan/53-mir-computational-core-roadmap.md`
- `plan/57-autonomous-computational-core-master-plan.md`
- `docs/hands_on/mir_computational_core_01.md`
- `docs/research_abstract/mir_computational_core_01.md`
- `sub-agent-pro/mirrorea_mir_computational_core_handoff.md`

## Actions taken

- Added `required_capabilities` and `failure_tag` to current Product Alpha-1 computational runtime input handling.
- Tightened computational package validation so current direct computational rows now require:
  one non-empty `runtime_input.mir_compute.required_capabilities`,
  one `runtime_input.mir_compute.failure_tag`,
  and exactly one active computational contract covering both `runtime_input.host_input.effect_ref` and `runtime_input.host_output.effect_ref`.
- Bound `failure_tag` to that active computational contract instead of accepting it from any unrelated contract.
- Updated the direct `add-one-pure-mir` package and its test fixtures to carry the same explicit computational boundary fields.
- Materialized `samples/product-alpha1/computational/host-io-internal-transform/` into:
  `positive/`,
  `negative-undeclared-effect/`,
  `negative-undeclared-failure/`,
  `negative-missing-capability/`.
- Replaced the old single planned-only `comp-04` row with four executable rows in `matrix.json`.
- Extended `scripts/mir_computational_samples.py` to support:
  direct `run-local`,
  direct `check`,
  accepted/runtime-rejection/check-rejection classification,
  diagnostic-code matching,
  and row-specific request/output summaries.
- Extended Python and Rust tests to cover:
  positive `P-COMP-04`,
  undeclared effect,
  undeclared failure,
  missing capability,
  missing computational boundary fields,
  and failure-tag-on-unrelated-contract bypass rejection.
- Synchronized snapshot docs, sample catalog docs, hands-on docs, research summaries, and roadmap/index docs to reflect `P-COMP-04` closeout and the next reopen point `P-POSE-02`.

## Files changed

- Rust schema/runtime tests:
  `crates/mir-ast/src/product_alpha1.rs`
  `crates/mir-ast/tests/product_alpha1_package_schema.rs`
  `crates/mir-runtime/tests/product_alpha1_session.rs`
- Computational sample/helper:
  `samples/product-alpha1/computational/add-one-pure-mir/package.mir.json`
  `samples/product-alpha1/computational/matrix.json`
  `samples/product-alpha1/computational/README.md`
  `samples/product-alpha1/computational/host-io-internal-transform/README.md`
  `samples/product-alpha1/computational/host-io-internal-transform/positive/**`
  `samples/product-alpha1/computational/host-io-internal-transform/negative-undeclared-effect/**`
  `samples/product-alpha1/computational/host-io-internal-transform/negative-undeclared-failure/**`
  `samples/product-alpha1/computational/host-io-internal-transform/negative-missing-capability/**`
  `scripts/mir_computational_samples.py`
  `scripts/tests/test_mir_computational_samples.py`
- Snapshot/docs/indexes:
  `README.md`
  `Documentation.md`
  `progress.md`
  `tasks.md`
  `samples_progress.md`
  `samples/README.md`
  `samples/product-alpha1/README.md`
  `scripts/README.md`
  `docs/hands_on/README.md`
  `docs/hands_on/mir_computational_core_01.md`
  `docs/research_abstract/mir_computational_core_01.md`
  `plan/00-index.md`
  `plan/53-mir-computational-core-roadmap.md`
  `plan/57-autonomous-computational-core-master-plan.md`
  `specs/00-document-map.md`

## Commands run

```bash
git status --short
python3 -m py_compile scripts/mir_computational_samples.py
python3 scripts/mir_computational_samples.py matrix --format json
python3 -m unittest scripts.tests.test_mir_computational_samples
cargo test -p mir-ast --test product_alpha1_package_schema comp04 -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session comp04 -- --nocapture
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/host-io-internal-transform/positive --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/computational/host-io-internal-transform/negative-undeclared-effect --format json
cargo fmt
python3 -m unittest scripts.tests.test_mir_computational_samples scripts.tests.test_validate_docs
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/mir_computational_samples.py run comp-04-host-io-internal-transform-positive --format json
python3 scripts/mir_computational_samples.py run comp-04-host-io-internal-transform-negative-undeclared-effect --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo test -p mir-semantics --test mir_computational_core -- --nocapture
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo fmt --check
git diff --check
rm -rf .mirrorea-alpha
date '+%Y-%m-%d %H:%M:%S %z'
```

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_mir_computational_samples scripts.tests.test_validate_docs`
  passed 27/27 after the helper/matrix/doc sync.
- `python3 scripts/mir_computational_samples.py matrix --format json`
  reported 15 executable rows total:
  7 accepted,
  5 expected runtime rejections,
  3 expected check rejections,
  0 planned-only rows.
- `python3 scripts/mir_computational_samples.py check-all --format json`
  passed with `failed = []`.
- `python3 scripts/mir_computational_samples.py run comp-04-host-io-internal-transform-positive --format json`
  returned accepted direct Product Alpha-1 evidence with:
  `mir_compute_function = add_two`,
  `actual_output_summary = Int(42)`,
  ordered event kinds including `host_input_received -> mir_compute_step -> host_output_emitted`.
- `python3 scripts/mir_computational_samples.py run comp-04-host-io-internal-transform-negative-undeclared-effect --format json`
  returned `check_rejection` with:
  `actual_diagnostic_code = SchemaDecode`,
  detail containing `runtime_input.host_output.effect_ref`.
- `cargo test -p mir-semantics --test mir_computational_core -- --nocapture`
  passed 4/4.
- `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture`
  passed 32/32, including:
  accepted `P-COMP-04`,
  undeclared effect,
  undeclared failure,
  missing capability,
  missing computational boundary fields,
  failure-tag-on-unrelated-contract reject.
- `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture`
  passed 29/29, including accepted `P-COMP-04` direct runtime execution.
- `python3 scripts/check_source_hierarchy.py`
  passed with `required = 235`, `missing = 0`.
- `python3 scripts/validate_docs.py`
  passed and reported documentation scaffold complete.
- `cargo fmt --check`
  passed after `cargo fmt`.
- `git diff --check`
  passed.

## What changed in understanding

- Nested computational sample roots are subject to the same sibling-tree dependency restriction as other Product Alpha-1 package roots, so direct `P-COMP-04` rows cannot honestly escape their root subtree just to point back at `add-one-pure-mir`.
- Reviewer pressure exposed that `required_capabilities` and `failure_tag` were initially too weak to justify the doc claims. The honest fix is not only wording; it is to require those declarations for current computational product rows and bind `failure_tag` to the actual computational contract rather than to any contract in the package.

## Open questions

- Broader computational effectful widening beyond the bounded host read/write wrapper remains open:
  publish / observe / witness / handoff should not be promoted silently from this package.
- `P-POSE-02` no-split-frame runtime evidence is still the next promoted implementation package.

## Suggested next prompt

`P-POSE-02` を実装し、same-client same-observation-snapshot の positive / negative PoseGraph runtime evidence、helper/matrix/docs/report/commit/push まで閉じてください。

## Plan update status

`plan/` 更新済み:
`plan/00-index.md`, `plan/53-mir-computational-core-roadmap.md`, `plan/57-autonomous-computational-core-master-plan.md`

## Documentation.md update status

`Documentation.md` 更新済み:
`P-COMP-04` closeout, checker-admission boundary wording, next reopen point `P-POSE-02`

## progress.md update status

`progress.md` 更新済み:
latest closeout package, computational row status, current blockers, next reopen point, recent log

## tasks.md update status

`tasks.md` 更新済み:
`P-COMP-04` closed, ordered self-driven queue promoted to `P-POSE-02`

## samples_progress.md update status

`samples_progress.md` 更新済み:
computational row status, root status, recent validation log

## Reviewer findings and follow-up

- Reviewer `Poincare` first identified four issues before the initial closeout was honest:
  helper syntax break,
  unmaterialized `comp-04` sample roots,
  weak `check_rejection` matching,
  and overclaim risk around boundary semantics.
  Those findings were fixed by materializing the four direct sample roots, strengthening helper diagnostic/returncode checks, and narrowing doc wording.
- Reviewer `Plato` then found two silent semantic gaps:
  accepted rows did not actually require `required_capabilities` / `failure_tag`,
  and `failure_tag` could be satisfied by an unrelated contract.
  Those findings were fixed by making both declarations mandatory for current computational product rows, binding `failure_tag` to the active computational contract, updating `add-one-pure-mir`, and adding regression tests for missing fields and unrelated-contract bypass.
- Worker `Faraday` was started early in the package and later closed without a useful result; the package was completed through local implementation plus the two reviewer passes above.

## Skipped validations and reasons

- Full product alpha release check, installed-binary adoption probe, operational suite check-all, and Docker-backed flows were not rerun in this package because `P-COMP-04` only changed the computational line plus snapshot docs and did not touch those runtime paths.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- `019e4a72-9a6b-7fe3-8b08-a1df5a37a847` (`Faraday`): closed without a usable result
- `019e4a7b-cf7e-7e50-b254-97b3959c055d` (`Poincare`): completed and closed
- `019e4a86-daf3-7ba3-9ad5-59329a127bce` (`Plato`): completed and closed
