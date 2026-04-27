# scripts

この directory は、**active runner、repo-local helper、detached/export assist、storage/env、tests** を置く。

## current taxonomy

### front-door checks and active runners

- `check_source_hierarchy.py`
- `validate_docs.py`
- `clean_near_end_samples.py`
- `current_l2_guided_samples.py`
- `sugoroku_world_samples.py`
- `avatar_follow_samples.py`
- `network_transport_samples.py`

### current-L2 helper / detached loop / support

- `current_l2_*`
  current-L2 source corpus、detached validation loop、diff/export assist、Lean sync、checker support
- `new_report.py`
  report utility

### storage / env

- `env/`
- `storage/`

### tests

- `tests/`

## reading rules

- active repo-local command path は上記 front-door runner を先に使う
- `current_l2_*` helper 群は public installed CLI ではなく repo-local support surface として読む
- storage / env script は root setup と cleanup policy を helper 本体から分離する

## staged reorganization policy

- いまは flat layout を維持する
- future に `samples/`, `validation/`, `docs/`, `visualization/` などへ rebucket する可能性はある
- ただし active alpha command を壊す move は、wrapper / alias なしでは行わない
