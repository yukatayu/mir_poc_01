# samples/clean-near-end

このディレクトリは、repo-local alpha 向けの **active clean sample suite** です。

## current rule

- active canonical sample はこのディレクトリだけを正本として扱います
- old sample は archive に残しても、active path には戻しません
- domain predicate は built-in ではありません
- authority / label / capture / region / cost は user-defined finite theory です
- first strong typing layer は finite decidable index fragment です
- mutex / weak-memory family は model-check second line です
- raw `◯` / `□` や low-level `memory_order_*` は final public source principal として扱っていません

## layout

- `00_index_theories.mir`
- `typing/`
- `order-handoff/`
- `model-check/`
- `modal/`
- `sugoroku-world/`

## built-in と user-defined の境界

current helper / sample line の built-in vocabulary は、`module`、`index`、`policy`、`principal`、`resource`、`effect`、`perform`、`require`、`ensure`、`transition`、`stage`、`publish`、`handoff`、`witness`、`model`、`property` などの最小構文語です。

一方で、`SecurityLabel`、`FingerprintAuthority`、`CaptureScope`、`Region`、`CostBudget`、`FingerprintReleasePolicy`、`AuthorityDrawWitness` などは user-defined です。

## 実行コマンド

```bash
python3 scripts/clean_near_end_samples.py list
python3 scripts/clean_near_end_samples.py run typing --format json
python3 scripts/clean_near_end_samples.py run order-handoff --format json
python3 scripts/clean_near_end_samples.py run model-check --format json
python3 scripts/clean_near_end_samples.py run modal --format json
python3 scripts/clean_near_end_samples.py matrix --format json
python3 scripts/clean_near_end_samples.py closeout --format json
```

Sugoroku world vertical slice は別 helper で実行します。

```bash
python3 scripts/sugoroku_world_samples.py list
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary
python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership
python3 scripts/sugoroku_world_samples.py model-check
python3 scripts/sugoroku_world_samples.py closeout --format json
```
