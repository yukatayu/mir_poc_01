# Report 1105 — Alpha-0 runtime package/avatar first cut

- Date: 2026-05-02 09:43 JST
- Author / agent: Codex
- Scope: `P-A0-10` runtime package / avatar skeleton closeout, including runtime-private executable floor, sidecar evidence refresh, snapshot/doc synchronization, and package validation
- Decision levels touched: existing `L1` / `L2` decisions in `specs/16` and `specs/17` only; no new normative decision introduced

## Objective

Close `P-A0-10` honestly by actualizing a runtime-private package/avatar manifest-admission floor for selected Alpha-0 rows, synchronizing repository memory/snapshot docs, and refreshing validation evidence without promoting avatar ecosystems or native execution into Mir core or public alpha-complete claims.

## Scope and assumptions

- Keep `specs/16-runtime-package-adapter-hotplug.md` and `specs/17-mirrorea-spaces-alpha-scope.md` as the normative boundary.
- Treat `samples/alpha/` as non-public sample-ID keyed runner evidence, not an active runnable root.
- Actualize only the smallest honest row set:
  - `AV-01`, `AV-02`, `AV-06`, `AV-08`, `AV-09`
  - `HP-11`, `HP-12`, `HP-15`
- Preserve stop lines:
  - signature proves provenance only
  - no native binary execution
  - no final avatar API / package ABI
  - no full VRM / VRChat / Unity compatibility
  - no dependent-aware detach completion

## Start state / dirty state

- Start state was not clean.
- Existing in-flight package diff at package start:
  - modified: `crates/mir-runtime/src/lib.rs`
  - untracked: `crates/mir-runtime/src/alpha_avatar_runtime.rs`
  - untracked: `crates/mir-runtime/examples/mirrorea_alpha_avatar_runtime.rs`
  - untracked: `crates/mir-runtime/tests/alpha_avatar_runtime.rs`
  - untracked: `scripts/alpha_avatar_runtime_samples.py`
  - untracked: `scripts/tests/test_alpha_avatar_runtime_samples.py`
- No pre-existing unrelated dirty files were observed in the worktree at this package start.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `samples/alpha/README.md`
- `samples/alpha/avatar-runtime/README.md`
- `samples/alpha/hotplug-runtime/README.md`
- `sub-agent-pro/alpha-0/03-stage-roadmap-and-phase-plan.md`
- `sub-agent-pro/alpha-0/07-runtime-package-avatar-adapter.md`
- `sub-agent-pro/alpha-0/08-sample-matrix.md`
- `sub-agent-pro/alpha-0/11-validation-commit-push-protocol.md`
- `sub-agent-pro/alpha-0/13-progress-dashboard-model.md`
- `sub-agent-pro/alpha-0/15-glossary-and-stop-lines.md`

## Actions taken

- Added `crates/mir-runtime/src/alpha_avatar_runtime.rs` as a runtime-private package/avatar manifest-admission floor over existing `HotPlugRequest` / `HotPlugVerdict` and logical runtime substrate.
- Added example `crates/mir-runtime/examples/mirrorea_alpha_avatar_runtime.rs`.
- Added focused integration test `crates/mir-runtime/tests/alpha_avatar_runtime.rs`.
- Added thin runner `scripts/alpha_avatar_runtime_samples.py` plus unit tests `scripts/tests/test_alpha_avatar_runtime_samples.py`.
- After reviewer follow-up, separated `AV-08` requested-package rejection from the placeholder fallback outcome instead of treating the requested package as accepted.
- Added exact sidecar parity enforcement in both Rust and Python validation paths.
- Expanded runner closeout inventory to keep planned and mirrored hot-plug rows explicit.
- Routed sample-ID lookup through a library entrypoint to avoid dead code and keep the example thin.
- Generated fresh `.expected.json` sidecars for current executable rows:
  - `AV-01`, `AV-02`, `AV-06`, `AV-08`, `AV-09`
  - `HP-11`, `HP-12`, `HP-15`
- Synchronized `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `samples/README.md`, `scripts/README.md`, `samples/alpha/README.md`, `samples/alpha/avatar-runtime/README.md`, and `samples/alpha/hotplug-runtime/README.md`.
- Updated repository memory in `plan/01-status-at-a-glance.md`, `plan/42-runtime-package-avatar-roadmap.md`, and `plan/43-alpha-e2e-roadmap.md`.
- Collected resource snapshots with `df -h .` and `free -h`.
- Ran focused review/validation loops and fixed the only detected mechanical issue: Rust formatting drift.

## Files changed

- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/alpha_avatar_runtime.rs`
- `crates/mir-runtime/examples/mirrorea_alpha_avatar_runtime.rs`
- `crates/mir-runtime/tests/alpha_avatar_runtime.rs`
- `scripts/alpha_avatar_runtime_samples.py`
- `scripts/tests/test_alpha_avatar_runtime_samples.py`
- `samples/alpha/avatar-runtime/av-01-placeholder_avatar_runtime.expected.json`
- `samples/alpha/avatar-runtime/av-02-custom_mir_avatar_runtime.expected.json`
- `samples/alpha/avatar-runtime/av-06-untrusted_native_avatar_rejected.expected.json`
- `samples/alpha/avatar-runtime/av-08-runtime_unavailable_placeholder.expected.json`
- `samples/alpha/avatar-runtime/av-09-adapter_attempts_undeclared_effect.expected.json`
- `samples/alpha/hotplug-runtime/hp-11-unsigned_native_package_rejected.expected.json`
- `samples/alpha/hotplug-runtime/hp-12-signed_overcapability_package_rejected.expected.json`
- `samples/alpha/hotplug-runtime/hp-15-revoked_signed_package_rejected.expected.json`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
- `samples/alpha/README.md`
- `samples/alpha/avatar-runtime/README.md`
- `samples/alpha/hotplug-runtime/README.md`
- `plan/01-status-at-a-glance.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `docs/reports/1105-alpha0-runtime-package-avatar-first-cut.md`
- `docs/reports/1106-p-a0-10-package-review.md`
- `docs/reports/1106-p-a0-10-package-review.md`

## Commands run

- `git status --short --branch`
- `sed -n '1,260p' crates/mir-runtime/src/alpha_avatar_runtime.rs`
- `sed -n '1,260p' crates/mir-runtime/tests/alpha_avatar_runtime.rs`
- `sed -n '1,260p' crates/mir-runtime/examples/mirrorea_alpha_avatar_runtime.rs`
- `sed -n '1,260p' scripts/alpha_avatar_runtime_samples.py`
- `sed -n '1,260p' scripts/tests/test_alpha_avatar_runtime_samples.py`
- `cargo test -p mir-runtime --test alpha_avatar_runtime`
- `python3 -m unittest scripts.tests.test_alpha_avatar_runtime_samples`
- `python3 -m py_compile scripts/alpha_avatar_runtime_samples.py`
- `cargo run -q -p mir-runtime --example mirrorea_alpha_avatar_runtime -- closeout`
- `python3 scripts/alpha_avatar_runtime_samples.py list --format json`
- `python3 scripts/alpha_avatar_runtime_samples.py check-all --format json`
- `python3 scripts/alpha_avatar_runtime_samples.py closeout --format json`
- `target/debug/examples/mirrorea_alpha_avatar_runtime run AV-01 > samples/alpha/avatar-runtime/av-01-placeholder_avatar_runtime.expected.json`
- `target/debug/examples/mirrorea_alpha_avatar_runtime run AV-02 > samples/alpha/avatar-runtime/av-02-custom_mir_avatar_runtime.expected.json`
- `target/debug/examples/mirrorea_alpha_avatar_runtime run AV-06 > samples/alpha/avatar-runtime/av-06-untrusted_native_avatar_rejected.expected.json`
- `target/debug/examples/mirrorea_alpha_avatar_runtime run AV-08 > samples/alpha/avatar-runtime/av-08-runtime_unavailable_placeholder.expected.json`
- `target/debug/examples/mirrorea_alpha_avatar_runtime run AV-09 > samples/alpha/avatar-runtime/av-09-adapter_attempts_undeclared_effect.expected.json`
- `target/debug/examples/mirrorea_alpha_avatar_runtime run HP-11 > samples/alpha/hotplug-runtime/hp-11-unsigned_native_package_rejected.expected.json`
- `target/debug/examples/mirrorea_alpha_avatar_runtime run HP-12 > samples/alpha/hotplug-runtime/hp-12-signed_overcapability_package_rejected.expected.json`
- `target/debug/examples/mirrorea_alpha_avatar_runtime run HP-15 > samples/alpha/hotplug-runtime/hp-15-revoked_signed_package_rejected.expected.json`
- `df -h .`
- `free -h`
- `cargo test -p mirrorea-core --test carriers`
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton --test alpha_local_runtime --test alpha_layer_insertion_runtime --test alpha_network_runtime --test alpha_avatar_runtime`
- `python3 -m unittest scripts.tests.test_alpha_avatar_runtime_samples scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `find samples/alpha/avatar-runtime -maxdepth 1 -type f | sort`
- `find samples/alpha/hotplug-runtime -maxdepth 1 -type f | sort`
- `cargo fmt`
- `cargo fmt --check`
- `git diff --check`
- `git diff --stat`

## Evidence / outputs / test results

- `cargo test -p mirrorea-core --test carriers`
  - 12/12 passed
- `cargo test -p mir-runtime --test hotplug_runtime_skeleton --test alpha_local_runtime --test alpha_layer_insertion_runtime --test alpha_network_runtime --test alpha_avatar_runtime`
  - hot-plug skeleton 8/8 passed
  - alpha local runtime 3/3 passed
  - alpha layer insertion 6/6 passed
  - alpha network runtime 7/7 passed
  - alpha avatar runtime 10/10 passed
- `python3 -m unittest scripts.tests.test_alpha_avatar_runtime_samples scripts.tests.test_validate_docs`
  - 17/17 passed
- `python3 scripts/alpha_avatar_runtime_samples.py check-all --format json`
  - `sample_count: 8`
  - `passed: AV-01, AV-02, AV-06, AV-08, AV-09, HP-11, HP-12, HP-15`
  - `failed: []`
- `python3 scripts/check_source_hierarchy.py`
  - required 60 / present 60 / missing 0
- `python3 scripts/validate_docs.py`
  - scaffold complete
  - detected 1107 numbered reports after adding this report and review report `1106`
- `cargo fmt --check`
  - passed after one `cargo fmt` run
- `git diff --check`
  - passed
- `df -h .`
  - root filesystem `99G`, used `65G`, available `30G`
- `free -h`
  - memory `960Mi` total, `392Mi` available, swap `19Gi` total

## What changed in understanding

- The smallest honest `P-A0-10` cut is not a general avatar ecosystem or native runtime story; it is a runtime-private manifest/admission floor with explicit positive and negative rows.
- For `AV-08`, the correct reading is: requested package `rejected`, placeholder fallback still `accepted` as a guarded availability path. Treating the requested package itself as accepted was too strong.
- `HP-11/12/15` belong naturally to the same manifest-admission runner as `AV-*` rows because the governing boundary is package/native trust policy, not a separate completed hot-plug lifecycle.
- Snapshot docs needed to advance the current package pointer to `P-A0-11` immediately after `P-A0-10` closeout; otherwise the repository would still read as if runtime/package admission were only planned.

## Open questions

- How much Stage-E typed visualization can be honestly pulled into `P-A0-11` without overclaiming a full devtools surface?
- Whether `P-A0-11` should satisfy the save/load prerequisite via local-only execution plus explicit distributed non-claim, or by widening CUT-side checker/runtime linkage first.
- Whether the first integrated alpha demo should remain placeholder/custom-Mir avatar only, or include any additional planned-only adapter rows as non-executable evidence.

## Suggested next prompt

Continue autonomously into `P-A0-11`, integrating the current local-runtime, layer-insertion, network/Docker, and avatar/package floors into a single alpha demo closeout package while keeping save/load distributed completion, native execution, and final public API claims out of scope.

## Plan update status

`plan/` 更新済み:

- `plan/01-status-at-a-glance.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`

## Documentation.md update status

`Documentation.md` 更新済み:

- Alpha-0 future-axis summary now includes the runtime-private package/avatar floor and its stop lines

## progress.md update status

`progress.md` 更新済み:

- large stage map percentages and current package pointer now show post-`P-A0-10` state

## tasks.md update status

`tasks.md` 更新済み:

- `P-A0-10` is marked closed and `P-A0-11` is promoted to the next package

## samples_progress.md update status

`samples_progress.md` 更新済み:

- `A0-AV` and `A0-HP` rows now reflect runtime-backed subset evidence and refreshed validation

## Reviewer findings and follow-up

- `Meitner` (`019de603-43f2-7e82-a194-3565353342ad`)
  - Recommended the smallest honest executable cut: `AV-01/02/08/09` and `HP-11/12/15`, while keeping broader adapter/runtime claims planned.
  - Follow-up: kept `AV-03/04/05/07/10` and non-selected `HP-*` lifecycle rows planned.
- `Tesla` (`019de603-468a-7992-9479-e7d33404150f`)
  - Emphasized signature != semantic safety, non-core avatar boundary, explicit failure on underdeclared metadata, and no silent dependent detach.
  - Follow-up: preserved explicit native-policy rejection rows and non-core wording in runtime/docs.
- `Rawls` (`019de603-45cb-7ca0-b5d8-87266d7425b3`)
  - Required a real runner/test surface plus synchronized docs/snapshots for honest closeout.
  - Follow-up: added cargo/example/script/test floor and synchronized `Documentation.md`, snapshots, sample docs, and plan memory.
- `Confucius` (`019de618-6740-74a1-a073-6b140c22968c`)
  - Produced review report `docs/reports/1106-p-a0-10-package-review.md` with four findings:
    `AV-08` fallback/admission blur, missing sidecar parity enforcement, premature closeout citation, and hot-plug planned-row underreporting.
  - Follow-up:
    - `AV-08` now keeps requested-package `rejected` while placeholder fallback remains explicit
    - Rust/Python validation now checks exact sidecar parity
    - package snapshots now cite real report `1105`
    - runner closeout now distinguishes planned rows from `HP-02..06` mirrored-elsewhere rows

## Skipped validations and reasons

- Did not rerun `python3 scripts/alpha_network_docker_e2e.py check-all --format json`.
  - Reason: `P-A0-10` changed no Stage-C transport code or Docker assets; focused rerun already included `alpha_network_runtime` Rust tests and the existing `P-A0-09` Docker evidence remains the current authority for network/container execution.
- Did not run any native binary sandbox/integration command.
  - Reason: the package explicitly keeps native execution out of scope and validates rejection-only policy rows.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- `Meitner` (`019de603-43f2-7e82-a194-3565353342ad`) completed earlier package review; close after package commit/push
- `Rawls` (`019de603-45cb-7ca0-b5d8-87266d7425b3`) completed earlier package review; close after package commit/push
- `Tesla` (`019de603-468a-7992-9479-e7d33404150f`) completed earlier package review; close after package commit/push
- `Confucius` (`019de618-6740-74a1-a073-6b140c22968c`) completed review report `1106`; close after package commit/push
