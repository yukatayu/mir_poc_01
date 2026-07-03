# tasks

最終更新: 2026-07-04 07:22 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins; cite legacy `specs/` /
`plan/` as `LAB:` evidence unless mirrored into canon.

## document role

This document is the repo-wide **current task map**. It is not normative source
and is not append-only history.

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: `plan/`, legacy `specs/`
- Status snapshot: `progress.md`
- Runnable dashboard: `samples_progress.md`
- Execution evidence: `docs/reports/`

## current promoted package

No current promoted Surface package after `P-SURF-99` closeout.

Current holding state:

- `mirrorea_canon/` is the canon-first source for direction, theory, ADRs,
  conformance, and process. Existing LAB evidence remains useful, but does not
  override canon.
- Canon phase reading is T0/G0 rebaseline. The immediate LAB-to-canon claim
  family ledger now exists at `plan/70-lab-to-canon-reconciliation-ledger.md`;
  it is LAB evidence, not G0 exit.
- Surface alpha `P-SURF-01..08` evidence rows remain runnable through
  `scripts/surface_mir_samples.py`.
- `P-SURF-99` reran full Surface validation and Product Alpha compatibility
  anchors.
- P-SURF-08 devtools diagnostics remain static source/Core evidence, not final
  viewer / telemetry ABI or runtime devtools completion.
- Post-`P-SURF-99` consultation synthesis is captured in
  `plan/69-consultation-synthesis-and-management-roadmap.md` as non-normative
  repository memory. It does not promote a new package or create a `specs/`
  decision.
- High-risk legacy LAB claim families are now mapped in `plan/70` to canon
  anchors, rejected historical claim patterns, or OPEN follow-up. The next
  safe package should stay on G1 ordinary assignment, not runtime widening.
- `plan/71-g1-ordinary-assignment-target.md` now drafts the LAB-only G1
  ordinary simple-assignment target/proof-boundary split. It does not claim G1 exit,
  theorem discharge, Lean proof completion, runtime MessageEnvelope dispatch,
  or final public grammar/API freeze.
- `plan/72-g1-scn01-scn02-static-consequence-drilldown.md` now maps SCN-01/SCN-02 C-static
  expectations to canon target rows, LAB support, LAB gaps, and runtime/proof
  boundaries. It does not claim C-static conformance or G1 exit.
- `plan/73-g1-obl001-lean-statement-inventory.md` now inventories the minimum
  datatypes, predicates, theorem-shape split, SCN coverage rows, and overfit
  guards needed before writing an actual repo-local OBL-001 Lean statement. It
  adds no Lean statement file and does not move canon OBL status.
- `plan/74-g1-obl001-lean-statement-draft.md` now records a LAB-only
  repo-local Lean statement-shape draft at
  `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`. It compiles
  as a `Prop` definition and does not move canon OBL status, prove THM-001,
  claim G1 exit, or edit canon.
- `plan/75-g1-scn-rhs-dependency-gap-evidence.md` now records LAB-only
  `ELAB-11/12` dependency evidence for SCN-01 same-field RHS and SCN-02
  target/self RHS reads. It does not claim C-static conformance, runtime read
  materialization, proof discharge, G1 exit, or canon movement.
- `plan/76-g1-obl020-021-dependency-inventory.md` now separates OBL-020
  well-formedness-preservation dependencies and OBL-021 elaboration-determinism
  dependencies from OBL-001/002. It is inventory-only and does not claim either
  obligation complete, Lean statement status, proof skeleton completion, G1
  exit, T1/T2 transition, conformance, or canon movement.
- `plan/77-g1-obl021-lean-statement-draft.md` now records a LAB-only
  repo-local OBL-021 Lean statement-shape draft at
  `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`.
  It compiles as a `Prop` definition and does not move canon OBL status, prove
  elaboration determinism, claim G1/T1/T2 exit, or edit canon.
- `plan/78-g1-obl020-lean-statement-draft.md` now records a LAB-only
  repo-local OBL-020 Lean statement-shape draft at
  `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`. It compiles
  as an aggregate `Prop` over abstract `WellFormed` / `Step` and does not move
  canon OBL status, prove WF preservation, claim proof skeleton completion,
  G1/T1/T2 exit, conformance, runtime implementation proof, or edit canon.
- `plan/79-g1-erow-diagnostic-alignment.md` now records LAB-only alignment for
  canon E-ROW-001/E-ROW-002 versus current `generated_failure_not_declared`
  evidence. It does not freeze diagnostic ABI, discharge OBL-024/025, claim
  conformance, claim G1 exit, or edit canon.
- `plan/80-g1-diagnostic-carrier-inventory.md` now inventories canon
  Diagnostic carrier fields against current LAB `code/message/span`,
  helper `diagnostic_codes`, remote request summaries, and source-span sidecar
  evidence. It does not implement diagnostic ABI, state/prove OBL-024/025,
  claim explanation soundness/completeness, claim conformance, or edit canon.
- `plan/81-g1-obl024-statement-shape-inventory.md` now inventories the
  statement shape for OBL-024 explanation soundness: emitted Diagnostic,
  reported rule instance / failed premise / bindings, and replay failure
  exactly at that premise. `plan/109` now adds the LAB-only compile-check
  statement draft; this inventory remains the pre-draft relation map. It does
  not prove OBL-024, freeze diagnostic ABI, claim conformance, claim G1 exit,
  or edit canon.
- `plan/109-g1-obl024-lean-statement-draft.md` now records a LAB-only
  repo-local OBL-024 Lean statement-shape draft at
  `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`.
  It compiles as a diagnostic projection / association-key / reported failed
  premise / trace-local replay `Prop` with non-repair mixed diagnostic branch
  boundary predicates. It does not define final Diagnostic ABI, JSON keys,
  request IDs, branch IDs, association-key ABI, replay semantics, prove
  OBL-024, move canon ledger status, claim root-cause uniqueness, claim
  conformance, claim G1 exit, or edit canon.
- `plan/82-g1-obl025-statement-shape-inventory.md` now inventories the
  statement shape for OBL-025 explanation completeness: Line-1 rejection,
  declared fragment, single-edit repair existence, non-empty suggested repair,
  and repair/failure matching. It does not add a Lean file, generate repairs,
  prove OBL-025, freeze diagnostic/repair ABI, claim conformance, claim G1
  exit, or edit canon.
- `plan/83-g1-erow-repair-payload-inventory.md` now inventories non-final
  E-ROW repair payload roles for a later prototype that may include
  `suggested_repair[]`. It does not implement repair generation, freeze
  diagnostic/repair ABI, prove OBL-024/025, claim conformance, claim G1 exit,
  or edit canon.
- `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md` now records
  the LAB-only E-ROW diagnostic detail carrier implementation. It preserves
  legacy `generated_failure_not_declared` output, adds non-final
  `lab_diagnostic_details` for E-ROW-001/E-ROW-002 classification and missing
  evidence, emits no `suggested_repair[]`, and does not freeze diagnostic ABI,
  prove OBL-024/025, claim conformance, claim G1 exit, or edit canon.
- `plan/85-g1-erow-carrier-precondition-hardening.md` now records LAB-only
  request and failure-row context inside `lab_diagnostic_details`. It exposes
  generated request identity, target row kind, required/declared/missing
  failures, and local premise for `ELAB-04/07/10`, emits no
  `suggested_repair[]`, and does not freeze diagnostic/repair ABI, prove
  OBL-024/025, claim conformance, claim G1 exit, or edit canon.
- `plan/86-g1-erow002-visibility-repair-carrier-prototype.md` now records
  LAB-only `suggested_repair` evidence for the `E-ROW-002` / `VisibilityDenied`
  row-containment failure shape represented by `ELAB-10`. `ELAB-04` remains
  no-repair mixed evidence; `ELAB-07` later uses the exact `plan/102`
  `E-ROW-001` set path and is outside this visibility-carrier package. It does
  not freeze diagnostic/repair ABI, prove OBL-024/025, claim explanation
  completeness, claim conformance, claim G1 exit, or edit canon.
- `plan/87-g1-obl025-lean-statement-draft.md` now records a LAB-only
  repo-local OBL-025 Lean statement-shape draft at
  `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`.
  It compiles as an existential repair-coverage `Prop` over abstract
  predicates. The current refinement adds whole-rejected-gap, set-insertion,
  grouped multi-edit, complete local repair, partial-guidance non-coverage, and
  branch-local non-coverage predicates / helper relations to keep
  set-insertion, bundles, partial guidance, and `ELAB-04` branch-local guidance
  from being overread as current whole-gap coverage. It does not freeze
  diagnostic/repair ABI, prove OBL-025, claim explanation completeness, claim
  conformance, claim G1 exit, or edit canon.
- `plan/88-g1-erow-repair-shape-inventory.md` now records the LAB-only
  taxonomy for repair output widening. Current singleton repair evidence is
  `ELAB-10` for `E-ROW-002` / `VisibilityDenied` and `ELAB-13..16` for
  `E-ROW-001` non-visibility base failures. Exact `ELAB-07` has one non-final
  `set_insertion` item under `plan/102`; `ELAB-04` remains no-repair. This
  does not prove OBL-025, claim repair ranking/multi-edit support, claim
  conformance, claim G1 exit, or edit canon.
- `plan/89-g1-erow001-non-visibility-singleton-fixture.md` now records a
  LAB-only `ELAB-13` fixture for non-visibility singleton `MissingWitness`
  omission. It began as no-repair evidence and now carries one LAB-only
  `E-ROW-001` singleton repair item after `plan/94`. It does not prove
  OBL-025, freeze diagnostic/repair ABI, claim conformance, claim G1 exit, or
  edit canon.
- `plan/92-g1-erow001-base-singleton-fixture-closure.md` now records LAB-only
  `ELAB-14..16` fixtures for the remaining non-visibility singleton base
  failures. Together with `ELAB-13`, the singleton repair-bearing set covers
  `MissingCapability`, `MissingWitness`, `RouteUnavailable`, and
  `StaleMembership` after `plan/94`. It does not prove OBL-025, freeze
  diagnostic/repair ABI, claim conformance, claim G1 exit, or edit canon.
- `plan/93-g1-erow001-singleton-repair-assumption.md` now records the LAB-only
  single-edit assumption and no-placeholder payload constraints for
  non-visibility singleton repair. `plan/94` implements that gate for
  `ELAB-13..16`; `ELAB-04` remains no-repair and `ELAB-07` uses the later
  exact set path instead of this singleton gate. It does not prove OBL-025,
  freeze diagnostic/repair ABI, claim conformance, claim G1 exit, or edit
  canon.
- `plan/94-g1-erow001-singleton-repair-prototype.md` now records LAB-only
  `E-ROW-001` singleton `add-to-fails-row` repair payloads for all four base
  remote-request failure atoms. It preserves the no-placeholder gate, keeps
  `ELAB-04` no-repair, leaves `ELAB-07` to the later exact set path, and does
  not prove OBL-025, freeze repair ABI, claim repair ranking, claim multi-edit
  support, claim conformance, claim G1 exit, or edit canon.
- `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md` now records
  the pre-`plan/102` no-repair policy for `ELAB-04/07`. `ELAB-07` has since
  taken the exact set-insertion path; `plan/107` now records docs-only
  `ELAB-04` branch ownership / association / ordering preflight while keeping
  executable output no-repair. This does not prove OBL-025, freeze repair ABI,
  claim repair ranking, claim multi-edit support, claim conformance, claim G1
  exit, or edit canon.
- `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md` now records
  candidate payload vocabulary for set insertion, conjunctive bundles, and
  partial guidance. `ELAB-07` uses only the exact `plan/102` set path; `ELAB-04`
  remains no-repair. `plan/107` names the mixed wrapper / base branch /
  visibility branch vocabulary and explicitly defers ranking. This does not add
  general set-insertion / bundle support, prove OBL-025, freeze repair ABI,
  claim repair ranking, claim multi-edit support, claim conformance, claim G1
  exit, or edit canon.
- `plan/97-g1-erow07-set-insertion-gate-review.md` records the docs-first
  gate review for `ELAB-07`. Later `plan/100..102` accepted the narrow
  source-locus edit assumption, designed the payload, and implemented one exact
  non-final set item. This does not prove OBL-025, freeze repair ABI, claim
  multi-edit support, claim conformance, claim G1 exit, or edit canon.
- `plan/98-g1-erow04-mixed-visibility-branch-inventory.md` now records the
  docs-first branch inventory for `ELAB-04`. The mixed base remote-request /
  `VisibilityDenied` row stays no-repair until diagnostic ownership, branch
  association, ordering / ranking, and visibility-repair alternatives are
  explicit. It does not widen `ELAB-04`, add bundle support, prove OBL-025,
  freeze repair ABI, claim repair ranking, claim multi-edit support, claim
  conformance, claim G1 exit, or edit canon.
- `plan/99-g1-erow07-set-insertion-executable-preflight.md` records the
  docs-first executable preflight for `ELAB-07`: atomic set insertion as one
  source edit, exact whole-gap coverage, no extraneous declared failures, one
  target row, and no `ELAB-04` / `E-ROW-002` broadening. `plan/102` implements
  only the exact positive path. This does not add general set-insertion or
  bundle support, prove OBL-025, freeze repair ABI, claim conformance, claim G1
  exit, or edit canon.
- `plan/100-g1-erow07-set-insertion-assumption-acceptance.md` accepts the
  narrow LAB source-locus edit assumption for `ELAB-07` only: completing one
  existing concrete `when_fails_row` by duplicate-free insertion of the exact
  missing base-failure set is one row-field edit with `element_insert_count =
  3`. `plan/102` now uses that assumption for exact `ELAB-07` only; this does
  not add general set-insertion support, bundle semantics, prove OBL-025,
  freeze repair ABI, claim conformance, claim G1 exit, or edit canon.
- `plan/101-g1-erow07-set-insertion-payload-model-design.md` designs the
  `ELAB-07` set payload as one top-level non-final `set_insertion` item with
  candidate roles, no singleton `missing_failure` field reuse for multi-failure
  coverage, exact whole-gap guards, and a positive / negative test matrix.
  `plan/102` implements the exact positive path only; this does not add general
  set-insertion support, bundle semantics, prove OBL-025, freeze repair ABI,
  claim conformance, claim G1 exit, or edit canon.
- `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md` now
  records the LAB-only executable prototype for exact `ELAB-07`: one
  top-level non-final `set_insertion` item with `element_insert_count = 3`.
  `ELAB-04` remains no-repair, `ELAB-10` and `ELAB-13..16` remain singleton
  repair evidence, and this does not add general set-insertion support, bundle
  semantics, prove OBL-025, freeze repair ABI, claim conformance, claim G1
  exit, or edit canon.
- `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md` now records
  Rust-only negative guard evidence for the exact `ELAB-07` set path. Proper
  subset, padded declaration, duplicate declaration, and multi generated-request
  variants do not receive the `set_insertion` repair; the tested rows reject
  without `suggested_repair`. At `plan/103` close, the multi-request guard was
  conservative and keyed by the LAB target reference; `plan/104` subsequently
  narrowed the internal association key with the existing `when` source span
  without creating a final row identity model. This does not add general
  set-insertion support, bundle semantics, prove OBL-025, freeze repair ABI,
  claim conformance, claim G1 exit, or edit canon.
- `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md` now
  records the LAB-only internal row association fix after `plan/103`. It keeps
  public `target_ref` unchanged, keys request association by existing `when`
  source span internally, and prevents distinct same-event `when` rows from
  suppressing each other's exact `set_insertion` repair. This does not add a
  final row identity model, general set-insertion support, bundle semantics,
  prove OBL-025, freeze repair ABI, claim conformance, claim G1 exit, or edit
  canon.
- `plan/105-g1-erow07-set-insertion-exact-locus-guard-hardening.md` now
  records the LAB-only exact-locus guard after `plan/104`. It keeps the
  `set_insertion` path limited to the current `ELAB-07` locus
  (`role:BrowserClient`, event `attack`, owner `S`, state `player`, field
  `hp`) and adds Rust-only omitted-row / event / role / owner-locus /
  state-name / field retargeting proxy guards. This does not add a final
  source-locus identity model, final row identity model, general set-insertion
  support, bundle semantics, prove OBL-025, freeze repair ABI, claim
  conformance, claim G1 exit, or edit canon.
- `plan/106-g1-erow07-child-bundle-partial-exclusion-fixtures.md` now records
  the LAB-only shape guard after `plan/105`. It keeps exact `ELAB-07` output as
  one complete top-level `set_insertion` item and verifies it is not child
  singleton alternatives, a conjunctive bundle, partial guidance, or
  textual-only guidance. This does not add bundle semantics, partial-guidance
  output, general set-insertion support, prove OBL-025, freeze repair ABI,
  claim conformance, claim G1 exit, or edit canon.
- `plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md` now records
  the LAB-only docs-first preflight for `ELAB-04`. It keeps executable output
  no-repair and names a conceptual mixed wrapper, base remote-request branch,
  visibility branch, association vocabulary, and ordering / ranking deferrals.
  This does not add an `ELAB-04` executable payload, `ELAB-04` mixed
  set-insertion support, general set-insertion support, bundle semantics,
  partial-guidance output, visibility-repair ranking, prove OBL-025, freeze
  repair ABI, claim conformance, claim G1 exit, or edit canon.
- `plan/108-g1-obl025-branch-local-noncoverage-refinement.md` now records the
  LAB-only Lean statement refinement after the `ELAB-04` mixed preflight. It
  adds abstract `RepairBranch` / branch-local witness / branch-local suggestion
  vocabulary and helper relations that keep branch-local guidance from counting
  as whole rejected-gap OBL-025 coverage. It does not define final branch IDs,
  JSON fields, diagnostic/repair ABI, prove OBL-025, move canon ledger status,
  widen executable repairs, add an `ELAB-04` payload, claim conformance, claim
  G1 exit, or edit canon.

## candidate next strategy packages

These are candidates only. They are not promoted until the user chooses the
next line.

| Candidate | Macro reading | Objective | Close condition |
|---|---|---|---|
| `OBL-020 statement refinement` | `G1` reserve | refine the LAB `StepWFStatementDraft.lean` only if review finds overfit, missing abstraction, or premature proof-interface wording | Lean still compile-check only; no canon ledger movement |
| `OBL-021 statement refinement` | `G1` reserve | refine the LAB `ElabDeterminismStatementDraft.lean` only if review finds overfit, missing projection-totality wording, or diagnostic-equivalence gaps | Lean still compile-check only; no canon ledger movement |
| `OBL-001 statement draft refinement` | `G1` reserve | refine the LAB `THM001StatementDraft.lean` only if review finds overfit or a missing predicate | Lean still compile-check only; no canon ledger movement |
| `ELAB-04 mixed executable payload model` | `G1` reserve | only after a later package accepts a mixed wrapper or separate associated-diagnostics model for `ELAB-04` | keep no-repair until that model is explicit; no ranking or executable widening by default |
| `LAB claim-family drilldown` | `T0/G0` reserve | split selected `plan/70` rows into line-level `LAB:` citations when needed for a future G0 close decision | no canon L0/L1 change; no historical rewrite beyond focused wording cleanup |
| `canon mental-model clarification proposal` | `G0/G1` reserve | draft a proposal, not an edit, if ordinary assignment or LAB citation rules need a short canon clarification | proposal remains advisory until human/canon process accepts it |
| `repo-triage recut` | `Macro 0` maintenance | classify existing Product Alpha / Full System V1 / Surface evidence as keep-core-idea, archive-exploration, or postpone/drop for future theory recut | no deletion or archive move without explicit follow-up package |

## ordered self-driven packages

| Order | Package | Objective | Close condition |
|---:|---|---|---|
| 1 | `P-SURF-01 surface brace parser` | parse `S { ... }`, role-instance blocks, `state`, and `when`; reject `S[ ... ]` | closed with `SURF-01..09`, parser test, sample helper, authoring check, and release check |
| 2 | `P-SURF-02 indexed state` | represent `S { state player[p: Participant]: Player }` as S-owned indexed state | closed with `IDX-01..05`, semantic checker test, sample helper, authoring check, and release check |
| 3 | `P-SURF-03 Surface-to-Core elaboration` | lower cross-locus read/write to explicit Core IR | closed with `ELAB-01/02/04/05/06/07/08`; later G1 dependency-gap evidence added `ELAB-11/12` without conformance claim |
| 4 | `P-SURF-04 auto communication` | generate MessageEnvelope / publish / observe / failure-row obligations | closed with generated MessageEnvelope rows, visible field publish/observe rows, `VisibilityDenied` failure-row containment, private/non-visible field rejection, and `ELAB-03/09/10` |
| 5 | `P-SURF-05 role admission` | implement role claim, admission request, capability grant, spoof/stale rejection | closed with `ROLE-01..04`, role claim / join admission / grant-backed accepted write / witness rows, missing-grant write rejection, stale membership rejection, and hash metadata non-safety-proof |
| 6 | `P-SURF-06 source patch hot-plug` | implement parse/typecheck/elaborate/admit/activation-cut patch pipeline | closed with CLI `check-source` / `parse-source` / `elaborate-source` / `patch-source` / `export-core-ir`, `PATCH-01..04`, HotPlugRequest / HotPlugVerdict / activation_cut rows, no-direct-eval evidence, and rejection-without-mutation rows |
| 7 | `P-SURF-07 source operational suite` | create Surface source WorldCore / MembershipChat / Sugoroku / related roots | closed with six source roots, `operational-matrix.json`, and `E2E-SURF-01..12` positive/negative rows |
| 8 | `P-SURF-08 devtools and diagnostics` | show Surface source, Core IR, generated communication, semantic indexed-state map, admission, redacted patch lifecycle | closed with `samples/full-system-v1-surface/devtools/`, `DEV-01..02`, required panels, diagnostics, redaction gates, and source-span evidence |
| 9 | `P-SURF-99 final audit` | rerun validation and compatibility anchors | closed with full validation, docs/report cleanup, non-claim audit |

## self-driven macro phase reading

| Macro | Reading | Closeout path |
|---|---|---|
| `Macro 0` | docs / reports / validator discipline | self-driven through every Surface package close |
| `Macro 1` | semantics and invariant boundary | self-driven for source authority, place syntax, indexed state, admission, patch pipeline |
| `Macro 3` | compile-ready minimal actualization | `P-SURF-01..08` and P-SURF-99 audit closed; maintenance only until a new package is promoted |
| `Macro 4` | executable sample expansion | `P-SURF-07` created operational roots; `P-SURF-08` added static diagnostics; P-SURF-99 audit closed |
| `Macro 6` | distributed fabric / runtime evolution | local/Docker alpha can be self-driven; WAN/federation remains user decision |
| `Macro 7` | toolchain / backend / developer surface | Surface CLI/devtools can be self-driven within alpha scope |
| `Macro 8` | domain/application realization | Surface operational suite can be self-driven after language/runtime base |

## user decision gates

| Gate | Affects | Main options | Current recommendation |
|---|---|---|---|
| final public grammar | final language/API | freeze Surface alpha / revise before public / keep package compatibility longer | do not freeze in Surface alpha; keep grammar explicitly alpha |
| final ABI / SDK | external developers | Rust library ABI / CLI-only / hosted API / engine SDK | defer until Surface parser/elaboration/runtime evidence exists |
| broader distribution | product delivery | developer-built bundle / release archive / installer / hosted service | keep current developer-built binary + generated host bundle |
| final shared-space catalog breadth | product scope | bounded showcase / broader room catalog / Reversed Library path | keep bounded showcase; decide separately |
| production WAN/federation | runtime/network | local/Docker only / WAN federation / hosted fabric | keep out of Surface alpha unless explicitly promoted |
| distributed durable save/load R3/R4 | persistence | R0/R2 only / R3 durable / R4 distributed replay | keep R3/R4 later |
| native/WASM execution | provider boundary | disabled/inventory / sandboxed WASM / bounded native | keep default disabled/inventory |
| final engine adapter ABI | engine/provider line | internal provider manifest / public SDK / engine-specific ABI | defer; no Unity/Unreal/VRM compatibility claim |

## research discovery items

| Item | Impact | Main options | Current recommendation |
|---|---|---|---|
| brace disambiguation | `P-SURF-01` | namespace-only / context-only / combined namespace + context | use combined namespace + context with ambiguous diagnostic |
| role-instance block parse | `P-SURF-01` | role path only / arbitrary indexed expression block | alpha accepts declared role path only |
| indexed-state runtime carrier | `P-SURF-02` / later runtime packages | plain map / membership-aware partial map / distributed table | `P-SURF-02` fixed checker semantics; use membership-aware owner-locus partial map first when runtime carrier is added |
| elaboration IR shape | `P-SURF-03` / `P-SURF-04` | direct Core transitions / intermediate elaboration report / both | closed with Core IR plus source-linked elaboration and generated communication rows |
| auto publish policy | `P-SURF-04` | publish all writes / visible-fields-only / explicit-only | closed narrow alpha: visible-fields-only; private/non-visible fields blocked; TypeMismatch discharge remains later |
| admission witness metadata | `P-SURF-05` | principal only / role + principal / optional package/runtime hash | closed narrow alpha: role + principal required; package/runtime hash optional report metadata and not safety proof |
| source patch compatibility | `P-SURF-06` | check-only / check+diff / full migration planner | closed narrow alpha: check+Core diff+HotPlugRequest/HotPlugVerdict+activation_cut; full migration planner later |
| Surface sample root shape | `P-SURF-07` | reuse `full-system-v1/` / new `full-system-v1-surface/` / product-alpha root | closed with `samples/full-system-v1-surface/` top-level operational roots distinct from Product Alpha roots |
| Surface diagnostics shape | `P-SURF-08` | static helper bundle / CLI export / runtime devtools integration | closed with static observer-safe report bundle first; final viewer/telemetry ABI later |

## maintenance tasks

| Task | Objective | Validation | Stop line |
|---|---|---|---|
| docs freshness audit | keep README, Documentation, progress, tasks, samples dashboard, indexes aligned | `python3 scripts/validate_docs.py`, `python3 scripts/check_source_hierarchy.py`, `git diff --check` | snapshot docs must not create new normative decisions |
| canon/LAB wording audit | keep touched LAB docs from re-promoting legacy `specs/` or helper closeouts to canon status | source-hierarchy grep plus validators | do not rewrite historical LAB evidence wholesale |
| product compatibility audit | preserve Product Alpha and operational suite while Surface advances | product release check, operational suite helper, minimal pattern verifier | do not reinterpret alpha workflow as final product |
| sample taxonomy audit | keep Surface planned roots distinct from active roots | source hierarchy and relevant helper checks | do not create or mark `samples/full-system-v1-surface/` workflow-ready until implementation rows exist |
| validator scaffold update | add required docs only when they exist | `python3 -m unittest scripts.tests.test_validate_docs` | validators check presence and heading shape, not semantic correctness |
| report discipline | write a new report for every non-trivial package | `python3 scripts/validate_docs.py` | never overwrite previous report |

## non-promoted references

- Product Alpha line remains bounded alpha workflow, not final product.
- Operational suite remains bounded local/Docker workflow, not production shared-space catalog completion.
- Full System V1 release-check closure remains bounded local/source-first evidence, not final grammar / final ABI / final server-client compiler.
- `samples/full-system-v1-surface/syntax/` is P-SURF-01 parser evidence only,
  not a Surface runtime or operational suite.
- `samples/full-system-v1-surface/indexed-state/` is P-SURF-02 semantic
  checker evidence only, not a Surface runtime, elaboration, or operational
  suite.
- `samples/full-system-v1-surface/elaboration/` is P-SURF-03/P-SURF-04
  elaboration and generated communication evidence only, not runtime
  MessageEnvelope dispatch, role admission, source patch activation, or an
  operational suite.
- `samples/full-system-v1-surface/role-admission/` is P-SURF-05 report-level
  admission/grant evidence only, not production identity, hardware attestation,
  WAN admission, runtime membership lifecycle, source patch activation, or an
  operational suite.
- `samples/full-system-v1-surface/source-patch/` is P-SURF-06 source patch
  hot-plug pipeline evidence only, not a final hot-plug ABI, distributed
  durable migration planner, production patch registry, or arbitrary
  native/WASM execution route.
- `samples/full-system-v1-surface/world-core/`, `membership-chat/`,
  `sugoroku-world/`, `portal-worldlink/`, `two-shard-hard-boundary/`, and
  `gradient-observation/` are P-SURF-07 source operational evidence only, not a
  final operational runtime/transport or final shared-space catalog.
- `S[ ... ]` remains rejected and must not be introduced as a compatibility sugar.
- `package.mir.json` remains alpha compatibility / package artifact, not semantic source authority.
- Direct LLVM/native backend remains later than Surface parser, elaboration, typed IR, projection IR, and preservation tests.
