# Report 2051 — P-OPS-09 developer package authoring guide

- Date: 2026-05-07 00:25 JST
- Author / agent: Codex
- Scope: operational template-only authoring starter, external developer guide, snapshot/doc sync, focused validation
- Decision levels touched: `L1`/`L2` wording sync only; no new `L0` decision introduced

## Objective

Close `P-OPS-09` by adding a bounded external-developer authoring path for the operational product suite without introducing a generic scaffold CLI, without promoting template roots into active operational sample roots, and without claiming final public grammar, ABI, or release-helper generality.

## Scope and assumptions

- Scope includes:
  - `samples/product-alpha1/operational/templates/` template-only starter family
  - `templates/world-core-starter/` as the first validated `world_core` starter
  - `docs/hands_on/operational_package_authoring_01.md`
  - `docs/research_abstract/operational_package_authoring_01.md`
  - focused product-alpha tests and source-hierarchy coverage for the starter root
  - required README / taxonomy / roadmap / dashboard / report sync
- Scope excludes:
  - generic scaffold command generation
  - final public grammar / ABI / SDK
  - backend implementation or LLVM/WASM codegen
  - automatic release-check helper generation for arbitrary new packages
  - broader template catalog beyond the first `world_core` starter
- Assumption:
  - the existing product alpha direct CLI surfaces remain the canonical entrypoint for authoring validation

## Start state / dirty state

- Start branch: `feature/operational-product-sample-001`
- Start worktree: dirty with intentional in-progress `P-OPS-09` authoring-guide/template files after `P-OPS-07` commit `178fafd`
- Existing operational suite state at start:
  - `P-OPS-07` already actualized `two-shard-hard-boundary/` as a bounded same-session hard-authority root
  - no template-only starter existed under `samples/product-alpha1/operational/`
  - no dedicated external-developer authoring guide existed for creating a new operational package from the validated suite line

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
- `plan/00-index.md`
- `plan/51-operational-product-sample-roadmap.md`
- `docs/hands_on/README.md`
- `docs/research_abstract/README.md`
- `docs/hands_on/operational_product_sample_01.md`
- `docs/reports/2050-p-ops-07-two-shard-hard-boundary-first-cut.md`
- `sub-agent-pro/operational-product-sample-001/09-sample-matrix.md`
- `sub-agent-pro/operational-product-sample-001/15-next-packages.md`

## Actions taken

- Added a template-only authoring root:
  - created `samples/product-alpha1/operational/templates/README.md`
  - created `samples/product-alpha1/operational/templates/world-core-starter/README.md`
  - created representative source `world-core-starter.mir`
  - created executable `package.mir.json` for a minimal `world_core` starter
- Added bounded external-developer guidance:
  - created `docs/hands_on/operational_package_authoring_01.md`
  - created `docs/research_abstract/operational_package_authoring_01.md`
  - fixed the documented order at `author -> check -> run-local -> session -> export-devtools -> view --check`
  - kept `scripts/operational_product_samples.py` explicitly out of the generic-authoring front door
- Added focused executable coverage for the starter root:
  - added `mir-ast` test coverage for schema acceptance of the starter root
  - added `mir-runtime` session coverage for `run-local` acceptance of the starter root
  - extended `scripts/check_source_hierarchy.py` to require the new template and docs paths
- Updated snapshot / taxonomy wording:
  - clarified in READMEs, `specs/26`, `plan/51`, `specs/00`, `plan/00`, `progress.md`, `tasks.md`, and `samples_progress.md` that template roots may be runnable through bounded `check` / `run-local` while remaining `template_only`
  - moved the reopen point from `P-OPS-09` to `P-OPS-08`

## Files changed

- Tests / structural validation:
  - `crates/mir-ast/tests/product_alpha1_package_schema.rs`
  - `crates/mir-runtime/tests/product_alpha1_session.rs`
  - `scripts/check_source_hierarchy.py`
- New template / authoring docs:
  - `samples/product-alpha1/operational/templates/README.md`
  - `samples/product-alpha1/operational/templates/world-core-starter/README.md`
  - `samples/product-alpha1/operational/templates/world-core-starter/world-core-starter.mir`
  - `samples/product-alpha1/operational/templates/world-core-starter/package.mir.json`
  - `docs/hands_on/operational_package_authoring_01.md`
  - `docs/research_abstract/operational_package_authoring_01.md`
- Snapshot / taxonomy / roadmap docs:
  - `README.md`
  - `Documentation.md`
  - `samples/README.md`
  - `samples/product-alpha1/README.md`
  - `samples/product-alpha1/operational/README.md`
  - `docs/hands_on/README.md`
  - `docs/research_abstract/README.md`
  - `scripts/README.md`
  - `specs/00-document-map.md`
  - `specs/26-operational-product-sample-suite.md`
  - `plan/00-index.md`
  - `plan/51-operational-product-sample-roadmap.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- Report:
  - `docs/reports/2051-p-ops-09-operational-package-authoring-guide.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M %Z'
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
cargo test -p mir-ast --test product_alpha1_package_schema product_alpha1_package_schema_accepts_operational_template_world_core_starter -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session product_alpha1_run_local_accepts_operational_world_core_starter_template -- --nocapture
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/templates/world-core-starter --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/templates/world-core-starter --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- session 'session#operational-world-core-starter' --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-world-core-starter' --out "$viewer_dir" --format json
cargo run -q -p mirrorea-cli -- view "$viewer_dir" --check --format json
git diff --stat
git diff -- README.md Documentation.md samples/README.md samples/product-alpha1/README.md samples/product-alpha1/operational/README.md docs/hands_on/operational_package_authoring_01.md docs/research_abstract/operational_package_authoring_01.md specs/26-operational-product-sample-suite.md plan/51-operational-product-sample-roadmap.md scripts/check_source_hierarchy.py crates/mir-ast/tests/product_alpha1_package_schema.rs crates/mir-runtime/tests/product_alpha1_session.rs
```

## Evidence / outputs / test results

- Docs / structure floor:
  - `python3 -m unittest scripts.tests.test_validate_docs`: pass (`13` tests)
  - `python3 scripts/check_source_hierarchy.py`: pass (`required = 144`, `missing = 0`)
  - `python3 scripts/validate_docs.py`: pass (`Documentation scaffold looks complete. Found 1203 numbered report(s).`)
  - `cargo fmt --check`: pass
  - `git diff --check`: pass
- Focused executable/test coverage:
  - `cargo test -p mir-ast --test product_alpha1_package_schema product_alpha1_package_schema_accepts_operational_template_world_core_starter -- --nocapture`: pass
  - `cargo test -p mir-runtime --test product_alpha1_session product_alpha1_run_local_accepts_operational_world_core_starter_template -- --nocapture`: pass
- Direct CLI guide flow:
  - `check` returned `verdict = accepted` and `package_id = operational-world-core-starter`
  - `run-local` returned `surface_kind = product_alpha1_run_local_report`, `session_id = session#operational-world-core-starter`, `typed_host_io_claimed = false`
  - `session` returned `surface_kind = product_alpha1_session_report`
  - `export-devtools` returned `surface_kind = product_alpha1_devtools_export_report`
  - `view --check` returned `surface_kind = product_alpha1_view_report`
  - no step promoted the starter into `product_alpha1_ready = true`; the guide remains explicitly bounded to alpha authoring evidence

## What changed in understanding

- A useful external authoring entrypoint does not require a generic scaffold command at this stage; a validated starter plus direct `mirrorea-cli` flow is enough and keeps the canonical entrypoint explicit.
- Template roots can legitimately be runnable through bounded `check` / `run-local` while remaining outside the active operational sample-root set, as long as docs and dashboards say `template_only` everywhere the root appears.
- For new-package guidance, `session` / `export-devtools` / `view --check` are more useful first-closeout anchors than prematurely widening into generic release-helper automation.

## Open questions

- Which package kind should receive the next starter root after `world_core`: `membership_chat`, `sugoroku_world`, or a narrower package-only authoring example?
- Should future authoring guides ever widen into a generic suite helper, or should helper composition stay package-specific by design?
- When `P-OPS-08` lands, should backend inventory wording be folded into the authoring guide or remain a separate backend-facing document?

## Suggested next prompt

Open `P-OPS-08 backend feasibility inventory` and audit LLVM/native backend vs WASM vs host launch bundle boundaries, requirements, and non-claims without implementing a backend or weakening the current host-bundle line.

## Plan update status

`plan/` 更新済み:

- `plan/00-index.md`
- `plan/51-operational-product-sample-roadmap.md`

## Documentation.md update status

`Documentation.md` 更新済み: operational suite current snapshot now includes the `P-OPS-09` template-only starter and bounded external authoring guide.

## progress.md update status

`progress.md` 更新済み: latest closeout, reopen point, workflow-readiness row, line snapshot, macro-phase wording, and recent log now reflect `P-OPS-09`.

## tasks.md update status

`tasks.md` 更新済み: `P-OPS-09` is marked actualized, ordered packages now start at `P-OPS-08`, and the recommendation now points to backend feasibility inventory.

## samples_progress.md update status

`samples_progress.md` 更新済み: operational suite row now includes the template-only starter and authoring docs, template validation anchors were added, and the validation log now records `P-OPS-09`.

## Reviewer findings and follow-up

- Spawned reviewer `Singer` (`019dfde6-299f-7910-949d-2a4821db84f3`) did not return within two waits and was shut down.
- Local focused review findings:
  - authoring docs had to keep `scripts/operational_product_samples.py` as a suite helper rather than silently recasting it as a generic scaffold tool
  - template roots had to stay explicitly `template_only` in `samples/README.md`, `samples/product-alpha1/README.md`, `samples/product-alpha1/operational/README.md`, `specs/26`, and the dashboards to avoid taxonomy drift
  - the minimal executable proof for the starter root needed both schema-level and runtime/session-level focused tests, not docs wording alone
- Follow-up:
  - when broader starter kinds are added later, keep each one explicit about whether it is a promoted runnable sample root, a template-only starter, or a blueprint-only inventory root

## Skipped validations and reasons

- `python3 scripts/operational_product_samples.py check-all --format json` was not rerun because `P-OPS-09` did not change runtime/helper semantics for the existing active operational roots; this package validated the new starter root directly through `mirrorea-cli` instead.
- broad `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture` and `cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture` were not rerun because no CLI transport/devtools implementation changed in this package; focused starter-root tests and direct CLI probes covered the new surface.

## Commit / push status

- Commit: pending at report creation time
- Push: pending at report creation time

## Sub-agent session close status

- Reviewer `Singer` (`019dfde6-299f-7910-949d-2a4821db84f3`) timed out twice, returned no findings, and was shut down.
