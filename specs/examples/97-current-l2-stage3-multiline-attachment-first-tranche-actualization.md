# 97 — current L2 stage 3 multiline attachment first tranche actualization

## 目的

この文書は、`specs/examples/96-current-l2-stage3-multiline-attachment-shape-comparison.md` で
shared single attachment frame を current next cut に採ったことを前提に、
**その helper-local / test-only first tranche が current repo でどこまで actualize 済みか**
を記録する。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only に留める。
- actualize するのは multiline attachment frame から isolated fragment string を切り出す bridge だけである。
- request head / option declaration の full parser actualization はまだ行わない。
- shared isolated predicate fragment helper は既存の `current_l2_stage3_predicate_fragment_spike_support.rs` をそのまま使う。

## actualized scope

current repo で actualize した scope は次の通りである。

1. `crates/mir-ast/tests/support/current_l2_stage3_multiline_attachment_spike_support.rs`
   - declaration-side `admit:` multiline block extraction
   - request-local `require:` / `ensure:` multiline block extraction
   - extracted fragment を dedented source text として返す helper
   - clause header は head の immediate child attachment line だけを探索する
   - blank line は helper-local で fail-closed に reject する
2. `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`
   - option-local `admit:` multiline extraction smoke
   - request-local `require:` / `ensure:` multiline extraction smoke
   - grouped `and` structure compare
   - missing block / nested header search / blank-line reject の helper-local malformed smoke

## compare / evidence shape

current tranche は次の evidence shape に留める。

- input:
  - inline test strings
- helper output:
  - dedented isolated fragment source text
- compare path:
  - extracted fragment source
  - shared isolated predicate fragment helper
  - fixture-side predicate subset or helper-local expected fragment

つまり current tranche は、
multiline attachment frame 自体の extraction bridge だけを actualize し、
predicate parse 自体は既存 helper に委ねる。

## working-set anchor

current first tranche の anchor は次である。

- option-local multiline `admit:` sample
  - grouped `and` fragment
- request-local multiline `require:` sample
  - grouped `and` fragment
- request-local multiline `ensure:` sample
  - single-line call fragment
- fixture-side compare anchor
  - `e2-try-fallback.json`
  - `e10-perform-on-ensure-failure.json`

## malformed-source first tranche

current tranche で actualize した helper-local malformed smoke は次に留める。

- `missing multiline predicate block after admit:`
- `missing multiline predicate block after require:`
- `missing \`ensure:\` attachment header`
- `blank line is not allowed inside multiline predicate block after admit:`

これは shared single attachment frame の fail-closed floor を示すための最小 tightening であり、
request-local clause suite completion や generic continuation rule はまだ持ち込まない。

## なぜこれが最小か

- declaration-side `admit:` と request-local `require:` / `ensure:` が、同じ attachment frame bridge を共有できる。
- request head / option declaration の full parser actualization を避けられる。
- helper-local bridge と existing predicate fragment helper を薄く接続するだけで actual evidence が取れる。
- nested clause-like line や blank line を hidden に generic continuation として受理しない。
- clause suite owner / ordering / multiplicity と fragment malformed diagnostics を still later stage に残せる。

## current stage でまだやらないこと

- request head full parse
- option declaration body continuation full parse
- request-local clause suite ordering / multiplicity / dedent completion
- `ensure:` missing-block malformed pair
- option-local sibling metadata family や request-local suite completion malformed family
- public parser API 化

## actual code anchor

- `crates/mir-ast/tests/support/current_l2_stage3_multiline_attachment_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`

## verification

この tranche の verification は少なくとも次を通す。

```bash
cargo test -p mir-ast --test current_l2_stage3_multiline_attachment_spike
cargo test -p mir-ast
python3 scripts/validate_docs.py
git diff --check
```

## next narrow step

次段では、

1. request-local clause suite completion を先に docs-only で比較するか
2. multiline attachment helper-local malformed-source pair をもう 1 段増やすか

を source-backed に比較するのが自然である。
