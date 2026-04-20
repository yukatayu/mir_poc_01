# ドキュメント要約

## この文書の役割

この文書は、repo 全体の **短い current snapshot** を日本語で読むための入口である。

- 規範判断の正本は `specs/`
- 進捗 snapshot は `progress.md`
- current task map は `tasks.md`
- 長期参照用の repository memory は `plan/`
- 詳細経緯と実行証跡は `docs/reports/`

## まず何の repo か

この repo は、次の 4 系統を **分離可能なまま** 設計・検証する specification-first research repo である。

- **Mir**: 計算の意味、因果、contract、effect、ownership、lifetime、安全な進化
- **Mirrorea**: logical naming、routing、overlay insertion、audit、dynamic reconfiguration
- **PrismCascade**: media domain の独立 kernel
- **Typed-Effect Wiring Platform**: service / container / legacy integration の inspectable effect layer

current 主眼は Mir current-L2 line にあり、他の subsystem は boundary を保ちながら扱う。

## 現在の正確な読み

この repo は **done** ではない。
ただし、current-L2 の runnable / inspectable floor はかなり厚い。

2026-04-21 時点で、少なくとも次は repo 内で再確認できる。

- authored sixteen と corrected prototype set `p01 ... p16`
- Rust runtime / CLI による sample 実行
- helper-local verification preview / artifact preview
- Problem 1 の typed / theorem / model-check representative bundle
- Problem 2 の order / handoff / authoritative-room representative bundle
- Lean foundation の self-contained proof fragment
- representative sample 由来の generated Lean stub corpus

重要なのは、**「repo-local near-end まで来ている」** と
**「final public contract まで終わった」** を分けることだ。

## 実際に動いているもの

### Problem 1

current first line は次である。

- checker-adjacent first strong typing layer
- notebook-first theorem line
- row-local model-check carrier first

repo で直接確かめられる代表サンプル:

- `p06-typed-proof-owner-handoff`
  - typed / theorem / model-check bridge の representative
- `p10-typed-authorized-fingerprint-declassification`
  - authority あり declassification success
- `p11-typed-unauthorized-fingerprint-release`
  - authority 欠如 rejection
- `p12-typed-classified-fingerprint-publication-block`
  - label-flow mismatch rejection
- `p15-typed-capture-escape-rejected`
  - capture / lifetime rejection
- `p16-typed-remote-call-budget-exceeded`
  - simple cost rejection

Lean 側は 2 層で読む。

- `samples/lean/foundations/`
  - `CurrentL2IfcSecretExamples.lean`
  - `CurrentL2FiniteIndexFirstLayer.lean`
  - `CurrentL2LabelModel.lean`
  - `CurrentL2ProofSkeleton.lean`
  これらは actual small proof fragment で、Lean で通る
- `samples/lean/current-l2/`
  - representative sample から生成した theorem stub
  - Lean は受理するが `sorry` を含む
  - したがって、意味は **artifact well-formedness / bridge alignment** であって、
    completed theorem discharge ではない

### Problem 2

current first line は次である。

- relation decomposition principal
- authoritative-room first default
- reserve strengthening lane split
- negative static-stop pair

repo で直接確かめられる代表サンプル:

- `p07-dice-late-join-visible-history`
  - representative success
- `p08-dice-stale-reconnect-refresh`
  - representative success
- `p09-dice-delegated-rng-provider-placement`
  - delegated RNG reserve route
- `p13-dice-late-join-missing-publication-witness`
  - publication witness 欠如で static stop
- `p14-dice-late-join-handoff-before-publication`
  - publish より先に handoff が現れて static stop

ここでの practical line は **order / handoff / authoritative-room** である。
low-level `memory_order` exact surface は current public line には上げていない。

## まだ done ではないもの

次は current docs で明示的に stop line として残す。

- final public parser grammar
- final public parser / checker / runtime API
- stronger typed source principal
- final public theorem result object
- consumer-shaped theorem payload public contract
- concrete theorem prover brand
- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- final public verifier contract
- low-level `memory_order` exact reinterpretation / exact source surface
- final source-surface handoff wording
- final public witness schema / provider receipt schema / combined contract
- packaging / installed binary / FFI / engine adapter

## すぐに確認できる入口

### representative bundle 全体

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
```

### Problem 1

```bash
python3 scripts/current_l2_guided_samples.py show problem1
python3 scripts/current_l2_guided_samples.py bundle problem1
python3 scripts/current_l2_guided_samples.py emit-theorem problem1
python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line
```

### Problem 2

```bash
python3 scripts/current_l2_guided_samples.py show problem2
python3 scripts/current_l2_guided_samples.py bundle problem2
python3 scripts/current_l2_guided_samples.py emit-scenario problem2
python3 scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness
python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service
```

### Lean foundation

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
source "$HOME/.elan/env" && python3 scripts/current_l2_lean_sample_sync.py
```

## 読み進め方

1. `specs/00-document-map.md`
   - 文書マップと各文書の役割
2. `specs/01-charter-and-decision-levels.md`
   - L0 / L1 / L2 / L3 の区別
3. `specs/02-system-overview.md`
   - 4 系統の役割
4. `specs/03-layer-model.md`
   - レイヤー境界
5. `specs/09-invariants-and-constraints.md`
   - 守るべき不変条件
6. `progress.md`
   - どこまで来ているか
7. `tasks.md`
   - 今どこから再開するのが自然か
8. `plan/01-status-at-a-glance.md`
   - 長期参照寄りの status memory

## この文書で意図的に省いたこと

この文書では、旧 FAQ 群や package 番号の細かい列挙は省いた。
細かな経緯が必要なら `docs/reports/` と `specs/examples/` を辿る。

この文書の主眼は、次の 3 点だけを短く保つことである。

- 今、何が実際に動くか
- それは何を意味し、何をまだ意味しないか
- どの command から再確認すればよいか
