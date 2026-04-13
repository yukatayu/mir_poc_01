# 399 — current L2 stable-malformed-capability-second-reopen-actualization-ready public-operational-cli-concrete-shell-naming comparison

## 目的

`specs/examples/398-current-l2-stable-malformed-capability-second-reopen-actualization-ready-minimal-stable-malformed-capability-second-reopen-threshold.md`
で capability second reopen comparison の minimum を fixed した次段として、

- runtime-led thin facade first cut、Rust-side operational wrapper second gate、runtime skeleton later support cutを巻き戻さずに、public operational CLI の concrete shell naming をどこまで narrow に固定するか
- final `mir` top-level hierarchy、inventory / regression verb、support-only verb、repo-local Python helper / cargo example surface をどこまで excluded に保つか
- sample selector / host-plan path / pretty-or-json output をどこまで current L2 scoped shell concern に留めるか

を比較する。

ここで固定するのは
**current L2 stable-malformed-capability-second-reopen-actualization-ready public-operational-cli-concrete-shell-naming comparison**
であり、

- final public parser / checker / runtime API
- final `mir` product-wide CLI hierarchy
- final host/input contract
- raw FFI / engine adapter / visualizer binding

はまだ固定しない。

## scope

- current package は docs-only naming comparison に留める。
- `specs/examples/393...394` の Rust-side operational wrapper second gate と `specs/examples/395...396` の later support cut を entry evidence に使う。
- current runtime-led thin facade first cut と library-before-CLI ordering は巻き戻さない。
- repo-local Python helper、cargo example emitter、inventory / regression helper verb は public operational shell へ昇格させない。

## current 前提

current repo では次が成立している。

1. `specs/examples/371...372` により、public-side later ordering は final public parser / checker / runtime API first、public operational CLI second に固定済みである。
2. `specs/examples/389...390` により、current first later cut は `run_current_l2_source_sample` + `CurrentL2SourceSampleRunReport` の runtime-led thin facade に固定済みである。
3. `specs/examples/393...394` により、public operational CLI second gate は Rust-side operational wrapper over `run_current_l2_source_sample` に narrow に固定済みである。
4. `specs/examples/395...396` により、later support cut は `run_current_l2_runtime_skeleton` + `CurrentL2RuntimeSkeletonReport` に固定済みであり、`lower_current_l2_fixed_source_text`、parser carrier floor、repo-local helper 群は current public shell concern の外に残っている。
5. runtime test / source-sample regression current flow は `samples/current-l2/README.md` と `.docs/current-l2-source-sample-authoring-policy.md` に寄せてあり、inventory / regression は repo-local maintenance flow として separate に扱っている。

したがって current 問いは、
**current L2 で public operational CLI の concrete shell naming を固定するとしても、final `mir` hierarchy を先取りせず、current-L2 scoped shell と single operational verb に narrow に留めるのが最小か**
である。

## 比較観点

1. runtime-led thin facade first cut、operational wrapper second gate、runtime skeleton later support cutを同時に保てるか
2. inventory / regression / support-only verb を public operational shell から clean に外せるか
3. sample selector / host-plan path / output formatting だけを current shell concern に留め、final host/input contract を先取りしないで済むか
4. next line を stable malformed capability second source-backed widening actualization comparison に clean に handoff できるか

## 比較対象

### 案 1. current-L2 scoped shell `mir-current-l2 run-source-sample <sample> --host-plan <path> --format pretty|json` を docs-only first cut に置く

#### shape

```text
public_operational_cli_concrete_shell_naming = {
  actualization_kind = current_l2_scoped_docs_only_operational_shell,
  chosen_shell_refs = [
    mir-current-l2,
    run-source-sample
  ],
  operational_argument_refs = [
    sample_selector_positional_argument,
    host_plan_explicit_path_flag
  ],
  operational_output_refs = [
    format_pretty_or_json_flag
  ],
  excluded_refs = [
    final_mir_top_level_hierarchy,
    inventory_and_regression_repo_local_verbs,
    runtime_skeleton_or_lowering_support_verbs,
    repo_local_python_orchestration_helpers,
    cargo_example_emitters_and_support_modules
  ]
}
```

#### 利点

- current runtime-led thin facade と operational wrapper cut を崩さずに、human-facing shell naming だけを narrow に固定できる。
- current L2 専用 shell に留めるため、final `mir` hierarchy や final public API を premature に固めにくい。
- `sample selector + host-plan path + pretty/json format` という既存 wrapper concern と自然に対応する。

#### 欠点

- final executable 名や subcommand tree ではなく、working shell naming に留まる。
- inventory / regression helper の command 体系は current public shell からは見えない。

### 案 2. final `mir run source-sample ...` のような product-wide top-level hierarchy を先に置く

#### 利点

- 将来像は見えやすい。

#### 欠点

- final parser/checker/runtime API や host/input contract より先に shell だけが確定しやすい。
- current L2 scoped cut と later support cut の境界が弱くなる。

### 案 3. repo-local Python helper / regression verb naming を public operational shell に流用する

#### 利点

- 既存 command 群に近く、今すぐ動く感触は出しやすい。

#### 欠点

- maintenance helper と public operational shell が混ざる。
- Rust-side operational wrapper second gate の判断と衝突する。

## current judgment

current L2 で最も自然なのは、
**案 1. current-L2 scoped shell `mir-current-l2 run-source-sample <sample> --host-plan <path> --format pretty|json` を docs-only first cut に置く**
である。

理由は次の通り。

1. current package の主眼は concrete shell naming を narrow に整理することであり、runtime-led thin facade first cut や operational wrapper second gate を巻き戻して final hierarchy を先取りする必要はない。
2. `run_current_l2_source_sample` が current public-pressure entry、`run_current_l2_runtime_skeleton` が later support cut、Python helper 群が repo-local maintenance flow という split を既に採っているため、shell naming もその split を mirror する方が drift を防ぎやすい。
3. `sample selector + explicit host-plan path + pretty/json report` は current operational wrapper concern と一致しており、これだけを docs-only naming に留めるのが最小である。

## current first choice details

- current first cut の command family は `mir-current-l2` に留める。
- current first cut の operational verb は `run-source-sample` に留める。
- current shell で見せる positional input は `<sample>` に留める。
- current shell で見せる explicit option は `--host-plan <path>` と `--format pretty|json` に留める。
- final `mir` top-level hierarchy、inventory / regression verb、`run_current_l2_runtime_skeleton` / `lower_current_l2_fixed_source_text` 系 support verb、repo-local Python helper / cargo example naming は excluded bucket に残す。
- concrete shell naming は docs-only comparison に留め、actual CLI binary / parser / shell parser binding は kept-later に残す。

## next promoted line

next promoted line は、
**public-operational-cli-concrete-shell-naming-ready stable-malformed-capability-second-source-backed-widening-actualization-comparison**
に置く。

## open questions

- current-L2 scoped shell family を later actualization で `mir-current-l2` のまま保つか、`mir current-l2` の二段 hierarchy へ薄く移すか
- `--format` default を later actualization で `pretty` に寄せるか、machine-facing `json` を先にするか
- `--host-plan` を later host/input contract でどこまで implicit 化できるか
