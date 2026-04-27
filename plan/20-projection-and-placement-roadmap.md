# plan/20 — projection / placement current roadmap

## 目的

Mirrorea future-axis の phase 12 `Projection / placement` を、
repo-local current layer で手戻りを増やさずに進めるための docs-first roadmap を置く。

ここで固定するのは、system-wide source と place-specific program の区別、
place split、validity checklist、stop line である。
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
- clean near-end order-handoff
  - `05_delegated_rng_service`
  - `provider_boundary`
  - `provider_boundary_redacted_flow`
- docs-first phase 9 planned family
  - `samples/not_implemented/typed-external-boundary/`

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

## stop line

- final projection IR
- generated place-specific program emitter
- placement optimizer
- cross-place equivalence checker
- production scheduler / deployment planner

## next relation

current next promoted package は hot-plug patch / `AttachPoint` である。
projection validity line を下敷きに、compatibility / activation / migration stop line を切る。
