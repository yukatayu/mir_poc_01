# 359 — current L2 Mirrorea/shared-space docs-first re-entry-ready model-check/public-checker second reserve inventory comparison

## 目的

`specs/examples/358-current-l2-mirrorea-shared-space-docs-first-re-entry-ready-minimal-mirrorea-shared-space-docs-first-re-entry-threshold.md`
で Mirrorea/shared-space docs-first re-entry の minimum を fixed した次段として、

- proof notebook first concrete pilot を巻き戻さずに machine-facing reserve line をどこまで inventory 化するか
- model-check side と public-checker migration side をどの bucket で分けるのが current first choice か
- concrete tool binding / actual public checker surface / emitted verifier handoff をどこまで still later に残すべきか

を比較する。

ここで固定するのは
**current L2 Mirrorea/shared-space docs-first re-entry-ready model-check/public-checker second reserve inventory comparison**
であり、

- model-check concrete carrier actualization
- actual public checker migration
- actual emitted verifier handoff artifact
- concrete tool binding
- bless / review-session metadata

はまだ固定しない。

## scope

- current package は docs-only reserve inventory に留める。
- theorem-side current first concrete pilot は `proof_notebook_review_unit` のまま維持する。
- public-checker side は existing docs-only chain と verifier handoff stop line を参照するだけに留め、actual public contract へ昇格させない。
- stable-static edge-pair reopen と public operational surface actualization gate は次段 mainline に残す。

## current 前提

current repo では次が成立している。

1. theorem-side current first concrete pilot は `specs/examples/327...328` と `347...348` により、tool-neutral formal hook を入力にした row-local `proof_notebook_review_unit` に fixed 済みである。
2. theorem-side docs-only bridge は `specs/examples/339...342` により plain / compare-ready bridge sketch まで fixed 済みである。
3. public-checker side は `specs/examples/271...286` と `297...298` により、supported-kind summary から verifier handoff surface まで docs-only stop line が固定済みである。
4. `specs/examples/356` により、public-looking surface を `pub visibility != final public contract` の guard 付き inventory として読む current repo-level judgmentがある。

したがって current 問いは、
**proof notebook first concrete pilot を current first carrier に保ったまま、model-check side と public-checker side を second reserve inventory としてどう並べるのが自然か**
である。

## 比較観点

1. proof notebook first concrete pilot を current first carrier に保てるか
2. model-check side と public-checker migration side を同一 bucket に潰さずに整理できるか
3. public-checker docs-only chain を actual public contract に誤昇格させないか
4. next promoted line を stable-static edge-pair first reopen に戻せる程度に narrow に閉じられるか

## 比較対象

### 案 1. proof notebook current cut だけを残し、second reserve inventory 自体は切らない

#### 利点

- source docs は増えない。

#### 欠点

- machine-facing later line の順序が snapshot で prose 依存に残る。
- model-check side と public-checker side の違いが tasks / progress で drift しやすい。

### 案 2. current first pilot の下に model-check second reserve refs と public-checker second reserve refs を分けて置く

#### 読み

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

#### 利点

- proof notebook first concrete pilot を current carrier として固定したまま later machine-facing line を lossless に整理できる。
- model-check side は theorem-side current bridge / hook を entry にした second reserve として読める。
- public-checker side は payload schema から verifier handoff surface までの docs-only chain を reserve bucket に隔離できる。
- actual public checker surface、actual emitted handoff artifact、tool binding を later に押し戻しやすい。

#### 欠点

- bucket が増える。
- model-check side の concrete carrier shape 自体は still later に残る。

### 案 3. model-check side か public-checker side のどちらかを immediately actualization line に昇格する

#### 利点

- machine-facing pressure を先に concrete に見られる。

#### 欠点

- proof notebook first concrete pilot の current cut を飛び越えやすい。
- public-checker docs-only chain を actual public contract と誤読しやすい。
- stable-static / public operational surface side の mainline を unnecessary に遅らせやすい。

## current judgment

current L2 で最も自然なのは、
**案 2. current first pilot の下に model-check second reserve refs と public-checker second reserve refs を分けて置く**
である。

理由は次の通り。

1. actual code path を already 持つ first concrete carrier は依然として `proof_notebook_review_unit` である。
2. model-check side は theorem-side current bridge を起点にする machine-facing second reserve として切るのが自然である。
3. public-checker side は docs-only payload/API/command/output/boundary/handoff chain を reserve bucket に留め、actual public migration は後段へ残す方が Phase 5/6 stop line と整合する。

## current first choice details

- `current_first_pilot_refs` は proof notebook review-unit pilot を 1 本の current carrier として保持する。
- `model_check_second_reserve_refs` は tool-neutral formal hook input と compare-ready bridge sketch を entry にしつつ、model-check concrete carrier 自体は still reserve に留める。
- `public_checker_second_reserve_refs` は `payload_schema -> API(read relation) -> command_surface -> shared_output_contract -> public_checker_boundary -> verifier_handoff_surface` docs-only chain を参照するだけに留める。
- actual public checker migration、actual emitted verifier handoff artifact、concrete tool binding、bless/session metadata は still later に残す。

## next promoted line

next promoted line は、
**model-check-public-checker-second-reserve-inventory-ready stable-static-edge-pair-first-reopen comparison**
に置く。

## open questions

- model-check side の first actualization gate を compare-ready bridge sketch から始める current orderをどこまで minimum に残すか
- public-checker second reserve bucket に verifier handoff surface を current stop ref として含めるのが最小十分か
- concrete tool binding を model-check side と public-checker sideのどちらへ先に寄せるべきか
