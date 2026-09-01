---
id: theory/00-overview
status: L1-fixed
maturity: draft
depends_on: [adr/ADR-0001, adr/ADR-0002, root/glossary]
summary: formal theoryが扱う意味層S0-S5、S6 Host境界、4 graph族と理論分業の見取り図。
open_items: []
---

# 00 — Overview: strata and the four graphs

## Semantic strata

| Stratum | Content | Main danger |
|---|---|---|
| S0 Surface intent | ordinary source: assignment, read, state decl, chain decl, `when`, `join` | leaking trace/event machinery into ordinary code |
| S1 Core elaboration | explicit owner-directed requests, generated publish/observe, effect/failure rows, capability/witness obligations, source spans | hiding generated obligations |
| S2 Trace / occurrence | history DAG, causal order, cuts, save/load, devtools rows | confusing domain events with occurrences |
| S3 Verification | decidable checker, model-check, proof side, diagnostics | turning Surface into a proof-assistant UI |
| S4 Projection | per-locus artifacts, generated communication boundaries, provider/view seam | treating projection as final ABI/codegen |
| S5 Domain / library | World, Room, Avatar, Portal, game logic | smuggling sample vocabulary into core |

The current semantic axis additionally includes `S6 Host` as the
non-authoritative realization boundary (`arch/01-strata`). These formal theory
chapters need only S0–S5; that scope does not make S6 optional or make legacy
`S7 Application` a current semantic stratum. Applications remain S5 domain
semantics and separate PL-5/PL-6 responsibilities (`arch/06-project-product-layers`).

The project axis in one line of theory: **S0 stays ordinary; S1–S4 make every hidden consequence explicit and diagnosable.** Reads are dependencies; writes are occurrences (ADR-0002).

## The four graph families

1. **Occurrence DAG** (S2): writes, sends/receives, publish/observe, witness create/use, grant/use, admission, activation, cuts. Strict partial order `≺` (theory/04). Never cyclic; no hidden backward edges.
2. **State / existence DAG** (S1): which state cells exist, parent-child existence dependency, lifetime, fallback lineage (theory/06). Changing a parent is an occurrence in graph 1.
3. **Locus / admission graph** (S1): loci, participants, membership epochs, incarnations, role claims vs capability grants (theory/05). A child locus cannot join without its admission path.
4. **Patch / overlay DAG** (S0–S2): source patches, compatibility, activation cuts, dependencies between patches (theory/08). E.g. an avatar patch deployed over a world system on loci {A, C}.

These four are distinct: collapsing any two (e.g. treating a participant as a locus, or a patch as an occurrence stream) is a known drift mode.

## Division of labor among chapters

- 01 fixes the syntax-independent core: configurations, the unified judgment, small-step rules.
- 02 fixes the type/effect/failure vocabulary used by 01.
- 03 fixes Surface→Core elaboration and THM-001 (the project's first target theorem).
- 04 fixes ordering, `atomic_cut`, consistent cuts, SaveObject, load admissibility, Z-cycles.
- 05–08 fix authority, existence/fallback, observation, patching, each as axioms + carriers over 01/04.
- 09 connects discrete verified time with high-rate streams. 10 makes rejection itself a formal object.
- 13 and 14 contribute finite M3/M4 carriers; 15 fixes their one concrete shared finite source for M6--M8.
- 11 is the single ledger of THM/OBL. 12 anchors everything in prior art.
