# 368 — current L2 model-check-concrete-carrier-first-actualization-gate-ready minimal-model-check-concrete-carrier-first-actualization-gate threshold

## 目的

`specs/examples/367-current-l2-shared-space-identity-auth-layering-reopen-ready-model-check-concrete-carrier-first-actualization-gate-comparison.md`
で model-check concrete carrier first actualization gate の current first choice を fixed した次段として、

- first actualization gate の minimum をどこまでに留めるか
- current first pilot / gate entry / parallel reserve / guard を minimum にどう残すか
- repo-level next line と kept-later machine-facing line をどう handoff するか

を比較する。

ここで固定するのは
**current L2 model-check-concrete-carrier-first-actualization-gate-ready minimal-model-check-concrete-carrier-first-actualization-gate threshold**
であり、

- model-check concrete carrier actualization
- actual public-checker migration
- actual emitted verifier handoff artifact
- concrete theorem / model-check tool binding
- bless / review-session metadata

はまだ固定しない。

## 比較観点

1. first actualization gate の cut を lossless に minimum へ残せるか
2. `proof_notebook_review_unit` current first pilot と model-check reopen gate を minimum に区別できるか
3. public-checker docs-only reserve chain を parallel reserve として保持できるか
4. repo-level next line を stable malformed broader follow-up inventory へ handoff できるか

## 比較対象

### 案 1. gate 名と current first pilot 名だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- entry、parallel reserve、guard、kept-later line が見えない。
- machine-facing drift suppression が弱い。

### 案 2. `gate_kind + entry_criteria_refs + current_first_pilot_refs + gate_entry_refs + gated_target_refs + parallel_reserve_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- first actualization gate の切り方を lossless に残せる。
- theorem-side current first pilot と machine-facing gate を minimum に区別できる。
- public-checker docs-only chain を parallel reserve に留める guard を minimum に残せる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. actual emitted carrier family や concrete tool binding candidate まで threshold に含める

#### 利点

- later actualization との接続は見えやすい。

#### 欠点

- threshold ではなく future actualization gate を先取りする。
- stable malformed broader follow-up inventory へ戻す current repo-level line を弱めやすい。

## current judgment

current L2 で最も自然なのは、
**案 2. `gate_kind + entry_criteria_refs + current_first_pilot_refs + gate_entry_refs + gated_target_refs + parallel_reserve_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は machine-facing gate の narrowing であり、actual carrier family や tool binding は kept-later に残すべきである。
2. `proof_notebook_review_unit` first concrete pilot を current carrier として minimum に残す必要がある。
3. public-checker docs-only chain を parallel reserve として明示しないと snapshot drift を起こしやすい。

## current first choice shape

```text
model_check_concrete_carrier_first_actualization_gate = {
  gate_kind = current_l2_model_check_concrete_carrier_first_actualization_gate,
  entry_criteria_refs = [
    model_check_public_checker_second_reserve_inventory,
    shared_space_identity_auth_layering_reopen
  ],
  current_first_pilot_refs = [
    proof_notebook_review_unit_first_concrete_pilot
  ],
  gate_entry_refs = [
    tool_neutral_formal_hook_only_input,
    compare_ready_docs_only_bridge_sketch
  ],
  gated_target_refs = [
    model_check_concrete_carrier
  ],
  parallel_reserve_refs = [
    public_checker_docs_only_chain
  ],
  guard_refs = [
    keep_proof_notebook_as_current_first_pilot,
    keep_public_checker_chain_docs_only,
    avoid_concrete_tool_binding,
    avoid_actual_emitted_verifier_handoff_artifact
  ],
  kept_later_refs = [
    model_check_concrete_carrier_actualization,
    actual_public_checker_migration,
    actual_emitted_verifier_handoff_artifact,
    bless_review_session_metadata,
    stable_malformed_broader_followup_inventory
  ]
}
```

## practical reading

current minimal model-check concrete carrier first actualization gate が示すのは、

- current first concrete carrier は `proof_notebook_review_unit` のままである
- model-check side は tool-neutral formal hook only input と compare-ready bridge sketchを entry にする narrow gate として読む
- public-checker side は docs-only reserve chain のまま parallel reserve に留める
- actual public-checker migration、actual emitted handoff artifact、concrete tool binding は still later に残す

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-model-check-concrete-carrier-first-actualization-gate-ready stable-malformed-broader-follow-up-inventory comparison**
に置く。

## open questions

- `parallel_reserve_refs` を public-checker docs-only chain title だけに留めるか、payload/API/boundary stop line まで mirror するか
- `gate_entry_refs` に compare basis family ref も current minimum で残すべきか
- model-check actualization line を reopen するとき、concrete tool binding と actual emitted artifact のどちらを先に inventory するか
