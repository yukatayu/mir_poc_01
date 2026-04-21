# order_01_detail

## この文書の目的

この文書は、[`docs/research_abstract/order_01.md`](./order_01.md) で紹介されている Problem 2 の current cut を、**紹介順を変えずに**、**初心者向けに極めて丁寧に**説明する standalone 文書である。

この文書だけ読んでも、少なくとも次が分かるようにする。

- `p07`, `p08`, `p09`, `p13`, `p14` がそれぞれ何を表すか
- `publish / handoff / observe` の因果関係が、コードのどの行で表されているか
- `late join`, `publication witness`, `stale reconnect` が何を意味するか
- `underdeclared` と `malformed` の違いが何か
- `emit-scenario`, `emit-reserve` が何を確認する helper か
- `static_gate`, `entered_evaluation`, `terminal_outcome`, `steps_executed`, `witness_strengthening_status`, `formal_hook_status` をどう読むか

この文書は研究サマリであり、規範判断の正本ではない。規範判断の正本は `specs/` にある。ここでは、`order_01.md` と同じ current reading を、順序を崩さずに読み下す。

## 先に結論

Problem 2 の current line は、low-level `memory_order` をそのまま前面に出す読みではなく、共有空間の実務的な関係として

- `publish`
- `handoff`
- `observe`
- `witness`
- `authoritative-room`

を追う読みである。

ここで重要なのは、「何となく後で読めそう」ではなく、**どの出来事がどの出来事の後にしか起きてはいけないか**を明示することだ。

- `p07` は、publish してから handoff し、その後で late joiner が過去の履歴を読める representative success である。
- `p08` は、古い reconnect を silent merge せず、失敗として落としてから refresh へ回す representative success である。
- `p09` は、乱数の取得元を外部 provider に逃がしても、commit と authority handoff までは authority 側に残す reserve practical route である。
- `p13` は、「late join に見せたい」と言っているのに publish の根拠がなく、`underdeclared` で止まる。
- `p14` は、publish 自体はあるが handoff が publish より前に来ており、構造そのものが壊れているので `malformed` で止まる。

## この文書で使う用語

### `place`

`place` は、処理や状態を置く入れ子の場所である。ここでは `root` の中に `room` があり、その中に `dice_authority` がある。初心者向けに言い換えると、「どの範囲でこの出来事が起きるか」を段階的に囲っている。

### `perform`

`perform` は「この操作を実行する」を意味する。`perform publish_roll_result on dice_state` なら、`dice_state` を対象に `publish_roll_result` という操作を行う。

### `require`

`require` は、その操作を始める前に満たしていなければならない条件である。満たさなければ、その操作は許可されない。

### `ensure`

`ensure` は、その操作が成功したときに成立していてほしい結果である。初心者向けには「この行で、成功後の約束を書いている」と読むとよい。

### `atomic_cut`

`atomic_cut` は、この repo の current reading では **place-local な明示的 cut** である。大事なのは、「これが global durable commit だ」と読みすぎないことである。ここでは、「この時点までの局所的な因果まとまりをはっきり切る」ために使われている。

### `late join`

`late join` は、すでにいくつかの出来事が起きた後から room に入ってくる参加者を指す。`player_c` が `player_a` と `player_b` のやり取りの後から参加する、というのが `p07`, `p13`, `p14` のイメージである。

### `publication witness`

この文書でいう `publication witness` は、「その結果は確かに publish された」と読むための根拠である。重要なのは、これは current sample では独立した `witness ...` という文法行として現れていないことだ。実際には、

- `publish_roll_result` を行ったこと
- その publish が late join visibility に使える形で先行していること
- それが helper / checker の current rule から見て順序的に正当であること

によって、「publication witness がある」と読める。

つまり、`publication witness` は「publish の後に late join visibility を支える根拠」であり、単なるコメントではない。

### `stale reconnect`

`stale reconnect` は、古い owner snapshot に基づいた reconnect message である。平たく言うと、「もう古くなっている再接続情報」であり、今の room 状態にそのまま混ぜると履歴や ownership を壊しかねない。

### `underdeclared`

`underdeclared` は、「必要な宣言や根拠が足りない」状態である。`p13` は publish に相当する根拠が足りない。つまり、「やりたいことは書いてあるが、その前提が書かれていない」という止め方である。

### `malformed`

`malformed` は、「書いてある要素はあるが、構造や順序が壊れている」状態である。`p14` は publish も handoff も observe もあるが、並び順が current rule に反している。つまり、「部品不足」ではなく「組み方が壊れている」という止め方である。

この違いを初心者向けに一言で言うと、次のようになる。

- `underdeclared`: 必要な根拠が足りない
- `malformed`: 根拠候補はあるが、形が壊れている

## 事前準備

`order_01.md` と同じく、repo root で次を確認する。

```bash
python3 --version
cargo --version
```

実行例全文:

```text
Python 3.12.3
cargo 1.93.0 (083ac5135 2025-12-15)
```

ここで確認しているのは、Problem 2 の helper と Rust runtime をその場で再現できる最低限の前提である。

## 最短確認

まず `smoke-all` で representative bundle が全体として崩れていないことを確かめる。

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
```

このコマンドが確認していること:

- Problem 1 と Problem 2 の representative path が、まとめて再実行できること
- `problem2` については `p07 / p08` の representative pair、matrix、bundle、inspector が通ること
- 「個別 sample だけ動いた」ではなく、guided helper の入口が揃っていること

実行例全文:

```json
[
  {
    "problem_id": "problem1",
    "title": "Problem 1 theorem-first pilot bundle",
    "status": "passed",
    "step_count": 5,
    "successful_steps": 5,
    "failed_step": null,
    "smoke_command": "python3 scripts/current_l2_guided_samples.py smoke problem1",
    "sample_bundle_doc": "samples/problem-bundles/problem1-typed-theorem-model-check.md",
    "primary_samples": [
      "p06-typed-proof-owner-handoff"
    ],
    "step_labels": [
      "runtime:p06-typed-proof-owner-handoff",
      "matrix:problem1",
      "bundle:problem1",
      "inspector:p06-typed-proof-owner-handoff",
      "mapping"
    ],
    "failed_command": null,
    "failed_return_code": null,
    "failed_output_excerpt": null
  },
  {
    "problem_id": "problem2",
    "title": "Problem 2 authoritative-room scenario bundle",
    "status": "passed",
    "step_count": 7,
    "successful_steps": 7,
    "failed_step": null,
    "smoke_command": "python3 scripts/current_l2_guided_samples.py smoke problem2",
    "sample_bundle_doc": "samples/problem-bundles/problem2-order-handoff-shared-space.md",
    "primary_samples": [
      "p07-dice-late-join-visible-history",
      "p08-dice-stale-reconnect-refresh"
    ],
    "step_labels": [
      "runtime:p07-dice-late-join-visible-history",
      "runtime:p08-dice-stale-reconnect-refresh",
      "matrix:problem2",
      "bundle:problem2",
      "inspector:p07-dice-late-join-visible-history",
      "inspector:p08-dice-stale-reconnect-refresh",
      "mapping"
    ],
    "failed_command": null,
    "failed_return_code": null,
    "failed_output_excerpt": null
  }
]
```

この出力の読み方:

- `status: "passed"` は、その problem bundle の smoke entry が通ったことを示す。
- `problem2` の `primary_samples` が `p07` と `p08` なのは、current first-line representative pair がこの 2 本だからである。
- `step_labels` に `matrix:problem2` や `bundle:problem2` が入っているので、単発実行ではなく guided helper 経由の入口も確認済みだと分かる。

## 1. 代表的な成功例 `p07` を見る

`p07` は、Problem 2 の current first-line representative である。読みたいポイントは次の 3 つである。

1. 先に publish していること
2. publish 済みのものを前提に handoff していること
3. そのあとから参加する late joiner が、過去の履歴として結果と owner history を読めること

### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  run-source-sample \
  samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt \
  --format pretty
```

このコマンドが確認していること:

- source sample 自体が static gate を通るか
- runtime evaluation に入るか
- publish と handoff と observe の順序が current line に沿っているか
- formal hook preview まで到達するか

### コード全文

```text
# handoff 済みのサイコロ結果を late joiner が past-visible な履歴として読む current L2 prototype。
place root {
  place room {
    place dice_authority {
      perform publish_roll_result on dice_state
        require owner_is(player_a)
        ensure result_is_visible(room_members)

      atomic_cut

      perform handoff_dice_authority on dice_state
        require owner_is(player_a)
        ensure owner_is(player_b)

      atomic_cut

      perform observe_late_join_view on dice_state
        require read
        ensure late_join_sees_published_result(player_c)
    }
  }
}
```

### 行ごとの解説

1. `# handoff 済みのサイコロ結果を late joiner が past-visible な履歴として読む current L2 prototype。`
   このコメントは、この sample の読み筋を先に説明している。重要なのは「late joiner が今の瞬間の状態だけではなく、publish 済みの過去を履歴として読める」という点である。
2. `place root {`
   一番外側の place である。以後の出来事はこの root の内側で起きる。
3. `place room {`
   問題設定の中心になる room の place である。shared-space の舞台がここにある。
4. `place dice_authority {`
   サイコロ状態の authority を持つ place を表している。publish も handoff も observe も、この authority まわりの出来事として書かれている。
5. `perform publish_roll_result on dice_state`
   ここで publish が起きる。`publish_roll_result` は、サイコロ結果を room から見える状態へ出す操作である。`on dice_state` は対象が `dice_state` だという意味である。
6. `require owner_is(player_a)`
   publish を始める前提として、現在の owner が `player_a` であることを要求している。owner が違うなら、この publish はそのままでは許されない。
7. `ensure result_is_visible(room_members)`
   publish 成功後の約束である。room のメンバーに結果が visible になることを明示している。ここが、後で late join visibility を支える重要な根拠になる。
8. 空行
   意味論の追加ではなく、publish ブロックを読みやすく区切るための空行である。
9. `atomic_cut`
   publish した出来事を、次の handoff より前の局所的な cut として区切る。ここで大事なのは、「publish を済ませたまとまり」と「これからの handoff」を混ぜないことだ。もしこの境界を曖昧にすると、「何が publish 済みで、何が handoff 後の出来事か」が読みにくくなる。
10. 空行
    publish 段と handoff 段の区切りである。
11. `perform handoff_dice_authority on dice_state`
    authority handoff を行う。ここでは `player_a` から `player_b` へ authority を移す操作である。
12. `require owner_is(player_a)`
    handoff を始める前にも、まだ owner が `player_a` であることを要求している。つまり、「publish をしたのと同じ owner が、handoff を正しく始めている」ことを保証したい。
13. `ensure owner_is(player_b)`
    handoff 成功後の約束である。ここで owner が `player_b` に変わる。
14. 空行
    handoff ブロックの見やすさのための空行である。
15. `atomic_cut`
    handoff を observe より前の局所的な区切りとして固定する。これによって、observe は「publish より後で、handoff よりも後」という位置づけになる。
16. 空行
    handoff 段と observe 段の区切りである。
17. `perform observe_late_join_view on dice_state`
    late joiner の観測を行う。ここでは `player_c` が、遅れて room に入ってきた参加者である。
18. `require read`
    観測には read 権限が必要であることを示す。observe は handoff と違って ownership を動かさないので、ここでは `read` が前提になっている。
19. `ensure late_join_sees_published_result(player_c)`
    観測成功後の約束である。`player_c` が publish 済みの結果を見られることを明示する。ここが late join visibility の宣言そのものである。
20. `}`
    `dice_authority` place を閉じる。
21. `}`
    `room` place を閉じる。
22. `}`
    `root` place を閉じる。

### どの行が `publish / handoff / observe` の因果関係を作っているか

- publish の宣言本体は 5 行目から 7 行目である。
- publish を独立した先行段として切っているのが 9 行目の `atomic_cut` である。
- handoff の宣言本体は 11 行目から 13 行目である。
- handoff を observe より前の独立段にしているのが 15 行目の `atomic_cut` である。
- observe の宣言本体は 17 行目から 19 行目である。

この並びにより、current line では次の順序になる。

1. publish
2. handoff
3. observe

ここで順序制約が必要な理由を、具体例で言う。

- もし handoff が publish より先に起きると、「新 owner に変わった後で初めて結果を公開した」ことになり、late joiner が読む履歴の因果が曖昧になる。
- もし observe が handoff より先に起きると、「誰の authority のもとで見えた履歴なのか」がぶれる。

### `publication witness` をこの sample ではどこで読めるか

`publication witness` は独立 keyword ではないが、この sample では次の流れで読める。

- 5 行目で publish 操作を宣言する
- 7 行目で結果が room members に visible になることを約束する
- 9 行目でその publish 段を cut として切る
- 17 行目から 19 行目で、その後段の observe が late join visibility を要求する

つまり、この sample は「publish 済みである」という根拠を先に置いてから late join observe を書いている。これが `p13` との決定的な差である。

### 実行例全文

```text
shell: mir-current-l2
command: run-source-sample
sample: p07-dice-late-join-visible-history
sample_path: /home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt
host_plan_path: /home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.host-plan.json
static_gate: valid
stage1_reconnect_clusters: not_available
stage2_try_rollback_verdict: not_available
entered_evaluation: true
terminal_outcome: success
steps_executed: 9
events:
- perform-success
- atomic-cut
- perform-success
- atomic-cut
- perform-success
debug_outputs:
- dice_debug_text_output:
  - publish_roll_result: player_a -> visible
  - handoff_dice_authority: player_a -> player_b
- observer_debug_text_output:
  - late_join_view: player_c sees result+owner history
verification_preview:
  formal_hook_status: reached
  subject_kind: runtime_try_cut_cluster
  subject_ref: p07-dice-late-join-visible-history
  proof_notebook_review_unit_obligations:
  - rollback_cut_non_interference
  model_check_concrete_carrier_obligations:
  - rollback_cut_non_interference
  guard_reason: none
artifact_preview:
  proof_notebook_review_units:
  - obligation_kind: rollback_cut_non_interference
    goal_text: Review that rollback / atomic-cut evidence does not interfere with surviving runtime behavior for `p07-dice-late-join-visible-history`.
    evidence_refs:
    - fixture:p07-dice-late-join-visible-history
    - runtime_cluster:p07-dice-late-join-visible-history
  model_check_concrete_carriers:
  - obligation_kind: rollback_cut_non_interference
    evidence_refs:
    - fixture:p07-dice-late-join-visible-history
    - runtime_cluster:p07-dice-late-join-visible-history
surface_preview:
  minimal_companion:
    status: reached
    guard_reason: none
    lines:
    - profile authoritative_room_default
    - activation authority-ack
    - authority single_room_authority
    - consistency authoritative_serial_transition
    - rng authority_rng
    - publication publish_roll_result@dice_state
    - handoff handoff_dice_authority@dice_state
    - late_join published_history_visible_as_past
    compare_floor_refs:
    - compare_floor:current_l2.authoritative_room.vertical_slice
    - compare_floor:current_l2.experimental_order_handoff_surface
    guard_refs:
    - guard:semantic_honesty_first
    - guard:helper_local_companion_only
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
  stage_block_secondary:
    status: reached
    guard_reason: none
    lines:
    - transition handoff_turn(dice_owner = player_a)
    - stage publish:
    -   publish publish_roll_result@dice_state
    - stage handoff:
    -   handoff handoff_dice_authority@dice_state
    -     after publish(publish_roll_result@dice_state)
    -     requires witness(publish_roll_result@dice_state)
    - stage observe:
    -   observe late_join_view@dice_state
    -     after handoff(handoff_dice_authority@dice_state)
    compare_floor_refs:
    - compare_floor:current_l2.experimental_order_handoff_surface
    - compare_floor:current_l2.experimental_stage_block_surface
    guard_refs:
    - guard:stage_block_secondary_candidate
    - guard:helper_local_companion_only
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:authoritative_room_serial_scope_sugar
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
  serial_scope_reserve:
    status: reached
    guard_reason: none
    lines:
    - serial on dice_authority {
    -   publish publish_roll_result@dice_state
    -   handoff handoff_dice_authority@dice_state
    -     requires witness(publish_roll_result@dice_state)
    -   observe late_join_view@dice_state
    - }
    compare_floor_refs:
    - compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption
    - compare_floor:current_l2.order_handoff.serial_scope_reserve_surface
    guard_refs:
    - guard:serial_scope_room_specific_reserve
    - guard:principal_edge_row_surface_unchanged
    - guard:helper_local_surface_only
    - guard:final_source_surface_handoff_wording_later
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:serial_scope_public_promotion
    - kept_later:serial_scope_beyond_authoritative_room
    - kept_later:final_source_surface_handoff_wording
    - kept_later:final_emitted_artifact_schema
    - kept_later:final_emitted_handoff_contract
    - kept_later:final_public_witness_schema
    - kept_later:final_public_provider_receipt_schema
    - kept_later:combined_provider_witness_public_contract
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
authoritative_room_first_scenario_actual_adoption:
  status: reached
  adoption_kind: helper_local_authoritative_room_first_scenario_manifest
  profile_axis_refs:
  - profile_axis:activation:authority-ack
  - profile_axis:authority_placement:single_room_authority
  - profile_axis:consistency_mode:authoritative_serial_transition
  - profile_axis:rng_source:authority_rng
  - profile_axis:late_join:published_history_visible_as_past
  - profile_axis:fairness_claim:no_distributed_fairness_theorem_required
  relation_refs:
  - relation_family:program_order
  - relation_family:publication_order
  - relation_family:observation_order
  - relation_family:witness_order
  - relation_family:finalization_order
  - relation_family:scoped_happens_before
  authority_handoff_refs:
  - authority_handoff:owner_slot:single_room_authority
  - authority_handoff:stage_family:authoritative_serial_transition
  - authority_handoff:stage_sequence:publish_then_handoff
  - authority_handoff:payload_ref:dice_state
  runtime_evidence_refs:
  - runtime_event:perform-success
  - runtime_event:atomic-cut
  - place_record:dice_state:publish_roll_result@dice_state
  - place_record:dice_state:handoff_dice_authority@dice_state
  - debug_output:observer_debug_text_output:late_join_view: player_c sees result+owner history
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p07-dice-late-join-visible-history:rollback_cut_non_interference
  - repo_local_emitted_artifact:model_check_concrete_carrier:p07-dice-late-join-visible-history:rollback_cut_non_interference
  reserve_route_refs: []
  negative_static_stop_refs: []
  contrast_refs:
  - contrast_target:append_friendly_notice_room
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:authoritative_room_first_scenario_actual_adoption
  - compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption
  compare_floor_refs:
  - compare_floor:current_l2.authoritative_room.vertical_slice
  - compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption
  guard_refs:
  - guard:authoritative_room_first_default_profile
  - guard:representative_first_scenario_pair
  - guard:no_distributed_fairness_theorem_required
  - guard:minimal_working_subset_only
  - guard:late_join_history_visible_as_past
  kept_later_refs:
  - kept_later:auditable_authority_witness
  - kept_later:delegated_rng_service
  - kept_later:distributed_randomness_provider
  - kept_later:final_emitted_handoff_contract
  - kept_later:exhaustive_shared_space_catalog
  - kept_later:final_consistency_fairness_catalog
  guard_reason: none
authoritative_room_reserve_strengthening_lane:
  status: reached
  lane_kind: helper_local_reserve_strengthening_lane_manifest
  witness_strengthening_status: reached
  delegated_rng_service_status: guarded_not_reached
  model_check_second_line_status: reached
  witness_strengthening_refs:
  - fairness_claim:auditable_authority_witness
  - witness_field:witness_kind
  - witness_field:action_ref
  - witness_field:draw_slot
  - witness_field:draw_result
  - witness_binding:p07-dice-late-join-visible-history:authority_draw_witness
  delegated_rng_service_refs: []
  model_check_second_line_refs:
  - property_preview:row_local:p07-dice-late-join-visible-history:canonical_normalization_law
  - property_preview:row_local:p07-dice-late-join-visible-history:no_re_promotion
  - model_check_request_preflight:p07-dice-late-join-visible-history:row_local_property_preview
  - model_check_request_preflight:p07-dice-late-join-visible-history:small_cluster_semantic_projection
  - public_checker_second_reserve:boundary
  first_line_boundary_refs:
  - first_line_boundary:representative_pair_kept_at_p07_p08
  - first_line_boundary:authoritative_room_default_profile_stays_principal
  - first_line_boundary:authority_rng_default_profile_unchanged
  reserve_boundary_refs:
  - reserve_boundary:auditable_authority_witness_second_strengthening
  - reserve_boundary:delegated_rng_service_second_practical
  - reserve_boundary:model_check_second_line_not_room_profile
  - reserve_boundary:public_checker_contract_kept_later
  - reserve_boundary:witness_provider_combined_public_contract_later
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p07-dice-late-join-visible-history:rollback_cut_non_interference
  - repo_local_emitted_artifact:model_check_concrete_carrier:p07-dice-late-join-visible-history:rollback_cut_non_interference
  compare_floor_refs:
  - compare_floor:current_l2.auditable_authority_witness.strengthening
  - compare_floor:current_l2.model_check.second_line_concretization
  - compare_floor:current_l2.reserve_strengthening_lane:p07-dice-late-join-visible-history
  guard_refs:
  - guard:first_line_boundary_preserved
  - guard:reserve_components_kept_separate
  - guard:model_check_second_line_not_room_profile
  - guard:witness_provider_public_contract_later
  kept_later_refs:
  - kept_later:combined_witness_provider_public_contract
  - kept_later:final_public_witness_schema
  - kept_later:final_public_provider_receipt_schema
  - kept_later:concrete_model_check_tool_brand
  - kept_later:actual_public_checker_migration
  - kept_later:distributed_fairness_theorem
  guard_reason: none
order_handoff_source_surface_artifact_actual_adoption:
  status: reached
  adoption_kind: helper_local_source_surface_artifact_route_manifest
  profile_axis_refs:
  - profile_axis:activation:authority-ack
  - profile_axis:authority_placement:single_room_authority
  - profile_axis:consistency_mode:authoritative_serial_transition
  - profile_axis:rng_source:authority_rng
  - profile_axis:late_join:published_history_visible_as_past
  - profile_axis:fairness_claim:no_distributed_fairness_theorem_required
  principal_surface_lines:
  - publish publish_roll_result@dice_state
  - handoff handoff_dice_authority@dice_state
  -   after publish(publish_roll_result@dice_state)
  -   requires witness(publish_roll_result@dice_state)
  - observe late_join_view@dice_state
  -   after handoff(handoff_dice_authority@dice_state)
  secondary_surface_lines:
  - stage publish:
  -   publish publish_roll_result@dice_state
  - stage handoff:
  -   handoff handoff_dice_authority@dice_state
  -     after publish(publish_roll_result@dice_state)
  -     requires witness(publish_roll_result@dice_state)
  - stage observe:
  -   observe late_join_view@dice_state
  -     after handoff(handoff_dice_authority@dice_state)
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p07-dice-late-join-visible-history:rollback_cut_non_interference
  - repo_local_emitted_artifact:model_check_concrete_carrier:p07-dice-late-join-visible-history:rollback_cut_non_interference
  source_wording_route_refs:
  - order_handoff_source_wording_actual_route:p07-dice-late-join-visible-history:edge_row_vertical_continuation_principal
  - order_handoff_source_wording_actual_route:p07-dice-late-join-visible-history:readable_high_level_relation_vocabulary
  - order_handoff_source_wording_actual_route:p07-dice-late-join-visible-history:stage_block_secondary_keep
  - order_handoff_source_wording_actual_route:p07-dice-late-join-visible-history:thread_node_same_causal_language
  - order_handoff_source_wording_actual_route:p07-dice-late-join-visible-history:final_source_surface_handoff_wording_later
  emitted_artifact_candidate_keep_refs:
  - order_handoff_emitted_artifact_keep:p07-dice-late-join-visible-history:repo_local_emitted_artifact_refs_first
  - order_handoff_emitted_artifact_keep:p07-dice-late-join-visible-history:source_surface_actual_adoption_adjacent
  - order_handoff_emitted_artifact_keep:p07-dice-late-join-visible-history:witness_provider_contract_adjacent_not_collapsed
  - order_handoff_emitted_artifact_keep:p07-dice-late-join-visible-history:final_emitted_artifact_schema_later
  negative_static_stop_refs: []
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:order_handoff_source_surface_artifact_actual_adoption
  - compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption
  compare_floor_refs:
  - compare_floor:current_l2.order_handoff.surface_actual_adoption
  - compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption
  - compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption
  guard_refs:
  - guard:edge_row_vertical_continuation_principal
  - guard:stage_block_secondary_keep
  - guard:repo_local_emitted_artifact_refs_first
  - guard:final_source_surface_handoff_wording_later
  - guard:final_emitted_artifact_schema_later
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_runtime_api
  - kept_later:final_source_surface_handoff_wording
  - kept_later:final_emitted_artifact_schema
  - kept_later:final_emitted_handoff_contract
  - kept_later:final_public_witness_schema
  - kept_later:authoritative_room_serial_scope_sugar
  - kept_later:low_level_memory_order_source_surface
  - kept_later:final_modal_foundation_adoption
  guard_reason: none
order_handoff_witness_provider_public_seam_compression:
  status: reached
  compression_kind: helper_local_public_seam_manifest
  profile_axis_refs:
  - profile_axis:activation:authority-ack
  - profile_axis:authority_placement:single_room_authority
  - profile_axis:consistency_mode:authoritative_serial_transition
  - profile_axis:rng_source:authority_rng
  - profile_axis:late_join:published_history_visible_as_past
  - profile_axis:fairness_claim:no_distributed_fairness_theorem_required
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p07-dice-late-join-visible-history:rollback_cut_non_interference
  - repo_local_emitted_artifact:model_check_concrete_carrier:p07-dice-late-join-visible-history:rollback_cut_non_interference
  source_wording_route_refs:
  - order_handoff_source_wording_actual_route:p07-dice-late-join-visible-history:edge_row_vertical_continuation_principal
  - order_handoff_source_wording_actual_route:p07-dice-late-join-visible-history:readable_high_level_relation_vocabulary
  - order_handoff_source_wording_actual_route:p07-dice-late-join-visible-history:stage_block_secondary_keep
  - order_handoff_source_wording_actual_route:p07-dice-late-join-visible-history:thread_node_same_causal_language
  - order_handoff_source_wording_actual_route:p07-dice-late-join-visible-history:final_source_surface_handoff_wording_later
  emitted_artifact_candidate_keep_refs:
  - order_handoff_emitted_artifact_keep:p07-dice-late-join-visible-history:repo_local_emitted_artifact_refs_first
  - order_handoff_emitted_artifact_keep:p07-dice-late-join-visible-history:source_surface_actual_adoption_adjacent
  - order_handoff_emitted_artifact_keep:p07-dice-late-join-visible-history:witness_provider_contract_adjacent_not_collapsed
  - order_handoff_emitted_artifact_keep:p07-dice-late-join-visible-history:final_emitted_artifact_schema_later
  serial_scope_lines:
  - serial on dice_authority {
  -   publish publish_roll_result@dice_state
  -   handoff handoff_dice_authority@dice_state
  -     requires witness(publish_roll_result@dice_state)
  -   observe late_join_view@dice_state
  - }
  witness_schema_route_refs:
  - witness_provider_schema_route_actual:p07-dice-late-join-visible-history:witness_schema_candidate_keep
  - witness_provider_schema_route_actual:p07-dice-late-join-visible-history:witness_route_first
  - witness_provider_schema_route_actual:p07-dice-late-join-visible-history:repo_local_emitted_artifact_refs_first
  - witness_provider_schema_route_actual:p07-dice-late-join-visible-history:combined_public_contract_later
  provider_receipt_route_refs:
  - witness_provider_schema_route_actual:p07-dice-late-join-visible-history:provider_receipt_candidate_keep
  - witness_provider_schema_route_actual:p07-dice-late-join-visible-history:provider_route_first
  - witness_provider_schema_route_actual:p07-dice-late-join-visible-history:repo_local_emitted_artifact_refs_first
  - witness_provider_schema_route_actual:p07-dice-late-join-visible-history:delegated_provider_attestation_later
  - witness_provider_schema_route_actual:p07-dice-late-join-visible-history:combined_public_contract_later
  combined_public_contract_keep_refs:
  - witness_provider_combined_contract_keep:p07-dice-late-join-visible-history:combined_public_contract_candidate_only
  - witness_provider_combined_contract_keep:p07-dice-late-join-visible-history:final_emitted_handoff_contract_adjacent_keep
  trace_alignment_pair_refs:
  - witness_provider_emitted_contract_trace_alignment_pair:p07-dice-late-join-visible-history:rollback_cut_non_interference
  public_seam_residual_refs:
  - order_handoff_public_seam_residual:p07-dice-late-join-visible-history:final_source_surface_handoff_wording_later
  - order_handoff_public_seam_residual:p07-dice-late-join-visible-history:final_emitted_artifact_schema_later
  - shared_space_public_seam_residual:p07-dice-late-join-visible-history:public_schema_pair_first
  - shared_space_public_seam_residual:p07-dice-late-join-visible-history:delegated_attestation_and_combined_contract_second
  - shared_space_public_seam_residual:p07-dice-late-join-visible-history:final_emitted_handoff_contract_third
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:order_handoff_witness_provider_public_seam_compression
  - compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression
  compare_floor_refs:
  - compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption
  - compare_floor:current_l2.order_handoff.serial_scope_reserve_surface
  - compare_floor:current_l2.witness_provider_emitted_contract_trace_alignment_bridge
  - compare_floor:current_l2.witness_provider_final_public_contract_reopen_threshold
  - compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression
  guard_refs:
  - guard:edge_row_vertical_continuation_principal
  - guard:serial_scope_reserve_surface_only
  - guard:witness_provider_trace_alignment_bridge
  - guard:public_schema_pair_first
  - guard:delegated_attestation_and_combined_contract_second
  - guard:final_source_surface_handoff_wording_later
  - guard:final_emitted_artifact_schema_later
  - guard:final_emitted_handoff_contract_third
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_runtime_api
  - kept_later:final_source_surface_handoff_wording
  - kept_later:final_emitted_artifact_schema
  - kept_later:final_public_witness_schema
  - kept_later:final_public_provider_receipt_schema
  - kept_later:delegated_provider_attestation
  - kept_later:combined_provider_witness_public_contract
  - kept_later:final_emitted_handoff_contract
  - kept_later:authoritative_room_serial_scope_sugar
  - kept_later:low_level_memory_order_source_surface
  - kept_later:final_modal_foundation_adoption
  - kept_later:exhaustive_shared_space_catalog
  guard_reason: none
theorem_result_object_preview:
  status: reached
  preview_kind: helper_local_actualization_manifest
  subject_kind: runtime_try_cut_cluster
  subject_ref: p07-dice-late-join-visible-history
  result_object_route_refs:
  - theorem_result_object_route:p07-dice-late-join-visible-history:notebook_consumer_object_first
  - theorem_result_object_route:p07-dice-late-join-visible-history:review_unit_anchor_bundle
  - theorem_result_object_route:p07-dice-late-join-visible-history:consumer_shaped_payload_preview_only
  - theorem_result_object_route:p07-dice-late-join-visible-history:repo_local_emitted_artifact_refs
  notebook_payload_preview_refs:
  - theorem_result_payload_preview:p07-dice-late-join-visible-history:notebook_consumer_first
  - theorem_result_payload_preview:p07-dice-late-join-visible-history:review_unit_reference_bundle
  - theorem_result_payload_preview:p07-dice-late-join-visible-history:consumer_shaped_payload_preview_only
  - theorem_result_payload_preview:p07-dice-late-join-visible-history:proof_object_public_schema_later
  proof_object_schema_reserve_refs:
  - proof_object_schema_reserve:brand_neutral_binding_keep
  - proof_object_schema_reserve:proof_object_public_schema_later
  - proof_object_schema_reserve:final_public_verifier_contract_later
  actual_adoption_default_refs:
  - theorem_result_object_preview_default:notebook_consumer_object_first
  - theorem_result_object_preview_default:consumer_shaped_payload_preview_only
  - theorem_result_object_preview_default:proof_object_schema_reserve_keep
  - theorem_result_object_preview_default:final_public_contract_later
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:theorem_result_object_preview
  - compare_floor:current_l2.theorem_result_object_preview_actualization
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.theorem_review_unit_transport_actual_adoption
  - compare_floor:current_l2.theorem_binding_preflight
  - compare_floor:current_l2.theorem_result_object_preview_actualization
  guard_refs:
  - guard:result_object_preview_actualization_only
  - guard:consumer_shaped_payload_preview_only
  - guard:proof_object_schema_reserve_keep
  - guard:concrete_theorem_prover_brand_later
  kept_later_refs:
  - kept_later:final_public_theorem_result_object
  - kept_later:consumer_shaped_theorem_payload
  - kept_later:concrete_theorem_prover_brand
  - kept_later:proof_object_public_schema
  - kept_later:final_public_verifier_contract
  guard_reason: none
model_check_public_checker_preview:
  status: reached
  preview_kind: helper_local_actualization_manifest
  subject_kind: runtime_try_cut_cluster
  subject_ref: p07-dice-late-join-visible-history
  checker_artifact_preview_refs:
  - model_check_public_checker_preview:p07-dice-late-join-visible-history:consumer_shaped_artifact_preview_only
  - model_check_public_checker_preview:p07-dice-late-join-visible-history:checker_boundary_bundle
  - model_check_public_checker_preview:p07-dice-late-join-visible-history:row_local_property_route_bundle
  - model_check_public_checker_preview:p07-dice-late-join-visible-history:repo_local_emitted_artifact_refs
  verifier_handoff_reserve_refs:
  - model_check_verifier_handoff_reserve:public_checker_migration_later
  - model_check_verifier_handoff_reserve:emitted_handoff_artifact_later
  - model_check_verifier_handoff_reserve:runtime_policy_contract_later
  tool_binding_reserve_refs:
  - model_check_tool_binding_reserve:brand_neutral_request_manifest
  - model_check_tool_binding_reserve:concrete_tool_brand_later
  - model_check_tool_binding_reserve:runtime_policy_contract_later
  actual_adoption_default_refs:
  - model_check_public_checker_preview_default:consumer_shaped_artifact_preview_only
  - model_check_public_checker_preview_default:verifier_handoff_reserve_keep
  - model_check_public_checker_preview_default:brand_neutral_tool_binding_reserve_keep
  - model_check_public_checker_preview_default:runtime_policy_contract_later
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:model_check_public_checker_preview
  - compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.model_check.row_local_property_actual_adoption
  - compare_floor:current_l2.model_check.second_line_concretization
  - compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization
  guard_refs:
  - guard:public_checker_artifact_preview_actualization_only
  - guard:verifier_handoff_reserve_keep
  - guard:brand_neutral_tool_binding_reserve_keep
  - guard:runtime_policy_contract_later
  kept_later_refs:
  - kept_later:first_settled_property_language
  - kept_later:concrete_model_check_tool_brand
  - kept_later:final_public_checker_artifact
  - kept_later:actual_public_checker_migration
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:production_checker_runtime_policy_contract
  - kept_later:final_public_verifier_contract
  guard_reason: none
theorem_final_public_contract_reopen_threshold:
  status: reached
  threshold_kind: helper_local_reopen_threshold_manifest
  subject_kind: runtime_try_cut_cluster
  subject_ref: p07-dice-late-join-visible-history
  result_object_route_refs:
  - theorem_result_object_actual_route:p07-dice-late-join-visible-history:review_unit_transport_first
  - theorem_result_object_actual_route:p07-dice-late-join-visible-history:notebook_consumer_object_first
  - theorem_result_object_actual_route:p07-dice-late-join-visible-history:repo_local_emitted_artifact_refs_first
  - theorem_result_object_actual_route:p07-dice-late-join-visible-history:consumer_shaped_payload_preview_keep
  - theorem_result_object_actual_route:p07-dice-late-join-visible-history:proof_object_schema_prover_brand_later
  payload_preview_keep_refs:
  - theorem_result_object_payload_preview_keep:p07-dice-late-join-visible-history:notebook_consumer_first
  - theorem_result_object_payload_preview_keep:p07-dice-late-join-visible-history:consumer_shaped_payload_preview_only
  - theorem_result_object_payload_preview_keep:p07-dice-late-join-visible-history:payload_public_contract_later
  proof_object_schema_candidate_refs:
  - theorem_proof_object_schema_candidate:p07-dice-late-join-visible-history:result_object_preview_adjacent
  - theorem_proof_object_schema_candidate:p07-dice-late-join-visible-history:refs_only_public_schema_candidate
  - theorem_proof_object_schema_candidate:p07-dice-late-join-visible-history:public_contract_not_adopted
  prover_brand_candidate_refs:
  - theorem_prover_brand_candidate:p07-dice-late-join-visible-history:brand_neutral_preflight_anchor
  - theorem_prover_brand_candidate:p07-dice-late-join-visible-history:adapter_boundary_refs_keep
  - theorem_prover_brand_candidate:p07-dice-late-join-visible-history:concrete_brand_not_adopted
  final_public_contract_reopen_sequence_refs:
  - theorem_final_public_contract_reopen:p07-dice-late-join-visible-history:result_object_and_payload_first
  - theorem_final_public_contract_reopen:p07-dice-late-join-visible-history:prover_brand_and_proof_schema_second
  - theorem_final_public_contract_reopen:p07-dice-late-join-visible-history:final_public_verifier_contract_third
  threshold_default_refs:
  - theorem_final_public_contract_reopen_default:result_object_and_payload_first
  - theorem_final_public_contract_reopen_default:prover_brand_and_proof_schema_second
  - theorem_final_public_contract_reopen_default:final_public_verifier_contract_third
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:theorem_final_public_contract_reopen_threshold
  - compare_floor:current_l2.theorem_final_public_contract_reopen_threshold
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.theorem_review_unit_transport_actual_adoption
  - compare_floor:current_l2.theorem_result_object_preview_actualization
  - compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate
  - compare_floor:current_l2.theorem_result_object_actual_adoption
  - compare_floor:current_l2.theorem_final_public_contract_reopen_threshold
  guard_refs:
  - guard:result_object_and_payload_first
  - guard:prover_brand_and_proof_schema_second
  - guard:final_public_verifier_contract_third
  kept_later_refs:
  - kept_later:final_public_theorem_result_object
  - kept_later:consumer_shaped_theorem_payload
  - kept_later:concrete_theorem_prover_brand
  - kept_later:proof_object_public_schema
  - kept_later:final_public_verifier_contract
  guard_reason: none
model_check_final_public_contract_reopen_threshold:
  status: reached
  threshold_kind: helper_local_reopen_threshold_manifest
  subject_kind: runtime_try_cut_cluster
  subject_ref: p07-dice-late-join-visible-history
  checker_artifact_route_refs:
  - model_check_checker_artifact_actual_route:p07-dice-late-join-visible-history:row_local_property_route_first
  - model_check_checker_artifact_actual_route:p07-dice-late-join-visible-history:checker_boundary_contract_anchor
  - model_check_checker_artifact_actual_route:p07-dice-late-join-visible-history:consumer_shaped_checker_artifact_candidate_only
  - model_check_checker_artifact_actual_route:p07-dice-late-join-visible-history:repo_local_emitted_artifact_refs_first
  - model_check_checker_artifact_actual_route:p07-dice-late-join-visible-history:final_public_checker_artifact_later
  migration_candidate_keep_refs:
  - model_check_checker_artifact_migration_keep:p07-dice-late-join-visible-history:verifier_handoff_candidate_adjacent_keep
  - model_check_checker_artifact_migration_keep:p07-dice-late-join-visible-history:tool_brand_candidate_adjacent_keep
  - model_check_checker_artifact_migration_keep:p07-dice-late-join-visible-history:actual_public_checker_migration_candidate_only
  - model_check_checker_artifact_migration_keep:p07-dice-late-join-visible-history:runtime_policy_contract_later
  verifier_handoff_candidate_refs:
  - model_check_verifier_handoff_candidate:p07-dice-late-join-visible-history:public_checker_preview_adjacent
  - model_check_verifier_handoff_candidate:p07-dice-late-join-visible-history:emitted_handoff_artifact_candidate
  - model_check_verifier_handoff_candidate:p07-dice-late-join-visible-history:runtime_policy_contract_candidate
  tool_brand_candidate_refs:
  - model_check_tool_brand_candidate:p07-dice-late-join-visible-history:brand_neutral_request_manifest_keep
  - model_check_tool_brand_candidate:p07-dice-late-join-visible-history:concrete_tool_brand_candidate
  - model_check_tool_brand_candidate:p07-dice-late-join-visible-history:public_checker_artifact_not_adopted
  final_public_contract_reopen_sequence_refs:
  - model_check_final_public_contract_reopen:p07-dice-late-join-visible-history:property_language_and_tool_brand_first
  - model_check_final_public_contract_reopen:p07-dice-late-join-visible-history:public_checker_artifact_and_migration_second
  - model_check_final_public_contract_reopen:p07-dice-late-join-visible-history:verifier_handoff_and_runtime_policy_contract_third
  - model_check_final_public_contract_reopen:p07-dice-late-join-visible-history:final_public_verifier_contract_fourth
  threshold_default_refs:
  - model_check_final_public_contract_reopen_default:property_language_and_tool_brand_first
  - model_check_final_public_contract_reopen_default:public_checker_artifact_and_migration_second
  - model_check_final_public_contract_reopen_default:verifier_handoff_and_runtime_policy_contract_third
  - model_check_final_public_contract_reopen_default:final_public_verifier_contract_fourth
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:model_check_final_public_contract_reopen_threshold
  - compare_floor:current_l2.model_check.final_public_contract_reopen_threshold
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization
  - compare_floor:current_l2.model_check.property_tool_threshold
  - compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate
  - compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate
  - compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption
  - compare_floor:current_l2.model_check.final_public_contract_reopen_threshold
  guard_refs:
  - guard:property_language_and_tool_brand_first
  - guard:public_checker_artifact_and_migration_second
  - guard:verifier_handoff_and_runtime_policy_contract_third
  - guard:final_public_verifier_contract_fourth
  kept_later_refs:
  - kept_later:first_settled_property_language
  - kept_later:concrete_model_check_tool_brand
  - kept_later:final_public_checker_artifact
  - kept_later:actual_public_checker_migration
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:production_checker_runtime_policy_contract
  - kept_later:final_public_verifier_contract
  guard_reason: none
typed_checker_hint_preview:
  status: guarded_not_reached
  preview_kind: sample_local_helper_preview
  cluster_kind: none
  case_label: none
  typed_reason_family_hint: none
  evidence_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.typed.sample_local_checker_hint_guard_only
  guard_refs:
  - guard:typed_checker_hint_preview_not_reached
  kept_later_refs:
  - kept_later:final_typed_source_principal
  - kept_later:final_finite_index_surface
  - kept_later:final_ifc_syntax
  - kept_later:actual_checker_payload_family
  - kept_later:checker_supported_kind_summary
  - kept_later:final_public_verifier_contract
  guard_reason: current typed checker-hint preview only actualizes the sample-local first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after verification preview reaches runtime try-cut evidence for `p07-dice-late-join-visible-history`
actual_checker_payload_family_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_payload_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_family_kind: none
  source_refs: []
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_checker_payload_family_threshold
  - compare_floor:current_l2.checker.actual_checker_payload_family
  compare_floor_refs:
  - compare_floor:current_l2.checker.actual_checker_payload_family.guard_only
  guard_refs:
  - guard:actual_checker_payload_family_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_family
  - kept_later:checker_payload_row_detail
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload family threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after typed checker-hint preview reaches the checker-adjacent helper floor for `p07-dice-late-join-visible-history`
actual_checker_payload_row_family_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_family_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_family_ref: none
  row_family_kind: none
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_checker_payload_row_family_threshold
  - compare_floor:current_l2.checker.checker_payload_row_family
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_family.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_family_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_detail
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-family threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload family threshold reaches the checker-adjacent helper floor for `p07-dice-late-join-visible-history`
actual_checker_payload_row_detail_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_detail_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_row_family_ref: none
  row_source_ref: none
  row_reason_kind: none
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_checker_payload_row_detail_threshold
  - compare_floor:current_l2.checker.checker_payload_row_detail
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_detail.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_detail_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-detail threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-family threshold reaches the checker-adjacent helper floor for `p07-dice-late-join-visible-history`
actual_checker_payload_row_body_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_body_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_row_family_ref: none
  row_source_ref: none
  row_reason_kind: none
  row_body: none
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_checker_payload_row_body_threshold
  - compare_floor:current_l2.checker.checker_payload_row_body
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_body.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_body_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-body threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-detail threshold reaches the checker-adjacent helper floor for `p07-dice-late-join-visible-history`
actual_checker_payload_supported_kind_summary_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_supported_kind_summary_threshold_manifest
  payload_row_family_ref: none
  supported_kind_scope: none
  supported_kind_refs: []
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_checker_payload_supported_kind_summary_threshold
  - compare_floor:current_l2.checker.checker_payload_supported_kind_summary
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_supported_kind_summary.guard_only
  guard_refs:
  - guard:actual_checker_payload_supported_kind_summary_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload supported-kind summary threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-body threshold reaches the checker-adjacent helper floor for `p07-dice-late-join-visible-history`
actual_checker_payload_public_schema_sketch_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_payload_schema_sketch_threshold_manifest
  actual_checker_payload_family_ref: none
  checker_payload_row_family_ref: none
  checker_payload_row_detail_ref: none
  checker_payload_row_body_ref: none
  checker_payload_supported_kind_summary_ref: none
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_checker_payload_public_schema_sketch_threshold
  - compare_floor:current_l2.checker.public_checker_payload_schema
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_payload_schema.guard_only
  guard_refs:
  - guard:actual_checker_payload_public_schema_sketch_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_api
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload public-schema sketch threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload supported-kind summary threshold reaches the checker-adjacent helper floor for `p07-dice-late-join-visible-history`
actual_public_checker_api_sketch_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_api_sketch_threshold_manifest
  checker_subject: none
  public_checker_payload_schema_ref: none
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_public_checker_api_sketch_threshold
  - compare_floor:current_l2.checker.public_checker_api
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_api.guard_only
  guard_refs:
  - guard:actual_public_checker_api_sketch_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_entry_criteria
  - kept_later:public_checker_command_surface
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker API sketch threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload public-schema sketch threshold reaches the checker-adjacent helper floor for `p07-dice-late-join-visible-history`
actual_public_checker_entry_criteria_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_entry_criteria_threshold_manifest
  public_checker_api_ref: none
  entry_criteria_refs: []
  family_facade_support_ref: none
  family_facade_script_refs: []
  smoke_command_refs: []
  next_comparison_target_ref: none
  deferred_boundary_refs: []
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_public_checker_entry_criteria_threshold
  - compare_floor:current_l2.checker.public_checker_entry_criteria
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_entry_criteria.guard_only
  guard_refs:
  - guard:actual_public_checker_entry_criteria_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_command_surface
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker entry-criteria threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker API sketch threshold reaches the checker-adjacent helper floor for `p07-dice-late-join-visible-history`
actual_public_checker_command_surface_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_command_surface_threshold_manifest
  command_surface_kind: none
  family_facade_command_refs: []
  public_checker_api_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_public_checker_command_surface_threshold
  - compare_floor:current_l2.checker.public_checker_command_surface
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_command_surface.guard_only
  guard_refs:
  - guard:actual_public_checker_command_surface_threshold_not_reached
  kept_later_refs:
  - kept_later:detached_loop_smoke_wrapper
  - kept_later:generic_shared_public_checker_entry
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker command-surface threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker entry-criteria threshold reaches the checker-adjacent helper floor for `p07-dice-late-join-visible-history`
actual_shared_output_contract_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_shared_output_contract_threshold_manifest
  output_contract_kind: none
  checker_cluster_name: none
  checker_status: none
  public_checker_payload_schema_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_shared_output_contract_threshold
  - compare_floor:current_l2.checker.shared_output_contract
  compare_floor_refs:
  - compare_floor:current_l2.checker.shared_output_contract.guard_only
  guard_refs:
  - guard:actual_shared_output_contract_threshold_not_reached
  kept_later_refs:
  - kept_later:detached_loop_wrapper_paths
  - kept_later:fixture_and_actual_rows_textual_rendering
  - kept_later:generic_shared_public_checker_entry
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual shared output contract threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker command-surface threshold reaches the checker-adjacent helper floor for `p07-dice-late-join-visible-history`
actual_public_checker_boundary_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_boundary_threshold_manifest
  boundary_kind: none
  public_checker_command_surface_ref: none
  shared_output_contract_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_public_checker_boundary_threshold
  - compare_floor:current_l2.checker.public_checker_boundary
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_boundary.guard_only
  guard_refs:
  - guard:actual_public_checker_boundary_threshold_not_reached
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:query_token_and_checker_subject_public_naming
  - kept_later:generic_shared_public_checker_entry
  - kept_later:detached_loop_wrapper_path_line
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker boundary threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual shared output contract threshold reaches the checker-adjacent helper floor for `p07-dice-late-join-visible-history`
actual_verifier_handoff_surface_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_verifier_handoff_surface_threshold_manifest
  handoff_surface_kind: none
  public_checker_boundary_ref: none
  proof_obligation_matrix_ref: none
  handoff_artifact_mode: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_verifier_handoff_surface_threshold
  - compare_floor:current_l2.checker.verifier_handoff_surface
  compare_floor_refs:
  - compare_floor:current_l2.checker.verifier_handoff_surface.guard_only
  guard_refs:
  - guard:actual_verifier_handoff_surface_threshold_not_reached
  kept_later_refs:
  - kept_later:subject_rows
  - kept_later:theorem_protocol_runtime_dedicated_handoff_artifact_family
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:tool_specific_schema_and_actual_emitter_policy
  - kept_later:final_parser_grammar
  - kept_later:query_token_and_shared_generic_entry
  - kept_later:final_public_verifier_contract
  guard_reason: current actual verifier handoff surface threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker boundary threshold reaches the checker-adjacent helper floor for `p07-dice-late-join-visible-history`
actual_minimal_parser_subset_freeze_threshold:
  status: guarded_not_reached
  threshold_kind: parser_front_minimal_parser_subset_freeze_threshold_manifest
  freeze_kind: none
  accepted_cluster_refs: []
  reject_cluster_refs: []
  retention_floor_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_minimal_parser_subset_freeze_threshold
  - compare_floor:current_l2.parser.minimal_parser_subset_freeze
  compare_floor_refs:
  - compare_floor:current_l2.parser.minimal_parser_subset_freeze.guard_only
  guard_refs:
  - guard:actual_minimal_parser_subset_freeze_threshold_not_reached
  kept_later_refs:
  - kept_later:stage3_admit_slot_branch
  - kept_later:stage3_request_clause_branch
  - kept_later:stage3_predicate_fragment_branch
  - kept_later:public_parser_api
  - kept_later:final_parser_grammar
  - kept_later:parser_to_checker_reconnect_freeze
  - kept_later:final_public_parser_checker_api
  - kept_later:final_public_verifier_contract
  guard_reason: current actual minimal parser subset freeze threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual verifier handoff surface threshold reaches the helper-local docs-only bridge floor for `p07-dice-late-join-visible-history`
actual_parser_to_checker_reconnect_freeze_threshold:
  status: guarded_not_reached
  threshold_kind: parser_checker_bridge_reconnect_freeze_threshold_manifest
  reconnect_kind: none
  parser_subset_freeze_ref: none
  checker_floor_refs: []
  retained_helper_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_parser_to_checker_reconnect_freeze_threshold
  - compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze
  compare_floor_refs:
  - compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze.guard_only
  guard_refs:
  - guard:actual_parser_to_checker_reconnect_freeze_threshold_not_reached
  kept_later_refs:
  - kept_later:stage3_request_predicate_reconnect_line
  - kept_later:stage1_direct_target_mismatch_redesign_line
  - kept_later:runtime_contrast_e21_e22_line
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_api
  - kept_later:actual_external_verifier_schema
  - kept_later:final_public_verifier_contract
  guard_reason: current actual parser-to-checker reconnect freeze threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual minimal parser subset freeze threshold reaches the stage1+stage2 parser floor for `p07-dice-late-join-visible-history`
actual_phase1_semantics_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase1_semantics_closeout_threshold_manifest
  closeout_kind: none
  core_semantics_refs: []
  invariant_bridge_refs: []
  notation_status_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_phase1_semantics_closeout_threshold
  - compare_floor:current_l2.closeout.phase1_semantics_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase1_semantics_closeout.guard_only
  guard_refs:
  - guard:actual_phase1_semantics_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_reserved_keyword_and_punctuation
  - kept_later:final_type_system
  - kept_later:actual_external_verifier_schema
  - kept_later:actual_emitted_verifier_artifact
  - kept_later:final_public_verifier_contract
  guard_reason: current actual phase1 semantics closeout threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual parser-to-checker reconnect freeze threshold reaches the checker-floor bridge for `p07-dice-late-join-visible-history`
actual_phase2_parser_free_poc_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase2_parser_free_poc_closeout_threshold_manifest
  closeout_kind: none
  compile_gate_refs: []
  helper_boundary_refs: []
  detached_loop_policy_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_phase2_parser_free_poc_closeout_threshold
  - compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout.guard_only
  guard_refs:
  - guard:actual_phase2_parser_free_poc_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:reference_update_bless_workflow
  - kept_later:final_retention_path_policy
  - kept_later:public_exporter_api
  - kept_later:production_host_interface
  guard_reason: current actual phase2 parser-free PoC closeout threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual phase1 semantics closeout threshold reaches the semantics closeout floor for `p07-dice-late-join-visible-history`
actual_phase4_shared_space_self_driven_closeout_threshold:
  status: reached
  threshold_kind: phase4_shared_space_self_driven_closeout_threshold_manifest
  closeout_kind: shared_space_practical_boundary_checkpoint
  current_package_refs:
  - authoritative_room_baseline_ref
  - working_subset_catalog_ref
  - minimal_authority_witness_core_ref
  - authoritative_delegated_provider_cut_ref
  - control_plane_threshold_ref
  user_spec_required_catalog_refs:
  - final_activation_overlay_catalog
  - final_authority_auth_identity_admission_catalog
  - final_consistency_fairness_catalog
  retained_later_refs:
  - append_friendly_optional_provider_attestation
  - control_plane_separated_carrier_actualization
  - distributed_fairness_protocol
  - final_operational_realization
  next_comparison_target_ref: phase5_proof_protocol_runtime_policy_handoff_closeout_comparison
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  guard_refs:
  - guard:phase4_shared_space_self_driven_closeout_threshold_only
  - guard:phase5_proof_protocol_runtime_policy_handoff_closeout_comparison_next
  - guard:user_spec_required_final_catalog_later
  - guard:distributed_fairness_protocol_later
  kept_later_refs:
  - kept_later:final_public_witness_provider_artifact_contract
  - kept_later:exhaustive_shared_space_catalog
  - kept_later:control_plane_separated_carrier_actualization
  - kept_later:distributed_fairness_protocol
  - kept_later:final_operational_realization
  guard_reason: none
actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold:
  status: reached
  threshold_kind: phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_manifest
  closeout_kind: proof_protocol_runtime_policy_handoff_stop_line
  verifier_handoff_surface_ref: minimal_verifier_handoff_surface
  theorem_retained_bridge_stop_ref: retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  boundary_inventory_ref: small_decidable_core_boundary_inventory
  retained_later_refs:
  - actual_subject_row_materialization
  - boundary_specific_handoff_artifact_family
  - actual_emitted_verifier_artifact
  - concrete_tool_binding
  - public_checker_migration
  - low_level_memory_order_family
  next_comparison_target_ref: phase6_actual_parser_ast_carrier_first_tranche_comparison
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  guard_refs:
  - guard:phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_only
  - guard:phase6_actual_parser_ast_carrier_first_tranche_comparison_next
  - guard:actual_subject_row_and_artifact_family_later
  - guard:tool_binding_public_checker_migration_and_low_level_memory_order_later
  kept_later_refs:
  - kept_later:actual_subject_row_materialization
  - kept_later:boundary_specific_handoff_artifact_family
  - kept_later:actual_emitted_verifier_artifact
  - kept_later:concrete_tool_binding
  - kept_later:public_checker_migration
  - kept_later:low_level_memory_order_family
  guard_reason: none
actual_phase6_actual_parser_ast_carrier_first_tranche_threshold:
  status: reached
  threshold_kind: phase6_actual_parser_ast_carrier_first_tranche_threshold_manifest
  carrier_kind: current_l2_nonproduction_parser_carrier
  accepted_surface_refs:
  - stage1_option_decl_chain_surface
  - stage2_try_fallback_structural_surface
  code_anchor_refs:
  - mir_ast_current_l2_module
  - stage1_stage2_parser_spike_tests
  retained_later_refs:
  - stage3_admit_slot_surface
  - stage3_request_clause_suite
  - stage3_predicate_fragment
  - perform_head_final_public_api
  - span_rich_diagnostics
  - final_grammar
  next_comparison_target_ref: phase6_actual_checker_runtime_skeleton_first_tranche_comparison
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  guard_refs:
  - guard:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_required
  - guard:phase6_actual_checker_runtime_skeleton_first_tranche_comparison_next
  kept_later_refs:
  - stage3_admit_slot_surface
  - stage3_request_clause_suite
  - stage3_predicate_fragment
  - perform_head_final_public_api
  - span_rich_diagnostics
  - final_grammar
  guard_reason: none
actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold:
  status: reached
  threshold_kind: phase6_actual_checker_runtime_skeleton_first_tranche_threshold_manifest
  skeleton_kind: current_l2_nonproduction_checker_runtime_skeleton
  semantic_entry_refs:
  - static_gate_program_detailed
  - direct_style_evaluator_from_program
  - fixture_host_stub_run_program
  runtime_bridge_refs:
  - mir_runtime_current_l2_module
  - current_l2_runtime_skeleton_report
  parser_bridge_contract_refs:
  - stage1_reconnect_clusters
  - stage2_try_rollback_structural_summary
  - parser_bridge_consistency_guard
  retained_later_refs:
  - parser_to_program_lowering
  - stage3_request_predicate_reconnect
  - richer_host_interface
  - final_public_runtime_checker_api
  - formal_hook_concrete_tool_binding
  next_comparison_target_ref: phase6_compile_ready_verification_and_formal_hook_comparison
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  guard_refs:
  - guard:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_required
  - guard:phase6_compile_ready_verification_and_formal_hook_comparison_next
  kept_later_refs:
  - parser_to_program_lowering
  - stage3_request_predicate_reconnect
  - richer_host_interface
  - final_public_runtime_checker_api
  - formal_hook_concrete_tool_binding
  guard_reason: none
actual_phase6_compile_ready_verification_and_formal_hook_threshold:
  status: reached
  threshold_kind: phase6_compile_ready_verification_and_formal_hook_threshold_manifest
  verification_gate_refs:
  - cargo_test_mir_ast
  - cargo_test_mir_runtime
  - cargo_test_mir_semantics_current_l2_minimal_interpreter
  - cargo_test_mir_semantics_current_l2_static_gate_support
  - cargo_test_mir_semantics_current_l2_detached_bundle_support
  - cargo_test_mir_semantics_current_l2_formal_hook_support
  - python_unittest_current_l2_static_and_detached_loop
  smoke_gate_refs:
  - smoke_formal_hook_static
  - smoke_formal_hook_runtime
  formal_hook_artifact_kind_ref: current_l2_tool_neutral_formal_hook
  formal_hook_subject_kind_refs:
  - fixture_static_cluster
  - runtime_try_cut_cluster
  formal_hook_contract_row_core_refs:
  - obligation_kind
  - evidence_refs
  formal_hook_evidence_ref_family_refs:
  - ref_kind
  - ref_id
  formal_hook_obligation_kind_refs:
  - canonical_normalization_law
  - no_re_promotion
  - rollback_cut_non_interference
  source_artifact_refs:
  - detached_static_gate_artifact
  - detached_bundle_artifact
  validation_refs:
  - input_schema_version_guard
  - input_artifact_kind_guard
  retained_later_refs:
  - concrete_theorem_tool_binding
  - concrete_model_check_tool_binding
  - parser_second_tranche_widen
  - final_public_surface
  next_comparison_target_ref: phase6_next_reopen_sequencing_comparison
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - source:phase6_compile_ready_verification_and_formal_hook_ready_sketch
  - source:phase6_compile_ready_verification_and_formal_hook_minimum
  - code_anchor:current_l2_emit_formal_hook_example
  - code_anchor:current_l2_formal_hook_support_tests
  - code_anchor:current_l2_detached_loop_smoke_family
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  guard_refs:
  - guard:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_required
  - guard:phase6_next_reopen_sequencing_comparison_next
  kept_later_refs:
  - concrete_theorem_tool_binding
  - concrete_model_check_tool_binding
  - parser_second_tranche_widen
  - final_public_surface
  guard_reason: none
actual_phase6_next_reopen_sequencing_threshold:
  status: reached
  threshold_kind: phase6_next_reopen_sequencing_threshold_manifest
  sequencing_kind_ref: phase6_checkpoint_postclose_next_reopen
  fixed_entry_criteria_refs:
  - phase6_parser_first_tranche
  - phase6_checker_runtime_first_tranche
  - phase6_compile_ready_formal_hook
  selected_first_reopen_ref: phase6_parser_second_tranche_attached_slot_and_predicate_fragment_route
  deferred_reopen_refs:
  - theorem_first_concrete_tool_binding_route
  - concrete_model_check_tool_binding
  minimum_guard_refs:
  - keep_tool_neutral_formal_hook_as_entry_criteria
  - avoid_request_head_clause_suite_richer_diagnostics_bulk_widen
  - keep_model_check_line_reserve_only
  next_comparison_target_ref: phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - source:phase6_compile_ready_verification_and_formal_hook_ready_sketch
  - source:phase6_compile_ready_verification_and_formal_hook_minimum
  - code_anchor:current_l2_emit_formal_hook_example
  - code_anchor:current_l2_formal_hook_support_tests
  - code_anchor:current_l2_detached_loop_smoke_family
  - helper_preview:actual_phase6_next_reopen_sequencing_threshold
  - source:phase6_next_reopen_sequencing_current_first_choice
  - source:phase6_next_reopen_sequencing_minimum
  - source:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  - compare_floor:current_l2.closeout.phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package
  guard_refs:
  - guard:actual_phase6_compile_ready_verification_and_formal_hook_threshold_required
  - guard:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison_next
  kept_later_refs:
  - request_clause_suite_bulk_widen
  - perform_head_final_public_api
  - concrete_theorem_tool_binding
  - concrete_model_check_tool_binding
  - final_public_surface
  guard_reason: none
actual_phase6_reserve_formal_tool_binding_inventory_threshold:
  status: reached
  threshold_kind: phase6_reserve_formal_tool_binding_inventory_threshold_manifest
  inventory_kind: phase6_postclose_formal_reserve_inventory
  fixed_entry_criteria_refs:
  - phase5_handoff_closeout
  - phase6_compile_ready_formal_hook
  - phase6_parser_second_tranche_first_package
  first_reserve_ref: theorem_first_notebook_pressure_concrete_tool_binding_route
  second_reserve_ref: model_check_protocol_property_concrete_tool_binding_route
  minimum_guard_refs:
  - keep_tool_neutral_formal_hook_as_current_entry_criteria
  - keep_parser_followup_package_as_current_mainline
  - avoid_dual_tool_choice_single_package
  - avoid_public_checker_runtime_surface_backpressure
  next_comparison_target_ref: phase6_parser_side_follow_up_package_sequencing_comparison
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - source:phase6_compile_ready_verification_and_formal_hook_ready_sketch
  - source:phase6_compile_ready_verification_and_formal_hook_minimum
  - code_anchor:current_l2_emit_formal_hook_example
  - code_anchor:current_l2_formal_hook_support_tests
  - code_anchor:current_l2_detached_loop_smoke_family
  - helper_preview:actual_phase6_next_reopen_sequencing_threshold
  - source:phase6_next_reopen_sequencing_current_first_choice
  - source:phase6_next_reopen_sequencing_minimum
  - source:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison
  - helper_preview:actual_phase6_reserve_formal_tool_binding_inventory_threshold
  - source:phase6_reserve_formal_tool_binding_inventory_current_first_choice
  - source:phase6_reserve_formal_tool_binding_inventory_minimum
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  - compare_floor:current_l2.closeout.phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package
  - compare_floor:current_l2.closeout.phase6_reserve_formal_tool_binding_inventory
  guard_refs:
  - guard:actual_phase6_next_reopen_sequencing_threshold_required
  - guard:phase6_parser_side_follow_up_package_sequencing_comparison_next
  kept_later_refs:
  - concrete_theorem_tool_name
  - concrete_model_check_tool_name
  - actual_ci_artifact_retention_policy
  - parser_side_followup_package_selection
  - final_public_parser_checker_runtime_surface
  guard_reason: none
actual_phase6_parser_side_followup_package_sequencing_threshold:
  status: reached
  threshold_kind: phase6_parser_side_followup_package_sequencing_threshold_manifest
  sequencing_kind: phase6_parser_followup_next_package_selection
  fixed_entry_criteria_refs:
  - phase6_parser_second_tranche_first_package
  - phase6_reserve_formal_tool_binding_inventory
  - stage3_multiline_attachment_first_tranche_actualization
  selected_next_package_ref: phase6_parser_second_tranche_shared_single_attachment_frame_first_package
  deferred_reopen_refs:
  - phase6_request_clause_suite_publicization
  - phase6_perform_head_final_public_parser_api
  - phase6_span_rich_diagnostics
  - phase6_final_grammar
  minimum_guard_refs:
  - reuse_existing_stage3_minimal_predicate_fragment_surface
  - keep_request_head_and_suite_ordering_out_of_scope
  - keep_source_sample_path_after_parser_followup_cut
  next_comparison_target_ref: phase6_parser_second_tranche_shared_single_attachment_frame_first_package_comparison
  evidence_refs:
  - sample:p07-dice-late-join-visible-history
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - source:phase6_compile_ready_verification_and_formal_hook_ready_sketch
  - source:phase6_compile_ready_verification_and_formal_hook_minimum
  - code_anchor:current_l2_emit_formal_hook_example
  - code_anchor:current_l2_formal_hook_support_tests
  - code_anchor:current_l2_detached_loop_smoke_family
  - helper_preview:actual_phase6_next_reopen_sequencing_threshold
  - source:phase6_next_reopen_sequencing_current_first_choice
  - source:phase6_next_reopen_sequencing_minimum
  - source:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison
  - helper_preview:actual_phase6_reserve_formal_tool_binding_inventory_threshold
  - source:phase6_reserve_formal_tool_binding_inventory_current_first_choice
  - source:phase6_reserve_formal_tool_binding_inventory_minimum
  - helper_preview:actual_phase6_parser_side_followup_package_sequencing_threshold
  - source:phase6_parser_side_followup_package_sequencing_current_first_choice
  - source:phase6_parser_side_followup_package_sequencing_minimum
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  - compare_floor:current_l2.closeout.phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package
  - compare_floor:current_l2.closeout.phase6_reserve_formal_tool_binding_inventory
  - compare_floor:current_l2.closeout.phase6_parser_side_followup_package_sequencing
  guard_refs:
  - guard:actual_phase6_reserve_formal_tool_binding_inventory_threshold_required
  - guard:phase6_parser_second_tranche_shared_single_attachment_frame_first_package_comparison_next
  kept_later_refs:
  - request_clause_suite_publicization
  - perform_head_final_public_parser_api
  - span_rich_diagnostics
  - source_sample_corpus_scope
  - final_public_parser_checker_runtime_surface
  guard_reason: none
non_admissible_metadata: []
```

### 出力の読み方

まず最初に見るべき行は次の 6 つである。

- `static_gate: valid`
  この sample は静的に受理された。少なくとも current rule 上、形と宣言は足りている。
- `entered_evaluation: true`
  static stop せず、runtime evaluation に入った。
- `terminal_outcome: success`
  runtime の終端結果が success である。
- `steps_executed: 9`
  実際に 0 より大きい step が進んでいる。静的に止まった sample ではここが 0 になる。
- `debug_outputs:` の `publish_roll_result: player_a -> visible` と `handoff_dice_authority: player_a -> player_b`
  publish と handoff が実際に runtime 側の debug output に現れている。
- `late_join_view: player_c sees result+owner history`
  late joiner が結果と owner history を見ていることが読める。これがこの sample の代表的成功点である。

さらに `verification_preview` では、

- `formal_hook_status: reached`
- `subject_kind: runtime_try_cut_cluster`

となっている。これは、「runtime まで進んだ cluster を formal hook preview の subject として扱えている」という意味である。ここで `reached` は theorem や model-check が完了したという意味ではなく、**helper-local な formal hook 入口に到達した**という意味で読む。

## 2. stale reconnect を `p08` で見る

`p08` は、「古い reconnect message を、そのまま成功扱いにして room に混ぜない」例である。current line の読みは **fail then refresh** である。

### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  run-source-sample \
  samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt \
  --format pretty
```

このコマンドが確認していること:

- stale reconnect をそのまま飲み込まず、明示的 failure に落とすか
- 失敗後に refresh へ回す fallback が current line として動いているか
- runtime cluster として formal hook preview まで到達するか

### コード全文

```text
# stale reconnect message は rollback で落とし、owner snapshot の refresh へ回す current L2 prototype。
place root {
  place room {
    place dice_authority {
      try {
        perform apply_stale_reconnect_message on dice_state
          require owner_snapshot_matches(player_a)
          ensure owner_is(player_a)
      } fallback {
        perform refresh_owner_snapshot on dice_state
          require read
          ensure owner_is(player_b)
      }
    }
  }
}
```

### 行ごとの解説

1. `# stale reconnect message は rollback で落とし、owner snapshot の refresh へ回す current L2 prototype。`
   この sample の要点を先に書いている。重要なのは「stale reconnect を成功扱いしない」ことと、「失敗後に refresh へ回す」ことの 2 点である。
2. `place root {`
   外側の root place である。
3. `place room {`
   room の place である。
4. `place dice_authority {`
   reconnect と owner snapshot refresh を扱う authority place である。
5. `try {`
   まず試してみる本体ブロックである。ここで stale reconnect message の適用を試す。
6. `perform apply_stale_reconnect_message on dice_state`
   古い reconnect message を `dice_state` に適用しようとする操作である。
7. `require owner_snapshot_matches(player_a)`
   reconnect message が、今の owner snapshot と一致していることを要求している。ここが stale なら、この前提は意味上ずれる。
8. `ensure owner_is(player_a)`
   適用成功後の約束である。今回の sample では、ここが成功まで進まず explicit failure に落ちる。
9. `} fallback {`
   `try` が失敗した場合に fallback へ移る。この sample では、これが current line の要となる。
10. `perform refresh_owner_snapshot on dice_state`
    stale reconnect の代わりに、owner snapshot を refresh する操作である。
11. `require read`
    refresh を行うには read が必要だと宣言している。
12. `ensure owner_is(player_b)`
    refresh 後には owner が `player_b` であることを確認する。つまり、「古い reconnect を無理に通す」代わりに、「今の owner snapshot へ追いつく」処理を選んでいる。
13. `}`
    fallback ブロックを閉じる。
14. `}`
    `dice_authority` place を閉じる。
15. `}`
    `room` place を閉じる。
16. `}`
    `root` place を閉じる。

### `stale reconnect` をなぜ失敗に落とすのか

具体例を一つ挙げる。

- `player_a` が owner だった時点の reconnect packet を、あとから `player_b` owner の room にそのまま混ぜる
- すると「古い ownership にもとづく再接続」を、現在の room history に混入させる危険がある

current line は、これを silent merge にしない。いったん失敗として落とし、その後に refresh へ回すことで、「今の room の事実」と再接続を合わせ直す。

### 実行例全文

```text
shell: mir-current-l2
command: run-source-sample
sample: p08-dice-stale-reconnect-refresh
sample_path: /home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt
host_plan_path: /home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.host-plan.json
static_gate: valid
stage1_reconnect_clusters: not_available
stage2_try_rollback_verdict: no_findings
stage2_try_rollback_findings: []
entered_evaluation: true
terminal_outcome: success
steps_executed: 7
events:
- perform-failure
- rollback
- perform-success
debug_outputs:
- reconnect_debug_text_output:
  - refresh_owner_snapshot: stale reconnect redirected
verification_preview:
  formal_hook_status: reached
  subject_kind: runtime_try_cut_cluster
  subject_ref: p08-dice-stale-reconnect-refresh
  proof_notebook_review_unit_obligations:
  - rollback_cut_non_interference
  model_check_concrete_carrier_obligations:
  - rollback_cut_non_interference
  guard_reason: none
artifact_preview:
  proof_notebook_review_units:
  - obligation_kind: rollback_cut_non_interference
    goal_text: Review that rollback / atomic-cut evidence does not interfere with surviving runtime behavior for `p08-dice-stale-reconnect-refresh`.
    evidence_refs:
    - fixture:p08-dice-stale-reconnect-refresh
    - runtime_cluster:p08-dice-stale-reconnect-refresh
  model_check_concrete_carriers:
  - obligation_kind: rollback_cut_non_interference
    evidence_refs:
    - fixture:p08-dice-stale-reconnect-refresh
    - runtime_cluster:p08-dice-stale-reconnect-refresh
surface_preview:
  minimal_companion:
    status: reached
    guard_reason: none
    lines:
    - profile authoritative_room_default
    - activation authority-ack
    - authority single_room_authority
    - consistency authoritative_serial_transition
    - rng authority_rng
    - rollback stale_reconnect
    - refresh refresh_owner_snapshot@dice_state
    - replay stale_incompatible_replay_invalidated
    compare_floor_refs:
    - compare_floor:current_l2.authoritative_room.vertical_slice
    - compare_floor:current_l2.experimental_order_handoff_surface
    guard_refs:
    - guard:semantic_honesty_first
    - guard:helper_local_companion_only
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
  stage_block_secondary:
    status: reached
    guard_reason: none
    lines:
    - transition reconnect_refresh(dice_owner = player_a)
    - stage rollback:
    -   rollback stale_reconnect
    - stage refresh:
    -   refresh refresh_owner_snapshot@dice_state
    -     after rollback(stale_reconnect)
    - stage replay:
    -   invalidate stale_incompatible_replay@dice_state
    -     after refresh(refresh_owner_snapshot@dice_state)
    compare_floor_refs:
    - compare_floor:current_l2.experimental_order_handoff_surface
    - compare_floor:current_l2.experimental_stage_block_surface
    guard_refs:
    - guard:stage_block_secondary_candidate
    - guard:helper_local_companion_only
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:authoritative_room_serial_scope_sugar
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
  serial_scope_reserve:
    status: reached
    guard_reason: none
    lines:
    - serial on dice_authority {
    -   rollback stale_reconnect
    -   refresh refresh_owner_snapshot@dice_state
    -   invalidate stale_incompatible_replay@dice_state
    - }
    compare_floor_refs:
    - compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption
    - compare_floor:current_l2.order_handoff.serial_scope_reserve_surface
    guard_refs:
    - guard:serial_scope_room_specific_reserve
    - guard:principal_edge_row_surface_unchanged
    - guard:helper_local_surface_only
    - guard:final_source_surface_handoff_wording_later
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:serial_scope_public_promotion
    - kept_later:serial_scope_beyond_authoritative_room
    - kept_later:final_source_surface_handoff_wording
    - kept_later:final_emitted_artifact_schema
    - kept_later:final_emitted_handoff_contract
    - kept_later:final_public_witness_schema
    - kept_later:final_public_provider_receipt_schema
    - kept_later:combined_provider_witness_public_contract
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
authoritative_room_first_scenario_actual_adoption:
  status: reached
  adoption_kind: helper_local_authoritative_room_first_scenario_manifest
  profile_axis_refs:
  - profile_axis:activation:authority-ack
  - profile_axis:authority_placement:single_room_authority
  - profile_axis:consistency_mode:authoritative_serial_transition
  - profile_axis:rng_source:authority_rng
  - profile_axis:stale_reconnect:fail_then_refresh
  - profile_axis:replay:stale_incompatible_replay_invalidated
  - profile_axis:fairness_claim:no_distributed_fairness_theorem_required
  relation_refs:
  - relation_family:program_order
  - relation_family:observation_order
  - relation_family:witness_order
  - relation_family:finalization_order
  - relation_family:scoped_happens_before
  authority_handoff_refs:
  - authority_handoff:owner_slot:single_room_authority
  - authority_handoff:stage_family:authoritative_serial_transition
  - authority_handoff:stage_sequence:fail_then_refresh
  - authority_handoff:payload_ref:dice_state
  runtime_evidence_refs:
  - runtime_event:perform-failure
  - runtime_event:rollback
  - runtime_event:perform-success
  - place_record:dice_state:refresh_owner_snapshot@dice_state
  - debug_output:reconnect_debug_text_output:refresh_owner_snapshot: stale reconnect redirected
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference
  - repo_local_emitted_artifact:model_check_concrete_carrier:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference
  reserve_route_refs: []
  negative_static_stop_refs: []
  contrast_refs:
  - contrast_target:append_friendly_notice_room
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:authoritative_room_first_scenario_actual_adoption
  - compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption
  compare_floor_refs:
  - compare_floor:current_l2.authoritative_room.vertical_slice
  - compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption
  guard_refs:
  - guard:authoritative_room_first_default_profile
  - guard:representative_first_scenario_pair
  - guard:no_distributed_fairness_theorem_required
  - guard:minimal_working_subset_only
  - guard:stale_reconnect_fail_then_refresh
  - guard:stale_replay_invalidated_not_merged
  kept_later_refs:
  - kept_later:auditable_authority_witness
  - kept_later:delegated_rng_service
  - kept_later:distributed_randomness_provider
  - kept_later:final_emitted_handoff_contract
  - kept_later:exhaustive_shared_space_catalog
  - kept_later:final_consistency_fairness_catalog
  guard_reason: none
authoritative_room_reserve_strengthening_lane:
  status: reached
  lane_kind: helper_local_reserve_strengthening_lane_manifest
  witness_strengthening_status: guarded_not_reached
  delegated_rng_service_status: guarded_not_reached
  model_check_second_line_status: reached
  witness_strengthening_refs: []
  delegated_rng_service_refs: []
  model_check_second_line_refs:
  - property_preview:row_local:p08-dice-stale-reconnect-refresh:canonical_normalization_law
  - property_preview:row_local:p08-dice-stale-reconnect-refresh:no_re_promotion
  - model_check_request_preflight:p08-dice-stale-reconnect-refresh:row_local_property_preview
  - model_check_request_preflight:p08-dice-stale-reconnect-refresh:small_cluster_semantic_projection
  - public_checker_second_reserve:boundary
  first_line_boundary_refs:
  - first_line_boundary:representative_pair_kept_at_p07_p08
  - first_line_boundary:authoritative_room_default_profile_stays_principal
  - first_line_boundary:authority_rng_default_profile_unchanged
  reserve_boundary_refs:
  - reserve_boundary:auditable_authority_witness_second_strengthening
  - reserve_boundary:delegated_rng_service_second_practical
  - reserve_boundary:model_check_second_line_not_room_profile
  - reserve_boundary:public_checker_contract_kept_later
  - reserve_boundary:witness_provider_combined_public_contract_later
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference
  - repo_local_emitted_artifact:model_check_concrete_carrier:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference
  compare_floor_refs:
  - compare_floor:current_l2.model_check.second_line_concretization
  - compare_floor:current_l2.reserve_strengthening_lane:p08-dice-stale-reconnect-refresh
  guard_refs:
  - guard:first_line_boundary_preserved
  - guard:reserve_components_kept_separate
  - guard:model_check_second_line_not_room_profile
  - guard:witness_provider_public_contract_later
  kept_later_refs:
  - kept_later:combined_witness_provider_public_contract
  - kept_later:final_public_witness_schema
  - kept_later:final_public_provider_receipt_schema
  - kept_later:concrete_model_check_tool_brand
  - kept_later:actual_public_checker_migration
  - kept_later:distributed_fairness_theorem
  guard_reason: none
order_handoff_source_surface_artifact_actual_adoption:
  status: reached
  adoption_kind: helper_local_source_surface_artifact_route_manifest
  profile_axis_refs:
  - profile_axis:activation:authority-ack
  - profile_axis:authority_placement:single_room_authority
  - profile_axis:consistency_mode:authoritative_serial_transition
  - profile_axis:rng_source:authority_rng
  - profile_axis:stale_reconnect:fail_then_refresh
  - profile_axis:replay:stale_incompatible_replay_invalidated
  - profile_axis:fairness_claim:no_distributed_fairness_theorem_required
  principal_surface_lines:
  - rollback stale_reconnect
  - refresh refresh_owner_snapshot@dice_state
  -   after rollback(stale_reconnect)
  - invalidate stale_incompatible_replay@dice_state
  -   after refresh(refresh_owner_snapshot@dice_state)
  secondary_surface_lines:
  - stage rollback:
  -   rollback stale_reconnect
  - stage refresh:
  -   refresh refresh_owner_snapshot@dice_state
  -     after rollback(stale_reconnect)
  - stage replay:
  -   invalidate stale_incompatible_replay@dice_state
  -     after refresh(refresh_owner_snapshot@dice_state)
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference
  - repo_local_emitted_artifact:model_check_concrete_carrier:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference
  source_wording_route_refs:
  - order_handoff_source_wording_actual_route:p08-dice-stale-reconnect-refresh:edge_row_vertical_continuation_principal
  - order_handoff_source_wording_actual_route:p08-dice-stale-reconnect-refresh:readable_high_level_relation_vocabulary
  - order_handoff_source_wording_actual_route:p08-dice-stale-reconnect-refresh:stage_block_secondary_keep
  - order_handoff_source_wording_actual_route:p08-dice-stale-reconnect-refresh:thread_node_same_causal_language
  - order_handoff_source_wording_actual_route:p08-dice-stale-reconnect-refresh:final_source_surface_handoff_wording_later
  emitted_artifact_candidate_keep_refs:
  - order_handoff_emitted_artifact_keep:p08-dice-stale-reconnect-refresh:repo_local_emitted_artifact_refs_first
  - order_handoff_emitted_artifact_keep:p08-dice-stale-reconnect-refresh:source_surface_actual_adoption_adjacent
  - order_handoff_emitted_artifact_keep:p08-dice-stale-reconnect-refresh:witness_provider_contract_adjacent_not_collapsed
  - order_handoff_emitted_artifact_keep:p08-dice-stale-reconnect-refresh:final_emitted_artifact_schema_later
  negative_static_stop_refs: []
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:order_handoff_source_surface_artifact_actual_adoption
  - compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption
  compare_floor_refs:
  - compare_floor:current_l2.order_handoff.surface_actual_adoption
  - compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption
  - compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption
  guard_refs:
  - guard:edge_row_vertical_continuation_principal
  - guard:stage_block_secondary_keep
  - guard:repo_local_emitted_artifact_refs_first
  - guard:final_source_surface_handoff_wording_later
  - guard:final_emitted_artifact_schema_later
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_runtime_api
  - kept_later:final_source_surface_handoff_wording
  - kept_later:final_emitted_artifact_schema
  - kept_later:final_emitted_handoff_contract
  - kept_later:final_public_witness_schema
  - kept_later:authoritative_room_serial_scope_sugar
  - kept_later:low_level_memory_order_source_surface
  - kept_later:final_modal_foundation_adoption
  guard_reason: none
order_handoff_witness_provider_public_seam_compression:
  status: reached
  compression_kind: helper_local_public_seam_manifest
  profile_axis_refs:
  - profile_axis:activation:authority-ack
  - profile_axis:authority_placement:single_room_authority
  - profile_axis:consistency_mode:authoritative_serial_transition
  - profile_axis:rng_source:authority_rng
  - profile_axis:stale_reconnect:fail_then_refresh
  - profile_axis:replay:stale_incompatible_replay_invalidated
  - profile_axis:fairness_claim:no_distributed_fairness_theorem_required
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference
  - repo_local_emitted_artifact:model_check_concrete_carrier:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference
  source_wording_route_refs:
  - order_handoff_source_wording_actual_route:p08-dice-stale-reconnect-refresh:edge_row_vertical_continuation_principal
  - order_handoff_source_wording_actual_route:p08-dice-stale-reconnect-refresh:readable_high_level_relation_vocabulary
  - order_handoff_source_wording_actual_route:p08-dice-stale-reconnect-refresh:stage_block_secondary_keep
  - order_handoff_source_wording_actual_route:p08-dice-stale-reconnect-refresh:thread_node_same_causal_language
  - order_handoff_source_wording_actual_route:p08-dice-stale-reconnect-refresh:final_source_surface_handoff_wording_later
  emitted_artifact_candidate_keep_refs:
  - order_handoff_emitted_artifact_keep:p08-dice-stale-reconnect-refresh:repo_local_emitted_artifact_refs_first
  - order_handoff_emitted_artifact_keep:p08-dice-stale-reconnect-refresh:source_surface_actual_adoption_adjacent
  - order_handoff_emitted_artifact_keep:p08-dice-stale-reconnect-refresh:witness_provider_contract_adjacent_not_collapsed
  - order_handoff_emitted_artifact_keep:p08-dice-stale-reconnect-refresh:final_emitted_artifact_schema_later
  serial_scope_lines:
  - serial on dice_authority {
  -   rollback stale_reconnect
  -   refresh refresh_owner_snapshot@dice_state
  -   invalidate stale_incompatible_replay@dice_state
  - }
  witness_schema_route_refs:
  - witness_provider_schema_route_actual:p08-dice-stale-reconnect-refresh:witness_schema_candidate_keep
  - witness_provider_schema_route_actual:p08-dice-stale-reconnect-refresh:witness_route_first
  - witness_provider_schema_route_actual:p08-dice-stale-reconnect-refresh:repo_local_emitted_artifact_refs_first
  - witness_provider_schema_route_actual:p08-dice-stale-reconnect-refresh:combined_public_contract_later
  provider_receipt_route_refs:
  - witness_provider_schema_route_actual:p08-dice-stale-reconnect-refresh:provider_receipt_candidate_keep
  - witness_provider_schema_route_actual:p08-dice-stale-reconnect-refresh:provider_route_first
  - witness_provider_schema_route_actual:p08-dice-stale-reconnect-refresh:repo_local_emitted_artifact_refs_first
  - witness_provider_schema_route_actual:p08-dice-stale-reconnect-refresh:delegated_provider_attestation_later
  - witness_provider_schema_route_actual:p08-dice-stale-reconnect-refresh:combined_public_contract_later
  combined_public_contract_keep_refs:
  - witness_provider_combined_contract_keep:p08-dice-stale-reconnect-refresh:combined_public_contract_candidate_only
  - witness_provider_combined_contract_keep:p08-dice-stale-reconnect-refresh:final_emitted_handoff_contract_adjacent_keep
  trace_alignment_pair_refs:
  - witness_provider_emitted_contract_trace_alignment_pair:p08-dice-stale-reconnect-refresh:rollback_cut_non_interference
  public_seam_residual_refs:
  - order_handoff_public_seam_residual:p08-dice-stale-reconnect-refresh:final_source_surface_handoff_wording_later
  - order_handoff_public_seam_residual:p08-dice-stale-reconnect-refresh:final_emitted_artifact_schema_later
  - shared_space_public_seam_residual:p08-dice-stale-reconnect-refresh:public_schema_pair_first
  - shared_space_public_seam_residual:p08-dice-stale-reconnect-refresh:delegated_attestation_and_combined_contract_second
  - shared_space_public_seam_residual:p08-dice-stale-reconnect-refresh:final_emitted_handoff_contract_third
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:order_handoff_witness_provider_public_seam_compression
  - compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression
  compare_floor_refs:
  - compare_floor:current_l2.order_handoff.source_wording_route_actual_adoption
  - compare_floor:current_l2.order_handoff.serial_scope_reserve_surface
  - compare_floor:current_l2.witness_provider_emitted_contract_trace_alignment_bridge
  - compare_floor:current_l2.witness_provider_final_public_contract_reopen_threshold
  - compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression
  guard_refs:
  - guard:edge_row_vertical_continuation_principal
  - guard:serial_scope_reserve_surface_only
  - guard:witness_provider_trace_alignment_bridge
  - guard:public_schema_pair_first
  - guard:delegated_attestation_and_combined_contract_second
  - guard:final_source_surface_handoff_wording_later
  - guard:final_emitted_artifact_schema_later
  - guard:final_emitted_handoff_contract_third
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_runtime_api
  - kept_later:final_source_surface_handoff_wording
  - kept_later:final_emitted_artifact_schema
  - kept_later:final_public_witness_schema
  - kept_later:final_public_provider_receipt_schema
  - kept_later:delegated_provider_attestation
  - kept_later:combined_provider_witness_public_contract
  - kept_later:final_emitted_handoff_contract
  - kept_later:authoritative_room_serial_scope_sugar
  - kept_later:low_level_memory_order_source_surface
  - kept_later:final_modal_foundation_adoption
  - kept_later:exhaustive_shared_space_catalog
  guard_reason: none
theorem_result_object_preview:
  status: reached
  preview_kind: helper_local_actualization_manifest
  subject_kind: runtime_try_cut_cluster
  subject_ref: p08-dice-stale-reconnect-refresh
  result_object_route_refs:
  - theorem_result_object_route:p08-dice-stale-reconnect-refresh:notebook_consumer_object_first
  - theorem_result_object_route:p08-dice-stale-reconnect-refresh:review_unit_anchor_bundle
  - theorem_result_object_route:p08-dice-stale-reconnect-refresh:consumer_shaped_payload_preview_only
  - theorem_result_object_route:p08-dice-stale-reconnect-refresh:repo_local_emitted_artifact_refs
  notebook_payload_preview_refs:
  - theorem_result_payload_preview:p08-dice-stale-reconnect-refresh:notebook_consumer_first
  - theorem_result_payload_preview:p08-dice-stale-reconnect-refresh:review_unit_reference_bundle
  - theorem_result_payload_preview:p08-dice-stale-reconnect-refresh:consumer_shaped_payload_preview_only
  - theorem_result_payload_preview:p08-dice-stale-reconnect-refresh:proof_object_public_schema_later
  proof_object_schema_reserve_refs:
  - proof_object_schema_reserve:brand_neutral_binding_keep
  - proof_object_schema_reserve:proof_object_public_schema_later
  - proof_object_schema_reserve:final_public_verifier_contract_later
  actual_adoption_default_refs:
  - theorem_result_object_preview_default:notebook_consumer_object_first
  - theorem_result_object_preview_default:consumer_shaped_payload_preview_only
  - theorem_result_object_preview_default:proof_object_schema_reserve_keep
  - theorem_result_object_preview_default:final_public_contract_later
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:theorem_result_object_preview
  - compare_floor:current_l2.theorem_result_object_preview_actualization
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.theorem_review_unit_transport_actual_adoption
  - compare_floor:current_l2.theorem_binding_preflight
  - compare_floor:current_l2.theorem_result_object_preview_actualization
  guard_refs:
  - guard:result_object_preview_actualization_only
  - guard:consumer_shaped_payload_preview_only
  - guard:proof_object_schema_reserve_keep
  - guard:concrete_theorem_prover_brand_later
  kept_later_refs:
  - kept_later:final_public_theorem_result_object
  - kept_later:consumer_shaped_theorem_payload
  - kept_later:concrete_theorem_prover_brand
  - kept_later:proof_object_public_schema
  - kept_later:final_public_verifier_contract
  guard_reason: none
model_check_public_checker_preview:
  status: guarded_not_reached
  preview_kind: helper_local_actualization_manifest
  subject_kind: runtime_try_cut_cluster
  subject_ref: p08-dice-stale-reconnect-refresh
  checker_artifact_preview_refs: []
  verifier_handoff_reserve_refs: []
  tool_binding_reserve_refs: []
  actual_adoption_default_refs: []
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:model_check_public_checker_preview
  - compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.model_check.public_checker_artifact_preview.guard_only
  guard_refs:
  - guard:model_check_public_checker_artifact_preview_not_reached
  kept_later_refs:
  - kept_later:first_settled_property_language
  - kept_later:concrete_model_check_tool_brand
  - kept_later:final_public_checker_artifact
  - kept_later:actual_public_checker_migration
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:production_checker_runtime_policy_contract
  - kept_later:final_public_verifier_contract
  guard_reason: current model-check public-checker preview only actualizes the representative checker quartet (`e5` / `p06` / `p07` / `p09`) after verification preview reaches the formal-hook route for `p08-dice-stale-reconnect-refresh`
theorem_final_public_contract_reopen_threshold:
  status: reached
  threshold_kind: helper_local_reopen_threshold_manifest
  subject_kind: runtime_try_cut_cluster
  subject_ref: p08-dice-stale-reconnect-refresh
  result_object_route_refs:
  - theorem_result_object_actual_route:p08-dice-stale-reconnect-refresh:review_unit_transport_first
  - theorem_result_object_actual_route:p08-dice-stale-reconnect-refresh:notebook_consumer_object_first
  - theorem_result_object_actual_route:p08-dice-stale-reconnect-refresh:repo_local_emitted_artifact_refs_first
  - theorem_result_object_actual_route:p08-dice-stale-reconnect-refresh:consumer_shaped_payload_preview_keep
  - theorem_result_object_actual_route:p08-dice-stale-reconnect-refresh:proof_object_schema_prover_brand_later
  payload_preview_keep_refs:
  - theorem_result_object_payload_preview_keep:p08-dice-stale-reconnect-refresh:notebook_consumer_first
  - theorem_result_object_payload_preview_keep:p08-dice-stale-reconnect-refresh:consumer_shaped_payload_preview_only
  - theorem_result_object_payload_preview_keep:p08-dice-stale-reconnect-refresh:payload_public_contract_later
  proof_object_schema_candidate_refs:
  - theorem_proof_object_schema_candidate:p08-dice-stale-reconnect-refresh:result_object_preview_adjacent
  - theorem_proof_object_schema_candidate:p08-dice-stale-reconnect-refresh:refs_only_public_schema_candidate
  - theorem_proof_object_schema_candidate:p08-dice-stale-reconnect-refresh:public_contract_not_adopted
  prover_brand_candidate_refs:
  - theorem_prover_brand_candidate:p08-dice-stale-reconnect-refresh:brand_neutral_preflight_anchor
  - theorem_prover_brand_candidate:p08-dice-stale-reconnect-refresh:adapter_boundary_refs_keep
  - theorem_prover_brand_candidate:p08-dice-stale-reconnect-refresh:concrete_brand_not_adopted
  final_public_contract_reopen_sequence_refs:
  - theorem_final_public_contract_reopen:p08-dice-stale-reconnect-refresh:result_object_and_payload_first
  - theorem_final_public_contract_reopen:p08-dice-stale-reconnect-refresh:prover_brand_and_proof_schema_second
  - theorem_final_public_contract_reopen:p08-dice-stale-reconnect-refresh:final_public_verifier_contract_third
  threshold_default_refs:
  - theorem_final_public_contract_reopen_default:result_object_and_payload_first
  - theorem_final_public_contract_reopen_default:prover_brand_and_proof_schema_second
  - theorem_final_public_contract_reopen_default:final_public_verifier_contract_third
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:theorem_final_public_contract_reopen_threshold
  - compare_floor:current_l2.theorem_final_public_contract_reopen_threshold
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.theorem_review_unit_transport_actual_adoption
  - compare_floor:current_l2.theorem_result_object_preview_actualization
  - compare_floor:current_l2.theorem_result_payload_public_contract.coupled_later_gate
  - compare_floor:current_l2.theorem_result_object_actual_adoption
  - compare_floor:current_l2.theorem_final_public_contract_reopen_threshold
  guard_refs:
  - guard:result_object_and_payload_first
  - guard:prover_brand_and_proof_schema_second
  - guard:final_public_verifier_contract_third
  kept_later_refs:
  - kept_later:final_public_theorem_result_object
  - kept_later:consumer_shaped_theorem_payload
  - kept_later:concrete_theorem_prover_brand
  - kept_later:proof_object_public_schema
  - kept_later:final_public_verifier_contract
  guard_reason: none
model_check_final_public_contract_reopen_threshold:
  status: guarded_not_reached
  threshold_kind: helper_local_reopen_threshold_manifest
  subject_kind: runtime_try_cut_cluster
  subject_ref: p08-dice-stale-reconnect-refresh
  checker_artifact_route_refs: []
  migration_candidate_keep_refs: []
  verifier_handoff_candidate_refs: []
  tool_brand_candidate_refs: []
  final_public_contract_reopen_sequence_refs: []
  threshold_default_refs: []
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:model_check_final_public_contract_reopen_threshold
  - compare_floor:current_l2.model_check.final_public_contract_reopen_threshold
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.model_check.final_public_contract_reopen_threshold.guard_only
  guard_refs:
  - guard:model_check_final_public_contract_reopen_threshold_not_reached
  kept_later_refs:
  - kept_later:first_settled_property_language
  - kept_later:concrete_model_check_tool_brand
  - kept_later:final_public_checker_artifact
  - kept_later:actual_public_checker_migration
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:production_checker_runtime_policy_contract
  - kept_later:final_public_verifier_contract
  guard_reason: current model-check final public-contract reopen threshold only actualizes the representative checker quartet (`e5` / `p06` / `p07` / `p09`) after verification preview reaches the formal-hook route for `p08-dice-stale-reconnect-refresh`
typed_checker_hint_preview:
  status: guarded_not_reached
  preview_kind: sample_local_helper_preview
  cluster_kind: none
  case_label: none
  typed_reason_family_hint: none
  evidence_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.typed.sample_local_checker_hint_guard_only
  guard_refs:
  - guard:typed_checker_hint_preview_not_reached
  kept_later_refs:
  - kept_later:final_typed_source_principal
  - kept_later:final_finite_index_surface
  - kept_later:final_ifc_syntax
  - kept_later:actual_checker_payload_family
  - kept_later:checker_supported_kind_summary
  - kept_later:final_public_verifier_contract
  guard_reason: current typed checker-hint preview only actualizes the sample-local first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after verification preview reaches runtime try-cut evidence for `p08-dice-stale-reconnect-refresh`
actual_checker_payload_family_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_payload_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_family_kind: none
  source_refs: []
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_checker_payload_family_threshold
  - compare_floor:current_l2.checker.actual_checker_payload_family
  compare_floor_refs:
  - compare_floor:current_l2.checker.actual_checker_payload_family.guard_only
  guard_refs:
  - guard:actual_checker_payload_family_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_family
  - kept_later:checker_payload_row_detail
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload family threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after typed checker-hint preview reaches the checker-adjacent helper floor for `p08-dice-stale-reconnect-refresh`
actual_checker_payload_row_family_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_family_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_family_ref: none
  row_family_kind: none
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_checker_payload_row_family_threshold
  - compare_floor:current_l2.checker.checker_payload_row_family
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_family.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_family_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_detail
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-family threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload family threshold reaches the checker-adjacent helper floor for `p08-dice-stale-reconnect-refresh`
actual_checker_payload_row_detail_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_detail_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_row_family_ref: none
  row_source_ref: none
  row_reason_kind: none
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_checker_payload_row_detail_threshold
  - compare_floor:current_l2.checker.checker_payload_row_detail
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_detail.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_detail_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-detail threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-family threshold reaches the checker-adjacent helper floor for `p08-dice-stale-reconnect-refresh`
actual_checker_payload_row_body_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_body_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_row_family_ref: none
  row_source_ref: none
  row_reason_kind: none
  row_body: none
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_checker_payload_row_body_threshold
  - compare_floor:current_l2.checker.checker_payload_row_body
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_body.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_body_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-body threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-detail threshold reaches the checker-adjacent helper floor for `p08-dice-stale-reconnect-refresh`
actual_checker_payload_supported_kind_summary_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_supported_kind_summary_threshold_manifest
  payload_row_family_ref: none
  supported_kind_scope: none
  supported_kind_refs: []
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_checker_payload_supported_kind_summary_threshold
  - compare_floor:current_l2.checker.checker_payload_supported_kind_summary
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_supported_kind_summary.guard_only
  guard_refs:
  - guard:actual_checker_payload_supported_kind_summary_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload supported-kind summary threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-body threshold reaches the checker-adjacent helper floor for `p08-dice-stale-reconnect-refresh`
actual_checker_payload_public_schema_sketch_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_payload_schema_sketch_threshold_manifest
  actual_checker_payload_family_ref: none
  checker_payload_row_family_ref: none
  checker_payload_row_detail_ref: none
  checker_payload_row_body_ref: none
  checker_payload_supported_kind_summary_ref: none
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_checker_payload_public_schema_sketch_threshold
  - compare_floor:current_l2.checker.public_checker_payload_schema
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_payload_schema.guard_only
  guard_refs:
  - guard:actual_checker_payload_public_schema_sketch_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_api
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload public-schema sketch threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload supported-kind summary threshold reaches the checker-adjacent helper floor for `p08-dice-stale-reconnect-refresh`
actual_public_checker_api_sketch_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_api_sketch_threshold_manifest
  checker_subject: none
  public_checker_payload_schema_ref: none
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_public_checker_api_sketch_threshold
  - compare_floor:current_l2.checker.public_checker_api
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_api.guard_only
  guard_refs:
  - guard:actual_public_checker_api_sketch_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_entry_criteria
  - kept_later:public_checker_command_surface
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker API sketch threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload public-schema sketch threshold reaches the checker-adjacent helper floor for `p08-dice-stale-reconnect-refresh`
actual_public_checker_entry_criteria_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_entry_criteria_threshold_manifest
  public_checker_api_ref: none
  entry_criteria_refs: []
  family_facade_support_ref: none
  family_facade_script_refs: []
  smoke_command_refs: []
  next_comparison_target_ref: none
  deferred_boundary_refs: []
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_public_checker_entry_criteria_threshold
  - compare_floor:current_l2.checker.public_checker_entry_criteria
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_entry_criteria.guard_only
  guard_refs:
  - guard:actual_public_checker_entry_criteria_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_command_surface
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker entry-criteria threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker API sketch threshold reaches the checker-adjacent helper floor for `p08-dice-stale-reconnect-refresh`
actual_public_checker_command_surface_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_command_surface_threshold_manifest
  command_surface_kind: none
  family_facade_command_refs: []
  public_checker_api_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_public_checker_command_surface_threshold
  - compare_floor:current_l2.checker.public_checker_command_surface
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_command_surface.guard_only
  guard_refs:
  - guard:actual_public_checker_command_surface_threshold_not_reached
  kept_later_refs:
  - kept_later:detached_loop_smoke_wrapper
  - kept_later:generic_shared_public_checker_entry
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker command-surface threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker entry-criteria threshold reaches the checker-adjacent helper floor for `p08-dice-stale-reconnect-refresh`
actual_shared_output_contract_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_shared_output_contract_threshold_manifest
  output_contract_kind: none
  checker_cluster_name: none
  checker_status: none
  public_checker_payload_schema_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_shared_output_contract_threshold
  - compare_floor:current_l2.checker.shared_output_contract
  compare_floor_refs:
  - compare_floor:current_l2.checker.shared_output_contract.guard_only
  guard_refs:
  - guard:actual_shared_output_contract_threshold_not_reached
  kept_later_refs:
  - kept_later:detached_loop_wrapper_paths
  - kept_later:fixture_and_actual_rows_textual_rendering
  - kept_later:generic_shared_public_checker_entry
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual shared output contract threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker command-surface threshold reaches the checker-adjacent helper floor for `p08-dice-stale-reconnect-refresh`
actual_public_checker_boundary_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_boundary_threshold_manifest
  boundary_kind: none
  public_checker_command_surface_ref: none
  shared_output_contract_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_public_checker_boundary_threshold
  - compare_floor:current_l2.checker.public_checker_boundary
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_boundary.guard_only
  guard_refs:
  - guard:actual_public_checker_boundary_threshold_not_reached
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:query_token_and_checker_subject_public_naming
  - kept_later:generic_shared_public_checker_entry
  - kept_later:detached_loop_wrapper_path_line
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker boundary threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual shared output contract threshold reaches the checker-adjacent helper floor for `p08-dice-stale-reconnect-refresh`
actual_verifier_handoff_surface_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_verifier_handoff_surface_threshold_manifest
  handoff_surface_kind: none
  public_checker_boundary_ref: none
  proof_obligation_matrix_ref: none
  handoff_artifact_mode: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_verifier_handoff_surface_threshold
  - compare_floor:current_l2.checker.verifier_handoff_surface
  compare_floor_refs:
  - compare_floor:current_l2.checker.verifier_handoff_surface.guard_only
  guard_refs:
  - guard:actual_verifier_handoff_surface_threshold_not_reached
  kept_later_refs:
  - kept_later:subject_rows
  - kept_later:theorem_protocol_runtime_dedicated_handoff_artifact_family
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:tool_specific_schema_and_actual_emitter_policy
  - kept_later:final_parser_grammar
  - kept_later:query_token_and_shared_generic_entry
  - kept_later:final_public_verifier_contract
  guard_reason: current actual verifier handoff surface threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker boundary threshold reaches the checker-adjacent helper floor for `p08-dice-stale-reconnect-refresh`
actual_minimal_parser_subset_freeze_threshold:
  status: guarded_not_reached
  threshold_kind: parser_front_minimal_parser_subset_freeze_threshold_manifest
  freeze_kind: none
  accepted_cluster_refs: []
  reject_cluster_refs: []
  retention_floor_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_minimal_parser_subset_freeze_threshold
  - compare_floor:current_l2.parser.minimal_parser_subset_freeze
  compare_floor_refs:
  - compare_floor:current_l2.parser.minimal_parser_subset_freeze.guard_only
  guard_refs:
  - guard:actual_minimal_parser_subset_freeze_threshold_not_reached
  kept_later_refs:
  - kept_later:stage3_admit_slot_branch
  - kept_later:stage3_request_clause_branch
  - kept_later:stage3_predicate_fragment_branch
  - kept_later:public_parser_api
  - kept_later:final_parser_grammar
  - kept_later:parser_to_checker_reconnect_freeze
  - kept_later:final_public_parser_checker_api
  - kept_later:final_public_verifier_contract
  guard_reason: current actual minimal parser subset freeze threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual verifier handoff surface threshold reaches the helper-local docs-only bridge floor for `p08-dice-stale-reconnect-refresh`
actual_parser_to_checker_reconnect_freeze_threshold:
  status: guarded_not_reached
  threshold_kind: parser_checker_bridge_reconnect_freeze_threshold_manifest
  reconnect_kind: none
  parser_subset_freeze_ref: none
  checker_floor_refs: []
  retained_helper_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_parser_to_checker_reconnect_freeze_threshold
  - compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze
  compare_floor_refs:
  - compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze.guard_only
  guard_refs:
  - guard:actual_parser_to_checker_reconnect_freeze_threshold_not_reached
  kept_later_refs:
  - kept_later:stage3_request_predicate_reconnect_line
  - kept_later:stage1_direct_target_mismatch_redesign_line
  - kept_later:runtime_contrast_e21_e22_line
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_api
  - kept_later:actual_external_verifier_schema
  - kept_later:final_public_verifier_contract
  guard_reason: current actual parser-to-checker reconnect freeze threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual minimal parser subset freeze threshold reaches the stage1+stage2 parser floor for `p08-dice-stale-reconnect-refresh`
actual_phase1_semantics_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase1_semantics_closeout_threshold_manifest
  closeout_kind: none
  core_semantics_refs: []
  invariant_bridge_refs: []
  notation_status_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_phase1_semantics_closeout_threshold
  - compare_floor:current_l2.closeout.phase1_semantics_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase1_semantics_closeout.guard_only
  guard_refs:
  - guard:actual_phase1_semantics_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_reserved_keyword_and_punctuation
  - kept_later:final_type_system
  - kept_later:actual_external_verifier_schema
  - kept_later:actual_emitted_verifier_artifact
  - kept_later:final_public_verifier_contract
  guard_reason: current actual phase1 semantics closeout threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual parser-to-checker reconnect freeze threshold reaches the checker-floor bridge for `p08-dice-stale-reconnect-refresh`
actual_phase2_parser_free_poc_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase2_parser_free_poc_closeout_threshold_manifest
  closeout_kind: none
  compile_gate_refs: []
  helper_boundary_refs: []
  detached_loop_policy_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_phase2_parser_free_poc_closeout_threshold
  - compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout.guard_only
  guard_refs:
  - guard:actual_phase2_parser_free_poc_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:reference_update_bless_workflow
  - kept_later:final_retention_path_policy
  - kept_later:public_exporter_api
  - kept_later:production_host_interface
  guard_reason: current actual phase2 parser-free PoC closeout threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual phase1 semantics closeout threshold reaches the semantics closeout floor for `p08-dice-stale-reconnect-refresh`
actual_phase4_shared_space_self_driven_closeout_threshold:
  status: reached
  threshold_kind: phase4_shared_space_self_driven_closeout_threshold_manifest
  closeout_kind: shared_space_practical_boundary_checkpoint
  current_package_refs:
  - authoritative_room_baseline_ref
  - working_subset_catalog_ref
  - minimal_authority_witness_core_ref
  - authoritative_delegated_provider_cut_ref
  - control_plane_threshold_ref
  user_spec_required_catalog_refs:
  - final_activation_overlay_catalog
  - final_authority_auth_identity_admission_catalog
  - final_consistency_fairness_catalog
  retained_later_refs:
  - append_friendly_optional_provider_attestation
  - control_plane_separated_carrier_actualization
  - distributed_fairness_protocol
  - final_operational_realization
  next_comparison_target_ref: phase5_proof_protocol_runtime_policy_handoff_closeout_comparison
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  guard_refs:
  - guard:phase4_shared_space_self_driven_closeout_threshold_only
  - guard:phase5_proof_protocol_runtime_policy_handoff_closeout_comparison_next
  - guard:user_spec_required_final_catalog_later
  - guard:distributed_fairness_protocol_later
  kept_later_refs:
  - kept_later:final_public_witness_provider_artifact_contract
  - kept_later:exhaustive_shared_space_catalog
  - kept_later:control_plane_separated_carrier_actualization
  - kept_later:distributed_fairness_protocol
  - kept_later:final_operational_realization
  guard_reason: none
actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold:
  status: reached
  threshold_kind: phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_manifest
  closeout_kind: proof_protocol_runtime_policy_handoff_stop_line
  verifier_handoff_surface_ref: minimal_verifier_handoff_surface
  theorem_retained_bridge_stop_ref: retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  boundary_inventory_ref: small_decidable_core_boundary_inventory
  retained_later_refs:
  - actual_subject_row_materialization
  - boundary_specific_handoff_artifact_family
  - actual_emitted_verifier_artifact
  - concrete_tool_binding
  - public_checker_migration
  - low_level_memory_order_family
  next_comparison_target_ref: phase6_actual_parser_ast_carrier_first_tranche_comparison
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  guard_refs:
  - guard:phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_only
  - guard:phase6_actual_parser_ast_carrier_first_tranche_comparison_next
  - guard:actual_subject_row_and_artifact_family_later
  - guard:tool_binding_public_checker_migration_and_low_level_memory_order_later
  kept_later_refs:
  - kept_later:actual_subject_row_materialization
  - kept_later:boundary_specific_handoff_artifact_family
  - kept_later:actual_emitted_verifier_artifact
  - kept_later:concrete_tool_binding
  - kept_later:public_checker_migration
  - kept_later:low_level_memory_order_family
  guard_reason: none
actual_phase6_actual_parser_ast_carrier_first_tranche_threshold:
  status: reached
  threshold_kind: phase6_actual_parser_ast_carrier_first_tranche_threshold_manifest
  carrier_kind: current_l2_nonproduction_parser_carrier
  accepted_surface_refs:
  - stage1_option_decl_chain_surface
  - stage2_try_fallback_structural_surface
  code_anchor_refs:
  - mir_ast_current_l2_module
  - stage1_stage2_parser_spike_tests
  retained_later_refs:
  - stage3_admit_slot_surface
  - stage3_request_clause_suite
  - stage3_predicate_fragment
  - perform_head_final_public_api
  - span_rich_diagnostics
  - final_grammar
  next_comparison_target_ref: phase6_actual_checker_runtime_skeleton_first_tranche_comparison
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  guard_refs:
  - guard:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_required
  - guard:phase6_actual_checker_runtime_skeleton_first_tranche_comparison_next
  kept_later_refs:
  - stage3_admit_slot_surface
  - stage3_request_clause_suite
  - stage3_predicate_fragment
  - perform_head_final_public_api
  - span_rich_diagnostics
  - final_grammar
  guard_reason: none
actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold:
  status: reached
  threshold_kind: phase6_actual_checker_runtime_skeleton_first_tranche_threshold_manifest
  skeleton_kind: current_l2_nonproduction_checker_runtime_skeleton
  semantic_entry_refs:
  - static_gate_program_detailed
  - direct_style_evaluator_from_program
  - fixture_host_stub_run_program
  runtime_bridge_refs:
  - mir_runtime_current_l2_module
  - current_l2_runtime_skeleton_report
  parser_bridge_contract_refs:
  - stage1_reconnect_clusters
  - stage2_try_rollback_structural_summary
  - parser_bridge_consistency_guard
  retained_later_refs:
  - parser_to_program_lowering
  - stage3_request_predicate_reconnect
  - richer_host_interface
  - final_public_runtime_checker_api
  - formal_hook_concrete_tool_binding
  next_comparison_target_ref: phase6_compile_ready_verification_and_formal_hook_comparison
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  guard_refs:
  - guard:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_required
  - guard:phase6_compile_ready_verification_and_formal_hook_comparison_next
  kept_later_refs:
  - parser_to_program_lowering
  - stage3_request_predicate_reconnect
  - richer_host_interface
  - final_public_runtime_checker_api
  - formal_hook_concrete_tool_binding
  guard_reason: none
actual_phase6_compile_ready_verification_and_formal_hook_threshold:
  status: reached
  threshold_kind: phase6_compile_ready_verification_and_formal_hook_threshold_manifest
  verification_gate_refs:
  - cargo_test_mir_ast
  - cargo_test_mir_runtime
  - cargo_test_mir_semantics_current_l2_minimal_interpreter
  - cargo_test_mir_semantics_current_l2_static_gate_support
  - cargo_test_mir_semantics_current_l2_detached_bundle_support
  - cargo_test_mir_semantics_current_l2_formal_hook_support
  - python_unittest_current_l2_static_and_detached_loop
  smoke_gate_refs:
  - smoke_formal_hook_static
  - smoke_formal_hook_runtime
  formal_hook_artifact_kind_ref: current_l2_tool_neutral_formal_hook
  formal_hook_subject_kind_refs:
  - fixture_static_cluster
  - runtime_try_cut_cluster
  formal_hook_contract_row_core_refs:
  - obligation_kind
  - evidence_refs
  formal_hook_evidence_ref_family_refs:
  - ref_kind
  - ref_id
  formal_hook_obligation_kind_refs:
  - canonical_normalization_law
  - no_re_promotion
  - rollback_cut_non_interference
  source_artifact_refs:
  - detached_static_gate_artifact
  - detached_bundle_artifact
  validation_refs:
  - input_schema_version_guard
  - input_artifact_kind_guard
  retained_later_refs:
  - concrete_theorem_tool_binding
  - concrete_model_check_tool_binding
  - parser_second_tranche_widen
  - final_public_surface
  next_comparison_target_ref: phase6_next_reopen_sequencing_comparison
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - source:phase6_compile_ready_verification_and_formal_hook_ready_sketch
  - source:phase6_compile_ready_verification_and_formal_hook_minimum
  - code_anchor:current_l2_emit_formal_hook_example
  - code_anchor:current_l2_formal_hook_support_tests
  - code_anchor:current_l2_detached_loop_smoke_family
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  guard_refs:
  - guard:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_required
  - guard:phase6_next_reopen_sequencing_comparison_next
  kept_later_refs:
  - concrete_theorem_tool_binding
  - concrete_model_check_tool_binding
  - parser_second_tranche_widen
  - final_public_surface
  guard_reason: none
actual_phase6_next_reopen_sequencing_threshold:
  status: reached
  threshold_kind: phase6_next_reopen_sequencing_threshold_manifest
  sequencing_kind_ref: phase6_checkpoint_postclose_next_reopen
  fixed_entry_criteria_refs:
  - phase6_parser_first_tranche
  - phase6_checker_runtime_first_tranche
  - phase6_compile_ready_formal_hook
  selected_first_reopen_ref: phase6_parser_second_tranche_attached_slot_and_predicate_fragment_route
  deferred_reopen_refs:
  - theorem_first_concrete_tool_binding_route
  - concrete_model_check_tool_binding
  minimum_guard_refs:
  - keep_tool_neutral_formal_hook_as_entry_criteria
  - avoid_request_head_clause_suite_richer_diagnostics_bulk_widen
  - keep_model_check_line_reserve_only
  next_comparison_target_ref: phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - source:phase6_compile_ready_verification_and_formal_hook_ready_sketch
  - source:phase6_compile_ready_verification_and_formal_hook_minimum
  - code_anchor:current_l2_emit_formal_hook_example
  - code_anchor:current_l2_formal_hook_support_tests
  - code_anchor:current_l2_detached_loop_smoke_family
  - helper_preview:actual_phase6_next_reopen_sequencing_threshold
  - source:phase6_next_reopen_sequencing_current_first_choice
  - source:phase6_next_reopen_sequencing_minimum
  - source:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  - compare_floor:current_l2.closeout.phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package
  guard_refs:
  - guard:actual_phase6_compile_ready_verification_and_formal_hook_threshold_required
  - guard:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison_next
  kept_later_refs:
  - request_clause_suite_bulk_widen
  - perform_head_final_public_api
  - concrete_theorem_tool_binding
  - concrete_model_check_tool_binding
  - final_public_surface
  guard_reason: none
actual_phase6_reserve_formal_tool_binding_inventory_threshold:
  status: reached
  threshold_kind: phase6_reserve_formal_tool_binding_inventory_threshold_manifest
  inventory_kind: phase6_postclose_formal_reserve_inventory
  fixed_entry_criteria_refs:
  - phase5_handoff_closeout
  - phase6_compile_ready_formal_hook
  - phase6_parser_second_tranche_first_package
  first_reserve_ref: theorem_first_notebook_pressure_concrete_tool_binding_route
  second_reserve_ref: model_check_protocol_property_concrete_tool_binding_route
  minimum_guard_refs:
  - keep_tool_neutral_formal_hook_as_current_entry_criteria
  - keep_parser_followup_package_as_current_mainline
  - avoid_dual_tool_choice_single_package
  - avoid_public_checker_runtime_surface_backpressure
  next_comparison_target_ref: phase6_parser_side_follow_up_package_sequencing_comparison
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - source:phase6_compile_ready_verification_and_formal_hook_ready_sketch
  - source:phase6_compile_ready_verification_and_formal_hook_minimum
  - code_anchor:current_l2_emit_formal_hook_example
  - code_anchor:current_l2_formal_hook_support_tests
  - code_anchor:current_l2_detached_loop_smoke_family
  - helper_preview:actual_phase6_next_reopen_sequencing_threshold
  - source:phase6_next_reopen_sequencing_current_first_choice
  - source:phase6_next_reopen_sequencing_minimum
  - source:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison
  - helper_preview:actual_phase6_reserve_formal_tool_binding_inventory_threshold
  - source:phase6_reserve_formal_tool_binding_inventory_current_first_choice
  - source:phase6_reserve_formal_tool_binding_inventory_minimum
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  - compare_floor:current_l2.closeout.phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package
  - compare_floor:current_l2.closeout.phase6_reserve_formal_tool_binding_inventory
  guard_refs:
  - guard:actual_phase6_next_reopen_sequencing_threshold_required
  - guard:phase6_parser_side_follow_up_package_sequencing_comparison_next
  kept_later_refs:
  - concrete_theorem_tool_name
  - concrete_model_check_tool_name
  - actual_ci_artifact_retention_policy
  - parser_side_followup_package_selection
  - final_public_parser_checker_runtime_surface
  guard_reason: none
actual_phase6_parser_side_followup_package_sequencing_threshold:
  status: reached
  threshold_kind: phase6_parser_side_followup_package_sequencing_threshold_manifest
  sequencing_kind: phase6_parser_followup_next_package_selection
  fixed_entry_criteria_refs:
  - phase6_parser_second_tranche_first_package
  - phase6_reserve_formal_tool_binding_inventory
  - stage3_multiline_attachment_first_tranche_actualization
  selected_next_package_ref: phase6_parser_second_tranche_shared_single_attachment_frame_first_package
  deferred_reopen_refs:
  - phase6_request_clause_suite_publicization
  - phase6_perform_head_final_public_parser_api
  - phase6_span_rich_diagnostics
  - phase6_final_grammar
  minimum_guard_refs:
  - reuse_existing_stage3_minimal_predicate_fragment_surface
  - keep_request_head_and_suite_ordering_out_of_scope
  - keep_source_sample_path_after_parser_followup_cut
  next_comparison_target_ref: phase6_parser_second_tranche_shared_single_attachment_frame_first_package_comparison
  evidence_refs:
  - sample:p08-dice-stale-reconnect-refresh
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - source:phase6_compile_ready_verification_and_formal_hook_ready_sketch
  - source:phase6_compile_ready_verification_and_formal_hook_minimum
  - code_anchor:current_l2_emit_formal_hook_example
  - code_anchor:current_l2_formal_hook_support_tests
  - code_anchor:current_l2_detached_loop_smoke_family
  - helper_preview:actual_phase6_next_reopen_sequencing_threshold
  - source:phase6_next_reopen_sequencing_current_first_choice
  - source:phase6_next_reopen_sequencing_minimum
  - source:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison
  - helper_preview:actual_phase6_reserve_formal_tool_binding_inventory_threshold
  - source:phase6_reserve_formal_tool_binding_inventory_current_first_choice
  - source:phase6_reserve_formal_tool_binding_inventory_minimum
  - helper_preview:actual_phase6_parser_side_followup_package_sequencing_threshold
  - source:phase6_parser_side_followup_package_sequencing_current_first_choice
  - source:phase6_parser_side_followup_package_sequencing_minimum
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  - compare_floor:current_l2.closeout.phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package
  - compare_floor:current_l2.closeout.phase6_reserve_formal_tool_binding_inventory
  - compare_floor:current_l2.closeout.phase6_parser_side_followup_package_sequencing
  guard_refs:
  - guard:actual_phase6_reserve_formal_tool_binding_inventory_threshold_required
  - guard:phase6_parser_second_tranche_shared_single_attachment_frame_first_package_comparison_next
  kept_later_refs:
  - request_clause_suite_publicization
  - perform_head_final_public_parser_api
  - span_rich_diagnostics
  - source_sample_corpus_scope
  - final_public_parser_checker_runtime_surface
  guard_reason: none
non_admissible_metadata: []
```

### 出力の読み方

この sample でまず見るべき行は次である。

- `static_gate: valid`
  sample の形自体は current rule に合っている。
- `entered_evaluation: true`
  runtime evaluation に入っている。
- `terminal_outcome: success`
  最終的には fallback を含めて success で終わる。
- `events:` に `perform-failure`, `rollback`, `perform-success`
  これが fail then refresh の要点である。最初の perform は失敗し、rollback が入り、その後で refresh 側が success している。
- `debug_outputs:` の `refresh_owner_snapshot: stale reconnect redirected`
  stale reconnect をそのまま通さず、refresh へ向け直したことが分かる。

さらに `stage2_try_rollback_verdict: no_findings` は、current helper がこの `try/fallback` 構造について、現段階で追加の structural finding を出していないことを示す。

`formal_hook_status: reached` と `subject_kind: runtime_try_cut_cluster` は、`p07` と同じく runtime まで進んだ cluster に formal hook preview が到達している、という読みである。

## 3. 一見それっぽいが実はダメな 2 つの negative sample を見る

`p13` と `p14` は、初心者がもっとも誤読しやすい 2 本である。どちらも「late joiner に何か見せたい」ようには見える。しかし current line では static stop にする。

### 3-1. `p13`: publication witness がない

#### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  run-source-sample \
  samples/prototype/current-l2-order-handoff/p13-dice-late-join-missing-publication-witness.txt \
  --format pretty
```

このコマンドが確認していること:

- publish を書かずに handoff と observe だけで late join visibility を主張したとき、static stop するか
- その止め方が `underdeclared` になるか
- runtime には入らないが、static cluster として formal hook preview は持てるか

#### コード全文

```text
# late join view を見せたいが、publish を欠いたまま handoff してしまう helper-local negative prototype。
# current authoritative-room line では publication witness 不在として static stop にする。
place root {
  place room {
    place dice_authority {
      perform handoff_dice_authority on dice_state
        require owner_is(player_a)
        ensure owner_is(player_b)

      atomic_cut

      perform observe_late_join_view on dice_state
        require read
        ensure late_join_sees_published_result(player_c)
    }
  }
}
```

#### 行ごとの解説

1. `# late join view を見せたいが、publish を欠いたまま handoff してしまう helper-local negative prototype。`
   コメントがすでに問題点を言っている。late join view を見せたいのに publish がない。
2. `# current authoritative-room line では publication witness 不在として static stop にする。`
   current line の判定方針を明示している。つまり、これは runtime で様子を見る話ではなく、静的に止める話である。
3. `place root {`
   root place。
4. `place room {`
   room place。
5. `place dice_authority {`
   authority place。
6. `perform handoff_dice_authority on dice_state`
   いきなり handoff をしている。ここで publish が存在しないことが後で効く。
7. `require owner_is(player_a)`
   handoff 前提として現在 owner が `player_a` であることを要求している。
8. `ensure owner_is(player_b)`
   handoff 後に `player_b` が owner になることを約束している。
9. 空行
   可読性のための空行。
10. `atomic_cut`
    handoff 段を cut として切っている。しかし、publish が先行していない以上、この cut が late join visibility の根拠にはならない。
11. 空行
    handoff 段と observe 段の区切り。
12. `perform observe_late_join_view on dice_state`
    late join view の観測を行おうとしている。
13. `require read`
    観測前提として read を要求している。
14. `ensure late_join_sees_published_result(player_c)`
    `player_c` が publish 済み結果を見られる、と書いている。しかしここで問題になるのは、「publish 済みと言っているのに、その publish がコード内に存在しない」ことである。
15. `}`
    `dice_authority` を閉じる。
16. `}`
    `room` を閉じる。
17. `}`
    `root` を閉じる。

#### なぜ `underdeclared` なのか

この sample は「late join に見せる」という目標自体は書いてある。しかし、そのために必要な先行 publish がない。つまり、部品の並び順以前に、**必要な根拠が足りない**。だから current line では `underdeclared` と読む。

初心者向けに言うと、これは「答えを書いたが、その答えに至る根拠を前段で宣言していない」状態である。

#### 実行例全文

```text
shell: mir-current-l2
command: run-source-sample
sample: p13-dice-late-join-missing-publication-witness
sample_path: /home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-order-handoff/p13-dice-late-join-missing-publication-witness.txt
host_plan_path: /home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-order-handoff/p13-dice-late-join-missing-publication-witness.host-plan.json
static_gate: underdeclared
static_reasons:
- missing publication witness before handoff for late-join visibility at root / room / dice_authority
stage1_reconnect_clusters: not_available
stage2_try_rollback_verdict: not_available
entered_evaluation: false
terminal_outcome: none
steps_executed: 0
events:
debug_outputs: []
verification_preview:
  formal_hook_status: reached
  subject_kind: fixture_static_cluster
  subject_ref: p13-dice-late-join-missing-publication-witness
  proof_notebook_review_unit_obligations:
  - canonical_normalization_law
  - no_re_promotion
  model_check_concrete_carrier_obligations:
  - canonical_normalization_law
  - no_re_promotion
  guard_reason: none
artifact_preview:
  proof_notebook_review_units:
  - obligation_kind: canonical_normalization_law
    goal_text: Review that canonical normalization preserves the rejected static shape for `p13-dice-late-join-missing-publication-witness`.
    evidence_refs:
    - fixture:p13-dice-late-join-missing-publication-witness
    - static_gate_artifact:p13-dice-late-join-missing-publication-witness
  - obligation_kind: no_re_promotion
    goal_text: Review that `p13-dice-late-join-missing-publication-witness` remains rejected and is not re-promoted by later tooling.
    evidence_refs:
    - fixture:p13-dice-late-join-missing-publication-witness
  model_check_concrete_carriers:
  - obligation_kind: canonical_normalization_law
    evidence_refs:
    - fixture:p13-dice-late-join-missing-publication-witness
    - static_gate_artifact:p13-dice-late-join-missing-publication-witness
  - obligation_kind: no_re_promotion
    evidence_refs:
    - fixture:p13-dice-late-join-missing-publication-witness
surface_preview:
  minimal_companion:
    status: guarded_not_reached
    guard_reason: current minimal companion surface only actualizes reached authoritative-room defaults: current authoritative-room vertical slice only actualizes reached current default samples (`p07` / `p08`): current default samples (`p07` / `p08`) were not reached for `p13-dice-late-join-missing-publication-witness`
    lines: []
    compare_floor_refs:
    - compare_floor:current_l2.experimental_order_handoff_guard_only
    guard_refs:
    - guard:companion_surface_not_reached
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
  stage_block_secondary:
    status: guarded_not_reached
    guard_reason: current stage-block secondary surface only actualizes reached authoritative-room defaults: current authoritative-room vertical slice only actualizes reached current default samples (`p07` / `p08`): current default samples (`p07` / `p08`) were not reached for `p13-dice-late-join-missing-publication-witness`
    lines: []
    compare_floor_refs:
    - compare_floor:current_l2.experimental_stage_block_guard_only
    guard_refs:
    - guard:stage_block_surface_not_reached
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:authoritative_room_serial_scope_sugar
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
  serial_scope_reserve:
    status: guarded_not_reached
    guard_reason: current serial-scope reserve surface only actualizes authoritative-room-specific reached routes (`p07` / `p08` / `p09`): sample `p13-dice-late-join-missing-publication-witness` did not reach the authoritative-room serial-scope reserve surface
    lines: []
    compare_floor_refs:
    - compare_floor:current_l2.order_handoff.serial_scope_reserve_surface.guard_only
    guard_refs:
    - guard:serial_scope_reserve_surface_not_reached
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:serial_scope_public_promotion
    - kept_later:serial_scope_beyond_authoritative_room
    - kept_later:final_source_surface_handoff_wording
    - kept_later:final_emitted_artifact_schema
    - kept_later:final_emitted_handoff_contract
    - kept_later:final_public_witness_schema
    - kept_later:final_public_provider_receipt_schema
    - kept_later:combined_provider_witness_public_contract
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
authoritative_room_first_scenario_actual_adoption:
  status: guarded_not_reached
  adoption_kind: helper_local_authoritative_room_first_scenario_manifest
  profile_axis_refs: []
  relation_refs: []
  authority_handoff_refs: []
  runtime_evidence_refs: []
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p13-dice-late-join-missing-publication-witness:canonical_normalization_law
  - repo_local_emitted_artifact:proof_notebook_review_unit:p13-dice-late-join-missing-publication-witness:no_re_promotion
  - repo_local_emitted_artifact:model_check_concrete_carrier:p13-dice-late-join-missing-publication-witness:canonical_normalization_law
  - repo_local_emitted_artifact:model_check_concrete_carrier:p13-dice-late-join-missing-publication-witness:no_re_promotion
  reserve_route_refs: []
  negative_static_stop_refs:
  - negative_static_stop:p13-dice-late-join-missing-publication-witness:publish_witness_required_before_handoff
  - negative_static_stop:p13-dice-late-join-missing-publication-witness:publish_then_handoff_then_observe_order_required
  contrast_refs:
  - contrast_target:append_friendly_notice_room
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:authoritative_room_first_scenario_actual_adoption
  - compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption
  compare_floor_refs:
  - compare_floor:current_l2.order_handoff.negative_static_stop_actualization
  - compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption.guard_only
  guard_refs:
  - guard:late_join_negative_static_stop
  - guard:first_scenario_pair_unchanged
  kept_later_refs:
  - kept_later:auditable_authority_witness
  - kept_later:delegated_rng_service
  - kept_later:distributed_randomness_provider
  - kept_later:final_emitted_handoff_contract
  - kept_later:exhaustive_shared_space_catalog
  - kept_later:final_consistency_fairness_catalog
  guard_reason: current authoritative-room first scenario keeps the late-join negative pair helper-local and guarded for `p13-dice-late-join-missing-publication-witness`: missing publication witness before handoff; missing publication witness before handoff for late-join visibility at root / room / dice_authority
authoritative_room_reserve_strengthening_lane:
  status: guarded_not_reached
  lane_kind: helper_local_reserve_strengthening_lane_manifest
  witness_strengthening_status: guarded_not_reached
  delegated_rng_service_status: guarded_not_reached
  model_check_second_line_status: guarded_not_reached
  witness_strengthening_refs: []
  delegated_rng_service_refs: []
  model_check_second_line_refs: []
  first_line_boundary_refs:
  - first_line_boundary:representative_pair_kept_at_p07_p08
  - first_line_boundary:authoritative_room_default_profile_stays_principal
  - first_line_boundary:authority_rng_default_profile_unchanged
  reserve_boundary_refs:
  - reserve_boundary:auditable_authority_witness_second_strengthening
  - reserve_boundary:delegated_rng_service_second_practical
  - reserve_boundary:model_check_second_line_not_room_profile
  - reserve_boundary:public_checker_contract_kept_later
  - reserve_boundary:witness_provider_combined_public_contract_later
  repo_local_emitted_artifact_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.reserve_strengthening_lane.guard_only
  - compare_floor:current_l2.reserve_strengthening_lane:p13-dice-late-join-missing-publication-witness
  guard_refs:
  - guard:representative_reserve_strengthening_sample_set
  - guard:first_line_pair_unchanged
  kept_later_refs:
  - kept_later:combined_witness_provider_public_contract
  - kept_later:final_public_witness_schema
  - kept_later:final_public_provider_receipt_schema
  - kept_later:concrete_model_check_tool_brand
  - kept_later:actual_public_checker_migration
  - kept_later:distributed_fairness_theorem
  guard_reason: current authoritative-room reserve strengthening lane only actualizes the representative reserve strengthening sample set (`p07` witness, `p08` reconnect-model-check, `p09` delegated RNG); `p13-dice-late-join-missing-publication-witness` stays guard-only until one of those reserve routes is actually exercised
order_handoff_source_surface_artifact_actual_adoption:
  status: guarded_not_reached
  adoption_kind: helper_local_source_surface_artifact_route_manifest
  profile_axis_refs: []
  principal_surface_lines: []
  secondary_surface_lines: []
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p13-dice-late-join-missing-publication-witness:canonical_normalization_law
  - repo_local_emitted_artifact:proof_notebook_review_unit:p13-dice-late-join-missing-publication-witness:no_re_promotion
  - repo_local_emitted_artifact:model_check_concrete_carrier:p13-dice-late-join-missing-publication-witness:canonical_normalization_law
  - repo_local_emitted_artifact:model_check_concrete_carrier:p13-dice-late-join-missing-publication-witness:no_re_promotion
  source_wording_route_refs: []
  emitted_artifact_candidate_keep_refs: []
  negative_static_stop_refs:
  - negative_static_stop:p13-dice-late-join-missing-publication-witness:publish_witness_required_before_handoff
  - negative_static_stop:p13-dice-late-join-missing-publication-witness:publish_then_handoff_then_observe_order_required
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:order_handoff_source_surface_artifact_actual_adoption
  - compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption
  compare_floor_refs:
  - compare_floor:current_l2.order_handoff.negative_static_stop_actualization
  - compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption.guard_only
  guard_refs:
  - guard:source_surface_artifact_actual_adoption_not_reached
  - guard:negative_static_stop_pair_kept_helper_local
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_runtime_api
  - kept_later:final_source_surface_handoff_wording
  - kept_later:final_emitted_artifact_schema
  - kept_later:final_emitted_handoff_contract
  - kept_later:final_public_witness_schema
  - kept_later:authoritative_room_serial_scope_sugar
  - kept_later:low_level_memory_order_source_surface
  - kept_later:final_modal_foundation_adoption
  guard_reason: current order-handoff source surface / emitted-artifact actual adoption keeps the late-join negative pair helper-local and guarded for `p13-dice-late-join-missing-publication-witness`: missing publication witness before handoff for late-join visibility at root / room / dice_authority
order_handoff_witness_provider_public_seam_compression:
  status: guarded_not_reached
  compression_kind: helper_local_public_seam_manifest
  profile_axis_refs: []
  repo_local_emitted_artifact_refs: []
  source_wording_route_refs: []
  emitted_artifact_candidate_keep_refs: []
  serial_scope_lines: []
  witness_schema_route_refs: []
  provider_receipt_route_refs: []
  combined_public_contract_keep_refs: []
  trace_alignment_pair_refs: []
  public_seam_residual_refs: []
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:order_handoff_witness_provider_public_seam_compression
  - compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression
  compare_floor_refs:
  - compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression.guard_only
  guard_refs:
  - guard:order_handoff_witness_provider_public_seam_compression_not_reached
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_runtime_api
  - kept_later:final_source_surface_handoff_wording
  - kept_later:final_emitted_artifact_schema
  - kept_later:final_public_witness_schema
  - kept_later:final_public_provider_receipt_schema
  - kept_later:delegated_provider_attestation
  - kept_later:combined_provider_witness_public_contract
  - kept_later:final_emitted_handoff_contract
  - kept_later:authoritative_room_serial_scope_sugar
  - kept_later:low_level_memory_order_source_surface
  - kept_later:final_modal_foundation_adoption
  - kept_later:exhaustive_shared_space_catalog
  guard_reason: current order-handoff/witness-provider public seam compression only actualizes the representative authoritative-room pair (`p07` / `p08`) after route/reserve/bridge/threshold floors align for `p13-dice-late-join-missing-publication-witness`
theorem_result_object_preview:
  status: guarded_not_reached
  preview_kind: helper_local_actualization_manifest
  subject_kind: fixture_static_cluster
  subject_ref: p13-dice-late-join-missing-publication-witness
  result_object_route_refs: []
  notebook_payload_preview_refs: []
  proof_object_schema_reserve_refs: []
  actual_adoption_default_refs: []
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:theorem_result_object_preview
  - compare_floor:current_l2.theorem_result_object_preview_actualization
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.theorem_result_object_preview.guard_only
  guard_refs:
  - guard:theorem_result_object_preview_not_reached
  kept_later_refs:
  - kept_later:final_public_theorem_result_object
  - kept_later:consumer_shaped_theorem_payload
  - kept_later:concrete_theorem_prover_brand
  - kept_later:proof_object_public_schema
  - kept_later:final_public_verifier_contract
  guard_reason: current theorem result-object preview only actualizes the representative theorem quartet (`e5` / `p06` / `p07` / `p08`) after verification preview reaches the formal-hook route for `p13-dice-late-join-missing-publication-witness`
model_check_public_checker_preview:
  status: guarded_not_reached
  preview_kind: helper_local_actualization_manifest
  subject_kind: fixture_static_cluster
  subject_ref: p13-dice-late-join-missing-publication-witness
  checker_artifact_preview_refs: []
  verifier_handoff_reserve_refs: []
  tool_binding_reserve_refs: []
  actual_adoption_default_refs: []
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:model_check_public_checker_preview
  - compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.model_check.public_checker_artifact_preview.guard_only
  guard_refs:
  - guard:model_check_public_checker_artifact_preview_not_reached
  kept_later_refs:
  - kept_later:first_settled_property_language
  - kept_later:concrete_model_check_tool_brand
  - kept_later:final_public_checker_artifact
  - kept_later:actual_public_checker_migration
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:production_checker_runtime_policy_contract
  - kept_later:final_public_verifier_contract
  guard_reason: current model-check public-checker preview only actualizes the representative checker quartet (`e5` / `p06` / `p07` / `p09`) after verification preview reaches the formal-hook route for `p13-dice-late-join-missing-publication-witness`
theorem_final_public_contract_reopen_threshold:
  status: guarded_not_reached
  threshold_kind: helper_local_reopen_threshold_manifest
  subject_kind: fixture_static_cluster
  subject_ref: p13-dice-late-join-missing-publication-witness
  result_object_route_refs: []
  payload_preview_keep_refs: []
  proof_object_schema_candidate_refs: []
  prover_brand_candidate_refs: []
  final_public_contract_reopen_sequence_refs: []
  threshold_default_refs: []
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:theorem_final_public_contract_reopen_threshold
  - compare_floor:current_l2.theorem_final_public_contract_reopen_threshold
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.theorem_final_public_contract_reopen_threshold.guard_only
  guard_refs:
  - guard:theorem_final_public_contract_reopen_threshold_not_reached
  kept_later_refs:
  - kept_later:final_public_theorem_result_object
  - kept_later:consumer_shaped_theorem_payload
  - kept_later:concrete_theorem_prover_brand
  - kept_later:proof_object_public_schema
  - kept_later:final_public_verifier_contract
  guard_reason: current theorem final public-contract reopen threshold only actualizes the representative theorem quartet (`e5` / `p06` / `p07` / `p08`) after verification preview reaches the formal-hook route for `p13-dice-late-join-missing-publication-witness`
model_check_final_public_contract_reopen_threshold:
  status: guarded_not_reached
  threshold_kind: helper_local_reopen_threshold_manifest
  subject_kind: fixture_static_cluster
  subject_ref: p13-dice-late-join-missing-publication-witness
  checker_artifact_route_refs: []
  migration_candidate_keep_refs: []
  verifier_handoff_candidate_refs: []
  tool_brand_candidate_refs: []
  final_public_contract_reopen_sequence_refs: []
  threshold_default_refs: []
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:model_check_final_public_contract_reopen_threshold
  - compare_floor:current_l2.model_check.final_public_contract_reopen_threshold
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.model_check.final_public_contract_reopen_threshold.guard_only
  guard_refs:
  - guard:model_check_final_public_contract_reopen_threshold_not_reached
  kept_later_refs:
  - kept_later:first_settled_property_language
  - kept_later:concrete_model_check_tool_brand
  - kept_later:final_public_checker_artifact
  - kept_later:actual_public_checker_migration
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:production_checker_runtime_policy_contract
  - kept_later:final_public_verifier_contract
  guard_reason: current model-check final public-contract reopen threshold only actualizes the representative checker quartet (`e5` / `p06` / `p07` / `p09`) after verification preview reaches the formal-hook route for `p13-dice-late-join-missing-publication-witness`
typed_checker_hint_preview:
  status: guarded_not_reached
  preview_kind: sample_local_helper_preview
  cluster_kind: none
  case_label: none
  typed_reason_family_hint: none
  evidence_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.typed.sample_local_checker_hint_guard_only
  guard_refs:
  - guard:typed_checker_hint_preview_not_reached
  kept_later_refs:
  - kept_later:final_typed_source_principal
  - kept_later:final_finite_index_surface
  - kept_later:final_ifc_syntax
  - kept_later:actual_checker_payload_family
  - kept_later:checker_supported_kind_summary
  - kept_later:final_public_verifier_contract
  guard_reason: current typed checker-hint preview only actualizes the sample-local first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after verification preview reaches runtime try-cut evidence for `p13-dice-late-join-missing-publication-witness`
actual_checker_payload_family_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_payload_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_family_kind: none
  source_refs: []
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_checker_payload_family_threshold
  - compare_floor:current_l2.checker.actual_checker_payload_family
  compare_floor_refs:
  - compare_floor:current_l2.checker.actual_checker_payload_family.guard_only
  guard_refs:
  - guard:actual_checker_payload_family_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_family
  - kept_later:checker_payload_row_detail
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload family threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after typed checker-hint preview reaches the checker-adjacent helper floor for `p13-dice-late-join-missing-publication-witness`
actual_checker_payload_row_family_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_family_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_family_ref: none
  row_family_kind: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_checker_payload_row_family_threshold
  - compare_floor:current_l2.checker.checker_payload_row_family
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_family.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_family_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_detail
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-family threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload family threshold reaches the checker-adjacent helper floor for `p13-dice-late-join-missing-publication-witness`
actual_checker_payload_row_detail_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_detail_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_row_family_ref: none
  row_source_ref: none
  row_reason_kind: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_checker_payload_row_detail_threshold
  - compare_floor:current_l2.checker.checker_payload_row_detail
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_detail.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_detail_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-detail threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-family threshold reaches the checker-adjacent helper floor for `p13-dice-late-join-missing-publication-witness`
actual_checker_payload_row_body_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_body_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_row_family_ref: none
  row_source_ref: none
  row_reason_kind: none
  row_body: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_checker_payload_row_body_threshold
  - compare_floor:current_l2.checker.checker_payload_row_body
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_body.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_body_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-body threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-detail threshold reaches the checker-adjacent helper floor for `p13-dice-late-join-missing-publication-witness`
actual_checker_payload_supported_kind_summary_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_supported_kind_summary_threshold_manifest
  payload_row_family_ref: none
  supported_kind_scope: none
  supported_kind_refs: []
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_checker_payload_supported_kind_summary_threshold
  - compare_floor:current_l2.checker.checker_payload_supported_kind_summary
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_supported_kind_summary.guard_only
  guard_refs:
  - guard:actual_checker_payload_supported_kind_summary_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload supported-kind summary threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-body threshold reaches the checker-adjacent helper floor for `p13-dice-late-join-missing-publication-witness`
actual_checker_payload_public_schema_sketch_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_payload_schema_sketch_threshold_manifest
  actual_checker_payload_family_ref: none
  checker_payload_row_family_ref: none
  checker_payload_row_detail_ref: none
  checker_payload_row_body_ref: none
  checker_payload_supported_kind_summary_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_checker_payload_public_schema_sketch_threshold
  - compare_floor:current_l2.checker.public_checker_payload_schema
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_payload_schema.guard_only
  guard_refs:
  - guard:actual_checker_payload_public_schema_sketch_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_api
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload public-schema sketch threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload supported-kind summary threshold reaches the checker-adjacent helper floor for `p13-dice-late-join-missing-publication-witness`
actual_public_checker_api_sketch_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_api_sketch_threshold_manifest
  checker_subject: none
  public_checker_payload_schema_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_public_checker_api_sketch_threshold
  - compare_floor:current_l2.checker.public_checker_api
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_api.guard_only
  guard_refs:
  - guard:actual_public_checker_api_sketch_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_entry_criteria
  - kept_later:public_checker_command_surface
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker API sketch threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload public-schema sketch threshold reaches the checker-adjacent helper floor for `p13-dice-late-join-missing-publication-witness`
actual_public_checker_entry_criteria_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_entry_criteria_threshold_manifest
  public_checker_api_ref: none
  entry_criteria_refs: []
  family_facade_support_ref: none
  family_facade_script_refs: []
  smoke_command_refs: []
  next_comparison_target_ref: none
  deferred_boundary_refs: []
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_public_checker_entry_criteria_threshold
  - compare_floor:current_l2.checker.public_checker_entry_criteria
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_entry_criteria.guard_only
  guard_refs:
  - guard:actual_public_checker_entry_criteria_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_command_surface
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker entry-criteria threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker API sketch threshold reaches the checker-adjacent helper floor for `p13-dice-late-join-missing-publication-witness`
actual_public_checker_command_surface_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_command_surface_threshold_manifest
  command_surface_kind: none
  family_facade_command_refs: []
  public_checker_api_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_public_checker_command_surface_threshold
  - compare_floor:current_l2.checker.public_checker_command_surface
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_command_surface.guard_only
  guard_refs:
  - guard:actual_public_checker_command_surface_threshold_not_reached
  kept_later_refs:
  - kept_later:detached_loop_smoke_wrapper
  - kept_later:generic_shared_public_checker_entry
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker command-surface threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker entry-criteria threshold reaches the checker-adjacent helper floor for `p13-dice-late-join-missing-publication-witness`
actual_shared_output_contract_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_shared_output_contract_threshold_manifest
  output_contract_kind: none
  checker_cluster_name: none
  checker_status: none
  public_checker_payload_schema_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_shared_output_contract_threshold
  - compare_floor:current_l2.checker.shared_output_contract
  compare_floor_refs:
  - compare_floor:current_l2.checker.shared_output_contract.guard_only
  guard_refs:
  - guard:actual_shared_output_contract_threshold_not_reached
  kept_later_refs:
  - kept_later:detached_loop_wrapper_paths
  - kept_later:fixture_and_actual_rows_textual_rendering
  - kept_later:generic_shared_public_checker_entry
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual shared output contract threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker command-surface threshold reaches the checker-adjacent helper floor for `p13-dice-late-join-missing-publication-witness`
actual_public_checker_boundary_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_boundary_threshold_manifest
  boundary_kind: none
  public_checker_command_surface_ref: none
  shared_output_contract_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_public_checker_boundary_threshold
  - compare_floor:current_l2.checker.public_checker_boundary
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_boundary.guard_only
  guard_refs:
  - guard:actual_public_checker_boundary_threshold_not_reached
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:query_token_and_checker_subject_public_naming
  - kept_later:generic_shared_public_checker_entry
  - kept_later:detached_loop_wrapper_path_line
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker boundary threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual shared output contract threshold reaches the checker-adjacent helper floor for `p13-dice-late-join-missing-publication-witness`
actual_verifier_handoff_surface_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_verifier_handoff_surface_threshold_manifest
  handoff_surface_kind: none
  public_checker_boundary_ref: none
  proof_obligation_matrix_ref: none
  handoff_artifact_mode: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_verifier_handoff_surface_threshold
  - compare_floor:current_l2.checker.verifier_handoff_surface
  compare_floor_refs:
  - compare_floor:current_l2.checker.verifier_handoff_surface.guard_only
  guard_refs:
  - guard:actual_verifier_handoff_surface_threshold_not_reached
  kept_later_refs:
  - kept_later:subject_rows
  - kept_later:theorem_protocol_runtime_dedicated_handoff_artifact_family
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:tool_specific_schema_and_actual_emitter_policy
  - kept_later:final_parser_grammar
  - kept_later:query_token_and_shared_generic_entry
  - kept_later:final_public_verifier_contract
  guard_reason: current actual verifier handoff surface threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker boundary threshold reaches the checker-adjacent helper floor for `p13-dice-late-join-missing-publication-witness`
actual_minimal_parser_subset_freeze_threshold:
  status: guarded_not_reached
  threshold_kind: parser_front_minimal_parser_subset_freeze_threshold_manifest
  freeze_kind: none
  accepted_cluster_refs: []
  reject_cluster_refs: []
  retention_floor_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_minimal_parser_subset_freeze_threshold
  - compare_floor:current_l2.parser.minimal_parser_subset_freeze
  compare_floor_refs:
  - compare_floor:current_l2.parser.minimal_parser_subset_freeze.guard_only
  guard_refs:
  - guard:actual_minimal_parser_subset_freeze_threshold_not_reached
  kept_later_refs:
  - kept_later:stage3_admit_slot_branch
  - kept_later:stage3_request_clause_branch
  - kept_later:stage3_predicate_fragment_branch
  - kept_later:public_parser_api
  - kept_later:final_parser_grammar
  - kept_later:parser_to_checker_reconnect_freeze
  - kept_later:final_public_parser_checker_api
  - kept_later:final_public_verifier_contract
  guard_reason: current actual minimal parser subset freeze threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual verifier handoff surface threshold reaches the helper-local docs-only bridge floor for `p13-dice-late-join-missing-publication-witness`
actual_parser_to_checker_reconnect_freeze_threshold:
  status: guarded_not_reached
  threshold_kind: parser_checker_bridge_reconnect_freeze_threshold_manifest
  reconnect_kind: none
  parser_subset_freeze_ref: none
  checker_floor_refs: []
  retained_helper_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_parser_to_checker_reconnect_freeze_threshold
  - compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze
  compare_floor_refs:
  - compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze.guard_only
  guard_refs:
  - guard:actual_parser_to_checker_reconnect_freeze_threshold_not_reached
  kept_later_refs:
  - kept_later:stage3_request_predicate_reconnect_line
  - kept_later:stage1_direct_target_mismatch_redesign_line
  - kept_later:runtime_contrast_e21_e22_line
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_api
  - kept_later:actual_external_verifier_schema
  - kept_later:final_public_verifier_contract
  guard_reason: current actual parser-to-checker reconnect freeze threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual minimal parser subset freeze threshold reaches the stage1+stage2 parser floor for `p13-dice-late-join-missing-publication-witness`
actual_phase1_semantics_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase1_semantics_closeout_threshold_manifest
  closeout_kind: none
  core_semantics_refs: []
  invariant_bridge_refs: []
  notation_status_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_phase1_semantics_closeout_threshold
  - compare_floor:current_l2.closeout.phase1_semantics_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase1_semantics_closeout.guard_only
  guard_refs:
  - guard:actual_phase1_semantics_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_reserved_keyword_and_punctuation
  - kept_later:final_type_system
  - kept_later:actual_external_verifier_schema
  - kept_later:actual_emitted_verifier_artifact
  - kept_later:final_public_verifier_contract
  guard_reason: current actual phase1 semantics closeout threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual parser-to-checker reconnect freeze threshold reaches the checker-floor bridge for `p13-dice-late-join-missing-publication-witness`
actual_phase2_parser_free_poc_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase2_parser_free_poc_closeout_threshold_manifest
  closeout_kind: none
  compile_gate_refs: []
  helper_boundary_refs: []
  detached_loop_policy_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_phase2_parser_free_poc_closeout_threshold
  - compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout.guard_only
  guard_refs:
  - guard:actual_phase2_parser_free_poc_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:reference_update_bless_workflow
  - kept_later:final_retention_path_policy
  - kept_later:public_exporter_api
  - kept_later:production_host_interface
  guard_reason: current actual phase2 parser-free PoC closeout threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual phase1 semantics closeout threshold reaches the semantics closeout floor for `p13-dice-late-join-missing-publication-witness`
actual_phase4_shared_space_self_driven_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase4_shared_space_self_driven_closeout_threshold_manifest
  closeout_kind: none
  current_package_refs: []
  user_spec_required_catalog_refs: []
  retained_later_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout.guard_only
  guard_refs:
  - guard:actual_phase4_shared_space_self_driven_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:final_public_witness_provider_artifact_contract
  - kept_later:exhaustive_shared_space_catalog
  - kept_later:control_plane_separated_carrier_actualization
  - kept_later:distributed_fairness_protocol
  - kept_later:final_operational_realization
  guard_reason: current actual phase4 shared-space self-driven closeout threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after the helper-local serial-scope reserve surface reaches the authoritative-room/delegated-provider floor for `p13-dice-late-join-missing-publication-witness`
actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_manifest
  closeout_kind: none
  verifier_handoff_surface_ref: none
  theorem_retained_bridge_stop_ref: none
  boundary_inventory_ref: none
  retained_later_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout.guard_only
  guard_refs:
  - guard:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:actual_subject_row_materialization
  - kept_later:boundary_specific_handoff_artifact_family
  - kept_later:actual_emitted_verifier_artifact
  - kept_later:concrete_tool_binding
  - kept_later:public_checker_migration
  - kept_later:low_level_memory_order_family
  guard_reason: current actual phase5 proof/protocol/runtime-policy handoff closeout threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase4 shared-space self-driven closeout threshold reaches the shared-space practical boundary checkpoint for `p13-dice-late-join-missing-publication-witness`
actual_phase6_actual_parser_ast_carrier_first_tranche_threshold:
  status: guarded_not_reached
  threshold_kind: phase6_actual_parser_ast_carrier_first_tranche_threshold_manifest
  carrier_kind: none
  accepted_surface_refs: []
  code_anchor_refs: []
  retained_later_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche.guard_only
  guard_refs:
  - guard:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_required
  - guard:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_not_reached
  kept_later_refs:
  - stage3_admit_slot_surface
  - stage3_request_clause_suite
  - stage3_predicate_fragment
  - perform_head_final_public_api
  - span_rich_diagnostics
  - final_grammar
  guard_reason: current actual phase6 parser / AST carrier first tranche threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase5 proof/protocol/runtime-policy handoff closeout threshold reaches the handoff stop line for `p13-dice-late-join-missing-publication-witness`
actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold:
  status: guarded_not_reached
  threshold_kind: phase6_actual_checker_runtime_skeleton_first_tranche_threshold_manifest
  skeleton_kind: none
  semantic_entry_refs: []
  runtime_bridge_refs: []
  parser_bridge_contract_refs: []
  retained_later_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche.guard_only
  guard_refs:
  - guard:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_required
  - guard:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_not_reached
  kept_later_refs:
  - parser_to_program_lowering
  - stage3_request_predicate_reconnect
  - richer_host_interface
  - final_public_runtime_checker_api
  - formal_hook_concrete_tool_binding
  guard_reason: current actual phase6 checker/runtime skeleton first tranche threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 parser / AST carrier first tranche threshold reaches the parser first-tranche minimum for `p13-dice-late-join-missing-publication-witness`
actual_phase6_compile_ready_verification_and_formal_hook_threshold:
  status: guarded_not_reached
  threshold_kind: phase6_compile_ready_verification_and_formal_hook_threshold_manifest
  verification_gate_refs: []
  smoke_gate_refs: []
  formal_hook_artifact_kind_ref: none
  formal_hook_subject_kind_refs: []
  formal_hook_contract_row_core_refs: []
  formal_hook_evidence_ref_family_refs: []
  formal_hook_obligation_kind_refs: []
  source_artifact_refs: []
  validation_refs: []
  retained_later_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook.guard_only
  guard_refs:
  - guard:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_required
  - guard:actual_phase6_compile_ready_verification_and_formal_hook_threshold_not_reached
  kept_later_refs:
  - concrete_theorem_tool_binding
  - concrete_model_check_tool_binding
  - parser_second_tranche_widen
  - final_public_surface
  guard_reason: current actual phase6 compile-ready verification / formal hook threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 checker/runtime skeleton first tranche threshold reaches the checker/runtime first-tranche minimum for `p13-dice-late-join-missing-publication-witness`
actual_phase6_next_reopen_sequencing_threshold:
  status: guarded_not_reached
  threshold_kind: phase6_next_reopen_sequencing_threshold_manifest
  sequencing_kind_ref: none
  fixed_entry_criteria_refs: []
  selected_first_reopen_ref: none
  deferred_reopen_refs: []
  minimum_guard_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_phase6_next_reopen_sequencing_threshold
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing.guard_only
  guard_refs:
  - guard:actual_phase6_compile_ready_verification_and_formal_hook_threshold_required
  - guard:actual_phase6_next_reopen_sequencing_threshold_not_reached
  kept_later_refs:
  - request_clause_suite_bulk_widen
  - perform_head_final_public_api
  - concrete_theorem_tool_binding
  - concrete_model_check_tool_binding
  - final_public_surface
  guard_reason: current actual phase6 next-reopen sequencing threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 compile-ready verification / formal hook threshold reaches the compile-ready minimum for `p13-dice-late-join-missing-publication-witness`
actual_phase6_reserve_formal_tool_binding_inventory_threshold:
  status: guarded_not_reached
  threshold_kind: phase6_reserve_formal_tool_binding_inventory_threshold_manifest
  inventory_kind: none
  fixed_entry_criteria_refs: []
  first_reserve_ref: none
  second_reserve_ref: none
  minimum_guard_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_phase6_reserve_formal_tool_binding_inventory_threshold
  - compare_floor:current_l2.closeout.phase6_reserve_formal_tool_binding_inventory
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase6_reserve_formal_tool_binding_inventory.guard_only
  guard_refs:
  - guard:actual_phase6_next_reopen_sequencing_threshold_required
  - guard:actual_phase6_reserve_formal_tool_binding_inventory_threshold_not_reached
  kept_later_refs:
  - concrete_theorem_tool_name
  - concrete_model_check_tool_name
  - actual_ci_artifact_retention_policy
  - parser_side_followup_package_selection
  - final_public_parser_checker_runtime_surface
  guard_reason: current actual phase6 reserve formal tool binding inventory threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 next-reopen sequencing threshold reaches the sequencing minimum for `p13-dice-late-join-missing-publication-witness`
actual_phase6_parser_side_followup_package_sequencing_threshold:
  status: guarded_not_reached
  threshold_kind: phase6_parser_side_followup_package_sequencing_threshold_manifest
  sequencing_kind: none
  fixed_entry_criteria_refs: []
  selected_next_package_ref: none
  deferred_reopen_refs: []
  minimum_guard_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p13-dice-late-join-missing-publication-witness
  - helper_preview:actual_phase6_parser_side_followup_package_sequencing_threshold
  - compare_floor:current_l2.closeout.phase6_parser_side_followup_package_sequencing
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase6_parser_side_followup_package_sequencing.guard_only
  guard_refs:
  - guard:actual_phase6_reserve_formal_tool_binding_inventory_threshold_required
  - guard:actual_phase6_parser_side_followup_package_sequencing_threshold_not_reached
  kept_later_refs:
  - request_clause_suite_publicization
  - perform_head_final_public_parser_api
  - span_rich_diagnostics
  - source_sample_corpus_scope
  - final_public_parser_checker_runtime_surface
  guard_reason: current actual phase6 parser-side follow-up package sequencing threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 reserve formal tool binding inventory threshold reaches the reserve-inventory minimum for `p13-dice-late-join-missing-publication-witness`
non_admissible_metadata: []
```

#### 出力の読み方

特に重要なのは次の行である。

- `static_gate: underdeclared`
  問題は「必要な根拠の不足」であり、静的に止まっている。
- `static_reasons:` の `missing publication witness before handoff for late-join visibility at root / room / dice_authority`
  何が足りないのかを具体的に言っている。late join visibility の前に publication witness が必要なのに、それが handoff より前に存在しない。
- `entered_evaluation: false`
  runtime evaluation に入っていない。
- `terminal_outcome: none`
  runtime 終端結果そのものが存在しない。なぜなら runtime に進んでいないからである。
- `steps_executed: 0`
  静的に止まったので step は 0 である。

ここで重要なのは、`verification_preview` がそれでも存在し、

- `formal_hook_status: reached`
- `subject_kind: fixture_static_cluster`

となっている点である。これは「失敗 sample でも formal hook preview があるのか」という初心者の疑問への答えになる。

答えは yes である。ただし subject は runtime cluster ではなく、**静的に reject された fixture cluster** になる。だから obligation も `canonical_normalization_law` と `no_re_promotion` になる。

- `canonical_normalization_law`
  この reject 形が正規化後も保たれるかを見る。
- `no_re_promotion`
  後段の tooling が、reject 済みサンプルを勝手に再昇格させないかを見る。

### 3-2. `p14`: publish より先に handoff している

#### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  run-source-sample \
  samples/prototype/current-l2-order-handoff/p14-dice-late-join-handoff-before-publication.txt \
  --format pretty
```

このコマンドが確認していること:

- publish 自体は書いてあっても、handoff が先に出てきたときに static stop するか
- その止め方が `malformed` になるか
- `underdeclared` と違って、構造破綻として止まるか

#### コード全文

```text
# publish は存在するが handoff より後ろにあり、late join visibility line を壊している helper-local negative prototype。
# current authoritative-room line では handoff-before-publish として static stop にする。
place root {
  place room {
    place dice_authority {
      perform handoff_dice_authority on dice_state
        require owner_is(player_a)
        ensure owner_is(player_b)

      atomic_cut

      perform publish_roll_result on dice_state
        require owner_is(player_a)
        ensure result_is_visible(room_members)

      atomic_cut

      perform observe_late_join_view on dice_state
        require read
        ensure late_join_sees_published_result(player_c)
    }
  }
}
```

#### 行ごとの解説

1. `# publish は存在するが handoff より後ろにあり、late join visibility line を壊している helper-local negative prototype。`
   問題点を先に言っている。publish 自体はあるが、位置が悪い。
2. `# current authoritative-room line では handoff-before-publish として static stop にする。`
   current line は、この順序違反を静的に止める方針だと明示している。
3. `place root {`
   root place。
4. `place room {`
   room place。
5. `place dice_authority {`
   authority place。
6. `perform handoff_dice_authority on dice_state`
   ここで最初に handoff してしまっている。これが current line の違反点である。
7. `require owner_is(player_a)`
   handoff 前に owner が `player_a` であることを要求している。
8. `ensure owner_is(player_b)`
   handoff 後に `player_b` になることを約束している。
9. 空行
   handoff 段と次段の区切り。
10. `atomic_cut`
    handoff を先に cut で確定してしまう。ここが publish より先に来ているので、後で publish を置いても current rule では順序 drift と読む。
11. 空行
    区切り。
12. `perform publish_roll_result on dice_state`
    publish 自体はここで現れる。しかし遅い。late join visibility の前提として必要なのは、「handoff より前の publish」である。
13. `require owner_is(player_a)`
    publish 前提として owner が `player_a` であることを要求している。しかしすでに 6 行目から 8 行目で owner 移譲を書いているので、読み筋が衝突する。
14. `ensure result_is_visible(room_members)`
    publish 成功後の visible を約束している。
15. 空行
    区切り。
16. `atomic_cut`
    publish 段をここで切るが、すでに handoff の後なので current line の因果順序は救われない。
17. 空行
    区切り。
18. `perform observe_late_join_view on dice_state`
    late join 観測を行う。
19. `require read`
    観測の read を要求している。
20. `ensure late_join_sees_published_result(player_c)`
    late join visibility を約束しているが、その前提となる publish の位置が current rule と衝突している。
21. `}`
    `dice_authority` を閉じる。
22. `}`
    `room` を閉じる。
23. `}`
    `root` を閉じる。

#### なぜ `malformed` なのか

`p13` では publish が存在しなかった。だから「根拠不足」で `underdeclared` だった。

しかし `p14` では publish 自体は存在する。問題は、**それが handoff より後ろに置かれていること**である。つまり部品不足ではなく、構造が current rule に反して壊れている。だから `malformed` になる。

初心者向けに具体例を言うと、これは次のような困り方をする。

- 先に authority を `player_b` に渡した
- あとから「さっきの結果は publish 済みだったことにしよう」と置く
- すると late joiner が読む履歴で、「誰の authority のもとで publish されたのか」が曖昧になる

current line は、この曖昧さを runtime bug にせず、設計段階で止めている。

#### 実行例全文

```text
shell: mir-current-l2
command: run-source-sample
sample: p14-dice-late-join-handoff-before-publication
sample_path: /home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-order-handoff/p14-dice-late-join-handoff-before-publication.txt
host_plan_path: /home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-order-handoff/p14-dice-late-join-handoff-before-publication.host-plan.json
static_gate: malformed
static_reasons:
- handoff appears before publish for late-join visibility at root / room / dice_authority
stage1_reconnect_clusters: not_available
stage2_try_rollback_verdict: not_available
entered_evaluation: false
terminal_outcome: none
steps_executed: 0
events:
debug_outputs: []
verification_preview:
  formal_hook_status: reached
  subject_kind: fixture_static_cluster
  subject_ref: p14-dice-late-join-handoff-before-publication
  proof_notebook_review_unit_obligations:
  - canonical_normalization_law
  - no_re_promotion
  model_check_concrete_carrier_obligations:
  - canonical_normalization_law
  - no_re_promotion
  guard_reason: none
artifact_preview:
  proof_notebook_review_units:
  - obligation_kind: canonical_normalization_law
    goal_text: Review that canonical normalization preserves the rejected static shape for `p14-dice-late-join-handoff-before-publication`.
    evidence_refs:
    - fixture:p14-dice-late-join-handoff-before-publication
    - static_gate_artifact:p14-dice-late-join-handoff-before-publication
  - obligation_kind: no_re_promotion
    goal_text: Review that `p14-dice-late-join-handoff-before-publication` remains rejected and is not re-promoted by later tooling.
    evidence_refs:
    - fixture:p14-dice-late-join-handoff-before-publication
  model_check_concrete_carriers:
  - obligation_kind: canonical_normalization_law
    evidence_refs:
    - fixture:p14-dice-late-join-handoff-before-publication
    - static_gate_artifact:p14-dice-late-join-handoff-before-publication
  - obligation_kind: no_re_promotion
    evidence_refs:
    - fixture:p14-dice-late-join-handoff-before-publication
surface_preview:
  minimal_companion:
    status: guarded_not_reached
    guard_reason: current minimal companion surface only actualizes reached authoritative-room defaults: current authoritative-room vertical slice only actualizes reached current default samples (`p07` / `p08`): current default samples (`p07` / `p08`) were not reached for `p14-dice-late-join-handoff-before-publication`
    lines: []
    compare_floor_refs:
    - compare_floor:current_l2.experimental_order_handoff_guard_only
    guard_refs:
    - guard:companion_surface_not_reached
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
  stage_block_secondary:
    status: guarded_not_reached
    guard_reason: current stage-block secondary surface only actualizes reached authoritative-room defaults: current authoritative-room vertical slice only actualizes reached current default samples (`p07` / `p08`): current default samples (`p07` / `p08`) were not reached for `p14-dice-late-join-handoff-before-publication`
    lines: []
    compare_floor_refs:
    - compare_floor:current_l2.experimental_stage_block_guard_only
    guard_refs:
    - guard:stage_block_surface_not_reached
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:authoritative_room_serial_scope_sugar
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
  serial_scope_reserve:
    status: guarded_not_reached
    guard_reason: current serial-scope reserve surface only actualizes authoritative-room-specific reached routes (`p07` / `p08` / `p09`): sample `p14-dice-late-join-handoff-before-publication` did not reach the authoritative-room serial-scope reserve surface
    lines: []
    compare_floor_refs:
    - compare_floor:current_l2.order_handoff.serial_scope_reserve_surface.guard_only
    guard_refs:
    - guard:serial_scope_reserve_surface_not_reached
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:serial_scope_public_promotion
    - kept_later:serial_scope_beyond_authoritative_room
    - kept_later:final_source_surface_handoff_wording
    - kept_later:final_emitted_artifact_schema
    - kept_later:final_emitted_handoff_contract
    - kept_later:final_public_witness_schema
    - kept_later:final_public_provider_receipt_schema
    - kept_later:combined_provider_witness_public_contract
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
authoritative_room_first_scenario_actual_adoption:
  status: guarded_not_reached
  adoption_kind: helper_local_authoritative_room_first_scenario_manifest
  profile_axis_refs: []
  relation_refs: []
  authority_handoff_refs: []
  runtime_evidence_refs: []
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p14-dice-late-join-handoff-before-publication:canonical_normalization_law
  - repo_local_emitted_artifact:proof_notebook_review_unit:p14-dice-late-join-handoff-before-publication:no_re_promotion
  - repo_local_emitted_artifact:model_check_concrete_carrier:p14-dice-late-join-handoff-before-publication:canonical_normalization_law
  - repo_local_emitted_artifact:model_check_concrete_carrier:p14-dice-late-join-handoff-before-publication:no_re_promotion
  reserve_route_refs: []
  negative_static_stop_refs:
  - negative_static_stop:p14-dice-late-join-handoff-before-publication:handoff_before_publish_for_late_join_visibility
  - negative_static_stop:p14-dice-late-join-handoff-before-publication:publish_then_handoff_then_observe_order_required
  contrast_refs:
  - contrast_target:append_friendly_notice_room
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:authoritative_room_first_scenario_actual_adoption
  - compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption
  compare_floor_refs:
  - compare_floor:current_l2.order_handoff.negative_static_stop_actualization
  - compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption.guard_only
  guard_refs:
  - guard:late_join_negative_static_stop
  - guard:first_scenario_pair_unchanged
  kept_later_refs:
  - kept_later:auditable_authority_witness
  - kept_later:delegated_rng_service
  - kept_later:distributed_randomness_provider
  - kept_later:final_emitted_handoff_contract
  - kept_later:exhaustive_shared_space_catalog
  - kept_later:final_consistency_fairness_catalog
  guard_reason: current authoritative-room first scenario keeps the late-join negative pair helper-local and guarded for `p14-dice-late-join-handoff-before-publication`: handoff-before-publish breaks late-join visibility; handoff appears before publish for late-join visibility at root / room / dice_authority
authoritative_room_reserve_strengthening_lane:
  status: guarded_not_reached
  lane_kind: helper_local_reserve_strengthening_lane_manifest
  witness_strengthening_status: guarded_not_reached
  delegated_rng_service_status: guarded_not_reached
  model_check_second_line_status: guarded_not_reached
  witness_strengthening_refs: []
  delegated_rng_service_refs: []
  model_check_second_line_refs: []
  first_line_boundary_refs:
  - first_line_boundary:representative_pair_kept_at_p07_p08
  - first_line_boundary:authoritative_room_default_profile_stays_principal
  - first_line_boundary:authority_rng_default_profile_unchanged
  reserve_boundary_refs:
  - reserve_boundary:auditable_authority_witness_second_strengthening
  - reserve_boundary:delegated_rng_service_second_practical
  - reserve_boundary:model_check_second_line_not_room_profile
  - reserve_boundary:public_checker_contract_kept_later
  - reserve_boundary:witness_provider_combined_public_contract_later
  repo_local_emitted_artifact_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.reserve_strengthening_lane.guard_only
  - compare_floor:current_l2.reserve_strengthening_lane:p14-dice-late-join-handoff-before-publication
  guard_refs:
  - guard:representative_reserve_strengthening_sample_set
  - guard:first_line_pair_unchanged
  kept_later_refs:
  - kept_later:combined_witness_provider_public_contract
  - kept_later:final_public_witness_schema
  - kept_later:final_public_provider_receipt_schema
  - kept_later:concrete_model_check_tool_brand
  - kept_later:actual_public_checker_migration
  - kept_later:distributed_fairness_theorem
  guard_reason: current authoritative-room reserve strengthening lane only actualizes the representative reserve strengthening sample set (`p07` witness, `p08` reconnect-model-check, `p09` delegated RNG); `p14-dice-late-join-handoff-before-publication` stays guard-only until one of those reserve routes is actually exercised
order_handoff_source_surface_artifact_actual_adoption:
  status: guarded_not_reached
  adoption_kind: helper_local_source_surface_artifact_route_manifest
  profile_axis_refs: []
  principal_surface_lines: []
  secondary_surface_lines: []
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p14-dice-late-join-handoff-before-publication:canonical_normalization_law
  - repo_local_emitted_artifact:proof_notebook_review_unit:p14-dice-late-join-handoff-before-publication:no_re_promotion
  - repo_local_emitted_artifact:model_check_concrete_carrier:p14-dice-late-join-handoff-before-publication:canonical_normalization_law
  - repo_local_emitted_artifact:model_check_concrete_carrier:p14-dice-late-join-handoff-before-publication:no_re_promotion
  source_wording_route_refs: []
  emitted_artifact_candidate_keep_refs: []
  negative_static_stop_refs:
  - negative_static_stop:p14-dice-late-join-handoff-before-publication:handoff_before_publish_for_late_join_visibility
  - negative_static_stop:p14-dice-late-join-handoff-before-publication:publish_then_handoff_then_observe_order_required
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:order_handoff_source_surface_artifact_actual_adoption
  - compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption
  compare_floor_refs:
  - compare_floor:current_l2.order_handoff.negative_static_stop_actualization
  - compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption.guard_only
  guard_refs:
  - guard:source_surface_artifact_actual_adoption_not_reached
  - guard:negative_static_stop_pair_kept_helper_local
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_runtime_api
  - kept_later:final_source_surface_handoff_wording
  - kept_later:final_emitted_artifact_schema
  - kept_later:final_emitted_handoff_contract
  - kept_later:final_public_witness_schema
  - kept_later:authoritative_room_serial_scope_sugar
  - kept_later:low_level_memory_order_source_surface
  - kept_later:final_modal_foundation_adoption
  guard_reason: current order-handoff source surface / emitted-artifact actual adoption keeps the late-join negative pair helper-local and guarded for `p14-dice-late-join-handoff-before-publication`: handoff appears before publish for late-join visibility at root / room / dice_authority
order_handoff_witness_provider_public_seam_compression:
  status: guarded_not_reached
  compression_kind: helper_local_public_seam_manifest
  profile_axis_refs: []
  repo_local_emitted_artifact_refs: []
  source_wording_route_refs: []
  emitted_artifact_candidate_keep_refs: []
  serial_scope_lines: []
  witness_schema_route_refs: []
  provider_receipt_route_refs: []
  combined_public_contract_keep_refs: []
  trace_alignment_pair_refs: []
  public_seam_residual_refs: []
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:order_handoff_witness_provider_public_seam_compression
  - compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression
  compare_floor_refs:
  - compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression.guard_only
  guard_refs:
  - guard:order_handoff_witness_provider_public_seam_compression_not_reached
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_runtime_api
  - kept_later:final_source_surface_handoff_wording
  - kept_later:final_emitted_artifact_schema
  - kept_later:final_public_witness_schema
  - kept_later:final_public_provider_receipt_schema
  - kept_later:delegated_provider_attestation
  - kept_later:combined_provider_witness_public_contract
  - kept_later:final_emitted_handoff_contract
  - kept_later:authoritative_room_serial_scope_sugar
  - kept_later:low_level_memory_order_source_surface
  - kept_later:final_modal_foundation_adoption
  - kept_later:exhaustive_shared_space_catalog
  guard_reason: current order-handoff/witness-provider public seam compression only actualizes the representative authoritative-room pair (`p07` / `p08`) after route/reserve/bridge/threshold floors align for `p14-dice-late-join-handoff-before-publication`
theorem_result_object_preview:
  status: guarded_not_reached
  preview_kind: helper_local_actualization_manifest
  subject_kind: fixture_static_cluster
  subject_ref: p14-dice-late-join-handoff-before-publication
  result_object_route_refs: []
  notebook_payload_preview_refs: []
  proof_object_schema_reserve_refs: []
  actual_adoption_default_refs: []
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:theorem_result_object_preview
  - compare_floor:current_l2.theorem_result_object_preview_actualization
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.theorem_result_object_preview.guard_only
  guard_refs:
  - guard:theorem_result_object_preview_not_reached
  kept_later_refs:
  - kept_later:final_public_theorem_result_object
  - kept_later:consumer_shaped_theorem_payload
  - kept_later:concrete_theorem_prover_brand
  - kept_later:proof_object_public_schema
  - kept_later:final_public_verifier_contract
  guard_reason: current theorem result-object preview only actualizes the representative theorem quartet (`e5` / `p06` / `p07` / `p08`) after verification preview reaches the formal-hook route for `p14-dice-late-join-handoff-before-publication`
model_check_public_checker_preview:
  status: guarded_not_reached
  preview_kind: helper_local_actualization_manifest
  subject_kind: fixture_static_cluster
  subject_ref: p14-dice-late-join-handoff-before-publication
  checker_artifact_preview_refs: []
  verifier_handoff_reserve_refs: []
  tool_binding_reserve_refs: []
  actual_adoption_default_refs: []
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:model_check_public_checker_preview
  - compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.model_check.public_checker_artifact_preview.guard_only
  guard_refs:
  - guard:model_check_public_checker_artifact_preview_not_reached
  kept_later_refs:
  - kept_later:first_settled_property_language
  - kept_later:concrete_model_check_tool_brand
  - kept_later:final_public_checker_artifact
  - kept_later:actual_public_checker_migration
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:production_checker_runtime_policy_contract
  - kept_later:final_public_verifier_contract
  guard_reason: current model-check public-checker preview only actualizes the representative checker quartet (`e5` / `p06` / `p07` / `p09`) after verification preview reaches the formal-hook route for `p14-dice-late-join-handoff-before-publication`
theorem_final_public_contract_reopen_threshold:
  status: guarded_not_reached
  threshold_kind: helper_local_reopen_threshold_manifest
  subject_kind: fixture_static_cluster
  subject_ref: p14-dice-late-join-handoff-before-publication
  result_object_route_refs: []
  payload_preview_keep_refs: []
  proof_object_schema_candidate_refs: []
  prover_brand_candidate_refs: []
  final_public_contract_reopen_sequence_refs: []
  threshold_default_refs: []
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:theorem_final_public_contract_reopen_threshold
  - compare_floor:current_l2.theorem_final_public_contract_reopen_threshold
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.theorem_final_public_contract_reopen_threshold.guard_only
  guard_refs:
  - guard:theorem_final_public_contract_reopen_threshold_not_reached
  kept_later_refs:
  - kept_later:final_public_theorem_result_object
  - kept_later:consumer_shaped_theorem_payload
  - kept_later:concrete_theorem_prover_brand
  - kept_later:proof_object_public_schema
  - kept_later:final_public_verifier_contract
  guard_reason: current theorem final public-contract reopen threshold only actualizes the representative theorem quartet (`e5` / `p06` / `p07` / `p08`) after verification preview reaches the formal-hook route for `p14-dice-late-join-handoff-before-publication`
model_check_final_public_contract_reopen_threshold:
  status: guarded_not_reached
  threshold_kind: helper_local_reopen_threshold_manifest
  subject_kind: fixture_static_cluster
  subject_ref: p14-dice-late-join-handoff-before-publication
  checker_artifact_route_refs: []
  migration_candidate_keep_refs: []
  verifier_handoff_candidate_refs: []
  tool_brand_candidate_refs: []
  final_public_contract_reopen_sequence_refs: []
  threshold_default_refs: []
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:model_check_final_public_contract_reopen_threshold
  - compare_floor:current_l2.model_check.final_public_contract_reopen_threshold
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.model_check.final_public_contract_reopen_threshold.guard_only
  guard_refs:
  - guard:model_check_final_public_contract_reopen_threshold_not_reached
  kept_later_refs:
  - kept_later:first_settled_property_language
  - kept_later:concrete_model_check_tool_brand
  - kept_later:final_public_checker_artifact
  - kept_later:actual_public_checker_migration
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:production_checker_runtime_policy_contract
  - kept_later:final_public_verifier_contract
  guard_reason: current model-check final public-contract reopen threshold only actualizes the representative checker quartet (`e5` / `p06` / `p07` / `p09`) after verification preview reaches the formal-hook route for `p14-dice-late-join-handoff-before-publication`
typed_checker_hint_preview:
  status: guarded_not_reached
  preview_kind: sample_local_helper_preview
  cluster_kind: none
  case_label: none
  typed_reason_family_hint: none
  evidence_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.typed.sample_local_checker_hint_guard_only
  guard_refs:
  - guard:typed_checker_hint_preview_not_reached
  kept_later_refs:
  - kept_later:final_typed_source_principal
  - kept_later:final_finite_index_surface
  - kept_later:final_ifc_syntax
  - kept_later:actual_checker_payload_family
  - kept_later:checker_supported_kind_summary
  - kept_later:final_public_verifier_contract
  guard_reason: current typed checker-hint preview only actualizes the sample-local first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after verification preview reaches runtime try-cut evidence for `p14-dice-late-join-handoff-before-publication`
actual_checker_payload_family_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_payload_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_family_kind: none
  source_refs: []
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_checker_payload_family_threshold
  - compare_floor:current_l2.checker.actual_checker_payload_family
  compare_floor_refs:
  - compare_floor:current_l2.checker.actual_checker_payload_family.guard_only
  guard_refs:
  - guard:actual_checker_payload_family_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_family
  - kept_later:checker_payload_row_detail
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload family threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after typed checker-hint preview reaches the checker-adjacent helper floor for `p14-dice-late-join-handoff-before-publication`
actual_checker_payload_row_family_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_family_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_family_ref: none
  row_family_kind: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_checker_payload_row_family_threshold
  - compare_floor:current_l2.checker.checker_payload_row_family
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_family.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_family_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_detail
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-family threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload family threshold reaches the checker-adjacent helper floor for `p14-dice-late-join-handoff-before-publication`
actual_checker_payload_row_detail_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_detail_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_row_family_ref: none
  row_source_ref: none
  row_reason_kind: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_checker_payload_row_detail_threshold
  - compare_floor:current_l2.checker.checker_payload_row_detail
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_detail.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_detail_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-detail threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-family threshold reaches the checker-adjacent helper floor for `p14-dice-late-join-handoff-before-publication`
actual_checker_payload_row_body_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_body_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_row_family_ref: none
  row_source_ref: none
  row_reason_kind: none
  row_body: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_checker_payload_row_body_threshold
  - compare_floor:current_l2.checker.checker_payload_row_body
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_body.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_body_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-body threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-detail threshold reaches the checker-adjacent helper floor for `p14-dice-late-join-handoff-before-publication`
actual_checker_payload_supported_kind_summary_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_supported_kind_summary_threshold_manifest
  payload_row_family_ref: none
  supported_kind_scope: none
  supported_kind_refs: []
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_checker_payload_supported_kind_summary_threshold
  - compare_floor:current_l2.checker.checker_payload_supported_kind_summary
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_supported_kind_summary.guard_only
  guard_refs:
  - guard:actual_checker_payload_supported_kind_summary_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload supported-kind summary threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-body threshold reaches the checker-adjacent helper floor for `p14-dice-late-join-handoff-before-publication`
actual_checker_payload_public_schema_sketch_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_payload_schema_sketch_threshold_manifest
  actual_checker_payload_family_ref: none
  checker_payload_row_family_ref: none
  checker_payload_row_detail_ref: none
  checker_payload_row_body_ref: none
  checker_payload_supported_kind_summary_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_checker_payload_public_schema_sketch_threshold
  - compare_floor:current_l2.checker.public_checker_payload_schema
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_payload_schema.guard_only
  guard_refs:
  - guard:actual_checker_payload_public_schema_sketch_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_api
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload public-schema sketch threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload supported-kind summary threshold reaches the checker-adjacent helper floor for `p14-dice-late-join-handoff-before-publication`
actual_public_checker_api_sketch_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_api_sketch_threshold_manifest
  checker_subject: none
  public_checker_payload_schema_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_public_checker_api_sketch_threshold
  - compare_floor:current_l2.checker.public_checker_api
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_api.guard_only
  guard_refs:
  - guard:actual_public_checker_api_sketch_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_entry_criteria
  - kept_later:public_checker_command_surface
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker API sketch threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload public-schema sketch threshold reaches the checker-adjacent helper floor for `p14-dice-late-join-handoff-before-publication`
actual_public_checker_entry_criteria_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_entry_criteria_threshold_manifest
  public_checker_api_ref: none
  entry_criteria_refs: []
  family_facade_support_ref: none
  family_facade_script_refs: []
  smoke_command_refs: []
  next_comparison_target_ref: none
  deferred_boundary_refs: []
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_public_checker_entry_criteria_threshold
  - compare_floor:current_l2.checker.public_checker_entry_criteria
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_entry_criteria.guard_only
  guard_refs:
  - guard:actual_public_checker_entry_criteria_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_command_surface
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker entry-criteria threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker API sketch threshold reaches the checker-adjacent helper floor for `p14-dice-late-join-handoff-before-publication`
actual_public_checker_command_surface_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_command_surface_threshold_manifest
  command_surface_kind: none
  family_facade_command_refs: []
  public_checker_api_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_public_checker_command_surface_threshold
  - compare_floor:current_l2.checker.public_checker_command_surface
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_command_surface.guard_only
  guard_refs:
  - guard:actual_public_checker_command_surface_threshold_not_reached
  kept_later_refs:
  - kept_later:detached_loop_smoke_wrapper
  - kept_later:generic_shared_public_checker_entry
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker command-surface threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker entry-criteria threshold reaches the checker-adjacent helper floor for `p14-dice-late-join-handoff-before-publication`
actual_shared_output_contract_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_shared_output_contract_threshold_manifest
  output_contract_kind: none
  checker_cluster_name: none
  checker_status: none
  public_checker_payload_schema_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_shared_output_contract_threshold
  - compare_floor:current_l2.checker.shared_output_contract
  compare_floor_refs:
  - compare_floor:current_l2.checker.shared_output_contract.guard_only
  guard_refs:
  - guard:actual_shared_output_contract_threshold_not_reached
  kept_later_refs:
  - kept_later:detached_loop_wrapper_paths
  - kept_later:fixture_and_actual_rows_textual_rendering
  - kept_later:generic_shared_public_checker_entry
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual shared output contract threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker command-surface threshold reaches the checker-adjacent helper floor for `p14-dice-late-join-handoff-before-publication`
actual_public_checker_boundary_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_boundary_threshold_manifest
  boundary_kind: none
  public_checker_command_surface_ref: none
  shared_output_contract_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_public_checker_boundary_threshold
  - compare_floor:current_l2.checker.public_checker_boundary
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_boundary.guard_only
  guard_refs:
  - guard:actual_public_checker_boundary_threshold_not_reached
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:query_token_and_checker_subject_public_naming
  - kept_later:generic_shared_public_checker_entry
  - kept_later:detached_loop_wrapper_path_line
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker boundary threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual shared output contract threshold reaches the checker-adjacent helper floor for `p14-dice-late-join-handoff-before-publication`
actual_verifier_handoff_surface_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_verifier_handoff_surface_threshold_manifest
  handoff_surface_kind: none
  public_checker_boundary_ref: none
  proof_obligation_matrix_ref: none
  handoff_artifact_mode: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_verifier_handoff_surface_threshold
  - compare_floor:current_l2.checker.verifier_handoff_surface
  compare_floor_refs:
  - compare_floor:current_l2.checker.verifier_handoff_surface.guard_only
  guard_refs:
  - guard:actual_verifier_handoff_surface_threshold_not_reached
  kept_later_refs:
  - kept_later:subject_rows
  - kept_later:theorem_protocol_runtime_dedicated_handoff_artifact_family
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:tool_specific_schema_and_actual_emitter_policy
  - kept_later:final_parser_grammar
  - kept_later:query_token_and_shared_generic_entry
  - kept_later:final_public_verifier_contract
  guard_reason: current actual verifier handoff surface threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker boundary threshold reaches the checker-adjacent helper floor for `p14-dice-late-join-handoff-before-publication`
actual_minimal_parser_subset_freeze_threshold:
  status: guarded_not_reached
  threshold_kind: parser_front_minimal_parser_subset_freeze_threshold_manifest
  freeze_kind: none
  accepted_cluster_refs: []
  reject_cluster_refs: []
  retention_floor_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_minimal_parser_subset_freeze_threshold
  - compare_floor:current_l2.parser.minimal_parser_subset_freeze
  compare_floor_refs:
  - compare_floor:current_l2.parser.minimal_parser_subset_freeze.guard_only
  guard_refs:
  - guard:actual_minimal_parser_subset_freeze_threshold_not_reached
  kept_later_refs:
  - kept_later:stage3_admit_slot_branch
  - kept_later:stage3_request_clause_branch
  - kept_later:stage3_predicate_fragment_branch
  - kept_later:public_parser_api
  - kept_later:final_parser_grammar
  - kept_later:parser_to_checker_reconnect_freeze
  - kept_later:final_public_parser_checker_api
  - kept_later:final_public_verifier_contract
  guard_reason: current actual minimal parser subset freeze threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual verifier handoff surface threshold reaches the helper-local docs-only bridge floor for `p14-dice-late-join-handoff-before-publication`
actual_parser_to_checker_reconnect_freeze_threshold:
  status: guarded_not_reached
  threshold_kind: parser_checker_bridge_reconnect_freeze_threshold_manifest
  reconnect_kind: none
  parser_subset_freeze_ref: none
  checker_floor_refs: []
  retained_helper_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_parser_to_checker_reconnect_freeze_threshold
  - compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze
  compare_floor_refs:
  - compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze.guard_only
  guard_refs:
  - guard:actual_parser_to_checker_reconnect_freeze_threshold_not_reached
  kept_later_refs:
  - kept_later:stage3_request_predicate_reconnect_line
  - kept_later:stage1_direct_target_mismatch_redesign_line
  - kept_later:runtime_contrast_e21_e22_line
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_api
  - kept_later:actual_external_verifier_schema
  - kept_later:final_public_verifier_contract
  guard_reason: current actual parser-to-checker reconnect freeze threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual minimal parser subset freeze threshold reaches the stage1+stage2 parser floor for `p14-dice-late-join-handoff-before-publication`
actual_phase1_semantics_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase1_semantics_closeout_threshold_manifest
  closeout_kind: none
  core_semantics_refs: []
  invariant_bridge_refs: []
  notation_status_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_phase1_semantics_closeout_threshold
  - compare_floor:current_l2.closeout.phase1_semantics_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase1_semantics_closeout.guard_only
  guard_refs:
  - guard:actual_phase1_semantics_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_reserved_keyword_and_punctuation
  - kept_later:final_type_system
  - kept_later:actual_external_verifier_schema
  - kept_later:actual_emitted_verifier_artifact
  - kept_later:final_public_verifier_contract
  guard_reason: current actual phase1 semantics closeout threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual parser-to-checker reconnect freeze threshold reaches the checker-floor bridge for `p14-dice-late-join-handoff-before-publication`
actual_phase2_parser_free_poc_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase2_parser_free_poc_closeout_threshold_manifest
  closeout_kind: none
  compile_gate_refs: []
  helper_boundary_refs: []
  detached_loop_policy_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_phase2_parser_free_poc_closeout_threshold
  - compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout.guard_only
  guard_refs:
  - guard:actual_phase2_parser_free_poc_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:reference_update_bless_workflow
  - kept_later:final_retention_path_policy
  - kept_later:public_exporter_api
  - kept_later:production_host_interface
  guard_reason: current actual phase2 parser-free PoC closeout threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual phase1 semantics closeout threshold reaches the semantics closeout floor for `p14-dice-late-join-handoff-before-publication`
actual_phase4_shared_space_self_driven_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase4_shared_space_self_driven_closeout_threshold_manifest
  closeout_kind: none
  current_package_refs: []
  user_spec_required_catalog_refs: []
  retained_later_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout.guard_only
  guard_refs:
  - guard:actual_phase4_shared_space_self_driven_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:final_public_witness_provider_artifact_contract
  - kept_later:exhaustive_shared_space_catalog
  - kept_later:control_plane_separated_carrier_actualization
  - kept_later:distributed_fairness_protocol
  - kept_later:final_operational_realization
  guard_reason: current actual phase4 shared-space self-driven closeout threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after the helper-local serial-scope reserve surface reaches the authoritative-room/delegated-provider floor for `p14-dice-late-join-handoff-before-publication`
actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_manifest
  closeout_kind: none
  verifier_handoff_surface_ref: none
  theorem_retained_bridge_stop_ref: none
  boundary_inventory_ref: none
  retained_later_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout.guard_only
  guard_refs:
  - guard:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:actual_subject_row_materialization
  - kept_later:boundary_specific_handoff_artifact_family
  - kept_later:actual_emitted_verifier_artifact
  - kept_later:concrete_tool_binding
  - kept_later:public_checker_migration
  - kept_later:low_level_memory_order_family
  guard_reason: current actual phase5 proof/protocol/runtime-policy handoff closeout threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase4 shared-space self-driven closeout threshold reaches the shared-space practical boundary checkpoint for `p14-dice-late-join-handoff-before-publication`
actual_phase6_actual_parser_ast_carrier_first_tranche_threshold:
  status: guarded_not_reached
  threshold_kind: phase6_actual_parser_ast_carrier_first_tranche_threshold_manifest
  carrier_kind: none
  accepted_surface_refs: []
  code_anchor_refs: []
  retained_later_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche.guard_only
  guard_refs:
  - guard:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_required
  - guard:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_not_reached
  kept_later_refs:
  - stage3_admit_slot_surface
  - stage3_request_clause_suite
  - stage3_predicate_fragment
  - perform_head_final_public_api
  - span_rich_diagnostics
  - final_grammar
  guard_reason: current actual phase6 parser / AST carrier first tranche threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase5 proof/protocol/runtime-policy handoff closeout threshold reaches the handoff stop line for `p14-dice-late-join-handoff-before-publication`
actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold:
  status: guarded_not_reached
  threshold_kind: phase6_actual_checker_runtime_skeleton_first_tranche_threshold_manifest
  skeleton_kind: none
  semantic_entry_refs: []
  runtime_bridge_refs: []
  parser_bridge_contract_refs: []
  retained_later_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche.guard_only
  guard_refs:
  - guard:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_required
  - guard:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_not_reached
  kept_later_refs:
  - parser_to_program_lowering
  - stage3_request_predicate_reconnect
  - richer_host_interface
  - final_public_runtime_checker_api
  - formal_hook_concrete_tool_binding
  guard_reason: current actual phase6 checker/runtime skeleton first tranche threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 parser / AST carrier first tranche threshold reaches the parser first-tranche minimum for `p14-dice-late-join-handoff-before-publication`
actual_phase6_compile_ready_verification_and_formal_hook_threshold:
  status: guarded_not_reached
  threshold_kind: phase6_compile_ready_verification_and_formal_hook_threshold_manifest
  verification_gate_refs: []
  smoke_gate_refs: []
  formal_hook_artifact_kind_ref: none
  formal_hook_subject_kind_refs: []
  formal_hook_contract_row_core_refs: []
  formal_hook_evidence_ref_family_refs: []
  formal_hook_obligation_kind_refs: []
  source_artifact_refs: []
  validation_refs: []
  retained_later_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook.guard_only
  guard_refs:
  - guard:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_required
  - guard:actual_phase6_compile_ready_verification_and_formal_hook_threshold_not_reached
  kept_later_refs:
  - concrete_theorem_tool_binding
  - concrete_model_check_tool_binding
  - parser_second_tranche_widen
  - final_public_surface
  guard_reason: current actual phase6 compile-ready verification / formal hook threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 checker/runtime skeleton first tranche threshold reaches the checker/runtime first-tranche minimum for `p14-dice-late-join-handoff-before-publication`
actual_phase6_next_reopen_sequencing_threshold:
  status: guarded_not_reached
  threshold_kind: phase6_next_reopen_sequencing_threshold_manifest
  sequencing_kind_ref: none
  fixed_entry_criteria_refs: []
  selected_first_reopen_ref: none
  deferred_reopen_refs: []
  minimum_guard_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_phase6_next_reopen_sequencing_threshold
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing.guard_only
  guard_refs:
  - guard:actual_phase6_compile_ready_verification_and_formal_hook_threshold_required
  - guard:actual_phase6_next_reopen_sequencing_threshold_not_reached
  kept_later_refs:
  - request_clause_suite_bulk_widen
  - perform_head_final_public_api
  - concrete_theorem_tool_binding
  - concrete_model_check_tool_binding
  - final_public_surface
  guard_reason: current actual phase6 next-reopen sequencing threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 compile-ready verification / formal hook threshold reaches the compile-ready minimum for `p14-dice-late-join-handoff-before-publication`
actual_phase6_reserve_formal_tool_binding_inventory_threshold:
  status: guarded_not_reached
  threshold_kind: phase6_reserve_formal_tool_binding_inventory_threshold_manifest
  inventory_kind: none
  fixed_entry_criteria_refs: []
  first_reserve_ref: none
  second_reserve_ref: none
  minimum_guard_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_phase6_reserve_formal_tool_binding_inventory_threshold
  - compare_floor:current_l2.closeout.phase6_reserve_formal_tool_binding_inventory
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase6_reserve_formal_tool_binding_inventory.guard_only
  guard_refs:
  - guard:actual_phase6_next_reopen_sequencing_threshold_required
  - guard:actual_phase6_reserve_formal_tool_binding_inventory_threshold_not_reached
  kept_later_refs:
  - concrete_theorem_tool_name
  - concrete_model_check_tool_name
  - actual_ci_artifact_retention_policy
  - parser_side_followup_package_selection
  - final_public_parser_checker_runtime_surface
  guard_reason: current actual phase6 reserve formal tool binding inventory threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 next-reopen sequencing threshold reaches the sequencing minimum for `p14-dice-late-join-handoff-before-publication`
actual_phase6_parser_side_followup_package_sequencing_threshold:
  status: guarded_not_reached
  threshold_kind: phase6_parser_side_followup_package_sequencing_threshold_manifest
  sequencing_kind: none
  fixed_entry_criteria_refs: []
  selected_next_package_ref: none
  deferred_reopen_refs: []
  minimum_guard_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p14-dice-late-join-handoff-before-publication
  - helper_preview:actual_phase6_parser_side_followup_package_sequencing_threshold
  - compare_floor:current_l2.closeout.phase6_parser_side_followup_package_sequencing
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase6_parser_side_followup_package_sequencing.guard_only
  guard_refs:
  - guard:actual_phase6_reserve_formal_tool_binding_inventory_threshold_required
  - guard:actual_phase6_parser_side_followup_package_sequencing_threshold_not_reached
  kept_later_refs:
  - request_clause_suite_publicization
  - perform_head_final_public_parser_api
  - span_rich_diagnostics
  - source_sample_corpus_scope
  - final_public_parser_checker_runtime_surface
  guard_reason: current actual phase6 parser-side follow-up package sequencing threshold only actualizes the representative shared-space trio (`p07` / `p08` / `p09`) after actual phase6 reserve formal tool binding inventory threshold reaches the reserve-inventory minimum for `p14-dice-late-join-handoff-before-publication`
non_admissible_metadata: []
```

#### 出力の読み方

重要な行は次である。

- `static_gate: malformed`
  これは根拠不足ではなく、構造破綻として止まっている。
- `static_reasons:` の `handoff appears before publish for late-join visibility at root / room / dice_authority`
  問題点をそのまま言っている。late join visibility のためには publish が先でなければならないのに、handoff が先に出ている。
- `entered_evaluation: false`
  runtime に入っていない。
- `terminal_outcome: none`
  runtime 終端結果は存在しない。
- `steps_executed: 0`
  静的 stop なので step は 0 である。

ここでも `formal_hook_status: reached` かつ `subject_kind: fixture_static_cluster` である。つまり `p13` と同様、runtime 成功ではなく **静的 reject cluster に対する formal hook preview** が作られている。

## 4. representative / reserve / negative pair をまとめて materialize する

次は helper で Problem 2 全体を 1 つの output dir に materialize する。

```bash
python3 scripts/current_l2_guided_samples.py emit-scenario problem2
```

このコマンドが確認していること:

- `p07 / p08` が first-line representative pair として reached しているか
- `p09` が reserve practical route として reached しているか
- `p13 / p14` が negative static-stop pair として読めるか
- 同じ output dir に run report を並べて current cut をまとめ直せるか

実行例全文:

```text
Problem 2 authoritative-room runnable scenario loop

Problem 2 first-line / reserve / negative pair を repo-local output dir に materialize し、authoritative-room current default を runnable scenario bundle として再確認する helper。

sample bundle doc: samples/problem-bundles/problem2-order-handoff-shared-space.md
command: python3 scripts/current_l2_guided_samples.py emit-scenario problem2
output dir: target/current-l2-guided/problem2-scenario-bundle

- p07-dice-late-join-visible-history: first-line representative
  output: target/current-l2-guided/problem2-scenario-bundle/p07-dice-late-join-visible-history.run.json
  static_gate: valid
  terminal_outcome: success
  first_line_status: reached
  reserve_lane_status: reached

- p08-dice-stale-reconnect-refresh: first-line representative
  output: target/current-l2-guided/problem2-scenario-bundle/p08-dice-stale-reconnect-refresh.run.json
  static_gate: valid
  terminal_outcome: success
  first_line_status: reached
  reserve_lane_status: reached

- p09-dice-delegated-rng-provider-placement: reserve practical route
  output: target/current-l2-guided/problem2-scenario-bundle/p09-dice-delegated-rng-provider-placement.run.json
  static_gate: valid
  terminal_outcome: success
  first_line_status: guarded
  reserve_lane_status: reached

- p13-dice-late-join-missing-publication-witness: negative static-stop
  output: target/current-l2-guided/problem2-scenario-bundle/p13-dice-late-join-missing-publication-witness.run.json
  static_gate: underdeclared
  terminal_outcome: none
  first_line_status: guarded
  reserve_lane_status: guarded

- p14-dice-late-join-handoff-before-publication: negative static-stop
  output: target/current-l2-guided/problem2-scenario-bundle/p14-dice-late-join-handoff-before-publication.run.json
  static_gate: malformed
  terminal_outcome: none
  first_line_status: guarded
  reserve_lane_status: guarded

stop line:
- final public witness schema
- final public provider receipt schema
- final public witness/provider/artifact contract
- exhaustive shared-space catalog

注意:
- `p07 / p08` representative pair、`p09` reserve route、`p13 / p14` negative pair を同じ output dir に materialize する narrow helper である。
- final source wording、final public witness/provider/artifact contract、exhaustive shared-space catalog には上げない。
```

### この helper 出力の読み方

ここで特に重要なのは、各 sample を 3 種類に分けて読んでいる点である。

- `first-line representative`
  current default profile の本線。`p07`, `p08` がここに入る。
- `reserve practical route`
  本線には上げないが、current reserve として保持する practical route。`p09` がここに入る。
- `negative static-stop`
  static gate で止めるべき negative pair。`p13`, `p14` がここに入る。

さらに各 status の意味は次のとおりである。

- `first_line_status`
  authoritative-room first default の current first line に達しているか。
  - `reached` なら本線に乗っている。
  - `guarded` なら本線には昇格していない。
- `reserve_lane_status`
  reserve strengthening lane まで含めて読めるか。
  - `reached` なら reserve lane には届いている。
  - `guarded` なら reserve lane にも入っていない。

ここで `p09` が

- `first_line_status: guarded`
- `reserve_lane_status: reached`

となっているのが重要である。これは「成功しているから first line だ」とは読まない、という current policy を示している。`p09` は成功 sample だが、**reserve route の成功**であって、representative pair の成功ではない。

## 4.5. reserve practical route `p09` を単独で見る

`order_01.md` は `p09` を first-line representative ではなく reserve practical route として紹介している。しかし、コードそのものを読まないと「何が reserve で、何が authority 側に残るのか」が分かりにくい。そこでここでは `p09` を単独で読む。

### 実行コマンド

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  run-source-sample \
  samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt \
  --format pretty
```

このコマンドが確認していること:

- external provider から draw を取る route が current sample として動くか
- その後の publish と handoff を authority 側に残しているか
- success しても first-line representative には昇格せず、reserve route として保持されているか

### コード全文

```text
# delegated RNG provider が draw を返すが commit と handoff は authority が保持する current L2 prototype。
place root {
  place room {
    place dice_authority {
      option delegated_rng on dice_state capability read lease live admit provider_online(room_epoch)

      chain delegated_rng_ref = delegated_rng

      perform fetch_provider_roll via delegated_rng_ref
        require read
        ensure provider_draw_available(primary)

      atomic_cut

      perform publish_roll_result on dice_state
        require owner_is(player_a)
        ensure result_is_visible(room_members)

      atomic_cut

      perform handoff_dice_authority on dice_state
        require owner_is(player_a)
        ensure owner_is(player_b)
    }
  }
}
```

### 行ごとの解説

1. `# delegated RNG provider が draw を返すが commit と handoff は authority が保持する current L2 prototype。`
   この sample の要点を一行で言っている。大事なのは、「provider が draw を返す」と「authority が commit と handoff を保持する」を分けることだ。
2. `place root {`
   root place。
3. `place room {`
   room place。
4. `place dice_authority {`
   authority place。provider を attach しても、room state の中心はここに残る。
5. `option delegated_rng on dice_state capability read lease live admit provider_online(room_epoch)`
   `delegated_rng` という option を宣言している。`on dice_state` はこの option が `dice_state` に関係することを示す。`capability read` は read 系 capability、`lease live` は今この room epoch で生きている attachable route と読むとよい。`admit provider_online(room_epoch)` は provider が現在 online であることを admit 条件としている。
6. 空行
   option 宣言と後続の使用を区切る空行。
7. `chain delegated_rng_ref = delegated_rng`
   5 行目で宣言した option を、後続で参照する chain ref に束ねている。初心者向けには「後で `via delegated_rng_ref` と書けるように名前を付け直している」と読むとよい。
8. 空行
   chain 宣言と perform を区切る空行。
9. `perform fetch_provider_roll via delegated_rng_ref`
   provider から roll を取得する。ここが `on dice_state` ではなく `via delegated_rng_ref` なのが重要で、state そのものを provider に渡して commit させているのではなく、provider route を経由して draw を取っている。
10. `require read`
    provider draw の取得前提として read を要求している。
11. `ensure provider_draw_available(primary)`
    provider 側から primary draw が利用可能になることを約束している。ここではまだ publish も handoff もしていない。
12. 空行
    provider draw 段と publish 段の区切り。
13. `atomic_cut`
    provider draw 取得段を次の publish 段と切り分ける。ここでも「provider draw を返すこと」と「room state を publish すること」を混ぜない。
14. 空行
    区切り。
15. `perform publish_roll_result on dice_state`
    ここで publish を authority 側が行う。つまり provider は draw を返したが、publish 自体は authority 側の責務である。
16. `require owner_is(player_a)`
    publish 前に owner が `player_a` であることを要求している。
17. `ensure result_is_visible(room_members)`
    publish 後に結果が room members から見えることを約束している。
18. 空行
    区切り。
19. `atomic_cut`
    publish 段を handoff 段と切り分ける。
20. 空行
    区切り。
21. `perform handoff_dice_authority on dice_state`
    authority handoff を行う。commit と handoff は provider ではなく authority 側に残っている、という current reading がここで実コードになっている。
22. `require owner_is(player_a)`
    handoff 前にまだ `player_a` owner であることを要求する。
23. `ensure owner_is(player_b)`
    handoff 後に `player_b` owner になることを約束する。
24. `}`
    `dice_authority` を閉じる。
25. `}`
    `room` を閉じる。
26. `}`
    `root` を閉じる。

### どこが reserve practical route なのか

`p09` が reserve route と呼ばれる理由は、success しているのに first-line representative pair には昇格していないからである。current line は `p07 / p08` を first line に保ち、`p09` は「外部 provider を attach した practical route」として分離している。

ここで守りたい境界は次の通りである。

- provider は draw を返す
- authority は publish を行う
- authority は handoff を行う

この順序制約が必要な理由を具体例で言う。

- もし provider が draw だけでなく room state の commit まで持つと、authority handoff の責任境界が崩れる
- すると「誰が room の事実を確定したのか」が不透明になり、publish witness や later audit の意味が弱くなる

### 実行例全文

```text
shell: mir-current-l2
command: run-source-sample
sample: p09-dice-delegated-rng-provider-placement
sample_path: /home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt
host_plan_path: /home/yukatayu/dev/mir_poc_01/samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.host-plan.json
static_gate: valid
stage1_reconnect_clusters: not_available
stage2_try_rollback_verdict: not_available
entered_evaluation: true
terminal_outcome: success
steps_executed: 11
events:
- perform-success
- atomic-cut
- perform-success
- atomic-cut
- perform-success
debug_outputs:
- dice_debug_text_output:
  - publish_roll_result: player_a -> visible
  - handoff_dice_authority: player_a -> player_b
- provider_debug_text_output:
  - delegated_rng_service.draw -> primary
verification_preview:
  formal_hook_status: reached
  subject_kind: runtime_try_cut_cluster
  subject_ref: p09-dice-delegated-rng-provider-placement
  proof_notebook_review_unit_obligations:
  - rollback_cut_non_interference
  model_check_concrete_carrier_obligations:
  - rollback_cut_non_interference
  guard_reason: none
artifact_preview:
  proof_notebook_review_units:
  - obligation_kind: rollback_cut_non_interference
    goal_text: Review that rollback / atomic-cut evidence does not interfere with surviving runtime behavior for `p09-dice-delegated-rng-provider-placement`.
    evidence_refs:
    - fixture:p09-dice-delegated-rng-provider-placement
    - runtime_cluster:p09-dice-delegated-rng-provider-placement
  model_check_concrete_carriers:
  - obligation_kind: rollback_cut_non_interference
    evidence_refs:
    - fixture:p09-dice-delegated-rng-provider-placement
    - runtime_cluster:p09-dice-delegated-rng-provider-placement
surface_preview:
  minimal_companion:
    status: guarded_not_reached
    guard_reason: current minimal companion surface only actualizes reached authoritative-room defaults: current authoritative-room vertical slice only actualizes reached current default samples (`p07` / `p08`): current default samples (`p07` / `p08`) were not reached for `p09-dice-delegated-rng-provider-placement`
    lines: []
    compare_floor_refs:
    - compare_floor:current_l2.experimental_order_handoff_guard_only
    guard_refs:
    - guard:companion_surface_not_reached
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
  stage_block_secondary:
    status: guarded_not_reached
    guard_reason: current stage-block secondary surface only actualizes reached authoritative-room defaults: current authoritative-room vertical slice only actualizes reached current default samples (`p07` / `p08`): current default samples (`p07` / `p08`) were not reached for `p09-dice-delegated-rng-provider-placement`
    lines: []
    compare_floor_refs:
    - compare_floor:current_l2.experimental_stage_block_guard_only
    guard_refs:
    - guard:stage_block_surface_not_reached
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:authoritative_room_serial_scope_sugar
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
  serial_scope_reserve:
    status: reached
    guard_reason: none
    lines:
    - serial on dice_authority {
    -   fetch fetch_provider_roll@delegated_rng
    -   publish publish_roll_result@dice_state
    -   handoff handoff_dice_authority@dice_state
    - }
    compare_floor_refs:
    - compare_floor:current_l2.delegated_rng_service.practical
    - compare_floor:current_l2.witness_provider_route_actual_adoption
    - compare_floor:current_l2.order_handoff.serial_scope_reserve_surface
    guard_refs:
    - guard:serial_scope_room_specific_reserve
    - guard:principal_edge_row_surface_unchanged
    - guard:helper_local_surface_only
    - guard:final_source_surface_handoff_wording_later
    kept_later_refs:
    - kept_later:final_parser_grammar
    - kept_later:final_public_parser_checker_runtime_api
    - kept_later:serial_scope_public_promotion
    - kept_later:serial_scope_beyond_authoritative_room
    - kept_later:final_source_surface_handoff_wording
    - kept_later:final_emitted_artifact_schema
    - kept_later:final_emitted_handoff_contract
    - kept_later:final_public_witness_schema
    - kept_later:final_public_provider_receipt_schema
    - kept_later:combined_provider_witness_public_contract
    - kept_later:low_level_memory_order_source_surface
    - kept_later:final_modal_foundation_adoption
authoritative_room_first_scenario_actual_adoption:
  status: guarded_not_reached
  adoption_kind: helper_local_authoritative_room_first_scenario_manifest
  profile_axis_refs: []
  relation_refs: []
  authority_handoff_refs: []
  runtime_evidence_refs: []
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p09-dice-delegated-rng-provider-placement:rollback_cut_non_interference
  - repo_local_emitted_artifact:model_check_concrete_carrier:p09-dice-delegated-rng-provider-placement:rollback_cut_non_interference
  reserve_route_refs:
  - reserve_route:delegated_rng_service_practical:p09-dice-delegated-rng-provider-placement
  - reserve_route:serial_scope_reserve_surface:p09-dice-delegated-rng-provider-placement
  - reserve_route:witness_provider_route_actual_adoption:p09-dice-delegated-rng-provider-placement
  negative_static_stop_refs: []
  contrast_refs:
  - contrast_target:append_friendly_notice_room
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:authoritative_room_first_scenario_actual_adoption
  - compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption
  compare_floor_refs:
  - compare_floor:current_l2.delegated_rng_service.practical
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.authoritative_room.first_scenario_actual_adoption.guard_only
  guard_refs:
  - guard:delegated_rng_service_practical_reserve
  - guard:first_scenario_pair_unchanged
  kept_later_refs:
  - kept_later:auditable_authority_witness
  - kept_later:delegated_rng_service
  - kept_later:distributed_randomness_provider
  - kept_later:final_emitted_handoff_contract
  - kept_later:exhaustive_shared_space_catalog
  - kept_later:final_consistency_fairness_catalog
  guard_reason: current authoritative-room first scenario keeps delegated RNG placement on the practical reserve route and does not promote it into the representative default pair (`p07` / `p08`) for `p09-dice-delegated-rng-provider-placement`
authoritative_room_reserve_strengthening_lane:
  status: reached
  lane_kind: helper_local_reserve_strengthening_lane_manifest
  witness_strengthening_status: guarded_not_reached
  delegated_rng_service_status: reached
  model_check_second_line_status: reached
  witness_strengthening_refs: []
  delegated_rng_service_refs:
  - profile_axis:rng_source:delegated_rng_service
  - provider_boundary:placement:delegated_rng_service
  - provider_boundary:authority_keeps_commit
  - optional_attachment:provider_draw_ref
  - provider_sample:p09-dice-delegated-rng-provider-placement:delegated_rng_draw_route
  model_check_second_line_refs:
  - property_preview:row_local:p09-dice-delegated-rng-provider-placement:canonical_normalization_law
  - property_preview:row_local:p09-dice-delegated-rng-provider-placement:no_re_promotion
  - model_check_request_preflight:p09-dice-delegated-rng-provider-placement:row_local_property_preview
  - model_check_request_preflight:p09-dice-delegated-rng-provider-placement:small_cluster_semantic_projection
  - public_checker_second_reserve:boundary
  first_line_boundary_refs:
  - first_line_boundary:representative_pair_kept_at_p07_p08
  - first_line_boundary:authoritative_room_default_profile_stays_principal
  - first_line_boundary:authority_rng_default_profile_unchanged
  - first_line_boundary:delegated_rng_not_promoted_into_default_pair
  reserve_boundary_refs:
  - reserve_boundary:auditable_authority_witness_second_strengthening
  - reserve_boundary:delegated_rng_service_second_practical
  - reserve_boundary:model_check_second_line_not_room_profile
  - reserve_boundary:public_checker_contract_kept_later
  - reserve_boundary:witness_provider_combined_public_contract_later
  repo_local_emitted_artifact_refs:
  - repo_local_emitted_artifact:proof_notebook_review_unit:p09-dice-delegated-rng-provider-placement:rollback_cut_non_interference
  - repo_local_emitted_artifact:model_check_concrete_carrier:p09-dice-delegated-rng-provider-placement:rollback_cut_non_interference
  compare_floor_refs:
  - compare_floor:current_l2.delegated_rng_service.practical
  - compare_floor:current_l2.model_check.second_line_concretization
  - compare_floor:current_l2.reserve_strengthening_lane:p09-dice-delegated-rng-provider-placement
  guard_refs:
  - guard:first_line_boundary_preserved
  - guard:reserve_components_kept_separate
  - guard:model_check_second_line_not_room_profile
  - guard:witness_provider_public_contract_later
  kept_later_refs:
  - kept_later:combined_witness_provider_public_contract
  - kept_later:final_public_witness_schema
  - kept_later:final_public_provider_receipt_schema
  - kept_later:concrete_model_check_tool_brand
  - kept_later:actual_public_checker_migration
  - kept_later:distributed_fairness_theorem
  guard_reason: none
order_handoff_source_surface_artifact_actual_adoption:
  status: guarded_not_reached
  adoption_kind: helper_local_source_surface_artifact_route_manifest
  profile_axis_refs: []
  principal_surface_lines: []
  secondary_surface_lines: []
  repo_local_emitted_artifact_refs: []
  source_wording_route_refs: []
  emitted_artifact_candidate_keep_refs: []
  negative_static_stop_refs: []
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:order_handoff_source_surface_artifact_actual_adoption
  - compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption
  compare_floor_refs:
  - compare_floor:current_l2.order_handoff.source_surface_artifact_actual_adoption.guard_only
  guard_refs:
  - guard:source_surface_artifact_actual_adoption_not_reached
  - guard:negative_static_stop_pair_kept_helper_local
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_runtime_api
  - kept_later:final_source_surface_handoff_wording
  - kept_later:final_emitted_artifact_schema
  - kept_later:final_emitted_handoff_contract
  - kept_later:final_public_witness_schema
  - kept_later:authoritative_room_serial_scope_sugar
  - kept_later:low_level_memory_order_source_surface
  - kept_later:final_modal_foundation_adoption
  guard_reason: current order-handoff source surface / emitted-artifact actual adoption keeps delegated RNG placement on the serial-scope practical route and does not promote it into the representative principal pair (`p07` / `p08`) for `p09-dice-delegated-rng-provider-placement`
order_handoff_witness_provider_public_seam_compression:
  status: guarded_not_reached
  compression_kind: helper_local_public_seam_manifest
  profile_axis_refs: []
  repo_local_emitted_artifact_refs: []
  source_wording_route_refs: []
  emitted_artifact_candidate_keep_refs: []
  serial_scope_lines: []
  witness_schema_route_refs: []
  provider_receipt_route_refs: []
  combined_public_contract_keep_refs: []
  trace_alignment_pair_refs: []
  public_seam_residual_refs: []
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:order_handoff_witness_provider_public_seam_compression
  - compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression
  compare_floor_refs:
  - compare_floor:current_l2.order_handoff_witness_provider_public_seam_compression.guard_only
  guard_refs:
  - guard:order_handoff_witness_provider_public_seam_compression_not_reached
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_runtime_api
  - kept_later:final_source_surface_handoff_wording
  - kept_later:final_emitted_artifact_schema
  - kept_later:final_public_witness_schema
  - kept_later:final_public_provider_receipt_schema
  - kept_later:delegated_provider_attestation
  - kept_later:combined_provider_witness_public_contract
  - kept_later:final_emitted_handoff_contract
  - kept_later:authoritative_room_serial_scope_sugar
  - kept_later:low_level_memory_order_source_surface
  - kept_later:final_modal_foundation_adoption
  - kept_later:exhaustive_shared_space_catalog
  guard_reason: current order-handoff/witness-provider public seam compression only actualizes the representative authoritative-room pair (`p07` / `p08`) after route/reserve/bridge/threshold floors align for `p09-dice-delegated-rng-provider-placement`
theorem_result_object_preview:
  status: guarded_not_reached
  preview_kind: helper_local_actualization_manifest
  subject_kind: runtime_try_cut_cluster
  subject_ref: p09-dice-delegated-rng-provider-placement
  result_object_route_refs: []
  notebook_payload_preview_refs: []
  proof_object_schema_reserve_refs: []
  actual_adoption_default_refs: []
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:theorem_result_object_preview
  - compare_floor:current_l2.theorem_result_object_preview_actualization
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.theorem_result_object_preview.guard_only
  guard_refs:
  - guard:theorem_result_object_preview_not_reached
  kept_later_refs:
  - kept_later:final_public_theorem_result_object
  - kept_later:consumer_shaped_theorem_payload
  - kept_later:concrete_theorem_prover_brand
  - kept_later:proof_object_public_schema
  - kept_later:final_public_verifier_contract
  guard_reason: current theorem result-object preview only actualizes the representative theorem quartet (`e5` / `p06` / `p07` / `p08`) after verification preview reaches the formal-hook route for `p09-dice-delegated-rng-provider-placement`
model_check_public_checker_preview:
  status: reached
  preview_kind: helper_local_actualization_manifest
  subject_kind: runtime_try_cut_cluster
  subject_ref: p09-dice-delegated-rng-provider-placement
  checker_artifact_preview_refs:
  - model_check_public_checker_preview:p09-dice-delegated-rng-provider-placement:consumer_shaped_artifact_preview_only
  - model_check_public_checker_preview:p09-dice-delegated-rng-provider-placement:checker_boundary_bundle
  - model_check_public_checker_preview:p09-dice-delegated-rng-provider-placement:row_local_property_route_bundle
  - model_check_public_checker_preview:p09-dice-delegated-rng-provider-placement:repo_local_emitted_artifact_refs
  verifier_handoff_reserve_refs:
  - model_check_verifier_handoff_reserve:public_checker_migration_later
  - model_check_verifier_handoff_reserve:emitted_handoff_artifact_later
  - model_check_verifier_handoff_reserve:runtime_policy_contract_later
  tool_binding_reserve_refs:
  - model_check_tool_binding_reserve:brand_neutral_request_manifest
  - model_check_tool_binding_reserve:concrete_tool_brand_later
  - model_check_tool_binding_reserve:runtime_policy_contract_later
  actual_adoption_default_refs:
  - model_check_public_checker_preview_default:consumer_shaped_artifact_preview_only
  - model_check_public_checker_preview_default:verifier_handoff_reserve_keep
  - model_check_public_checker_preview_default:brand_neutral_tool_binding_reserve_keep
  - model_check_public_checker_preview_default:runtime_policy_contract_later
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:model_check_public_checker_preview
  - compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.model_check.row_local_property_actual_adoption
  - compare_floor:current_l2.model_check.second_line_concretization
  - compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization
  guard_refs:
  - guard:public_checker_artifact_preview_actualization_only
  - guard:verifier_handoff_reserve_keep
  - guard:brand_neutral_tool_binding_reserve_keep
  - guard:runtime_policy_contract_later
  kept_later_refs:
  - kept_later:first_settled_property_language
  - kept_later:concrete_model_check_tool_brand
  - kept_later:final_public_checker_artifact
  - kept_later:actual_public_checker_migration
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:production_checker_runtime_policy_contract
  - kept_later:final_public_verifier_contract
  guard_reason: none
theorem_final_public_contract_reopen_threshold:
  status: guarded_not_reached
  threshold_kind: helper_local_reopen_threshold_manifest
  subject_kind: runtime_try_cut_cluster
  subject_ref: p09-dice-delegated-rng-provider-placement
  result_object_route_refs: []
  payload_preview_keep_refs: []
  proof_object_schema_candidate_refs: []
  prover_brand_candidate_refs: []
  final_public_contract_reopen_sequence_refs: []
  threshold_default_refs: []
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:theorem_final_public_contract_reopen_threshold
  - compare_floor:current_l2.theorem_final_public_contract_reopen_threshold
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.theorem_final_public_contract_reopen_threshold.guard_only
  guard_refs:
  - guard:theorem_final_public_contract_reopen_threshold_not_reached
  kept_later_refs:
  - kept_later:final_public_theorem_result_object
  - kept_later:consumer_shaped_theorem_payload
  - kept_later:concrete_theorem_prover_brand
  - kept_later:proof_object_public_schema
  - kept_later:final_public_verifier_contract
  guard_reason: current theorem final public-contract reopen threshold only actualizes the representative theorem quartet (`e5` / `p06` / `p07` / `p08`) after verification preview reaches the formal-hook route for `p09-dice-delegated-rng-provider-placement`
model_check_final_public_contract_reopen_threshold:
  status: reached
  threshold_kind: helper_local_reopen_threshold_manifest
  subject_kind: runtime_try_cut_cluster
  subject_ref: p09-dice-delegated-rng-provider-placement
  checker_artifact_route_refs:
  - model_check_checker_artifact_actual_route:p09-dice-delegated-rng-provider-placement:row_local_property_route_first
  - model_check_checker_artifact_actual_route:p09-dice-delegated-rng-provider-placement:checker_boundary_contract_anchor
  - model_check_checker_artifact_actual_route:p09-dice-delegated-rng-provider-placement:consumer_shaped_checker_artifact_candidate_only
  - model_check_checker_artifact_actual_route:p09-dice-delegated-rng-provider-placement:repo_local_emitted_artifact_refs_first
  - model_check_checker_artifact_actual_route:p09-dice-delegated-rng-provider-placement:final_public_checker_artifact_later
  migration_candidate_keep_refs:
  - model_check_checker_artifact_migration_keep:p09-dice-delegated-rng-provider-placement:verifier_handoff_candidate_adjacent_keep
  - model_check_checker_artifact_migration_keep:p09-dice-delegated-rng-provider-placement:tool_brand_candidate_adjacent_keep
  - model_check_checker_artifact_migration_keep:p09-dice-delegated-rng-provider-placement:actual_public_checker_migration_candidate_only
  - model_check_checker_artifact_migration_keep:p09-dice-delegated-rng-provider-placement:runtime_policy_contract_later
  verifier_handoff_candidate_refs:
  - model_check_verifier_handoff_candidate:p09-dice-delegated-rng-provider-placement:public_checker_preview_adjacent
  - model_check_verifier_handoff_candidate:p09-dice-delegated-rng-provider-placement:emitted_handoff_artifact_candidate
  - model_check_verifier_handoff_candidate:p09-dice-delegated-rng-provider-placement:runtime_policy_contract_candidate
  tool_brand_candidate_refs:
  - model_check_tool_brand_candidate:p09-dice-delegated-rng-provider-placement:brand_neutral_request_manifest_keep
  - model_check_tool_brand_candidate:p09-dice-delegated-rng-provider-placement:concrete_tool_brand_candidate
  - model_check_tool_brand_candidate:p09-dice-delegated-rng-provider-placement:public_checker_artifact_not_adopted
  final_public_contract_reopen_sequence_refs:
  - model_check_final_public_contract_reopen:p09-dice-delegated-rng-provider-placement:property_language_and_tool_brand_first
  - model_check_final_public_contract_reopen:p09-dice-delegated-rng-provider-placement:public_checker_artifact_and_migration_second
  - model_check_final_public_contract_reopen:p09-dice-delegated-rng-provider-placement:verifier_handoff_and_runtime_policy_contract_third
  - model_check_final_public_contract_reopen:p09-dice-delegated-rng-provider-placement:final_public_verifier_contract_fourth
  threshold_default_refs:
  - model_check_final_public_contract_reopen_default:property_language_and_tool_brand_first
  - model_check_final_public_contract_reopen_default:public_checker_artifact_and_migration_second
  - model_check_final_public_contract_reopen_default:verifier_handoff_and_runtime_policy_contract_third
  - model_check_final_public_contract_reopen_default:final_public_verifier_contract_fourth
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:model_check_final_public_contract_reopen_threshold
  - compare_floor:current_l2.model_check.final_public_contract_reopen_threshold
  bridge_floor_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.model_check.public_checker_artifact_preview_actualization
  - compare_floor:current_l2.model_check.property_tool_threshold
  - compare_floor:current_l2.model_check.tool_brand_verifier_handoff_coupled_later_gate
  - compare_floor:current_l2.model_check.public_checker_artifact_migration_coupled_later_gate
  - compare_floor:current_l2.model_check.checker_artifact_route_actual_adoption
  - compare_floor:current_l2.model_check.final_public_contract_reopen_threshold
  guard_refs:
  - guard:property_language_and_tool_brand_first
  - guard:public_checker_artifact_and_migration_second
  - guard:verifier_handoff_and_runtime_policy_contract_third
  - guard:final_public_verifier_contract_fourth
  kept_later_refs:
  - kept_later:first_settled_property_language
  - kept_later:concrete_model_check_tool_brand
  - kept_later:final_public_checker_artifact
  - kept_later:actual_public_checker_migration
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:production_checker_runtime_policy_contract
  - kept_later:final_public_verifier_contract
  guard_reason: none
typed_checker_hint_preview:
  status: guarded_not_reached
  preview_kind: sample_local_helper_preview
  cluster_kind: none
  case_label: none
  typed_reason_family_hint: none
  evidence_refs: []
  compare_floor_refs:
  - compare_floor:current_l2.typed.sample_local_checker_hint_guard_only
  guard_refs:
  - guard:typed_checker_hint_preview_not_reached
  kept_later_refs:
  - kept_later:final_typed_source_principal
  - kept_later:final_finite_index_surface
  - kept_later:final_ifc_syntax
  - kept_later:actual_checker_payload_family
  - kept_later:checker_supported_kind_summary
  - kept_later:final_public_verifier_contract
  guard_reason: current typed checker-hint preview only actualizes the sample-local first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after verification preview reaches runtime try-cut evidence for `p09-dice-delegated-rng-provider-placement`
actual_checker_payload_family_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_payload_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_family_kind: none
  source_refs: []
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_checker_payload_family_threshold
  - compare_floor:current_l2.checker.actual_checker_payload_family
  compare_floor_refs:
  - compare_floor:current_l2.checker.actual_checker_payload_family.guard_only
  guard_refs:
  - guard:actual_checker_payload_family_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_family
  - kept_later:checker_payload_row_detail
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload family threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after typed checker-hint preview reaches the checker-adjacent helper floor for `p09-dice-delegated-rng-provider-placement`
actual_checker_payload_row_family_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_family_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_family_ref: none
  row_family_kind: none
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_checker_payload_row_family_threshold
  - compare_floor:current_l2.checker.checker_payload_row_family
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_family.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_family_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_detail
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-family threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload family threshold reaches the checker-adjacent helper floor for `p09-dice-delegated-rng-provider-placement`
actual_checker_payload_row_detail_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_detail_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_row_family_ref: none
  row_source_ref: none
  row_reason_kind: none
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_checker_payload_row_detail_threshold
  - compare_floor:current_l2.checker.checker_payload_row_detail
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_detail.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_detail_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_payload_row_body
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-detail threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-family threshold reaches the checker-adjacent helper floor for `p09-dice-delegated-rng-provider-placement`
actual_checker_payload_row_body_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_row_body_threshold_manifest
  cluster_kind: none
  case_label: none
  family_refs: []
  coverage_state: none
  payload_row_family_ref: none
  row_source_ref: none
  row_reason_kind: none
  row_body: none
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_checker_payload_row_body_threshold
  - compare_floor:current_l2.checker.checker_payload_row_body
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_row_body.guard_only
  guard_refs:
  - guard:actual_checker_payload_row_body_threshold_not_reached
  kept_later_refs:
  - kept_later:checker_supported_kind_summary
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload row-body threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-detail threshold reaches the checker-adjacent helper floor for `p09-dice-delegated-rng-provider-placement`
actual_checker_payload_supported_kind_summary_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_supported_kind_summary_threshold_manifest
  payload_row_family_ref: none
  supported_kind_scope: none
  supported_kind_refs: []
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_checker_payload_supported_kind_summary_threshold
  - compare_floor:current_l2.checker.checker_payload_supported_kind_summary
  compare_floor_refs:
  - compare_floor:current_l2.checker.checker_payload_supported_kind_summary.guard_only
  guard_refs:
  - guard:actual_checker_payload_supported_kind_summary_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_payload_schema
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload supported-kind summary threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload row-body threshold reaches the checker-adjacent helper floor for `p09-dice-delegated-rng-provider-placement`
actual_checker_payload_public_schema_sketch_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_payload_schema_sketch_threshold_manifest
  actual_checker_payload_family_ref: none
  checker_payload_row_family_ref: none
  checker_payload_row_detail_ref: none
  checker_payload_row_body_ref: none
  checker_payload_supported_kind_summary_ref: none
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_checker_payload_public_schema_sketch_threshold
  - compare_floor:current_l2.checker.public_checker_payload_schema
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_payload_schema.guard_only
  guard_refs:
  - guard:actual_checker_payload_public_schema_sketch_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_api
  - kept_later:final_public_checker_artifact
  - kept_later:final_typed_source_principal
  - kept_later:final_ifc_syntax
  - kept_later:final_public_verifier_contract
  guard_reason: current actual checker payload public-schema sketch threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload supported-kind summary threshold reaches the checker-adjacent helper floor for `p09-dice-delegated-rng-provider-placement`
actual_public_checker_api_sketch_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_api_sketch_threshold_manifest
  checker_subject: none
  public_checker_payload_schema_ref: none
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_public_checker_api_sketch_threshold
  - compare_floor:current_l2.checker.public_checker_api
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_api.guard_only
  guard_refs:
  - guard:actual_public_checker_api_sketch_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_entry_criteria
  - kept_later:public_checker_command_surface
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker API sketch threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual checker payload public-schema sketch threshold reaches the checker-adjacent helper floor for `p09-dice-delegated-rng-provider-placement`
actual_public_checker_entry_criteria_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_entry_criteria_threshold_manifest
  public_checker_api_ref: none
  entry_criteria_refs: []
  family_facade_support_ref: none
  family_facade_script_refs: []
  smoke_command_refs: []
  next_comparison_target_ref: none
  deferred_boundary_refs: []
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_public_checker_entry_criteria_threshold
  - compare_floor:current_l2.checker.public_checker_entry_criteria
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_entry_criteria.guard_only
  guard_refs:
  - guard:actual_public_checker_entry_criteria_threshold_not_reached
  kept_later_refs:
  - kept_later:public_checker_command_surface
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker entry-criteria threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker API sketch threshold reaches the checker-adjacent helper floor for `p09-dice-delegated-rng-provider-placement`
actual_public_checker_command_surface_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_command_surface_threshold_manifest
  command_surface_kind: none
  family_facade_command_refs: []
  public_checker_api_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_public_checker_command_surface_threshold
  - compare_floor:current_l2.checker.public_checker_command_surface
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_command_surface.guard_only
  guard_refs:
  - guard:actual_public_checker_command_surface_threshold_not_reached
  kept_later_refs:
  - kept_later:detached_loop_smoke_wrapper
  - kept_later:generic_shared_public_checker_entry
  - kept_later:shared_output_contract
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker command-surface threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker entry-criteria threshold reaches the checker-adjacent helper floor for `p09-dice-delegated-rng-provider-placement`
actual_shared_output_contract_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_shared_output_contract_threshold_manifest
  output_contract_kind: none
  checker_cluster_name: none
  checker_status: none
  public_checker_payload_schema_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_shared_output_contract_threshold
  - compare_floor:current_l2.checker.shared_output_contract
  compare_floor_refs:
  - compare_floor:current_l2.checker.shared_output_contract.guard_only
  guard_refs:
  - guard:actual_shared_output_contract_threshold_not_reached
  kept_later_refs:
  - kept_later:detached_loop_wrapper_paths
  - kept_later:fixture_and_actual_rows_textual_rendering
  - kept_later:generic_shared_public_checker_entry
  - kept_later:parser_front_public_checker_boundary
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual shared output contract threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker command-surface threshold reaches the checker-adjacent helper floor for `p09-dice-delegated-rng-provider-placement`
actual_public_checker_boundary_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_public_checker_boundary_threshold_manifest
  boundary_kind: none
  public_checker_command_surface_ref: none
  shared_output_contract_ref: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_public_checker_boundary_threshold
  - compare_floor:current_l2.checker.public_checker_boundary
  compare_floor_refs:
  - compare_floor:current_l2.checker.public_checker_boundary.guard_only
  guard_refs:
  - guard:actual_public_checker_boundary_threshold_not_reached
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:query_token_and_checker_subject_public_naming
  - kept_later:generic_shared_public_checker_entry
  - kept_later:detached_loop_wrapper_path_line
  - kept_later:verifier_handoff_surface
  - kept_later:final_public_verifier_contract
  guard_reason: current actual public checker boundary threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual shared output contract threshold reaches the checker-adjacent helper floor for `p09-dice-delegated-rng-provider-placement`
actual_verifier_handoff_surface_threshold:
  status: guarded_not_reached
  threshold_kind: checker_adjacent_verifier_handoff_surface_threshold_manifest
  handoff_surface_kind: none
  public_checker_boundary_ref: none
  proof_obligation_matrix_ref: none
  handoff_artifact_mode: none
  next_comparison_target_ref: none
  deferred_surface_refs: []
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_verifier_handoff_surface_threshold
  - compare_floor:current_l2.checker.verifier_handoff_surface
  compare_floor_refs:
  - compare_floor:current_l2.checker.verifier_handoff_surface.guard_only
  guard_refs:
  - guard:actual_verifier_handoff_surface_threshold_not_reached
  kept_later_refs:
  - kept_later:subject_rows
  - kept_later:theorem_protocol_runtime_dedicated_handoff_artifact_family
  - kept_later:actual_emitted_verifier_handoff_artifact
  - kept_later:tool_specific_schema_and_actual_emitter_policy
  - kept_later:final_parser_grammar
  - kept_later:query_token_and_shared_generic_entry
  - kept_later:final_public_verifier_contract
  guard_reason: current actual verifier handoff surface threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual public checker boundary threshold reaches the checker-adjacent helper floor for `p09-dice-delegated-rng-provider-placement`
actual_minimal_parser_subset_freeze_threshold:
  status: guarded_not_reached
  threshold_kind: parser_front_minimal_parser_subset_freeze_threshold_manifest
  freeze_kind: none
  accepted_cluster_refs: []
  reject_cluster_refs: []
  retention_floor_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_minimal_parser_subset_freeze_threshold
  - compare_floor:current_l2.parser.minimal_parser_subset_freeze
  compare_floor_refs:
  - compare_floor:current_l2.parser.minimal_parser_subset_freeze.guard_only
  guard_refs:
  - guard:actual_minimal_parser_subset_freeze_threshold_not_reached
  kept_later_refs:
  - kept_later:stage3_admit_slot_branch
  - kept_later:stage3_request_clause_branch
  - kept_later:stage3_predicate_fragment_branch
  - kept_later:public_parser_api
  - kept_later:final_parser_grammar
  - kept_later:parser_to_checker_reconnect_freeze
  - kept_later:final_public_parser_checker_api
  - kept_later:final_public_verifier_contract
  guard_reason: current actual minimal parser subset freeze threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual verifier handoff surface threshold reaches the helper-local docs-only bridge floor for `p09-dice-delegated-rng-provider-placement`
actual_parser_to_checker_reconnect_freeze_threshold:
  status: guarded_not_reached
  threshold_kind: parser_checker_bridge_reconnect_freeze_threshold_manifest
  reconnect_kind: none
  parser_subset_freeze_ref: none
  checker_floor_refs: []
  retained_helper_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_parser_to_checker_reconnect_freeze_threshold
  - compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze
  compare_floor_refs:
  - compare_floor:current_l2.parser.parser_to_checker_reconnect_freeze.guard_only
  guard_refs:
  - guard:actual_parser_to_checker_reconnect_freeze_threshold_not_reached
  kept_later_refs:
  - kept_later:stage3_request_predicate_reconnect_line
  - kept_later:stage1_direct_target_mismatch_redesign_line
  - kept_later:runtime_contrast_e21_e22_line
  - kept_later:final_parser_grammar
  - kept_later:final_public_parser_checker_api
  - kept_later:actual_external_verifier_schema
  - kept_later:final_public_verifier_contract
  guard_reason: current actual parser-to-checker reconnect freeze threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual minimal parser subset freeze threshold reaches the stage1+stage2 parser floor for `p09-dice-delegated-rng-provider-placement`
actual_phase1_semantics_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase1_semantics_closeout_threshold_manifest
  closeout_kind: none
  core_semantics_refs: []
  invariant_bridge_refs: []
  notation_status_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_phase1_semantics_closeout_threshold
  - compare_floor:current_l2.closeout.phase1_semantics_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase1_semantics_closeout.guard_only
  guard_refs:
  - guard:actual_phase1_semantics_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:final_parser_grammar
  - kept_later:final_reserved_keyword_and_punctuation
  - kept_later:final_type_system
  - kept_later:actual_external_verifier_schema
  - kept_later:actual_emitted_verifier_artifact
  - kept_later:final_public_verifier_contract
  guard_reason: current actual phase1 semantics closeout threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual parser-to-checker reconnect freeze threshold reaches the checker-floor bridge for `p09-dice-delegated-rng-provider-placement`
actual_phase2_parser_free_poc_closeout_threshold:
  status: guarded_not_reached
  threshold_kind: phase2_parser_free_poc_closeout_threshold_manifest
  closeout_kind: none
  compile_gate_refs: []
  helper_boundary_refs: []
  detached_loop_policy_refs: []
  next_comparison_target_ref: none
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_phase2_parser_free_poc_closeout_threshold
  - compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase2_parser_free_poc_closeout.guard_only
  guard_refs:
  - guard:actual_phase2_parser_free_poc_closeout_threshold_not_reached
  kept_later_refs:
  - kept_later:reference_update_bless_workflow
  - kept_later:final_retention_path_policy
  - kept_later:public_exporter_api
  - kept_later:production_host_interface
  guard_reason: current actual phase2 parser-free PoC closeout threshold only actualizes the first strong typing sample set (`p10` / `p11` / `p12` / `p15` / `p16`) after actual phase1 semantics closeout threshold reaches the semantics closeout floor for `p09-dice-delegated-rng-provider-placement`
actual_phase4_shared_space_self_driven_closeout_threshold:
  status: reached
  threshold_kind: phase4_shared_space_self_driven_closeout_threshold_manifest
  closeout_kind: shared_space_practical_boundary_checkpoint
  current_package_refs:
  - authoritative_room_baseline_ref
  - working_subset_catalog_ref
  - minimal_authority_witness_core_ref
  - authoritative_delegated_provider_cut_ref
  - control_plane_threshold_ref
  user_spec_required_catalog_refs:
  - final_activation_overlay_catalog
  - final_authority_auth_identity_admission_catalog
  - final_consistency_fairness_catalog
  retained_later_refs:
  - append_friendly_optional_provider_attestation
  - control_plane_separated_carrier_actualization
  - distributed_fairness_protocol
  - final_operational_realization
  next_comparison_target_ref: phase5_proof_protocol_runtime_policy_handoff_closeout_comparison
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  guard_refs:
  - guard:phase4_shared_space_self_driven_closeout_threshold_only
  - guard:phase5_proof_protocol_runtime_policy_handoff_closeout_comparison_next
  - guard:user_spec_required_final_catalog_later
  - guard:distributed_fairness_protocol_later
  kept_later_refs:
  - kept_later:final_public_witness_provider_artifact_contract
  - kept_later:exhaustive_shared_space_catalog
  - kept_later:control_plane_separated_carrier_actualization
  - kept_later:distributed_fairness_protocol
  - kept_later:final_operational_realization
  guard_reason: none
actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold:
  status: reached
  threshold_kind: phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_manifest
  closeout_kind: proof_protocol_runtime_policy_handoff_stop_line
  verifier_handoff_surface_ref: minimal_verifier_handoff_surface
  theorem_retained_bridge_stop_ref: retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  boundary_inventory_ref: small_decidable_core_boundary_inventory
  retained_later_refs:
  - actual_subject_row_materialization
  - boundary_specific_handoff_artifact_family
  - actual_emitted_verifier_artifact
  - concrete_tool_binding
  - public_checker_migration
  - low_level_memory_order_family
  next_comparison_target_ref: phase6_actual_parser_ast_carrier_first_tranche_comparison
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  guard_refs:
  - guard:phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_only
  - guard:phase6_actual_parser_ast_carrier_first_tranche_comparison_next
  - guard:actual_subject_row_and_artifact_family_later
  - guard:tool_binding_public_checker_migration_and_low_level_memory_order_later
  kept_later_refs:
  - kept_later:actual_subject_row_materialization
  - kept_later:boundary_specific_handoff_artifact_family
  - kept_later:actual_emitted_verifier_artifact
  - kept_later:concrete_tool_binding
  - kept_later:public_checker_migration
  - kept_later:low_level_memory_order_family
  guard_reason: none
actual_phase6_actual_parser_ast_carrier_first_tranche_threshold:
  status: reached
  threshold_kind: phase6_actual_parser_ast_carrier_first_tranche_threshold_manifest
  carrier_kind: current_l2_nonproduction_parser_carrier
  accepted_surface_refs:
  - stage1_option_decl_chain_surface
  - stage2_try_fallback_structural_surface
  code_anchor_refs:
  - mir_ast_current_l2_module
  - stage1_stage2_parser_spike_tests
  retained_later_refs:
  - stage3_admit_slot_surface
  - stage3_request_clause_suite
  - stage3_predicate_fragment
  - perform_head_final_public_api
  - span_rich_diagnostics
  - final_grammar
  next_comparison_target_ref: phase6_actual_checker_runtime_skeleton_first_tranche_comparison
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  guard_refs:
  - guard:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold_required
  - guard:phase6_actual_checker_runtime_skeleton_first_tranche_comparison_next
  kept_later_refs:
  - stage3_admit_slot_surface
  - stage3_request_clause_suite
  - stage3_predicate_fragment
  - perform_head_final_public_api
  - span_rich_diagnostics
  - final_grammar
  guard_reason: none
actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold:
  status: reached
  threshold_kind: phase6_actual_checker_runtime_skeleton_first_tranche_threshold_manifest
  skeleton_kind: current_l2_nonproduction_checker_runtime_skeleton
  semantic_entry_refs:
  - static_gate_program_detailed
  - direct_style_evaluator_from_program
  - fixture_host_stub_run_program
  runtime_bridge_refs:
  - mir_runtime_current_l2_module
  - current_l2_runtime_skeleton_report
  parser_bridge_contract_refs:
  - stage1_reconnect_clusters
  - stage2_try_rollback_structural_summary
  - parser_bridge_consistency_guard
  retained_later_refs:
  - parser_to_program_lowering
  - stage3_request_predicate_reconnect
  - richer_host_interface
  - final_public_runtime_checker_api
  - formal_hook_concrete_tool_binding
  next_comparison_target_ref: phase6_compile_ready_verification_and_formal_hook_comparison
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  guard_refs:
  - guard:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold_required
  - guard:phase6_compile_ready_verification_and_formal_hook_comparison_next
  kept_later_refs:
  - parser_to_program_lowering
  - stage3_request_predicate_reconnect
  - richer_host_interface
  - final_public_runtime_checker_api
  - formal_hook_concrete_tool_binding
  guard_reason: none
actual_phase6_compile_ready_verification_and_formal_hook_threshold:
  status: reached
  threshold_kind: phase6_compile_ready_verification_and_formal_hook_threshold_manifest
  verification_gate_refs:
  - cargo_test_mir_ast
  - cargo_test_mir_runtime
  - cargo_test_mir_semantics_current_l2_minimal_interpreter
  - cargo_test_mir_semantics_current_l2_static_gate_support
  - cargo_test_mir_semantics_current_l2_detached_bundle_support
  - cargo_test_mir_semantics_current_l2_formal_hook_support
  - python_unittest_current_l2_static_and_detached_loop
  smoke_gate_refs:
  - smoke_formal_hook_static
  - smoke_formal_hook_runtime
  formal_hook_artifact_kind_ref: current_l2_tool_neutral_formal_hook
  formal_hook_subject_kind_refs:
  - fixture_static_cluster
  - runtime_try_cut_cluster
  formal_hook_contract_row_core_refs:
  - obligation_kind
  - evidence_refs
  formal_hook_evidence_ref_family_refs:
  - ref_kind
  - ref_id
  formal_hook_obligation_kind_refs:
  - canonical_normalization_law
  - no_re_promotion
  - rollback_cut_non_interference
  source_artifact_refs:
  - detached_static_gate_artifact
  - detached_bundle_artifact
  validation_refs:
  - input_schema_version_guard
  - input_artifact_kind_guard
  retained_later_refs:
  - concrete_theorem_tool_binding
  - concrete_model_check_tool_binding
  - parser_second_tranche_widen
  - final_public_surface
  next_comparison_target_ref: phase6_next_reopen_sequencing_comparison
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - source:phase6_compile_ready_verification_and_formal_hook_ready_sketch
  - source:phase6_compile_ready_verification_and_formal_hook_minimum
  - code_anchor:current_l2_emit_formal_hook_example
  - code_anchor:current_l2_formal_hook_support_tests
  - code_anchor:current_l2_detached_loop_smoke_family
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  guard_refs:
  - guard:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold_required
  - guard:phase6_next_reopen_sequencing_comparison_next
  kept_later_refs:
  - concrete_theorem_tool_binding
  - concrete_model_check_tool_binding
  - parser_second_tranche_widen
  - final_public_surface
  guard_reason: none
actual_phase6_next_reopen_sequencing_threshold:
  status: reached
  threshold_kind: phase6_next_reopen_sequencing_threshold_manifest
  sequencing_kind_ref: phase6_checkpoint_postclose_next_reopen
  fixed_entry_criteria_refs:
  - phase6_parser_first_tranche
  - phase6_checker_runtime_first_tranche
  - phase6_compile_ready_formal_hook
  selected_first_reopen_ref: phase6_parser_second_tranche_attached_slot_and_predicate_fragment_route
  deferred_reopen_refs:
  - theorem_first_concrete_tool_binding_route
  - concrete_model_check_tool_binding
  minimum_guard_refs:
  - keep_tool_neutral_formal_hook_as_entry_criteria
  - avoid_request_head_clause_suite_richer_diagnostics_bulk_widen
  - keep_model_check_line_reserve_only
  next_comparison_target_ref: phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - source:phase6_compile_ready_verification_and_formal_hook_ready_sketch
  - source:phase6_compile_ready_verification_and_formal_hook_minimum
  - code_anchor:current_l2_emit_formal_hook_example
  - code_anchor:current_l2_formal_hook_support_tests
  - code_anchor:current_l2_detached_loop_smoke_family
  - helper_preview:actual_phase6_next_reopen_sequencing_threshold
  - source:phase6_next_reopen_sequencing_current_first_choice
  - source:phase6_next_reopen_sequencing_minimum
  - source:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  - compare_floor:current_l2.closeout.phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package
  guard_refs:
  - guard:actual_phase6_compile_ready_verification_and_formal_hook_threshold_required
  - guard:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison_next
  kept_later_refs:
  - request_clause_suite_bulk_widen
  - perform_head_final_public_api
  - concrete_theorem_tool_binding
  - concrete_model_check_tool_binding
  - final_public_surface
  guard_reason: none
actual_phase6_reserve_formal_tool_binding_inventory_threshold:
  status: reached
  threshold_kind: phase6_reserve_formal_tool_binding_inventory_threshold_manifest
  inventory_kind: phase6_postclose_formal_reserve_inventory
  fixed_entry_criteria_refs:
  - phase5_handoff_closeout
  - phase6_compile_ready_formal_hook
  - phase6_parser_second_tranche_first_package
  first_reserve_ref: theorem_first_notebook_pressure_concrete_tool_binding_route
  second_reserve_ref: model_check_protocol_property_concrete_tool_binding_route
  minimum_guard_refs:
  - keep_tool_neutral_formal_hook_as_current_entry_criteria
  - keep_parser_followup_package_as_current_mainline
  - avoid_dual_tool_choice_single_package
  - avoid_public_checker_runtime_surface_backpressure
  next_comparison_target_ref: phase6_parser_side_follow_up_package_sequencing_comparison
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - source:phase6_compile_ready_verification_and_formal_hook_ready_sketch
  - source:phase6_compile_ready_verification_and_formal_hook_minimum
  - code_anchor:current_l2_emit_formal_hook_example
  - code_anchor:current_l2_formal_hook_support_tests
  - code_anchor:current_l2_detached_loop_smoke_family
  - helper_preview:actual_phase6_next_reopen_sequencing_threshold
  - source:phase6_next_reopen_sequencing_current_first_choice
  - source:phase6_next_reopen_sequencing_minimum
  - source:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison
  - helper_preview:actual_phase6_reserve_formal_tool_binding_inventory_threshold
  - source:phase6_reserve_formal_tool_binding_inventory_current_first_choice
  - source:phase6_reserve_formal_tool_binding_inventory_minimum
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  - compare_floor:current_l2.closeout.phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package
  - compare_floor:current_l2.closeout.phase6_reserve_formal_tool_binding_inventory
  guard_refs:
  - guard:actual_phase6_next_reopen_sequencing_threshold_required
  - guard:phase6_parser_side_follow_up_package_sequencing_comparison_next
  kept_later_refs:
  - concrete_theorem_tool_name
  - concrete_model_check_tool_name
  - actual_ci_artifact_retention_policy
  - parser_side_followup_package_selection
  - final_public_parser_checker_runtime_surface
  guard_reason: none
actual_phase6_parser_side_followup_package_sequencing_threshold:
  status: reached
  threshold_kind: phase6_parser_side_followup_package_sequencing_threshold_manifest
  sequencing_kind: phase6_parser_followup_next_package_selection
  fixed_entry_criteria_refs:
  - phase6_parser_second_tranche_first_package
  - phase6_reserve_formal_tool_binding_inventory
  - stage3_multiline_attachment_first_tranche_actualization
  selected_next_package_ref: phase6_parser_second_tranche_shared_single_attachment_frame_first_package
  deferred_reopen_refs:
  - phase6_request_clause_suite_publicization
  - phase6_perform_head_final_public_parser_api
  - phase6_span_rich_diagnostics
  - phase6_final_grammar
  minimum_guard_refs:
  - reuse_existing_stage3_minimal_predicate_fragment_surface
  - keep_request_head_and_suite_ordering_out_of_scope
  - keep_source_sample_path_after_parser_followup_cut
  next_comparison_target_ref: phase6_parser_second_tranche_shared_single_attachment_frame_first_package_comparison
  evidence_refs:
  - sample:p09-dice-delegated-rng-provider-placement
  - helper_preview:actual_phase4_shared_space_self_driven_closeout_threshold
  - source:phase4_shared_space_closeout_ready_sketch
  - source:authoritative_room_baseline_ref
  - source:control_plane_threshold_ref
  - helper_preview:actual_phase5_proof_protocol_runtime_policy_handoff_closeout_threshold
  - source:phase5_handoff_closeout_ready_sketch
  - source:minimal_verifier_handoff_surface
  - source:retained_payload_body_materialization_theorem_export_handoff_transport_channel_body
  - source:small_decidable_core_boundary_inventory
  - helper_preview:actual_phase6_actual_parser_ast_carrier_first_tranche_threshold
  - source:phase6_actual_parser_ast_carrier_first_tranche_ready_sketch
  - source:mir_ast_current_l2_first_tranche
  - code_anchor:mir_ast_current_l2_module
  - code_anchor:stage1_stage2_parser_spike_tests
  - helper_preview:actual_phase6_actual_checker_runtime_skeleton_first_tranche_threshold
  - source:phase6_actual_checker_runtime_skeleton_first_tranche_ready_sketch
  - source:phase6_current_l2_checker_runtime_first_tranche
  - code_anchor:mir_runtime_current_l2_module
  - code_anchor:current_l2_runtime_skeleton_tests
  - helper_preview:actual_phase6_compile_ready_verification_and_formal_hook_threshold
  - source:phase6_compile_ready_verification_and_formal_hook_ready_sketch
  - source:phase6_compile_ready_verification_and_formal_hook_minimum
  - code_anchor:current_l2_emit_formal_hook_example
  - code_anchor:current_l2_formal_hook_support_tests
  - code_anchor:current_l2_detached_loop_smoke_family
  - helper_preview:actual_phase6_next_reopen_sequencing_threshold
  - source:phase6_next_reopen_sequencing_current_first_choice
  - source:phase6_next_reopen_sequencing_minimum
  - source:phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package_comparison
  - helper_preview:actual_phase6_reserve_formal_tool_binding_inventory_threshold
  - source:phase6_reserve_formal_tool_binding_inventory_current_first_choice
  - source:phase6_reserve_formal_tool_binding_inventory_minimum
  - helper_preview:actual_phase6_parser_side_followup_package_sequencing_threshold
  - source:phase6_parser_side_followup_package_sequencing_current_first_choice
  - source:phase6_parser_side_followup_package_sequencing_minimum
  compare_floor_refs:
  - compare_floor:current_l2.closeout.phase4_shared_space_self_driven_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase5_proof_protocol_runtime_policy_handoff_closeout
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_parser_ast_carrier_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_actual_checker_runtime_skeleton_first_tranche
  - compare_floor:current_l2.closeout.phase6_compile_ready_verification_and_formal_hook
  - compare_floor:current_l2.closeout.phase6_next_reopen_sequencing
  - compare_floor:current_l2.closeout.phase6_parser_second_tranche_attached_slot_and_predicate_fragment_first_package
  - compare_floor:current_l2.closeout.phase6_reserve_formal_tool_binding_inventory
  - compare_floor:current_l2.closeout.phase6_parser_side_followup_package_sequencing
  guard_refs:
  - guard:actual_phase6_reserve_formal_tool_binding_inventory_threshold_required
  - guard:phase6_parser_second_tranche_shared_single_attachment_frame_first_package_comparison_next
  kept_later_refs:
  - request_clause_suite_publicization
  - perform_head_final_public_parser_api
  - span_rich_diagnostics
  - source_sample_corpus_scope
  - final_public_parser_checker_runtime_surface
  guard_reason: none
non_admissible_metadata: []
```

### 出力の読み方

まず見るべき行は次である。

- `static_gate: valid`
  形としては current rule に受理されている。
- `entered_evaluation: true`
  runtime に入っている。
- `terminal_outcome: success`
  sample 自体は成功している。
- `debug_outputs:` の `delegated_rng_service.draw -> primary`
  provider から draw を取ったことが見える。
- `debug_outputs:` の `publish_roll_result: player_a -> visible`
  publish は authority 側で行われたことが見える。
- `debug_outputs:` の `handoff_dice_authority: player_a -> player_b`
  handoff も authority 側で行われたことが見える。

さらに current line で本当に重要なのは、helper preview の status である。

- `authoritative_room_first_scenario_actual_adoption.status: guarded_not_reached`
  first-line representative には昇格していない。
- `authoritative_room_reserve_strengthening_lane.delegated_rng_service_status: reached`
  delegated RNG service reserve としては reached している。
- `serial_scope_reserve.status: reached`
  serial-scope reserve surface ではこの route が読める。

つまり `p09` は、**成功 sample ではあるが、current default の代表例ではなく reserve route の代表例**である。
## 5. reserve package を順に見る

`order_01.md` と同じく、Problem 2 は representative pair だけで終わらない。current line では reserve lane が分離されている。ここでは 2 つの helper を順に見る。

### 5-1. auditable authority witness

```bash
python3 scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness
```

このコマンドが確認していること:

- `auditable_authority_witness` という strengthening を、representative default とは別 lane で読めるか
- `p07` で witness strengthening が reached しているか
- `p08` と `p05` を contrast / guard-only sample として区別できるか

実行例全文:

```text
auditable authority witness reserve package

`auditable_authority_witness` の minimal witness core strengthening を `p07 / p08 / p05` で再確認する repo-local reserve helper。

sample bundle doc: samples/problem-bundles/problem2-order-handoff-shared-space.md
command: python3 scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness
output dir: target/current-l2-guided/reserve-packages/auditable-authority-witness
package summary markdown: target/current-l2-guided/reserve-packages/auditable-authority-witness/package-summary.md
package summary json: target/current-l2-guided/reserve-packages/auditable-authority-witness/package-summary.json
room profile claim: fairness_claim = auditable_authority_witness

witness core fields:
- witness_kind
- action_ref
- draw_slot
- draw_result

witness binding refs:
- witness_binding:witness_kind:authority_draw_witness
- witness_binding:action_ref:handoff_dice_authority@dice_state
- witness_binding:draw_slot:primary
- witness_binding:draw_result_binding:publish_roll_result@dice_state

- p07-dice-late-join-visible-history: witness-strengthening reached
  output: target/current-l2-guided/reserve-packages/auditable-authority-witness/p07-dice-late-join-visible-history.run.json
  static_gate: valid
  terminal_outcome: success
  reserve_detail: witness+model-check
  witness_strengthening_status: reached
  first_line_status: reached
  reserve_lane_status: reached

- p08-dice-stale-reconnect-refresh: guard-only non-witness-bearing contrast
  output: target/current-l2-guided/reserve-packages/auditable-authority-witness/p08-dice-stale-reconnect-refresh.run.json
  static_gate: valid
  terminal_outcome: success
  reserve_detail: model-check
  witness_strengthening_status: guarded_not_reached
  first_line_status: reached
  reserve_lane_status: reached

- p05-dice-owner-guarded-chain: guard-only pre-default comparison
  output: target/current-l2-guided/reserve-packages/auditable-authority-witness/p05-dice-owner-guarded-chain.run.json
  static_gate: valid
  terminal_outcome: success
  reserve_detail: guarded
  witness_strengthening_status: guarded_not_reached
  first_line_status: guarded
  reserve_lane_status: guarded

stop line:
- final public witness schema
- final public provider receipt schema
- final public witness/provider/artifact contract
- delegated_rng_service practical package
- distributed fairness theorem

anchor refs:
- specs/examples/476-current-l2-auditable-authority-witness-strengthening-actualization.md
- specs/examples/571-current-l2-authoritative-room-reserve-strengthening-lane-tightening.md
- specs/examples/610-current-l2-auditable-authority-witness-reserve-package-summary-index-actualization.md

注意:
- `p07` reached、`p08 / p05` guard-only の strengthening cut を repo-local summary に materialize する narrow helper である。
- final public witness schema、provider receipt schema、delegated RNG practical package には上げない。
```

#### `witness_strengthening_status` の意味

- `reached`
  witness strengthening の current reserve がその sample で実際に読める。
- `guarded_not_reached`
  その sample は reserve package の比較対象には入っているが、witness strengthening 自体は達成していない。

この helper 出力で重要なのは、`p07` だけが

- `witness_strengthening_status: reached`

になっていることだ。これは `p07` の publish-handoff-observe line が、witness strengthening の current reserve を支える代表例だと読んでいることを意味する。

逆に `p08` は stale reconnect の representative sample なので、

- `witness_strengthening_status: guarded_not_reached`

となる。つまり `p08` は useful ではあるが、witness strengthening の本体ではない。

### 5-2. delegated RNG service

```bash
python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service
```

このコマンドが確認していること:

- delegated RNG provider placement を reserve package として分離して読めるか
- `p09` で delegated provider placement が reached しているか
- `p07 / p08` は contrast として残しつつ、本線には昇格しないか

実行例全文:

```text
delegated RNG service reserve package

`delegated_rng_service` provider placement practical line を `p09 / p07 / p08` で再確認する repo-local reserve helper。

sample bundle doc: samples/problem-bundles/problem2-order-handoff-shared-space.md
command: python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service
output dir: target/current-l2-guided/reserve-packages/delegated-rng-service
package summary markdown: target/current-l2-guided/reserve-packages/delegated-rng-service/package-summary.md
package summary json: target/current-l2-guided/reserve-packages/delegated-rng-service/package-summary.json
room profile provider: fairness_source = delegated_rng_service
room profile claim: fairness_claim = opaque_authority_trust

provider boundary refs:
- provider_boundary:placement:delegated_rng_service
- provider_boundary:authority_keeps_commit
- provider_boundary:provider_returns_draw_not_state_transition
- provider_boundary:room_state_mutation_by_authority
- provider_boundary:witness_core_unchanged

optional attachment refs:
- optional_attachment:provider_draw_ref
- optional_attachment:provider_receipt_ref

- p09-dice-delegated-rng-provider-placement: delegated provider placement reached
  output: target/current-l2-guided/reserve-packages/delegated-rng-service/p09-dice-delegated-rng-provider-placement.run.json
  static_gate: valid
  terminal_outcome: success
  reserve_detail: delegated-rng+model-check
  delegated_rng_service_status: reached
  first_line_status: guarded
  reserve_lane_status: reached

- p07-dice-late-join-visible-history: guard-only authority-rng baseline contrast
  output: target/current-l2-guided/reserve-packages/delegated-rng-service/p07-dice-late-join-visible-history.run.json
  static_gate: valid
  terminal_outcome: success
  reserve_detail: witness+model-check
  delegated_rng_service_status: guarded_not_reached
  first_line_status: reached
  reserve_lane_status: reached

- p08-dice-stale-reconnect-refresh: guard-only reconnect contrast
  output: target/current-l2-guided/reserve-packages/delegated-rng-service/p08-dice-stale-reconnect-refresh.run.json
  static_gate: valid
  terminal_outcome: success
  reserve_detail: model-check
  delegated_rng_service_status: guarded_not_reached
  first_line_status: reached
  reserve_lane_status: reached

stop line:
- final public provider receipt schema
- delegated provider attestation public contract
- delegated_rng_service + auditable_authority_witness combined public contract
- distributed_randomness_provider
- control-plane separated carrier
- exhaustive shared-space catalog

anchor refs:
- specs/examples/477-current-l2-delegated-rng-service-practical-actualization.md
- specs/examples/571-current-l2-authoritative-room-reserve-strengthening-lane-tightening.md
- specs/examples/611-current-l2-delegated-rng-service-reserve-package-summary-index-actualization.md

注意:
- `p09` reached、`p07 / p08` guard-only の provider placement cut を repo-local summary に materialize する narrow helper である。
- final public provider receipt schema、delegated provider attestation public contract、combined witness/provider public contract には上げない。
```

#### `delegated_rng_service_status` の意味

- `reached`
  delegated RNG service の reserve practical route が、その sample で実際に成立している。
- `guarded_not_reached`
  delegated RNG service はその sample の current reading では本体ではない。

この helper 出力で最重要なのは、`provider boundary refs` で次を明示している点である。

- `provider_boundary:placement:delegated_rng_service`
- `provider_boundary:authority_keeps_commit`
- `provider_boundary:provider_returns_draw_not_state_transition`
- `provider_boundary:room_state_mutation_by_authority`

初心者向けに言い換えると、これは「外部 provider は draw を返すだけで、room の state transition そのものは authority 側が責任を持つ」という宣言である。

なぜこの切り分けが必要か。具体例で言うと、もし外部 RNG service が「乱数を返す」だけでなく「そのまま room state を確定した」と読まれると、authority handoff と provider attachment の責任境界が崩れる。current line はそこを分けている。

## 6. なぜ今は low-level `memory_order` を直接出していないのか

初心者が Problem 2 を読むと、「これは結局 memory order の話ではないのか」と感じやすい。理解としては半分正しい。しかし current repo の public-near line は、それを source surface の前面には出していない。

理由は単純で、今 reader に見せたい失敗例が high-level relation の方がはるかに読みやすいからである。

- publish witness がないのに late join で見せたい
- publish より先に handoff してしまった
- delegated provider と authority commit を混同してしまった
- stale reconnect を refresh せず silent merge してしまった

これらは low-level order 語彙だけでも説明できるが、current research abstract では room-level / scenario-level の高水準 relation として説明した方が誤読が少ない。

したがって current line では、

- `publish`
- `handoff`
- `observe`
- `witness`
- `authoritative-room`

を前面に出し、low-level `memory_order` exact surface は later に残している。

## 7. 出力の項目を丁寧に読む

ここでは `order_01.md` で重要とされた出力項目を、初心者向けに一つずつ整理する。

### `static_gate`

これは static checker の最初の関門である。`scripts/current_l2_guided_samples.py` でも `checker_floor.static_gate.verdict` を引いている。

- `valid`
  static gate を通った。少なくとも current rule で見て、必要な宣言や順序の形は受理された。
- `underdeclared`
  必要な根拠が足りない。`p13` がこれで、publish witness が足りない。
- `malformed`
  構造や順序そのものが壊れている。`p14` がこれで、handoff-before-publish になっている。

### `entered_evaluation`

これは runtime evaluation に入ったかどうかである。

- `true`
  static gate を越えて runtime まで進んだ。
- `false`
  runtime に入る前に止まった。

`specs/examples/17-current-l2-detached-exporter-entry-comparison.md` と `specs/examples/37-current-l2-detached-bundle-transform-helper.md` でも、この項目は `RunReport` payload core の一部として扱われている。

### `terminal_outcome`

runtime の終端結果である。

- `success`
  runtime が成功して終わった。
- `none`
  runtime 終端結果が存在しない。多くの場合、static stop して runtime に入っていない。

`scripts/current_l2_guided_samples.py` では `runtime.terminal_outcome` が `None` なら文字列として `none` に変換している。

### `steps_executed`

実行された step 数である。`0` なら static stop で runtime に進んでいない、と読める。

ここで注意したいのは、`steps_executed` は event の件数と完全には一致しないことがある点である。helper は interpreter 側の step count を出しているので、目に見える `events:` より多いことがある。初心者はまず「0 かどうか」「他 sample よりどれくらい進んだか」を読むとよい。

### `formal_hook_status`

これは `verification_preview` における formal hook preview の状態である。

- `reached`
  theorem / model-check へ渡すための **tool-neutral preview 入口** には到達している。
- ただし、これは theorem や model-check の最終完了ではない。

特に大事なのは、`formal_hook_status: reached` は success sample にだけ出るわけではない、という点である。

- `p07`, `p08`, `p09` では `subject_kind: runtime_try_cut_cluster`
  runtime まで進んだ cluster が subject である。
- `p13`, `p14` では `subject_kind: fixture_static_cluster`
  静的に reject された cluster が subject である。

つまり、formal hook は「runtime success の印」ではなく、「現在の sample から formal preview を組める」という印である。

### `witness_strengthening_status`

これは reserve lane の中で、auditable authority witness strengthening がどこまで reached しているかを示す。

- `reached`
  witness strengthening の代表例として読める。
- `guarded_not_reached`
  reserve package 内の比較対象ではあるが、witness strengthening 本体ではない。

### `first_line_status`

これは first-line representative current default にその sample が入っているかを示す。

- `reached`
  本線に入っている。
- `guarded`
  本線には入っていない。

`p09` は success だが `first_line_status: guarded` である。この一点が、「success sample だから first line」という早合点を防ぐ。

### `reserve_lane_status`

これは reserve strengthening lane まで含めて読めるかを示す。

- `reached`
  reserve lane の読みがある。
- `guarded`
  reserve lane にも到達していない。

`p09` が `reserve_lane_status: reached` なのは、delegated RNG service reserve の成功例だからである。

## 8. `publish / handoff / observe` の因果関係を sample 横断で整理する

### `p07` の因果

- 5 行目から 7 行目で publish を宣言する
- 9 行目の `atomic_cut` で publish 段を切る
- 11 行目から 13 行目で handoff を宣言する
- 15 行目の `atomic_cut` で handoff 段を切る
- 17 行目から 19 行目で observe を宣言する

したがって `p07` は、**publish -> handoff -> observe** を守っている。

### `p13` の失敗点

- handoff はある
- observe もある
- しかし publish がない

つまり、「observe が要求する published result の根拠が足りない」。これが `underdeclared` である。

### `p14` の失敗点

- handoff もある
- publish もある
- observe もある
- しかし handoff が publish より先にある

つまり、「必要要素はあるが並び方が壊れている」。これが `malformed` である。

### `p09` の注意点

`p09` は publish -> handoff を保っているが、その前に provider から draw を取る段がある。ここで current line が守りたいのは、「provider は draw を返すだけで、commit と handoff は authority が持つ」という境界である。

## 9. どこで何を宣言しているか

最後に、ユーザーが再確認しやすいように、「どこで何を宣言しているか」を短い引用付きで整理する。

### publish witness の宣言

publish witness は独立 keyword ではなく、publish を先に置き、その結果を visible にし、その後段で late join visibility を読むことで成立している。

主な場所:

- `p07` 5 行目から 9 行目
  - `perform publish_roll_result on dice_state`
  - `ensure result_is_visible(room_members)`
  - `atomic_cut`
- `p09` 15 行目から 19 行目
  - `perform publish_roll_result on dice_state`
  - `ensure result_is_visible(room_members)`
  - `atomic_cut`

この宣言が足りない例:

- `p13`
  - handoff と observe はあるが publish 行が存在しない

### handoff の宣言

主な場所:

- `p07` 11 行目から 13 行目
  - `perform handoff_dice_authority on dice_state`
  - `ensure owner_is(player_b)`
- `p09` 21 行目から 23 行目
  - `perform handoff_dice_authority on dice_state`
  - `ensure owner_is(player_b)`
- `p14` 6 行目から 8 行目
  - `perform handoff_dice_authority on dice_state`
  - `ensure owner_is(player_b)`

`p14` は handoff 自体の宣言が誤りなのではなく、**置き場所が publish より前なのが誤り**である。

### late join visibility の宣言

主な場所:

- `p07` 17 行目から 19 行目
  - `perform observe_late_join_view on dice_state`
  - `ensure late_join_sees_published_result(player_c)`
- `p13` 12 行目から 14 行目
  - `perform observe_late_join_view on dice_state`
  - `ensure late_join_sees_published_result(player_c)`
- `p14` 18 行目から 20 行目
  - `perform observe_late_join_view on dice_state`
  - `ensure late_join_sees_published_result(player_c)`

ここで `p13` と `p14` は late join visibility の**要求文**自体は書いてある。しかし current line では、

- `p13`: それを支える publish witness がない
- `p14`: それを支える publish の位置が handoff より後ろ

なので止まる。

### reserve practical route の宣言

主な場所:

- `p09` 5 行目
  - `option delegated_rng on dice_state capability read lease live admit provider_online(room_epoch)`
- `p09` 7 行目
  - `chain delegated_rng_ref = delegated_rng`
- `p09` 9 行目から 11 行目
  - `perform fetch_provider_roll via delegated_rng_ref`
  - `ensure provider_draw_available(primary)`
- `emit-reserve delegated-rng-service` 出力
  - `provider_boundary:placement:delegated_rng_service`
  - `provider_boundary:authority_keeps_commit`
  - `provider_boundary:provider_returns_draw_not_state_transition`

つまり reserve practical route は、「provider を attach できる」ことだけではなく、「provider は draw を返すが commit と state transition は authority が持つ」という境界まで含めて宣言されている。

## 10. まとめ

`order_01.md` の current reading を、初心者向けに一番短く言い直すと次の通りである。

- `p07` は publish してから handoff し、その後で late joiner が履歴を読める representative success である。
- `p08` は stale reconnect を silent merge せず、fail then refresh にする representative success である。
- `p09` は delegated RNG provider を使うが、authority commit は authority 側に残す reserve practical route である。
- `p13` は publish witness 不足なので `underdeclared` で止まる。
- `p14` は handoff-before-publish なので `malformed` で止まる。

そして一番大事なのは、Problem 2 が「何となく順序が大事そう」という話ではなく、**publish / handoff / observe のどれをどの前後関係で置くかを、sample と helper と static stop pair で明示する current cut** だという点である。
