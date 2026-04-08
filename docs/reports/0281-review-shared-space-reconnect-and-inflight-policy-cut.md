# Report 0281 — review shared space reconnect / late leave / in-flight policy cut

- Date: 2026-04-08 12:04 JST
- Author / agent: Codex
- Scope: Uncommitted docs-only review of the reconnect / late leave / in-flight policy cut and its mirrors
- Decision levels touched: L1/L2 wording review only; no normative change

## 1. Objective

Review the current uncommitted reconnect / late leave / in-flight policy cut in:

- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `progress.md`
- `docs/reports/0280-shared-space-reconnect-and-inflight-policy-cut.md`

with focus on:

- whether room profile and external policy layer remain separated
- whether `member_incarnation` and uncommitted action invalidation are scoped as the minimal room-profile kernel
- whether timeout / retry / resend are kept external
- report hygiene / traceability

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/08-cross-system-relations.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/90-source-traceability.md`
- `docs/reports/0280-shared-space-reconnect-and-inflight-policy-cut.md`
- `docs/reports/0278-shared-space-authoritative-game-room-profile-comparison.md`

## 3. Actions taken

1. Read the repo’s required normative stack and relevant `plan/` memory before reviewing the diff.
2. Inspected the uncommitted diff for the four target files.
3. Checked the reconnect-policy wording against the shared-space separation rules and the earlier authoritative room bundle comparison.
4. Compared report 0280’s hygiene and traceability against the immediately preceding shared-space report chain.

## 4. Files changed

- `docs/reports/0281-review-shared-space-reconnect-and-inflight-policy-cut.md` (new)
- `plan/` 更新不要
- `progress.md` 更新不要

## 5. Commands run and exact outputs

```text
$ git status --short --branch
## main...origin/main
 M plan/12-open-problems-and-risks.md
 M plan/16-shared-space-membership-and-example-boundary.md
 M progress.md
?? docs/reports/0280-shared-space-reconnect-and-inflight-policy-cut.md

$ git diff -- plan/12-open-problems-and-risks.md plan/16-shared-space-membership-and-example-boundary.md progress.md docs/reports/0280-shared-space-reconnect-and-inflight-policy-cut.md
Reviewed the uncommitted docs-only diff for the four target files.

$ rg -n "0280|shared-space-reconnect-and-inflight-policy-cut" plan/90-source-traceability.md docs/reports
docs/reports/0280-shared-space-reconnect-and-inflight-policy-cut.md:1:# 0280 — shared-space reconnect / late leave / in-flight policy cut
docs/reports/0280-shared-space-reconnect-and-inflight-policy-cut.md:56:- `docs/reports/0280-shared-space-reconnect-and-inflight-policy-cut.md`

$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 12:04 JST
```

## 6. Evidence / findings

Initial review found 2 hygiene issues, and both were fixed in-task:

1. `plan/90` traceability mirror needed `0280` / `0281` references for both [plan/12-open-problems-and-risks.md](/home/yukatayu/dev/mir_poc_01/plan/12-open-problems-and-risks.md) and [plan/16-shared-space-membership-and-example-boundary.md](/home/yukatayu/dev/mir_poc_01/plan/16-shared-space-membership-and-example-boundary.md). This is now closed in [plan/90-source-traceability.md](/home/yukatayu/dev/mir_poc_01/plan/90-source-traceability.md).

2. Report 0280 needed final validation closure in `Commands run` / `Evidence / outputs / test results`. This is now closed in [0280](/home/yukatayu/dev/mir_poc_01/docs/reports/0280-shared-space-reconnect-and-inflight-policy-cut.md).

No substantive semantic finding on the comparison content itself. The new wording keeps the room-profile kernel narrow, keeps timeout / retry / resend outside the room profile, and uses `member_incarnation` as the safety boundary for uncommitted actions in a way that remains consistent with the existing shared-space line.

## 7. Changes in understanding

- The substantive docs line is sound: the policy cut is treated as a split between room safety semantics and operational delivery policy, not as a hidden expansion of the room profile.
- The remaining work is repository-memory hygiene and validation closeout, not a semantic rewrite.

## 8. Open questions

- None from this review. Remaining open questions stay with the main comparison itself, not the review closure.

## 9. Suggested next prompt

`shared-space membership epoch / incarnation と causal metadata の接続について、plain vector deletion / epoch-incarnation split / control-plane separated carrier を narrow に比較してください。`
