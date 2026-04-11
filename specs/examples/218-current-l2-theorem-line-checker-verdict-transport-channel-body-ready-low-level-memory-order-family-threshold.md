# 218 — current L2 theorem line checker-verdict-transport-channel-body-ready low-level-memory-order-family threshold

## 目的

`specs/examples/217-current-l2-theorem-line-checker-verdict-transport-receipt-ready-checker-verdict-transport-channel-body-threshold.md`
までを前提に、

- checker-verdict-transport-channel-body-ready retained bridge に low-level memory-order family をどこまで近づけるか
- low-level memory-order family を theorem-line retained bridge に入れるべきか
- next promoted line を low-level memory-order family に置くべきか、それとも higher-level async-control family に切り替えるべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の checker-verdict-transport-channel-body-ready low-level-memory-order-family threshold** であり、

- final async-control vocabulary
- final memory model
- public checker API
- actual theorem prover / protocol verifier integration

はまだ固定しない。

## scope

- current `proof_notebook` first bridge を起点にする。
- checker-facing contract は `theorem_export_checker` に限る。
- low-level memory-order family は、theorem-line retained bridge の next candidate としてだけ比較する。
- actual hardware memory model、scheduler API、distributed protocol actualization には進まない。

## current 前提

current repo では次が成立している。

1. `retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body`
   までは current first choice に入っている。
2. `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
   により、`atomic_cut` は place-local finalizing cut の最小核であり、
   C++ 的 low-level memory-order family は current immediate candidate にしない cut が固定済みである。
3. `plan/13-heavy-future-workstreams.md` と
   `docs/reports/0358-async-control-memory-boundary-inventory.md`
   により、higher-level async-control は
   event-tree / authority-serial transition / witness-aware commit の line として
   別に比較する余地が残っている。

したがって current 問いは、
**checker verdict transport channel body の次段として low-level memory-order family を retained bridge に近づけるべきか、それとも theorem-line はここで止めて、次の promoted line を higher-level async-control family comparison に切り替えるべきか**
である。

## 比較観点

1. `atomic_cut` の local cut と low-level memory-order family を premature に同一視しないか
2. theorem-line retained bridge に C++ 的 / hardware-like vocabulary を既成事実化しないか
3. higher-level async-control family を別 line として残す余地を守れるか
4. current small decidable core / proof boundary / protocol boundary の 4-way split を壊さないか
5. user-facing 説明で `perform` / `try` / `atomic_cut` / witnessed draw を誤って low-level memory-order で読む drift を増やさないか

## 比較対象

### 案 1. checker verdict transport channel body を theorem-line retained bridge の terminal cut にし、low-level memory-order family は still 外に残す

#### 読み

theorem-line retained bridge は
`retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body`
でいったん止める。

low-level memory-order family は current theorem-line には足さず、
次の promoted line は
**higher-level async-control family comparison**
へ切り替える。

#### 利点

- `atomic_cut` の local cut と low-level memory-order family を混ぜにくい
- checker-verdict transport line と async-control line の責務を分けられる
- event-tree / authority-serial / witness-aware commit の higher-level family を別 line で比較しやすい
- C++ 的 low-level vocabulary を current theorem-line bridge に既成事実化しない

#### 欠点

- theorem-line retained bridge の next step が prose だけだと見えにくい
- low-level memory-order family そのものの future reopen 条件を別に明文化する必要がある

### 案 2. low-level memory-order family marker だけを持つ retained bridge にする

#### 読み

checker-verdict-transport-channel-body-ready retained bridge に、
actual order tag や actual transport protocol を導入せず
**`retained_payload_body_materialization_theorem_export_low_level_memory_order_family`**
だけを足す。

最小 shape は次に留める。

```text
proof_notebook_bridge_retained_payload_low_level_memory_order_family_ready_sketch = {
  proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_channel_body_ready_sketch,
  retained_payload_body_materialization_theorem_export_low_level_memory_order_family = {
    theorem_export_checker_verdict_transport_channel_body_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body,
    family_kind = low_level_memory_order
  }
}
```

#### 利点

- low-level memory-order family が future reopen candidate であること自体は bridge で見える
- actual order tag / transport carrier detail を immediate に入れずに済む

#### 欠点

- `low_level_memory_order` という語彙を current theorem-line bridge に持ち込むだけで、immediate candidate だと誤読されやすい
- higher-level async-control family と low-level memory-order family の順序を濁しやすい
- `atomic_cut` / room authority / fairness witness の line と hardware-like order family が同じ reopen に見えやすい

### 案 3. low-level memory-order carrier detail / payload を current bridge へ actualize する

#### 利点

- low-level verifier / checker integration に直接つながりやすい
- future memory-model lowering を一見 concrete に見せやすい

#### 欠点

- current repo の 4-way split を最も壊しやすい
- parser / checker / runtime / proof の全部に同時 pressure をかけやすい
- high-level async-control family より先に hardware-like vocabulary を既成事実化しやすい

## current judgment

current L2 で最も自然なのは、
**案 1. checker verdict transport channel body を theorem-line retained bridge の terminal cut にし、low-level memory-order family は still 外に残す**
である。

理由は次の通り。

1. `atomic_cut` は current でも place-local finalizing cut の最小核であり、low-level memory-order family とは責務が違う
2. theorem-line retained bridge は proof obligation / checker-facing payload の symbolic line であり、hardware-like order family を早く持ち込むと boundary が濁る
3. current next step としては、low-level memory-order family そのものより
   event-tree / authority-serial transition / witness-aware commit のような
   **higher-level async-control family**
   を比較する方が理論的に自然である

## theorem-line retained bridge の current stop line

current docs-only で fixed にしてよい stop line は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_checker_verdict_transport_channel_body_ready_sketch = {
  ...,
  retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body = {
    theorem_export_checker_verdict_transport_receipt_ref =
      retained_payload_body_materialization_theorem_export_checker_verdict_transport_receipt,
    channel_body_kind = checker_verdict_transport_channel_body
  }
}
```

### stop line の意味

`retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body`
は、checker verdict transport line が theorem-line retained bridge の中で届いてよい最小 channel-body 側 boundary である。

current task では、この stop line の次に
`low_level_memory_order` marker を追加しない。

## なぜ low-level memory-order family を marker にもしないか

marker だけなら harmless に見えるが、current phase では次の誤読を招きやすい。

- theorem-line retained bridge の immediate next candidate が low-level memory-order family だという誤読
- `atomic_cut` や witnessed draw を low-level memory-order で説明すべきだという誤読
- event-tree / authority-serial / witness-aware commit より hardware-like order family を先に比較するべきだという誤読

current repo では、これは still 強すぎる。

## low-level memory-order family を reopen してよい threshold

low-level memory-order family を future に reopen 候補へ上げてよいのは、
少なくとも次の 3 条件が揃ったときだけである。

1. **higher-level async-control family insufficiency**
   - `event_tree_execution_view` / `authority_serial_transition_family` /
     `witness_aware_commit_family`
     を比較しても、target external verifier の ordering 要求を still 表せない
2. **concrete external verifier pressure**
   - theorem prover / protocol verifier / lowering target のいずれかが、
     high-level family ではなく low-level order family を明示的に要求する
3. **boundary stabilization**
   - theorem-line retained bridge と protocol / runtime policy boundary の handoff shape が
     ある程度 stable で、low-level family を入れても subject boundary を混ぜない

この 3 条件が揃わない限り、low-level memory-order family は still later candidate に留める。

## practical examples

### example A — `try` / `atomic_cut` は low-level memory-order marker を要求しない

```text
try {
  perform on doc_ref {
    ensure saved(doc_ref)
  }
  atomic_cut
  perform on audit_log {
    ensure notified(user)
  }
} fallback {
  perform on recovery_log
}
```

ここで theorem-line retained bridge に欲しいのは、

- `atomic_cut` の structural floor
- rollback-cut non-interference の proof obligation

であって、
`memory_order_release` / `memory_order_acquire` のような low-level family marker ではない。

### example B — authoritative room の witnessed draw も first question は high-level async-control である

```text
authoritative room:
  authority-ack
  single room authority
  authoritative serial transition
  auditable_authority_witness
```

ここで先に比較したいのは、

- authority-serial transition
- witness-aware commit
- event-tree derived execution / replay view

であり、checker-side retained bridge に low-level memory-order family を直結することではない。

## next promoted line

current next promoted line は、
**checker-verdict-transport-channel-body-ready higher-level-async-control-family comparison**
である。

ここで比べる first candidates は、少なくとも次である。

1. `event_tree_execution_view`
2. `authority_serial_transition_family`
3. `witness_aware_commit_family`

low-level memory-order family は、その line の後でも still later candidate に留めてよい。

## what is decided here

### decided

- current package では theorem-line retained bridge を `retained_payload_body_materialization_theorem_export_checker_verdict_transport_channel_body` で止める
- low-level memory-order family は current theorem-line retained bridge に入れない
- `low_level_memory_order` marker も current phase では作らない
- next promoted line は checker-verdict-transport-channel-body-ready higher-level-async-control-family comparison である

### not decided

- higher-level async-control family の first cut を `event_tree_execution_view` / `authority_serial_transition_family` / `witness_aware_commit_family` のどれから取るか
- low-level memory-order family を future にまったく採らないか、external verifier vocabulary としてだけ残すか
- theorem-line retained bridge から async-control family へ接続する最小 handoff shape
