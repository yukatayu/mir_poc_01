# Report 0153 — detached bundle transform helper

- Date: 2026-04-05T20:42:00+09:00
- Author / agent: Codex
- Scope: current L2 detached validation loop における bundle-first emitter 内 private transform を repo 内 callable boundary へ切り出し、docs / plan / progress mirror を整合させる
- Decision levels touched: L2

## 1. Objective

- `current_l2_emit_detached_bundle.rs` 内 private transform を `examples/support/` へ切り出し、aggregate / static-gate helper と同じ non-production shared support cut を bundle 側にも与える。
- `FixtureBundle + BundleRunReport -> detached bundle artifact` の actual narrow cut を docs / plan / tests / progress に揃える。
- public `lib.rs` / `harness.rs` API を増やさず、detached validation loop の emitter smoke を維持する。

## 2. Scope and assumptions

- current L2 runtime semantics、parser grammar、failure family、machine-check policy は変えない。
- bundle-first detached artifact の `bundle_context` / `payload_core` / `detached_noncore` split は維持する。
- `host_plan_coverage_failure` は bundle success artifact へ持ち込まず aggregate-only のまま維持する。
- actual exporter API finalization、final path policy、production serialization versioning は今回固定しない。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `progress.md`
- `specs/examples/18-current-l2-bundle-first-detached-payload-context-split.md`
- `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
- `specs/examples/31-current-l2-detached-aggregate-transform-helper.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
- `crates/mir-semantics/examples/support/current_l2_detached_aggregate_support.rs`
- `crates/mir-semantics/examples/support/current_l2_static_gate_support.rs`
- `crates/mir-semantics/tests/current_l2_detached_aggregate_support.rs`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
- `scripts/current_l2_detached_loop.py`

## 4. Actions taken

1. current detached validation loop の bundle-side code anchor を再読し、bundle artifact transform が example private code に残っている一方、aggregate / static-gate 側は `examples/support/` shared helper まで actualized 済みであることを確認した。
2. `crates/mir-semantics/tests/current_l2_detached_bundle_support.rs` を先に追加し、missing support module により compile failure する RED を確認した。
3. `crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs` を追加し、`FixtureBundle + BundleRunReport -> detached bundle artifact` の pure transform と carrier struct を shared support helper として切り出した。
4. `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs` を thin wrapper 化し、artifact 生成を new support helper へ委譲するようにした。
5. `specs/examples/37-current-l2-detached-bundle-transform-helper.md` を追加し、bundle-side actual narrow cut を docs-only / helper-boundary judgment として整理した。
6. `Documentation.md`、`specs/00-document-map.md`、`plan/07`、`plan/09`、`plan/11`、`plan/12`、`progress.md` を更新し、bundle transform helper を detached exporter chain の current understanding へ反映した。
7. `plan/90-source-traceability.md` に current task の spec / report anchor を追加した。
8. code mapper subagent `Leibniz` を dispatch し、bundle emitter private transform が narrow に shared support helper 化可能であり、blast radius は bundle emitter / support module / tests / docs mirror に主に限られることを確認した。
9. reviewer `Curie` を 1 回だけ dispatch して completion まで待ち、`scripts/__pycache__/` が task diff に混入していた low finding を回収した。bytecode を除去し、runtime fixture test に formal `non_admissible_metadata` の direct assertion を追加した。

## 5. Files changed

- `crates/mir-semantics/examples/current_l2_emit_detached_bundle.rs`
- `crates/mir-semantics/examples/support/current_l2_detached_bundle_support.rs`
- `crates/mir-semantics/tests/current_l2_detached_bundle_support.rs`
- `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
- `specs/examples/37-current-l2-detached-bundle-transform-helper.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0153-detached-bundle-transform-helper.md`
- `docs/reports/0154-detached-bundle-transform-helper-review.md`

## 6. Commands run and exact outputs

```text
$ cargo test -p mir-semantics --test current_l2_detached_bundle_support
   Compiling mir-semantics v0.1.0 (/home/yukatayu/dev/mir_poc_01/crates/mir-semantics)
error: couldn't read `crates/mir-semantics/tests/../examples/support/current_l2_detached_bundle_support.rs`: No such file or directory (os error 2)
 --> crates/mir-semantics/tests/current_l2_detached_bundle_support.rs:6:1
  |
6 | mod current_l2_detached_bundle_support;
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: could not compile `mir-semantics` (test "current_l2_detached_bundle_support") due to 1 previous error
```

```text
$ cargo test -p mir-semantics --test current_l2_detached_bundle_support
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.26s
     Running tests/current_l2_detached_bundle_support.rs (target/debug/deps/current_l2_detached_bundle_support-6331bdde91fe7cd9)

running 2 tests
test detached_bundle_support_preserves_static_only_fixture_split ... ok
test detached_bundle_support_preserves_runtime_fixture_core_and_context ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```text
$ python3 scripts/current_l2_detached_loop.py emit-fixture crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --run-label bundle-helper-e3 --overwrite
/home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/bundle-helper-e3/e3-option-admit-chain.detached.json
```

```text
$ python3 scripts/current_l2_detached_loop.py emit-fixture crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json --run-label bundle-helper-e4 --overwrite
/home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/bundle-helper-e4/e4-malformed-lineage.detached.json
```

```text
$ cargo test -p mir-semantics
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.53s
...
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
...
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
...
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
...
test result: ok. 40 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
...
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
...
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 154 numbered report(s).
```

```text
$ git diff --check
(no output)
```

## 7. Evidence / findings

- bundle emitter 側にも aggregate / static-gate と同型の shared support helper を置けることが、failing test と final green test の両方で確認できた。
- emitter CLI 名、wrapper command、artifact field 名は維持されており、`scripts/current_l2_detached_loop.py` 側の path / overwrite / compare boundary は壊していない。
- runtime fixture `e3-option-admit-chain` と static-only fixture `e4-malformed-lineage` の双方で artifact emit が成功し、bundle support helper が runtime / static-only split の両方を保持していることを smoke で確認した。
- runtime fixture `e3-option-admit-chain` の direct test で formal `non_admissible_metadata = [{ option_ref = owner_writer, subreason = admit-miss }]` も helper extraction 後に保持されることを固定した。
- current code anchor と docs-only judgment の gap は、「bundle transform だけ shared support helper 化されていなかった」点にほぼ限られており、`lib.rs` / `harness.rs` の public API finalization まではまだ進める必要がない。
- reviewer finding は generated bytecode の混入 1 件だけで、semantic / boundary regression は指摘されなかった。

## 8. Changes in understanding

- detached validation loop の actual narrow cut は、bundle / aggregate / static-gate の 3 つすべてで `examples/support/` shared support helper まで actualize してよい段階に入った。
- それでも public exporter API を `lib.rs` / `harness.rs` に昇格させる理由はまだなく、current phase では non-production shared helper cut が最も自然である。
- bundle emitter の private transform 切り出しは detached validation loop を前へ進めるが、actual exporter API finalization、storage/path policy finalization、fixture authoring bottleneck は依然として別問題である。

## 9. Open questions

- bundle artifact transform helper を actual compare contract の finalization にどこまで近づけるか。
- `scripts/current_l2_detached_loop.py` の compare input discovery を explicit path 主体のまま維持するか、run-label convenience をどこまで formalize するか。
- detached validation loop を fixture authoring / elaboration 実地反復へどうさらに接続するか。

## 10. Suggested next prompt

- current L2 parser-free PoC 基盤を前提に、static-only / malformed / underdeclared fixture を 1〜2 本新規に authoring し、scaffold helper、`suggest-checked-reasons`、bundle/static-gate emit、aggregate smoke を使って detached validation loop の実地 friction を narrow に棚卸ししてください。
