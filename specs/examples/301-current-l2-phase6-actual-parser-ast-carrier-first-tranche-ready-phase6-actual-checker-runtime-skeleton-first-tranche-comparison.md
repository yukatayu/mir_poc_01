# 301 — current L2 phase6-actual-parser-ast-carrier-first-tranche-ready phase6-actual-checker-runtime-skeleton-first-tranche comparison

## 目的

`specs/examples/299-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-phase6-actual-parser-ast-carrier-first-tranche-comparison.md`
と
`specs/examples/300-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-minimal-phase6-actual-parser-ast-carrier-first-tranche-threshold.md`
で Phase 6 parser first tranche を fixed した次段として、

- parsed subset -> checker floor -> runtime skeleton の compile path をどの cut で actualize するか
- `mir-semantics` の parser-free fixture wrapper 依存をどこまで薄く剥がすか
- `mir-runtime` をどこまで non-production thin orchestrator に留め、どこから later public surface に残すか

を比較する。

ここで固定するのは
**current L2 phase6-actual-parser-ast-carrier-first-tranche-ready phase6-actual-checker-runtime-skeleton-first-tranche comparison**
であり、

- actual parser-to-`Program` lowering
- stage 3 request / predicate reconnect
- richer host interface
- final public runtime / checker API
- formal hook concrete tool binding

はまだ固定しない。

## scope

- current L2 subset の Phase 6 front-half first checker/runtime tranche だけを扱う。
- parser side の accepted floor source は `specs/examples/287...290` と `specs/examples/299...300` を前提にする。
- code anchor は `crates/mir-semantics/src/lib.rs`、`crates/mir-semantics/src/harness.rs`、`crates/mir-runtime/src/current_l2.rs`、`crates/mir-runtime/tests/current_l2_runtime_skeleton.rs` を主に扱う。
- Phase 5 closeout で retained-later に残した formal hook concrete binding は本 task の範囲外に置く。

## current 前提

current repo では次が成立している。

1. `mir-ast` には stage 1 option/chain と stage 2 try/fallback structural floor だけを持つ non-production carrier が actualize 済みである。
2. `mir-semantics` には parser-free current L2 interpreter と harness があり、`CurrentL2Fixture` 単位の static gate / evaluation path は compile / test evidence を持つ。
3. `mir-runtime` public crate は placeholder skeleton のままであり、parsed subset と semantic evaluator をつなぐ actual compile path はまだ薄い。

したがって current 問いは、
**parser first tranche を壊さずに、どの cut なら checker/runtime first tranche を compile-ready と読めるか**
である。

## 比較観点

1. parsed subset -> checker floor -> runtime skeleton の compile path を actual code anchor として示せるか
2. parser-first-tranche evidence と semantic `Program` を混同せず、boundary を明示できるか
3. `mir-runtime` を premature な public API にせずに済むか
4. next line を compile-ready verification / formal hook first tranche へ自然に送れるか

## 比較対象

### 案 1. `mir-runtime` を placeholder のまま維持し、Phase 6 compile-ready gate を docs-only に留める

#### 利点

- library surface は増えない
- public runtime API の premature 既成事実化を避けやすい

#### 欠点

- parsed subset -> checker floor -> runtime skeleton の compile path を actual code anchor として閉じられない
- `mir-semantics` program-level entry の薄い actualization が先送りになる
- compile-ready checkpoint の入口が docs-only のまま残る

### 案 2. `mir-semantics` program-level entry と `mir-runtime::current_l2` thin orchestrator を first tranche として actualize する

#### shape

```text
phase6_current_l2_checker_runtime_first_tranche = {
  skeleton_kind = current_l2_nonproduction_checker_runtime_skeleton,
  semantic_entry_refs = [
    static_gate_program_detailed,
    direct_style_evaluator_from_program,
    fixture_host_stub_run_program
  ],
  runtime_bridge_refs = [
    mir_runtime_current_l2_module,
    current_l2_runtime_skeleton_report
  ],
  parser_bridge_contract_refs = [
    stage1_reconnect_clusters,
    stage2_try_rollback_structural_summary,
    parser_bridge_consistency_guard
  ],
  retained_later_refs = [
    parser_to_program_lowering,
    stage3_request_predicate_reconnect,
    richer_host_interface,
    final_public_runtime_checker_api,
    formal_hook_concrete_tool_binding
  ]
}
```

#### 利点

- semantic `Program` を直接受ける checker/runtime compile path を actualize できる
- stage 1 / stage 2 parser evidence は optional parser bridge input として明示 separation できる
- parser bridge と semantic `Program` の mismatch guard を first tranche contract に入れられる
- next line を formal hook / compile-ready verification first tranche へ自然に送れる

#### 欠点

- `mir-runtime` は placeholder ではなくなる
- actual parser-to-`Program` lowering がまだ later であることを wording で明示する必要がある

### 案 3. actual parser-to-`Program` lowering や public runtime surface まで同時に widen する

#### 利点

- compile path は大きく見える

#### 欠点

- parser first tranche で intentionally 留めた boundary を越えて widen しやすい
- stage 3 reconnect / richer host / public runtime API / formal tool binding が一気に逆流する
- Phase 6 front-half first tranche の minimum を重くしすぎる

## current judgment

current L2 で最も自然なのは、
**案 2. `mir-semantics` program-level entry と `mir-runtime::current_l2` thin orchestrator を first tranche として actualize する**
である。

理由は次の通り。

1. Phase 6 front-half で必要なのは final runtime ではなく、compile-ready minimal path の入口である。
2. parser first tranche は `Program` そのものの actual lowering ではないため、semantic `Program` と parser evidence を分けて持つ thin bridge が必要である。
3. `static_gate_program_detailed`、`DirectStyleEvaluator::from_program`、`FixtureHostStub::run_program` を切り出せば、fixture wrapper を壊さずに program-level entry を actualize できる。
4. `mir-runtime::current_l2` に thin orchestrator と bridge consistency guard を置けば、compile path actualization と later retained line の境界を同時に示せる。

## current first choice shape

```text
phase6_current_l2_checker_runtime_first_tranche = {
  skeleton_kind = current_l2_nonproduction_checker_runtime_skeleton,
  semantic_entry_refs = [
    static_gate_program_detailed,
    direct_style_evaluator_from_program,
    fixture_host_stub_run_program
  ],
  runtime_bridge_refs = [
    mir_runtime_current_l2_module,
    current_l2_runtime_skeleton_report
  ],
  parser_bridge_contract_refs = [
    stage1_reconnect_clusters,
    stage2_try_rollback_structural_summary,
    parser_bridge_consistency_guard
  ],
  retained_later_refs = [
    parser_to_program_lowering,
    stage3_request_predicate_reconnect,
    richer_host_interface,
    final_public_runtime_checker_api,
    formal_hook_concrete_tool_binding
  ]
}
```

## practical reading

current Phase 6 checker/runtime first tranche が示すのは、次の 5 点だけである。

1. `mir-semantics` に program-level static gate / evaluator / host-runner entry を足す
2. `mir-runtime` に current L2 thin orchestrator module を足す
3. parser evidence は stage 1 summary / stage 2 structural summary の optional bridge input として扱う
4. bridge input と semantic `Program` の mismatch は fail-closed に止める
5. actual parser-to-`Program` lowering、stage 3 reconnect、richer host、public runtime API、formal hook concrete binding は still later に残す

## next promoted line

next promoted line は、
**phase6-actual-checker-runtime-skeleton-first-tranche-ready phase6-compile-ready-verification-and-formal-hook comparison**
に置く。

## open questions

- tool-neutral formal hook を theorem-side / model-check side のどの row familyで first tranche 化するか
- actual parser-to-`Program` lowering を second tranche widen のどこで reopen するか
- `mir-runtime` current L2 module を later public surface とどの時点で分けるか
