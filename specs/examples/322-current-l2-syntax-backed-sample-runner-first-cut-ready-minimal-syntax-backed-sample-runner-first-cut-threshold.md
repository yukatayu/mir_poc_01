# 322 — current L2 syntax-backed-sample-runner-first-cut-ready minimal-syntax-backed-sample-runner-first-cut threshold

## 目的

`specs/examples/321-current-l2-actual-parser-to-program-lowering-first-cut-ready-syntax-backed-sample-runner-first-cut-comparison.md`
で syntax-backed sample runner first cut の current first choice を fixed した次段として、

- runner first cut の minimum をどこまでに留めるか
- sample argument / host plan / report shape / fail-closed boundary を minimum にどう残すか
- next line の verification ladder wiring へどこまで渡すか

を比較する。

ここで固定するのは
**current L2 syntax-backed-sample-runner-first-cut-ready minimal-syntax-backed-sample-runner-first-cut threshold**
であり、

- sample stage matrix
- bless / regression update flow
- final public CLI / host-plan discovery

はまだ固定しない。

## 比較観点

1. runner boundary を minimum で読めるか
2. sample argument mode と host plan mode を minimum に残せるか
3. verification ladder task を先取りしないか

## 比較対象

### 案 1. `code_anchor_refs` だけを持つ

#### 利点

- 軽い。

#### 欠点

- sample argument mode、host plan mode、report shape、fail-closed boundary が minimum に現れない。
- ladder task へ渡したい current cut が弱い。

### 案 2. `runner_kind + sample_argument_mode + host_plan_mode + code_anchor_refs + report_shape_refs + fail_closed_refs + retained_later_refs` を持つ

#### 利点

- runner first cut の guard を minimum で読める。
- explicit path / stem shorthand と explicit host plan input を narrow に残せる。
- ladder task へ sample id / path / runtime report carrier を渡せる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. reached stage inventory と bless policy まで minimum に含める

#### 利点

- 後段 task との接続は見えやすい。

#### 欠点

- runner threshold ではなく ladder / policy threshold を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `runner_kind + sample_argument_mode + host_plan_mode + code_anchor_refs + report_shape_refs + fail_closed_refs + retained_later_refs` を持つ**
である。

理由は次の通り。

1. current task は file-path-backed source sample runner の boundary を固定する task である。
2. sample argument mode と host plan mode を minimum に残さないと、ladder task で runner contract が drift しやすい。
3. reached stage inventory と bless policy は next package へ分離したままでよい。

## current first choice shape

```text
syntax_backed_sample_runner_first_cut = {
  runner_kind = current_l2_fixed_source_sample_runner_first_cut,
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

current minimal runner first cut が示すのは、

- `mir-runtime::current_l2` の helper-local runner を current anchor に置く
- sample argument は accepted sample set 内の explicit path と stem shorthand に留める
- host plan は explicit input に留める
- report は sample id / path / lowered source program / runtime skeleton report に留める
- auto-resolution / reached stage / bless policy / final public CLI は still later に押し戻す

という cut である。

## next promoted line

next promoted line は、
**syntax-backed-sample-runner-first-cut-ready verification-ladder-wiring comparison**
に置く。

## open questions

- reached stage matrix を docs-only row と helper evidence のどちらへ寄せるか
- runtime sample と static-only sample で host plan policy をどこまで共通化するか
- final public runner/CLI line を Phase 6 に残すか later actualization に送るか
