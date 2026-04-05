# Report 0165 — review static reason codes authoring assist

- Date: 2026-04-05T14:05:00Z
- Author / agent: Codex
- Scope: `static reason codes authoring assist` task の current working tree changes を review し、display-only / helper-local 境界、fixture schema 非昇格、docs/plan/progress/report mirror 整合、loop wrapper の非-core 性を確認する
- Decision levels touched: L2

## 1. Objective

`reason_codes` assist が current L2 boundary を壊していないかを、code / tests / docs / plan / progress / report の working tree change 全体で確認する。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/32-current-l2-static-gate-artifact-loop.md`
- `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
- `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
- `specs/examples/35-current-l2-detached-static-reason-code-mirror.md`
- `specs/examples/36-current-l2-checked-reasons-authoring-assist.md`
- `specs/examples/37-current-l2-detached-bundle-transform-helper.md`
- `specs/examples/38-current-l2-static-reason-codes-authoring-assist.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `docs/reports/0163-static-reason-codes-authoring-assist.md`
- `scripts/current_l2_reason_codes_assist.py`
- `scripts/current_l2_checked_reasons_assist.py`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_reason_codes_assist.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`

## 3. Actions taken

1. 必読順に repo baseline と current L2 boundary 文書を再読した。
2. working tree diff と untracked files を確認し、task 対象の code / docs / plan / progress / report を読み合わせた。
3. Python tests と real smoke を実行し、stable cluster と duplicate cluster の display-only behavior を確認した。
4. synthetic fixture に fixture-side `reason_codes` field を仮挿入した再現ケースを作り、assist が境界逸脱を検知するかを確認した。

## 4. Files changed

- 追加:
  - `docs/reports/0165-review-static-reason-codes-authoring-assist.md`
- `plan/` 更新不要
- `progress.md` 更新不要

## 5. Commands run and exact outputs

- `python3 -m unittest scripts.tests.test_current_l2_reason_codes_assist scripts.tests.test_current_l2_static_gate_loop`
  - `Ran 7 tests in 0.007s`
  - `OK`
- `python3 scripts/current_l2_detached_loop.py suggest-reason-codes crates/mir-ast/tests/fixtures/current-l2/e19-malformed-target-mismatch.json --run-label review-e19 --overwrite`
  - `reason_codes_scope: stable-clusters-only`
  - suggestion row に `declared_target_mismatch` を表示
- `python3 scripts/current_l2_detached_loop.py suggest-reason-codes crates/mir-ast/tests/fixtures/current-l2/e14-malformed-duplicate-option-declaration.json --run-label review-e14 --overwrite`
  - `detached artifact has no reason_codes suggestion`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 164 numbered report(s).`
- synthetic repro:
  - fixture 側 `expected_static.reason_codes` を仮挿入した一時 JSON と detached artifact を作成し、
    `python3 scripts/current_l2_reason_codes_assist.py <fixture> <artifact>` を実行
  - 出力は `current fixture-side typed reason code carrier: absent (no current fixture field)` のままで、fixture 側 field 挿入を検知しなかった

## 6. Evidence / findings

- Medium — `scripts/current_l2_reason_codes_assist.py:57-75` never reads the fixture document, but still prints that the fixture-side typed reason-code carrier is absent. A synthetic repro with `expected_static.reason_codes` already present still returns `0` and prints `absent`. This means the new assist documents the L2 boundary but does not enforce it, so an accidental typed-field addition can slip through silently instead of being surfaced as a boundary violation. That directly weakens the “fixture schema must not silently gain a typed field” requirement. The tests in `scripts/tests/test_current_l2_reason_codes_assist.py:65-159` only cover normal suggestion/no-suggestion paths, so this negative case is currently unguarded.

## 7. Changes in understanding

- docs / plan / progress / report mirrors are otherwise aligned: all of them consistently describe the new assist as display-only, source-limited to `detached_noncore.reason_codes`, and non-promoting with respect to machine-check core.
- `scripts/current_l2_detached_loop.py` keeps the new path wrapper-local and does not feed `reason_codes` into compare helpers or bundle machine-check.
- the main residual risk is not silent promotion by the wrapper, but silent non-detection if fixture-side typed carrier creep happens later.

## 8. Open questions

- should the assist hard-fail if it detects any fixture-side typed reason-code field, or merely warn and return non-zero?
- which exact fixture-side path should be treated as forbidden while the schema remains `verdict` / `reasons` / optional `checked_reasons` only?

## 9. Suggested next prompt

Harden `scripts/current_l2_reason_codes_assist.py` so it actually inspects the fixture JSON and fails closed if any fixture-side typed reason-code carrier is present, then add a focused negative test proving that accidental schema promotion is rejected.
