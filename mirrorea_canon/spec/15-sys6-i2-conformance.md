---
id: spec/15-sys6-i2-conformance
status: L1-fixed
maturity: reviewed
depends_on: [spec/11-m10-i1plus-conformance, spec/12-sys3-per-locus-projection, spec/13-sys4-in-process-generated-dispatch, spec/14-sys5-local-toy-devtools, adr/ADR-0032]
summary: SYS-6 finite source-first I2 producer/verifier、22-row inventory、typed rejection、observer-safe report、lifecycle evidence boundary。
open_items: []
---

# 15 — SYS-6 finite I2 conformance

## Profile and inputs

The finite profile is named `mirrorea-i2-systems-foundation-finite` and has
scope `bounded-finite-i2`. One invocation requires:

```text
primary_source: ordinary SYS-5 four-locus .mir source
selected_ow1_source: ordinary source eligible for exactly one OW1 worker locus
patches:
  one accepted designated-only candidate
  one rejected owner-RMW-changing candidate
format: observer-safe JSON
```

All sources enter through the existing parser/checker/elaborator. A path is a
host-boundary locator only and is never serialized or used as semantic
identity. Missing, invalid, incomplete, duplicated, or unexpected inputs
produce a typed redacted error and nonzero command status.

The provisional command form is:

```text
mir conform-i2 PRIMARY
  --selected-ow1-source SELECTED
  --patch ACCEPTED_PATCH
  --patch REJECTED_PATCH
  --format json
```

This is not public CLI or JSON compatibility.

## Producer/verifier separation

The producer performs these actual operations:

```text
load source content
  -> check/elaborate and bind checked-program identity to content
  -> deterministic SYS-3 projection and validator controls
  -> complete source-derived M9 admission
  -> SYS-5 project/workflow over SYS-4 generated endpoints
  -> separate selected ST and OW1 execution
  -> SYS-2 bounded ordering/model evidence
  -> actual negative-control candidates
  -> typed RawI2Evidence inventories
```

The verifier receives only `RawI2Evidence`. It must not parse, project, admit,
schedule, execute, select a route, consult expected output, or mint semantic
facts. Lower SYS-2--SYS-5 layers must not depend on the SYS-6 conformance
module. M10 conformance/release orchestration is outside this dependency path.

## Exact row inventory

An accepted report contains exactly these rows and no others:

| Row | Bounded scope | Evidence class |
|---|---|---|
| `i2.ordinary_source_authority` | primary source-first path and manual-interface rejection | runtime-monitored |
| `i2.checked_global_core_identity` | content-bound finite checked Core | runtime-monitored |
| `i2.core_to_locus_artifacts` | four-locus projection and owner-preservation subclaim | runtime-monitored |
| `i2.generated_communication_complete` | exact generated edge inventory | runtime-monitored |
| `i2.actual_dispatch_over_generated_edges` | actual ST endpoint lifecycle | runtime-monitored |
| `i2.st_ow_selected_correspondence` | separate exactly-one-worker selected source | runtime-monitored |
| `i2.owner_data_race_freedom_selected_backend` | worker-exclusive selected OW1 state | runtime-monitored |
| `i2.no_hidden_communication` | Core-derived edges only | runtime-monitored |
| `i2.no_direct_remote_store` | locus endpoint boundary | runtime-monitored |
| `i2.no_source_free_authority_mint` | bounded model plus actual runtime control | model-checked-bounded |
| `i2.no_source_free_state_mint` | source-admitted state only | runtime-monitored |
| `i2.failure_containment` | typed failure before mutation | runtime-monitored |
| `i2.visibility_redaction_preserved` | model visibility plus actual redaction control | runtime-monitored |
| `i2.relation_projection_coherence` | accepted two-anchor fallback fragment | runtime-monitored |
| `i2.semantic_presentation_fallback_separation` | presentation gap nonmutation | runtime-monitored |
| `i2.designated_evaluator_non_reexecution` | publication and named result delivery | runtime-monitored |
| `i2.source_core_artifact_trace_correspondence` | actual joined causal segment | runtime-monitored |
| `i2.save_restore_consistent_local_cut` | ST local cut and restore | runtime-monitored |
| `i2.patch_lifecycle_checked` | accepted/rejected bounded patch lifecycle | runtime-monitored |
| `i2.observer_safe_devtools` | single reference-only joined report | runtime-monitored |
| `i2.projection_determinism` | same-source repeated projection | runtime-monitored |
| `i2.non_claims_and_lifecycle_boundaries` | non-authorizing pre-acceptance candidate | runtime-monitored |

Every row must carry a nonempty bounded scope, one of the five Canon evidence
class labels, executed positive and falsifier references, explicit controls,
and an actual provenance anchor whose domain and producer are permitted for
that property. Owner preservation is an explicit subclaim on the artifact
row. Optional Core/artifact/edge/request/occurrence fields must either join an
actual inventory member or carry a typed not-applicable reason; invented
property references are forbidden.

## Selected ST/OW1 boundary

The primary four-locus toy is the complete ST workflow. A separate ordinary
selected source establishes the finite OW1 comparison. Both selected backend
executions must complete their generated dispatches and agree in typed result,
state digest, frontier, and trace digest. OW1 telemetry must show one worker-
owned M8 runtime and mailbox FIFO evidence.

The primary four-locus toy has multiple combined owner/source-owner loci and
therefore retains a typed `BackendIneligible` residual for whole-workflow OW1.
This residual mutates no state and cannot be hidden or counted as a positive
whole-toy OW1 claim.

## Identity and correspondence

Checked-program identity includes source content, not only the logical path.
Artifact and manifest identities change when same-logical-name content
changes. Same content under the same logical basename is independent of the
host directory. The I2 manifest uses an I2-specific domain and must not equal
or reuse the accepted M10 implementation cut.

Rows cross-join these actual inventories where applicable:

```text
checked-program identities
Core refs
artifact refs
generated communication-edge refs
request identities
runtime occurrence refs
property-specific provenance anchors
executed positive and falsifier evidence
source-first causal provenance
```

Request identity is distinct from queue position and every occurrence.
Transport, worker, profile, report, and receipt identity remain non-authority.

## Fail-closed controls

The fixed test-only controls cover at least:

- missing or inserted non-derived communication edge;
- owner-operation movement and broken source map;
- manual route/interface admission;
- source-free authority or state mint and direct remote-store mutation;
- selected ST/OW typed-result, state, frontier, or trace divergence;
- offline cut corruption and checked-patch evidence failure;
- wrong diagnostic substitution or unexecuted bound evidence;
- missing required property provenance anchor;
- observer-sensitive primary/selected-source identifiers; and
- lifecycle overclaim without runtime mutation.

Each control changes a cloned candidate or attempts the real guarded endpoint.
The verifier checks the observed diagnostic, producer invocation, before/after
identity or state, nonmutation where required, and exact affected row. A
registered but unexecuted control cannot have a detected outcome and cannot
support acceptance. Test-only corruptions are not exposed through source or
CLI.

## Observer-safe report

The report schema is internally versioned. Its only external materialization
clones and redacts immediately before serialization. It may expose safe opaque
references, fixed row labels/scopes/classes, typed diagnostics, selected
backend summaries, and explicit non-claims. It must not expose host paths,
source text, credentials, raw capability/witness material, private values,
raw M8/M9 identity, or source-controlled sensitive identifiers.

An accepted report has `status = accepted`, 22 passing rows, and
`public_api_or_wire_contract = false`, `final_public_api_frozen = false`, and
`public_wire_frozen = false`. A rejected report remains typed and causes the
CLI to exit nonzero.

## Lifecycle evidence semantics

The runtime report always evaluates a non-authorizing lifecycle candidate. In
normal evidence it records broad I1, I2 entry/exit, I3, transport, production,
and public-transport claims as false. A test-only overclaim candidate must be
detected without changing runtime state.

These bits describe the producer's authority boundary, not the current Canon
lifecycle after an ADR acceptance. ADR-0032 separately evaluates the completed
evidence and applies official I2 entry then exit. The command remains unable
to self-authorize that change and may continue to list `I2 lifecycle exit` as
a report-local non-claim.

## Evidence boundary and non-claims

OBL-063 classifies the aggregate profile at cut
`5429712de89a7e41c46cfd7fb4a39c4a492864c4` as `runtime-monitored`. Its
no-source-free-authority row references existing bounded-model evidence; the
aggregate profile is not thereby a general or whole-profile model proof.

This specification does not accept broad PHASE-I1, move theory T1, define a
public CLI/API/ABI/JSON/artifact/wire, select real transport, activate I3,
claim four-locus whole-workflow OW1, durable/distributed persistence,
browser/View product, production, arbitrary relation DAG/scheduler/fairness,
or general projection/communication/data-race/authority/save/patch/
noninterference theorem.
