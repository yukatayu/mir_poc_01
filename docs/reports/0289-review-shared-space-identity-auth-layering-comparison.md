# Report 0289 — review shared-space identity / auth layering comparison

- Date: 2026-04-08 12:36 JST
- Author / agent: Codex
- Scope: Local review fallback for the docs-only identity / auth layering comparison
- Decision levels touched: L1/L2 wording review only; no normative change

## 1. Objective

Record the review closeout for the new shared-space identity / auth layering comparison while checking whether it:

- keeps identity core, auth stack, admission policy, and display identity separate enough,
- avoids turning auth protocol details into room semantics,
- stays consistent with the existing membership / authority / fairness line,
- closes traceability and report hygiene.

## 2. Inputs consulted

- `AGENTS.md`
- `Documentation.md`
- `progress.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/reports/0288-shared-space-identity-auth-layering-comparison.md`
- `docs/reports/0284-shared-space-fairness-trust-model-comparison.md`
- `https://blog.yukatayu.tech/blog/sync_language_01/`
- `https://blog.yukatayu.tech/blog/sync_language_02/`

## 3. Actions taken

1. Re-read the changed shared-space section with focus on the new identity / auth line.
2. Checked whether the recommended cut keeps participant carrier, principal continuity, and room-local permission distinct.
3. Checked whether the authoritative game room pseudocode still avoids raw credential leakage into room semantics.
4. Checked `plan/12`, `progress.md`, and `plan/90` mirrors for consistency and traceability.

## 4. Files changed

- `docs/reports/0289-review-shared-space-identity-auth-layering-comparison.md` (new)
- `plan/` 更新不要
- `progress.md` 更新不要

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 12:36 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 289 numbered report(s).

$ git diff --check
[no output]
```

## 6. Evidence / findings

No substantive finding.

The comparison stays consistent with the existing shared-space line:

- participant registry remains the source of truth for membership rather than for raw auth protocol state,
- principal continuity is preserved better than in the fully opaque actor-handle option,
- authoritative room pseudocode still keeps `request_roll` on `member_ref` and room-local predicates rather than on auth token material,
- traceability now names `0288` / `0289` in the relevant `plan/90` rows.

Operational note:

- reviewer completion was not obtainable through the currently available interface during this task window, so this file records the required local-review fallback explicitly.

## 7. Changes in understanding

- The useful boundary is not simply “auth is external”; it is “membership keeps identity core, while auth stack and admission policy stay external enough not to pollute room semantics.”

## 8. Open questions

- Whether future fairness witness should bind to `principal_ref`, `member_ref`, or both.
- Whether `display_ref` belongs in identity core or should be treated as a projection-layer concern.

## 9. Suggested next prompt

`shared-space admission policy と compile-time over-approximation の接点を、room capability / visibility requirement の line とどう切るか、current shared-space line の残課題として narrow に比較してください。`
