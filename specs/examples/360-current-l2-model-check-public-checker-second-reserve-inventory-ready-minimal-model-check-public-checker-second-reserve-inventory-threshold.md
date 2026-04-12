# 360 — current L2 model-check/public-checker second reserve inventory-ready minimal-model-check/public-checker second reserve inventory threshold

## 目的

`specs/examples/359-current-l2-mirrorea-shared-space-docs-first-re-entry-ready-model-check-public-checker-second-reserve-inventory-comparison.md`
で model-check/public-checker second reserve inventory の current first choice を fixed した次段として、

- second reserve inventory の minimum をどこまでに留めるか
- current first pilot / model-check reserve / public-checker reserve / guard を minimum にどう残すか
- next promoted line と kept-later actualization line をどう handoff するか

を比較する。

ここで固定するのは
**current L2 model-check/public-checker second reserve inventory-ready minimal-model-check/public-checker second reserve inventory threshold**
であり、

- model-check concrete carrier actualization
- actual public checker migration
- actual emitted verifier handoff artifact
- concrete tool binding
- bless / review-session metadata

はまだ固定しない。

## 比較観点

1. second reserve inventory の cut を lossless に残せるか
2. proof notebook first pilot と later machine-facing reserve line を distinguish できるか
3. public-checker docs-only chain を actual public contract に誤昇格させない guard を minimum に残せるか
4. next mainline を stable-static edge-pair first reopen へ戻せるか

## 比較対象

### 案 1. current first pilot 名と reserve title だけを minimum に残す

#### 利点

- 軽い。

#### 欠点

- model-check side と public-checker side の bucket が見えない。
- guard と later actualization line が弱い。

### 案 2. `inventory_kind + entry_criteria_refs + current_first_pilot_refs + model_check_second_reserve_refs + public_checker_second_reserve_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- second reserve inventory の切り方を lossless に残せる。
- proof notebook current cut と later machine-facing line を minimum に区別できる。
- public-checker docs-only chain の accidental promotion を guard に残せる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. model-check concrete carrier candidate や public-checker actual migration candidate を minimum に含める

#### 利点

- later actualization との接続は見えやすい。

#### 欠点

- threshold ではなく future actualization gate を先取りする。
- stable-static side へ戻す current mainline を弱めやすい。

## current judgment

current L2 で最も自然なのは、
**案 2. `inventory_kind + entry_criteria_refs + current_first_pilot_refs + model_check_second_reserve_refs + public_checker_second_reserve_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は machine-facing reserve line の棚卸しであり、actualization candidate 自体は kept-later に残すべきである。
2. `proof_notebook_review_unit` first concrete pilot を current carrier として minimum に残す必要がある。
3. public-checker docs-only chain の accidental promotion を防ぐ guard が見えないと snapshot drift を起こしやすい。

## current first choice shape

```text
model_check_public_checker_second_reserve_inventory = {
  inventory_kind = current_l2_machine_facing_second_reserve_inventory,
  entry_criteria_refs = [
    proof_model_check_first_concrete_tool_pilot,
    parser_checker_runtime_public_surface_inventory
  ],
  current_first_pilot_refs = [
    proof_notebook_review_unit_first_concrete_pilot
  ],
  model_check_second_reserve_refs = [
    tool_neutral_formal_hook_only_input,
    compare_ready_docs_only_bridge_sketch,
    model_check_concrete_carrier
  ],
  public_checker_second_reserve_refs = [
    public_checker_payload_schema,
    public_checker_api_read_relation,
    public_checker_command_surface,
    shared_output_contract,
    public_checker_boundary,
    verifier_handoff_surface
  ],
  guard_refs = [
    keep_proof_notebook_as_first_concrete_pilot,
    keep_public_checker_chain_docs_only,
    avoid_concrete_tool_binding,
    avoid_actual_public_checker_promotion
  ],
  kept_later_refs = [
    model_check_concrete_carrier_actualization,
    actual_public_checker_migration,
    actual_emitted_verifier_handoff_artifact,
    bless_review_session_metadata
  ]
}
```

## practical reading

current minimal model-check/public-checker second reserve inventory が示すのは、

- current first concrete carrier は `proof_notebook_review_unit` のままである
- model-check side は tool-neutral formal hook / compare-ready bridge を entry にする second reserve である
- public-checker side は payload schema から verifier handoff surface までの docs-only chain を reserve bucket に留める
- actual public checker migration、actual emitted handoff artifact、concrete tool binding は still later に残す

という最小 cut である。

## next promoted line

next promoted line は、
**minimal-model-check-public-checker-second-reserve-inventory-ready stable-static-edge-pair-first-reopen comparison**
に置く。

## open questions

- `model_check_second_reserve_refs` に plain bridge sketch も current minimum で残すべきか
- `public_checker_second_reserve_refs` を verifier handoff surface stop line まで current threshold に含めるのが最小か
- machine-facing actualization gate を model-check side から先に reopen する current orderをどこまで snapshot に mirror するか
