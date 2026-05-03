# Report 1144 — P-A1-04a Layer-Only Practical Hot-Plug First Floor

- Date: 2026-05-03 18:23 JST
- Author / agent: Codex
- Scope: close `P-A1-04a` by actualizing a manifest-driven, layer-only practical alpha-1 hot-plug first floor over real practical package fixtures, exact expected reports, and synchronized practical snapshot docs, while recutting `P-A1-04` internally into `04a` and `04b`
- Decision levels touched: L1, L2
- 日本語要約: `P-A1-04` をそのまま package/hot-plug stage 全体として閉じるのは overclaim になるため、今回の package は `P-A1-04a` として narrow に切り直した。actualize したのは `HP-A1-01..05` の layer-only hot-plug first floor だけであり、checked package -> hotplug plan -> non-final hot-plug report の distinct carrier split を practical line に追加した。object package attach、missing-witness / stale-membership negatives、detach minimal contract、Docker/local TCP、save/load、devtools、final public ABI はまだ claim していない。

## Objective

Close `P-A1-04a` as the first practical alpha-1 package/hot-plug floor:

- add a distinct hotplug-plan carrier over practical package input
- accept/reject practical layer packages through a manifest-driven attach path
- emit exact expected hot-plug reports for `HP-A1-01..05`
- keep the practical line distinct from Alpha-0 Stage D evidence and from any public/final package ABI claim

## Scope and assumptions

- Normative authority remains `specs/18-practical-alpha1-scope.md`.
- `P-A1-04` is recut internally:
  - `P-A1-04a`: layer-only practical hot-plug first floor
  - `P-A1-04b`: missing-witness/stale-membership negatives plus narrow object-attach seam
- This package actualizes only `HP-A1-01..05`:
  - debug layer attach accepted
  - non-admin debug attach rejected
  - auth layer accepted only via explicit contract-update path
  - rate-limit layer accepted with explicit preview reject evidence
  - incompatible patch rejected
- `samples/practical-alpha1/` remains a practical evidence root, not the repo-wide active runnable root.
- This package does not close:
  - object package attach
  - missing-witness hot-plug reject
  - stale-membership hot-plug reject
  - detach minimal contract
  - Docker/local TCP transport
  - local save/load
  - devtools / viewer
  - final public package/hot-plug/runtime ABI

## Start state / dirty state

- Start state: `main` branch with `P-A1-04a` implementation work already in the working tree, not yet committed.
- Dirty state at closeout start:
  - modified practical alpha-1 Rust library files and tests
  - new practical hot-plug example, script, fixtures, and expected reports
  - modified docs / roadmap / snapshot files for the practical alpha-1 line
  - prior review-only report `docs/reports/review-1140-p-a1-04-boundary.md`
- No unrelated user changes were reverted.

## Documents consulted

- `sub-agent-pro/alpha-1/04-stage-roadmap.md`
- `sub-agent-pro/alpha-1/05-theory-freeze.md`
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`
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
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/1143-p-a1-03-local-runtime-from-runtime-plan.md`
- `docs/reports/review-1140-p-a1-04-boundary.md`

## Actions taken

1. Added a distinct practical hot-plug lowering carrier in `mir-ast`:
   - `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
   - rejects front-door-only packages and malformed hot-plug inputs before runtime attach
   - narrows the current floor to `layer` packages with explicit attach metadata
2. Extended practical package input types in `crates/mir-ast/src/practical_alpha1.rs`:
   - added `manifest.attach_profile`
   - added `alpha_local_hotplug_input`
   - widened accepted front-door surface refs to include the practical hot-plug helper input
3. Added a distinct practical hot-plug report in `mir-runtime`:
   - `crates/mir-runtime/src/practical_alpha1_hotplug.rs`
   - keeps `hotplug_plan_scope` and `hotplug_scope` separate
   - reuses `alpha_layer_insertion_runtime` only as a lower attach-time authority
   - keeps explicit later refs for object attach, freshness negatives, detach, transport, save/load, and final public ABI
4. Widened `alpha_layer_insertion_runtime` internal reuse points so the practical hot-plug report can assemble through the existing layer-compatibility/runtime-private authority without copying semantics.
5. Added a Rust example command:
   - `crates/mir-runtime/examples/mir_practical_alpha1_attach.rs`
6. Added practical hot-plug fixtures and exact expected outputs:
   - `samples/practical-alpha1/packages/hp-a1-01..05/package.mir.json`
   - `samples/practical-alpha1/expected/hp-a1-01..05.expected.json`
   - expected reports were generated from the real example output and committed as exact evidence artifacts
7. Added the first practical hot-plug script surface:
   - `scripts/practical_alpha1_attach.py`
   - row-keyed `HP-A1-01..05` plus aggregate `check-all` / `closeout`
8. Added focused tests:
   - `crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs`
   - `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`
   - `scripts/tests/test_practical_alpha1_attach.py`
9. Recut the stage reading:
   - `P-A1-04a` is now the closed layer-only first floor
   - `P-A1-04b` is the next promoted package inside `PA1-4`
10. Synchronized `README.md`, `Documentation.md`, `specs/18`, `plan/01`, `plan/42`, `plan/44`, `progress.md`, `tasks.md`, `samples_progress.md`, `samples/README.md`, `samples/practical-alpha1/*/README.md`, and `scripts/README.md` so the practical line no longer overclaims `PA1-4` as fully closed.

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/src/practical_alpha1.rs`
- `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
- `crates/mir-ast/tests/practical_alpha1_front_door.rs`
- `crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/alpha_layer_insertion_runtime.rs`
- `crates/mir-runtime/src/practical_alpha1_hotplug.rs`
- `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`
- `crates/mir-runtime/examples/mir_practical_alpha1_attach.rs`
- `samples/practical-alpha1/packages/hp-a1-01-debug-layer-attach/package.mir.json`
- `samples/practical-alpha1/packages/hp-a1-02-non-admin-debug-rejected/package.mir.json`
- `samples/practical-alpha1/packages/hp-a1-03-auth-layer-contract-update/package.mir.json`
- `samples/practical-alpha1/packages/hp-a1-04-ratelimit-declared-failure/package.mir.json`
- `samples/practical-alpha1/packages/hp-a1-05-incompatible-patch-rejected/package.mir.json`
- `samples/practical-alpha1/expected/hp-a1-01-debug-layer-attach.expected.json`
- `samples/practical-alpha1/expected/hp-a1-02-non-admin-debug-rejected.expected.json`
- `samples/practical-alpha1/expected/hp-a1-03-auth-layer-contract-update.expected.json`
- `samples/practical-alpha1/expected/hp-a1-04-ratelimit-declared-failure.expected.json`
- `samples/practical-alpha1/expected/hp-a1-05-incompatible-patch-rejected.expected.json`
- `scripts/practical_alpha1_attach.py`
- `scripts/tests/test_practical_alpha1_attach.py`
- `docs/reports/1144-p-a1-04a-layer-only-practical-hotplug-first-floor.md`

## Commands run

```bash
git status --short
ls -R samples/practical-alpha1
sed -n '1,260p' crates/mir-runtime/tests/practical_alpha1_hotplug.rs
sed -n '1,260p' scripts/practical_alpha1_attach.py
sed -n '1,260p' scripts/tests/test_practical_alpha1_attach.py
sed -n '1,260p' crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs
cargo run -q -p mir-runtime --example mir_practical_alpha1_attach -- samples/practical-alpha1/packages/hp-a1-01-debug-layer-attach
bash -lc 'for row in hp-a1-01-debug-layer-attach hp-a1-02-non-admin-debug-rejected hp-a1-03-auth-layer-contract-update hp-a1-04-ratelimit-declared-failure hp-a1-05-incompatible-patch-rejected; do cargo run -q -p mir-runtime --example mir_practical_alpha1_attach -- "samples/practical-alpha1/packages/${row}" > "samples/practical-alpha1/expected/${row}.expected.json"; done'
python3 -m unittest scripts.tests.test_practical_alpha1_attach
cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture
cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture
cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture
cargo test -p mir-runtime --test alpha_layer_insertion_runtime
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_attach.py closeout --format json
python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_validate_docs
date '+%Y-%m-%d %H:%M JST'
```

## Evidence / outputs / test results

- `cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture` passed `11` tests after the accepted front-door surface widened to include practical hot-plug helper input.
- `cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture` passed `3` tests covering:
  - positive plan lowering
  - front-door-only package rejection
  - malformed auth profile rejection
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture` passed `7` tests covering:
  - exact expected report parity for `HP-A1-01..05`
  - front-door-only rejection
  - mutated temp-package malformed auth rejection
- `cargo test -p mir-runtime --test alpha_layer_insertion_runtime` passed `6` tests, keeping the Alpha-0 Stage D layer authority green.
- `python3 scripts/practical_alpha1_check.py check-all --format json` passed all `10` checker rows.
- `python3 scripts/practical_alpha1_run_local.py check-all --format json` passed `RUN-01/02`.
- `python3 scripts/practical_alpha1_attach.py check-all --format json` returned:
  - `sample_count: 5`
  - `passed = ["HP-A1-01", "HP-A1-02", "HP-A1-03", "HP-A1-04", "HP-A1-05"]`
  - `package_hotplug_first_floor_complete: true`
  - `hotplug_plan_boundary_present: true`
  - `object_attach_claimed: false`
  - `freshness_negative_complete: false`
  - `stage_pa1_4_complete: false`
- `python3 scripts/practical_alpha1_attach.py closeout --format json` re-emitted the same first-floor booleans and stop lines from executed checks rather than hardcoded success.
- `python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_validate_docs` passed `32` tests.
- Exact expected report sidecars now exist for `HP-A1-01..05` and are checked byte-for-byte in both Rust and Python helper tests.

## What changed in understanding

- `P-A1-04` could not be closed honestly as a single stage-wide package without overclaiming object/package admission and attach-time freshness negatives.
- The practical line can still advance safely if `PA1-4` is read as a staged package family:
  - `P-A1-04a` closes the layer-only first floor
  - `P-A1-04b` stays focused on missing-witness/stale-membership negatives and a narrow object-attach seam
- The existing Alpha-0 Stage D layer-insertion floor is strong enough to serve as lower authority for the practical layer-only path, but not strong enough to imply practical object attach or practical attach-time freshness completion.

## Open questions

- `P-A1-04b` still needs to decide the narrow object package attach seam without collapsing into the Alpha-0 runtime-private avatar/package floor.
- `P-A1-04b` still needs explicit practical attach-time rows for missing witness and stale membership; `RUN-02` remains runtime evidence, not hot-plug admission evidence.
- The practical line still lacks any Docker/local TCP, save/load, devtools, or product-prototype command surface.

## Suggested next prompt

Close `P-A1-04b` by adding explicit practical attach-time missing-witness and stale-membership rejects plus the narrowest honest object-package attach seam, while keeping `PA1-4` distinct from Alpha-0 Stage D evidence and from any public/final package ABI claim.

## Plan update status

`plan/` 更新済み: `plan/01-status-at-a-glance.md`, `plan/42-runtime-package-avatar-roadmap.md`, `plan/44-practical-alpha1-roadmap.md` は `P-A1-04a` closed / `P-A1-04b` next を mirror する practical line memory に同期した。

## Documentation.md update status

`Documentation.md` 更新済み: practical alpha-1 line summary を layer-only hot-plug first floor まで widening し、`P-A1-04a` の carrier split と non-claim を追記した。

## progress.md update status

`progress.md` 更新済み: `PA1-4` を in-progress へ recut し、`P-A1-04a` close / `P-A1-04b` next、practical alpha-1 3-axis row、recent log を同期した。

## tasks.md update status

`tasks.md` 更新済み: `P-A1-04a` freshness を反映し、`PA1-4` を stage-internal recut として書き換え、`P-A1-04b` を promoted next package に昇格した。

## samples_progress.md update status

`samples_progress.md` 更新済み: practical toolchain summary、`PA1-4` package row、`HP-A1-01..05` dedicated sample row、top active-package line を同期した。

## Reviewer findings and follow-up

- Theory/type-system review:
  reviewer `Fermat` concluded that the current practical manifest/hot-plug surface is only safe as a layer-only local first floor. I followed that finding by recutting `P-A1-04` into `04a/04b` and by keeping object package attach, detach, and freshness negatives explicitly later.
- Runtime/transport review:
  reviewer `Godel` found that the current lower runtime floor does not enforce `membership_epoch`, `member_incarnation`, or witness refs semantically at attach time, and that `HotPlugRequest` / `HotPlugVerdict` are post-admission carriers rather than a practical admission engine. I followed that finding by leaving `freshness_negative_complete = false`, by not actualizing missing-witness/stale-membership rows, and by promoting `P-A1-04b` for that seam instead of claiming `PA1-4` completion.
- Docs/progress consistency review:
  reviewer `Bacon` found stale roadmap/snapshot wording that still described package/hot-plug as wholly absent or that risked collapsing `P-A1-04` into Alpha-0 Stage D evidence. I updated `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `plan/01`, `plan/42`, and `plan/44` to describe `P-A1-04a` as a separate practical layer-only first floor.
- Sample/validation review:
  reviewer `Feynman` confirmed that practical `HP-*` fixtures and exact `hp-a1-*.expected.json` reports were required, and that attach-time missing-witness/stale-membership rows remained absent. I added `HP-A1-01..05` plus exact expected reports, but I kept the missing-witness/stale-membership gap open and promoted it into `P-A1-04b`.

## Skipped validations and reasons

- `cargo test -p mir-ast` full crate sweep:
  not rerun because this package touched the practical front-door/hot-plug lane narrowly and the focused front-door/hotplug-plan tests plus Python/docs floor were sufficient for package-closeout evidence
- `cargo test -p mir-runtime` full crate sweep:
  not rerun because this package touched the practical hot-plug floor and reused the existing layer-insertion authority; focused `practical_alpha1_hotplug` and `alpha_layer_insertion_runtime` were the relevant floor
- Docker/local TCP practical transport validation:
  skipped because `P-A1-05` remains later and `P-A1-04a` explicitly does not claim transport execution
- save/load/devtools/product validation:
  skipped because these remain outside `P-A1-04a` scope and were not claimed

## Commit / push status

Pending at report write.

## Sub-agent session close status

- reviewer sessions completed for the current package:
  - `Fermat` theory/type-system review
  - `Godel` runtime/package boundary review
  - `Bacon` docs/progress consistency review
  - `Feynman` sample/validation review
- close commands will be issued after package commit/push unless the context is reused immediately for `P-A1-04b`.
