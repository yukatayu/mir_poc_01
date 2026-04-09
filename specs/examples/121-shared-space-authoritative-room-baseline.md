# 121 — shared-space authoritative room baseline

## 目的

shared-space / membership line で積み上げてきた比較のうち、authoritative room を practical example として前に進めるための **current baseline** を 1 本に畳む。

ここで固定するのは current phase の docs-first judgment であり、

- final activation rule
- final consistency catalog
- final fairness / auth / reconnect protocol
- final operational realization

はまだ固定しない。

## scope

この文書で扱うのは、少なくとも次を持つ authoritative room である。

- join / leave / rejoin がある
- exclusive action がある
- global reset / winner notify / room-wide snapshot publish がある
- room rule を 1 本の authoritative transition sequence として読みたい

典型例は「途中参加可能なすごろく room」である。

## baseline に入れるもの

current baseline は、次の 4 軸 bundle と side condition を分けて読む。

### 4 軸 bundle

| 軸 | current baseline |
|---|---|
| activation rule | `authority-ack` |
| authority placement | `single room authority` |
| consistency mode | `authoritative serial transition` |
| fairness source | `authority_rng` |

### side condition

- participant source of truth は `session-scoped membership registry + derived snapshot view`
- fairness trust model は current baseline では `opaque authority trust`
- reconnect kernel は `member_incarnation + uncommitted action invalidation`
- compile-time には activation visibility / capability / notify path の over-approximation だけを残し、actual activation / reconciliation / ack frontier は runtime control-plane に残す

## 各軸の current baseline 読み

### `authority-ack`

- join / leave / rejoin は room authority が受理した時点で active / inactive 遷移を始める
- その後の dissemination は authority から visibility 対象へ流す
- `full-coverage-like activation` や `quorum-like activation` は overlay 可能な room policy option に残す

### `single room authority`

- room ごとに **authoritative owner slot / write authority slot** を 1 つ置く
- ここでいう owner は「全 resource を人間 participant に単純還元する」という意味ではない
- room-critical resource の final handoff / commit / reset はこの slot を通す
- read-mostly snapshot や delegated capability は別 carrier に残してよい

### `authoritative serial transition`

- room mutation は 1 本の authoritative transition sequence に載せる
- exclusive lock / pending / winner / reset / publish を同じ sequence で説明する
- `append-friendly room` は contrast room の current first choice に残す

### `authority_rng`

- current baseline では RNG source は authority が持つ
- fairness claim は最小では authority trust に依存する
- provider placement を差し替えたい場合の next practical candidate は `delegated_rng_service`
- ただし fairness trust / witness の strengthening 軸は別であり、provider placement の差し替え自体を overall next strengthening と同一視しない

## authoritative room の owner / capability 読み

current baseline では、room rule を説明する carrier を次のように分ける。

- membership identity / activation は membership registry
- room-critical mutation は authoritative owner slot
- read / observe / append などの user-visible access は delegated capability
- reconnect / handoff は `member_incarnation` や handoff epoch などの control-plane carrier

この分離により、

- participant carrier
- resource owner
- delegated capability
- consistency mode
- fairness source

を 1 carrier に潰さずに済む。

## practical example A — authoritative すごろく room

```text
room sugoroku_room {
  membership_source = session_membership_registry
  players_view      = derived_snapshot(active_members where role == player)
  watchers_view     = derived_snapshot(active_members where role == watcher)

  activation_rule   = authority_ack
  authority_model   = single_room_authority
  consistency_mode  = authoritative_serial_transition
  fairness_source   = authority_rng
  fairness_claim    = opaque_authority_trust

  resource board_state   owner = room_authority
  resource roll_lock     owner = room_authority
  resource winner_notice owner = room_authority

  capability request_roll    delegated_to = players_view
  capability read_positions  delegated_to = players_view + watchers_view
}

request roll(player p) {
  require active_member(p)
  require capability(request_roll, p)
  require roll_lock == idle

  transition by room_authority {
    roll_lock <- pending
    n <- authority_rng.d6()
    board_state[p] <- min(goal, board_state[p] + n)
    emit positions_snapshot(board_state)

    if board_state[p] == goal {
      emit winner_notice(p)
      reset board_state[*] <- start
      emit positions_snapshot(board_state)
    }

    roll_lock <- idle
  }
}
```

### この room で baseline が自然な理由

- pending 中に誰も触れない、winner 通知後に全員 reset、という room rule を 1 本の transition sequence で説明しやすい
- participant list と position snapshot は derived view として publish すればよい
- activation / authority / consistency / RNG を別軸のまま束ねられる

## practical example B — append-friendly notice room との contrast

```text
room notice_room {
  membership_source = session_membership_registry
  activation_rule   = authority_ack
  authority_model   = single_room_authority   // membership control-plane only
  consistency_mode  = append_friendly_room

  capability append_notice delegated_to = active_members
  capability read_notice   delegated_to = active_members + watchers
}
```

ここでは membership activation に authority が残っていても、data-plane append を authoritative serial transition に固定する必要はない。
この contrast があるため、authoritative room baseline を shared-space 全体の既定と誤読してはならない。

## next practical candidate

current baseline に最も近い **fairness-source 軸の next practical candidate** は、4 軸 bundle のうち **RNG source だけ**を差し替える形である。

```text
activation_rule   = authority_ack
authority_model   = single_room_authority
consistency_mode  = authoritative_serial_transition
fairness_source   = delegated_rng_service
```

この差し替えは、

- HW entropy
- deterministic debug provider
- external randomness service

の切り替え余地を増やす一方、activation / authority / consistency の baseline reasoning は保ちやすい。

ただしこれは **provider placement の practical candidate** であって、

- `auditable authority witness`
- delegated provider attestation

のような witness / trust strengthening の順序をここで確定するものではない。

## baseline にまだ入れないもの

current baseline からは次を外に残す。

- `full-coverage-like activation`
- `quorum-like activation`
- `replicated authority`
- `relaxed projection authority`
- `distributed_randomness_provider`
- `auditable authority witness`
- auth stack / admission protocol
- timeout / retry / resend / reconnect protocol の詳細
- actual consensus algorithm 名

これらは current phase では **policy option / future strengthening / operational realization** として比較し、baseline 自体には混ぜない。

## current judgment

- **authoritative room baseline の current first choice は**
  - `authority-ack`
  - `single room authority`
  - `authoritative serial transition`
  - `authority_rng`
  **である**
- **source of truth は membership registry、tree-like room view は derived snapshot / debug / explanation view に留める**
- **fairness-source 軸だけを見るなら、次の practical candidate は RNG source だけを `delegated_rng_service` に差し替える形である**
- **fairness witness、replicated authority、activation overlay、distributed randomness は baseline にはまだ入れない**

したがって current phase では、authoritative room を先に practical にするための最小読みに限るなら、

- room-critical mutation は authoritative owner slot を通す
- participant list / snapshot は derived view で見る
- compile-time は visibility over-approximation を持つ
- actual room lifecycle は runtime control-plane に残す

という cut が最も自然である。
