# 356 — current L2 parser-checker-runtime-public-surface-inventory-ready minimal-parser-checker-runtime-public-surface-inventory threshold

## 目的

`specs/examples/355-current-l2-stable-static-malformed-post-contrast-sequencing-ready-parser-checker-runtime-public-surface-inventory-comparison.md`
で public surface inventory の current first choice を fixed した次段として、

- inventory minimum をどこまでに留めるか
- already-public parser-free bucket、crate-public but non-production bucket、repo-local helper bucket の境界をどう残すか
- next repo-level line と kept-later public API line をどう handoff するか

を比較する。

ここで固定するのは
**current L2 parser-checker-runtime-public-surface-inventory-ready minimal-parser-checker-runtime-public-surface-inventory threshold**
であり、

- final public parser / checker / runtime API shape
- concrete public runner / exporter CLI
- theorem/model-check/public-checker migration actualization

はまだ固定しない。

## 比較観点

1. inventory close を minimum に保てるか
2. 3 bucket split を lossless に残せるか
3. `pub visibility != final public contract` の guard を残せるか
4. next repo-level line を Mirrorea / shared-space docs-first re-entry へ handoff できるか

## 比較対象

### 案 1. inventory title と bucket 名だけを残す

#### 利点

- 軽い。

#### 欠点

- concrete bucket contents と guard が抜け、later actualization threshold が弱い。

### 案 2. `inventory_kind + already_public_parser_free_refs + crate_public_nonproduction_refs + repo_local_helper_refs + guard_refs + kept_later_refs` を持つ

#### 利点

- 3 bucket split と kept-later line を lossless に残せる。
- parser-free existing public behavior を維持しつつ、current tranche の promotion guard を明示できる。
- next repo-level line と later public actualization line を両方 handoff できる。

#### 欠点

- 案 1 より fields は増える。

### 案 3. future public API candidate や promoted namespace まで minimum に含める

#### 利点

- later design は見えやすい。

#### 欠点

- threshold ではなく later public API design を先取りする。

## current judgment

current L2 で最も自然なのは、
**案 2. `inventory_kind + already_public_parser_free_refs + crate_public_nonproduction_refs + repo_local_helper_refs + guard_refs + kept_later_refs` を持つ**
である。

理由は次の通り。

1. current package の本体は inventory close であり、actual promotion や namespace fixing は次段へ残すべきである。
2. 3 bucket split と guard が見えないと、`pub` visibility を final public contract と誤読しやすい。
3. next repo-level line は Mirrorea / shared-space docs-first re-entry だが、later public API / CLI line も inventory 側に kept-later として残す必要がある。

## current first choice shape

```text
parser_checker_runtime_public_surface_inventory = {
  inventory_kind = current_l2_public_operational_surface_inventory,
  already_public_parser_free_refs = [
    bundle_batch_selection_profile_stack
  ],
  crate_public_nonproduction_refs = [
    mir_ast_current_l2,
    mir_runtime_current_l2,
    program_level_checker_runtime_entry
  ],
  repo_local_helper_refs = [
    example_emitters_and_support_modules,
    repo_local_python_orchestration_helpers
  ],
  guard_refs = [
    keep_parser_free_existing_public_behavior_stable,
    do_not_promote_by_pub_visibility_only,
    keep_examples_support_and_scripts_outside_final_public_contract
  ],
  kept_later_refs = [
    mirrorea_shared_space_docs_first_reentry,
    model_check_public_checker_second_reserve_inventory,
    stable_static_edge_pair_first_reopen,
    final_public_parser_checker_runtime_api,
    public_operational_cli
  ]
}
```

## practical reading

current minimal public surface inventory が示すのは、

- parser-free helper stack には already-public behavior がある
- current compile-ready parser / checker / runtime tranche は crate-visible でも non-production と読む
- example emitter / support layer と repo-local helper は public crate contract に混ぜない
- repo-level next line は Mirrorea / shared-space docs-first re-entry へ進む

という最小 cut である。

## next promoted line

next promoted line は、
**parser-checker-runtime-public-surface-inventory-ready Mirrorea-shared-space-docs-first-re-entry comparison**
に置く。

## open questions

- current compile-ready tranche を later public API actualization へ送るときの first sub-cut
- public runner / exporter CLI を repo-local helper から切り出す timing
- theorem/model-check/public-checker migration と public parser/runtime API のどちらを先に reopen するか
