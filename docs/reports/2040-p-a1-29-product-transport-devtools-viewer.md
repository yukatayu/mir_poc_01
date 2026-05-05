# 2040 - P-A1-29 product transport and devtools viewer

Identifier: `P-A1-29`

## Objective

Implement the Product/Public-ready alpha-1 first cut for product transport and devtools/viewer over the same product session carrier. The package closes `transport --mode local`, `transport --mode docker`, `export-devtools`, and `view --check` as alpha-stable Rust CLI commands while preserving observer-safe redaction, explicit auth/membership/capability/witness lanes, and all non-claims for WAN/federation, final public viewer ABI, distributed durable save/load, native package execution, and product alpha release readiness.

## Scope and assumptions

- Scope is P-A1-29 only: product local/Docker transport plus non-final product devtools viewer UX.
- Base session carrier is the P-A1-27/P-A1-28 local file-backed product session carrier.
- Implemented commands in this package:
  `transport`, `export-devtools`, and `view`.
- `build-native-bundle` and `demo` remain later-package structured unsupported diagnostics.
- Local transport uses a real loopback TCP one-shot round trip.
- Docker transport uses a controlled Docker Compose TCP world/participant fixture under `samples/product-alpha1/docker/`.
- Docker endpoint helper commands are fixture-internal and gated by environment variables plus a fixture token; they are not the advertised public CLI family.
- Docker mode executes the already-built host CLI binary as fixture infrastructure only. It does not execute package-native code, and package `NativeExecutionPolicy` remains `disabled`.
- Devtools export is observer-safe JSON plus static HTML. Raw witness/auth material and private capability grant names are not allowed in `bundle.json` or viewer HTML.
- Admin/debug full view remains explicit `kept_later`.
- Docker validation requires Docker and Docker Compose. In this environment both were available and the Docker probe was run; if unavailable, future closeouts must record an environment-gated skip with explicit non-claims.
- Product alpha-1 remains not release-ready until native launch bundle and release validation close.
- Final textual `.mir` grammar, final public CLI/runtime/viewer/telemetry ABI, WAN/federation, distributed durable save/load R3/R4, durable audit backend, arbitrary native execution, full engine/avatar compatibility, PrismCascade, and Reversed Library remain non-goals.
- The handoff under `sub-agent-pro/product-alpha1-001/` was treated as working directive / handoff input, not normative source, and not committed.

## Start state / dirty state

- start branch:
  `main`
- start tracked state:
  clean tracked worktree after pushed P-A1-28 metadata commit `b7e2808`.
- start untracked state:
  `sub-agent-pro/product-alpha1-001/` was present as handoff input and remains untracked.
- temporary generated state:
  `.mirrorea-alpha/` was created by one local probe and removed before closeout because it was generated local session output.
- resource posture:
  no heavy build artifact, LLVM work, generated corpora, or persistent Docker output was committed. Docker Compose pulled/used the standard `ubuntu:24.04` image as fixture runtime in local Docker storage.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `.docs/progress-task-axes.md`
- `samples/README.md`
- `scripts/README.md`
- `plan/00-index.md`
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
- `samples/product-alpha1/docker/README.md`
- `tmp_faq/faq_015.md`
- `docs/reports/2037-p-a1-26-alpha-cli-package-schema-stabilization.md`
- `docs/reports/2038-p-a1-27-product-demo-same-session-runtime.md`
- `docs/reports/2039-p-a1-28-message-recovery-quiescent-save.md`
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

1. Added RED runtime tests for product local transport, Docker Compose TCP boundary reporting, and product devtools panel/redaction coverage.
2. Added RED CLI tests for `transport`, `export-devtools`, and `view --check` using the same product session store.
3. Implemented `crates/mir-runtime::product_alpha1_transport`.
4. Added a real local loopback TCP round trip for `transport --mode local`.
5. Added product transport reports that carry transport medium, transport lane, route/event references, lane preservation, WAN/federation non-claims, Docker evidence paths, and native/package execution non-claims.
6. Added Docker endpoint report types and endpoint functions for a one-shot world server and participant client.
7. Added explicit auth decision ref, capability decision ref, observer-safe witness relation refs, and requirement counts to the transport wire request/response so the route does not collapse auth/capability/witness lanes into transport alone.
8. Implemented `samples/product-alpha1/docker/docker-compose.product-alpha1.yml` and its README.
9. Wired `mirrorea-alpha transport <session> --mode local|docker`.
10. Implemented Docker Compose execution from the CLI and validated the top-level report from `world.json` and `participant.json` endpoint reports.
11. Added fixture-only gating for hidden endpoint helpers with environment variables plus `--fixture-token`.
12. Ensured Docker endpoint helpers exit non-zero for rejected endpoint reports.
13. Mapped Docker/Compose failures to `ProductAlpha1SessionErrorKind::Transport` instead of session-store I/O.
14. Implemented `crates/mir-runtime::product_alpha1_devtools`.
15. Added non-final JSON bundle and static HTML rendering with required product overview, place graph, event DAG, route graph, membership frontier, witness relation, hot-plug lifecycle, save/load/quiescent timeline, message recovery, fallback, auth/capability decision, redaction, and retention panels.
16. Reworked devtools observer-safe panels to avoid serializing raw `witness_refs`, `granted_capabilities`, raw auth evidence, or private debug capability names.
17. Implemented `mirrorea-alpha export-devtools <session> --out <dir>` over the same session carrier.
18. Implemented `mirrorea-alpha view <dir> --check` with bundle schema/openability, required panel, HTML, and raw forbidden-key validation.
19. Added negative CLI tests for malformed/placeholder viewer bundles, observer-unsafe tampered bundle keys, and ungated endpoint helper invocation.
20. Updated source hierarchy checks for the product Docker fixture.
21. Updated README, Documentation, progress, tasks, samples dashboards, product sample READMEs, scripts README, and plan/50 to classify P-A1-29 as product carrier first cut and P-A1-30 as the next reopen point.
22. Ran focused runtime/CLI/schema tests, Docker Compose probe, viewer probe, and documentation/source validation floor.

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/product-alpha1/README.md`
- `samples/product-alpha1/demo/README.md`
- `samples/product-alpha1/docker/README.md`
- `samples/product-alpha1/docker/docker-compose.product-alpha1.yml`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/product_alpha1_session.rs`
- `crates/mir-runtime/src/product_alpha1_transport.rs`
- `crates/mir-runtime/src/product_alpha1_devtools.rs`
- `crates/mir-runtime/tests/product_alpha1_transport_devtools.rs`
- `crates/mirrorea-cli/src/main.rs`
- `crates/mirrorea-cli/tests/alpha_cli.rs`
- `docs/reports/2040-p-a1-29-product-transport-devtools-viewer.md`

## Commands run

```bash
git status --short
git branch --show-current
rg -n "P-A1-28|P-A1-29|transport|viewer|export-devtools|view|local/Docker" README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md samples/product-alpha1/README.md samples/product-alpha1/demo/README.md scripts/README.md plan/50-product-alpha1-public-boundary-roadmap.md specs/25-product-alpha1-public-boundary.md
cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
cargo fmt
cargo fmt --check
cargo check -p mirrorea-cli
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
docker --version
docker compose version
MIRROREA_ALPHA_SESSION_DIR=<temp-session-store> cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
MIRROREA_ALPHA_SESSION_DIR=<temp-session-store> cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode local --format json
MIRROREA_ALPHA_SESSION_DIR=<temp-session-store> cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode docker --format json
MIRROREA_ALPHA_SESSION_DIR=<temp-session-store> cargo run -q -p mirrorea-cli -- export-devtools 'session#product-alpha1-demo' --out <temp-devtools-dir> --format json
cargo run -q -p mirrorea-cli -- view <temp-devtools-dir> --check --format json
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
rm -rf .mirrorea-alpha
```

## Evidence / outputs / test results

- Runtime/CLI RED:
  the initial P-A1-29 tests failed before implementation because `product_alpha1_transport` and `product_alpha1_devtools` did not exist and CLI `transport` / `export-devtools` / `view` were still unsupported.
- Runtime GREEN:
  `cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture` ran 3 tests and passed. It covers local loopback transport lane preservation, Docker Compose TCP boundary/non-WAN report shape, and required devtools panels/redaction.
- CLI GREEN:
  `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture` ran 13 tests and passed. It covers the same-session `transport` / `export-devtools` / `view` flow, observer-safe bundle output, malformed viewer rejection, tampered observer-unsafe bundle rejection, and endpoint-helper fixture gating.
- Existing product session regression:
  `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture` ran 14 tests and passed.
- Product schema regression:
  `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture` ran 9 tests and passed.
- CLI compile check:
  `cargo check -p mirrorea-cli` passed.
- Docker availability:
  `docker --version` and `docker compose version` succeeded.
- Product CLI probe:
  `run-local -> transport --mode local -> transport --mode docker -> export-devtools -> view --check` over one temp session store passed. Local reported `wire_roundtrip_executed = true` and `wire_response_status = accepted`; Docker reported `docker_compose_executed = true`, endpoint outcomes `accepted` / `accepted`, `wire_roundtrip_executed = true`, and `package_native_execution_claimed = false`; export reported 13 panels, `admin_debug_view_status = kept_later`, and no forbidden raw witness/capability strings in the bundle; view reported `viewer_openable = true`, `bundle_valid = true`, and `html_contains_required_panels = true`.
- Required documentation/source/format validation:
  `python3 -m unittest scripts.tests.test_validate_docs` ran 13 tests and passed;
  `python3 scripts/check_source_hierarchy.py` reported required 94, present 94, missing 0;
  `python3 scripts/validate_docs.py` reported a complete scaffold and 1192 numbered reports;
  `cargo fmt --check` passed;
  `git diff --check` passed.

## What changed in understanding

- P-A1-29 cannot honestly close by emitting a Docker-shaped report alone; the top-level Docker report must be derived from endpoint reports produced over the Docker Compose TCP round trip.
- Observer-safe devtools export must validate both the generated bundle structure and raw JSON text, because unknown JSON keys can otherwise be discarded during typed deserialization.
- Product transport may mount and run the host CLI binary as Docker fixture infrastructure only if it remains gated, provenance/non-claim surfaced, and clearly separated from package-native execution.
- The first product viewer can be non-final static HTML, but it still needs enough panel data to explain role, redaction, retention, event/route structure, save/load/quiescent timeline, recovery, fallback, and auth/capability decisions without leaking private material.
- P-A1-29 is now product carrier first cut for transport/viewer, not product alpha release completion.

## Open questions

- Native host launch bundle remains unimplemented and is the next promoted package.
- Release-candidate validation and clean-clone guide remain unimplemented.
- Full admin/debug viewer remains `kept_later`.
- Accepted detach execution remains open; current product attach covers accepted debug-layer attach and observable deferred/rejected boundaries only.
- Docker endpoint-report tamper testing is covered by code validation path and live Docker round trip, but there is not yet a dedicated automated test that corrupts endpoint reports between Compose and top-level validation.
- R3/R4 durable distributed save/load, WAN/federation, exactly-once delivery, arbitrary crash recovery, final textual `.mir` grammar, final public ABI, arbitrary native execution, full engine/avatar compatibility, PrismCascade, and Reversed Library remain non-goals.

## Suggested next prompt

Proceed with `P-A1-30` native launch bundle: implement `mirrorea-alpha build-native-bundle` so it emits a native host launch bundle containing the compiled Rust runtime/CLI, versioned product package bundle, manifest, devtools viewer assets, run script / launch metadata, verification report, provenance-only signature metadata, and explicit `NativeExecutionPolicy = Disabled` / no arbitrary native execution non-claims.

## Plan update status

- updated:
  `plan/50-product-alpha1-public-boundary-roadmap.md` now records P-A1-29 as closed, lists delivered local/Docker transport and devtools viewer first-cut behavior, moves native launch bundle and release validation to the remaining gaps, and advances the next reopen point to P-A1-30.

## Documentation.md update status

- updated:
  `Documentation.md` now lists P-A1-29 local loopback TCP / Docker Compose TCP `transport`, non-final `export-devtools`, and `view --check` as actualized while leaving release validation, native bundle, final public product, final public viewer/telemetry ABI, and R3/R4 save/load as later gates.

## progress.md update status

- updated:
  `progress.md` now records P-A1-29 as latest closeout package, P-A1-30 as current promoted reopen point, updates validation anchors for transport/viewer, and appends a 2026-05-05 15:53 JST recent log entry.

## tasks.md update status

- updated:
  `tasks.md` now records P-A1-29 as actualized transport/viewer first cut and makes P-A1-30 native launch bundle the next self-driven package.

## samples_progress.md update status

- updated:
  `samples_progress.md` now classifies product alpha-1 as `product-carrier-first-cut`, adds `product_alpha1_transport` / `product_alpha1_devtools` evidence, adds transport/export/view validation anchors, and records the P-A1-29 validation log row.

## Reviewer findings and follow-up

- theory reviewer:
  found observer-safe bundle leakage of raw witness/capability material. Fixed by replacing raw witness/auth/capability structs with observer-safe summary panels and tests that scan `bundle.json` and HTML for forbidden strings.
- checker/runtime reviewer:
  found Docker top-level success was not derived from endpoint reports, `view --check` accepted empty/malformed bundles, and Docker failures were labeled session-store I/O. Fixed by endpoint report validation, malformed/tampered viewer negative tests, and `Transport` diagnostics.
- devtools/UX reviewer:
  found viewer/bundle validation and panel content were too weak. Fixed by adding role/redaction/retention overview fields, retention trace structure, schema validation, and required-panel checks.
- product/public boundary reviewer:
  found Docker docs needed environment-gated wording and Docker success needed endpoint evidence. Fixed by docs wording and endpoint-derived Docker report evidence.
- docs/source-hierarchy reviewer:
  found missing report, stale snapshot timestamps, and missing validation anchors. Fixed timestamps and anchors; this report closes the missing report gap.
- security/auth/native-policy reviewer:
  found raw private grants in bundle, hidden helper exposure, and ambiguity between host CLI fixture execution and package-native execution. Fixed by bundle redaction, helper token/env gating, endpoint non-zero rejection, and report fields/documentation showing `host_cli_fixture_execution = true` with `package_native_execution_claimed = false`.
- narrow re-review:
  Docker transport/gating had no blocking findings after fixes. Devtools re-review found a raw-JSON unknown-key bypass; fixed by raw bundle text validation and tampered-bundle negative test. Docs re-review found `samples_progress.md` validation anchor drift; fixed by adding transport/export/view anchors.

## Skipped validations and reasons

- No required P-A1-29 validation was skipped in this environment.
- Full product release-candidate validation was not run because `build-native-bundle`, `demo`, and `scripts/product_alpha1_release_check.py` are P-A1-30/P-A1-31 scope and not yet implemented.
- Full admin/debug viewer validation was not run because admin/debug viewer remains explicit `kept_later`.
- Docker endpoint-report tamper validation is not a dedicated automated test yet; the validation function is exercised by live accepted endpoint reports, while a future tamper test can be added if endpoint report ingestion is widened.

## Commit / push status

- pending at report creation.

## Sub-agent session close status

- initial P-A1-29 code-mapping sub-agents were closed after their findings were incorporated.
- six required review sub-agents were closed after recording findings:
  theory, checker/runtime, devtools/UX, product/public boundary, docs/source-hierarchy, and security/auth/native-policy.
- three narrow re-review sub-agents were closed after follow-up findings were incorporated.
