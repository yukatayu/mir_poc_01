# Report 2079 — P-COMP-03 computational first-floor widening

- Date: 2026-05-21 20:59:38 +0900
- Author / agent: Codex
- Scope: `P-COMP-03` variables / arrays / records / control-flow / imports first-floor widening, sample/helper actualization, snapshot-doc sync
- Decision levels touched: `L1` existing computational-core direction, `L2` executable sample/helper realization

## Objective

Close `P-COMP-03` by widening the Mir-owned computational core beyond the single `add_one` row, while preserving the existing Product Alpha-1 operational floor and keeping legacy adapter-owned `typed_host_io.add_one` as host-boundary evidence only.

## Scope and assumptions

- Keep `samples/product-alpha1/computational/add-one-pure-mir/` as the only direct Product Alpha-1 `run-local` root in this package.
- Allow `P-COMP-03` rows to be helper-executable via per-row helper package contracts as long as runtime/schema tests cover the same widened computational registry honestly.
- Do not claim final textual `.mir` grammar, backend realization, final public ABI/SDK, or PoseGraph runtime evidence.

## Start state / dirty state

- Start point was `main` after `P-COMP-02` (`936eebba`) had already been committed and pushed.
- Working tree was then widened locally for `P-COMP-03`; no unrelated user changes were reverted.
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

- Widened `crates/mir-semantics::computational_core` from pure `add_one` only to a bounded first floor with:
  `Bool`, `Vec[Int64]`, `Vec3`, lexical bindings, mutable assignment, `if`, `while`, `return`, record field access, array indexing, and imported function calls.
- Added declared computational modules for:
  `Computational.Scope.Positive`, `Computational.Scope.NegativeUseBeforeDeclare`, `Computational.Arrays.Positive`, `Computational.Arrays.NegativeOutOfBounds`, `Computational.Vec3.Positive`, `Computational.Vec3.NegativeField`, `Computational.ControlFlow.Positive`, `Computational.ControlFlow.NegativeCondition`, `Computational.Compose.Positive`, `Computational.Compose.NegativeMissingImport`.
- Generalized Product Alpha-1 computational schema validation away from `Computational.AddOne` only and added registry-backed module/function/type checks.
- Extended runtime-facing tests so widened computational packages run or reject with stable reasons, and kept the ordered `host_input -> mir_compute -> host_output` evidence for the direct runtime row.
- Expanded `samples/product-alpha1/computational/matrix.json` and `scripts/mir_computational_samples.py` so:
  `comp-02` stays on the real runtime path,
  `comp-03` splits into accepted/runtime-rejection rows with per-row `package_input`,
  `comp-04` stays `planned_only`.
- Added positive/negative sample directories and helper package contracts under:
  `variables-scope/`, `arrays-bounds/`, `records-vec3/`, `control-flow/`, `imports-functions/`.
- Synchronized snapshot docs, sample catalog docs, hands-on docs, research summaries, and roadmap/index docs to reflect `P-COMP-03` closeout and the next reopen point `P-COMP-04`.

## Files changed

- Rust semantics/schema/runtime tests:
  `crates/mir-semantics/src/computational_core/{ast.rs,eval.rs,mod.rs,typecheck.rs,value.rs}`
  `crates/mir-semantics/tests/mir_computational_core.rs`
  `crates/mir-ast/src/product_alpha1.rs`
  `crates/mir-ast/tests/product_alpha1_package_schema.rs`
  `crates/mir-runtime/tests/product_alpha1_session.rs`
- Computational sample/helper:
  `samples/product-alpha1/computational/matrix.json`
  `samples/product-alpha1/computational/{variables-scope,arrays-bounds,records-vec3,control-flow,imports-functions}/**`
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
  `samples/product-alpha1/computational/README.md`
  `scripts/README.md`
  `docs/hands_on/README.md`
  `docs/hands_on/mir_computational_core_01.md`
  `docs/research_abstract/README.md`
  `docs/research_abstract/mir_computational_core_01.md`
  `plan/00-index.md`
  `plan/53-mir-computational-core-roadmap.md`
  `plan/57-autonomous-computational-core-master-plan.md`
  `specs/00-document-map.md`

## Commands run

```bash
cargo fmt
cargo test -p mir-semantics --test mir_computational_core -- --nocapture
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 scripts/mir_computational_samples.py matrix --format json
python3 scripts/mir_computational_samples.py check-all --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/add-one-pure-mir --format json
python3 -m unittest scripts.tests.test_mir_computational_samples scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
date '+%Y-%m-%d %H:%M:%S %z'
git status --short
git diff --stat
rm -rf .mirrorea-alpha
```

## Evidence / outputs / test results

- `cargo test -p mir-semantics --test mir_computational_core -- --nocapture`
  passed 4/4, including positive and negative declared `P-COMP-03` module cases.
- `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture`
  passed 26/26, including widened computational module acceptance and wrong-module / mixed-runtime-input rejects.
- `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture`
  passed 28/28, including widened positive/negative computational execution and ordered event evidence for the direct runtime row.
- `python3 -m unittest scripts.tests.test_mir_computational_samples`
  passed 11/11.
- `python3 scripts/mir_computational_samples.py matrix --format json`
  reported 12 rows total:
  11 executable,
  6 accepted,
  5 expected runtime rejections,
  1 planned-only (`comp-04-host-io-internal-transform`).
- `python3 scripts/mir_computational_samples.py check-all --format json`
  passed with `failed = []` and preserved the split between accepted and expected runtime rejection rows.
- `cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/add-one-pure-mir --format json`
  still returned accepted direct runtime evidence for `ReadInt(41) -> add_one -> WriteInt(42)`.
- `python3 -m unittest scripts.tests.test_mir_computational_samples scripts.tests.test_validate_docs`
  passed 25/25.
- `python3 scripts/check_source_hierarchy.py`
  passed with `required = 235`, `missing = 0`.
- `python3 scripts/validate_docs.py`
  passed and reported documentation scaffold complete.
- `cargo fmt --check`
  passed after `cargo fmt`.
- `git diff --check`
  passed.

## What changed in understanding

- `P-COMP-03` can close honestly without pretending every widened row is already on the same direct Product Alpha-1 front door.
- The honest split is:
  one direct runtime row for `comp-02`,
  helper-executable per-row contracts for `comp-03`,
  runtime/schema tests proving the same widened computational registry,
  `comp-04` still explicitly planned.
- This keeps the computational-core widening real while avoiding an overclaim about finalized package shapes for every first-floor row.

## Open questions

- `P-COMP-04` still needs an executable surface for declared effect / failure / capability rejection around internal computation.
- Whether widened computational rows should later move from helper-package contracts to direct Product Alpha-1 sample roots remains open and should be decided in a bounded follow-up package, not silently in docs.

## Suggested next prompt

`P-COMP-04` を実装し、pure computation と host read/write effect wrapper を分離しつつ、undeclared effect / failure / capability rows の reject evidence、helper/docs/report/commit/push まで閉じてください。

## Plan update status

`plan/` 更新済み:
`plan/00-index.md`, `plan/53-mir-computational-core-roadmap.md`, `plan/57-autonomous-computational-core-master-plan.md`

## Documentation.md update status

`Documentation.md` 更新済み:
`P-COMP-03` closeout and next reopen point `P-COMP-04`

## progress.md update status

`progress.md` 更新済み:
latest closeout package, computational row status, next reopen point, recent log

## tasks.md update status

`tasks.md` 更新済み:
`P-COMP-03` closed, ordered self-driven queue promoted to `P-COMP-04`

## samples_progress.md update status

`samples_progress.md` 更新済み:
computational row status, root status, recent validation log

## Reviewer findings and follow-up

- Reviewer `Aquinas` identified four required widenings before `P-COMP-03` could be honest:
  schema/runtime generalization beyond `Computational.AddOne`,
  richer sample matrix fields,
  widening `mir-semantics::computational_core` before runtime special cases,
  stronger regression coverage for ordered evidence and negative rejects.
- Those findings were incorporated into the implementation.
- Worker `Heisenberg` implemented the sample/helper-side row split and machine-readable matrix expansion; the result was reviewed locally and integrated.
- Final read-only reviewer attempts `Cicero` and `Mendel` were started after the implementation but did not return before timeout; both were closed after one retry, and the package was closed using local diff inspection plus the focused validation evidence above.

## Skipped validations and reasons

- Full product alpha release check, installed-binary adoption probe, operational suite check-all, and Docker-backed flows were not rerun in this package because `P-COMP-03` only changed the computational line plus snapshot docs and did not touch those runtime paths.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- `019e4a51-95b4-7a40-88c3-8bf97dae9bb4` (`Aquinas`): completed and closed
- `019e4a53-bfd2-7720-a361-eab05b2dad6e` (`Heisenberg`): completed and closed
- `019e4a69-82d3-7953-a759-d535f2c79cac` (`Cicero`): timed out, then closed
- `019e4a6d-306d-71a2-b752-d990a188c86c` (`Mendel`): retry reviewer timed out, then closed
