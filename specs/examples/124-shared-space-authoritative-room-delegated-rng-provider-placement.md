# 124 — shared-space authoritative room delegated RNG provider placement

## 目的

`specs/examples/121-shared-space-authoritative-room-baseline.md` で authoritative room baseline を

- `authority-ack`
- `single room authority`
- `authoritative serial transition`
- `authority_rng`

の 4 軸 bundle として固定し、`specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md` で `auditable_authority_witness` の minimal witness core を切った後の次段として、

- `delegated_rng_service`

を authoritative room 側でも **provider-placement candidate** としてどこまで practical に読めるかを整理する。

ここで固定するのは current phase の docs-first judgment であり、

- final provider receipt schema
- final provider identity / auth integration
- final delegated-provider attestation rule
- final distributed randomness protocol

はまだ固定しない。

## scope と前提

- authoritative room baseline の 4 軸 bundle自体は維持する。
- 今回の問いは **authority placement を変えるか** ではなく、**RNG source の実体だけを authority から provider へ差し替えても baseline reasoning を保てるか** である。
- provider placement と fairness claim strengthening は別軸に保つ。
- `auditable_authority_witness` の minimal witness core

```text
witness_kind + action_ref + draw_slot + draw_result
```

は current first choice のまま維持する。

## 比較したい 3 案

### 案A — `authority_rng` facade の内側に delegation を隠す

#### 読み

- room profile には引き続き `fairness_source = authority_rng` だけを書く
- authority 実装の内側で外部 provider を叩いていても、language / room profile 上は見せない

#### 利点

- room profile は最小のままで済む
- authority baseline を一切触らずに済む

#### 欠点

- provider placement が hidden になる
- HW entropy / debug provider / external RNG service を room policy として比較しにくい
- provider failure policy や provider receipt hook を docs-first に扱いにくい
- `authority_rng` と `delegated_rng_service` の違いが implementation detail に埋もれる

### 案B — `delegated_rng_service` を provider placement だけ room profile に上げる

#### 読み

- room profile では `fairness_source = delegated_rng_service` とだけ読む
- authority は request / lock / commit / publish の owner であり続ける
- provider は randomness result を返すが、room state mutation や activation frontier の更新はしない
- provider receipt / audit ref は optional attachment として残してよく、current witness core には入れない

#### 例

```text
activation_rule   = authority-ack
authority_model   = single_room_authority
consistency_mode  = authoritative_serial_transition
fairness_source   = delegated_rng_service
fairness_claim    = opaque_authority_trust
```

または

```text
activation_rule   = authority-ack
authority_model   = single_room_authority
consistency_mode  = authoritative_serial_transition
fairness_source   = delegated_rng_service
fairness_claim    = auditable_authority_witness
```

#### 利点

- provider placement を room policy として見える化できる
- authority placement と fairness-source placement を別軸に保てる
- `authority_rng` baseline と同じ authoritative serial transition を維持しやすい
- deterministic debug provider / HW entropy / external RNG service を replaceable layer として差し替えやすい

#### 欠点

- provider failure policy を別に整理する必要がある
- provider identity / receipt ref をどこまで typed に持つかが次の論点になる
- provider を room profile に出した瞬間に attestation まで一緒に入れたくなる pressure が出る

### 案C — provider placement と delegated attestation を同時に上げる

#### 読み

- `fairness_source = delegated_rng_service`
- かつ `fairness_claim = delegated_provider_attestation`
- もしくは provider receipt / provider signature / provider identity を current room-profile line の必須核に入れる

#### 利点

- fairness trust の強化をすぐ見せやすい
- provider receipt を audit / replay と直結しやすい

#### 欠点

- provider placement と fairness claim strengthening を同じ軸に潰す
- auth / identity / receipt validation と早く混ざる
- current `auditable_authority_witness` minimal core を unnecessary に肥大化させやすい
- authoritative room baseline の 4 軸 bundleに対して、一度に増える自由度が多い

## current recommendation

current phase の first choice は **案B — provider placement だけ room profile に上げる** である。

理由は次の通り。

1. `delegated_rng_service` を次の practical candidate にしたい理由は、authority placement を動かすことではなく、**RNG source を replaceable layer に出したい**からである。
2. authoritative room baseline の核心は `single room authority` と `authoritative serial transition` にあり、これを維持したまま `fairness_source` だけ差し替える方が理論的にきれいである。
3. provider receipt / attestation を current core に入れると、provider placement と fairness claim strengthening が同じ比較軸になってしまう。

## authoritative room での minimal delegated-provider cut

current first choice は、次の 4 点を満たす cut である。

1. **authority keeps commit**
   - request accept / reject
   - lock transition
   - room state mutation
   - publish / notify
   は引き続き authority が行う
2. **provider returns draw, not state transition**
   - provider は `draw_result` を返す
   - room state や membership epoch を直接更新しない
3. **witness core stays unchanged**
   - `auditable_authority_witness` を使う場合でも、

```text
witness_kind + action_ref + draw_slot + draw_result
```

   から始めてよい
4. **provider receipt stays optional attachment**
   - `provider_draw_ref` や `provider_receipt_ref` は current room core や witness core の必須 field にしない
   - audit / receipt side の optional attachment として比較すればよい

## practical example — delegated provider 付き authoritative すごろく room

```text
room sugoroku_room {
  membership_source = session_membership_registry
  activation_rule   = authority_ack
  authority_model   = single_room_authority
  consistency_mode  = authoritative_serial_transition
  fairness_source   = delegated_rng_service
  fairness_claim    = auditable_authority_witness
}

request roll(player p) {
  require active_member(p)
  require roll_lock == idle

  transition by room_authority {
    roll_lock <- pending

    provider_draw <- delegated_rng_service.draw(
      room_epoch,
      turn_epoch,
      slot = primary
    )

    n <- provider_draw.result
    board_state[p] <- min(goal, board_state[p] + n)
    emit positions_snapshot(board_state)

    witness <- {
      witness_kind = authority_draw_witness,
      action_ref   = turn_transition(room_epoch, turn_epoch),
      draw_slot    = primary,
      draw_result  = n,
    }

    attach_optional provider_draw_ref = provider_draw.ref

    if board_state[p] == goal {
      emit winner_notice(p)
      reset board_state[*] <- start
      emit positions_snapshot(board_state)
    }

    roll_lock <- idle
  }
}
```

ここで重要なのは、

- `turn_transition(...)` を commit するのは authority
- `provider_draw.ref` は audit attachment であり、transition owner ではない
- witness の `draw_result` は committed result と一致すればよく、provider receipt 自体は current witness core の必須核ではない

という点である。

## preserved invariants

この cut は、少なくとも次の invariants を保つように読むのが自然である。

### authority-serial invariant

room-critical mutation は引き続き authority 1 本の serial transition sequence を通る。

したがって `authority_rng` から `delegated_rng_service` への差し替えは、**draw value の供給源**を変えるだけで、transition ownership は変えない。

### provider-non-committing invariant

provider は `draw_result` を返してよいが、

- lock state
- board state
- winner notice
- membership activation

を直接 commit してはならない。

これにより provider placement が hidden second authority になることを避けられる。

### witness-binding invariant

`fairness_claim = auditable_authority_witness` のとき、witness は少なくとも

- committed `action_ref`
- committed `draw_slot`
- committed `draw_result`

に bind される。

provider receipt を持つとしても、それは current witness core の前段 attachment であり、authority が commit した action binding を置き換えない。

### source-substitution invariant

`authority_rng` と `delegated_rng_service` の差は、current phase では

- draw source の差
- provider audit hook の有無

に留める。

したがって activation rule、authority placement、consistency mode を同時に変えない限り、room profile の reasoning は baseline 4 軸から大きく逸脱しない。

## current room-profile reading

current phase では、authoritative room 側の delegated provider cut を次のように読むのが自然である。

| 項目 | current first choice |
|---|---|
| activation rule | `authority-ack` |
| authority placement | `single room authority` |
| consistency mode | `authoritative serial transition` |
| fairness source | `delegated_rng_service` |
| fairness claim | `opaque_authority_trust` または `auditable_authority_witness` |
| provider receipt | audit / receipt side optional attachment |

ここで `fairness_claim` が 2 候補あるのは、provider placement と witness requirement を別軸に保つためである。

## まだ current cut に入れないもの

- `delegated_provider_attestation` を room profile の current必須核にすること
- provider signature / receipt validation を current witness core に入れること
- provider が authority の代わりに transition commit を行うこと
- `distributed_randomness_provider` を authoritative room の default candidate にすること
- auth / identity / service admission policy を provider placement と同時に固定すること

## next narrow step

provider placement の practical cut を current phase で 1 段整理した後の次 narrow step は、**control-plane separated causal carrier を authoritative room side line に reopen する threshold** を比べることである。

理由は、provider placement と fairness claim strengthening の current first choice がそろった後は、

- membership epoch / member incarnation
- activation / reconciliation
- in-flight action invalidation

を data-plane witness / provider attachment とどこまで分けるかが、catalog comparison の次の stop line になるためである。

## current judgment

- **authoritative room 側でも `delegated_rng_service` は provider-placement candidate として practical に読める**
- **その際も authority は request / lock / commit / publish の owner であり続ける**
- **provider は draw を返すが、room mutation を commit しない**
- **`auditable_authority_witness` の minimal witness core は unchanged でよい**
- **provider receipt / provider draw ref は current witness core に入れず、audit / receipt side optional attachment に留める**
- **provider placement と fairness claim strengthening は別軸に保ち、`delegated_provider_attestation` は still later candidate に残す**

したがって current phase では、authoritative room baseline の次の practical widening としては、

- `authority_rng`

から

- `delegated_rng_service`

へ **fairness_source だけ**を差し替え、authority placement / consistency mode / witness core は保つ cut が最も自然である。
