# plan/70 - LAB-to-canon reconciliation ledger

## Purpose

This document is non-normative LAB repository memory.

It records how high-risk legacy LAB claims should be read after
`mirrorea_canon/` became the sole canon source. It does not change canon, does
not close G0, does not discharge any proof obligation, and does not promote a
new implementation package.

Use this file when a legacy LAB document, sample, report, or helper claim needs
to be cited without accidentally treating it as canon.

## Source hierarchy

- Canon source: `mirrorea_canon/`
- LAB evidence / repository memory: root docs, legacy `specs/`, `plan/`,
  samples, scripts, and reports outside `mirrorea_canon/`
- Current snapshots: `progress.md` and `tasks.md`
- Runnable sample dashboard: `samples_progress.md`
- Work evidence: `docs/reports/`

If a LAB claim conflicts with canon, canon wins. Cite LAB sources as
`LAB:path` unless the same claim is mirrored by a canon ADR, theory file,
specification, conformance scenario, gate, or phase file.

## Disposition vocabulary

| Disposition | Meaning |
|---|---|
| `canonized` | Canon already contains the authoritative version. LAB can be cited only as evidence/history. |
| `partial-canon` | Canon fixes the direction, but detailed LAB evidence still needs per-claim drilldown. |
| `LAB-evidence-only` | Useful implementation/sample/report evidence, not canon status. |
| `superseded` | Old LAB wording was replaced by canon and should not be repeated as current truth. |
| `rejected-as-canon` | Claim pattern must not be promoted into canon in its old form. |
| `OPEN` | Needs a future canon proposal, OPEN entry, ADR, or gate decision. |

## Ledger columns

The rows below are claim-family rows. They are not a complete line-by-line
audit. A later drilldown may split any row into individual `LAB:` references.

| Column | Meaning |
|---|---|
| LAB source family | Main old locations or evidence families. |
| Claim summary | What the old line was trying to preserve. |
| Kind | vocabulary / semantics / implementation status / sample evidence / process. |
| Canon disposition | One of the dispositions above. |
| Canon anchor | ADR / CON / SCN / Gate / Phase / theory / architecture anchor. |
| Safety note | What must not be inferred. |
| Follow-up | Next safe action. |
| Decision level | Canon level if known; otherwise `not canon`. |

## High-risk claim-family ledger

| LAB source family | Claim summary | Kind | Canon disposition | Canon anchor | Safety note | Follow-up | Decision level |
|---|---|---|---|---|---|---|---|
| Root docs, `Documentation.md`, `progress.md`, `tasks.md`, legacy `specs/`, `plan/69`, `plan/91` | Source hierarchy and project governance. | process | `canonized` | `mirrorea_canon/README.md`, `MAP.md`, `meta/source-hierarchy.md`, `ADR-0012`, `plan/02-operating-model.md`, `architecture/02-boundary-contracts.md` | Legacy `specs/` and `plan/` are not normative after canon adoption. | Keep stale wording audits small and mechanical. | L0 |
| `README.md`, `Documentation.md`, `specs/01`, `specs/02`, `plan/69`, old reports | Project axis and non-axis vocabulary. | vocabulary / semantics | `canonized` | `NORTH-STAR.md`, `GLOSSARY.md`, `ADR-0001`, `ADR-0002`, `theory/00-overview.md` | `World`, `Room`, `Avatar`, `Game`, and `Reversed Library` are domain/library vocabulary, not Mir core primitives. `Event` is not the primary surface model. | Use canon vocabulary in new summaries; keep old terms only as LAB evidence names. | L0/L1 |
| `specs/03`, `plan/69`, layer maps in status docs | Surface/Core/Trace/Verification/Projection/Domain separation. | semantics | `canonized` | `architecture/01-strata.md`, `theory/00-overview.md`, `architecture/02-boundary-contracts.md` | Upper-layer names must not be smuggled into lower semantic primitives. | Use canon strata names when recutting old implementation rows. | L0/L1 |
| `specs/39`, `plan/64`, `progress.md`, Surface P-SURF rows | Ordinary Surface assignment should elaborate to explicit owner-directed Core consequences. | semantics / implementation evidence | `OPEN` | `theory/03-elaboration.md`, `theory/11-metatheory-ledger.md`, `spec/02-surface-grammar.md`, `spec/03-static-semantics.md`, `spec/04-core-ir.md`, `plan/00-gates.md` G1 | LAB parser/elaboration evidence is not a discharged theorem and not canon implementation-state completion. | `plan/71-g1-ordinary-assignment-target.md` drafts the LAB-only G1 simple-assignment target boundary and proof-obligation split; `plan/118-g0-g1-ordinary-assignment-claim-family-drilldown.md` now splits this claim family into line-level `LAB:` citation rows for future G0/G1 review. Disposition remains `OPEN`; this does not claim G1 exit, SCN conformance, theorem discharge, implementation completion, or canon movement. | L2/L3 until canon gate decision |
| `specs/40`, `specs/41`, `plan/65`, `plan/66`, indexed-state and role-admission samples | Indexed state, participant/keyspace rows, admission requests, capability grants, witnesses, and stale rejection. | semantics / sample evidence | `partial-canon` | `GLOSSARY.md` CON-001..010, `theory/05-authority.md`, `spec/03-static-semantics.md`, `spec/04-core-ir.md`, `spec/06-conformance.md` SCN-03/04, `plan/00-gates.md` G3 | Key, role name, locus name, provider, package, or transport is not authority. | Drill down after G1 into G3 authority/admission rows. | L1/L2 |
| `specs/20`, `specs/22`, `specs/39`, `plan/69`, helper traces | Reads, writes, occurrence rows, dependency rows, and graph families. | semantics | `partial-canon` | `ADR-0002`, `theory/00-overview.md`, `theory/01-mircore-v0.md`, `theory/02-types-effects-failures.md`, `theory/04-ordering-and-cuts.md`, `theory/07-observation.md` | Do not expose all occurrence machinery as ordinary source vocabulary; a domain event is not the semantic unit. | Use only as narrow G1 ordinary-assignment support unless a later gate explicitly opens G4 observation work; `plan/119` says this is not an immediate default drilldown and must not widen into runtime graph or event-model semantics. | L1/L2 |
| `specs/13`, `specs/20`, `plan/39`, `plan/41`, clean-near-end and alpha fallback rows | Lifetime, lease, freshness, fallback, load/rollback, and stale non-resurrection. | semantics / sample evidence | `partial-canon` | `GLOSSARY.md` CON-020..024, `theory/06-existence-fallback.md`, `plan/00-gates.md` G2, `spec/README.md` OPEN-005 for `chain` | Fallback is not authority strengthening; old sample closure is not a final lifetime theory. | Keep as G2 input after ordinary assignment. | L1/L2 |
| `specs/15`, `specs/20`, `specs/37`, `specs/40`, `plan/41`, save/load and `atomic_cut` rows | Cut/save-load, activation cut, durable state, and no stale resurrection. | semantics / sample evidence | `partial-canon` | `GLOSSARY.md` CON-025..029, `theory/04-ordering-and-cuts.md`, `spec/06-conformance.md` SCN-05, `plan/00-gates.md` G5 | Do not infer R3/R4 distributed durable save/load or final memory-order family from alpha evidence. | Reopen under G5 only after G1/G2/G3 are stable enough. | L1/L2 |
| `specs/21`, `specs/24`, `specs/27`, `specs/41`, `plan/45`, `plan/66`, role/auth reports | Authentication, authorization, membership, capability, witness, freshness, and admission. | semantics / process | `canonized` for principle, `OPEN` for details | `ADR-0005`, `GLOSSARY.md` CON-002..009, `theory/05-authority.md`, `architecture/02-boundary-contracts.md` BND-005, `plan/00-gates.md` G3 | Never collapse these into transport or role labels. | Use old rows as examples only after capability/witness vocabulary is canon-aligned. | L0/L1 |
| `specs/22`, `specs/43`, `plan/47`, `plan/68`, devtools/static diagnostics samples | Observation, debug, telemetry, redaction, retention, and source-span diagnostics. | semantics / sample evidence | `partial-canon` | `GLOSSARY.md` CON-035/036, `theory/02-types-effects-failures.md`, `theory/07-observation.md`, `theory/10-diagnostics.md`, `architecture/02-boundary-contracts.md` BND-008, `plan/00-gates.md` G4 | Static helper output is not final viewer / telemetry ABI and not an untyped debug leak. | Preserve effect/authority/redaction labels when using sample evidence. | L1/L2 |
| `specs/30`, `specs/31`, `specs/36`, `specs/38`, `plan/55`, `plan/62`, `plan/63`, provider/backend rows | Projection, backend, provider adapter, FFI/packet/schema inventory, and target manifests. | semantics / implementation evidence | `partial-canon` | `GLOSSARY.md` CON-032..034, `architecture/02-boundary-contracts.md` BND-005..007, `plan/00-gates.md` G6 | Provider, renderer, engine, backend, and transport are not semantic owners. | Reopen after preservation targets are crisp. | L1/L2 |
| `specs/42`, `plan/67`, P-SURF-06 rows, hot-plug reports | Source patch pipeline, compatibility, admission, activation cut, and rejection without mutation. | semantics / implementation evidence | `partial-canon` | `ADR-0006`, `GLOSSARY.md` CON-030/031, `theory/08-patch-hotplug.md`, `architecture/02-boundary-contracts.md` BND-004, `plan/00-gates.md` G7 | Hot-plug is a capstone correctness operation, not direct eval and not the first theory target. | Keep as later G7 input after G1..G6. | L1/L2 |
| Product Alpha, operational product suite, Full System V1, Surface P-SURF, `samples_progress.md`, helper reports | Runnable workflows, sample matrices, release checks, and alpha closeouts. | implementation status / sample evidence | `LAB-evidence-only` | `meta/source-hierarchy.md`, `plan/01-phases.md`, `spec/06-conformance.md` for future conformance target | Canon plan state remains T0: file existence, helper success, or old closeout does not mean canon implementation-state completion. | Cite exact commands/results as evidence only. | not canon |
| `samples/lean/`, proof-stub reports, Lean sync reports | Mechanization evidence and theorem stubs. | proof evidence | `LAB-evidence-only` / `OPEN` | `theory/11-metatheory-ledger.md`, `plan/00-gates.md` G1/T2 | Do not treat generated stubs or local Lean checks as proof obligation discharge unless `theory/11` says so. | Use as supporting evidence for future proof packages. | not canon until ledger discharge |
| Reversed Library, PrismCascade, Typed-Effect Wiring Platform, upper application docs | Satellite systems and upper-layer directions. | vocabulary / architecture | `canonized` for separability, `OPEN` for product detail | `NORTH-STAR.md`, `architecture/01-strata.md`, `architecture/02-boundary-contracts.md` | Do not collapse Mir, Mirrorea, PrismCascade, and Typed-Effect Wiring Platform into one implementation. | Keep separate workstream rows. | L0/L1 |
| `.docs/oracle-chatgpt-pro-operations.md`, `AGENTS.md`, consultation reports, `plan/69` | Oracle and sub-agent consultation outputs. | process / advisory evidence | `LAB-evidence-only` | `plan/02-operating-model.md`, `meta/agent-instructions.md` | External chat output is advisory; useful conclusions must be mirrored into repo source hierarchy before use. | Continue using Oracle for difficult theory/review decisions, but judge results against canon. | not canon |

## Rejected or superseded claim patterns

| Old pattern | Disposition | Reason |
|---|---|---|
| Legacy `specs/` as current normative source | `superseded` | `mirrorea_canon/` is the sole canon source after ADR-0012. |
| `World`, `Room`, `Avatar`, `Game`, or sample root names as Mir core primitives | `rejected-as-canon` | Canon treats these as domain/library vocabulary. |
| `Event` as the primary ordinary source model | `rejected-as-canon` | Canon distinguishes occurrence/publication/domain event vocabulary. |
| `package.mir.json` as semantic source authority | `superseded` | It is an alpha compatibility/package artifact; source meaning belongs to `.mir` or canon-defined source language. |
| Hot-plug as direct eval or first semantics target | `rejected-as-canon` | Canon puts hot-plug behind parse/check/elaborate/compatibility/admission and later G7 gates. |
| Helper/report/sample closeout as canon implementation-state completion | `rejected-as-canon` | Canon `plan/01-phases.md` says the project is in T0 and implementation state is spoken only there. |
| Provider/transport/runtime kind as authority | `rejected-as-canon` | Authority comes through capability/admission/witness lineage, not transport or provider labels. |
| Debug/telemetry as untyped leak | `rejected-as-canon` | Observation is an effect boundary with authority, redaction, retention, and source-span duties. |

## Current safe next actions

1. Use this ledger as the first LAB downgrade map for T0/G0 work.
2. Do not claim G0 exit from this ledger. G0 exit still requires canon gate
   judgment and any required human decision.
3. The ordinary-assignment row now has `plan/118` as its line-level LAB
   citation drilldown. Do not re-open it as if `plan/71..78` and `plan/117`
   did not already exist.
4. `plan/119-g0-remaining-claim-family-drilldown-priority.md` now classifies
   the remaining rows by gate / evidence priority. Use it before opening
   another `plan/70` drilldown.
5. Later drilldowns may split this file into per-claim rows or machine-readable
   ledgers, but should not create a second normative source.

## Open questions

- Should the ledger later become machine-readable, or remain human-readable
  `plan/` memory?
- Which legacy LAB claim families need exact line-level citations before a G0
  close decision?
- Should canon add a short mental-model note for ordinary assignment before G1,
  or should that wait for the G1 target draft?
- Should old LAB wording be linted by script, or handled by focused manual
  audits as packages touch relevant files?
