# 24 — current L2 detached export storage and aggregate API

## 目的

この文書は、current L2 parser-free PoC 基盤の detached exporter chain を、

- 1 bundle の detached artifact を保存する
- 後でその artifact を比較する
- さらに batch aggregate export と narrow に接続する

ところまで進めるための、**storage / path policy** と **aggregate exporter API の最小 cut** を docs-only で整理する。

ここで固定するのは production exporter API ではない。
固定するのは、current detached validation loop を回しやすくするための **non-production の最小運用面** である。

## 適用範囲

この文書は次を前提にする。

- current L2 の core semantics は変えない
- payload core / `bundle_context` / `detached_noncore` / human-facing explanation の cut は維持する
- bundle-first detached exporter の first entry は `run_bundle` / `BundleRunReport` とする
- `host_plan_coverage_failure` は current success artifact では aggregate-only に残す
- future typed bundle failure artifact の最小核は `failure.failure_kind` discriminator-only とする
- aggregate 側に typed 集約を持たせるなら最小候補は `bundle_failure_kind_counts` とする
- current bool/list anchor と typed count field の additive coexistence を許す
- `fixture authoring / elaboration` は独立 bottleneck のまま残す

## current detached validation loop の接続点

current detached validation loop では、実行境界を次の 2 つに分けて考える。

1. bundle-first export
2. batch aggregate export

bundle-first export は `run_bundle` / `BundleRunReport` を起点に 1 bundle artifact を作る。
batch aggregate export は `BatchRunSummary` を起点に coarse summary を作る。

この 2 つを 1 つの artifact に混ぜない理由は次の通りである。

- bundle-first artifact は exact-compare core と bundle identity を 1 対 1 で保ちやすい
- aggregate export は selection / profile / batch execution の coarse summary に留める方が helper boundary を壊しにくい
- `host_plan_coverage_failure` のような batch classifier 起源の signal を、success-side payload core に逆流させずに済む

## aggregate export の最小 docs-only API cut

### current understanding

aggregate export が持つべき current minimal responsibilities は次である。

- `BatchRunSummary` を coarse summary として保持する
- current bool/list anchor を維持する
  - `BatchBundleOutcome::Failed.host_plan_coverage_failure`
  - `BatchRunSummary.host_plan_coverage_failures`
- future typed aggregate を足すなら `bundle_failure_kind_counts` を additive に追加する

### future typed aggregate の最小 shape

aggregate 側に typed 集約を持たせるなら、最小は histogram / kind count である。
ただし current migration cut では **migrated kind だけを数える partial histogram** として始め、coverage 以外の bundle failure を黙って full histogram に見せない。

```json
{
  "bundle_failure_kind_counts": [
    {
      "failure_kind": "host-plan-coverage-failure",
      "count": 1
    }
  ]
}
```

ここで aggregate 側へ持ち込まないものは次である。

- bundle failure summary の薄い再掲
- `bundle_context`
- detached non-core explanation
- `must_explain`

### additive coexistence の current docs-only judgment

current docs-only の migration cut は、**置換ではなく additive coexistence** である。

したがって aggregate export 側では、当面次を併存させる。

- current `host_plan_coverage_failures` list
- current `BatchBundleOutcome::Failed.host_plan_coverage_failure` bool
- future `bundle_failure_kind_counts`

この coexistence を current L2 で許す理由は次の通りである。

- current harness error wording と batch classifier anchor を直ちに壊さない
- docs / tests / code の mirror を 1 task で全面更新しなくて済む
- typed aggregate naming を先に切り、actual exporter API finalization は後段へ残せる

## artifact 保存先 / path policy の比較

### 比較した候補

#### 候補 1. repo 追跡下の固定ディレクトリ

例:

```text
artifacts/current-l2-detached/
```

これは今回の最小案としては採らない。

理由:

- generated artifact を repo 管理対象に見せやすい
- production manifest / final path policy を早く固定したように見えやすい
- cleanup rule と overwrite policy を別途強く決めたくなる

#### 候補 2. 完全に外部の一時ディレクトリ

例:

```text
/tmp/current-l2-detached/
```

これは override としては許容できるが、current default にはしない。

理由:

- repo 内ドキュメントから path を説明しづらい
- compare input discovery を repo 相対で書けない
- task ごとの evidence path が人によって散りやすい

#### 候補 3. repo 内の gitignored build area

例:

```text
target/current-l2-detached/
```

これを current non-production default candidate とする。

理由:

- 既に `/target/` は `.gitignore` 対象である
- repo 相対の運用説明を書きやすい
- production storage policy を固定しないまま、generated artifact の置き場だけを narrow に揃えられる
- cleanup も build artifact と同じ感覚で扱いやすい

## current non-production storage layout 候補

current non-production default candidate は次とする。

```text
target/current-l2-detached/
  bundles/
    <run-label>/
      <fixture-stem>.detached.json
  aggregates/
    <run-label>/
      batch-summary.detached.json
```

### layout の意味

- `bundles/`
  - bundle-first exporter が出す 1 bundle artifact を置く
- `aggregates/`
  - 将来 narrow に batch aggregate export を追加するときの sibling placement を確保する
- `<run-label>`
  - 1 回の validation run を人間が区別するための最小 label
- `<fixture-stem>.detached.json`
  - current bundle-first artifact の最小 naming

### current docs-only judgment

- current sprint では `bundles/` placement を先に整える
- `aggregates/` placement は docs-only reserve に留める
- final path canonicalization policy は未決のまま残す

## file naming / compare input discovery / overwrite policy

### file naming の最小候補

bundle artifact:

```text
<fixture-stem>.detached.json
```

aggregate artifact:

```text
batch-summary.detached.json
```

これは current loop で必要な識別が

- bundle 側では fixture stem
- aggregate 側では run label

でほぼ足りるためである。

### compare input discovery の最小候補

current loop では compare input discovery を過剰に一般化しない。
最小候補は次の 2 段である。

1. explicit artifact path を直接渡す
2. convenience helper が `(artifact_root, run_label, fixture_stem)` から bundle artifact path を導出する

current L2 では、recursive scan や manifest-driven discovery はまだ採らない。

### overwrite policy の最小候補

current non-production loop では、**fail-closed default** を採る。

- target path が既に存在するときは失敗させる
- 明示的な `--overwrite` がある場合だけ上書きを許す

この policy を採る理由は次の通りである。

- どの artifact がどの run の evidence かを曖昧にしない
- compare result が暗黙の上書きに依存しない
- current docs-only loop を production-style retention policy へ早く寄せすぎない

## actual exporter API でまだ未決のもの

### docs-only で先に決めるもの

- bundle-first export を `run_bundle` / `BundleRunReport` 起点にすること
- aggregate export を `BatchRunSummary` 起点の coarse summary に留めること
- `bundle_failure_kind_counts` を typed aggregate の最小候補とすること
- storage root の current non-production candidate を `target/current-l2-detached/` とすること
- run label / fixture stem / explicit paths を基本に compare input を導くこと

### 引き続き OPEN に残すもの

- actual library/API surface を `lib.rs` / `harness.rs` にどう切るか
- exporter を example / script / lib helper のどこへ昇格させるか
- aggregate exporter を actual 実装へ入れる timing
- `bundle_failure_kind_counts` を row list にするか object map にするか
- current bool/list anchor をいつ除去するか
- final path canonicalization policy
- retained artifact cleanup rule の final contract

## settled current docs-only judgment と next narrow step

### settled current docs-only judgment

- bundle-first detached exporter は `run_bundle` / `BundleRunReport` を起点にする
- aggregate export は `BatchRunSummary` を coarse summary として扱う
- aggregate 側の最小 typed 集約は `bundle_failure_kind_counts`
- current bool/list anchor と typed count field の migration cut は additive coexistence
- current non-production storage root の最小候補は `target/current-l2-detached/`
- overwrite policy の最小候補は fail-closed default

### next narrow step

次に narrow に進めてよいのは次である。

1. tiny wrapper が `target/current-l2-detached/` を default root として bundle artifact を吐けるようにする
2. same wrapper か sibling helper が 2 artifact compare を回せるようにする
3. fixture authoring template に detached validation loop の保存 / 比較手順を追加する
4. aggregate export の actual 実装は、その後に `BatchRunSummary` 起点で narrow に足す

## 依然として OPEN のもの

- actual exporter API finalization
- detached artifact 保存先の final policy
- aggregate exporter の actual serialization shape
- final parser syntax
- richer host interface の typed carrier 化
- multi-request scheduler
- `Approximate` / `Compensate`
- static analysis / theorem prover 側との boundary
