# 255 — current L2 theorem line minimal-handoff-transport-channel-body-ready low-level-memory-order-family threshold

## 目的

`specs/examples/254-current-l2-theorem-line-handoff-transport-channel-body-ready-minimal-handoff-transport-channel-body-threshold.md`
までを前提に、

- handoff-transport-channel-body-ready retained bridge に low-level memory-order family をどこまで近づけるか
- low-level memory-order family を theorem-line retained bridge に入れるべきか
- next promoted line を low-level memory-order family に置くべきか、それとも theorem-line current package close として別の Phase 5 line に戻すべきか

を比較する。

ここで固定するのは **current Phase 5 theorem-line の
minimal-handoff-transport-channel-body-ready low-level-memory-order-family threshold** であり、

- final memory model
- final public checker API
- actual runtime handoff transport protocol
- final type system

はまだ固定しない。

## scope

- current `proof_notebook` retained bridge を起点にする。
- handoff transport channel body の次段としてだけ low-level memory-order family を比較する。
- `atomic_cut` の current meaning は変えない。
- actual scheduler / hardware-memory-like semantics / distributed protocol actualization には進まない。

## current 前提

current repo では次が成立している。

1. theorem-line retained bridge の current stop line は
   `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body`
   である。
2. `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
   により、`atomic_cut` は place-local finalizing cut の最小核であり、
   low-level memory-order family は current language core の immediate candidate ではない。
3. `specs/examples/218-current-l2-theorem-line-checker-verdict-transport-channel-body-ready-low-level-memory-order-family-threshold.md`
   により、checker-verdict line でも low-level memory-order family は retained bridge に入れず、
   higher-level async-control family 側へ戻す cut が source-backed に固定済みである。
4. current handoff line は、すでに
   `authority_serial_transition_family`
   から
   `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body`
   まで進んでおり、source-of-truth ordering / witness / replay / transport line は一通り narrow に切れている。

したがって current 問いは、
**handoff transport channel body の次段として low-level memory-order family を retained bridge に近づけるべきか、それとも theorem-line はここで止めて、次の promoted line を small decidable core / checker cluster side へ戻すべきか**
である。

## 比較観点

1. `atomic_cut` の place-local cut と low-level memory-order family を premature に同一視しないか
2. authoritative handoff transport line と hardware-like ordering vocabulary を premature に結び付けないか
3. theorem-line retained bridge を docs-only symbolic / narrow detail line に保てるか
4. Phase 5 の `core_static_checker` / `theorem_prover_boundary` / `protocol_verifier_boundary` / `runtime_policy_boundary` の 4-way split を壊さないか
5. Phase 5 の次の自走 line を、より小さい decidable / checker side inventory に戻しやすいか

## 比較対象

### 案 1. theorem-line retained bridge を handoff transport channel body で止め、low-level memory-order family は still 外に残す

#### 読み

theorem-line retained bridge は
`retained_payload_body_materialization_theorem_export_handoff_transport_channel_body`
で止める。

low-level memory-order family は current theorem-line に足さず、
next promoted line は
**small-decidable-core-ready checker-cluster-matrix comparison**
へ切り替える。

#### 利点

- handoff transport line と low-level memory-order family を混ぜない
- `atomic_cut` / authority-serial / witness-aware handoff と hardware-like order family を premature に結び付けない
- theorem-line retained bridge の ratchet をきれいに閉じられる
- Phase 5 の次の self-driven line を core-static-checker inventory 側へ戻しやすい

#### 欠点

- low-level memory-order family そのものは still prose 依存で残る
- concrete external verifier pressure が出たときの reopen 条件を別に明文化する必要がある

### 案 2. low-level memory-order family marker だけを持つ retained bridge にする

#### 読み

`retained_payload_body_materialization_theorem_export_handoff_transport_channel_body`
の次段として、
`retained_payload_body_materialization_theorem_export_low_level_memory_order_family`
だけを足す。

#### 利点

- future reopen candidate であること自体は bridge 上に見える
- actual order tag / transport protocol は immediate に持ち込まなくてよい

#### 欠点

- marker だけでも low-level vocabulary を current theorem-line bridge に既成事実化しやすい
- channel body と low-level ordering を同じ reopen で扱う誤読を招きやすい
- Phase 5 の current promoted line を small decidable core 側へ戻しにくい

### 案 3. low-level memory-order carrier detail / payload を current retained bridge に actualize する

#### 利点

- external verifier / lowering target へ一見近づく
- memory-model 側の議論を早く concrete に見せやすい

#### 欠点

- current repo の 4-way split を最も壊しやすい
- parser / checker / runtime / theorem-line retained bridge のすべてに同時 pressure をかけやすい
- current phase では premature である

## current judgment

current L2 で最も自然なのは、
**案 1. theorem-line retained bridge を handoff transport channel body で止め、low-level memory-order family は still 外に残す**
である。

理由は次の通り。

1. current handoff line は authority-serial / witness-aware / replay / transport channel body までで、higher-level async-control 側の narrow retained bridge として十分 source-backed である
2. ここで low-level memory-order family を marker にしても、actual verifier pressure ではなく hardware-like vocabulary だけを premature に入れることになる
3. Phase 5 の次段としては、theorem-line をさらに low-level へ寄せるより、small decidable core / checker cluster side を docs-first に整理し直す方が自然である

## theorem-line retained bridge の current stop line

current docs-only で fixed にしてよい stop line は次である。

```text
proof_notebook_bridge_retained_payload_theorem_export_handoff_transport_channel_body_ready_sketch = {
  ...,
  retained_payload_body_materialization_theorem_export_handoff_transport_channel_body = {
    handoff_transport_receipt_ref =
      retained_payload_body_materialization_theorem_export_handoff_transport_receipt_row,
    handoff_transport_channel_body = authority_handoff_transport_channel_body
  }
}
```

### stop line の意味

`retained_payload_body_materialization_theorem_export_handoff_transport_channel_body`
は、authoritative handoff transport line が theorem-line retained bridge の中で届いてよい最小 channel-body 側 boundary である。

current task では、この stop line の次に
`low_level_memory_order_family`
を追加しない。

## なぜ low-level memory-order family を marker にもしないか

marker だけでも、current phase では次の誤読を招きやすい。

- theorem-line retained bridge の immediate next candidate が low-level memory-order family だという誤読
- `atomic_cut` / authority-serial transition / handoff witness row detail を low-level ordering で読むべきだという誤読
- small decidable core 側の checker cluster inventory より先に hardware-like order family を詰めるべきだという誤読

current repo では、これは still 強すぎる。

## low-level memory-order family を reopen してよい threshold

low-level memory-order family を future に reopen 候補へ上げてよいのは、
少なくとも次の 3 条件が揃ったときだけである。

1. **concrete external verifier pressure**
   - theorem prover / protocol verifier / lowering target のいずれかが、
     handoff transport channel body まででは不足で、
     low-level order relation を明示的に要求する
2. **higher-level async-control insufficiency**
   - authority-serial / witness-aware / replay / transport channel body line まで source-backed に揃えても、
     ordering obligation を still 表せない
3. **boundary stabilization**
   - low-level order family を入れても、
     language core / runtime policy / protocol verifier boundary を混ぜない handoff shape が見えている

この 3 条件が揃わない限り、low-level memory-order family は still later candidate に留める。

## practical examples

### example A — `try` / `atomic_cut` は low-level memory-order family を要求しない

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

ここで current Phase 5 に欲しいのは、

- `atomic_cut` の structural floor
- rollback-cut non-interference の proof obligation

であって、
hardware-like ordering vocabulary ではない。

### example B — authority handoff transport line でも first question は channel body までで足りる

```text
handoff to next_authority
witness by audit_sink
replay attachment authority_handoff_replay
payload authority_handoff_payload
transport receipt authority_handoff_transport_receipt
transport channel body authority_handoff_transport_channel_body
```

ここで theorem-line retained bridge に今ほしいのは、

- authority handoff line がどの replay / payload / receipt / channel body row に届くか

であり、
`memory_order_release` / `memory_order_acquire`
のような family marker ではない。

## next promoted line

current next promoted line は、
**small-decidable-core-ready checker-cluster-matrix comparison**
に置く。

ここで比較する first candidates は、少なくとも次である。

1. same-lineage / malformed / capability / clause / predicate / `try`-rollback locality を prose inventory のまま維持する
2. docs-only checker-cluster matrix を置く
3. minimal typing judgment skeleton へ早く進む

low-level memory-order family は、その line の後でも still later candidate に留めてよい。

## what is decided here

### decided

- current package では theorem-line retained bridge を `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body` で止める
- low-level memory-order family は current theorem-line retained bridge に入れない
- `low_level_memory_order_family` marker も current phase では作らない
- next promoted line は `small-decidable-core-ready checker-cluster-matrix comparison` に切り替える

### not decided

- low-level memory-order family の最終 vocabulary
- final type system
- final public checker API
- actual theorem prover / protocol verifier integration

## open questions

- low-level memory-order family を将来 external verifier vocabulary としてだけ残すのか
- checker-cluster matrix の current first cut を docs-only row bundle に留めるか
- minimal typing judgment skeleton をどの threshold で reopen するか
