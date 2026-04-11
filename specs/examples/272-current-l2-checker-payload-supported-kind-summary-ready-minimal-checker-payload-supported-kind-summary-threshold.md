# 272 — current L2 checker-payload-supported-kind-summary-ready minimal-checker-payload-supported-kind-summary threshold

## 目的

`specs/examples/271-current-l2-minimal-checker-payload-row-body-ready-checker-payload-supported-kind-summary-comparison.md`
で checker payload supported kind summary を
`payload_row_family_ref` keyed な独立 summary line
として切る判断を fixed した次段として、

- checker payload supported kind summary の minimal field core をどこまでに留めるか
- current stable subset の境界をどこまで explicit に持たせるか
- public checker payload schema をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 checker-side line の
checker-payload-supported-kind-summary-ready minimal-checker-payload-supported-kind-summary threshold** であり、

- public checker payload schema
- public checker API
- final checker implementation contract

はまだ固定しない。

## 比較観点

1. row family と自然に接続しているか
2. current stable subset inventory を source-backed に見せる最小 summary になっているか
3. source bridge や per-row payload を duplicate に混ぜないか
4. next 段の public checker payload schema comparison を narrow に保てるか

## 比較対象

### 案 1. `payload_row_family_ref + supported_kind_refs` だけを持つ minimal summary

#### shape

```text
actual_checker_payload_supported_kind_summary = {
  payload_row_family_ref = actual_checker_payload_row_family,
  supported_kind_refs = [
    missing_lineage_assertion,
    lineage_assertion_edge_mismatch,
    declared_target_missing,
    declared_target_mismatch,
    capability_strengthens,
    missing_chain_head_option,
    missing_predecessor_option,
    missing_successor_option
  ]
}
```

#### 利点

- 最も軽い
- stable kind inventory 自体は見せられる

#### 欠点

- detached `reason_codes_scope = stable-clusters-only` との接点が still implicit になる
- current summary が final exhaustive catalog と誤読されやすい

### 案 2. `payload_row_family_ref + supported_kind_scope + supported_kind_refs` を持つ minimal summary

#### shape

```text
actual_checker_payload_supported_kind_summary = {
  payload_row_family_ref = actual_checker_payload_row_family,
  supported_kind_scope = stable_clusters_only,
  supported_kind_refs = [
    missing_lineage_assertion,
    lineage_assertion_edge_mismatch,
    declared_target_missing,
    declared_target_mismatch,
    capability_strengthens,
    missing_chain_head_option,
    missing_predecessor_option,
    missing_successor_option
  ]
}
```

#### 利点

- row family ref と supported inventory の接点を明示しつつ、current stable subset 境界も explicit にできる
- detached `reason_codes_scope` と `is_supported_checked_reason_code` の source-backed reading に合う
- source refs や per-row payload は family / detail / body 側に残せる
- public checker payload schema を still later に残せる

#### 欠点

- `supported_kind_scope` naming discipline が要る
- kind list だけよりは 1 段重い

### 案 3. `payload_row_family_ref + supported_kind_scope + supported_kind_refs + source_support_map` まで持たせる

#### 利点

- source family と stable subset inventory の接点を最も直接に見せられる

#### 欠点

- `payload_family_kind + source_refs` minimal bundle と役割が重なりやすい
- source ごとの差分がない current repo に対して過剰である
- public checker payload schema の先食いになりやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `payload_row_family_ref + supported_kind_scope + supported_kind_refs` を持つ minimal summary**
である。

## current first choice shape

```text
actual_checker_payload_supported_kind_summary_ready_sketch = {
  actual_checker_payload_supported_kind_summary = {
    payload_row_family_ref = actual_checker_payload_row_family,
    supported_kind_scope = stable_clusters_only,
    supported_kind_refs = [
      missing_lineage_assertion,
      lineage_assertion_edge_mismatch,
      declared_target_missing,
      declared_target_mismatch,
      capability_strengthens,
      missing_chain_head_option,
      missing_predecessor_option,
      missing_successor_option
    ]
  }
}
```

### この shape でまだ入れないもの

- `source_support_map`
- public checker payload schema wrapper
- public checker API contract

これらは still later である。

## practical reading

current summary line が示すのは、

- どの row family の supported inventory か
- その inventory が `stable_clusters_only` scope に限定されること
- current supported kind set が何か

の 3 点だけである。

fixture source / detached source の bridge は payload family 側に、
row ごとの actual kind と body は row detail / row body 側に残す。

## next promoted line

next promoted line は、
**minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison**
に置く。

## open questions

- `supported_kind_scope` を detached `reason_codes_scope` とどこまで naming 共有するか
- `supported_kind_refs` の token order を current phase で stable にしてよいか
- public checker payload schema を summary の後段でどの粒度から reopen するか
