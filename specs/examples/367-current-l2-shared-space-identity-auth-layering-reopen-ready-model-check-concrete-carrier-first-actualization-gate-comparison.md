# 367 — current L2 shared-space-identity-auth-layering-reopen-ready model-check-concrete-carrier-first-actualization-gate comparison

## 目的

`specs/examples/366-current-l2-shared-space-identity-auth-layering-reopen-ready-minimal-shared-space-identity-auth-layering-reopen-threshold.md`
で shared-space identity/auth layering reopen の minimum を fixed した次段として、

- proof notebook first concrete pilot を current first carrier に保ったまま、model-check side の machine-facing reopen をどの gate から始めるのが自然か
- tool-neutral formal hook only input、compare-ready bridge sketch、public-checker docs-only reserve chain をどこで切り分けるのが current first choice か
- concrete tool binding、actual public-checker migration、actual emitted verifier handoff artifact をどこまで still later に残すべきか

を比較する。

ここで固定するのは
**current L2 shared-space-identity-auth-layering-reopen-ready model-check-concrete-carrier-first-actualization-gate comparison**
であり、

- model-check concrete carrier actualization
- actual public-checker migration
- actual emitted verifier handoff artifact
- concrete theorem / model-check tool binding
- bless / review-session metadata

はまだ固定しない。

## scope

- current package は docs-only gate package に留める。
- entry criteria は `specs/examples/327...328`、`341...342`、`347...348`、`359...360`、`365...366` に置く。
- `proof_notebook_review_unit` は current first concrete pilot のまま維持する。
- public-checker side は second reserve docs-only chain のまま保持し、actual public contract へ昇格させない。
- repo-level next line は stable malformed broader follow-up inventory に handoff し、shared-space admission / compile-time visibility reopen は reserve line として残す。

## current 前提

current repo では次が成立している。

1. `specs/examples/327...328` と `347...348` により、tool-neutral formal hook artifact を入力にする row-local `proof_notebook_review_unit` が theorem-side current first concrete pilot に fixed 済みである。
2. `specs/examples/341...342` により、compare-ready docs-only bridge sketch は `comparison_basis_refs` を持つ second reopen として fixed 済みである。
3. `specs/examples/359...360` により、machine-facing later line は model-check second reserve refs と public-checker second reserve refs に分けて inventory 済みである。
4. `specs/examples/365...366` により、shared-space identity/auth layering reopen も fixed 済みであり、repo-level current line は machine-facing reopen 側へ戻してよい状態にある。

したがって current 問いは、
**`proof_notebook_review_unit` current first pilot を巻き戻さずに、model-check concrete carrier side の first actualization gate を compare-ready bridge sketch 起点で narrow に切るのが自然か**
である。

## 比較観点

1. `proof_notebook_review_unit` current first concrete pilot を current carrier のまま保てるか
2. model-check side を reopen しつつ、concrete tool binding や actual public-checker migration を premature に要求しないか
3. public-checker docs-only reserve chain を parallel reserve として保てるか
4. repo-level next line を stable malformed broader follow-up inventory へ clean に handoff できるか

## 比較対象

### 案 1. tool-neutral formal hook only input から model-check concrete carrier へ直接 gate を切る

#### 利点

- gate の entry は最も短い。
- machine-facing pressure をすぐ見やすい。

#### 欠点

- compare-ready bridge sketch current line を飛び越えやすい。
- theorem-side current bridge と public-checker second reserve inventory の間に 1 段の整理が残りにくい。
- concrete tool binding や actual artifact 形へ drift しやすい。

### 案 2. compare-ready bridge sketch を含む first actualization gate だけを固定し、current first pilot と parallel reserve を保つ

#### 読み

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
    bless_review_session_metadata
  ]
}
```

#### 利点

- `proof_notebook_review_unit` current first concrete pilot を current carrier として保ったまま、model-check side の reopen point を 1 段 narrow に固定できる。
- tool-neutral formal hook と compare-ready bridge sketch の両方を entry に残せる。
- public-checker docs-only reserve chain を parallel reserve として保てる。
- concrete tool binding と actual public-checker migration を still later に押し戻しやすい。

#### 欠点

- gate layer が 1 つ増える。
- model-check concrete carrier 自体の actual shape は still later に残る。

### 案 3. model-check concrete carrier actualization か public-checker actual migration まで current package に含める

#### 利点

- machine-facing path をすぐ concrete に見られる。

#### 欠点

- `proof_notebook_review_unit` current first pilot を飛び越えやすい。
- public-checker docs-only chain を actual public contract と誤読しやすい。
- stable malformed broader follow-up inventory を unnecessary に遅らせやすい。

## current judgment

current L2 で最も自然なのは、
**案 2. compare-ready bridge sketch を含む first actualization gate だけを固定し、current first pilot と parallel reserve を保つ**
である。

理由は次の通り。

1. current actual code anchor を already 持つ first concrete carrier は依然として `proof_notebook_review_unit` である。
2. model-check side は tool-neutral formal hook only input と compare-ready docs-only bridge sketch を entry にする narrow gate として切る方が、theorem-side current lineと連続的である。
3. public-checker side は payload schema から verifier handoff surface までの docs-only chain を parallel reserve に留め、actual public migration と emitted artifact は still later に残す方が Phase 5/6 stop line と整合する。

## current first choice details

- `current_first_pilot_refs` は proof notebook review-unit first concrete pilot を 1 本の current carrier として保持する。
- `gate_entry_refs` は `tool_neutral_formal_hook_only_input + compare_ready_docs_only_bridge_sketch` に留める。
- `gated_target_refs` は `model_check_concrete_carrier` symbolic target に留め、actual carrier shape 自体は still later に残す。
- `parallel_reserve_refs` は public-checker docs-only chain をそのまま保持し、actual public-checker migration へ昇格させない。
- concrete tool binding、actual emitted verifier handoff artifact、bless / review-session metadata は kept-later に残す。

## next promoted line

next promoted line は、
**model-check-concrete-carrier-first-actualization-gate-ready stable-malformed-broader-follow-up-inventory comparison**
に置く。

## open questions

- `gate_entry_refs` に prior review-unit family ref も current minimum で残すべきか
- `model_check_concrete_carrier` symbolic target を actual emitted artifact family へ近づけるのはどの later gate が自然か
- public-checker side を actual migration へ戻す順序を public operational CLI / final public contract later gate とどう揃えるか
