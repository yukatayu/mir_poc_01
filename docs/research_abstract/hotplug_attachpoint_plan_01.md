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

## current closeout inventory

- `hotplug_scope`
  `helper_local_package_manager_preview`
- `hotplug_anchor_samples`
  `01_runtime_attach_game / 09_detach_todo`
- `hotplug_package_concerns`
  `attachpoint_compatibility / activation_cut / detach_boundary / migration_stop_line / rollback_protocol`
- `hotplug_lifecycle_lanes`
  `attachpoint_id / patch_id / lifecycle_state / compatibility / activation_cut / detach_boundary / migration_contract`
- `hotplug_anchor_envelopes`
  `attach_request#1 / detach_request#1 / detached_roll_request#1`
- `hotplug_view_ids`
  `attach_lifecycle / detach_lifecycle`
- `hotplug_telemetry_row_ids`
  `attach_activation#1 / detach_boundary#1`
- `hotplug_kept_later_gates`
  `runtime_crate_hotplug_engine / rollback_protocol / durable_migration_engine / final_public_hotplug_abi`
- `hotplug_validation_floor`
  helper-local attach/detach lifecycle evidence only。completed migration / rollback / runtime-crate ownership ではない。

## `R2` closeout memory

`R2` では、この current closeout inventory の上に
`compatibility / activation_cut / detach_boundary / migration_contract`
minimal contract row を narrow に整理した。

- helper-local `hotplug_lifecycle` は `MessageEnvelope` 由来の evidence summary
- `attach_request#1` / `detach_request#1` / `detached_roll_request#1`
  を grounding anchor に残す
- `attach_lifecycle` / `detach_lifecycle` と
  `attach_activation#1` / `detach_boundary#1` を current view / telemetry anchor に残す
- rollback / durable migration / final public hot-plug ABI は kept-later gate に残す

reader-facing current memory は
`attachpoint_detach_minimal_contract_01.md`、
`../hands_on/attachpoint_detach_minimal_contract_01.md`、
`../../plan/30-attachpoint-detach-minimal-contract.md`
を参照する。

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
