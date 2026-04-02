# Report 0078 — current L2 fallback / lease regression fixtures

## Objective

current L2 parser-free PoC 基盤を前提に、fallback / `lease` / monotone degradation の current L2 reading が drift していないことを、spec prose だけでなく machine-check 可能な regression fixture でも確認できるようにする。

## Scope and assumptions

- current L2 semantics 自体は変更しない。
- 新しい failure class、event surface、machine-readable catalog asset、manifest は導入しない。
- 今回は runtime fixture 2 本の追加と、それに伴う bundle / batch / selection / profile / named-profile 既存 tests の追随、ならびに最小限の prose mirror 更新に限る。
- `must_explain` は machine-check に上げず、human-facing explanation obligation のまま残す。
- 作業開始時点の worktree は clean だった。

## Documents consulted

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/04-mir-core.md`
8. `specs/09-invariants-and-constraints.md`
9. `specs/10-open-questions.md`
10. `specs/11-roadmap-and-workstreams.md`
11. `specs/12-decision-register.md`
12. `specs/examples/00-representative-mir-programs.md`
13. `specs/examples/01-current-l2-surface-syntax-candidates.md`
14. `specs/examples/02-current-l2-ast-fixture-schema.md`
15. `specs/examples/03-current-l2-evaluation-state-schema.md`
16. `specs/examples/04-current-l2-step-semantics.md`
17. `specs/examples/05-current-l2-oracle-api.md`
18. `specs/examples/06-current-l2-interpreter-skeleton.md`
19. `specs/examples/07-current-l2-host-stub-harness.md`
20. `specs/examples/08-current-l2-host-plan-schema.md`
21. `specs/examples/09-current-l2-bundle-loader.md`
22. `specs/examples/10-current-l2-batch-runner.md`
23. `specs/examples/11-current-l2-selection-helper.md`
24. `specs/examples/12-current-l2-selection-profiles.md`
25. `specs/examples/13-current-l2-profile-catalog.md`
26. `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
27. `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
28. `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
29. `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
30. `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`
31. `docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md`
32. `docs/reports/0037-option-local-admit-runtime-admissibility-current-l2.md`
33. `docs/reports/0039-admit-vs-lease-trace-audit-current-l2.md`
34. `docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md`
35. `docs/reports/0045-capability-mismatch-non-admissible-taxonomy-current-l2.md`
36. `docs/reports/0046-review-0045-short-rereview.md`
37. `docs/reports/0071-current-l2-profile-catalog-docs-tests-code-boundary.md`
38. `docs/reports/0073-current-l2-named-profile-test-helper-boundary.md`
39. `docs/reports/0076-current-l2-profile-catalog-internal-integration-test-boundary.md`
40. `docs/reports/0077-current-l2-helper-layer-responsibility-alignment.md`
41. `crates/mir-ast/tests/fixtures/current-l2/`
42. `crates/mir-semantics/src/lib.rs`
43. `crates/mir-semantics/src/harness.rs`
44. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## Actions taken

1. 既存 coverage を棚卸しし、runtime では E3 が `admit-miss` から later writer success、E6 が `lease-expired` から final `Reject` を持つ一方で、`lease-expired` から later writer success するケースと、`admit-miss` + explicit failure + final `Reject` を同じ chain で持つケースが fixture 不足だと確認した。
2. 先に `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` の runtime fixture arrays、runtime bundle counts、runtime-only selection / profile summary counts を 6 runtime bundles / 8 total bundles 前提へ更新し、fixture 不足で test が落ちる状態を作った。
3. runtime regression fixture を 2 本追加した。
   - `e7-write-fallback-after-expiry`
     - earlier writer が `lease-expired` でも、same-lineage の later write-capable option に degrade して success する。
   - `e8-monotone-degradation-reject`
     - `admit-miss`、middle option の explicit failure、final `Reject` を left-to-right degradation として読む。
4. 両 fixture に対応する `.host-plan.json` sidecar を追加し、current interpreter の structural rule と minimal host harness で実行できるようにした。
5. integration tests に focused regression test `fallback_and_lease_regression_fixtures_preserve_current_l2_boundaries` を追加し、今回の 2 fixture が要求する exact compare をまとめて固定した。
6. `specs/examples/02-current-l2-ast-fixture-schema.md` の fixture set を 8 本へ更新し、`specs/examples/06-current-l2-interpreter-skeleton.md` に E6 補完 regression fixtures の coverage を追記した。
7. narrow review は local diff inspection で行い、次だけを再確認した。
   - `e7` / `e8` が既存 current L2 reading を越えていないこと
   - `e8` の「earlier option への再昇格禁止」が stateful cross-request semantics を既成事実化していないこと

## Files changed

- `crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json`
- `crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.host-plan.json`
- `crates/mir-ast/tests/fixtures/current-l2/e8-monotone-degradation-reject.json`
- `crates/mir-ast/tests/fixtures/current-l2/e8-monotone-degradation-reject.host-plan.json`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/06-current-l2-interpreter-skeleton.md`

## Commands run

```bash
git status --short --branch
sed -n '1,260p' crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
sed -n '260,620p' crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
sed -n '620,980p' crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
sed -n '740,930p' crates/mir-semantics/src/lib.rs
rg -n "6 fixture|six-fixture|6 本|current six|現在の6|runtime fixture|E6|e6-write-after-expiry|e3-option-admit-chain" specs/examples/02-current-l2-ast-fixture-schema.md specs/examples/06-current-l2-interpreter-skeleton.md specs/examples/07-current-l2-host-stub-harness.md specs/examples/00-representative-mir-programs.md specs/examples/01-current-l2-surface-syntax-candidates.md Documentation.md specs/00-document-map.md
cargo test -p mir-semantics runtime_fixtures_reach_expected_outcomes_via_declarative_host_plan discovery_finds_fixture_bundles_and_classifies_runtime_vs_static_only selection_runtime_only_keeps_only_runtime_bundles -- --nocapture
cargo test -p mir-semantics -- --nocapture
cargo test -p mir-semantics fallback_and_lease_regression_fixtures_preserve_current_l2_boundaries -- --nocapture
cargo test -p mir-semantics
python3 scripts/validate_docs.py
git diff --check
git add crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.host-plan.json crates/mir-ast/tests/fixtures/current-l2/e8-monotone-degradation-reject.json crates/mir-ast/tests/fixtures/current-l2/e8-monotone-degradation-reject.host-plan.json crates/mir-semantics/tests/current_l2_minimal_interpreter.rs specs/examples/02-current-l2-ast-fixture-schema.md specs/examples/06-current-l2-interpreter-skeleton.md
git commit --no-gpg-sign -m "fallback と lease の regression fixture を追加する"
```

## Evidence / outputs / test results

- 初回の `cargo test -p mir-semantics runtime_fixtures_reach_expected_outcomes_via_declarative_host_plan discovery_finds_fixture_bundles_and_classifies_runtime_vs_static_only selection_runtime_only_keeps_only_runtime_bundles -- --nocapture`
  - `cargo test` は test 名を 1 つしか直接受け取れず、`unexpected argument` で失敗した。
  - これは fixture 不足ではなく command invocation 側の誤りであり、直後に package 全体実行へ切り替えた。
- failing-first の `cargo test -p mir-semantics -- --nocapture`
  - 7 tests failed
  - failure 内容は 2 種類に収束した。
    - 新 fixture path が未作成で `No such file or directory`
    - runtime bundle count / selected count が `4 -> 6`, `6 -> 8` へ未追随
- fixture 追加後の `cargo test -p mir-semantics fallback_and_lease_regression_fixtures_preserve_current_l2_boundaries -- --nocapture`
  - integration test 1 件 pass
- 最終 `cargo test -p mir-semantics`
  - unit tests 2 件 pass
  - integration tests 33 件 pass
  - doc tests 0 件
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 77 numbered report(s).`
- `git diff --check`
  - 無出力
- narrow local review
  - no findings
  - `e7` は E6 の「write-after-expiry でも later write-capable option があれば explicit fallback できる」を fixture 化しただけで、new semantics はない
  - `e8` は intra-request の left-to-right scan を固定しており、cross-request state や persisted degradation state を既成事実化していない

## What changed in understanding

- current L2 の fallback / `lease` drift は、E3 と E6 だけだと「success 側の write fallback after expiry」と「admit miss / explicit failure / final Reject を同じ chain で並べる」部分が machine-check から抜けやすい。
- `lease-expired` は structural skip であり、later write-capable option が存在すれば host plan はその option だけを oracle coverage すれば十分である。
- monotone degradation と earlier option への再昇格禁止は、current interpreter では persisted state ではなく single request 内の left-to-right scan discipline として machine-check するのが current L2 の最小である。
- capability mismatch は引き続き narrative explanation に留めたままで十分であり、今回の regression fixture でも formal subreason へ上げる必要はなかった。

## Open questions

- final fallback surface syntax は依然として **未決定** である。今回固定したのは AST fixture と host plan sidecar による machine-readable regression だけである。
- machine-readable catalog asset / manifest、selector grammar / alias grammar の長期固定、path canonicalization policy は引き続き **未決定** である。
- detached trace / audit serialization、richer host interface、multi-request scheduler、`Approximate` / `Compensate` は今回も扱っておらず、引き続き **未決定** である。
- earlier option への再昇格禁止を cross-request / post-rollback / post-cut まで含む persisted runtime state に上げるかどうかは current L2 では **未決定** である。

## Commits

- fixture / test / minimal spec mirror: `dc1b4a4` `fallback と lease の regression fixture を追加する`
- この report 自身の commit hash は self-reference の都合で本文に固定しない。

## Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、fallback / lease regression fixture 群を selection/profile catalog から targeted に呼び出せる小さな named smoke profile を追加すべきか、それとも current runtime-only / single-fixture helper だけで十分かを比較整理してください。`
