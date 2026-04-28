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

## next relation

transport widening は `plan/22-network-transport-roadmap.md` に切り出した。
hot-plug lifecycle は transport widening と分けたまま保つ。
`P8` close 後の current snapshot では、next promoted package は `P9` avatar fairy follow hardening、next reopen point は `P10` `mirrorea-core` first real implementation tranche である。
