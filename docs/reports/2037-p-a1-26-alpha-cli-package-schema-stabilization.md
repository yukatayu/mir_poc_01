# 2037 - P-A1-26 alpha CLI / package schema stabilization

Identifier: `P-A1-26`

## Objective

Add the first product/public alpha-1 developer entrypoint and versioned package front door: a Rust `mirrorea-alpha` binary, a product alpha `package.mir.json` schema/checker surface, a separate `samples/product-alpha1/` demo root, and focused validation that keeps this package at schema/entrypoint readiness without claiming same-session product runtime readiness.

## Scope and assumptions

- Scope is Rust CLI entrypoint, product alpha package schema/checker admission, sample scaffold, docs/dashboard updates, focused tests, and validation.
- This package intentionally implements only `check` in the alpha-stable command family.
- The remaining command family is present as structured unsupported diagnostics: `run-local`, `session`, `attach`, `transport`, `save`, `load`, `quiescent-save`, `export-devtools`, `view`, `build-native-bundle`, and `demo`.
- Product alpha-1 package format is `mirrorea-product-alpha1-v0`, alpha-stable only, and not a final public API.
- Direct textual `.mir` input remains a product alpha-1 non-goal.
- Same-session runtime state, Docker transport, quiescent-save execution, message failure/recovery runtime evidence, product viewer UX, native launch bundle, and clean-clone release validation remain later-package obligations.
- The handoff under `sub-agent-pro/product-alpha1-001/` was treated as working directive / handoff input, not normative source, and not committed.

## Start state / dirty state

- start branch:
  `main`
- start tracked state:
  clean tracked worktree on latest pushed main after `P-A1-25`, commit `1de89c3`.
- start untracked state:
  `sub-agent-pro/product-alpha1-001/` was present as handoff input and remains untracked.
- resource posture:
  no heavy build artifacts, LLVM work, generated corpora, external workdir writes, Docker images, or cleanup operations were introduced.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `specs/19-verification-stratification.md`
- `specs/20-cut-save-load-semantics.md`
- `specs/21-auth-layer-algebra.md`
- `specs/22-observability-devtools-semantics.md`
- `specs/23-typed-external-host-boundary.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `specs/25-product-alpha1-public-boundary.md`
- `plan/00-index.md`
- `plan/44-practical-alpha1-roadmap.md`
- `plan/45-operational-alpha05-roadmap.md`
- `plan/46-operational-alpha08-roadmap.md`
- `plan/47-operational-alpha09-devtools-roadmap.md`
- `plan/48-theory-freeze-proof-obligations.md`
- `plan/49-host-io-and-session-runtime-roadmap.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `samples/practical-alpha1/README.md`
- `tmp_faq/faq_015.md`
- `docs/reports/2034-p-a1-24-workflow-readiness-metric-recut.md`
- `docs/reports/2035-root-md-concision-operational-verification.md`
- `docs/reports/2036-p-a1-25-product-alpha1-public-boundary-recut.md`
- `sub-agent-pro/product-alpha1-001/00-index.md`
- `sub-agent-pro/product-alpha1-001/01-current-state-and-gap.md`
- `sub-agent-pro/product-alpha1-001/02-product-alpha1-definition.md`
- `sub-agent-pro/product-alpha1-001/03-architecture-and-repo-plan.md`
- `sub-agent-pro/product-alpha1-001/04-theory-freeze-requirements.md`
- `sub-agent-pro/product-alpha1-001/05-message-recovery-and-savepoint.md`
- `sub-agent-pro/product-alpha1-001/06-auth-and-layer-algebra.md`
- `sub-agent-pro/product-alpha1-001/07-devtools-ux-design.md`
- `sub-agent-pro/product-alpha1-001/08-native-package-and-host-policy.md`
- `sub-agent-pro/product-alpha1-001/09-sample-and-validation-matrix.md`
- `sub-agent-pro/product-alpha1-001/10-package-sequence.md`
- `sub-agent-pro/product-alpha1-001/11-subagent-review-plan.md`
- `sub-agent-pro/product-alpha1-001/12-closeout-protocol.md`
- `sub-agent-pro/product-alpha1-001/13-risk-register.md`

## Actions taken

1. Continued from the Discord task baseline established for the product alpha-1 line.
2. Reconfirmed the required docs, product boundary, and `P-A1-26` package scope.
3. Spawned code-mapping lanes for existing CLI/workspace patterns and product package/sample expectations.
4. Added failing tests for the product schema, CLI command family, and docs scaffold before implementation.
5. Added the workspace crate `crates/mirrorea-cli` with binary name `mirrorea-alpha`.
6. Added `mir_ast::product_alpha1` with versioned package structs, strict JSON decoding, shape validation, dependency package checking, accepted-obligation evidence, residual-obligation evidence, and stop lines.
7. Added `samples/product-alpha1/` and `samples/product-alpha1/demo/` with a product demo world package and debug-layer dependency package.
8. Updated documentation and source-hierarchy validators to require the product alpha sample root and product package front door.
9. Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `samples/README.md`, `scripts/README.md`, and `plan/50` to distinguish P-A1-26 schema/entrypoint readiness from product workflow readiness.
10. Ran focused tests and validation, then requested review in CLI/schema and docs/security/source-hierarchy lanes.
11. Applied reviewer-driven fixes for missing/unknown schema fields, contract/savepoint/auth/message validation breadth, invalid CLI invocation diagnostics, direct `.mir` non-goal diagnostics on later commands, dependency package admission, residual obligations, and docs validation-floor wording.
12. Reran focused tests and the package validation floor after the fixes.
13. Added this report for the package closeout.

## Files changed

- `Cargo.toml`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/product-alpha1/README.md`
- `samples/product-alpha1/demo/README.md`
- `samples/product-alpha1/demo/package.mir.json`
- `samples/product-alpha1/demo/packages/debug-layer/package.mir.json`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/src/product_alpha1.rs`
- `crates/mir-ast/tests/product_alpha1_package_schema.rs`
- `crates/mirrorea-cli/Cargo.toml`
- `crates/mirrorea-cli/src/main.rs`
- `crates/mirrorea-cli/tests/alpha_cli.rs`
- `docs/reports/2037-p-a1-26-alpha-cli-package-schema-stabilization.md`

## Commands run

```bash
git status --short
date '+%Y-%m-%d %H:%M:%S %Z'
find docs/reports -maxdepth 1 -type f -name '[0-9][0-9][0-9][0-9]-*.md' -printf '%f\n' | sort -V | tail -n 6
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo check -p mirrorea-cli
cargo fmt
cargo fmt --check
git diff --check
git diff --stat
git diff -- Cargo.toml crates/mir-ast/src/lib.rs scripts/validate_docs.py scripts/check_source_hierarchy.py scripts/tests/test_validate_docs.py
sed -n '1,280p' crates/mir-ast/src/product_alpha1.rs
sed -n '1,260p' crates/mirrorea-cli/src/main.rs
sed -n '1,260p' crates/mir-ast/tests/product_alpha1_package_schema.rs
sed -n '1,260p' crates/mirrorea-cli/tests/alpha_cli.rs
sed -n '1,220p' samples/product-alpha1/demo/package.mir.json
sed -n '1,220p' samples/product-alpha1/demo/packages/debug-layer/package.mir.json
```

## Evidence / outputs / test results

- Initial schema RED:
  `cargo test -p mir-ast --test product_alpha1_package_schema` failed because `mir_ast::product_alpha1` did not exist.
- Initial CLI RED:
  `cargo test -p mirrorea-cli --test alpha_cli` failed because the new CLI crate and stdout behavior did not exist.
- Initial docs scaffold RED:
  `python3 -m unittest scripts.tests.test_validate_docs` failed until `samples/product-alpha1/README.md`, `samples/product-alpha1/demo/README.md`, and `samples/product-alpha1/demo/package.mir.json` were registered.
- Reviewer-driven REDs were added and reproduced for:
  missing `auth_stack`, unknown nested `redacted_fieldz`, unsupported contract variance `banana`, unsupported savepoint class `R4`, extra `check` arguments, bogus `check` flags, unsupported `--format yaml`, unimplemented-command direct `.mir`, and missing dependency package admission.
- Focused schema tests passed after fixes:
  `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture` ran 9 tests and passed.
- Focused CLI tests passed after fixes:
  `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture` ran 5 tests and passed.
- Product package check passed:
  `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json` returned an accepted `mirrorea_product_alpha1_check_report` with `product_alpha1_ready: false`, `final_public_api_frozen: false`, accepted dependency evidence, and residual runtime/release obligations.
- Later command family remains structured unsupported:
  `cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json` exited 2 by design and returned `diagnostic_code: not_yet_implemented` and `product_alpha1_ready: false`.
- Documentation validation passed:
  `python3 -m unittest scripts.tests.test_validate_docs` ran 13 tests and passed.
- Source hierarchy validation passed:
  `python3 scripts/check_source_hierarchy.py` reported `required: 91`, `present: 91`, `missing: 0`.
- Documentation scaffold validation passed after this report was added:
  `python3 scripts/validate_docs.py` reported a complete scaffold and found 1189 numbered reports.
- `cargo check -p mirrorea-cli` passed.
- `cargo fmt --check` passed after `cargo fmt`.
- `git diff --check` passed.

## What changed in understanding

- Product alpha-1 now has a canonical Rust entrypoint and a concrete versioned package front door, but only the checker/admission surface is implemented.
- Explicit acceptance evidence is now represented as report rows, not inferred from absence of diagnostics.
- Dependency packages are part of package admission for the product alpha front door; a declared dependency must resolve to a parseable package.
- Runtime obligations must remain residual until later packages produce same-session state transition evidence, quiescent-save runtime evidence, and product devtools/native-bundle artifacts.
- Native output remains provenance-oriented host launch bundle work; arbitrary native package execution remains rejected by schema policy.

## Open questions

- `P-A1-27` must decide the minimal durable session-carrier representation for event DAG, membership, witness, route, hot-plug, save/load, and recovery state.
- `P-A1-28` must decide the first implemented message failure classes and runtime representation for `MessageState`, `TransportContract`, `RecoveryPolicy`, and R0/R1/R2/R3/R4 savepoint classification.
- Accepted detach execution remains open; product alpha-1 may still limit detach to accepted/rejected/deferred semantics if execution is not implemented.
- Final textual `.mir` grammar, final public ABI, WAN/federation, distributed durable save/load, arbitrary native execution, full engine/avatar compatibility, PrismCascade integration, and Reversed Library remain non-goals.

## Suggested next prompt

Proceed with `P-A1-27` product demo same-session runtime: implement `check -> runtime plan -> run-local -> session -> attach` on one product session carrier, with local transport boundary, typed host-I/O observation, hot-plug lifecycle state, and focused validation.

## Plan update status

- updated:
  `plan/50-product-alpha1-public-boundary-roadmap.md` now records that `P-A1-26` has produced the alpha Rust CLI entrypoint, `mirrorea-product-alpha1-v0` package schema, product sample root, explicit accepted/residual checker report, and later-command unsupported diagnostics.
- no normative `plan/` replacement was made; `plan/50` remains repository memory, not the normative source.

## Documentation.md update status

- updated:
  `Documentation.md` now points external developers to `samples/product-alpha1/` as the product alpha package front-door root while keeping product alpha-1 not workflow-ready.

## progress.md update status

- updated:
  `progress.md` now records P-A1-26 as schema/entrypoint-ready, not product workflow-ready, and adds the 2026-05-05 JST work log row.

## tasks.md update status

- updated:
  `tasks.md` now advances the current promoted line from `P-A1-26` toward `P-A1-27` same-session runtime work and keeps release validation blockers separate.

## samples_progress.md update status

- updated:
  `samples_progress.md` now lists `samples/product-alpha1/demo/` as product alpha package/checker evidence with the focused CLI/schema validation commands, not as a workflow-ready runtime sample.

## Reviewer findings and follow-up

- CLI/schema reviewer finding:
  schema accepted missing required fields and unknown nested fields. Fixed by making the alpha package fields required and adding `#[serde(deny_unknown_fields)]` to package/policy structs.
- CLI/schema reviewer finding:
  checker accepted unsupported contract variance, unsupported savepoint classes, incomplete auth/message rows, and missing dependency packages. Fixed with explicit shape validation and dependency package parse checks.
- CLI/schema reviewer finding:
  CLI ignored invalid invocation shape and unsupported formats. Fixed with structured `unexpected_arguments` and `unsupported_format` diagnostics.
- CLI/schema reviewer finding:
  unimplemented commands did not preserve the direct `.mir` non-goal. Fixed so later command stubs emit `direct_mir_non_goal` for `.mir` inputs.
- Docs/security/source-hierarchy reviewer finding:
  progress and sample dashboards omitted the new CLI/schema validation floor. Fixed by adding focused `cargo test`, `cargo run ... check`, and validator commands to the relevant docs.
- Docs/security/source-hierarchy reviewer finding:
  `scripts/README.md` did not describe the product sample front-door guard. Fixed by documenting the product alpha sample paths in the validator descriptions.
- Security/native-policy review found no observer-safe raw witness/auth leak in committed sample data and no native execution overclaim. The schema continues to reject native package execution with `NativeExecutionPolicy must remain disabled`.
- Follow-up:
  runtime semantics, viewer leak tests, native bundle provenance, Docker/local transport, and clean-clone release validation remain assigned to later packages.

## Skipped validations and reasons

- Full workspace `cargo test` was not run in this package because P-A1-26 only touches the new CLI crate, `mir-ast` package-schema module, docs, validators, and samples; focused Rust tests plus the required validation floor were run instead.
- Docker Compose transport validation was not run because transport execution is a `P-A1-27` / later-package implementation target.
- Same-session runtime, quiescent-save runtime, devtools export, and native launch bundle commands were not expected to pass; this package intentionally returns structured unsupported diagnostics for them.
- Storage audit commands were not run because no heavy build artifact, external workdir, LLVM, Docker image, or cleanup operation was introduced.

## Commit / push status

- package commit:
  `e2d235b` `mirrorea: close P-A1-26 alpha CLI schema`.
- package push:
  succeeded; `main -> main` on `origin`.
- report metadata follow-up:
  this section was updated after the package push and will be committed separately.

## Sub-agent session close status

- code-mapping lanes:
  completed before implementation; close attempts after context compaction reported that the agent IDs were no longer present.
- CLI/schema reviewer lane:
  completed; findings integrated before closeout.
- docs/security/source-hierarchy reviewer lane:
  completed; findings integrated before closeout.
- remaining open sub-agent sessions:
  none visible in the resumed context.
