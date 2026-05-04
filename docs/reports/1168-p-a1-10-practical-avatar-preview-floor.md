# Report 1168 — P-A1-10 Practical Avatar Preview First Floor

- Date: 2026-05-04
- Author / agent: Codex
- Scope: practical alpha-1 avatar preview companion floor
- Decision levels touched: `L1`, `L2`

## Objective

Close `P-A1-10` by actualizing a distinct practical avatar-preview companion floor over exact hot-plug source reports, without collapsing it into native execution, same-session product runtime completion, or final avatar ABI.

## 日本語要約

`P-A1-10` では、`AV-A1-01/02/03` を practical alpha-1 の distinct avatar-preview companion floor として閉じた。これは `checked package -> hotplug plan -> exact hot-plug report -> distinct avatar preview report` の lane を追加する作業であり、custom avatar preview と unsupported-runtime visible fallback preview を actualize するが、native execution、same-session runtime、final avatar ABI は claim しない。

## Scope and assumptions

- `AV-A1-01/02/03` だけを actualize する。
- `PE2E-*` product-preview floor は widen しない。
- `samples/practical-alpha1/` は active runnable root に昇格しない。
- native execution、final avatar ABI、VRM / VRChat / Unity compatibility は scope 外とする。
- report 時点で sub-agent delegation は使わず、local focused review で閉じる。

## Start state / dirty state

- Start state was dirty.
- Existing in-flight code changes for practical alpha-1 avatar preview implementation were already present in:
  `crates/mir-ast/src/practical_alpha1.rs`,
  `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`,
  `crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs`,
  `crates/mir-runtime/src/lib.rs`,
  `crates/mir-runtime/src/practical_alpha1_hotplug.rs`,
  `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`,
  plus new untracked avatar preview source/sample/script files.
- Docs / roadmap / dashboard sync and report creation were completed in the same task.

## Documents consulted

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
- `plan/01-status-at-a-glance.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `samples/alpha/avatar-runtime/README.md`
- `scripts/README.md`
- `docs/reports/TEMPLATE.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`

## Actions taken

- Added practical avatar preview profiles to the practical alpha-1 package/hot-plug path.
- Added a distinct Rust avatar preview report surface and example CLI.
- Added practical alpha-1 avatar preview sample fixtures and exact expected reports for `AV-A1-01/02/03`.
- Added `scripts/practical_alpha1_avatar.py` and unit tests.
- Re-synchronized the `HP-A1-06` exact expected hot-plug sidecar after the object-package manifest-check naming widened from a placeholder-specific label to the shared avatar-capability label.
- Updated `specs/18`, `plan/44`, `plan/01`, `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, sample READMEs, and `scripts/README.md` to describe the new companion floor and keep non-claims explicit.
- Performed focused validation for the new floor and repo-wide docs/format/diff guardrails.

## Files changed

- `crates/mir-ast/src/practical_alpha1.rs`
- `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
- `crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/practical_alpha1_hotplug.rs`
- `crates/mir-runtime/src/practical_alpha1_avatar.rs`
- `crates/mir-runtime/examples/mir_practical_alpha1_avatar.rs`
- `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`
- `crates/mir-runtime/tests/practical_alpha1_avatar.rs`
- `samples/practical-alpha1/packages/av-a1-01-placeholder-avatar-runtime/package.mir.json`
- `samples/practical-alpha1/packages/av-a1-02-custom-mir-avatar-runtime/package.mir.json`
- `samples/practical-alpha1/packages/av-a1-03-unsupported-runtime-fallback/package.mir.json`
- `samples/practical-alpha1/expected/av-a1-01-placeholder-avatar-runtime.expected.json`
- `samples/practical-alpha1/expected/av-a1-02-custom-mir-avatar-runtime.expected.json`
- `samples/practical-alpha1/expected/av-a1-03-unsupported-runtime-fallback.expected.json`
- `scripts/practical_alpha1_avatar.py`
- `scripts/tests/test_practical_alpha1_avatar.py`
- `README.md`
- `Documentation.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/1168-p-a1-10-practical-avatar-preview-floor.md`

## Commands run

- `df -h .`
- `free -h`
- `python3 scripts/practical_alpha1_attach.py check-all --format json`
- `python3 scripts/practical_alpha1_avatar.py check-all --format json`
- `python3 -m unittest scripts.tests.test_practical_alpha1_avatar scripts.tests.test_validate_docs`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `cargo fmt`
- `cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture`
- `cargo test -p mir-runtime --test practical_alpha1_avatar -- --nocapture`
- `python3 scripts/practical_alpha1_avatar.py closeout --format json`
- `cargo fmt --check`
- `git diff --check`
- `git status --short`

## Evidence / outputs / test results

- Resource check on the small VPS remained acceptable for continued work:
  `df -h .` showed `30G` free on `/dev/vda2`, and `free -h` showed `177Mi` free RAM with `19Gi` swap available.
- `python3 scripts/practical_alpha1_attach.py check-all --format json` passed all `9/9` hot-plug prerequisite rows, including `HP-A1-06` and `HP-A1-07`, with `object_attach_seam_present: true`, `freshness_negative_complete: true`, and `stage_pa1_4_complete: true`.
- `python3 scripts/practical_alpha1_avatar.py check-all --format json` passed `AV-A1-01/02/03`, with `avatar_preview_first_floor_complete: true`, `native_execution_claimed: false`, and `final_avatar_abi_frozen: false`.
- `python3 -m unittest scripts.tests.test_practical_alpha1_avatar scripts.tests.test_validate_docs` passed `17` tests.
- `cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture` passed `10` tests.
- `cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture` passed `17` tests.
- `cargo test -p mir-runtime --test practical_alpha1_avatar -- --nocapture` passed `3` tests.
- `python3 scripts/practical_alpha1_avatar.py closeout --format json` reported `implemented_rows = [AV-A1-01, AV-A1-02, AV-A1-03]`, `avatar_preview_first_floor_complete: true`, `native_execution_claimed: false`, and `final_avatar_abi_frozen: false`.
- `python3 scripts/check_source_hierarchy.py` passed with `73/73` required paths present.
- `python3 scripts/validate_docs.py` reported `Documentation scaffold looks complete.` and `Found 1169 numbered report(s).`
- `cargo fmt --check` and `git diff --check` were clean on the closeout tree.

## What changed in understanding

- The safe carrier for practical avatar semantics is not product-preview widening by default.
- `AV-A1-02` and `AV-A1-03` can be actualized honestly only as distinct avatar-preview companion reports over exact hot-plug source reports.
- `AV-A1-03` remains a rejected source hot-plug report plus visible monotone placeholder fallback companion preview, not unsupported-runtime execution success.

## Open questions

- Whether `PE2E-*` should consume `AV-A1-*` exact reports next, and if so which preview row should widen first without implying same-session product runtime.
- Whether `VIS-A1-03/05/07` or broader save/load widening should still wait behind product-preview/avatar composition.

## Suggested next prompt

Close `P-A1-10`, then promote the narrowest safe package that composes `AV-A1-*` exact reports into `PE2E-*` without collapsing companion preview evidence into same-session runtime completion.

## Plan update status

`plan/` 更新済み:

- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`

## Documentation.md update status

`Documentation.md` 更新済み:

- practical alpha-1 summary now includes the distinct avatar-preview companion floor and its non-claims.

## progress.md update status

`progress.md` 更新済み:

- practical alpha-1 stage snapshot now names `P-A1-10` as the latest closed package and updates the large-stage map percentage.

## tasks.md update status

`tasks.md` 更新済み:

- current package map, next reopen point, and avatar blocker wording now reflect `AV-A1-01/02/03` actualization.

## samples_progress.md update status

`samples_progress.md` 更新済み:

- practical alpha-1 package map and sample matrix now include the avatar preview companion floor.

## Reviewer findings and follow-up

- No sub-agents were used in this task.
- Follow-up was local focused review only, because current tool policy for this turn did not authorize delegation without explicit user request.
- Local review focus:
  - carrier separation between hot-plug report, avatar preview report, and product-preview bundle
  - non-claim wording around native execution / same-session runtime / final avatar ABI
  - dashboard and roadmap sync

## Skipped validations and reasons

- Did not rerun the broader practical product-preview, transport, save/load, or run-local package suites in this package, because `P-A1-10` touched the avatar-preview lane plus its exact hot-plug prerequisite and did not change those wider carrier implementations.
- Did not use sub-agent review, because the current tool policy for this turn did not authorize delegation without an explicit user request.

## Commit / push status

- Main implementation closeout commit: `f85bceb` (`mirrorea: close p-a1-10 practical avatar preview`)
- Push status at report finalize: pushed to `origin/main`
- This report section was finalized in a docs-only follow-up commit after the main closeout push.

## Sub-agent session close status

- No sub-agent sessions opened in this task.
