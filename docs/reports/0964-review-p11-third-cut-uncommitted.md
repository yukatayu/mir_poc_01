# 0964 — review P11 third-cut uncommitted changes

## Objective

Review the current uncommitted `P11` third-cut changes with maintainer focus on semantic correctness, regression risk, doc overclaiming, and stop-line adherence. Primary code focus: `crates/mirrorea-core/src/runtime.rs` and `crates/mirrorea-core/tests/runtime_substrate.rs`. Primary wording focus: the paired `P11` updates in snapshot, plan, roadmap, and reader-facing docs.

## Scope and assumptions

- This task is a review only, not an implementation task.
- The review is limited to the current uncommitted worktree state.
- The review treats helper-side Sugoroku bootstrap / join / leave behavior as the parity anchor.
- No normative judgment is taken from `progress.md`, `tasks.md`, `samples_progress.md`, or `plan/`; those are reviewed for overclaiming against code and specs.

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/reports/0960-p11-logical-multi-place-runtime-first-cut.md`
- `docs/reports/0962-p11-participant-place-runtime-shell-hardening.md`
- `docs/reports/0963-p11-shell-bootstrap-join-leave-parity.md`
- `scripts/sugoroku_world_samples.py`

## Actions taken

1. Read the required repo front-door documents and normative spec chain before judging the implementation.
2. Inspected the uncommitted diff for the targeted runtime/test files and the requested wording updates.
3. Compared the new Rust shell helpers against the helper-side Sugoroku membership anchor.
4. Ran the relevant crate tests and lightweight repo validation commands.
5. Recorded maintainer findings with concrete file/line references.

## Evidence / outputs / test results

- `cargo test -p mirrorea-core`
  - passed
  - carrier tests: `7 passed`
  - runtime substrate tests: `10 passed`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 961 numbered report(s).`
- `python3 scripts/check_source_hierarchy.py`
  - `required: 23`
  - `present: 23`
  - `missing: 0`
  - `all required paths present`
- `git diff --check`
  - no output

Review findings:

1. Medium: `add_initial_participant` / `add_participant` do not enforce helper-side participant-place association, so the new API names and docs overstate bootstrap/join parity. The helper anchor derives `ParticipantPlace[{principal}]` from the principal (`scripts/sugoroku_world_samples.py:353-367`, `1814-1817`, `2052-2053`), but the new Rust helpers accept any caller-supplied participant-place string and only force the place kind (`crates/mirrorea-core/src/runtime.rs:257-272`). This allows states such as principal `Alice` bound to `ParticipantPlace[Bob]`, while the updated docs describe these methods as parity helpers (`README.md:61`, `Documentation.md:69`, `docs/hands_on/current_phase_closeout_01.md:59`, `docs/research_abstract/mirrorea_future_axis_01.md:49`). The new tests cover only happy paths and do not reject a principal/place mismatch (`crates/mirrorea-core/tests/runtime_substrate.rs:186-229`).
2. Medium: the composite helpers are not failure-atomic. Both new methods register the participant place before calling membership mutation (`crates/mirrorea-core/src/runtime.rs:257-272`), while membership insertion can still fail for bootstrap-after-epoch, duplicate principal, or blank inputs (`crates/mirrorea-core/src/runtime.rs:53-75`, `141-150`). On those error paths, the place catalog remains mutated even though the bootstrap/join operation failed. That leaves stray participant places in the shell state and is not covered by the test suite, which exercises registry failure cases only directly and shell helper success cases only (`crates/mirrorea-core/tests/runtime_substrate.rs:59-93`, `186-229`).

## What changed in understanding

- The current third cut does stay inside the stated stop line: it does not move `WorldState`, `PlaceRuntime`, `MessageQueue`, `SugorokuState`, or visualization/timeline ownership into Rust.
- The main review risk is not stop-line collapse; it is that the new composite shell API claims stronger helper parity than the implementation currently enforces.

## Open questions

- Should `add_initial_participant` / `add_participant` derive `ParticipantPlace[{principal}]` directly, or should they validate the caller-supplied place against the current repo-local naming invariant?
- If caller-supplied participant-place strings must remain supported, should the docs stop calling these methods parity helpers and instead describe them as convenience wrappers over explicit membership/place mutation?

## Suggested next prompt

Address the two review findings in `crates/mirrorea-core/src/runtime.rs` and `crates/mirrorea-core/tests/runtime_substrate.rs`: enforce or validate principal-to-participant-place association for the new shell helpers, make the composite helper behavior failure-atomic or document the weaker contract explicitly, then sync the affected `P11` wording and add regression tests.
