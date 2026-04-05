# 0216 — try-rollback AST helper first tranche cut

## Objective

future dedicated AST structural helper を actualize するときの first tranche を、helper code、fixture-side fields、malformed static family、static gate smoke path のどこまで一体で切るべきかという観点から narrow に比較する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- dedicated AST structural helper は future option であり、今回は docs-only comparison だけを扱う。
- actual helper 実装、actual fixture field 追加、malformed fixture actualization、shared carrier / public checker API actualization は行わない。
- `plan/07` / `plan/09` は helper stack の actual execution path 自体に変更が無いため更新不要とする。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/61-current-l2-try-rollback-ast-helper-subcommand-and-wrapper-family.md`
- `specs/examples/63-current-l2-try-rollback-ast-helper-public-checker-entry-criteria.md`
- `specs/examples/64-current-l2-try-rollback-malformed-static-family-timing.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. helper code 先行案、helper code + fixture fields + malformed corpus + smoke path を一体で切る案、そこへ shared carrier / public-looking surface まで混ぜる案を比較した。
2. `specs/examples/65-current-l2-try-rollback-ast-helper-first-tranche-cut.md` を追加し、first tranche は helper code、fixture-side fields、minimal malformed static family、static gate smoke path を一体で切り、shared carrier / public checker API は外に残す judgment を固定した。
3. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/15`、`plan/90`、`progress.md` を更新した。

## Files changed

- `specs/examples/65-current-l2-try-rollback-ast-helper-first-tranche-cut.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0216-try-rollback-ast-helper-first-tranche-cut.md`
- `docs/reports/0217-review-try-rollback-ast-helper-first-tranche-cut.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 217 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- local comparison の要点
  - helper code だけを first tranche にすると、fixture-side field と smoke path が揃わず半完成状態になりやすい
  - malformed static family timing judgement と整合させるなら、first tranche では helper / fields / corpus / smoke path を同時に切る方が自然である
  - shared carrier / public checker API は first tranche に含めず、後段 threshold へ残す方が current boundary を壊しにくい

## What changed in understanding

- dedicated helper actualization の first tranche は、helper code 単体ではなく helper-local family を実地反復可能にする最小単位として切る必要がある。

## Open questions

- minimal malformed static family tranche の exact size。
- actual helper actualization 時の exact subcommand 名。

## Suggested next prompt

future dedicated AST structural helper の first tranche cut を前提に、その minimal malformed static family tranche の exact size をどこまで narrow に切るかを docs-only で比較してください。
