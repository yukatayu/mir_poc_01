# Report 2053 — P-OPS-10 broader operational template catalog first cut

- Date: 2026-05-07 00:56 JST
- Author / agent: Codex
- Scope: operational template catalog widening, dependency-retarget guide sync, focused schema/runtime validation
- Decision levels touched: `L1`/`L2` wording sync only; no new `L0` decision introduced

## Objective

Close `P-OPS-10` by widening the operational authoring template catalog from `world_core` only to a bounded `world_core` / `membership_chat` / `sugoroku_world` starter chain without claiming portal/shard starter completion or any generic scaffold CLI.

## Scope and assumptions

- Scope includes:
  - `samples/product-alpha1/operational/templates/membership-chat-starter/`
  - `samples/product-alpha1/operational/templates/sugoroku-world-starter/`
  - authoring guide / summary updates for starter selection and dependency retarget
  - roadmap / snapshot / dashboard sync for the widened starter catalog
  - focused schema/runtime checks for the new starter roots
- Scope excludes:
  - new runtime/helper semantics for active operational roots
  - portal/shard starter templates
  - gradient observation runtime or profile implementation
  - generic scaffold CLI generation
  - final public grammar / ABI / SDK
- Assumptions:
  - template roots must stay `template_only` and must not be promoted into active operational sample roots
  - current product-alpha loader/runtime rules for dependency confinement remain authoritative over any docs wording

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001`
- Start worktree: clean after `P-OPS-08` commit `51e48b6d` and push
- Existing operational suite state at start:
  - `P-OPS-09` had only `templates/world-core-starter/`
  - `P-OPS-08` had already fixed the backend-adjacent non-claim line around `native host launch bundle`
  - roadmap/task memory still treated broader starter widening as the next reopen point

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
- `plan/23-compiler-backend-llvm-guardrail-roadmap.md`
- `plan/51-operational-product-sample-roadmap.md`
- `plan/52-portal-spatial-world-roadmap.md`
- `docs/hands_on/operational_package_authoring_01.md`
- `docs/research_abstract/operational_package_authoring_01.md`
- `docs/reports/2051-p-ops-09-operational-package-authoring-guide.md`
- `docs/reports/2052-p-ops-08-backend-feasibility-inventory.md`
- `sub-agent-pro/operational-product-sample-001/15-next-packages.md`

## Actions taken

- Added two new validated starter roots:
  - `templates/membership-chat-starter/`
  - `templates/sugoroku-world-starter/`
- Extended the starter catalog docs:
  - starter selection matrix in `docs/hands_on/operational_package_authoring_01.md`
  - dependency-retarget wording for copied starter roots
  - updated summary/index/taxonomy/front-door docs to treat the authoring surface as a catalog rather than a single `world_core` starter
- Used test-first validation:
  - added schema/runtime tests for the new starter roots before creating the new files
  - confirmed initial RED state because the roots did not exist
- Root-cause investigation for the first GREEN attempt:
  - initial manifests pointed dependencies to `../../world-core` / `../../membership-chat`
  - runtime rejected them because the declared dependencies escaped the package sibling tree rooted at `samples/product-alpha1/operational/templates`
  - fixed the root cause by converting the starter chain into sibling template dependencies:
    - `membership-chat-starter -> ../world-core-starter`
    - `sugoroku-world-starter -> ../membership-chat-starter`
- Synced repository memory:
  - `plan/51` now records `P-OPS-10`
  - `progress.md`, `tasks.md`, and `samples_progress.md` now move the reopen point to gradient observation profile

## Files changed

- New starter roots:
  - `samples/product-alpha1/operational/templates/membership-chat-starter/README.md`
  - `samples/product-alpha1/operational/templates/membership-chat-starter/membership-chat-starter.mir`
  - `samples/product-alpha1/operational/templates/membership-chat-starter/package.mir.json`
  - `samples/product-alpha1/operational/templates/sugoroku-world-starter/README.md`
  - `samples/product-alpha1/operational/templates/sugoroku-world-starter/sugoroku-world-starter.mir`
  - `samples/product-alpha1/operational/templates/sugoroku-world-starter/package.mir.json`
- Focused tests:
  - `crates/mir-ast/tests/product_alpha1_package_schema.rs`
  - `crates/mir-runtime/tests/product_alpha1_session.rs`
- Authoring/docs/taxonomy sync:
  - `README.md`
  - `Documentation.md`
  - `docs/hands_on/README.md`
  - `docs/hands_on/operational_package_authoring_01.md`
  - `docs/research_abstract/README.md`
  - `docs/research_abstract/operational_package_authoring_01.md`
  - `samples/README.md`
  - `samples/product-alpha1/README.md`
  - `samples/product-alpha1/operational/README.md`
  - `samples/product-alpha1/operational/templates/README.md`
  - `scripts/README.md`
  - `scripts/check_source_hierarchy.py`
  - `specs/00-document-map.md`
  - `specs/26-operational-product-sample-suite.md`
  - `plan/00-index.md`
  - `plan/51-operational-product-sample-roadmap.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- Report:
  - `docs/reports/2053-p-ops-10-operational-template-catalog-first-cut.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M %Z'
sed -n '1,220p' samples/product-alpha1/operational/templates/world-core-starter/README.md
sed -n '1,240p' samples/product-alpha1/operational/templates/world-core-starter/package.mir.json
sed -n '1,240p' docs/hands_on/operational_package_authoring_01.md
sed -n '1,260p' samples/product-alpha1/operational/world-core/package.mir.json
sed -n '1,280p' samples/product-alpha1/operational/membership-chat/package.mir.json
sed -n '1,320p' samples/product-alpha1/operational/sugoroku-world/package.mir.json
cargo test -p mir-ast --test product_alpha1_package_schema product_alpha1_package_schema_accepts_operational_template_membership_chat_starter -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session product_alpha1_run_local_accepts_operational_membership_chat_starter_template -- --nocapture
cargo test -p mir-ast --test product_alpha1_package_schema operational_template_ -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session starter_template -- --nocapture
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/templates/membership-chat-starter --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/templates/sugoroku-world-starter --format json
session_dir=$(mktemp -d /tmp/mirrorea-ops-template-chat-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/templates/membership-chat-starter --format json
session_dir=$(mktemp -d /tmp/mirrorea-ops-template-sugoroku-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/templates/sugoroku-world-starter --format json
git status --short
git diff --stat HEAD
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- RED evidence before implementation:
  - schema test failed with `MissingPackageFile` because `membership-chat-starter` did not yet exist
  - runtime test failed with the same front-door absence
- Root-cause evidence during first GREEN attempt:
  - runtime rejected `../../world-core` and `../../membership-chat` with `declared dependency ... escapes the package sibling tree rooted at .../templates`
- GREEN evidence after the dependency-chain fix:
  - `cargo test -p mir-ast --test product_alpha1_package_schema operational_template_ -- --nocapture`: pass (`world-core`, `membership-chat`, `sugoroku-world` starter schema acceptance)
  - `cargo test -p mir-runtime --test product_alpha1_session starter_template -- --nocapture`: pass (`world-core`, `membership-chat`, `sugoroku-world` starter run-local acceptance)
  - `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/templates/membership-chat-starter --format json`: `verdict = accepted`, `package_kind = membership_chat`
  - `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/templates/sugoroku-world-starter --format json`: `verdict = accepted`, `package_kind = sugoroku_world`
  - `run-local` on `membership-chat-starter`: `surface_kind = product_alpha1_run_local_report`, `session_id = session#operational-membership-chat-starter`, `declared_dependencies = ["../world-core-starter"]`, `typed_host_io_claimed = true`
  - `run-local` on `sugoroku-world-starter`: `surface_kind = product_alpha1_run_local_report`, `session_id = session#operational-sugoroku-world-starter`, `declared_dependencies = ["../membership-chat-starter"]`, `typed_host_io_claimed = true`
- Docs / formatting floor after report addition:
  - `python3 -m unittest scripts.tests.test_validate_docs`: pass
  - `python3 scripts/check_source_hierarchy.py`: pass
  - `python3 scripts/validate_docs.py`: pass
  - `cargo fmt --check`: pass
  - `git diff --check`: pass

## What changed in understanding

- The current product-alpha dependency rule for `run-local` is stricter than a mere “dependency path exists” check: the path must stay within the sibling package tree rooted at the current package family.
- For validated starter catalogs, a template-to-template chain is a better current fit than pointing starters directly at active operational roots.
- The right authoring guidance is therefore:
  - validate the catalog as-is through sibling starters
  - retarget dependency paths only after copying a starter into a real working package

## Open questions

- Should `portal_worldlink` and `two_shard_hard_boundary` ever receive validated starter roots, or should the catalog intentionally stop at `SugorokuWorld`?
- Should a later gradient observation package include a starter/profile pair, or remain docs/profile-only?
- At what point should the starter catalog gain helper-assisted copy/rename tooling, if ever?

## Suggested next prompt

Open the gradient observation profile package and keep it docs-first / manifest-first without claiming portal/shard runtime widening.

## Plan update status

`plan/` 更新済み:

- `plan/00-index.md`
- `plan/51-operational-product-sample-roadmap.md`

## Documentation.md update status

`Documentation.md` 更新済み: `P-OPS-10` starter catalog widening and the dependency-retarget reading are now included in the operational line snapshot.

## progress.md update status

`progress.md` 更新済み: latest closeout, current reopen point, macro-phase wording, executable sample corpus row, and recent log now reflect `P-OPS-10`.

## tasks.md update status

`tasks.md` 更新済み: `P-OPS-10` is marked actualized, the ordered package queue now promotes gradient observation profile, and portal/shard starter scope is demoted to a reserve decision.

## samples_progress.md update status

`samples_progress.md` 更新済み: operational suite row, validation anchors, and recent validation log now include the widened starter catalog.

## Reviewer findings and follow-up

- Reviewer `Hooke` (`019dfe04-4b2c-7f20-8a66-a077ceaaaf3e`) was started for semantic review of the starter catalog widening but did not return within two waits and was shut down.
- Local focused review findings:
  - dependency anchors for template starters must remain inside the `templates/` sibling tree or `run-local` rejects them
  - docs must describe the catalog as `template_only` and must not imply that the new starters are promoted operational roots
  - roadmap/snapshot text had to move the reopen point from starter-catalog widening to gradient observation once the `membership_chat` / `sugoroku_world` starters landed
- Follow-up:
  - if a later package adds portal/shard starters, revalidate dependency confinement explicitly before claiming `run-local`

## Skipped validations and reasons

- `python3 scripts/operational_product_samples.py check-all --format json` was not rerun because the helper semantics for active operational roots were unchanged; `P-OPS-10` widened only the template-only authoring catalog and validated it directly via focused schema/runtime tests and direct `check` / `run-local` commands.
- no Docker command was rerun because the package changed neither transport semantics nor release-check helper scope.

## Commit / push status

- Commit: pending at report creation time
- Push: pending at report creation time

## Sub-agent session close status

- Reviewer `Hooke` (`019dfe04-4b2c-7f20-8a66-a077ceaaaf3e`) timed out twice, returned no findings, and was shut down.
