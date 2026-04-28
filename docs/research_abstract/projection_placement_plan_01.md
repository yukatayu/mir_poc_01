# projection / placement plan 01

## 目的

Mirrorea の source を、あとで place-specific program へ safely projection できるようにするための
current plan と helper/report-local preview floor の summary です。

## current rule

- source principal を server/client に早期固定しない
- `Place` を participant と同一視しない
- adapter / visualizer path でも auth / witness / visualization / telemetry を separate lane に保つ

## current preview floor

2026-04-28 時点では、docs-first に留まらず、次の preview floor を actualize しています。

- Sugoroku helper:
  `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug projection --format json`
  - `projection_view`
  - `SugorokuWorldSource#1`
  - `WorldServerPlace` / `SugorokuGamePlace#1` / `ParticipantPlace[*]`
  - membership frontier
  - observer view refs
- clean near-end runtime report-local inventory:
  `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - `cross_place_projection`
  - authority placement と provider placement の分離
  - message envelope refs
  - redaction rule refs

これは final emitted place program ではありません。
projection validity を collapse せずに preview する current floor です。

## current emitted-program gate

`P3` current package で固定するのは、actual emitted place-specific program ではなく、
次の docs-first boundary です。

- current preview floor は helper/report-local preview only
- projection validity report の minimum contents は category 単位で固定する
- generated place-specific program family は reserve path / external-workdir preference を保つ
- actual emitted place-specific program family は `P15` へ handoff する

## current place split

- server / authoritative path
- participant path
- domain runtime place path
- external adapter path
- visualizer / observer path

## validity checklist

- authority placement が explicit か
- membership frontier が explicit か
- witness path が preserved されるか
- label / authority / redaction が preserved されるか
- failure / rejection が hidden にならないか

## stop line

- final projection IR
- generated place-specific program emitter
- placement optimizer
- cross-place equivalence checker

## generated artifact reserve

- `samples/generated/` は future reserve path
- source sample は置かない
- heavy disposable emitted artifact は external workdir を優先する
- committed generated artifact は generated / source distinction を明示した bridge evidence に限る

## 関連

- `docs/hands_on/projection_placement_views_01.md`
- `plan/20-projection-and-placement-roadmap.md`
- `docs/reports/0924-projection-placement-plan.md`
