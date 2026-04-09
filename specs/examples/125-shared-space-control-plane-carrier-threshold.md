# 125 — shared-space control-plane separated causal carrier threshold

## 目的

`specs/examples/121-shared-space-authoritative-room-baseline.md`、
`specs/examples/122-shared-space-catalog-working-subset-comparison.md`、
`specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md`、
`specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md`
を前提に、

- `membership_epoch + member_incarnation`

を current first practical candidate として維持する条件と、

- control-plane separated causal carrier

を **authoritative room side line に reopen する threshold** を整理する。

ここで固定するのは current phase の docs-first judgment であり、

- final control-plane log schema
- final exporter / receipt schema
- final authority handoff protocol
- final distributed control algorithm

はまだ固定しない。

## scope と前提

- 焦点は authoritative room である。
- append-friendly room への contrast は使うが、full catalog は広げない。
- provider placement と witness requirement は引き続き別軸に保つ。
- `delegated_rng_service` を使っても authority が commit owner のままである、という `specs/examples/124...` の cut を維持する。
- current causal membership carrier の first practical candidate は

```text
membership_epoch + member_incarnation
```

である。

## なぜ threshold comparison が必要か

`membership_epoch + member_incarnation` は、

- join / leave / rejoin
- reconnect
- uncommitted action invalidation

まではかなり素直に説明できる。

一方で、authoritative room を practical にしていくと、

- authority handoff
- delegated provider binding の差し替え
- activation ack frontier の比較
- control-plane decision と data-plane action の stale mismatch

のように、**membership change ではない control-plane 変化** が出てくる。

このとき、

```text
membership_epoch + member_incarnation
```

だけで十分か、それとも

```text
control_epoch
```

相当の control-plane carrier を別に立てるべきかが次の論点になる。

## 比較したい 3 案

### 案A — current carrier を維持する

#### 読み

- data-plane action / witness / provider attachment は、引き続き

```text
membership_epoch + member_incarnation
```

だけと整合すればよい
- control-plane change は membership change の延長で読む
- authority handoff や provider binding change は current room-profile line では still external policy / operational realization に残す

#### 利点

- current authoritative room baseline と最も整合する
- carrier を増やさずに済む
- `auditable_authority_witness` の minimal core や provider optional attachment を肥大化させない

#### 欠点

- membership が変わらない control-plane change を明示しにくい
- stale action rejection が membership-only mismatch に寄りやすい
- authority handoff / provider rotation を audit compare に上げたくなった時に carrier が足りなくなる

### 案B — minimal control-plane epoch split を reopen する

#### 読み

- control-plane 側に、membership とは別の軽量 carrier を立てる
- current first cut は full control log ではなく、

```text
control_epoch
```

相当の世代だけを足す
- data-plane action / witness / provider attachment は、必要なら

```text
membership_epoch + member_incarnation + control_epoch
```

と整合する
- control-plane change の authoritative source は別 log / separate carrier にあるが、その event ref 自体は current room core の必須 field にしない

#### 利点

- membership change ではない control-plane 変化を fail-closed に扱える
- authority handoff / provider binding / activation frontier の stale mismatch を `membership_epoch` とは別に見られる
- full control-plane log を今すぐ固定せずに済む

#### 欠点

- carrier が 1 段増える
- `control_epoch` をどこで increment するかの current policy が必要になる
- compare / audit に何を mirror するかの次段が生まれる

### 案C — full control-plane separated carrier を今すぐ入れる

#### 読み

- control-plane log を data-plane log から明示的に分ける
- current carrier には、たとえば

```text
control_epoch
control_event_ref
authority_epoch
provider_binding_epoch
activation_frontier_ref
```

のような field 群まで入れる

#### 利点

- authority handoff / provider attestation / activation frontier compare を最もきれいに扱える
- audit / replay / theorem prover inventory への接続は強い

#### 欠点

- current phase では重すぎる
- room profile、witness core、provider attachment、export schema を同時に押し広げやすい
- operational realization を early に既成事実化しやすい

## practical example A — current carrier で十分な authoritative すごろく room

```text
room sugoroku_room {
  activation_rule   = authority_ack
  authority_model   = single_room_authority
  consistency_mode  = authoritative_serial_transition
  fairness_source   = delegated_rng_service
  fairness_claim    = auditable_authority_witness

  causal_membership_carrier = membership_epoch + member_incarnation
}

request roll(player p) {
  require active_member(p)
  require roll_lock == idle

  transition by room_authority {
    provider_draw <- delegated_rng_service.draw(room_epoch, turn_epoch, slot = primary)
    draw_result   <- provider_draw.result
    commit_turn(p, draw_result)
  }
}
```

この room では、まだ

- authority handoff を room rule に上げていない
- provider binding change を room core で compare しない
- activation ack frontier を separate audit artifact にしていない

ため、

```text
membership_epoch + member_incarnation
```

だけでも current rule は保ちやすい。

## practical example B — threshold を超える例

次のような room rule を compare / audit / replay で見たくなったとする。

```text
control-plane:
  authority_handoff  authority_A -> authority_B
  provider_rebind    rng_provider_1 -> rng_provider_2

data-plane:
  pending_roll(player = C, membership_epoch = 9, member_incarnation = 4)
```

ここで、

- membership 自体は変わっていない
- player `C` の `member_incarnation` も同じ

が、

- authority は `A` から `B` に変わった
- RNG provider も `1` から `2` に変わった

という場合、`membership_epoch + member_incarnation` だけでは

- old authority / old provider で作られた pending action
- current control-plane で正当な pending action

を区別しにくい。

この場合は、少なくとも

```text
control_epoch
```

相当を別に立て、

```text
pending_roll.control_epoch == current_control_epoch
```

を fail-closed に見る方が自然である。

## threshold 条件

current phase で control-plane separated carrier を **まだ reopen しなくてよい**のは、少なくとも次を満たす間である。

1. authority placement が stable で、handoff を room-profile compare に上げていない
2. delegated provider binding の差し替えを room core / witness core の accept-reject 条件にしていない
3. activation ack frontier を separate compare artifact にしていない
4. stale action rejection を `membership_epoch + member_incarnation` と uncommitted action invalidation だけで説明できる

逆に、次のいずれかが current line に入るなら reopen threshold は満たしたと読める。

1. authority handoff を room rule / audit compare に上げる
2. provider binding rotation や provider receipt mismatch を accept-reject rule に入れる
3. activation / reconciliation frontier 自体を separate carrier として compare したくなる
4. membership 不変のまま control-plane だけが変わる stale mismatch を、room core で fail-closed に扱いたい

## current recommendation

current phase の first choice は、**案A を維持しつつ、reopen するなら案B を最小 cut にする** である。

理由は次の通り。

1. `specs/examples/121...` から `specs/examples/124...` までの current package では、authority commit ownership を保ち、provider receipt も optional attachment に留めている。
2. したがって current line では、`membership_epoch + member_incarnation` を first practical candidate に置いたままでも room profile reasoning は保てる。
3. ただし authority handoff / provider rotation / activation frontier compare を current line に入れるなら、membership-only carrier では不足する。
4. それでも full control-plane log は早すぎるため、first reopen cut は `control_epoch` 相当の軽量 split に留める方が自然である。

## preserved invariants

### current-baseline preservation

`specs/examples/121...` から `specs/examples/124...` の baseline judgment は、
この threshold comparisonによって直ちには壊れない。

### provider / witness separation preservation

provider placement と witness requirement は、引き続き別軸である。
control-plane carrier を reopen するとしても、`auditable_authority_witness` の minimal core をすぐには広げない。

### control-before-data invariant

control-plane separated carrier を reopen する場合でも、
control-plane decision が data-plane action の owner になるのではなく、
data-plane action が **どの control generation の下で正当か** を示すために使う。

## current judgment

- **current first practical candidate は、引き続き `membership_epoch + member_incarnation` である**
- **authoritative room side line では、control-plane separated causal carrier はまだ current default にしない**
- **ただし authority handoff / provider binding / activation frontier が membership change とは別に compare need を持った時点で、reopen threshold は満たす**
- **そのときの first reopen cut は full control-plane log ではなく、`control_epoch` 相当の lightweight split に留めるのが自然である**

したがって current repo の next reading は、

- Phase 4 current package は `specs/examples/121...` から `specs/examples/125...` までで checkpoint close とみなし、
- stronger control-plane split 自体の actualization は later pressure が出たときに narrow に reopen する

である。
