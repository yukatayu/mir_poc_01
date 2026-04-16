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
| 11 | handoff-before-publication negative | precondition falsifier |
| 12 | handoff-without-witness negative | precondition falsifier |
| 13 | duplicate / stale witness receipt negative | replay falsifier |
| 14 | handoff epoch mismatch negative | replay / epoch falsifier |
| 15 | provider-authority mismatch negative | provider placement falsifier |
| 16 | fairness fails while safety holds | fairness falsifier |
| 17 | snapshot visible but not commit-evidenced | snapshot vs durable-cut falsifier |

## property-to-boundary matrix

| property | core static checker | theorem prover boundary | protocol verifier boundary | runtime policy boundary | primary corpus ids |
|---|---|---|---|---|---|
| same-owner / same-authority structural floor | yes | optional proof reuse | no | no | 1, 11, 15 |
| stage sequence well-formedness | yes | optional | optional | no | 1, 11, 12 |
| witness field presence | structural only | optional | yes | optional | 1, 12 |
| local cut non-interference | no | yes | no | optional evidence | 3, 4 |
| no hidden re-promotion analog | no | yes | optional | no | 1, 6, 7 |
| handoff requires prior publication | no | optional | yes | no | 1, 11 |
| handoff requires witness | no | optional | yes | optional validation | 1, 12, 13 |
| replay / duplicate invalidation | no | no | yes | yes | 7, 13, 14 |
| room-level single-transition seriality | no | optional model lemma | yes | optional operational support | 1, 9 |
| late-join / rejoin safety | no | no | yes | yes | 6, 7, 14 |
| fairness claim | no | optional proof note | yes | yes | 8, 16 |
| distributed provider / receipt validation | no | no | yes | yes | 8, 15 |
| observation / `emit` ordering | no | yes if semantic law | optional | yes | 2, 5, 10, 17 |

## current judgment

- structural floor は checker に寄せる。
- global law / non-interference は theorem side を first choice に置く。
- room protocol / replay / fairness / provider receipt は protocol verifier boundary を first choice に置く。
- activation / rejoin / retry / stale receipt handling は runtime policy も担う。

## what is not decided here

- final property language
- final theorem / protocol tool binding
- final emitted artifact schema
