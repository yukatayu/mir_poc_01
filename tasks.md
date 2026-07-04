# tasks

最終更新: 2026-07-04 20:46 JST

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
- `plan/118-g0-g1-ordinary-assignment-claim-family-drilldown.md` now drills
  down the ordinary Surface assignment claim-family row from `plan/70`. The
  explanatory center is the `plan/71` G1 target plus `plan/72` SCN-01/02 static
  consequence map; OBL-001/020/021 inventories, statement drafts, and guards
  are supporting proof-boundary evidence. It does not edit canon, claim G0/G1
  exit, move OBL status, claim conformance, or promote LAB helper / Lean
  compile-check evidence to proof.
- `plan/119-g0-remaining-claim-family-drilldown-priority.md` now classifies
  the remaining `plan/70` rows. No remaining row should be drilled down
  immediately by default; canonized rows are stale-wording audit targets only,
  later-gate rows should wait for prerequisite gate context, and evidence-only
  / process-only rows should not be promoted to status.
- `plan/120-repo-triage-recut-matrix.md` now classifies Product Alpha, Full
  System V1, and Surface evidence as `keep-core-idea`,
  `useful-floor`, `archive-exploration`, or
  `postpone/drop-from-current-recut` for the next theory / management recut.
  The labels are a LAB management overlay on top of existing
  workflow/evidence vocabulary. They do not move files, rename roots,
  archive/delete samples, edit canon, claim gate exit, or relabel runnable
  sample status.
- `plan/121-g1-minimal-vertical-slice-candidate-map.md` now narrows the next
  G1 ordinary-assignment source-first candidate to
  `G1-MVS-ASSIGNMENT-STATIC`: Surface syntax, indexed-state owner/keyspace
  pressure, owner-directed write, RHS dependency, visible publish / observe,
  failure-row containment, authority-obligation carrier, source spans, and
  OBL-001/020/021 statement-boundary evidence. It explicitly keeps role
  admission, fallback, patch, projection, devtools ABI, runtime, product, and
  transport scope outside the immediate slice and does not claim canon edit,
  G1 exit, proof discharge, conformance, runtime/product/API freeze, or sample
  status relabel.
- `plan/122-g1-scn-exact-static-slice-manifest.md` now maps that candidate to
  SCN-01 / SCN-02 static bullets. `ELAB-11` is exact current executable
  evidence for the SCN-01 static request / dependency / visible publish-observe
  / span bundle, and `ELAB-12` is exact current executable evidence for the
  SCN-02 static request / target-self dependency / positive containment /
  nested-locus edge-shape bundle. `ELAB-02/05/07/09/10` remain structural
  support only, and runtime bullets remain explicit gaps / out of scope. This
  does not claim canon edit, G1 exit, proof discharge, conformance,
  runtime/product/API freeze, or sample status relabel.
- `plan/123-g1-scn01-visibility-negative-actualization.md` and `ELAB-17` now
  actualize the exact SCN-01 visible-write `VisibilityDenied` negative row that
  `plan/122` had kept as structural support. OBL-001 predicate refinement stays
  reserve-only because the current LAB statement draft already has abstract
  RHS-dependency and visible-consequence predicates. Oracle advisory review
  preferred doing that boundary audit before additional SCN fixtures, so the
  next package should audit OBL-001 before adding another static gap row. This
  does not claim canon edit, G1 exit, proof discharge, conformance, runtime
  dispatch, final diagnostic/repair ABI, or sample status relabel.
- `plan/124-g1-obl001-boundary-audit.md` now closes that immediate boundary
  audit. The existing LAB OBL-001 abstract predicates can carry `ELAB-11`,
  `ELAB-12`, and `ELAB-17` without importing helper JSON names, final
  Diagnostic / repair ABI fields, or sample-specific vocabulary. Lean predicate
  refinement is not needed at this checkpoint. `ELAB-17` is OBL-001
  failure-containment pressure only; diagnostic projection and repair payload
  details remain OBL-024 / OBL-025 LAB evidence. `plan/122` now has a
  post-`plan/123` / `plan/124` addendum so the prior SCN-01 negative gap
  classification is not stale. This does not claim canon edit, OBL completion,
  proof discharge, conformance, G1 exit, runtime dispatch, final ABI, or sample
  status relabel.
- `plan/125-g1-scn02-direct-local-write-blocker-review.md` now closes the
  immediate SCN-02 direct-local-write blocker check as non-blocking for the
  current G1 bridge. The review keeps `ELAB-12` as exact positive
  owner-directed SCN-02 evidence and `ELAB-02` / `IDX-05` as structural
  non-ambient-authority support, while explicitly not claiming SCN-02 negative
  (b) as exact executable negative evidence. No new sample, Rust/Lean
  predicate, canon edit, conformance claim, G1 exit, runtime dispatch, final
  ABI, or sample status changed.
- `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md` now
  closes the immediate OBL-020/021 boundary-audit package for the current G1
  bridge. It finds no Lean predicate refinement needed for OBL-020 or OBL-021
  at this checkpoint, while hardening sync tests so bare `:= True` bodies and
  comment-only required OBL-001/020/021 body links do not pass. No canon edit,
  OBL completion, proof discharge, conformance claim, G1 exit, runtime
  dispatch, final ABI, or sample status changed.
- `plan/127-g1-ordinary-assignment-bridge-readiness-nonreadiness-map.md` now
  maps the post-`plan/126` G1 ordinary-assignment bridge by canon criterion /
  pressure, current LAB support, remaining blocker, and forbidden claim. It
  says the narrow LAB bridge can continue without a new executable row, Lean
  predicate refinement, or canon wording proposal by default, while G1 exit,
  T0 -> T1 transition, OBL completion, proof discharge, conformance, runtime
  readiness, and sample status remain unclaimed.
- `plan/128-g1-bridge-handoff-blocker-ledger.md` now classifies those blockers
  by next owner: human/canon acceptance, future statement / proof-package work,
  canon-open deferral, static LAB support-only, later runtime / conformance /
  product, and reserve triggers. It separates G1 OBL statement/status from T2
  proof discharge and keeps SCN-02 direct-local-write negative (b) as
  reserve-only, non-blocking structural support.
- `plan/129-g1-acceptance-packet-preflight.md` now prepares a future G1
  acceptance packet by listing the canon files a human/canon review would
  inspect, the exact LAB evidence supporting each acceptance point, the
  OBL-001/020/021 statement/status blockers, the OPEN-014 deferral point, and
  runtime / conformance / product exclusions. It is a preflight routing
  checklist, not the acceptance packet itself, and it does not edit canon, move
  OBL status, claim G1 exit, add executable rows, or refine Lean predicates.
- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md` now
  inventories criteria for a later proposal to move OBL-001 / OBL-020 /
  OBL-021 status. It separates current LAB support from proposal criteria and
  human/canon decisions, records common Lean statement criteria such as
  compile-check / no-admitted-stub / drift-guard / ledger-target mapping, and
  keeps proof, runtime, conformance, OPEN-014, authority theorem, and final ABI
  claims out of scope.
- `plan/131-g1-status-proposal-packet-outline.md` now turns the `plan/130`
  criteria matrix into a future proposal packet outline. It defines required
  slots for cover sheet, canon-state summary, requested-status matrix, artifact
  identity, evidence trace, OPEN / deferral handling, non-claims, ledger delta
  placeholder, and submission checklist, while leaving requested status,
  draft submission, canon edit, ledger movement, proof, conformance, runtime,
  and G1 exit unclaimed.
- A focused source-hierarchy stale wording audit corrected `plan/01`, `plan/07`,
  `plan/09`, `plan/19`, `plan/57`, `README.md`, `Documentation.md`,
  `samples_progress.md`, `plan/70`, and `plan/90` so LAB memory files no longer
  read as overriding canon, and Surface alpha evidence rows no longer read as
  workflow-ready runtime status.
  `mirrorea_canon/` remains canon; legacy `specs/` remain LAB evidence /
  historical package-line memory.
- `scripts/validate_docs.py` now includes a source-hierarchy wording lint for
  `CANON.md`, root/snapshot docs, `samples/README.md`, `.docs/`,
  `docs/hands_on/`, `docs/research_abstract/`, and `plan/`. It rejects stale
  reader-facing `specs/`-as-normative wording, including split-line bullets and
  the English `Normative source remains specs...` shape, while excluding
  historical reports, legacy specs, and archived research material. The first
  lint pass corrected stale wording in `docs/hands_on/README.md`, selected
  `docs/research_abstract/*.md`, `plan/19`, `plan/50`, and `plan/58`.
  `scripts/README.md` now mirrors the expanded validator responsibility,
  including source-hierarchy wording lint, active host-path lint, and snapshot
  top `progress.md` / `tasks.md` `最終更新` plus `samples_progress.md`
  `Last updated` freshness guard.
- `scripts/README.md` also mirrors the current `check_source_hierarchy.py`
  responsibility after the structural source-hierarchy guard widened past the
  old `plan/39..86` description. The guard and docs validator now require
  `plan/39..131`, including the current G1 E-ROW / OBL addenda, remaining
  claim-family priority map, repo-triage recut matrix, and G1 minimal vertical
  slice / SCN manifest / SCN-01 negative maps; `scripts/README.md` mirrors that range alongside
  Product Alpha demo entry files, `docs/hands_on/`, `docs/research_abstract/`,
  and the operational product sample sub-agent handoff root. `plan/00-index.md`
  now also lists the previously omitted detailed filenames for `plan/106..108`,
  `plan/118..131`. This is documentation taxonomy maintenance only.
- `scripts/validate_docs.py` now rejects numbered `plan/*.md` files that exist
  in the repository but are not registered in its explicit `REQUIRED` scaffold
  list. Its explicit plan scaffold and the source-hierarchy plan scaffold now
  cover current numbered `plan/00..131`. This keeps future plan-file additions
  from silently bypassing the docs scaffold guard while preserving deletion
  detection through the explicit list.
- `scripts/tests/test_validate_docs.py` now also asserts that numbered plan
  files in `validate_docs.REQUIRED`, `check_source_hierarchy.REQUIRED_PATHS`,
  and the real `plan/` directory stay in sync.
- Full System V1 provider / renderer helper inputs and release-check
  representative CLI commands now preserve portability by passing in-repo CLI
  paths as repo-relative `samples/...` argv. Renderer-pose generated nested
  provider-admission reports now store repo-relative path fields rather than
  host absolute paths, and release-check reports / bundle / viewer display
  output-root paths relatively. This is maintenance hardening only; no sample
  status, semantics, ABI, or execution-scope claim changed.
- Full System V1 textual Mir helper raw parser payloads now preserve
  portability for repo-owned source paths: `source_path` and diagnostic path
  text are repo-relative `samples/...` in helper/release JSON.
- Full System V1 typed/runtime/operational helper output and nested source argv
  now preserve portability: `scripts/full_system_v1_samples.py check-all
  --format json` passes 41 rows with repo-root absolute match 0, and nested
  Cargo checker/runtime source argv receive repo-relative `samples/...` paths
  for repo-owned `.mir` sources while preserving external absolute paths.
- Surface helper subprocess argv now preserves portability for repo-owned
  source inputs: nested Cargo examples and `mirrorea-alpha patch-source` receive
  repo-relative `samples/...` paths. Public helper JSON was already repo-root
  clean; the P-SURF-99 release-check sample_count gate is now synchronized with
  the current 53-row matrix.
- Surface release-check output serialization now preserves portability for
  release-owned and repo-owned paths: plan JSON, per-command report JSON,
  bundle JSON, and generated viewer HTML display output-root paths relatively
  and strip repo-root prefixes from free-text stdout/stderr. External absolute
  paths remain unchanged.
- Mir computational helper subprocess argv now preserves portability for
  repo-owned computational sample roots: nested `mirrorea-alpha run-local` /
  `check` receive repo-relative `samples/...` paths. Public helper JSON was
  already repo-root clean.
- Practical alpha-1 transport helper now preserves portability for repo-owned
  package inputs and closeout fields: local transport cargo example argv,
  Docker Compose `-f` argv, closeout `compose_file`, and closeout `binary_path`
  are repo-relative. Docker bind mount env remains host-path internal only.
- Practical alpha-1 checker helper now preserves portability for repo-owned
  package inputs: nested checker Cargo example argv uses repo-relative
  `samples/...` paths. Public helper JSON was already repo-root clean.
- Practical alpha-1 run-local helper now preserves portability for repo-owned
  package inputs: nested local-runtime Cargo example argv uses repo-relative
  `samples/...` paths. Public helper JSON was already repo-root clean.
- Practical alpha-1 attach helper now preserves portability for repo-owned
  package inputs: nested hotplug Cargo example argv uses repo-relative
  `samples/...` paths. Public helper JSON was already repo-root clean.
- Practical alpha-0.9 devtools helper now preserves portability for repo-owned
  base / host-I/O / attach package inputs: nested
  `mir_practical_alpha05_session` command argv uses repo-relative
  `samples/...` paths while the temporary session file remains an absolute temp
  path. Public helper JSON, `closeout`, and `render-html` were already
  repo-root clean on the success path.
- Practical alpha-0.8 same-session hot-plug helper now preserves portability
  for repo-owned base session and attach package inputs: nested
  `mir_practical_alpha05_session` command argv uses repo-relative
  `samples/...` paths while the temporary session file remains an absolute temp
  path. Public helper JSON and `closeout` were already repo-root clean on the
  success path.
- Practical alpha-0.5 session helper now preserves portability for repo-owned
  session-start and host-I/O package inputs: nested
  `mir_practical_alpha05_session` command argv uses repo-relative
  `samples/...` paths while the temporary session file remains an absolute temp
  path. This closes the lower session carrier used by alpha-0.8 / alpha-0.9
  workflows on the same path-portability line.
- Practical alpha-1 avatar helper now preserves portability for repo-owned
  package inputs: nested avatar Cargo example argv uses repo-relative
  `samples/...` paths. Public helper JSON and `closeout` were already
  repo-root clean on the success path.
- Practical alpha-1 save/load helper now preserves portability for repo-owned
  runtime-backed package inputs: nested save-load Cargo example argv uses
  repo-relative `samples/...` paths. The checker-backed preflight branch still
  delegates through `practical_alpha1_check.py`, which was already hardened.
  Public helper JSON and `closeout` were already repo-root clean on the success
  path.
- The read-only code-mapper practical-helper portability candidate list is now
  closed for `practical_alpha09_devtools.py`,
  `practical_alpha08_session_hotplug.py`, `practical_alpha1_avatar.py`, and
  `practical_alpha1_save_load.py`; the broader scan also found and closed
  `practical_alpha05_session.py`, Full System V1 nested source argv, and alpha
  network Docker success/failure path serialization.
  Shared practical failure-path redaction is now also closed, so the known
  high-confidence broader path-portability candidate set is closed for now.
- Alpha network Docker helper output now preserves portability for repo-owned
  sample root / compose file / binary path displays, nested `docker compose -f`
  argv, Docker stdout, missing repo binary errors, Docker failure stderr, and
  missing compose-output JSON errors. Docker bind mount environment values remain
  host absolute where Compose requires them. Real Docker validation passed
  `run NET-02`, `check-all` 6/6, and `stage-c-closeout` with returned payload
  host path matches 0.
- Current-L2 detached-loop and theorem/model/Lean pipeline helper surfaces now
  preserve portability for repo-owned helper argv, artifact displays, plan/result
  JSON, source-regression command plans, and Lean verification argv. External
  artifact roots remain absolute. Full current-L2 source regression passed 23/23
  commands, focused Python unit coverage passed 82 tests,
  `scan-reason-code-readiness` output had repo-root absolute match 0, generated
  artifact scans found no repo-root absolute paths, and `/tmp` checker
  invocation printed repo-relative fixture/artifact paths.
- Source-hierarchy status output now preserves portability: `repo_root` display
  is `.` in both pretty and JSON output while internal existence checks still
  use absolute `REPO_ROOT`. Direct source-hierarchy output has repo-root
  absolute match 0, release-check wrapper unit coverage passed, and a Surface
  release-check representative run produced no repo-root absolute paths in
  stdout or generated output.
- Product Alpha release-check and generated evidence now preserve portability
  for repo-owned source inputs: release-check representative CLI argv use
  repo-relative `samples/product-alpha1/demo...` paths, release-check aggregate
  output displays release-owned paths relative to its output root, and
  generated demo / native provenance / Docker compose source fields avoid
  repo-root absolute paths. This is maintenance hardening only; Product Alpha
  remains alpha release-candidate evidence, not final product or broader
  distribution.
- Product Alpha installed-binary helper evidence now also preserves portability
  for repo-owned binary/package inputs: nested argv and top `binary_path` use
  repo-relative `target/debug/mirrorea-alpha` and
  `samples/product-alpha1/demo`. It also displays generated `out_dir`,
  `session_dir`, `native_bundle_dir`, `demo_dir`, command-result argv, and
  stderr paths relative to the output root or repo root while preserving
  external absolute paths. This is adoption-probe hardening only; final
  CLI/API/ABI, packaging, and broader distribution remain undefined.
- Operational product helper nested command evidence now preserves portability
  for repo-owned operational roots and layer package inputs: nested
  `mirrorea-alpha` argv use repo-relative `samples/product-alpha1/operational/...`
  paths. Product Alpha release-check, installed-binary probe, operational
  `check-all`, and minimal alpha-1 pattern verifier remain passing; this does
  not change final product, final API, or broader distribution status.
- Active clean-near-end research abstract detail docs and
  `samples/current-l2/README.md` now avoid host-specific `/home/yukatayu/...`
  links for current reader-facing sample paths. `mir-clean-near-end` and
  `scripts/current_l2_lean_sample_sync.py` now emit repo-relative paths for the
  same current sample/manifest surfaces, so `actual output` snippets do not
  depend on the host checkout path. `current_l2_guided_samples.py closeout`
  now also reports repo-relative `lean_roots`, keeping the current-L2 closeout
  output portable for active reader/automation use. Historical reports, old
  research abstract archives, old Lean bundles, `tmp_faq/`, and external
  `/home/codex/.codex/...` operation paths remain classified as out-of-scope
  evidence or environment references. `scripts/validate_docs.py` now guards
  active reader-facing docs/samples against reintroducing host-specific repo
  paths while preserving those exclusions. This is maintenance hardening only;
  no sample status, semantics, ABI, workflow, or canon claim changed.
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
- `plan/117-g1-obl001-020-021-statement-guard-hardening.md` now records a
  LAB-only sync-test guard hardening for the older OBL-001 / OBL-020 /
  OBL-021 statement drafts. It checks body-level links for assignment request
  evidence / postcondition, WF preservation / family threading, and
  elaboration determinism component equivalence / diagnostic equivalence /
  success-reject exclusion. It does not prove OBL-001/020/021, move canon
  ledger status, create a proof skeleton, freeze final equality or diagnostic
  ABI, claim runtime dispatch or runtime scheduling determinism, claim
  conformance, claim G1 exit, or edit canon.
- `plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md` now
  audits those OBL-020/021 boundaries after the SCN-02 blocker review and keeps
  Lean predicate refinement reserve-only. It also hardens the sync-test helper
  so required body links are checked after Lean comment stripping and bare
  `:= True` vacuity is rejected. It does not prove OBL-020/021, move canon
  ledger status, create a proof skeleton, freeze final equality or diagnostic
  ABI, claim runtime dispatch or runtime scheduling determinism, claim
  conformance, claim G1 exit, or edit canon.
- `plan/127-g1-ordinary-assignment-bridge-readiness-nonreadiness-map.md` now
  keeps the G1 bridge support state and non-readiness blockers in one map:
  current LAB support is enough to continue narrow support work, but not enough
  for G1 exit, T1 transition, OBL completion, proof discharge, conformance, or
  runtime readiness.
- `plan/128-g1-bridge-handoff-blocker-ledger.md` now turns that map into a
  docs-only handoff ledger. It adds explicit `canon-open / deferral decision`
  handling for OPEN-014, separates G1 OBL statement/status work from later T2
  proof discharge, and keeps reserve-only non-blockers from becoming required
  closeout work.
- `plan/129-g1-acceptance-packet-preflight.md` now turns that handoff ledger
  into a preflight routing checklist for a future human/canon G1 acceptance
  packet. It includes `theory/01` as part of the acceptance surface, keeps
  OBL-002 proof as later T2 work rather than a G1 exit blocker, classifies
  `ELAB-17` diagnostic / repair payloads under OBL-024/025 support only, and
  leaves all canon edit / OBL movement / conformance / runtime claims
  forbidden.
- `plan/130-g1-obl-statement-status-completion-criteria-inventory.md` now
  turns the `plan/129` statement/status blocker into a criteria matrix. It does
  not say current LAB drafts are enough to move status; it says what a later
  status proposal would need to cite, decide, defer, or keep out of scope.
- `plan/131-g1-status-proposal-packet-outline.md` now turns that criteria
  matrix into an outline-only packet shape. It does not choose `stated` vs
  `lean-stated`, submit a draft proposal, include actual ledger patch text, or
  move canon status.
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
  It compiles as a diagnostic projection / report-local association key /
  future proof-level association relation / reported failed premise /
  report-local replay anchor / future proof-level replay relation `Prop` with
  non-repair mixed diagnostic branch boundary predicates. It does not define
  final Diagnostic ABI, JSON keys, request IDs, branch IDs,
  association-key ABI, replay semantics, prove OBL-024, move canon ledger
  status, claim root-cause uniqueness, claim conformance, claim G1 exit, or
  edit canon.
- `plan/110-g1-obl024-executable-projection-carrier.md` now records LAB-only
  executable E-ROW projection evidence for OBL-024 inside current
  `lab_diagnostic_details`. `ELAB-04/07/10/13..17` now carry non-final
  `diagnostic_soundness_projection` with helper-local diagnostic id,
  report-local association key, reported bindings, and report-local trace
  replay anchor. It does not define final Diagnostic JSON / request ID /
  association-key ABI / replay semantics, prove OBL-024, move canon ledger
  status, widen repair output, add an `ELAB-04` payload, claim conformance,
  claim G1 exit, or edit canon.
- `plan/111-g1-obl024-projection-rust-fixture-guards.md` now records test-only
  Rust fixture guard hardening for the same carrier. Rust tests now guard
  projection/context consistency and skipped internal association-field
  non-serialization across `ELAB-04/07/10/13..17`. It does not change
  production behavior, expected JSON, repair output, final Diagnostic / replay
  ABI, proof status, conformance, G1 exit, or canon.
- `plan/112-g1-obl024-replay-vocabulary-preflight.md` now records docs-first
  replay vocabulary for OBL-024. It separates current report-local
  `trace_local_replay` anchors from future proof-level replay relations and
  keeps proof-level replay semantics, exactness, ordering, association keys,
  branch replay, and final ABI as OPEN. It does not change production behavior,
  expected JSON, Lean files, repair output, proof status, conformance, G1 exit,
  or canon.
- `plan/113-g1-obl024-lean-replay-vocabulary-refinement.md` now records the
  LAB-only Lean refinement that mirrors `plan/112` in
  `DiagnosticSoundnessStatementDraft.lean`. The draft now separates
  `ReportLocalReplayAnchor` from `ProofLevelReplayWitness` /
  `ProofLevelReplayRelation` and guards that vocabulary in the Lean sync unit
  tests. It does not prove OBL-024, move canon ledger status, freeze final
  Diagnostic / replay ABI, change runtime JSON, change repair output, claim
  conformance, claim G1 exit, or edit canon.
- `plan/114-g1-obl024-lean-association-vocabulary-refinement.md` now records
  the LAB-only Lean refinement that separates `ReportLocalAssociationKey` from
  `ProofLevelAssociationWitness` / `ProofLevelAssociationRelation` and
  guards that vocabulary in the Lean sync unit tests. It does not prove
  OBL-024, move canon ledger status, freeze final Diagnostic /
  association-key / replay ABI, change runtime JSON, change repair output,
  claim conformance, claim G1 exit, or edit canon.
- `plan/115-g1-obl024-association-guard-hardening.md` now records a LAB-only
  static guard hardening that keeps the OBL-024 report-local association key
  from drifting into semantic key equality, branch-local association-key
  vocabulary, final-looking request / branch / ABI names, or key comparability /
  uniqueness pressure. It does not prove OBL-024, move canon ledger status,
  freeze final Diagnostic / association-key / replay ABI, change runtime JSON,
  change repair output, claim conformance, claim G1 exit, or edit canon.
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
- `plan/116-g1-obl025-repair-completeness-guard-hardening.md` now records a
  LAB-only sync-test guard hardening for OBL-025. It checks that
  `RepairCompletenessForRejection` still goes through an eligible single-edit
  witness, `SuggestedRepairOf`, and `SuggestionCoversWitness`, and that grouped
  multi-edit, partial guidance, and branch-local guidance remain explicit
  non-coverage helpers. It does not prove OBL-025, move canon ledger status,
  freeze final Diagnostic / repair ABI, change runtime JSON, change repair
  output, claim conformance, claim G1 exit, or edit canon.

## candidate next strategy packages

These are candidates only. They are not promoted until the user chooses the
next line.

| Candidate | Macro reading | Objective | Close condition |
|---|---|---|---|
| `G1 status evidence readiness dry-run` | `G1` docs/validation-only | after `plan/131`, run and record the Lean statement draft compile / sync guard commands a future packet would cite, scan OBL-001 / OBL-020 / OBL-021 drafts for admitted stubs / placeholder bodies, and list any evidence gaps against the packet outline | readiness dry-run only; no canon edit, no ledger movement, no requested-status choice, no executable row, no Lean refinement unless a blocker is found and explicitly opened |
| `OBL-001 sync guard hardening` | `G1` reserve | add docs-only or test-only guard hardening only if future review finds that the OBL-001 statement sync tests can drift away from the `plan/124` boundary without failing | Lean remains compile-check only; no predicate refinement by default; no canon ledger movement |
| `OBL-020 statement refinement` | `G1` reserve | after `plan/126`, refine the LAB `StepWFStatementDraft.lean` only if a future proof package or concrete bridge blocker finds overfit, missing abstraction, or premature proof-interface wording | Lean still compile-check only; no canon ledger movement |
| `OBL-021 statement refinement` | `G1` reserve | after `plan/126`, refine the LAB `ElabDeterminismStatementDraft.lean` only if a future proof package or concrete bridge blocker finds overfit, missing projection-totality wording, or diagnostic-equivalence gaps | Lean still compile-check only; no canon ledger movement |
| `OBL-001 statement draft refinement` | `G1` reserve | refine the LAB `THM001StatementDraft.lean` only if review finds overfit or a missing predicate | Lean still compile-check only; no canon ledger movement |
| `ELAB-04 mixed executable payload model` | `G1` reserve | only after a later package accepts a mixed wrapper or separate associated-diagnostics model for `ELAB-04` | keep no-repair until that model is explicit; no ranking or executable widening by default |
| `remaining LAB claim-family drilldowns` | `T0/G0` reserve | split non-ordinary-assignment `plan/70` rows only when a future G0 close decision or touched-doc stale wording audit needs exact citations; the read/write/dependency row may also be narrowed only if a concrete G1 ordinary-assignment support gap remains | `plan/119` says no remaining row is an immediate default target; no canon L0/L1 change; no historical rewrite beyond focused wording cleanup; no G4 observation or runtime graph widening |
| `canon mental-model clarification proposal` | `G0/G1` reserve | draft a proposal, not an edit, if ordinary assignment or LAB citation rules need a short canon clarification | proposal remains advisory until human/canon process accepts it |
| `repo-triage recut follow-up` | `Macro 0` maintenance reserve | use `plan/120` if a future package needs line-level citation, actual archive move comparison, or front-door taxonomy rewrite after the initial Product Alpha / Full System V1 / Surface useful-floor / keep-core / archive / postpone matrix | no deletion, archive move, root rename, sample status relabel, or canon edit without explicit follow-up package |
| `path-portability broader sweep` | `Macro 0` maintenance | known high-confidence host-path leak / nested absolute argv candidates from the broader audit are closed for now; reopen only for new evidence or touched-surface drift | each package proves affected helper output has repo-root absolute match 0 or documents a skipped external-path boundary |

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
| docs freshness audit | keep README, Documentation, scripts README, progress, tasks, samples dashboard, indexes aligned; keep `progress.md` / `tasks.md` top `最終更新` and `samples_progress.md` top `Last updated` headers present and not lagging behind timestamped body entries | `python3 scripts/validate_docs.py`, `python3 scripts/check_source_hierarchy.py`, `git diff --check` | snapshot docs must not create new normative decisions |
| canon/LAB wording audit | keep touched LAB docs from re-promoting legacy `specs/` or helper closeouts to canon status | `python3 scripts/validate_docs.py` source-hierarchy wording lint plus `python3 scripts/check_source_hierarchy.py`; latest lint-backed pass also corrected `docs/hands_on/README.md`, selected `docs/research_abstract/*.md`, `plan/19`, `plan/50`, and `plan/58` | do not rewrite historical LAB evidence wholesale or change runnable row status |
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
