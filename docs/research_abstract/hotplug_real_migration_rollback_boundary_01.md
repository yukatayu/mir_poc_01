# hot-plug real migration / rollback boundary 01

## 何を固定したか

`R4` では、`R2` で固定した
`compatibility / activation_cut / detach_boundary / migration_contract`
minimal contract row の先にある

- real migration
- rollback protocol
- runtime-crate hot-plug engine
- distributed activation ordering
- final public hot-plug ABI

を、**current helper-local evidence がまだ証明していない kept-later boundary**
として docs-first に切り分けました。

## current evidence

current anchor は次です。

- `01_runtime_attach_game`
- `09_detach_todo`
- `attach_request#1 / detach_request#1 / detached_roll_request#1`
- `attach_lifecycle / detach_lifecycle`
- `attach_activation#1 / detach_boundary#1`
- helper closeout `hotplug_scope = helper_local_package_manager_preview`

ここから読めるのは、
requested attach と active state mutation の分離、
detach 後の domain action rejection の explicitness、
そして migration / rollback が still deferred だという honest stop line までです。

## 読み違えてはいけないこと

- `activation_cut` は distributed activation ordering ではありません
- `migration_contract.status = deferred` は migration schema の存在証明ではありません
- rejected `detached_roll_request#1` は completed rollback ではありません
- storage detach script と runtime detach lifecycle は別 concern です
- helper-local `hotplug_lifecycle` は runtime-crate engine ownership や final public ABI ではありません

## current recommendation

helper floor は widening しません。
`migration_contract` は protocol ではなく unresolved-state honesty lane として読みます。
historical next relation at `R4` close time では、helper-local preview と crate-side runtime ownership を分ける
`R5` runtime-crate hot-plug engine ownership cut を読むのが自然でした。
current repo state では `R5` / `R6` / `P19` / `P20` も close 済みであり、post-`P20` の current promoted-next package は未昇格です。

## 関連

- `../hands_on/hotplug_real_migration_rollback_boundary_01.md`
- `../../plan/32-hotplug-real-migration-rollback-boundary.md`
- `attachpoint_detach_minimal_contract_01.md`
- `hotplug_attachpoint_plan_01.md`
