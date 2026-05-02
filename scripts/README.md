# scripts

この directory は、**active runner、repo-local helper、detached/export assist、storage/env、tests** を置く。

## current taxonomy

### front-door checks and active runners

- `check_source_hierarchy.py`
  required root docs / specs / plan / support directory が存在するかを見る structural check。current line では `specs/13..17`、`plan/39..43`、`samples/alpha/`、`sub-agent-pro/alpha-0/` も structural presence の対象に入る。文書内容、stale wording、normative consistency、report template completeness は判定しない。
- `validate_docs.py`
  required documentation scaffold、numbered report、report template closeout headings、latest numbered report の required heading presence / order、empty required section、unresolved update-status placeholder を確認する scaffold check。current line では snapshot docs、`samples/README.md` / `scripts/README.md`、`samples/alpha/README.md`、`plan/39..43`、`specs/13..17` も required scaffold に入る。historical report 全体の semantic validation、active/current wording lint、sample execution、Cargo validation は別 command の責務。
- `clean_near_end_samples.py`
- `current_l2_guided_samples.py`
  compatibility wrapper for `list` / `smoke-all` / `closeout` over `clean_near_end_samples.py`
- `sugoroku_world_samples.py`
- `avatar_follow_samples.py`
- `typed_external_boundary_samples.py`
- `network_transport_samples.py`
  runnable helper-local transport canaries are `NET-02` / `NET-03` / `NET-04` / `NET-05`; `NET-01` remains a reported Sugoroku loopback parity anchor rather than a standalone sample ID
- `projection_codegen_samples.py`
- `visual_debugger_viewer_samples.py`

### current-L2 helper / detached loop / support

- `current_l2_*`
  current-L2 source corpus、detached validation loop、diff/export assist、Lean sync、checker support
- `current_l2_model_check_carrier_pipeline.py`
  current-L2 authored source sample の formal-hook smoke から model-check carrier emit までを確認する repo-local conformance helper。production model checker binding ではない。
- `new_report.py`
  report utility
- alpha-specific parser/runtime runner family は still planned であり、まだ `scripts/` に actualize していない
  - `alpha_lifetime_fallback_checker.py`、`alpha_contract_variance_checker.py`、`alpha_cut_save_load_checker.py` は current first checker-floor helper として actualize 済み
  - これは selected `samples/alpha/` sidecar の `expected_static.checked_reason_codes` と synthetic detached artifact を照合する non-public helper であり、shared support は `current_l2_family_checker_support.py` を reuse する。parser/runtime integration ではない
  - `P-A0-07` local-runtime first cut と `P-A0-08` layer-insertion first cut は `scripts/` ではなく `crates/mir-runtime` の `alpha_local_runtime` / `alpha_layer_insertion_runtime` modules, examples, and integration tests に actualize している。current sample identity anchors は `samples/alpha/local-runtime/` と `samples/alpha/layer-insertion/` だが、`.mir` files are still source-ish placeholders rather than parsed inputs
  - `P-A0-09` は `crates/mir-runtime/src/alpha_network_runtime.rs` と example `mirrorea_alpha_network_runtime` を主体にしつつ、thin Docker runner `alpha_network_docker_e2e.py` を `scripts/` に actualize した。これは `samples/alpha/network-docker/` の `NET-02/03/04/05/07/09` を narrow local-container / TCP bridge cut として検証するもので、helper-local `network_transport_samples.py` の canary familyを置き換えない

### storage / env

- `env/`
- `env/mirrorea_storage_env.sh`
  mounted external workdir 前提の env export surface。`MIRROREA_WORKDIR`、`CARGO_TARGET_DIR`、`CARGO_HOME`、LLVM staging path、mount/ownership/writable status を返し、`--ensure-dirs` は unmounted default root への heavy dir 作成を拒否する
- `storage/`
- `storage/setup_mirrorea_workdisk_root.sh`
  mount / fstab / symlink / ownership repair を伴う one-time root setup path。routine helper ではない
- `storage/detach_prepare.sh`
  non-destructive storage audit。disk / mount / repo usage / external workdir usage / LLVM staging dir ownership / disposable candidates を確認する
- `storage/cleanup_disposable_artifacts.sh`
  explicit `--confirm` 必須の disposable cleanup helper。known disposable dir だけを対象にし、`llvm/src` は意図的に対象外、`llvm` parent が non-writable な場合の build/install cleanup も拒否する

### tests

- `tests/`

## reading rules

- active repo-local command path は上記 front-door runner を先に使う
- `current_l2_guided_samples.py` は current-L2 front-door compatibility path であり、legacy bundle commands は持たない
- `current_l2_*` helper 群は public installed CLI ではなく repo-local support surface として読む
- `samples/alpha/` 向けの future runner 名は roadmap / sample matrix にだけ置き、実在しない command を current validation floor に入れない
- `alpha_network_docker_e2e.py` は current actualized command だが、active clean-suite front door ではなく Alpha-0 package closeout evidence command として読む
- storage / env script は root setup と cleanup policy を helper 本体から分離する

## staged reorganization policy

- いまは flat layout を維持する
- future に `samples/`, `validation/`, `docs/`, `visualization/` などへ rebucket する可能性はある
- ただし active alpha command を壊す move は、wrapper / alias なしでは行わない
