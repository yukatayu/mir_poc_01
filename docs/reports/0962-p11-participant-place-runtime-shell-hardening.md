# 0962 — P11 participant-place runtime shell hardening

## Objective

`P11` logical multi-place runtime tranche の current second cut として追加した `LogicalPlaceRuntimeShell` を harden し、membership mutation が arbitrary registered place ではなく participant-place kind にしか向かわないことを `mirrorea-core` 側で明示する。あわせて snapshot docs / repository memory / runnable dashboard を current shell cut に同期する。

## Scope and assumptions

- この task は `P11` package close ではなく current second-cut hardening である。
- `WorldState`、`PlaceRuntime`、`MessageQueue`、`SugorokuState`、event / timeline / visualization catalog は current scope 外に残す。
- participant-place kind の exact string は current repo-local shell invariant としてのみ扱い、final public place-kind taxonomy を freeze したことにはしない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/reports/0960-p11-logical-multi-place-runtime-first-cut.md`
- `docs/reports/0961-p11-runtime-shell-review.md`
- `scripts/sugoroku_world_samples.py`

## Actions taken

1. Read the `0961` review finding and confirmed that the shell only checked “registered place,” not participant-place kind.
2. Tightened `LogicalPlaceRuntimeShell` so `add_initial_member` / `add_member` require the target place to be registered with kind `ParticipantPlace`.
3. Added a regression test that proves a member cannot be attached to `WorldServerPlace` even when that place is registered in the catalog.
4. Synchronized front-door docs, snapshot docs, repository memory, hands-on closeout, and reader-facing summary to say that the current shell cut is participant-place-kind-gated rather than merely registered-place-gated.
5. Kept the stop line explicit: no queue, world, game, turn-order, timeline, hot-plug lifecycle, or visualization catalog ownership moved into `mirrorea-core`.

## Files changed

- `crates/mirrorea-core/src/runtime.rs`
- `crates/mirrorea-core/tests/runtime_substrate.rs`
- `README.md`
- `Documentation.md`
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
- `docs/reports/0962-p11-participant-place-runtime-shell-hardening.md`

## Commands run and exact outputs

- `source scripts/env/mirrorea_storage_env.sh --ensure-dirs >/dev/null && cargo test -p mirrorea-core`
  - fresh green after invariant hardening:
    - `test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
    - `test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `source scripts/env/mirrorea_storage_env.sh --ensure-dirs >/dev/null && cargo fmt --all`
  - exact output:
    - no output
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - output anchors:
    - `place_model.description = "single OS process logical multi-place emulator"`
    - `membership_model.source_of_truth = "MembershipRegistry"`
    - `runtime_components` still include helper-local `PlaceRuntime` / `MessageQueue` / `SugorokuState`
- `python3 scripts/check_source_hierarchy.py`
  - exact output:
    - `required: 23`
    - `present: 23`
    - `missing: 0`
    - `all required paths present`
- `python3 scripts/validate_docs.py`
  - exact output:
    - `Documentation scaffold looks complete.`
    - `Found 960 numbered report(s).`
- `git diff --check`
  - exact output:
    - no output

## Evidence / outputs / test results

- `LogicalPlaceRuntimeShell` no longer equates “membership target” with any arbitrary registered place.
- The current shell invariant is now explicit:
  - missing participant place registration fails
  - wrong place kind fails
  - happy-path registered participant places still work
- The new negative test protects the exact semantic hole found in `0961`.
- `cargo test -p mirrorea-core` now covers:
  - carrier tests `7/7`
  - runtime substrate tests `8/8`
- The docs now keep three layers distinct:
  - helper-side Sugoroku runtime still owns world/game/queue/timeline concerns
  - `mirrorea-core` owns substrate + thin shell only
  - final public place taxonomy remains deferred

## What changed in understanding

- The next safe `P11` ratchet is not “any runtime shell,” but a shell whose names and invariants agree with the helper-side participant/place split already documented in the repo.
- The participant-place constraint is strong enough to belong in the current shell cut, while queue and world ownership are still too entangled with later packages.

## Open questions

- Should the next `P11` slice add shell-backed bootstrap / join / leave parity helpers, or stay at snapshot-only shell composition a little longer?
- When place-kind normalization is widened beyond the current helper naming, should `ParticipantPlace` remain a shell constant, move behind a type wrapper, or become caller-supplied configuration?

## Suggested next prompt

`P11` logical multi-place runtime tranche を続け、participant-place-kind-gated `LogicalPlaceRuntimeShell` の上に shell-backed bootstrap / join / leave parity cut を追加してください。`WorldState` / `PlaceRuntime` / `MessageQueue` / `SugorokuState` / turn timeline / visualization ownershipはまだ移さず、helper-side membership boundary と docs / dashboard / report を同期してください。
