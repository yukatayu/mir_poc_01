# 561 — current L2 fixed-subset-source-sample-corpus-scope-and-file-layout threshold helper mirror

## 目的

Package 88 の closeout として、
fixed-subset source-sample corpus の scope / directory / naming / non-goal minimum を
`mir-runtime` 側の inspectable manifest に narrow に mirror する。

ここで actualize するのは
**current L2 fixed-subset source-sample corpus scope-and-file-layout minimum**
であり、

- representative / fixture / source mapping matrix の拡張
- actual sample file content の widening
- parser-to-`Program` lowering の widening
- bless / regression policy の public surface 化

は固定しない。

## current cut

- manifest:
  `CurrentL2FixedSubsetSourceSampleCorpusScopeAndFileLayoutManifest`
- getter:
  `current_l2_fixed_subset_source_sample_corpus_scope_and_file_layout_manifest()`
- minimum field:
  - `scope_kind`
  - `source_cluster_refs`
  - `directory_ref`
  - `file_layout_ref`
  - `file_extension_policy`
  - `sample_id_policy`
  - `non_goal_refs`

## actualized minimum

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

- source corpus は representative prose と fixture corpus の間にある第 3 層として読む。
- initial cluster は `e1 / e2 / e3 / e4 / e21 / e23` の 6 本に留める。
- directory / flat layout / `.txt` / fixture-stem-aligned naming だけを minimum に actualize する。
- final grammar / reverse generation / verdict-stage-in-filename は non-goal に押し戻す。

## stop line

- representative / fixture / source mapping matrix の widening
- actual sample content の widening
- parser-to-`Program` lowering の widening
- bless / regression policy の public contract 化
- final parser / checker / runtime public surface

## historical package-local next reopen line

historical parser-side queue memory では、
parser-side reopen route を
**phase6 request-clause-suite publicization comparison**
に置く読みが compare-anchor memory に残る。

source-sample corpus scope / layout minimum を actualize したことで、
parser-side next package は source path policy の曖昧さを減らした状態で
request clause suite publicization を比較できるという historical rationale も保持してよい。

ただし current repo-level queue authority は `progress.md` / `tasks.md` にあり、
2026-04-30 current-line maintenance closeout 後に
この package から parser-side reopen line を promote しない。
