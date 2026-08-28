---
id: meta/proposal-035
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, root/design-constitution, adr/ADR-0026, adr/ADR-0028, adr/ADR-0029, adr/ADR-0030, adr/ADR-0031, arch/03-toolchain, arch/04-runtime-carriers, theory/11-metatheory-ledger, spec/12-sys3-per-locus-projection, spec/13-sys4-in-process-generated-dispatch, spec/14-sys5-local-toy-devtools]
summary: SYS-6のsource-first finite I2 conformance profileをcut 5429712dで受理し、broad I1を残したままofficial I2 entry/exitを適用する提案。
open_items: []
---

# PROPOSAL-035 — SYS-6 finite I2 conformance and lifecycle closeout

## Owner disposition and selected capability

Under ADR-0026, accept the smallest source-first assurance layer that verifies
the already accepted SYS-3--SYS-5 capability without becoming its runtime
architecture:

```text
ordinary primary .mir source + two checked patch sources
  + one separate selected-OW1 .mir source
  -> actual checking / projection / admission / execution / bounded model
  -> fixed finite evidence inventories
  -> independent 22-row verifier
  -> deterministic observer-safe conform-i2 report
```

The selected implementation/evidence cut is
`5429712de89a7e41c46cfd7fb4a39c4a492864c4`. The provisional command is
`mir conform-i2`. Its spelling, arguments, JSON schema, row identifiers, and
hash representation are internal finite-profile surfaces, not public API,
ABI, artifact, or wire commitments.

The producer consumes the real SYS-2 bounded model and the checked/projected/
executed SYS-3--SYS-5 objects. The verifier consumes only the producer's typed
evidence inventories and fixed predicates. Neither side imports the M10
reference-system facade as a semantic source, and no verifier result, expected
JSON, source path, fixture name, release hash, or row declaration can add a
route, Core fact, authority, state, occurrence, or successful verdict.

## Fixed finite assurance inventory

The accepted profile has exactly 22 property rows:

```text
ordinary source authority
checked global Core identity
Core -> locus artifacts
generated communication completeness
actual dispatch over generated edges
selected ST/OW1 correspondence
selected-backend owner data-race freedom
no hidden communication
no direct remote store
no source-free authority mint
no source-free state mint
typed failure containment
visibility/redaction preservation
relation projection coherence
semantic/presentation fallback separation
designated evaluator non-reexecution
source/Core/artifact/trace correspondence
local save/restore consistent cut
checked patch lifecycle
observer-safe devtools
projection determinism
non-claims and lifecycle boundary
```

Every accepted row has a bounded scope, an exact evidence class, at least one
executed positive reference, at least one executed representative falsifier,
and a property-specific actual provenance anchor. References cross-join the
actual checked-program, Core, artifact, communication-edge, request,
occurrence, lifecycle, model, and executed-control inventories. Missing or
failed evidence, a substituted diagnostic, an absent property anchor, or a
row-inventory mismatch rejects the profile; omission is not success.

The no-source-free-authority row uses the accepted bounded model together
with an actual runtime endpoint control and is classified
`model-checked-bounded`. The other 21 rows are `runtime-monitored`. In
particular, selected-backend owner data-race freedom is a worker-exclusive
runtime observation, not a general data-race theorem. The accepted report
therefore advertises only `model-checked-bounded` and `runtime-monitored`.

The four-locus toy remains an ST whole-workflow scenario. OW1 correspondence
uses a separate ordinary four-locus source whose combined semantic owner/
source-owner set has exactly one worker locus. The profile does not claim that
the four-locus toy as a whole is OW1-eligible; its typed
`MultipleCombinedOwnerSourceOwnerLoci` residual remains visible and
non-mutating.

## Identity, observer safety, and controls

The profile identity is I2-namespaced and content-bound. Same content with the
same logical basename is independent of the host directory; changed content
changes checked-program, artifact, and I2 manifest identity. The manifest is
not the M10 accepted implementation cut and is not a public runtime identity.

The only CLI serialization path applies observer redaction before JSON
materialization. Host paths, source text, credentials, raw capabilities,
witness payloads, private state, and observer-sensitive source-controlled
identifiers cannot escape. The report retains only typed opaque references
and declared safe summaries. A marker-bearing primary or selected-OW1 source
is rejected or redacted through actual producer/output controls.

Representative controls include missing and non-derived communication edges,
moved owner operations, broken source maps, manual route/interface admission,
direct remote-store mutation, source-free authority/state mint, selected
ST/OW typed-result/state/frontier/trace divergence, offline cut corruption,
wrong bound diagnostics, unexecuted evidence, missing row-specific provenance,
observer leakage, and lifecycle overclaim. Corruption hooks are test-only and
are not a CLI or source capability.

## Lifecycle audit and acceptance proposal

The `conform-i2` producer deliberately materializes a pre-acceptance lifecycle
candidate: broad I1 exit, I2 entry/exit, I3 activation, transport, and
production bits are false. Its `I2 lifecycle exit` non-claim means the
runtime/verifier cannot authorize its own lifecycle transition. It does not
mean that Canon can never accept the transition after reviewing the evidence.

The lifecycle audit yields two separate results:

1. **Broad PHASE-I1 remains unaccepted.** Architecture/04 remains L2-working;
   OPEN-026 field/IR exchange, OPEN-027 external delivery observability, and
   the full internal carrier freeze remain unresolved. Public compatibility
   freeze is not manufactured to close them.
2. **The ADR-0025 I2 entry contract and the existing I2 exit criteria are now
   satisfied.** ADR-0026 supplies the owner-directed roadmap and bounded
   non-public carrier lane; SYS-3 derives artifacts and communication from
   checked Core; SYS-4 actually dispatches them between independent in-process
   locus runtimes; SYS-5 supplies the minimal typed causal view and local toy;
   and this SYS-6 profile independently verifies their bounded positive and
   negative cases.

Therefore the authorized acceptance record may apply, in order, official I2
entry and official I2 exit while retaining theory T1 and broad PHASE-I1 as an
exact residual. This is not a public/product completion claim and does not
activate I3.

## Evidence, direct consumer, and stop condition

Fresh evidence at the accepted cut includes 25/25 library conformance tests,
8/8 CLI tests, preserved SYS-2 28/28, SYS-3 28/28, SYS-4 104/104, SYS-5 62/62,
M10 conformance 67/67 plus CLI 4/4, the complete workspace test run,
formatting, warnings-denied Clippy, diff validation, and a final independent
assurance/lifecycle review result of ACCEPT. OBL-063 classifies the aggregate
finite implementation bridge as `runtime-monitored`; it reuses OBL-058's
bounded model but adds no Lean or general theorem status.

```text
Direct consumer: SYS-7 inactive I3 goal and entry contract
Blocker reduced: accepted generated projection/dispatch/toy evidence lacked one
  fail-closed, source-first, classified I2 acceptance surface and lifecycle audit
Acceptance use: pin the exact bounded I2 boundary from which future transport
  requirements may be stated without treating transport as authority
```

Close SYS-6 at this cut. Make SYS-7 the sole active milestone. Reopen only if a
listed falsifier passes, a row can pass without actual bound evidence, a
source-controlled identifier escapes redaction, lower SYS layers depend on
the conformance aggregator, the selected ST/OW observations diverge, the M10
regression fails, or lifecycle acceptance is later shown not to satisfy the
pre-existing I2 criteria.

## Non-effects

This proposal does not accept broad PHASE-I1; change theory T1 or general
OBL/THM status; freeze final Surface, CLI, JSON, API, ABI, artifact, or wire;
select or implement real transport; activate I3; claim four-locus whole-
workflow OW1, arbitrary scheduler fairness, arbitrary relation DAG, durable
distributed save/load, general exactly-once/noninterference/projection/
authority/data-race theorem, browser/View/renderer product, production
deployment, or public completion.
