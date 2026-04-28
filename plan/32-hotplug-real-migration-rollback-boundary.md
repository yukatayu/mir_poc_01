# plan/32 — hot-plug real migration / rollback boundary

## 目的

この文書は、`R4` docs-first package として、
`R2` で固定した current minimal contract row の先に残る
real migration / rollback / runtime-crate hot-plug engine /
distributed activation ordering を、
helper-local evidence floor と切り離した **kept-later boundary** として整理する。

ここで固定するのは、current helper-local evidence が
何をまだ証明していないかの boundary matrix である。
helper schema の widening、runtime-crate engine 実装、final public ABI の固定は行わない。

## fixed current facts

- active sample anchors は
  `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir` と
  `samples/clean-near-end/sugoroku-world/09_detach_todo.mir`
- helper-local evidence carrier は
  `scripts/sugoroku_world_samples.py` の `hotplug_lifecycle`
- current minimal contract row は
  `compatibility / activation_cut / detach_boundary / migration_contract`
- grounding anchors は
  `attach_request#1 / detach_request#1 / detached_roll_request#1`
  `attach_lifecycle / detach_lifecycle`
  `attach_activation#1 / detach_boundary#1`
- helper closeout aggregate floor は
  `hotplug_scope`
  `hotplug_anchor_samples`
  `hotplug_package_concerns`
  `hotplug_lifecycle_lanes`
  `hotplug_anchor_envelopes`
  `hotplug_view_ids`
  `hotplug_telemetry_row_ids`
  `hotplug_kept_later_gates`
  `hotplug_validation_floor`
  `hotplug_stop_line`
- current helper closeout は
  `hotplug_scope = helper_local_package_manager_preview`
  `hotplug_anchor_samples = 01_runtime_attach_game / 09_detach_todo`
  `hotplug_package_concerns = attachpoint_compatibility / activation_cut / detach_boundary / migration_stop_line / rollback_protocol`
  を返す

## current evidence vs kept-later boundary

| Boundary | Current anchor | What current evidence proves | What current evidence does not prove |
|---|---|---|---|
| distributed activation ordering | `activation_cut`, accepted `attach_request#1`, `hotplug_activation_boundary` | requested attach と active state mutation が分離され、current helper-local cut の後で only visible active state が出る | multi-place / multi-server activation ordering、durable activation commit、consensus sequencing |
| rollback protocol | `detached_roll_request#1` rejection, `todo_deferred`, `migration_contract.status = deferred` | detach 後 domain action rejection が explicit であり、rollback 未実装が honest に visible である | rollback state machine、compensation order、rollback witness schema、completed rollback |
| durable migration engine | `migration_contract` row, `detach_boundary`, helper stop line | current line が migration unresolved のまま stop していること | durable state transfer、reattach protocol、migration replay contract、cross-place transfer evidence |
| runtime-crate hot-plug engine | helper closeout inventory, `hotplug_scope = helper_local_package_manager_preview` | current ownership が helper-local preview / docs-first package に留まること | crate ownership split、runtime orchestration layer、carrier move plan、engine-side authority boundary |
| final public hot-plug ABI | `hotplug_lifecycle` lane inventory, view / telemetry anchors | helper-local evidence carrier が current minimal contract を読めること | public request/response schema、public lifecycle/event schema、viewer/API names、client/server ABI |

## explicit non-equivalences

- `activation_cut`
  != distributed activation ordering
- accepted `attach_request#1`
  != durable activation commit
- `migration_contract.status = deferred`
  != migration schema exists
- rejected `detached_roll_request#1`
  != rollback protocol completion
- `scripts/storage/detach_prepare.sh`
  != runtime detach lifecycle
- network replay / reconnect concern
  != detach replay / migration concern
- helper-local `hotplug_lifecycle`
  != runtime-crate engine ownership
- helper-local view / telemetry anchor names
  != final public viewer / telemetry API

## current recommendation

- `R2` current minimal contract row は、そのまま helper-local evidence floor として読む
- `migration_contract` row は protocol ではなく、
  current unresolved state を honest に visible にするための row として読む
- `activation_cut` を広げるときも、first claim は
  request-vs-active-state split に留め、
  distributed activation ordering は kept-later boundary に残す
- rollback / migration / runtime-crate engine / final public ABI を、
  current helper-local package-manager inventory に仮託して claim しない
- historical next relation after `R4` close は
  `R5` runtime-crate hot-plug engine ownership cut とし、
  helper-local preview、crate-side carrier、runtime orchestration の owner split を narrow に整理する
- `R5` closeout 後の owner split memory は
  `plan/33-runtime-crate-hotplug-engine-ownership-cut.md`
  を reader-facing current memory として参照する
- current repo state では `P21` も close 済みであり、
  post-`P21` later family の first recommendation は
  `plan/36-post-p21-rollback-durable-migration-family.md`
  を入口にした `rollback / durable migration` family hardening と読む
- current repo state では
  second recommendation も
  `plan/37-post-p21-distributed-activation-ordering-family.md`
  を入口にした docs-first hardening として close 済みである
- current remaining third recommendation は
  final public hot-plug ABI であり、
  post-`P18` mixed gate / `U1` hold line に残す

## validation floor

- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
- `python3 -m unittest scripts.tests.test_visual_debugger_viewer_samples`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## stop line

- helper closeout field を増やして completed migration の印象を作らない
- `distributed_activation_ordering` を current helper output field としてでっち上げない
- storage detach / network replay / runtime detach lifecycle を collapse しない
- runtime-crate ownership と final public ABI を同じ package で固定しない
