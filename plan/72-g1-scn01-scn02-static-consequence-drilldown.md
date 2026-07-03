# plan/72 - G1 SCN-01/SCN-02 static consequence drilldown

## Purpose

This file is LAB repository memory. It maps the canon C-static expectations for
SCN-01 and SCN-02 to the G1 simple-assignment target in `plan/71`, then records
which LAB Surface elaboration evidence supports each row and which parts remain
open.

This file does not edit canon, does not claim C-static conformance, does not
claim G1 exit, and does not discharge THM-001 or any obligation.

In this file, "static consequence" means parse/check/elaborate-time generated
request / publish / observe / dependency / obligation / diagnostic inventory.
It does not mean runtime occurrence trace.

## Source hierarchy

- Normative source: `mirrorea_canon/`
- LAB evidence / history: legacy `specs/`, `plan/`, samples, helpers, reports,
  and Rust code outside `mirrorea_canon/`
- Advisory input: reviewer and Oracle results after local source review

LAB evidence is cited only as `LAB:` support. If LAB conflicts with canon,
canon wins.

## Shared C-static reading

| Canon item | Static consequence reading for this drilldown | Boundary |
|---|---|---|
| `spec/06-conformance.md` | C-static means parse/check/elaborate the SCN sources with expected edges, obligations, diagnostics, and spans. | This package does not claim any SCN pass count. |
| `theory/03-elaboration.md` BND-001 | Every generated request/publish/observe/witness/dependency consequence must be explicit and span-preserving. | This is trace target wording, not proof. |
| `theory/03-elaboration.md` THM-001 | Every assignment write is owner-local or an explicit owner-directed request with authority obligation, failure containment, dependency recording, and span mapping. | THM-001 remains open in `theory/11`. |
| `theory/01-mircore-v0.md` READ/WRITE/LOCUS-BLOCK/HANDLER | Reads, writes, nested locus blocks, and failure containment must stay separate. | No ambient authority from nested locus syntax. |
| `theory/02-types-effects-failures.md` | Generated failure rows must be contained; static errors are not generic runtime rejects. | Runtime failure behavior is out of scope here. |

## SCN-01 static consequences

SCN-01 source shape: `BrowserClient[self]` handles `roll(draw)` and contains
`World { player[self].position = player[self].position + draw }`. The field
`position` is visible as `observer_safe`.

| Row ID | Case family | Canon expectation | LAB support | LAB gap / boundary |
|---|---|---|---|---|
| `SCN01-SIMPLE-SCOPE` | simple assignment scope | The source is a simple assignment, not compound assignment. | `LAB:plan/71` scopes the first target to simple assignment. | No Lean statement yet. |
| `SCN01-CROSS-WRITE-REQUEST` | owner-directed write request | Non-owner ordinary assignment to `World` state yields a request edge from `BrowserClient[self]` to `World` for `write player[self].position`. | `LAB:ELAB-02` shows a nested foreign place write lowering to `request_kind = write`, requester `role:BrowserClient`, owner `S`, generated from `nested_place_block`; `LAB:ELAB-09` shows the same with visible communication. | LAB uses `S`, `hp`, and `target`, not exact `World`, `position`, and `self`; it is structural support, not SCN pass evidence. |
| `SCN01-RHS-READ-DEPENDENCY` | read dependency | The RHS read of `player[self].position` produces a dependency row for the same field. | `LAB:ELAB-11` records exact `World/player/self/position` RHS dependency evidence; `LAB:ELAB-02` and `LAB:ELAB-09` also now expose RHS dependency summaries for their write rows. | This is LAB evidence only, not a C-static conformance pass or proof discharge. |
| `SCN01-VISIBLE-PUBLISH` | visible publish / observe | Because `position` is visible, the write generates an observer-safe publish row and related observe consequence. | `LAB:ELAB-09` has `auto_publish`, `auto_observe`, `publication_summaries`, and `observation_summaries` for a visible write. | LAB `MessageEnvelope` is helper evidence only; canon target vocabulary is publish/observe/request. |
| `SCN01-FAILURE-CONTAINMENT` | static failure containment | Generated failures, including visibility-related failure, must be contained in the declared `fails` row. | `LAB:ELAB-09` has `failure_row_complete = true`; `LAB:ELAB-10` demonstrates underdeclared visibility failure rejection for a visible read/observe path. | Canon names the missing-visibility case as E-ROW-002, while LAB uses `generated_failure_not_declared`. There is no exact SCN-01 write-publish negative row for missing `VisibilityDenied`; use as partial support only. |
| `SCN01-CAP-OBLIGATION` | authority obligation | Obligations include write capability for `player`. | `LAB:ELAB-02` and `LAB:ELAB-09` carry remote write request evidence. | LAB elaboration rows do not prove the full canon capability theorem; G3 authority work remains separate. |
| `SCN01-SOURCE-SPAN` | static source mapping | All generated consequences preserve assignment/source spans. | `LAB:ELAB-05` and `LAB:ELAB-09` expose source-span entity kinds for generated requests/edges/publication/observation. | Span proof remains OBL-level work. |
| `SCN01-C-RUNTIME-BOUNDARY` | conformance-level boundary | After admission and `roll(3)`, store and occurrence order match SCN-01. | None in this package. | Runtime request serving, store update, request-before-serve-before-publish order, and distributed transport are out of scope. |

## SCN-02 static consequences

SCN-02 source shape: `BrowserClient[self]` handles `attack(target)` and
contains `S { player[target].hp = player[target].hp - player[self].atk }`.

| Row ID | Case family | Canon expectation | LAB support | LAB gap / boundary |
|---|---|---|---|---|
| `SCN02-SIMPLE-SCOPE` | simple assignment scope | The assignment to `player[target].hp` is the simple write target. | `LAB:plan/71` scopes the target to simple assignment. | The RHS is nontrivial and has two reads; do not collapse read obligations into the write row. |
| `SCN02-CROSS-WRITE-REQUEST` | owner-directed write request | The write to `S` state yields an owner-directed request to `S`, authorized from the actor locus. | `LAB:ELAB-02` shows positive nested write request; `LAB:ELAB-07` shows generated write request rejected when the failure row is underdeclared. | LAB field/key names differ and do not prove SCN pass. |
| `SCN02-RHS-TARGET-READ` | read dependency | The RHS read of `player[target].hp` is a dependency row. | `LAB:ELAB-12` records exact `target.hp` RHS dependency evidence. | This is LAB evidence only, not a C-static conformance pass or read-materialization policy. |
| `SCN02-RHS-SELF-READ` | read dependency | The RHS read of `player[self].atk` is a dependency row. | `LAB:ELAB-12` records exact `self.atk` RHS dependency evidence. | This is LAB evidence only, not a runtime read/freshness/observe policy. |
| `SCN02-FAILURE-CONTAINMENT` | static failure containment | Generated failure set is contained in declared `fails`; dropping `MissingCapability` yields E-ROW-001. | `LAB:ELAB-02` has `failure_row_complete = true`; `LAB:ELAB-07` has `failure_row_complete = false` and `generated_failure_not_declared`. | LAB diagnostic naming is alpha helper evidence, not canon diagnostic freeze. Record `generated_failure_not_declared` as a LAB alias/gap, not as the canon diagnostic id. |
| `SCN02-NESTED-LOCUS-NON-AUTHORITY` | nested locus block | Nested foreign locus block does not convert the actor into owner; generated request remains owner-directed and authorized from the actor locus. | `LAB:ELAB-02` and `LAB:ELAB-08` record `generated_from = nested_place_block` with requester `role:BrowserClient` and owner `S`. | This does not prove the authority theorem family; THM-004/G3 remains separate. |
| `SCN02-DIRECT-LOCAL-WRITE-REJECT` | static drift guard | An implementation that treats the nested `S` block as a local write fails C-static. | `LAB:ELAB-02` positive request shape supports the required edge shape. | No separate LAB negative row exists for a direct-local-write implementation. |
| `SCN02-C-RUNTIME-MISSING-CAP` | conformance-level boundary | Without write capability, runtime produces explicit `MissingCapability` and store stays unchanged. | None in this package. | C-runtime behavior and fail-closed store update are out of scope. |
| `SCN02-C-RUNTIME-STALE` | conformance-level boundary | Attack after target leave yields `StaleMembership`. | None in this package. | This belongs to runtime/lifetime/admission work, not the G1 static consequence draft. |

## Main LAB gaps before OBL-001

1. SCN-01 same-field RHS dependency is canon-required and now has LAB evidence
   in `ELAB-11` / `plan/75`.
2. SCN-02 two-read RHS dependency set (`target.hp` and `self.atk`) is
   canon-required and now has LAB evidence in `ELAB-12` / `plan/75`.
3. Canon diagnostic ids E-ROW-001 and E-ROW-002 are not the same as the current
   LAB helper diagnostic `generated_failure_not_declared`.
4. OPEN-014 leaves the final materialization policy for cross-locus reads open;
   the safe G1 wording is "dependency/read consequence is explicit," not a
   frozen transport/cache/projection shape.

## Evidence classification

| LAB row | Supports | Does not support |
|---|---|---|
| `ELAB-01` | Cross-locus read/observe request shape, failure containment, source spans. | Exact SCN-01 same-field dependency row or exact SCN-02 two-read dependency set. |
| `ELAB-02` | Nested foreign place write as owner-directed remote write request; non-ambient authority shape. | Visible publish, exact SCN field names, runtime capability behavior. |
| `ELAB-04` | Underdeclared generated failure rejection for read/observe. | Write failure containment or SCN-01 write-publish negative. |
| `ELAB-05` | Source span evidence for generated rows. | Proof of span preservation. |
| `ELAB-07` | Underdeclared generated failure rejection for write request. | Exact canon diagnostic id or runtime MissingCapability behavior. |
| `ELAB-08` | Nested place read as owner-directed read/observe request. | Exact SCN-02 target/self read pair. |
| `ELAB-09` | Visible write generates explicit publish/observe plus remote write request. | Runtime MessageEnvelope dispatch or final telemetry/viewer ABI. |
| `ELAB-10` | Visibility-related underdeclared failure rejection for visible read/observe. | Exact SCN-01 missing-VisibilityDenied write-publish negative. |
| `ELAB-11` | SCN-01-shaped same-field RHS dependency, visible write publish/observe, and owner-directed write request. | C-static conformance, runtime request serving, or proof discharge. |
| `ELAB-12` | SCN-02-shaped target/self RHS dependency pair and owner-directed write request without publish/observe materialization. | OPEN-014 read materialization, runtime MissingCapability behavior, or authority theorem. |

## Non-claims

- No C-static conformance claim.
- No C-runtime or C-distributed claim.
- No G0/G1/T1 exit.
- No theorem discharge.
- No Lean proof completion.
- No final grammar/API or Core JSON freeze.
- No runtime MessageEnvelope dispatch.
- No direct remote store.
- No ambient authority from nested `S { ... }`.
- No authority from role names, key values, provider names, transport, or
  package artifacts.
- No promotion of `World`, `S`, `Player`, `hp`, `position`, or `atk` into Mir
  core primitives.

## Open questions

- Should `THM001StatementDraft.lean` later mention the concrete LAB
  `rhs_indexed_read` carrier as evidence, or remain fully abstract until canon
  statement work?
- Does OPEN-014 require the SCN-02 read dependencies to be represented as
  observe/read requests, dependency-only rows, or both in the initial statement?
- Should an exact LAB evidence row be added later for SCN-01 missing
  `VisibilityDenied` on visible write publish, or is canon scenario text enough
  until the formal statement?

## Close condition

Close condition for this file is a plan-scoped static consequence map only. No
G1 exit, theorem/OBL discharge, C-static/C-runtime/C-distributed conformance
claim, runtime `MessageEnvelope` dispatch, public grammar/API freeze, or canon
semantic change is claimed.

## Next safe package

The immediate RHS dependency gaps now have LAB evidence in `plan/75`. The next
theory package should keep OBL-020/021 separate or refine the OBL-001 statement
draft without claiming proof discharge.
