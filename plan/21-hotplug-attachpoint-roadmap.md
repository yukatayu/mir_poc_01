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

## 2026-04-28 R2 minimal-contract closeout note

`R2` では、`P14` closeout inventory の上に
`AttachPoint` current minimal contract row を narrow に整理した。

fixed current reading:

- helper-local `hotplug_lifecycle` は `MessageEnvelope` 由来の evidence summary であり、
  final public hot-plug ABI ではない
- current minimal contract row は
  `compatibility / activation_cut / detach_boundary / migration_contract`
  の 4 row に限る
- `attachpoint_id` / `patch_id` は identity column として残し、
  row を `attach_request#1` / `detach_request#1` / `detached_roll_request#1`、
  `attach_lifecycle` / `detach_lifecycle`、
  `attach_activation#1` / `detach_boundary#1` に grounded に読む
- `attach_request#1` accepted は requested attach と active state mutation を分ける
- `detached_roll_request#1` rejected は detach 後 domain action rejection の
  explicit boundary evidence であり、completed rollback ではない
- `migration_contract` row は current unresolved gate を evidence として残し、
  `runtime_crate_hotplug_engine / rollback_protocol / durable_migration_engine / final_public_hotplug_abi`
  は kept-later gate に残す

reader-facing current memory は
`plan/30-attachpoint-detach-minimal-contract.md`、
`docs/research_abstract/attachpoint_detach_minimal_contract_01.md`、
`docs/hands_on/attachpoint_detach_minimal_contract_01.md`
を参照する。

## 2026-04-28 R4 kept-later boundary closeout note

`R4` では、`R2` current minimal contract row の先に残る
real migration / rollback / runtime-crate hot-plug engine /
distributed activation ordering を、
helper-local evidence floor と切り離した kept-later boundary として narrow に整理した。

fixed current reading:

- `activation_cut` は requested attach と active state mutation の split を示すが、
  distributed activation ordering は示さない
- `migration_contract` row は unresolved-state honesty lane であり、
  protocol / schema ではない
- rejected `detached_roll_request#1` は explicit boundary evidence であって、
  completed rollback ではない
- storage detach / network replay / runtime detach lifecycle は別 concern として残す
- helper-local `hotplug_lifecycle` は runtime-crate ownership や final public ABI を示さない

reader-facing current memory は
`plan/32-hotplug-real-migration-rollback-boundary.md`、
`docs/research_abstract/hotplug_real_migration_rollback_boundary_01.md`、
`docs/hands_on/hotplug_real_migration_rollback_boundary_01.md`
を参照する。

## 2026-04-28 R5 ownership-cut closeout note

`R5` では、
helper-local `hotplug_lifecycle` / anchor envelope IDs、
`mirrorea-core` generic carrier / logical runtime substrate、
`mir-runtime` thin runtime/report assembly
の owner split を narrow に固定した。

fixed current reading:

- helper `hotplug_lifecycle`
  != runtime-crate engine ownership
- sample-grounded `attach_request#1 / detach_request#1 / detached_roll_request#1`
  != Rust-side canonical hot-plug protocol family
- `LogicalPlaceRuntimeShell`
  != world/game/hot-plug engine
- `mir-runtime`
  != current hot-plug engine owner
- Python/Rust carrier duplication
  != ownership migration complete

reader-facing current memory は
`plan/33-runtime-crate-hotplug-engine-ownership-cut.md`、
`docs/research_abstract/runtime_crate_hotplug_engine_ownership_cut_01.md`、
`docs/hands_on/runtime_crate_hotplug_engine_ownership_cut_01.md`
を参照する。

## 2026-04-28 R6 carrier-admission closeout note

`R6` では、`R5` owner split の次に
**何が first admissible Rust-side hot-plug-specific family か**
を narrow に整理した。

fixed current reading:

- first admissible Rust-side hot-plug-specific family は
  engine-neutral request / verdict carrier に限る
- helper-local
  `hotplug_lifecycle`
  `attach_request#1 / detach_request#1 / detached_roll_request#1`
  `attach_lifecycle / detach_lifecycle`
  `attach_activation#1 / detach_boundary#1`
  は preview ownership に残す
- reusable carrier ownership は `mirrorea-core` 側の later tranche 候補として読み、
  thin runtime/report assembly は `mir-runtime` 側のさらに後段の orchestration candidate として残す
- current sequence は
  `R6` ->
  `P19` `mirrorea-core` hot-plug request/verdict carrier tranche ->
  `P20` `mir-runtime` hot-plug orchestration skeleton first tranche
  と読む
- request / verdict carrier を lifecycle engine、rollback protocol、
  durable migration engine、distributed activation ordering、final public ABI と混同しない

reader-facing current memory は
`plan/34-runtime-crate-hotplug-carrier-admission-cut.md`、
`docs/research_abstract/runtime_crate_hotplug_carrier_admission_cut_01.md`、
`docs/hands_on/runtime_crate_hotplug_carrier_admission_cut_01.md`
を参照する。

## next relation

transport widening は `plan/22-network-transport-roadmap.md` に切り出した。
hot-plug lifecycle は transport widening と分けたまま保つ。
`P14 -> P15 -> P16 -> P17 -> P18` の repo-global queue 自体はすでに進んでいる。
この文書では queue の正本を持たず、hot-plug 固有 memory だけを保持する。
repo-global current line は `progress.md`、`tasks.md`、`plan/11-roadmap-near-term.md`、
post-`P18` mixed-gate memory は `plan/27-public-api-parser-gate-roadmap.md` を参照する。
