# Report 0108 — detached validation-loop sprint

- Date: 2026-04-03
- Author / agent: Codex
- Scope: current L2 parser-free PoC 基盤に対する detached validation-loop sprint
- Decision levels touched: current L2 settled judgment の mirror、current parser-free PoC reading、docs-only boundary refinement

## 1. Objective

current L2 parser-free PoC を、

- 実行する
- detached artifact を保存する
- payload core を比較する
- fixture を追加してまた回す

という継続的 validation loop の入口へ近づけるために、detached exporter chain の docs-only judgment を storage / aggregate API / fixture authoring guidance まで接続し、non-production の tiny wrapper を追加する。

## 2. Inputs consulted

- repo 入口
  - `AGENTS.md`
  - `README.md`
  - `Documentation.md`
  - `specs/00-document-map.md`
- 基本 spec
  - `specs/01-charter-and-decision-levels.md`
  - `specs/02-system-overview.md`
  - `specs/03-layer-model.md`
  - `specs/04-mir-core.md`
  - `specs/09-invariants-and-constraints.md`
  - `specs/10-open-questions.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `specs/12-decision-register.md`
- parser-free PoC / detached exporter chain
  - `specs/examples/02`〜`10`
  - `specs/examples/11`〜`13`
  - `specs/examples/15`
  - `specs/examples/16`〜`24`
- plan mirror
  - `plan/00-index.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/05-fallback-lease-and-chain-semantics.md`
  - `plan/06-surface-notation-status.md`
  - `plan/07-parser-free-poc-stack.md`
  - `plan/08-representative-programs-and-fixtures.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/15-current-l2-fixture-authoring-template.md`
  - `plan/90-source-traceability.md`
  - `plan/91-maintenance-rules.md`
- report chain
  - `docs/reports/0089`
  - `docs/reports/0090`〜`0107`
- code anchor
  - `crates/mir-semantics/src/lib.rs`
  - `crates/mir-semantics/src/harness.rs`
  - `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
  - `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
  - `scripts/current_l2_diff_detached_artifacts.py`
  - `crates/mir-ast/tests/fixtures/current-l2/`

## 3. Actions taken

1. detached exporter chain の current docs-only judgment と code anchor を再確認した。
2. `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md` を追加し、次を 1 箇所へ統合した。
   - bundle-first detached exporter から aggregate export への接続面
   - `bundle_failure_kind_counts` の additive coexistence
   - artifact 保存先 / path policy の最小候補
   - file naming / overwrite policy / compare input discovery
3. `scripts/current_l2_detached_loop.py` を追加し、bundle-first emitter と payload-core diff helper をつなぐ non-production wrapper を用意した。
4. `plan/15-current-l2-fixture-authoring-template.md` を改良し、detached validation loop 前提の fixture authoring 手順、profile-targeted run の扱い、責務境界を追記した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/00`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`plan/90`、`specs/examples/23` を最小更新した。
6. E3 / E6 を使って tiny wrapper の smoke evidence を取った。
7. 仕様本文と helper 変更を commit `0b9acef detached validation loop の運用面を整える` として確定した。

## 4. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
- `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `scripts/current_l2_detached_loop.py`

## 5. Commands run and exact outputs

### task-start dirty state

```text
$ git status --short --branch
## main...origin/main [ahead 2]
```

### tiny wrapper help

```text
$ python3 scripts/current_l2_detached_loop.py --help
usage: current_l2_detached_loop.py [-h]
                                   {emit-fixture,compare-artifacts,compare-fixtures}
                                   ...
```

### 1 fixture export

```text
$ python3 scripts/current_l2_detached_loop.py emit-fixture crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --run-label smoke-e3 --overwrite
/home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/smoke-e3/e3-option-admit-chain.detached.json
```

### self-compare smoke

```text
$ python3 scripts/current_l2_detached_loop.py compare-fixtures crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --left-label smoke-e3-left --right-label smoke-e3-right --overwrite
left artifact : /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/smoke-e3-left/e3-option-admit-chain.detached.json
right artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/smoke-e3-right/e3-option-admit-chain.detached.json
=== current L2 detached artifact diff ===
left : /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/smoke-e3-left/e3-option-admit-chain.detached.json
right: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/smoke-e3-right/e3-option-admit-chain.detached.json

payload_core: exact-compare core matched

note: must_explain と long-form explanation は比較対象に含めない
```

### cross-compare smoke

```text
$ python3 scripts/current_l2_detached_loop.py compare-fixtures crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.json --left-label smoke-e3 --right-label smoke-e6 --overwrite
left artifact : /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/smoke-e3/e3-option-admit-chain.detached.json
right artifact: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/smoke-e6/e6-write-after-expiry.detached.json
=== current L2 detached artifact diff ===
left : /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/smoke-e3/e3-option-admit-chain.detached.json
right: /home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/smoke-e6/e6-write-after-expiry.detached.json

payload_core differences:
- payload_core.terminal_outcome: left="success" right="Reject"
- payload_core.event_kinds: left=["perform-success"] right=["Reject"]
- payload_core.non_admissible_metadata: left=[{"option_ref": "owner_writer", "subreason": "admit-miss"}] right=[{"option_ref": "writer", "subreason": "lease-expired"}]
- payload_core.narrative_explanations: left=[] right=["readonly remains a request/capability mismatch narrative explanation"]

reference-only differences:
- bundle_context: left={"fixture_id": "e3_option_admit_chain", "fixture_path": "crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json", "host_plan_path": "crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.host-plan.json", "runtime_requirement": "runtime-with-host-plan"} right={"fixture_id": "e6_write_after_expiry", "fixture_path": "crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.json", "host_plan_path": "crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.host-plan.json", "runtime_requirement": "runtime-with-host-plan"}
- detached_noncore: left={"steps_executed": 8} right={"steps_executed": 7}

note: must_explain と long-form explanation は比較対象に含めない
```

### docs / diff / tests

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 109 numbered report(s).

$ git diff --check
<no output>

$ cargo test -p mir-semantics
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
Doc-tests mir_semantics
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 6. Evidence / findings

- detached validation loop の current non-production default candidate として `target/current-l2-detached/` を採っても、既存 helper boundary は壊れない。
- `scripts/current_l2_detached_loop.py` は `harness.rs` に新しい public API を追加せず、既存の emitter example と diff helper を薄く接続するだけに留まっている。
- compare input discovery は explicit path を基本としつつ、`artifact_root + run_label + fixture_stem` から bundle artifact path を導出する最小 convenience に留めるのが自然だった。
- `host_plan_coverage_failure` については、今回も success-side payload core へ逆流させず、aggregate-only / future typed carrier の current split を維持できている。
- `plan/15` を detached validation loop 前提へ寄せたことで、runtime fixture / static-only fixture / profile-targeted run の違いと、fixture authoring と exporter / batch / host interface の責務境界が 1 箇所で辿れるようになった。

## 7. Changes in understanding

- detached exporter chain は docs-only judgment の比較段階を超えて、current non-production default candidate と tiny wrapper を持つ「運用入口」まで進めてよい状態にある。
- ただし、これは production exporter API を決めたことではない。`target/current-l2-detached/` も final policy ではなく、generated artifact を repo 相対で扱いやすくする current default candidate に過ぎない。
- aggregate export の next narrow step は、`BatchRunSummary` を起点に `bundle_failure_kind_counts` を additive に持つ actual exporter sketch を切ることだが、bundle artifact summary の再掲や richer host interface typed 化へはまだ進まない方がよい。
- `fixture authoring / elaboration` は依然 independent bottleneck だが、detached validation loop と接続する手順は template 化できる。

## 8. Open questions

- actual exporter API を `lib.rs` / `harness.rs` / example / script のどこで切るか
- detached artifact 保存先の final policy
- `bundle_failure_kind_counts` を row list のままにするか object map にするか
- current bool/list anchor をいつ actual 実装で置き換えるか
- compare input discovery をどこまで formalize するか
- richer host interface の typed carrier 化をどの順で後段へ送るか
- final parser syntax
- multi-request scheduler
- `Approximate` / `Compensate`
- static analysis / theorem prover 側との boundary

## 9. Suggested next prompt

```text
あなたはこのリポジトリに初めて入る CodeX です。会話の過去文脈は信用せず、必ずリポジトリ内の文書とコードを正本として扱ってください。

今回は、current L2 parser-free PoC 基盤と specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md、および detached validation-loop sprint の tiny helper を前提に、
BatchRunSummary を起点にした non-production aggregate exporter sketch を 1 本だけ narrow に追加してください。

目的は、bundle-first artifact を壊さずに、
- current bool/list anchor
- future bundle_failure_kind_counts
の additive coexistence を actual JSON aggregate 出力として 1 度だけ通してみることです。

runtime semantics、parser grammar、failure family、machine-check policy、richer host interface は変更しないでください。
```

## 仕様本文コミット hash

- detached validation-loop sprint の仕様本文 / helper 本体 commit:
  - `0b9acef detached validation loop の運用面を整える`

report 自身の commit hash は self-reference の都合で本文に固定していない。
