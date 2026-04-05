# 79 — current L2 stage 1 parser spike input surface and helper naming

## 目的

この文書は、`specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md` で固定した

- actual stage 1 parser spike は `crates/mir-ast/tests/support/` 配置の private helper として始める
- compare surface は lowered fixture-subset compare に留める

という current judgment を前提に、
**actual implementation へ入る直前に、input surface、`decl_guard_slot` の internal carrier、private helper family の naming をどこまで narrow に決めるべきか**
を比較する。

ここで固定するのは final parser grammar や final parser API ではない。
固定するのは、non-production / test-driven な stage 1 parser spike を
余計な lexical freeze や hidden elaboration なしで actualize するための最小 cut だけである。

## 前提

- current L2 の core semantics は変更しない。
- stage 1 parser spike は chain / declaration structural floor に留める。
- `decl_guard_slot` は parser-side opaque slot carrier であり、predicate fragment parse は later stage に残す。
- bridge は option-level structural transfer であり、lowered fixture-subset compare に留める。
- public parser API、final parser crate boundary、final text fixture policy は OPEN のままである。

## current source anchor

### current fixture-side anchor

current stage 1 smoke family の representative fixture は次である。

- `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- `crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json`

この 2 本は、stage 1 で見たい

- option declaration header
- `lease` compatibility anchor
- chain head / edge / lineage assertion

を current fixture subset だけで source-backed に持っている。

### current parser-side anchor

`crates/mir-ast/src/lib.rs` はまだ placeholder skeleton であり、
parser module / public parser API は存在しない。
したがって current next step では、actual parser spike を
`crates/mir-ast/tests/support/` 配置の private helper として始める方が自然である。

### previous docs-only judgment

spec 74 / 75 / 76 / 78 により、current stage 1 parser spike で固定済みなのは次である。

1. declaration-side guard slot は opaque attached slot に留める
2. parser-side field 名の第一候補は `decl_guard_slot`
3. bridge は option-level structural transfer
4. compare surface は lowered fixture-subset compare

未決なのは、actual implementation へ入るときの

- input surface
- `decl_guard_slot` internal carrier
- private helper / file 名

である。

## 比較する input surface

### 案 1. current JSON fixture から parse input を逆生成する

既存の `e4` / `e7` JSON fixture から text-like input を reverse generate する。

#### 利点

- current fixture corpus と expected subset を 1 source に寄せやすい。

#### 欠点

- parser spike の入力が fixture schema に従属しやすい。
- parser 実装ではなく reverse serializer を先に作る形になり、責務がねじれる。
- final parser grammar 未決の current phase では artificial すぎる。

### 案 2. test inline string を input surface にする

`e4` / `e7` 相当の tiny text program を integration test 側で inline string として持ち、
parser spike はその string を入力に受ける。
compare target は current JSON fixture から取り出した lowered fixture subset に留める。

#### 利点

- final text fixture policy を先に固定しない。
- actual parser spike の入力が「text である」ことだけを最小に確認できる。
- compare target は current fixture corpus に anchored したままにできる。
- OS / path policy / file discovery の未決事項を stage 1 へ持ち込まない。

#### 欠点

- inline text が test file に埋まるため、長くは育てにくい。
- later に dedicated text fixture file へ移すときの move は必要になる。

### 案 3. dedicated text fixture file を新設する

`crates/mir-ast/tests/fixtures/current-l2-stage1/` などに dedicated text fixture file を置く。

#### 利点

- parser input を file-based に揃えやすい。
- later parser workstream への continuity は見えやすい。

#### 欠点

- final parser syntax 未決の current phase では file naming / path policy / discovery rule を早く固定しやすい。
- current taskは placement と compare surface の narrow actualization であり、file policy は一段先でよい。

## 比較する `decl_guard_slot` internal carrier

### 案 A. bare `String`

parser-side option declaration が `decl_guard_slot: String` のように raw owned text を直接持つ。

#### 利点

- 実装は最も短い。
- `OptionDecl.lease` への lowering も単純である。

#### 欠点

- parser-side opaque slot と bare text が同一視されやすい。
- later stage で richer carrier に進めるとき、type distinction が弱い。

### 案 B. borrowed span / slice

parser-side carrier が source text への borrowed slice や span を持つ。

#### 利点

- later lexer / span-aware parser には接続しやすい。
- source location の保持を早く始められる。

#### 欠点

- current private spike に対して lifetime と ownership が重い。
- lowered fixture-subset compare しか見ない stage 1 には過剰である。
- `tests/support` の narrow helper としては implementation noise が大きい。

### 案 C. dedicated wrapper + owned surface text

parser-side option declaration が dedicated type
`Stage1DeclGuardSlot` のような wrapper を持ち、その内部に owned `surface_text` を置く。

#### 利点

- opaque slot であることを type で保てる。
- bare `String` より drift しにくく、borrowed span より軽い。
- later stage で span / token row / predicate fragment を足すとしても widening point が明確である。

#### 欠点

- bare `String` よりは type を 1 個増やす必要がある。

## 比較する private helper family naming

### 案 α. `stage1_parser_spike_support`

短い generic naming で support helper を切る。

#### 利点

- 短い。
- stage 1 の用途には読める。

#### 欠点

- current repo の `current_l2_*` family と揃わない。
- later に別 stage や別 parser spike が増えると generic すぎる。

### 案 β. `current_l2_stage1_parser_spike_support`

repo-scoped / phase-scoped naming にする。

#### 利点

- current helper family の naming と揃う。
- non-production / current L2 / stage 1 の範囲が name で読める。
- later stage 2 / stage 3 spike とも混線しにくい。

#### 欠点

- 少し長い。

### 案 γ. `current_l2_decl_chain_parser_support`

structural floor の subject を直接 name に入れる。

#### 利点

- stage 1 の subject を強く示せる。

#### 欠点

- `decl_guard_slot`、chain、lineage をひとまとめに semantics-loaded に読ませやすい。
- later に stage 1 scope が少し動くと name が窮屈になる。

## 比較

### actual implementation へ入る最小さ

- input surface は inline string が最も narrow である。
- internal carrier は dedicated wrapper + owned surface text が最も軽く、それでいて opaque distinction を保てる。
- helper naming は `current_l2_stage1_parser_spike_support` が current repo family と最も整合する。

### hidden elaboration を避けられるか

- JSON fixture reverse generation は parser input と fixture carrier を混ぜやすい。
- dedicated text fixture file は file policy を早く固定しやすい。
- inline string + fixture subset compare なら、input と expected subset の責務を分けたまま actual spike を始めやすい。

### later stage への widening point を残せるか

- bare `String` は widening point が暗い。
- borrowed span / slice は今の stage 1 には重い。
- dedicated wrapper + owned surface text なら、later に span や token row を足す widening point が明確である。

### helper boundary と整合するか

- `current_l2_stage1_parser_spike_support` は current repo の non-production helper naming と揃う。
- parser spike を `tests/support` に置く current judgment とも矛盾しない。

## current judgment

current repo の next narrow step としては、次が最も自然である。

1. input surface は **test inline string**
2. compare target は **existing `e4` / `e7` JSON fixture から取り出した lowered fixture subset**
3. `decl_guard_slot` internal carrier は **dedicated wrapper + owned `surface_text`**
4. private helper family / file 名の第一候補は **`current_l2_stage1_parser_spike_support`**
5. actual helper surface は
   - `parse_stage1_program_text(...)`
   - `lower_stage1_option_decl_to_fixture_option(...)`
   の 2 段に割るのが最小である

ここでいう 2 段 split は、

- parse
- lowering bridge

を分けるだけであり、public API 化を意味しない。
`e4` / `e7` の lowered fixture-subset compare を束ねる loop や compare harness は、
current phase では test-local assembly に留め、program-level bridge として既成事実化しない。

## current stage でまだ決めないこと

- dedicated text fixture file の directory layout
- final parser-side type 名
- span / source location を stage 1 から持つかどうか
- actual parser crate / module の長期配置
- final parser API
- final parser grammar / punctuation / indentation policy

## current meaning

- actual stage 1 parser spike は `tests/support` 配置の private helper として始める
- input は inline text に留め、expected compare target は current fixture subset に anchored する
- `decl_guard_slot` は bare text ではなく dedicated wrapper で持ちつつ、内部は owned `surface_text` に留める
- これにより stage 1 actualization を始めても、final grammar、final file policy、span-aware parser を early freeze しにくい
