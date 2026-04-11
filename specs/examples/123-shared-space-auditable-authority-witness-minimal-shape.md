# 123 — shared-space auditable authority witness minimal shape

## 目的

`specs/examples/122-shared-space-catalog-working-subset-comparison.md` で

- `authoritative_room_witnessed_draw`
- `append_friendly_room_with_rng_capability` optional capability candidate

を current working subset に置いた後の次段として、`auditable_authority_witness` を **room profile と audit / receipt side を混ぜずに** どこまで narrow に切れるかを整理する。

ここで固定するのは current phase の docs-first judgment であり、

- final cryptographic receipt format
- final provider attestation format
- final exporter / audit serialization schema
- final auth / identity integration

はまだ固定しない。

## scope と前提

- provider placement と fairness claim strengthening は別軸に保つ。
- current comparison の焦点は `authority_rng` / `delegated_rng_service` の選択ではなく、`auditable_authority_witness` という fairness claim を名乗るための **最小 typed witness** である。
- room profile row 自体は
  - `fairness_claim = auditable_authority_witness`
  を持つだけに留め、witness の実体は audit / receipt side に出す。
- authoritative room と append-friendly room optional capability の両方で読める shape を優先する。

## 比較したい 3 案

### 案A — note-only witness

#### 読み

- audit には `draw was witnessed` に相当する note だけを残す
- もしくは `witness_ref` だけを prose 的に置く

#### 例

```text
fairness_claim = auditable_authority_witness
audit_note     = "authority witnessed draw=4 at turn 18"
```

#### 利点

- 最小に見える
- room profile 側への影響が最も小さい

#### 欠点

- typed compare / replay hook が弱い
- `opaque authority trust` との差が prose wording に寄りやすい
- append-friendly room optional capability と authoritative room で共通 carrier を持ちにくい

### 案B — minimal typed witness bundle

#### 読み

audit / receipt side には、少なくとも次の 4 field を持つ typed witness を置く。

```text
witness = {
  witness_kind,
  action_ref,
  draw_slot,
  draw_result
}
```

#### field の意味

- `witness_kind`
  - current phase では `authority_draw_witness` に相当する typed discriminator
- `action_ref`
  - その draw を消費した room-local action / transition / append event を指す stable ref
- `draw_slot`
  - 1 action 中に複数 draw がありうる場合の slot 識別子
  - 1 draw しかない room rule では `primary` のような fixed slot でよい
- `draw_result`
  - authority が commit に使った random result

#### authoritative room での例

```text
transition_ref = { room_epoch = 8, turn_epoch = 18 }

witness = {
  witness_kind = authority_draw_witness,
  action_ref   = transition_ref,
  draw_slot    = primary,
  draw_result  = 4,
}
```

#### append-friendly room optional capability での例

```text
append_ref = notice_append#991

witness = {
  witness_kind = authority_draw_witness,
  action_ref   = append_ref,
  draw_slot    = primary,
  draw_result  = 7,
}
```

#### 利点

- provider placement を変えずに fairness claim を 1 段 strengthen できる
- `opaque authority trust` と違い、typed replay / debug hook を持てる
- authoritative room と append-friendly room optional capability の両方に同じ carrier を使える
- witness 側に membership snapshot や full room state を持ち込まずに済む

#### 欠点

- `action_ref` の stable ref policy は later task に残る
- `draw_slot` の naming は final syntax / exporter schema ではまだ未決である

### 案C — expanded attested receipt

#### 読み

minimal bundle に加えて、次のような field まで current witness core に入れる。

```text
witness = {
  witness_kind,
  action_ref,
  draw_slot,
  draw_result,
  provider_ref?,
  provider_receipt_ref?,
  authority_signature_ref?,
  membership_epoch?,
  actor_ref?,
}
```

#### 利点

- provider attestation や stronger audit へ接続しやすい
- auth / identity / receipt validation への導線は明確になる

#### 欠点

- `delegated_provider_attestation` や auth layering と早く混ざる
- provider placement と witness shape の separation を弱めやすい
- current phase では operational realization 寄りである

## current recommendation

current phase の first choice は **案B — minimal typed witness bundle** である。

理由は次の通り。

1. `auditable_authority_witness` の目的は provider placement を変えずに、`opaque authority trust` より 1 段だけ typed fairness claim を強くすることにある。
2. `action_ref + draw_slot + draw_result` があれば、authoritative room の turn transition と append-friendly room の optional draw append を同じ carrier で読める。
3. provider receipt / signature / membership snapshot を current witness core に入れると、`delegated_provider_attestation` や auth layer を先取りしすぎる。

## room profile と witness carrier の cut

current phase では、次の cut を維持する。

### room profile に残すもの

```text
fairness_claim = auditable_authority_witness
```

room profile は fairness claim の **requirement** だけを宣言し、witness payload 自体は持たない。

### audit / receipt side に送るもの

```text
witness = {
  witness_kind,
  action_ref,
  draw_slot,
  draw_result
}
```

### current witness core に入れないもの

- full room snapshot
- member list / membership registry state
- auth / identity evidence
- provider attestation / provider receipt
- long-form explanation
- human-facing fairness prose

## provider placement との関係

current judgment では、

- `authority_rng`
- `delegated_rng_service`

のどちらを採るかと、

- `opaque authority trust`
- `auditable_authority_witness`

のどちらを採るかを同じ軸に潰さない。

したがって、たとえば current phase では次の 2 row を別に読める。

```text
fairness_source = authority_rng
fairness_claim  = auditable_authority_witness
```

```text
fairness_source = delegated_rng_service
fairness_claim  = auditable_authority_witness
```

このとき witness core 自体は同じ shape を保ち、provider receipt / attestation を later strengthening に残す方が自然である。

## causal metadata / membership carrier との関係

`auditable_authority_witness` の minimal core は、causal metadata や membership carrier を置き換えるものではない。

- `membership_epoch`
- `member_incarnation`
- control-plane separated carrier

は引き続き別 line に残し、必要なら `action_ref` の背後にある room log / action log / append log がそれらと整合する、という読みで十分である。

つまり current witness core は、

- fairness claim strengthening
- replay / debug hook

のための **small typed bridge** であって、membership / causality / auth の carrier ではない。

## current judgment

- current phase で `auditable_authority_witness` を minimal に切るなら、**room profile には claim だけ、実体は audit / receipt side の typed witness bundle に置く**のが最も自然である
- minimal witness core の first choice は
  - `witness_kind`
  - `action_ref`
  - `draw_slot`
  - `draw_result`
  の 4 field である
- note-only witness は弱すぎ、expanded attested receipt は current phase では強すぎる
- provider placement、membership / causality、auth / identity は引き続き別軸で扱う

したがって current repo では、`auditable_authority_witness` を provider attestation や distributed fairness に肥大化させることではなく、**この minimal witness core と delegated-provider practical cut を前提に、control-plane separated causal carrier を authoritative room side line に reopen する threshold を比べる**のが自然であり、その threshold judgment 自体は `specs/examples/125-shared-space-control-plane-carrier-threshold.md` に切り出した。
