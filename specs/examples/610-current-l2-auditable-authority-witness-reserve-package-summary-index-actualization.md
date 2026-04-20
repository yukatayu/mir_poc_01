# 610. current-l2 auditable-authority-witness reserve package summary index actualization

## 目的

`specs/examples/476`、
`571`、
`606`
と
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p05-dice-owner-guarded-chain`
を前提に、

- `auditable_authority_witness` reserve package の単独 entrypoint
- repo-local emitted summary index
- `p07` reached / `p08` guard-only contrast / `p05` pre-default comparison
- current recommendation
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**`scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness` を使って、
minimal witness core strengthening を repo-local output dir に narrow materialize する helper-local summary index**
であり、

- final public witness schema
- final public provider receipt schema
- final public witness/provider/artifact contract
- `delegated_rng_service` practical package
- distributed fairness theorem

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. `auditable_authority_witness` strengthening package 自体は `specs/examples/476` で close 済みである
2. reserve strengthening lane は `specs/examples/571` で
   - `p07` witness + model-check reached
   - `p08` model-check reached / witness guard-only
   - `p05` guard-only
   の separate status を保っている
3. closeout 後の reserve reopen line は `specs/examples/606` で helper / snapshot に同期済みである

したがって current open problem は、
`auditable_authority_witness` の判断自体を増やすことではなく、
**この package を単独 command と repo-local summary index で再実行可能にすること**
である。

## current actualization cut

current package では次を採る。

1. `python3 scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness`
   を reserve package の単独 entrypoint に置く
2. output dir は
   `target/current-l2-guided/reserve-packages/auditable-authority-witness`
   に固定する
3. 同 dir に
   - `package-summary.md`
   - `package-summary.json`
   を置く
4. sample set は
   - `p07` reached
   - `p08` guard-only non-witness-bearing contrast
   - `p05` guard-only pre-default comparison
   として narrow に保つ
5. room profile claim と witness payload は collapse せず、
   summary index には
   - `fairness_claim = auditable_authority_witness`
   - `witness_kind`
   - `action_ref`
   - `draw_slot`
   - `draw_result`
   - witness binding refs
   をそのまま残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness` | reserve package 単独の repo-local summary index |
| `target/current-l2-guided/reserve-packages/auditable-authority-witness/package-summary.md` | human-facing summary index |
| `target/current-l2-guided/reserve-packages/auditable-authority-witness/package-summary.json` | machine-readable summary index |
| `p07-dice-late-join-visible-history.run.json` | witness-strengthening reached |
| `p08-dice-stale-reconnect-refresh.run.json` | non-witness-bearing guard-only contrast |
| `p05-dice-owner-guarded-chain.run.json` | pre-default guard-only comparison |

## current recommendation

1. `auditable_authority_witness` reserve package は、
   reserve list の中に置くだけでなく、
   単独 command と summary index まで actualize してよい。
2. current cut は
   `p07 / p08 / p05`
   の 3 本を保ったまま、minimal witness core strengthening を
   helper-local / non-production summary index に留めるのが自然である。
3. `emit-scenario problem2` は Problem 2 全体の runnable scenario loop、
   `emit-reserve auditable-authority-witness` は witness strengthening package 単独の reopen entrypoint
   として読み分ける。

## retained alternatives

- `emit-scenario problem2` だけで reserve package detail を兼ねる
- witness/provider combined public contract を先に materialize する
- `delegated_rng_service` と `auditable_authority_witness` を 1 本の helper に collapse する

## stop line

current package は次で止める。

- final public witness schema
- final public provider receipt schema
- final public witness/provider/artifact contract
- `delegated_rng_service` practical package
- distributed fairness theorem

## next self-driven line

current package を close した後の next reopen line は、

1. `delegated_rng_service`
2. model-check second-line
3. later mixed gate lane / true user-spec hold line

に移るのが自然である。
