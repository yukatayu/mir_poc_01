# Report 0038 — review admit runtime reading

- Date: 2026-03-31T07:33:15.151222Z
- Author / agent: Codex
- Scope: Review only the uncommitted diff in `specs/examples/01-current-l2-surface-syntax-candidates.md`, `specs/examples/00-representative-mir-programs.md`, `specs/10-open-questions.md`, and `specs/12-decision-register.md`.
- Decision levels touched: L2 review only; no normative text changed.

## 1. Objective

Review the current L2 option-local `admit` runtime reading in the uncommitted diff and determine whether it:

- breaks canonical normalization law / rejection phase / static evidence floor / underdeclared handling / request-local `require`-`ensure` policy
- conflicts with existing boundaries between explicit failure and dynamic `Reject`
- prematurely fixes an unresolved observation surface

Result: no findings.

## 2. Scope and assumptions

- Scope was limited to the uncommitted diff in the four files named above.
- Baseline semantics were read from the repository-required document order before reviewing the diff.
- This was a review task only. Specs were not edited. Only this report was added.

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`

## 4. Actions taken

1. Read the required baseline documents in repository order.
2. Read the relevant Mir Core sections on canonical normalization, rejection phase, static evidence floor, underdeclared handling, and runtime `Reject`.
3. Inspected the uncommitted diff for the four target files.
4. Compared the new `admit` runtime wording against the existing L2 constraints and the representative examples.
5. Recorded the review result in this report.

## 5. Files changed

- `docs/reports/0038-review-admit-runtime-reading.md`

## 6. Commands run

```bash
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/requesting-code-review/SKILL.md
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/verification-before-completion/SKILL.md
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/systematic-debugging/SKILL.md
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' specs/00-document-map.md
sed -n '1,260p' specs/01-charter-and-decision-levels.md
sed -n '1,260p' specs/02-system-overview.md
sed -n '1,260p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,320p' specs/04-mir-core.md
git diff -- specs/examples/01-current-l2-surface-syntax-candidates.md specs/examples/00-representative-mir-programs.md specs/10-open-questions.md specs/12-decision-register.md
rg -n "admit|admissible|canonical law|rejection phase|static evidence floor|underdeclared|require|ensure|Reject|explicit failure|observation|observ|non-admissible skip" specs/04-mir-core.md specs/examples/00-representative-mir-programs.md specs/examples/01-current-l2-surface-syntax-candidates.md specs/10-open-questions.md specs/12-decision-register.md
nl -ba specs/04-mir-core.md | sed -n '118,183p'
nl -ba specs/examples/01-current-l2-surface-syntax-candidates.md | sed -n '100,145p'
nl -ba specs/examples/01-current-l2-surface-syntax-candidates.md | sed -n '330,342p'
nl -ba specs/examples/00-representative-mir-programs.md | sed -n '220,232p'
nl -ba specs/examples/00-representative-mir-programs.md | sed -n '336,344p'
nl -ba specs/10-open-questions.md | sed -n '72,83p'
nl -ba specs/12-decision-register.md | sed -n '38,44p'
python scripts/new_report.py --slug review-admit-runtime-reading
python3 scripts/new_report.py --slug review-admit-runtime-reading
```

Notable output:

- `python scripts/new_report.py --slug review-admit-runtime-reading`
  - `/usr/bin/bash: line 1: python: command not found`
- `python3 scripts/new_report.py --slug review-admit-runtime-reading`
  - created `docs/reports/0038-review-admit-runtime-reading.md`

## 7. Evidence / outputs / test results

No findings.

Evidence for that conclusion:

- Canonical law / rejection phase:
  - `specs/04-mir-core.md:141-152` already defines canonical evaluation in terms of the leftmost viable or admissible option and reserves dynamic `Reject` for the case where a well-formed chain runs out of admissible options.
  - The new text in `specs/examples/01-current-l2-surface-syntax-candidates.md:122-129`, `specs/examples/00-representative-mir-programs.md:226-230`, `specs/10-open-questions.md:77-81`, and `specs/12-decision-register.md:43` applies that existing runtime boundary to option-local `admit` without changing the canonical law itself.
- Static evidence floor / underdeclared handling:
  - `specs/04-mir-core.md:175-181` keeps underdeclared fallback cases as surface-level static errors when declared information is missing.
  - The diff does not soften that rule. It only specifies the runtime reading after `admit` metadata is present and evaluates to false.
- Request-local `require` / `ensure` policy:
  - `specs/examples/01-current-l2-surface-syntax-candidates.md:105-112` and `specs/10-open-questions.md:77-79` still keep `admit` separate from statement-local `require` / `ensure`.
  - The new runtime wording continues to treat `require` as request-local and `admit` as option-local admissibility metadata.
- Explicit failure vs dynamic `Reject`:
  - Existing example E3 already uses explicit per-option failure followed by fallback success at `specs/examples/00-representative-mir-programs.md:186-192`.
  - Existing example E6 already uses runtime `Reject` only after no write-admissible option remains at `specs/examples/00-representative-mir-programs.md:337-343`.
  - The new `admit` wording aligns with that same boundary instead of redefining it.
- Observation surface:
  - The diff explicitly leaves unresolved whether `admit` non-satisfaction is represented as a dedicated trace / audit event or only as skip reason at `specs/examples/01-current-l2-surface-syntax-candidates.md:129`, `specs/10-open-questions.md:81`, and `specs/12-decision-register.md:43`.
  - Therefore the change does not prematurely fix the observation surface.

## 8. What changed in understanding

- Before review, the main risk was that option-local `admit` might have introduced a third runtime category that bypassed the existing canonical chain and rejection rules.
- After review, the added text reads as a clarification of the existing L2 boundary: `admit` false means the option never becomes a success-side candidate, while explicit failure and final dynamic `Reject` remain request-level runtime outcomes with the same boundaries as before.

## 9. Open questions

- No new open questions were introduced by this review.
- Existing unresolved item remains: whether `admit` non-satisfaction should be exposed as a dedicated trace / audit event or only as skip reason.

## 10. Suggested next prompt

Review whether the current L2 wording needs a single explicit cross-reference from the `admit` runtime reading back to `specs/04-mir-core.md` canonical evaluation terminology (`leftmost viable option`) for terminological consistency only, without changing semantics.
