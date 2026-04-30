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
4. current live compare floor は current clean-near-end model-check family
   `01_peterson_sc_pass / 02_peterson_relaxed_counterexample / 03_broken_mutex_counterexample`
   に取り、historical `e5 / p06 / p07 / p09` compare anchors と `p05` guard-only reading は package-reading memory に留める。`p08` は historical theorem/model-check mixed-helper asymmetry anchor 側に残し、この compression cut の reached floor には戻さない
5. final public model-check contract adoption 群には上げない

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/clean_near_end_samples.py run model-check --format json` | clean near-end model-check family の representative runtime inventory を読み、property/tool probe・checker-artifact route・reopen threshold を carry-over した current public-seam compression floor を doc-level に再確認する |
| `python3 scripts/clean_near_end_samples.py closeout --format json` | canonical runtime inventory と current emitted rows を与える。public-seam residual matrix judgment 自体は helper-local / doc-level に残り、closeout が直接 emitted するわけではない |
| `python3 scripts/current_l2_guided_samples.py smoke-all --format json` | active clean-near-end representative runtime / static corpus floor が compatibility front door から green であることを再確認する |
| `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/clean-near-end/model-check/01_peterson_sc_pass.mir --format json` | clean model-check sample を CLI-shaped current surface からも読めることを再確認する。ただし helper-local public-seam compression manifest の 1:1 public surface ではない |

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
