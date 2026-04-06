# 90 — current L2 stage 3 request-local clause spillover comparison

## 目的

この文書は、`specs/examples/89-current-l2-stage3-admit-node-handoff-comparison.md` で
fixture-side `OptionDecl.admit` handoff が current phase では docs-only deferred に留まると整理したことを前提に、
**stage 3 later branch として request-local `require` / `ensure` spillover を helper-local malformed-source smoke にどこまで持たせるべきか**
を narrow に比較する。

ここで固定するのは final parser grammar でも request attachment rule でもない。
固定するのは、stage 3 private helper が later-stage request-local clause を hidden attachment なしに fail-closed で示す
**最小 malformed-source cut** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semanticsは変更しない。
- stage 3 declaration-side `admit` branch の success-side / malformed-source first tranche は actualize 済みである。
- fixture-side `OptionDecl.admit` handoff は current phase では docs-only deferred に留める。
- `PerformVia` request head spillover は helper-local malformed-source first tranche で既に actualize 済みである。
- request-local `require` / `ensure` の semantics anchor は `e10-perform-on-ensure-failure.json` と `e11-perform-via-ensure-then-success.json` にある。

## current source anchor

- `specs/examples/88-current-l2-stage3-admit-next-step-sequencing.md`
  - stage 3 next step は request-local clause spillover より先に handoff comparison を扱った
- `specs/examples/89-current-l2-stage3-admit-node-handoff-comparison.md`
  - handoff line は docs-only deferred に留める
- `specs/examples/07-current-l2-host-stub-harness.md`
  - request-local `require` / `ensure` は request head に attached された contract である
- `crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json`
  - `PerformOn.contract.require` / `ensure`
- `crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json`
  - `PerformVia.contract.require` / `ensure`

current issue は、stage 3 private helper が `PerformVia` request head spillover までは fail-closed に示せている一方で、
bare `require` / `ensure` line を later-stage request-local clause としてどこまで明示的に reject すべきかがまだ未決なことである。

## 比較する 3 案

### 案 1. request-local clause spillover は current helper ではまだ扱わない

`require` / `ensure` line も generic `unsupported stage 3 admit-slot input` に留める。

#### 利点

- helper 実装は最も薄い。
- later-stage request cluster を明示的に増やさない。

#### 欠点

- `PerformVia` request head と bare clause spillover の区別が helper-local wording では見えない。
- request-local clause が request head にしか attach しないという current source anchor が、malformed-source smoke へ反映されない。

### 案 2. bare `require` / `ensure` line を helper-local malformed-source pair として actualize する

`require` と `ensure` を、それぞれ later-stage request-local clause spillover として
substring compare で reject する。

#### 利点

- request attachment rule を parse せずに、later-stage clause boundary を fail-closed に示せる。
- `e10` / `e11` の source anchor と対応しやすい。
- `PerformVia` request head spillover と併せて、request head と request-local clause の両方が still later stage であることを helper-local に示せる。

#### 欠点

- helper-local wording fragment が 2 件増える。
- bare clause line は final grammar 候補ではないため、malformed-source smoke という位置づけを明記し続ける必要がある。

### 案 3. request head + clause attachment shape まで最小 malformed-source actualization に含める

たとえば `perform ...` の直後に `require ...` / `ensure ...` を置く multiline shape まで helper で扱う。

#### 利点

- request cluster により近い malformed-source shape を見られる。

#### 欠点

- request attachment rule を hidden に先食いしやすい。
- current private helper が request head / clause block syntax を部分的に持ち始める。
- stage 3 line の lexical pressure が一気に上がる。

## 比較

### hidden attachment rule の回避

- 案 1 は hidden attachment を増やさないが、later-stage clause boundary も見えない。
- 案 2 は bare clause line の reject に留めるため、attachment rule をまだ持ち込まない。
- 案 3 は request attachment rule を暗黙に持ち込みやすい。

### `PerformVia` spillover との補完性

- 案 1 では request head だけ明示 reject され、request-local clause 側が generic unsupported のまま残る。
- 案 2 は request head と request-local clause の両方を later-stage branch として並べられる。
- 案 3 は補完性は高いが、代償として parse pressure が大きい。

### current source anchor との対応

- `e10` / `e11` はどちらも `require` / `ensure` が request head の contract に attached されることを示している。
- bare clause line を malformed-source pair にする案 2 は、この anchor を「独立 line としてはまだ outside stage 3 helper」であると読む最小 cut である。

## current judgment

current repo の next narrow step としては、
**案 2. bare `require` / `ensure` line を helper-local malformed-source pair として actualize する**
のが最も自然である。

つまり、

1. stage 3 private helper は request head spillover に加えて bare request-local clause spillover も fail-closed に示す
2. wording は `require` と `ensure` を区別した helper-local fragment に留める
3. request head + clause attachment shape や predicate parse は still later stage に残す

## なぜこれが最小か

- request-local clause は request head に attached されることが fixture anchor で既に見えている。
- その一方で current helper は request cluster を parse しないので、attachment shape まで扱うのは早い。
- bare clause reject なら hidden attachment を持ち込まず、`PerformVia` spillover と対になる later-stage evidence を増やせる。

## current stage でまだやらないこと

- request head + clause multiline malformed-source parse
- `PerformOn` / `PerformVia` input surface の actual parse
- request-local clause attachment rule の actualization
- predicate fragment parse / normalization

## next narrow step

この judgment を actualize するなら、
`specs/examples/91-current-l2-stage3-request-local-clause-spillover-first-tranche-actualization.md`
として、

- `require` spillover 1 件
- `ensure` spillover 1 件

の helper-local malformed-source smoke を追加するのが最小である。
