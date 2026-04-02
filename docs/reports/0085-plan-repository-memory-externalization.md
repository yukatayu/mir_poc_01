# Report 0085 — plan repository memory externalization

- Date: 2026-04-02T15:01:33.787426Z
- Author / agent: Codex
- Scope: `plan/` 新設、`AGENTS.md` への plan maintenance 追記、`Documentation.md` と `specs/00-document-map.md` の導線更新、current repo status / roadmap / PoC stack / open questions の再編成
- Decision levels touched: current L2 の整理を mirror したが、意味論そのものは未変更。`plan/` は規範文書ではなく repository memory 層として追加

## 1. Objective

この repo に散っている current L2 semantics、parser-free PoC stack、helper layer 境界、representative fixtures、named profile catalog、roadmap、open problems、heavy future workstreams を、今後の PoC 検証ループで再読しやすい長期参照資料として `plan/` 以下へ構造化する。

あわせて、今後の task で `plan/` を継続運用するための maintenance rule を `AGENTS.md` に追加し、入口導線を `Documentation.md` と `specs/00-document-map.md` に反映する。

## 2. Inputs consulted

### 入口と基本文書

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/04-mir-core.md`
9. `specs/05-mirrorea-fabric.md`
10. `specs/06-prismcascade-positioning.md`
11. `specs/07-typed-effects-wiring-platform.md`
12. `specs/08-cross-system-relations.md`
13. `specs/09-invariants-and-constraints.md`
14. `specs/10-open-questions.md`
15. `specs/11-roadmap-and-workstreams.md`
16. `specs/12-decision-register.md`

### examples / companion 文書

17. `specs/examples/00-representative-mir-programs.md`
18. `specs/examples/01-current-l2-surface-syntax-candidates.md`
19. `specs/examples/02-current-l2-ast-fixture-schema.md`
20. `specs/examples/03-current-l2-evaluation-state-schema.md`
21. `specs/examples/04-current-l2-step-semantics.md`
22. `specs/examples/05-current-l2-oracle-api.md`
23. `specs/examples/06-current-l2-interpreter-skeleton.md`
24. `specs/examples/07-current-l2-host-stub-harness.md`
25. `specs/examples/08-current-l2-host-plan-schema.md`
26. `specs/examples/09-current-l2-bundle-loader.md`
27. `specs/examples/10-current-l2-batch-runner.md`
28. `specs/examples/11-current-l2-selection-helper.md`
29. `specs/examples/12-current-l2-selection-profiles.md`
30. `specs/examples/13-current-l2-profile-catalog.md`
31. `specs/examples/14-current-l2-profile-catalog-externalization.md`
32. `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`

### report chain

33. `docs/reports/0016-japanese-canonicalization-and-semantic-drift-check.md`
34. `docs/reports/0017-terminology-audit-and-cross-reference-alignment.md`
35. `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
36. `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
37. `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
38. `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
39. `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`
40. `docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md`
41. `docs/reports/0024-representative-mir-programs-current-l2.md`
42. `docs/reports/0025-perform-and-option-chain-surface-syntax-candidates.md`
43. `docs/reports/0026-try-fallback-surface-syntax-candidates.md`
44. `docs/reports/0028-clause-separator-and-block-nesting-surface-syntax-candidates.md`
45. `docs/reports/0029-contract-surface-policy-current-l2.md`
46. `docs/reports/0030-require-ensure-punctuation-and-multiline-current-l2.md`
47. `docs/reports/0032-predicate-sublanguage-current-l2.md`
48. `docs/reports/0034-option-local-declared-contract-surface-current-l2.md`
49. `docs/reports/0037-option-local-admit-runtime-admissibility-current-l2.md`
50. `docs/reports/0039-admit-vs-lease-trace-audit-current-l2.md`
51. `docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md`
52. `docs/reports/0045-capability-mismatch-non-admissible-taxonomy-current-l2.md`
53. `docs/reports/0046-review-0045-short-rereview.md`
54. `docs/reports/0047-current-l2-ast-fixture-schema-and-example-fixtures.md`
55. `docs/reports/0048-current-l2-evaluation-state-schema.md`
56. `docs/reports/0049-current-l2-step-semantics.md`
57. `docs/reports/0050-review-0049-short-rereview.md`
58. `docs/reports/0051-current-l2-oracle-api.md`
59. `docs/reports/0052-review-0051-short-rereview.md`
60. `docs/reports/0054-current-l2-parser-free-interpreter-skeleton.md`
61. `docs/reports/0055-review-0054-short-rereview.md`
62. `docs/reports/0056-current-l2-host-stub-harness.md`
63. `docs/reports/0058-review-0056-short-rereview.md`
64. `docs/reports/0059-current-l2-host-plan-sidecar-loader.md`
65. `docs/reports/0060-current-l2-bundle-loader.md`
66. `docs/reports/0061-review-0060-short-rereview.md`
67. `docs/reports/0062-current-l2-batch-runner.md`
68. `docs/reports/0063-current-l2-selection-helper.md`
69. `docs/reports/0064-current-l2-selection-profiles.md`
70. `docs/reports/0065-review-0064-short-rereview.md`
71. `docs/reports/0066-current-l2-profile-catalog.md`
72. `docs/reports/0067-current-l2-profile-catalog-externalization-comparison.md`
73. `docs/reports/0068-review-current-l2-profile-catalog-externalization-narrow-scope.md`
74. `docs/reports/0069-current-l2-profile-catalog-single-source-of-truth.md`
75. `docs/reports/0070-current-l2-profile-alias-mirror-boundary.md`
76. `docs/reports/0071-current-l2-profile-catalog-docs-tests-code-boundary.md`
77. `docs/reports/0072-review-helper-boundary-profile-catalog.md`
78. `docs/reports/0073-current-l2-named-profile-test-helper-boundary.md`
79. `docs/reports/0074-current-l2-profile-catalog-test-responsibility-boundary.md`
80. `docs/reports/0075-current-l2-profile-catalog-followup-review.md`
81. `docs/reports/0076-current-l2-profile-catalog-internal-integration-test-boundary.md`
82. `docs/reports/0077-current-l2-helper-layer-responsibility-alignment.md`
83. `docs/reports/0078-current-l2-fallback-lease-regression-fixtures.md`
84. `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md`
85. `docs/reports/0080-review-current-l2-fallback-reconciliation-and-compact-syntax.md`
86. `docs/reports/0081-current-l2-ladder-vs-edge-row-notation.md`
87. `docs/reports/0082-review-current-l2-fallback-compact-syntax-comparison.md`
88. `docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md`
89. `docs/reports/0084-review-current-l2-explicit-edge-row-notation-polishing.md`

### 実装 / fixture anchor

90. `crates/mir-ast/src/lib.rs`
91. `crates/mir-ast/tests/fixtures/current-l2/`
92. `crates/mir-semantics/src/lib.rs`
93. `crates/mir-semantics/src/harness.rs`
94. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. `AGENTS.md` の順序に従い、入口文書、基礎 specs、examples 文書、report chain、code anchor を再読した。
2. current repo の現在地を、次の 4 系統へ分解して棚卸しした。
   - current L2 settled semantics
   - current parser-free PoC reading
   - open questions / risks
   - heavy future workstreams
3. `plan/` を新設し、入口、現況、位置づけ、決定強度、core semantics、fallback / `lease`、surface notation、parser-free PoC stack、fixture catalog、helper stack、roadmap、risk、future workstream、glossary、source traceability、maintenance rule へ分割した。
4. fallback / preference chain については、outer-longer-lifetime wrapper 読みではなく guarded option chain 読み、left-to-right monotone degradation、no re-promotion、write-after-expiry try-later-or-`Reject`、rollback / `atomic_cut` non-interference を `plan/05` と `plan/06` へ明示した。
5. parser-free PoC stack については、AST fixture から named profile catalog までの call chain と public behavior / thin delegation を `plan/07` と `plan/09` に整理した。
6. representative fixtures については、`e1`〜`e8` を runtime/static-only の区別、static verdict、runtime outcome、trace / audit expectation の要点、host plan sidecar 有無とともに `plan/08` に整理した。
7. named profile catalog については、hard-coded table 維持、current aliases、externalization を future option に留めること、docs / tests / code boundary、internal / integration tests の責務分担を `plan/07` と `plan/09` に整理した。
8. AGENTS に `plan/` maintenance rule と review / close 運用を追記した。
9. `Documentation.md` と `specs/00-document-map.md` に `plan/` への導線と code anchor の最小説明を追加した。
10. `plan/90-source-traceability.md` を作り、各 plan 文書がどの spec / report / code anchor に依拠するかを source file / report 単位で追跡できるようにした。

## 4. Files changed

### 新規追加

- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/02-system-overview-and-positioning.md`
- `plan/03-decision-strengths-and-boundaries.md`
- `plan/04-core-semantics-current-l2.md`
- `plan/05-fallback-lease-and-chain-semantics.md`
- `plan/06-surface-notation-status.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `docs/reports/0085-plan-repository-memory-externalization.md`
- `docs/reports/0086-review-plan-memory-doc-boundary-consistency.md`

### 更新

- `AGENTS.md`
- `Documentation.md`
- `specs/00-document-map.md`

## 5. Commands run and exact outputs

```bash
git status --short --branch
```

```text
## main...origin/main [ahead 5]
?? plan/
?? "旧資料_参考_ChatGPT_01_69c5e3f6/"
```

```bash
python3 scripts/new_report.py --slug plan-repository-memory-externalization
```

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0085-plan-repository-memory-externalization.md
```

```bash
python3 scripts/validate_docs.py
```

```text
Documentation scaffold looks complete.
Found 86 numbered report(s).
```

```bash
git diff --check
```

```text
<no output>
```

```bash
cargo test -p mir-semantics
```

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.32s
running 2 tests in src/lib.rs ... ok
running 33 tests in tests/current_l2_minimal_interpreter.rs ... ok
running 0 doc-tests ... ok
test result: 35 passed; 0 failed
```

```bash
git diff --check HEAD~2 HEAD
```

```text
<no output>
```

```bash
git status --short --branch
```

```text
## main...origin/main [ahead 7]
?? "旧資料_参考_ChatGPT_01_69c5e3f6/"
```

## 6. Evidence / findings

### `plan/` をこの構成にした理由

- `plan/00`〜`plan/03` で、入口、現況、位置づけ、decision strength を先に揃えた。
- `plan/04`〜`plan/09` で、current L2 semantics、fallback / `lease`、surface notation、PoC stack、fixtures、helper responsibility を current repo の中心として整理した。
- `plan/10`〜`plan/14` で、roadmap、near-term tasks、risk register、heavy future workstreams、glossary を分けた。
- `plan/90` / `plan/91` で、source traceability と maintenance rule を独立させた。

この分割により、将来 task で「意味論を見たい」「PoC stack を見たい」「近い task を決めたい」「未決事項を見たい」をファイル単位で切り分けられる。

### current L2 settled / PoC current reading / open / future workstream の分け方

- current L2 settled:
  - fallback = guarded option chain
  - left-to-right monotone degradation
  - no re-promotion
  - write-after-expiry try-later-or-`Reject`
  - rollback / `atomic_cut` non-interference
  - explicit edge-row form を暫定 companion notation に維持
- current parser-free PoC reading:
  - AST fixture
  - evaluation state
  - step semantics
  - oracle API
  - minimal interpreter
  - host harness
  - host plan sidecar
  - bundle / batch / selection / profile / named profile helper stack
- OPEN:
  - final parser grammar
  - machine-readable catalog / manifest 採用
  - path canonicalization
  - detached trace / audit serialization
  - richer host interface
  - multi-request scheduler
  - `Approximate` / `Compensate`
- FUTURE WORKSTREAM:
  - 型システムの強さ
  - 静的解析
  - 定理証明可能性
  - 決定可能性
  - 実装可能性 / complexity
  - external verifier / theorem prover への分離

### source を plan へどう割り当てたか

- 基礎境界と decision strength:
  - `specs/01`〜`specs/12`
- fallback / `lease` / chain:
  - `0018`〜`0023`、`0037`、`0039`、`0043`、`0045`、`0078`〜`0084`
- parser-free PoC stack:
  - `specs/examples/02`〜`13`
  - `0047`〜`0077`
  - `crates/mir-semantics/src/harness.rs`
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- representative fixtures:
  - `specs/examples/00`、`02`、`04`
  - `crates/mir-ast/tests/fixtures/current-l2/`
- notation:
  - `specs/examples/01`
  - `specs/examples/15`
  - `0079`〜`0084`

詳細 trace は `plan/90-source-traceability.md` に置いた。

### `AGENTS.md` に追加したこと

- `plan/` を人間向け repository memory として扱う rule
- semantics / examples / fixtures / helper stack / roadmap / syntax candidate / current status が変わる task で relevant `plan/` を同 task で更新する rule
- 更新不要なら report に `plan/ 更新不要` と書く rule
- `plan/` を docs mirror と同等の一級成果物として扱う rule
- review は最後に 1 回長めに待つのを基本にし、必要なら retry 1 回、その後は local evidence を report に残す rule

### current repository status の理解

- repo の主眼は Mir semantics である。
- current L2 fallback reading は guarded option chain であり、outer-longer-lifetime wrapper 読みではない。
- parser-free PoC stack は named profile catalog まで揃っている。
- named profile catalog は hard-coded table が最小で、externalization は comparison 止まりである。
- helper stack の public behavior / thin delegation、docs / tests / code boundary はかなり狭く揃ってきている。
- 今後の大きな workstream は、parser / richer host / static analysis / type / theorem prover / complexity 側へ分岐する。

### local evidence

- `python3 scripts/validate_docs.py` は成功した。
- `git diff --check` と `git diff --check HEAD~2 HEAD` は無出力だった。
- `cargo test -p mir-semantics` は unit 2 件、integration 33 件、doc-tests 0 件で成功した。
- reviewer は final 1 回 + retry 1 回の運用で completion し、指摘は 3 件だった。
  - `plan/91` の読書順が AGENTS の mandatory read order を狭めていた点
  - `plan/09` が `run_directory_named_profile` と `ProfileCatalog` の責務境界を少し曖昧にしていた点
  - report evidence trail が弱かった点
- 上記 3 点はすべて今回の task 内で修正した。
- reviewer evidence は `docs/reports/0086-review-plan-memory-doc-boundary-consistency.md` に残した。

### task-start dirty state

task 開始時点で次の pre-existing dirty state があった。

- `?? 旧資料_参考_ChatGPT_01_69c5e3f6/`

今回の作業ではこの directory には触れていない。

### spec 本文 commit hash

- spec / plan / docs 本体の commit hash:
  - `8003e26` `plan ディレクトリと維持ルールを追加する`
- report / review 追加の commit hash:
  - `6a8044d` `plan 外在化の作業報告を追加する`
- report 自身の commit hash は self-reference の都合で本文に固定できない場合がある。その場合は本文でその旨を明記する。

## 7. Changes in understanding

- この repo は、意味論・helper stack・report chain がかなり進んでおり、問題は「情報が無いこと」より「散っていて task ごとに再構成コストが高いこと」にあると整理できた。
- current L2 fallback reading は、semantics 側はかなり安定しており、残る tension は outer/inner 直感と notation drift 側にある。
- parser-free PoC stack は単発 helper の寄せ集めではなく、`run_directory_named_profile -> run_directory_profiled -> select_bundles_from_request -> batch_summary_from_discovery -> run_bundle` という call chain を持つ検証基盤として読む方が適切だった。
- named profile catalog は small hard-coded alias layer に留めているからこそ、docs / tests / code boundary を narrow に保てている。
- 今後の重い workstream は「今すぐやらない」のではなく、entry criteria 付きで plan に明示して scope creep を防ぐ方がよい。

## 8. Open questions

- `plan/` の粒度を今後もこの分割で維持するか、それとも phase change ごとに統廃合するか。
- parser 導入前に、syntax decision inventory を `plan/` のどこへ常設するか。
- heavier workstream に入る前に、parser-free PoC stack からどの程度の formal IR を切り出す必要があるか。
- review infrastructure の遅延を前提に、report 内 evidence の標準粒度をさらに明文化する必要があるか。

## 9. Suggested next prompt

`plan/` を正本の repository memory として使いながら、次は parser 導入前の boundary inventory を narrow scope で作ってください。特に、current companion notation のどこまでが parser 決定前に安定しており、どこからが未決のまま残すべきかを、`plan/06` / `plan/11` / `plan/12` を更新しつつ整理してください。
