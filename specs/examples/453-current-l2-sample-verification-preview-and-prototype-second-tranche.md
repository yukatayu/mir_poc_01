# 453 — current L2 sample verification preview and prototype second tranche

## 目的

current authored sample と corrected runnable prototype を実行するときに、
theorem / model-check bridge の current floor を **final public verifier contract と混同せずに**
見える化する helper cut を与える。

## current judgment

1. current operational CLI は、sample / prototype 実行 summary に
   `verification_preview` を helper-local に出してよい。
2. `verification_preview` では少なくとも次を表示してよい。
   - `formal_hook_status`
   - `subject_kind`
   - `subject_ref`
   - proof notebook review unit obligation list
   - model-check concrete carrier obligation list
   - `guard_reason`
3. current preview は final public artifact schema をそのまま出す cut ではなく、
   current runtime/static evidence から読める **preview summary** に留める。
4. current preview は次の 3 状態を区別してよい。
   - runtime try/cut cluster reached
   - fixture static cluster reached
   - guarded not reached
5. current prototype second tranche として、
   - `p04-dice-owner-duplicate-declaration`
   - `p05-dice-owner-guarded-chain`
   を追加してよい。
   これにより prototype bucket で
   - runtime reached
   - static reached
   - guarded not reached
   の 3 状態を比較できる。

## current non-goals

- final public theorem / model-check contract を決めること
- emitted artifact schema を final shell / JSON contract として固定すること
- concrete theorem prover / model-check tool binding をここで決めること
- prototype bucket を authored current-L2 inventory へ hidden promotion すること
