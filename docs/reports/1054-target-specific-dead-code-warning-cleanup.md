# 1054 - Target-specific dead-code warning cleanup

## Objective

Reduce existing Cargo `dead_code` warning noise from target-specific shared test/example support modules without deleting real helper code or widening sample behavior.

## Scope and assumptions

- Scope is test/example support warning hygiene only.
- The warned helpers are real support surfaces used by other test or example targets; removal is out of scope.
- The cleanup must stay item-level and behavior-neutral.
- Stop line: this package does not change parser semantics, sample semantics, proof obligations, public API, or public ABI.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `AGENTS.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/06-prismcascade-positioning.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `samples/README.md`
- `scripts/README.md`
- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_predicate_fragment_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_perform_head_spike.rs`
- `crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`
- `crates/mir-semantics/examples/support/current_l2_formal_hook_support.rs`
- `crates/mir-semantics/tests/current_l2_lean_theorem_stub_actual_probe.rs`
- `crates/mir-semantics/tests/current_l2_detached_bundle_support.rs`
- `crates/mir-semantics/tests/current_l2_formal_hook_support.rs`
- `sub-agent-pro/mir_poc_01_research_handoff_2026-04-30.md`

## Actions taken

- Reproduced the warning sites from `cargo test -p mir-ast` and `cargo test -p mir-semantics`.
- Ran targeted red checks with `RUSTFLAGS="-D warnings"` to prove the selected warning-source targets failed before the cleanup.
- Mapped warned helpers to real consumers and confirmed the code is not globally dead.
- Added item-level `#[allow(dead_code)]` to target-specific shared support items:
  - `load_fixture_perform_head`
  - `parse_fixture_perform_head`
  - detached bundle artifact structs and mapping helpers
  - detached-bundle formal-hook constants / builder / validator
- Avoided artificial test use and avoided deleting helper code.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` with the focused cleanup and validation evidence.

## Files changed

- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`
- `crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`
- `crates/mir-semantics/examples/support/current_l2_formal_hook_support.rs`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/1054-target-specific-dead-code-warning-cleanup.md`

## Evidence / outputs / test results

- Package start:
  - `git status --short` -> clean.
  - `git branch --show-current` -> `main`.
  - `git log -1 --oneline` -> `26a782f Refresh full validation evidence`.
  - `date '+%Y-%m-%d %H:%M %Z'` -> `2026-05-01 09:34 JST`.
- Warning reproduction:
  - `cargo test -p mir-ast 2>&1 | rg ...` showed `load_fixture_perform_head` and `parse_fixture_perform_head` warnings in `current_l2_stage3_predicate_fragment_spike_support.rs`.
  - `cargo test -p mir-semantics 2>&1 | rg ...` showed detached bundle artifact/support warnings and detached-bundle formal-hook branch warnings when compiling `current_l2_lean_theorem_stub_actual_probe`.
- Red checks before cleanup:
  - `RUSTFLAGS="-D warnings" cargo test -p mir-ast --test current_l2_stage3_predicate_fragment_spike` -> failed on the two `mir-ast` warning sites.
  - `RUSTFLAGS="-D warnings" cargo test -p mir-semantics --test current_l2_lean_theorem_stub_actual_probe` -> failed on the 16 `mir-semantics` warning sites.
- Code-mapper sub-agent result:
  - warned helpers are target-specific unused, not globally dead;
  - do not remove;
  - prefer item-level `#[allow(dead_code)]`;
  - guard with warning-source targets plus package-level `cargo test`.
- Green checks after cleanup:
  - `RUSTFLAGS="-D warnings" cargo test -p mir-ast --test current_l2_stage3_predicate_fragment_spike` -> pass; 7 tests passed.
  - `RUSTFLAGS="-D warnings" cargo test -p mir-semantics --test current_l2_lean_theorem_stub_actual_probe` -> pass; 1 test passed, Lean 4.29.1 executed.
  - `cargo test -p mir-ast` -> pass; 73 tests passed.
  - `cargo test -p mir-semantics` -> pass; 79 tests passed.
  - `cargo fmt --check` -> pass.
  - `git diff --check` -> pass before report write.
- Post-report docs / diff validation:
  - `python3 scripts/check_source_hierarchy.py` -> pass; required 24 / present 24 / missing 0.
  - `python3 scripts/validate_docs.py` -> pass; documentation scaffold complete, 1052 numbered reports found.
  - `git diff --check` -> pass.

## What changed in understanding

- The warnings were caused by shared support modules being compiled by narrower targets that intentionally exercise only one branch.
- The helpers remain live in other targets, so deletion would be unsafe.
- Item-level allows are a better fit than artificial uses because they document the target-specific nature of the warning without changing test coverage.

## Open questions

- Actual `U1` commitment remains user-facing and open.
- Whether other warning classes should be made `-D warnings` clean remains a future maintenance question, not a requirement for this package.

## Suggested next prompt

Continue autonomous maintenance by rerunning the docs floor after this warning cleanup, then inspect another narrow stale-doc or validation guardrail package if no blocker appears.

## Plan update status

`plan/` 更新不要。No roadmap, repository memory, boundary, or sequencing interpretation changed.

## progress.md update status

`progress.md` 更新済み。Recent log records this warning cleanup.

## tasks.md update status

`tasks.md` 更新済み。Current task map records this as maintenance-only and not a new implementation queue.

## samples_progress.md update status

`samples_progress.md` 更新済み。Recent validation records the warning-noise focused floor without changing runnable sample progress.

## Skipped validations and reasons

- Full validation floor was not rerun because this package touched only test/example support warning attributes and snapshot docs.
- Sample closeouts were not rerun because no sample source, runner behavior, or runtime surface changed.

## Commit / push status

Pending at report write. This report is intended to be committed with the package using `git commit --no-gpg-sign` and pushed immediately after post-report validation.

## Sub-agent session close status

Code-mapper sub-agent `019de0f1-4833-7c60-a180-3fc09b0246c0` (`Poincare`) mapped the warning sources, confirmed deletion was unsafe, recommended item-level allows, and was closed after incorporation.
