# 2104 P-SURF-04 Auto Communication Publish / Observe

- Date: 2026-05-24
- Author / agent: Codex
- Scope: Surface Mir P-SURF-04 generated communication elaboration
- Decision levels touched: L2 alpha implementation evidence, L3 privacy annotation non-claim

## Objective

- Identifier: `P-SURF-04 auto-communication publish/observe`
- Package: Surface Mir brace complete autonomous implementation
- Report path: `docs/reports/2104-p-surf-04-auto-communication-publish-observe.md`

Implement the Surface Mir generated communication floor after P-SURF-03: visible remote reads / writes must elaborate into explicit Core IR `MessageEnvelope`, publish, observe, source-span, and failure-row evidence while preserving `S { ... }` as the canonical place-scope syntax.

## Scope and assumptions

- Scope is elaboration evidence only. This package does not claim runtime dispatch, local queue execution, final transport, role admission authority, source patch activation, or final public grammar/API.
- `.mir` files remain semantic source authority; `package.mir.json` remains an alpha artifact.
- `visible observer_safe` without `fields { ... }` is treated as whole-state observer-safe for the alpha elaborator; `fields { ... }` narrows the generated communication surface.
- P-SURF-04 uses a narrow alpha private-looking-name guard only when cross-locus auto communication would be generated. This is not a final privacy annotation system.
- `TypeMismatch` typechecker discharge remains outside this package and is recorded as a non-claim.

## Start state / dirty state

Started from the pushed P-SURF-03 closeout on branch `main` with untracked `sub-agent-pro/surface-mir-brace-completion-001/` present. That handoff directory was intentionally left untracked and unstaged. P-SURF-04 began with clean committed P-SURF-03 sources plus local P-SURF-04 edits made in this task.

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
- Reviewer findings from sub-agent `019e5935-a47f-7580-a658-618251e81da2`

## Actions taken

- Added explicit Core IR rows for generated `MessageEnvelope`, publication, and observation output in `crates/mir-semantics::surface_to_core_elaboration`.
- Added generated communication edges for `message_envelope`, `auto_publish`, and `auto_observe`.
- Added `VisibilityDenied` to required failure rows when visible generated communication is emitted.
- Rejected non-visible/private-field cross-locus communication with `private_field_auto_publish_rejected`.
- Preserved source spans for generated message, publish, observe, and edge rows.
- Added elaboration samples `ELAB-03`, `ELAB-09`, and `ELAB-10`.
- Updated `scripts/surface_mir_samples.py` projection output to include message envelope, publication, and observation summaries.
- Updated `scripts/surface_mir_release_check.py` and surface helper tests to the 24-row P-SURF-04 matrix.
- Updated documentation snapshots and roadmap/status docs to mark P-SURF-04 closed and P-SURF-05 next.
- Addressed reviewer findings before close:
  - whole-record `visible observer_safe` now permits field-level generated communication;
  - private-looking-field rejection no longer fires at declaration time;
  - matrix status and `samples_progress.md` timestamp were synchronized.

## Files changed

- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`
- `scripts/surface_mir_samples.py`
- `scripts/surface_mir_release_check.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/tests/test_surface_mir_release_check.py`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `samples/full-system-v1-surface/elaboration/**`
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
- `specs/39-surface-mir-placement-elaboration.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/00-index.md`
- `plan/64-surface-mir-placement-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`

## Commands run

- `python3 -m unittest scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `cargo fmt`
- `cargo fmt --check`
- `cargo test -p mir-ast --test surface_mir_parser -- --nocapture`
- `cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture`
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`
- `python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check`
- `python3 scripts/surface_mir_samples.py check-all --format json`
- `python3 scripts/surface_mir_authoring_check.py check-all --format json`
- `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-p-surf-04`
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-04`
- `python3 scripts/operational_product_samples.py check-all --format json`
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json`
- `git diff --check`

## Evidence / outputs / test results

- `scripts.tests.test_validate_docs`: 18 tests passed.
- `scripts/check_source_hierarchy.py`: 466 required paths present, 0 missing.
- `scripts/validate_docs.py`: documentation scaffold complete.
- `cargo fmt --check`: passed.
- `mir-ast surface_mir_parser`: 13 tests passed.
- `mir-semantics indexed_state_semantics`: 7 tests passed.
- `mir-semantics surface_to_core_elaboration`: 13 tests passed after reviewer fixes.
- Surface helper unit tests: 26 tests passed.
- `scripts/surface_mir_samples.py check-all`: 24 rows passed, failed `[]`, workflow-ready `false`.
- `scripts/surface_mir_authoring_check.py check-all`: 24 sources accepted as `.mir` authority.
- `scripts/surface_mir_release_check.py`: accepted; output written to `/tmp/mirrorea-surface-release-p-surf-04`.
- `scripts/product_alpha1_release_check.py`: accepted; output written to `/tmp/mirrorea-alpha1-release-p-surf-04`.
- `scripts/operational_product_samples.py`: accepted.
- `scripts/minimal_alpha1_patterns.py`: accepted.

## What changed in understanding

Generated communication must be explicit in Core IR even when Surface syntax hides the communication ceremony from the author. `visible observer_safe` has two alpha meanings: without `fields`, it opens whole-state observer-safe communication; with `fields`, it narrows communication. Private-field rejection must be tied to generated cross-locus communication rather than to declaration parsing, otherwise local-only declarations become false negatives.

## Open questions

- Final privacy annotation syntax remains unresolved; P-SURF-04 only uses a narrow alpha guard.
- Runtime `MessageEnvelope` dispatch, queueing, retry, and transport are not implemented here.
- `TypeMismatch` typechecker discharge remains a later elaboration/typechecking widening.
- Role admission remains pending for P-SURF-05.
- Source patch activation remains pending for P-SURF-06.

## Suggested next prompt

`P-SURF-05 role admission capability grant`

## Plan update status

Updated `plan/00-index.md`, `plan/64-surface-mir-placement-roadmap.md`, and `plan/68-surface-full-system-v1-roadmap.md` to mark P-SURF-04 as generated communication evidence and P-SURF-05 as next.

## Documentation.md update status

Updated. `Documentation.md` now identifies P-SURF-04 as the generated `MessageEnvelope` / publish / observe / private-field rejection evidence floor and keeps runtime dispatch, role admission, and source patch hot-plug as non-claims.

## progress.md update status

Updated. `progress.md` records P-SURF-04 closure, current runnable commands, implemented samples, current non-claims, and next gap `P-SURF-05`.

## tasks.md update status

Updated. `tasks.md` now makes `P-SURF-05 role admission capability grant` the current next autonomous package.

## samples_progress.md update status

Updated. `samples_progress.md` records the 24-row Surface Mir source matrix, P-SURF-04 generated communication evidence, and current validation commands. Timestamp synchronized to `2026-05-24 18:05 JST`.

## Reviewer findings and follow-up

- Code mapper sub-agent `019e5923-3446-7922-b226-c6535673e880` found the expected implementation surfaces and warned to keep parser behavior unchanged and `S[ ... ]` rejected. Follow-up: parser was not widened; existing parser tests still pass.
- Reviewer sub-agent `019e5935-a47f-7580-a658-618251e81da2` reported:
  - whole-record `visible observer_safe` incorrectly rejected field access;
  - private-looking-field rejection fired eagerly at declaration time and used an undocumented broad substring heuristic;
  - matrix status and dashboard timestamp were stale.
- Follow-up implemented:
  - whole-record visible field read/write tests added and passing;
  - eager declaration-time private rejection removed;
  - private-looking guard narrowed and documented as alpha-only, communication-generation-time behavior;
  - matrix status, dashboard timestamp, and helper matrix test updated.
- Reviewer session was closed after findings were incorporated.

## Skipped validations and reasons

No requested P-SURF-04 validations were skipped. The package does not run P-SURF-05/P-SURF-06 runtime/admission/source-patch tests because those packages remain next in sequence.

## Commit / push status

Pending at report creation. The intended commit message is `p-surf-04: add surface auto communication`; final commit hash and push status are reported in the package close response.

## Sub-agent session close status

- `019e5923-3446-7922-b226-c6535673e880`: completed earlier as code mapper.
- `019e5935-a47f-7580-a658-618251e81da2`: completed reviewer pass and was closed after follow-up fixes.
