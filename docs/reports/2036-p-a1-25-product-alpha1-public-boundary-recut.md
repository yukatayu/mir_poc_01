# 2036 — P-A1-25 product/public alpha-1 boundary recut

Identifier: `P-A1-25`

## Objective

Product/Public-ready Mirrorea Spaces alpha-1 の promoted line を開くため、bounded practical alpha-1 workflow と product/public-ready alpha-1 を分け直し、alpha `U1` defaults、package sequence、CLI / package / runtime / transport / save/load / devtools / native bundle / release validation の境界を `specs/` と `plan/` に固定する。

## Scope and assumptions

- Scope is documentation, repository-memory, and scaffold validation only.
- This package does not implement the product CLI, product demo root, message recovery runtime, quiescent-save runtime, product viewer, or native launch bundle.
- The handoff under `sub-agent-pro/product-alpha1-001/` was treated as working directive / handoff input, not as normative source and not as a file to commit.
- Product alpha-1 is now boundary-fixed, but not workflow-ready and not product-ready.
- Final textual `.mir` grammar, final public ABI, WAN/federation, distributed durable save/load, arbitrary native execution, full engine / avatar compatibility, PrismCascade integration, and Reversed Library remain non-goals.

## Start state / dirty state

- start branch:
  `main`
- start tracked state:
  clean tracked worktree on latest pushed main before this package's edits.
- start untracked state:
  `sub-agent-pro/product-alpha1-001/` was present as handoff input and remains untracked.
- resource posture:
  no heavy build artifacts, LLVM work, generated corpora, external workdir writes, or cleanup operations were introduced.

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
- `plan/00-index.md`
- `plan/44-practical-alpha1-roadmap.md`
- `plan/45-operational-alpha05-roadmap.md`
- `plan/46-operational-alpha08-roadmap.md`
- `plan/47-operational-alpha09-devtools-roadmap.md`
- `plan/48-theory-freeze-proof-obligations.md`
- `plan/49-host-io-and-session-runtime-roadmap.md`
- `samples/practical-alpha1/README.md`
- `tmp_faq/faq_015.md`
- `docs/reports/2033-p-a1-23-practical-alpha1-integrated-workflow-carrier.md`
- `docs/reports/2034-p-a1-24-workflow-readiness-metric-recut.md`
- `docs/reports/2035-root-md-concision-operational-verification.md`
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

1. Recorded the Discord task baseline before edits.
2. Read the required root docs, snapshot docs, normative specs, roadmap memory, latest reports, practical alpha sample README, FAQ, and product alpha handoff.
3. Added a failing documentation-scaffold test requiring the product alpha boundary spec and roadmap to be registered in both `validate_docs` and source-hierarchy checks.
4. Added `specs/25-product-alpha1-public-boundary.md` as the normative product/public alpha-1 boundary.
5. Added `plan/50-product-alpha1-public-boundary-roadmap.md` as repository memory for `P-A1-25..31`.
6. Updated source hierarchy and documentation scaffold validators to require `specs/25` and `plan/50`.
7. Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `specs/00`, `specs/18`, `specs/24`, `plan/00`, `plan/44`, and `scripts/README.md` to preserve the new boundary split.
8. Spawned six reviewer lanes requested by the user: theory, checker/runtime, devtools/UX, product/public boundary, docs/source hierarchy, and security/auth/native policy.
9. Applied reviewer-driven fixes for R1/R2 semantics, checker/runtime split, CLI command semantics, local/Docker transport coverage, release validation breadth, viewer redaction roles, auth/native-policy stop lines, dashboard status vocabulary, and stale blocker wording.
10. Closed all sub-agent sessions after their findings were integrated or recorded.
11. Ran the package validation floor before adding this report.
12. Added this report for the package closeout.
13. Reran the package validation floor after adding this report so `validate_docs.py` checked the latest numbered report.

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `specs/00-document-map.md`
- `specs/18-practical-alpha1-scope.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `specs/25-product-alpha1-public-boundary.md`
- `plan/00-index.md`
- `plan/44-practical-alpha1-roadmap.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `docs/reports/2036-p-a1-25-product-alpha1-public-boundary-recut.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
date '+%Y-%m-%d %H:%M:%S %Z'
sed -n '1,220p' README.md
sed -n '1,220p' Documentation.md
sed -n '1,220p' progress.md
sed -n '1,220p' tasks.md
sed -n '1,220p' samples_progress.md
sed -n '1,260p' specs/00-document-map.md
sed -n '1,240p' specs/01-charter-and-decision-levels.md
sed -n '1,240p' specs/02-system-overview.md
sed -n '1,240p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/18-practical-alpha1-scope.md
sed -n '1,260p' specs/19-verification-stratification.md
sed -n '1,260p' specs/20-cut-save-load-semantics.md
sed -n '1,260p' specs/21-auth-layer-algebra.md
sed -n '1,260p' specs/22-observability-devtools-semantics.md
sed -n '1,260p' specs/23-typed-external-host-boundary.md
sed -n '1,260p' specs/24-operational-alpha05-alpha08-readiness.md
sed -n '1,260p' plan/44-practical-alpha1-roadmap.md
sed -n '1,260p' plan/45-operational-alpha05-roadmap.md
sed -n '1,260p' plan/46-operational-alpha08-roadmap.md
sed -n '1,260p' plan/47-operational-alpha09-devtools-roadmap.md
sed -n '1,260p' plan/48-theory-freeze-proof-obligations.md
sed -n '1,260p' plan/49-host-io-and-session-runtime-roadmap.md
sed -n '1,220p' samples/practical-alpha1/README.md
sed -n '1,220p' tmp_faq/faq_015.md
find docs/reports -maxdepth 1 -type f -name '[0-9][0-9][0-9][0-9]-*.md' -printf '%f\n' | sort -V | tail -n 20
sed -n '1,260p' docs/reports/2033-p-a1-23-practical-alpha1-integrated-workflow-carrier.md
sed -n '1,260p' docs/reports/2034-p-a1-24-workflow-readiness-metric-recut.md
sed -n '1,260p' docs/reports/2035-root-md-concision-operational-verification.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/00-index.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/01-current-state-and-gap.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/02-product-alpha1-definition.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/03-architecture-and-repo-plan.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/04-theory-freeze-requirements.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/05-message-recovery-and-savepoint.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/06-auth-and-layer-algebra.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/07-devtools-ux-design.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/08-native-package-and-host-policy.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/09-sample-and-validation-matrix.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/10-package-sequence.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/11-subagent-review-plan.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/12-closeout-protocol.md
sed -n '1,220p' sub-agent-pro/product-alpha1-001/13-risk-register.md
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
rg -n 'check -> runtime plan -> run-local -> typed host-I/O -> hot-plug -> observe|product/public readiness remains not ready until public boundary is fixed|specs/18\.\.24|plan/39\.\.49|product-ready α-1 は引き続き未固定' README.md Documentation.md progress.md tasks.md samples_progress.md specs/24-operational-alpha05-alpha08-readiness.md plan/50-product-alpha1-public-boundary-roadmap.md
git diff --stat
git diff -- README.md Documentation.md progress.md tasks.md samples_progress.md
git diff -- specs/00-document-map.md specs/18-practical-alpha1-scope.md specs/24-operational-alpha05-alpha08-readiness.md plan/00-index.md plan/44-practical-alpha1-roadmap.md
git diff -- scripts/tests/test_validate_docs.py scripts/validate_docs.py scripts/check_source_hierarchy.py scripts/README.md
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
git status --short
git add README.md Documentation.md progress.md tasks.md samples_progress.md scripts/README.md scripts/check_source_hierarchy.py scripts/tests/test_validate_docs.py scripts/validate_docs.py specs/00-document-map.md specs/18-practical-alpha1-scope.md specs/24-operational-alpha05-alpha08-readiness.md specs/25-product-alpha1-public-boundary.md plan/00-index.md plan/44-practical-alpha1-roadmap.md plan/50-product-alpha1-public-boundary-roadmap.md docs/reports/2036-p-a1-25-product-alpha1-public-boundary-recut.md
git commit --no-gpg-sign -m "mirrorea: close P-A1-25 product alpha boundary"
git push
```

## Evidence / outputs / test results

- The intentional RED test run of `python3 -m unittest scripts.tests.test_validate_docs` failed before `specs/25` and `plan/50` were registered, proving the new scaffold guard was active.
- After adding the required docs and registries, `python3 -m unittest scripts.tests.test_validate_docs` passed:
  `Ran 12 tests in 0.046s`, `OK`.
- `python3 scripts/check_source_hierarchy.py` passed:
  `required: 86`, `present: 86`, `missing: 0`.
- `python3 scripts/validate_docs.py` passed before this report was added:
  `Documentation scaffold looks complete. Found 1187 numbered report(s).`
- `cargo fmt --check` passed.
- `git diff --check` passed.
- The stale-wording search for pre-recut product/public boundary phrases returned no matches.
- Post-report validation also passed:
  - `python3 -m unittest scripts.tests.test_validate_docs`:
    `Ran 12 tests in 0.084s`, `OK`.
  - `python3 scripts/check_source_hierarchy.py`:
    `required: 86`, `present: 86`, `missing: 0`.
  - `python3 scripts/validate_docs.py`:
    `Documentation scaffold looks complete. Found 1188 numbered report(s).`
  - `cargo fmt --check` passed.
  - `git diff --check` passed.
- package commit succeeded:
  `2660ce8` `mirrorea: close P-A1-25 product alpha boundary`.
- package push succeeded:
  `main -> main` on `origin`.

## What changed in understanding

- `U1` is no longer an unresolved alpha-blocker for this promoted product line: alpha defaults now choose a Rust CLI / binary entrypoint, versioned `package.mir.json`, native process in local/Docker scope, non-final viewer, and native host launch bundle.
- Product alpha-1 is distinct from bounded practical alpha-1 workflow. The latter is workflow-ready evidence; the former still needs implementation and release validation.
- The product alpha-1 checker must stay split from runtime/preflight/model-check/proof obligations. Finite package facts are Line 1 checker work; stateful recovery and quiescent-save claims require runtime and later-line evidence.
- `R2 QuiescentSavePoint` is not just three boolean flags. It is the implemented R1/load-admissibility subset plus quiescence evidence: `NoInFlight`, `AllPlacesSealed`, and `NoPostCutSend`.
- Product alpha-1 native output means host launch bundle, not direct Mir-to-machine-code and not arbitrary native package execution.

## Open questions

- Final public grammar and final public ABI remain undecided.
- Production WAN/federation and distributed durable save/load remain outside the product alpha-1 line.
- Accepted detach execution may remain deferred unless a later package implements it explicitly.
- The exact breadth of bounded admin/debug viewer implementation versus explicit `kept_later` remains a `P-A1-29` decision.
- The precise finite-fragment breadth of the product checker remains a `P-A1-26..28` implementation choice, bounded by `specs/25`.

## Suggested next prompt

Proceed with `P-A1-26` alpha CLI / package schema stabilization: add the canonical alpha Rust CLI command family, versioned product alpha package schema, compatibility policy, explicit unsupported diagnostics, focused tests, docs updates, report, commit, and push.

## Plan update status

- updated:
  `plan/00-index.md` now includes product alpha-1 public boundary memory.
- updated:
  `plan/44-practical-alpha1-roadmap.md` now points product/public alpha-1 boundary work to `specs/25` / `plan/50` and keeps practical alpha-1 as first-floor / bounded workflow evidence.
- added:
  `plan/50-product-alpha1-public-boundary-roadmap.md` records product alpha-1 alpha defaults, package sequence, validation direction, sample-root target, native-bundle target, blocker split, and next reopen point.

## Documentation.md update status

- updated:
  `Documentation.md` now separates repo-local alpha-ready current layer, current-scope evidence, practical alpha-1 first-floor evidence, operational alpha readiness, product/public alpha-1, and final public product.
- updated:
  it now points to `specs/25` / `plan/50` and mirrors the major non-goals for product alpha-1.

## progress.md update status

- updated:
  `progress.md` now records `P-A1-25` as the latest closeout, `P-A1-26` as the next promoted reopen point, and product alpha-1 as boundary-fixed but not workflow-ready.
- updated:
  current blockers now include canonical CLI, versioned product package schema, product demo root, local/Docker transport command path, message failure/recovery, bounded R2 quiescent-save, product viewer, native launch bundle, and clean-clone validation.
- updated:
  recent log contains a dated entry for this package.

## tasks.md update status

- updated:
  `tasks.md` now moves the self-driven queue to `P-A1-26..31`.
- updated:
  task rows distinguish product CLI/schema, same-session product runtime, message recovery/quiescent-save, viewer UX, native launch bundle, and release-candidate validation.
- updated:
  blocker rows keep final public grammar / ABI / WAN / distributed durable save-load as later user/final decisions and narrow product alpha-1 to a showcase rather than a broad final catalog.

## samples_progress.md update status

- updated:
  `samples_progress.md` now adds `boundary-fixed` as an explicit dashboard status.
- updated:
  the product alpha-1 public boundary row is documented as docs-only, not workflow-ready, and missing canonical CLI / product demo root / quiescent-save implementation / product viewer / native bundle / release validation.
- updated:
  validation log records that behavior implementation was not changed.

## Reviewer findings and follow-up

- theory reviewer:
  found that R2 quiescent-save was too narrow, the checker/runtime split was blurred, and `specs/24` still pointed to "public boundary fixed" as the blocker. Follow-up was applied by tying R1 to `Consistent(cut)`, defining R2 as R1/load-admissibility subset plus quiescence evidence, splitting Line 1 checker facts from runtime/preflight/model/proof obligations, requiring message/recovery transitions to live in event DAG or linked carriers, and updating `specs/24`.
- checker/runtime reviewer:
  found that transport was missing from the planned same-session demo and release-validation path, and that `progress.md` understated message recovery / quiescent-save work. Follow-up was applied by adding local/Docker transport product command paths to `specs/25`, `plan/50`, `tasks.md`, and `progress.md`. Semantic linting beyond scaffold registration remains a residual improvement, not part of this docs-only package.
- devtools/UX reviewer:
  found that viewer role/redaction/admin-kept-later behavior was not testable enough, root docs under-mirrored non-goals, and the sample dashboard title over-read as operational readiness. Follow-up was applied by requiring active view role, redaction level, retention scope, admin/debug `kept_later` status, source records, and clearer dashboard naming.
- product/public boundary reviewer:
  found that CLI semantics and release validation did not cover the full public-ish command family. Follow-up was applied by adding minimum semantics for `check`, `run-local`, `session`, `attach`, `transport`, `save`, `load`, `quiescent-save`, `export-devtools`, `view`, `build-native-bundle`, and `demo`, plus release validation coverage for the full family.
- docs/source-hierarchy reviewer:
  found that the package needed a numbered report, the `boundary-fixed` dashboard status needed a legend entry, and a stale final shared-space catalog blocker needed recutting. Follow-up was applied by adding this report, adding the dashboard legend, and recutting final catalog breadth as a post-product-alpha user/final decision.
- security/auth/native-policy reviewer:
  found that native policy validation was too weak, auth was not first-class enough in schema/session/viewer lanes, and root docs could over-read native bundle as native execution. Follow-up was applied by adding `auth_policy` / `auth_stack`, separating auth and capability summaries, strengthening native `NativeExecutionPolicy = Disabled` / arbitrary execution / signature-is-not-safety stop lines, and mirroring those caveats in root/task docs.

## Skipped validations and reasons

- Product CLI behavior tests were skipped because `P-A1-25` defines the boundary only; the CLI is scheduled for `P-A1-26`.
- Runtime behavior tests were skipped because no Rust runtime behavior was changed.
- Product demo, local/Docker transport, quiescent-save, viewer, native bundle, and full release-candidate commands were skipped because those implementations are explicitly scheduled for `P-A1-27..31`.
- Heavy storage audit was skipped because this package did not add heavy build artifacts, generated corpora, external workdir usage, or cleanup operations.
- Semantic lints for overclaim phrases were not added beyond the focused stale-wording search and reviewer pass; this package added scaffold guards for the new boundary documents, while semantic claim enforcement remains a future maintenance improvement.

## Commit / push status

- package commit:
  `2660ce8` `mirrorea: close P-A1-25 product alpha boundary`
- push:
  pushed to `origin/main`.
- report metadata follow-up:
  this section was updated after the package commit and will be committed / pushed as docs-only metadata.

## Sub-agent session close status

- theory reviewer `Mencius`:
  completed and closed.
- checker/runtime reviewer `Herschel`:
  completed and closed.
- devtools/UX reviewer `Lagrange`:
  completed and closed.
- product/public boundary reviewer `Gauss`:
  completed and closed.
- docs/source-hierarchy reviewer `Galileo`:
  completed and closed.
- security/auth/native-policy reviewer `Schrodinger`:
  completed and closed.
