# plan/20 — projection / placement current roadmap

## 目的

Mirrorea future-axis の phase 12 `Projection / placement` を、
repo-local current layer で手戻りを増やさずに進めるための roadmap を置く。

ここで固定するのは、system-wide source と place-specific program の区別、
place split、validity checklist、helper/report-local preview floor、stop line である。
final projection IR、generator、optimizer、equivalence checker は固定しない。

## current invariant

- Mirrorea は system-wide source を後で place-specific program に projection できる性質を保つ。
- source principal を server / participant / adapter / visualizer のどれかに早期固定しない。
- projection でも auth / membership / capability / witness / visualization / telemetry を潰さない。

## place split

current docs-first line では、少なくとも次を分けて読む。

- server / authoritative room path
- participant path
- domain runtime place path
- external adapter path
- visualizer / observer path

`Place` は participant や principal と同一ではなく、
queue / state / capability / visibility / observation frontier を持つ execution locus として扱う。

## current evidence anchors

- Sugoroku helper
  - `WorldServerPlace`
  - `ParticipantPlace[*]`
  - `SugorokuGamePlace#1`
  - `message_envelopes`
  - `visualization_views`
  - `projection_view`
  - `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
- clean near-end order-handoff
  - `05_delegated_rng_service`
  - `provider_boundary`
  - `provider_boundary_redacted_flow`
  - `cross_place_projection`
  - `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
- docs-first phase 9 planned family
  - `samples/not_implemented/typed-external-boundary/`

## current executable preview floor

- helper-local preview:
  Sugoroku `03_roll_publish_handoff` の `projection_view` は、
  `SugorokuWorldSource#1` から `WorldServerPlace` / `SugorokuGamePlace#1` / `ParticipantPlace[*]`
  への place split、membership frontier、observer view refs、transport seam `local_queue` を
  evidence-oriented に見せる current preview である。
- report-local preview:
  clean near-end `05_delegated_rng_service` の `cross_place_projection` は、
  authority placement と provider placement を分離したまま provider-boundary lane を読む
  report-local inventory である。
- どちらも final emitted place program ではない。
  executable widening の current scope は projection validity を collapse せずに preview することであり、
  optimizer / scheduler / deployment planner を導入することではない。

## projection validity checklist

projection / placement plan を current line に上げるときは、少なくとも次を確認する。

1. source principal を early server/client split に潰していないか
2. authority placement が explicit か
3. membership frontier が explicit か
4. witness path が preserved されるか
5. label / authority / redaction が visualization path まで preserved されるか
6. adapter path で transport / auth / witness が collapse していないか
7. failure / rejection / detach stop line が place-local に explicit か
8. helper-local preview を final public projection API と誤読させていないか
9. provider placement と authority placement を collapse していないか

## stop line

- final projection IR
- generated place-specific program emitter
- placement optimizer
- cross-place equivalence checker
- production scheduler / deployment planner

## next relation

projection validity line は `plan/21-hotplug-attachpoint-roadmap.md` と
`plan/22-network-transport-roadmap.md` の両方を下支えする。
source-to-place split を transport widening や hot-plug activation と collapse しないことが current relation である。
