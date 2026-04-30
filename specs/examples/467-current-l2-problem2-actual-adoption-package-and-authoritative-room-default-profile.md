# 467 — current L2 Problem 2 actual adoption package and authoritative room default profile

## 目的

`faq_007.md`、`specs/examples/460`、`121`、`122`、`123`、`124`、`125` と
`p07-dice-late-join-visible-history`、`p08-dice-stale-reconnect-refresh`
を前提に、

- cut / order / handoff current first line
- authoritative room first actual adoption profile
- thread / node parity wording default
- low-level mapping note
- fairness / replay first default
- retained alternatives
- stop line
- remaining mixed gate

を 1 本に束ねる。

ここで fixed するのは、
**Problem 2 を compare-floor から near-end actual adoption package へ進める current judgment**
であり、

- final source-surface handoff wording
- final emitted-artifact schema
- final public verifier / handoff contract
- exhaustive shared-space catalog
- distributed fairness theorem

は fixed しない。

## applied defaults

current package では、user-authorized default として次を採る。

1. relation decomposition principal
2. low-level `std::memory_order` / `std::kill_dependency` family は retained-later backend/reference family
3. thread / node parity wording は
   - `thread と node は同じ causal language で書く`
   - `違いは lowering / evidence / transport / failure / durability / policy に残す`
4. first actual adoption profile は
   - `authority-ack`
   - `single room authority`
   - `authoritative serial transition`
   - `authority_rng`
   - late join = published history visible as past
   - stale reconnect = fail then refresh
   - replay = stale/incompatible replay invalidated, not silently merged
   - fairness claim = no distributed fairness theorem required in first completion line
5. `auditable_authority_witness` = second strengthening package
6. `delegated_rng_service` = second practical package

## source-backed floor

current repo では、少なくとも次が source-backed にある。

1. cut family decomposition
   - `atomic_cut`
   - `barrier`
   - `snapshot_cut` / `consistent_cut` comparison candidate
   - `durable_cut`
2. relation decomposition
   - `program_order`
   - `dependency_order`
   - `publication_order`
   - `observation_order`
   - `witness_order`
   - `finalization_order`
   - `scoped_happens_before`
3. authority-handoff current first line
   - `authority_serial_transition_family` first
   - `witness_aware_commit_family` second
   - family-first / minimal-row / symbolic-ref ratchet
4. thread / node parity current wording
5. authoritative room baseline and append-friendly contrast room
6. corrected runnable prototypes
   - historical `p07` reading maps to current Sugoroku handoff / late-join evidence
   - historical `p08` reading maps to current stale-reconnect canary plus family check; no single current sample reproduces the old combined story

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json` | publication / handoff core floor が current Sugoroku slice として runnable |
| `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership --format json` | history-visible-as-past late-join floor が current Sugoroku slice として runnable |
| `python3 scripts/network_transport_samples.py run NET-03 --debug reconnect --format json` | stale reconnect / membership-epoch guard canary が current network floor として runnable |
| `python3 scripts/network_transport_samples.py check-all --format json` | network canary family は green であり、historical `p08` fail-then-refresh story は current docs では partial replacement として扱う |
| `python3 scripts/current_l2_guided_samples.py smoke-all --format json` | compatibility front door から active clean-near-end corpus の runner floor を再確認できる。old `current_l2_source_sample_runner` target 自体は `p07/p08` の direct proof surface ではない |
| `cargo run -q -p mir-runtime --bin mir-current-l2 -- check-source-sample samples/clean-near-end/order-handoff/01_authorized_roll_publish_handoff.mir --format json` | clean sample を CLI / JSON surface からも読める。old `current_l2_operational_cli` target 自体は `p07/p08` の direct proof surface ではない |
| `python3 scripts/clean_near_end_samples.py run model-check --format json` | model-check compare floor は active adjacent evidence として残るが、`p07/p08` 自体を model-check sample に変えるわけではない |
| `python3 scripts/current_l2_lean_sample_sync.py` | theorem-side proof-bridge anchor は current sync されているが、`p07/p08` 自体を theorem sample に変えるわけではない |

## actual first adoption profile

current Problem 2 actual package では、
authoritative shared-space turn-based room の first completion line を
次に置く。

| axis | current default |
|---|---|
| activation | `authority-ack` |
| authority placement | `single room authority` |
| consistency mode | `authoritative serial transition` |
| RNG source | `authority_rng` |
| late join | published history is visible as past |
| stale reconnect | fail then refresh |
| replay | stale/incompatible replay is invalidated rather than silently merged |
| fairness claim | no distributed fairness theorem required in first completion line |

append-friendly room は、
**contrast target / retained alternative family**
として残す。

## relation decomposition and low-level mapping note

| principal source family | current role | retained low-level reference |
|---|---|---|
| `program_order` | same-place program order | sequenced-before / intra-thread analog |
| `dependency_order` | dependency-preserving edge | `consume` / `kill_dependency` reference line |
| `publication_order` | publish edge | release-like reference family |
| `observation_order` | observe edge | acquire-like / consume-like reference family |
| `witness_order` | receipt / proof / handoff evidence edge | no exact direct low-level synonym |
| `finalization_order` | local/scoped finalization edge | fence/commit reference material only |
| `scoped_happens_before` | scope-aware admissible order | scope-aware lowering comparison |

current default is:
**high-level relation decomposition principal / low-level family retained-later reference.**

## thread / node parity note

current default wording は次である。

- `thread と node は同じ causal language で書く`
- `違いは lowering / evidence / transport / failure / durability / policy に残す`

したがって current package では、

- `thread == node`
- `distributed operational contract == same-process analog`

の short-hand は採らない。

## `p07` / `p08` integration note

historical `p07` reading は、current first completion line のうち

- `publication_order`
- `observation_order`
- history-visible-as-past late join

を current Sugoroku slices (`03_roll_publish_handoff` / `05_late_join_history_visible`) へ分けて読む。

historical `p08` reading は、current first completion line のうち

- stale/incompatible replay invalidation
- fail-then-refresh reconnect
- no silent merge

を current stale-reconnect canary (`NET-03`) と family check へ分けて読む。old combined story を単独で再現する current active sample は存在しない。

したがって historical `p07` / `p08` reading は、
final replay theorem や final shared-space catalog の evidence ではなく、
**first actual adoption profile の current runnable replacement が誤読なく比較できる**
ことの evidence と読む。

## current recommendation

1. Problem 2 の near-end actual adoption package は、
   - cut family decomposition
   - relation decomposition principal
   - `authority_serial_transition_family` first
   - thread/node parity default wording
   - authoritative room default first profile
   に置く。
2. low-level `std::memory_order` / `std::kill_dependency` family は、
   current source principal ではなく backend/reference family に残す。
3. first completion line では distributed fairness theorem を要求しない。
4. replay は stale/incompatible replay invalidation を default に置き、silent merge を current first line に採らない。
5. append-friendly room は contrast target として残す。

## retained alternatives

- `witness_aware_commit_family` stronger promotion
- `event_tree_execution_view` primary-control-family promotion
- `snapshot_cut` / `consistent_cut` settled vocabulary adoption
- low-level memory-order wording import
- `auditable_authority_witness` second strengthening package
- `delegated_rng_service` second practical package
- distributed fairness theorem / provider-attestation-first line
- broader exhaustive shared-space catalog

## stop line

current package は次で止める。

- final source-surface handoff wording
- final emitted-artifact schema
- final public verifier / handoff contract
- exhaustive final shared-space catalog
- distributed fairness theorem
- concrete provider receipt / attestation public schema
- final replay invalidation protocol taxonomy

## remaining mixed gates

- final source-surface handoff wording gate
- final emitted-artifact / handoff-contract gate
- low-level public-stance gate
- `auditable_authority_witness` strengthening gate
- `delegated_rng_service` practical package gate
- exhaustive catalog / stronger fairness profile gate

## next self-driven line

next reserve/self-driven line は、次の 2 本で読むのが自然である。

1. `auditable_authority_witness` second strengthening package
2. `delegated_rng_service` second practical package
