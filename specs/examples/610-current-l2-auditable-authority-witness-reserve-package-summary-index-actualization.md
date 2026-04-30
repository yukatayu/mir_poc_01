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

ここで保持するのは、
**`scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness` を使って
minimal witness core strengthening を repo-local output dir に narrow materialize していた historical helper-local summary index memory**
であり、current active compatibility front door は clean-near-end `list / smoke-all / closeout` に置く。

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
3. closeout 後の reserve reopen line は `specs/examples/606` で historical helper / snapshot memory に同期済みである

したがって current open problem は、
`auditable_authority_witness` の判断自体を増やすことではなく、
**この package の historical reserve-summary memory を current compatibility front door と混同せずに保持すること**
である。

## current actualization cut

current package では次を採る。

1. `python3 scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness`
   は 2026-04-22 clean-sample migration 前の historical reserve package entrypoint として扱い、
   current active compatibility front door には戻さない
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く
3. historical output dir は
   `target/current-l2-guided/reserve-packages/auditable-authority-witness`
   に固定する
4. 同 dir には
   - `package-summary.md`
   - `package-summary.json`
   が materialize されていた
5. sample set は
   - `p07` reached
   - `p08` guard-only non-witness-bearing contrast
   - `p05` guard-only pre-default comparison
   として narrow に保つ
6. room profile claim と witness payload は collapse せず、
   historical summary index には
   - `fairness_claim = auditable_authority_witness`
   - `witness_kind`
   - `action_ref`
   - `draw_slot`
   - `draw_result`
   - witness binding refs
   をそのまま残す

## current evidence

| evidence | current reading |
|---|---|
| `python3 scripts/current_l2_guided_samples.py list` | current active compatibility front door が clean-near-end accepted sample setのみを列挙することを示す |
| `python3 scripts/current_l2_guided_samples.py smoke-all --format json` | current active compatibility front door が active clean-near-end suite を実行することを示す |
| `python3 scripts/current_l2_guided_samples.py closeout --format json` | current active compatibility front door が canonical current inventory を返すことを示す |
| `specs/examples/606` | reserve integration entrypoint summary は historical helper memory として保持する |
| `target/current-l2-guided/reserve-packages/auditable-authority-witness/package-summary.md` | historical human-facing summary index path |
| `target/current-l2-guided/reserve-packages/auditable-authority-witness/package-summary.json` | historical machine-readable summary index path |

## current recommendation

1. `auditable_authority_witness` reserve package は、
   reserve list の中に置くだけでなく、
   historical helper-local summary index memory として保持してよい。ただし current active command へ戻さない。
2. current cut は
   `p07 / p08 / p05`
   の 3 本を保ったまま、minimal witness core strengthening を
   helper-local / non-production summary index に留めるのが自然である。
3. historical `emit-scenario problem2` は Problem 2 全体の runnable scenario loop memory、
   historical `emit-reserve auditable-authority-witness` は witness strengthening package 単独の retired reopen memory
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

## historical closeout queue memory

historical package close 後の next reopen line memory では、

1. `delegated_rng_service`
2. model-check second-line
3. later mixed gate lane / true user-spec hold line

に移るのが自然だった。current queue authority は `progress.md` / `tasks.md` に残す。
