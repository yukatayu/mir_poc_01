# 323 — current L2 syntax-backed-sample-runner-first-cut-ready verification-ladder-wiring comparison

## 目的

`specs/examples/321-current-l2-actual-parser-to-program-lowering-first-cut-ready-syntax-backed-sample-runner-first-cut-comparison.md`
と
`specs/examples/322-current-l2-syntax-backed-sample-runner-first-cut-ready-minimal-syntax-backed-sample-runner-first-cut-threshold.md`
で syntax-backed sample runner first cut を fixed した次段として、

- sample ごとの `static gate` / `interpreter` / `formal hook` reached stage をどこで揃えるか
- current authored source sample first trio と source target only row をどう分けるか
- tool-neutral formal hook を current top に維持したまま、concrete theorem/model-check line をどう guard するか

を比較する。

ここで固定するのは
**current L2 syntax-backed-sample-runner-first-cut-ready verification-ladder-wiring comparison**
であり、

- source-sample bless / regression update flow
- theorem-first concrete tool choice
- final public runner / CLI widening

はまだ固定しない。

## scope

- entry criteria は `specs/examples/303...304` と `315...322` で fixed 済みの tool-neutral formal hook、source corpus scope / layout、mapping matrix、lowering first cut、runner first cut とする。
- current authored source sample は `e4` / `e2` / `e23` first trio に留める。
- representative / fixture / source mapping matrix 上の `e1` / `e3` / `e21` は source target only row として扱い、failure 扱いしない。

## current 前提

current repo では次が成立している。

1. `samples/current-l2/README.md` と `specs/examples/317...318` は current initial cluster 6 本の representative / fixture / source target path を固定済みであり、この package では first authored trio に限った reached stage inventory を narrow addendum として載せるかが問いになる。
2. `mir_runtime::current_l2::run_current_l2_source_sample` は helper-local runner として first authored trio `e4` / `e2` / `e23` を read / lower / static / interpreter へ fail-closed に束ねる。
3. `crates/mir-runtime/tests/current_l2_source_lowering.rs` と `current_l2_source_sample_runner.rs` は first trio の source-backed static/interpreter floor を already machine-check している。
4. `crates/mir-semantics/examples/support/current_l2_formal_hook_support.rs` と detached/static smoke helper は、tool-neutral formal hook の subject kind を `runtime_try_cut_cluster` と `fixture_static_cluster` に narrow actualize 済みである。

したがって current 問いは、
**first authored trio の reached stage を narrow matrix + helper evidence に切り、source target only row を still later authored set として保つのが自然か**
である。

## 比較観点

1. current authored sample ごとの reached stage を明示できるか
2. authored sample と source target only row を混同せずに保てるか
3. formal hook top を tool-neutral に維持できるか
4. bless policy / concrete tool choice / final runner widening を premature に混ぜないか

## 比較対象

### 案 1. first authored trio だけを ladder row にし、source target only row は deferred authored set に置く

#### shape

```text
source_sample_verification_ladder = {
  ladder_kind = current_l2_source_sample_verification_ladder_first_trio,
  fixed_entry_criteria_refs = [
    phase6_compile_ready_formal_hook,
    phase6_source_mapping_matrix,
    phase6_source_lowering_first_cut,
    phase6_source_runner_first_cut
  ],
  authored_sample_rows = [
    {
      sample_stem = e2_try_fallback,
      authored_status = source_authored,
      static_gate_stage = reached_valid_static_gate,
      interpreter_stage = reached_runtime_success,
      formal_hook_stage = reached_runtime_try_cut_cluster,
      evidence_refs = [
        current_l2_source_sample_runner_test_e2,
        current_l2_formal_hook_runtime_smoke_e2
      ]
    },
    {
      sample_stem = e4_malformed_lineage,
      authored_status = source_authored,
      static_gate_stage = reached_malformed_static_gate,
      interpreter_stage = not_reached_static_stop,
      formal_hook_stage = reached_fixture_static_cluster,
      evidence_refs = [
        current_l2_source_sample_runner_test_e4,
        current_l2_formal_hook_static_smoke_e4
      ]
    },
    {
      sample_stem = e23_malformed_try_fallback_missing_fallback_body,
      authored_status = source_authored,
      static_gate_stage = reached_malformed_static_gate,
      interpreter_stage = not_reached_static_stop,
      formal_hook_stage = reached_fixture_static_cluster,
      evidence_refs = [
        current_l2_source_sample_runner_test_e23,
        current_l2_formal_hook_static_smoke_e23
      ]
    }
  ],
  deferred_source_target_rows = [
    { sample_stem = e1_place_atomic_cut, authored_status = source_target_only_not_yet_authored },
    { sample_stem = e3_option_admit_chain, authored_status = source_target_only_not_yet_authored },
    { sample_stem = e21_try_atomic_cut_frontier, authored_status = source_target_only_not_yet_authored }
  ],
  guard_refs = [
    keep_tool_neutral_formal_hook_as_current_top,
    keep_first_trio_as_current_authored_set,
    avoid_public_runner_widening,
    avoid_concrete_tool_binding_backpressure
  ]
}
```

#### 利点

- current authored set と current target-only row を混ぜずに済む。
- existing helper evidence をそのまま ladder row に接続できる。
- authoring / bless policy を next package に押し分けやすい。

#### 欠点

- current initial cluster 6 本すべてが runnable になるわけではない。

### 案 2. initial cluster 6 本をすべて authored sample として current ladder に含める

#### 利点

- table 上は一様になる。

#### 欠点

- `e1` / `e3` / `e21` source file actualization と accepted sample surface wideningを同時に進める必要がある。
- current authored set と bless policy を premature に混ぜやすい。

### 案 3. reached stage inventory を docs に置かず、final public runner / concrete tool binding と同時に扱う

#### 利点

- temporary matrix を減らせる。

#### 欠点

- current bottleneck を先送りする。
- source sample path と tool-neutral formal hook の current entry criteria が繋がらない。

## current judgment

current L2 で最も自然なのは、
**案 1. first authored trio だけを ladder row にし、source target only row は deferred authored set に置く**
である。

理由は次の通り。

1. current authored sample set は `e4` / `e2` / `e23` で source-backed evidence が既にある。
2. `e1` / `e3` / `e21` は mapping matrix 上の target path として保持しつつ、authoring/policy task まで widened authored set にしない方が guard が明確である。
3. formal hook reached stage は runtime/static detached artifact 経由の tool-neutral top で十分であり、concrete theorem/model-check line をこの package に混ぜる必要がない。

## current first choice shape

```text
source_sample_verification_ladder = {
  ladder_kind = current_l2_source_sample_verification_ladder_first_trio,
  fixed_entry_criteria_refs = [
    phase6_compile_ready_formal_hook,
    phase6_source_mapping_matrix,
    phase6_source_lowering_first_cut,
    phase6_source_runner_first_cut
  ],
  authored_sample_rows = [
    { sample_stem = e2_try_fallback, authored_status = source_authored, static_gate_stage = reached_valid_static_gate, interpreter_stage = reached_runtime_success, formal_hook_stage = reached_runtime_try_cut_cluster, evidence_refs = [current_l2_source_sample_runner_test_e2, current_l2_source_sample_verification_ladder_test_e2, current_l2_formal_hook_runtime_smoke_e2] },
    { sample_stem = e4_malformed_lineage, authored_status = source_authored, static_gate_stage = reached_malformed_static_gate, interpreter_stage = not_reached_static_stop, formal_hook_stage = reached_fixture_static_cluster, evidence_refs = [current_l2_source_sample_runner_test_e4, current_l2_source_sample_verification_ladder_test_e4, current_l2_formal_hook_static_smoke_e4] },
    { sample_stem = e23_malformed_try_fallback_missing_fallback_body, authored_status = source_authored, static_gate_stage = reached_malformed_static_gate, interpreter_stage = not_reached_static_stop, formal_hook_stage = reached_fixture_static_cluster, evidence_refs = [current_l2_source_sample_runner_test_e23, current_l2_source_sample_verification_ladder_test_e23, current_l2_formal_hook_static_smoke_e23] }
  ],
  deferred_source_target_rows = [
    { sample_stem = e1_place_atomic_cut, authored_status = source_target_only_not_yet_authored },
    { sample_stem = e3_option_admit_chain, authored_status = source_target_only_not_yet_authored },
    { sample_stem = e21_try_atomic_cut_frontier, authored_status = source_target_only_not_yet_authored }
  ],
  guard_refs = [
    keep_tool_neutral_formal_hook_as_current_top,
    keep_first_trio_as_current_authored_set,
    avoid_public_runner_widening,
    avoid_concrete_tool_binding_backpressure
  ]
}
```

## practical reading

current verification ladder wiring が示すのは、次の 6 点である。

1. current ladder row は first authored trio `e4` / `e2` / `e23` に留める
2. `e1` / `e3` / `e21` は source target only / not yet authored row として保持する
3. `e2` は `static gate -> interpreter -> runtime_try_cut_cluster formal hook` まで reached と読む
4. `e4` / `e23` は `static gate -> fixture_static_cluster formal hook` まで reached と読み、interpreter は static stop で not reached と読む
5. formal hook reached stage は current package では tool-neutral detached artifact route を使う
6. source-sample bless policy / concrete theorem/model-check tool binding / final runner widening は still later に残す

## next promoted line

next promoted line は、
**verification-ladder-wiring-ready source-sample-authoring-bless-regression-policy comparison**
に置く。

## open questions

- source target only row `e1` / `e3` / `e21` をどの task で authored set へ昇格するか
- source-sample bless policy で reached-stage matrix と update flow をどこまで同居させるか
- source-runner-native formal artifact を later に切る必要があるか
