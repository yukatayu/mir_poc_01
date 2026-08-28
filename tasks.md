# Current Task Map (LAB)

最終更新: 2026-08-28 20:06 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins. This LAB snapshot does not
create a goal, queue, lifecycle decision, proof claim, or compatibility promise.

## document role

Plan 247 and Plan 249 are closed execution records. Reports hold exact evidence.
“Current” below means repository maintenance state, not a promoted semantic
milestone or Canon L2 promotion.

## current promoted package

**None.** SYS-0--SYS-7 and the ADR-0026 Mirrorea I2 Systems Foundation program
are closed. There is no active bounded program, current roadmap, active
semantic milestone, or active goal.

Accepted SYS-6 implementation/evidence cut is `5429712d...`; Canon/status
integration cut is `bcb0f767...`. ADR-0032 accepted official I2 entry then
exit. Theory remains T1 and broad PHASE-I1 remains unaccepted.

PROPOSAL-036 / ADR-0033 / Canon plan/05 close SYS-7 with an inactive I3 entry
contract. Candidate A TLS-over-TCP framed reliable stream and Candidate B QUIC
reliable stream are both **UNSELECTED**; QUIC datagrams are not admitted or
evaluated. I3 remains inactive and OPEN-032 unresolved. A successor requires
fresh owner direction and a new current roadmap.

Direct consumer: a future owner-authorized I3 program that does not yet exist.

Primary falsifier: a change selects/implements transport without new owner
direction, treats transport metadata as authority, hides retry/exactly-once,
freezes the internal carrier as public wire, or activates I3.

Sources: `mirrorea_canon/adr/ADR-0033.md`,
`mirrorea_canon/plan/05-i3-entry-contract.md`, and
`plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`.

## ordered self-driven packages

There is no promoted implementation package. Only maintenance or separately
eligible ADR-0014 research can proceed autonomously:

| Order | Package | Capability / evidence | Current state / rough estimate |
|---|---|---|---|
| 1 | accepted regression maintenance | preserve M10 and SYS-1--SYS-6 behavior/classes | as needed; Macro 1/3/4, bounded |
| 2 | Canon/LAB consistency | repair stale hierarchy/lifecycle/queue wording without widening | as needed; Macro 0, short |
| 3 | direct-consumer L3 research | reversible `working/WRK-####` under ADR-0014 | optional; Macro 1/5 reserve, bounded |

No package authorizes socket/process runtime, transport selection, public wire,
deployment, I3 entry, or a new semantic frontier.

## self-driven macro phase reading

| Macro | Current state | Startability |
|---|---|---|
| 0 governance/repository memory | program closed; no active queue | consistency maintenance |
| 1 semantic kernel | finite kernel/backend/projection/runtime accepted | maintenance / ADR-0014 L3 |
| 2 parser-free history | retained; not current architecture | maintenance |
| 3 source/checker/runtime | source-first I2 boundary accepted | no I3 implementation authority |
| 4 executable samples | four-locus toy + conform reproducible | regression only |
| 5 theorem/model bridge | OBL-058 bounded; 059--063 runtime | class maintenance |
| 6 generated/distributed fabric | in-process accepted; network absent | owner direction required |
| 7 toolchain/backend | provisional commands | no public freeze |
| 8 upper application | toy remains sample/library consumer | no Core promotion |

## user decision gates

| Overview | Impact | Major options | Current recommendation |
|---|---|---|---|
| activate successor I3 program | distributed fabric | new bounded direction / inactive | remain inactive until authorized |
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

These are future I3 requirements, not current tasks or parallel queues:

| Question | Direct consumer | Evidence required | Boundary |
|---|---|---|---|
| Candidate A vs B | future I3 program | comparative C-distributed evidence under same gates | both UNSELECTED |
| internal/public representation | future wire design | checked carrier mapping/redaction | no codec/version/wire freeze |
| network failure matrix | future profile | typed positive/falsifier executions | no hidden retry/exactly-once |
| network ordering | future runtime/model | request/serve, revoke/use, publish/observe, patch/cut mapping | stream order insufficient |
| C-distributed gates | future acceptance | ordinary-source SCN-01/02/03/06, correspondence, review | I2 alone insufficient |

Open a WRK only with a named direct consumer, current blocker reduction,
alternative/falsifier, acceptance use, and adoption/discard rule. Historical
WRKs do not become a queue automatically.

## maintenance tasks

- Preserve Canon > LAB and both closed roadmap baselines.
- Preserve M10 cut `23f5a813...`; never reuse it as I2/I3 identity.
- Preserve accepted SYS-1--SYS-6 cuts and exact evidence classes.
- Keep `conform-i2` downstream evidence, not lifecycle authority.
- Keep Theory T1, broad PHASE-I1 residual, official I2 exit, I3 inactivity,
  and OPEN-032 unresolved as independent axes.
- Keep transport/session/certificate/route identity non-authoritative.
- Keep internal carrier/public wire separate; no hidden retry/exactly-once.
- Reopen SYS-6 only for a PROPOSAL-035/ADR-0032 falsifier or M10 regression.
- Reopen SYS-7 wording only for authority collapse, a missing required network
  failure/order case, premature selection/freeze, or unusable direct-consumer
  contract.
- Do not create an active roadmap until owner direction designates one.

## non-promoted references

Historical plans, reports, WRKs, Product Alpha, Full System V1, Surface Mir,
parser-free helpers, View/provider samples, and candidate transports are
repository memory or future consumers, not active queues. Closed SYS-7 does
not activate I3 or authorize OPEN-032 selection.
