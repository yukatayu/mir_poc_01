# 78 — current L2 stage 1 parser spike placement and compare surface

## 目的

この文書は、`specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md` で固定した

- stage 1 smoke family は `e4` + `e7` の two-fixture pair
- `e3` は later-stage contrast reference

という current judgmentを前提に、
**actual stage 1 parser spike をどこに置き、何を compare surface として持つのが最小か**
を比較する。

ここで固定するのは final parser module ではない。
固定するのは、non-production / private / test-driven な spike を始める最小配置だけである。

## 前提

- current L2 の core semantics は変更しない。
- final parser grammar、final parser API、final parser crate 境界は OPEN のままである。
- stage 1 parser spike は chain / declaration structural floor に留める。
- parser-side carrier は `decl_guard_slot` を第一候補にする。
- bridge は option-level structural transfer であり、parsed fact を stage 3 から先取りしない。
- public `lib.rs` / `harness.rs` API は増やさない。

## current source anchor

### helper placement anchor

current non-production helper の actual narrow cut は、
`examples/support/` や `scripts/` に private helper を置いて public API を増やさない形で進んでいる。

### test anchor

current machine-check の mainline は

- fixture corpus
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

を中心に回っており、
actual behavior の固定は integration test に寄せる current repo culture がある。

### handoff anchor

spec 75 / 76 / 77 により、stage 1 parser spike で見たいのは次だけである。

- option declaration header
- `decl_guard_slot`
- chain head / edge / lineage assertion
- `decl_guard_slot -> OptionDecl.lease` の structural transfer

runtime expectation や full trace / audit compare は current step の主題ではない。

## 比較する配置案

### 案 1. `scripts/` 配置

Python などの script から stage 1 parser spike を回し、fixture subset compare を行う。

#### 利点

- 立ち上げは速い。
- OS 非依存の text processing だけで進めやすい。

#### 欠点

- Rust 側 actual parser spike と ownership が分離しやすい。
- fixture-side lowering compare を later Rust 実装へ移すときに二重化しやすい。
- current repo の AST / fixture carrier とやや遠い。

### 案 2. `crates/mir-semantics/examples/support/` 配置

non-production detached helper と同様に、`examples/support/` に parser spike helper を置く。

#### 利点

- current helper placement パターンには揃う。
- example binary を薄い wrapper にしやすい。

#### 欠点

- parser spike の主題は AST / fixture lowering であり、semantics example helper 側へ置くと責務が少し遠い。
- runtime emitter / exporter helper と stage 1 parser spike が同じ support family に並び、関心が混ざりやすい。

### 案 3. `crates/mir-ast/tests/support/` 配置

test-only / private helper として、`crates/mir-ast/tests/support/` に stage 1 parser spike を置く。

#### 利点

- fixture corpus と最も近い。
- compare surface を integration test の fixture subset compare に留めやすい。
- public parser API を既成事実化しない。
- later に actual parser crate / module を作るときも spike を捨てやすい。

#### 欠点

- detached helper の `examples/support/` pattern とは別 family になる。
- example binary から直接呼ぶ convenience は弱い。

## 比較する compare surface

### 案 A. parser-side raw AST snapshot compare

stage 1 parser spike が返す temporary AST をそのまま compare する。

#### 利点

- parser-side temporary carrier を細かく観察できる。

#### 欠点

- temporary carrier 名や field が drift source になりやすい。
- fixture-side compatibility anchor を使う spec 75 / 76 の line から外れやすい。

### 案 B. lowered fixture-subset compare

stage 1 parser spike の output を thin lowering bridge で fixture-side subset へ落とし、
`OptionDecl` / `ChainDecl` の relevant field だけを compare する。

#### 利点

- spec 75 / 76 の handoff cut と最も整合する。
- `decl_guard_slot -> OptionDecl.lease` の bridge を directly 見られる。
- runtime / trace / audit expectation を巻き込まない。

#### 欠点

- parser-side temporary AST 自体の差分は compare surface から外れる。

### 案 C. full fixture compare

program fixture 全体、場合によっては `expected_static` / `expected_runtime` まで compare する。

#### 利点

- 1 本の compare で多くを見られる。

#### 欠点

- stage 1 scope を超える。
- runtime / request / admissibility cluster を誤って巻き込みやすい。
- parser spike の failure mode と semantic mismatch が混ざる。

## 比較

### private / non-production に留められるか

- `scripts/` も private には留めやすいが、Rust 側 spike と ownership が離れる。
- `examples/support/` も private にはできるが、parser spike の責務が semantics helper に寄りすぎる。
- `tests/support/` は current step では最も safe に private へ留めやすい。

### fixture corpus との近さ

- `tests/support/` が最も近い。
- `examples/support/` は detached helper には近いが fixture compare の主戦場ではない。
- `scripts/` はさらに遠い。

### compare surface の narrowness

- raw AST snapshot compare は temporary carrier 依存が強い。
- lowered fixture-subset compare は stage 1 handoff judgment に一致する。
- full fixture compare は広すぎる。

## current judgment

current repo の next narrow step としては、次が最も自然である。

1. actual stage 1 parser spike は **`crates/mir-ast/tests/support/` 配置**の private helper として始める
2. compare surface は **lowered fixture-subset compare** に留める
3. したがって initial actualization は、
   `stage1 parse -> stage1 temporary carrier -> thin lowering bridge -> fixture subset compare`
   という test-driven line にする

## current stage でまだ決めないこと

- actual parser crate / module の長期配置
- public parser API
- parser-side temporary carrier の最終 struct 名
- text fixture を別ファイル化するか test inline string にするか
- example binary / script convenience を後から足すか

## current meaning

- stage 1 parser spike は test-only / private helper として始める
- compare は lowered fixture subset に留める
- これにより actual spike を進めても、current docs-only handoff cut と stage split を壊しにくい
