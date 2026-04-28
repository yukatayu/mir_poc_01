# plan/30 — AttachPoint compatibility / detach minimal contract

## 目的

この文書は、`P14` helper-local package-manager first-cut closeout の後に残る
`R2` docs-first package として、
`AttachPoint` attach / detach lifecycle の **current minimal contract row** を
repo memory として固定する。

ここで固定するのは helper-local evidence floor の読み方であり、
final public hot-plug ABI、runtime-crate hot-plug engine、rollback protocol、
durable migration engine を固定することではない。

## current fixed reading

- helper-local `hotplug_lifecycle` は current minimal contract を読むための
  evidence carrier である
- helper-local `hotplug_lifecycle` は `MessageEnvelope` 由来の evidence summary であり、
  final public hot-plug ABI ではない
- authoritative seam は accepted / rejected `MessageEnvelope` に残し、
  authentication / authorization / membership / capability / witness を collapse しない
- attach request と active state mutation を同一視しない
- detach TODO boundary と durable migration complete を同一視しない
- storage detach script と runtime detach lifecycle を同一視しない
- current minimal contract row は
  `compatibility / activation_cut / detach_boundary / migration_contract`
  の 4 row に限る
- `attachpoint_id` と `patch_id` は current row の identity column として扱い、
  helper-local envelope / view / telemetry anchor と結びつけて読む

## current evidence anchors

### active sample anchors

- `samples/clean-near-end/sugoroku-world/01_runtime_attach_game.mir`
  - attach compatibility
  - activation cut
  - helper-local active state `attached_active`
- `samples/clean-near-end/sugoroku-world/09_detach_todo.mir`
  - detach TODO boundary
  - post-detach domain-action rejection
  - helper-local boundary state `detached_todo_boundary`

### message-envelope anchors

- `attach_request#1`
  - accepted attach request at `attach_point_boundary`
- `detach_request#1`
  - accepted detach request at `detach_point_boundary`
- `detached_roll_request#1`
  - rejected post-detach domain request at `game_action_boundary`

### view / telemetry anchors

- views:
  `attach_lifecycle` / `detach_lifecycle`
- telemetry rows:
  `attach_activation#1` / `detach_boundary#1`

### layer-signature anchors

- `hotplug_activation_boundary`
- `hotplug_detach_boundary`

## current minimal contract matrix

| Row | Current carrier | Current evidence floor | What current evidence proves | What current evidence does not prove |
|---|---|---|---|---|
| compatibility | `hotplug_lifecycle.compatibility` | required capability, required authority, membership freshness, package check, accepted request envelope | current helper-local attach/detach request is checked against explicit capability / freshness / package constraints before lifecycle state changes | final public package ABI、real distributed compatibility negotiation、runtime-crate package manager ownership |
| activation_cut | `hotplug_lifecycle.activation_cut` | accepted `attach_request#1`, preconditions, visible frontier after cut, `hotplug_activation_boundary` | requested attach and active state mutation are distinct; active state appears only after the current cut preconditions are satisfied | rollback-safe reactivation protocol、distributed activation ordering、durable activation commit |
| detach_boundary | `hotplug_lifecycle.detach_boundary` | accepted `detach_request#1`, rejected `detached_roll_request#1`, `hotplug_detach_boundary`, visible detached frontier | detach is an explicit lifecycle boundary, and post-detach domain action rejection is visible in the current helper-local floor | completed migration、state transfer、reattach protocol、final detach replay contract |
| migration_contract | `hotplug_lifecycle.migration_contract` | `not_started` / `deferred` status, stop-line notes, kept-later gates | current line intentionally keeps migration / rollback unresolved and records that fact as evidence rather than silently claiming completion | durable migration engine、rollback protocol、final public migration schema、runtime hot-upgrade engine |

## current lane inventory

### hotplug lifecycle lanes

- `attachpoint_id`
- `patch_id`
- `lifecycle_state`
- `compatibility`
- `activation_cut`
- `detach_boundary`
- `migration_contract`

### grounding envelope lanes

current grounding carrier は `message_envelopes` であり、少なくとも次の split を保つ。

- `transport_medium`
- `transport_seam`
- `emitter_principal`
- `membership_epoch`
- `member_incarnation`
- `freshness_checks`
- `capability_requirements`
- `authorization_checks`
- `witness_refs`
- `dispatch_outcome`

この split は hot-plug line でも collapse しない。

## explicit non-equivalences

- helper-local `hotplug_lifecycle`
  != final public hot-plug ABI
- `attach_request#1`
  != active state mutation itself
- `detach_request#1`
  != durable migration completion
- `detached_roll_request#1` rejection
  != rollback protocol completion
- `scripts/storage/detach_prepare.sh`
  != runtime detach lifecycle
- helper-local `attach_lifecycle` / `detach_lifecycle`
  != final public viewer contract

## kept-later gates

- `runtime_crate_hotplug_engine`
- `rollback_protocol`
- `durable_migration_engine`
- `final_public_hotplug_abi`
- `distributed_activation_ordering`

## validation floor

- `python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
- `python3 -m unittest scripts.tests.test_visual_debugger_viewer_samples`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

## current recommendation

- current minimal contract row は helper-local evidence floor に留める
- attach / detach wording を広げるときも、accepted/rejected `MessageEnvelope` seam を正本に残す
- migration / rollback / final public ABI を、current helper-local lifecycle evidence に仮託して claim しない
- `R2` closeout 後の promoted next line は、`FAIRY-05` carrier bundling を docs-first に narrow にする `R3` として読む
