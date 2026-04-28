# 0961 — P11 runtime shell semantic review

## Objective

Review the current uncommitted `P11` substep in `crates/mirrorea-core/src/runtime.rs`, `crates/mirrorea-core/src/lib.rs`, and `crates/mirrorea-core/tests/runtime_substrate.rs` for semantic bugs, regressions, stop-line violations, missing tests, and misleading API semantics.

## Scope and assumptions

- This was a read-only review of the three target files plus the required normative context.
- No source-code changes were made to the reviewed implementation.
- The review criteria were the current `P11` stop lines: no `WorldState`, `PlaceRuntime`, `MessageQueue`, `SugorokuState`, or event / timeline / visualization catalog freeze.
- `progress.md` 更新不要
- `tasks.md` 更新不要
- `samples_progress.md` 更新不要
- `plan/` 更新不要

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `docs/reports/0960-p11-logical-multi-place-runtime-first-cut.md`
- `scripts/sugoroku_world_samples.py`

## Actions taken

1. Read the required repo front-door and normative documents in the order mandated by `AGENTS.md`.
2. Read the prior `P11` first-cut report to anchor the intended scope of the uncommitted follow-up.
3. Inspected the uncommitted diff for the three target files.
4. Read the current contents of the three target files with line numbers for precise review references.
5. Cross-checked the new shell semantics against the helper-side Sugoroku runtime boundary and participant-place naming.
6. Ran `cargo test -p mirrorea-core` to confirm the worktree is currently green despite the review finding.

## Files changed

- `docs/reports/0961-p11-runtime-shell-review.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short`
- `sed -n '1,220p' README.md`
- `sed -n '1,260p' Documentation.md`
- `sed -n '1,260p' specs/00-document-map.md`
- `sed -n '1,260p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,260p' specs/02-system-overview.md`
- `sed -n '1,260p' specs/03-layer-model.md`
- `sed -n '1,260p' specs/09-invariants-and-constraints.md`
- `sed -n '1,260p' specs/05-mirrorea-fabric.md`
- `sed -n '1,260p' plan/00-index.md`
- `sed -n '1,220p' plan/09-helper-stack-and-responsibility-map.md`
- `sed -n '1,220p' plan/16-shared-space-membership-and-example-boundary.md`
- `sed -n '1,220p' docs/reports/0960-p11-logical-multi-place-runtime-first-cut.md`
- `git diff -- crates/mirrorea-core/src/runtime.rs crates/mirrorea-core/src/lib.rs crates/mirrorea-core/tests/runtime_substrate.rs`
- `nl -ba crates/mirrorea-core/src/runtime.rs | sed -n '1,320p'`
- `nl -ba crates/mirrorea-core/src/lib.rs | sed -n '1,220p'`
- `nl -ba crates/mirrorea-core/tests/runtime_substrate.rs | sed -n '1,320p'`
- `rg -n "MembershipRegistry|PlaceCatalog|LogicalPlaceRuntimeShell|P11|multi-place|logical place runtime|logical runtime shell" specs plan docs/reports crates/mirrorea-core -g '*.md'`
- `rg -n "participant_place|ParticipantPlace|membership_model|member place|place kind" docs/reports/0960-p11-logical-multi-place-runtime-first-cut.md plan/16-shared-space-membership-and-example-boundary.md plan/09-helper-stack-and-responsibility-map.md specs/11-roadmap-and-workstreams.md scripts/sugoroku_world_samples.py -g '*.md'`
- `nl -ba scripts/sugoroku_world_samples.py | sed -n '340,390p'`
- `nl -ba scripts/sugoroku_world_samples.py | sed -n '1808,1835p'`
- `cargo test -p mirrorea-core`

## Evidence / outputs / test results

- `cargo test -p mirrorea-core` passed:
  - carrier tests: `7 passed`
  - `runtime_substrate` tests: `7 passed`
- No stop-line violation was found in the reviewed diff:
  - no `WorldState`
  - no `PlaceRuntime`
  - no `MessageQueue`
  - no `SugorokuState`
  - no event / timeline / visualization catalog freeze
- Review finding:
  - `LogicalPlaceRuntimeShell::add_initial_member` and `LogicalPlaceRuntimeShell::add_member` only check that `participant_place` is present in the catalog, not that it is registered as a participant place kind.
  - `ensure_registered_place()` treats any catalog entry as valid membership target (`crates/mirrorea-core/src/runtime.rs:255-261`), so `WorldServerPlace` or another non-participant locus can be used as a member’s `participant_place`.
  - The helper-side anchor still models membership as `ParticipantPlace[{principal}]` and registers those places with kind `ParticipantPlace` before inserting members (`scripts/sugoroku_world_samples.py:353-372`, `1809-1817`).
  - The new tests cover happy-path registration and missing-place rejection, but they do not cover wrong-kind rejection (`crates/mirrorea-core/tests/runtime_substrate.rs:128-170`).

## What changed in understanding

- The uncommitted shell remains structurally thin and does not cross the `P11` stop line by itself.
- The main risk is not premature runtime expansion; it is that the shell’s current API overstates the meaning of “registered place” for membership mutation.
- In this repository, the relevant invariant is stronger than “the place exists”: membership targets are participant places, not arbitrary execution loci.

## Open questions

- Should `LogicalPlaceRuntimeShell` enforce a concrete participant-place kind such as `ParticipantPlace`, or is place-kind normalization still intentionally unresolved at this stage?
- If kind normalization remains unresolved, should the API avoid the stronger `participant_place` naming until that invariant is enforced?

## Suggested next prompt

`LogicalPlaceRuntimeShell` の review finding を踏まえて、membership mutation が `participant` 系 place kind にしか向かないようにするか、もしくは API 名を弱めて誤読を避けるかを source-backed に決めてください。対応する negative test も `crates/mirrorea-core/tests/runtime_substrate.rs` に追加し、P11 stop line を越えないことを確認してください。
