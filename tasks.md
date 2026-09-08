# Current Task Map (LAB)

最終更新: 2026-09-08 21:14 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project direction,
theory, ADRs, conformance, and process. Everything outside it is LAB evidence,
implementation or history. If LAB conflicts with canon, canon wins. This
snapshot creates no goal, phase, proof or public compatibility decision.

## document role

Plan 250 is the sole current roadmap under PROPOSAL-037 / ADR-0034.
Plan 247 and Plan 249 remain closed baselines, not successor queues.
Report 2606 holds exact I3-3 commands, negative evidence, reviews and cuts.
This is a rewritten current snapshot, not an append-only checkpoint history.

## current promoted package

Authority: `mirrorea_canon/adr/ADR-0034.md` and
`mirrorea_canon/spec/17-i3-read-only-provider-effect.md`.
Execution: `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`;
evidence: `docs/reports/2606-mirrorea-i3-distributed-foundation-i3-3-failure-ordering.md`.

**I3-3 alone is active.** Six of eleven milestones are accepted. This is a
milestone count, not a workload-weighted percentage or completion forecast.
Official I3 lifecycle remains unentered; Theory T1 and broad-I1 residuals are
unchanged. The private selected transport is QUIC reliable stream (ADR-0037);
TLS/TCP remains the deferred baseline and QUIC datagrams remain excluded.

Owner control: finish all I3-3 gates, independent review, commit/push and remote
parity, then stop. Retain Plan 250 with no active semantic milestone. I3-4
requires explicit owner resume. This pause is neither blocked nor program close.

### Accepted input history: I3-2

Accepted per-locus artifacts run in at least two actual OS processes over
private QUIC. Deployment maps logical loci to processes/endpoints only.
Source/Core/generated edges, operations, authority and results cannot be
supplied by deployment. There is no shared remote semantic store. This is
accepted input history, not the current I3-3 acceptance claim.

### Active goal: I3-3 network failure, retry, and ordering refinement

Complete all 20 required failure families and concrete-to-Mir ordering.
Preserve owner-local evaluation, current membership/capability/witness checks,
source/Core/artifact provenance, redaction, explicit retry and bounded shutdown.
Primary falsifiers: second mutation/consume, stale authority resurrection,
false success/nonexecution, blind retry, leaked private information, or hang.

Current component is the source-declared provider effect selected by
ADR-0042/spec/17 at contract cut 985ee179. Source/guard cut `9e8d674a` is
pushed with clean remote parity observed 2026-09-08 14:05 JST. Stage 2a static
coverage/projection/composite verification is retained at `3862b168`.
Accepted Stage2b cut `7142b205b4e2805d50ce78156de99e6feb1db37a` has clean
HEAD/origin/main/live parity at 2026-09-08 19:22 JST. Stage 2b actual resource binding and M9
authorization/scoped-M8 are accepted inactive evidence; invocation/runtime remain downstream.

Retained frozen Stage 2a evidence:

- Semantics composite 6/6, provider source 17/17, legacy finite 8/8.
- Composite privacy doctest 1/1; static runtime module 11/11.
- Full feature-union runtime library 345/345.
- Independent spec and quality reviews: no remaining P0/P1/P2.
- Fresh process 63/63, I2/M10 5/8/67, public guards 5/5, three-crate Clippy,
  workspace format and all-target compilation pass. The latter is not
  workspace test execution. Exact Git state remains in Report 2606.

Current Stage2b runtime is 362/362, including provider filtering 26/26 within
that total; process is 63/63, M10 is 67/67, I2 local/CLI is 5/5 and 8/8, and
public guards are 5/5. Three-crate all-target warnings-denied Clippy, workspace
format, and workspace all-target check pass; workspace tests and CLI/probe
Clippy are not claimed. Independent spec/quality review is P0/P1/P2 zero after
three direct tests and generic M8 typed refusal. The accepted Stage2b source/cut
is frozen; current Stage2c is dirty. Stage2b component is accepted but inactive.
Current Stage2c is the existing
SYS4/SYS5 inactive mixed-image handoff: final focused module/runtime is 13/13
included in runtime 375/375 (362 filtered; 9.65s/88.17s), process 63/63, M10
67/67, I2 local/CLI 5/5 and 8/8, guards 5/5, and actual ordinary localnet
41/41 (59.92s). Feature-union all-target Clippy `-Dwarnings` (22.06s), workspace
format and all-target check (17.41s) pass with no warnings; workspace tests and
probe-all tests are not claimed. Final independent review is P0/P1/P2 zero after
two P1 falsifiers and three P2 coverage/comment repairs. Stage2c is inactive/
green for parent integration; a new component commit/push remains pending. The
next direct consumer is actual Stage3 provider host/QUIC runtime; private M8
version 2 is required in scope, with no compatibility promise. This is not
Stage2c acceptance, host activity, provider row-18 completion or I3-3 completion.
Report 2606 retains exact logs.

Four review defects were reproduced and repaired: cross-source composite
candidate acceptance, raw legacy-discharge escape, nested provider-only
snapshot markers, and total provider-row omission by locus restriction.
Exact source binding and typed guards now preserve the pending requirement;
an identity query is not authentication. The production dependency guard also
required relocating tests, not weakening the guard. Stage 1 M8 7/7 and M9
external 1/1 remain earlier evidence. Stage2a results above are retained at
`3862b168`; the separately labelled current Stage2b packet reran process/I2/M10.
Exact commands and fresh versus retained results remain in Report 2606.

Stage2b is an incomplete local effect-authorization prototype: local tests
contain partial effect-authorization behavior, but no accepted host invocation,
provider network workflow, supervisor availability, M5/Lean clean-runner
registration or row-18 acceptance. The new ordinary source is
checking evidence, not an operational workflow.

The current-to-next component sequence is:

1. Bind a genuine trusted resource
   incarnation -> actual M9 membership/authentication and full composite
   verification -> separate effect policy -> inactive scoped M8 component.
   Preserve full checked-source identity; a legacy M8 component must not
   pretend to be the complete mixed program or escape through bare snapshots.
2. In the existing SYS4/SYS5 path, check the mixed private-image/admission
   variant without a second runtime or transport, followed by bounded ledger,
   admitted supervisor/resource binding, trusted host read, typed result/failure
   and the shared existing QUIC process path.
3. Real success with two fresh fixture values, actual resource failure,
   duplicate/loss/revocation/capacity/redaction falsifiers and integration.
4. Remaining membership/redaction/cut families, full ordering, final I3-3
   regressions/review/acceptance, then the owner-requested stop.

These are direct consumers within I3-3, not extra milestones or parallel queues.
Runtime source work and tests are separately owned; one evaluator serializes
Cargo. Parent owns Canon interpretation, integration and acceptance.

Retained time/reply checkpoint 55f1fd7f is pushed with verified parity. It
provides real successful/expiry reply replay rejection and initial-session
prewrite refusal; its full probe 41/41 and replay 3/3 are earlier evidence,
not provider-component reruns. Genuine G1-expiry→G2 rejection is local binder
evidence with verified network-code correspondence, not a cross-generation
network requester update. Older detailed counts remain in Report 2606.

### Dated LAB rough remaining estimates (not acceptance or guarantee)

The 2026-09-07 13:57 JST estimate was I3-3 12–24h, I3-4 6–12h,
I3-5 3–6h, I3-6 4–8h and NEXT-0 1–2h. It is a historical planning estimate,
not a current countdown. No new measured remaining-time estimate exists.
Provider admission/runtime coupling, negative-case review and broad validation
dominate uncertainty. Gates are not reduced to meet an estimate.

## ordered self-driven packages

The fixed sequence is unchanged. No later milestone starts merely because a
component test passes.

| Order | Milestone | Outcome | State / macro position |
|---:|---|---|---|
| 1 | ALIGN-0 | one roadmap, authority and regression floor | completed; Macro 0 front |
| 2 | ALIGN-1 | semantic/product/lifecycle three-axis map | completed; Macro 0 middle |
| 3 | ALIGN-2 | Browser/Host/package/View/provider boundaries | completed; Macro 1 front |
| 4 | I3-0 | transport comparison and private selection | completed; Macro 6 front |
| 5 | I3-1 | checked private codec/adapter boundary | completed; ADR-0038 |
| 6 | I3-2 | generated-artifact real process/network runtime | completed; ADR-0039, bounded FM-5 |
| 7 | I3-3 | twenty failure families and ordering refinement | active; Macro 3/5/6 middle, heavy |
| 8 | I3-4 | C-distributed gates and pressure slices | inactive; acceptance plus explicit resume required |
| 9 | I3-5 | joined observer-safe network workflow | inactive; dependency-gated |
| 10 | I3-6 | finite conformance and actual lifecycle close | inactive; dependency-gated |
| 11 | NEXT-0 | inactive I4/I5 entry contracts only | inactive reserve path; no implementation |

## I3-2 closed work-package inventory

Its deployment, independent locus startup, generated owner dispatch, bounded
launcher/reaping and joined correspondence are accepted regression inputs.
Their detailed inventory remains in Plan 250 / Report 2605. None is reopened
as an active queue by this snapshot.

## self-driven macro phase reading

| Macro | Current position / maturity | Self-drive boundary |
|---|---|---|
| 0 repository memory | three-axis/current-frontier discipline maintained | snapshot maintenance |
| 1 semantic kernel | finite I2 boundary; selected spec/17 contract | bounded I3 consumers only |
| 2 parser-free substrate | historical evidence | maintenance, not active architecture |
| 3 source/checker/runtime | provider source/static projection tested; actual provider execution absent | current I3-3 component |
| 4 samples | I2 toy/conformance reproducible; provider source-only | evidence-backed expansion |
| 5 theorem/model bridge | bounded model/runtime evidence; no new Lean claim | exact classifications |
| 6 distributed fabric | I3-2 accepted; I3-3 failure/order incomplete | through I3-3, then pause |
| 7 toolchain/backend | provisional developer commands | no public freeze |
| 8 applications | domain/library consumers, separate from fabric | no Core promotion; later owner scope |

Mirrorea fabric, Typed-Effect, PrismCascade, Browser/View and upper applications
remain separable. Browser/Host responsibilities are Canon-fixed; their safe
package runtime is not implemented. Shared-Space is a responsibility horizon,
not an addressing/governance/product specification. Reversed Library is separate.

## user decision gates

No semantic owner decision is currently required for bounded I3-3 implementation.
At 2026-09-08 19:08 JST, measured free disk is 30,906,707,968 bytes (~28.78
GiB) after the approved five-file cleanup and subsequent owner housekeeping.
The 10 GiB safety floor has no current capacity
hold; no inferred or broadened cleanup permission is used.
The following remain mandatory escalation boundaries, not autonomous options
to weaken the goal.

| Question | Impact | Options | Recommendation |
|---|---|---|---|
| North Star / authority / privacy / stale resurrection | whole semantics | preserve / change guarantees | preserve; stop for owner if impossible |
| Concrete domain Core primitive | language layering | library / Core promotion | keep domain/library |
| Hidden transaction or retry | atomicity and ambiguity | explicit / hidden | explicit only; stop if unavoidable |
| Irreversible public API/ABI/wire/package/FFI freeze | compatibility | provisional / freeze | remain private/provisional |
| Production/publication/paid or user-data risk | external systems/data | local evidence / external deployment | stay local; require authority |
| Non-migratable semantic tie | future meaning | Constitution choice / owner decision | escalate only the specified tie |

Theory T1, broad I1, general OBLs, undefined public formats, browser/product
choices and unoptimized performance are not themselves stop conditions.

## research discovery items

| Bounded question | Direct consumer | Required evidence / recommendation |
|---|---|---|
| Mixed legacy M8 and independent provider authorization | current provider Stage 2b | genuine binding, actual M9 verification/policy, retained component scope; no silent erasure or owner-grant reuse |
| Effect invocation and release/revocation | provider Stage 3/4 | actual host call and retained typed outcome; no retry/false nonexecution |
| Membership/redaction/cut with in-flight traffic | remaining I3-3 families | actual producer state and positive/falsifier; no distributed durability claim |
| Concrete-to-abstract ordering | I3-3 | request/serve/result/consume and authority/cut edges, not stream order |
| Official distributed scenarios | inactive I3-4/I3-6 | SCN-01/02/03/06 and exact source-first evidence |

Compare at most the current smallest design and one viable alternative.
No new WRK/report lane without a direct consumer and bounded closure.

## maintenance tasks

- Preserve Canon > LAB, source-first behavior and accepted I2/M10 regressions.
- Keep finite runtime evidence distinct from Lean/general proof and lifecycle.
- Keep source/Core/artifacts/authority separate from deployment/session/certificate.
- Keep provisional internal representations separate from public compatibility.
- Maintain one report (2606), one roadmap (250), one active milestone (I3-3).
- Run resources preflight before heavy commands. The latest regression gate
  measured 30,906,707,968 bytes (~28.78 GiB) free; keep the 10 GiB safety
  floor. No unmounted external workdir is assumed.
- Stage 1 lint/docs/diff/secret scan and commit/push/parity are complete;
  repeat applicable gates at the next integration. Full workspace/probe/matrix
  gates remain at I3-3 close.

## non-promoted references

Older plans/reports/WRKs, Product Alpha, Full System V1, historical provider
helpers and parser-free paths remain history or later consumers. They are not
current authority, a second queue, provider execution evidence or I3 lifecycle.
