# 517 — current L2 model-check public seam compression after threshold and probe

## 目的

`specs/examples/480`、
`501`、
`507`
と
`e5-underdeclared-lineage`、
`p05-dice-owner-guarded-chain`、
`p06-typed-proof-owner-handoff`、
`p07-dice-late-join-visible-history`、
`p08-dice-stale-reconnect-refresh`、
`p09-dice-delegated-rng-provider-placement`
を前提に、

- model-check public seam compression
- property/tool probe carry-over
- checker-artifact route carry-over
- final public-contract reopen threshold carry-over
- actual runnable evidence
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**model-check final public-contract reopen threshold と property/tool seam probe を保ったまま、remaining model-check public seams を helper-local compression manifest に actualize する current cut**
であり、

- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker/runtime-policy contract
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. model-check property/tool seam probe
   - property-language probe
   - tool-seam probe
   - checker-boundary probe
2. model-check checker-artifact route actual adoption
   - row-local property route first
   - checker boundary contract anchor
   - consumer-shaped checker artifact candidate only
3. model-check final public-contract reopen threshold
   - property language and tool brand first
   - public checker artifact and migration second
   - verifier handoff and runtime-policy contract third
   - final public verifier contract fourth

したがって current open problem は、
model-check mixed gate を loose list のまま残すことではなく、
**property-language/tool-brand pair、public checker artifact/migration pair、verifier-handoff/runtime-policy pair、final public verifier contract residual を 1 本の current recommendation に圧縮できるか**
である。

## current compression cut

current package では、次を採る。

1. model-check public seam residual は
   - property language and tool brand first
   - public checker artifact and migration second
   - verifier handoff and runtime-policy contract third
   - final public verifier contract fourth
   に圧縮する
2. property/tool seam probe は carry-over し、
   property-language probe / tool-seam probe / checker-boundary probe を mixed gate anchor に残す
3. checker-artifact route / verifier-handoff route / repo-local emitted artifact refs は threshold carry-over に置く
4. representative reached sample は `e5 / p06 / p07 / p08 / p09`、guard-only は `p05` に取る
5. final public model-check contract adoption 群には上げない

## actual runnable evidence

| evidence | current reading |
|---|---|
| `build_current_l2_source_sample_model_check_public_seam_compression` | threshold + property/tool probe を束ね、remaining model-check public seam residual を helper-local に actualize する runtime support |
| `current_l2_model_check_public_seam_compression` | `e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only の model-check public-seam compression を machine-check する focused runtime test |
| `model_check_public_seam_residual:*` refs | current model-check mixed gate を subject-local residual matrix として固定する repo-local ref family |

## current recommendation

1. model-check line は public seam residual を compression manifest に actualize してよい。
2. property/tool seam probe は final property language / tool brand を fixed せずに keep してよい。
3. final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contract は still later に残す。

## retained alternatives

- property language / tool brand pair を分離して reopen
- public checker artifact / migration pair を分離して reopen
- verifier handoff / runtime-policy pair を分離して reopen
- unified public model-check contract first adoption

## stop line

current package は次で止める。

- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- actual emitted verifier handoff artifact
- production checker/runtime-policy contract
- final public verifier contract
