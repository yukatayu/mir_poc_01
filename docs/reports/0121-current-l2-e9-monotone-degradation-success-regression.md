# Report 0121 — current L2 e9 monotone degradation success regression

## 1. Title and identifier

- Report 0121
- current L2 e9 monotone degradation success regression

## 2. Objective

current L2 parser-free PoC 基盤で、`admit-miss` のあとに middle option が explicit failure しても、
later same-lineage write-capable option へ left-to-right monotone degradation して success しうることを
runtime regression fixture として固定する。

## 3. Scope and assumptions

- current L2 の core semantics、parser grammar、failure family、machine-check policy は変更しない。
- current fallback reading は guarded option chain / left-to-right monotone degradation / no re-promotion のままとする。
- 今回は runtime fixture 1 本の追加、その fixture に伴う tests / prose mirror / progress 更新に限定する。
- `plan/` は relevant mirror を更新する。

## 4. Documents consulted

1. `README.md`
2. `Documentation.md`
3. `progress.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. `specs/examples/02-current-l2-ast-fixture-schema.md`
10. `specs/examples/06-current-l2-interpreter-skeleton.md`
11. `specs/examples/08-current-l2-host-plan-schema.md`
12. `plan/05-fallback-lease-and-chain-semantics.md`
13. `plan/08-representative-programs-and-fixtures.md`
14. `plan/15-current-l2-fixture-authoring-template.md`
15. `plan/90-source-traceability.md`
16. `docs/reports/0078-current-l2-fallback-lease-regression-fixtures.md`
17. `crates/mir-semantics/src/harness.rs`
18. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
19. `crates/mir-ast/tests/fixtures/current-l2/`

## 5. Actions taken

1. 未完了だった red state を確認し、new runtime regression を `e9-monotone-degradation-success` として確定した。
2. `crates/mir-ast/tests/fixtures/current-l2/e9-monotone-degradation-success.json` と sidecar を整え、`owner_writer` admit miss、`delegated_writer` explicit failure、`backup_writer` success を current host plan schema で表現した。
3. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` の focused test を green にし、directory discovery / runtime-only selection / profiled runtime summary の counts を 9 bundles / 7 runtime / 2 static に追随させた。
4. `plan/08-representative-programs-and-fixtures.md`、`specs/examples/02-current-l2-ast-fixture-schema.md`、`specs/examples/06-current-l2-interpreter-skeleton.md` を更新し、e9 を E3/E6 success-side 補完 regression として mirror した。
5. `progress.md` を更新し、fixture authoring / detached validation loop の進捗と作業ログに e9 追加を反映した。
6. `plan/90-source-traceability.md` に report `0121` を追加した。
7. final reviewer を 1 回だけ使い、`plan/05` の fallback memory、runtime fixture integration loop、`progress.md` の状態文、report の docs count を narrow に補正した。

## 6. Evidence / outputs / test results

### failing test first

task 開始時点では、focused test が missing fixture で red になっていた。

```text
cargo test -p mir-semantics monotone_degradation_can_reach_later_success_after_middle_failure -- --nocapture
...
called `Result::unwrap()` on an `Err` value: Io(... No such file or directory)
```

### green evidence

```text
cargo test -p mir-semantics monotone_degradation_can_reach_later_success_after_middle_failure -- --nocapture
...
test monotone_degradation_can_reach_later_success_after_middle_failure ... ok
```

```text
cargo test -p mir-semantics
...
test result: ok. 34 passed; 0 failed
```

### detached loop smoke

```text
python3 scripts/current_l2_detached_loop.py emit-fixture crates/mir-ast/tests/fixtures/current-l2/e9-monotone-degradation-success.json --run-label e9-smoke --overwrite
/home/yukatayu/dev/mir_poc_01/target/current-l2-detached/bundles/e9-smoke/e9-monotone-degradation-success.detached.json
```

```text
python3 scripts/current_l2_detached_loop.py compare-fixtures crates/mir-ast/tests/fixtures/current-l2/e8-monotone-degradation-reject.json crates/mir-ast/tests/fixtures/current-l2/e9-monotone-degradation-success.json --left-label e8-smoke --right-label e9-smoke --overwrite
payload_core differences:
- payload_core.terminal_outcome: left="Reject" right="success"
- payload_core.event_kinds: left=["perform-failure", "Reject"] right=["perform-failure", "perform-success"]
- payload_core.narrative_explanations: left=["readonly remains a request/capability mismatch narrative explanation"] right=[]
```

### docs validation

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 121 numbered report(s).
```

```text
git diff --check
```

無出力。

### review follow-up

- reviewer finding:
  - `plan/05` に `e9` が反映されていなかった
  - broad runtime fixture loops に `e9` が入っていなかった
  - `progress.md` の log 文が「実装中」のままで drift していた
  - report 内の docs count が `120` のまま残っていた
- fixes:
  - `plan/05`、`current_l2_minimal_interpreter.rs`、`progress.md`、report 本文を補正した

## 7. What changed in understanding

- current L2 では、middle option の explicit failure は chain exhaustion や earlier option への re-promotion を意味しない。
- `admit-miss` で先頭 option が formal non-admissible metadata に残り、そのあと middle option が explicit failure しても、later same-lineage write-capable option が request を満たせば success しうる。
- `e8` が reject-side regression を固定するのに対し、`e9` は success-side 補完 regression として対になる。

## 8. Open questions

- `e9` に続く次の narrow regression として、`PerformOn` / `ensure` failure まわりを fixture 化するか。
- detached validation loop の aggregate compare helper をどこまで exact-compare core に寄せるか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、detached validation loop の aggregate compare helper を narrow に追加し、bundle_failure_kind_counts_scope = "migrated-kinds-only" を前提に 2 run の aggregate artifact を比較できる non-production helper を実装してください。e9 fixture 追加後の progress と plan mirror も必要に応じて更新してください。`
