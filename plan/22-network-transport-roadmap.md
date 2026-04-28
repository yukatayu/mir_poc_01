# plan/22 — network transport current roadmap

## 目的

Mirrorea future-axis の phase 13 `Network transport` を、
repo-local current layer で無理なく前進させるための docs-first roadmap を置く。

ここで固定するのは、current evidence anchor、loopback / reconnect / failure matrix の sample ladder、
transport widening の invariant、stop line である。
final socket / broker choice、cryptographic session protocol、multi-server consensus は固定しない。

2026-04-28 P13 first-cut closeout note:
current helper closeout に
`transport_scope = helper_local_process_boundary`、
`process_boundary_canaries = NET-02 / NET-03 / NET-04 / NET-05`、
`loopback_parity_sources = 01_runtime_attach_game / 03_roll_publish_handoff / 04_non_owner_roll_rejected`、
`non_collapse_lanes = transport / auth / membership / capability / witness / visualization`、
`kept_later_gates = real_socket_or_broker / crypto_session_protocol / durable_replay_commit / continuous_shared_runtime_state / final_public_transport_abi`、
`validation_floor`
を actualize してよい。
これは helper-local process-boundary closeout であり、real socket / broker / session / replay runtime actualization ではない。

## current anchors

- `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir`
  - `attach_request#1`
  - `local_queue`
  - helper-local `loopback_socket` preview
- `samples/clean-near-end/sugoroku-world/03_roll_publish_handoff.mir`
  - `roll_request#1`
  - `handoff_notice#1`
  - helper-local `message_envelopes`
- `samples/clean-near-end/sugoroku-world/04_non_owner_roll_rejected.mir`
  - helper-local reject-path parity on `loopback_socket`
- `samples/clean-near-end/sugoroku-world/05_late_join_history_visible.mir`
  - membership freshness
  - late join / published history path
- `samples/clean-near-end/sugoroku-world/09_detach_todo.mir`
  - detached-domain-action reject path
  - transport widening does not erase lifecycle stop line
- `samples/clean-near-end/network-transport/README.md`
  - active helper-local landing page for `NET-02..05`
- `scripts/network_transport_samples.py`
  - subprocess JSON bridge / reconnect guard / typed failure / redacted route trace helper-local canary
- `samples/clean-near-end/order-handoff/05_delegated_rng_service.mir`
  - runtime report-local `provider_boundary`
  - typed external boundary receipt path

## planned family

- `samples/not_implemented/network-transport/`
  - `NET-01` loopback attach / envelope parity
  - `NET-02` two-process roll / publish / handoff
  - `NET-03` reconnect with membership epoch guard
  - `NET-04` typed transport failure / timeout / queue-full
  - `NET-05` redacted route trace / observer-safe network debug

これらは working sample ID であり、規範正本や final public command surface ではない。

## transport widening invariant

1. transport は authentication / authorization / membership / capability / witness を mint しない
2. `MessageEnvelope` の `envelope_id`、`membership_epoch`、`member_incarnation`、`witness_refs` を widen 後も追える
3. reconnect / retry は hidden success にせず、failure / replay / stale-epoch rejection を explicit に保つ
4. visualization / telemetry は route trace を出せても、label / authority / redaction を崩さない
5. helper-local loopback preview を final public transport ABI と誤読させない
6. `NET-03..05` helper-local canary は widening の evidence surface であり、real transport success を主張しない

## minimal widening ladder

1. `local_queue` baseline
   - same-process logical multi-place current floor
2. `loopback_socket` parity
   - same envelope fields / same rejection family / same witness references
   - helper-local `NET-01` canary として `SUG-01` attach、`SUG-03` publish/handoff、`SUG-04` reject path、`check-all --transport loopback_socket` を actualize 済み
3. `two_process_loopback`
   - process boundary をまたぐ attach / publish / handoff path
   - 2026-04-27 current cut では `scripts/network_transport_samples.py run NET-02` として helper-local subprocess JSON bridge canary を actualize 済み
4. `reconnect_epoch_guard`
   - reconnect 後も membership freshness を失わない path
   - 2026-04-27 current cut では `scripts/network_transport_samples.py run NET-03` として stale epoch / incarnation reject canary を actualize 済み
5. `typed_transport_failure`
   - timeout / queue-full / route-not-found / detach-after-send を explicit failure family で返す path
   - 2026-04-27 current cut では `scripts/network_transport_samples.py run NET-04` として helper-local typed failure matrix を actualize 済み
6. `observer_safe_route_trace`
   - route trace / telemetry を authority / redaction 付き observer-safe view で返す path
   - 2026-04-27 current cut では `scripts/network_transport_samples.py run NET-05` として redacted route trace canary を actualize 済み

## current process-boundary closeout inventory

- helper closeout は次を返してよい。
  - `transport_scope`
  - `process_boundary_canaries`
  - `loopback_parity_sources`
  - `non_collapse_lanes`
  - `kept_later_gates`
  - `validation_floor`
- current reading は、`NET-01..05` helper-local canary を process-boundary closeout として束ね、
  real transport runtime と誤読させない inventory を executable closeout に固定した、というところで止める。

## stop line

- final socket / broker / QUIC / WebRTC choice
- cryptographic session / signature / federation protocol
- congestion control / backpressure tuning
- multi-server consensus
- durable distributed commit
- production deployment contract

## relation to adjacent packages

- `plan/21-hotplug-attachpoint-roadmap.md`
  - transport widening は `AttachPoint` compatibility / activation / migration と別 lane に保つ
- `samples/not_implemented/typed-external-boundary/`
  - phase 9 adapter boundary ladderの外側に transport lane を切る

## next relation

`NET-01` helper-local loopback preview は actualize 済みである。
`NET-02..05` も helper-local canary として actualize 済みであり、
`P13` current first-cut closeout では `NET-01` preview を floor にしたまま subprocess JSON bridge / stale reconnect reject / typed transport failure / redacted route trace を current evidence surface と helper closeout inventory に固定した。
ただし、real socket / broker / session / durable replay は still later である。
