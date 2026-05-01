# AttachPoint detach minimal contract 01

## 何を固定したか

`R2` では、helper-local `hotplug_lifecycle` / explicit detach TODO boundary を
helper-local evidence floor に保ったまま、
`AttachPoint` line の最小 contract row を docs-first に整理した。

ここでの minimal row は次の 4 つだけです。

- `compatibility`
- `activation_cut`
- `detach_boundary`
- `migration_contract`

## boundary reading

- helper-local `hotplug_lifecycle` は `MessageEnvelope` 由来の evidence summary
- final public hot-plug ABI ではない
- attach request と active state mutation は別
- detach TODO boundary と durable migration complete は別
- storage detach script と runtime detach lifecycle も別
- `attachpoint_id` / `patch_id` は identity column として残し、
  row は envelope / view / telemetry anchor に紐づけて読む

## evidence anchor

- sample:
  `01_runtime_attach_game` / `09_detach_todo`
- envelope:
  `attach_request#1` / `detach_request#1` / `detached_roll_request#1`
- view:
  `attach_lifecycle` / `detach_lifecycle`
- telemetry:
  `attach_activation#1` / `detach_boundary#1`
- layer row:
  `hotplug_activation_boundary` / `hotplug_detach_boundary`

## 何が分かるか

- attach / detach は helper-local floor でも explicit lifecycle boundary として読める
- compatibility / freshness / capability / authorization を request envelope と cut boundary に残したまま読める
- detach 後の domain action rejection は visible な boundary evidence として読める
- migration / rollback / final public ABI はまだ未決だと明示したまま保てる

## 何を言っていないか

- runtime-crate hot-plug engine がある
- durable migration engine がある
- rollback protocol が決まった
- final public hot-plug ABI が決まった

## 関連

- `../../plan/30-attachpoint-detach-minimal-contract.md`
- `hotplug_attachpoint_plan_01.md`
- `../hands_on/attachpoint_detach_minimal_contract_01.md`
