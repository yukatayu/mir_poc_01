# 217 — current L2 theorem line checker-verdict-transport-receipt-ready checker-verdict-transport-channel-body threshold

## 目的

`specs/examples/216-current-l2-theorem-line-checker-verdict-transport-payload-ready-checker-verdict-transport-receipt-threshold.md`
までを前提に、

- checker-verdict-transport-receipt-ready retained bridge に checker verdict transport channel body をどこまで近づけるか
- checker verdict transport channel body を retained bridge に入れるべきか
- low-level memory-order family を同じ reopen で呼ぶべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の checker-verdict-transport-receipt-ready checker-verdict-transport-channel-body threshold** であり、
low-level memory-order family はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- checker-facing contract は `theorem_export_checker` に限る。
- checker verdict transport channel body は theorem-side retained bridge の field としてのみ扱う。
- low-level memory-order family は巻き込まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_theorem_export_checker_verdict_transport_receipt` までは current first choice に入っている。
2. checker verdict transport receipt は bridge 側へ入るが、checker verdict transport channel body と low-level memory-order family は still 後段に残す cut が固定済みである。
3. checker verdict transport channel body は、checker verdict transport receipt の直後に置ける next practical channel-body candidate として最も近い。

したがって current 問いは、
**checker verdict transport receipt の次段として checker verdict transport channel body を retained bridge に近づけるなら、それを current first cut にするべきか、それとも low-level memory-order family まで同時に呼ぶべきか**
である。

## 比較観点

1. checker verdict transport receipt と checker verdict transport channel body の line を narrow に切れるか
2. checker verdict transport channel body を retained bridge に足しても、low-level memory-order family を still 後段に残せるか
3. theorem-line retained bridge に low-level memory-order family を premature に押し込まないか
4. next later reopen を checker-verdict-transport-channel-body-ready low-level-memory-order-family comparison へ狭く進められるか

## 比較対象

### 案 1. checker verdict transport receipt を terminal cut にし、checker verdict transport channel body も bridge 外へ残す

#### 利点

- current bridge を最も動かさない
- checker verdict transport channel body の premature actualization を避けられる

#### 欠点

- checker verdict transport receipt と checker verdict transport channel body の gap が prose 依存に残りやすい
- transport line の next concrete candidate が current bridge で見えない

### 案 2. checker verdict transport channel body だけを持つ retained bridge にする

#### 読み

checker-verdict-transport-receipt-ready retained bridge に、
low-level memory-order family を導入せず
**`retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_channel_body_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_receipt_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body = {
    theorem_export_checker_verdict_transport_receipt_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_transport_receipt,
    channel_body_kind = checker_verdict_transport_channel_body
  }
}
```

#### 利点

- checker verdict transport channel body 自体は current bridge で見える
- low-level memory-order family を still 後段に残せる
- next later reopen を low-level-memory-order-family comparison へ狭く進めやすい

#### 欠点

- bridge-level field が 1 本増える
- channel body と memory-order family を誤読されない説明が要る

### 案 3. low-level memory-order family まで current bridge へ同時に入れる

#### 利点

- theorem-line から async-control のさらに低い層へ一気に接続できる
- eventual checker / verifier integration へ直接つながりやすい

#### 欠点

- checker verdict transport channel body と low-level memory-order family を同じ reopen で混ぜやすい
- theorem-line retained bridge を low-level concurrency vocabulary へ既成事実化しやすい
- next later reopen を narrow に保ちにくい

## current judgment

current L2 で最も自然なのは、
**案 2. checker verdict transport channel body だけを持つ retained bridge にする**
である。

理由は次の通り。

1. checker verdict transport receipt の次段として checker verdict transport channel body 自体は narrow に橋渡しできる
2. low-level memory-order family を still 後段に残せる
3. next later reopen を checker-verdict-transport-channel-body-ready low-level-memory-order-family comparison へ狭く進めやすい

## minimal checker-verdict-transport-channel-body-ready retained bridge

current docs-only で stable sketch として切ってよい最小 shape は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_channel_body_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_receipt_ready_sketch,
  retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body = {
    theorem_export_checker_verdict_transport_receipt_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_transport_receipt,
    channel_body_kind = checker_verdict_transport_channel_body
  }
}
```

### `retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body`

`retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body` は、
checker verdict transport receipt の次段として
checker verdict transport channel body 自体を theorem-side retained bridge で最小限に表す field である。

current task では、この channel body を low-level memory-order family には昇格させない。

## なぜ low-level memory-order family をまだ入れないか

low-level memory-order family を current phase で入れると、

- checker verdict transport receipt
- checker verdict transport channel body
- low-level memory-order family

が theorem-line retained bridge の同じ reopen で混ざりやすい。

current pressure はまず checker verdict transport channel body 自体を source-backed に切るところまでで十分である。

## practical examples

### example A — fallback chain checker-verdict-transport-channel-body-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_channel_body_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_receipt_ready_sketch =
    bridge_sketch:e12.theorem_export_checker_verdict_transport_receipt_ready,
  retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body = {
    theorem_export_checker_verdict_transport_receipt_ref =
      retained_payload_theorem_export_checker_verdict_transport_receipt:e12.latest,
    channel_body_kind = checker_verdict_transport_channel_body
  }
}
```

ここで bridge が示すのは checker verdict transport channel body までであり、
low-level memory-order family まではまだ bridge に入れない。

### example B — witnessed draw checker-verdict-transport-channel-body-ready retained bridge

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_channel_body_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_receipt_ready_sketch =
    bridge_sketch:sugoroku.draw.theorem_export_checker_verdict_transport_receipt_ready,
  retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body = {
    theorem_export_checker_verdict_transport_receipt_ref =
      retained_payload_theorem_export_checker_verdict_transport_receipt:sugoroku.draw.latest,
    channel_body_kind = checker_verdict_transport_channel_body
  }
}
```

draw case でも checker verdict transport channel body 自体は bridge で追えるが、
low-level memory-order family までは still theorem-line 外に残す。

## what is decided here

### decided

- current package で checker verdict transport channel body comparison を切り、current first choice は `retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body` だけを足す retained bridge である
- low-level memory-order family は still 後段に残す
- next later reopen は checker-verdict-transport-channel-body-ready low-level-memory-order-family comparison である

### not decided

- low-level memory-order family を実際に比較対象へ入れるべきか
- `retained_payload_body_materialization_theorem_export_checker_verdict_transport_receipt` を retained bridge のまま維持するか checker verdict transport channel body へ actualize するか
- theorem-line retained bridge から async-control / concurrency vocabulary へ接続する最小 handoff shape
