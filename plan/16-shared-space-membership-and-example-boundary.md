# plan/16 — shared-space membership と example boundary

## current working boundary

- authoritative-room minimal working subset は current docs-first line に含めてよい
- exhaustive shared-space catalog はまだ final public line に上げない
- membership / authority / provider placement / witness requirement / replay / fairness は分けて扱う

## active example boundary

- active order/handoff examples は `samples/clean-near-end/order-handoff/`
- mutex / weak-memory / broken mutex は `samples/clean-near-end/model-check/`
  に分ける
- delegated RNG practical route は sample-visible にしてよいが、
  provider/public contract finalization を意味しない

## current judgment

- thread/node を同じ causal language で書いてよい
- ただし lowering / transport / failure / durability / policy は分ける
- authoritative room を shared-space 全体の exhaustive default と同一視しない
- `atomic_cut` を room-level durable commit にしない

## still later

- exhaustive shared-space catalog
- final witness/provider public contract
- final replay taxonomy
- distributed fairness theorem
