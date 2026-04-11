# Report 0602 — phase5 public checker boundary package

- Date: 2026-04-11 19:31 JST
- Author / agent: Codex
- Scope: Phase 5 checker-side current promoted line continuation through public checker boundary package
- Decision levels touched: current L2 docs-first boundary only (`specs/` normative additions at L2; no L0/L1 change)

## 1. Objective

- Continue the same self-driven Phase 5 checker-side line after the shared output contract package.
- Determine what the first docs-only public checker boundary cut should be, and what its minimum shape should be.
- Update the snapshots so the next promoted line moves from shared output contract to verifier handoff surface.

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/111-current-l2-stage3-request-contract-subset-freeze-sequencing.md`
- `specs/examples/112-current-l2-phase3-resume-side-selection.md`
- `specs/examples/113-current-l2-first-checker-reconnect-family-selection.md`
- `specs/examples/281-current-l2-minimal-public-checker-command-surface-ready-shared-output-contract-comparison.md`
- `specs/examples/282-current-l2-shared-output-contract-ready-minimal-shared-output-contract-threshold.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `scripts/current_l2_family_checker_support.py`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_family_checker_support.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`

## 3. Actions taken

- Re-checked the Phase 3 reserve-path docs so the new boundary package would not silently collapse parser grammar finalization into the checker-side docs-first line.
- Added:
  - `specs/examples/283-current-l2-minimal-shared-output-contract-ready-public-checker-boundary-comparison.md`
  - `specs/examples/284-current-l2-public-checker-boundary-ready-minimal-public-checker-boundary-threshold.md`
- Fixed the current first choice as:
  - public checker boundary should start from a docs-only parser-front bundle that connects command surface and shared output contract
  - the minimum boundary shape should be `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref`
  - final parser grammar, generic shared entry, and verifier handoff should remain later
- Updated the same mirror set in the same task so `Documentation.md` / `progress.md` / `tasks.md` / `plan/` / research abstract all point at the new next promoted line.
- `python3 scripts/new_report.py --slug phase5-public-checker-boundary-package` reused `0601` because the earlier report was still untracked, so the boundary report filename and title were corrected manually to `0602`.

## 4. Files changed

- `specs/examples/283-current-l2-minimal-shared-output-contract-ready-public-checker-boundary-comparison.md`
- `specs/examples/284-current-l2-public-checker-boundary-ready-minimal-public-checker-boundary-threshold.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0602-phase5-public-checker-boundary-package.md`

## 5. Commands run and exact outputs

- `python3 scripts/new_report.py --slug phase5-public-checker-boundary-package`
  - `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - `  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0601-phase5-public-checker-boundary-package.md`
- `python3 -m unittest scripts.tests.test_current_l2_family_checker_support scripts.tests.test_current_l2_same_lineage_checker scripts.tests.test_current_l2_missing_option_checker scripts.tests.test_current_l2_capability_checker scripts.tests.test_current_l2_static_gate_loop`
  - `Ran 21 tests in 0.021s`
  - `OK`
- `python3 scripts/current_l2_detached_loop.py smoke-same-lineage-checker crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json --artifact-root /tmp/codex-phase5-public-checker-smoke --run-label shared-output-smoke --overwrite`
  - `static gate artifact: /tmp/codex-phase5-public-checker-smoke/static-gates/shared-output-smoke/e4-malformed-lineage.static-gate.json`
  - `cluster: same_lineage_evidence_floor`
  - `status: matched`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 601 numbered report(s).`
- `git diff --check`
  - no output
- `git status --short`
  - ` M Documentation.md`
  - ` M docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
  - ` M plan/11-roadmap-near-term.md`
  - ` M plan/12-open-problems-and-risks.md`
  - ` M plan/13-heavy-future-workstreams.md`
  - ` M plan/17-research-phases-and-autonomy-gates.md`
  - ` M plan/90-source-traceability.md`
  - ` M progress.md`
  - ` M specs/00-document-map.md`
  - ` M tasks.md`
  - `?? docs/reports/0601-phase5-public-checker-shared-output-contract-package.md`
  - `?? docs/reports/0602-phase5-public-checker-boundary-package.md`
  - `?? specs/examples/281-current-l2-minimal-public-checker-command-surface-ready-shared-output-contract-comparison.md`
  - `?? specs/examples/282-current-l2-shared-output-contract-ready-minimal-shared-output-contract-threshold.md`
  - `?? specs/examples/283-current-l2-minimal-shared-output-contract-ready-public-checker-boundary-comparison.md`
  - `?? specs/examples/284-current-l2-public-checker-boundary-ready-minimal-public-checker-boundary-threshold.md`

## 6. Evidence / findings

- `specs/examples/283` では、public checker boundary comparison の first candidate を final parser grammar や query token surface ではなく、command surface と shared output contract をつなぐ docs-only parser-front bundle に置いた。
- `specs/examples/284` では、その minimum を
  - `boundary_kind`
  - `public_checker_command_surface_ref`
  - `shared_output_contract_ref`
  の 3 field に留め、query token / parser input / verifier handoff は still later に残す current first choice を固定した。
- Phase 3 reserve path の `specs/examples/112` / `113` を前提にしても、current package は final parser grammar を fix せずに閉じられており、Phase 5 側の docs-first public checker line と整合する。
- `python3 scripts/validate_docs.py` は通過し、`git diff --check` は無出力で、docs scaffold と whitespace hygiene に問題はない。

## 7. Changes in understanding

- shared output contract まで切れた段階では、public checker boundary を final parser grammar の proxy にせず、command side と output side の接点を示す docs-only relation として切るのが最も自然である。
- これにより parser boundary reserve path は later blocker のまま保てる一方、checker-side public-looking line は verifier handoff surface まで narrow に進められる。
- current pressure の順番は `command surface -> shared output contract -> public checker boundary -> verifier handoff` と読むのが source-backed である。

## 8. Open questions

- verifier handoff surface の minimum を何 field までに留めるべきか
- query token / `checker_subject` public naming を later でどう切るべきか
- generic shared public checker entry を public checker boundary のどの後段で扱うべきか

## 9. Suggested next prompt

- `public checker boundary minimum を前提に、minimal-public-checker-boundary-ready verifier-handoff-surface comparison を進め、mirror sweep と validation までお願いします。`
