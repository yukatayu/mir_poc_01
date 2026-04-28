# 0963 — P11 shell bootstrap/join/leave parity cut

## Objective

`P11` logical multi-place runtime tranche の current third cut として、participant-place-kind-gated `LogicalPlaceRuntimeShell` に principal-derived `ParticipantPlace[{principal}]` shell-backed `add_initial_participant` / `add_participant` / `leave_participant` helper を追加し、helper-side Sugoroku runtime が already 使っている bootstrap / join / leave membership boundary を queue や world state を持ち込まずに `mirrorea-core` へ寄せる。

## Scope and assumptions

- この task は `P11` package close ではなく current third-cut parity hardening である。
- helper-side `WorldState`、`PlaceRuntime`、`MessageQueue`、`SugorokuState`、turn order、visualization / telemetry ownership は current scope 外に残す。
- participant place naming は current repo-local parity helper の invariant としてのみ使い、final public place taxonomy を freeze した意味にはしない。

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
- `docs/reports/0962-p11-participant-place-runtime-shell-hardening.md`
- `docs/reports/0964-review-p11-third-cut-uncommitted.md`
- `docs/reports/0965-review-p11-third-cut-rerun-after-fixes.md`
- `scripts/sugoroku_world_samples.py`

## Actions taken

1. Read the helper-side bootstrap / join / leave anchor in `scripts/sugoroku_world_samples.py` and kept only the membership/place frontier.
2. Added RED tests that asked `LogicalPlaceRuntimeShell` for `add_initial_participant`, `add_participant`, and `leave_participant` before those methods existed.
3. Implemented the composite helper methods so the shell now derives `ParticipantPlace[{principal}]`, keeps registration failure-atomic, and performs bootstrap / join / leave membership mutation without importing queue, world, or game ownership.
4. Updated snapshot docs, repository memory, and reader-facing summaries so `P11` now reads as a current third cut rather than stopping at the participant-place gate alone.
5. Left the stop line explicit in docs: no `WorldState`, `PlaceRuntime`, `MessageQueue`, `SugorokuState`, timeline, projection, hot-plug lifecycle, or visualization catalog ownership was moved into `mirrorea-core`.

## Files changed

- `crates/mirrorea-core/src/runtime.rs`
- `crates/mirrorea-core/tests/runtime_substrate.rs`
- `scripts/sugoroku_world_samples.py`
- `scripts/tests/test_sugoroku_world_samples.py`
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
- `docs/research_abstract/hands_on_sugoroku_05_join_leave.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `docs/reports/0963-p11-shell-bootstrap-join-leave-parity.md`
- `docs/reports/0964-review-p11-third-cut-uncommitted.md`
- `docs/reports/0965-review-p11-third-cut-rerun-after-fixes.md`

## Commands run and exact outputs

- `source scripts/env/mirrorea_storage_env.sh --ensure-dirs >/dev/null && cargo test -p mirrorea-core`
  - RED first:
    - `no method named 'add_initial_participant' found for struct 'LogicalPlaceRuntimeShell'`
    - `no method named 'add_participant' found for struct 'LogicalPlaceRuntimeShell'`
    - `no method named 'leave_participant' found for struct 'LogicalPlaceRuntimeShell'`
  - GREEN after implementation:
    - `test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
    - `test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
- `source scripts/env/mirrorea_storage_env.sh --ensure-dirs >/dev/null && cargo fmt --all`
  - exact output:
    - no output
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - exact output:
    - `Ran 42 tests in 0.118s`
    - `OK`
- `python3 scripts/sugoroku_world_samples.py run 06_leave_non_owner --format json`
  - output anchors:
    - `"member": "Carol"`
    - `"membership_epoch": 1`
    - `"member_incarnation": 1`
    - `"pending_actions_invalidated": true`
- `python3 scripts/sugoroku_world_samples.py run 07_owner_leave_reassign --format json`
  - output anchors:
    - `"left_member": "Bob"`
    - `"new_dice_owner": "Alice"`
    - `"phase_after": "Running"`
    - `"membership_epoch": 2`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - output anchors:
    - `membership_model.source_of_truth = "MembershipRegistry"`
    - `runtime_components` still include helper-local `PlaceRuntime` / `MessageQueue` / `SugorokuState`
    - `layer_signatures` now include `membership_owner_reassignment` with `dice_owner:Bob->Alice`
- `python3 scripts/check_source_hierarchy.py`
  - exact output:
    - `required: 23`
    - `present: 23`
    - `missing: 0`
    - `all required paths present`
- `python3 scripts/validate_docs.py`
  - exact output:
    - `Documentation scaffold looks complete.`
    - `Found 963 numbered report(s).`
- `git diff --check`
  - exact output:
    - no output

## Evidence / outputs / test results

- `LogicalPlaceRuntimeShell` now owns a minimal membership/place composite helper layer:
  - bootstrap participant
  - late-join participant
  - leave participant
- The composite helper now truly mirrors helper-side place naming:
  - `Alice -> ParticipantPlace[Alice]`
  - `Bob -> ParticipantPlace[Bob]`
- Composite helper failures are now rollback-safe for newly registered participant places.
- Sugoroku helper evidence is now internally consistent for owner leave:
  - runtime result says `new_dice_owner = Alice`
  - `membership_owner_reassignment` layer signature now says `dice_owner:Bob->Alice`
- The helper stays within the `P11` stop line:
  - no queue ownership
  - no world ownership
  - no game ownership
  - no turn-order or dice-owner policy
- The runtime substrate tests mirror the helper-side boundary without importing helper-side world/game state, and the helper-side unittest suite now has direct regressions for `06_leave_non_owner` and `07_owner_leave_reassign`.
- reviewer follow-up:
  - `0964` found the parity-mismatch and failure-atomicity gaps in the initial third cut
  - `0965` reran after fixes and reported no findings

## What changed in understanding

- The next safe `P11` ratchet after participant-place gating is not queue wrapping; it is parity for the non-domain membership frontier the helper already depends on.
- Composite shell helpers can advance `P11` materially as long as they stop before any sample-specific state machine or transport responsibility.

## Open questions

- Should the next `P11` slice add a shell-side batch bootstrap carrier, or is the current per-participant helper enough until queue / host boundary packages reopen?
- When the repo later widens place taxonomy, should these helper methods remain on `LogicalPlaceRuntimeShell`, move behind a higher-level bootstrap carrier, or stay as repo-local convenience methods?

## Suggested next prompt

`P11` logical multi-place runtime tranche を続け、shell-backed bootstrap / join / leave parity cut の次として、helper-side kept-later boundary を queue へ広げずにどこまで tighten できるかを整理してください。`WorldState` / `PlaceRuntime` / `MessageQueue` / `SugorokuState` / timeline / visualization ownershipはまだ移さず、review、docs、dashboard、report を同期してください。
