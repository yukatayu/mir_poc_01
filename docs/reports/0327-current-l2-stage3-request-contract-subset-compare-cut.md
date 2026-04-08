# Report 0327 — current L2 stage3 request contract subset compare cut

- Date: 2026-04-08T08:19:17.224251Z
- Author / agent: Codex
- Scope: Phase 3 later branch で fixed two-slot suite bridge を fixture-side full request contract subset compare へ actualize する first cut の docs-only comparison
- Decision levels touched: L2

## 1. Objective

spec107 の reopen judgment を受けて、two-slot suite bridge を fixture-side full request contract compare へ進めるときの first actualization cut を、ad-hoc per-slot compare / dedicated contract subset carrier / full request node compare の 3 案で比較する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/100-current-l2-stage3-request-clause-suite-first-tranche-comparison.md`
- `specs/examples/101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md`
- `specs/examples/107-current-l2-stage3-suite-reopen-conditions.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.json`
- `crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json`
- `crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json`

## 3. Actions taken

1. current suite spike tests が already どこまで fixture-side compare をしているかを確認した。
2. `load_fixture_request_clause_fragment` など existing helper で individual clause fragment load が可能なことを確認した。
3. ad-hoc compare / dedicated contract subset carrier / full request node compare の 3 案を比較した。
4. current next cut を `Stage3RequestContractSubset` 相当の dedicated helper-local carrier に置く judgment を `specs/examples/108...` に記述した。

## 4. Files changed

- `specs/examples/108-current-l2-stage3-request-contract-subset-compare-cut.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0327-current-l2-stage3-request-contract-subset-compare-cut.md`

## 5. Commands run and exact outputs

```bash
sed -n '1,260p' crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs
sed -n '1,260p' crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs
sed -n '1,220p' crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.json
sed -n '1,220p' crates/mir-ast/tests/fixtures/current-l2/e10-perform-on-ensure-failure.json
sed -n '1,220p' crates/mir-ast/tests/fixtures/current-l2/e11-perform-via-ensure-then-success.json
rg -n "load_fixture_request_clause_fragment|request_clause_fragment|contract" crates/mir-ast/tests/support crates/mir-ast/tests -g '*.rs'
```

主要出力:

- `load_fixture_request_clause_fragment(...)` で fixture-side individual clause predicate load が already 可能
- current suite spike tests は `require` / `ensure` を slot ごとに ad-hoc compare している

## 6. Evidence / findings

- current suite spike tests は individual clause fragment compare までは already 可能だが、`contract subset compare` 自体は still implicit である。
- dedicated helper-local contract subset carrier を切れば、source-side two-slot carrier と fixture-side expected shape の接点を explicit にできる。
- full request node compare は request head parse pressure が強く、spec107 の reopen 条件から外れる。

## 7. Changes in understanding

- next narrow step は ad-hoc per-slot compare の維持ではなく、fixture-side `contract.require` / `contract.ensure` subset を dedicated helper-local carrier に切る line に置くのが自然だと確認した。
- compare carrier は `Stage3RequestContractSubset` のような 2 slot shape に留め、request head kind / op / target / chain_ref は still later に残す。

## 8. Open questions

- dedicated helper-local contract subset carrier を
  - `current_l2_stage3_predicate_fragment_spike_support.rs` に置くか
  - suite helper support 側に置くか。
- source-side compare helper まで切るか、それとも first tranche は tests から explicit compare で十分か。

## 9. Suggested next prompt

`specs/examples/108-current-l2-stage3-request-contract-subset-compare-cut.md` を前提に、`Stage3RequestContractSubset` 相当の helper-local / test-only first tranche を actualize し、fixed two-slot suite bridge と fixture-side contract subset compare の接点を source-backed にしてください。`
