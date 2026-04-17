# Phase 4 要約 — shared-space membership と practical room boundary

## この phase の役割

Phase 4 は、shared-space / room / membership line を **language core に premature に押し込まず、practical boundary として整理する phase** である。

## 固まった current reading

- participant carrier と causal metadata は分ける。
- source of truth は session-scoped membership registry first に置く。
- `membership_epoch` と `member_incarnation` の split は current practical candidate である。
- authoritative room baseline は
  `authority-ack / single room authority / authoritative serial transition / authority_rng`
  で読む。
- identity / admission / authority / room-profile / host-binding の docs-first boundary は fixed 済みである。
- fairness / replay は mixed-gate boundary に留める。

## source-backed evidence

- `specs/examples/121...125`
- shared-space follow-up packages
- practical room / host-binding / fairness-replay boundary notes

## まだここで決めていないこと

- shared-space final operational catalog
- final fairness / replay operational profile
- distributed provider / fairness protocol
- upper-layer room / app target

## 次へ渡したもの

Phase 5 以降では、shared-space line を theorem / protocol / runtime-policy boundary と接続してよい。
ただし final catalog と operational policy は mixed gate か user specification に残す。
