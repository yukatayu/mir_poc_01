# 395 — current L2 public-operational-cli-second-later-gate-actualization-comparison-ready final-public-parser-checker-runtime-thin-facade-later-support-actualization comparison

## 目的

`specs/examples/394-current-l2-public-operational-cli-second-later-gate-actualization-comparison-ready-minimal-public-operational-cli-second-later-gate-threshold.md`
で public operational CLI second later gate の minimum を fixed した次段として、

- runtime-led thin facade first cut と Rust-side operational wrapper second gate を巻き戻さず、standalone parser/checker/runtime support entry をどこまで later public support として actualize するか
- `run_current_l2_runtime_skeleton` / `CurrentL2RuntimeSkeletonReport`、`lower_current_l2_fixed_source_text`、semantic/checker core、parser carrier floor をどの bucket に置くか
- sample path resolver / accepted-set / repo-local helper surface をどこまで excluded に維持するか

を比較する。

ここで固定するのは
**current L2 public-operational-cli-second-later-gate-actualization-comparison-ready final-public-parser-checker-runtime-thin-facade-later-support-actualization comparison**
であり、

- final public parser grammar
- final public parser/checker/runtime API
- final host/input contract
- public operational CLI concrete command/flag naming

はまだ固定しない。

## scope

- current package は later support actualization の narrow cut に留める。
- `specs/examples/389...390` の runtime-led thin facade first cut と `specs/examples/393...394` の CLI second gate cut を entry evidence に使う。
- `run_current_l2_source_sample` current first public cut は変えない。
- repo layout / accepted-set / repo-local helper / example surface は current support cut に混ぜない。

## current 前提

current repo では次が成立している。

1. `specs/examples/355...356` により、public-looking surface は already-public parser-free stack / crate-public but non-production tranche / repo-local helper surface の 3 bucket split に整理済みである。
2. `specs/examples/363...364` により、`run_current_l2_source_sample` は first docs-only public-pressure candidate、`run_current_l2_runtime_skeleton` / `lower_current_l2_fixed_source_text` は support-only tranche に留まる。
3. `specs/examples/389...390` により、current first later cut は `run_current_l2_source_sample` + `CurrentL2SourceSampleRunReport` の runtime-led thin library facade に固定済みである。
4. `specs/examples/393...394` により、public operational CLI second gate は Rust-side operational wrapper over `run_current_l2_source_sample` に narrow に固定済みである。
5. code anchor としては `mir_runtime::current_l2::run_current_l2_runtime_skeleton` が `Program + FixtureHostPlan + optional CurrentL2ParserBridgeInput` を受け、checker-floor summary と runtime report を束ねる current narrow support entry を already 持っている。

したがって current 問いは、
**runtime-led thin facade first cut を保ったまま、later public support の current first cut を `run_current_l2_runtime_skeleton` + `CurrentL2RuntimeSkeletonReport` に置くのが最小か**
である。

## 比較観点

1. thin facade first cut と CLI second gate を巻き戻さずに済むか
2. standalone support entry を source lowerer / path resolver / repo helper から切り離して narrow に actualize できるか
3. parser carrier floor や semantic core を hidden promotion せず、support-only / deeper-support bucket を維持できるか
4. next line を stable malformed capability second reopen actualization comparison に clean に handoff できるか

## 比較対象

### 案 1. `run_current_l2_runtime_skeleton` + `CurrentL2RuntimeSkeletonReport` を first later support cut に置く

#### shape

```text
final_public_parser_checker_runtime_thin_facade_later_support = {
  actualization_kind = current_l2_runtime_support_entry,
  chosen_support_entry = run_current_l2_runtime_skeleton,
  chosen_support_report = CurrentL2RuntimeSkeletonReport,
  explicit_input_surface = [
    Program,
    FixtureHostPlan,
    optional_CurrentL2ParserBridgeInput
  ],
  deeper_support = [
    lower_current_l2_fixed_source_text,
    static_gate_program_detailed,
    run_program_to_completion,
    FixtureHostStub_run_program,
    mir_ast_current_l2_parser_carrier_floor
  ],
  excluded = [
    run_current_l2_source_sample,
    CurrentL2SourceSampleRunReport,
    resolve_current_l2_source_sample_path,
    repo_layout_bound_accepted_sample_set,
    repo_local_python_orchestration_helpers,
    cargo_example_emitters_and_support_modules
  ]
}
```

#### 利点

- thin facade first cut と operational CLI second gate を崩さず、support entry だけを narrow に actualize できる。
- path resolver / accepted-set / repo-local helper surface を excluded に残したまま、standalone support surface を切れる。
- existing runtime skeleton tests で support cut を ratchet しやすい。

#### 欠点

- `Program` / `FixtureHostPlan` / optional parser bridge input という explicit support input は still thin ではない。

### 案 2. `lower_current_l2_fixed_source_text` を先に later support cut に置く

#### 利点

- text-based support surface は見えやすい。

#### 欠点

- partial parser surface promotion を早めやすい。
- source policy / sample policy / host-plan decoupling の問題を current package に持ち込みやすい。

### 案 3. `run_current_l2_runtime_skeleton` と `lower_current_l2_fixed_source_text` の layered pair を一括で later support cut に置く

#### 利点

- parser-support と runtime-support を一度に見せられる。

#### 欠点

- support cut が厚くなり、final public parser/checker/runtime API を premature に先取りしやすい。
- thin facade first cutとの差が弱くなる。

## current judgment

current L2 で最も自然なのは、
**案 1. `run_current_l2_runtime_skeleton` + `CurrentL2RuntimeSkeletonReport` を first later support cut に置く**
である。

理由は次の通り。

1. current package の目的は thin facade first cut の後段にある standalone support entry を narrow に切ることであり、runtime skeleton は既に checker-floor summary と runtime report を束ねる最小 support cut を持っている。
2. `lower_current_l2_fixed_source_text` を先に昇格させると partial parser surface と sample policy coupling を前に出しやすい。
3. `run_current_l2_source_sample` は first public cut、CLI second gate は operational wrapper cut として既に separate に整理済みであり、support package でもその ordering を保つ必要がある。

## current first choice details

- final public parser/checker/runtime thin-facade later support の current first cut は `run_current_l2_runtime_skeleton` と `CurrentL2RuntimeSkeletonReport` に置く。
- explicit input surface は `Program`、`FixtureHostPlan`、optional `CurrentL2ParserBridgeInput` に留める。
- `lower_current_l2_fixed_source_text`、semantic/checker core、parser carrier floorは deeper-support bucket に残す。
- `run_current_l2_source_sample` と `CurrentL2SourceSampleRunReport` は first public cut に据え置き、later support cutへは含めない。
- `resolve_current_l2_source_sample_path`、accepted-set hard-coding、repo-local Python helper、cargo example emitter / support module は excluded bucket に残す。
- focused ratchet として runtime skeleton test で runtime fixture direct path と static-stop direct path を確認する。

## next promoted line

next promoted line は、
**final-public-parser-checker-runtime-thin-facade-later-support-actualization-ready stable-malformed-capability-second-reopen-actualization-comparison**
に置く。

## open questions

- `Program` / `FixtureHostPlan` / optional parser bridge input の explicit support input をどこまで public-facing later support に残すか
- `lower_current_l2_fixed_source_text` を later support へ昇格させる必要があるか
- thin facade first cut と support cut の間で host/input contract をどう薄くするか
