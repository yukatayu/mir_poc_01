# Report 0668 — phase6 public operational cli final public contract later gate package

- Date: 2026-04-12T23:50:49.347134Z
- Author / agent: Codex
- Scope: Phase 6 docs-only close for `public operational CLI / final public contract later gate`, including normative package `371...372` and snapshot / plan mirror updates.
- Decision levels touched: current L2 package; docs-only later-gate ordering only.

## 1. Objective

- Close `public operational CLI / final public contract later gate` without promoting any symbol to a final public contract.
- Preserve `run_current_l2_source_sample` as the current docs-only public-pressure candidate while fixing the later ordering beyond it.
- Hand off the repo-level current line cleanly to `shared-space admission / compile-time visibility reopen`.

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/355...356`
- `specs/examples/363...364`
- `specs/examples/369...370`
- `docs/reports/0656-phase6-parser-checker-runtime-public-surface-inventory-package.md`
- `docs/reports/0662-phase6-public-operational-surface-actualization-gate-package.md`
- `docs/reports/0667-phase6-stable-malformed-broader-followup-inventory-package.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`

## 3. Actions taken

1. Re-read the public surface inventory and actualization gate to preserve the existing 3 bucket split and the `run_current_l2_source_sample` first-candidate cut.
2. Wrote `specs/examples/371...372` to fix the later ordering:
   - final public parser / checker / runtime API as the first later gate,
   - public operational CLI as the second later gate,
   - repo layout / accepted-set / repo-local helpers kept outside the current final contract.
3. Updated `Documentation.md`, `progress.md`, `tasks.md`, `specs/00-document-map.md`, relevant `plan/`, and the Phase 6 abstract so the repo-level current line advances to `shared-space admission / compile-time visibility reopen`.
4. Left code, sample corpus, runner behavior, and `.docs` authoring flow untouched because this package only fixes later-gate ordering.

## 4. Files changed

- `Documentation.md`
- `docs/reports/0668-phase6-public-operational-cli-final-public-contract-later-gate-package.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/examples/371-current-l2-stable-malformed-broader-follow-up-inventory-ready-public-operational-cli-final-public-contract-later-gate-comparison.md`
- `specs/examples/372-current-l2-public-operational-cli-final-public-contract-later-gate-ready-minimal-public-operational-cli-final-public-contract-later-gate-threshold.md`
- `tasks.md`

## 5. Commands run and exact outputs

- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 668 numbered report(s).`
- `git diff --check`
  - no output

## 6. Evidence / findings

- The current docs now distinguish three layers cleanly:
  - current docs-only candidate: `run_current_l2_source_sample`
  - first later gate: final public parser / checker / runtime API
  - second later gate: public operational CLI
- The later ordering remains library-before-CLI, which matches the fact that the current candidate is a library-side entry rather than a repo-local operational wrapper.
- `resolve_current_l2_source_sample_path`, accepted-set hard-coding, example/support modules, and repo-local Python helpers remain outside the current final contract.

## 7. Changes in understanding

- The unresolved public-surface question is no longer “API or CLI?” in the abstract. The order itself is now fixed: library contract first, CLI second.
- The public operational line should stay decoupled from repo layout and explicit host-plan details for as long as possible.
- The next repo-level pressure is not more public-surface work; it is the shared-space admission / compile-time visibility boundary.

## 8. Open questions

- How should the later final public library contract reduce the current `FixtureHostPlan` coupling?
- Should `run_current_l2_runtime_skeleton` ever become public-facing support, or stay tranche-internal?
- When the CLI line reopens, should it remain a thin wrapper over the final library contract, or carry separate operational policy?
- How should public theorem / model-check / checker migration pressure reconnect to the public contract line later?

## 9. Suggested next prompt

- Continue with `shared-space admission / compile-time visibility reopen`, then refresh snapshot docs and run a repository-wide consistency audit.
