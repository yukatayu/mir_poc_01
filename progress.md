# progress

最終更新: 2026-07-04 09:04 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins; cite legacy `specs/` /
`plan/` as `LAB:` evidence unless mirrored into canon.

## document role

This document is the repo-wide **current roadmap snapshot**. It is not normative
source.

- Normative source: `mirrorea_canon/`
- LAB repository memory / evidence: `plan/`, legacy `specs/`
- Runnable dashboard: `samples_progress.md`
- Current task map: `tasks.md`
- Execution evidence: `docs/reports/`

Use workflow status and evidence class as the primary reading. Do not use
percentage as the main metric.

## project axis

```text
正しい理論に基づき、Mir source files を意味の正本として、
各 server / browser-like runtime / backend がそれ由来 artifact を実行し、
正しく hot-plug / 通信 / 検証 / 可視化できる仮想空間 system を作る。
```

This keeps Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform
separable.

## final ideal

The current long direction is source-first:

```text
.mir source files
  -> Surface Mir parser / AST
  -> Surface-to-Core elaboration
  -> Core Mir / typed IR
  -> checker / residual proof-model obligations
  -> interpreter and runtime session
  -> projection IR / deployment plan
  -> server / client / adapter artifacts
  -> provider boundary and devtools evidence
```

`package.mir.json` remains an alpha compatibility / package artifact. It is not
semantic source authority.

## current milestone position

- Current package: none promoted for the Surface line after `P-SURF-99`.
- Current canon position: `mirrorea_canon/plan/01-phases.md` places the project
  at T0/G0 rebaseline. LAB implementation and sample rows below are evidence,
  not canon implementation-state completion.
- Current migration note: root LAB entry points now point to `mirrorea_canon/`
  as canon, and `plan/70-lab-to-canon-reconciliation-ledger.md` now maps
  high-risk legacy LAB claim families to canon IDs, rejected historical claim
  patterns, or OPEN follow-up. This is LAB evidence and does not claim G0 exit.
- Current G1 planning note: `plan/71-g1-ordinary-assignment-target.md` now
  drafts the ordinary simple-assignment target/proof-boundary split in LAB memory. It
  targets THM-001 / BND-001 / SCN-01 / SCN-02 alignment only and does not claim
  G1 exit, theorem discharge, Lean proof completion, runtime dispatch, or public
  API freeze.
- Current SCN consequence note: `plan/72-g1-scn01-scn02-static-consequence-drilldown.md` maps
  SCN-01 and SCN-02 C-static expectations to canon targets, LAB ELAB evidence,
  LAB gaps, and runtime/proof boundaries. It does not claim C-static
  conformance.
- Current OBL-001 inventory note: `plan/73-g1-obl001-lean-statement-inventory.md`
  inventories the minimum Lean statement vocabulary, predicate split, SCN row
  coverage, adjacent-obligation separation, and overfit guards for THM-001. It
  adds no Lean statement file and does not move OBL status.
- Current OBL-001 statement-draft note:
  `plan/74-g1-obl001-lean-statement-draft.md` records the first repo-local
  LAB Lean statement-shape draft at
  `samples/lean/lab-statements/obl001/THM001StatementDraft.lean`. It compiles
  as a `Prop` definition under a LAB namespace and does not move OBL status,
  prove THM-001, claim G1 exit, or edit canon.
- Current SCN dependency-gap evidence note:
  `plan/75-g1-scn-rhs-dependency-gap-evidence.md` records LAB-only evidence
  rows `ELAB-11` and `ELAB-12` for SCN-01 same-field RHS dependency and SCN-02
  target/self RHS dependencies. These rows close the immediate LAB evidence gap
  without claiming C-static conformance, proof discharge, G1 exit, runtime read
  materialization, or final Core IR JSON/API.
- Current OBL-020/021 inventory note:
  `plan/76-g1-obl020-021-dependency-inventory.md` separates the
  well-formedness-preservation and elaboration-determinism dependency
  inventories from OBL-001/002. It does not claim either obligation complete,
  create Lean statement files, move canon ledger status, prove a skeleton,
  claim G1/T1/T2 exit, or claim conformance.
- Current OBL-021 statement-draft note:
  `plan/77-g1-obl021-lean-statement-draft.md` records a LAB-only
  repo-local Lean statement-shape draft at
  `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean`.
  It compiles as a `Prop` definition for success/result equivalence,
  diagnostic equivalence, and success/reject mutual exclusion. It does not
  move OBL status, prove OBL-021, claim G1/T1/T2 exit, or edit canon.
- Current OBL-020 statement-draft note:
  `plan/78-g1-obl020-lean-statement-draft.md` records a LAB-only
  repo-local Lean statement-shape draft at
  `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean`. It compiles
  as an aggregate `Prop` over abstract `WellFormed` and `Step`, keeps WF
  clauses behind `WellFormed`, and does not move OBL status, prove OBL-020,
  claim proof skeleton completion, G1/T1/T2 exit, conformance, runtime
  implementation proof, or edit canon.
- Current E-ROW alignment note:
  `plan/79-g1-erow-diagnostic-alignment.md` maps canon E-ROW-001/E-ROW-002 to
  current LAB `generated_failure_not_declared` evidence. It treats ELAB-07 as
  clean E-ROW-001-shaped evidence, ELAB-10 as clean E-ROW-002-shaped pressure
  evidence still carried by the same helper-local diagnostic family, and
  ELAB-04 as mixed E-ROW-shaped evidence. It does not freeze a diagnostic ABI,
  discharge OBL-024/025, claim conformance, or edit canon.
- Current diagnostic carrier inventory note:
  `plan/80-g1-diagnostic-carrier-inventory.md` inventories the gap between
  canon Diagnostic fields and current LAB `code/message/span` plus helper
  `diagnostic_codes` / source-span sidecar evidence. It does not implement a
  diagnostic ABI, state or prove OBL-024/025, claim explanation
  soundness/completeness, or edit canon.
- Current OBL-024 statement-shape inventory note:
  `plan/81-g1-obl024-statement-shape-inventory.md` inventories the abstract
  vocabulary needed to state explanation soundness: emitted Diagnostic,
  reported rule instance / failed premise / bindings, and replay failure
  exactly at that premise. It treats E-ROW as the immediate G1 pressure case
  and remains the pre-draft relation inventory; `plan/109` now adds the
  compile-check-only Lean statement draft. This does not prove OBL-024, freeze
  diagnostic ABI, claim conformance, or edit canon.
- Current OBL-024 statement-draft note:
  `plan/109-g1-obl024-lean-statement-draft.md` records a LAB-only
  repo-local Lean statement-shape draft at
  `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`.
  It compiles as a diagnostic-soundness `Prop` over abstract diagnostic
  projection, report-local association key, future proof-level diagnostic
  association, reported rule / premise / bindings, report-local replay anchor,
  future proof-level replay relation, and non-repair mixed diagnostic branch
  predicates. `plan/113` refines the draft so current `trace_local_replay`
  evidence is represented by `ReportLocalReplayAnchor`, while
  `ProofLevelReplayWitness` / `ProofLevelReplayRelation` remain future
  proof-level vocabulary. `plan/114` refines the draft so current
  `lab_association_key` evidence is represented by
  `ReportLocalAssociationKey`, while `ProofLevelAssociationWitness` /
  `ProofLevelAssociationRelation` remain future proof-level vocabulary. It
  does not move canon OBL status, prove OBL-024, freeze Diagnostic ABI /
  request ID / branch ID / association-key ABI / replay semantics, claim
  root-cause uniqueness, claim conformance, or claim G1 exit.
- Current OBL-024 executable projection carrier note:
  `plan/110-g1-obl024-executable-projection-carrier.md` records LAB-only
  executable E-ROW projection evidence inside current `lab_diagnostic_details`.
  `ELAB-04/07/10/13..16` now carry non-final
  `diagnostic_soundness_projection` with helper-local diagnostic id,
  report-local association key, reported bindings, and report-local trace
  replay anchor.
  This does not move canon OBL status, prove OBL-024, freeze Diagnostic JSON /
  request ID / association-key ABI / replay semantics, widen repair output, add
  an `ELAB-04` payload, claim conformance, or claim G1 exit.
- Current OBL-024 Rust fixture guard note:
  `plan/111-g1-obl024-projection-rust-fixture-guards.md` records test-only
  Rust guard hardening for the current projection-bearing fixtures. The Rust
  helper now checks projection / request-context / failure-row-context
  consistency and non-serialization of skipped internal association fields, and
  fixture-backed Rust tests cover `ELAB-04/07/10/13..16`. This does not change
  production emission logic, expected JSON, repair output, Diagnostic ABI,
  proof status, conformance, or G1 exit.
- Current OBL-024 replay vocabulary preflight note:
  `plan/112-g1-obl024-replay-vocabulary-preflight.md` records a docs-first
  split between current report-local replay anchors and future proof-level
  replay relations. It keeps `trace_local_replay` as helper-local consistency
  evidence, marks proof-level replay vocabulary as OPEN, and does not freeze
  replay ABI / Diagnostic ABI, prove OBL-024, change repair output, claim
  conformance, or claim G1 exit.
- Current OBL-025 statement-shape inventory note:
  `plan/82-g1-obl025-statement-shape-inventory.md` inventories the abstract
  vocabulary needed to state Line-1 explanation completeness: Line-1
  rejection, declared fragment, single-edit repair existence, non-empty
  suggested repair, and repair/failure matching. It does not add a Lean file,
  generate repairs, prove OBL-025, freeze diagnostic/repair ABI, claim
  conformance, or edit canon.
- Current E-ROW repair payload inventory note:
  `plan/83-g1-erow-repair-payload-inventory.md` inventories a non-final
  repair payload vocabulary for future E-ROW diagnostics if a later prototype
  includes `suggested_repair[]`. It does not implement repairs, freeze
  diagnostic/repair ABI, prove OBL-024/025, claim explanation
  soundness/completeness, or edit canon.
- Current E-ROW carrier-only diagnostic detail note:
  `plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md` records the
  LAB-only implementation of non-final `lab_diagnostic_details` for
  Surface-to-Core generated failure-row containment. The carrier preserves the
  legacy `generated_failure_not_declared` code, splits clean/mixed omissions to
  E-ROW-001 and clean `VisibilityDenied`-only omission to E-ROW-002, exposes
  severity / rule / premise / missing-evidence refs, emits no
  `suggested_repair[]`, and does not freeze diagnostic ABI, prove OBL-024/025,
  claim conformance, or claim G1 exit.
- Current E-ROW carrier precondition hardening note:
  `plan/85-g1-erow-carrier-precondition-hardening.md` records the LAB-only
  addition of non-final `request_context` and `failure_row_context` inside
  `lab_diagnostic_details`. It exposes generated-request identity and the local
  row-containment precondition for `ELAB-04/07/10`. At its package close it
  emitted no `suggested_repair[]`; later packages add `ELAB-10` singleton,
  `ELAB-13..16` singleton, and exact `ELAB-07` set repair evidence. This does
  not freeze diagnostic/repair ABI, prove OBL-024/025, claim conformance, or
  claim G1 exit.
- Current E-ROW-002 visibility repair carrier prototype note:
  `plan/86-g1-erow002-visibility-repair-carrier-prototype.md` records the
  LAB-only `suggested_repair` item emitted only for the `E-ROW-002` /
  `VisibilityDenied` row-containment failure shape represented by `ELAB-10`.
  `ELAB-04` continues to emit no repair row; `ELAB-07` uses the later exact
  `E-ROW-001` set path under `plan/102`, outside this visibility-carrier
  package. This does not freeze diagnostic/repair ABI, prove OBL-024/025,
  claim explanation completeness, claim conformance, or claim G1 exit.
- Current OBL-025 statement-draft note:
  `plan/87-g1-obl025-lean-statement-draft.md` records a LAB-only
  repo-local Lean statement-shape draft at
  `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`.
  It compiles as an existential repair-coverage `Prop` over abstract
  Line-1/rejection/declared-fragment/repair/diagnostic predicates. The current
  refinement adds whole-rejected-gap, set-insertion, grouped multi-edit,
  complete local repair, partial-guidance non-coverage, and branch-local
  non-coverage predicates so set insertion can enter only through the existing
  single-edit relation, while grouped multi-edit / partial guidance and
  `ELAB-04` branch-local guidance remain outside current OBL-025 coverage
  unless a later whole-gap relation covers every missing failure. It does not
  move canon OBL status, prove OBL-025, freeze
  diagnostic/repair ABI, claim explanation completeness, claim conformance, or
  claim G1 exit.
- Current E-ROW repair shape inventory note:
  `plan/88-g1-erow-repair-shape-inventory.md` records the LAB-only taxonomy
  for repair output widening. Current repair-bearing singleton evidence is
  `ELAB-10` for `E-ROW-002` / `VisibilityDenied` and `ELAB-13..16` for
  `E-ROW-001` non-visibility base failures. Exact `ELAB-07` now carries one
  non-final `set_insertion` item under `plan/102`, while `ELAB-04` remains
  mixed visibility/non-visibility multi-missing no-repair evidence. This does
  not prove OBL-025, freeze ABI, claim ranking/multi-edit support, or claim G1
  exit.
- Current E-ROW-001 non-visibility singleton fixture note:
  `plan/89-g1-erow001-non-visibility-singleton-fixture.md` records LAB-only
  `ELAB-13` as a non-visibility singleton `MissingWitness` omission. It began
  as no-repair evidence and is now `E-ROW-001` singleton repair evidence after
  `plan/94`. This does not prove OBL-025, freeze diagnostic/repair ABI, claim
  conformance, or claim G1 exit.
- Current E-ROW-001 base singleton fixture closure note:
  `plan/92-g1-erow001-base-singleton-fixture-closure.md` records LAB-only
  `ELAB-14..16` for `MissingCapability`, `RouteUnavailable`, and
  `StaleMembership`, completing one singleton fixture per base remote-request
  failure atom when read with `ELAB-13`. Those rows are now LAB-only
  repair-bearing evidence after `plan/94`. This does not prove OBL-025,
  freeze diagnostic/repair ABI, claim conformance, or claim G1 exit.
- Current E-ROW-001 singleton repair assumption gate note:
  `plan/93-g1-erow001-singleton-repair-assumption.md` records the LAB-only
  single-edit assumption and no-placeholder payload constraints for
  non-visibility singleton repair. `plan/94` implements that gate for
  `ELAB-13..16`; `ELAB-04` remains no-repair and `ELAB-07` uses the later
  exact set path in `plan/102`. This does not prove OBL-025,
  freeze diagnostic/repair ABI, claim conformance, or claim G1 exit.
- Current E-ROW-001 singleton repair prototype note:
  `plan/94-g1-erow001-singleton-repair-prototype.md` records the LAB-only
  `E-ROW-001` singleton `add-to-fails-row` repair payload for
  `MissingCapability`, `MissingWitness`, `RouteUnavailable`, and
  `StaleMembership` singleton omissions. It preserves the `plan/93`
  no-placeholder / local-witness gate, keeps `ELAB-04` no-repair, and is
  separate from the later `ELAB-07` set path. It does not prove OBL-025, freeze
  diagnostic/repair ABI, claim repair ranking, claim multi-edit support, claim
  conformance, or claim G1 exit.
- Current E-ROW mixed / multi repair decomposition inventory note:
  `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md` records why
  `ELAB-07` originally needed set-insertion / bundle / partial-guidance /
  multi-edit decomposition before repair output. `plan/102` now implements the
  exact set-insertion path for `ELAB-07`; `plan/107` now records docs-only
  `ELAB-04` branch ownership / association / ordering preflight while keeping
  executable output no-repair. This does not prove OBL-025, freeze
  diagnostic/repair ABI, claim repair ranking, claim multi-edit support, claim
  conformance, or claim G1 exit.
- Current E-ROW set-insertion / bundle payload inventory note:
  `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md` records
  candidate LAB vocabulary for future set insertion, conjunctive bundles, and
  partial guidance. After `plan/102`, exact `ELAB-07` has one non-final set
  item, while `ELAB-04` remains no-repair. `plan/107` names the mixed wrapper /
  base branch / visibility branch vocabulary and explicitly defers ranking.
  This does not add general executable set-insertion / bundle support, prove
  OBL-025, freeze diagnostic/repair ABI, claim repair ranking, claim multi-edit
  support, claim conformance, or claim G1 exit.
- Current ELAB-07 set-insertion gate review note:
  `plan/97-g1-erow07-set-insertion-gate-review.md` records that `ELAB-07`
  was kept no-repair until a single-source-edit set-insertion decision or
  bundle semantics with whole rejected-gap coverage was explicit. Later
  `plan/100..102` accepted the narrow source-locus edit assumption, designed
  the payload, and implemented the exact `ELAB-07` set item. This still does
  not prove OBL-025, freeze repair ABI, claim multi-edit support, or claim G1
  exit.
- Current ELAB-07 set-insertion assumption acceptance note:
  `plan/100-g1-erow07-set-insertion-assumption-acceptance.md` accepts, only
  for the exact `ELAB-07` candidate gate, that duplicate-free insertion of the
  complete missing base-failure set into the one existing concrete
  `when_fails_row` counts as one LAB source-locus edit with
  `element_insert_count = 3`. `plan/102` now uses this assumption for the
  exact `ELAB-07` executable payload only; this does not add general
  set-insertion support, bundle semantics, OBL-025 proof / completion,
  conformance, or G1 exit.
- Current ELAB-07 set-insertion payload-model design note:
  `plan/101-g1-erow07-set-insertion-payload-model-design.md` narrows the
  `ELAB-07` set payload to one top-level non-final `set_insertion` item with
  candidate roles, no singleton `missing_failure` field reuse for multi-failure
  coverage, exact whole-gap guards, and a positive / negative test matrix.
  `plan/102` implements the exact positive path only; this does not add
  general set-insertion support, bundle semantics, OBL-025 proof / completion,
  conformance, final ABI, or G1 exit.
- Current ELAB-07 set-insertion executable payload prototype note:
  `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md`
  implements one LAB-only non-final `set_insertion` `suggested_repair[]` item
  for the exact current `ELAB-07` fact pattern. `ELAB-04` remains no-repair,
  `ELAB-10` and `ELAB-13..16` keep singleton output, sample row count remains
  52, and this does not add general set-insertion support, bundle semantics,
  OBL-025 proof / completion, conformance, final ABI, or G1 exit.
- Current ELAB-07 set-insertion negative-guard hardening note:
  `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md` adds
  Rust-only guard evidence around the exact `ELAB-07` set path. Proper subset,
  padded declaration, duplicate declaration, and multi generated-request
  variants now do not receive the `set_insertion` repair; the tested rows
  reject without `suggested_repair`. At `plan/103` close, the multi-request
  guard was conservative and keyed by the LAB target reference; `plan/104`
  subsequently narrowed the internal association key with the existing `when`
  source span without creating a final row identity model. Sample row count
  remains 52, and this does not add general set-insertion support, bundle
  semantics, OBL-025 proof / completion, conformance, final ABI, or G1 exit.
- Current ELAB-07 set-insertion row-identity guard hardening note:
  `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md` keeps the
  public LAB `target_ref` stable but narrows the internal request association
  key with the existing `when` source span. Distinct same-event `when` rows
  under one role locus no longer suppress each other's exact `set_insertion`
  repair, while two requests in one `when` row still suppress it. This does not
  add a final row identity model, general set-insertion support, bundle
  semantics, OBL-025 proof / completion, conformance, final ABI, or G1 exit.
- Current ELAB-07 set-insertion exact-locus guard hardening note:
  `plan/105-g1-erow07-set-insertion-exact-locus-guard-hardening.md` keeps the
  set path limited to the exact current `ELAB-07` locus:
  `role:BrowserClient`, event `attack`, owner place `S`, state `player`, and
  field `hp`. Current Surface-expressible omitted-row / event / role / owner /
  state / field retargeting proxies now reject without `set_insertion` repair.
  This does not add a final source-locus identity model, final row identity
  model, general set-insertion support, bundle semantics, OBL-025 proof /
  completion, conformance, final ABI, or G1 exit.
- Current ELAB-07 child / bundle / partial exclusion fixture note:
  `plan/106-g1-erow07-child-bundle-partial-exclusion-fixtures.md` adds a
  Rust-only shape guard over the exact positive `ELAB-07` payload. The current
  payload remains one complete top-level `set_insertion` item, not three child
  singleton alternatives, not a conjunctive bundle, not partial guidance, and
  not textual-only guidance. This does not add bundle semantics, partial
  guidance output, general set-insertion support, OBL-025 proof / completion,
  conformance, final ABI, or G1 exit.
- Current ELAB-04 mixed visibility payload-model preflight note:
  `plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md` records a
  docs-only ownership / association / ordering preflight for future `ELAB-04`
  mixed payload work. It keeps executable output unchanged and records a
  conceptual mixed wrapper, base remote-request branch, visibility branch,
  association vocabulary, and ranking deferrals. This does not add an
  `ELAB-04` executable payload, set-insertion support, bundle semantics,
  partial-guidance output, visibility-repair ranking, OBL-025 proof /
  completion, conformance, final ABI, or G1 exit.
- Current OBL-025 branch-local non-coverage note:
  `plan/108-g1-obl025-branch-local-noncoverage-refinement.md` records a
  LAB-only Lean refinement after the `ELAB-04` mixed payload preflight. It adds
  abstract `RepairBranch` / branch-local witness and suggestion predicates plus
  helper relations that explicitly keep branch-local guidance from counting as
  whole rejected-gap OBL-025 coverage. `RepairBranch` is statement-shape
  vocabulary only, not a final branch ID, JSON field, diagnostic ABI, or repair
  ABI. This does not add proof, canon ledger movement, executable repair
  widening, `ELAB-04` payload output, branch ranking, conformance, final ABI,
  or G1 exit.
- Current planning note: consultation-derived strategy has been captured as
  non-normative repository memory in `plan/69`. It does not promote a new
  package, change `specs/`, or decide whether work should move to a separate
  theory/design repository.
- Current status after this snapshot: `P-SURF-99` closed the bounded Surface
  alpha audit over `P-SURF-01..08`. Surface alpha/LAB evidence now has 52 helper rows
  and 53 `.mir` source files, with parser, indexed-state, elaboration,
  generated communication, role admission, source patch, source operational,
  static devtools diagnostics floors, and G1 RHS dependency-gap rows revalidated.
- Next gap: OBL-025 now has a compile-check-only LAB statement shape, an
  E-ROW repair-shape inventory, a full base-failure non-visibility singleton
  repair-bearing fixture set, and a LAB single-edit / no-placeholder gate
  implemented for `ELAB-13..16`. `ELAB-04/07` now have a LAB decomposition
  inventory plus candidate set-insertion / bundle payload vocabulary and an
  `ELAB-07` gate review / executable preflight / assumption acceptance /
  payload design / exact executable prototype for one non-final set item,
  Rust-only negative guard hardening for subset / padded / duplicate /
  multi-request variants, span-based internal association hardening for
  distinct same-event rows, exact-locus hardening for current
  Surface-expressible omitted-row / retargeting proxies, and child / bundle /
  partial / textual guidance exclusion shape assertions, plus an `ELAB-04`
  mixed visibility payload-model preflight that names the conceptual mixed
  wrapper / base branch / visibility branch and keeps ranking deferred, and an
  OBL-025 branch-local non-coverage refinement that keeps branch-local guidance
  outside whole-gap repair coverage.
  `ELAB-07` now emits one `set_insertion` item only for the current exact
  write-side base-failure set gap. `plan/100` accepts the narrow LAB
  source-locus edit assumption for `ELAB-07` only, with
  `element_insert_count = 3`; `plan/101` designs the payload roles / test
  matrix; `plan/102` implements the exact positive path; `plan/103`
  suppresses nearby negative variants without changing sample rows; and
  `plan/104` prevents cross-row suppression for distinct same-event rows without
  changing public `target_ref`; `plan/105` narrows the set path to the exact
  current `ELAB-07` locus without claiming a final source-locus identity; and
  `plan/106` keeps the emitted repair as one complete top-level set item
  without claiming bundle or partial-guidance semantics.
  `ELAB-04` now has a separate mixed visibility branch inventory and
  payload-model preflight that keep it no-repair while recording conceptual
  mixed wrapper / base remote-request branch / `VisibilityDenied` branch
  vocabulary and association / ordering / ranking deferrals. It still has no
  proof, no canon status movement, no executable `ELAB-04` payload, no final
  repair ABI, no repair ranking, no visibility-repair ranking, no general
  set-insertion support, and no multi-edit support. This remains before any
  later user-spec-required reopen for final runtime/transport, final source
  patch ABI, final viewer/telemetry ABI, or broader public grammar.
- Current truthful summary:
  Product Alpha-1 and the operational product suite remain bounded alpha floors.
  Full System V1 remains closed through bounded release-check / final audit.
  The closed Surface Mir alpha line has fixed that `.mir` source files own semantic
  authority, `package.mir.json` is alpha artifact, indexed state is owner-locus
  state keyed by participants or later constrained keyspaces, role claims are
  not authority, source patches go through parse/typecheck/elaborate/admit and
  activation cut, and generated communication / publish / observe must be
  visible in Core IR and devtools. `P-SURF-01` is parser/helper/sample evidence
  only. `P-SURF-02` is indexed-state semantic checker/sample evidence only; it
  does not claim runtime execution or role admission. `P-SURF-03` is
  elaboration evidence and `P-SURF-04` is generated communication elaboration
  evidence, `P-SURF-05` is role-admission evidence, `P-SURF-06` is source
  patch pipeline evidence, `P-SURF-07` is source operational evidence, and
  `P-SURF-08` is static devtools diagnostics evidence; `P-SURF-99` is final
  validation / claim-non-claim audit evidence. None
  claims runtime MessageEnvelope dispatch, production identity, hardware
  attestation, WAN admission, final source patch ABI, distributed durable
  migration, final operational runtime/transport, or final devtools viewer /
  telemetry ABI completion.

## milestone map

| Milestone | Meaning | Status | Evidence | Next gap |
|---|---|---|---|---|
| `P-A1` | Product alpha release candidate | `product-alpha-ready` | `mirrorea-alpha`, `package.mir.json`, product release check | keep as alpha compatibility floor |
| `P-OPS` | Operational product suite | `workflow-ready` | six bounded Product Alpha operational roots and helper checks | final catalog breadth remains user decision |
| `P-FSV1` | Full System V1 bounded source-first line | `workflow-ready release-check lane; audit closed` | `specs/33..38`, `plan/58..63`, `scripts/full_system_v1_release_check.py` | later public/broader reopen only |
| `P-SURF-00B` | Surface Mir brace/source-authority docs rebaseline | `closed` | `specs/39..43`, `plan/64..68`, snapshot docs and guides | implementation line opened |
| `P-SURF-01` | Surface brace parser | `evidence-closed parser lane` | `crates/mir-ast::surface_alpha`, `surface_mir_alpha_parse`, `samples/full-system-v1-surface/syntax/`, `scripts/surface_mir_samples.py` | keep non-final grammar; feed parser AST into later Surface packages |
| `P-SURF-02` | indexed state | `evidence-closed semantic checker lane` | `crates/mir-semantics::surface_indexed_state`, `surface_indexed_state_check`, `samples/full-system-v1-surface/indexed-state/`, `IDX-01..05` | integrate with Surface-to-Core elaboration and runtime carrier later |
| `P-SURF-03` | Surface-to-Core elaboration | `evidence-closed elaboration lane plus G1 dependency-gap / E-ROW rows` | `crates/mir-semantics::surface_to_core_elaboration`, `surface_to_core_elaborate`, `samples/full-system-v1-surface/elaboration/`, `ELAB-01..16` | keep feeding later runtime/admission/proof work; no conformance claim |
| `P-SURF-04` | auto communication / publish / observe | `evidence-closed generated communication lane` | `crates/mir-semantics::surface_to_core_elaboration`, `surface_to_core_elaborate`, `samples/full-system-v1-surface/elaboration/`, `ELAB-03/09/10` plus widened `ELAB-01/05/08` | runtime dispatch and TypeMismatch discharge remain later |
| `P-SURF-05` | role admission | `evidence-closed admission/grant lane` | `crates/mir-semantics::surface_role_admission`, `surface_role_admission_check`, `samples/full-system-v1-surface/role-admission/`, `ROLE-01..04` | runtime identity/admission lifecycle remains later |
| `P-SURF-06` | source patch hot-plug | `evidence-closed source patch lane` | `crates/mir-runtime::surface_source_patch_hotplug`, `mirrorea-alpha check-source/parse-source/elaborate-source/patch-source/export-core-ir`, `samples/full-system-v1-surface/source-patch/`, `PATCH-01..04` | final hot-plug ABI and migration planner remain later |
| `P-SURF-07` | Surface source operational suite | `evidence-closed source operational lane` | `samples/full-system-v1-surface/{world-core,membership-chat,sugoroku-world,portal-worldlink,two-shard-hard-boundary,gradient-observation}/`, `E2E-SURF-01..12` | final operational runtime/transport remain later |
| `P-SURF-08` | Surface devtools / diagnostics | `evidence-closed static diagnostics lane` | `samples/full-system-v1-surface/devtools/`, `DEV-01..02` | final viewer/telemetry ABI and runtime devtools remain later |
| `P-SURF-99` | Surface Mir alpha audit | `audit-closed` | Surface release check, Product Alpha anchor, operational helper, minimal pattern anchor, docs validators | later public/broader reopen only |

## line snapshots

### Product Alpha line

Status: `product-alpha-ready`

Current evidence:

- `mirrorea-alpha` command family.
- versioned `package.mir.json`.
- local/Docker controlled runtime.
- observer-safe devtools/viewer.
- R0/R2 save evidence.
- native host launch bundle.

Next gap:

- keep as alpha floor while Surface Mir shifts source authority to `.mir` files.

### Operational Suite line

Status: `workflow-ready`

Current evidence:

- `samples/product-alpha1/operational/`
- `WorldCore -> MembershipChat -> SugorokuWorld -> PortalWorldLink -> TwoShardHardBoundary -> TwoShardGradientObservation`
- `python3 scripts/operational_product_samples.py check-all --format json`

Next gap:

- no widening in `P-SURF-00B`; later Surface roots must not overwrite this
  product-alpha compatibility floor.

### Mir Language line

Status: `first-floor-evidence` for Full System V1, `parser-floor-evidence` plus
`indexed-state-checker-evidence` plus `elaboration-evidence` plus
`generated-communication-evidence` plus `role-admission-evidence` plus
`source-patch-hotplug-evidence` plus `source-operational-evidence` plus
`static-devtools-evidence`; Surface audit is closed through P-SURF-99.

Current evidence:

- Full System V1 parser/checker/runtime/projection/provider/release-check line
  remains closed through final audit.
- Legacy LAB Surface Mir evidence docs are `specs/39..43`; canon targets for
  source language, elaboration, conformance, and proof obligations live under
  `mirrorea_canon/spec/` and `mirrorea_canon/theory/`.
- Surface Mir repository memory is `plan/64..68`.
- `crates/mir-ast::surface_alpha` parses canonical `S { ... }`, role-instance
  blocks, `state`, `when`, `join`, record literals, and expected syntax
  rejections.
- `crates/mir-semantics::surface_indexed_state` checks S-owned
  Participant-indexed state declarations, owner/keyspace/value metadata,
  key-not-authority rejection, stale-key rejection, retained-savepoint
  compaction rejection, and nested-place ambient-authority rejection.
- `crates/mir-semantics::surface_to_core_elaboration` generates Core IR
  transitions, remote request rows, generated edges, source spans, and
  obligations for cross-locus indexed reads/writes.
- `crates/mir-semantics::surface_to_core_elaboration` now also emits LAB-only
  `lab_diagnostic_details` for underdeclared generated failure rows, preserving
  legacy `generated_failure_not_declared` diagnostics while exposing
  E-ROW-001/E-ROW-002 candidate canon IDs, missing generated failures, and
  non-final request / failure-row context. `ELAB-10` also carries LAB-only
  `E-ROW-002` / `VisibilityDenied` repair-carrier evidence; `ELAB-13..16`
  carry one `E-ROW-001` singleton `add-to-fails-row` repair payload per base
  remote-request failure atom under the `plan/93` single-edit /
  no-placeholder gate. Exact `ELAB-07` now carries one non-final
  `set_insertion` item under `plan/102`, while Rust-only `plan/103` guard
  tests keep subset / padded / duplicate / multi-request variants from
  receiving the `set_insertion` repair, and `plan/104` keeps distinct
  same-event rows from suppressing each other's exact set repair;
  `ELAB-04` remains the mixed base / `VisibilityDenied` no-repair fence.
- `crates/mir-semantics::surface_to_core_elaboration` also generates
  MessageEnvelope, visible publish/observe, and observer-safe redaction /
  retention rows for P-SURF-04.
- `crates/mir-semantics::surface_role_admission` records role claims,
  admission requests/verdicts, capability grants, witnesses, stale membership
  rejections, and optional hash metadata for P-SURF-05.
- `crates/mir-runtime::surface_source_patch_hotplug` and `mirrorea-alpha`
  source commands route patches through parse/typecheck/elaborate/compatibility
  / admission and produce HotPlugRequest, HotPlugVerdict, Core IR diff, and
  activation_cut report rows for P-SURF-06.
- `samples/full-system-v1-surface/{world-core,membership-chat,sugoroku-world,portal-worldlink,two-shard-hard-boundary,gradient-observation}/`
  and `operational-matrix.json` provide P-SURF-07 source-first operational
  evidence with `.mir` authority and 12 representative positive/negative rows.
- `samples/full-system-v1-surface/devtools/` provides P-SURF-08 static
  observer-safe diagnostics evidence for source, Core IR, indexed state map,
  communication, admission, patch lifecycle, and source spans.

Next gap:

- No current promoted Surface package. Later work should reopen only with an
  explicit package for final runtime/transport, final devtools viewer/telemetry
  ABI, final source patch ABI, public grammar/API, or broader distribution.

### PoseGraph line

Status: `first-floor-evidence`

Current evidence:

- Product Alpha PoseGraph helper evidence remains bounded.
- Full System V1 avatar-pose runtime/save/devtools evidence remains bounded.

Next gap:

- Surface Mir can later provide source-facing PoseGraph roots, but renderer /
  Unity / UE / WASM / native remain providers, not semantic owners.

### Projection/Backend line

Status: `first-floor-evidence`

Current evidence:

- Full System V1 projection IR / packet schema / FFI schema / local role split
  evidence remains bounded.

Next gap:

- Surface elaboration must preserve generated Core IR and boundary schemas
  before any backend widening.

### Engine/Provider line

Status: `first-floor-evidence`

Current evidence:

- bounded provider admission and renderer pose backend evidence exists under
  Full System V1.

Next gap:

- Surface Mir must preserve provider non-ownership and disabled/inventory
  defaults for native/WASM unless a later explicit package admits more.

### Surface Mir line

Status: `parser-floor-evidence` + `indexed-state-checker-evidence` +
`elaboration-evidence` + `generated-communication-evidence` +
`role-admission-evidence` + `source-patch-hotplug-evidence` +
`g1-erow-carrier-evidence`

Current evidence:

- `specs/39-surface-mir-placement-elaboration.md`
- `specs/40-indexed-state-semantics.md`
- `specs/41-role-admission-and-capability-grant.md`
- `specs/42-source-patch-hotplug-semantics.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/64..68`
- `crates/mir-ast/src/surface_alpha.rs`
- `crates/mir-semantics/src/surface_indexed_state.rs`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/src/surface_role_admission.rs`
- `crates/mir-runtime/src/surface_source_patch_hotplug.rs`
- `samples/full-system-v1-surface/syntax/matrix.json`
- `samples/full-system-v1-surface/indexed-state/matrix.json`
- `samples/full-system-v1-surface/elaboration/matrix.json`
- `samples/full-system-v1-surface/role-admission/matrix.json`
- `samples/full-system-v1-surface/source-patch/matrix.json`
- `samples/full-system-v1-surface/devtools/matrix.json`
- `scripts/surface_mir_samples.py`

Next gap:

- no current promoted final/public Surface package; later G1 LAB work should
  continue only as non-final diagnostic/proof-boundary evidence unless a
  runtime/transport/public-ABI/devtools-viewer package is explicitly promoted.

## validation floor

Required for the current Surface package close:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
cargo test -p mir-ast --test surface_mir_parser -- --nocapture
cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo test -p mir-semantics --test role_admission_capability_grant -- --nocapture
cargo test -p mir-runtime --test source_patch_hotplug -- --nocapture
cargo test -p mirrorea-cli --test surface_mir_cli -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release
```

Compatibility anchors when environment permits:

```bash
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
```

## non-claims

- No final public grammar completion.
- No runtime MessageEnvelope dispatch, local queue delivery, or final transport
  implementation yet.
- No TypeMismatch typechecker discharge for generated communication yet.
- No production identity provider, hardware attestation, or WAN admission.
- No indexed-state runtime carrier or distributed compaction protocol yet.
- No final source patch hot-plug ABI, distributed durable migration planner,
  production patch registry/signing workflow, or arbitrary native/WASM
  execution through patches.
- No final Surface operational runtime / transport or shared-space catalog
  completion yet; P-SURF-07 is source operational evidence only.
- No Rust-level language completion.
- No LLVM/native codegen completion.
- No final server/client split compiler completion.
- No arbitrary native/WASM/Unity/UE provider execution.
- No production WAN/federation.
- No distributed durable save-load R3/R4.
- No final public ABI / SDK.
- No final shared-space catalog breadth decision.

## user decision items vs research-discovery items

User decision items:

- final public grammar and compatibility window.
- final ABI / SDK / engine adapter public surface.
- broader distribution beyond developer-built binary plus generated host launch
  bundle.
- final shared-space catalog breadth.
- production WAN/federation and R3/R4 durable distributed save-load.

Research-discovery items:

- indexed-state owner/keyspace/access/stale runtime carrier.
- Surface-to-Core obligation carrier shape.
- role admission capability grant carrier.
- indexed-state tombstone / compaction runtime carrier.
- role admission witness metadata shape.
- source patch compatibility diff and activation-cut carrier.

## macro phase map

| Macro | Focus | Current position | Weight | Self-drive |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | Surface alpha audit closed; no current promoted Surface package | light | maintenance only |
| `Macro 1` | semantic kernel / invariant / boundary stabilization | Surface authority / placement / indexed state / admission / patch boundaries fixed | medium | 着手可能 |
| `Macro 2` | parser-free validation substrate | existing alpha/product helpers remain compatibility anchors | medium | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | parser, indexed-state checker, elaboration, generated communication, role admission, source patch, source operational, static devtools diagnostics, and final audit floors closed | heavy | maintenance only |
| `Macro 4` | executable sample expansion | Surface operational and devtools roots exist as alpha source evidence; final audit closed | heavy | maintenance only |
| `Macro 5` | theorem / model-check / verifier bridge | LAB OBL-001, OBL-020, OBL-021, OBL-024, and OBL-025 statement-shape drafts now compile, but Surface elaboration soundness / WF preservation / elaboration determinism / diagnostic soundness / repair completeness are not proved or ledger-moved | medium | 着手可能 |
| `Macro 6` | distributed fabric / runtime evolution | local/Docker alpha remains floor | heavy | 後段依存 |
| `Macro 7` | toolchain / backend / developer surface | Surface parser / indexed-state / elaboration helper commands exist; product alpha CLI remains compatibility floor | heavy | 着手可能 |
| `Macro 8` | domain / application realization | Surface WorldCore/MembershipChat/Sugoroku/Portal/TwoShard/Gradient roots are alpha source evidence; final runtime/catalog remain later | heavy | 後段依存 |

## feature maturity rows

| Feature | Status | Reading | Actionability |
|---|---|---|---|
| Surface Mir brace syntax | `parser-floor-evidence` | canonical `S { ... }` parses; `S[ ... ]` rejects with `bracket_place_scope_not_supported`; no sugar | 着手可能 |
| textual Mir source | `first-floor-evidence` | Full System V1 parser exists; Surface parser floor now exists separately | 着手可能 |
| typed IR / checker | `first-floor-evidence` | existing Full System V1 checker remains floor | 着手可能 |
| Surface-to-Core elaboration | `elaboration-evidence` | cross-locus indexed reads/writes lower to explicit Core IR remote requests, RHS indexed reads on remote writes now record dependency rows, generated edges, source spans, obligations, and LAB-only E-ROW diagnostic details with request / failure-row context plus OBL-024 `diagnostic_soundness_projection` carrier evidence for `ELAB-04/07/10/13..16`, Rust fixture guards for those projection-bearing rows, and docs-first replay vocabulary separating report-local anchors from future proof-level replay relations, `E-ROW-002` / `VisibilityDenied` repair evidence, `E-ROW-001` non-visibility singleton repair evidence for all base remote-request failure atoms, one exact `ELAB-07` non-final set-insertion repair payload, Rust-only guards that withhold the `set_insertion` repair for subset / padded / duplicate / multi-request `ELAB-07` variants, span-based internal association hardening for distinct same-event `when` rows, exact-locus hardening for omitted-row / retargeting proxies, child / bundle / partial / textual guidance exclusion shape assertions, explicit no-repair decomposition inventory for mixed / multi-missing rows, docs-only set-insertion / bundle payload vocabulary, an `ELAB-07` gate review / executable preflight / narrow source-locus edit assumption acceptance / payload-model design / executable prototype / negative-guard / row-identity / exact-locus / child-bundle-partial exclusion hardening, and an `ELAB-04` mixed visibility branch inventory plus payload-model preflight that keeps executable output no-repair while recording mixed wrapper / base branch / visibility branch / association / ordering deferrals | 着手可能 |
| indexed state | `semantic-checker-evidence` | S-owned Participant-indexed map accepted; key-as-authority, stale key, retained-savepoint compaction, and nested-place ambient-authority negatives reject | 着手可能 |
| auto communication / publish / observe | `generated-communication-evidence` | generated MessageEnvelope / visible publish / observe rows and `VisibilityDenied` failure containment exist in Core IR; runtime dispatch remains later | 着手可能 |
| role admission / capability grant | `role-admission-evidence` | role claim, join admission request, capability grant-backed accepted write, witness, stale rejection with a post-stale write fence, and hash metadata rows exist; runtime identity/admission lifecycle remains later | 着手可能 |
| source patch hot-plug | `source-patch-hotplug-evidence` | parse/typecheck/elaborate/compatibility/admission report, HotPlugRequest/HotPlugVerdict, Core IR diff, activation_cut, no-direct-eval and rejection-without-mutation evidence exist; final ABI/migration planner later | 着手可能 |
| Surface source operational suite | `source-operational-evidence` | `E2E-SURF-01..12` cover WorldCore, MembershipChat, Sugoroku, PortalWorldlink, TwoShardHardBoundary, and GradientObservation positive/negative source rows through required alpha checks | 着手可能 |
| Surface devtools diagnostics | `static-devtools-evidence` | `DEV-01..02` expose required Surface source/Core IR/semantic-checker-backed indexed-state/communication/admission/redacted patch/source-span panels without final viewer/telemetry ABI claims | 着手可能 |
| Product Alpha | `product-alpha-ready` | bounded alpha workflow, not final product | maintenance only |
| operational suite | `workflow-ready` | bounded local/Docker suite remains compatibility anchor | maintenance only |
| projection/backend | `first-floor-evidence` | bounded projection/provider evidence remains lower floor | 着手可能 |
| LAB Lean statement drafts | `lab-compile-check-only` | OBL-001 `THM001StatementDraft.lean`, OBL-020 `StepWFStatementDraft.lean`, OBL-021 `ElabDeterminismStatementDraft.lean`, OBL-024 `DiagnosticSoundnessStatementDraft.lean`, and OBL-025 `RepairCompletenessStatementDraft.lean` compile as LAB `Prop` shapes under `samples/lean/lab-statements/`; OBL-024 now has abstract diagnostic projection / report-local association key / future proof-level association relation / report-local replay anchor / future proof-level replay relation / mixed diagnostic branch predicates without proof or canon ledger status, and OBL-025 has abstract whole-rejected-gap / set-insertion / grouped multi-edit / complete-local-repair / partial-guidance non-coverage / branch-local non-coverage predicates without proof or canon ledger status | 着手可能 |

## recent log

Entries below are historical snapshots at each task close. Earlier
`ELAB-07` no-repair lines record the state before `plan/102`; the current
snapshot above is the exact `ELAB-07` set payload, `plan/103` Rust-only
negative guards, `plan/104` span-based internal association hardening,
`plan/105` exact-locus guard hardening, `plan/106` child / bundle / partial
exclusion fixtures, `ELAB-04` no-repair payload-model preflight, OBL-025
branch-local non-coverage refinement, OBL-024 diagnostic-soundness Lean
statement draft, OBL-024 executable projection carrier, OBL-024 Rust fixture
guard hardening, OBL-024 replay vocabulary preflight, and OBL-024 Lean replay
vocabulary refinement, plus OBL-024 Lean association vocabulary refinement.

- 2026-07-04 09:04 JST
  `plan/114-g1-obl024-lean-association-vocabulary-refinement.md` を追加し、
  `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
  の association vocabulary を `ReportLocalAssociationKey` と
  `ProofLevelAssociationWitness` / `ProofLevelAssociationRelation` に分けた。
  Lean compile と sync unit test で確認したが、OBL-024 proof / completion、
  canon ledger movement、final Diagnostic / association-key / replay ABI、
  runtime JSON、repair output、conformance、G1 exit は主張しない。

- 2026-07-04 08:44 JST
  `plan/113-g1-obl024-lean-replay-vocabulary-refinement.md` を追加し、
  `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
  の replay vocabulary を `ReportLocalReplayAnchor` と
  `ProofLevelReplayWitness` / `ProofLevelReplayRelation` に分けた。Lean
  compile、sync unit test、`current_l2_lean_sample_sync.py` で確認したが、
  OBL-024 proof / completion、canon ledger movement、final Diagnostic /
  replay ABI、runtime JSON、repair output、conformance、G1 exit は主張しない。

- 2026-07-04 08:29 JST
  `plan/112-g1-obl024-replay-vocabulary-preflight.md` を追加し、current
  `trace_local_replay` を report-local consistency anchor として扱い、future
  proof-level replay relation / exactness / mixed branch replay boundary を
  OPEN vocabulary として分離した。production code、expected JSON、Lean file、
  canon、repair output は変更せず、final replay ABI / Diagnostic ABI、
  OBL-024 proof / completion、conformance、G1 exit は主張しない。

- 2026-07-04 08:12 JST
  `plan/111-g1-obl024-projection-rust-fixture-guards.md` を追加し、
  `diagnostic_soundness_projection` の Rust guard を強化した。`ELAB-04/07/10`
  の実 sample fixture を Rust から直接読み、既存の `ELAB-13..16` fixture loop
  と合わせて `ELAB-04/07/10/13..16` の projection / context alignment と
  skipped internal association fields の非 serialization を確認する。production
  emission、expected JSON、repair output、OBL-024 proof / completion、final
  Diagnostic JSON / replay ABI、conformance、G1 exit は主張しない。

- 2026-07-04 07:48 JST
  `plan/110-g1-obl024-executable-projection-carrier.md` を追加し、
  `ELAB-04/07/10/13..16` の `lab_diagnostic_details` に LAB-only
  `diagnostic_soundness_projection` を追加した。projection は helper-local
  diagnostic id / association key / reported bindings / report-local
  trace-local replay anchor を固定するが、OBL-024 proof / completion、canon
  ledger movement、final Diagnostic JSON / request ID / association-key ABI /
  replay semantics、repair output widening、`ELAB-04` payload output、
  conformance、G1 exit は主張しない。

- 2026-07-04 07:22 JST
  `plan/109-g1-obl024-lean-statement-draft.md` と
  `samples/lean/lab-statements/obl024/DiagnosticSoundnessStatementDraft.lean`
  を追加し、diagnostic projection / reported rule / failed premise / bindings /
  association key / trace-local replay / non-repair mixed diagnostic branch
  boundary を compile-check-only `Prop` として置いた。OBL-024 proof /
  completion、canon ledger movement、final Diagnostic ABI、request ID /
  branch ID / association-key ABI、root-cause uniqueness、repair output
  widening、conformance、G1 exit は主張しない。

- 2026-07-04 07:00 JST
  `plan/108-g1-obl025-branch-local-noncoverage-refinement.md` を追加し、
  OBL-025 Lean statement draft に abstract `RepairBranch` / branch-local
  witness / branch-local suggestion vocabulary と non-coverage helper を足した。
  `ELAB-04` の base branch / visibility branch は classification / guidance
  pressure として扱えるが、whole rejected-gap coverage ではない。proof、
  canon ledger movement、executable repair widening、`ELAB-04` payload output、
  final branch ID / JSON / repair ABI、conformance、G1 exit は主張しない。

- 2026-07-04 06:31 JST
  `plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md` を追加し、
  `ELAB-04` の mixed wrapper / base remote-request branch /
  `VisibilityDenied` branch / association vocabulary / ordering-ranking
  deferral を docs-only で整理した。executable output、expected JSON、sample
  row count は変えず、`ELAB-04` は no-repair のまま。set-insertion support、
  bundle semantics、partial guidance output、visibility-repair ranking、
  OBL-025 proof / completion、final repair ABI、conformance、G1 exit は主張しない。

- 2026-07-04 06:07 JST
  `plan/106-g1-erow07-child-bundle-partial-exclusion-fixtures.md` を追加し、
  exact `ELAB-07` の current repair output が one complete top-level
  `set_insertion` item であり、three child singleton alternatives、
  conjunctive bundle fields、partial guidance、textual-only guidance ではない
  ことを Rust-only shape guard で固定した。sample row count / expected JSON /
  production repair emission logic は変えていない。bundle semantics support、
  partial-guidance output、general set-insertion support、OBL-025 proof /
  completion、canon ledger movement、final repair ABI、conformance、G1 exit は
  主張していない。
- 2026-07-04 05:43 JST
  `plan/105-g1-erow07-set-insertion-exact-locus-guard-hardening.md` を追加し、
  current Surface-expressible omitted-row / event retargeting / role
  retargeting / owner-locus retargeting / state-name retargeting / field
  retargeting proxies に exact `ELAB-07` の
  `set_insertion` repair を出さない Rust-only guards を追加した。set path は
  `role:BrowserClient` / `attack` / `S.player.hp` の current exact locus に
  限定され、public JSON と sample row count は変えていない。final
  source-locus identity model、final row identity model、general
  set-insertion support、OBL-025 proof / completion、canon ledger movement、
  final repair ABI、conformance、G1 exit は主張していない。
- 2026-07-04 05:22 JST
  `plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md` を追加し、
  public `target_ref` を維持したまま internal association key を existing
  `when` source span で狭めた。同一 role/event の別 `when` rows は互いの
  exact `set_insertion` repair を suppress せず、同一 `when` row 内の
  multi-request は引き続き suppress する。sample row count は 52 のまま。
  final row identity model、general set-insertion support、OBL-025 proof /
  completion、canon ledger movement、final repair ABI、conformance、G1 exit は
  主張していない。
- 2026-07-04 04:58 JST
  `plan/103-g1-erow07-set-insertion-negative-guard-hardening.md` を追加し、
  exact `ELAB-07` の set path を保ったまま、proper subset / padded /
  duplicate / multi generated-request variants では `set_insertion` repair を
  出さない Rust-only guard tests と内部 associated-request count guard を追加
  した。tested rows は `suggested_repair` を持たない。
  sample row count は 52 のまま。general set-insertion support、bundle
  semantics support、OBL-025 proof / completion、canon ledger movement、
  final repair ABI、conformance、G1 exit は主張していない。
- 2026-07-04 04:14 JST
  `plan/102-g1-erow07-set-insertion-executable-payload-prototype.md` を追加し、
  exact `ELAB-07` に one top-level non-final `set_insertion`
  `suggested_repair[]` を実装した。`ELAB-04` は no-repair、
  `ELAB-10` / `ELAB-13..16` は singleton repair のまま、sample row count は
  52 のまま。general set-insertion support、bundle semantics support、
  OBL-025 proof / completion、canon ledger movement、final repair ABI、
  conformance、G1 exit は主張していない。
- 2026-07-04 03:48 JST
  `plan/101-g1-erow07-set-insertion-payload-model-design.md` を追加し、
  `ELAB-07` の future set payload を one top-level set item、no singleton
  `missing_failure` field reuse、exact whole-gap guards、future positive /
  negative test matrix として docs-only で設計した。この時点では executable
  output をまだ広げず、repair output widening、executable set-insertion
  support、bundle semantics support、OBL-025 proof / completion、canon ledger
  movement、final repair ABI、conformance、G1 exit は主張していない。
- 2026-07-04 03:24 JST
  `plan/100-g1-erow07-set-insertion-assumption-acceptance.md` を追加し、
  `ELAB-07` だけについて one existing `when_fails_row` source-locus edit /
  `element_insert_count = 3` として exact missing base-failure set insertion
  を LAB gate で受け入れた。この時点では executable output をまだ広げず、
  repair output widening、set-insertion support、bundle semantics support、
  OBL-025 proof / completion、canon ledger movement、final repair ABI、
  conformance、G1 exit は主張していない。
- 2026-07-04 03:00 JST
  `plan/99-g1-erow07-set-insertion-executable-preflight.md` を追加し、
  `ELAB-07` の future widening に必要な atomic set-insertion / one target /
  no-extras / exact whole rejected-gap coverage / focused test predicates を
  docs-only で整理した。executable `suggested_repair[]` は広げず、
  set-insertion support、bundle semantics support、OBL-025 proof /
  completion、canon ledger movement、final repair ABI、conformance、G1 exit は
  主張していない。
- 2026-07-04 02:42 JST
  `plan/98-g1-erow04-mixed-visibility-branch-inventory.md` を追加し、
  `ELAB-04` は base remote-request branch と `VisibilityDenied` branch が
  混在する no-repair row として扱う gate を整理した。executable
  `suggested_repair[]` は広げず、diagnostic ownership、branch association、
  ordering / ranking、visibility-repair ranking、set-insertion support、
  bundle semantics support、OBL-025 proof / completion、canon ledger movement、
  final repair ABI、G1 exit は主張していない。
- 2026-07-04 02:10 JST
  `plan/97-g1-erow07-set-insertion-gate-review.md` を追加し、この時点では
  `ELAB-07` の repair output widening を保留すると整理した。当時は singleton
  repair だけを持ち、`ELAB-07` の multi-missing row は set-insertion atomicity
  または bundle semantics と whole rejected-gap coverage が決まるまで
  executable `suggested_repair[]` に広げない。repair output widening、
  set-insertion support、bundle semantics support、OBL-025 proof / completion、
  canon ledger movement、final repair ABI、G1 exit は主張していない。
- 2026-07-04 01:54 JST
  `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
  を compile-check-only のまま refinement し、whole rejected gap、
  set-insertion、grouped multi-edit、complete local repair、
  partial-guidance non-coverage の抽象 predicate / helper relation を追加した。
  set-insertion は current single-edit relation を満たす場合だけ OBL-025
  coverage に入り、grouped multi-edit / partial guidance は current coverage
  外として保持した。OBL-025 proof / completion、canon ledger movement、final
  repair ABI、repair ranking、multi-edit support、G1 exit は主張していない。
- 2026-07-04 01:27 JST
  `plan/96-g1-erow-set-insertion-bundle-payload-inventory.md` を追加し、
  set-insertion / conjunctive bundle / partial guidance の候補 payload
  vocabulary を docs-only で整理した。この時点では `ELAB-04/07` の
  executable output を広げていない。set-insertion support、bundle semantics
  support、repair output widening、diagnostic/repair ABI freeze、OBL-025 proof
  / completion、repair ranking、multi-edit support、conformance、G1 exit は
  主張していない。
- 2026-07-04 01:14 JST
  `plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md` を追加し、
  `ELAB-07` は set-insertion / conjunctive bundle / partial-repair /
  multi-edit deferral の未決、`ELAB-04` はそれに加えて visibility split と
  ranking / ordering の未決として no-repair policy を整理した。repair output
  widening、diagnostic/repair ABI freeze、OBL-025 proof / completion、
  repair ranking、multi-edit support、conformance、G1 exit は主張していない。
- 2026-07-04 00:58 JST
  `plan/94-g1-erow001-singleton-repair-prototype.md` を追加し、
  `ELAB-13..16` を LAB-only `E-ROW-001` singleton `add-to-fails-row`
  repair-bearing evidence に widened した。`ELAB-04/07` は mixed /
  multi-missing no-repair fence のまま。diagnostic/repair ABI freeze、
  OBL-025 proof / completion、repair ranking、multi-edit support、
  conformance、G1 exit は主張していない。
- 2026-07-04 00:38 JST
  `plan/93-g1-erow001-singleton-repair-assumption.md` を追加し、
  non-visibility singleton `E-ROW-001` repair prototype の LAB-only
  single-edit assumption と no-placeholder payload gate を固定した。
  Python / Rust の guard tests は既存 `ELAB-10` repair payload が local
  witness-compatible で placeholder ではないこと、`ELAB-13..16` が
  target/request context を持つ no-repair fence のままであることを確認する。
  repair output widening、diagnostic/repair ABI freeze、OBL-025 proof /
  completion、conformance、G1 exit は主張していない。
- 2026-07-04 00:18 JST
  `ELAB-14..16` と `plan/92-g1-erow001-base-singleton-fixture-closure.md` を
  追加し、`MissingCapability` / `RouteUnavailable` / `StaleMembership`
  singleton omissions を `E-ROW-001` no-repair evidence として固定した。
  `ELAB-13` と合わせて base remote-request failure atom 4 種の singleton
  no-repair fixture set が揃った。Surface helper は 52 rows / 53 `.mir`
  sources に増えたが、repair output widening、OBL-025 proof / completion、
  diagnostic/repair ABI freeze、repair ranking、multi-edit support、
  conformance、G1 exit は主張していない。
- 2026-07-04 00:04 JST
  `ELAB-13` と `plan/89-g1-erow001-non-visibility-singleton-fixture.md` を
  追加し、non-visibility singleton `MissingWitness` omission を
  `E-ROW-001` no-repair evidence として固定した。Surface helper は 49 rows /
  50 `.mir` sources に増えたが、repair output widening、OBL-025 proof /
  completion、diagnostic/repair ABI freeze、repair ranking、multi-edit support、
  conformance、G1 exit は主張していない。
- 2026-07-03 23:44 JST
  `plan/88-g1-erow-repair-shape-inventory.md` を追加し、repair widening 前の
  E-ROW taxonomy を整理した。現行 repair-bearing evidence は `ELAB-10`
  `E-ROW-002` / `VisibilityDenied` singleton のみで、`ELAB-07` は
  non-visibility multi-missing no-repair、`ELAB-04` は mixed
  visibility/non-visibility multi-missing no-repair として保持した。repair
  output widening、OBL-025 proof / completion、diagnostic/repair ABI freeze、
  repair ranking、multi-edit support、conformance、G1 exit は主張していない。
- 2026-07-03 23:28 JST
  `samples/lean/lab-statements/obl025/RepairCompletenessStatementDraft.lean`
  と `plan/87` を追加し、OBL-025 を compile-check-only の existential
  repair-coverage `Prop` として記録した。`statement_drafts` manifest 区分を
  OBL-025 まで広げたが、OBL-025 proof / completion、diagnostic/repair ABI
  freeze、repair ranking、multi-edit support、conformance、G1 exit は主張していない。
- 2026-07-03 23:16 JST
  Oracle advisory review を反映し、LAB-only `suggested_repair[]` を
  `E-ROW-002` / `VisibilityDenied`-only に限定した。repair item は
  `target_ref`、target row、missing failure、required / declared failures、
  local effect、local premise、local single-row-addition assumption、non-goal、
  `repair_non_final`、`lab_non_final` を持つが、diagnostic/repair ABI freeze、
  OBL-024/025 discharge、explanation completeness、conformance、G1 exit は
  主張していない。
- 2026-07-03 22:50 JST
  `lab_diagnostic_details` に LAB-only `request_context` と
  `failure_row_context` を追加し、`ELAB-04` / `ELAB-07` / `ELAB-10` の
  expected JSON と Rust/Python tests に同期した。これは repair-bearing 前の
  row-containment precondition carrier hardening であり、`suggested_repair[]`、
  diagnostic/repair ABI freeze、OBL-024/025 discharge、conformance、G1 exit は
  主張していない。
- 2026-07-03 22:37 JST
  `lab_diagnostic_details` を Surface-to-Core elaboration report / example
  JSON / Surface helper projection に追加し、`ELAB-04` mixed E-ROW omission と
  `ELAB-07` clean non-visibility omissionを E-ROW-001、`ELAB-10`
  visibility-only omissionを E-ROW-002 として expected JSON に記録した。
  legacy `generated_failure_not_declared` と `diagnostic_codes` は維持し、
  `suggested_repair[]`、diagnostic/repair ABI freeze、OBL-024/025 discharge、
  conformance、G1 exit は主張していない。
- 2026-07-03 17:38 JST
  `mirrorea_canon/` を canon-first source hierarchy として読み込み、local
  checks、sub-agent review、Oracle consult で整合性を確認した。canon 内部は
  T0/G0 の大局正本として採用可能だが、既存 LAB 文書が旧 `specs/` 正本表現を
  持っていたため、root `CANON.md`、entry banner、validator guardrail、
  clean-near-end / highlighter の旧 `world` 語彙注記を追加した。次は
  LAB-to-canon reconciliation ledger と G1 ordinary assignment target。
- 2026-07-03 18:38 JST
  `plan/70-lab-to-canon-reconciliation-ledger.md` を追加し、legacy LAB claim
  family を canon anchor / rejected historical pattern / OPEN follow-up に
  照合した。G0 exit は主張せず、次の安全な自走 package を G1 ordinary
  assignment target draft として整理した。
- 2026-07-03 19:55 JST
  `plan/71-g1-ordinary-assignment-target.md` を追加し、G1 ordinary simple assignment
  の target/proof-boundary split を THM-001、BND-001、SCN-01、SCN-02、
  OBL-001/020/021 に絞って整理した。G1 exit、theorem discharge、
  runtime dispatch、final grammar/API freeze は主張していない。次は
  SCN-01/02 static consequence drilldown または OBL-001 Lean statement inventory。
- 2026-07-03 20:05 JST
  `plan/72-g1-scn01-scn02-static-consequence-drilldown.md` を追加し、SCN-01/SCN-02 の
  C-static 期待を canon target、LAB ELAB support、LAB gap、runtime/proof
  boundary に分解した。C-static conformance、G1 exit、theorem discharge は
  主張していない。次は OBL-001 Lean statement inventory。
- 2026-07-03 20:21 JST
  `plan/73-g1-obl001-lean-statement-inventory.md` を追加し、THM-001 / OBL-001 の
  repo-local Lean statement に必要な最小 datatypes / predicates / theorem shape /
  SCN row coverage / overfit guard を整理した。Lean statement file、OBL status
  movement、G1 exit、theorem discharge は主張していない。次は actual OBL-001
  statement draft または SCN dependency-gap package。
- 2026-07-03 20:41 JST
  `samples/lean/lab-statements/obl001/THM001StatementDraft.lean` と `plan/74` を
  追加し、THM-001 / OBL-001 の LAB-only statement-shape draft を Lean
  compile-check 可能な `Prop` として記録した。`statement_drafts` manifest 区分を
  追加したが、OBL status movement、proof discharge、G1 exit、conformance は
  主張していない。次は SCN dependency-gap、OBL-020/021 inventory、または
  focused statement refinement。
- 2026-07-03 21:10 JST
  `SurfaceCoreIr.dependencies` と `rhs_indexed_read` LAB dependency rows を追加し、
  `ELAB-11` で SCN-01 same-field RHS、`ELAB-12` で SCN-02 target/self RHS
  dependency evidence を固定した。Surface helper は 48 rows / 49 `.mir`
  sources に増え、Rust elaboration test、Surface helper unit tests、
  source hierarchy / docs validators、`surface_mir_samples.py check-all` 48/48
  を確認したが、C-static conformance、OPEN-014 materialization、proof
  discharge、G1 exit は主張していない。次は OBL-020/021 dependency inventory
  または diagnostic alignment。
- 2026-07-03 21:19 JST
  `plan/76-g1-obl020-021-dependency-inventory.md` を追加し、OBL-020
  well-formedness preservation と OBL-021 elaboration determinism の依存関係を
  OBL-001/002 から分離して整理した。これは inventory-only で、OBL 完了、
  Lean statement、proof skeleton、G1/T1/T2 exit、conformance は主張していない。
  次は separate LAB Lean statement-shape draft、OBL-001 refinement、または
  E-ROW diagnostic alignment。
- 2026-07-03 21:30 JST
  `samples/lean/lab-statements/obl021/ElabDeterminismStatementDraft.lean` と
  `plan/77` を追加し、OBL-021 elaboration determinism の LAB-only
  statement-shape draft を Lean compile-check 可能な `Prop` として記録した。
  `SameElabResult` / `SameDiagnostic` により equality relation と diagnostic
  ABI は抽象のままにし、OBL status movement、proof discharge、G1/T1/T2 exit、
  conformance は主張していない。次は OBL-020 statement-shape draft、
  focused statement refinement、または E-ROW diagnostic alignment。
- 2026-07-03 21:41 JST
  `samples/lean/lab-statements/obl020/StepWFStatementDraft.lean` と `plan/78` を
  追加し、OBL-020 well-formedness preservation の LAB-only statement-shape
  draft を Lean compile-check 可能な aggregate `Prop` として記録した。
  WF clause は `WellFormed` の背後に残し、proof skeleton、OBL completion、
  G1/T1/T2 exit、conformance、runtime implementation proof、canon ledger
  movement は主張していない。次は focused statement refinement または
  E-ROW diagnostic alignment。
- 2026-07-03 21:47 JST
  `plan/79-g1-erow-diagnostic-alignment.md` を追加し、canon E-ROW-001 /
  E-ROW-002 と LAB `generated_failure_not_declared` evidence の対応を整理した。
  ELAB-04/07 は E-ROW-001 evidence、ELAB-10 は同じ helper-local diagnostic
  family で表現された E-ROW-002 pressure evidence として読み、diagnostic ABI
  freeze、OBL-024/025 discharge、C-static conformance、G1 exit は主張していない。
- 2026-07-03 21:54 JST
  `plan/80-g1-diagnostic-carrier-inventory.md` を追加し、canon Diagnostic carrier
  fields と current LAB `TextualMirDiagnostic` / Surface helper projection /
  expected JSON の差分を整理した。`code/message/span` と `diagnostic_codes`
  evidence は現状の足場として扱い、diagnostic ABI freeze、OBL-024/025
  statement/proof discharge、explanation soundness/completeness、conformance、
  G1 exit は主張していない。
- 2026-07-03 22:05 JST
  `plan/81-g1-obl024-statement-shape-inventory.md` を追加し、OBL-024
  explanation soundness を emitted Diagnostic / reported rule instance /
  failed premise / bindings / replay failure の statement-shape inventory として
  整理した。E-ROW は immediate pressure case として読むが、Lean statement、
  proof、diagnostic ABI freeze、conformance、G1 exit は主張していない。
- 2026-07-03 22:14 JST
  `plan/82-g1-obl025-statement-shape-inventory.md` を追加し、OBL-025
  explanation completeness を Line-1 rejection / declared fragment /
  single-edit repair existence / non-empty suggested repair /
  repair-failure matching の statement-shape inventory として整理した。
  repair generation、repair ABI freeze、proof、conformance、G1 exit は主張していない。
- 2026-07-03 22:22 JST
  `plan/83-g1-erow-repair-payload-inventory.md` を追加し、future E-ROW
  diagnostic prototype が `suggested_repair[]` を含む場合の non-final payload
  vocabulary を整理した。repair generation、diagnostic/repair ABI freeze、
  OBL-024/025 proof、explanation completeness、conformance、G1 exit は主張していない。
- 2026-07-02 18:03 JST
  Oracle 運用メモを更新し、理論的に難しい判断、全体像、roadmap、複雑な
  design review では積極的に Oracle consult を投げる方針を明記した。長時間
  実行は非同期 reviewer として扱い、必要なら sub-agent に起動 / 監視を任せつつ、
  採否と repo への mirror は main agent が source hierarchy に照らして判断する。
- 2026-07-02 17:52 JST
  ChatGPT 5.5 Pro Extended Oracle browser consults の repo-local 運用メモを
  `.docs/oracle-chatgpt-pro-operations.md` として追加し、`AGENTS.md`、
  `Documentation.md`、`plan/00-index.md` から導線を張った。Oracle は advisory
  review input であり、規範正本ではない。実行は分単位で待ち、遅い場合も
  重複起動せず `oracle status` / `oracle session` で確認する運用にした。
- 2026-06-25 18:10 JST
  相談会話と repo 基礎文書を照合し、post-`P-SURF-99` の source-first
  management synthesis を `plan/69` として非規範の repository memory に保存した。
  新しい promoted package や `specs/` 判断は作らず、ordinary assignment
  elaboration、Surface/Core/Trace/Projection separation、World/Event 非 core
  primitive、ledger 管理案を今後の検討候補として切り分けた。
- 2026-06-25 17:49 JST
  `current_l2_lean_sample_sync.py` の clean-near-end `source_path` 出力を repo-relative に変更し、`samples/lean/clean-near-end/*/*.bundle.json` と各 README の source reference を `samples/...` 相対パスへ再生成した。Lean sync は idempotent に通り、active Lean files、Rust actual Lean probe、`make check`、`git diff --check`、absolute path scan を確認した。
- 2026-06-25 17:38 JST
  `elan` 経由で repo の `lean-toolchain` に合わせた Lean 4.29.1 / Lake 5.0.0 を導入し、active `samples/lean/foundations` / `samples/lean/clean-near-end` と historical `samples/lean/old` を含む 32 Lean files、Lean sync helper、Lean theorem-stub pipeline、Python Lean unit tests、Rust Lean actual probe を確認した。新規 disk 使用量は filesystem 差分で 2,851,463,168 bytes、約 2.66 GiB、`~/.elan` 実体で 2,819,461,716 bytes、約 2.63 GiB。
- 2026-06-25 17:12 JST
  broad build/execution audit と clippy hardening を実施し、`cargo fmt --check`、`cargo check --workspace --all-targets`、`cargo build --workspace --all-targets`、`cargo test --workspace --all-targets --no-fail-fast`、`cargo clippy --workspace --all-targets -- -D warnings`、`python3 -m unittest discover -s scripts/tests`、主要 sample / release check / `mirrorea-alpha` demo 起動確認を通した。Lean stub artifact pipeline は `/tmp` 出力で通ったが、`lean` / `lake` / `elan` は PATH 不在のため compiler mechanization check は未実行。
- 2026-05-24 20:42 JST
  `P-SURF-99` で final Surface alpha audit を close し、Surface release check、Surface helper / authoring check、Product Alpha release anchor、operational product helper、minimal alpha-1 pattern verifier、docs validators、`cargo fmt --check`、`git diff --check` を再実行した。Surface line は 46 helper rows / 47 `.mir` source files の bounded alpha evidence として閉じ、final runtime/transport、final source patch ABI、final devtools viewer/telemetry ABI、public grammar/API は later gate のまま。
- 2026-05-24 20:13 JST
  `P-SURF-08` で static devtools diagnostics evidence floor を actualize し、`samples/full-system-v1-surface/devtools/`、`DEV-01..02`、`scripts/surface_mir_samples.py check-all` を同期した。panel は Surface source / generated Core IR / semantic-checker-backed indexed-state map / generated communication / role admission / redacted patch lifecycle / source spans を持つが、final viewer / telemetry ABI や runtime devtools dispatch ではない。then-promoted package は `P-SURF-99 final surface alpha audit`。
- 2026-05-24 19:58 JST
  `P-SURF-07` で source operational suite evidence floor を actualize し、`samples/full-system-v1-surface/world-core/`、`membership-chat/`、`sugoroku-world/`、`portal-worldlink/`、`two-shard-hard-boundary/`、`gradient-observation/`、`operational-matrix.json`、`E2E-SURF-01..12`、`scripts/surface_mir_samples.py check-all` を同期した。review 後に MembershipChat row も role admission と elaboration / generated communication を同時に通し、operational projection の `source_authority` / `final_public_api_frozen` は下位 semantic payload 由来にした。各 root は `.mir` source authority の positive / negative rows を持つが、final operational runtime / transport ではない。then-promoted package は `P-SURF-08 devtools and diagnostics`。
- 2026-05-24 19:27 JST
  `P-SURF-06` で source patch hot-plug evidence floor を actualize し、`surface_source_patch_hotplug`、`mirrorea-alpha check-source/parse-source/elaborate-source/patch-source/export-core-ir`、`PATCH-01..04`、`cargo test -p mir-runtime --test source_patch_hotplug -- --nocapture`、`cargo test -p mirrorea-cli --test surface_mir_cli -- --nocapture`、`scripts/surface_mir_samples.py check-all` を同期した。source patch は direct eval ではなく、accepted `patch-source` は HotPlugRequest / HotPlugVerdict / activation_cut を出し、`check-source` / `elaborate-source` は inspection-only、rejected patch は mutation しない。then-promoted package は `P-SURF-07 source operational suite`。
- 2026-05-24 18:21 JST
  `P-SURF-05` で role admission / capability grant evidence floor を actualize し、`surface_role_admission_check`、`ROLE-01..04`、`cargo test -p mir-semantics --test role_admission_capability_grant -- --nocapture`、`scripts/surface_mir_samples.py check-all` を同期した。role claim は authority ではなく、authority は admission grant から来る。then-promoted package は `P-SURF-06 source patch hot-plug`。
- 2026-05-24 17:48 JST
  `P-SURF-04` で generated communication evidence floor を actualize し、Core IR に `MessageEnvelope` / publish / observe / observer-safe redaction-retention rows を追加し、`ELAB-03/09/10` と widened `ELAB-01/05/08` で private/non-visible field rejection、visible write publish/observe、`VisibilityDenied` failure-row containment、`cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`、`scripts/surface_mir_samples.py check-all` を同期した。then-promoted package は `P-SURF-05 role admission capability grant`。
- 2026-05-24 16:48 JST
  `P-SURF-03` で Surface-to-Core elaboration evidence floor を actualize し、`ELAB-01/02/04/05/06/07/08` の cross-locus read/write remote request、generated edge、source span、obligation、read/write underdeclared generated failure-row rejection、nested read placement、unsupported-statement rejection、`cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`、`scripts/surface_mir_samples.py check-all` を同期した。then-promoted package は `P-SURF-04 auto communication publish/observe`。
- 2026-05-24 16:14 JST
  `P-SURF-02` で Surface Mir indexed-state semantic checker floor を actualize し、`IDX-01..05` の owner/keyspace/value metadata、key-not-authority rejection、stale-key rejection、retained-savepoint compaction rejection、nested-place ambient-authority rejection、`cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture`、`scripts/surface_mir_samples.py check-all` を同期した。then-promoted package は `P-SURF-03 Surface-to-Core elaboration`。
- 2026-05-24 15:38 JST
  `P-SURF-01` で Surface Mir alpha parser / sample helper floor を actualize し、`SURF-01..09` の positive/negative rows、`surface_mir_alpha_parse` example、authoring check、release-check check-all を同期した。then-promoted package は `P-SURF-02 indexed-state semantics`。
- 2026-05-24 14:00 JST
  `P-SURF-00B` で Surface Mir place-scope syntax を canonical `S { ... }` に rebaseline し、`S[ ... ]` を sugar としても採用しない方針、`.mir` source authority、indexed state owner/keyspace split、role admission/capability grant split、source patch hot-plug pipeline、Surface package sequenceを docs/spec/plan snapshot に固定した。検証結果と commit/push status は report を正本にする。
