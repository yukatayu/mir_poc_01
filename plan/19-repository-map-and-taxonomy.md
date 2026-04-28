# plan/19 — repository map and taxonomy

## 目的

この文書は、current repo を **layer-aware に読むための repository memory** を与える。

- final public package structure を固定する文書ではない
- crate rename / module move / path move を強制する文書でもない
- current repo-local alpha を壊さずに、今後の安全な staged migration を支えることが目的である

## current top-level reading

### front door / snapshot

- `README.md`
  repo identity と最短入口
- `Documentation.md`
  current reader entry
- `progress.md`
  rough progress snapshot
- `tasks.md`
  current package queue と blocker
- `samples_progress.md`
  runnable sample dashboard

### normative / memory / evidence

- `specs/`
  規範正本
- `plan/`
  repository memory
- `docs/reports/`
  task 単位の証跡
- `docs/research_abstract/`
  reader-facing summary / hands-on

### code lanes

- `crates/mir-ast`
  parser / AST carrier
- `crates/mir-semantics`
  semantics / theorem / model-check bridge
- `crates/mir-runtime`
  current runner / CLI / report-local evidence carrier
- `crates/mir-lsp`
  editor / visualization laneの placeholder
- `crates/mirrorea-core`
  Mirrorea minimal carrier substrate
- `crates/mirrorea-control`
  Mirrorea control-plane placeholder
- `crates/prism-*`
  PrismCascade separate lane の placeholder
- `crates/shared-*`
  cross-subsystem utility placeholder
- `crates/engine-abi`
  future adapter / host boundary placeholder

## current crate classification

| crate | current reading | move risk |
|---|---|---|
| `mir-ast` | Mir language substrate | high |
| `mir-semantics` | Mir verification substrate | high |
| `mir-runtime` | current runner / host-facing proof-of-concept boundary | high |
| `mir-lsp` | visualization / editor-support placeholder | medium |
| `mirrorea-core` | Mirrorea minimal carrier substrate (`LayerSignature` / `PrincipalClaim` / `AuthEvidence` / `MessageEnvelope` / `MembershipRegistry` / `PlaceCatalog`) | medium |
| `mirrorea-control` | Mirrorea control-plane placeholder | medium |
| `prism-meta` / `prism-core` / `prism-runtime` | PrismCascade separate lane placeholder | medium |
| `engine-abi` | external adapter / host boundary placeholder | medium |
| `shared-ids` / `shared-contracts` | shared utility placeholder | low |

current recommendation:

- `mir-ast` / `mir-semantics` / `mir-runtime` は live chain なので、rename / rebucket しない
- `mirrorea-core` は current ownership cut と `P11` first cut が入り始めたが、まだ final public crate shape ではないので rename / rebucket しない
- `mirrorea-control` / `prism-*` / `engine-abi` は placeholder だが、名前自体が subsystem separation を表しているので、早い rename はしない
- crate split / merge は docs-first lane で boundary が固まってから行う

## sample taxonomy

### current active roots

- `samples/clean-near-end/`
  active canonical executable suite
- `samples/current-l2/`
  base current-L2 source corpus
- `samples/lean/`
  mechanization evidence

### current non-active roots

- `samples/not_implemented/`
  planned skeleton family
  - `typed-external-boundary/`
    phase 9 planned source family。current repo では synthetic preview helper がこの source stub family を参照する
- `samples/prototype/`
  historical prototype / compatibility anchor
- `samples/old/`
  archive
- `samples/generated/`
  future non-Lean generated sample artifact reserve

### important boundary

- `samples/lean/clean-near-end/` は generated theorem stub だが、Lean bridge evidence として committed されている
- heavy disposable generated artifact は repo root ではなく external workdir を優先する
- planned skeleton family は active sample として扱わない

## script taxonomy

### current active runners / front-door checks

- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/clean_near_end_samples.py`
- `scripts/current_l2_guided_samples.py`
- `scripts/sugoroku_world_samples.py`
- `scripts/avatar_follow_samples.py`
- `scripts/typed_external_boundary_samples.py`
- `scripts/network_transport_samples.py`

### current repo-local helper / detached loop / support

- `scripts/current_l2_*`
  base current-L2 helper、detached loop、diff/export assist、Lean sync、checker support
- `scripts/new_report.py`
  report utility

### current storage / env

- `scripts/env/`
- `scripts/storage/`

### tests

- `scripts/tests/`

current recommendation:

- active command path は維持する
- re-bucketing が必要なら wrapper / alias を先に残す
- public CLI / installed binary / final host contract と Python helper surface を混同しない

## current docs taxonomy

- `docs/research_abstract/`
  summary / detail / hands-on
- `docs/hands_on/`
  current closeout landing page。詳細な hands-on 本体は当面 `docs/research_abstract/` に残す
- `docs/diagrams/`
  Mermaid source
- `docs/reports/`
  historical evidence

future candidate:

- `docs/history/`
- `docs/visualizer_notes/`

ただし、今は大移動せず、existing `docs/research_abstract/` を current reader-facing root として維持する。
`docs/hands_on/` は landing page だけ actualize し、既存詳細文書の物理移動は later とする。

## target layer-aware map

```text
repo/
  specs/                    # normative
  plan/                     # repository memory
  docs/                     # reader-facing summary + reports
  crates/                   # flat workspace, conceptually separated lanes
  samples/                  # active/base/planned/prototype/archive/generated
  scripts/                  # active runners + helper/support + storage/env + tests
  sub-agent-pro/            # handoff only
```

conceptual lanes:

- Mir language substrate
- Mir verification substrate
- Mirrorea runtime substrate
- external adapter / host boundary
- visualization / debug / telemetry
- samples / hands-on / examples
- docs / specs / reports / planning

## staged migration plan

### Stage 1 — docs-only hardening

- `samples/README.md` と `scripts/README.md` で current taxonomy を固定する
- stale reference を除去する
- `samples_progress.md` で active / base corpus / planned row を読み分けやすくする
- root docs から `plan/19` への導線を付ける

### Stage 2 — taxonomy hardening without breaking commands

- `samples/generated/` を generated sample artifact reserve として使い始める
- archive banner / archive link policy を整理する
- script class の grouping を docs で明示し、必要なら wrapper 先行で再配置する

### Stage 3 — root cleanup

- `faq_*.md`
- `diff_investigation_*.md`
- `handson_tmp`
- `旧資料_*`

を `docs/history/` などへ寄せるかを比較する。
ただし current task では move しない。

### Stage 4 — physical crate/path migration

- `samples/not_implemented` vs `samples/planned`
- `samples/old` vs `samples/archive`
- `scripts/current_l2_*` の rebucket
- future crate split

を必要性と wrapper strategy を持って比較する。
current task では move しない。

## intentionally not moved now

- `crates/mir-ast`
- `crates/mir-semantics`
- `crates/mir-runtime`
- `crates/mirrorea-*`
- `crates/prism-*`
- `crates/engine-abi`
- `samples/not_implemented/avatar-fairy-follow/`
- `samples/prototype/current-l2-dynamic-attach-detach/`
- `mir_hilight.html`
- `scripts/current_l2_*`

理由:

- current repo-local alpha の runnable path を壊すリスクが高い
- final public package structure が未決
- subsystem separation を名前で保持している lane が多い

## open questions

- final physical crate/package layout をどこまで Mir / Mirrorea / adapters / visualization に split するか
- `docs/research_abstract/` 内の詳細 hands-on を `docs/hands_on/` へどこまで移すべきか
- archive docs の stale link をどこまで rewrite するか
- generated sample artifact を `samples/generated/` と external workdir でどう分担するか

## current recommendation

- current repo は **flat workspace + explicit docs taxonomy** で十分に前へ進める
- まず docs / plan / sample/script README を整え、filesystem move は later stage に送る
- current alpha の active command path を壊す move は、wrapper か alias が用意できるまで行わない
