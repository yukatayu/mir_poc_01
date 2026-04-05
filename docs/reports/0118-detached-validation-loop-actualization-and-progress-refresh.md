# Report 0118 — detached validation-loop actualization and progress refresh

## 1. Title and identifier

- Report 0118
- detached validation-loop actualization and progress refresh

## 2. Objective

current L2 parser-free PoC 基盤を前提に、

- `progress.md` を current repo の実装・運用状況へ合わせて更新し、
- detached validation loop の aggregate 側 actual narrow cut を実際に動く non-production helper として揃え、
- fixture authoring / detached export / aggregate summary の current loop を 1 段前へ進める。

あわせて、今後の長期研究 task で必要になる

- progress / maintenance の更新規律
- disk / memory 監視
- `--no-gpg-sign` commit
- commit ごとの push
- OS / hardware 非依存性
- step 実行 / graph 可視化に伸ばせる observability 境界

を mirror 文書へ残す。

## 3. Scope and assumptions

- current L2 の core semantics、failure family、parser grammar、machine-check policy は変更しない。
- detached exporter は production API に昇格させない。
- aggregate 側は `run_directory` / `BatchRunSummary` 起点の non-production helper に留める。
- `host_plan_coverage_failure` は success-side payload core へ逆流させない。
- `bundle_failure_kind_counts` は aggregate 側の additive typed field 候補として扱い、current list anchor を直ちに除去しない。
- current actual sketch では `bundle_failure_kind_counts` を migrated kind only の partial histogram とし、`bundle_failure_kind_counts_scope = "migrated-kinds-only"` でその範囲を明示する。
- task 開始時点の worktree は clean だった。

## 4. Documents consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. `specs/10-open-questions.md`
10. `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
11. `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
12. `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`
13. `plan/00-index.md`
14. `plan/07-parser-free-poc-stack.md`
15. `plan/09-helper-stack-and-responsibility-map.md`
16. `plan/11-roadmap-near-term.md`
17. `plan/12-open-problems-and-risks.md`
18. `plan/15-current-l2-fixture-authoring-template.md`
19. `plan/90-source-traceability.md`
20. `plan/91-maintenance-rules.md`
21. `progress.md`
22. `crates/mir-semantics/src/harness.rs`
23. `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
24. `scripts/current_l2_detached_loop.py`
25. `scripts/current_l2_diff_detached_artifacts.py`
26. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
27. `crates/mir-ast/tests/fixtures/current-l2/`

## 5. Actions taken

1. `progress.md` を current detached validation loop 状態へ合わせて更新し、aggregate emitter 実装後の rough estimate と、portability / observability reminder を反映した。
2. `AGENTS.md` と `plan/91-maintenance-rules.md` に、long-running research task の disk / memory 確認、`git commit --no-gpg-sign`、commit ごとの push を追記した。
3. detached validation loop の aggregate 側 actual narrow cut と current storage / migration judgment を docs mirror に反映した。
4. `crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs` を追加し、`run_directory` / `BatchRunSummary` 起点で
   - `aggregate_context`
   - `summary_core`
   - `detached_noncore`
   を持つ non-production aggregate detached artifact を JSON へ出せるようにした。
5. `scripts/current_l2_detached_loop.py` の current narrow loop をレビューし、`emit-aggregate` subcommand を current helper boundary として採用する前提で smoke を取り直した。
6. Python unittest を `scripts/tests/test_current_l2_detached_loop.py` に追加し、aggregate artifact path 導出が
   - `target/current-l2-detached/aggregates/<run-label>/batch-summary.detached.json`
   に落ちること
   - `run_label` validation を再利用すること
   を固定した。
7. `Documentation.md` の current helper 導線と environment memo を更新した。
8. relevant `plan/` mirror、`specs/00-document-map.md`、`plan/90-source-traceability.md` を current state に合わせて揃えた。
9. wrapper を用いて、
   - bundle artifact export
   - aggregate summary export
   - bundle payload core diff
   の smoke evidence を取り、current loop が回ることを確認した。

## 6. Files changed

- `AGENTS.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `progress.md`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_detached_loop.py`
- `specs/00-document-map.md`
- `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
- `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
- `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`
- `crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs`

## 7. Commands run

```bash
git status --short --branch
df -h .
free -h
cargo test -p mir-semantics --example current_l2_emit_detached_aggregate
python3 -m unittest scripts/tests/test_current_l2_detached_loop.py
python3 scripts/current_l2_detached_loop.py emit-fixture crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --run-label sprint-e3 --overwrite
python3 scripts/current_l2_detached_loop.py emit-aggregate crates/mir-ast/tests/fixtures/current-l2 --run-label sprint-batch --overwrite
python3 scripts/current_l2_detached_loop.py compare-fixtures crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.json --left-label sprint-left --right-label sprint-right --overwrite
cat target/current-l2-detached/aggregates/sprint-batch/batch-summary.detached.json
python3 scripts/validate_docs.py
python3 -m unittest scripts/tests/test_current_l2_detached_loop.py
cargo test -p mir-semantics --example current_l2_emit_detached_aggregate
cargo test -p mir-semantics
git diff --check
```

## 8. Evidence / outputs / test results

### task-start dirty state

```bash
git status --short --branch
## main...origin/main
```

### resource check

```bash
df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   76G   19G  81% /

free -h
Mem: 960Mi total / 304Mi available
Swap: 19Gi total / 19Gi available
```

### TDD red/green evidence

```bash
cargo test -p mir-semantics --example current_l2_emit_detached_aggregate
```

- red:
  - `not yet implemented: aggregate detached artifact mapping is implemented after failing tests`
- red (second pass):
  - `DetachedAggregateArtifact has no field named aggregate_core`
  - `missing field host_plan_coverage_failures`
- green:
  - example test 2 件 pass

```bash
python3 -m unittest scripts/tests/test_current_l2_detached_loop.py
```

- green:
  - 2 tests pass

### loop smoke evidence

```bash
python3 scripts/current_l2_detached_loop.py emit-fixture crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --run-label sprint-e3 --overwrite
```

- output:
  - `target/current-l2-detached/bundles/sprint-e3/e3-option-admit-chain.detached.json`

```bash
python3 scripts/current_l2_detached_loop.py emit-aggregate crates/mir-ast/tests/fixtures/current-l2 --run-label sprint-batch --overwrite
```

- output:
  - `target/current-l2-detached/aggregates/sprint-batch/batch-summary.detached.json`

aggregate artifact の主要部:

```json
{
  "aggregate_context": {
    "directory_path": "crates/mir-ast/tests/fixtures/current-l2",
    "aggregate_scope": "directory-all"
  },
  "summary_core": {
    "total_bundles": 8,
    "runtime_bundles": 6,
    "static_only_bundles": 2,
    "passed": 8,
    "failed": 0,
    "bundle_failure_kind_counts_scope": "migrated-kinds-only",
    "bundle_failure_kind_counts": []
  }
}
```

```bash
python3 scripts/current_l2_detached_loop.py compare-fixtures crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.json --left-label sprint-left --right-label sprint-right --overwrite
```

- diff helper は exit code 1 で expected difference を返した。
- 主差分:
  - `terminal_outcome`
  - `event_kinds`
  - `non_admissible_metadata`
  - `narrative_explanations`

### reviewer follow-up

- reviewer finding 1:
  - `bundle_failure_kind_counts` が full histogram に見えるのに、actual helper は `host-plan-coverage-failure` だけを数えていた。
  - fix:
    - `bundle_failure_kind_counts_scope = "migrated-kinds-only"` を actual helper と docs mirror に追加した。
- reviewer finding 2:
  - `plan/90-source-traceability.md` に report `0118` の traceability が落ちていた。
  - fix:
    - relevant row に `0118` を追記した。

### fresh verification

```bash
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 119 numbered report(s).

python3 -m unittest scripts/tests/test_current_l2_detached_loop.py
Ran 2 tests in 0.000s
OK

cargo test -p mir-semantics --example current_l2_emit_detached_aggregate
2 passed; 0 failed

cargo test -p mir-semantics
unit 2 passed; integration 33 passed; doc tests 0

git diff --check
# no output
```

## 9. What changed in understanding

- detached validation loop は、bundle-first artifact だけでなく aggregate summary も non-production helper として実際に保存できる段階に入った。
- ただしこれは aggregate export の production API finalization を意味しない。`BatchRunSummary` の coarse role、current list anchor、`bundle_failure_kind_counts` の additive coexistence は維持する必要がある。
- actual sketch の `bundle_failure_kind_counts` は full histogram ではなく migrated kind only の partial histogram であり、その範囲を `bundle_failure_kind_counts_scope` で明示するのが安全である。
- `fixture authoring / elaboration` bottleneck は依然として独立であり、aggregate emitter が入っても自動化済みになったわけではない。次は「fixture を 1 本足し、bundle artifact と aggregate summary を取り、差分を読む」実地反復が主になる。
- portability / observability は current semantics blocker ではないが、helper boundary を焼き付ける前に reminder として管理し続ける価値がある。

## 10. Open questions

- aggregate compare helper を bundle diff helper と同じ粒度で足すべきか、それとも current loop では summary 読み取りだけで十分か。
- aggregate exporter を named profile / profiled summary へ広げる timing。
- `bundle_failure_kind_counts` の actual public API cut と、current list/bool anchor をいつ除去するか。
- fixture authoring を補助する tiny elaboration helper をどこまで許すか。
- portability / observability hook の concrete API をいつ comparison から implementation candidate に上げるか。

## 11. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、detached validation loop を実地で 1 本進めるため、新しい narrow regression fixture を 1 本追加し、bundle artifact と aggregate summary の両方を保存して、既存 fixture との差分を読みながら fixture authoring template の不足を洗い出してください。`
