# Report 2058 — P-OPS-15 gradient observation runtime first cut

- Date: 2026-05-07 09:08 JST
- Author / agent: Codex
- Scope: operational shard/gradient runtime widening, freshness/runtime evidence hardening, docs/source hierarchy sync
- Decision levels touched: `L1` operational suite executable-root surface and shard/gradient boundary wording sync, `L2` roadmap/dashboard/package queue sync

## Objective

Close `P-OPS-15` by adding a separate runnable `two-shard-gradient-observation/` operational root, carrying observer-only gradient freshness/runtime evidence through session/devtools/helper surfaces, keeping `future/gradient-observation.profile.json` non-executable, and synchronizing the full source hierarchy plus validation floor around that new bounded runtime.

## Scope and assumptions

- Scope includes:
  - new operational root `samples/product-alpha1/operational/two-shard-gradient-observation/`
  - `mir-ast` package schema acceptance for `two_shard_gradient_observation`
  - `mir-runtime` session/devtools/runtime evidence for bounded observer-only gradient lanes
  - `mirrorea-cli` operational-package recognition
  - `scripts/operational_product_samples.py` orchestration / semantic checks / release-check wiring
  - expected JSON, future profile pairing, reader-facing guides, roadmap memory, and dashboards
  - focused semantic review follow-up and full validation rerun
- Scope excludes:
  - continuous spatial synchronization
  - write-authority overlap runtime
  - WAN / federation
  - distributed durable save/load beyond existing local `R0/R2`
  - final public grammar / ABI / viewer / portal ABI
- Assumptions:
  - the hard-authority shard cut from `P-OPS-07` remains the authority baseline
  - the correct first runtime shape is a separate runnable root, not an implicit widening of `two-shard-hard-boundary/`
  - `future/gradient-observation.profile.json` must remain inventory even after runtime actualization

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001`
- Start worktree: clean at package open after `P-OPS-14`
- Existing current status at start:
  - `P-OPS-11` had fixed the gradient profile as docs-first inventory
  - `P-OPS-12` had fixed the portal/shard starter boundary
  - `P-OPS-14` had promoted `gradient observation runtime first cut` as the next reopen point
  - no runnable gradient root existed yet
- Resource checks before heavier runtime/test work:
  - `df -h .`: `/dev/vda2` 99G total, 74G used, 21G available, 78% used
  - `free -h`: `Mem 960Mi total / 474Mi used / 89Mi free / 485Mi available`, `Swap 19Gi total / 1.2Gi used / 18Gi free`

## Documents consulted

- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/26-operational-product-sample-suite.md`
- `specs/27-spatial-portal-and-shard-extension-boundary.md`
- `plan/00-index.md`
- `plan/51-operational-product-sample-roadmap.md`
- `plan/52-portal-spatial-world-roadmap.md`
- `samples/product-alpha1/README.md`
- `samples/product-alpha1/operational/README.md`
- `docs/hands_on/operational_product_sample_01.md`
- `docs/research_abstract/operational_product_sample_01.md`
- `docs/hands_on/operational_gradient_observation_profile_01.md`
- `docs/research_abstract/operational_gradient_observation_profile_01.md`
- `docs/hands_on/operational_portal_shard_starter_boundary_01.md`
- `docs/research_abstract/operational_portal_shard_starter_boundary_01.md`
- `sub-agent-pro/operational-product-sample-001/00-index.md`
- relevant companion handoff docs under `sub-agent-pro/operational-product-sample-001/`

## Actions taken

- Added a new runnable operational root:
  - created `samples/product-alpha1/operational/two-shard-gradient-observation/`
  - introduced representative `.mir` source and versioned `package.mir.json`
  - fixed the root as a bounded observer-only runtime rather than a profile-only placeholder
- Extended schema/runtime/tooling support:
  - added `two_shard_gradient_observation` package-kind acceptance in `crates/mir-ast`
  - routed the new kind through `product_alpha1_session` runtime materialization
  - added observer-only event and route evidence for:
    - `gradient_view_observed`
    - `gradient_handoff_hint_published`
    - `gradient_write_rejected`
    - `gradient_stale_view_dropped`
    - `gradient_missing_freshness_rejected`
  - surfaced the new root through `mirrorea-cli` and `scripts/operational_product_samples.py`
- Hardened the semantics after review findings:
  - extended `ProductAlpha1RouteEntry` with `config_epoch`, `owner_epoch`, and `sequence`
  - populated freshness tuples in the gradient runtime routes and devtools membership timeline
  - removed the contradictory `RejectGradientWrite` granted capability from the sample package
  - modeled write rejection as observer-only/no-write evidence rather than a granted positive authority
- Updated devtools and helper surfaces:
  - added bounded gradient runtime status/panel wiring in `product_alpha1_devtools`
  - added helper semantic checks and `run-two-shard-gradient-observation`
  - updated expected JSON and future profile files to pair the new runtime root with the still-non-executable profile
- Synchronized the docs/source hierarchy:
  - updated root docs, operational guides, research summaries, roadmap memory, dashboards, and specs
  - narrowed all stale “no gradient runtime exists” wording to the correct statement:
    the profile file remains non-executable, while the separate runtime root is actualized
- Revalidated the entire package:
  - reran focused two-shard/gradient tests while iterating on the semantic fixes
  - reran the full docs/Rust/helper validation floor
  - resolved one rustfmt drift by running `cargo fmt`

## Files changed

- Runtime / schema / CLI / helper:
  - `crates/mir-ast/src/product_alpha1.rs`
  - `crates/mir-ast/tests/product_alpha1_package_schema.rs`
  - `crates/mir-runtime/src/product_alpha1_session.rs`
  - `crates/mir-runtime/src/product_alpha1_transport.rs`
  - `crates/mir-runtime/src/product_alpha1_devtools.rs`
  - `crates/mir-runtime/tests/product_alpha1_session.rs`
  - `crates/mir-runtime/tests/product_alpha1_transport_devtools.rs`
  - `crates/mirrorea-cli/src/main.rs`
  - `scripts/operational_product_samples.py`
  - `scripts/tests/test_operational_product_samples.py`
- New operational root / expected / future profile sync:
  - `samples/product-alpha1/operational/two-shard-gradient-observation/README.md`
  - `samples/product-alpha1/operational/two-shard-gradient-observation/package.mir.json`
  - `samples/product-alpha1/operational/two-shard-gradient-observation/two-shard-gradient-observation.mir`
  - `samples/product-alpha1/operational/expected/workflow.expected.json`
  - `samples/product-alpha1/operational/expected/future-boundary.expected.json`
  - `samples/product-alpha1/operational/future/gradient-observation.profile.json`
  - `samples/product-alpha1/operational/future/spatial-shard-future.profile.json`
- Specs / roadmap / snapshot / guide sync:
  - `README.md`
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `specs/26-operational-product-sample-suite.md`
  - `specs/27-spatial-portal-and-shard-extension-boundary.md`
  - `plan/00-index.md`
  - `plan/51-operational-product-sample-roadmap.md`
  - `plan/52-portal-spatial-world-roadmap.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/README.md`
  - `samples/product-alpha1/README.md`
  - `samples/product-alpha1/operational/README.md`
  - `scripts/README.md`
  - `docs/hands_on/README.md`
  - `docs/research_abstract/README.md`
  - `docs/hands_on/operational_product_sample_01.md`
  - `docs/research_abstract/operational_product_sample_01.md`
  - `docs/hands_on/operational_gradient_observation_profile_01.md`
  - `docs/research_abstract/operational_gradient_observation_profile_01.md`
  - `docs/hands_on/operational_portal_shard_starter_boundary_01.md`
  - `docs/research_abstract/operational_portal_shard_starter_boundary_01.md`
- Report:
  - `docs/reports/2058-p-ops-15-gradient-observation-runtime-first-cut.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
df -h .
free -h
git status --short
date '+%Y-%m-%d %H:%M %Z'
sed -n '1,260p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' tasks.md
sed -n '1,260p' samples_progress.md
sed -n '1,260p' docs/hands_on/operational_product_sample_01.md
sed -n '1,240p' docs/research_abstract/operational_product_sample_01.md
sed -n '1,240p' docs/hands_on/operational_gradient_observation_profile_01.md
sed -n '1,220p' docs/research_abstract/operational_gradient_observation_profile_01.md
sed -n '1,240p' docs/hands_on/operational_portal_shard_starter_boundary_01.md
sed -n '1,220p' docs/research_abstract/operational_portal_shard_starter_boundary_01.md
sed -n '1,260p' plan/52-portal-spatial-world-roadmap.md
rg -n '...gradient...' tasks.md samples_progress.md docs/hands_on docs/research_abstract plan/52-portal-spatial-world-roadmap.md
python3 -m unittest scripts.tests.test_operational_product_samples -k two_shard
cargo test -p mir-ast --test product_alpha1_package_schema operational_sample_suite_roots -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session two_shard -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools two_shard -- --nocapture
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_operational_product_samples
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
cargo fmt
git diff --check
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
python3 scripts/operational_product_samples.py check-all --format json
```

## Evidence / outputs / test results

- Focused pre-closeout regressions passed:
  - `python3 -m unittest scripts.tests.test_operational_product_samples -k two_shard`
  - `cargo test -p mir-ast --test product_alpha1_package_schema operational_sample_suite_roots -- --nocapture`
  - `cargo test -p mir-runtime --test product_alpha1_session two_shard -- --nocapture`
  - `cargo test -p mir-runtime --test product_alpha1_transport_devtools two_shard -- --nocapture`
- Full docs / hierarchy floor passed:
  - `python3 -m unittest scripts.tests.test_validate_docs`
    - 13 tests passed
  - `python3 -m unittest scripts.tests.test_operational_product_samples`
    - 18 tests passed
  - `python3 scripts/check_source_hierarchy.py`
    - `required = 155`
    - `present = 155`
    - `missing = 0`
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 1209 numbered report(s).`
  - `cargo fmt --check`
    - initially failed on one rustfmt line in `crates/mir-runtime/src/product_alpha1_devtools.rs`
    - passed after `cargo fmt`
  - `git diff --check`
    - passed
- Full Rust behavior floor passed:
  - `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture`
    - 19 tests passed
  - `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture`
    - 24 tests passed
    - includes `product_alpha1_run_local_accepts_operational_two_shard_gradient_observation_root`
  - `cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture`
    - 7 tests passed
    - includes `product_alpha1_operational_two_shard_gradient_observation_devtools_bundle_surfaces_observer_only_gradient_runtime`
  - `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture`
    - 20 tests passed
- Operational helper closeout passed:
  - `python3 scripts/operational_product_samples.py check-all --format json`
    - `status = accepted`
    - `docker_included = true`
    - `failed_commands = []`
    - `release_check.gradient_runtime_ok = true`
    - `release_check.gradient_devtools_ok = true`
    - `release_check.shard_runtime_ok = true`
    - `release_check.shard_devtools_ok = true`
    - `release_check.portal_runtime_ok = true`
    - `release_check.portal_devtools_ok = true`
    - `release_check.membership_chat_chat_text_ok = true`
    - `release_check.membership_chat_devtools_ok = true`
    - `release_check.projection_inventory_ok = true`
    - `release_check.sugoroku_runtime_ok = true`
    - `release_check.sugoroku_devtools_ok = true`

## What changed in understanding

- The safe way to actualize gradient observation in the current operational line is a separate bounded runtime root paired with the non-executable profile, not an in-place widening that blurs the hard-authority baseline.
- The freshness tuple cannot stay narrative-only. Once gradient runtime is executable, `config_epoch`, `owner_epoch`, and `sequence` need to be carried through routes/devtools alongside `membership_epoch` and `member_incarnation`.
- “Observer-only” is stricter than “a write-like action was rejected.” The sample and runtime evidence needed to avoid granting a positive write-related capability and instead prove that overlap views remain no-write by construction.
- The operational source hierarchy now depends on the distinction:
  executable gradient root actualized, profile file non-executable, continuous sync still deferred.

## Open questions

- Which final-public gate should be promoted next: final grammar/ABI scoping, broader room-chat widening, or a later operational/public surface selection package?
- If room-chat is widened later, should the next step be richer bounded room message rows inside `MembershipChat`, or transport-coupled chat semantics in a separate package?
- If portal/shard authoring is revisited later, is a starter catalog still necessary now that active roots plus future inventory are both explicit?

## Suggested next prompt

`P-OPS-16 final-public gate scoping を開き、current operational suite と product alpha release-candidate workflow を前提に、next promoted line を final grammar / ABI / WAN / distributed durability / packaging adoption のどこに置くかを docs / roadmap / dashboards / report まで含めて閉じてください。`

## Plan update status

`plan/` 更新済み: `plan/00-index.md`, `plan/51-operational-product-sample-roadmap.md`, `plan/52-portal-spatial-world-roadmap.md` を `TwoShardGradientObservation` actualization と next reopen point `final-public gate scoping` に同期した。

## Documentation.md update status

`Documentation.md` 更新済み: operational sample suite の current executable chain と `P-OPS-15` bounded observer-only gradient runtime actualizationを追記した。

## progress.md update status

`progress.md` 更新済み: latest closeout package を `P-OPS-15` に進め、runtime/sample/dashboard snapshot と next reopen point を `final-public gate scoping` に更新した。

## tasks.md update status

`tasks.md` 更新済み: `P-OPS-15` closeout を current task-level status に追加し、ordered self-driven packages と current recommendation を `final-public gate scoping` に切り替えた。

## samples_progress.md update status

`samples_progress.md` 更新済み: operational suite row に `two-shard-gradient-observation` を追加し、validation anchors と recent validation log を `P-OPS-15` closeout に同期した。

## Reviewer findings and follow-up

- Sub-agent reviewer `Carver` found two semantic gaps:
  - gradient freshness tuple was narrated but not carried in route/devtools evidence
  - the sample granted `RejectGradientWrite`, which weakened the no-write invariant
- Follow-up implemented:
  - added `config_epoch`, `owner_epoch`, `sequence` to `ProductAlpha1RouteEntry`
  - populated gradient freshness values and surfaced them in devtools timeline status
  - removed `RejectGradientWrite` from granted sample capabilities
  - kept write rejection as observer-only/no-write evidence
- Sub-agent reviewer `Hegel` found source hierarchy drift:
  - specs/roadmaps/snapshots/guides still described gradient runtime as future work
- Follow-up implemented:
  - synchronized `specs/26..27`, `plan/51..52`, `progress.md`, `tasks.md`, `samples_progress.md`, and all relevant operational hands-on / research guides

## Skipped validations and reasons

- No validations were skipped in the final closeout floor.

## Commit / push status

- Commit: pending at report creation time
- Push: pending at report creation time

## Sub-agent session close status

- Reviewer agents completed and were closed:
  - `Carver` (`019dffb4-49c3-7c91-b436-3a5282706945`)
  - `Hegel` (`019dffb4-47da-7432-998b-1fdbf8d54aa9`)
