# 220 — current L2 theorem line higher-level-async-control-family-ready authority-serial-transition-family threshold

## 目的

`specs/examples/219-current-l2-theorem-line-checker-verdict-transport-channel-body-ready-higher-level-async-control-family-comparison.md`
で `authority_serial_transition_family` を current first cut とした判断を前提に、

- theorem-line retained bridge に authority-serial transition family をどこまで近づけるか
- family marker に留めるか、minimal contract row まで足すか
- witness / replay / fairness attachment をどこまで still later に残すか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
higher-level-async-control-family-ready authority-serial-transition-family threshold**
であり、

- actual authority protocol row
- final room ordering semantics
- witness / replay / fairness attachment
- public checker / theorem export API

はまだ固定しない。

## scope

- theorem-line retained bridge の next step としてだけ authority-serial transition family を比較する。
- authoritative room baseline を参照するが、Phase 4 の final catalog は固定しない。
- event-tree execution view と witness-aware commit family は second/later candidate に残す。

## 比較観点

1. source-of-truth ordering family を narrow に bridge へ乗せられるか
2. witness / replay / fairness attachment を premature に混ぜないか
3. theorem-line retained bridge を docs-only symbolic ref family に保てるか
4. Phase 4 authoritative room baseline と整合するか

## 比較対象

### 案 1. comparison judgment だけを残し、bridge には field を足さない

#### 利点

- retained bridge は最も軽い
- authority-serial transition family を premature に shape 化しない

#### 欠点

- higher-level async-control family の first cut が retained bridge から見えない
- next reopen line が prose 依存になりやすい

### 案 2. `authority_serial_transition_family` marker を持つ symbolic ref family を足す

#### 読み

`retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body`
の次段として、次だけを足す。

```text
proof_notebook_bridge_retained_payload_theorem_export_authority_serial_transition_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_channel_body_ready_sketch,
  retained_payload_body_materialization_theorem_export_authority_serial_transition_family = {
    theorem_export_checker_verdict_transport_channel_body_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body,
    family_kind = authority_serial_transition_family
  }
}
```

#### 利点

- theorem-line retained bridge に next async-control family が見える
- actual authority protocol row を持ち込まずに済む
- witness-aware commit family と event-tree execution view を still later に残しやすい

#### 欠点

- family marker だけでは actual contract row は見えない
- room profile catalog と 1 対 1 に対応すると誤読される余地がある

### 案 3. minimal authority-serial transition contract row を足す

#### 利点

- theorem-line retained bridge から actual contract row に近づけやすい
- authoritative room baseline と直結して見える

#### 欠点

- actual authority protocol / replay / witness claim を early に押し込みやすい
- docs-only symbolic retained bridge の narrow line を崩しやすい

## current judgment

current L2 で最も自然なのは、
**案 2. `authority_serial_transition_family` marker を持つ symbolic ref family を足す**
である。

理由は次の通り。

1. theorem-line retained bridge の next promoted line を source-backed に見せられる
2. actual authority protocol row や witness attachment はまだ later に残せる
3. current bridge の基本方針である symbolic ref family を維持できる

## current first choice shape

```text
proof_notebook_bridge_retained_payload_theorem_export_authority_serial_transition_family_ready_sketch = {
  ...,
  retained_payload_body_materialization_theorem_export_authority_serial_transition_family = {
    theorem_export_checker_verdict_transport_channel_body_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body,
    family_kind = authority_serial_transition_family
  }
}
```

### この shape でまだ入れないもの

- `authority_scope`
- `authority_owner_ref`
- `serial_transition_contract_row`
- witness / replay / fairness attachment

これらは still later である。

## practical example

```text
authoritative room:
  authority-ack
  single room authority
  authoritative serial transition
```

ここで theorem-line retained bridge に今ほしいのは、
「この room line が higher-level async-control family として
authority-serial transition family に属する」
という **family-level bridge** である。

authority owner の actual slot detail や replay witness までは、
まだ current task で扱わない。

## next promoted line

next promoted line は、
**authority-serial-transition-family-ready authority-serial-transition-contract comparison**
に置く。

## open questions

- `authority_serial_transition_contract` の minimal row core を
  `obligation_kind + authority_subject_ref + transition_kind`
  程度に留めるべきか
- `witness_aware_commit_family` を later reopen するとき、
  authority-serial transition contract からどこまで shared row を再利用するか
