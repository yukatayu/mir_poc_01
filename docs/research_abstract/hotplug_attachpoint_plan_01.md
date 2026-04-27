# hot-plug AttachPoint plan 01

## 目的

runtime hot-plug を、attach request / compatibility / activation / migration stop line に分けて読むための
docs-first current plan です。

## current anchors

- `01_runtime_attach_game`
- `09_detach_todo`
- `attach_request#1`
- `detached_roll_request#1`

## current rule

- requested attach と active state mutationを同一視しない
- detach TODO boundary を durable migration complete と同一視しない
- storage detach script と runtime detach lifecycle を混同しない

## compatibility checklist

- required capability / witness
- membership freshness
- activation 前後の visible frontier
- detach 後 failure の explicitness

## stop line

- final hot-plug ABI
- rollback protocol
- durable migration engine

## 関連

- `plan/21-hotplug-attachpoint-roadmap.md`
- `docs/reports/0925-hotplug-attachpoint-plan.md`
