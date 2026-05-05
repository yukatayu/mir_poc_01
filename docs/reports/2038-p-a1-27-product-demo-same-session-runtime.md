# 2038 - P-A1-27 product demo same-session runtime

Identifier: `P-A1-27`

## Objective

Implement the first product alpha same-session runtime path for `samples/product-alpha1/demo/`: `mirrorea-alpha run-local` writes a product session carrier, `mirrorea-alpha session` reads the same local session file, and `mirrorea-alpha attach` mutates that same carrier with the debug-layer package. The package must show runtime plan, local session state transition, typed host-I/O observation, core fabric lane validation, hot-plug lifecycle, membership/witness/route state, and explicit residual save/load and recovery obligations without claiming product alpha release readiness.

## Scope and assumptions

- Scope is local same-session product runtime first cut only.
- Implemented commands in this package:
  `run-local`, `session`, and `attach`.
- `check` remains the P-A1-26 product schema/checker entrypoint.
- `transport`, `save`, `load`, `quiescent-save`, `export-devtools`, `view`, `build-native-bundle`, and `demo` remain structured unsupported diagnostics.
- The session store is local file-backed and selected with `MIRROREA_ALPHA_SESSION_DIR`; default local session storage is alpha-local and not a durable audit backend.
- Product packages and practical packages remain distinct carriers. This package adds `crates/mir-runtime::product_alpha1_session` rather than silently treating product packages as practical runtime packages.
- Local/Docker transport command behavior, message failure/retry execution, ordinary save/load execution, R2 quiescent-save, product viewer, native launch bundle, clean-clone guide, and release validation remain later-package obligations.
- Final textual `.mir` grammar, final public CLI/runtime ABI, WAN/federation, distributed durable save/load, arbitrary native execution, signature-is-safety, PrismCascade, and Reversed Library remain non-goals.
- The handoff under `sub-agent-pro/product-alpha1-001/` was treated as working directive / handoff input, not normative source, and not committed.

## Start state / dirty state

- start branch:
  `main`
- start tracked state:
  clean tracked worktree on latest pushed main after `P-A1-26`, commit `98857ff`.
- start untracked state:
  `sub-agent-pro/product-alpha1-001/` was present as handoff input and remains untracked.
- resource posture:
  `df -h .` reported 29G available on `/`; `free -h` reported 419Mi available memory and swap available. No heavy build artifacts, LLVM work, generated corpora, Docker images, or cleanup operations were introduced.

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
- `tmp_faq/faq_015.md`
- `docs/reports/2035-root-md-concision-operational-verification.md`
- `docs/reports/2036-p-a1-25-product-alpha1-public-boundary-recut.md`
- `docs/reports/2037-p-a1-26-alpha-cli-package-schema-stabilization.md`
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

1. Continued from the product alpha task baseline and confirmed resource posture with `df -h .` and `free -h`.
2. Mapped existing practical/operational same-session runtime code and CLI/docs implications with code-mapping sub-agents.
3. Added RED tests for a product runtime module and for CLI `run-local` / `attach` same-session persistence.
4. Added `crates/mir-runtime::product_alpha1_session` and exported it from `mir-runtime`.
5. Added a product session carrier with runtime plan, `LogicalPlaceRuntimeSnapshot`, checker/runtime report refs, event DAG, membership state, witness state, route graph, active layers, hot-plug lifecycle, declared typed host-I/O history, auth/capability state and decisions, save/load state, message recovery state, and observer-safe export.
6. Used `mirrorea-core` carriers in the product runtime path:
   `LogicalPlaceRuntimeShell`, `MessageEnvelope`, `AuthEvidence`, `PrincipalClaim`, `HotPlugRequest`, and `HotPlugVerdict`.
7. Added `mir-runtime` as a dependency of `mirrorea-cli`.
8. Implemented `mirrorea-alpha run-local` to check/load the product package, execute the declared `runtime_input.host_io` `AddOne` lane, build the runtime report, and write a local session file.
9. Implemented `mirrorea-alpha session` to read the local session file.
10. Implemented `mirrorea-alpha attach` to load the same session file, evaluate auth/capability/membership/witness gates, attach the debug-layer package when accepted, update event DAG / activation cut / route graph / message state / active layers / hot-plug lifecycle / observer-safe export, and persist the mutated session.
11. Kept direct `.mir` non-goal diagnostics for implemented and unimplemented commands.
12. Updated README, Documentation, progress, tasks, sample dashboards, sample READMEs, scripts README, and plan/50 to classify P-A1-27 as local same-session first-cut evidence, not release-ready.
13. Spawned reviewer lanes for runtime/session semantics, product/public boundary, docs/source hierarchy, and security/auth/native/redaction.
14. Addressed reviewer findings: removed local transport overclaim, added explicit host-I/O runtime input, added attach activation cut and route/message state, made rejected attach observable without active runtime mutation, preserved observer-safe policy monotonically, recorded auth/capability decisions on the session, and hardened CLI session paths with hash suffix plus locked atomic writes.
15. Ran focused Rust tests, CLI probes, docs validation, formatting check, and diff check.

## Files changed

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
- `crates/mir-ast/src/product_alpha1.rs`
- `crates/mir-ast/tests/product_alpha1_package_schema.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/product_alpha1_session.rs`
- `crates/mir-runtime/tests/product_alpha1_session.rs`
- `crates/mirrorea-cli/Cargo.toml`
- `crates/mirrorea-cli/src/main.rs`
- `crates/mirrorea-cli/tests/alpha_cli.rs`
- `samples/product-alpha1/demo/package.mir.json`
- `docs/reports/2038-p-a1-27-product-demo-same-session-runtime.md`

## Commands run

```bash
df -h .
free -h
git status --short
date '+%Y-%m-%d %H:%M %Z'
rg -n 'practical_alpha1|session|run-local|hotplug|save_load|devtools|runtime_plan' crates samples scripts -g '*.rs' -g '*.py' -g '*.json' -g '*.md'
rg --files crates/mir-runtime crates/mirrorea-core crates/mirrorea-cli samples/practical-alpha1 samples/product-alpha1 scripts | sort
sed -n '1,280p' crates/mir-runtime/src/practical_alpha05_session.rs
sed -n '1,340p' crates/mir-runtime/src/practical_alpha08_hotplug_session.rs
sed -n '1,300p' crates/mir-runtime/src/practical_alpha1_local_runtime.rs
sed -n '1,260p' crates/mir-runtime/src/practical_alpha1_hotplug.rs
sed -n '1,280p' crates/mirrorea-core/src/runtime.rs
sed -n '1,360p' crates/mirrorea-core/src/fabric.rs
cargo check -p mirrorea-cli
cargo fmt
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
tmpdir=$(mktemp -d /tmp/mirrorea-alpha1-session-XXXXXX); MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
tmpdir=$(mktemp -d /tmp/mirrorea-alpha1-session-XXXXXX); MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json; MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- session 'session#product-alpha1-demo' --format json; MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/debug-layer --format json; MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- session 'session#product-alpha1-demo' --format json
tmpdir=$(mktemp -d /tmp/mirrorea-alpha1-p27-XXXXXX); MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json; MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/debug-layer --format json; MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- session 'session#product-alpha1-demo' --format json
cargo fmt --check
git diff --check
git diff --stat
```

## Evidence / outputs / test results

- Initial runtime RED:
  `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture` failed with unresolved import `mir_runtime::product_alpha1_session`.
- Initial CLI RED:
  `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture` failed because `run-local` still returned the P-A1-26 unsupported diagnostic.
- Runtime GREEN after reviewer hardening:
  `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture` ran 6 tests and passed. The tests cover same-session carrier construction, declared host-I/O payload execution, non-world `run-local` rejection, accepted attach activation cut/route state/auth decision persistence, rejected attach without active runtime mutation, and observer-safe policy monotonicity.
- CLI GREEN after reviewer hardening:
  `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture` ran 7 tests and passed. The tests cover `run-local`/`session`/`attach` persistence and distinct session paths for IDs that sanitize to the same string.
- Schema regression:
  `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture` ran 9 tests and passed.
- Docs validation:
  `python3 -m unittest scripts.tests.test_validate_docs` ran 13 tests and passed.
- Source hierarchy validation:
  `python3 scripts/check_source_hierarchy.py` reported `required: 91`, `present: 91`, `missing: 0`.
- Documentation scaffold validation:
  `python3 scripts/validate_docs.py` reported a complete scaffold and found 1190 numbered reports including this report.
- CLI check probe:
  `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json` returned `mirrorea_product_alpha1_check_report accepted false` with 10 accepted obligations, including `runtime_input_host_io`.
- Same-session CLI probe:
  a temp `MIRROREA_ALPHA_SESSION_DIR` run produced:
  `run-local` reported `local_transport_claimed: false`, `local_session_store_claimed: true`, and `typed_host_io_claimed: true`,
  `session-before product_alpha1_session_report run_local 5`,
  `attach product_alpha1_attach_report accepted True attached ['product-alpha1-debug-layer']` with `activation_cut#product-alpha1-debug-layer`,
  and `session-after product_alpha1_session_report attached ['product-alpha1-debug-layer:accepted']` with persisted auth/capability decision rows.
- `cargo check -p mirrorea-cli` passed.
- `cargo fmt --check` passed after `cargo fmt`.
- `git diff --check` passed.
- Note:
  two CLI probe summary one-liners initially failed because of shell/Python wrapper typos; the corrected `check` and same-session probes above passed, and the underlying focused CLI tests passed.

## What changed in understanding

- Product alpha now has a real local same-session carrier path, but it remains a product-specific alpha carrier rather than a practical alpha carrier alias.
- Product runtime state is no longer exact-report recomposition: `run-local`, `session`, and `attach` write/read/mutate one persisted product session file, with hash-suffixed session paths and locked atomic writes.
- The product runtime path now validates core fabric lanes with `MessageEnvelope`, `HotPlugRequest`, and `HotPlugVerdict`, keeping auth, membership, capability, witness, and transport lanes separate.
- Typed host-I/O is now driven by versioned package `runtime_input.host_io`; P-A1-27 no longer synthesizes `Int(41)->Int(42)` from package effects alone.
- Debug-layer attach is not treated as a transparent overlay: accepted attach records an activation cut, auth/capability decisions, explicit route/message state, and observer-safe summaries; rejected attach mutates only the persisted observation, not active runtime state.
- Save/load and message recovery state can be carried before execution is implemented; this package deliberately leaves ordinary save/load, quiescent-save, retry/reject execution, and R2 evidence as residual obligations for P-A1-28.
- Observer-safe output is sufficient for current local runtime understanding but not a final viewer or telemetry API.

## Open questions

- `P-A1-28` must decide the first executable recovery transitions for `MessageState`, `TransportContract`, `RecoveryPolicy`, and modal `○` / `□` obligations.
- `P-A1-28` must implement or explicitly reject bounded ordinary save/load and R2 quiescent-save with `NoInFlight`, `AllPlacesSealed`, and `NoPostCutSend`.
- Local/Docker product `transport` command behavior remains unimplemented and should either land with P-A1-28 or remain an explicit P-A1-31 release blocker.
- Accepted detach execution remains open; current product attach covers debug-layer attach only.
- Final textual `.mir` grammar, final public ABI, WAN/federation, distributed durable save/load, arbitrary native execution, full engine/avatar compatibility, PrismCascade, and Reversed Library remain non-goals.

## Suggested next prompt

Proceed with `P-A1-28` message failure/recovery + quiescent-save: implement bounded recovery/failure rows and local R2 quiescent-save over the product session carrier, with positive and negative runtime reports for `NoInFlight`, `AllPlacesSealed`, and `NoPostCutSend`.

## Plan update status

- updated:
  `plan/50-product-alpha1-public-boundary-roadmap.md` now records P-A1-27 as actualized, lists delivered run-local/session/attach behavior, and keeps transport/quiescent-save/viewer/native/release validation as later obligations.

## Documentation.md update status

- updated:
  `Documentation.md` now includes P-A1-27 product same-session runtime first cut and preserves the release validation/product-ready separation.

## progress.md update status

- updated:
  `progress.md` now moves latest closeout to P-A1-27, next reopen point to P-A1-28, adds product local same-session runtime to implementation status, and records the 2026-05-05 14:27 JST work log row.

## tasks.md update status

- updated:
  `tasks.md` now marks P-A1-27 as actualized and advances the ordered self-driven package list to P-A1-28.

## samples_progress.md update status

- updated:
  `samples_progress.md` now classifies the product alpha row as `local-session-first-cut, not release-ready`, adds P-A1-27 validation anchors, and records the validation log row.

## Reviewer findings and follow-up

- runtime/session semantics reviewer:
  found high-risk overclaim in host-I/O synthesis and missing attach activation route/message state, plus medium-risk rejected attach/session-store issues. Follow-up: `runtime_input.host_io` was added to the product package schema and demo, runtime executes declared AddOne payloads, accepted attach records an activation cut plus route/message state, rejected attach is persisted as observation without active runtime mutation, and CLI session store now uses hash-suffixed paths, lock files, and atomic rename.
- product/public boundary reviewer:
  found local transport overclaim, stale checker residual wording, and overly strong `runtime-ready` status wording. Follow-up: `local_transport_claimed` is now false, `local_session_store_claimed` is explicit, checker residuals refer to release validation rather than missing same-session demo, and docs classify P-A1-27 as first-cut / not release-ready.
- docs/source hierarchy reviewer:
  found stale P-A1-26 sample reading and stale P-A1-27 next-package wording in `plan/50`. Follow-up: `samples/README.md` now records the P-A1-27 local same-session first cut, `plan/50` advances next promoted package to P-A1-28, and release validation examples use session-id based store commands.
- security/auth/native/redaction reviewer:
  found attach authorization based only on package kind, observer-safe policy weakening risk, and missing persisted auth/capability decision summaries. Follow-up: attach now evaluates session auth bindings, membership, witness refs, and capability grants; observer-safe role/redaction/retention are monotone across attach; and auth/capability decisions are persisted on the session carrier.

## Skipped validations and reasons

- Full workspace `cargo test` was not run in this package because P-A1-27 touches product runtime/CLI/docs; focused `mir-runtime`, `mirrorea-cli`, and product schema tests plus the required validation floor were run instead.
- Docker Compose transport validation was not run because product `transport --mode docker` remains unimplemented in P-A1-27.
- Ordinary save/load, quiescent-save, devtools export, viewer, native launch bundle, and demo commands were not expected to pass; they remain structured unsupported diagnostics for later packages.
- Storage audit beyond `df -h .` and `free -h` was not run because no heavy artifact, external workdir, LLVM, Docker, or cleanup work was introduced.

## Commit / push status

- package commit:
  `67fb6c6` (`mirrorea: close P-A1-27 product same-session runtime`)
- package push:
  pushed to `origin/main` (`98857ff..67fb6c6`).
- report metadata follow-up:
  required to record the package commit / push status after the first commit.

## Sub-agent session close status

- code-mapping lanes:
  completed; findings were used to preserve product/practical carrier separation and identify CLI persistence as the missing path.
- review lanes:
  runtime/session, product/public boundary, docs/source hierarchy, and security/auth/native/redaction reviewers completed and their findings were either fixed in this package or recorded as residual non-goals.
- remaining open sub-agent sessions:
  none required for P-A1-27 after closeout review.
