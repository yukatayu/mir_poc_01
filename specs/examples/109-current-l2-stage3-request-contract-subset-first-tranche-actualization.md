# 109 — current L2 stage 3 request contract subset first tranche actualization

## 目的

この文書は、`specs/examples/108-current-l2-stage3-request-contract-subset-compare-cut.md` で
fixture-side full request contract subset compare の first cut を
`Stage3RequestContractSubset` 相当の helper-local carrier に置くと整理したことを前提に、
**その helper-local / test-only first tranche が current repo でどこまで actualize 済みか**
を記録する。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- actualize するのは fixture-side `contract.require` / `contract.ensure` subset compare だけである。
- source-side helper output は still `Stage3RequestClauseSuite { require_fragment_text, ensure_fragment_text }` に留める。
- request head kind / op / target / chain_ref parse は still later stage に残す。
- compare carrier は helper-local / test-only に留める。

## actualized scope

current repo で actualize した scope は次の通りである。

1. `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`
   - fixture-side expected carrier
     - `Stage3RequestContractSubset`
       - `require_fragment: Option<Stage3PredicateFragment>`
       - `ensure_fragment: Option<Stage3PredicateFragment>`
   - helper
     - `load_fixture_request_contract_subset(fixture_name, perform_index)`
   - fixture-side contract subset load rule
     - `contract.require` / `contract.ensure` array を読む
     - empty array は `None`
     - single predicate は `Some(Stage3PredicateFragment)`
     - 2 個以上の predicate row は
       - `outside stage 3 first tranche`
       として fail-closed にする
2. `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
   - `Stage3RequestClauseSuite` から
     - `require_fragment_text`
     - `ensure_fragment_text`
     を `Stage3RequestContractSubset` 相当へ parse する local compare path を追加
   - fixture-side compare smoke
     - `e10-perform-on-ensure-failure.json`
     - `e11-perform-via-ensure-then-success.json`

## compare / evidence shape

current tranche の compare shape は次に留める。

- source-side input:
  - inline suite string
- source-side extracted carrier:
  - `Stage3RequestClauseSuite`
- source-side local compare carrier:
  - `Stage3RequestContractSubset`
- fixture-side expected carrier:
  - `Stage3RequestContractSubset`

この compare では、`PerformOn` と `PerformVia` の違いは
fixture-side `perform_index` が指す request node の `contract` 部分だけに還元し、
request head metadata は deliberately compare しない。

## working-set anchor

current first tranche の anchor は次である。

- source-side suite string
  - single-line `require write`
  - single-line `ensure owner_is(session_user)`
- fixture-side `PerformOn` anchor
  - `e10-perform-on-ensure-failure.json`
- fixture-side `PerformVia` anchor
  - `e11-perform-via-ensure-then-success.json`

この tranche で重要なのは、
**同じ source-side suite carrier を `PerformOn` / `PerformVia` の両 fixture に対して contract-only compare できる**
ことを source-backed に示した点である。

## deliberately outside first tranche

current tranche では、次を still later に残す。

- request head kind / op / target / chain_ref parse
- fixture-side full request node compare
- public parser API 化
- typed diagnostics carrier
- clause array に 2 個以上の predicate row を持つ fixture-side compare widening

## なぜこれが最小か

- `Stage3RequestClauseSuite` と fixture-side contract subset compare の接点を explicit にできる。
- request head metadata を持ち込まず、spec107 / spec108 の reopen 条件を守れる。
- `PerformOn` と `PerformVia` の両方で同じ contract-only carrier が使えることを、current repo の focused smoke として示せる。
- multi-predicate clause array や full request node compare へ滑らずに、Phase 3 later branch の structural compare line を 1 段進められる。

## actual code anchor

- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`
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

1. request head metadata を still later に残したまま contract-only compare surface をもう 1 段広げるか
2. request head neutral carrier を保ったまま fixture-side widening guard を別 comparison として切るか

を source-backed に比較するのが自然である。
