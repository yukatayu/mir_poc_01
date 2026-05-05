# 2041 - P-A1-30 Native Launch Bundle

## Objective

Close `P-A1-30` by adding the product alpha-1 `mirrorea-alpha build-native-bundle` first cut.

The target was a native host launch bundle, not direct Mir-to-machine-code and not arbitrary package-native execution. The bundle must carry the compiled Rust CLI, versioned product package files, observer-safe devtools assets, manifest / launch metadata, run script, verification report, and provenance metadata.

## Scope and assumptions

- Scope: `mirrorea-alpha build-native-bundle <package> --out <dir> --format json`, product demo package bundle generation, generated run script validation, native policy non-claims, docs / dashboard updates, focused validation.
- Assumption: Product alpha-1 native output means a native host launch bundle around the compiled Rust CLI and versioned package files. It does not mean package-native binary execution or final code generation.
- Assumption: `demo` command and clean-clone release candidate validation remain `P-A1-31`.
- Non-claims preserved: final public CLI/API/ABI, final viewer / telemetry ABI, direct textual `.mir` grammar, WAN/federation, distributed durable save/load R3/R4, arbitrary native execution, signature-is-safety.
- The untracked `sub-agent-pro/product-alpha1-001/` handoff remains non-normative working input and is not committed.

## Start state / dirty state

Start state was the pushed `P-A1-29` closeout:

- `e45b5e9 docs: update P-A1-29 report metadata`
- `3a63438 mirrorea: close P-A1-29 product transport viewer`

Working tree during closeout contained P-A1-30 edits to CLI source/tests and product docs. `sub-agent-pro/product-alpha1-001/` was untracked before this package and remains excluded from commit scope.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
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
- `plan/44-practical-alpha1-roadmap.md`
- `plan/45-operational-alpha05-roadmap.md`
- `plan/46-operational-alpha08-roadmap.md`
- `plan/47-operational-alpha09-devtools-roadmap.md`
- `plan/48-theory-freeze-proof-obligations.md`
- `plan/49-host-io-and-session-runtime-roadmap.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `samples/practical-alpha1/README.md`
- `samples/product-alpha1/README.md`
- `samples/product-alpha1/demo/README.md`
- `tmp_faq/faq_015.md` when present in the initial handoff read
- Latest numbered reports before this package: `2038`, `2039`, `2040`
- `sub-agent-pro/product-alpha1-001/00-index.md` through `13-risk-register.md`

## Actions taken

- Implemented `mirrorea-alpha build-native-bundle`.
- Added product bundle output structure:
  - `bin/mirrorea-alpha`
  - `packages/<package-id>/`
  - `devtools/bundle.json`
  - `devtools/index.html`
  - `reports/*.json`
  - `manifest.json`
  - `launch.json`
  - `provenance.json`
  - `run.sh`
  - `README.md`
- Generated bundle evidence by running the existing product carrier in memory:
  - `check`
  - `run-local`
  - debug-layer attach when declared
  - R0 `save`
  - bounded R2 `quiescent-save`
  - local transport
  - `export-devtools`
  - generated `run.sh check`
  - generated `run.sh view`
- Kept `run.sh` intentionally narrow: only `check` and `view` are supported in `P-A1-30`; `demo-local` and CLI `demo` remain `P-A1-31`.
- Rejected direct `.mir` input with the explicit `direct_mir_non_goal` diagnostic.
- Rejected unsafe output directories under the package root and rejected non-empty output directories to avoid recursive or stale bundle evidence.
- Copied only declared package files:
  - root `package.mir.json`
  - declared dependency `package.mir.json` files
  - no generated scratch files or stray local secrets
- Required copied package policies to remain `NativeExecutionPolicy = Disabled`.
- Added manifest / verification non-claims:
  - package-native execution is not claimed
  - arbitrary native execution is not supported
  - direct Mir-to-machine-code is not supported
  - signature metadata is provenance only
  - release `demo` command is deferred
- Ensured observer-safe generated reports do not expose raw witness refs, granted capability names, or private debug/auth capability names.
- Documented generated viewer inspection paths in bundle README: `devtools/index.html` and `devtools/bundle.json`.
- Updated product alpha docs, progress, tasks, sample dashboard, and roadmap memory.

## Files changed

- `crates/mirrorea-cli/src/main.rs`
- `crates/mirrorea-cli/tests/alpha_cli.rs`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/product-alpha1/README.md`
- `samples/product-alpha1/demo/README.md`
- `scripts/README.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `docs/reports/2041-p-a1-30-native-launch-bundle.md`

## Commands run

```bash
cargo fmt --check
cargo test -p mirrorea-cli --test alpha_cli -- build_native_bundle --nocapture
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo check -p mirrorea-cli
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/demo --out "$BUNDLE_DIR" --format json
sh "$BUNDLE_DIR/run.sh" check
sh "$BUNDLE_DIR/run.sh" view
sh "$BUNDLE_DIR/run.sh" demo-local
git diff --check
```

The first manual probe failed because the local verifier script expected `status` on the checker report produced by `run.sh check`; the checker report correctly uses `verdict: accepted`. The probe was corrected and rerun successfully.

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_validate_docs`: 13 tests, OK.
- `python3 scripts/check_source_hierarchy.py`: required 94, present 94, missing 0.
- `python3 scripts/validate_docs.py`: scaffold complete, 1192 numbered reports.
- `cargo fmt --check`: exit 0.
- `cargo check -p mirrorea-cli`: exit 0.
- `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture`: 9 passed.
- `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture`: 14 passed.
- `cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture`: 3 passed.
- `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture`: 18 passed.
- Manual bundle probe:
  - `build-native-bundle` accepted the product demo.
  - generated `run.sh check` produced checker `verdict: accepted`.
  - generated `run.sh view` produced viewer `status: accepted`.
  - generated `run.sh demo-local` rejected as unsupported.
  - `manifest.json` and top-level CLI report shared the same `bundle_id`.
  - `manifest.launch.supported_script_commands` was exactly `["check", "view"]`.
  - copied package files were exactly `["package.mir.json", "packages/debug-layer/package.mir.json"]`.
  - `reports/run-script-demo.json` was absent.
  - generated reports and devtools bundle did not contain `witness_refs`, `granted_capabilities`, `ObserveDebugSummary`, or `AttachDebugLayer`.
- `git diff --check`: exit 0.

## What changed in understanding

`P-A1-30` made the product native output boundary concrete: the α-1 native artifact can be useful to an external developer as a host launch bundle without implying package-native execution. The native policy belongs in both admission and output evidence: the bundle rejects non-disabled package policies and records the disabled policy in manifest / verification output.

The bundle builder also needs stricter output hygiene than an ordinary report exporter. Copying a full package directory can recursively include the output bundle or stray files; for product alpha, only declared package front-door files are admitted.

## Open questions

- `P-A1-31` must decide how much `mirrorea-alpha demo` orchestrates directly versus delegates to documented subcommands.
- Release check must define the clean-clone validation command and decide whether Docker Compose TCP is mandatory or environment-gated with an explicit non-claim.
- Durable audit and distributed durable save/load remain outside this package.

## Suggested next prompt

Continue with `P-A1-31` clean-clone product alpha-1 validation / release candidate closeout: add CLI `demo`, `docs/hands_on/product_alpha1_01.md`, `docs/research_abstract/product_alpha1_01.md`, and `scripts/product_alpha1_release_check.py check-all`.

## Plan update status

Updated `plan/50-product-alpha1-public-boundary-roadmap.md`:

- marked `P-A1-30` as closed;
- recorded the native host launch bundle shape;
- recorded `run.sh check/view` only;
- kept CLI `demo` in `P-A1-31`;
- preserved direct codegen / arbitrary native execution / signature-is-safety non-claims.

## Documentation.md update status

Updated. It now names `build-native-bundle` as implemented and keeps product alpha-1 release validation, CLI `demo`, final public viewer / telemetry ABI, R3/R4 durable distributed save/load, and arbitrary native execution outside the current claim.

## progress.md update status

Updated. It now records product alpha local same-session/save/transport/viewer/native-bundle first cut and keeps the next reopen point at `P-A1-31`.

## tasks.md update status

Updated. It now moves the active promoted line from P-A1-30 to P-A1-31 and records the remaining release candidate work.

## samples_progress.md update status

Updated. It now lists `build-native-bundle` as the validation anchor for the native host launch bundle first cut and keeps CLI `demo` / release validation as `P-A1-31` scope.

## Reviewer findings and follow-up

- Theory reviewer: flagged overclaim risk around arbitrary native execution rejection and copied package policy. Follow-up: verification no longer claims an arbitrary-native negative probe; copied package files are checked for disabled native policy.
- Checker/runtime reviewer: flagged nested output recursion, stale output reuse, synthetic `demo-local`, and inconsistent bundle id. Follow-up: unsafe nested output and non-empty output are rejected; `run.sh demo-local` was removed; bundle id is shared across CLI report / manifest.
- Devtools/UX reviewer: flagged stale package risk and missing viewer inspection guidance. Follow-up: copied package files are allowlisted; bundle README names `devtools/index.html` and `devtools/bundle.json`.
- Product/public boundary reviewer: flagged that a generated `demo-local` path could blur `P-A1-31`. Follow-up: generated script only supports `check` and `view`; CLI `demo` remains unsupported until release candidate work.
- Docs/source-hierarchy reviewer: flagged missing package report and stale native-bundle wording. Follow-up: this report was added and current docs distinguish P-A1-29 historical non-claims from P-A1-30 implementation.
- Security/auth/native-policy reviewer: flagged raw directory copy, nested output, stale output, and native execution policy overclaim. Follow-up: bundle output is not allowed inside package root, non-empty output is rejected, only declared package files are copied, and native execution remains disabled / non-claimed.

## Skipped validations and reasons

- Full product release candidate validation was not run because `P-A1-31` is still open and `mirrorea-alpha demo` / `scripts/product_alpha1_release_check.py` are not implemented yet.
- Docker Compose TCP transport was not rerun in this package because P-A1-30 bundle generation uses the local transport lane; Docker validation remains part of P-A1-29 evidence and P-A1-31 release validation.
- Full workspace `cargo test` was not run; focused package/schema/runtime/CLI tests were run for the touched implementation.

## Commit / push status

- Implementation commit: `f332810 mirrorea: close P-A1-30 native launch bundle`
- Push status: pushed to `main`.
- Report metadata follow-up commit: pending.

## Sub-agent session close status

P-A1-30 used separate review lanes for theory, checker/runtime, devtools/UX, product/public boundary, docs/source hierarchy, and security/auth/native-policy. Their findings are summarized above. After context transition, `close_agent` returned `not found` for the previous review-agent IDs, so there are no retained P-A1-30 sub-agent sessions to close in this context.
