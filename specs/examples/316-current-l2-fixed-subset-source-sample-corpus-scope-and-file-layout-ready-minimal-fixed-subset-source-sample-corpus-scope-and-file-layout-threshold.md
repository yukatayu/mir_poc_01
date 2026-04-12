# 316 — current L2 fixed-subset-source-sample-corpus-scope-and-file-layout-ready minimal-fixed-subset-source-sample-corpus-scope-and-file-layout threshold

## 目的

`specs/examples/315-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready-fixed-subset-source-sample-corpus-scope-and-file-layout-comparison.md`
で fixed-subset source-sample corpus scope / file layout の current first choice を
repo-root `samples/current-l2/` flat `.txt` layer に置く判断を fixed した次段として、

- source-sample corpus scope / file layout の minimum をどこまでに留めるか
- scope / directory / extension / naming / non-goal を minimum にどう反映するか
- next mainline を mapping matrix へどう渡すか

を比較する。

ここで固定するのは
**current L2 fixed-subset-source-sample-corpus-scope-and-file-layout-ready minimal-fixed-subset-source-sample-corpus-scope-and-file-layout threshold**
であり、

- representative / fixture / source mapping matrix の shape
- actual sample file content
- parser-to-`Program` lowering の shape

はまだ固定しない。

## 比較観点

1. source corpus の第 3 層性を minimum に反映できるか
2. initial cluster / path / extension / naming / non-goal を minimum に残せるか
3. mapping matrix へ進む entry criteria を minimum に残せるか

## 比較対象

### 案 1. `directory_ref` だけを持つ

#### shape

```text
fixed_subset_source_sample_corpus_scope_and_file_layout = {
  directory_ref = repo_root_samples_current_l2_directory
}
```

#### 利点

- 軽い

#### 欠点

- cluster / extension / naming / non-goal が minimum に現れない
- mapping matrix へ渡す information が弱い

### 案 2. `scope_kind + source_cluster_refs + directory_ref + file_layout_ref + file_extension_policy + sample_id_policy + non_goal_refs` を持つ

#### shape

```text
fixed_subset_source_sample_corpus_scope_and_file_layout = {
  scope_kind = current_l2_fixed_subset_source_sample_corpus_scope,
  source_cluster_refs = [
    e1_place_atomic_cut,
    e2_try_fallback,
    e3_option_admit_chain,
    e4_malformed_lineage,
    e21_try_atomic_cut_frontier,
    e23_malformed_try_fallback_missing_fallback_body
  ],
  directory_ref = repo_root_samples_current_l2_directory,
  file_layout_ref = flat_one_file_per_sample_layout,
  file_extension_policy = neutral_text_dot_txt_until_final_grammar,
  sample_id_policy = fixture_stem_aligned_kebab_case_sample_id,
  non_goal_refs = [
    not_final_parser_grammar,
    not_fixture_reverse_generation,
    not_verdict_or_stage_in_filename
  ]
}
```

#### 利点

- source corpus の第 3 層性と initial cluster を minimum に残せる
- directory / extension / naming / non-goal が明示される
- mapping matrix へ進む entry criteria が読みやすい

#### 欠点

- 案 1 より field は増える

### 案 3. mapping matrix や lowering policy まで minimum に含める

#### 利点

- 次 task との繋がりは見えやすい

#### 欠点

- scope / file layout threshold ではなく next package threshold を先取りする
- task boundary が混線する

## current judgment

current L2 で最も自然なのは、
**案 2. `scope_kind + source_cluster_refs + directory_ref + file_layout_ref + file_extension_policy + sample_id_policy + non_goal_refs` を持つ**
である。

理由は次の通り。

1. current task は source corpus の actual content ではなく、scope / layout threshold を固定する task である。
2. path だけでなく initial cluster と naming / non-goal を minimum に残さないと、mapping matrix へ渡す基準が弱い。
3. final grammar / reverse-generation / verdict/stage naming を non-goal として押し戻す必要がある。

## current first choice shape

```text
fixed_subset_source_sample_corpus_scope_and_file_layout = {
  scope_kind = current_l2_fixed_subset_source_sample_corpus_scope,
  source_cluster_refs = [
    e1_place_atomic_cut,
    e2_try_fallback,
    e3_option_admit_chain,
    e4_malformed_lineage,
    e21_try_atomic_cut_frontier,
    e23_malformed_try_fallback_missing_fallback_body
  ],
  directory_ref = repo_root_samples_current_l2_directory,
  file_layout_ref = flat_one_file_per_sample_layout,
  file_extension_policy = neutral_text_dot_txt_until_final_grammar,
  sample_id_policy = fixture_stem_aligned_kebab_case_sample_id,
  non_goal_refs = [
    not_final_parser_grammar,
    not_fixture_reverse_generation,
    not_verdict_or_stage_in_filename
  ]
}
```

## practical reading

current minimal source-sample corpus scope / file layout が示すのは、

- source corpus を第 3 層として repo-root `samples/current-l2/` に置く
- initial cluster を `e1` / `e2` / `e3` / `e4` / `e21` / `e23` に留める
- layout は flat `.txt` / fixture-stem-aligned naming にする
- final grammar / reverse-generation / verdict/stage-in-filename は current non-goal にする

という最小 cut である。

## next promoted line

next promoted line は、
**fixed-subset-source-sample-corpus-scope-and-file-layout-ready representative-fixture-source-mapping-matrix comparison**
に置く。

## open questions

- mapping matrix で representative prose と `source_example_id` をどう結ぶか
- initial cluster 以外の later sample をどの条件で追加するか
- source corpus authoring / bless policy をいつ template 化するか
