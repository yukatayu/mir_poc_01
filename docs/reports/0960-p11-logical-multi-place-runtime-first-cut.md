# 0960 — P11 logical multi-place runtime first cut

## Objective

`P11` logical multi-place runtime tranche を start し、full helper-local emulator を premature に移さずに、`MembershipRegistry` typed source-of-truth substrate と `PlaceCatalog` logical multi-place catalog substrate だけを `crates/mirrorea-core` に actualize する。`WorldState`、`PlaceRuntime`、`MessageQueue`、`SugorokuState`、event / timeline / visualization catalog は kept-later boundary に残す。

## Scope and assumptions

- この task は `P11` package close ではなく、current first cut の start である。
- shared-space runtime の domain semantics 全体を Rust に移さず、non-domain frontier だけを切り出す。
- turn-order insertion policy、session/auth/signature、queue/bridge/failure/route-trace、hot-plug lifecycle ownership、avatar `state_timeline` / `anchor_switch` は current scope 外とする。
- `P10` で actualize した `LayerSignature` / `PrincipalClaim` / `AuthEvidence` / `MessageEnvelope` narrow carrier ownership cut は維持する。

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
- `scripts/sugoroku_world_samples.py`
- `scripts/avatar_follow_samples.py`
- `scripts/network_transport_samples.py`
- `docs/reports/0957-p10-mirrorea-core-first-tranche-extraction-candidates.md`
- `docs/reports/0958-p10-mirrorea-core-runtime-carrier-boundary-memo.md`
- `docs/reports/0959-p10-mirrorea-core-first-real-implementation-tranche.md`

## Actions taken

1. Read the P11 task map and the shared-space membership boundary docs to keep the source-of-truth / helper-local split explicit.
2. Used the already-completed explorer findings to narrow the first cut to `MembershipRegistry` and place catalog only.
3. Wrote a RED test in `crates/mirrorea-core/tests/runtime_substrate.rs` that imports `MembershipRegistry` and `PlaceCatalog` before the crate exported them; confirmed the unresolved-import failure.
4. Added `crates/mirrorea-core/src/runtime.rs` with:
   - `MemberRecord`
   - `MemberSnapshot`
   - `MembershipSnapshot`
   - `MembershipRegistry`
   - `PlaceCatalog`
   - `PlaceCatalogSnapshot`
5. Kept the API typed and non-domain:
   - membership methods require explicit `principal` and `participant_place`
   - duplicate member insertion fails fast because rejoin semantics remain unresolved
   - place registration accepts identical duplicates but rejects kind drift
6. Updated front-door docs, snapshot docs, repository memory, and the runnable dashboard to state that `P11` is active and that the current first cut stops before `WorldState` / `PlaceRuntime` / `MessageQueue` / `SugorokuState` / event-timeline-view catalog.
7. Addressed reviewer follow-up by making `add_member` failure non-mutating with respect to `membership_epoch`, making `add_initial` bootstrap-only once the epoch advances, and correcting the P11 debug-surface wording in `tasks.md`.

## Files changed

- `crates/mirrorea-core/src/lib.rs`
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
- `docs/reports/0960-p11-logical-multi-place-runtime-first-cut.md`

## Commands run and exact outputs

- `source scripts/env/mirrorea_storage_env.sh --ensure-dirs >/dev/null && cargo test -p mirrorea-core`
  - RED first:
    - `error[E0432]: unresolved imports mirrorea_core::MembershipRegistry, mirrorea_core::PlaceCatalog`
  - GREEN after implementation:
    - `test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
    - `test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
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
    - `Found 958 numbered report(s).`
- `git diff --check`
  - exact output:
    - no output

## Evidence / outputs / test results

- `mirrorea-core` now owns typed shared-space runtime substrate carriers beyond envelope/layer rows:
  - `MembershipRegistry` keeps `membership_epoch`, active/inactive frontier, incarnation increments, and typed snapshots
  - `PlaceCatalog` keeps logical place ID to kind mapping and rejects silent kind drift
- Reviewer follow-up fixes now keep `membership_epoch` monotone only on successful mutation and make `add_initial` explicitly bootstrap-only once the epoch has advanced.
- The first cut stays within the project stop line:
  - no `WorldState`
  - no `PlaceRuntime`
  - no `MessageQueue`
  - no `SugorokuState`
  - no event / timeline / visualization catalog freeze
- Sugoroku helper closeout remains the active anchor for the still-helper-local runtime aggregate and confirms that the new core substrate did not silently collapse world/game/queue ownership.
- The implementation forces explicit non-decisions rather than inventing semantics:
  - duplicate principal insertion fails with `rejoin semantics remain unresolved`
  - participant place naming is caller-provided, not baked into the crate

## What changed in understanding

- The cleanest `P11` start is not a generic event/timeline abstraction; it is the source-of-truth frontier that multiple later lanes depend on.
- `MembershipRegistry` and `PlaceCatalog` are stable enough to move because they are typed, non-domain, and already explicit in the helper closeout reading.
- Trying to move `WorldState` or `PlaceRuntime` as-is would over-couple host/world sugar, queue preview, and domain-specific game state.

## Open questions

- Should the next P11 cut add a typed wrapper for helper-local place queues, or keep queue ownership deferred until `P12` / `P13` because payloads remain untyped today?
- How much of the helper-side membership snapshot should later become runtime-side debug surface versus staying helper-local derived output?
- When `MembershipRegistry` is eventually wired into runtime execution, should rejoin semantics be a new method, a distinct lifecycle struct, or remain sample-defined longer?

## Suggested next prompt

`P11` logical multi-place runtime tranche を続け、`MembershipRegistry` / `PlaceCatalog` first cut を helper-side Sugoroku runtime boundaryにどう接続するかを source-backed に整理してください。`WorldState` / `PlaceRuntime` / `MessageQueue` / `SugorokuState` はまだ移さず、next candidate が queue wrapper なのか membership-backed runtime shell なのかを report、`progress.md`、`tasks.md`、`samples_progress.md`、relevant `plan/` に反映してください。
