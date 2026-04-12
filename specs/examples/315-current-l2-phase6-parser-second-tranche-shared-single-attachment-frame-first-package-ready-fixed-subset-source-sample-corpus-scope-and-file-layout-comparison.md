# 315 — current L2 phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready fixed-subset-source-sample-corpus-scope-and-file-layout comparison

## 目的

`specs/examples/313-current-l2-phase6-parser-side-follow-up-package-sequencing-ready-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-comparison.md`
と
`specs/examples/314-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready-minimal-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-threshold.md`
で shared single attachment frame first package を actualize した次段として、

- fixed-subset source-sample corpus をどの scope / directory / file layout で置くか
- representative prose / fixture corpus / source sample の 3 層をどう分けるか
- final grammar や fixture reverse-generation とどう切り分けるか

を比較する。

ここで固定するのは
**current L2 phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready fixed-subset-source-sample-corpus-scope-and-file-layout comparison**
であり、

- representative / fixture / source mapping matrix
- actual parser-to-`Program` lowering
- syntax-backed runner
- bless / regression policy

はまだ固定しない。

## scope

- entry criteria は `specs/examples/311...314` で fixed 済みの parser-side follow-up line とする。
- root source は `specs/examples/00...02`、`plan/08`、`plan/15`、fixture corpus path、stage parser spike tests inline source を主に扱う。
- actual source sample content authoring までは進まないが、repo path と naming policy は actualize してよい。

## current 前提

current repo では次が成立している。

1. representative programs は `specs/examples/00` と `01` にあり、説明用であって final grammar 正本ではない。
2. machine-check baseline は `crates/mir-ast/tests/fixtures/current-l2/` にあり、fixture JSON は parser-free / detached loop / static gate / interpreter の current正本である。
3. parser spike の source text evidence は各 test file に inline で散っており、dedicated repo-tracked source corpus directory はまだない。
4. current near-term line は source-sample corpus を第 3 層として置き、mapping / lowering / runner / ladder へ進む discipline を必要としている。

したがって current 問いは、
**fixed-subset source-sample corpus を repo-root `samples/current-l2/` flat `.txt` layer として切るのが自然か**
である。

## 比較観点

1. representative prose / fixture corpus と責務分離できるか
2. parser / lowering / runner / formal ladder の cross-crate path に自然に使えるか
3. final grammar を premature に固定しないか
4. sample ID と fixture stem の対応を後続 task へ自然に渡せるか

## 比較対象

### 案 1. repo-root `samples/current-l2/` flat `.txt` layer を置く

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

- representative prose と fixture corpus の間に第 3 層を明確に置ける。
- parser / lowering / runner / formal ladder のどの crate からも path を共有しやすい。
- final grammar を `.mir` extension で既成事実化しなくて済む。

#### 欠点

- top-level directory が 1 つ増える。

### 案 2. `crates/mir-ast/tests/source-samples/current-l2/` に置く

#### 利点

- parser spike test に近い。

#### 欠点

- parser test 専用 layer に見えやすく、later lowering / runner / formal ladder の cross-crate path として弱い。
- fixture corpus と responsibilities が近すぎる。

### 案 3. representative prose や fixture JSON をそのまま source corpus とみなす

#### 利点

- 新 directory を増やさずに見える。

#### 欠点

- representative prose と fixture corpus の責務分離が崩れる。
- final grammar / reverse-generation / explanation-first prose が混線する。

## current judgment

current L2 で最も自然なのは、
**案 1. repo-root `samples/current-l2/` flat `.txt` layer を置く**
である。

理由は次の通り。

1. source-sample corpus は representative prose でも fixture corpus でもない第 3 層であることを path で明示できる。
2. later lowering / runner / formal ladder は cross-crate path になるため、repo-root の方が自然である。
3. `.txt` と flat layout に留めることで final grammar や verdict/stage naming を先取りしない。

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

current source-sample corpus scope / file layout judgment が示すのは、次の 6 点だけである。

1. source corpus は repo-root `samples/current-l2/` に置く
2. current layout は flat / one-file-per-sample にする
3. extension は `.txt` に留める
4. sample stem は fixture stem / `fixture_id` と揃える
5. initial cluster は `e1` / `e2` / `e3` / `e4` / `e21` / `e23` とする
6. final grammar / reverse-generation / verdict/stage-in-filename は current non-goal にする

## next promoted line

next promoted line は、
**fixed-subset-source-sample-corpus-scope-and-file-layout-ready representative-fixture-source-mapping-matrix comparison**
に置く。

## open questions

- initial cluster にどの representative example rows を 1 対 1 で結び付けるか
- `samples/current-l2/README.md` を source corpus policy の実体 path としてどこまで使うか
- mapping matrix で `source_example_id` と `fixture_id` の両方をどう mirror するか
