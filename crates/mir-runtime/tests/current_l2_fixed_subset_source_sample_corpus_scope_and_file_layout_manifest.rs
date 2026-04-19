use mir_runtime::current_l2::current_l2_fixed_subset_source_sample_corpus_scope_and_file_layout_manifest;

#[test]
fn current_l2_fixed_subset_source_sample_corpus_scope_and_file_layout_manifest_keeps_minimum_cut() {
    let manifest = current_l2_fixed_subset_source_sample_corpus_scope_and_file_layout_manifest();

    assert_eq!(
        manifest.scope_kind,
        "current_l2_fixed_subset_source_sample_corpus_scope"
    );
    assert_eq!(
        manifest.source_cluster_refs,
        &[
            "e1_place_atomic_cut",
            "e2_try_fallback",
            "e3_option_admit_chain",
            "e4_malformed_lineage",
            "e21_try_atomic_cut_frontier",
            "e23_malformed_try_fallback_missing_fallback_body",
        ]
    );
    assert_eq!(
        manifest.directory_ref,
        "repo_root_samples_current_l2_directory"
    );
    assert_eq!(manifest.file_layout_ref, "flat_one_file_per_sample_layout");
    assert_eq!(
        manifest.file_extension_policy,
        "neutral_text_dot_txt_until_final_grammar"
    );
    assert_eq!(
        manifest.sample_id_policy,
        "fixture_stem_aligned_kebab_case_sample_id"
    );
    assert_eq!(
        manifest.non_goal_refs,
        &[
            "not_final_parser_grammar",
            "not_fixture_reverse_generation",
            "not_verdict_or_stage_in_filename",
        ]
    );
}
