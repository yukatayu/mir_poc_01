# Report 0329 — current l2 stage3 request contract subset first tranche actualization

- Date: 2026-04-08T08:32:55.701784Z
- Author / agent: Codex
- Scope: Phase 3 later branch で `Stage3RequestContractSubset` 相当の helper-local / test-only first tranche を actualize し、fixed two-slot suite bridge と fixture-side contract subset compare の接点を source-backed にする
- Decision levels touched: L2

## 1. Objective

spec108 の first-cut judgment を actual code anchor へ落とし、request head parse を still later に残したまま、fixture-side `contract.require` / `contract.ensure` subset compare を explicit carrier と focused smoke で actualize する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/107-current-l2-stage3-suite-reopen-conditions.md`
- `specs/examples/108-current-l2-stage3-request-contract-subset-compare-cut.md`
- `specs/examples/101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json`
- `crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json`

## 3. Actions taken

1. existing suite bridge test と fixture-side predicate fragment helper を再確認した。
2. `Stage3RequestContractSubset` carrier と `load_fixture_request_contract_subset(...)` helper を fixture-side support へ追加した。
3. source-side `Stage3RequestClauseSuite` を local に contract subset carrier へ parse し、fixture-side expected carrier と compare する focused smoke を追加した。
4. actualized scope / outside-first-tranche line を `specs/examples/109...` に記述した。

## 4. Files changed

- `specs/examples/109-current-l2-stage3-request-contract-subset-first-tranche-actualization.md`
- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0329-current-l2-stage3-request-contract-subset-first-tranche-actualization.md`

## 5. Commands run and exact outputs

```bash
cargo test -p mir-ast --test current_l2_stage3_request_clause_suite_spike
```

targeted suite first failure:

```text
error[E0432]: unresolved imports `current_l2_stage3_predicate_fragment_spike_support::load_fixture_request_contract_subset`, `current_l2_stage3_predicate_fragment_spike_support::Stage3RequestContractSubset`
```

after helper actualization:

```bash
cargo test -p mir-ast --test current_l2_stage3_request_clause_suite_spike
```

```text
running 11 tests
test result: ok. 11 passed; 0 failed
```

full verification:

```bash
cargo test -p mir-ast
python3 scripts/validate_docs.py
git diff --check
```

主要出力:

```text
test result: ok. mir-ast targeted suites all passed; 0 failed
Documentation scaffold looks complete.
Found 331 numbered report(s).
git diff --check => no output
```

## 6. Evidence / findings

- helper-local expected carrier を dedicated に切ることで、source-side suite bridge と fixture-side contract subset compare の接点を explicit に actualize できた。
- same source-side suite carrier を `e10` (`PerformOn`) と `e11` (`PerformVia`) の両方へ compare でき、request head metadata を still later に残す cut が source-backed になった。
- fixture-side helper は clause array 長が 0 or 1 の first tranche に留め、2 個以上は `outside stage 3 first tranche` で fail-closed にしている。

## 7. Changes in understanding

- `Stage3RequestContractSubset` は source-side parse carrierではなく、fixture-side expected shape と local compare path を結ぶ helper-local bridge として置くのが最も自然だと確認した。
- request head kind / op / target / chain_ref を carry しなくても、current Phase 3 later branch の contract-only compare line は十分に前進できる。

## 8. Open questions

- next comparison では、
  - request head neutral carrier のまま contract-only compare surface を widening するか
  - fixture-side widening guard を別 docs-only comparison として切るか
  を決める必要がある。

## 9. Suggested next prompt

`specs/examples/109-current-l2-stage3-request-contract-subset-first-tranche-actualization.md` を前提に、request head metadata を still later に残したまま、contract-only compare surface の次段 widening をどこで止めるかを narrow に比較してください。
