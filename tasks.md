# Current Task Map (LAB)

最終更新: 2026-09-09 14:10 JST

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

**I3-3 is accepted and owner-paused.** Seven of eleven milestones are accepted. This is a
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

The current provider component implements ADR-0042/spec/17, building on accepted
source/static cuts9e8d674a/3862b168, inactive M9/M8 authorization7142b205 and
inactive process handoffa027d61. Historical counts and repairs stay in Report2606.

Stage3 now runs actual FD3-installed A/B children, source-derived QUIC provider
requests, bounded host reads, current-authority result consumption and separately
authorized normal v2 observation. Fresh row13 follow-up evidence passes runtime
library413, all probe targets (ordinary43/provider20), process68, M1067,
I2 local5/CLI8 and provider public guards5. Scoped all-target Clippy and format
pass; independent review has no remaining P0/P1/P2. The finite provider component
is committed/pushed at `94ad584577fadfd0b016ff798a22b84df536939f`; the row13
membership component is committed at `f6aae7ca277690ac558a08f124e414ae2d2d35ad`, with clean
HEAD/origin/main/live parity observed 2026-09-09 07:46 JST; this is not a new
milestone or official lifecycle change.

Row13 verifies source-derived WorldAuthority membership retirement with full-parent
M9 and qualified restricted-child G1→G2 coverage; old-G1 CarrierAdmissionRejected
is before owner use and LOCAL genuine M8 StaleMembership is separate. No observer
renewal, grant mint or session authority follows.

Retained provider fault evidence: the four actual two-session fault profiles use genuine child-private assertions
and generic completion, not exported provider-fault traces. The lost-result case
receives/decodes then discards the frame before semantic admission, not wire
packet loss. AdapterUnavailable has LOCAL injected-read/codec evidence, not
an actual OS operational-error claim. B64+A65 is not independent B65 or global
exactly-once. Supported live provider cut/export and public interfaces are not
claimed.

The current-to-next sequence is:

1. I3-3 is accepted at source/evidence cut
   `fe5dd972e2ddb3a513c785458a07702e4d4d99fa`, covering all twenty failure/order
   families with workspace 1573, runtime doctests 4, format, Clippy and final
   review P0/P1/P2=0. Preserve the finite boundary and historical ADR-0039 pause /
   ADR-0040 resume records.
2. Owner pause leaves no active semantic milestone. Plan 250 remains the sole
   retained current roadmap; I3-4/I3-5/I3-6/NEXT-0 remain dependency-gated
   inactive, and I3-4 requires explicit owner resume. Official I3 lifecycle entry
   remains unaccepted; Browser, world semantics and public/production claims are
   outside this cut.
3. No further I3-3 implementation is current. Reopen only for a reproducible
   accepted-profile falsifier or explicit owner resume; do not treat this pause as
   blocked, stale or program close.

Retained I3-3 execution practice: component consumers were not extra milestones
or parallel queues. Runtime source and tests had separate owners; one evaluator
serialized Cargo. Parent retains Canon interpretation, integration and acceptance.

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
| 7 | I3-3 | twenty failure families and ordering refinement | accepted; owner-paused |
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
The owner approved removal of the identified `target/debug` cache; at11:57JST
that exact cleanup recovered65G free. Source, .git and logs remain. The former
storage decision gate is resolved and bounded-debug Cargo validation resumes;
unrelated deletion or broader external actions are not inferred.
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
- Retain one report (2606), one roadmap (250), and no active semantic milestone
  during the owner pause; I3-4 requires explicit resume.
- Run resources preflight before heavy commands. The final validation checkpoint
  measured approximately63GiB free at2026-09-09 14:08 JST; keep the 10 GiB safety
  floor. No unmounted external workdir is assumed.
- Final source workspace/probe/matrix, doctest, lint and format gates passed;
  exact docs/diff/limited secret-scan and Git handoff evidence is in Report2606.
  Repeat applicable gates only for a new authorized change.

## non-promoted references

Older plans/reports/WRKs, Product Alpha, Full System V1, historical provider
helpers and parser-free paths remain history or later consumers. They are not
current authority, a second queue, provider execution evidence or I3 lifecycle.
