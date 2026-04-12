# Report 0629 — phase6 source sample authoring bless regression policy

- Date: 2026-04-12T05:21:00Z
- Author / agent: Codex
- Scope: Phase 6 source-sample authoring / bless / regression policy, including repo-local helper actualization, reviewed sync + regression success reading, normative package `325...326`, and snapshot mirror updates.
- Decision levels touched: current L2 package; repo-local non-production helper / policy actualization.

## 1. Objective

- Fix the current source-sample authoring / bless / regression flow for the first-authored trio.
- Keep `bless` at repo-local reviewed sync + regression success level instead of widening to retained artifact archive or public CLI semantics.
- Hand off the next mainline cleanly to theorem-first concrete tool pilot.

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-mir-core.md`
- `specs/02-language-and-compiler-co-design.md`
- `specs/03-context-and-roadmap.md`
- `specs/09-current-l2.md`
- `specs/examples/315...324`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`

## 3. Actions taken

1. Added `scripts/current_l2_source_sample_regression.py` as a narrow repo-local helper with `inventory` and `regression` subcommands.
2. Added `scripts/tests/test_current_l2_source_sample_regression.py` to ratchet inventory content, command planning, run-label validation, mismatch handling, and failure short-circuit behavior.
3. Strengthened `inventory` so it checks current file presence/absence for authored versus deferred rows instead of only printing a static table.
4. Added `specs/examples/325...326` to fix the comparison and threshold for source-sample authoring / bless / regression policy.
5. Added `.docs/current-l2-source-sample-authoring-policy.md` as the repo-local policy anchor and updated snapshot / plan mirrors to use it.
6. Switched the current promoted line from source-sample policy work to theorem-first concrete tool pilot.

## 4. Files changed

- `.docs/current-l2-source-sample-authoring-policy.md`
- `Documentation.md`
- `docs/reports/0629-phase6-source-sample-authoring-bless-regression-policy.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/current-l2/README.md`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`
- `specs/00-document-map.md`
- `specs/examples/325-current-l2-verification-ladder-wiring-ready-source-sample-authoring-bless-regression-policy-comparison.md`
- `specs/examples/326-current-l2-source-sample-authoring-bless-regression-policy-ready-minimal-source-sample-authoring-bless-regression-policy-threshold.md`
- `tasks.md`

## 5. Commands run and exact outputs

- `python3 -m unittest scripts.tests.test_current_l2_source_sample_regression`
  - `Ran 13 tests in 0.007s`
  - `OK`
- `python3 scripts/current_l2_source_sample_regression.py inventory`
  - `current L2 fixed-subset first-cluster inventory`
  - `e2-try-fallback | source-authored | valid | success | runtime_try_cut_cluster | present | first authored trio runtime path`
  - `e4-malformed-lineage | source-authored | malformed | not_evaluated | fixture_static_cluster | present | first authored trio static stop`
  - `e23-malformed-try-fallback-missing-fallback-body | source-authored | malformed | not_evaluated | fixture_static_cluster | present | first authored trio static stop`
  - `e1-place-atomic-cut | source-target-only | not_yet_authored | not_yet_authored | not_yet_authored | absent | deferred authored row`
  - `e3-option-admit-chain | source-target-only | not_yet_authored | not_yet_authored | not_yet_authored | absent | deferred authored row`
  - `e21-try-atomic-cut-frontier | source-target-only | not_yet_authored | not_yet_authored | not_yet_authored | absent | deferred authored row`
- `python3 scripts/current_l2_source_sample_regression.py regression --artifact-root /tmp/mir-phase6-source-sample-regression --run-label phase6-helper-check`
  - `[1/7] runtime lowering test`
  - `[2/7] source sample runner test`
  - `[3/7] verification ladder test`
  - `[4/7] formal hook support test`
  - `bundle artifact: /tmp/mir-phase6-source-sample-regression/bundles/phase6-helper-check-e2-try-fallback/e2-try-fallback.detached.json`
  - `formal hook artifact: /tmp/mir-phase6-source-sample-regression/formal-hooks/phase6-helper-check-e2-try-fallback/e2-try-fallback.formal-hook.json`
  - `static gate artifact: /tmp/mir-phase6-source-sample-regression/static-gates/phase6-helper-check-e4-malformed-lineage/e4-malformed-lineage.static-gate.json`
  - `formal hook artifact: /tmp/mir-phase6-source-sample-regression/formal-hooks/phase6-helper-check-e4-malformed-lineage/e4-malformed-lineage.formal-hook.json`
  - `static gate artifact: /tmp/mir-phase6-source-sample-regression/static-gates/phase6-helper-check-e23-malformed-try-fallback-missing-fallback-body/e23-malformed-try-fallback-missing-fallback-body.static-gate.json`
  - `formal hook artifact: /tmp/mir-phase6-source-sample-regression/formal-hooks/phase6-helper-check-e23-malformed-try-fallback-missing-fallback-body/e23-malformed-try-fallback-missing-fallback-body.formal-hook.json`
  - `all regression commands passed`

## 6. Evidence / findings

- The smallest coherent policy package is manual reviewed sync plus a repo-local regression helper and explicit regression success.
- The helper can now detect mismatch between authored/deferred status and actual file presence before running the regression bundle.
- Current `bless` can stay as repo-local reviewed sync + regression success terminology without forcing retained artifact archive or public CLI semantics.
- The current promoted line can move cleanly to theorem-first concrete tool pilot after this package.

## 7. Changes in understanding

- Source-sample policy became materially stronger once `inventory` checked actual file presence, not just a hard-coded table.
- The current gap is no longer authoring flow ambiguity but the next proof-consumer pressure after tool-neutral formal hook.
- `plan/15` should stay fixture-only; source-sample policy belongs in `.docs/` plus the dedicated helper.

## 8. Open questions

- Which theorem-first concrete consumer shape is smallest: proof notebook review unit, richer bridge sketch, or another docs-only reserve.
- When to widen `e1` / `e3` / `e21` from deferred rows to authored rows.
- Whether later retained detached artifact bless/update semantics should stay separate from source-sample policy.

## 9. Suggested next prompt

- Continue with `Phase 6 theorem-first concrete tool pilot`, then close `post-checkpoint drift suppression / mirror sweep`.
