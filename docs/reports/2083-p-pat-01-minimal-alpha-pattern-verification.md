# Report 2083 — P-PAT-01 minimal alpha-1 pattern verification

- Date: 2026-05-21 23:37:44 +0900
- Author / agent: Codex
- Scope: add a compact reader-facing verifier and guide for the minimal practical alpha-1 pattern set
- Decision levels touched: no new normative decision; `L2` maintenance / sample verification package over existing rows

## Objective

Add/update samples and docs so an external reader can see and verify the minimal practical alpha-1 patterns without opening the whole release-candidate and computational/PoseGraph history.

The package must:

- describe the smallest useful executable / intentionally rejected alpha-1 patterns.
- verify exact row counts, expected rejection IDs, compatibility rows, and inventory-only policies.
- keep product release-candidate and operational Sugoroku as workflow anchors, not final product claims.
- preserve theory boundaries: no stdio builtin, typed host boundary, explicit effects/failures/capabilities, no codegen/provider overclaim.

## Scope and assumptions

- This package is a verifier / documentation package, not a new runtime semantics package.
- Reuse existing helpers:
  `mir_computational_samples.py`, `posegraph_samples.py`, `projection_boundary_samples.py`, `engine_adapter_boundary_samples.py`, `product_alpha1_release_check.py`, and `operational_product_samples.py`.
- Default `check-all` should stay compact and strict. Heavy product/operational workflows are optional via `--include-workflows`.
- Docker-backed workflow anchors count as release evidence only when Docker is included and the underlying helper reports accepted.

## Start state / dirty state

- Start point was `main` at `0c7006de`.
- Workspace was clean before this task.
- Resource preflight before heavy workflow validation:
  `/dev/vda2` had `20G` available on a `99G` filesystem;
  memory available was `296MiB` with `17GiB` free swap.

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
- `plan/57-autonomous-computational-core-master-plan.md`
- `docs/hands_on/mir_computational_core_01.md`
- `docs/hands_on/transform_posegraph_01.md`
- `docs/hands_on/product_alpha1_01.md`
- `docs/research_abstract/mir_computational_core_01.md`
- Existing helper/test sources under `scripts/`

## Actions taken

- Added `scripts/minimal_alpha1_patterns.py` with:
  `list`, `matrix`, `check-all`, `run`, and `closeout`.
- Added unit tests for the new verifier using TDD:
  first run failed because the module did not exist; after implementation, tests passed.
- Implemented strict default checks over:
  computational,
  PoseGraph,
  projection,
  engine-adapter.
- Added optional `check-all --include-workflows --out <dir>` to run product release-candidate and operational suite workflow anchors.
- Added a hands-on guide and research summary for minimal alpha-1 patterns.
- Updated top-level and sample/script dashboards to include the new verifier.
- Fixed one newly exposed drift in the new verifier itself:
  initial projection / engine expected IDs were guessed from old naming; the actual matrix IDs in existing tests were the source of truth and the verifier was corrected.

## Files changed

- New verifier / tests:
  `scripts/minimal_alpha1_patterns.py`
  `scripts/tests/test_minimal_alpha1_patterns.py`
- New docs:
  `docs/hands_on/minimal_alpha1_patterns_01.md`
  `docs/research_abstract/minimal_alpha1_patterns_01.md`
- Updated snapshot / index docs:
  `README.md`
  `Documentation.md`
  `progress.md`
  `tasks.md`
  `samples_progress.md`
  `samples/README.md`
  `samples/product-alpha1/README.md`
  `scripts/README.md`
  `docs/hands_on/README.md`
  `docs/research_abstract/README.md`
- Updated repository memory:
  `plan/57-autonomous-computational-core-master-plan.md`
- Report:
  `docs/reports/2083-p-pat-01-minimal-alpha-pattern-verification.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
git rev-parse --short HEAD
python3 -m unittest scripts.tests.test_minimal_alpha1_patterns
python3 scripts/minimal_alpha1_patterns.py matrix --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/projection_boundary_samples.py check-all --format json
python3 scripts/engine_adapter_boundary_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py run mir-compute-host-io-transform --format json
python3 scripts/minimal_alpha1_patterns.py run mir-compute-missing-effect-reject --format json
python3 scripts/minimal_alpha1_patterns.py run posegraph-no-split-frame --format json
python3 scripts/minimal_alpha1_patterns.py run posegraph-split-frame-violation --format json
python3 scripts/minimal_alpha1_patterns.py run projection-inventory-boundary --format json
python3 scripts/minimal_alpha1_patterns.py run engine-adapter-wasm-inventory --format json
df -h .
free -h
python3 scripts/minimal_alpha1_patterns.py check-all --include-workflows --out /tmp/mirrorea-minimal-alpha1-patterns-XXXXXX --format json
date '+%Y-%m-%d %H:%M:%S %z'
python3 -m unittest scripts.tests.test_minimal_alpha1_patterns scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/minimal_alpha1_patterns.py closeout --format json
```

## Evidence / outputs / test results

- TDD red:
  `python3 -m unittest scripts.tests.test_minimal_alpha1_patterns`
  initially failed with `ModuleNotFoundError: No module named 'minimal_alpha1_patterns'`.
- Unit test after implementation:
  `python3 -m unittest scripts.tests.test_minimal_alpha1_patterns`
  passed 8/8.
- `python3 scripts/minimal_alpha1_patterns.py matrix --format json`
  reported:
  `pattern_count = 9`,
  `workflow_anchor_count = 2`,
  `default_strict_pattern_count = 7`,
  `default_strict_family_count = 4`,
  `final_public_product_claimed = false`.
- First `check-all` run rejected projection / engine-adapter because the new verifier used stale guessed IDs.
  Existing helpers proved the actual IDs:
  projection planned rows are
  `proj-01-server-client-target-manifest`,
  `proj-01-packet-boundary-schema`,
  `proj-01-ffi-boundary-schema`,
  `proj-01-manifest-provider-compatibility`;
  engine provider rows are
  `renderer`,
  `input-device`,
  `asset-loader`,
  `physics-spatial-query`,
  `host-runtime-bridge`,
  `wasm-sandbox`,
  `native-library-bridge`,
  `viewer-diagnostic-exporter`.
- Corrected `check-all` then passed with:
  `status = accepted`,
  4 strict families,
  computational 15 rows / 7 accepted / 5 runtime rejections / 3 check rejections,
  PoseGraph 1 accepted / 1 violation / 7 planned,
  projection 4 planned plus accepted/rejected compatibility rows,
  engine-adapter 8 planned providers with native disabled and WASM inventory-only.
- Pattern run checks passed for:
  `mir-compute-host-io-transform`,
  `mir-compute-missing-effect-reject`,
  `posegraph-no-split-frame`,
  `posegraph-split-frame-violation`,
  `projection-inventory-boundary`,
  `engine-adapter-wasm-inventory`.
- Heavy workflow-inclusive check passed:
  `status = accepted`,
  `workflow_anchors_checked = true`,
  product release check `status = accepted`,
  `include_docker = true`,
  operational product samples `status = accepted`,
  `failed = []`.
- Final validation after report write:
  `python3 -m unittest scripts.tests.test_minimal_alpha1_patterns scripts.tests.test_validate_docs`
  passed 22/22;
  `python3 scripts/check_source_hierarchy.py` passed with `required = 235`, `missing = 0`;
  `python3 scripts/validate_docs.py` passed with documentation scaffold complete;
  `cargo fmt --check` passed;
  `git diff --check` passed.
- `python3 scripts/minimal_alpha1_patterns.py closeout --format json`
  reported all 9 patterns, 7 strict patterns, 2 workflow anchors, explicit validation floor, and `final_public_product_claimed = false`.

## What changed in understanding

- The minimal pattern verifier is useful precisely because it fails on row-name drift, not only on helper exit status.
- Projection and engine-adapter inventory IDs are already fixed by their existing helpers/tests; new aggregators must read those as source-of-truth rather than inventing friendlier names.
- Product/operational workflows are already reproducible; the missing reader-facing piece was a compact strict map that explains which rows are executable, intentionally rejected, or inventory-only.

## Open questions

- No new package-level blocker was introduced.
- Broader computational publish / observe / witness / handoff widening remains later.
- PoseGraph runtime/save-load/devtools widening remains later.
- Projection codegen and engine/WASM/native provider admission remain later or user-spec-required.
- Final distribution, final grammar/API/SDK, final catalog breadth, WAN/federation, and distributed durable save-load remain non-claims.

## Suggested next prompt

If the next package should widen behavior, choose one explicit reopen point:
broader computational effect semantics,
PoseGraph runtime/save-load/devtools,
projection realization,
engine/provider admission,
or final distribution/catalog decision.

## Plan update status

`plan/` 更新済み:
`plan/57-autonomous-computational-core-master-plan.md`

## Documentation.md update status

`Documentation.md` 更新済み:
minimal alpha-1 pattern verifier and no-promoted-reopen wording added

## progress.md update status

`progress.md` 更新済み:
latest package, workflow axes, validation floor, feature row, and recent log updated

## tasks.md update status

`tasks.md` 更新済み:
current task status, recommendation, and maintenance task row updated

## samples_progress.md update status

`samples_progress.md` 更新済み:
minimal verifier row, validation anchor, and recent validation log updated

## Reviewer findings and follow-up

- Local focused review found a real issue in the first verifier implementation:
  projection / engine expected IDs were stale guesses. Existing helper tests and `check-all` outputs were used as source-of-truth, then the verifier constants and unit tests were corrected.
- Local docs review found one stale deferred phrase in `docs/research_abstract/README.md` that still implied all Mir-owned computational sample/runtime evidence was absent. It was narrowed to broader future evidence beyond current bounded rows.
- No sub-agent reviewers were spawned in this turn because the latest user request did not explicitly request sub-agent/delegated work; validation was local.

## Skipped validations and reasons

- Full workspace-wide `cargo test` was not run because this package added Python verifier and docs only. The workflow-inclusive verifier did run the product alpha release-check focused Cargo tests:
  `mir-ast` product schema,
  `mir-runtime` session/devtools,
  and `mirrorea-cli` alpha CLI.
- Docker was not skipped in the workflow-inclusive check; Docker path was included by the product release check.

## Commit / push status

Pending at report write.

## Sub-agent session close status

No sub-agents were opened for this task.
