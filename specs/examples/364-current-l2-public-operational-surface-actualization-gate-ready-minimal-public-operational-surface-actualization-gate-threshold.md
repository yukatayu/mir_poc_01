# 364 — current L2 public-operational-surface-actualization-gate-ready minimal-public-operational-surface-actualization-gate threshold

## 目的

`specs/examples/363-current-l2-stable-static-edge-pair-first-reopen-ready-public-operational-surface-actualization-gate-comparison.md`
で public operational surface actualization gate の current first choice を fixed した次段として、

- first docs-only candidate と stable public bucket をどこまで minimum に残すか
- tranche-internal support と excluded helper をどう guard 付きで handoff するか
- next promoted line と kept-later public contract line をどう残すか

を比較する。

ここで固定するのは
**current L2 public-operational-surface-actualization-gate-ready minimal-public-operational-surface-actualization-gate threshold**
であり、

- shared-space identity / auth layering reopen の具体形
- model-check concrete carrier first actualization gate の具体形
- final public parser / checker / runtime API
- public operational CLI

はまだ固定しない。

## 比較観点

1. first docs-only candidate を lossless に残せるか
2. stable public bucket と tranche-internal support を区別できるか
3. repo-layout 焼き込み helper と repo-local operational aid を excluded bucket に明示できるか
4. next promoted line を shared-space identity / auth layering reopen へ handoff できるか

## 比較対象

### 案 1. first candidate symbol 名だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- stable public bucket、support bucket、excluded bucket の guard が抜ける。

### 案 2. `gate_kind + stable_public_refs + first_candidate_refs + tranche_internal_support_refs + excluded_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- stable parser-free behavior、first docs-only candidate、excluded helper を minimum にまとめて残せる。
- `pub visibility != final public contract` を threshold 側の guard に維持できる。
- next line と later public contract line の両方を handoff できる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. future public CLI や final public crate surface candidate まで threshold に含める

#### 利点

- later line は見えやすい。

#### 欠点

- threshold ではなく future public contract design を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `gate_kind + stable_public_refs + first_candidate_refs + tranche_internal_support_refs + excluded_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は public contract 固定ではなく docs-only gate の narrow actualization である。
2. stable parser-free bucket と compile-ready tranche internal support を分けて残さないと、crate visibility の accidental promotion を防げない。
3. shared-space identity / auth layering reopen と model-check concrete carrier gate は next / reserve line として kept-later に残すのが最小である。

## current first choice shape

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
  ],
  kept_later_refs = [
    shared_space_identity_auth_layering_reopen,
    model_check_concrete_carrier_first_actualization_gate,
    final_public_parser_checker_runtime_api,
    public_operational_cli
  ]
}
```

## practical reading

current minimal public operational surface actualization gate が示すのは、

- parser-free helper stack は already-public stable behavior として据え置く
- first docs-only candidate は `run_current_l2_source_sample` に限定する
- `run_current_l2_runtime_skeleton` と `lower_current_l2_fixed_source_text` は tranche-internal support に留める
- `resolve_current_l2_source_sample_path`、accepted-set hard-coding、example/support surface、repo-local Python helper は excluded bucket に残す
- repo-level next line は shared-space identity / auth layering reopen に進む

という最小 cut である。

## next promoted line

next promoted line は、
**public-operational-surface-actualization-gate-ready shared-space-identity-auth-layering-reopen comparison**
に置く。

## open questions

- `run_current_l2_source_sample` の explicit `FixtureHostPlan` を later public-facing surface でどう薄くするか
- parser carrier side の first later public pressure を separate line で持つ必要があるか
- public operational CLI と public library surface をどの順で reopen するか
