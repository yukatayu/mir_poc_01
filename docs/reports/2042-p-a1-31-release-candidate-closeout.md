# 2042 — P-A1-31 Release Candidate Closeout

## Objective

Close `P-A1-31` as the Product Alpha-1 clean-clone validation / release-candidate workflow package without claiming final public product completion.

## Scope and assumptions

- Scope: Rust `mirrorea-alpha` product demo command, release-check script, product demo package breadth, non-final viewer, local/Docker transport validation, native host launch bundle validation, clean-clone docs, and dashboards.
- Assumption: Docker / Docker Compose are available for the release-candidate path. `--skip-docker` is only a partial local probe and is not release-candidate evidence.
- Non-claims: final public grammar / ABI, WAN/federation, distributed durable save/load, durable audit backend, arbitrary native package execution, signature-is-safety, final public viewer / telemetry service.

## Start state / dirty state

- Start branch: `main`.
- Start state inherited closed `P-A1-30` commits and an untracked `sub-agent-pro/product-alpha1-001/` handoff directory.
- During validation, generated `.mirrorea-alpha/` local session output was removed before closeout.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
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
- `sub-agent-pro/product-alpha1-001/00-index.md` through `13-risk-register.md`
- latest prior numbered reports: `2039-p-a1-29-product-transport-viewer.md`, `2040-p-a1-30-native-launch-bundle.md`, `2041-p-a1-30-native-launch-bundle.md`

## Actions taken

- Added `mirrorea-alpha demo [package] --out <dir>` as the product alpha demo orchestrator.
- Added `scripts/product_alpha1_release_check.py check-all` with validation-floor commands, focused tests, command-family execution, native bundle run script probes, and JSON payload semantic checks.
- Added `docs/hands_on/product_alpha1_01.md` and `docs/research_abstract/product_alpha1_01.md`.
- Expanded `samples/product-alpha1/demo/` with auth, rate-limit, placeholder-object, and custom-avatar-preview packages.
- Moved debug/auth/rate-limit authority from hidden runtime bootstrap grants into explicit demo source declarations.
- Added demo attach-matrix verification for debug/auth/rate-limit accepted layer attach and object/avatar-preview deferred boundary evidence.
- Split demo output into observer-safe `sessions/` artifacts and admin/debug `session-store/` replay artifacts, and verified same-session reopenability.
- Made Docker fixture tokens per-run and fixed Docker Compose cleanup so fixture containers/networks are removed after validation.
- Expanded the non-final viewer from panel-name scaffolding to concrete observer-safe JSON panel rendering.
- Updated docs, validators, and dashboards for the new product docs and sample subtree.

## Files changed

- `crates/mirrorea-cli/src/main.rs`
- `crates/mirrorea-cli/tests/alpha_cli.rs`
- `crates/mir-runtime/src/product_alpha1_session.rs`
- `crates/mir-runtime/src/product_alpha1_devtools.rs`
- `crates/mir-runtime/tests/product_alpha1_session.rs`
- `crates/mir-runtime/tests/product_alpha1_transport_devtools.rs`
- `crates/mir-ast/src/product_alpha1.rs`
- `crates/mir-ast/tests/practical_alpha1_front_door.rs`
- `scripts/product_alpha1_release_check.py`
- `scripts/tests/test_product_alpha1_release_check.py`
- `samples/product-alpha1/demo/package.mir.json`
- `samples/product-alpha1/demo/packages/auth-layer/package.mir.json`
- `samples/product-alpha1/demo/packages/rate-limit-layer/package.mir.json`
- `samples/product-alpha1/demo/packages/placeholder-object/package.mir.json`
- `samples/product-alpha1/demo/packages/custom-avatar-preview/package.mir.json`
- `samples/product-alpha1/docker/docker-compose.product-alpha1.yml`
- `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`
- `samples/README.md`, `samples/product-alpha1/README.md`, `samples/product-alpha1/demo/README.md`, `samples/product-alpha1/docker/README.md`
- `docs/hands_on/README.md`, `docs/hands_on/product_alpha1_01.md`
- `docs/research_abstract/README.md`, `docs/research_abstract/product_alpha1_01.md`
- `scripts/README.md`, `scripts/check_source_hierarchy.py`, `scripts/validate_docs.py`
- `specs/00-document-map.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `sub-agent-pro/product-alpha1-001/*`

## Commands run

```bash
python3 -m unittest scripts.tests.test_product_alpha1_release_check
cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
cargo run -q -p mirrorea-cli -- demo samples/product-alpha1/demo --out <tmp> --skip-docker --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out <tmp> --skip-docker
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo test -p mir-ast -- --nocapture
cargo test -p mir-runtime -- --nocapture
cargo test -p mirrorea-core -- --nocapture
cargo test -p mirrorea-cli -- --nocapture
cargo run -q -p mirrorea-cli -- demo samples/product-alpha1/demo --out <tmp> --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out <tmp>
cargo fmt --check
git diff --check
docker ps -a --format '{{.Names}}' | rg '^mirrorea-product-a1-' || true
docker network ls --format '{{.Name}}' | rg '^mirrorea-product-a1-' || true
```

Disposable Docker fixture cleanup commands were also run for stale `mirrorea-product-a1-*` containers/networks left by earlier failed probes:

```bash
docker network ls --format '{{.Name}}' | rg '^mirrorea-product-a1-' | xargs -r docker network rm
docker ps -a --format '{{.Names}}' | rg '^mirrorea-product-a1-' | xargs -r docker rm
```

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_validate_docs`: pass, 13 tests.
- `python3 scripts/check_source_hierarchy.py`: pass, required `105`, present `105`.
- `python3 scripts/validate_docs.py`: pass, `1194` numbered reports including this report.
- `cargo test -p mir-ast -- --nocapture`: pass.
- `cargo test -p mir-runtime -- --nocapture`: pass.
- `cargo test -p mirrorea-core -- --nocapture`: pass.
- `cargo test -p mirrorea-cli -- --nocapture`: pass, `alpha_cli` 19 tests.
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-final-TYv85Q`: pass with `status = accepted`, `product_alpha1_release_candidate_ready = true`, `failed_commands = []`, `planned_commands = 29`.
- `mirrorea-alpha demo ... --skip-docker`: pass as `status = partial`, `product_alpha1_release_candidate_ready = false`.
- Full Docker demo after cleanup fix: pass with `status = accepted`, `docker_transport_status = accepted`, and zero remaining `mirrorea-product-a1-*` containers/networks.

## What changed in understanding

- Product alpha release-candidate readiness must include Docker Compose TCP; skip paths are useful local probes but not release evidence.
- The demo output has two authority classes: observer-safe artifacts for inspection and an admin/debug local session store for same-session replay.
- Debug/auth/rate-limit attach evidence must be source-backed; hidden bootstrap authority is not acceptable under the auth layer algebra.
- A viewer that only lists panel names is not enough for clean-clone inspection; the non-final viewer must render concrete observer-safe records.

## Open questions

- Final public grammar / ABI remains a future gate.
- Full admin/debug viewer semantics remain kept-later.
- Durable audit backend and distributed durable save/load R3/R4 remain non-goals for this package.
- Accepted detach execution remains outside this product alpha release candidate.

## Suggested next prompt

Open a new final-public gate scoping package, or run a maintenance package that audits docs/source hierarchy and stale generated Docker/session artifacts after P-A1-31.

## Plan update status

`plan/50-product-alpha1-public-boundary-roadmap.md` updated for P-A1-31 closeout, skip-docker non-readiness, source-backed attach authority, observer-safe/admin session split, and release-check validation semantics.

## Documentation.md update status

`Documentation.md` updated to point to the product alpha hands-on guide and to classify the remaining gap as final-public hardening rather than product alpha release validation.

## progress.md update status

`progress.md` updated with `2026-05-05 17:48 JST` snapshot and review-hardening log.

## tasks.md update status

`tasks.md` updated with `2026-05-05 17:48 JST` snapshot, P-A1-31 source-backed/deferred boundary wording, skip-docker partial status, and final-public reopen recommendation.

## samples_progress.md update status

`samples_progress.md` updated with `2026-05-05 17:48 JST` snapshot, product-release-candidate row details, and review-hardening validation log.

## Reviewer findings and follow-up

- Theory reviewer: found hidden admin/capability bootstrap authority and skip-docker overclaim. Fixed by moving authority into demo source declarations and downgrading skip-docker to partial non-release.
- Checker/runtime reviewer: found demo session output was not reusable by `session`, skip-docker overclaim, weak payload validation, and stale output risk. Fixed canonical admin session store, same-session reopen evidence, semantic release-check validation, and non-empty output preflight.
- Devtools/UX reviewer: found viewer scaffold did not render concrete product state and redaction wording was too strong. Fixed concrete JSON panel rendering and changed demo redaction claim to bounded/limited check, not complete proof.
- Product/public boundary reviewer: found skip-docker readiness overclaim, release-check floor omission, and missing attach matrix verification. Fixed all three.
- Docs/source hierarchy reviewer: found missing report, source hierarchy drift, stale docs, stale timestamps, and unenforced sample subtree. This report closes the report gap; docs and validators were updated.
- Security/auth/native-policy reviewer: found raw full-session demo artifact ambiguity, bypassable fixed Docker helper token, weak redaction attestation, and stale-output fragility. Fixed observer-safe/admin session split, per-run fixture token, bounded redaction wording, output preflight, and Docker cleanup env.

## Skipped validations and reasons

None for the P-A1-31 required floor. Final public grammar / ABI, WAN/federation, distributed durable save/load, durable audit, arbitrary native package execution, and final viewer telemetry were not run because they are explicit non-goals.

## Commit / push status

- Implementation closeout commit: `afa6269` (`mirrorea: close P-A1-31 release candidate`), pushed to `main`.
- Report metadata follow-up: pending at this report update; final hash is recorded in the user-facing closeout answer to avoid a self-referential metadata loop.

## Sub-agent session close status

All six requested reviewer perspectives completed and were incorporated. After resume, stored P-A1-31 reviewer agent IDs were no longer addressable by `close_agent`; the two remaining visible inherited agent sessions were closed. No active reviewer session is intentionally retained.
