# Report 0056 — current L2 host stub harness

- Date: 2026-04-01
- Author / agent: Codex
- Scope: current L2 parser-free minimal interpreter 向け host stub layer / fixture runner harness の一般化
- Decision levels touched: L2

## 1. Objective

`PredicateOracle` / `EffectOracle` の test-only stub を ad hoc match 文のまま増やさず、fixture ごとに predicate verdict、effect outcome、success-side carrier、trace expectation を declarative に差し替えられる current L2 向け最小 harness を `crates/mir-semantics` に追加する。

## 2. Scope and assumptions

- Mir-0 / Mir-1 / Mirrorea の境界は変更しない。
- canonical normalization law、rejection phase、static evidence floor、underdeclared handling、lineage annotation surface form、`perform` / option chain / `try` / `fallback` / `contract` policy / predicate sublanguage / option-local `admit` runtime reading / trace-audit reading / capability mismatch taxonomy / AST fixture schema / evaluation state schema / step semantics / oracle API / parser-free minimal interpreter skeleton は既存判断を維持する。
- host stub / harness は current L2 semantics の**検証基盤**に留め、production host interface にはしない。
- `must_explain` は machine-check へ上げず、human-facing explanation obligation に残す。
- task 開始時点の worktree に既存 dirty state はなく、`git status --short --branch` は `## main...origin/main` のみだった。
- 前段として `0054` short re-review を実施し、place-scoped declaration resolution と duplicate declaration rejection を最小修正した。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/03-current-l2-evaluation-state-schema.md`
- `specs/examples/04-current-l2-step-semantics.md`
- `specs/examples/05-current-l2-oracle-api.md`
- `specs/examples/06-current-l2-interpreter-skeleton.md`
- `docs/reports/0050-review-0049-short-rereview.md`
- `docs/reports/0051-current-l2-oracle-api.md`
- `docs/reports/0052-review-0051-short-rereview.md`
- `docs/reports/0053-final-review-current-l2-interpreter-skeleton.md`
- `docs/reports/0054-current-l2-parser-free-interpreter-skeleton.md`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/fixtures/current-l2/`
- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 4. Actions taken

1. `0054` に対する short re-review を実施し、global `HashMap<String, ...>` による declaration 解決が `current place` visibility と no-silent-shadowing invariant を壊している finding を確認した。
2. `crates/mir-semantics/src/lib.rs` の declaration indexing を place-scoped に変更し、`PerformVia` は current `place` から見える `ChainDecl` / `OptionDecl` を ancestor-search で解決するようにした。
3. 同時に static gate を tightening し、visible scope chain 上の duplicate `OptionDecl` / `ChainDecl` は `malformed` として rejection するようにした。
4. `crates/mir-semantics/src/harness.rs` を新設し、次を持つ declarative host harness を追加した。
   - `PredicatePlanRule`
   - `EffectPlanRule`
   - `EffectPlanVerdict`
   - `FixtureCommitPlan`
   - `FixtureStoreMutation`
   - `TraceExpectationOverride`
   - `FixtureHostPlan`
   - `FixtureHostStub`
5. harness では、
   - predicate verdict
   - effect outcome
   - success-side carrier / tentative post-state
   - formal な non-admissible metadata expectation
   - exact compare する narrative explanation expectation
   を fixture ごとに declarative に差し替えられるようにした。
6. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` を harness 前提に書き換え、テスト責務を次に分けた。
   - static gate
   - runtime outcome
   - trace / audit expectation
   - trace expectation override の解決
7. E1 / E2 / E3 variant / E6 は harness 経由で実行し、E4 / E5 は static gate で止まることを維持した。
8. 新規補助文書 `specs/examples/07-current-l2-host-stub-harness.md` を追加し、machine-check と narrative explanation の境界、host plan の最小 shape、current L2 に入れないものを整理した。
9. `Documentation.md` と `specs/00-document-map.md` に 07 文書への導線を追加した。
10. code/docs/report の本体を commit `b184965` `current L2 の host stub harness を一般化する` として記録した。
11. final reviewer は依頼したが completion が返らず、local verification を主証跡にした。

## 5. Files changed

仕様本文 / 実装本文 commit `b184965` に含めた変更:

- 更新: `Documentation.md`
- 更新: `crates/mir-semantics/src/lib.rs`
- 新規: `crates/mir-semantics/src/harness.rs`
- 更新: `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- 新規: `docs/reports/0055-review-0054-short-rereview.md`
- 更新: `specs/00-document-map.md`
- 新規: `specs/examples/07-current-l2-host-stub-harness.md`

report commit に含める変更:

- 新規: `docs/reports/0056-current-l2-host-stub-harness.md`

## 6. Commands run and exact outputs

task 開始時点:

```bash
git status --short --branch
```

```text
## main...origin/main
```

RED after test rewrite:

```bash
cargo test -p mir-semantics
```

```text
error: lifetime may not live long enough
...
error: could not compile `mir-semantics` (lib) due to 1 previous error
```

GREEN after harness + place-scoped declaration fix:

```bash
cargo check -p mir-semantics
```

```text
Checking mir-semantics v0.1.0 (/home/yukatayu/dev/mir_poc_01/crates/mir-semantics)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.13s
```

```bash
cargo test -p mir-semantics
```

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 2.08s
running 4 tests
test harness_can_override_trace_expectation_without_changing_runtime_plan ... ok
test static_gate_rejects_malformed_and_underdeclared_fixtures ... ok
test runtime_fixtures_reach_expected_outcomes_via_declarative_host_plan ... ok
test trace_and_audit_expectations_follow_fixture_or_harness_override ... ok
test result: ok. 4 passed; 0 failed
```

```bash
git diff --check
```

```text

```

```bash
python3 scripts/validate_docs.py
```

```text
Documentation scaffold looks complete.
Found 57 numbered report(s).
```

commit:

```text
b184965 current L2 の host stub harness を一般化する
```

review / commit note:

```text
final reviewer completion は返らず、close 前の status も取得できなかったため、本 report では未返却として扱う。
```

## 7. Evidence / findings

- host stub / harness を declarative plan に切り出したことで、predicate verdict と effect outcome を fixture ごとに差し替えながら、interpreter 側の structural semantics をそのまま保てるようになった。
- `admit-miss` / `lease-expired` / capability mismatch narrative は、host plan が直接生成するのではなく interpreter 側で生じる。harness はそれらを machine-check する expectation override だけを持ち、runtime semantics と explanation layer を混ぜていない。
- `must_explain` は引き続き human-facing obligation に残り、tests は exact compare しない。これにより current L2 の event / metadata / narrative 分離と矛盾しない。
- E6 は empty host plan でも `lease-expired` metadata と final `Reject` を出せるため、non-admissible structural rule が oracle / host layer に漏れていないことを確認できた。
- `PerformVia` の declaration 解決を place-scoped に直したことで、`specs/examples/04-current-l2-step-semantics.md` と `specs/examples/06-current-l2-interpreter-skeleton.md` の current-place reading と実装が揃った。
- duplicate declaration を static gate で止めるようにしたため、current L2 interpreter skeleton に silent last-write-wins が残らなくなった。
- 仕様本文 / 実装本文 commit hash は `b184965` である。report 自身の commit hash は self-reference の都合で本文に固定しない。

## 8. Changes in understanding

- oracle boundary を決めたあとに harness を切ると、runtime semantics を変えずに「どこまで host が差し替え、どこから interpreter が持つか」を非常に小さく整理できることが明確になった。
- place-scoped declaration resolution は current fixture set では目立たないが、parser-free skeleton を今後一般化する前に直しておかないと current L2 の current-place reading とずれ始める。
- narrative explanation を machine-check から完全に外すのではなく、短い `narrative_explanations` だけ optional override にして `must_explain` を prose obligation に残す、という二段構えが current L2 には最も扱いやすい。

## 9. Open questions

- richer host interface を current L2 harness でどこまで許すか。
- detached trace / audit serialization を 언제 harness から独立 carrier に切り出すか。
- multi-request scheduler を導入する段階で harness API をどう分離するか。
- `Approximate` / `Compensate` を harness / oracle carrier に入れる必要があるか。
- parser 実装前に、fixture 側へ host plan を併置するか、Rust 側 plan builder に留めるかは **未決定**。

## 10. Suggested next prompt

`crates/mir-semantics` の current L2 host stub harness を前提に、fixture JSON と別ファイルの machine-readable host plan schema を試作してください。parser には進まず、E1 / E2 / E3 variant / E6 の Rust 側 plan builder を JSON plan 読み込みへ置き換えられる最小 shape と loader に絞ってください。
