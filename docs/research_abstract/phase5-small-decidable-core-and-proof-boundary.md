# Phase 5: small decidable core と proof boundary

## この phase の意味

Phase 5 は、型システム / theorem / model-check のうち、**first line で decidable に止める部分** と **second line へ handoff する部分** を分ける層です。

## 2026-04-23 時点で固まっていること

- first strong typing layer は finite decidable index fragment
- authority / label / capture / region / cost は finite theory として current sample に接続している
- invalid sample は static malformed として止まる
- mutex / weak-memory family は model-check second line に分けている
- Lean foundations は small mechanization fragment として動いている

## この phase の current boundary

次は first line に入ります。

- `label_flow`
- `authority_ge`
- `capture_subset`
- `region_outlives`
- `cost_leq`
- `effect_row_subset`
- `requires_witness`
- `requires_publication`

一方で、次は second line へ送る repository-memory reading です。

- full theorem discharge
- broken mutex / weak-memory counterexample search
- concrete theorem prover / model-check production binding

## まだ残ること

- full dependent type theory
- final public proof obligation / theorem result contract
- concrete public model-check binding

## 関連する summary / detail

- `clean_near_end_typing_01.md`
- `clean_near_end_order_model_01.md`
- `clean_near_end_lean_01.md`
