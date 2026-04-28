# plan/21 — hot-plug patch / AttachPoint current roadmap

## 目的

Mirrorea future-axis の phase 14 `HotPlug Patch / AttachPoint` を、
repo-local current layer で無理なく前進させるための docs-first + helper-local roadmap を置く。

ここで固定するのは、compatibility checklist、activation cut、migration stop line、
attach / detach current anchor である。
final hot-plug ABI、rollback、durable migration engine は固定しない。

## current anchors

- `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir`
  - attach request
  - runtime attach envelope
  - helper-local `hotplug_lifecycle` attach compatibility / activation summary
- `samples/clean-near-end/sugoroku-world/09_detach_todo.mir`
  - detach TODO boundary
  - `detach_request#1`
  - detached domain action rejection / `todo_deferred`
- `scripts/sugoroku_world_samples.py`
  - `attach_request#1`
  - `detach_request#1`
  - `detached_roll_request#1`
  - `--debug hotplug`

## minimum concepts

- `Patch`
- `AttachPoint`
- compatibility check
- activation cut
- migration contract

## compatibility checklist

1. required capabilities / witnesses が explicit か
2. membership freshness を attach / detach lifecycle で保持できるか
3. activation 前と activation 後の visible state frontier が分かれているか
4. detach 後の domain action failure が explicit か
5. visualization / telemetry が attach / detach lifecycle を誤読させないか

## activation cut

current docs-first line では、attach request と actual active state mutation の間に
activation cut があると読む。
requested だけでは active にならず、compatibility / authorization / migration precondition を通った後で active になる。
current helper-local line では、この activation cut は `hotplug_lifecycle.activation_cut`
として見えるが、authoritative seam は `MessageEnvelope` の accepted request envelope に残す。

## migration stop line

current docs-first line では、detach / reattach を
durable state migration completed と同一視しない。
current Sugoroku helper closeout では、
`hotplug_stop_line.detach_boundary = explicit_todo_boundary`、
`rollback_protocol = deferred`、
`durable_migration_engine = deferred`、
`final_public_hotplug_abi = deferred`
を helper closeout stop line として見せてよい。

未決のもの:

- final package ABI
- rollback protocol
- durable state migration engine
- distributed activation ordering

## important non-equivalence

`scripts/storage/detach_prepare.sh` と
`scripts/storage/cleanup_disposable_artifacts.sh` は storage / ops concern である。
runtime hot-plug lifecycle の `detach` と同一ではない。

## 2026-04-28 P14 first-cut closeout note

current helper closeout に
`hotplug_scope = helper_local_package_manager_preview`、
`hotplug_anchor_samples = 01_runtime_attach_game / 09_detach_todo`、
`hotplug_package_concerns = attachpoint_compatibility / activation_cut / detach_boundary / migration_stop_line / rollback_protocol`、
`hotplug_lifecycle_lanes = attachpoint_id / patch_id / lifecycle_state / compatibility / activation_cut / detach_boundary / migration_contract`、
`hotplug_anchor_envelopes = attach_request#1 / detach_request#1 / detached_roll_request#1`、
`hotplug_view_ids = attach_lifecycle / detach_lifecycle`、
`hotplug_telemetry_row_ids = attach_activation#1 / detach_boundary#1`、
`hotplug_kept_later_gates = runtime_crate_hotplug_engine / rollback_protocol / durable_migration_engine / final_public_hotplug_abi`、
`hotplug_validation_floor`
を actualize してよい。
これは helper-local package-manager first-cut closeout であり、runtime crate hot-plug engine や completed migration actualization ではない。

## next relation

transport widening は `plan/22-network-transport-roadmap.md` に切り出した。
hot-plug lifecycle は transport widening と分けたまま保つ。
`P14 -> P15 -> P16 -> P17 -> P18` の repo-global queue 自体はすでに進んでいる。
この文書では queue の正本を持たず、hot-plug 固有 memory だけを保持する。
repo-global current line は `progress.md`、`tasks.md`、`plan/11-roadmap-near-term.md`、
post-`P18` mixed-gate memory は `plan/27-public-api-parser-gate-roadmap.md` を参照する。
