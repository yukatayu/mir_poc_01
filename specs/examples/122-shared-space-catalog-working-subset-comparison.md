# 122 — shared-space catalog working subset comparison

## 目的

`specs/examples/121-shared-space-authoritative-room-baseline.md` で authoritative room の baseline を 1 本に集約した後の次段として、

- consistency mode
- fairness source
- fairness claim
- causal membership carrier

を **room profile catalog の working subset** としてどこまで current phase に入れてよいかを整理する。

ここでの目的は final catalog を固定することではなく、authoritative room と append-friendly room をまたぐ **最小 row set** と **stop line** を与えることである。

## scope と前提

- 今回は docs-only comparison に留める。
- activation / authority placement の baseline は `specs/examples/121-shared-space-authoritative-room-baseline.md` を前提にする。
- final parser syntax、final room profile syntax、final exporter / audit schema は固定しない。
- `delegated_rng_service` は provider placement の practical candidate であり、fairness claim の final strengthening と同一視しない。
- `membership_epoch + member_incarnation` split は current first practical candidate として扱い、control-plane separated carrier は next stronger candidate に残す。

## 比較したい 3 案

### 案A — axis list だけを残し、row catalog をまだ作らない

#### 読み

- consistency / fairness / causal metadata の候補を軸ごとに列挙する
- room profile row は later task に残す

#### 利点

- early lock-in を避けやすい
- final catalog を先取りしない

#### 欠点

- authoritative room baseline と append-friendly room contrast が weak になる
- 次段の fairness witness / provider placement / epoch split の比較単位が曖昧になる
- practical example に落としたときに、どの bundle が current subset に入っているか読みにくい

### 案B — small cross-room working subset catalog を切る

#### 読み

- current phase で本当に比較したい room profile row だけを 2〜4 本に絞る
- row ごとに
  - `consistency_mode`
  - `fairness_source`
  - `fairness_claim`
  - `causal_membership_carrier`
  を並べる
- final catalog に入れない candidate は stop line に送る

#### 利点

- practical example と theory の距離が短い
- authoritative room baseline から次段の strengthening axis へ進みやすい
- append-friendly room を mere contrast ではなく row-level candidate として読める
- `delegated_rng_service`、`auditable authority witness`、epoch / incarnation split の関係を room profile で説明しやすい

#### 欠点

- row 数を欲張ると半端な catalog 固定に見えやすい
- final exhaustive catalog と誤読されない stop line の明記が必要

### 案C — semi-exhaustive catalog を先に広げる

#### 読み

- `relaxed merge-friendly room`
- `distributed fairness protocol`
- control-plane separated carrier
- replicated authority

まで含めて catalog 候補を広く並べる

#### 利点

- 将来拡張の見通しはよい

#### 欠点

- current phase では比較軸が増えすぎる
- authoritative room baseline を practical checkpoint として閉じた利点を崩しやすい
- final catalog を固定していないのに catalog だけ大きく見える

## current recommendation

current phase の first choice は **案B — small cross-room working subset catalog** である。

理由は次の通り。

1. authoritative room baseline は already checkpoint close なので、次は row-level working subset を切る方が practical である。
2. append-friendly room を row として並べないと、consistency / fairness / causal metadata の比較が authoritative room 内の strengthening に閉じやすい。
3. 一方で semi-exhaustive catalog は current phase では広すぎる。

## current working subset

current phase で row-level に持ってよい working subset は、少なくとも次の 4 本である。

### row 1 — authoritative_room_baseline

```text
activation_rule            = authority-ack
authority_placement        = single_room_authority
consistency_mode           = authoritative_serial_transition
fairness_source            = authority_rng
fairness_claim             = opaque_authority_trust
causal_membership_carrier  = membership_epoch + member_incarnation
```

これは `specs/examples/121...` の baseline を、そのまま catalog row として読み下したものである。

### row 2 — authoritative_room_witnessed_draw

```text
activation_rule            = authority-ack
authority_placement        = single_room_authority
consistency_mode           = authoritative_serial_transition
fairness_source            = authority_rng
fairness_claim             = auditable_authority_witness
causal_membership_carrier  = membership_epoch + member_incarnation
```

これは provider placement を変えずに、fairness claim だけを next narrow strengthening した row である。
current phase では、authoritative room baseline のすぐ隣に置く candidate として最も自然である。

### row 3 — append_friendly_room_baseline

```text
activation_rule            = authority-ack
authority_placement        = single_room_authority
consistency_mode           = append_friendly_room
fairness_source            = [room core では要求しない]
fairness_claim             = [room core では要求しない]
causal_membership_carrier  = membership_epoch + member_incarnation
```

ここで大事なのは、append-friendly room では fairness が room core の必須核ではないことを row-level に明示することである。
この row は notice board / presence board のような append-heavy room の最小候補である。

### row 4 — append_friendly_room_with_rng_capability

```text
activation_rule            = authority-ack
authority_placement        = single_room_authority
consistency_mode           = append_friendly_room
fairness_source            = delegated_rng_service
fairness_claim             = delegated_provider_attestation
causal_membership_carrier  = membership_epoch + member_incarnation
```

これは append-friendly room に抽選や random notice ordering のような optional capability を足す場合の practical candidate である。
ただし authoritative room と違い、ここでの fairness claim は room core の必須核ではなく optional capability として読む。

## practical examples

### authoritative room — 途中参加可能なすごろく room

```text
room Sugoroku:
  activation_rule   = authority-ack
  authority         = single_room_authority
  consistency       = authoritative_serial_transition
  fairness_source   = authority_rng
  fairness_claim    = opaque_authority_trust
  causal_membership = membership_epoch + member_incarnation

on roll(player):
  require room.turn_state == idle
  ensure room.turn_state == idle

  room.turn_state = pending
  draw = authority_rng(room.turn_epoch)
  room.position[player] = saturating_add(room.position[player], draw, GOAL)
  notify_all(position_snapshot(room))

  if room.position[player] == GOAL:
    notify_all(goal(player))
    room.position = reset_to_start()

  room.turn_state = idle
```

ここでは draw と global reset が authoritative serial transition の中で読む方が自然である。

### append-friendly room — notice / presence board

```text
room NoticeBoard:
  activation_rule   = authority-ack
  authority         = single_room_authority
  consistency       = append_friendly_room
  fairness_source   = [room core では要求しない]
  fairness_claim    = [room core では要求しない]
  causal_membership = membership_epoch + member_incarnation

on post_notice(member, text):
  append room.notices <- { member, text, room.membership_epoch, member_incarnation }
  notify_all(notice_appended(member, text))
```

ここでは append visibility が主眼であり、global single-winner draw の fairness は room core に要らない。

## stop line

current phase の working subset に **まだ入れない**ものは次である。

1. `relaxed_merge_friendly_room`
2. `distributed_fairness_protocol`
3. control-plane separated causal carrier
4. `replicated_authority`
5. final activation overlay catalog
6. final auth / identity / admission policy catalog

これらは存在候補としては重要だが、current phase では row-level working subset に混ぜない。

## current sequencing outcome

working subset を current row set に切った後の sequencing は、次の順で source-backed に整理できた。

1. `auditable_authority_witness` の最小 witness shape → `specs/examples/123...`
2. `delegated_rng_service` を authoritative room 側でも provider-placement candidate としてどこまで practical に読むか → `specs/examples/124...`
3. control-plane separated causal carrier を reopen する threshold → `specs/examples/125...`

したがって current phase では、**provider placement と fairness claim strengthening を別軸に保ったまま** working subset package を 1 段閉じ、stronger control-plane split 自体の actualization は later pressure が出たときだけ reopen 候補に残すのが自然である。

## current judgment

- current phase では **small cross-room working subset catalog** を切るのが最も自然である
- authoritative room baseline は row 1 として保持する
- authoritative room の next narrow strengthening は row 2 の `auditable_authority_witness` である
- append-friendly room は row 3 を baseline、row 4 を optional capability candidate として持つ
- final exhaustive catalog はまだ作らない
