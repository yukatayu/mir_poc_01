# Report 1146 — P-A1-04b Practical Hotplug Freshness/Object Seam

- Date: 2026-05-03
- Author / agent: Codex
- Scope: `P-A1-04b` practical hot-plug freshness negatives and narrow object-attach preview seam
- Decision levels touched: `specs/18` practical alpha-1 scope boundary, `plan/44` roadmap memory, practical hot-plug carrier/report floor, snapshot/dashboard wording
- 日本語要約: practical alpha-1 の hot-plug floor に、attach-time stale-membership reject、attach-time missing-witness reject、narrow object package attach preview seam を追加した。`PA1-4` は前進したが、detach minimal contract はまだ未了なので practical alpha-1 の package/hot-plug stage は 100% としていない。

## Objective

Close `P-A1-04b` without overclaim. Add attach-time freshness/witness negatives and a narrow object package attach preview seam to the practical alpha-1 hot-plug floor, keep the `checked package -> hotplug plan -> non-final hot-plug report` carrier split explicit, synchronize repo snapshots, and leave `PA1-4` open for `P-A1-04c` detach minimal contract.

## Scope and assumptions

- Scope is limited to the practical alpha-1 hot-plug floor in `samples/practical-alpha1/`, `crates/mir-ast`, `crates/mir-runtime`, `crates/mirrorea-core`, and the corresponding scripts/docs.
- `HP-A1-04B1` actualizes attach-time stale-membership reject only.
- `HP-A1-04B2` actualizes attach-time missing-witness reject only.
- `HP-A1-06` actualizes a narrow object package attach preview seam only.
- `HP-A1-06` does not prove final object package attach completion. `object_attach_claimed` remains false.
- `PA1-4` remains incomplete because detach minimal contract is still open and promoted to `P-A1-04c`.
- No Docker/local TCP, save/load, devtools, rollback, migration, native execution, public ABI, or public product claim is added here.

## Start state / dirty state

- Start state was a dirty in-progress `P-A1-04b` worktree.
- Modified tracked files already existed under the current package in:
  `crates/mirrorea-core`, `crates/mir-ast`, `crates/mir-runtime`, `scripts/`, and `samples/practical-alpha1/`.
- New untracked files at start were the three practical fixtures/expected reports for `HP-A1-04B1`, `HP-A1-04B2`, `HP-A1-06`, plus four reviewer memo reports.
- No unrelated user-authored changes were detected in the touched package scope.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/1144-p-a1-04a-layer-only-practical-hotplug-floor.md`
- `docs/reports/2026-05-03-pa1-04b-theory-spec-review.md`
- `docs/reports/2026-05-03-pa1-04b-runtime-object-attach-review.md`
- `docs/reports/review-1145-p-a1-04b-sample-validation-seam.md`
- `docs/reports/review-2026-05-03-pa1-04b-snapshot-overclaim-review.md`

## Actions taken

1. Extended the practical hot-plug carrier lanes so freshness and witness evidence are explicit rather than inferred from generic rejects.
2. Widened practical fixture/schema support:
   `membership_epoch`, `member_incarnation`, `required_witness_refs`, `pre_attach_membership_advances`, and the object preview attach profile.
3. Added new practical fixtures and exact expected reports for:
   `HP-A1-04B1`, `HP-A1-04B2`, `HP-A1-06`.
4. Reworked the practical hot-plug runtime floor so:
   layer packages still reuse the Stage-D authority preview where appropriate,
   practical-side package admission remains explicit,
   missing-witness rejection uses a dedicated typed witness lane,
   stale-membership rejection compares offered freshness data,
   object attach remains a preview seam with `object_attach_claimed = false`.
5. Added/updated Rust and Python tests for the widened floor, including mutated-fixture tests proving freshness/witness rejects are content-driven rather than sample-ID driven.
6. Synchronized `README.md`, `Documentation.md`, `specs/18`, `plan/44`, `progress.md`, `tasks.md`, `samples_progress.md`, `samples/README.md`, practical sample READMEs, and `scripts/README.md`.
7. Added this closeout report so the latest-report validator points at a template-complete closeout report rather than a reviewer memo.

## Files changed

- Core/runtime/code:
  - `crates/mirrorea-core/src/fabric.rs`
  - `crates/mirrorea-core/tests/carriers.rs`
  - `crates/mir-ast/src/practical_alpha1.rs`
  - `crates/mir-ast/src/practical_alpha1_hotplug_plan.rs`
  - `crates/mir-ast/tests/practical_alpha1_hotplug_plan.rs`
  - `crates/mir-runtime/src/hotplug_runtime.rs`
  - `crates/mir-runtime/src/alpha_layer_insertion_runtime.rs`
  - `crates/mir-runtime/src/alpha_avatar_runtime.rs`
  - `crates/mir-runtime/src/practical_alpha1_hotplug.rs`
  - `crates/mir-runtime/tests/hotplug_runtime_skeleton.rs`
  - `crates/mir-runtime/tests/practical_alpha1_hotplug.rs`
- Practical fixtures / expected reports:
  - `samples/practical-alpha1/packages/hp-a1-01-debug-layer-attach/package.mir.json`
  - `samples/practical-alpha1/packages/hp-a1-02-non-admin-debug-rejected/package.mir.json`
  - `samples/practical-alpha1/packages/hp-a1-03-auth-layer-contract-update/package.mir.json`
  - `samples/practical-alpha1/packages/hp-a1-04-ratelimit_declared_failure/package.mir.json`
  - `samples/practical-alpha1/packages/hp-a1-05-incompatible-patch-rejected/package.mir.json`
  - `samples/practical-alpha1/packages/hp-a1-04b1-stale-membership-attach-rejected/package.mir.json`
  - `samples/practical-alpha1/packages/hp-a1-04b2-missing-witness-attach-rejected/package.mir.json`
  - `samples/practical-alpha1/packages/hp-a1-06-object-package-attach/package.mir.json`
  - `samples/practical-alpha1/expected/hp-a1-01-debug-layer-attach.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-02-non-admin-debug-rejected.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-03-auth-layer-contract-update.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-04-ratelimit_declared_failure.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-05-incompatible-patch-rejected.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-04b1-stale-membership-attach-rejected.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-04b2-missing-witness-attach-rejected.expected.json`
  - `samples/practical-alpha1/expected/hp-a1-06-object-package-attach.expected.json`
- Scripts/tests/docs:
  - `scripts/practical_alpha1_attach.py`
  - `scripts/tests/test_practical_alpha1_attach.py`
  - `README.md`
  - `Documentation.md`
  - `specs/18-practical-alpha1-scope.md`
  - `plan/44-practical-alpha1-roadmap.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/README.md`
  - `samples/practical-alpha1/README.md`
  - `samples/practical-alpha1/packages/README.md`
  - `samples/practical-alpha1/expected/README.md`
  - `scripts/README.md`
  - `docs/reports/1146-p-a1-04b-practical-hotplug-freshness-object-seam.md`

## Commands run

```bash
git status --short
date '+%Y-%m-%d %H:%M JST'
rg -n "PA1-4|HP-A1|practical hot-plug|object attach|detach" README.md Documentation.md specs/18-practical-alpha1-scope.md plan/44-practical-alpha1-roadmap.md progress.md tasks.md samples_progress.md samples/README.md samples/practical-alpha1/README.md samples/practical-alpha1/packages/README.md samples/practical-alpha1/expected/README.md scripts/README.md
cargo test -p mirrorea-core --test carriers -- --nocapture
cargo test -p mir-ast --test practical_alpha1_front_door -- --nocapture
cargo test -p mir-ast --test practical_alpha1_checker -- --nocapture
cargo test -p mir-ast --test practical_alpha1_hotplug_plan -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_local_runtime -- --nocapture
cargo test -p mir-runtime --test hotplug_runtime_skeleton -- --nocapture
cargo test -p mir-runtime --test practical_alpha1_hotplug -- --nocapture
cargo test -p mir-runtime --test alpha_layer_insertion_runtime
python3 scripts/practical_alpha1_check.py check-all --format json
python3 scripts/practical_alpha1_run_local.py check-all --format json
python3 scripts/practical_alpha1_attach.py check-all --format json
python3 scripts/practical_alpha1_attach.py closeout --format json
python3 -m unittest scripts.tests.test_practical_alpha1_check scripts.tests.test_practical_alpha1_run_local scripts.tests.test_practical_alpha1_attach scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt
cargo fmt --check
git diff --check
git add <package files>
git commit --no-gpg-sign -m "mirrorea: close p-a1-04b practical hotplug freshness seam"
git push
```

## Evidence / outputs / test results

- Rust tests passed:
  - `mirrorea-core` carriers: 12 passed
  - `mir-ast` practical front door: 11 passed
  - `mir-ast` practical checker: 3 passed
  - `mir-ast` practical hotplug plan: 5 passed
  - `mir-runtime` practical local runtime: 4 passed
  - `mir-runtime` hotplug runtime skeleton: 8 passed
  - `mir-runtime` practical hotplug: 12 passed
  - `mir-runtime` alpha layer insertion runtime: 6 passed
- `python3 scripts/practical_alpha1_check.py check-all --format json` passed 10/10 with `first_checker_floor_complete: true`.
- `python3 scripts/practical_alpha1_run_local.py check-all --format json` passed 2/2 with `local_runtime_first_floor_complete: true`.
- `python3 scripts/practical_alpha1_attach.py check-all --format json` passed 8/8 with:
  - `package_hotplug_first_floor_complete: true`
  - `hotplug_plan_boundary_present: true`
  - `object_attach_seam_present: true`
  - `object_attach_claimed: false`
  - `freshness_negative_complete: true`
  - `stage_pa1_4_complete: false`
- `python3 scripts/practical_alpha1_attach.py closeout --format json` reported:
  - implemented rows `HP-A1-01..05`, `HP-A1-04B1`, `HP-A1-04B2`, `HP-A1-06`
  - `limitations` still include no detach-minimal contract, Docker/local TCP, save/load, or final public ABI
- `python3 -m unittest ...` passed 32 tests.
- `python3 scripts/check_source_hierarchy.py` reported `73/73/0`.
- An intermediate `python3 scripts/validate_docs.py` run failed before this closeout report existed because the latest report was a reviewer memo (`2026-05-03-pa1-04b-theory-spec-review.md`) without template-required sections. Adding this numbered closeout report resolves that latest-report guard by design.
- Final docs/format guards passed:
  - `python3 scripts/validate_docs.py` reported `Documentation scaffold looks complete.` and `Found 1150 numbered report(s).`
  - `cargo fmt --check` passed after one `cargo fmt` normalization on touched Rust files
  - `git diff --check` was clean

## What changed in understanding

- `RUN-02` was not a sound proxy for hot-plug freshness. Practical hot-plug needs its own offered membership epoch/incarnation lane.
- Missing witness evidence cannot remain in generic rejection refs only; a distinct witness lane is clearer and avoids mixing attach capability rejects with missing-witness rejects.
- The safe object-package move at this stage is a preview seam only. Reusing broader avatar/package claims would overstate completion.
- The correct in-stage next gate after `P-A1-04b` is still `PA1-4`, not `PA1-5`. Detach minimal contract is the remaining package/hot-plug blocker.

## Open questions

- Which detach-time outcome shape is the narrowest safe `P-A1-04c`: reject-only, defer-only, or a mixed reject/defer/fallback contract?
- Should the first practical detach contract reuse existing `HotPlugVerdict` states only, or add a dedicated practical-side detach boundary row while still avoiding public ABI claims?
- How much of the future `P-A1-05` transport lane can reuse the widened practical hot-plug package schema without introducing early public CLI/Docker shape commitments?

## Suggested next prompt

Close `P-A1-04c` by adding the minimal detach-time contract for the practical hot-plug floor, keeping `PA1-4` non-final until an explicit reject/defer/fallback detach outcome is validated and documented.

## Plan update status

`plan/` 更新済み:
`plan/44-practical-alpha1-roadmap.md` を更新し、`P-A1-04b` closeout、current actualized rows、`P-A1-04c` promoted next、validation path wideningを反映した。

## Documentation.md update status

`Documentation.md` 更新済み:
practical hot-plug floor を `HP-A1-01..05`、`HP-A1-04B1`、`HP-A1-04B2`、`HP-A1-06` に同期し、freshness/witness lanes と object preview seam、non-claims を明記した。

## progress.md update status

`progress.md` 更新済み:
`PA1-4` を 65% に更新し、last closed package を `P-A1-04b` に進め、next autonomous package を `P-A1-04c` に変更した。

## tasks.md update status

`tasks.md` 更新済み:
current practical status、ordered current work、package map を `P-A1-04b` close / `P-A1-04c` promoted next に同期した。

## samples_progress.md update status

`samples_progress.md` 更新済み:
current practical package summary を `P-A1-04b` close に更新し、`PA1-4` の進捗と blocker を detach minimal contract に寄せた。

## Reviewer findings and follow-up

- Theory/spec review:
  `2026-05-03-pa1-04b-theory-spec-review.md`
  - Follow-up: added explicit offered membership/witness lanes; kept object path seam-only and non-final.
- Runtime/object-attach review:
  `2026-05-03-pa1-04b-runtime-object-attach-review.md`
  - Follow-up: practical-side manifest checks now block final admit; object path uses `PlaceholderAvatarObjectPackage` preview seam only.
- Sample/validation review:
  `review-1145-p-a1-04b-sample-validation-seam.md`
  - Follow-up: added real `HP-A1-04B1/04B2/06` fixtures and mutated-fixture tests; did not reuse `RUN-02` as hot-plug evidence.
- Docs/snapshot review:
  `review-2026-05-03-pa1-04b-snapshot-overclaim-review.md`
  - Follow-up: `PA1-4` remains below 100%; `P-A1-04c` is promoted next inside the same stage; docs now explicitly keep detach unfinished.

## Skipped validations and reasons

- Did not run practical transport/Docker, devtools, save/load, or product-prototype floors because `P-A1-04b` does not modify or claim those lanes.
- Did not run full workspace Cargo regression because the changed code is confined to carrier/hot-plug practical lanes and targeted package-close tests already cover the touched floor.

## Commit / push status

- Package implementation commit `780956b` (`mirrorea: close p-a1-04b practical hotplug freshness seam`) was created and pushed to `origin/main`.
- This report metadata is being synchronized in a docs-only follow-up commit.

## Sub-agent session close status

- Reviewer agents were used for theory/spec (`Tesla`), runtime/object-attach (`Volta`), docs/snapshot (`Dirac`), and sample/validation (`Faraday`) review.
- All four reviewer sessions were closed after package push.
