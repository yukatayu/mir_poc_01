# Report 0047 — current l2 ast fixture schema and example fixtures

- Date: 2026-04-01T00:14:02.726675Z
- Author / agent: Codex
- Scope: current L2 representative examples を parser なしで machine-readable に扱う最小 AST fixture schema の整理、example fixture 追加、導線更新、verification、commit
- Decision levels touched: L2

## 1. Objective

current L2 の representative Mir programs を、parser 実装を先送りしたまま machine-readable な fixture に落とせるようにする。そのために、最小 AST node set、expected static / runtime / trace-audit carrier、4〜6 個の example fixture を current L2 の companion asset として整備する。

## 2. Inputs consulted

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
- `docs/reports/0024-representative-mir-programs-current-l2.md`
- `docs/reports/0025-perform-and-option-chain-surface-syntax-candidates.md`
- `docs/reports/0026-try-fallback-surface-syntax-candidates.md`
- `docs/reports/0029-contract-surface-policy-current-l2.md`
- `docs/reports/0030-require-ensure-punctuation-and-multiline-current-l2.md`
- `docs/reports/0032-predicate-sublanguage-current-l2.md`
- `docs/reports/0034-option-local-declared-contract-surface-current-l2.md`
- `docs/reports/0037-option-local-admit-runtime-admissibility-current-l2.md`
- `docs/reports/0039-admit-vs-lease-trace-audit-current-l2.md`
- `docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md`
- `docs/reports/0045-capability-mismatch-non-admissible-taxonomy-current-l2.md`
- `docs/reports/0046-review-0045-short-rereview.md`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-semantics/src/lib.rs`

## 3. Actions taken

1. まず `0045` の short re-review を行い、手元確認と reviewer subagent の短い返答の両方で no findings を確認した。
2. current L2 representative examples と companion notation を読み直し、parser 文字列に依存せず保持すべき意味構造を抽出した。
3. 最小 node set を `Program`、`PlaceBlock`、`PerformOn`、`PerformVia`、`OptionDecl`、`ChainDecl`、`TryFallback`、`AtomicCut` に絞った。
4. `contract` は surface keyword にせず、fixture AST では semantic role object として `PerformOn` / `PerformVia` の下に保持する方針にした。
5. option-local `admit` は `OptionDecl` にぶら下がる admission-side metadata とし、option-local `ensure` や outcome guarantee は持たせなかった。
6. predicate carrier は `atom`、`call`、`and` の 3 種だけに留め、括弧や改行の見た目は AST に持ち込まない方針にした。
7. machine-readable fixture の実体は、code map で自然な置き場と示された `crates/mir-ast/tests/fixtures/current-l2/` に置き、schema の説明文書は `specs/examples/02-current-l2-ast-fixture-schema.md` に追加した。
8. 6 本の example fixture を追加した。
   - E1 place + `atomic_cut`
   - E2 local `try` + `fallback`
   - E3 比較用 variant の option-local `admit`
   - E4 malformed lineage
   - E5 underdeclared lineage
   - E6 write-after-expiry / `Reject`
9. 各 fixture に `expected_static`、`expected_runtime`、`expected_trace_audit` を持たせ、current examples の読解と対応づけた。
10. `Documentation.md`、`specs/00-document-map.md`、`specs/examples/00-representative-mir-programs.md`、`specs/examples/01-current-l2-surface-syntax-candidates.md` に導線を追加し、`specs/10-open-questions.md` と `specs/12-decision-register.md` に mirror を追加した。
11. reviewer subagent に最終 review も依頼したが completion が返らなかったため、timeout 後に close し、その旨を report に残した。

## 4. Files changed

- Changed:
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `specs/10-open-questions.md`
  - `specs/12-decision-register.md`
  - `specs/examples/00-representative-mir-programs.md`
  - `specs/examples/01-current-l2-surface-syntax-candidates.md`
  - `specs/examples/02-current-l2-ast-fixture-schema.md`
  - `crates/mir-ast/tests/fixtures/current-l2/e1-place-atomic-cut.json`
  - `crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.json`
  - `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
  - `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
  - `crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json`
  - `crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.json`
  - `docs/reports/0046-review-0045-short-rereview.md`
  - `docs/reports/0047-current-l2-ast-fixture-schema-and-example-fixtures.md`
- Checked but not changed:
  - `README.md`
  - `specs/01-charter-and-decision-levels.md`
  - `specs/02-system-overview.md`
  - `specs/03-layer-model.md`
  - `specs/04-mir-core.md`
  - `specs/09-invariants-and-constraints.md`
  - `specs/11-roadmap-and-workstreams.md`
- Task-start dirty state:
  - task 開始時点では clean worktree だった。
  - 途中で `docs/reports/0046-review-0045-short-rereview.md` がこの task により未追跡で生成されたが、既存 dirty state ではない。

## 5. Commands run and exact outputs

- `python3 scripts/new_report.py --slug current-l2-ast-fixture-schema-and-example-fixtures`
  - `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - `  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0047-current-l2-ast-fixture-schema-and-example-fixtures.md`
- fixture JSON load check
  - `Loaded 6 fixture(s).`
  - `e1-place-atomic-cut.json: valid / explicit_failure`
  - `e2-try-fallback.json: valid / success`
  - `e3-option-admit-chain.json: valid / success`
  - `e4-malformed-lineage.json: malformed / not_evaluated`
  - `e5-underdeclared-lineage.json: underdeclared / not_evaluated`
  - `e6-write-after-expiry.json: valid / Reject`
- `git diff --check`
  - `[no output]`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 47 numbered report(s).`
- `git commit --no-gpg-sign -m "0045 の短い再 review を記録する"`
  - `[main ddd40bf] 0045 の短い再 review を記録する`
  - ` 1 file changed, 73 insertions(+)`
  - ` create mode 100644 docs/reports/0046-review-0045-short-rereview.md`
- `git commit --no-gpg-sign -m "current L2 の AST fixture schema と例を追加する"`
  - `[main 1a80bb5] current L2 の AST fixture schema と例を追加する`
  - ` 13 files changed, 817 insertions(+), 1 deletion(-)`
  - ` create mode 100644 crates/mir-ast/tests/fixtures/current-l2/e1-place-atomic-cut.json`
  - ` create mode 100644 crates/mir-ast/tests/fixtures/current-l2/e2-try-fallback.json`
  - ` create mode 100644 crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
  - ` create mode 100644 crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
  - ` create mode 100644 crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json`
  - ` create mode 100644 crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.json`
  - ` create mode 100644 specs/examples/02-current-l2-ast-fixture-schema.md`

## 6. Evidence / findings

### 6.1 最小 AST node set

current L2 representative examples を parser なしで落とすには、次の node set で足りる。

- `Program`
- `PlaceBlock`
- `PerformOn`
- `PerformVia`
- `OptionDecl`
- `ChainDecl`
- `TryFallback`
- `AtomicCut`

この構成なら、surface punctuation を固定せずに、`place` 入れ子、request-local clause、option-local `admit`、canonical chain、local rollback、`atomic_cut` をすべて保持できる。

### 6.2 syntax 依存情報を fixture に埋めていないこと

- `require pred` と `require:` block の見た目の差は残していない。
- `contract` は surface keyword にせず、semantic role object として保持した。
- `lineage(A -> B)` の token 形は残さず、edge-local assertion object として保持した。
- predicate の括弧や改行位置は残さず、`atom` / `call` / `and` tree に正規化した。

したがって、未決の punctuation や field naming を final parser decision として schema に埋め込んではいない。

### 6.3 representative examples の fixture 化

6 本の fixture で current L2 representative set の主要構成要素をカバーできた。

- E1: `place` 入れ子、`PerformOn`、statement-local `require` / `ensure`、`AtomicCut`
- E2: `TryFallback`
- E3 比較用 variant: `OptionDecl`、`ChainDecl`、option-local `admit`、`PerformVia`
- E4: malformed lineage
- E5: underdeclared lineage
- E6: runtime `Reject`、`lease-expired` metadata、capability mismatch narrative

### 6.4 expected behavior carrier

fixture 側の expected behavior は、次で十分だった。

- `expected_static.verdict`
  - `valid`
  - `malformed`
  - `underdeclared`
- `expected_runtime`
  - `enters_evaluation`
  - `final_outcome`
- `expected_trace_audit`
  - `event_kinds`
  - `non_admissible_metadata`
  - `narrative_explanations`
  - `must_explain`

この粒度で、E3 比較用 variant の `admit-miss` と、E6 の `lease-expired` / capability mismatch narrative を current L2 の読みどおりに保持できる。

### 6.5 review

- `0045` の short re-review は reviewer subagent から `No findings.` を得た。
- 本 task の最終 reviewer は completion を返さなかったため、timeout 後に close した。
- したがって final review finding は捏造せず、手元 verification と diff inspection で補完した。

### 6.6 commit

- 先行 review report commit:
  - `ddd40bf` `0045 の短い再 review を記録する`
- 仕様本文 / fixture schema commit:
  - `1a80bb5` `current L2 の AST fixture schema と例を追加する`
- report 自身の commit hash は self-reference の都合で本書には固定せず、最終 assistant response と `git log` で示す。

## 7. Changes in understanding

current L2 の representative examples を machine-readable にするうえで重要なのは、surface syntax を無理に AST へ写すことではなく、**syntax で未決の部分を落としても失われない意味構造**を切り出すことだった。`contract` を semantic role object として持たせる、`admit` を option-local metadata に留める、`lineage` を edge assertion として持つ、という 3 点で、current L2 の companion notation と parser-free fixture の間をかなり素直につなげられた。

## 8. Open questions

- field 名をどの decision で固定するか。
- detached trace / audit serialization が必要になったとき、`request ref` や `reason kind` をどの段階で追加するか。
- predicate carrier に `or` / `not` / comparison を入れる必要が出たとき、current L2 fixture schema をどこまで拡張するか。
- fixture から最小 interpreter 実装へ進むとき、evaluation state、rollback state、cut state をどの粒度で持つか。

## 9. Suggested next prompt

current L2 AST fixture schema を前提に、fixture から最小 interpreter を組むための evaluation state schema を整理してください。特に `place` stack、current request、chain evaluation cursor、local rollback region、`atomic_cut` frontier、expected trace sink をどの抽象粒度で持てば E1・E2・E3 variant・E6 を実行できるかだけに絞ってください。
