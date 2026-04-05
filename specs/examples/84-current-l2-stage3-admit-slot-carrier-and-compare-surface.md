# 84 — current L2 stage 3 admit-slot carrier and compare surface

## 目的

この文書は、`specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md` で

- stage 3 request / admissibility cluster の最初の sub-cut は declaration-side `admit` attached slot にする
- `PerformVia` / request-local clause は still later stage に残す

という current judgment を固定したことを前提に、
**その first tranche を actual parser spike として切るなら、parser-side carrier の naming と compare surface をどこまで narrow に決めるのが自然か**
を比較する。

ここで固定するのは final parser grammar ではない。
固定するのは、

- `admit` attached slot を parser-side でどう持つか
- current parser-free fixture corpus とどう narrow に突き合わせるか
- request / predicate parse を先食いしない first tranche の compare contract は何か

という actualization 直前の cut だけである。

## 前提

- current L2 の core semantics は変更しない。
- stage 1 parser spike の current actualization は維持する。
- stage 3 first tranche では declaration-side `admit` を **opaque attached slot** に留める。
- `PerformVia` / request-local `require` / `ensure` / predicate fragment parse は still later stage に残す。
- current parser-free AST fixture schema は維持する。
- public parser API、final parser grammar、typed parser diagnostics は導入しない。

## current source anchor

### current fixture-side anchor

`crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json` の `OptionDecl` は、

- `lease`
- `admit`

を同時に持つ。

ただし `admit` は current fixture schema では predicate node として表現される。
たとえば `owner_writer` の `admit` は

- `kind = "call"`
- `callee = "owner_is"`
- `args = ["session_user"]`

である。

これは current parser-free fixture schema が already elaborated predicate carrier を持つことを意味する。

### current parser-side anchor

stage 1 parser spike は、`decl_guard_slot.surface_text` を parser-side opaque slot として保持しつつ、
thin lowering bridge で fixture-side structural subset compare へ接続している。

current actual code anchor:

- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`

この line が current phase で source-backed に示しているのは、

1. parser-side opaque slot carrier は fixture-side compatibility anchor と直結させない
2. lowered fixture-subset compare は structural subset に留める
3. slot 自体の raw surface 保持は parser-side smoke で別に見てよい

という cut である。

### current contrast anchor

`e3-option-admit-chain` は、

- option declaration 側の `admit`
- chain declaration
- `PerformVia`
- request-local `require`

を同居させている。

したがって stage 3 first tranche では、
`admit` attached slot と request cluster の残りを誤って同時に parse しない compare surface が必要である。

## 比較する parser-side carrier naming

### 案 1. `admit`

parser-side option declaration に `admit` field をそのまま置く。

#### 利点

- 短い。
- fixture-side field 名と直感的には揃う。

#### 欠点

- parser-side opaque slot と fixture-side predicate node を同一視しやすい。
- current stage では predicate parse をまだしないのに、field 名が fixture-side semantics を先取りしやすい。

### 案 2. `admit_slot`

generic な attached slot として `admit_slot` を使う。

#### 利点

- opaque slot であることは読める。
- fixture-side `admit` よりは区別しやすい。

#### 欠点

- declaration-side attached slot なのか request-side clause なのかが name だけでは弱い。
- current repo では request / admissibility cluster の残りも later stage に残しているため、generic すぎる。

### 案 3. `decl_admit_slot`

parser-side option declaration に `decl_admit_slot` を置く。

#### 利点

- declaration-side attached slot であることが明確である。
- fixture-side `admit` predicate node と区別しやすい。
- stage 1 の `decl_guard_slot` と対称な naming になる。

#### 欠点

- 少し長い。
- final parser-side schema にそのまま残る保証はない。

## 比較する compare surface

### 案 A. parser-side `decl_admit_slot` を fixture-side `OptionDecl.admit` へ直接 lower する

parser-side `decl_admit_slot.surface_text` を current fixture-side predicate node へ直接変換し、
fixture subset compare に含める。

#### 利点

- `admit` field まで fixture-side compare に入れられる。
- 1 本の lowered subset compare で見える情報が増える。

#### 欠点

- predicate parse / normalization を hidden に先食いしやすい。
- current first tranche が opaque attached slot だけを見るという docs-only cut を壊しやすい。
- current fixture-side `admit` node は already elaborated なので、stage 3 first tranche の compare surface として広すぎる。

### 案 B. fixture-side `admit` node を helper-local canonical surface へ再構成して compare する

fixture-side `admit` node から helper-local に canonical surface string を再構成し、
parser-side `decl_admit_slot.surface_text` と比較する。

#### 利点

- predicate node をそのまま parser-side へ持ち込まずに済む。
- compare は `admit` まで含められる。

#### 欠点

- helper-local canonicalization rule を先に導入する必要がある。
- current stage では compare のためだけに predicate pretty-print / normalization を持ち込むことになりやすい。
- later stage の predicate fragment parse と競合しやすい。

### 案 C. lowered fixture-subset compare は structural subset に留め、`decl_admit_slot` は parser-side retention smoke で別に見る

fixture-side compare は

- option declaration header
- `lease` compatibility anchor
- chain head / edge / lineage assertion

までに留める。

そのうえで parser-side では、

- `decl_admit_slot` が option declaration に attached していること
- `surface_text` を raw に保持していること

を dedicated smoke で別に確認する。

#### 利点

- opaque attached slot という current cut を最も保ちやすい。
- fixture-side `admit` node を hidden に parse / normalize しない。
- `e3` を full-program compare にせず、option / chain subset reference に留められる。
- stage 1 の `decl_guard_slot.surface_text` retention smoke と対称になる。

#### 欠点

- compare surface が 2 本立てになる。
- fixture-side `admit` node との direct equality はまだ見ない。

## 比較

### stage 3 first tranche の narrowness

- 案 A は `admit` predicate node まで current first tranche に持ち込みやすい。
- 案 B は direct lowering よりは narrow だが、helper-local canonicalization が増える。
- 案 C は structural compare と slot retention を分けるため、最も narrow である。

### stage 1 line との対称性

- `decl_guard_slot` と同様に parser-side opaque slot を保つなら、carrier 名は `decl_admit_slot` が最も自然である。
- compare surface も、fixture-side structural compare と parser-side slot retention smoke を分ける案が stage 1 と整合する。

### hidden predicate parsing / normalization の回避

- 案 A は最も危険である。
- 案 B も canonical surface reconstruction が hidden normalization point になりうる。
- 案 C は `admit` 内部を later stage に残せる。

### `e3` anchor の使い方

- 案 A / B は `e3` の predicate node を先に compare surface へ持ち込みやすい。
- 案 C は `e3` を option / chain subset reference と declaration-side `admit` retention anchor の両方に narrow に使える。

## current judgment

current repo の next narrow step としては、次が最も自然である。

1. parser-side carrier 名の第一候補は **`decl_admit_slot`**
2. stage 3 first tranche では `decl_admit_slot` を **opaque attached slot** に留める
3. compare surface は **lowered fixture-subset compare + parser-side slot retention smoke** の 2 本立てにする
4. current first tranche では parser-side `decl_admit_slot` を fixture-side `OptionDecl.admit` へ direct lower しない
5. fixture-side `admit` node の canonical surface reconstruction も、current first tranche では導入しない

## first tranche で見てよいもの

### lowered fixture-subset compare

- option name
- target
- capability
- `lease` compatibility anchor
- chain head
- fallback edge
- edge-local lineage assertion

### parser-side retention smoke

- option declaration に `decl_admit_slot` が attached していること
- `decl_admit_slot.surface_text` が raw に保持されること

## current stage でまだやらないこと

- `decl_admit_slot` の internal predicate parse
- `PerformVia`
- request-local `require` / `ensure`
- fixture-side `OptionDecl.admit` への direct lowering
- helper-local predicate canonicalization
- public parser API

## next narrow step

この comparison を actualize するなら、次が最小である。

1. `crates/mir-ast/tests/support/` 配置の private helper を追加する
2. `e3` 相当の option / chain subset inline text を parser input に使う
3. lowered fixture-subset compare は structural subset に留める
4. `decl_admit_slot.surface_text` retention smoke を別 test で確認する
