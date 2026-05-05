# 2039 - P-A1-28 message recovery and quiescent save

Identifier: `P-A1-28`

## Objective

Implement the product alpha-1 first executable save/recovery slice on the existing local same-session product carrier: bounded message failure/recovery rows, local R0 `save` / `load`, and bounded local R2 `quiescent-save` with explicit `NoInFlight`, `AllPlacesSealed`, and `NoPostCutSend` evidence. The package must preserve the distinction between local bounded save/load and durable distributed save/load, and it must not claim product alpha release readiness.

## Scope and assumptions

- Scope is product alpha-1 local same-session save/recovery first cut only.
- Implemented commands in this package:
  `save`, `load`, and `quiescent-save`.
- Existing `check`, `run-local`, `session`, and `attach` behavior from P-A1-26/P-A1-27 remains the base session carrier path.
- `transport`, `export-devtools`, `view`, `build-native-bundle`, and `demo` remain later-package command surfaces.
- Ordinary local save/load is classified as `R0_Local`; bounded quiescent save is classified as `R2_Quiescent`.
- `NoInFlight`, `AllPlacesSealed`, and `NoPostCutSend` are runtime/preflight obligations surfaced in the report, not proof of distributed durability.
- Message recovery in this package covers bounded timeout / retry-then-reject / reject rows only.
- The session store remains local file-backed storage selected by `MIRROREA_ALPHA_SESSION_DIR`; it is not a durable audit backend.
- Product packages, practical packages, and operational alpha carriers remain distinct.
- Final textual `.mir` grammar, final public CLI/runtime ABI, WAN/federation, distributed durable save/load R3/R4, exactly-once transport, arbitrary crash recovery, arbitrary native execution, full engine/avatar compatibility, PrismCascade, and Reversed Library remain non-goals.
- The handoff under `sub-agent-pro/product-alpha1-001/` was treated as working directive / handoff input, not normative source, and not committed.

## Start state / dirty state

- start branch:
  `main`
- start tracked state:
  clean tracked worktree on latest pushed main after `P-A1-27`, commit `6e7bf87`.
- start untracked state:
  `sub-agent-pro/product-alpha1-001/` was present as handoff input and remains untracked.
- resource posture:
  `df -h .` reported 29G available on `/`; `free -h` reported 450Mi available memory and swap available. No heavy build artifacts, LLVM work, generated corpora, Docker images, or cleanup operations were introduced.

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
- `tmp_faq/faq_015.md`
- `docs/reports/2036-p-a1-25-product-alpha1-public-boundary-recut.md`
- `docs/reports/2037-p-a1-26-alpha-cli-package-schema-stabilization.md`
- `docs/reports/2038-p-a1-27-product-demo-same-session-runtime.md`
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

1. Continued from the P-A1-27 local same-session runtime carrier.
2. Reconfirmed resource posture with `df -h .` and `free -h`.
3. Used TDD for the new behavior: added failing runtime tests for message recovery rows, local save/load restore, quiescent-save success, and quiescent-save rejection with an in-flight message.
4. Added a failing CLI test for same-session `save`, `quiescent-save`, and `load`.
5. Extended `crates/mir-runtime::product_alpha1_session` with product save/load/quiescent-save reports, savepoint records, session-carried quiescence preflight state, and load-admissibility errors.
6. Added same-session savepoint storage to `ProductAlpha1SessionCarrier`.
7. Added `MessageState`, `TransportContract`, `RecoveryPolicy`, DAG/route-linked failure observation, and modal obligation rows to the product runtime carrier.
8. Implemented `save_product_alpha1_session` for local R0 savepoints.
9. Implemented `load_product_alpha1_session` for restoring runtime snapshot, event DAG, membership, witness, route graph, hot-plug lifecycle, host-I/O history, auth/capability decisions, save-load state, and recovery state from a local savepoint, with stale membership/witness/auth/capability and accepted-activation-cut rewind rejection.
10. Implemented `quiescent_save_product_alpha1_session` for bounded local R2 savepoints with `NoInFlight`, session-carried `AllPlacesSealed`, and session-carried `NoPostCutSend` preflight/report flags.
11. Added rejected quiescent-save evidence when an in-flight message remains.
12. Implemented CLI `save`, canonical `load <savepoint> --session <session>`, legacy `load <session> --savepoint <savepoint>`, and `quiescent-save` over the existing locked local session store.
13. Hardened repeated save/load/quiescent-save operations so event IDs remain unique.
14. Narrowed `save` / `load` / `quiescent-save` command stdout to observer-safe session summaries while the local session store remains the admin/debug persistence carrier.
15. Kept `transport`, `export-devtools`, `view`, `build-native-bundle`, and `demo` as structured unsupported diagnostics.
16. Updated README, Documentation, progress, tasks, sample dashboards, sample READMEs, scripts README, and plan/50 to classify P-A1-28 as local save/recovery first cut and to advance the next promoted package to P-A1-29 with explicit transport ownership.
17. Ran focused runtime/CLI/schema tests and product CLI probes.

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
- `specs/25-product-alpha1-public-boundary.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `crates/mir-runtime/src/product_alpha1_session.rs`
- `crates/mir-runtime/tests/product_alpha1_session.rs`
- `crates/mirrorea-cli/src/main.rs`
- `crates/mirrorea-cli/tests/alpha_cli.rs`
- `docs/reports/2039-p-a1-28-message-recovery-quiescent-save.md`

## Commands run

```bash
git status --short
git branch --show-current
git log --oneline -5
rg -n "save/load|quiescent-save|unsupported|P-A1-28|P-A1-29|message recovery|R0|R2|R3|R4" scripts/README.md plan/50-product-alpha1-public-boundary-roadmap.md README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md samples/product-alpha1/README.md samples/product-alpha1/demo/README.md
sed -n '1,260p' scripts/README.md
sed -n '1,320p' plan/50-product-alpha1-public-boundary-roadmap.md
sed -n '320,520p' plan/50-product-alpha1-public-boundary-roadmap.md
rg -n "save_product_alpha1_session|load_product_alpha1_session|quiescent_save_product_alpha1_session|build_session_savepoint|append_quiescent_protocol_events|ProductAlpha1QuiescentSaveReport|build_run_local_session" crates/mir-runtime/src/product_alpha1_session.rs
sed -n '1,260p' crates/mir-runtime/src/product_alpha1_session.rs
sed -n '260,620p' crates/mir-runtime/src/product_alpha1_session.rs
sed -n '620,980p' crates/mir-runtime/src/product_alpha1_session.rs
sed -n '980,1220p' crates/mir-runtime/src/product_alpha1_session.rs
cargo fmt
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
cargo check -p mirrorea-cli
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
command -v jq
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
MIRROREA_ALPHA_SESSION_DIR=<temp-session-store> cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
MIRROREA_ALPHA_SESSION_DIR=<temp-session-store> cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/debug-layer --format json
MIRROREA_ALPHA_SESSION_DIR=<temp-session-store> cargo run -q -p mirrorea-cli -- save 'session#product-alpha1-demo' --savepoint 'savepoint#r0' --format json
MIRROREA_ALPHA_SESSION_DIR=<temp-session-store> cargo run -q -p mirrorea-cli -- quiescent-save 'session#product-alpha1-demo' --savepoint 'savepoint#r2' --format json
MIRROREA_ALPHA_SESSION_DIR=<temp-session-store> cargo run -q -p mirrorea-cli -- load 'savepoint#r0' --session 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR=<temp-session-store> cargo run -q -p mirrorea-cli -- session 'session#product-alpha1-demo' --format json
date '+%Y-%m-%d %H:%M %Z'
df -h .
free -h
git diff --stat
rg -n 'load '\''session|--savepoint latest|product devtools viewer UX|local same-session run/session/attach first cut|15:06 JST|load <session' README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md samples/product-alpha1/README.md samples/product-alpha1/demo/README.md scripts/README.md plan/50-product-alpha1-public-boundary-roadmap.md docs/reports/2039-p-a1-28-message-recovery-quiescent-save.md specs/25-product-alpha1-public-boundary.md
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
cargo check -p mirrorea-cli
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- Runtime RED:
  `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture` failed before implementation because `load_product_alpha1_session`, `quiescent_save_product_alpha1_session`, `save_product_alpha1_session`, and recovery row fields did not exist.
- CLI RED:
  `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture` failed before implementation because `save`, `load`, and `quiescent-save` still returned structured unsupported diagnostics.
- Runtime GREEN:
  `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture` ran 14 tests and passed. The tests cover same-session runtime construction, declared host-I/O execution, attach behavior, DAG/route-linked message recovery row presence, R0 save/load restore, load-admissibility rejection for stale membership and activation-cut rewind, unique event IDs for repeated save/load/quiescent-save, coherent save-load timeline restore, R2 quiescent-save success, missing R2 / missing preflight guard rejection, and in-flight rejection.
- CLI GREEN:
  `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture` ran 9 tests and passed. The tests cover `save`, `quiescent-save`, canonical `load <savepoint> --session <session>`, repeated default save/quiescent-save event-ID uniqueness, and observer-safe save stdout over the same session file.
- CLI compile check:
  `cargo check -p mirrorea-cli` passed.
- Product schema regression:
  `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture` ran 9 tests and passed.
- Product CLI probe:
  a temp `MIRROREA_ALPHA_SESSION_DIR` run produced `product_alpha1_run_local_report run_local`, `product_alpha1_attach_report accepted`, `product_alpha1_save_report saved R0_Local`, `product_alpha1_quiescent_save_report saved R2_Quiescent`, `product_alpha1_load_report loaded R0_Local`, and final `product_alpha1_session_report loaded`.
- Required documentation/source/format validation:
  `python3 -m unittest scripts.tests.test_validate_docs` ran 13 tests and passed;
  `python3 scripts/check_source_hierarchy.py` reported required 91, present 91, missing 0;
  `python3 scripts/validate_docs.py` reported a complete scaffold and 1191 numbered reports;
  `cargo fmt --check` passed;
  `git diff --check` passed.

## What changed in understanding

- Product alpha-1 now has local same-session save/load execution, but only for the bounded local session store.
- The R2 quiescent-save path can demonstrate `NoInFlight`, `AllPlacesSealed`, and `NoPostCutSend` from session-carried preflight state for the controlled product demo carrier, including missing-R2/missing-guard and in-flight negative rows.
- Message failure/recovery is now represented as typed carrier data linked from the event DAG and route graph with bounded timeout/retry/reject observations rather than a textual residual note alone.
- Savepoints are explicit local session records with roundtrip evidence and load-admissibility gates; they are not durable distributed checkpoints and do not imply R3/R4 behavior.
- P-A1-29 can now consume the same product session carrier and save/recovery timelines for the transport/viewer path, rather than relying on exact report bundles only.
- Normative update:
  `specs/25-product-alpha1-public-boundary.md` package sequence now names P-A1-29 as product local/Docker transport plus devtools viewer UX so the required product transport command family has an explicit owning package before release validation.

## Open questions

- Local/Docker product `transport` command behavior remains unimplemented and is still a release blocker.
- Product viewer export/openability remains unimplemented and is the next promoted package.
- Native launch bundle and release-candidate validation remain later packages.
- Accepted detach execution remains open; current product attach covers accepted debug-layer attach and observable deferred/rejected boundaries only.
- R3/R4 durable distributed save/load, WAN/federation, exactly-once delivery, arbitrary crash recovery, final textual `.mir` grammar, final public ABI, arbitrary native execution, full engine/avatar compatibility, PrismCascade, and Reversed Library remain non-goals.

## Suggested next prompt

Proceed with `P-A1-29` product local/Docker transport plus devtools viewer UX: implement `transport --mode local|docker`, `export-devtools`, and `view` for the product demo session so the viewer shows overview, place graph, event DAG, routes, membership/witness timelines, hot-plug lifecycle, save/load/quiescent-save timeline, message recovery, fallback, auth/capability decisions, redaction, and retention/on-demand trace without leaking observer-unsafe secrets.

## Plan update status

- updated:
  `plan/50-product-alpha1-public-boundary-roadmap.md` now records P-A1-28 as actualized, lists delivered local R0/R2 save/recovery behavior, removes message recovery/quiescent-save from the remaining product alpha gaps, and advances the next promoted package to P-A1-29 with explicit product transport ownership.

## Documentation.md update status

- updated:
  `Documentation.md` now includes P-A1-28 bounded message recovery rows, R0 local `save` / `load`, and R2 local `quiescent-save`, while preserving the release validation/product-ready separation.

## progress.md update status

- updated:
  `progress.md` now moves latest closeout to P-A1-28, next reopen point to P-A1-29 transport/viewer, adds product local save/recovery first cut to implementation status, and records the 2026-05-05 15:06 JST work log row.

## tasks.md update status

- updated:
  `tasks.md` now marks P-A1-28 as actualized and advances the ordered self-driven package list to P-A1-29 transport/viewer, P-A1-30, and P-A1-31.

## samples_progress.md update status

- updated:
  `samples_progress.md` now classifies the product alpha row as local same-session runtime/save first cut, adds P-A1-28 validation anchors, and records the validation log row.

## Reviewer findings and follow-up

- theory reviewer:
  found missing load-admissibility gates, constructed R2 proof points, and recovery rows not linked to the event DAG/route graph. Follow-up: `LoadAdmissibility` errors now reject stale membership/witness/auth/capability and accepted activation-cut rewind; R2 evaluates session-carried quiescence state; recovery observations are linked into event DAG and route graph; focused runtime tests cover each case.
- checker/runtime reviewer:
  found fabricated `AllPlacesSealed` / `NoPostCutSend`, duplicate event IDs on repeated save/load/quiescent-save, recovery policy drift from package declarations, missing R2 policy checks, and CLI `load` syntax drift. Follow-up: quiescence preflight state now records sealed places and rejected post-cut sends, event IDs are generated uniquely, recovery rows derive from declared policy, R2 is required for quiescent-save success, and CLI supports canonical `load <savepoint> --session <session>` while retaining legacy session/savepoint form.
- devtools/UX reviewer:
  found message recovery and quiescent-save data would mislead P-A1-29 viewer panels, and load could mix old frontiers with newer save-load metadata. Follow-up: recovery is DAG/route-linked, quiescence is session-backed, savepoints store and restore save-load state, and tests assert coherent save-load timeline restore.
- product/public boundary reviewer:
  found missing ownership for local/Docker transport command behavior and `load` command mismatch with `specs/25`. Follow-up: `specs/25`, `plan/50`, `tasks.md`, and progress docs now make P-A1-29 own product local/Docker transport plus viewer; CLI/docs use canonical `load <savepoint> --session <session>`.
- docs/source hierarchy reviewer:
  found stale progress/sample command summaries, incomplete consulted-doc trace, and stale `tasks.md` timestamp. Follow-up: progress, tasks, `samples/README.md`, `samples_progress.md`, scripts/sample docs, and this report were synchronized; consulted docs now include `samples/README.md`, `scripts/README.md`, and `plan/00-index.md`.
- security/auth/native-policy reviewer:
  found load could resurrect stale membership/witness/auth/capability state and save/load/quiescent-save stdout exposed full session internals. Follow-up: load-admissibility gates reject those stale frontiers; save/load/quiescent-save stdout now embeds observer-safe session summaries instead of full savepoint internals; local persisted session file remains the admin/debug local store and native execution remains disabled.

## Skipped validations and reasons

- Full workspace `cargo test` has not yet been run in this package because P-A1-28 touches product runtime/CLI/docs; focused `mir-runtime`, `mirrorea-cli`, and product schema tests are the package-specific floor before full release-candidate validation.
- Docker Compose transport validation was not run because product `transport --mode docker` remains unimplemented in P-A1-28.
- Product devtools viewer, native launch bundle, and release check commands were not expected to pass; they remain later-package obligations.
- Storage audit beyond `df -h .` and `free -h` was not run because no heavy artifact, external workdir, LLVM, Docker, or cleanup work was introduced.

## Commit / push status

- package commit:
  `09fae22` (`mirrorea: close P-A1-28 recovery quiescent save`)
- package push:
  pushed to `origin/main` (`6e7bf87..09fae22`).
- report metadata follow-up:
  required to record package commit / push status after the first commit.

## Sub-agent session close status

- code-mapping lanes:
  completed; findings were used to preserve product/practical carrier separation and target the minimal runtime/CLI/docs write set.
- review lanes:
  theory, checker/runtime, devtools/UX, product/public boundary, docs/source hierarchy, and security/auth/native-policy reviewers completed. Findings are summarized above and were fixed or explicitly assigned to P-A1-29/P-A1-30/P-A1-31 non-goal/next-gate work.
- remaining open sub-agent sessions:
  none for P-A1-28.
