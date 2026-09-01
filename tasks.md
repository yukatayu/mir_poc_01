# Current Task Map (LAB)

最終更新: 2026-09-01 22:05 JST

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

**ALIGN-1 (sole active goal; activation only).** PROPOSAL-037 / ADR-0034 authorize the bounded
Mirrorea I3 Distributed Foundation program. ALIGN-0 is completed; ALIGN-2 is next and not active;
fixed sequence: ALIGN-0..2 → I3-0..6 → NEXT-0. Official I3 lifecycle entry is
not claimed.

Accepted SYS-6 implementation/evidence cut is `5429712d...`; Canon/status
integration cut is `bcb0f767...`. ADR-0032 accepted official I2 entry then
exit. Theory remains T1 and broad PHASE-I1 remains unaccepted.

PROPOSAL-036 / ADR-0033 / Canon plan/05 close SYS-7 with an inactive I3 entry
contract. Candidate A TLS-over-TCP framed reliable stream and Candidate B QUIC
reliable stream are both **UNSELECTED**; QUIC datagrams are not admitted or
evaluated. The bounded I3 program is active, while lifecycle entry and OPEN-032
remain unresolved. Both candidates remain UNSELECTED; QUIC datagrams are
excluded.

Direct consumer: the active Plan 250 ALIGN-1 goal; no ALIGN-1 work/evidence has started.

Primary falsifier: a change selects transport before equal I3-0 executable
canaries and its ADR, treats transport metadata as authority, hides
retry/exactly-once, freezes the internal carrier as public wire, or claims
official I3 entry before I3-6 acceptance.

Sources: `mirrorea_canon/adr/ADR-0034.md`,
`mirrorea_canon/plan/05-i3-entry-contract.md`, and
`plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`.

## ordered self-driven packages

Only ALIGN-1 is active (activation only). Later packages become active one at a time after the
preceding report, validation, review, commit/push, and remote parity close:

| Order | Package | Capability / evidence | Current state / rough estimate |
|---|---|---|---|
| 1 | ALIGN-0 | authority, sole roadmap, baseline, meta-drift control | completed; Macro 0 front, short |
| 2 | ALIGN-1 | three-axis architecture and Browser/Host/View/provider trust boundaries | sole active goal (activation only); Macro 0 middle, medium |
| 3 | ALIGN-2 | execution/evidence boundary | next; not active |
| 3 | I3-0 | equal transport canaries and one selected adapter ADR | later; Macro 6 front, heavy |
| 4 | I3-1 / I3-2 | checked private encoding, adapter, and real multi-process runtime | later; Macro 6 middle, heavy |
| 5 | I3-3 / I3-4 | full finite fault/order matrix and C-distributed scenarios | later; Macro 5/6 middle, heavy |
| 6 | I3-5 / I3-6 | joined network workflow and finite conformance/lifecycle close | later; Macro 6/7 close, heavy |
| 7 | NEXT-0 | two separate inactive I4/I5 entry contracts only | reserve path, short |

Plan 250 authorizes socket/process implementation and transport selection only
at their fixed milestone gates. It does not authorize public wire freeze,
production deployment, or official I3 entry before I3-6 acceptance.

## self-driven macro phase reading

| Macro | Current state | Startability |
|---|---|---|
| 0 governance/repository memory | ALIGN-0 completed; ALIGN-1 activation-only | active; short |
| 1 semantic kernel | finite kernel/backend/projection/runtime accepted | maintenance / ADR-0014 L3 |
| 2 parser-free history | retained; not current architecture | maintenance |
| 3 source/checker/runtime | source-first I2 boundary accepted | ALIGN-1/2 first; I3-1/2 later |
| 4 executable samples | four-locus toy + conform reproducible | regression only |
| 5 theorem/model bridge | OBL-058 bounded; 059--063 runtime | class maintenance |
| 6 generated/distributed fabric | in-process accepted; network absent | fixed I3-0..6 sequence authorized |
| 7 toolchain/backend | provisional commands | no public freeze |
| 8 upper application | toy remains sample/library consumer | no Core promotion |

## user decision gates

OPEN-032 is delegated to I3-0 comparative evidence and ADR. The remaining
rows are owner-reserved stop boundaries rather than current blockers.

| Overview | Impact | Major options | Current recommendation |
|---|---|---|---|
| OPEN-032 transport selection | adapter architecture | Candidate A / Candidate B | retain both UNSELECTED pending evidence |
| public API/ABI/wire freeze | compatibility | provisional / freeze | keep provisional and separate |
| production/publication/resources | external risk | local evidence / deploy | remain local |
| North Star or safety/privacy change | whole semantics | preserve / weaken | preserve; owner decision bundle |
| domain vocabulary as Core | Core architecture | sample/library / promote | keep sample/library |
| hidden multi-owner transaction | authority/atomicity | explicit / hidden | preserve explicit operations |
| irreversible semantic tie | migration | Constitution / owner | stop if non-migratable |

Theory T1, broad-I1 residuals, missing public ABI, deferred general OBLs,
unoptimized performance, and incomplete I3+ do not activate a task.

## research discovery items

These are fixed later milestones, not parallel queues while ALIGN-1 is active:

| Question | Direct consumer | Evidence required | Boundary |
|---|---|---|---|
| Candidate A vs B | I3-0 | equal two-process executable canaries | both UNSELECTED until ADR |
| internal/private/public representation | I3-1 | checked carrier mapping/redaction | private provisional encoding; no public freeze |
| network failure matrix | I3-3 | typed positive/falsifier executions | no hidden retry/exactly-once |
| network ordering | I3-3 | request/serve, revoke/use, publish/observe, patch/cut mapping | stream order insufficient |
| C-distributed gates | I3-4 / I3-6 | ordinary-source SCN-01/02/03/06, correspondence, review | I2 alone insufficient |

Open a WRK only with a named direct consumer, current blocker reduction,
alternative/falsifier, acceptance use, and adoption/discard rule. Historical
WRKs do not become a queue automatically.

## maintenance tasks

- Preserve Canon > LAB and both closed roadmap baselines.
- Preserve M10 cut `23f5a813...`; never reuse it as I2/I3 identity.
- Preserve accepted SYS-1--SYS-6 cuts and exact evidence classes.
- Keep `conform-i2` downstream evidence, not lifecycle authority.
- Keep Theory T1, broad PHASE-I1 residual, official I2 exit, active bounded I3
  program, inactive official I3 lifecycle, and OPEN-032 unresolved as
  independent axes.
- Keep transport/session/certificate/route identity non-authoritative.
- Keep internal carrier/public wire separate; no hidden retry/exactly-once.
- Reopen SYS-6 only for a PROPOSAL-035/ADR-0032 falsifier or M10 regression.
- Reopen SYS-7 wording only for authority collapse, a missing required network
  failure/order case, premature selection/freeze, or unusable direct-consumer
  contract.
- Do not create another roadmap or activate ALIGN-2 before ALIGN-1 closes under
  the fixed sequence and owner boundaries.

## non-promoted references

Historical plans, reports, WRKs, Product Alpha, Full System V1, Surface Mir,
parser-free helpers, View/provider samples, and candidate transports are
repository memory or future consumers, not parallel active queues. ALIGN-0
activation does not authorize OPEN-032 selection; both candidates remain
UNSELECTED.
