# projection / placement plan 01

## 目的

Mirrorea の source を、あとで place-specific program へ safely projection できるようにするための
docs-first current plan です。

## current rule

- source principal を server/client に早期固定しない
- `Place` を participant と同一視しない
- adapter / visualizer path でも auth / witness / visualization / telemetry を separate lane に保つ

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
- generated place-specific emitter
- placement optimizer
- equivalence checker

## 関連

- `plan/20-projection-and-placement-roadmap.md`
- `docs/reports/0924-projection-placement-plan.md`
