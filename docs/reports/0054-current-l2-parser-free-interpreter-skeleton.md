# Report 0054 — current L2 parser-free interpreter skeleton

- Date: 2026-04-01
- Author / agent: Codex
- Scope: current L2 の representative examples・AST fixture schema・evaluation state schema・step semantics・oracle API を前提にした parser-free 最小 interpreter skeleton の実装
- Decision levels touched: L2

## 1. Objective

current L2 fixture を parser なしで直接読み、E1 / E2 / E3 variant / E6 を最小限実行できる parser-free minimal interpreter skeleton を `crates/mir-semantics` に実装する。
あわせて E4 / E5 を static gate で停止させ、existing fixture の `expected_static` / `expected_runtime` / `expected_trace_audit` と矛盾しないことを確認する。

## 2. Scope and assumptions

- Mir-0 / Mir-1 / Mirrorea の境界は変更しない。
- canonical normalization law、rejection phase、static evidence floor、underdeclared handling、lineage annotation surface form、`perform` と option chain の companion notation、`try` / `fallback` block form、`contract` semantic-role-only policy、predicate sublanguage、option-local `admit` の runtime reading、trace / audit reading、capability mismatch taxonomy、AST fixture schema、evaluation state schema、step semantics、oracle API は既存判断を維持する。
- parser 文字列 syntax は固定せず、fixture schema に正規化された意味構造だけを実行対象にする。
- current L2 では `OptionDecl` / `ChainDecl` を declaration-only no-op とし、full scheduler、detached trace serialization、rich host interface、`Approximate` / `Compensate` は実装しない。
- task 開始時点の worktree に既存 dirty state はなく、`git status --short --branch` は `## main...origin/main [ahead 9]` のみだった。
- 前段として `0051` short re-review を行い、via-chain `Reject` boundary の tightening を mirror に反映した。

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
- `docs/reports/0046-review-0045-short-rereview.md`
- `docs/reports/0047-current-l2-ast-fixture-schema-and-example-fixtures.md`
- `docs/reports/0048-current-l2-evaluation-state-schema.md`
- `docs/reports/0049-current-l2-step-semantics.md`
- `docs/reports/0050-review-0049-short-rereview.md`
- `docs/reports/0051-current-l2-oracle-api.md`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/tests/fixtures/current-l2/`
- `crates/mir-semantics/src/lib.rs`

## 4. Actions taken

1. `0051` に対する short re-review を実施し、via-chain `Reject` boundary が narrow すぎるという finding を確認した。
2. `specs/examples/04-current-l2-step-semantics.md`、`specs/examples/05-current-l2-oracle-api.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` を最小修正し、well-formed chain が success を返さずに尽きた場合の request-level `Reject` 読みを揃えた。
3. `crates/mir-semantics/Cargo.toml` に `serde` / `serde_json` を追加し、fixture を直接読む最小依存を整えた。
4. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` を先に追加し、次の RED を確認した。
   - public API 不足
   - runtime carrier / static gate / fixture loader 不足
5. `crates/mir-semantics/src/lib.rs` に current L2 専用の direct-style evaluator skeleton を実装した。
   - fixture carrier (`CurrentL2Fixture`, `Program`, `Statement`, `Contract`, `Predicate` など)
   - `StaticGateVerdict` / `static_gate` / `static_gate_detailed`
   - `EvaluationState`
   - `step_once`
   - `run_to_completion`
   - `PredicateOracle` / `EffectOracle` / `SuccessCarrier` skeleton
   - `TraceAuditSink` と non-admissible metadata carrier
6. `Program` / `PlaceBlock` / `PerformOn` / `PerformVia` / `TryFallback` / `AtomicCut` を最小実行対象にし、`OptionDecl` / `ChainDecl` は runtime declaration-only no-op とした。
7. `PerformOn` では request-local `require`、effect attempt、`ensure`、commit を direct-style で順に処理し、`ensure` 通過後だけ `place_store` を更新するようにした。
8. `PerformVia` では chain left-to-right evaluation を実装し、
   - `lease-expired` は formal metadata
   - capability mismatch は narrative explanation
   - `admit-miss` は formal metadata
   - final-candidate `require` / `explicit_failure` / `ensure` failure は request-level `Reject`
   として current L2 を維持した。
9. `TryFallback` と `rollback_stack` により E2 の local rollback を表し、`AtomicCut` は active rollback frame の `restore_snapshot` 更新として表した。
10. テストを green に寄せた後、fixture の `expected_runtime` / `expected_trace_audit` をより直接比較するように tightening した。
11. `specs/examples/06-current-l2-interpreter-skeleton.md` を追加し、machine-checked expectation と narrative explanation の境界、full interpreter に進まない範囲を補助文書として整理した。
12. `Documentation.md` と `specs/00-document-map.md` に 06 文書への導線を追加した。
13. final reviewer は依頼したが completion が返らず、`0053` に未返却として記録した。

## 5. Evidence / outputs / test results

### Files changed

仕様本文 / 実装本文 commit `ef797c3` に含めた変更:

- 更新: `Documentation.md`
- 更新: `crates/mir-semantics/Cargo.toml`
- 更新: `crates/mir-semantics/src/lib.rs`
- 新規: `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- 更新: `specs/00-document-map.md`
- 更新: `specs/10-open-questions.md`
- 更新: `specs/12-decision-register.md`
- 更新: `specs/examples/04-current-l2-step-semantics.md`
- 更新: `specs/examples/05-current-l2-oracle-api.md`
- 新規: `specs/examples/06-current-l2-interpreter-skeleton.md`

report commit に含める変更:

- 新規: `docs/reports/0052-review-0051-short-rereview.md`
- 新規: `docs/reports/0053-final-review-current-l2-interpreter-skeleton.md`
- 新規: `docs/reports/0054-current-l2-parser-free-interpreter-skeleton.md`

### Commands run and exact outputs

task 開始時点:

```bash
git status --short --branch
```

```text
## main...origin/main [ahead 9]
```

RED:

```bash
cargo test -p mir-semantics
```

```text
error[E0369]: binary operation `==` cannot be applied to type `&Vec<Statement>`
...
error: could not compile `mir-semantics` (lib) due to 7 previous errors
```

GREEN:

```bash
cargo test -p mir-semantics
```

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 3.01s
Running tests/current_l2_minimal_interpreter.rs
running 2 tests
test static_gate_rejects_malformed_and_underdeclared_fixtures ... ok
test runtime_fixtures_reach_expected_outcomes ... ok
test result: ok. 2 passed; 0 failed
```

fresh verification:

```bash
cargo check -p mir-semantics
```

```text
Checking mir-semantics v0.1.0 (/home/yukatayu/dev/mir_poc_01/crates/mir-semantics)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 3.38s
```

```bash
cargo test -p mir-semantics
```

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 3.41s
running 2 tests
test static_gate_rejects_malformed_and_underdeclared_fixtures ... ok
test runtime_fixtures_reach_expected_outcomes ... ok
test result: ok. 2 passed; 0 failed
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
Found 54 numbered report(s).
```

```bash
python3 - <<'PY'
import json
from pathlib import Path
paths = sorted(Path('crates/mir-ast/tests/fixtures/current-l2').glob('*.json'))
for path in paths:
    with path.open() as f:
        json.load(f)
print(f'Loaded {len(paths)} fixture(s).')
PY
```

```text
Loaded 6 fixture(s).
```

仕様本文 / 実装本文 commit:

```text
ef797c3 current L2 の parser-free interpreter skeleton を実装する
```

### Findings

- parser-free minimal interpreter として必要な最小 node set は、current L2 では `Program` / `PlaceBlock` / `PerformOn` / `PerformVia` / `TryFallback` / `AtomicCut` と declaration-only no-op の `OptionDecl` / `ChainDecl` で足りた。
- static gate と runtime evaluation を分離したことで、E4 / E5 は runtime state に入る前に停止し、E1 / E2 / E3 variant / E6 だけが `run_to_completion` に入る構図を保てた。
- `expected_trace_audit` は dedicated skip event を増やさずに、`event_kinds` / `non_admissible_metadata` / `narrative_explanations` の最小比較で十分照合できた。
- `must_explain` は引き続き human-facing explanation であり、machine-checked field には入れていない。
- `ensure` は preview / commit 分離を実装しないと semantically dead になりやすいが、`SuccessCarrier` を介して tentative post-state を見る構成にしたことで current L2 を保てた。
- `lease-expired` と `admit-miss` は formal metadata、capability mismatch は narrative explanation、final `Reject` は request-level outcome という current L2 の境界を実装側でも維持できた。
- final reviewer completion は返らなかったため、`0053` に未返却として記録し、local verification を主証跡にした。

## 6. What changed in understanding

- AST fixture schema、evaluation state schema、step semantics、oracle API を順に固定してから実装に入ると、parser なしでも direct-style evaluator をかなり素直に切れることが明確になった。
- current L2 では `non-admissible` と `explicit failure` と `Reject` を同じ carrier に混ぜない方が、E3 variant と E6 のような例を小さく実装できる。
- representative examples の `expected_trace_audit` は、event / metadata / narrative の三層に分けると machine check と human explanation の役割分担が安定する。

## 7. Open questions

- richer predicate evaluator API は **未決定**。
- effect oracle の host interface / serialization は **未決定**。
- multi-request scheduler は **未決定**。
- detached trace / audit serialization と field naming は **未決定**。
- `Approximate` / `Compensate` を parser-free minimal interpreter に入れるかは **未決定**。
- `must_explain` を machine-check field に上げるかどうかは **未決定**。
- parser 実装に進む前に、fixture-to-host binding と predicate / effect stub の拡張方針をもう一段整理する余地がある。

## 8. Suggested next prompt

`crates/mir-semantics` の parser-free minimal interpreter skeleton を前提に、fixture runner 用の host stub layer を整理してください。特に `PredicateOracle` / `EffectOracle` の test-only stub を一般化し、fixture ごとに predicate verdict と success-side carrier を declarative に差し替えられる小さな harness を追加すると、current L2 の example coverage を増やしやすくなります。
