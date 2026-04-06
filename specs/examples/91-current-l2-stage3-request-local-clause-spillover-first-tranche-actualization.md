# 91 — current L2 stage 3 request-local clause spillover first tranche actualization

## 目的

この文書は、`specs/examples/90-current-l2-stage3-request-local-clause-spillover-comparison.md` で
stage 3 later branch の最小 malformed-source cut は bare `require` / `ensure` line の pair だと整理したことを前提に、
**その helper-local malformed-source pair が current repo でどこまで actualize 済みか**
を記録する。

## 前提

- current L2 の core semantics、fixture schema、runtime semantics は変更しない。
- stage 3 helper は private / test-only のまま維持する。
- request head spillover と declaration-side admit-slot pair は既に actualize 済みである。
- current tranche は request-local clause spillover を helper-local wording fragment 2 件へ narrow actualize するだけであり、request attachment rule は導入しない。

## actualized scope

current repo で actualize した scope は次の通りである。

1. `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
   - `stage3_admit_slot_parser_spike_rejects_request_local_require_spillover`
   - `stage3_admit_slot_parser_spike_rejects_request_local_ensure_spillover`
2. `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
   - line-level reject
     - `request-local require clause is outside stage 3 admit-slot first tranche`
     - `request-local ensure clause is outside stage 3 admit-slot first tranche`

## compare / evidence shape

current tranche は helper-local malformed-source substring compare に留める。

- source input:
  - declaration-side `admit` attached slot を含む minimal stage 3 accepted prefix
  - bare `require ...`
  - bare `ensure ...`
- compare:
  - parser helper error string に helper-local wording fragment が含まれることだけを見る

## なぜこれが最小か

- bare clause line なら request-local clause の later-stage boundary を示せる。
- request head や clause attachment block を parse しないので、hidden attachment rule を持ち込まない。
- `PerformVia` request head spillover と組み合わせることで、request cluster の
  - head
  - attached clause
  の両側が current helper の outside であることを narrow に示せる。

## current stage でまだやらないこと

- request head + clause multiline malformed-source shape
- request-local `require` / `ensure` attachment rule の actual parse
- request-local clause payload の predicate fragment parse
- fixture-side request contract node compare

## actual code anchor

- `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`

## verification

この tranche の verification は少なくとも次を通す。

```bash
cargo test -p mir-ast --test current_l2_stage3_admit_slot_spike
cargo test -p mir-ast
python3 scripts/validate_docs.py
git diff --check
```

## next narrow step

次段では、

1. request-local clause spillover line をこの wording pair で止めたままにするか
2. request head + clause attachment malformed-source shape を docs-only comparison に進めるか
3. predicate fragment boundary 側の reopen 条件を stage 3 line に接続するか

を source-backed に比較するのが自然である。
