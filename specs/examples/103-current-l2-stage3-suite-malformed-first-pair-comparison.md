# 103 — current L2 stage 3 suite malformed first pair comparison

## 目的

この文書は、`specs/examples/102-current-l2-stage3-suite-malformed-vs-request-compare-sequencing.md` で
fixture-side full request contract compare より先に
**request-local suite bridge family の helper-local malformed/source extension を比較する**
と整理したことを前提に、
**その malformed/source family の first pair をどこから切るのが最小か**
を narrow に比較する。

ここで固定するのは final diagnostics taxonomy でも public parser error API でもない。
固定するのは、current stage で first pair として actualize してよい
**helper-local malformed/source pair の選び方** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- fixed two-slot suite bridge first tranche は actualize 済みである。
- request-local suite bridge family の next step は malformed/source extension を先に比較する line で固定済みである。
- duplicate `require` と clause-between blank line は already actualized している。

## current issue

current helper が暗黙に持つ、またはすぐ隣にある malformed/source 候補は次である。

- duplicate `ensure`
- missing multiline `ensure:` block
- unsupported direct child line inside suite

次段では、このうちどの 2 件を first pair として actualize するのが最小かを決める必要がある。

## 比較する 3 案

### 案 1. duplicate `ensure` + unsupported direct child line

```text
perform write_profile on profile_doc
  ensure owner_is(session_user)
  ensure owner_is(session_user)
```

```text
perform write_profile on profile_doc
  require write
  note delegated
```

#### 利点

- duplicate `require` と対になる at-most-one symmetry を補える。
- unsupported direct child line を明示することで、suite helper が generic continuation parser ではないことを source-backed に示せる。
- suite-level malformed family としてまとまりがよい。

#### 欠点

- missing `ensure:` block が still implicit なまま残る。

### 案 2. duplicate `ensure` + missing multiline `ensure:` block

```text
perform write_profile on profile_doc
  ensure:
```

#### 利点

- duplicate symmetry と clause payload 欠落をまとめて扱える。
- `ensure` 側だけの local pair としては分かりやすい。

#### 欠点

- unsupported direct child line が hidden generic failure のまま残りやすい。
- `missing multiline block` は already multiline attachment bridge family の延長に近く、suite bridge family 固有の前進量がやや弱い。

### 案 3. missing multiline `ensure:` block + unsupported direct child line

#### 利点

- `ensure:` multiline family と non-generic continuation line の両方を触れる。

#### 欠点

- duplicate `ensure` symmetry が残る。
- at-most-one floor の actual evidence が片側だけになる。

## 比較

### suite bridge family 固有の前進量

- 案 1 は at-most-one symmetry と non-generic continuation boundary を同時に示せるため、suite bridge family 固有の前進量が最も大きい。
- 案 2 は `missing multiline block` が multiline attachment bridge family と近く、suite bridge family 固有の tightening としては少し弱い。
- 案 3 も duplicate symmetry を後ろへ残す。

### hidden behavior の抑制

- unsupported direct child line は current helper の hidden fail-closed path になりやすいため、first pair に入れる価値が高い。
- この観点で案 1 と案 3 が有利だが、案 1 は duplicate symmetry も一緒に補える。

### staged line との整合

- current suite bridge first tranche は `duplicate require` を already 持っている。
- その次段としては duplicate symmetry を補いつつ generic continuation との誤読も抑える案 1 が最も連続的である。

## current judgment

current repo の next narrow malformed/source pair としては、
**案 1. duplicate `ensure` + unsupported direct child line**
を採るのが最も自然である。

## なぜこれが最小か

- `require` 側 already-actualized family と対になる `ensure` 側 symmetry を補える。
- suite helper が generic continuation parser ではないことを source-backed に固定できる。
- missing multiline `ensure:` block は still later stage に残しても、suite bridge family の境界は十分明確になる。

## current stage でまだやらないこと

- missing multiline `ensure:` block family の actualization
- fixture-side full request contract compare
- public diagnostics carrier

## next narrow step

次は、
**duplicate `ensure` + unsupported direct child line の helper-local malformed/source pair を actualize する**
のが自然である。
