# network transport plan 01

## 目的

Mirrorea の transport widening を、`local_queue` baseline から
loopback / process boundary / reconnect / typed failure / redacted route trace へ安全に広げるための docs-first current plan です。

## current anchors

- Sugoroku helper `message_envelopes`
- `01_runtime_attach_game` の `attach_request#1`
- `03_roll_publish_handoff` の local-queue / loopback envelope
- `04_non_owner_roll_rejected` の reject-path parity
- `05_late_join_history_visible` の membership freshness
- `06_leave_non_owner` の stale epoch / incarnation reject
- `09_detach_todo` の detach-after-send reject
- clean near-end `05_delegated_rng_service` の `provider_boundary`
- `scripts/network_transport_samples.py` の helper-local canary

## current rule

- transport は auth / membership / capability / witness を mint しない
- reconnect は stale epoch / stale incarnation を hidden repair しない
- typed transport failure を explicit に残す
- visualization / telemetry でも route trace は label / authority / redaction 付きで読む

## planned family

- `NET-01` loopback attach / envelope parity
- `NET-02` two-process roll / publish / handoff
- `NET-03` reconnect with membership epoch guard
- `NET-04` typed transport failure / timeout / queue-full
- `NET-05` redacted route trace / observer-safe network debug

2026-04-27 current cut では、`NET-01` を helper-local `--transport loopback_socket` preview として actualizeし、
さらに `scripts/network_transport_samples.py` で `NET-02..05` helper-local canary を actualize しました。

- `NET-02`
  subprocess JSON bridge による process-boundary route trace
- `NET-03`
  stale `membership_epoch` / `member_incarnation` reconnect reject
- `NET-04`
  timeout / queue-full / route-not-found / detach-after-send の typed failure matrix
- `NET-05`
  observer-safe redacted route trace

ただし、これは real socket / cryptographic session / durable replay ではありません。

## stop line

- final public transport ABI
- concrete broker / socket / QUIC choice
- cryptographic session protocol
- multi-server consensus
- durable distributed commit

## 関連

- `plan/22-network-transport-roadmap.md`
- `docs/reports/0926-network-transport-plan.md`
