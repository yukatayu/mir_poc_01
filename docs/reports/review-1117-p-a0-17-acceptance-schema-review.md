# Report 1117 — P-A0-17 Acceptance Schema Theory Review

- Date: 2026-05-02 15:13 JST
- Author / agent: Codex
- Scope: theory/spec review for `P-A0-17` helper-local acceptance artifact schema wording and scope
- Decision levels touched: none; review only

## Objective

Review whether the planned `P-A0-17` helper-local acceptance artifact schema for positive LIF/VAR rows would overclaim semantics, with focus on the explicit constraints that it remain alpha-local/helper-local/non-public synthetic evidence, stay separate from negative `reason_codes`, avoid parser/runtime/public-checker claims, and admit only `LIF-02/03/04` and `VAR-01/04/06`.

## Scope and assumptions

- Read order followed from `README.md` through `Documentation.md`, `progress.md`, `specs/00..03`, `specs/09`, `plan/00-index.md`, then the user-requested Alpha-0 specs / plans / report.
- This task is review-only; no normative or snapshot edits were made in this package.
- Findings are about wording risk, scope drift, and spec/roadmap mismatch, not about implementing the acceptance schema itself.

## Start state / dirty state

- Started from a clean worktree.
- No unrelated user-owned dirty changes were present.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/13-type-system-lifetime-fallback.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `plan/00-index.md`
- `plan/39-type-system-freeze-roadmap.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `docs/reports/1116-p-a0-16-checker-widening-closeout.md`

## Actions taken

1. Read the required repository context and Alpha-0 theory-freeze materials in the mandated order.
2. Cross-checked normative sample-family sections in `specs/13` and `specs/14` against the requested admitted positive subset.
3. Cross-checked roadmap/snapshot wording in `plan/39`, `plan/40`, `plan/43`, `progress.md`, `tasks.md`, and report `1116` for scope, non-claim, and carrier-separation wording.
4. Recorded concrete findings and wording risks for `P-A0-17`.

## Files changed

- `docs/reports/1117-p-a0-17-acceptance-schema-review.md`

## Commands run

```bash
git status --short
date '+%Y-%m-%d %H:%M:%S %z'
rg -n "^(#|##|###) " README.md Documentation.md progress.md tasks.md plan/00-index.md specs/00*.md specs/01*.md specs/02*.md specs/03*.md specs/09*.md specs/13-type-system-lifetime-fallback.md specs/14-contract-subtyping-layer-compatibility.md plan/39-type-system-freeze-roadmap.md plan/40-layer-compatibility-freeze-roadmap.md plan/43-alpha-e2e-roadmap.md docs/reports/1116-p-a0-16-checker-widening-closeout.md
rg -n "acceptance|artifact|helper-local|alpha-local|reason_code|positive row|negative row|LIF-0[234]|VAR-0[146]|parser|runtime|public checker|semantics|overclaim|widening" Documentation.md progress.md tasks.md plan/39-type-system-freeze-roadmap.md plan/40-layer-compatibility-freeze-roadmap.md plan/43-alpha-e2e-roadmap.md specs/13-type-system-lifetime-fallback.md specs/14-contract-subtyping-layer-compatibility.md docs/reports/1116-p-a0-16-checker-widening-closeout.md
sed -n '1,240p' README.md
sed -n '1,240p' Documentation.md
sed -n '1,220p' tasks.md
sed -n '1,220p' plan/00-index.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,140p' specs/01-charter-and-decision-levels.md
sed -n '1,120p' specs/02-system-overview.md
sed -n '1,120p' specs/03-layer-model.md
sed -n '1,120p' specs/09-invariants-and-constraints.md
sed -n '1,340p' specs/13-type-system-lifetime-fallback.md
sed -n '1,330p' specs/14-contract-subtyping-layer-compatibility.md
sed -n '1,220p' plan/39-type-system-freeze-roadmap.md
sed -n '1,240p' plan/40-layer-compatibility-freeze-roadmap.md
sed -n '1,220p' plan/43-alpha-e2e-roadmap.md
sed -n '1,220p' docs/reports/1116-p-a0-16-checker-widening-closeout.md
rg -n "LIF-02|LIF-03|LIF-04|VAR-01|VAR-04|VAR-06|positive/accept|positive/valid|accept-side evidence|acceptance artifact schema|reason_codes_scope|checked_reason_codes" samples/alpha specs/13-type-system-lifetime-fallback.md specs/14-contract-subtyping-layer-compatibility.md plan/39-type-system-freeze-roadmap.md plan/40-layer-compatibility-freeze-roadmap.md plan/43-alpha-e2e-roadmap.md progress.md tasks.md docs/reports/1116-p-a0-16-checker-widening-closeout.md
find samples/alpha/lifetime-fallback -maxdepth 1 -type f | sort
find samples/alpha/contract-variance -maxdepth 1 -type f | sort
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_validate_docs
git diff --check
```

## Evidence / outputs / test results

- Finding 1:
  `specs/13-type-system-lifetime-fallback.md:280-295` does not match the requested admitted helper-local positive subset. It includes later-valid rows such as `LIF-10`, `LIF-11`, and `LIF-14`, while omitting `LIF-02`. If `P-A0-17` is constrained to helper-local synthetic acceptance for only `LIF-02/03/04`, this normative sample-family section currently points too wide and misses one admitted row.
- Finding 2:
  `specs/14-contract-subtyping-layer-compatibility.md:258-280` likewise points wider than the requested helper-local acceptance subset. It lists additional valid rows such as `VAR-08`, `VAR-11`, `VAR-13`, and `VAR-14`. Without a narrower qualifier, this invites `P-A0-17` to treat broader valid-family semantics as part of the synthetic acceptance carrier instead of keeping the helper-local floor to `VAR-01/04/06`.
- Finding 3:
  Roadmap/snapshot wording is broader than the requested subset. `plan/39-type-system-freeze-roadmap.md:53-56,100-105`, `plan/40-layer-compatibility-freeze-roadmap.md:58-61,105-110`, `plan/43-alpha-e2e-roadmap.md:179-182`, `progress.md:35-38,54-55`, `tasks.md:22,77,143,179`, and `docs/reports/1116-p-a0-16-checker-widening-closeout.md:133-141` all speak in terms of generic "positive rows", "positive/valid rows", or "accept-side evidence carrier". That wording does preserve the non-claim boundary, but it does not freeze the admitted set to `LIF-02/03/04` and `VAR-01/04/06`, so a future `P-A0-17` implementation can still over-widen by following the queue wording literally.
- Finding 4:
  The current roadmap/report wording does not explicitly say that the acceptance schema must be separate from negative `reason_codes`. This omission is material because the only existing Alpha-0 checker carrier vocabulary is the negative sidecar path through `expected_static.checked_reason_codes` and `reason_codes_scope` (`plan/39-type-system-freeze-roadmap.md:44-55`, `plan/40-layer-compatibility-freeze-roadmap.md:41-42,60-61`, `docs/reports/1116-p-a0-16-checker-widening-closeout.md:123-141`). Without an explicit "do not reuse negative reason-code schema for accept-side rows" line, `P-A0-17` can accidentally collapse positive acceptance evidence into diagnostic machinery.
- Non-finding:
  The repository is already reasonably disciplined about the parser/runtime/public-checker non-claim. `Documentation.md:49-63`, `progress.md:31-38`, `plan/39-type-system-freeze-roadmap.md:44-49,100-105`, `plan/40-layer-compatibility-freeze-roadmap.md:41-44,105-110`, and `plan/43-alpha-e2e-roadmap.md:176-182` consistently keep helper-local/current-floor work distinct from parser/runtime/public completion. The larger risk is subset/scope drift, not an existing direct public-claim violation.
- `python3 scripts/check_source_hierarchy.py`
  passed.
- `python3 scripts/validate_docs.py`
  passed.
- `python3 -m unittest scripts.tests.test_validate_docs`
  passed.
- `git diff --check`
  passed.

## What changed in understanding

- The current non-claim line is mostly intact; the overclaim risk is not "public checker" wording drift so much as helper-local acceptance scope drift.
- The sharpest mismatch is at the normative sample-family boundary: `specs/13` omits one requested admitted row (`LIF-02`) and both `specs/13`/`specs/14` currently describe broader valid families than the requested helper-local acceptance subset.
- The second sharp risk is schema-shape ambiguity: the docs ask for an acceptance carrier, but do not yet say that it must be structurally distinct from negative `reason_codes`.

## Open questions

- Should the narrow admitted subset for `P-A0-17` be expressed normatively in `specs/13`/`specs/14`, or only in roadmap/snapshot docs as a helper-local floor?
- Should `P-A0-17` introduce a new acceptance-only field/schema name immediately, or first add an explicit stop line that positive rows must not reuse `checked_reason_codes` / `reason_codes_scope`?
- Should later-valid rows such as `LIF-10/11/14` and `VAR-08/11/13/14` be explicitly marked "not part of helper-local acceptance schema" in roadmap memory to prevent queue drift?

## Suggested next prompt

Narrow `P-A0-17` docs explicitly: freeze the admitted helper-local positive subset to `LIF-02/03/04` and `VAR-01/04/06`, add an explicit stop line that acceptance artifacts are separate from negative `checked_reason_codes` / `reason_codes_scope`, and keep parser/runtime/public-checker claims out of the schema wording.

## Plan update status

`plan/` 更新不要: this task was a review-only audit and did not resolve the wording/scope mismatch yet.

## Documentation.md update status

`Documentation.md` 更新不要: no direct mismatch was found in the current non-claim framing there.

## progress.md update status

`progress.md` 更新不要: this task did not change current repo status, only reviewed the wording risk in the queued `P-A0-17` work.

## tasks.md update status

`tasks.md` 更新不要: the scope issue was identified, but no queue rewrite was performed in this review-only task.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample state changed.

## Reviewer findings and follow-up

- No external reviewer or sub-agent was invoked for this review-only task.
- Follow-up required in a later package:
  narrow the queued `P-A0-17` wording and, if chosen, update the normative sample-family references so they do not imply a broader helper-local acceptance carrier than intended.

## Skipped validations and reasons

- No Cargo/runtime/sample-family execution floors were rerun because this task made no implementation changes and evaluated wording/scope only.

## Commit / push status

Pending at report write.

No commit or push was performed in this review-only task.

## Sub-agent session close status

- No sub-agent sessions were started.
