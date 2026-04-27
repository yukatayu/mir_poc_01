# plan/16 — shared-space membership と example boundary

## current working boundary

- authoritative-room minimal working subset は current docs-first line に含めてよい
- exhaustive shared-space catalog はまだ final public line に上げない
- membership / authority / provider placement / witness requirement / replay / fairness は分けて扱う
- `Place` は participant と同一ではなく、queue / state / capability / visibility / observation frontier を持つ execution locus として読む
- current Sugoroku sample の `world` は host/server-side sugar として読み、Mir core primitive だと既成事実化しない

## active example boundary

- active order / handoff examples は `samples/clean-near-end/order-handoff/`
- mutex / weak-memory / broken mutex は `samples/clean-near-end/model-check/`
  に分ける
- Sugoroku world runtime attachment example は `samples/clean-near-end/sugoroku-world/`
  に置き、single OS process logical multi-place emulator として扱う
- Sugoroku 側の participant set は fixed literal principal だけでなく `MembershipRegistry` から読む line を current reading に置く
- next representative sample 候補は avatar fairy follow slice であり、まだ active canonical sample ではない
- phase 8 planned skeleton family は `samples/not_implemented/avatar-fairy-follow/` に置き、historical prototype anchor は `samples/prototype/current-l2-dynamic-attach-detach/` に残す

## current judgment

- thread / node を同じ causal language で書いてよい
- ただし lowering / transport / failure / durability / policy は分ける
- authoritative room を shared-space 全体の exhaustive default と同一視しない
- authentication を transport に潰さない
- helper-local current cut では `auth none` baseline、`membership_epoch` / `member_incarnation` freshness、`witness_refs` separate lane を visible にしてよい
- visualization / telemetry を untyped debug leak にしない
- `atomic_cut` を room-level durable commit にしない
- system-wide source から place-specific program へ projection できる性質を future line の invariant として保つ

## still later

- exhaustive shared-space catalog
- final witness / provider public contract
- final replay taxonomy
- distributed fairness theorem
- avatar fairy follow helper promotion
- real network transport / multi-server consensus / durable distributed commit
- projection generator / placement optimizer
- detach lifecycle implementation
- `AttachPoint` compatibility / migration actualization
