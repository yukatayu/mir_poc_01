# Report 2078 — P-COMP-02 Mir-owned add-one

- Date: 2026-05-21
- Author / agent: Codex
- Scope: `P-COMP-02` executable computational row, schema/runtime/session/savepoint widening, helper promotion, and snapshot doc synchronization
- Decision levels touched: existing `L1` / `L2` computational-core boundary wording was implemented additively; no new normative decision beyond `specs/28` / `specs/32`

## Objective

Close `P-COMP-02` by making one bounded `add_one` path demonstrably Mir-owned while preserving the legacy adapter-owned `typed_host_io.add_one` lane for demo / operational roots.

## Scope and assumptions

- The new computational lane must be additive and must not reinterpret legacy `runtime_input.host_io`.
- Current executable input remains versioned `package.mir.json`; `.mir` files stay explanatory only.
- The runtime proof point is bounded to `ReadInt -> add_one -> WriteInt`.
- Session/savepoint JSON compatibility should be preserved by adding new fields with serde defaults rather than breaking old stores.

## Start state / dirty state

- Started on `main` after `P-ENG-01` / front-half closeout were already pushed.
- Local worktree was intentionally dirty with red-phase edits for:
  `scripts/tests/test_mir_computational_samples.py`,
  `crates/mir-semantics/tests/mir_computational_core.rs`,
  `crates/mir-ast/tests/product_alpha1_package_schema.rs`,
  `crates/mir-runtime/tests/product_alpha1_session.rs`.
- Those red-phase changes were retained and implemented forward rather than reverted.

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
- `samples/README.md`
- `samples/product-alpha1/README.md`
- `samples/product-alpha1/computational/README.md`
- `docs/hands_on/mir_computational_core_01.md`
- `docs/research_abstract/mir_computational_core_01.md`
- `sub-agent-pro/mirrorea_mir_computational_core_handoff.md`
- sub-agent findings from `Russell`, `Confucius`, and `Curie`

## Actions taken

- Confirmed the red tests fail at the intended seams: missing `mir_semantics::computational_core`, unknown `runtime_input.host_input`, missing `mir_computation_claimed` / `mir_compute_history`, and planned-only helper expectations.
- Added `crates/mir-semantics/src/computational_core/` with a narrow pure fragment:
  `Module`, `Function`, `Expr`, `Type`, `Value`, `typecheck_module()`, `eval_function()`, `declared_module()`, and `add_one_module()`.
- Extended `crates/mir-ast/src/product_alpha1.rs` so `runtime_input` can carry additive `host_input`, `mir_compute`, and `host_output` fields while preserving legacy `host_io`.
- Added explicit validation for:
  exclusivity between legacy `host_io` and the new computational lane,
  `ReadInt` / `WriteInt` identity boundaries,
  `Computational.AddOne::add_one`,
  and `host_input -> mir_compute -> host_output` coherence.
- Extended `crates/mir-runtime/src/product_alpha1_session.rs` to:
  preserve legacy host-I/O execution,
  execute a new computational lane,
  emit `mir_compute_history`,
  serialize it through run-local/session/savepoint/load,
  and materialize event order `host_input_received -> mir_compute_step -> host_output_emitted`.
- Added `mir_computation_claimed` to the run-local report and surfaced it in CLI JSON.
- Promoted `samples/product-alpha1/computational/add-one-pure-mir/` with executable `package.mir.json` and `expected/add-one-pure-mir.expected.json`.
- Reworked `scripts/mir_computational_samples.py` so `comp-02-pure-add-one` executes via `mirrorea-cli run-local`, validates computational evidence, and leaves the remaining rows planned-only.
- Added or widened tests in:
  `crates/mir-semantics/tests/mir_computational_core.rs`,
  `crates/mir-ast/tests/product_alpha1_package_schema.rs`,
  `crates/mir-runtime/tests/product_alpha1_session.rs`,
  and `scripts/tests/test_mir_computational_samples.py`.
- Added a save/load regression proving `mir_compute_history` survives `R0` save/load.
- Updated snapshot docs, dashboards, and repository-memory docs so `P-COMP-02` is closed and the next reopen point becomes `P-COMP-03`.

## Files changed

- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/computational_core/ast.rs`
- `crates/mir-semantics/src/computational_core/eval.rs`
- `crates/mir-semantics/src/computational_core/mod.rs`
- `crates/mir-semantics/src/computational_core/typecheck.rs`
- `crates/mir-semantics/src/computational_core/value.rs`
- `crates/mir-semantics/tests/mir_computational_core.rs`
- `crates/mir-ast/src/product_alpha1.rs`
- `crates/mir-ast/tests/product_alpha1_package_schema.rs`
- `crates/mir-runtime/src/product_alpha1_session.rs`
- `crates/mir-runtime/tests/product_alpha1_session.rs`
- `crates/mirrorea-cli/src/main.rs`
- `samples/product-alpha1/computational/README.md`
- `samples/product-alpha1/computational/matrix.json`
- `samples/product-alpha1/computational/add-one-pure-mir/README.md`
- `samples/product-alpha1/computational/add-one-pure-mir/package.mir.json`
- `samples/product-alpha1/computational/expected/add-one-pure-mir.expected.json`
- `scripts/mir_computational_samples.py`
- `scripts/tests/test_mir_computational_samples.py`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/product-alpha1/README.md`
- `scripts/README.md`
- `docs/hands_on/README.md`
- `docs/hands_on/mir_computational_core_01.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mir_computational_core_01.md`
- `plan/00-index.md`
- `plan/53-mir-computational-core-roadmap.md`
- `plan/57-autonomous-computational-core-master-plan.md`
- `specs/00-document-map.md`

## Commands run

```bash
git status --short
date '+%Y-%m-%d %H:%M:%S %Z'
df -h .
free -h
cargo test -p mir-semantics --test mir_computational_core -- --nocapture
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 scripts/mir_computational_samples.py matrix --format json
python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json
python3 scripts/mir_computational_samples.py check-all --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/add-one-pure-mir --format json
python3 -m unittest scripts.tests.test_mir_computational_samples scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt
cargo fmt --check
git diff --check
date '+%Y-%m-%d %H:%M JST'
```

## Evidence / outputs / test results

- Red-phase failures were observed before implementation:
  - `cargo test -p mir-semantics --test mir_computational_core` failed on unresolved `mir_semantics::computational_core`
  - `cargo test -p mir-ast --test product_alpha1_package_schema` failed on unknown `runtime_input.host_input`
  - `cargo test -p mir-runtime --test product_alpha1_session` failed on missing `mir_computation_claimed` / `mir_compute_history`
  - `python3 -m unittest scripts.tests.test_mir_computational_samples` failed because helper state was still planned-only
- `cargo test -p mir-semantics --test mir_computational_core -- --nocapture`
  - passed: 2 tests
- `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture`
  - passed: 21 tests
- `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture`
  - passed: 26 tests
  - includes new `product_alpha1_run_local_executes_mir_owned_add_one_path`
  - includes new `product_alpha1_save_and_load_preserve_mir_compute_history`
- `python3 -m unittest scripts.tests.test_mir_computational_samples scripts.tests.test_validate_docs`
  - passed: 23 tests
- `python3 scripts/mir_computational_samples.py check-all --format json`
  - `sample_count = 7`
  - `passed = ["comp-02-pure-add-one"]`
  - `planned = 6 rows`
  - `failed = []`
- `python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json`
  - `terminal_outcome = accepted`
  - `mir_computation_claimed = true`
  - `mir_compute_function = add_one`
  - `event_kinds_after` contains `host_input_received`, `mir_compute_step`, `host_output_emitted` in order
- `cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/add-one-pure-mir --format json`
  - `surface_kind = product_alpha1_run_local_report`
  - `typed_host_io_claimed = true`
  - `mir_computation_claimed = true`
  - `session.mir_compute_history[0] = Computational.AddOne / add_one / Int(41) -> Int(42)`
  - `session.host_io_history = ReadInt(Int(41))`, `WriteInt(Int(42))`
- `python3 scripts/check_source_hierarchy.py`
  - `required = 235`
  - `present = 235`
  - `missing = 0`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 1229 numbered report(s).`
- `cargo fmt --check`
  - passed after formatting
- `git diff --check`
  - passed

## What changed in understanding

- The smallest safe implementation path was not to mutate current-L2 semantics, but to add a new isolated `mir-semantics::computational_core` leaf and wrap it from product-alpha runtime.
- Product-alpha `run-local` already serializes the full runtime report, so the computational helper could rely on CLI JSON directly instead of inventing a fake bridge surface.
- Save/load compatibility mattered immediately. Without `#[serde(default)]` on the new session/savepoint compute fields, local session stores would have become brittle.

## Open questions

- `P-COMP-03` still needs the exact positive/negative row surface for variables / arrays / records / control-flow, but this is a bounded implementation choice inside the existing `specs/28` package order.
- The current computational row still carries the generic product-alpha message-recovery tail after output. This is acceptable for `P-COMP-02`, but `P-COMP-03` may want to decide whether positive computational rows should keep or narrow that tail.

## Suggested next prompt

Proceed with `P-COMP-03`, widening the same computational core to variables / arrays / records / control-flow positive and negative rows without changing the legacy adapter-owned `typed_host_io.add_one` semantics.

## Plan update status

`plan/` 更新済み:
`plan/00-index.md`, `plan/53-mir-computational-core-roadmap.md`, and `plan/57-autonomous-computational-core-master-plan.md` now record that `P-COMP-02` is closed and `P-COMP-03` is the next computational reopen point.

## Documentation.md update status

`Documentation.md` 更新済み:
the computational-core line now states that one executable Mir-owned row exists while PoseGraph / projection / engine-adapter remain scaffold or inventory lines.

## progress.md update status

`progress.md` 更新済み:
latest closeout package, current reopen point, subsystem rows, blockers, and recent log were synchronized to `P-COMP-02`.

## tasks.md update status

`tasks.md` 更新済み:
`P-COMP-02` is marked closed, the ordered queue now starts at `P-COMP-03`, and the recommendation now points at first-floor widening.

## samples_progress.md update status

`samples_progress.md` 更新済み:
the computational row now records one executable runtime root plus planned widening rows, and the recent log now includes `P-COMP-02`.

## Reviewer findings and follow-up

- `Russell` mapped the existing adapter-owned AddOne path and the minimal product/runtime insertion points.
- `Confucius` confirmed that `mir-semantics` had no existing pure computational layer and recommended a new isolated leaf module rather than extending current-L2.
- `Curie` highlighted the highest-risk regressions:
  additive `runtime_input` shape, session/savepoint compatibility, CLI/report contract drift, and legacy AddOne reinterpretation.
- Follow-up taken:
  implemented additive schema only,
  preserved legacy `host_io`,
  added serde defaults for new session/savepoint fields,
  added legacy negative assertions,
  and added a computational save/load regression.

## Skipped validations and reasons

- `python3 scripts/product_alpha1_release_check.py --format json check-all --out ...` was not rerun because `P-COMP-02` does not change the release-candidate demo package or operational suite workflow contract directly; focused runtime and computational helper tests were used instead.
- Docker-backed flows were not rerun for the same reason: this package widens the computational sample line, not transport behavior.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- `Russell`, `Confucius`, and `Curie` completed and were closed after their findings were integrated.
