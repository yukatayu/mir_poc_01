# Report 0051 — current L2 oracle API

- Date: 2026-04-01
- Author / agent: Codex
- Scope: current L2 representative examples・AST fixture schema・evaluation state schema・step semantics を前提にした parser-free 最小 interpreter 用 predicate / effect oracle API
- Decision levels touched: L2

## 1. Objective

current L2 の representative examples を parser なしで実行準備できるようにするため、`PerformOn` / `PerformVia` が predicate / effect oracle に何を渡し、何を受け取り、step semantics の `Continue` / `BubbleFailure(kind)` / `Halt` にどう接続するかを最小限で整理する。

## 2. Scope and assumptions

- Mir-0 / Mir-1 / Mirrorea の境界は変更しない。
- canonical normalization law、rejection phase、static evidence floor、underdeclared handling、lineage annotation surface form、`perform` と option chain の companion notation、`try` / `fallback` block form、`contract` semantic-role-only policy、predicate sublanguage、option-local `admit` の current runtime reading、trace / audit reading、capability mismatch taxonomy、AST fixture schema、evaluation state schema、step semanticsは既存判断を維持する。
- parser 文字列 syntax は固定せず、意味側の oracle boundary だけを扱う。
- full interpreter 実装には進まず、必要なら trait / enum skeleton までに留める。
- 前段として `0049` の short re-review を行い、見つかった齟齬は最小修正で解消する。

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
- `docs/reports/0046-review-0045-short-rereview.md`
- `docs/reports/0047-current-l2-ast-fixture-schema-and-example-fixtures.md`
- `docs/reports/0048-current-l2-evaluation-state-schema.md`
- `docs/reports/0049-current-l2-step-semantics.md`
- `docs/reports/0050-review-0049-short-rereview.md`
- `crates/mir-semantics/src/lib.rs`

## 4. Actions taken

1. 指定順で正本を読み直し、AST fixture schema、evaluation state schema、step semantics の責務境界を再確認した。
2. `0049` を短く re-review し、`ensure` が step semantics から落ちている点と、E3 variant における `PerformVia` success 後の `place_store` 更新が state/schema/step 間で割れている点を抽出した。
3. `specs/examples/03-current-l2-evaluation-state-schema.md` と `specs/examples/04-current-l2-step-semantics.md` を最小修正し、次を current L2 として揃えた。
   - effect success は success-side carrier を返し、`ensure` 通過後にだけ `place_store` へ反映される
   - E3 variant は `place_store` を必要 state に含む
4. 新規補助文書 `specs/examples/05-current-l2-oracle-api.md` を追加し、次を整理した。
   - `PredicateOracle` / `EffectOracle` の最小分割
   - `PerformOn` / `PerformVia` が渡す最小 input
   - oracle の最小 return carrier
   - `success` / `explicit_failure` / `Reject` / non-admissible skip と step control の接続
   - E1 / E2 / E3 variant / E6 の読み
5. `Reject` を admitted option 単位の local effect verdict に入れると current L2 の request-level outcome 境界を壊す、という review finding を受け、`Reject` は effect oracle carrier から外して interpreter 側の request-level outcome に戻した。
6. `Documentation.md`、`specs/00-document-map.md`、`specs/examples/02-current-l2-ast-fixture-schema.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` に mirror と導線を追加した。
7. `crates/mir-semantics/src/lib.rs` には、field 名や host interface を固定しない generic な trait / enum skeleton だけを追加した。

## 5. Files changed

- 更新: `Documentation.md`
- 更新: `crates/mir-semantics/src/lib.rs`
- 更新: `specs/00-document-map.md`
- 更新: `specs/10-open-questions.md`
- 更新: `specs/12-decision-register.md`
- 更新: `specs/examples/02-current-l2-ast-fixture-schema.md`
- 更新: `specs/examples/03-current-l2-evaluation-state-schema.md`
- 更新: `specs/examples/04-current-l2-step-semantics.md`
- 新規: `specs/examples/05-current-l2-oracle-api.md`
- 新規: `docs/reports/0050-review-0049-short-rereview.md`
- 新規: `docs/reports/0051-current-l2-oracle-api.md`

## 6. Commands run and exact outputs

```bash
git status --short --branch
```

```text
## main...origin/main [ahead 7]
```

`0049` short re-review:

```text
1. High: `ensure` の実行意味が抜けています。
2. Medium: `PerformVia` 成功時の state 更新が文書間で割れています。
```

oracle API review:

```text
1. High: `EffectOracle::Reject` を admitted option 単位の verdict として導入すると、current L2 の request-level `Reject` 境界を広げすぎる。
```

fresh final reviewer:

```text
completion は返らず、close 時点の previous_status は running だった。
```

verification:

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
Found 51 numbered report(s).
```

```bash
cargo check -p mir-semantics
```

```text
Checking mir-semantics v0.1.0 (/home/yukatayu/dev/mir_poc_01/crates/mir-semantics)
Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.16s
```

仕様本文 commit:

```text
d5dfae2 current L2 の oracle API を整理する
```

## 7. Evidence / findings

- current L2 の parser-free 最小 interpreter では、predicate evaluator と effect executor を分けた方が小さく整合的である。non-admissible skip を effect oracle の local failure carrier に入れないことで、E3 variant と E6 の読みをそのまま保てる。
- `ensure` は request-local outcome-side predicate として保持し、effect success が返す success-side carrier を tentative post-state として評価した後にだけ `place_store` へ commit すると読むのが、`Mir Core` の contract 読みと最もよく噛み合う。
- `Reject` を effect oracle の local verdict に入れると、current L2 が保っている request-level outcome の境界を広げすぎる。review を受けて、`Reject` は effect oracle から外し、chain exhaustion など interpreter 側の structural rule からだけ導く形へ戻した。
- `crates/mir-semantics/src/lib.rs` の skeleton は、field 名や serialization を固定せずに将来の実装受け皿だけを置く最小形に留めた。
- fresh final reviewer は completion を返さなかったため、最終確認は reviewer の先行 finding 解消と local verification 出力に基づいて行った。
- 仕様本文 commit hash は `d5dfae2` である。report 自身の commit hash は self-reference の都合で本文に固定しない。

## 8. What changed in understanding

- `ensure` を単に request-local predicate の一種として扱うだけでは足りず、post-state をどう predicate oracle に見せるかまで companion API で切らないと semantically dead になりやすいことが明確になった。
- `Reject` は outcome lattice 上の named resultであって local effect verdict ではない、という current L2 の線引きが、oracle API を切る段階で初めて十分に具体化された。
- parser-free 最小 interpreter へ進む上では、surface syntax より先に oracle boundary を固定した方が、state schema / step semantics / fixture expectation の接続が安定する。

## 9. Open questions

- predicate oracle に richer verdict や explanation payload を持たせる必要があるか。
- effect oracle の host interface と success-side carrier の concrete layoutをどうするか。
- direct target に対する request-level `Reject` を、将来 oracle carrier に入れる必要があるか。
- `Approximate` / `Compensate` を parser-free 最小 interpreter の oracle carrier に入れる必要があるか。
- multi-request scheduler、detached trace / audit serialization、event id はなお **未決定**。

## 10. Suggested next prompt

`specs/examples/05-current-l2-oracle-api.md` を前提に、parser-free 最小 interpreter の direct-style evaluator skeleton を `crates/mir-semantics` に追加してください。特に `EvaluationState` を受けて `step_once` 相当が `PredicateOracle` / `EffectOracle` を呼び分け、E1 / E2 / E3 variant / E6 の fixture expectation に到達できる最小 Rust skeleton と walkthrough に絞ってください。
