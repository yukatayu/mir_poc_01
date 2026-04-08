# 94 — current L2 stage 3 predicate fragment first tranche actualization

## 目的

この文書は、`specs/examples/93-current-l2-stage3-predicate-fragment-boundary-comparison.md` で
stage 3 later branch の next cut は shared isolated predicate fragment helper だと整理したことを前提に、
**その helper-local first tranche が current repo でどこまで actualize 済みか**
を記録する。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- helper は private / test-only に留める。
- program parser の accepted cluster は広げない。
- request head + clause attachment multiline shape は still later stage に残す。

## actualized scope

current repo で actualize した scope は次の通りである。

1. `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`
   - isolated minimal predicate fragment parser
   - fixture-side `OptionDecl.admit` / request-local clause subset loader
2. `crates/mir-ast/tests/current_l2_stage3_predicate_fragment_spike.rs`
   - declaration-side `admit` compare
   - request-local `require` / `ensure` compare
   - explicit `and` / grouping compare

## compare / evidence shape

current tranche は次の evidence shape に留める。

- input:
  - isolated fragment string
- compare target:
  - fixture-side predicate node subset
- accepted fragment:
  - bare atom
  - application-like form
  - explicit `and`
  - 括弧 grouping

request head / clause attachment / multiline suite / fixture-side full request node compare は、
この tranche の対象外である。

## working-set anchor

current first tranche の fixture / sample anchor は次である。

- `e3-option-admit-chain.json`
  - declaration-side `admit`
- `e10-perform-on-ensure-failure.json`
  - request-local `require`
  - request-local `ensure`
- `e11-perform-via-ensure-then-success.json`
  - request-local `ensure`
- `e2-try-fallback.json`
  - explicit `and`
  - 括弧 grouping を含む minimal fragment compare

## なぜこれが最小か

- declaration-side `admit` と request-local clause の shared floor を 1 helper で持てる。
- request head attachment rule をまだ持ち込まない。
- fixture-side predicate node anchor を使って actual evidence を取れる。
- first checker cut の `minimal predicate fragment well-formedness` と parser boundary line を、helper-local compare だけで接続できる。

## current stage でまだやらないこと

- request head + clause attachment multiline shape
- declaration-side `admit:` / request-local `require:` / `ensure:` の block extraction
- fixture-side full request contract compare
- direct lowering や canonical surface reconstruction
- malformed-source pair の actualization

## actual code anchor

- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_predicate_fragment_spike.rs`

## verification

この tranche の verification は少なくとも次を通す。

```bash
cargo test -p mir-ast --test current_l2_stage3_predicate_fragment_spike
cargo test -p mir-ast
python3 scripts/validate_docs.py
git diff --check
```

## next narrow step

次段では、

1. predicate fragment helper の malformed-source pair を先に actualize するか
2. request head + clause attachment multiline shape の docs-only comparison を先に開くか

を source-backed に比較するのが自然である。
