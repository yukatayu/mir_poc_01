# 27 — current L2 fixture scaffold helper

## 目的

この文書は、current L2 parser-free PoC 基盤で new fixture を 1 本追加するときの
**non-production scaffold helper** を整理する。

ここで固定するのは final authoring DSL や parser ではない。
固定するのは、hand-written fixture を正本に保ったまま、

- required carrier の骨組みを作る
- runtime fixture なら `.host-plan.json` sidecar の空骨格を作る
- overwrite を fail-closed にする

という narrow helper boundary だけである。

## 適用範囲

- current L2 の core semantics、parser grammar、failure family は変えない
- fixture expectation の意味は helper が推論しない
- generated scaffold は completed fixture ではなく、authoring 作業前の骨組みである
- hand-written fixture と review された expectation を正本に保つ
- final authoring format、final parser syntax、production authoring API は固定しない

## current helper の位置づけ

current narrow cut では、fixture authoring bottleneck のうち
**boilerplate 作成だけ** を helper に逃がしてよい。

その actual helper は `scripts/current_l2_scaffold_fixture.py` である。

この helper は次を行う。

- fixture stem から fixture JSON path を作る
- runtime / static-only の別に応じた minimal skeleton を作る
- runtime fixture なら empty `.host-plan.json` sidecar を作る
- target が既にある場合は `--overwrite` 無しで失敗する

この helper は次を行わない。

- program body の意味を推論すること
- `expected_static` / `expected_runtime` / `expected_trace_audit` を completion 済み値で埋めること
- detached artifact exporter や batch helper の public API を増やすこと
- final parser / authoring syntax を既成事実化すること

## minimal output shape

### fixture JSON

helper が最低限作ってよいのは、次の top-level fields を持つ skeleton に限る。

- `schema_version`
- `fixture_id`
- `source_example_id`
- `program`
- `expected_static`
- `expected_runtime`
- `expected_trace_audit`

### runtime fixture skeleton

runtime scaffold では、

- `expected_runtime.enters_evaluation = true`
- `expected_runtime.final_outcome = "OPEN_RUNTIME_OUTCOME"`
- `expected_static.verdict = "OPEN_STATIC_VERDICT"`

のような placeholder を残してよい。

理由は、runtime expectation の completion を helper が推論すると
hidden elaboration になりやすいからである。

### static-only fixture skeleton

static-only scaffold では、

- `expected_runtime.enters_evaluation = false`
- `expected_runtime.final_outcome = "not_evaluated"`

までは helper が埋めてよい。

これは static-only fixture で evaluation に入らないこと自体は
fixture kind の選択から直ちに従うからである。

ただし、

- `expected_static.verdict`
- static rejection reasons
- `must_explain`

は authoring 側が埋める。

## host plan sidecar

runtime scaffold のときだけ、helper は次の shape の empty sidecar を作ってよい。

```json
{
  "schema_version": "current-l2-host-plan-v0",
  "predicate_rules": [],
  "effect_rules": [],
  "trace_expectation_override": {}
}
```

これは authoring 開始点を揃えるための skeleton であり、
coverage complete な host plan を意味しない。

## current non-production output candidate

current helper の default candidate は次とする。

```text
target/current-l2-fixture-scaffolds/
```

この placement を採る理由は次の通りである。

- generated scaffold を repo tracked fixture と誤読しにくい
- `target/` なので cleanup しやすい
- authoring 中の incomplete placeholder を current fixture catalog に混ぜずに済む

## fail-closed overwrite policy

current scaffold helper は fail-closed default を採る。

- target file が既にあるときは失敗する
- `--overwrite` があるときだけ上書きを許す

この policy により、既存 fixture や authoring 中 scaffold の accidental clobber を避ける。

## fixture authoring との接続

current authoring flow では、scaffold helper は次の前段だけを担う。

1. skeleton を作る
2. author が program / expectation / sidecar を埋める
3. fixture review と regression test を通す
4. detached artifact loop に入れて emit / compare する

したがって scaffold helper は、
detached validation loop の replacement ではなく、
**authoring 前段の boilerplate cut** に留まる。

## OPEN に残すもの

- fixture scaffold helper を detached loop wrapper に統合するか
- scaffold placeholder の文字列 naming を長期固定するか
- fixture skeleton を row template にするか object template にするか
- final authoring DSL / parser syntax とどう接続するか
- scaffold helper を repo tracked fixture directory に直接書ける mode を許すか

## current judgment

- fixture authoring bottleneckのうち、boilerplate 作成だけを thin helper に逃がしてよい
- helper は semantics inference をしてはならない
- runtime fixture なら empty `.host-plan.json` sidecar を作ってよい
- default output は `target/current-l2-fixture-scaffolds/` としてよい
- hand-written fixture と reviewed expectation を引き続き正本に保つ
