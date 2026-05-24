# Report 2100 — P-SURF-00B surface-mir-brace-source-authority-rebaseline

- Date: 2026-05-24 14:25 JST
- Author / agent: Codex
- Scope: Surface Mir brace syntax / source-authority documentation rebaseline
- Decision levels touched: `L1`, `L2`

## Objective

Rebaseline the Surface Mir alpha documentation and roadmap around canonical place-scope syntax `S { ... }`, explicitly reject `S[ ... ]` including as sugar, restore `.mir` files as semantic source authority, add the Surface Mir specs/plans/guides requested by the handoff, replace `progress.md` / `tasks.md`, and prepare `P-SURF-01 surface brace parser` as the next package.

## Scope and assumptions

This was a docs/spec/roadmap package, not a parser/runtime implementation package. `S { ... }` is fixed as the canonical place-block syntax. `Role[instance] { ... }` is the role-instance block form. Bare role blocks and `S[ ... ]` are rejected. Surface Mir remains user-facing source and Core Mir remains the elaboration target. `package.mir.json` remains alpha compatibility / generated artifact, not semantic source authority.

The user-provided handoff was treated as working directive input, not as normative source. Existing repo canonical filenames were used for the user-listed alias paths: `specs/30-projection-and-backend-boundary.md`, `specs/31-engine-wasm-ffi-adapter-boundary.md`, `specs/32-autonomous-execution-and-completion-contract.md`, and `plan/57-autonomous-computational-core-master-plan.md`.

## Start state / dirty state

Initial dirty state included the untracked handoff directory `sub-agent-pro/surface-mir-brace-completion-001/`. It was read as requested and left untracked. The tracked worktree had no relevant pre-existing edits to the Surface docs/spec/plan files before this task's changes.

Resource preflight for the docs-heavy task:

- `date '+%Y-%m-%d %H:%M %Z'` -> `2026-05-24 14:00 JST`
- `df -h .` -> root filesystem 99G total, 68G used, 27G available, 72% used
- `free -h` -> 960Mi memory total, 336Mi available, 19Gi swap total, 18Gi free

## Documents consulted

- Root / status docs: `README.md`, `Documentation.md`, `AGENTS.md`, `progress.md`, `.docs/progress-task-axes.md`, `tasks.md`, `samples_progress.md`
- Required base specs: `specs/00-document-map.md`, `specs/01-charter-and-decision-levels.md`, `specs/02-system-overview.md`, `specs/03-layer-model.md`, `specs/09-invariants-and-constraints.md`
- Required subsystem specs: `specs/19-verification-stratification.md`, `specs/20-cut-save-load-semantics.md`, `specs/21-auth-layer-algebra.md`, `specs/22-observability-devtools-semantics.md`, `specs/23-typed-external-host-boundary.md`, `specs/24-operational-alpha05-alpha08-readiness.md`, `specs/25-product-alpha1-public-boundary.md`, `specs/28-mir-computational-core.md`, `specs/29-transform-posegraph-semantics.md`, `specs/30-projection-and-backend-boundary.md`, `specs/31-engine-wasm-ffi-adapter-boundary.md`, `specs/32-autonomous-execution-and-completion-contract.md`, `specs/34-textual-mir-alpha-grammar.md`
- Required plans: `plan/53-mir-computational-core-roadmap.md`, `plan/54-transform-posegraph-roadmap.md`, `plan/55-projection-backend-roadmap.md`, `plan/56-engine-adapter-roadmap.md`, `plan/57-autonomous-computational-core-master-plan.md`
- Handoff package: all Markdown under `sub-agent-pro/surface-mir-brace-completion-001/` and `sub-agent-pro/surface-mir-brace-completion-001/sample-blueprints/`
- Validator / report references: `docs/reports/TEMPLATE.md`, `scripts/validate_docs.py`, `scripts/check_source_hierarchy.py`, `scripts/tests/test_validate_docs.py`

## Actions taken

- Added `specs/39..43` for Surface Mir placement/elaboration, indexed state, role admission/capability grant, source patch hot-plug, and Surface Mir V1 alpha scope.
- Added `plan/64..68` for the Surface placement, indexed-state, role-admission, source-patch, and full Surface alpha roadmap.
- Updated `specs/34-textual-mir-alpha-grammar.md` to reject `S[ ... ]`, define `S { ... }` as the place-block syntax, define `Role[instance] { ... }` as the role-instance block form, add Surface alpha parser-target constructs, and scope those constructs as future Surface parser targets rather than existing Full System V1 parser evidence.
- Replaced `progress.md` and `tasks.md` with Surface Mir rebaseline snapshots that preserve the validator-required heading order.
- Updated root/status/index/sample/script docs so the new Surface docs are discoverable while planned sample/script roots remain explicitly not workflow-ready.
- Added hands-on and research guide pages for Surface Mir alpha and source patch hot-plug.
- Expanded `scripts/validate_docs.py`, `scripts/check_source_hierarchy.py`, and `scripts/tests/test_validate_docs.py` so the new Surface specs/plans/guides are structurally required.
- Addressed reviewer findings by adding: explicit `when ... fails ...` generated-failure containment, foreign nested `S { ... }` owner-directed request semantics, capability-ref grant-lineage validation, source-patch membership/witness frontier drift rejection, and direct-remote-store rejection wording.

## Files changed

Added:

- `docs/hands_on/source_patch_hotplug_01.md`
- `docs/hands_on/surface_mir_alpha_01.md`
- `docs/research_abstract/surface_mir_alpha_01.md`
- `plan/64-surface-mir-placement-roadmap.md`
- `plan/65-indexed-state-roadmap.md`
- `plan/66-role-admission-roadmap.md`
- `plan/67-source-patch-hotplug-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`
- `specs/39-surface-mir-placement-elaboration.md`
- `specs/40-indexed-state-semantics.md`
- `specs/41-role-admission-and-capability-grant.md`
- `specs/42-source-patch-hotplug-semantics.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `docs/reports/2100-p-surf-00b-surface-mir-brace-source-authority-rebaseline.md`

Updated:

- `README.md`
- `Documentation.md`
- `docs/hands_on/README.md`
- `docs/research_abstract/README.md`
- `plan/00-index.md`
- `progress.md`
- `samples/README.md`
- `samples_progress.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/tests/test_validate_docs.py`
- `scripts/validate_docs.py`
- `specs/00-document-map.md`
- `specs/34-textual-mir-alpha-grammar.md`
- `tasks.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `date '+%Y-%m-%d %H:%M %Z'`
- `df -h .`
- `free -h`
- `git status --short`
- `git diff --stat`
- `git diff --name-only`
- `rg -n "S\\[|S \\{|Surface Mir|surface_mir|full-system-v1-surface" ...`
- `python3 -m py_compile scripts/validate_docs.py scripts/check_source_hierarchy.py scripts/tests/test_validate_docs.py`
- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `cargo fmt --check`
- `git diff --check`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-00b`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-00b-final`
- `python3 scripts/operational_product_samples.py check-all --format json`
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json`

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_validate_docs`: passed, 18 tests.
- `python3 scripts/check_source_hierarchy.py`: passed, required 395, present 395, missing 0.
- `python3 scripts/validate_docs.py`: passed; `Documentation scaffold looks complete. Found 1252 numbered report(s).`
- `cargo fmt --check`: passed with exit code 0.
- `git diff --check`: passed with exit code 0 after removing trailing blank lines from `progress.md` and `tasks.md`.
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release`: failed as expected because the output directory already existed and the helper rejected non-empty output (`diagnostic_code: output_dir_not_empty`).
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-00b-final`: accepted; `product_alpha1_release_candidate_ready: true`, `product_alpha1_ready: true`, final product/API still not claimed.
- `python3 scripts/operational_product_samples.py check-all --format json`: accepted after report write; `failed_commands: []`.
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json`: accepted after report write; `status: accepted`.

## What changed in understanding

The rebaseline is not just a bracket-to-brace rewrite. The safer boundary is:

- `S { ... }` is a place block, not a role block.
- role-instance behavior uses `Role[instance] { ... }`.
- foreign nested `S { ... }` is readable Surface syntax for an owner-directed request/effect, not a way to acquire owner-local authority.
- generated communication needs an explicit source failure-row site; the alpha target is `when ... fails ...`.
- capability refs need admission/principal/target/epoch/incarnation lineage.
- source-patch activation must bind to the checked membership/witness frontier and reject or defer frontier drift.

## Open questions

- Exact final public grammar for `when ... fails ...` remains non-final; it is an alpha parser target for generated-failure containment.
- Exact AST type names for Surface parser implementation may change during `P-SURF-01`, but the semantic distinctions above must be preserved.
- Surface sample root `samples/full-system-v1-surface/` remains planned and should not be marked workflow-ready until parser/elaboration/helper rows exist.

## Suggested next prompt

`P-SURF-01 surface brace parser`: implement parser support for `S { ... }` place blocks, `Role[instance] { ... }` role-instance blocks, `state`, `visible`, `when ... fails ...`, admission statements, and rejection diagnostics for bare role blocks and `S[ ... ]`.

## Plan update status

`plan/` was updated. Added `plan/64..68` and updated `plan/00-index.md` so the Surface Mir roadmap is discoverable as repository memory while normative judgments remain in `specs/`.

## Documentation.md update status

`Documentation.md` was updated. It now records the Surface Mir alpha source-authority line, the canonical `S { ... }` decision, the non-claim for parser/runtime implementation, and the next promoted package.

## progress.md update status

`progress.md` was fully replaced per the handoff replacement model while preserving required heading order. It now points to `P-SURF-01 surface brace parser` as next promoted work and records `P-SURF-00B` as a docs/spec rebaseline, not executable Surface evidence.

## tasks.md update status

`tasks.md` was fully replaced per the handoff replacement model while preserving required heading order. It separates self-driven Surface packages, research-discovery items, user decision gates, and maintenance tasks.

## samples_progress.md update status

`samples_progress.md` was updated. It adds the Surface Mir alpha source-authority row and keeps `samples/full-system-v1-surface/` planned-only, not workflow-ready.

## Reviewer findings and follow-up

Three read-only reviewer agents were used.

- Syntax/type reviewer found missing generated-failure declaration syntax, missing parser-visible Surface constructs in `specs/34`, role-block wording ambiguity, and direct-remote-store ambiguity. Follow-up: added `when ... fails ...`, added Surface constructs to `specs/34`, clarified `Role[instance] { ... }` only, and stated capabilities authorize generated owner-directed requests, not direct remote stores.
- Runtime/security/indexed-state reviewer found foreign nested `S { ... }` could be read as ambient authority switch, source-patch activation lacked membership-frontier binding, and `capability_ref` could look like a bearer token. Follow-up: specified foreign nested place blocks elaborate to owner-directed requests, added patch frontier drift rejection, and added capability-ref lineage validation.
- Docs/status reviewer found the report was not yet present, `specs/34` needed parser-implementation scoping, and validators did not yet require the new Surface docs. Follow-up: this report was added, `specs/34` was scoped as a Surface alpha extension target rather than existing Full System V1 parser evidence, and structural validators/tests now require `specs/39..43`, `plan/64..68`, and the new guides.

## Skipped validations and reasons

No requested validation was skipped. The first optional Product Alpha release check used an existing non-empty output directory and failed before execution; it was rerun with `/tmp/mirrorea-alpha1-release-p-surf-00b` and accepted. Surface-specific executable validations are not run because `P-SURF-00B` does not create `surface_mir_samples.py`, a parser, or runtime helpers.

## Commit / push status

Pending at report write. The intended commit command is:

```bash
git commit --no-gpg-sign -m "docs: rebaseline Surface Mir brace syntax"
```

The final response records the resulting commit hash and push status.

## Sub-agent session close status

Sub-agent review sessions `019e5869-6e8c-70e0-87ea-6634f8186d9c`, `019e5869-8c12-7473-a6ca-f8f61315c10f`, and `019e5869-ab15-7b61-9f75-a62e7e4e0673` completed and were closed.
