# Report 0470 — phase5 archive member body compare threshold

- Date: 2026-04-10 08:57 JST
- Author / agent: Codex
- Scope: Phase 5 theorem-line later reopen package のうち、archive-member-family-ready retained bridge の次段で actual archive member body compare をどこまで足すかの docs-first comparison
- Decision levels touched: L2

## 1. Objective

`specs/examples/177...` の次段として、
actual archive member body compare を theorem-line bridge にどこまで近づけるかを narrow に比較し、
bless / update policy との cut を current first choice として固定する。

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
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `specs/examples/176-current-l2-theorem-line-archive-bundle-ready-archive-manifest-threshold.md`
- `specs/examples/177-current-l2-theorem-line-archive-manifest-ready-archive-member-family-threshold.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`

## 3. Actions taken

1. current theorem-line later package の終点を読み直し、`archive_bundle_member_family_ref` の次段で member body compare と bless / update policy を同時に actualize しない方が narrow だと再確認した。
2. `specs/examples/178-current-l2-theorem-line-archive-member-family-ready-archive-member-body-compare-threshold.md` を追加し、`archive_member_body_compare_ref` のみを足す retained bridge を current first choice として整理した。
3. `Documentation.md`、`specs/00-document-map.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md` を current snapshot に揃えた。

## 4. Files changed

- Added: `specs/examples/178-current-l2-theorem-line-archive-member-family-ready-archive-member-body-compare-threshold.md`
- Added: `docs/reports/0470-phase5-archive-member-body-compare-threshold.md`
- Modified: `Documentation.md`
- Modified: `specs/00-document-map.md`
- Modified: `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- Modified: `plan/11-roadmap-near-term.md`
- Modified: `plan/12-open-problems-and-risks.md`
- Modified: `plan/13-heavy-future-workstreams.md`
- Modified: `plan/17-research-phases-and-autonomy-gates.md`
- Modified: `plan/90-source-traceability.md`
- Modified: `progress.md`
- Modified: `tasks.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M:%S %Z'
2026-04-10 08:57:19 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 471 numbered report(s).

$ git diff --check
(no output)
```

## 6. Evidence / findings

- `specs/examples/178-current-l2-theorem-line-archive-member-family-ready-archive-member-body-compare-threshold.md` を追加し、archive-member-family-ready retained bridge の次段では `archive_member_body_compare_ref` のみを current first choice にする判断を固定した。
- `Documentation.md`、`specs/00-document-map.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/90`、`progress.md`、`tasks.md` を `178` 基準に揃えた。
- next later reopen は `actual archive bless / update policy comparison` と読むのが自然であり、`proof_assistant_adapter` pressure は still second practical candidate に残す。

## 7. Changes in understanding

- theorem-line later package は `archive_bundle_member_family_ref` で止めず、`archive_member_body_compare_ref` までは symbolic retained bridge に寄せても narrow ratchet を壊さない。
- ただし bless / update policy と retained archive payload は同じ reopen に混ぜず、次段へ残す方が current docs-first discipline に合う。

## 8. Open questions

- actual archive bless / update policy の最小 shape
- bless-side ref と update-side ref を分けるか、policy family ref 1 本に留めるか
- retained archive payload / archive retention layout への接続

## 9. Suggested next prompt

`actual archive bless / update policy comparison` を、archive-member-body-compare-ready retained bridge の次段として docs-first で比較し、policy-family ref と retained archive payload pressure の cut を narrow に整理してください。
