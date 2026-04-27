# network transport plan 01

## 目的

Mirrorea の transport widening を、`local_queue` baseline から
loopback / reconnect / typed failure へ安全に広げるための docs-first current plan です。

## current anchors

- Sugoroku helper `message_envelopes`
- `01_runtime_attach_game` の `attach_request#1`
- `03_roll_publish_handoff` の local-queue envelope
- `05_late_join_history_visible` の membership freshness
- clean near-end `05_delegated_rng_service` の `provider_boundary`

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

## stop line

- final public transport ABI
- concrete broker / socket / QUIC choice
- cryptographic session protocol
- multi-server consensus
- durable distributed commit

## 関連

- `plan/22-network-transport-roadmap.md`
- `docs/reports/0926-network-transport-plan.md`
