# 363 — current L2 stable-static-edge-pair-first-reopen-ready public-operational-surface-actualization-gate comparison

## 目的

`specs/examples/362-current-l2-stable-static-edge-pair-first-reopen-ready-minimal-stable-static-edge-pair-first-reopen-threshold.md`
で stable-static edge-pair first reopen の minimum を fixed した次段として、

- parser / checker / runtime public surface inventory を current actualization gate へどこまで進めるか
- already-public parser-free behavior を壊さずに、どの compile-ready tranche を first public-pressure candidate に置くか
- repo-layout 焼き込み helper、example/support surface、repo-local Python orchestration をどこまで非 public に残すか

を比較する。

ここで固定するのは
**current L2 stable-static-edge-pair-first-reopen-ready public-operational-surface-actualization-gate comparison**
であり、

- final public parser / checker / runtime API
- public operational CLI
- shared-space identity / auth layering reopen
- model-check concrete carrier first actualization gate

はまだ固定しない。

## scope

- current package は docs-only gate に留める。
- already-public parser-free helper stack は stable bucket として維持する。
- current compile-ready tranche は symbol-level で切り、crate 単位の一括 promotion はしない。
- repo-local script / example / support surface は public contract 候補へ上げない。

## current 前提

current repo では次が成立している。

1. `specs/examples/355...356` により、public-looking surface は
   - already-public parser-free helper stack
   - crate-public but non-production compile-ready tranche
   - repo-local helper / example emitter surface
   の 3 bucket split で inventory 済みである。
2. `specs/examples/321...324` と `333...362` により、`mir_runtime::current_l2` の fixed-subset source-sample path は authored octet / runner / ladder / regression まで actualize 済みである。
3. `plan/09-helper-stack-and-responsibility-map.md` の current inventory では、parser-free helper stack と compile-ready tranche と repo-local operational aid の guard を `pub visibility != final public contract` の形で維持している。
4. stable-static edge-pair first reopen も closed したので、current immediate question は broader malformed widening ではなく public-pressure gate の narrow actualization である。

したがって current 問いは、
**already-public parser-free stack を据え置いたまま、later public pressure の first candidate を `mir_runtime::current_l2` source-sample runner family へ narrow に置くのが最小か**
である。

## 比較観点

1. already-public parser-free behavior を安定側に残せるか
2. compile-ready tranche を crate 単位ではなく symbol 単位で narrow に切れるか
3. repo-layout 焼き込み helper と repo-local operational aid を public contract 候補から外せるか
4. next promoted line を shared-space identity / auth layering reopen へ clean に handoff できるか

## 比較対象

### 案 1. `run_current_l2_source_sample` を中心にした source-sample runner family だけを first docs-only candidate に置く

#### shape

```text
public_operational_surface_actualization_gate = {
  gate_kind = current_l2_docs_only_public_pressure_gate,
  stable_public_refs = [
    parser_free_bundle_discovery_selection_profile_stack
  ],
  first_candidate_refs = [
    current_l2_source_sample_runner_entry
  ],
  tranche_internal_support_refs = [
    current_l2_runtime_skeleton_entry,
    current_l2_source_lowering_entry,
    current_l2_parser_carrier,
    program_level_semantic_entries
  ],
  excluded_refs = [
    source_sample_path_resolution,
    repo_layout_bound_accepted_sample_set,
    example_emitters_and_support_modules,
    repo_local_python_orchestration_helpers
  ],
  guard_refs = [
    keep_parser_free_public_behavior_stable,
    do_not_promote_by_pub_visibility_only,
    avoid_freezing_sample_layout_and_host_plan_coupling,
    avoid_promoting_partial_parser_surface
  ]
}
```

#### 利点

- public pressure を current compile-ready tranche の end-to-end path へ narrow に掛けられる。
- `run_current_l2_source_sample` を候補にしつつ、`resolve_current_l2_source_sample_path` や accepted-set hard-coding を public contract 候補から外せる。
- parser carrier や repo-local script/example surface を先に固定せずに済む。

#### 欠点

- `mir_runtime::current_l2` 内でも entry と support の切り分けを prose で明示する必要がある。

### 案 2. `mir_runtime::current_l2` や `mir_ast::current_l2` を module / crate 単位で first candidate に置く

#### 利点

- docs 上の表現は単純になる。

#### 欠点

- partial parser surface や repo-layout 焼き込み helper まで accidentally promotion しやすい。
- `pub` visibility を final contract と誤読しやすい。

### 案 3. public surface inventory をそのまま維持し、actualization gate 自体は切らない

#### 利点

- 追加の境界 prose は減る。

#### 欠点

- later public pressure の first sub-cut が snapshot に残らず、current mainline が tasks / progress で drift しやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. `run_current_l2_source_sample` を中心にした source-sample runner family だけを first docs-only candidate に置く**
である。

理由は次の通り。

1. current compile-ready tranche の中で、parser carrier と program-level semantics entry を跨ぐ narrow end-to-end surface は `mir_runtime::current_l2` source-sample runner family にまとまっている。
2. ただし `resolve_current_l2_source_sample_path` と accepted sample set は repo layout を焼き込んでおり、public contract 候補へは早い。
3. parser-free helper stack は already-public stable behavior として扱うべきであり、compile-ready tranche の docs-only gate のために巻き戻すべきではない。

## current first choice details

- stable public bucket は parser-free helper stack
  - `run_bundle`
  - `discover_bundles_in_directory`
  - `select_bundles_from_request`
  - `run_directory_profiled`
  - `run_directory_named_profile`
  を中心に維持する。
- first docs-only public-pressure candidate は `mir_runtime::current_l2::run_current_l2_source_sample` に置く。
- `mir_runtime::current_l2::run_current_l2_runtime_skeleton` と `lower_current_l2_fixed_source_text` は tranche-internal support に留める。
- `resolve_current_l2_source_sample_path` と hard-coded accepted-set は repo-layout / sample policy coupling を持つため、public contract 候補から外す。
- `mir_ast::current_l2` parser carrier、program-level semantics entry、example/support modules、repo-local Python orchestration helper は still non-public bucket に残す。

## next promoted line

next promoted line は、
**public-operational-surface-actualization-gate-ready shared-space-identity-auth-layering-reopen comparison**
に置く。

## open questions

- `run_current_l2_source_sample` の host-plan coupling を later public surface でどう薄くするか
- `run_current_l2_runtime_skeleton` を public-facing support へ上げる必要が later にあるか
- public operational CLI を crate-level surface と分けていつ reopen するか
