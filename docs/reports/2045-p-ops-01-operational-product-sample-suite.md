# Report 2045 — P-OPS-01 operational product sample suite

- Date: 2026-05-06 21:12 JST
- Author / agent: Codex
- Scope: canonical operational product sample suite scaffold, first runnable workflow, docs/dashboard sync, focused review closeout
- Decision levels touched: `L1`/`L2` wording sync only; no new foundational `L0` decision introduced

## Objective

Close `P-OPS-01 operational product sample suite scaffold and first workflow` by adding a separate operational sample root above the product alpha release-candidate demo root, with externally reproducible bounded workflow evidence and explicit non-claims.

## Scope and assumptions

- Scope includes:
  - `specs/26..27`
  - `plan/51..52`
  - `samples/product-alpha1/operational/`
  - `scripts/operational_product_samples.py`
  - product alpha runtime / CLI / devtools adjustments needed to run the operational suite
  - required root/index/dashboard docs
- Scope excludes:
  - final textual `.mir` grammar
  - final server/client binary split
  - direct LLVM backend
  - WAN / federation completion
  - distributed durable save/load R3/R4
  - portal/shard runtime completion
- Assumption: Docker and `docker compose` are available for the bounded local/Docker operational transport leg in this environment.

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001` after `main` fast-forward pull and branch creation.
- Start worktree: dirty only with this package’s in-progress edits and untracked operational handoff/docs/sample files.
- Resource check was run before heavy work:
  - `df -h .`: `/dev/vda2` 99G total, 24G available
  - `free -h`: 960Mi RAM, 19Gi swap
- During work, local generated `.mirrorea-alpha/` session artifacts appeared and were removed before closeout.

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
- `samples/product-alpha1/README.md`
- `samples/product-alpha1/demo/README.md`
- `tmp_faq/faq_015.md`
- `tmp_faq/faq_016.md`
- `docs/reports/2042-p-a1-31-release-candidate-closeout.md`
- `sub-agent-pro/operational-product-sample-001/00-index.md`
- `sub-agent-pro/operational-product-sample-001/*.md`
- `sub-agent-pro/operational-product-sample-001/sample-blueprints/*.md`

## Actions taken

- Added `specs/26-operational-product-sample-suite.md` as the normative boundary for the canonical operational suite.
- Added `specs/27-spatial-portal-and-shard-extension-boundary.md` for portal/world-link and shard/federation future boundaries.
- Added `plan/51-operational-product-sample-roadmap.md` and `plan/52-portal-spatial-world-roadmap.md`.
- Added `samples/product-alpha1/operational/` with:
  - runnable roots `world-core/`, `membership-chat/`, `sugoroku-world/`
  - shared attach packages `debug-layer`, `auth-layer`, `rate-limit-layer`, `placeholder-object`, `custom-avatar-preview`
  - local/Docker/projection deployment inventory
  - portal/shard future inventory
  - expected JSON anchor files
- Extended product alpha schema/runtime support to admit operational world-like package kinds and sibling dependency paths.
- Extended product devtools export with source/import graph, package dependency graph, projection target graph, server/client process graph, contract/effect/failure summary, portal future panel, and shard-map future panel.
- Hardened route/devtools payloads so route rows now carry bounded message-state/transport-contract/membership/capability/witness/dispatch summaries, and membership frontier rows explicitly mark `config_epoch` as kept-later manifest-only.
- Fixed operational Docker transport wiring so operational sessions use `samples/product-alpha1/operational/deployments/docker/docker-compose.operational.yml`, and made that compose file call real internal transport helper commands.
- Fixed native bundle assembly so operational bundles include shared attach packages under the bundled root package tree and emit attach reports for accepted and deferred attach rows.
- Added `scripts/operational_product_samples.py` as an orchestration helper and extended it to:
  - accept `--format` before or after subcommands
  - include deferred object/avatar attach rows in the visible attach/release path
  - fail `release-check` when the operational attach matrix is incomplete
  - include its own unittest in `check-all`
- Added `docs/hands_on/operational_product_sample_01.md` and `docs/research_abstract/operational_product_sample_01.md`.
- Updated required root/index/dashboard docs for the new operational line.
- Ran sub-agent review for theory/security, runtime/toolchain/devtools, and docs/source-hierarchy.
- Addressed the concrete runtime/toolchain/docs reviewer findings in this package; theory/security findings were kept as residual boundedness/non-claim notes rather than silently overclaimed away.

## Files changed

- Runtime / CLI / tests:
  - `crates/mir-ast/src/product_alpha1.rs`
  - `crates/mir-ast/tests/product_alpha1_package_schema.rs`
  - `crates/mir-runtime/src/product_alpha1_session.rs`
  - `crates/mir-runtime/src/product_alpha1_transport.rs`
  - `crates/mir-runtime/src/product_alpha1_devtools.rs`
  - `crates/mir-runtime/tests/product_alpha1_session.rs`
  - `crates/mir-runtime/tests/product_alpha1_transport_devtools.rs`
  - `crates/mirrorea-cli/src/main.rs`
  - `crates/mirrorea-cli/tests/alpha_cli.rs`
- New operational sample/docs/spec/plan/script surfaces:
  - `samples/product-alpha1/operational/**`
  - `specs/26-operational-product-sample-suite.md`
  - `specs/27-spatial-portal-and-shard-extension-boundary.md`
  - `plan/51-operational-product-sample-roadmap.md`
  - `plan/52-portal-spatial-world-roadmap.md`
  - `scripts/operational_product_samples.py`
  - `scripts/tests/test_operational_product_samples.py`
  - `docs/hands_on/operational_product_sample_01.md`
  - `docs/research_abstract/operational_product_sample_01.md`
- Root/index/dashboard docs:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/README.md`
  - `samples/product-alpha1/README.md`
  - `docs/hands_on/README.md`
  - `docs/research_abstract/README.md`
  - `scripts/README.md`
  - `specs/00-document-map.md`
  - `plan/00-index.md`
  - `scripts/check_source_hierarchy.py`
  - `scripts/validate_docs.py`
  - `scripts/tests/test_validate_docs.py`
- Review evidence:
  - `docs/reports/review-2026-05-06-operational-product-suite-theory-security.md`
  - `docs/reports/review-2026-05-06-p-ops-01-operational-runtime-toolchain-review.md`
- This report:
  - `docs/reports/2045-p-ops-01-operational-product-sample-suite.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git checkout main
git pull --ff-only
git checkout -b feature/operational-product-sample-001
df -h .
free -h
sed -n '1,260p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' progress.md
sed -n '1,280p' tasks.md
sed -n '1,260p' samples_progress.md
sed -n '1,260p' specs/00-document-map.md
sed -n '1,260p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/18-practical-alpha1-scope.md
sed -n '1,260p' specs/19-verification-stratification.md
sed -n '1,260p' specs/20-cut-save-load-semantics.md
sed -n '1,260p' specs/21-auth-layer-algebra.md
sed -n '1,260p' specs/22-observability-devtools-semantics.md
sed -n '1,260p' specs/23-typed-external-host-boundary.md
sed -n '1,260p' specs/24-operational-alpha05-alpha08-readiness.md
sed -n '1,520p' specs/25-product-alpha1-public-boundary.md
sed -n '1,260p' plan/44-practical-alpha1-roadmap.md
sed -n '1,260p' plan/45-operational-alpha05-roadmap.md
sed -n '1,260p' plan/46-operational-alpha08-roadmap.md
sed -n '1,260p' plan/47-operational-alpha09-devtools-roadmap.md
sed -n '1,260p' plan/48-theory-freeze-proof-obligations.md
sed -n '1,260p' plan/49-host-io-and-session-runtime-roadmap.md
sed -n '1,260p' plan/50-product-alpha1-public-boundary-roadmap.md
sed -n '1,260p' samples/product-alpha1/README.md
sed -n '1,260p' samples/product-alpha1/demo/README.md
sed -n '1,260p' tmp_faq/faq_015.md
sed -n '1,260p' tmp_faq/faq_016.md
sed -n '1,260p' docs/reports/2042-p-a1-31-release-candidate-closeout.md
sed -n '1,260p' sub-agent-pro/operational-product-sample-001/00-index.md
find sub-agent-pro/operational-product-sample-001 -type f | sort
git diff --stat
git status --short
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
python3 -m unittest scripts.tests.test_operational_product_samples
python3 scripts/operational_product_samples.py list --format json
python3 scripts/operational_product_samples.py check-all --format json
date '+%Y-%m-%d %H:%M %Z'
```

Final scaffold/formatting/source-hierarchy validations were rerun after this report was created so the report itself was included in those checks.

## Evidence / outputs / test results

- `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture`: pass, 10 tests.
- `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture`: pass, 15 tests.
- `cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture`: pass, 3 tests.
- `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture`: pass, 20 tests.
- `python3 -m unittest scripts.tests.test_operational_product_samples`: pass, 5 tests.
- `python3 scripts/operational_product_samples.py list --format json`: pass.
- `python3 scripts/operational_product_samples.py check-all --format json`: pass with:
  - `status = accepted`
  - `docker_included = true`
  - `failed_commands = []`
  - operational `check` for `world-core`, `membership-chat`, `sugoroku-world`
  - `run-local`, `session`, accepted debug/auth/rate-limit attach
  - deferred `placeholder-object` / `custom-avatar-preview` attach
  - `save`, `quiescent-save`, local/Docker `transport`
  - `export-devtools`, `view --check`
  - `build-native-bundle`
- Docker availability probe before the final helper run succeeded: `Docker version 29.3.0, build 5927d80`.

## What changed in understanding

- `demo/` and `operational/` need different semantics: `demo/` is the product alpha release-candidate walkthrough root, while `operational/` is the more realistic package/import/process suite.
- The operational suite exposed real wiring gaps that the demo root had hidden:
  - Docker transport compose selection
  - shared attach package preservation inside native bundles
  - deferred object/avatar boundaries needing to stay visible in release helper flows
- The theory/security review showed that some runtime rows are still bounded same-session evidence rather than final proof-grade semantics. The right response in this package was to document that boundedness explicitly, not to overclaim.

## Open questions

- Should `MembershipChat` widen to `EchoText` or `ChatText` first for `P-OPS-03`?
- How far should Sugoroku runtime behavior move into the current product alpha session carrier in `P-OPS-04`?
- Should projection target / packet / FFI schema land as manifest-adjacent fields or a separate projection IR file in `P-OPS-05`?
- Theory/security residuals left open:
  - attach-time membership/auth evidence is still bounded same-session bootstrap evidence, not external issuer-backed attestation
  - `R2` success remains bounded session-preflight evidence, not durable/distributed proof completion
  - `NoInFlight` still requires stronger default-carrier evidence if promoted beyond current bounded semantics

## Suggested next prompt

Open `P-OPS-03 operational chat / direct text host boundary`, or, if projection-first is preferable, open `P-OPS-05 operational projection manifest and packet schema`.

## Plan update status

`plan/` 更新済み:

- `plan/00-index.md`
- `plan/51-operational-product-sample-roadmap.md`
- `plan/52-portal-spatial-world-roadmap.md`

## Documentation.md update status

`Documentation.md` 更新済み: operational product sample suite line and its reader-facing references were added.

## progress.md update status

`progress.md` 更新済み: latest closeout, reopen point, line snapshot, validation floor, and recent log now include `P-OPS-01`.

## tasks.md update status

`tasks.md` 更新済み: `P-OPS-01` actualization and next OPS reopen packages were added.

## samples_progress.md update status

`samples_progress.md` 更新済み: operational suite row, validation anchors, and recent validation log were added.

## Reviewer findings and follow-up

- Docs/source-hierarchy reviewer:
  - found stale `progress.md`, `tasks.md`, `samples_progress.md`, and missing task report
  - follow-up: addressed in this package by updating all three dashboards and adding this report
- Runtime/toolchain/devtools reviewer:
  - found operational Docker transport was still wired to demo compose
  - found native bundle omitted operational shared attach packages
  - found helper/release-check skipped deferred object/avatar attach rows
  - found route/config panels were stronger as inventory than as semantically complete panels
  - follow-up:
    - Docker compose selection and compose file were fixed
    - bundled attach package copying and bundle attach reports were added
    - helper release-check now verifies the full attach matrix including deferred rows
    - route rows were enriched with bounded summary fields and membership timeline now explicitly marks `config_epoch` as kept-later manifest-only
- Theory/security reviewer:
  - found attach-time membership/auth evidence remains same-session self-carried bootstrap evidence
  - found `quiescent-save` currently synthesizes part of its own success evidence
  - found default `NoInFlight` evidence is weak unless injected by test
  - follow-up:
    - not fully fixed in this package
    - boundedness/non-claim wording was added to `specs/26`, `samples/product-alpha1/operational/README.md`, and `docs/hands_on/operational_product_sample_01.md`
    - reopen remains separate runtime-semantics hardening work, not silent completion
- Additional reviewer note:
  - one reviewer timed out during the first wait and completed later; no additional narrow-scope re-review was dispatched after the final fixes, so the final confidence comes from local focused validation plus the original review evidence

## Skipped validations and reasons

- Full workspace `cargo test` across all crates was not run.
  - Reason: this package touched focused product-alpha1/operational surfaces; targeted tests plus helper closeout were used instead.
- Final public grammar / ABI, WAN/federation, distributed durable save/load, final viewer/telemetry service, arbitrary native package execution, and portal/shard runtime execution were not run.
  - Reason: they remain explicit non-goals / future boundaries for this package.

## Commit / push status

- Commit: pending at report creation time.
- Push: pending at report creation time.

## Sub-agent session close status

- Sub-agent review was used.
- Completed reviewer evidence:
  - `docs/reports/review-2026-05-06-operational-product-suite-theory-security.md`
  - `docs/reports/review-2026-05-06-p-ops-01-operational-runtime-toolchain-review.md`
- Reviewer sessions were not yet closed at report creation time; closeout is handled after final validation/commit.
