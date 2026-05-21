# 2084 — P-FS-00 full-system-v1-roadmap-rebaseline

## Objective

Rebaseline the repository roadmap before implementation work, adding the Full System V1 source-first scope and replacing the current status/task snapshots so the final target, current alpha floor, next packages, and non-claims are explicit.

## Scope and assumptions

Scope was documentation, repository memory, dashboards, and validator scaffold only. No runtime implementation or sample root execution semantics were added.

Working assumptions:

- Mir source files are the intended semantic source of truth.
- `package.mir.json` remains alpha compatibility / package artifact.
- Product Alpha-1 release-candidate workflow remains useful but is not final product.
- Current computational and PoseGraph rows remain first-floor/helper evidence, not Rust-level language or runtime completion.
- Unity / Unreal / WASM / native / FFI are typed provider/backend boundaries, not semantic owners.
- Direct LLVM/native codegen remains later than typed IR, projection IR, and boundary schemas.

Requested path mismatches found and handled by using the actual repo paths:

- requested `specs/30-projection-backend-boundary.md`; actual `specs/30-projection-and-backend-boundary.md`.
- requested `specs/31-engine-adapter-boundary.md`; actual `specs/31-engine-wasm-ffi-adapter-boundary.md`.
- requested `specs/32-autonomous-execution-contract.md`; actual `specs/32-autonomous-execution-and-completion-contract.md`.
- requested `plan/57-autonomous-execution-roadmap.md`; actual `plan/57-autonomous-computational-core-master-plan.md`.

## Start state / dirty state

Initial `git status --short` was clean. The task baseline was recorded with the repo Discord skill before edits.

During the first edit pass, untracked `specs/33..38`, `plan/58..63`, and modified `progress.md` / `tasks.md` appeared in the worktree. Because the initial baseline was clean, these were treated as task-local changes and completed rather than reverted.

## Documents consulted

Read in required order:

- `README.md`
- `Documentation.md`
- `AGENTS.md`
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
- `specs/25-product-alpha1-public-boundary.md`
- `specs/26-operational-product-sample-suite.md`
- `specs/27-spatial-portal-and-shard-extension-boundary.md`
- `specs/28-mir-computational-core.md`
- `specs/29-transform-posegraph-semantics.md`
- `specs/30-projection-and-backend-boundary.md`
- `specs/31-engine-wasm-ffi-adapter-boundary.md`
- `specs/32-autonomous-execution-and-completion-contract.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `plan/51-operational-product-sample-roadmap.md`
- `plan/53-mir-computational-core-roadmap.md`
- `plan/54-transform-posegraph-roadmap.md`
- `plan/55-projection-backend-roadmap.md`
- `plan/56-engine-adapter-roadmap.md`
- `plan/57-autonomous-computational-core-master-plan.md`
- `samples/product-alpha1/README.md`
- `samples/product-alpha1/operational/README.md`
- `samples/product-alpha1/computational/README.md`
- `samples/product-alpha1/posegraph/README.md`
- all `sub-agent-pro/full-system-completion-001/*.md`

Also consulted:

- `plan/00-index.md`
- `samples/README.md`
- `scripts/README.md`
- `docs/hands_on/README.md`
- `docs/research_abstract/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`

## Actions taken

- Added Full System V1 normative docs `specs/33..38`.
- Added repository memory roadmaps `plan/58..63`.
- Replaced `progress.md` and `tasks.md` as current snapshots instead of append-only history.
- Added reader-facing Full System V1 hands-on and research summary docs.
- Updated root and index documentation to include the new roadmap.
- Updated sample and script dashboards, explicitly keeping `samples/full-system-v1/` planned-only.
- Updated documentation validators to require the new files and enforce `progress.md` / `tasks.md` heading contracts.
- Ran five read-only sub-agent reviews and patched all actionable findings.

## Files changed

Added:

- `specs/33-full-system-v1-scope.md`
- `specs/34-textual-mir-alpha-grammar.md`
- `specs/35-mir-typed-ir-and-interpreter.md`
- `specs/36-projection-ir-and-boundary-preservation.md`
- `specs/37-posegraph-runtime-semantics.md`
- `specs/38-engine-provider-admission.md`
- `plan/58-full-system-v1-roadmap.md`
- `plan/59-textual-mir-roadmap.md`
- `plan/60-computational-runtime-roadmap.md`
- `plan/61-posegraph-runtime-roadmap.md`
- `plan/62-projection-backend-roadmap.md`
- `plan/63-engine-provider-roadmap.md`
- `docs/hands_on/full_system_v1_roadmap_01.md`
- `docs/research_abstract/full_system_v1_roadmap_01.md`
- `docs/reports/2084-p-fs-00-full-system-v1-roadmap-rebaseline.md`

Updated:

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
- `docs/hands_on/README.md`
- `docs/research_abstract/README.md`
- `specs/00-document-map.md`
- `plan/00-index.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
date '+%Y-%m-%d %H:%M %Z'
git status --short
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/minimal_alpha1_patterns.py check-all --format json > /tmp/pfs00-minimal-alpha1-patterns.json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
rm -rf /tmp/mirrorea-alpha1-release
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release > /tmp/pfs00-product-alpha1-release.json
python3 scripts/operational_product_samples.py check-all --format json > /tmp/pfs00-operational-product-samples.json
rg -n '"status"|"product_alpha1_release_candidate_ready"|"failed_commands"|"failed"|"failures"' /tmp/pfs00-minimal-alpha1-patterns.json /tmp/pfs00-operational-product-samples.json /tmp/pfs00-product-alpha1-release.json
git diff --stat
git status --short
```

Sub-agent review tools were also used for five read-only review agents.

## Evidence / outputs / test results

Passed:

- `python3 -m unittest scripts.tests.test_validate_docs`
  - `Ran 17 tests ... OK`
- `python3 scripts/check_source_hierarchy.py`
  - `required: 250`, `present: 250`, `missing: 0`
- `python3 scripts/validate_docs.py`
  - documentation scaffold complete
- `cargo fmt --check`
  - passed
- `git diff --check`
  - passed
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json`
  - `status: accepted`, `failed: []`, `failures: []`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release`
  - final rerun `status: accepted`, `failed_commands: []`, `product_alpha1_release_candidate_ready: true`
- `python3 scripts/operational_product_samples.py check-all --format json`
  - `status: accepted`, `failed_commands: []`

One initial product release-check attempt failed with `output_dir_not_empty` because `/tmp/mirrorea-alpha1-release` already existed. The disposable output directory was removed and the exact anchor was rerun successfully.

## What changed in understanding

The repo now has a source-first Full System V1 roadmap with explicit FS-00..FS-11 milestones. The key rebaseline is that Product Alpha-1 and the operational suite are preserved as the runnable alpha floor, while final system completion requires textual Mir, typed IR, interpreter, effectful runtime integration, PoseGraph runtime, projection IR, provider admission, devtools panels, and a Full V1 release check.

Reviewer feedback tightened several boundaries: alpha grammar needs explicit transition/effect/capability carriers, typed IR needs source-carried capability requirements, PoseGraph runtime needs full `AnchorSwitch` fields, cut/save-load behavior must inherit existing negative obligations, projection preservation must include authority/provider/rollback policy families, and provider admission must prove native/WASM stop lines.

## Open questions

- Final public grammar remains open.
- Final ABI / SDK / engine adapter public surface remains open.
- Broader distribution beyond developer-built binary plus generated host launch bundle remains a user decision.
- Production WAN/federation and R3/R4 durable distributed save/load remain later gates.
- Sandboxed WASM or bounded native admission remains future work after provider admission policy is implemented.

## Suggested next prompt

`P-MIR-01 textual Mir alpha grammar`

## Plan update status

Updated. Added `plan/58..63` and updated `plan/00-index.md`.

## Documentation.md update status

Updated. Added the Full System V1 source-first roadmap to the current repo reading and stop-line summary.

## progress.md update status

Updated. Completely replaced with the Full System V1 snapshot including project axis, final ideal, current milestone position, FS-00..FS-11 milestone map, line snapshots, validation floor, non-claims, user-decision vs research-discovery split, macro phase map, feature maturity rows, and recent log.

## tasks.md update status

Updated. Completely replaced with current promoted package, ordered self-driven packages, macro phase reading, user decision gates, research discovery items, maintenance tasks, and non-promoted references.

## samples_progress.md update status

Updated. Added Full System V1 as boundary-fixed/no-sample-claim, planned sample roots, validation anchor context, and recent log entry while keeping planned roots non-runnable.

## Reviewer findings and follow-up

Five read-only sub-agent reviews were requested and completed:

- language/type reviewer found missing transition/effect/operator/capability grammar and IR carriers. Fixed in `specs/34`, `specs/35`, `specs/33`, `plan/60`, and `progress.md`.
- runtime/cut reviewer found missing `AnchorSwitch` runtime fields, weak PoseGraph close conditions, and weak cut/save-load inheritance. Fixed in `specs/35`, `specs/37`, `plan/60`, `plan/61`, `progress.md`, and `tasks.md`.
- projection/backend reviewer found missing authority/provider/rollback preservation and weak server/client negative rows. Fixed in `specs/36`, `plan/62`, `tasks.md`, `samples_progress.md`, `samples/README.md`, and `plan/58`.
- engine/provider reviewer found missing `authority_policy`, weak provider admission closeout, and missing arbitrary WASM non-claim. Fixed in `specs/38`, `plan/63`, `tasks.md`, `specs/33`, `README.md`, `Documentation.md`, and reader-facing Full V1 docs.
- docs/status reviewer found missing required snapshot headings/splits and lack of validator enforcement. Fixed in `progress.md`, `tasks.md`, `scripts/validate_docs.py`, `scripts/tests/test_validate_docs.py`, and `scripts/README.md`.

## Skipped validations and reasons

No requested validation was skipped. Outputs for the large JSON anchors were redirected to `/tmp/pfs00-*.json` to avoid excessive terminal output.

## Commit / push status

Pending at report write time because this report must be included in the commit. The final response records the resulting commit hash and push status.

## Sub-agent session close status

All five review sub-agent sessions were closed after their findings were integrated.
