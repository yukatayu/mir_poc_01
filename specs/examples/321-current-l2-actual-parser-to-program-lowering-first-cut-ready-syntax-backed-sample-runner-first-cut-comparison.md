# 321 — current L2 actual-parser-to-program-lowering-first-cut-ready syntax-backed-sample-runner-first-cut comparison

## 目的

`specs/examples/319-current-l2-representative-fixture-source-mapping-matrix-ready-actual-parser-to-program-lowering-first-cut-comparison.md`
と
`specs/examples/320-current-l2-actual-parser-to-program-lowering-first-cut-ready-minimal-actual-parser-to-program-lowering-first-cut-threshold.md`
で actual parser-to-`Program` lowering first cut を fixed した次段として、

- source sample をどこで file path から read し、lowerer と runtime skeleton へ束ねるか
- sample stem shorthand と explicit path をどの boundary で受けるか
- host plan / report shape / fail-closed boundary をどこまで first cut に含めるか

を比較する。

ここで固定するのは
**current L2 actual-parser-to-program-lowering-first-cut-ready syntax-backed-sample-runner-first-cut comparison**
であり、

- sample ごとの reached stage inventory
- source-sample bless / regression policy
- final public CLI / exporter

はまだ固定しない。

## scope

- entry criteria は `specs/examples/315...320` で fixed 済みの source corpus scope / layout、mapping matrix、lowering first cut とする。
- root source は `crates/mir-runtime/src/current_l2.rs`、`crates/mir-runtime/tests/current_l2_source_sample_runner.rs`、`samples/current-l2/README.md` を主に扱う。
- current authored source file は `e4` / `e2` / `e23` first trio に留める。

## current 前提

current repo では次が成立している。

1. `mir_runtime::current_l2::lower_current_l2_fixed_source_text` は helper-local lowerer として source text を semantic `Program` + optional stage 1 / stage 2 bridge evidence へ fail-closed に落とせる。
2. `mir_runtime::current_l2::run_current_l2_runtime_skeleton` は semantic `Program`、`FixtureHostPlan`、optional parser bridge input を受け、checker-floor summary + static gate + runtime report を束ねられる。
3. source corpus actual file first trio `e4` / `e2` / `e23` は repo-root `samples/current-l2/` に置かれている。
4. current task では public CLI や final host-plan discovery を固定せず、helper-local thin wrapper に留める必要がある。

したがって current 問いは、
**source sample を helper-local runner として `mir-runtime` 側で束ねるのが自然か**
である。

## 比較観点

1. source sample を file path から read / lower / static / interpreter へ一貫して流せるか
2. parser-free fixture runner と責務を分離したまま保てるか
3. sample stem shorthand と explicit path を narrow に受けられるか
4. verification ladder / bless policy / final public CLI を still later に残せるか

## 比較対象

### 案 1. lowerer test だけに留め、runner helper は作らない

#### 利点

- surface を増やさない。

#### 欠点

- source sample を file path から actual runnable path へ通せない。
- next task の verification ladder へ entry criteria を渡せない。

### 案 2. `mir-runtime::current_l2` に helper-local source sample runner を足す

#### shape

```text
syntax_backed_sample_runner_first_cut = {
  runner_kind = current_l2_fixed_source_sample_runner_first_cut,
  accepted_sample_refs = [
    sample_e4_malformed_lineage_txt,
    sample_e2_try_fallback_txt,
    sample_e23_malformed_try_fallback_missing_fallback_body_txt
  ],
  sample_argument_mode = [
    explicit_accepted_sample_path,
    sample_stem_shorthand
  ],
  host_plan_mode = explicit_fixture_host_plan_input,
  code_anchor_refs = [
    mir_runtime_current_l2_run_current_l2_source_sample,
    mir_runtime_current_l2_resolve_current_l2_source_sample_path,
    mir_runtime_current_l2_source_sample_runner_tests
  ],
  report_shape_refs = [
    sample_id,
    sample_path,
    lowered_source_program,
    current_l2_runtime_skeleton_report
  ],
  fail_closed_refs = [
    missing_sample_reject,
    file_read_failure_reject,
    source_lowering_failure_reject,
    parser_bridge_mismatch_reject
  ],
  retained_later_refs = [
    fixture_sidecar_auto_resolution,
    reached_stage_inventory,
    source_sample_bless_policy,
    final_public_cli_exporter
  ]
}
```

#### 利点

- source sample を actual runnable path へ通せる。
- lowerer / runtime skeleton を再利用し、public CLI を増やさずに済む。
- next task の verification ladder に sample id / path / runtime report をそのまま渡せる。

#### 欠点

- helper-local runner surface は 1 つ増える。
- host plan auto-resolution はまだ caller 側 responsibility に残る。

### 案 3. fixture/path discovery と final CLI まで同時に入れる

#### 利点

- long-term には一見まとまって見える。

#### 欠点

- helper-local thin wrapper の current stop lineを越えやすい。
- bless / regression policy や final public API を premature に固定しやすい。

## current judgment

current L2 で最も自然なのは、
**案 2. `mir-runtime::current_l2` に helper-local source sample runner を足す**
である。

理由は次の通り。

1. `mir-runtime` は current phase で lowerer と runtime skeleton の splice point を already 持つ。
2. source sample runner をここに置けば、parser-free fixture runner と public CLI を増やさずに syntax-backed path だけ actualize できる。
3. host plan は explicit input に留めることで、fixture-side auto-resolution や bless policy を still later に残せる。

## current first choice shape

```text
syntax_backed_sample_runner_first_cut = {
  runner_kind = current_l2_fixed_source_sample_runner_first_cut,
  accepted_sample_refs = [
    sample_e4_malformed_lineage_txt,
    sample_e2_try_fallback_txt,
    sample_e23_malformed_try_fallback_missing_fallback_body_txt
  ],
  sample_argument_mode = [
    explicit_accepted_sample_path,
    sample_stem_shorthand
  ],
  host_plan_mode = explicit_fixture_host_plan_input,
  code_anchor_refs = [
    mir_runtime_current_l2_run_current_l2_source_sample,
    mir_runtime_current_l2_resolve_current_l2_source_sample_path,
    mir_runtime_current_l2_source_sample_runner_tests
  ],
  report_shape_refs = [
    sample_id,
    sample_path,
    lowered_source_program,
    current_l2_runtime_skeleton_report
  ],
  fail_closed_refs = [
    missing_sample_reject,
    file_read_failure_reject,
    source_lowering_failure_reject,
    parser_bridge_mismatch_reject
  ],
  retained_later_refs = [
    fixture_sidecar_auto_resolution,
    reached_stage_inventory,
    source_sample_bless_policy,
    final_public_cli_exporter
  ]
}
```

## practical reading

current runner first cut が示すのは、次の 6 点である。

1. source sample runner は `mir-runtime::current_l2` の helper-local thin wrapper に置く
2. sample argument は accepted sample set 内の explicit path と sample stem shorthand の 2 形態に留める
3. host plan は current cut では explicit input に留める
4. report は `sample_id + sample_path + lowered + runtime_report` に留める
5. missing sample / read failure / lowering failure / bridge mismatch は fail-closed に止める
6. reached stage inventory / bless policy / final CLI は still later に残す

## next promoted line

next promoted line は、
**syntax-backed-sample-runner-first-cut-ready verification-ladder-wiring comparison**
に置く。

## open questions

- host plan auto-resolution を fixture path 側から導出する line を ladder task と policy task のどちらに残すか
- `e1` / `e3` / `e21` を runner actual file next cluster としてどの task で足すか
- formal hook reached stage を report shape に含めるか、それとも ladder task で別 matrix に切るか
