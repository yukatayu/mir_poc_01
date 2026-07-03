# plan/69 — consultation synthesis and management roadmap

## purpose

This document is non-normative repository memory.

It records the distilled reading from the provided consultation conversation and
the current repo snapshot. It does not create a new `specs/` decision, does not
promote a package, and does not decide whether work should move to a separate
theory/design repository.

Normative truth remains in `mirrorea_canon/`. Current status remains in
`progress.md` and `tasks.md`. Reports remain the evidence trail.

## source-hierarchy reading

The consultation is useful as strategy input, but it must be reconciled with the
repo's existing source hierarchy:

- `mirrorea_canon/` is the normative source.
- legacy `specs/` and `plan/` are LAB evidence / long-lived repository memory.
- `progress.md` and `tasks.md` are current snapshots.
- `docs/reports/` records work evidence and reasoning history.
- sample dashboards and helper outputs are evidence, not completion.

Therefore, consultation-derived ideas enter this repo first as `plan/` memory
and report evidence. They become normative only through the canon process:
proposal, human decision where required, canon file / ADR / changelog update,
and index regeneration.

## current status after readthrough

The current repo is active, not abandoned history.

- No Surface package is currently promoted after `P-SURF-99`.
- Product Alpha-1 and the operational suite remain bounded alpha floors, not
  final public product.
- Full System V1 remains bounded source-first evidence, not final grammar,
  final ABI, or final distributed runtime.
- Surface Mir alpha is audit-closed as bounded evidence for parser,
  indexed-state checking, elaboration, generated communication, role admission,
  source patch, source operational rows, static devtools diagnostics, and final
  non-claim audit.
- `.mir` source files are the Surface semantic authority; `package.mir.json` is
  an alpha compatibility / package artifact.
- Final runtime/transport, final source patch ABI, final viewer/telemetry ABI,
  final public grammar/API, production WAN/federation, distributed durable
  save/load, native/WASM admission, and final engine adapter ABI remain later
  gates.

The main management gap is not "no work exists"; it is that many closed evidence
lines now obscure the next strategic question.

## distilled project axis

Working axis for discussion:

```text
正しい理論に基づき、source-level に自然に書かれた仮想空間的 system の意味を、
locus、state、dependency、authority、effect、observation、cut、projection の理論で保存しながら、
複数の実行場所へ展開し、後から安全に拡張できるようにする。
```

This is a management formulation, not a replacement for `specs/01`.

Important non-axis boundaries:

- `World`, `Room`, `Avatar`, `Game`, and `Reversed Library` are domain/library
  vocabulary, not Mir core primitives.
- `Event` is not the primary surface programming model.
- communication/API boundaries are derived projection artifacts, not the
  starting design object.
- providers, renderers, engines, runtime kind, transport, and role names are not
  semantic owners or authority.
- hot-plug is not direct eval.
- SysML v2 / Capella may help later as traceability or architecture views, but
  they are not the current normative semantics.

## semantic strata to keep separate

Use these strata as a planning aid:

| Stratum | Role | Main danger |
|---|---|---|
| `S0 Surface Intent` | ordinary source code: assignment, read, state declaration, fallback, capability requirement | leaking trace/event machinery into ordinary code |
| `S1 Core Elaboration` | explicit owner-directed write, generated communication, effect boundary, failure row, capability/witness obligation, source span | hiding generated obligations |
| `S2 Trace / Occurrence` | execution history, causal dependency, occurrence rows, cut/save-load interaction, devtools trace | confusing domain event with semantic occurrence |
| `S3 Verification / Obligation` | static check, model check, theorem proving, runtime monitor, contract boundary | turning Surface Mir into a proof-assistant surface |
| `S4 Projection / Deployment` | per-locus/per-node/per-provider artifacts, packet/FFI schema, provider/view boundary | treating projection as optimal codegen or final ABI |
| `S5 Domain / Library` | World, Room, Avatar, Portal, game logic, upper applications | smuggling sample vocabulary into core |

The consultation's strongest useful correction is that ordinary programming
should stay ordinary at `S0`, while `S1..S4` must make the hidden obligations
explicit and diagnosable.

## first theory target candidate

The strongest candidate next theory target is ordinary assignment elaboration,
not hot-plug.

Candidate target obligation:

```text
If a Surface assignment elaborates successfully,
then every generated write is either owner-local
or represented as an explicit owner-directed request/effect
whose authority obligation, failure row, dependency, and source span are preserved.
```

Reason:

- It directly tests the project origin: ordinary source should generate the
  correct lower-level communication/effect/diagnostic structure.
- It forces read/write, ownership, authority, failure, trace, and projection
  boundaries to become explicit.
- It avoids starting with hot-plug before the state/authority/cut substrate is
  crisp.

Status: candidate only. It is not yet a promoted package or discharged theorem.

## read / write interpretation

Current management reading:

- write produces an occurrence / trace-visible state change.
- read is source-transparent, but Core / verification / devtools may record a
  dependency.
- local reads need not all materialize as trace events.
- cross-locus, audited, public-observer, provider, or save/load-relevant reads
  may materialize as observe/request/effect/audit rows.
- diagnostics must return generated obligations to Surface source spans.

This suggests keeping "read is dependency" and "write is occurrence" as the
default mental model, while preserving explicit effect rows where a boundary is
crossed.

## four-graph management model

The consultation repeatedly converges on four related graph families:

| Graph family | What it tracks | Repo-aligned caveat |
|---|---|---|
| occurrence / history DAG | writes, sends/receives, publish/observe, witness, grant/use, patch activation, cut attempts | do not expose all of this as ordinary source vocabulary |
| state / existence DAG | state/object validity, parent-child existence, lifetime, fallback lineage | avoid making `World` an absolute root |
| locus / admission graph | loci, places, participants, membership, role claims, capability grants, witness/freshness | role/key/locus/provider/transport are not authority |
| patch / overlay DAG | source patch dependency, compatibility, activation, migration/rollback stop lines | source patch is not direct eval |

This is a planning model, not a new normative graph taxonomy.

## fallback and lifetime

Fallback is central to the virtual-space direction, but must remain monotone:

- fallback is availability/degradation, not authority strengthening.
- read fallback and write fallback must be separated.
- hidden re-promotion is not allowed; returning to a stronger target requires
  explicit reacquire / witness / freshness.
- load/rollback must not resurrect expired leases, stale membership, stale
  witnesses, stale capability provenance, or degraded fallback positions.

The useful next wording should prefer `Locus / State / Dependency / Capability /
Witness / Patch / Cut` over `World Semantics Core`, because `World` is relative
and library-level.

## authority principles to preserve

Keep these as drift guards:

- role claim is not authority.
- indexed-state key is not authority.
- locus name / apparent location is not authority.
- runtime kind, transport, provider name, package name, and engine brand are not
  authority.
- capability grant and admission/witness lineage carry authority.
- stale authority is rejected, including after load/rollback.
- authentication / authorization / membership / capability / witness must not be
  collapsed into transport.

## projection principles

Projection should be treated first as semantic preservation, not optimal
placement or final codegen.

It must preserve or expose:

- state ownership.
- read/write dependency.
- effect and failure rows.
- capability and witness requirements.
- visibility, redaction, and retention.
- fallback lineage.
- cut/save-load obligations.
- provider non-ownership.
- source-span diagnostics and devtools mapping.

Generated communication boundaries are artifacts derived after the source meaning
is checked.

## hot-plug position

Hot-plug remains central, but it is a capstone correctness operation:

- patch source must pass parse/check/elaborate/compatibility/admission before
  runtime mutation.
- rejected patches must not mutate active runtime state.
- accepted patches need activation-cut visibility and devtools/save-load trace.
- patches must not self-grant authority, underdeclare generated communication,
  weaken failure/visibility/redaction, or rewrite finalized prefixes.

This does not erase the existing P-SURF-06 source-patch alpha evidence. It only
says that a future theory recut should not start by redefining hot-plug before
ordinary read/write and authority are crisp.

## management objects

The consultation recommends ledger-like management. In this repo, ledgers should
not become a parallel normative authority unless explicitly designed.

Safe introduction path:

1. Keep this `plan/69` as the synthesis memory.
2. If useful, create plan-scoped or generated non-normative inventories:
   concept ledger, claim ledger, open-problem ledger, ADR candidate list,
   diagram list, minimal-slice outline.
3. Promote any actual normative change through `mirrorea_canon/` according to
   ADR-0012 and the canon operating model.
4. Mirror current state only in `progress.md` and `tasks.md`.
5. Use `plan/70-lab-to-canon-reconciliation-ledger.md` to map legacy LAB claim
   families to canon anchors, rejected historical claims, or OPEN follow-up.

Candidate ledger records:

- Concept: `Locus`, `State`, `Dependency`, `Occurrence`, `Effect`, `Contract`,
  `Failure`, `Capability`, `Witness`, `Freshness`, `Lifetime`, `Fallback`,
  `Patch`, `Cut`, `Projection`, `Observation`, `Provider`, `View`.
- Claim: Surface assignment elaboration preserves authority and failure
  obligations.
- Open problem: transparent cross-locus read materialization policy.
- ADR candidate: `World` is not primitive.
- ADR candidate: `Event` is not the primary surface programming model.
- ADR candidate: provider is not semantic owner.
- ADR candidate: patch is not direct eval.

## freeze-gate candidate sequence

Candidate discussion sequence:

| Gate | Goal | Do not claim |
|---|---|---|
| `Gate 0 Axis` | axis/non-axis, core vs library vocabulary, source authority | final public product direction settled |
| `Gate 1 Assignment` | ordinary assignment, transparent read, write occurrence, owner-local vs owner-directed write | full distributed runtime |
| `Gate 2 Existence/Fallback` | existence dependency, lifetime/lease/freshness, monotone fallback, no hidden re-promotion | full dependent lifetime theory |
| `Gate 3 Authority` | role/key/provider/transport non-authority, grant/witness lineage | production identity/auth stack |
| `Gate 4 Effect/Observation` | typed effect boundary, observer-safe/admin-debug split, redaction/retention | final viewer/telemetry ABI |
| `Gate 5 Cut/Save-Load` | cut consistency, SaveObject contents, no stale resurrection, local-vs-distributed split | R3/R4 durable distributed save/load |
| `Gate 6 Projection` | preservation targets, generated communication, provider non-ownership | optimal placement/codegen/final ABI |
| `Gate 7 Hot-Plug` | patch pipeline, activation cut, rejection no-mutation, save/load/devtools lifecycle | final hot-plug ABI or migration engine |

These are planning gates, not current promoted packages.

## candidate minimal vertical slice

A future minimal slice should be small and semantic-first:

- parent locus and child locus.
- child state depending on parent state.
- ordinary assignment across local and cross-locus cases.
- capability-missing rejection.
- transparent read that records dependency.
- one fallback path.
- one accepted patch and one rejected patch.
- projection generating communication boundary evidence.
- devtools explaining source span, generated Core, dependency, failure row,
  fallback degradation, and patch lifecycle.
- save/load refusing stale resurrection.

Do not include real networking, renderer, physics, asset loading, arbitrary
provider execution, native/WASM, production distributed persistence, or full
avatar/portal systems in the first minimal slice.

## current open questions

These need user or future-package decisions before promotion:

- Should the next phase stay in this repository, or should a separate
  theory/design repository be created? If separate, which source is normative?
- Is the next promoted line docs-only theory recut, implementation, or a
  management-ledger package?
- Should ordinary assignment elaboration correctness become the first new target
  package?
- What decision level should be assigned if `World is not primitive` and `Event
  is not the primary surface model` become normative?
- Should machine-readable ledgers be introduced, and if so under `plan/`,
  `.docs/`, `docs/`, or generated output?
- What sample vocabulary should be used for the first minimal slice without
  smuggling `World` / `Avatar` into core?
- Should SysML v2 / Capella remain postponed, or should a small non-normative
  spike be tracked later?

## recommended immediate next prompt

Ask for a strategy package that does only the following:

```text
Create the first non-normative planning ledger for:
axis/non-axis,
semantic strata,
ordinary assignment target obligation,
and open questions for promotion.
Do not edit specs yet.
```

That keeps momentum while avoiding premature normative commitments.
