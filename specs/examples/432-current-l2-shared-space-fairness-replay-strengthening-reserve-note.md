# 432 — current L2 shared-space fairness / replay strengthening reserve note

## 目的

`specs/examples/424` までで fixed した
shared-space room-profile / host-binding bridge-only line を前提に、

- fairness source / trust model
- replay attachment / stale receipt invalidation
- provider placement
- witness requirement
- payload / observation visibility

を潰さずに、next reserve package として整理する。

ここで fixed するのは
**fairness / replay line を final operational catalog に昇格させずに strengthen する reserve boundary**
であり、

- final shared-space catalog
- concrete replay protocol/profile
- distributed fairness protocol
- concrete authority binding
- final property language
- final emitted-artifact schema

は still later に残す。

## source-backed floor

- authoritative room baseline は
  `authority-ack`、`single room authority`、`authoritative serial transition`、`authority_rng`
  を current practical bundle に置く。
- provider placement と witness requirement は separate axes に保つ。
- `delegated_rng_service` は next practical candidate に残すが、
  authority commit ownership は authority side に残す。
- default carrier floor は
  `membership_epoch + member_incarnation`
  に置き、`control_epoch` は first reopen cut に留める。
- order / handoff adequacy corpus では、
  late join / rejoin、stale receipt、epoch mismatch、provider-authority mismatch、
  fairness fails while safety holds を separate negative family に保つ。

## current reserve shape

```text
shared_space_fairness_replay_reserve = {
  reserve_kind,
  motivating_room_refs,
  negative_family_refs,
  preserved_axes,
  carrier_floor_refs,
  protocol_verifier_refs,
  runtime_policy_refs,
  kept_later_refs
}
```

## current judgment

1. fairness / replay strengthening note は、
   **room-profile facts を final operational profile へ昇格させないための reserve package**
   として読む。
2. preserve すべき principal axes は、
   authority placement、
   provider placement、
   witness requirement、
   fairness source / trust model、
   replay attachment、
   payload / observation visibility
   の 6 つである。
3. replay / fairness / provider-receipt family は、
   主に `protocol_verifier_boundary` と `runtime_policy_boundary`
   の line に残し、
   `core_static_checker` や room-core semantics に collapse しない。
4. `control_epoch` は default carrier ではなく、
   provider rotation / authority handoff / activation frontier が
   accept-reject semantics に入るときだけ reopen 候補になる。

## guard

- final shared-space catalog を fixed しない。
- concrete replay protocol/profile を fixed しない。
- distributed fairness protocol を fixed しない。
- concrete authority binding を fixed しない。
- final property language / emitted-artifact schema を fixed しない。
- provider placement / fairness source / witness requirement を 1 軸に潰さない。

## next promoted line

next promoted line は、
**request / predicate / `try` cluster typed-surface reserve note**
に置く。

## what is not decided here

- final shared-space catalog
- concrete replay protocol/profile
- distributed fairness protocol
- concrete authority binding
- `control_epoch` actualization timing
- final property language
- final emitted-artifact schema
