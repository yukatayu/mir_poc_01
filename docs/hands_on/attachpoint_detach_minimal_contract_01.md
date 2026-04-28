# AttachPoint detach minimal contract 01

`R2` の current hands-on では、attach / detach helper-local evidence を読んで、
current minimal contract row と kept-later gate を見分けます。

## 実行コマンド

```bash
python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
```

## まず見る場所

`01_runtime_attach_game` では次を見ます。

- `hotplug_lifecycle.compatibility`
- `hotplug_lifecycle.activation_cut`
- `attach_request#1`
- `attach_lifecycle`
- `attach_activation#1`

`09_detach_todo` では次を見ます。

- `hotplug_lifecycle.detach_boundary`
- `hotplug_lifecycle.migration_contract`
- `detach_request#1`
- `detached_roll_request#1`
- `detach_lifecycle`
- `detach_boundary#1`

## current reading

- attach request と active state mutation は別
- detach boundary と durable migration complete は別
- helper-local `hotplug_lifecycle` は `MessageEnvelope` 由来の evidence summary
- helper-local view / telemetry inventory は final public contract ではない

## current stop line

- runtime-crate hot-plug engine
- rollback protocol
- durable migration engine
- final public hot-plug ABI
- storage detach script と runtime detach lifecycle の collapse

## 関連

- `../research_abstract/attachpoint_detach_minimal_contract_01.md`
- `../../plan/30-attachpoint-detach-minimal-contract.md`
- `current_phase_closeout_01.md`
