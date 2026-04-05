# 0223 — review trail for try-rollback AST helper first tranche actualization

## Objective

`0222` の first tranche actualization が、current L2 semantics、helper boundary、docs / plan / progress mirror を壊していないかを review する。

## Scope and assumptions

- reviewer は 1 回だけ起動する。
- wait は長めに 2 回まで行う。
- completion が返らない場合だけ retry を 1 回行う。
- それでも返らない場合は local evidence fallback を採る。

## Documents consulted

- `docs/reports/0222-try-rollback-ast-helper-first-tranche-actualization.md`
- `specs/examples/65-current-l2-try-rollback-ast-helper-first-tranche-cut.md`
- `specs/examples/66-current-l2-try-rollback-malformed-static-tranche-size.md`
- `specs/examples/67-current-l2-try-rollback-malformed-pattern-slot-selection.md`
- `specs/examples/68-current-l2-try-rollback-ast-helper-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`

## Actions taken

1. dedicated helper-local first tranche の code / docs / mirror diff を self-check した。
2. fresh validation (`unittest`、smoke、`cargo test`、`validate_docs`、`git diff --check`) を揃えた。
3. reviewer completion が取れれば findings を反映し、返らなければ local evidence fallback を残す。
4. reviewer から 2 finding が返ったため、dedicated helper を artifact-driven compare に修正し、helper-local carrier の public leak を private field / private type + crate-local unit test へ戻した。

## Evidence / outputs / test results

- reviewer

```text
1. dedicated helper が fixture AST から findings を再計算しており、emitted static gate artifact contract を検査していない。
2. helper-local carrier が `mir-semantics` public surface に漏れている。
```

- post-fix verification

```text
python3 -m unittest scripts.tests.test_current_l2_try_rollback_structural_checker scripts.tests.test_current_l2_static_gate_loop
OK

python3 scripts/current_l2_detached_loop.py smoke-try-rollback-structural-checker ...e23...
status: matched

python3 scripts/current_l2_detached_loop.py smoke-try-rollback-structural-checker ...e24...
status: matched

cargo test -p mir-semantics
test result: ok

python3 scripts/validate_docs.py
Documentation scaffold looks complete.

git diff --check
<no output>
```

## What changed in understanding

- dedicated try/rollback first tranche は、docs-only judgment、fixture corpus、static gate smoke、helper-local compare の 4 点が揃って初めて close できることが確認できた。
- smoke family は helper が fixture AST を読んで self-fulfilling に通る形では不十分であり、emitted artifact contract を読む negative coverage を伴って初めて close できることが確認できた。
- current remaining OPEN は second tranche / shared carrier / public checker comparison であり、first tranche 自体の helper boundary は壊れていない。
- helper-local carrier は public surface に出さず、fixture decode のための crate-local detail として維持する cut に戻した。

## Open questions

- next narrow step を second tranche comparison に置くか、first-tranche wording stability comparison に置くか。

## Suggested next prompt

first tranche actualization 済みの dedicated try/rollback AST structural helper について、次に second malformed static tranche を足すべきか、それとも helper-local wording / finding family を first tranche のまま維持して saved artifact compare need を観察すべきかを narrow に比較してください。
