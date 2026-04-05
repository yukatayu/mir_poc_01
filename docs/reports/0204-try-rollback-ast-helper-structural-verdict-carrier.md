# 0204 — try-rollback AST helper structural verdict carrier

## Objective

future dedicated AST structural helper の structural verdict carrier / name を、finding rows と full static gate verdict を混同しない範囲でどこまで narrow に切るべきかを source-backed に比較する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- dedicated AST structural helper は future option であり、今回は docs-only comparison だけを扱う。
- actual helper 実装、actual fixture schema field 追加、shared detached carrier 追加は行わない。
- `plan/07` / `plan/09` は helper stack 実 execution path 自体に変更が無いため更新不要とする。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/56-current-l2-try-rollback-ast-helper-compare-contract.md`
- `specs/examples/57-current-l2-try-rollback-ast-helper-expected-field-name.md`
- `specs/examples/58-current-l2-try-rollback-ast-helper-detached-loop-insertion.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Actions taken

1. structural verdict carrier の placement を、findings list 推論、dedicated verdict field、`expected_static.verdict` 流用の 3 案で比較した。
2. dedicated verdict field の field 名候補として `checked_try_rollback_structural_verdict`、`try_rollback_structural_verdict`、`checked_structural_verdict`、`verdict` を比較した。
3. verdict value shape 候補として bool、`valid` / `malformed`、helper-local string enum `no_findings` / `findings_present` を比較した。
4. `specs/examples/59-current-l2-try-rollback-ast-helper-structural-verdict-carrier.md` を追加し、current docs-only minimum を `expected_static.checked_try_rollback_structural_verdict` と helper-local string enum `no_findings` / `findings_present` に固定した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/15`、`plan/90`、`progress.md` を更新した。

## Files changed

- `specs/examples/59-current-l2-try-rollback-ast-helper-structural-verdict-carrier.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0204-try-rollback-ast-helper-structural-verdict-carrier.md`
- `docs/reports/0205-review-try-rollback-ast-helper-structural-verdict-carrier.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 205 numbered report(s).
```

- `git diff --check`

```text
<no output>
```

- local comparison の要点
  - findings list だけから verdict を推論すると compare contract の meaning が hidden になる
  - `expected_static.verdict` を流用すると full static gate verdict と dedicated helper-local structural verdict が衝突する
  - dedicated verdict field を `checked_try_rollback_structural_verdict` として分け、value を helper-local string enum に留めると、future helper-local contract と current fixture authoring boundary の両方を保ちやすい

## What changed in understanding

- future dedicated AST structural helper の compare contract は、finding rows だけでなく verdict も dedicated field に分けた方が自然である。
- ただしその verdict は full static gate verdict ではなく、helper-local finding contract に結びついた narrow string enum に留める方が current phase に合う。

## Open questions

- detached artifact shared carrier へ上げる閾値。
- actual subcommand 名をいつ決めるか。
- malformed static family を actual corpus に増やす必要が本当にあるか。

## Suggested next prompt

future dedicated AST structural helper の expected finding field、detached-loop insertion、structural verdict carrier を前提に、その helper を detached artifact shared carrier へ上げる閾値をどこに置くかを docs-only で比較してください。current helper-local dedicated cut は維持してください。

