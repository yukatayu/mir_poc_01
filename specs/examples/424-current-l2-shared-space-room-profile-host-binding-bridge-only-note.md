# 424 — current L2 shared-space room-profile host-binding bridge-only note

## 目的

shared-space authoritative-room / working-subset docs と
capability-scoped host-I/O boundary docs を前提に、

- room-profile 側で既に固定済みの row
- compile-time over-approx bridge
- runtime control-plane bridge
- host boundary refs

を bridge-only note として整理する。

ここで fixed するのは
**room-profile facts を host-binding note へどこまで surface してよいか**
であり、

- final shared-space catalog
- concrete auth binding
- actual host interface contract
- FFI / engine adapter

は still later に残す。

## source-backed floor

- authoritative room baseline は
  `authority-ack`、`single room authority`、`authoritative serial transition`、`authority_rng`
  を current kernel に置く。
- source of truth は session-scoped membership registry first であり、
  tree / array view は derived snapshot に留める。
- reconnect kernel は
  `member_incarnation + uncommitted action invalidation`
  を current first practical cut に置く。
- host-facing side は privileged `stdin/stdout` を持たず、
  capability-scoped I/O / adapter boundary を current floor に置く。

## current bridge shape

```text
shared_space_room_profile_host_binding_bridge = {
  bridge_kind,
  entry_criteria_refs,
  room_profile_refs,
  compile_time_bridge_refs,
  runtime_control_plane_refs,
  host_boundary_refs,
  guard_refs,
  kept_later_refs
}
```

## current judgment

1. current bridge-only note は
   **already-fixed room-profile floor と already-fixed host boundary floor を繋ぐ note**
   に留め、どちらの side も reopen しない。
2. compile-time bridge refs は
   `required_role`、`required_capability`、`visibility`、`notify_path`
   の over-approx line に留める。
3. runtime control-plane bridge refs は
   `admission_policy`、`activation_event`、`active_member_set`、`late_join_reconciliation`
   に留め、timeout / retry / resend / liveness / UI grace は external policy に残す。
4. provider placement、witness requirement、fairness source / trust model は 1 軸に潰さず、
   room-profile side の separate axes に残す。

## guard

- privileged `stdin/stdout` を導入しない。
- concrete host interface contract を fixed しない。
- concrete auth protocol / engine adapter / FFI target を fixed しない。
- room-profile facts を delivery / timeout / UI policy と同一視しない。
- control-plane separated carrier actualization をここで昇格させない。

## next promoted line

next promoted line は、
**checker attachment から handoff row への migration note**
に置く。

## what is not decided here

- final shared-space catalog
- concrete auth binding
- actual host interface contract
- `control_epoch` actualization timing
- FFI / game engine adapter line
