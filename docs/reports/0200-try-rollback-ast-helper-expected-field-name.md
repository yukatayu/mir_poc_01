# 0200 — try-rollback AST helper expected field name

## Objective

future dedicated AST structural helper を actualize する場合の expected field 名と focused compare shape を、helper-local dedicated contract を崩さない範囲でどこまで narrow に切るべきかを source-backed に比較する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- malformed static family はまだ actualize しない current judgment を維持する。
- dedicated AST structural helper は future option であり、今回は docs-only refinement だけを扱う。
- `plan/07` と `plan/09` は helper stack 自体の変更が無いため更新不要とする。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
- `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`
- `specs/examples/55-current-l2-try-rollback-malformed-static-family-actualization.md`
- `specs/examples/56-current-l2-try-rollback-ast-helper-compare-contract.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0198-try-rollback-ast-helper-compare-contract.md`
- `docs/reports/0199-review-try-rollback-ast-helper-compare-contract.md`

## Actions taken

1. optional expected field 名候補として `checked_try_rollback_structural_findings`、`try_rollback_structural_findings`、`structural_findings`、`checked_try_rollback_structural_rows` を比較した。
2. focused compare shape 候補として string list、helper-local row list、generic structural carrier を比較した。
3. `specs/examples/57-current-l2-try-rollback-ast-helper-expected-field-name.md` を追加し、current docs-only minimum を `expected_static.checked_try_rollback_structural_findings` と `subject_kind` / `finding_kind` row list に固定した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/15-current-l2-fixture-authoring-template.md`、`plan/90-source-traceability.md`、`progress.md` を更新した。

## Files changed

- `specs/examples/57-current-l2-try-rollback-ast-helper-expected-field-name.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0200-try-rollback-ast-helper-expected-field-name.md`
- `docs/reports/0201-review-try-rollback-ast-helper-expected-field-name.md`

## Commands run

```bash
df -h .
free -h
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

### task-start dirty state

- `git status --short --branch`

```text
## main...origin/main
?? docs/reports/0200-try-rollback-ast-helper-expected-field-name.md
?? docs/reports/0201-review-try-rollback-ast-helper-expected-field-name.md
```

### Commands run and exact outputs

- `df -h .`

```text
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   79G   16G  84% /
```

- `free -h`

```text
               total        used        free      shared  buff/cache   available
Mem:           960Mi       711Mi        73Mi       1.2Mi       328Mi       248Mi
Swap:           19Gi       1.9Gi        18Gi
```

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 200 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

### Findings

- current phase の最小 expected field 名候補は `expected_static.checked_try_rollback_structural_findings` である。
- `checked_` prefix を落とすと compare field と explanatory prose の境界が弱くなり、generic `structural_findings` まで短くすると helper-local dedicated cut が濁る。
- focused compare shape は string list より helper-local row list の方が自然であり、minimum は `subject_kind` / `finding_kind` の 2 要素で足りる。
- `subject_path`、source span、checker family label を入れる案は current phase では premature であり、malformed family actualization や shared carrier を先取りしすぎる。

## What changed in understanding

- future dedicated AST structural helper を actualize するとしても、fixture-side expected field と compare row は generic structural carrier へ広げず、helper-local dedicated naming family に留める方が current docs-only cut と整合する。
- `plan/15` には future field 候補として書けるが、current fixture schema の actual field と誤読しないよう明示する必要がある。

## Open questions

- dedicated helper を detached validation loop のどこへ差し込むか。
- dedicated helper を actualize した後、detached artifact shared carrier へ上げる閾値をどこに置くか。
- malformed static family を actual corpus に増やす必要が本当にあるか。

## Suggested next prompt

future dedicated AST structural helper の optional expected field 名と focused compare shape を前提に、その helper を detached validation loop のどこへ差し込むのが最小かを docs-only で比較してください。shared detached carrier や public checker API にはまだ上げない current cut を維持してください。
