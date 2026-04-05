# 80 — current L2 stage 1 parser spike first tranche actualization

## 目的

この文書は、`specs/examples/78-current-l2-stage1-parser-spike-placement-and-compare-surface.md` と
`specs/examples/79-current-l2-stage1-parser-spike-input-surface-and-helper-naming.md`
で固定した current docs-only judgment を前提に、
**actual stage 1 parser spike の first tranche が current repo でどこまで actualize 済みか**
を source-backed に記録する。

ここで固定するのは final parser grammar や public parser API ではない。
固定するのは、`e4` / `e7` の two-fixture pair を使った
**test-only / private / non-production の structural smoke**
がどこまで実コードとして成立したかである。

## 前提

- current L2 の core semantics は変更しない。
- final parser grammar、final parser module、public parser API は OPEN のままである。
- stage 1 parser spike は declaration / chain structural floor に留める。
- declaration-side guard slot は opaque attached slot として扱い、predicate fragment parse は later stage に残す。
- compare surface は lowered fixture-subset compare に留める。
- `lib.rs` / `harness.rs` の public API は増やさない。

## current code anchor

### parser spike helper

- `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`

この helper は test-only の private support module であり、
current stage では public parser module を既成事実化しない。

### smoke integration test

- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`

この test は inline text input を用い、
existing JSON fixture から取り出した lowered fixture subset と compare する。

### fixture anchor

- `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- `crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json`

### local dependency anchor

- `crates/mir-ast/Cargo.toml`

current first tranche では fixture JSON の subset load にだけ `serde_json` を local `dev-dependencies` として足す。

## actualized scope

### accepted text surface

current first tranche が受ける input surface は、test inline string のうち次だけである。

- `option NAME on TARGET capability CAP lease SLOT`
- `chain NAME = HEAD`
- `fallback SUCCESSOR @ lineage(PRED -> SUCC)`

ここでの `SLOT` は parser-side では opaque attached slot であり、
current helper は `surface_text` として保持する。

### parser-side temporary carrier

current helper は次の temporary carrier family を持つ。

- `Stage1DeclGuardSlot`
- `Stage1ParsedOptionDecl`
- `Stage1ParsedChainDecl`
- `Stage1ParsedChainEdge`
- `Stage1ParsedLineageAssertion`
- `Stage1ParsedProgram`

これは current phase の internal helper carrier であり、
fixture schema や future public parser IR と同一視しない。

### thin lowering bridge

current first tranche は、parser-side carrier を
fixture-side subset compare 用の carrier へ落とす thin bridge を持つ。

- `lower_stage1_option_decl_to_fixture_option(...)`
- `lower_stage1_chain_decl_to_fixture_chain(...)`

ここでの bridge は slot-only helper ではなく、
option-level / chain-level structural transfer である。

### compare contract

current compare contract は test-local assembly に留める。
program-level bridge helper は current tranche では切らない。

compare するのは次だけである。

- `OptionDecl`
  - `name`
  - `target`
  - `capability`
  - `lease`
- `ChainDecl`
  - `name`
  - `head`
  - `edges[].predecessor`
  - `edges[].successor`
  - `edges[].lineage_assertion`

## actualized behavior

### `e4` smoke

`e4` 相当の inline text を parse し、lowered fixture subset compare で
`e4-malformed-lineage.json` の `OptionDecl` / `ChainDecl` subset と一致することを確認する。

この compare は parser-side raw AST snapshot ではなく、
fixture-side anchor に揃えた lowered subset compare である。

### `e7` smoke

`e7` 相当の inline text を parse し、lowered fixture subset compare で
`e7-write-fallback-after-expiry.json` の `OptionDecl` / `ChainDecl` subset と一致することを確認する。

### guard slot surface retention

`writer` option の declaration-side guard slot について、
`decl_guard_slot.surface_text == "expired"` を current carrier 上で確認する。

これは predicate fragment parse を行っていない current stage でも、
opaque slot として text retention が成立していることの smoke である。

## current meaning

current repo の first tranche actualization としては、次が成立している。

1. actual stage 1 parser spike は `crates/mir-ast/tests/support/` 配置の private helper として始まった
2. input surface は inline test strings に留めた
3. parser-side guard slot は dedicated wrapper + owned `surface_text` に留めた
4. compare surface は lowered fixture-subset compare に留めた
5. `e4` / `e7` の two-fixture pair と guard-slot retention smoke が cargo test で通る

## current stage でまだやらないこと

- final parser grammar の固定
- public parser API
- `mir-ast` crate 本体への parser module actualization
- dedicated text fixture file の導入
- span / token row / source location の保持
- predicate fragment parse
- option-local `admit` parse
- request / admissibility cluster parse
- program-level bridge helper の blessed API 化
- detached artifact export との直接統合

## next narrow step

current first tranche の次に比較すべき narrow step は、少なくとも次のいずれかである。

1. current helper surface を壊さずに、`e3` を later-stage contrast reference として stage 2 に送る
2. stage 1 parser spike の malformed-source smoke を parser helper 自身へどこまで持たせるかを比べる
3. current private helper を actual parser crate / module へ昇格させる前提条件をどこまで narrow に切るかを比べる
