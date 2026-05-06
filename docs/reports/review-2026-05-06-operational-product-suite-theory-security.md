# review-2026-05-06 operational product suite theory/security review

## 1. Title and identifier

- Title: operational product sample suite theory/security review
- Identifier: `review-2026-05-06-operational-product-suite-theory-security`

## 2. Objective

- Review the operational product sample suite work from a theory/security perspective, focused on `crates/mir-ast/src/product_alpha1.rs`, `crates/mir-runtime/src/product_alpha1_session.rs`, `samples/product-alpha1/operational/**`, `specs/26-operational-product-sample-suite.md`, and `specs/27-spatial-portal-and-shard-extension-boundary.md`.

## 3. Scope and assumptions

- Scope was limited to review; no product code changes were made.
- Review criteria emphasized:
  - no stdio builtin assumptions in Mir core
  - typed external host-I/O boundary
  - Place vs participant separation
  - explicit auth/rate-limit overlays
  - explicit quiescent-save obligations
  - portal/shard remaining future-boundary only
  - native bundle claims not overreaching
- Assumption: pre-existing uncommitted changes in the target files belong to the current package under review and were not reverted.

## 4. Start state / dirty state

- Branch: `feature/operational-product-sample-001`
- Dirty worktree existed before this review. Pre-existing modified/untracked files included the target implementation/spec/sample set and related docs/scripts/tests.
- Review-local generated state:
  - `.mirrorea-alpha/sessions/session_operational-sugoroku.f28932bddbd515ca.session.json`

## 5. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/21-auth-layer-algebra.md`
- `specs/23-typed-external-host-boundary.md`
- `specs/25-product-alpha1-public-boundary.md`
- `specs/26-operational-product-sample-suite.md`
- `specs/27-spatial-portal-and-shard-extension-boundary.md`
- `plan/51-operational-product-sample-roadmap.md`
- `plan/52-portal-spatial-world-roadmap.md`

## 6. Actions taken

- Read repository front-door and required base specs in AGENTS.md order.
- Inspected the target schema/runtime/sample files and the operational sample manifests.
- Ran targeted Rust tests for `mir-ast` and `mir-runtime` product-alpha1 rows.
- Ran targeted CLI probes for:
  - `run-local` on `samples/product-alpha1/operational/sugoroku-world`
  - `attach` on the operational auth layer
  - `quiescent-save` on the resulting session
  - `check` on the planned-only portal directory to confirm it does not present as runnable

## 7. Files changed

- Added this report:
  - `docs/reports/review-2026-05-06-operational-product-suite-theory-security.md`

## 8. Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' specs/00*.md specs/01*.md specs/02*.md specs/03*.md specs/09*.md specs/21-auth-layer-algebra.md specs/23-typed-external-host-boundary.md specs/25-product-alpha1-public-boundary.md specs/26-operational-product-sample-suite.md specs/27-spatial-portal-and-shard-extension-boundary.md
nl -ba crates/mir-ast/src/product_alpha1.rs
nl -ba crates/mir-runtime/src/product_alpha1_session.rs
nl -ba samples/product-alpha1/operational/**/*
cargo test -q -p mir-ast product_alpha1 -- --nocapture
cargo test -q -p mir-runtime product_alpha1 -- --nocapture
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/future/portal-worldlink --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/sugoroku-world --format json
cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/auth-layer --format json
cargo run -q -p mirrorea-cli -- quiescent-save 'session#operational-sugoroku' --savepoint savepoint#ops-r2 --format json
git status --short
git rev-parse --abbrev-ref HEAD
date '+%Y-%m-%d %H:%M:%S %z %Z'
```

## 9. Evidence / outputs / test results

- `cargo test -q -p mir-ast product_alpha1 -- --nocapture`
  - Passed targeted product-alpha1 tests.
- `cargo test -q -p mir-runtime product_alpha1 -- --nocapture`
  - Passed targeted product-alpha1 tests.
- `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/future/portal-worldlink --format json`
  - Returned `MissingPackageFile`, confirming the planned-only future portal directory does not present as a current runnable package root.
- `cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/sugoroku-world --format json`
  - Produced a session whose `bootstrap_membership` and active membership were exactly the declared `membership_requirements`.
- `cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/auth-layer --format json`
  - Accepted the auth layer based on `membership_present:active_admin_participant` and `auth_binding_present:admin_membership`.
- `cargo run -q -p mirrorea-cli -- quiescent-save 'session#operational-sugoroku' --savepoint savepoint#ops-r2 --format json`
  - Returned `terminal_outcome = saved`, `all_places_sealed = true`, `no_inflight = true`, `no_post_cut_send = true`, and a synthetic rejected post-cut send record.

## 10. What changed in understanding

- The package/spec wording is careful about non-claims, but the current runtime/session carrier still manufactures some of the security/semantics evidence it reports.
- The most important gaps are not doc overclaim. They are runtime-model shortcuts that make auth membership admission and R2 quiescent-save success largely tautological.

## 11. Open questions

- Should `membership_requirements` remain declarative policy refs only, with bootstrap participants supplied by a separate sample/session input?
- Should bounded R2 remain exposed as a successful command before the carrier records real seal state / in-flight state / post-cut send attempts instead of synthesizing them?
- Should observer-safe export remain a single surface if future package rows start using `admin_debug`?

## 12. Suggested next prompt

- Tighten `product_alpha1_session` so membership/auth evidence is not self-issued from package declarations, and make R2 quiescent-save prove or explicitly reject `NoInFlight`, `AllPlacesSealed`, and `NoPostCutSend` without synthetic success records.

## 13. `plan/` update status

- `plan/` 更新不要

## 14. `Documentation.md` update status

- `Documentation.md` 更新不要

## 15. `progress.md` update status

- `progress.md` 更新不要

## 16. `tasks.md` update status

- `tasks.md` 更新不要

## 17. `samples_progress.md` update status

- `samples_progress.md` 更新不要

## 18. reviewer findings and follow-up

- Finding 1:
  `product_alpha1_session` turns declared membership/auth requirements into active runtime principals and bindings, so attach-time admission is satisfied by construction rather than by external evidence. Anchors: `crates/mir-runtime/src/product_alpha1_session.rs:1016`, `crates/mir-runtime/src/product_alpha1_session.rs:1574`, `crates/mir-runtime/src/product_alpha1_session.rs:1804`, `crates/mir-runtime/src/product_alpha1_session.rs:1904`, `crates/mir-runtime/src/product_alpha1_session.rs:2106`, `samples/product-alpha1/operational/sugoroku-world/package.mir.json:34`.
- Finding 2:
  R2 quiescent-save success is manufactured by the carrier: it auto-seals every place, injects a synthetic post-cut send attempt, and treats the resulting rejection as proof of `NoPostCutSend`. Anchors: `crates/mir-runtime/src/product_alpha1_session.rs:824`, `crates/mir-runtime/src/product_alpha1_session.rs:834`, `crates/mir-runtime/src/product_alpha1_session.rs:857`, `crates/mir-runtime/src/product_alpha1_session.rs:886`, `crates/mir-runtime/tests/product_alpha1_session.rs:504`.
- Finding 3:
  `NoInFlight` is effectively vacuous on the default operational path because the session starts with only `Delivered`/terminal failure states; tests have to inject an `InFlight` record manually to exercise rejection. Anchors: `crates/mir-runtime/src/product_alpha1_session.rs:1048`, `crates/mir-runtime/src/product_alpha1_session.rs:1253`, `crates/mir-runtime/tests/product_alpha1_session.rs:557`.
- Follow-up:
  No additional reviewer pass was run. Findings are based on direct code/spec inspection plus targeted CLI/runtime evidence.

## 19. skipped validations and reasons

- Did not run the full release-check or Docker transport flow.
  - Reason: this review was scoped to theory/security invariants in the target files, and the core findings were reproducible without full environment-dependent flows.
- Did not run native bundle generation.
  - Reason: native bundle overclaim was reviewable from schema/spec/doc/runtime stop lines without building artifacts.

## 20. commit / push status

- No commit created.
- No push performed.

## 21. sub-agent session close status

- No sub-agent sessions were opened for this task.
