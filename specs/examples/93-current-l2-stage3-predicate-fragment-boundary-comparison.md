# 93 — current L2 stage 3 predicate fragment boundary comparison

## 目的

この文書は、`specs/examples/92-current-l2-stage3-predicate-fragment-reopen-sequencing.md` で
stage 3 later branch の次段は request head + clause attachment multiline shape ではなく
predicate fragment boundary の reopen 条件を先に扱うと整理したことを前提に、
**current stage で minimal predicate fragment をどの carrier / compare surface から reopen するのが最小か**
を narrow に比較する。

ここで固定するのは final parser grammar でも final checker API でもない。
固定するのは、declaration-side `admit` と request-local `require` / `ensure` が共有する
**helper-local first tranche の cut** だけである。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only のまま維持する。
- fixture-side `OptionDecl.admit` handoff は、predicate fragment boundary が見えるまで docs-only deferred にしていた。
- request-local `require` / `ensure` は bare spillover reject までは actualize 済みだが、request head attachment rule はまだ導入しない。
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` では、minimal predicate fragment well-formedness を first checker cut 候補 cluster に入れてよいと整理済みである。
- `specs/examples/01-current-l2-surface-syntax-candidates.md` では、minimal predicate fragment として
  - bare atom
  - application-like form
  - explicit `and`
  - 括弧 grouping
  を current companion candidate に置いている。

## current source anchor

- `specs/examples/89-current-l2-stage3-admit-node-handoff-comparison.md`
  - fixture-side `OptionDecl.admit` handoff は predicate fragment boundary が見えるまで deferred
- `specs/examples/90-current-l2-stage3-request-local-clause-spillover-comparison.md`
  - request-local clause spillover は bare line reject に留め、attachment rule をまだ持ち込まない
- `specs/examples/92-current-l2-stage3-predicate-fragment-reopen-sequencing.md`
  - shared floor として predicate fragment を先に reopen する
- `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
  - declaration-side `admit` の fixture-side predicate node anchor
- `crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json`
  - request-local `require` / `ensure` の fixture-side predicate node anchor
- `crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json`
  - `PerformVia` request-local `ensure` の fixture-side predicate node anchor
- `crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.json`
  - explicit `and` と grouping を含む current fixture-side predicate node anchor

current issue は、predicate fragment を reopen するとしても、
次のどこから actualize するのが最小かである。

1. slot / clause payload は opaque surface のまま retention だけ増やす
2. declaration-side `admit` だけを parsed fragment compare に上げる
3. declaration-side `admit` と request-local `require` / `ensure` の両方が共有できる isolated fragment helper を切る

## 比較する 3 案

### 案 1. opaque surface retention だけを増やす

`decl_admit_slot.surface_text` のような opaque carrier を declaration-side / request-local の両方に広げ、
fragment parse 自体はまだ導入しない。

#### 利点

- hidden grammar を最も増やさない。
- request head attachment rule や fixture-side lowering を引き続き完全に deferred にできる。

#### 欠点

- shared floor が surface string retention に留まり、first checker cut の
  `minimal predicate fragment well-formedness` と直接つながらない。
- declaration-side / request-local の両 branch に共通する structural compare が増えない。
- fixture-side predicate node anchor を使った evidence がまだ取れない。

### 案 2. declaration-side `admit` だけを parsed fragment compare に上げる

`decl_admit_slot.surface_text` から minimal predicate fragment を parse し、
fixture-side `OptionDecl.admit` node と compare する。
request-local `require` / `ensure` は still later stage に残す。

#### 利点

- `specs/examples/89` の handoff deferred line を最短で reopen できる。
- declaration-side branch 単体では narrow に見える。

#### 欠点

- request-local branch と shared floor を持たない。
- `admit` だけ先に parsed fragment compare を持つため、stage 3 line が declaration-side 側へ偏る。
- 後から request-local branch を reopen すると、別 helper か second compare surface を足す pressure が増える。

### 案 3. shared isolated predicate fragment helper を切る

program parser を太らせず、

- declaration-side `admit` slot payload
- request-local `require` / `ensure` payload

を **isolated fragment string** として別 entry から parse し、
fixture-side predicate node subset と helper-local compare する。
request head + clause attachment multiline shape は still later stage に残す。

#### 利点

- declaration-side / request-local の shared floor を最初から 1 つにできる。
- request head attachment rule をまだ導入しない。
- fixture-side `OptionDecl.admit` handoff reopen と request-local clause reopen を同じ fragment helper に接続できる。
- `specs/examples/30` の first checker cut inventory と最も整合する。

#### 欠点

- opaque surface retention だけの案よりは helper が 1 段増える。
- fragment parser の compare contractを helper-local に明記しないと hidden canonicalization に見えやすい。

## 比較

### shared floor の強さ

- 案 1 は shared floor が surface retention だけで、node-level / structural floor を持たない。
- 案 2 は declaration-side にしか shared floor を与えない。
- 案 3 は declaration-side `admit` と request-local `require` / `ensure` が同じ fragment floor を使える。

### hidden attachment rule の回避

- 案 1 と案 3 は request head attachment rule をまだ外に残せる。
- 案 2 も attachment rule は持ち込まないが、shared floor が declaration-side に偏る。
- この観点では案 1 と案 3 が安全だが、案 3 の方が first checker cut とつながる。

### fixture-side anchor との接続

- 案 1 は fixture-side predicate node anchor を compare evidence に使えない。
- 案 2 は `OptionDecl.admit` にだけつながる。
- 案 3 は `e3` の `admit`、`e10` / `e11` の request-local clause、`e2` の `and` / grouping まで shared floor に乗せられる。

### checker-led staged line との整合

- `specs/examples/30` が切っているのは request head attachment ではなく minimal predicate fragment well-formedness である。
- したがって stage 3 next step として自然なのは、request-local block suite ではなく fragment floor である。
- この観点でも案 3 が最も素直である。

## current judgment

current repo の next narrow step としては、
**案 3. shared isolated predicate fragment helper を切る**
のが最も自然である。

つまり、

1. stage 3 later branch の first predicate-fragment tranche は、program parser の accepted cluster を広げる task としてではなく、isolated fragment helper を切る task として扱う
2. helper は declaration-side `admit` と request-local `require` / `ensure` の両方に使える shared floor にする
3. compare は fixture-side predicate node subset に留め、request head + clause attachment multiline shape と fixture-side full request node compare は still later stage に残す

## なぜこれが最小か

- `OptionDecl.admit` handoff reopen と request-local clause reopen を同じ floor に接続できる。
- request head / clause attachment / multiline suite を hidden に持ち込まない。
- first checker cut の `minimal predicate fragment well-formedness` と parser boundary line を、最小の helper-local actual evidence で結べる。
- declaration-side だけ、あるいは request-local だけを先に太らせる片寄りを避けられる。

## first helper tranche の想定 scope

shared isolated helper を current phase で actualize するなら、scope は次に留める。

1. input は isolated fragment string だけ
2. accepted fragment は
   - bare atom
   - application-like form
   - explicit `and`
   - 括弧 grouping
   まで
3. compare target は fixture-side predicate node subset だけ
4. declaration-side slot extraction と request head attachment / multiline suite parsing は含めない
5. pretty-print / canonical surface reconstruction は導入しない

## current stage でまだやらないこと

- request head + clause attachment multiline shape
- request-local clause suite parser
- fixture-side request contract node 全体 compare
- fixture-side `OptionDecl.admit` への direct lowering
- predicate canonicalization / pretty-print

## next narrow step

この judgment を actualize するなら、次は
`specs/examples/94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md`
として、

- helper-local isolated fragment parser
- `e2` / `e3` / `e10` / `e11` を使った fixture-side predicate subset compare

までを first tranche として記録するのが自然である。
