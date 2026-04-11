# 219 — current L2 theorem line checker-verdict-transport-channel-body-ready higher-level-async-control-family comparison

## 目的

`specs/examples/218-current-l2-theorem-line-checker-verdict-transport-channel-body-ready-low-level-memory-order-family-threshold.md`
までを前提に、

- checker-verdict-transport-channel-body-ready retained bridge の次段として
  higher-level async-control family をどこから比較するのが最小か
- `event_tree_execution_view` / `authority_serial_transition_family` /
  `witness_aware_commit_family` のどれを current first cut に置くのが自然か
- low-level memory-order family を still later candidate に残したまま、
  theorem-line retained bridge をどこへ narrow に伸ばしてよいか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
checker-verdict-transport-channel-body-ready higher-level-async-control-family comparison**
であり、

- final async-control vocabulary
- final scheduler / replay / fairness semantics
- final shared-space consistency catalog
- final public checker API

はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- checker-facing retained bridge の次段としてだけ higher-level async-control family を比較する。
- `atomic_cut` の current meaning は変えない。
- shared-space の final activation / authority / consistency catalog は固定しない。

## current 前提

current repo では次が成立している。

1. theorem-line retained bridge の current stop line は
   `retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body`
   である。
2. low-level memory-order family は theorem-line retained bridge に still 入れない。
3. `docs/reports/0358-async-control-memory-boundary-inventory.md` と
   `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
   により、higher-level async-control は
   low-level memory-order と別 line で比較する current reading が固定済みである。
4. Phase 4 current package により、shared-space 側では
   `single room authority`、
   `authoritative serial transition`、
   `auditable_authority_witness`
   まで source-backed に切れている。

したがって current 問いは、
**theorem-line retained bridge の次段として、higher-level async-control family のどれを first cut に置くと、Phase 4 shared-space baseline と Phase 5 proof boundary の双方に最も自然につながるか**
である。

## 比較観点

1. `atomic_cut` の place-local cut を保ったまま higher-level async-control を扱えるか
2. theorem-line retained bridge に source-of-truth 寄りの family を先に置けるか
3. shared-space baseline package (`121...125`) と接続しやすいか
4. derived execution / explanation view を premature に source-of-truth と誤読しないか
5. fairness witness を early に押し込みすぎないか

## 比較対象

### 案 1. `event_tree_execution_view` を first cut にする

#### 読み

leaf-to-root event bubbling / replay / debug / explanation view を
theorem-line retained bridge の first higher-level async-control family に置く。

#### 利点

- user-facing explanation には最も見えやすい
- tree-like room view や replay / graph export hook と接続しやすい
- derived execution / debug view を早く比較できる

#### 欠点

- source-of-truth ではなく **derived view** を first cut にすることになる
- authoritative room の ordering / commit source を先に切れていない
- theorem-line retained bridge に説明用 surface を先に混ぜやすい

### 案 2. `authority_serial_transition_family` を first cut にする

#### 読み

authoritative room で、

- authority が request / lock / commit / publish を serial に owner すること
- commit ordering が room authority line で first-class に読めること

を higher-level async-control family の first cut に置く。

#### 利点

- shared-space Phase 4 baseline と最も直接つながる
- source-of-truth 寄りであり、derived execution view より先に置くのが自然
- `witness_aware_commit_family` を later stronger family として残しやすい
- `atomic_cut` の local cut と authority-level serial transition をきれいに分けられる

#### 欠点

- authoritative room に寄った family なので、append-friendly room の読みは still later
- fairness witness や replay view を immediate には扱えない

### 案 3. `witness_aware_commit_family` を first cut にする

#### 読み

commit が

- witness claim
- receipt / replay attachment
- fairness explanation

と結びつく family を first cut にする。

#### 利点

- fairness / auditable draw の line とつながりやすい
- authoritative room と delegated provider の双方に橋を掛けやすい
- theorem-line retained bridge に consumer-facing claim surface を近づけやすい

#### 欠点

- `authority_serial_transition_family` より一段強い
- witness / receipt / replay を early に押し込みやすい
- source-of-truth ordering と witness attachment を同時に扱うことになる

## current judgment

current L2 で最も自然なのは、
**案 2. `authority_serial_transition_family` を first cut にする**
である。

理由は次の通り。

1. theorem-line retained bridge の next step としては、derived execution view より source-of-truth ordering family を先に切る方が自然である
2. `authority_serial_transition_family` は Phase 4 の authoritative room baseline と直接つながり、shared-space と proof boundary の両方に bridge を掛けやすい
3. `witness_aware_commit_family` は `authority_serial_transition_family` を土台にした stronger family として second candidate に残す方が narrow である

## current order

current order は次とする。

1. `authority_serial_transition_family`
2. `witness_aware_commit_family`
3. `event_tree_execution_view`

ここで 3 番目を later に置く理由は、
`event_tree_execution_view` が current repo では
**derived execution / debug / explanation view**
として読む方が自然であり、
source-of-truth ordering family より先に retained bridge へ乗せる必要がないからである。

## practical examples

### example A — authoritative room の roll request

```text
authoritative room:
  authority-ack
  single room authority
  authoritative serial transition
  authority_rng

request:
  roll(player = P)
```

この line で first question になるのは、

- authority が request / lock / commit / publish をどの serial transition family で owner するか

であり、
event bubbling view や witness attachment ではない。

### example B — witness-aware draw は second family でよい

```text
witness = {
  witness_kind = auditable_authority_witness,
  action_ref   = roll#42,
  draw_slot    = primary,
  draw_result  = 4
}
```

この witness は重要だが、
first cut で必要なのは witness そのものより
**その witness が attach される commit ordering family**
である。

## next promoted line

next promoted line は、
**higher-level-async-control-family-ready authority-serial-transition-family threshold**
に置く。

## open questions

- `authority_serial_transition_family` を theorem-line retained bridge に足すとき、
  family marker に留めるか、minimal contract row まで足すか
- `witness_aware_commit_family` を later second candidate に残すとき、
  witness bundle / provider receipt / replay claim のどこから narrow に reopen するか
- `event_tree_execution_view` を derived execution / debug view として
  theorem-line へどう接続するか
