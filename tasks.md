# Current Task Map (LAB)

最終更新: 2026-08-28 18:58 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## document role

This is the repository-wide current task-map snapshot. Plan 249 is the sole
active execution roadmap, Plan 247 is the closed M0--M10 baseline, and reports
hold detailed evidence. “Promoted” below means selected by ADR-0026 and the
current roadmap; it does not mean Canon L2 promotion, proof completion, public
compatibility, or product acceptance.

## current promoted package

**Active: SYS-7 I3 entry contract only.** SYS-0--SYS-6 are completed. SYS-6 is
accepted at implementation/evidence cut
`5429712de89a7e41c46cfd7fb4a39c4a492864c4` by PROPOSAL-035 / ADR-0032 /
spec/15 / OBL-063 / Report 2598.

ADR-0032 accepted official I2 entry then I2 exit after the exact 22-row
source-first profile and independent review. Theory remains T1. Broad
PHASE-I1 remains unaccepted because OPEN-026/027 and full carrier freeze stay
open. I3 remains inactive.

Direct consumer: a future owner-authorized I3 bounded program.

Blocker reduced by SYS-7: the future program needs one inactive parent goal,
carrier/authority boundary, failure/ordering matrix, and C-distributed entry
evidence without selecting or implementing transport.

Primary falsifier: SYS-7 text selects/implements transport, treats transport
metadata as authority, freezes public wire, activates I3, or claims production.

Sources: `mirrorea_canon/adr/ADR-0032.md`,
`mirrorea_canon/spec/15-sys6-i2-conformance.md`, and
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

## ordered self-driven packages

| Order | Package | Capability / evidence | Current state / rough estimate |
|---|---|---|---|
| 1 | Restate future I3 goal | map accepted I2 artifacts across at least two OS processes while preserving authority/failure/provenance/order | active; Macro 0/6 front, short |
| 2 | Candidate boundary | list at most two transports; non-authority; internal/public carrier split | active; Macro 6 middle, short |
| 3 | Failure/order contract | disconnect/reconnect, duplicate/reorder, wrong target, stale membership/revocation, send/receive and visibility mappings | active; Macro 5/6 middle, short |
| 4 | C-distributed entry evidence | source-first scenarios, negative matrix, assurance classification, no production/public freeze | active; Macro 5/6 reserve, short |
| 5 | SYS-7 close | Canon/status/report review; close Plan 249; stop with I3 inactive | terminal; Macro 0 reserve, short |

No production source, socket, process runtime, public wire schema, deployment,
or new semantic frontier belongs to these packages.

## self-driven macro phase reading

| Macro | Current state | Startability |
|---|---|---|
| 0 governance/repository memory | SYS-0--SYS-6 closed; SYS-7 active | contract/status self-driven |
| 1 semantic kernel | finite kernel/backend/projection/runtime accepted | maintenance only |
| 2 parser-free history | retained; not current architecture | maintenance only |
| 3 source/checker/runtime | source-first I2 systems boundary accepted | I3 contract only |
| 4 executable samples | four-locus toy + conform-i2 reproducible | regression only |
| 5 theorem/model bridge | OBL-058 bounded; OBL-059--063 runtime evidence | retain exact classes |
| 6 generated/distributed fabric | in-process accepted; network absent | entry contract only |
| 7 toolchain/backend | provisional project/run/inspect/conform-i2 | no public freeze |
| 8 upper application | toy remains sample/library consumer | no Core promotion |

SYS-7 must stop when its direct consumer can use the inactive contract, the
falsifier is excluded, independent review finds no major counterexample, and
non-goals/reopen triggers are explicit.

## user decision gates

No owner decision is required to finish SYS-7. The following are reserved and
must stop later work if they become necessary:

| Overview | Impact | Major options | Current recommendation |
|---|---|---|---|
| North Star or safety/privacy/no-stale weakening | whole semantics | preserve / weaken | preserve; return decision bundle |
| domain vocabulary as Core | Core architecture | library/sample / promote | keep library/sample |
| hidden multi-owner transaction | authority/atomicity | explicit operations / hidden transaction | preserve explicit operations |
| public API/ABI/wire freeze | compatibility | provisional / freeze | keep provisional |
| real transport selection/implementation | I3 architecture | future program / choose now | defer; OPEN-032 unresolved |
| production/publication/paid resources | external risk | local / deploy | remain local |
| irreversible observable-semantic tie | migration | Constitution order / owner choice | stop if non-migratable |
| reproducible North-Star contradiction | program validity | revise program / axis | return decision bundle |

Theory T1, broad I1 residuals, public ABI absence, deferred general OBLs,
performance non-optimization, and incomplete I3+ are not SYS-7 user blockers.

## research discovery items

These are bounded questions to record inside the single SYS-7 milestone/report;
they are not parallel semantic frontiers or independent WRKs.

| Question | Direct consumer | Evidence/decision required | Boundary |
|---|---|---|---|
| candidate transport inventory | future I3 program | at most two viable candidates and constraints | no selection |
| internal/public carrier split | future wire design | preservation mapping + unresolved encoding | no ABI/wire freeze |
| failure matrix | C-distributed profile | disconnect/reconnect/duplicate/reorder/stale/wrong-target typed outcomes | no exactly-once |
| network ordering refinement | future runtime/model | send/receive, revoke/use, publish/observe, patch/cut mappings | no general fairness theorem |
| C-distributed scenarios | future acceptance | ordinary source positive cases + typed network fault falsifiers | no production |
| OPEN-032 trigger | future owner decision | owner-authorized I3 program/entry review | unresolved in SYS-7 |

Open a new WRK only if a required fact cannot fit Report 2599 and has a named
direct consumer, explicit falsifier, and adoption/discard rule.

## maintenance tasks

- Keep Canon > LAB, Plan 249 sole-current until SYS-7 closes, and Plan 247
  historical.
- Preserve M10 cut `23f5a813...` as regression baseline, never I2 identity.
- Preserve accepted SYS-1--SYS-6 cuts and exact evidence classifications.
- Preserve four-locus ST versus selected-source OW1 scope.
- Keep `conform-i2` downstream evidence, not semantic/lifecycle authority.
- Keep theory T1, broad PHASE-I1 residual, official I2 exit, and I3 inactivity
  as independent lifecycle axes.
- Keep OPEN-032 unresolved until a future owner-authorized I3 decision.
- Reopen SYS-6 only for a PROPOSAL-035/ADR-0032 falsifier or M10 regression.
- At SYS-7 close, synchronize Plan 249, status snapshots, samples dashboard if
  commands change, one report, Canon index/hierarchy, validation, commit/push,
  and remote parity.

## non-promoted references

- Plan 247 and older numbered plans are repository memory, not active queues.
- Historical reports document evidence and do not self-authorize lifecycle.
- Existing WRK-0001/0002 and other L3 records are not SYS-7 blockers.
- Product Alpha, Full System V1, Surface Mir, parser-free helpers, View, and
  provider samples remain separate LAB lines, not hidden I3 work.
- A completed SYS-7 entry contract does not activate I3. After SYS-7, the
  ADR-0026 program stops until a new owner-authorized program exists.
