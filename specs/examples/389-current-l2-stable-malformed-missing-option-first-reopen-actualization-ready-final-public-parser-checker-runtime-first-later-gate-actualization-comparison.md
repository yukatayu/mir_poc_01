# 389 — current L2 stable-malformed-missing-option-first-reopen-actualization-ready final-public-parser-checker-runtime-first-later-gate-actualization comparison

## 目的

`specs/examples/388-current-l2-stable-malformed-missing-option-first-reopen-actualization-ready-minimal-stable-malformed-missing-option-first-reopen-threshold.md`
で missing-option first reopen actualization の minimum を fixed した次段として、

- library-side final public parser / checker / runtime API の first later gate actualization をどの symbol cut から narrow に始めるか
- `run_current_l2_source_sample` current gate を entry に保ったまま、parser/checker/runtime の見せ方をどこまで report-nested に留めるか
- public operational CLI second later gate と repo-local helper / example surface を混ぜずに current first actualization cut をどこへ置くか

を比較する。

ここで固定するのは
**current L2 stable-malformed-missing-option-first-reopen-actualization-ready final-public-parser-checker-runtime-first-later-gate-actualization comparison**
であり、

- final public parser grammar
- final public operational CLI
- public theorem / model-check / checker migration
- repo-layout 非依存の final host/input contract

はまだ固定しない。

## scope

- current package は docs-only actualization comparison に留める。
- `specs/examples/355...356` の public surface inventory、`specs/examples/363...364` の public-pressure gate、`specs/examples/371...372` の library-before-CLI later ordering を entry evidence に使う。
- `run_current_l2_source_sample` の current/report shape は変えない。
- crate-level `pub` visibility を final public contract と同一視しない。

## current 前提

current repo では次が成立している。

1. `specs/examples/355...356` により、public-looking surface は
   - already-public parser-free helper stack
   - crate-public but non-production parser/checker/runtime tranche
   - repo-local helper / example surface
   の 3 bucket split に inventory 済みである。
2. `specs/examples/363...364` により、later public pressure の first docs-only candidate は `run_current_l2_source_sample` に narrow に置かれ、`run_current_l2_runtime_skeleton` / `lower_current_l2_fixed_source_text` は tranche-internal support、`resolve_current_l2_source_sample_path` と accepted-set hard-coding は excluded bucket に留めている。
3. `specs/examples/371...372` により、public-side later ordering は final public parser / checker / runtime API first、public operational CLI second に固定済みである。
4. `specs/examples/385...386` により、host-facing integration は privileged `stdin/stdout` ではなく docs-first capability-scoped I/O / adapter boundary に留めており、current package で raw host / FFI / engine adapter line を actual public contract に混ぜない。

したがって current 問いは、
**`run_current_l2_source_sample` current gate を保ったまま、runtime-led thin library facade を first actualization cut に置くのが最小か**
である。

## 比較観点

1. `run_current_l2_source_sample` current gate と library-before-CLI later orderingを保てるか
2. parser/checker/runtime の current visible data を standalone public entry へ premature に昇格させず、report-nested shape に留められるか
3. `resolve_current_l2_source_sample_path`、accepted sample set、repo-local helper 群を final public contract 候補から外せるか
4. package close 後の repo-level next line を malformed-side widening reserve と CLI second gate に clean に handoff できるか

## 比較対象

### 案 1. `run_current_l2_source_sample` + report-nested parser/checker/runtime carriers を first actualization cut に置く

#### shape

```text
final_public_parser_checker_runtime_first_later_gate = {
  actualization_kind = current_l2_runtime_led_thin_library_facade,
  runtime_entry = run_current_l2_source_sample,
  public_report = CurrentL2SourceSampleRunReport,
  nested_parser_checker_runtime = [
    CurrentL2LoweredSourceProgram,
    CurrentL2RuntimeSkeletonReport,
    CurrentL2CheckerFloorReport,
    RunReport
  ],
  support_only = [
    run_current_l2_runtime_skeleton,
    lower_current_l2_fixed_source_text,
    static_gate_program_detailed,
    run_program_to_completion,
    FixtureHostStub::run_program,
    mir_ast_current_l2_parser_carrier_floor
  ],
  excluded = [
    resolve_current_l2_source_sample_path,
    repo_layout_bound_accepted_sample_set,
    repo_local_python_orchestration_helpers,
    example_emitters_and_support_modules
  ]
}
```

#### 利点

- current gate を壊さず、end-to-end library-side thin facade を 1 本に絞れる。
- parser/checker/runtime の details は nested report carrier に留め、standalone entry promotion を避けられる。
- `resolve_current_l2_source_sample_path` や accepted-set hard-coding を final contract 候補から外したままにできる。

#### 欠点

- parser/checker/runtime API を 3 本の independent public entry としてはまだ切らない。

### 案 2. `run_current_l2_source_sample` / `run_current_l2_runtime_skeleton` / `lower_current_l2_fixed_source_text` の layered trio を一括で first actualization cut に置く

#### 利点

- parser / checker / runtime の層が見えやすい。

#### 欠点

- support-only tranche を premature に public facade と誤読しやすい。
- explicit `FixtureHostPlan` coupling や parser carrier floor を final contract へ近づけやすい。

### 案 3. `mir_runtime::current_l2` や `mir_ast::current_l2` を module-wide first candidate に置く

#### 利点

- docs の記述は単純になる。

#### 欠点

- `pub` visibility を final public contract と同一視しやすい。
- path resolver、accepted-set、stage parser helpers まで hidden promotion しやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. `run_current_l2_source_sample` + report-nested parser/checker/runtime carriers を first actualization cut に置く**
である。

理由は次の通り。

1. `run_current_l2_source_sample` は current compile-ready tranche の中で最も narrow に parser carrier / checker floor / runtime report を end-to-end で跨ぐ entry であり、already-public parser-free stack と public operational CLI second gate の間に置く first later cut として最も素直である。
2. `run_current_l2_runtime_skeleton` と `lower_current_l2_fixed_source_text` は current docs では support-only tranche と読む方が guard が強く、standalone public entry に上げるのは早い。
3. `resolve_current_l2_source_sample_path`、accepted-set hard-coding、repo-local Python helpers、example/support modules を excluded bucket に残さないと、repo layout / host-plan coupling が final contract 候補に混ざりやすい。

## current first choice details

- runtime-led thin facade の first public later cut は `mir_runtime::current_l2::run_current_l2_source_sample` に置く。
- public report shape は `CurrentL2SourceSampleRunReport` に置き、parser/checker/runtime detail は `CurrentL2LoweredSourceProgram` / `CurrentL2RuntimeSkeletonReport` / `CurrentL2CheckerFloorReport` / `RunReport` の nested carrier として扱う。
- `run_current_l2_runtime_skeleton` と `lower_current_l2_fixed_source_text` は current tranche-internal support に留める。
- `static_gate_program_detailed`、`run_program_to_completion`、`FixtureHostStub::run_program`、`mir_ast::current_l2` parser carrier helpersは support-only / non-final bucket に留める。
- `resolve_current_l2_source_sample_path`、accepted-set hard-coding、repo-local Python orchestration helper、example/support surface は excluded bucket に残す。
- public operational CLI second later gate、repo-layout 非依存の final input contract、public theorem/model-check/checker migration は kept-later に残す。

## next promoted line

next promoted line は、
**final-public-parser-checker-runtime-first-later-gate-actualization-ready stable-malformed-missing-option-first-source-backed-widening-actualization comparison**
に置く。

## open questions

- runtime-led thin facade actualization の次段で、standalone parser/checker support entry を later public support として reopen する必要があるか
- explicit `FixtureHostPlan` coupling を library-side final contract からどう薄くするか
- public operational CLI second gate を runtime-led thin facade とどう分けて actualize するか
