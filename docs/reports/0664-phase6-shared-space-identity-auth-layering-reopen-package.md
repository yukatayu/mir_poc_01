# Report 0664 — phase6 shared-space identity/auth layering reopen package

- Date: 2026-04-13T07:28:33+0900
- Author / agent: Codex
- Scope: Close the docs-first package that fixes the shared-space identity/auth layering reopen line (`365...366`) and sync the repository snapshots to the next current line.
- Decision levels touched: current L2 docs-first boundary package only.

## 1. Objective

- Close `shared-space identity / auth layering reopen` without collapsing auth / admission / projection concerns into the membership carrier.
- Keep the current Mirrorea/shared-space docs-first boundary narrow and hand off the repo-level current line to `model-check concrete carrier first actualization gate`.
- Refresh mirrors so the repository no longer reads as if identity/auth layering were still only an open queue item.

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/121...125`
- `specs/examples/295...296`
- `specs/examples/357...364`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `faq_003.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`

## 3. Actions taken

1. Re-read the shared-space re-entry package, risk register, and long-form membership boundary memo to isolate the narrowest next docs-first cut.
2. Promoted the plan/16 comparison into normative package `365...366`, comparing:
   - embedding identity/auth/projection into membership,
   - keeping membership identity core while splitting auth/admission/projection,
   - pushing everything to opaque handles.
3. Fixed the current first choice to keep `member_ref + principal_ref + member_incarnation + activation_state` in the membership identity core, while leaving auth/admission and projection in side carriers.
4. Updated repository snapshots and plans so the current line advances from `shared-space identity / auth layering reopen` to `model-check concrete carrier first actualization gate`.
5. Left compile-time admission visibility, authority/resource ownership, and concrete auth protocol binding as later shared-space follow-up lines.

## 4. Files changed

- `.docs/current-l2-source-sample-authoring-policy.md`
- `Documentation.md`
- `docs/reports/0664-phase6-shared-space-identity-auth-layering-reopen-package.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `faq_003.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/current-l2/README.md`
- `specs/00-document-map.md`
- `specs/examples/365-current-l2-public-operational-surface-actualization-gate-ready-shared-space-identity-auth-layering-reopen-comparison.md`
- `specs/examples/366-current-l2-shared-space-identity-auth-layering-reopen-ready-minimal-shared-space-identity-auth-layering-reopen-threshold.md`
- `tasks.md`

## 5. Commands run and exact outputs

- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 663 numbered report(s).`
- `git diff --check`
  - no output

## 6. Evidence / findings

- The narrowest current shared-space cut keeps principal continuity in the membership registry but leaves transport/service auth, room admission, and display/projection identity outside the membership core.
- This split preserves authoritative-room audit/blame readability without leaking raw auth protocol into room semantics.
- Compile-time admission visibility and authority/resource ownership are still valid next reopen lines, but they do not need to be folded into the identity/auth package.
- After this package, the repository current line naturally advances to `model-check concrete carrier first actualization gate`.

## 7. Changes in understanding

- The previous long-form memo already contained the decisive comparison; the missing step was promoting it into the normative spec chain so snapshot docs could stop treating it as prose-only.
- `display_ref` is safer as a projection-side concern than as a membership-core field in the current docs-first line, because the repository keeps platform/world/avatar layering separable.
- The next shared-space follow-up should be admission/compile-time visibility, not a premature auth-protocol or operational-runtime binding.

## 8. Open questions

- Should a future derived snapshot carry a projection/display ref while still keeping projection outside the canonical membership core?
- How much of admission/visibility over-approximation should be declaration-side versus plan-only before the next shared-space reopen?
- When auth-heavy deployments are revisited, should the opaque-handle cut return as a later alternative track?

## 9. Suggested next prompt

- Close `model-check concrete carrier first actualization gate`, then run a consistency audit so snapshots, FAQs, and abstracts all point at the new repo-level current line.
