---
id: theory/12-literature
status: L1-fixed
maturity: draft
depends_on: [theory/00-overview, adr/ADR-0003]
summary: 先行研究対比表。何を借り、どこが違うか。新規性の主張の根拠。
open_items: [OPEN-025]
---

# 12 — Literature anchoring

Rule: every core mechanism must name its nearest prior art and its delta.
"Nobody did this" claims require a row here.

## Placement / generated communication

| System | Solves | Mir takes | Mir differs |
|---|---|---|---|
| ML5 (Murphy et al.) | modal types for located computation | mode/locus in the judgment | Mir adds authority/failure rows to every generated edge |
| Links / Eliom / Hop / Ur/Web | tierless web, generated client-server comm | source-first authority (.mir) | Mir targets N loci, membership, hot-plug; comm is visible, not hidden |
| ScalaLoci | placement types, multitier reactives | placement as types | Mir: capability-gated writes, single-authority model |
| Choral / HasChor / Pirouette | choreographies, endpoint projection (verified) | projection viewpoint (S4) | Mir projects from checked state semantics, not choreography scripts |
| [Scribble / MPST endpoint API generation](https://www.doc.ic.ac.uk/~rhu/scribble/index.html) | multiparty protocols, endpoint projection, protocol-specific typed APIs | generated endpoint shape as a comparison point | Mir starts from checked state semantics rather than a multiparty protocol script; every generated edge keeps authority/failure/witness obligations visible |

## Authority / information flow

| E lang / Spritely Goblins / CapTP | object capabilities, distributed ocap | claim ≠ authority; grant lineage | Mir adds membership epochs/incarnations + witness carriers |
| Fabric / Jif | IFC labels + distributed transactions | label/redaction monotonicity | Mir rejects transactions (ADR-0003); authority is serial owner |

## Shared worlds / concurrency stance

| Croquet / TeaTime | replicated deterministic world compute | frontier-style coherence intuition | Mir: authority+witness, not replicated determinism |
| Verse (Epic) | transactional metaverse language | ordinary-assignment ergonomics goal | Mir: explicit requests + failure rows instead of speculation/rollback |
| CRDTs | convergence under concurrency | (comparison only) | rejected for core: weak intent/authority semantics (OPEN-003) |

## Federated shared-space contrast

| System | Solves | Mir takes | Mir differs |
|---|---|---|---|
| [Matrix](https://spec.matrix.org/latest/) / [Third Room](https://thirdroom.io/docs/guides/) | federated room event DAGs and a Matrix-based shared 3D client | concrete shared-space and federation contrast | Mir does not use a federated room-event DAG as its core source semantics; its source-level authority/witness obligations remain a separate comparison axis |

## Evolution / dynamic update

| Erlang/OTP hot swap; DSU (Kitsune, Ginseng); K42 | live update mechanics & safe-point theory | activation cut ≈ safe point; compat checks | Mir binds activation to membership/witness frontier; patch DAG; no eval |

| [Urbit / Arvo](https://docs.urbit.org/urbit-os/kernel/arvo) | deterministic event-log state, snapshots, and interpreter source-code updates | replay/snapshot and dynamic-update contrast | Mir keeps the no-eval update boundary and binds patch activation to membership/witness frontier plus a patch DAG |

## Event-driven verification / dataflow

| P language (Microsoft) | event-driven programs + model checking | Line-2 model-check stance | Mir hides events from S0 (ADR-0002) |
| Timely/differential dataflow | frontiers for progress tracking | observation/snapshot frontier shape | Mir frontiers carry authority/membership, not just progress |
| [Distributed REScala](https://www.rescala-lang.com/assets/pdf/2014%20Distributed%20REScala.pdf) | distributed reactive propagation with glitch freedom | distributed reactivity as a comparison baseline | Mir keeps occurrence and generated request/publication machinery out of S0 (ADR-0002); generated cross-locus obligations retain authority/failure/witness rows |

## Ordering / cuts / memory (foundations kept from LAB plan/18)

Lamport 1978 (happens-before); Chandy–Lamport 1985 (snapshots ⇒ consistent
cuts); Netzer–Xu (zigzag paths ⇒ Z-cycle rule); Boehm–Adve 2008, Herlihy–Wing
1990, Jeffrey–Riely 2019 (memory/correctness, Line-2 profiles); Davies–
Pfenning 2001, Guarded λ, MDTT/MTT, Fitch-style multimodal (candidate spine
for the modal core, LAB D-054/D-067).

## Distributed language classics

Emerald (object migration ≈ handoff), Argus (guardians+atomic actions —
rejected transaction path, kept failure explicitness), Obliq (lexical
distributed scope — contrast for our non-ambient locus blocks).

## Novelty claims (each must survive this table)

1. Unified single semantics for: placement elaboration + capability/witness
   authority + monotone fallback/lease + consistent-cut save/load (Z-cycle) +
   three-line verification + patch-DAG hot-plug + typed observation, with the
   norm that **generated obligations are visible at every stratum**.
2. Monotone fallback/lease lineage theory with a static evidence floor
   (edge-local lineage annotations; underdeclared ⇒ static error).
3. Witness/publication relation family as the source-level replacement for
   memory-order vocabulary.
4. Frontier-bound patch activation (admission frontier = activation frontier).

OPEN-025: keep scanning: session types for generated protocols; distributed
reactive programming (REScala); Matrix/Third Room; Urbit (eval-centric —
contrast); add rows as found.
