# Report 0119 — review of detached validation-loop actualization and progress refresh

## 1. Title and identifier

- Report 0119
- review of detached validation-loop actualization and progress refresh

## 2. Objective

report `0118` と、その task で入れた detached validation-loop actualization 一式について、

- detached aggregate emitter の actual sketch が current docs-only judgment と矛盾していないか
- helper boundary と traceability mirror が壊れていないか

を final review として確認し、reviewer finding があれば narrow に補正して閉じる。

## 3. Scope and assumptions

- current L2 の core semantics、parser grammar、machine-check policy は変えない。
- review 対象は `0118` task の helper / docs / plan mirror 一式に限定する。
- review で出た修正は narrow に留める。
- `plan/ 更新不要`
- `progress.md 更新不要`

## 4. Documents consulted

1. `docs/reports/0118-detached-validation-loop-actualization-and-progress-refresh.md`
2. `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
3. `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
4. `specs/examples/25-current-l2-detached-aggregate-emitter-sketch.md`
5. `plan/07-parser-free-poc-stack.md`
6. `plan/09-helper-stack-and-responsibility-map.md`
7. `plan/90-source-traceability.md`
8. `crates/mir-semantics/examples/current_l2_emit_detached_aggregate.rs`
9. `scripts/current_l2_detached_loop.py`

## 5. Actions taken

1. final reviewer を 1 回だけ使い、aggregate helper と docs mirror の整合を確認した。
2. reviewer finding を 2 件受け取り、scope marker と traceability mirror を narrow に補正した。
3. `0118` report 本文に reviewer follow-up を追記し、files changed list の落ちも補った。

## 6. Evidence / outputs / test results

### reviewer findings

1. `bundle_failure_kind_counts` が full histogram と誤読されうる。
   - current actual helper は `host-plan-coverage-failure` だけを数えるので、scope を明示しないと docs-only judgment とズレる。
2. `plan/90-source-traceability.md` に report `0118` の導線が落ちている。

### fixes applied

1. aggregate helper と docs mirror に `bundle_failure_kind_counts_scope = "migrated-kinds-only"` を追加した。
2. `plan/90-source-traceability.md` に `0118` を追記した。
3. `0118` report に reviewer follow-up を追記した。

## 7. What changed in understanding

- current aggregate emitter の `bundle_failure_kind_counts` は、現段階では full failure histogram ではなく **migrated kind only の partial histogram** としてしか読めない。
- この partial 性は helper 実装だけでなく docs mirror と report にも明示しないと、future typed aggregate の読みを誤らせる。
- traceability mirror は helper 実装と同時に report chain も拾う必要があり、review 記録は narrow task でも有効である。

## 8. Open questions

- `bundle_failure_kind_counts_scope` を actual future API にも残すか、それとも full histogram へ移る過渡期だけの field にするか。
- aggregate compare helper を別 task で足すとき、partial histogram の比較 contract をどこまで exact-compare core に寄せるか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、detached validation loop の aggregate compare helper を narrow に追加し、bundle_failure_kind_counts_scope = "migrated-kinds-only" を前提に 2 run の aggregate artifact を比較できる non-production helper を検討してください。`
