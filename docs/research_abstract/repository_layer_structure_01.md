# repository_layer_structure_01

この文書は、repo を **layer-aware に読むための短い reader-facing summary** です。

## 何を分けて読むか

- `specs/`
  規範正本
- `plan/`
  repository memory
- `docs/reports/`
  historical evidence
- `progress.md` / `tasks.md` / `samples_progress.md`
  current snapshot

## code lane

- `crates/mir-ast`
  parser / AST carrier
- `crates/mir-semantics`
  semantics / proof / model-check bridge
- `crates/mir-runtime`
  current runner / CLI / report-local evidence carrier
- `crates/shared-*`
  shared utility placeholder lane
- `crates/mirrorea-*`
  Mirrorea runtime/control placeholder lane
- `crates/prism-*`
  PrismCascade separate lane
- `crates/engine-abi`
  future adapter / host boundary placeholder

今は flat workspace のままですが、責務はこのように分けて読みます。
current repo-local alpha を壊す crate rename / move は、まだ行いません。

## sample lane

- `samples/clean-near-end/`
  active canonical executable suite
  - `typed-external-boundary/`
    phase 9 planned source family。current repo では synthetic preview helper がこの source stub family を参照する
- `samples/current-l2/`
  base current-L2 source corpus
- `samples/lean/`
  mechanization evidence
- `samples/not_implemented/`
  planned skeleton family
- `samples/prototype/`
  historical prototype / compatibility anchor
- `samples/old/`
  archive
- `samples/generated/`
  generated sample artifact reserve

## script lane

- active runner:
  `clean_near_end_samples.py`
  `current_l2_guided_samples.py`
  `sugoroku_world_samples.py`
  `avatar_follow_samples.py`
  `typed_external_boundary_samples.py`
  `network_transport_samples.py`
- docs / hierarchy check:
  `check_source_hierarchy.py`
  `validate_docs.py`
- repo-local support:
  `current_l2_*`
- storage / env:
  `scripts/env/`
  `scripts/storage/`

## repository-memory recommendation

- まず docs / plan / README で taxonomy を固定する
- active path を壊す move は later stage へ送る
- final public package structure と repo-local alpha current layout を混同しない
