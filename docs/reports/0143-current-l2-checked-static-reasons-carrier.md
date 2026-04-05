# Report 0143 — current L2 checked static reasons carrier

- Date: 2026-04-05T10:40:00Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC の `expected_static.reasons` dual-use を future checker API でどう分離するのが最小かを比較し、additive optional `checked_reasons` carrier を narrow に導入する
- Decision levels touched: L2

## 1. Objective

current L2 の first checker cut と static gate artifact loop を前提に、

- `expected_static.reasons` を explanation に残したまま
- actual static gate compare を bundle-level machine-check へ narrow transfer する最小 carrier

を source-backed に比較し、実装可能なら additive に導入する。

## 2. Scope and assumptions

- current L2 core semantics、runtime semantics、failure family は変更しない
- detached static gate artifact loop は維持する
- typed reason code taxonomy はまだ固定しない
- current fixture corpus は `expected_static.reasons` を explanatory note として保持し続けてよい

## 3. Documents consulted

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. `specs/examples/02-current-l2-ast-fixture-schema.md`
9. `specs/examples/07-current-l2-host-stub-harness.md`
10. `specs/examples/27-current-l2-fixture-scaffold-helper.md`
11. `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
12. `specs/examples/32-current-l2-static-gate-artifact-loop.md`
13. `plan/07-parser-free-poc-stack.md`
14. `plan/11-roadmap-near-term.md`
15. `plan/12-open-problems-and-risks.md`
16. `plan/15-current-l2-fixture-authoring-template.md`
17. `plan/90-source-traceability.md`
18. `progress.md`
19. `docs/reports/0141-current-l2-static-gate-reasons-machine-check.md`
20. `docs/reports/0142-review-current-l2-static-gate-reasons-machine-check.md`
21. `crates/mir-semantics/src/lib.rs`
22. `crates/mir-semantics/src/harness.rs`
23. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
24. `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
25. `crates/mir-ast/tests/fixtures/current-l2/`

## 4. Actions taken

1. fixture corpus の `expected_static.reasons` を棚卸しし、valid fixture では explanatory prose、invalid fixture でも user-facing wording が混在していることを再確認した。
2. `static_gate_detailed().reasons` と detached static gate artifact の `checker_core.reasons` が actual compare source であることを確認した。
3. 3 案を比較した。
   - additive optional `checked_reasons`
   - typed `reason_codes`
   - detached-only 維持
4. 最小案として `checked_reasons` を採り、RED test を先に追加した。
5. `ExpectedStatic` に optional `checked_reasons` を追加し、`run_bundle()` が field present のときだけ actual static gate reasons を fail-closed compare する最小実装を入れた。
6. schema / harness / first checker cut / fixture authoring template / roadmap / progress を更新した。
7. local focused diff review を行い、`expected_static.reasons` を machine-check へ誤昇格させていないことを確認した。

## 5. Files changed

- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/07-current-l2-host-stub-harness.md`
- `specs/examples/27-current-l2-fixture-scaffold-helper.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/32-current-l2-static-gate-artifact-loop.md`
- `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0143-current-l2-checked-static-reasons-carrier.md`

## 6. Commands run

```text
cargo test -p mir-semantics --test current_l2_minimal_interpreter run_bundle_rejects_checked_static_reason_mismatch -- --nocapture
error[E0609]: no field `checked_reasons` on type `ExpectedStatic`
```

```text
cargo test -p mir-semantics --test current_l2_minimal_interpreter run_bundle_rejects_checked_static_reason_mismatch -- --nocapture
... ok
```

```text
cargo test -p mir-semantics --test current_l2_minimal_interpreter run_bundle_allows_explanatory_static_reasons_without_checked_carrier -- --nocapture
... ok
```

```text
cargo test -p mir-semantics --test current_l2_minimal_interpreter run_bundle_accepts_matching_checked_static_reasons -- --nocapture
... ok
```

```text
cargo run -p mir-semantics --example current_l2_emit_static_gate -- crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json
checker_core.reasons = []
```

```text
cargo run -p mir-semantics --example current_l2_emit_static_gate -- crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json
checker_core.reasons = ["missing lineage assertion for primary -> mirror"]
```

## 7. Evidence / outputs / test results

- `checked_reasons` を追加する前の RED は compile failure であり、field が schema / code に存在しないことを正しく示した。
- 実装後の targeted tests により、
  - mismatch は fail-closed で落ちる
  - explanatory `reasons` だけを変えても、`checked_reasons` absent なら `run_bundle()` は落ちない
  - actual static gate reasons と一致する `checked_reasons` は通る
  ことを確認した。
- actual static gate source は引き続き `static_gate_detailed().reasons` と detached static gate artifact `checker_core.reasons` である。
- valid fixture `e3` の actual static gate reasons は空であり、fixture 側 explanation と machine-check source が一致しない current corpus を再確認できた。
- underdeclared fixture `e5` の actual static gate reasons は user-facing prose ではなく machine-oriented wording であることも再確認できた。

## 8. What changed in understanding

- `checked_reasons` は `expected_static.reasons` の replacement ではなく、future checker API への **additive optional bridge carrier** として扱うのが最小である。
- detached static gate artifact は引き続き actual source of truth であり、fixture 側の `checked_reasons` はその bridge に留めるのが自然である。
- typed `reason_codes` は有望だが、current L2 では taxonomy を先取りしすぎるため後段に残す方がよい。

## 9. Open questions

- `checked_reasons` を long-term に維持するか、typed `reason_codes` へ移行するか。
- detached static gate artifact に将来 `reason_codes` mirror を持たせる必要があるか。
- malformed / underdeclared を超える richer static reason taxonomy をどの layer で定義するか。

## 10. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、optional checked_reasons carrier を typed reason code へ移行する条件を source-backed に比較し、string carrier を維持する範囲と code 化してよい static gate reason cluster を narrow に棚卸ししてください。`
