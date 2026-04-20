# 611. current-l2 delegated-rng-service reserve package summary index actualization

## 目的

`specs/examples/477`、
`571`、
`606`
と
`p09-dice-delegated-rng-provider-placement`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`
を前提に、

- `delegated_rng_service` reserve package の単独 entrypoint
- repo-local emitted summary index
- `p09` reached / `p07` authority-rng baseline contrast / `p08` reconnect contrast
- current recommendation
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**`scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service` を使って、
provider placement practical line を repo-local output dir に narrow materialize する helper-local summary index**
であり、

- final public provider receipt schema
- delegated provider attestation public contract
- final public witness/provider/artifact contract
- `delegated_rng_service + auditable_authority_witness` combined public contract
- `distributed_randomness_provider`
- control-plane separated carrier

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. `delegated_rng_service` practical package 自体は `specs/examples/477` で close 済みである
2. reserve strengthening lane は `specs/examples/571` で
   - `p09` delegated-rng + model-check reached
   - `p07` witness + model-check reached / delegated-rng guard-only
   - `p08` model-check reached / delegated-rng guard-only
   の separate status を保っている
3. closeout 後の reserve reopen line は `specs/examples/606` で helper / snapshot に同期済みである

したがって current open problem は、
`delegated_rng_service` の判断自体を増やすことではなく、
**この package を単独 command と repo-local summary index で再実行可能にすること**
である。

## current actualization cut

current package では次を採る。

1. `python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service`
   を reserve package の単独 entrypoint に置く
2. output dir は
   `target/current-l2-guided/reserve-packages/delegated-rng-service`
   に固定する
3. 同 dir に
   - `package-summary.md`
   - `package-summary.json`
   を置く
4. sample set は
   - `p09` delegated provider placement reached
   - `p07` authority-rng baseline contrast
   - `p08` reconnect contrast
   として narrow に保つ
5. provider placement と authority commit owner は collapse せず、
   summary index には
   - `fairness_source = delegated_rng_service`
   - `fairness_claim = opaque_authority_trust`
   - provider boundary refs
   - optional attachment refs
   をそのまま残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service` | reserve package 単独の repo-local summary index |
| `target/current-l2-guided/reserve-packages/delegated-rng-service/package-summary.md` | human-facing summary index |
| `target/current-l2-guided/reserve-packages/delegated-rng-service/package-summary.json` | machine-readable summary index |
| `p09-dice-delegated-rng-provider-placement.run.json` | delegated provider placement reached |
| `p07-dice-late-join-visible-history.run.json` | authority-rng baseline contrast |
| `p08-dice-stale-reconnect-refresh.run.json` | reconnect contrast |

## current recommendation

1. `delegated_rng_service` reserve package は、
   reserve list の中に置くだけでなく、
   単独 command と summary index まで actualize してよい。
2. current cut は
   `p09 / p07 / p08`
   の 3 本を保ったまま、provider placement practical line を
   helper-local / non-production summary index に留めるのが自然である。
3. `emit-scenario problem2` は Problem 2 全体の runnable scenario loop、
   `emit-reserve delegated-rng-service` は provider placement package 単独の reopen entrypoint
   として読み分ける。

## retained alternatives

- `emit-scenario problem2` だけで reserve package detail を兼ねる
- delegated provider attestation を先に materialize する
- `delegated_rng_service` と `auditable_authority_witness` を 1 本の helper に collapse する

## stop line

current package は次で止める。

- final public provider receipt schema
- delegated provider attestation public contract
- final public witness/provider/artifact contract
- `delegated_rng_service + auditable_authority_witness` combined public contract
- `distributed_randomness_provider`
- control-plane separated carrier

## next self-driven line

current package を close した後の next reopen line は、

1. model-check second-line
2. later mixed gate lane
3. true user-spec hold line

に移るのが自然である。
