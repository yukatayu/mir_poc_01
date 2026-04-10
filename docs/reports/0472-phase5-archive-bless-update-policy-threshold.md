# 0472 — Phase 5 archive bless/update policy threshold

## Objective

`specs/examples/178-current-l2-theorem-line-archive-member-family-ready-archive-member-body-compare-threshold.md`
の次段として、
actual archive bless / update policy を theorem-line retained bridge にどこまで寄せるかを
docs-first で比較し、current first choice を固定する。

## Scope and assumptions

- `proof_notebook` first bridge だけを扱う。
- `archive_member_body_compare_ref` までは current first choice として入っている前提に立つ。
- retained archive payload / archive retention layout は今回の reopen に巻き込まない。

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
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md`
- `specs/examples/177-current-l2-theorem-line-archive-manifest-ready-archive-member-family-threshold.md`
- `specs/examples/178-current-l2-theorem-line-archive-member-family-ready-archive-member-body-compare-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. `178` までの theorem-line retained bridge を読み直し、archive bless / update policy の reopen が
   - body compare ready を terminal cut に保つ
   - policy family ref 1 本だけを足す
   - bless/update split や retained payload をまとめて足す
   の 3 案に整理できることを確認した。
2. `specs/examples/179-current-l2-theorem-line-archive-member-body-compare-ready-archive-bless-update-policy-threshold.md`
   を追加し、current first choice を
   `archive_bless_update_policy_ref` だけを足す archive-bless-update-policy-ready retained bridge
   に固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、
   `plan/90`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
   を更新し、next later reopen を retained archive payload comparison に切り替えた。

## Evidence / outputs / test results

- resource check:
  - `df -h .`
  - `free -h`
- validation:
  - `python3 scripts/validate_docs.py`
  - `git diff --check`
- reviewer:
  - reviewer subagent を 1 回だけ起動し、completion まで待って substantive finding が無いことを確認した

## What changed in understanding

- `archive_member_body_compare_ref` の次段では、bless/update split 自体ではなく
  **policy family ref 1 本だけを retained bridge に足す** のが最も narrow である。
- retained archive payload は policy family と同じ reopen に混ぜず、next later reopen に残す方が
  theorem-line bridge discipline を保ちやすい。
- したがって current next pressure は actual archive bless/update policy comparison ではなく、
  **retained archive payload comparison** に移る。

## Open questions

- retained archive payload の最小 shape
- actual bless-side row と update-side row をどこで split するか
- `proof_assistant_adapter` pressure を second practical candidate のまま維持する条件

## Suggested next prompt

`retained archive payload comparison` を Phase 5 theorem-line の next later reopen として進め、`archive_bless_update_policy_ref` の次段で retained archive payload をどこまで retained bridge に寄せるかを docs-first で比較してください。
