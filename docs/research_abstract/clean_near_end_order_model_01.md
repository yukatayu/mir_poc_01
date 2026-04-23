# clean near-end order / model 01

## 要約

current clean near-end order line は、**high-level order / handoff relation family** を source principal に置き、mutex / weak-memory family は **model-check second line** に分ける構成です。

## 何が built-in か

built-in として扱うのは、`publish`、`handoff`、`witness`、`atomic_cut`、`model`、`property` などの最小構文語です。

## 何が user-defined か

この family では、次が user-defined です。

- authority hierarchy とその element 名
- `dice_owner`
- `draw_pub`
- `DelegatedRngReceipt`
- `AuthorityDrawWitness`
- `PetersonSC`
- `PetersonRelaxedNoPublication`
- `BrokenMutex`

また、低レベル `memory_order_release` や `memory_order_seq_cst` は source principal syntax ではありません。
current repo ではそれらを backend / reference family として位置づけています。

## active sample

order / handoff:

- `01_authorized_roll_publish_handoff`
- `02_missing_witness_rejected`
- `03_handoff_before_publication_rejected`
- `04_stage_block_authorized_handoff`
- `05_delegated_rng_service`
- `06_auditable_authority_witness`

model-check:

- `01_peterson_sc_pass`
- `02_peterson_relaxed_counterexample`
- `03_broken_mutex_counterexample`

## この family で確認できること

- publication と witness が handoff を正当化する
- witness がない handoff は static malformed で止まる
- publication より前の handoff は static malformed で止まる
- `atomic_cut` は local finalization frontier として扱われる
- Peterson under SC は pass する
- relaxed visibility / broken mutex は counterexample になる

## 現在の評価

この family は Problem 2 のうち、**repo-local alpha-ready current layer** までは前進しています。
ただし、`memory_order` family の final public source surface、public artifact contract、production model-check binding まではまだ完了していません。

## 実行コマンド

```bash
python3 scripts/clean_near_end_samples.py run order-handoff --format json
python3 scripts/clean_near_end_samples.py run model-check --format json
```

## detail

full sample code と actual JSON output は `clean_near_end_order_model_01_detail.md` を参照してください。
