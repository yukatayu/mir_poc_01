# 2107 P-SURF-07 Source Operational Suite

- Date: 2026-05-24
- Author / agent: Codex
- Scope: Surface Mir P-SURF-07 source operational suite
- Decision levels touched: L2 alpha implementation evidence, L3 final runtime / transport / catalog non-claim

## Objective

- Identifier: `P-SURF-07 source operational suite`
- Package: Surface Mir brace complete autonomous implementation
- Report path: `docs/reports/2107-p-surf-07-source-operational-suite.md`

Create the source-first Surface Mir operational evidence roots for WorldCore, MembershipChat, SugorokuWorld, PortalWorldlink, TwoShardHardBoundary, and GradientObservation using canonical `S { ... }` syntax and `.mir` source authority. Close the package with positive and negative rows, helper checks, documentation updates, reviewer follow-up, validation, commit, and push.

## Scope and assumptions

- `.mir` files remain semantic source authority; `package.mir.json` remains an alpha artifact.
- Canonical Surface Mir place-scope syntax remains `S { ... }`; `S[ ... ]` remains rejected and is not sugar.
- P-SURF-07 is source-first alpha operational evidence. It does not claim final Surface runtime/transport, final shared-space catalog, final devtools, final ABI/SDK, production identity, WAN/federation, arbitrary native/WASM execution, or distributed durable save-load R3/R4.
- Operational rows are representative evidence rows from the P-SURF matrix, not a complete product catalog.
- Role claims are not authority; admission grants are the authority source. Indexed-state keys are not authority.

## Start state / dirty state

Started from pushed P-SURF-06 closeout on branch `main` at `93b8ee34f3b58a7080cd42fc65c22d00dfde4542`. The worktree contained the untracked handoff directory `sub-agent-pro/surface-mir-brace-completion-001/`, which was intentionally left untracked and unstaged. P-SURF-07 edits were local and uncommitted at report creation.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/39-surface-mir-placement-elaboration.md`
- `specs/40-indexed-state-semantics.md`
- `specs/41-role-admission-and-capability-grant.md`
- `specs/42-source-patch-hotplug-semantics.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/64-surface-mir-placement-roadmap.md`
- `plan/65-indexed-state-roadmap.md`
- `plan/66-role-admission-roadmap.md`
- `plan/67-source-patch-hotplug-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`
- `sub-agent-pro/surface-mir-brace-completion-001/*.md`
- `sub-agent-pro/surface-mir-brace-completion-001/sample-blueprints/*.md`
- Reviewer findings from sub-agent `019e5998-683c-7df1-8bec-d8384f9ea983`

## Actions taken

- Added `samples/full-system-v1-surface/operational-matrix.json` with `E2E-SURF-01..12`.
- Added six Surface source operational roots: `world-core/`, `membership-chat/`, `sugoroku-world/`, `portal-worldlink/`, `two-shard-hard-boundary/`, and `gradient-observation/`.
- Added positive and negative `.mir` source rows with expected `operational.json` projections under each root.
- Extended `scripts/surface_mir_samples.py` with the `operational_source` runner and per-row required checks across parser, indexed-state, role-admission, and elaboration payloads.
- Made operational projection derive `source_authority` and `final_public_api_frozen` from lower semantic payloads instead of hardcoding them.
- Let Surface-to-Core elaboration retain `join` as a `surface_role_join_admission` transition after the P-SURF-05 admission floor, while leaving unsupported manual `publish` statements rejected.
- Updated MembershipChat rows so the positive row runs role admission plus elaboration/generated communication and the negative row records both missing grant and generated failure-row evidence.
- Updated source hierarchy / docs validators, sample helper tests, release-check tests, and status docs for the 44-row Surface matrix.

## Files changed

- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/surface_mir_samples.py`
- `scripts/surface_mir_release_check.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/tests/test_surface_mir_release_check.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `samples/full-system-v1-surface/operational-matrix.json`
- `samples/full-system-v1-surface/{world-core,membership-chat,sugoroku-world,portal-worldlink,two-shard-hard-boundary,gradient-observation}/**`
- `samples/full-system-v1-surface/elaboration/elab-06-unsupported-statement-negative/main/src/unsupported-statement-negative.mir`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/full-system-v1-surface/README.md`
- `scripts/README.md`
- `docs/hands_on/surface_mir_alpha_01.md`
- `docs/research_abstract/surface_mir_alpha_01.md`
- `specs/00-document-map.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/00-index.md`
- `plan/66-role-admission-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`

## Commands run

- `date '+%Y-%m-%d %H:%M %Z'`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `cargo fmt`
- `cargo fmt --check`
- `git diff --check`
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`
- `python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check`
- `python3 scripts/surface_mir_samples.py check-all --format json`
- `python3 scripts/surface_mir_authoring_check.py check-all --format json`
- `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-p-surf-07`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-07`
- `python3 scripts/operational_product_samples.py check-all --format json`
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json`
- `python3 crates?` failed as a mistyped local inspection command and made no repo change.
- A first `jq` post-filter over `surface_mir_release_check.py` failed because the filter expected a different JSON shape; the release check was rerun and summarized from `/tmp/mirrorea-surface-release-p-surf-07.json`.

## Evidence / outputs / test results

- `scripts.tests.test_validate_docs`: 18 tests passed.
- `scripts/check_source_hierarchy.py`: 537 required paths present, 0 missing.
- `scripts/validate_docs.py`: documentation scaffold complete; 1259 numbered reports expected after this report is added.
- `cargo fmt --check`: passed after applying `cargo fmt`.
- `git diff --check`: passed.
- `mir-semantics surface_to_core_elaboration`: 14 tests passed, including `surface_role_join_admission` transition retention.
- Surface helper unit tests: 41 tests passed across `scripts.tests.test_surface_mir_samples` and `scripts.tests.test_surface_mir_release_check`.
- `scripts/surface_mir_samples.py check-all`: 44 rows passed, failed `[]`, workflow-ready `false`.
- `scripts/surface_mir_authoring_check.py check-all`: 44 sources accepted as `.mir` authority.
- `scripts/surface_mir_release_check.py`: scope `p_surf_07_source_operational_suite`, 15 checks passed, failed `[]`.
- Surface release check also ran and passed `cargo fmt --check`, `mir-ast surface_mir_parser`, `mir-semantics indexed_state_semantics`, `mir-semantics surface_to_core_elaboration`, `mir-semantics role_admission_capability_grant`, `mir-runtime source_patch_hotplug`, `mirrorea-cli surface_mir_cli`, Surface sample helper, and Surface authoring helper.
- `scripts/product_alpha1_release_check.py`: status `accepted`, failed commands `[]`.
- `scripts/operational_product_samples.py`: status `accepted`, failed commands `[]`.
- `scripts/minimal_alpha1_patterns.py`: status `accepted`, failed `[]`.

## What changed in understanding

P-SURF-07 needs the operational runner to compose existing alpha checks rather than merely collect row metadata. MembershipChat exposed that split: a row can pass role admission while still missing generated communication evidence unless elaboration also runs. After P-SURF-05, `join` can be retained as a Core IR transition for observability without making the role claim an authority source; the role-admission checker remains the owner of grant semantics.

## Open questions

- Final Surface operational runtime/transport remains open.
- Final Surface devtools / diagnostics suite is the next package, not closed here.
- Indexed-state runtime carrier, runtime MessageEnvelope dispatch, and production identity/admission lifecycle remain later.
- Surface PoseGraph / projection / engine-provider rows remain later than this operational source evidence floor.
- Final public grammar, ABI, SDK, shared-space catalog breadth, and WAN/federation remain non-claims.

## Suggested next prompt

`P-SURF-08 devtools and diagnostics`

## Plan update status

Updated `plan/00-index.md`, `plan/66-role-admission-roadmap.md`, and `plan/68-surface-full-system-v1-roadmap.md` to mark P-SURF-07 as closed source operational evidence, remove stale "not yet" wording for role/source-patch/operational evidence, and make P-SURF-08 the next promoted package.

## Documentation.md update status

Updated. `Documentation.md` now records P-SURF-07 as the six-root source operational evidence floor with 44 Surface source rows and keeps final runtime/transport/devtools/ABI/SDK as non-claims.

## progress.md update status

Updated. `progress.md` records P-SURF-07 closure, current runnable commands, implemented roots, current non-claims, reviewer follow-up for MembershipChat and derived source authority fields, and next gap `P-SURF-08 devtools and diagnostics`.

## tasks.md update status

Updated. `tasks.md` now makes `P-SURF-08 devtools and diagnostics` the current promoted autonomous package while preserving P-SURF-07 as alpha source operational evidence only.

## samples_progress.md update status

Updated. `samples_progress.md` records the six P-SURF-07 operational roots, `E2E-SURF-01..12`, 44 Surface rows, MembershipChat role-admission plus elaboration coverage, and the non-claim that this is not final runtime/transport evidence.

## Reviewer findings and follow-up

Reviewer sub-agent `019e5998-683c-7df1-8bec-d8384f9ea983` reported:

- `operational_source` hardcoded `source_authority` and `final_public_api_frozen` instead of deriving them from lower payloads.
- MembershipChat rows only ran parse plus role admission and therefore did not catch generated communication regressions.
- Status docs still contained stale pre-P-SURF-07 underclaims.

Follow-up implemented:

- Operational projection now derives `source_authority` and `final_public_api_frozen` from semantic checker payloads and reports conflicts as mismatches.
- MembershipChat positive now runs parser, role admission, and elaboration/generated communication; the negative row also includes elaboration failure-row evidence.
- Surface-to-Core elaboration now retains `join` as `surface_role_join_admission` transition while manual `publish` remains the unsupported-statement negative row.
- Stale status wording in `plan/68`, `progress.md`, `samples_progress.md`, and `scripts/README.md` was updated.

## Skipped validations and reasons

No requested P-SURF-07 validation was skipped. Whole-workspace `cargo test` was not run because the package close validation uses the focused Surface release check plus compatibility anchors requested for this package.

## Commit / push status

Pending at report creation. The intended commit message is `p-surf-07: add source operational suite`; final commit hash and push status are reported in the package close response.

## Sub-agent session close status

- `019e5998-683c-7df1-8bec-d8384f9ea983`: completed reviewer pass, findings were addressed, and the session was closed.
