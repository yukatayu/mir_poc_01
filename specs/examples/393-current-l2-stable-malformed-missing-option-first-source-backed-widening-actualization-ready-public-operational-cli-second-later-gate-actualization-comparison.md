# 393 — current L2 stable-malformed-missing-option-first-source-backed-widening-actualization-ready public-operational-cli-second-later-gate-actualization comparison

## 目的

`specs/examples/392-current-l2-stable-malformed-missing-option-first-source-backed-widening-actualization-ready-minimal-stable-malformed-missing-option-first-source-backed-widening-threshold.md`
で missing-option source-backed widening first cut の minimum を fixed した次段として、

- runtime-led thin library facade first later cut を巻き戻さずに public operational CLI second gate をどの operational wrapper cut から narrow に actualize するか
- library-side final public contract、repo-local Python helper / example surface、host-facing I/O / adapter line をどこまで separate に保つか
- sample selector / host-plan input / report output のどこまでを current operational wrapper cut に入れてよいか

を比較する。

ここで固定するのは
**current L2 stable-malformed-missing-option-first-source-backed-widening-actualization-ready public-operational-cli-second-later-gate-actualization comparison**
であり、

- final public parser / checker / runtime thin-facade later support actual shape
- final public parser grammar
- final host/input contract
- raw FFI / engine adapter / visualizer operational binding

はまだ固定しない。

## scope

- current package は docs-only actualization comparison に留める。
- `specs/examples/389...390` の runtime-led thin library facade first later cutを entry evidence に使う。
- `specs/examples/371...372` の library-before-CLI later orderingを巻き戻さない。
- repo-local Python helper、cargo example emitter、accepted-set hard-coding、path resolverは current CLI contract 候補へ上げない。

## current 前提

current repo では次が成立している。

1. `specs/examples/355...356` により、public-looking surface は already-public parser-free stack / crate-public but non-production tranche / repo-local helper surface の 3 bucket split に整理済みである。
2. `specs/examples/363...364` により、later public pressure の first docs-only candidate は `run_current_l2_source_sample` に narrow に置かれ、`run_current_l2_runtime_skeleton` / `lower_current_l2_fixed_source_text` は support-only、`resolve_current_l2_source_sample_path` と accepted-set hard-coding は excluded bucket に留めている。
3. `specs/examples/371...372` により、public-side later ordering は final public parser / checker / runtime API first、public operational CLI second に固定済みである。
4. `specs/examples/385...386` により、host-facing integration は privileged `stdin/stdout` ではなく capability-scoped I/O / adapter boundary として separate gate に置いており、current package で raw host / FFI / engine adapter line を actual operational CLI に混ぜない。
5. `specs/examples/389...390` により、current first later cut は `run_current_l2_source_sample` + `CurrentL2SourceSampleRunReport` の runtime-led thin library facade に固定済みであり、`run_current_l2_runtime_skeleton` / `lower_current_l2_fixed_source_text` は still support-only bucket に留めている。

したがって current 問いは、
**runtime-led thin facade first cut の上に載る operational wrapper を、repo-local helper 群と混ぜずに second later gate としてどこまで narrow に切るのが最小か**
である。

## 比較観点

1. runtime-led thin library facade first cut と library-before-CLI later orderingを保てるか
2. repo-local Python helper / cargo example / accepted-set hard-coding を current operational wrapper から外せるか
3. sample selector / host-plan input / report output を operational wrapper cut に narrow に残しつつ、final host/input contract を先取りしないで済むか
4. next line を final public parser/checker/runtime thin-facade later support actualization に clean に handoff できるか

## 比較対象

### 案 1. Rust-side operational wrapper over `run_current_l2_source_sample` を second later gate の current first cut に置く

#### shape

```text
public_operational_cli_second_later_gate = {
  actualization_kind = current_l2_rust_operational_wrapper_over_thin_facade,
  delegated_library_entry = run_current_l2_source_sample,
  delegated_public_report = CurrentL2SourceSampleRunReport,
  operational_request_surface = [
    sample_selector_argument,
    explicit_host_plan_input_mode
  ],
  operational_output_surface = [
    source_sample_run_report_json_or_pretty_summary
  ],
  support_only = [
    run_current_l2_runtime_skeleton,
    lower_current_l2_fixed_source_text
  ],
  excluded = [
    resolve_current_l2_source_sample_path,
    repo_layout_bound_accepted_sample_set,
    repo_local_python_orchestration_helpers,
    cargo_example_emitters_and_support_modules
  ]
}
```

#### 利点

- library-side thin facade first cutを保ったまま、CLI を separate operational wrapper として切れる。
- Rust-heavy core の読みを保ち、repo-local Python helper を public operational surface へ誤昇格しにくい。
- sample selector / host-plan input / output report だけを operational concern として narrow に残せる。

#### 欠点

- concrete command name、flag naming、JSON formatting、host-plan decoupling は still later に残る。

### 案 2. 既存の repo-local Python helper を public operational CLI の first cut に置く

#### 利点

- すぐに動く command surface は見えやすい。

#### 欠点

- regression / detached-loop / authoring helper を public operational surface と混同しやすい。
- Rust-heavy core + mixed-tool helper workflow の current split を崩しやすい。
- repo-local orchestration と final operational contract の境界が弱くなる。

### 案 3. `run_current_l2_source_sample` だけを operational CLI とみなす

#### 利点

- 境界は軽い。

#### 欠点

- library contract と operational wrapper の差が消える。
- sample selector / host-plan input / rendered output を separate concern として扱えない。

## current judgment

current L2 で最も自然なのは、
**案 1. Rust-side operational wrapper over `run_current_l2_source_sample` を second later gate の current first cut に置く**
である。

理由は次の通り。

1. `run_current_l2_source_sample` は current first later cut として既に fixed 済みなので、CLI 側はそれを巻き戻さない thin operational wrapper に留める方が later ordering と整合する。
2. repo-local Python helper は regression / detached-loop / authoring assist の責務を持っており、public operational CLI second gate へそのまま昇格させるのは境界違反になりやすい。
3. concrete command naming や host-plan decoupling はまだ OPEN であり、current package では `sample_selector + explicit_host_plan_input_mode + delegated_report_output` の narrow shell だけを残すのが最小である。

## current first choice details

- public operational CLI second later gate の current first cut は、Rust-side operational wrapper over `run_current_l2_source_sample` に置く。
- delegated library entry は `run_current_l2_source_sample`、delegated public report は `CurrentL2SourceSampleRunReport` に留める。
- operational wrapper が追加で持ってよい concern は `sample_selector_argument`、`explicit_host_plan_input_mode`、`source_sample_run_report_json_or_pretty_summary` に留める。
- `run_current_l2_runtime_skeleton` / `lower_current_l2_fixed_source_text` は still support-only bucket に残す。
- `resolve_current_l2_source_sample_path`、accepted-set hard-coding、repo-local Python orchestration helper、cargo example emitter / support module は excluded bucket に残す。
- concrete command 名、flag 名、JSON schema hardening、host-plan decoupling、raw host/FFI/engine adapter binding は kept-later に残す。

## next promoted line

next promoted line は、
**public-operational-cli-second-later-gate-actualization-comparison-ready final-public-parser-checker-runtime-thin-facade-later-support-actualization**
に置く。

## open questions

- Rust-side operational wrapper の concrete command name をどこで固定するか
- explicit host-plan input mode を later public contract でどう薄くするか
- `resolve_current_l2_source_sample_path` / accepted-set hard-coding を operational wrapper からどこまで隠蔽できるか
