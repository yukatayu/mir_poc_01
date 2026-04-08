# 101 — current L2 stage 3 request clause suite first tranche actualization

## 目的

この文書は、`specs/examples/100-current-l2-stage3-request-clause-suite-first-tranche-comparison.md` で
request-local `require:` / `ensure:` の fixed two-slot suite bridge を
current next cut に採ったことを前提に、
**その helper-local / test-only first tranche が current repo でどこまで actualize 済みか**
を記録する。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only に留める。
- actualize するのは `perform` owner の fixed two-slot suite bridge だけである。
- request head full parse や fixture-side full request contract compare はまだ行わない。
- clause payload parse 自体は既存 isolated predicate fragment helper を流用する。

## actualized scope

current repo で actualize した scope は次の通りである。

1. `crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs`
   - `perform` head 配下の request-local clause suite bridge extraction
   - helper output:
     - `require_fragment_text: Option<String>`
     - `ensure_fragment_text: Option<String>`
   - single-line clause payload (`require pred` / `ensure pred`) の direct extraction
   - multiline clause payload (`require:` / `ensure:`) の dedented block extraction
   - helper-local structural fail-closed:
     - `require` after `ensure` reject
     - duplicate `require` reject
     - clause-between blank line reject
2. `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
   - single-line require + ensure slot extraction smoke
   - multiline require + single-line ensure の mixed suite compare
   - ensure-only suite smoke
   - structural fail-closed smoke

## compare / evidence shape

current tranche は次の evidence shape に留める。

- input:
  - inline test strings
- helper output:
  - `Stage3RequestClauseSuite`
    - `require_fragment_text: Option<String>`
    - `ensure_fragment_text: Option<String>`
- compare path:
  - clause slot ごとの extracted fragment text
  - existing isolated predicate fragment helper
  - fixture-side predicate subset compare

つまり current tranche は、
request-local suite owner / ordering / multiplicity / blank-line floor を helper-local で見るが、
predicate parse と fixture-side semantic subset compare は既存 helper を再利用する。

## working-set anchor

current first tranche の anchor は次である。

- single-line require + ensure sample
  - `write`
  - `owner_is(session_user)`
- mixed suite sample
  - multiline `require:` grouped `and`
  - single-line `ensure`
- ensure-only sample
  - `owner_is(session_user)`
- fixture-side compare anchor
  - `e2-try-fallback.json`
  - `e10-perform-on-ensure-failure.json`
  - `e11-perform-via-ensure-then-success.json`

## malformed-source first tranche

current tranche で actualize した helper-local structural fail-closed smoke は次に留める。

- `require clause cannot appear after ensure clause`
- `duplicate \`require\` clause is not allowed`
- `blank line is not allowed between request-local clauses`

これは fixed two-slot suite floor の最小 structural guard を示すための tightening であり、

- duplicate `ensure`
- missing multiline `ensure:` block
- unsupported direct child line family
- public diagnostics carrier

は still later stage に残す。

## なぜこれが最小か

- `summary only` helper より 1 段だけ強い bridge であり、slot ごとの fragment compare まで一続きで通せる。
- existing multiline attachment bridge と isolated predicate fragment helper の間に不要な compare gap を作らない。
- full request head parse や fixture-side full request contract compare を避けたまま、request-local suite の structural floor を actual evidence にできる。
- malformed family の public wideningや typed diagnostics carrier をまだ持ち込まない。

## current stage でまだやらないこと

- full request head parse
- fixture-side full request contract node compare
- public parser API 化
- typed diagnostics carrier
- duplicate `ensure` family や unsupported direct child family の widening

## actual code anchor

- `crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`

## verification

この tranche の verification は少なくとも次を通す。

```bash
cargo test -p mir-ast --test current_l2_stage3_request_clause_suite_spike
cargo test -p mir-ast
python3 scripts/validate_docs.py
git diff --check
```

## next narrow step

次段では、

1. suite bridge malformed/source family をもう 1 段広げるか
2. fixture-side full request contract compare へ近づく structural compare を先に比べるか

を source-backed に比較するのが自然である。
