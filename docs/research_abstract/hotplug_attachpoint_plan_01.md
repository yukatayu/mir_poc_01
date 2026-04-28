# hot-plug AttachPoint plan 01

## 目的

runtime hot-plug を package-manager concern として、attach request / compatibility / activation /
migration stop line に分けて読むための docs-first current plan です。

## current anchors

- `01_runtime_attach_game`
- `09_detach_todo`
- `attach_request#1`
- `detach_request#1`
- `detached_roll_request#1`
- `hotplug_lifecycle`
- `--debug hotplug`

## current rule

- requested attach と active state mutationを同一視しない
- detach TODO boundary を durable migration complete と同一視しない
- storage detach script と runtime detach lifecycle を混同しない
- helper-local `hotplug_lifecycle` は `MessageEnvelope` 由来の evidence summary であり、public ABI ではない
- current safest first cut は helper/test/docs closeout hardening に留め、runtime crate ownership や durable rollback を先取りしない

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
- `docs/reports/0931-hotplug-attachpoint-auth-none-envelope-canary.md`
