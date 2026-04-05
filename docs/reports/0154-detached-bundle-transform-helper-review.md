# Report 0154 — detached bundle transform helper review

- Date: 2026-04-05T11:49:19.705799Z
- Author / agent: Codex
- Scope: in-progress task `detached bundle transform helper` の local working tree review。semantic correctness、helper-boundary discipline、docs / plan / report consistency、bundle-first detached artifact split の regression 有無を確認する
- Decision levels touched: none

## 1. Objective

- local working tree change が、bundle-first detached artifact split を壊さずに bundle-side private transform を `examples/support/` の non-production helper へ切り出しているかを確認する。
- `lib.rs` / `harness.rs` public API が未変更のまま維持されているかを確認する。
- `host_plan_coverage_failure` が bundle artifact 側へ漏れず aggregate-only の current judgmentを保っているかを確認する。
- docs / plan / report mirror が、この narrow cut を過不足なく反映しているかを確認する。

## 2. Scope and assumptions

- review 対象は local working tree 上の未コミット変更に限定する。
- runtime semantics 自体の redesign や detached compare contract の finalization は今回の review scope 外とする。
- non-trivial review task として本 report を新規作成するが、spec / plan / progress の内容更新は行わない。
- `plan/ 更新不要`。
- `progress.md 更新不要`。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `progress.md`
- `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
- `specs/examples/37-current-l2-detached-bundle-transform-helper.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0153-detached-bundle-transform-helper.md`
- `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
- `crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`
- `crates/mir-semantics/tests/current_l2_detached_bundle_support.rs`
- `crates/mir-semantics/examples/support/current_l2_detached_aggregate_support.rs`
- `crates/mir-semantics/tests/current_l2_detached_aggregate_support.rs`

## 4. Actions taken

1. AGENTS.md 指示に従って基礎文書、current L2 helper stack 関連 spec / plan / progress を順に再読した。
2. local working tree の `git status --short` と `git diff --stat` を確認し、bundle helper task の対象変更と未追跡ファイルを把握した。
3. `current_l2_emit_detached_bundle.rs` の差分を元実装と比較し、artifact transform の extraction が意味的に等価かを確認した。
4. 新規 support module と新規 test を読み、`FixtureBundle + BundleRunReport -> detached bundle artifact` の shared helper 化が `examples/support/` に留まり public API を越境していないことを確認した。
5. docs / plan / progress / report の差分を読み、bundle helper actualization の記述が current docs-only judgment と整合しているかを確認した。
6. `cargo test -p mir-semantics --test current_l2_detached_bundle_support`、`python3 scripts/validate_docs.py`、`git diff --check`、`git diff -- crates/mir-semantics/src/lib.rs crates/mir-semantics/src/harness.rs` を実行し、targeted verification と public API unchanged 条件を確認した。

## 5. Files changed

- `docs/reports/0154-detached-bundle-transform-helper-review.md`

## 6. Evidence / outputs / test results

```text
$ cargo test -p mir-semantics --test current_l2_detached_bundle_support
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running tests/current_l2_detached_bundle_support.rs (target/debug/deps/current_l2_detached_bundle_support-6331bdde91fe7cd9)

running 2 tests
test detached_bundle_support_preserves_static_only_fixture_split ... ok
test detached_bundle_support_preserves_runtime_fixture_core_and_context ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 153 numbered report(s).
```

```text
$ git diff --check
(no output)
```

```text
$ git diff -- crates/mir-semantics/src/lib.rs crates/mir-semantics/src/harness.rs
(no output)
```

- `current_l2_emit_detached_bundle.rs` の変更は private transform を support module へ委譲するだけで、CLI 引数、fixture loading、`run_bundle()` 呼び出し、JSON 出力経路は維持されていた。
- 新規 `current_l2_detached_bundle_support.rs` は bundle artifact carrier と pure transform だけを保持し、path policy・diff helper・public re-export を持たない。
- helper 実装には `host_plan_coverage_failure` を追加する変更がなく、bundle artifact split は `bundle_context` / `payload_core` / `detached_noncore.steps_executed` のまま維持されていた。
- `lib.rs` / `harness.rs` に差分はなく、public API unchanged 制約は local working tree 上で満たされていた。
- docs / plan / report mirror は、新 spec 37 と support module 追加を一貫して追記しており、「non-production shared helper まで actualize、public exporter API は OPEN」の記述で整合していた。

## 7. What changed in understanding

- bundle-side detached artifact transform も aggregate / static-gate と同じく `examples/support/` まで actualize してよい current narrow cut として整っている。
- 今回の working tree change は、detached artifact split や aggregate-only `host_plan_coverage_failure` judgment を動かす task ではなく、repo 内 callable boundary の symmetry を回復する task と理解してよい。
- 現時点の主な残課題は semantics ではなく、helper actualization 後の compare coverage と fixture authoring loop の実地反復である。

## 8. Open questions

- bundle support helper の payload mapping について、`non_admissible_metadata` や `narrative_explanations` を持つ runtime fixture での direct assertion を追加する価値があるか。
- `docs/reports/0153-detached-bundle-transform-helper.md` の “code mapper subagent” 記述について、review evidence として追加出力を残す必要があるか。

## 9. Suggested next prompt

- current working tree を前提に、`detached bundle transform helper` へ direct payload-shape coverage を 1 本追加してください。特に `non_admissible_metadata` または `narrative_explanations` を持つ fixture を使い、bundle support helper の extraction が bundle-first split を壊していないことを test で固定してください。
