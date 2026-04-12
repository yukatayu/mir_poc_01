# 355 — current L2 stable-static-malformed-post-contrast-sequencing-ready parser-checker-runtime-public-surface-inventory comparison

## 目的

`specs/examples/354-current-l2-stable-static-malformed-post-contrast-sequencing-ready-minimal-stable-static-malformed-post-contrast-sequencing-threshold.md`
で stable static malformed post-contrast sequencing の minimum を fixed した次段として、

- current repo で既に public behavior を持つ parser-free helper stack はどこか
- crate から見えているが still non-production と読む parser / checker / runtime surface はどこか
- repo-local helper / example emitter / support layer を later public operational surface とどう切り分けるか

を inventory 化する。

ここで固定するのは
**current L2 stable-static-malformed-post-contrast-sequencing-ready parser-checker-runtime-public-surface-inventory comparison**
であり、

- final public parser API
- final public checker / runtime API
- public operational CLI
- public theorem / model-check / checker migration

はまだ固定しない。

## scope

- current inventory は `mir-ast` / `mir-semantics` / `mir-runtime` と repo-root helper 群に scoped する。
- Rust の `pub` visibility と final public operational contract を同一視しない。
- parser-free helper stack の existing public behavior は壊さず、Phase 6 current tranche の later promotion pressure だけを整理する。

## current 前提

current repo では次が成立している。

1. parser-free helper stack には `mir-semantics` public library surface として bundle / discovery / selection / profiled execution の current public behavior が already ある。
2. `mir_ast::current_l2`、`mir_runtime::current_l2`、`mir-semantics` program-level entry、`FixtureHostStub::run_program` は crate から見えていても、current docs では non-production minimal tranche と読んでいる。
3. example/support helper と repo-root Python helper は operational aid として使えても、public crate contract とは別に扱う current docs が already ある。

したがって current 問いは、
**already-public parser-free stack、crate-public but non-production parser/checker/runtime tranche、repo-local helper surface の 3 bucket を分けた inventory にするのが最小か**
である。

## 比較観点

1. parser-free 既存 public behavior を accidental に巻き戻さないか
2. `pub` visibility だけで current tranche を final public API と誤読させないか
3. repo-local helper / example emitter / support layer を public crate contract と切り分けられるか
4. next repo-level line を Mirrorea / shared-space docs-first re-entry へ handoff できるか

## 比較対象

### 案 1. current crate-visible symbol をすべて single public operational surface として inventory する

#### 利点

- list は単純になる。

#### 欠点

- parser-free existing public behavior と current non-production tranche が混ざる。
- `examples/support` や repo-local helper を hidden に public contract へ近づけやすい。
- later actual public API design を先取りしやすい。

### 案 2. 3 bucket inventory に分ける

#### shape

```text
public_surface_inventory = {
  already_public_parser_free_refs = [
    parser_free_bundle_batch_selection_profile_stack
  ],
  crate_public_nonproduction_refs = [
    mir_ast_current_l2,
    mir_runtime_current_l2,
    program_level_checker_runtime_entry
  ],
  repo_local_helper_refs = [
    example_emitters_and_support_modules,
    python_repo_local_orchestration_helpers
  ],
  kept_later_refs = [
    final_public_parser_checker_runtime_api,
    public_operational_cli,
    theorem_model_check_public_checker_migration
  ]
}
```

#### 利点

- parser-free existing public behavior を stable public bucket として維持できる。
- current compile-ready tranche は crate-visible でも non-production と読める。
- repo-local helper / example emitter を public crate contract に混ぜずに済む。

#### 欠点

- inventory row は少し増える。

### 案 3. parser-free stack と current compile-ready tranche を public / non-public の 2 bucket だけに分ける

#### 利点

- bucket 数は少ない。

#### 欠点

- repo-local helper / example emitter / support layer の扱いが曖昧になる。
- later public CLI / exporter line の reopen timing が見えにくい。

## current judgment

current L2 で最も自然なのは、
**案 2. 3 bucket inventory に分ける**
である。

理由は次の通り。

1. parser-free helper stack には already-public behavior があり、current tranche の visibility と混ぜるべきではない。
2. `mir_ast::current_l2`、`mir_runtime::current_l2`、program-level checker/runtime entry は current docs で non-production thin tranche として使っており、`pub` visibility だけで final public contract と誤読させたくない。
3. example emitter / support helper / Python orchestration は repo-local operational aid であり、crate public surface とは別 bucket に残す方が guard が強い。

## current first choice details

- already-public parser-free bucket には、`run_bundle`、`discover_bundles_in_directory`、`select_bundles_from_request`、`run_directory_profiled`、`run_directory_named_profile` を中心とする bundle / discovery / selection / profile helper stack を置く。
- crate-public but non-production bucket には、`mir_ast::current_l2`、`mir_runtime::current_l2`、`static_gate_program_detailed`、`DirectStyleEvaluator::from_program`、`run_program_to_completion`、`FixtureHostStub::run_program` を置く。
- repo-local helper bucket には、example emitter / support modules、`scripts/current_l2_detached_loop.py`、`scripts/current_l2_source_sample_regression.py` を含む orchestration helper 群を置く。
- final public parser / checker / runtime API、public runner / exporter CLI、theorem/model-check/public-checker migration は kept-later に残す。

## next promoted line

next promoted line は、
**parser-checker-runtime-public-surface-inventory-ready Mirrorea-shared-space-docs-first-re-entry comparison**
に置く。

## open questions

- current compile-ready tranche を later public API actualization へ送る順序を parser / runtime / theorem-side のどれから切るか
- repo-local helper のうち、将来 public operational CLI へ昇格するものがあるか
- parser-free existing public behavior と future public checker/runtime API の namespace をどう並べるか
