# Report 0185 — review try atomic cut frontier runtime fixture

- Date: 2026-04-05T16:41:07.931496Z
- Author / agent: Codex
- Scope: uncommitted work for `try-body atomic_cut frontier runtime fixture` の semantic correctness / regression risk / docs-code drift review
- Decision levels touched: L2

## 1. Objective

- `e21-try-atomic-cut-frontier` fixture、関連 test expectation/count、spec / plan / progress / traceability / report 更新が current `lib.rs` semantics と整合しているかを review する。
- 特に `TryFallback` + `AtomicCut` rollback frontier の current reading、runtime-only selection/count、docs の overclaim を確認する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `progress.md`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e21-try-atomic-cut-frontier.json`
- `crates/mir-ast/tests/fixtures/current-l2/e21-try-atomic-cut-frontier.host-plan.json`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/04-current-l2-step-semantics.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/11-current-l2-selection-helper.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/11-roadmap-near-term.md`
- `plan/90-source-traceability.md`
- `docs/reports/0184-try-atomic-cut-frontier-runtime-fixture.md`

## 3. Actions taken

1. Required repo docs を順に読み、current L2 / report policy / plan policy / progress policy を確認した。
2. uncommitted diff と新規 fixture / sidecar / report を確認し、`e21` の期待動作を抽出した。
3. `crates/mir-semantics/src/lib.rs` の `Statement::AtomicCut`、`Statement::TryFallback`、`propagate_failure()` を読み、rollback frontier update と restore の実装実体を確認した。
4. `cargo test -p mir-semantics --test current_l2_minimal_interpreter -- --nocapture` と `python3 scripts/current_l2_detached_loop.py smoke-fixture ...` を実行し、runtime behavior と count / selection 更新を実測した。
5. 隣接 docs を grep し、`E21` 追加後も古い runtime corpus を記述している箇所が残っていないかを確認した。
6. review report としてこの report を記入した。
7. main thread はこの review の 2 finding を受けて、rollback locality の prose と adjacent runtime corpus docs drift を補正した。

## 4. Files changed

- `docs/reports/0185-review-try-atomic-cut-frontier-runtime-fixture.md`

## 5. Commands run and exact outputs

1. diff / scope inspection

```bash
git status --short
git diff --stat
git diff --name-only
```

- modified: `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- modified: `plan/08-representative-programs-and-fixtures.md`
- modified: `plan/11-roadmap-near-term.md`
- modified: `plan/90-source-traceability.md`
- modified: `progress.md`
- modified: `specs/examples/00-representative-mir-programs.md`
- modified: `specs/examples/04-current-l2-step-semantics.md`
- untracked: `crates/mir-ast/tests/fixtures/current-l2/e21-try-atomic-cut-frontier.host-plan.json`
- untracked: `crates/mir-ast/tests/fixtures/current-l2/e21-try-atomic-cut-frontier.json`
- untracked: `docs/reports/0184-try-atomic-cut-frontier-runtime-fixture.md`
- untracked: `docs/reports/0185-review-try-atomic-cut-frontier-runtime-fixture.md`

2. focused test verification

```bash
cargo test -p mir-semantics --test current_l2_minimal_interpreter try_body_atomic_cut_updates_rollback_frontier_without_skipping_fallback -- --nocapture
```

- `test try_body_atomic_cut_updates_rollback_frontier_without_skipping_fallback ... ok`

3. broader interpreter regression verification

```bash
cargo test -p mir-semantics --test current_l2_minimal_interpreter -- --nocapture
```

- `running 45 tests`
- `45 passed; 0 failed`
- `try_body_atomic_cut_updates_rollback_frontier_without_skipping_fallback ... ok`
- `discovery_finds_fixture_bundles_and_classifies_runtime_vs_static_only ... ok`
- `selection_runtime_only_keeps_only_runtime_bundles ... ok`
- `run_directory_returns_summary_for_current_l2_fixture_dir ... ok`
- `run_directory_selected_runtime_only_executes_only_runtime_bundles ... ok`
- `run_directory_profiled_includes_profile_name_in_summary ... ok`

4. detached smoke

```bash
python3 scripts/current_l2_detached_loop.py smoke-fixture \
  crates/mir-ast/tests/fixtures/current-l2/e21-try-atomic-cut-frontier.json \
  --reference-fixture crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.json \
  --run-label e21-atomic-cut-frontier \
  --reference-label e2-try-fallback \
  --overwrite
```

- bundle diff: `payload_core.event_kinds`
  - left = `["perform-success", "atomic-cut", "perform-success", "perform-failure", "rollback", "perform-success"]`
  - right = `["perform-success", "perform-failure", "rollback", "perform-success"]`
- aggregate diff:
  - `summary_core.total_bundles: left=21 right=1`
  - `summary_core.runtime_bundles: left=10 right=1`
  - `summary_core.static_only_bundles: left=11 right=0`
  - `summary_core.passed: left=21 right=1`

5. docs / worktree checks

```bash
python3 scripts/validate_docs.py
git diff --check
find crates/mir-ast/tests/fixtures/current-l2 -maxdepth 1 -name '*.json' ! -name '*.host-plan.json' | wc -l
find crates/mir-ast/tests/fixtures/current-l2 -maxdepth 1 -name '*.host-plan.json' | wc -l
```

- `Documentation scaffold looks complete.`
- `Found 185 numbered report(s).`
- `git diff --check`: 無出力
- fixture count: `21`
- host-plan sidecar count: `10`

## 6. Evidence / findings

1. Medium — new prose still overstates rollback locality relative to the current implementation.  
   `e21` itself matches the concrete `AtomicCut` frontier behavior in `lib.rs`, but the touched docs describe rollback as local to the current place, while the implementation snapshots and restores the entire `place_store`. `place_anchor` only gates whether `AtomicCut` updates the frame; it does not scope rollback restoration by place. That means the new prose is stronger than what the code actually enforces.  
   Evidence:
   - `TryFallback` stores `restore_snapshot: self.state.place_store.clone()` at entry in [lib.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-semantics/src/lib.rs#L727)
   - `AtomicCut` updates that full snapshot in [lib.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-semantics/src/lib.rs#L711)
   - failure restores `self.state.place_store = frame.restore_snapshot.clone()` in [lib.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-semantics/src/lib.rs#L1185)
   - the fixture and prose claim locality in [e21-try-atomic-cut-frontier.json](/home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/fixtures/current-l2/e21-try-atomic-cut-frontier.json#L97), [00-representative-mir-programs.md](/home/yukatayu/dev/mir_poc_01/specs/examples/00-representative-mir-programs.md#L191), and [04-current-l2-step-semantics.md](/home/yukatayu/dev/mir_poc_01/specs/examples/04-current-l2-step-semantics.md#L262)

2. Low — adjacent spec docs still describe the pre-`E21` runtime fixture set, so the task leaves observable docs drift around fixture inventory and runtime selection examples.  
   The new tests and counts are correct, but nearby docs still say the current fixture set is 20 and that `runtime-only` selects only the older runtime examples. After adding `e21`, those statements are stale.  
   Evidence:
   - `current fixture set is 20` in [02-current-l2-ast-fixture-schema.md](/home/yukatayu/dev/mir_poc_01/specs/examples/02-current-l2-ast-fixture-schema.md#L178)
   - old runtime bundle example in [09-current-l2-bundle-loader.md](/home/yukatayu/dev/mir_poc_01/specs/examples/09-current-l2-bundle-loader.md#L94)
   - old batch discovery example in [10-current-l2-batch-runner.md](/home/yukatayu/dev/mir_poc_01/specs/examples/10-current-l2-batch-runner.md#L113)
   - old `runtime-only` selection wording in [11-current-l2-selection-helper.md](/home/yukatayu/dev/mir_poc_01/specs/examples/11-current-l2-selection-helper.md#L98)

Confirmed non-findings:
- The concrete `e21` scenario does match current `lib.rs` behavior: `AtomicCut` updates the top rollback frame to the post-cut snapshot, `annotate_profile_patch` is removed by rollback, `stage_profile_patch` remains, and fallback still executes.
- runtime-only counts / selection lists in [current_l2_minimal_interpreter.rs](/home/yukatayu/dev/mir_poc_01/crates/mir-semantics/tests/current_l2_minimal_interpreter.rs) are internally consistent with the actual directory state (`21 total / 10 runtime / 11 static-only`) and pass in real test runs.
- `must_explain` remains prose-only; the touched code/docs did not silently promote it into machine-check semantics.

Resolution status:
- finding 1 は main thread で解消済み。`specs/examples/00-current-l2-representative-mir-programs.md`、`specs/examples/04-current-l2-step-semantics.md`、`e21-try-atomic-cut-frontier.json` の wording は、current implementation が whole `place_store` snapshot を restore しつつ `place_anchor` で frontier update を gate する actual code anchor に合わせて補正された。
- finding 2 は main thread で解消済み。`specs/examples/02-current-l2-ast-fixture-schema.md`、`specs/examples/09-current-l2-bundle-loader.md`、`specs/examples/10-current-l2-batch-runner.md`、`specs/examples/11-current-l2-selection-helper.md` が `E21` / `21 total / 10 runtime / 11 static-only` の current corpus に追従した。

## 7. Changes in understanding

- `e21` is a valid regression witness for the existing frontier-update rule; it does not introduce new evaluator behavior.
- The actual semantic gap exposed by this task was not the new fixture itself, but the continued use of “current place local rollback” wording in docs while the PoC implementation still restored a whole-store snapshot. That wording gap is now closed in the touched scope.
- runtime selection / directory summary changes are implemented correctly and validated by the current test binary and detached smoke path.

## 8. Open questions

- Should current docs stay aligned with the present whole-store snapshot restore model for a while longer, or should the evaluator later be narrowed so rollback is actually scoped by place?
- If the intended semantics is truly place-local, which carrier should represent place-scoped snapshots without overcommitting the parser-free PoC?
- この review report 自体は `plan/` や `progress.md` を直接編集しないが、main thread は finding 2 の補正で関連 docs / plan / progress を更新した。

## 9. Suggested next prompt

`The review findings for the try-body atomic_cut frontier task have been applied. Next, compare how the current whole-store rollback implementation should connect to a future checker-side “try / rollback locality” structural floor without overcommitting the parser-free PoC.`
