# Current Task Map (LAB)

最終更新: 2026-09-02 02:52 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins. This LAB snapshot does not
create a goal, queue, lifecycle decision, proof claim, or compatibility promise.

## document role

Plan 247 and Plan 249 are closed execution records. Plan 250 is the sole
current roadmap for the active bounded I3 program. Reports hold exact evidence.
“Current” below means repository maintenance state, not a promoted semantic
milestone or Canon L2 promotion.

## current promoted package

**I3-1 (sole active goal).** PROPOSAL-037 / ADR-0034 authorize the bounded
Mirrorea I3 Distributed Foundation program. ALIGN-0, ALIGN-1, ALIGN-2 and I3-0
are completed. The fixed sequence remains ALIGN-0..2 → I3-0..6 → NEXT-0.
Official I3 lifecycle entry is not claimed.

PROPOSAL-040 / ADR-0037 resolve OPEN-032 only for this bounded program and
select QUIC reliable stream as the private adapter. Both TLS/TCP
and QUIC reliable stream passed the same source/Core-bound nine-case actual-
process canary and tied on criteria 1--7; criteria 8 and 9 had no auditable
winner; criterion 10 future browser relevance
was the first material difference. TLS-over-TCP is retained as a rejected/deferred
comparison and replacement baseline. QUIC
datagrams remain excluded.

I3-1 must implement a checked transport-neutral, private provisional
encode/decode/admission boundary from the accepted internal carrier to the
selected QUIC reliable stream. It must preserve source/Core/artifact provenance, effect/failure/
visibility/redaction and request/serve/result/receipt separation, reject partial/
malformed/oversized/version/tampered input before semantic admission, and never
treat transport/session/certificate as authority.

Direct consumer: I3-2 actual locus-process runtime and generated communication.

Primary falsifier: an untrusted or incomplete byte sequence becomes a semantic
carrier, a required field is lost/defaulted/invented, protected data is logged,
or the private format is presented as public compatibility.

Sources: `mirrorea_canon/adr/ADR-0037.md`,
`mirrorea_canon/plan/05-i3-entry-contract.md`, and
`plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`.

## ordered self-driven packages

Only I3-1 is active. Later packages become active one at a time after the
preceding report, validation, review, commit/push and remote parity close:

| Order | Package | Capability / evidence | Current state / rough estimate |
|---|---|---|---|
| 1 | ALIGN-0 | authority, sole roadmap, baseline, meta-drift control | completed; Macro 0 front |
| 2 | ALIGN-1 | three-axis architecture and responsibility map | completed; Macro 0 middle |
| 3 | ALIGN-2 | Browser/Host/package/View/provider contracts | completed; Macro 1 front |
| 4 | I3-0 | equal transport canaries and selected private adapter | completed; Macro 6 front |
| 5 | I3-1 | checked private encoding/adapter/admission | sole active goal; Macro 6 middle, heavy |
| 6 | I3-2 | generated-artifact two-or-more-process owner runtime | next/inactive; Macro 6 middle, heavy |
| 7 | I3-3 / I3-4 | full finite fault/order matrix and C-distributed scenarios | later; Macro 5/6 middle, heavy |
| 8 | I3-5 / I3-6 | joined workflow and finite conformance/lifecycle close | later; Macro 6/7 close, heavy |
| 9 | NEXT-0 | two separate inactive I4/I5 entry contracts only | reserve path, short |

## I3-1 self-driven work packages

These are dependency-ordered parts of one active milestone, not separate goals:

| Order | Work package | Acceptance evidence | Ownership boundary |
|---:|---|---|---|
| 1 | exact carrier inventory and private mapping | exhaustive field table; request/serve/result/receipt remain distinct | production implementer; no process runtime |
| 2 | limits/version/decoder hardening | partial, malformed, oversize, duplicate-key, marker and version negatives | test author separate from production writer |
| 3 | selected QUIC adapter integration | lossless round trip and transport non-authority checks | selected adapter only; TLS/TCP remains baseline |
| 4 | P2 security/lifecycle hardening | credential lifetime/zeroization, bounded reaper, no-mint-before-cache-reuse | security review; no production claim |
| 5 | identity and observer-safety cleanup | request-hash label alignment, owner-request scope, secret/redaction scan | no relation/designated overclaim |
| 6 | regression and independent close review | focused/property/fuzz, I2/M10 floor, P0/P1 zero | parent integrates Report 2604 and advances I3-2 |

## self-driven macro phase reading

| Macro | Current state | Startability |
|---|---|---|
| 0 governance/repository memory | ALIGN-0--2 and I3-0 completed; I3-1 active | maintenance |
| 1 semantic kernel | finite kernel/backend/projection/runtime accepted | maintenance / ADR-0014 L3 |
| 2 parser-free history | retained; not current architecture | maintenance |
| 3 source/checker/runtime | source-first I2 boundary accepted | I3-1 active; I3-2 later |
| 4 executable samples | four-locus toy + conform reproducible | regression only |
| 5 theorem/model bridge | OBL-058 bounded; 059--063 runtime | class maintenance |
| 6 generated/distributed fabric | transport selected; checked mapping absent | I3-1 active |
| 7 toolchain/backend | provisional commands | no public freeze |
| 8 upper application | toy remains sample/library consumer | no Core promotion |

## user decision gates

OPEN-032 is no longer a user blocker for this program. The remaining rows are
owner-reserved stop boundaries, not current missing specifications:

| Overview | Impact | Major options | Current recommendation |
|---|---|---|---|
| public API/ABI/wire freeze | compatibility | provisional / freeze | keep private and provisional |
| production/publication/resources | external risk | local evidence / deploy | remain local |
| North Star or safety/privacy change | whole semantics | preserve / weaken | preserve; owner decision bundle |
| domain vocabulary as Core | Core architecture | sample/library / promote | keep sample/library |
| hidden multi-owner transaction/retry | authority/atomicity | explicit / hidden | preserve explicit operations |
| irreversible semantic tie | migration | Constitution / owner | stop if non-migratable |

Theory T1, broad-I1 residuals, missing public ABI, deferred general OBLs,
unoptimized performance, untested production/browser platforms and incomplete
I3+ are not by themselves stop conditions.

## research discovery items

These are fixed direct consumers, not parallel queues:

| Question | Direct consumer | Evidence required | Boundary |
|---|---|---|---|
| internal/private/public representation | I3-1 | exhaustive checked carrier mapping/redaction | private provisional only; no public freeze |
| decoder/limit policy | I3-1 | property/fuzz plus typed falsifiers | no partial request or pre-limit allocation |
| actual process runtime | I3-2 | generated-plan-only owner dispatch | no source reparse/manual route/shared store |
| network failure matrix | I3-3 | typed positive/falsifier executions | no hidden retry/exactly-once |
| network ordering | I3-3 | request/serve, revoke/use, publish/observe, patch/cut mapping | stream order insufficient |
| C-distributed gates | I3-4 / I3-6 | SCN-01/02/03/06 source-first correspondence | I2 and I3-0 canaries alone insufficient |

Historical WRKs do not become a queue automatically. Open a WRK only with a
named direct consumer, blocker reduction, alternative/falsifier, acceptance use
and adoption/discard rule.

## maintenance tasks

- Preserve Canon > LAB and closed Plan 247/249 baselines.
- Preserve M10 cut `23f5a813...` and accepted SYS-1--SYS-6 cuts/evidence classes.
- Keep `conform-i2` downstream evidence, never lifecycle authority.
- Keep Theory T1, broad PHASE-I1 unaccepted, official I2 exit, active bounded I3,
  official I3 unentered and bounded OPEN-032 resolution as independent axes.
- Keep transport/process/session/certificate/route identity non-authoritative.
- Keep internal carrier, private provisional encoding and future public wire
  distinct; no hidden retry/exactly-once.
- Treat the I3-0 fixed-capacity in-memory cache as canary evidence only, not
  actual owner runtime, durability or exactly-once.
- Reopen I3-0 only for its equal-canary/selection/security falsifier.
- Do not activate I3-2 or create another roadmap before I3-1 closes under the
  fixed sequence.

## non-promoted references

Historical plans, reports, WRKs, Product Alpha, Full System V1, Surface Mir,
parser-free helpers, View/provider samples and the deferred TLS/TCP candidate are
repository memory or future consumers, not parallel active queues. I3-0 probe
execution is evidence-only and does not itself constitute I3-1 adapter
acceptance, I3-2 owner runtime, workflow/product completion or official I3 entry.
