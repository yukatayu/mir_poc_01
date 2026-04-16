# 411 — current L2 order / handoff adequacy corpus and verification-boundary matrix

## 目的

order / handoff line を prose-only comparison にせず、

- adequacy corpus
- property-to-boundary matrix

を docs-only で揃える。

## adequacy corpus

| id | benchmark family | current role |
|---|---|---|
| 1 | authoritative game room: A rolls, publication, handoff to B, B sees witnessed past | primary motivating scenario |
| 2 | same scenario の shared-memory / same-process analog | parity / lowering comparison |
| 3 | rollback before local cut | local finalization contrast |
| 4 | rollback after local cut | local finalization contrast |
| 5 | snapshot-only cut vs durable-cut difference | observation vs commit contrast |
| 6 | late join / rejoin after handoff | membership / replay contrast |
| 7 | leave / reconnect with stale message | stale carrier / incarnation contrast |
| 8 | delegated RNG provider with authority commit | provider placement / witness contrast |
| 9 | append-friendly room where authority-serial transition is overkill | overfit seriality contrast |
| 10 | observation / `emit`-bearing variant | later conceptual variant |

## property-to-boundary matrix

| property | core static checker | theorem prover boundary | protocol verifier boundary | runtime policy boundary |
|---|---|---|---|---|
| same-owner / same-authority structural floor | yes | optional proof reuse | no | no |
| stage sequence well-formedness | yes | optional | optional | no |
| witness field presence | structural only | optional | yes | optional |
| local cut non-interference | no | yes | no | optional evidence |
| no hidden re-promotion analog | no | yes | optional | no |
| handoff requires prior publication | no | optional | yes | no |
| handoff requires witness | no | optional | yes | optional validation |
| replay / duplicate invalidation | no | no | yes | yes |
| room-level single-transition seriality | no | optional model lemma | yes | optional operational support |
| late-join / rejoin safety | no | no | yes | yes |
| fairness claim | no | optional proof note | yes | yes |
| distributed provider / receipt validation | no | no | yes | yes |
| observation / `emit` ordering | no | yes if semantic law | optional | yes |

## current judgment

- structural floor は checker に寄せる。
- global law / non-interference は theorem side を first choice に置く。
- room protocol / replay / fairness / provider receipt は protocol verifier boundary を first choice に置く。
- activation / rejoin / retry / stale receipt handling は runtime policy も担う。

## what is not decided here

- final property language
- final theorem / protocol tool binding
- final emitted artifact schema
