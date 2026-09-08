# Current Task Map (LAB)

最終更新: 2026-09-08 12:43 JST

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

**I3-3 active.** I3-2 is a completed/accepted inventory item; the owner has
resumed at its accepted cut. PROPOSAL-037 / ADR-0034 authorize the bounded
Mirrorea I3 Distributed Foundation program. ALIGN-0, ALIGN-1, ALIGN-2 and I3-0
are completed, and I3-1 is closed by ADR-0038. I3-2 is accepted at the bounded FM-5 source/evidence cut. The fixed sequence remains ALIGN-0..2 → I3-0..6 → NEXT-0; I3-4/I3-5/I3-6/NEXT-0 are dependency-gated inactive.
Official I3 lifecycle entry is not claimed.

Latest owner instruction schedules a pause after I3-3 acceptance, required
validation/review, commit/push and parity. Do not activate I3-4 without explicit
owner resume. I3-3 remains active now; after acceptance retain Plan 250 with no
active milestone, neither blocked nor program-closed.

PROPOSAL-040 / ADR-0037 resolve OPEN-032 only for this bounded program and
select QUIC reliable stream as the private adapter. Both TLS/TCP
and QUIC reliable stream passed the same source/Core-bound nine-case actual-
process canary and tied on criteria 1--7; criteria 8 and 9 had no auditable
winner; criterion 10 future browser relevance
was the first material difference. TLS-over-TCP is retained as a rejected/deferred
comparison and replacement baseline. QUIC
datagrams remain excluded.

### Accepted input history: I3-2

I3-2 launched at least two independent OS processes from accepted generated
per-locus artifacts and dispatched remote owner work through the accepted private
QUIC adapter. Deployment data mapped only logical loci to process/endpoint;
communication edges, owner operations, authority, state, arguments and
occurrence identity remained checked-artifact facts. The runtime preserved
source/Core/artifact/carrier/network correspondence, prohibited a shared remote
store, and terminated/reaped all processes deterministically. This is accepted
input history, not the promoted I3-3 goal.

### Active goal: I3-3 network failure, retry, and ordering refinement

Milestone direct consumer: I3-4 C-distributed scenarios and cross-process
semantic pressure consume the I3-3 matrix only after milestone acceptance.
The genuine M9 successor/retained-ingress and selected-adapter delivery slices
have passed bounded repair review. Actual endpoint closure, a complete frame
split over two application writes, and strict truncation/FIN execute with
validated-only observer evidence. Pushed checkpoint `050f5067` selects and
preserves ADR-0041/spec/16's source-declared owner-admission budget with default
execution guards. Pushed `30429d5` implements the owner clock, bounded ledger,
one-use handoff, M9 revalidation, genuine expiry transport/consumption and
requester-local wait. The successor replay delta is reviewed with P0/P1/P2 zero:
successful and expiry replies are consumed once then rejected on verified
session-2 replay, while wrong-initial-session replay rejects before write.
Current full probe 41/41, focused replay 3/3, two-crate all-target Clippy and
workspace format pass. Runtime 332/process 63/I2-M10 5/8/67 are retained
`30429d5` results, not reruns. Genuine G1-expiry→G2 rejection remains local
production-binder evidence; actual QUIC's use of that binder is verified.
The row-11/spec/16 consumer is integrated/pushed at `55f1fd7f`, with clean
remote parity confirmed at `2026-09-08T12:16:45+09:00`.
Current work: implement the selected ADR-0042/spec/17 provider contract,
starting with test-first source/checker preservation, then effect grant, projection and real
external attempt. Remaining membership/redaction/cut families, full ordering,
regressions and I3-3 acceptance remain required. Root has about 12.00 GiB free.

Scope: execute all 20 required network failure families against the accepted
generated-artifact runtime; make retry and ambiguous delivery request-bound and
operation-specific; refine concrete occurrences to Mir dependency/frontier/
provenance order rather than stream order; preserve owner-local mutation,
authority, provenance, redaction, and bounded termination.

Primary falsifier: second mutation or consume, stale resurrection, false
unavailable or false success, blind/universal retry, lost redaction, or an
unbounded/orphaned resource or lifecycle.

Sources: `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`,
`docs/reports/2606-mirrorea-i3-distributed-foundation-i3-3-failure-ordering.md`,
`mirrorea_canon/adr/ADR-0039.md`, and
`mirrorea_canon/plan/05-i3-entry-contract.md`. I3-2 evidence remains the
input history in Report 2605, not the active goal.

Retained delivery-checkpoint evidence: runtime integration 61/61 (55.23s), I2 local 5/5
(0.05s), I2 CLI 8/8 (11.60s), probe 32/32, private QUIC unit 2/2, focused
Clippy and format/diff pass. Feature library 289/289 and M10 67/67 remain prior
evidence; I2 regressions 5/5 + 8/8 pass. Independent repair review has no
remaining P0/P1. B installs genuine revocation before reconnect and rejects
retained ingress without owner mutation; A stays pending. The earlier probe
lifecycle unit 2/2 is prior evidence; the full probe library now passes 3/3.
Pre-write reconnect consumes one checked receipt;
post-admission retry is duplicate-rejected while the original requester stays
pending/ambiguous. Report 2606 retains prior checkpoint `87ee2418` runtime
51/51, library 285/285, probe 17/17 and bounded model 3/3 evidence; those suites
were not all rerun for this delta. The model is not a general proof. Root free
space was about 15 GiB at that prior delivery checkpoint; the later C1/C2
checkpoint had 14 GiB free. Remaining authority/
ordering and the full 20-family matrix remain open; this
is not milestone acceptance.

### Dated LAB rough remaining estimates (not acceptance or guarantee)

As of 2026-09-07 13:57 JST, rough uninterrupted work estimates are I3-3
12–24h, I3-4 6–12h, I3-5 3–6h, I3-6 4–8h, and NEXT-0 1–2h (total 26–52h).
These are planning estimates only; provider/time contracts, review findings,
validation breadth, and interruption time are uncertain and excluded. Historical
I3-2 elapsed time is not used as an active-work estimate.

## ordered self-driven packages

I3-3 is the only active semantic milestone; I3-1 and I3-2 are closed bounded evidence. Later packages become active one at a time after the
preceding report, validation, review, commit/push and remote parity close;
the scheduled pause after I3-3 additionally requires explicit owner resume:

| Order | Package | Capability / evidence | Current state / rough estimate |
|---|---|---|---|
| 1 | ALIGN-0 | authority, sole roadmap, baseline, meta-drift control | completed; Macro 0 front |
| 2 | ALIGN-1 | three-axis architecture and responsibility map | completed; Macro 0 middle |
| 3 | ALIGN-2 | Browser/Host/package/View/provider contracts | completed; Macro 1 front |
| 4 | I3-0 | equal transport canaries and selected private adapter | completed; Macro 6 front |
| 5 | I3-1 | checked private encoding/adapter/admission | completed by ADR-0038; bounded evidence |
| 6 | I3-2 | generated-artifact two-or-more-process owner runtime | completed/accepted FM-5; Macro 6 middle, heavy |
| 7 | I3-3 | all 20 failure families, explicit retry/ambiguity, and ordering refinement | active; Macro 5/6 middle, heavy |
| 8 | I3-4 | C-distributed scenarios | inactive; dependency-gated on I3-3 |
| 9 | I3-5 / I3-6 | joined workflow and finite conformance/lifecycle close | inactive; dependency-gated |
| 10 | NEXT-0 | two separate inactive I4/I5 entry contracts only | reserve path; dependency-gated |

## I3-2 closed work-package inventory

These were dependency-ordered parts of the accepted I3-2 milestone, not
separate goals or a current queue:

| Order | Work package | Acceptance evidence | Ownership boundary |
|---:|---|---|---|
| 1 | process/deployment contract | locus→process/endpoint only; generated edge and artifact census remains exact | code mapping/planner; no source reparse or manual route |
| 2 | independent locus process runtime | each process starts only assigned artifacts and local state/queues | production implementer; no cross-process semantic store |
| 3 | generated remote owner dispatch | request/serve/result/receipt crosses accepted QUIC seam | production implementer; endpoint/session remains non-authority |
| 4 | launcher and deterministic cleanup | at least two OS processes, fresh ports, complete reap/no residual | test author separate from production writer |
| 5 | correspondence and falsifiers | source/Core/artifact/edge/carrier/network/runtime join plus route/owner/store negatives | independent runtime/security review |
| 6 | I3-3 fault-harness entry | actual route exposes bounded controls without injecting semantic facts | I3-3 consumes accepted runtime |

## self-driven macro phase reading

| Macro | Current state | Startability |
|---|---|---|
| 0 governance/repository memory | ALIGN-0--2 and I3-0/I3-2 completed | maintenance |
| 1 semantic kernel | finite kernel/backend/projection/runtime accepted | maintenance / ADR-0014 L3 |
| 2 parser-free history | retained; not current architecture | maintenance |
| 3 source/checker/runtime | source-first I2 boundary accepted; I3-2 accepted; I3-3 active | I3-3 self-drive |
| 4 executable samples | four-locus toy + conform reproducible | regression only |
| 5 theorem/model bridge | OBL-058 bounded; 059--063 runtime | class maintenance |
| 6 generated/distributed fabric | actual two-process owner runtime accepted; I3-3 fault/order active | later I3 gates dependency-gated |
| 7 toolchain/backend | provisional commands | no public freeze |
| 8 upper application | toy remains sample/library consumer | no Core promotion |

## user decision gates

OPEN-032 is no longer a user blocker for this program. The remaining rows are
owner-reserved stop boundaries. The explicit resume decision is resolved history:
the owner resumed at the verified I3-2 cut and activated I3-3 only.

| Overview | Impact | Major options | Current recommendation |
|---|---|---|---|
| public API/ABI/wire freeze | compatibility | provisional / freeze | keep private and provisional |
| production/publication/resources | external risk | local evidence / deploy | remain local |
| North Star or safety/privacy change | whole semantics | preserve / weaken | preserve; owner decision bundle |
| domain vocabulary as Core | Core architecture | sample/library / promote | keep sample/library |
| hidden multi-owner transaction/retry | authority/atomicity | explicit / hidden | preserve explicit operations |
| irreversible semantic tie | migration | Constitution / owner | stop if non-migratable |

The resume decision is resolved history, not a current blocker. Theory T1, broad-I1 residuals, missing public ABI, deferred general OBLs,
unoptimized performance, untested production/browser platforms and incomplete
I3+ are not by themselves stop conditions.

## research discovery items

These are fixed direct consumers, not parallel queues:

| Question | Direct consumer | Evidence required | Boundary |
|---|---|---|---|
| internal/private/public representation | I3-1 (closed) | accepted exhaustive checked carrier mapping/redaction | private provisional only; no public freeze |
| decoder/limit policy | I3-1 (closed) | accepted deterministic property/mutation plus typed falsifiers | no partial request or pre-limit allocation; no coverage-guided fuzz claim |
| actual process runtime | I3-2 accepted input history | generated-plan-only owner dispatch | no source reparse/manual route/shared store; I3-3 refines faults/order on this seam |
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
- Do not activate I3-4/I3-5/I3-6/NEXT-0 before their preceding package acceptance
  under the fixed sequence; no new roadmap is created.

The working interpretation remains ordinary meaning -> generated distribution ->
continually checked composition. World/Avatar remain domain-library vocabulary,
not Mir Core primitives; ledger64/coordinator/closed-cohort finite-profile
machinery are bounded evidence, not general Mir requirements, and do not create
a new theory gate or roadmap item.

## non-promoted references

Historical plans, reports, WRKs, Product Alpha, Full System V1, Surface Mir,
parser-free helpers, View/provider samples and the deferred TLS/TCP candidate are
repository memory or future consumers, not parallel active queues. I3-0 probe
execution is evidence-only and does not itself constitute I3-1 adapter
acceptance, I3-2 bounded owner-runtime evidence, workflow/product completion or official I3 entry.
