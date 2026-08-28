use crate::sys5_local_slice::{
    Sys5CutCorruptionKind, Sys5LocalAdmissionRequest, Sys5LocalCut, Sys5LocalCutPatchErrorKind,
    Sys5LocalPatchCandidate, Sys5LocalRuntimeProfile, Sys5PatchDiagnosticKind, Sys5PatchVerdict,
    Sys5RelationBootstrapPolicy, Sys5SourceInput, Sys5VerticalAction, Sys5VerticalDiagnosticKind,
    Sys5VerticalSliceRuntime, build_project,
};

const SYS5_CUT_PATCH_PATH: &str = "tests/inline/sys5_local_cut_patch_source.mir";

const SYS5_CUT_PATCH_SOURCE: &str = r#"
module Mirrorea.Sys5.LocalCutPatch

locus WorldAuthority
locus ParticipantA
locus ParticipantB
locus ViewerC
principal self
principal target
type Player
type Bird

state avatar[id: Player] at WorldAuthority {
  hp: Int
  atk: Int
  visible observer_safe fields (hp)
}

state participant_input[id: Player] at ParticipantA {
  focus: Int
  visible observer_safe fields (focus)
}

state bird_pose[id: Bird] at ParticipantB {
  x: Int
  y: Int
  visible observer_safe fields (x, y)
}

Role[self] at ParticipantA {
  when init_avatar_hp() fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at WorldAuthority {
      avatar[self].hp = 21
    }
  }

  when init_avatar_atk() fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at WorldAuthority {
      avatar[self].atk = 5
    }
  }

  when attack(target: Player) fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at WorldAuthority {
      avatar[target].hp = avatar[target].hp - avatar[self].atk
    }
  }
}

Role[self] at WorldAuthority {
  when init_focus() fails (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {
    at ParticipantA {
      participant_input[self].focus = 10
    }
  }
}

relation bird_follow at ParticipantB {
  subject bird: Bird
  primary participant_a_shoulder at ParticipantA epoch membership_epoch transform translate(0, 0)
  fallback participant_b_shoulder at ParticipantB epoch local_epoch transform identity
  bind frontier bird_follow_frontier
  publish relation
  project at ViewerC local
}

designated evaluate WorldAuthority on tick world_tick publish result = participant_input[self].focus + 1
designated consume WorldAuthority.result at ViewerC

with auth MembershipAuth

verify finite_refinement
"#;

fn source_declared_request() -> Sys5LocalAdmissionRequest {
    Sys5LocalAdmissionRequest::source_declared(
        "self",
        "WorldAuthority",
        "epoch:sys5-cut-patch-world",
        "incarnation:self:WorldAuthority:epoch:sys5-cut-patch-world",
        Sys5LocalRuntimeProfile::St,
    )
    .with_source_declared_membership(
        "self",
        "ParticipantA",
        "epoch:sys5-cut-patch-a",
        "incarnation:self:ParticipantA:epoch:sys5-cut-patch-a",
    )
    .with_source_declared_membership(
        "self",
        "ParticipantB",
        "epoch:sys5-cut-patch-b",
        "incarnation:self:ParticipantB:epoch:sys5-cut-patch-b",
    )
    .with_source_declared_membership(
        "self",
        "ViewerC",
        "epoch:sys5-cut-patch-c",
        "incarnation:self:ViewerC:epoch:sys5-cut-patch-c",
    )
    .with_relation_bootstrap_policy(Sys5RelationBootstrapPolicy::FreshAtAdmission)
    .with_auth_discharge("MembershipAuth")
    .with_optional_verification_discharge("finite_refinement")
}

fn prepared_source(source: impl Into<String>) -> crate::sys5_local_slice::Sys5PreparedAdmission {
    build_project(Sys5SourceInput::inline(SYS5_CUT_PATCH_PATH, source.into()))
        .expect("ordinary SYS-5 cut/patch source must check and project")
        .prepare_finite_admission(source_declared_request())
        .expect("ordinary SYS-5 cut/patch source must admit through sealed M9")
}

fn vertical_runtime(source: impl Into<String>) -> Sys5VerticalSliceRuntime {
    prepared_source(source)
        .start_vertical_slice_runtime()
        .expect("ordinary SYS-5 cut/patch source starts through generated SYS-4 endpoints")
}

fn designated_plus_two_source() -> String {
    SYS5_CUT_PATCH_SOURCE.replace(
        "participant_input[self].focus + 1",
        "participant_input[self].focus + 2",
    )
}

fn owner_rmw_plus_one_source() -> String {
    SYS5_CUT_PATCH_SOURCE.replace(
        "avatar[target].hp = avatar[target].hp - avatar[self].atk",
        "avatar[target].hp = avatar[target].hp + 1",
    )
}

fn source_first_patch_candidate(
    runtime: &Sys5VerticalSliceRuntime,
    patch_id: &str,
    source: String,
) -> Sys5LocalPatchCandidate {
    let project = build_project(Sys5SourceInput::inline(SYS5_CUT_PATCH_PATH, source))
        .expect("patch source must be ordinary Surface checked/projected first");
    let admission = project
        .prepare_finite_admission(source_declared_request())
        .expect("patch source must be fully M9 admitted before candidate wrapping");
    let candidate = Sys5LocalPatchCandidate::from_source_project_and_admission(
        patch_id, runtime, project, admission,
    )
    .expect("SYS-5 patch candidate wrapper derives base frontier from the live runtime");
    assert!(
        candidate
            .boundary_inspection()
            .caller_supplied_no_core_authority_or_frontier()
    );
    assert!(candidate.admission_summary().is_complete_for_projection());
    assert_observer_safe_no_raw_material(&format!("{candidate:?}"));
    candidate
}

#[test]
fn vertical_cut_restore_preserves_joined_prefix_and_semantic_state() {
    let project = build_project(Sys5SourceInput::inline(
        SYS5_CUT_PATCH_PATH,
        SYS5_CUT_PATCH_SOURCE,
    ))
    .expect("ordinary SYS-5 source checks and projects");
    let checked_program_identity = project.checked_program_identity_ref().to_string();
    let prepared = project
        .prepare_finite_admission(source_declared_request())
        .expect("source-declared admission seals the complete finite inventory");
    let admission_identity = prepared
        .observer_safe_admission_summary()
        .sealed_inventory_attestation_ref()
        .to_string();
    let mut runtime = prepared
        .start_vertical_slice_runtime()
        .expect("runtime starts from source-declared init path");
    assert_eq!(
        runtime.checked_program_identity_ref(),
        checked_program_identity
    );
    assert_eq!(
        runtime.sealed_admission_attestation_ref(),
        admission_identity
    );

    runtime
        .dispatch(Sys5VerticalAction::participant_a_attack_declared_target())
        .expect("attack runs through generated owner endpoint");
    runtime
        .dispatch(Sys5VerticalAction::world_tick("tick:sys5-cut-before-save"))
        .expect("designated tick publishes through generated input endpoints");
    let consumed = runtime
        .dispatch(Sys5VerticalAction::viewer_c_consume_world_result())
        .expect("ViewerC consumes the published designated result once");
    assert_eq!(consumed.typed_int(), Some(11));
    runtime
        .dispatch(Sys5VerticalAction::publish_relation("bird_follow"))
        .expect("relation publish imports the primary shadow");
    runtime
        .dispatch(Sys5VerticalAction::participant_a_leave_relation_primary(
            "bird_follow",
        ))
        .expect("source-bound ParticipantA leave retires M9 before relation fallback");
    let fresh = runtime
        .dispatch(Sys5VerticalAction::fresh_reacquire_relation_primary(
            "bird_follow",
        ))
        .expect("fresh binding reacquires the primary exactly once");

    let before_restore = runtime.observer_safe_runtime_snapshot();
    assert!(before_restore.owner_state_contains_int("WorldAuthority", "avatar", "self", "hp", 16));
    assert!(before_restore.designated_cache_contains("WorldAuthority.result", "ViewerC"));
    assert!(before_restore.relation_binding_consumed_fresh("bird_follow"));
    assert!(
        before_restore
            .m9_summary()
            .has_complete_final_residual_discharge()
    );
    assert!(
        before_restore
            .verification_summary()
            .is_discharged("finite_refinement")
    );
    let relation_digest = runtime
        .relation_semantic_digest("bird_follow")
        .expect("relation digest exists after fresh reacquire")
        .to_string();
    let relation_shadow = runtime
        .observer_relation_shadow("ViewerC", "bird_follow")
        .expect("ViewerC relation shadow exists after fresh reacquire")
        .clone();
    let last_fresh_request = fresh
        .generated_endpoint_chain()
        .request_identity()
        .to_string();

    let cut: Sys5LocalCut = runtime
        .save_local_cut("sys5-local-cut-after-fresh-reacquire")
        .expect("typed ST cut captures the whole vertical runtime");
    assert_eq!(cut.checked_program_identity_ref(), checked_program_identity);
    assert_eq!(cut.sealed_admission_attestation_ref(), admission_identity);
    assert!(cut.covers_owner_relation_designated_cache_m9_verification_and_counters());
    assert!(
        cut.observer_safe_provenance()
            .is_source_core_artifact_bound()
    );
    assert_observer_safe_no_raw_material(&format!("{cut:?}"));

    let rows_before_restore = runtime
        .observer_safe_joined_report()
        .ordered_rows()
        .to_vec();
    let save_report = runtime.observer_safe_joined_report().render_compact();
    let save_cut = assert_lifecycle_row_has_before_after_frontier(&save_report, "SaveCut");

    let mut restored = prepared_source(SYS5_CUT_PATCH_SOURCE)
        .restore_vertical_slice_runtime(&cut)
        .expect("fresh checked/admitted candidate restores the exact typed cut");
    let rows_after_restore = restored.observer_safe_joined_report().ordered_rows();
    assert!(
        rows_after_restore.starts_with(&rows_before_restore),
        "restore must preserve the observer-safe joined event prefix"
    );
    let restored_report = restored.observer_safe_joined_report().render_compact();
    let restore_cut =
        assert_lifecycle_row_has_before_after_frontier(&restored_report, "RestoreCut");
    assert_same_activation_frontier_lineage(&save_cut, &restore_cut, "no-patch restore");
    assert_eq!(
        restored.checked_program_identity_ref(),
        checked_program_identity
    );
    assert_eq!(
        restored.sealed_admission_attestation_ref(),
        admission_identity
    );
    assert_eq!(restored.observer_safe_runtime_snapshot(), before_restore);
    assert_eq!(
        restored.relation_semantic_digest("bird_follow"),
        Some(relation_digest.as_str())
    );
    assert_eq!(
        restored
            .observer_relation_shadow("ViewerC", "bird_follow")
            .expect("restored ViewerC shadow exists"),
        &relation_shadow
    );

    let duplicate = restored
        .dispatch(Sys5VerticalAction::fresh_reacquire_relation_primary(
            "bird_follow",
        ))
        .expect_err("restored used fresh binding rejects a second consume");
    assert_eq!(
        duplicate.kind(),
        Sys5VerticalDiagnosticKind::RelationFreshBindingAlreadyConsumed
    );
    assert!(duplicate.rejected_before_generated_endpoint());
    assert_eq!(restored.observer_safe_runtime_snapshot(), before_restore);

    let next_publish = restored
        .dispatch(Sys5VerticalAction::publish_relation("bird_follow"))
        .expect("runtime continues after duplicate fresh rejection");
    assert!(
        request_suffix(next_publish.generated_endpoint_chain().request_identity())
            > request_suffix(&last_fresh_request),
        "restore must retain counters so the next action cannot collide with a pre-cut request id"
    );

    for corruption in [
        Sys5CutCorruptionKind::WrapperIdentity,
        Sys5CutCorruptionKind::SourceProgramIdentity,
        Sys5CutCorruptionKind::ArtifactProjectionIdentity,
        Sys5CutCorruptionKind::CounterRollback,
        Sys5CutCorruptionKind::RelationDigest,
        Sys5CutCorruptionKind::ParticipantLeaveEvidence,
    ] {
        let corrupted = cut.clone().for_test_corrupt(corruption);
        let err = prepared_source(SYS5_CUT_PATCH_SOURCE)
            .restore_vertical_slice_runtime(&corrupted)
            .expect_err("corrupt typed cut must reject before partial runtime install");
        assert_eq!(err.kind(), Sys5LocalCutPatchErrorKind::CutRejected);
        assert!(err.rejected_before_partial_runtime());
        assert!(err.partial_runtime().is_none());
    }
}

#[test]
fn local_cut_after_participant_leave_restores_lifecycle_evidence_for_fresh_reacquire() {
    let mut runtime = vertical_runtime(SYS5_CUT_PATCH_SOURCE);
    runtime
        .dispatch(Sys5VerticalAction::publish_relation("bird_follow"))
        .expect("primary relation publication reaches ViewerC before ParticipantA leaves");
    let leave = runtime
        .dispatch(Sys5VerticalAction::participant_a_leave_relation_primary(
            "bird_follow",
        ))
        .expect("source-bound ParticipantA leave retires M9 and publishes the fallback");
    assert!(
        leave.participant_leave_evidence().is_some(),
        "the pre-cut runtime must retain actual observer-safe leave evidence"
    );
    assert_eq!(
        runtime
            .observer_relation_shadow("ViewerC", "bird_follow")
            .expect("fallback relation shadow exists after ParticipantA leave")
            .selected_floor(),
        "fallback-anchor"
    );

    let cut = runtime
        .save_local_cut("sys5-cut-after-participant-a-leave")
        .expect("post-leave runtime saves a complete local cut");
    let corrupt = cut
        .clone()
        .for_test_corrupt(Sys5CutCorruptionKind::ParticipantLeaveEvidence);
    let corruption = prepared_source(SYS5_CUT_PATCH_SOURCE)
        .restore_vertical_slice_runtime(&corrupt)
        .expect_err("cut with altered leave evidence rejects before a partial restore");
    assert_eq!(corruption.kind(), Sys5LocalCutPatchErrorKind::CutRejected);
    assert!(corruption.rejected_before_partial_runtime());
    assert!(corruption.partial_runtime().is_none());

    let mut restored = prepared_source(SYS5_CUT_PATCH_SOURCE)
        .restore_vertical_slice_runtime(&cut)
        .expect("exact cut restores the M9 leave lineage and observer evidence");
    let fresh = restored
        .dispatch(Sys5VerticalAction::fresh_reacquire_relation_primary(
            "bird_follow",
        ))
        .expect("fresh source-bound re-admission succeeds after restored ParticipantA leave");
    assert!(
        fresh.fresh_reacquire_evidence().is_some(),
        "fresh receipt must retain actual M9 re-admission evidence"
    );
    let shadow = restored
        .observer_relation_shadow("ViewerC", "bird_follow")
        .expect("fresh primary relation shadow reaches ViewerC after restore");
    assert_eq!(shadow.selected_anchor(), "participant_a_shoulder");
    assert_eq!(shadow.selected_floor(), "live-primary");
}

#[test]
fn owner_state_tampered_cut_rejects_before_partial_runtime_install() {
    let mut runtime = vertical_runtime(SYS5_CUT_PATCH_SOURCE);
    let original_hp = runtime
        .observer_safe_int("WorldAuthority", "avatar", "self", "hp")
        .expect("observer-safe owner hp exists before cut");
    assert_eq!(original_hp, 21);

    let cut = runtime
        .save_local_cut("sys5-owner-state-integrity-control-cut")
        .expect("control owner-state cut saves successfully");
    let control = prepared_source(SYS5_CUT_PATCH_SOURCE)
        .restore_vertical_slice_runtime(&cut)
        .expect("untampered control cut restores successfully");
    assert_eq!(
        control.observer_safe_int("WorldAuthority", "avatar", "self", "hp"),
        Some(original_hp),
        "control restore must preserve the untampered owner state value"
    );

    let tampered = cut.clone().for_test_tamper_owner_state_value(
        "WorldAuthority",
        "avatar",
        "self",
        "hp",
        original_hp + 7,
    );
    let err = prepared_source(SYS5_CUT_PATCH_SOURCE)
        .restore_vertical_slice_runtime(&tampered)
        .expect_err("owner-state tampered cut must reject before partial runtime install");
    assert_eq!(
        err.kind(),
        Sys5LocalCutPatchErrorKind::CutRejected,
        "owner-state tamper must be detected by the typed cut integrity boundary"
    );
    assert!(err.rejected_before_partial_runtime());
    assert!(err.partial_runtime().is_none());
    assert_observer_safe_no_raw_material(&format!("{err:?}"));
    assert_observer_safe_no_raw_material(&format!("{tampered:?}"));
}

#[test]
fn repeated_quiescent_cut_restore_events_keep_distinct_ordered_lifecycle_occurrences() {
    let mut runtime = vertical_runtime(SYS5_CUT_PATCH_SOURCE);

    let first_cut = runtime
        .save_local_cut("sys5-quiescent-repeat-cut-a")
        .expect("first quiescent save succeeds at the current no-patch frontier");
    let mut restored_once = prepared_source(SYS5_CUT_PATCH_SOURCE)
        .restore_vertical_slice_runtime(&first_cut)
        .expect("first quiescent restore succeeds through the saved frontier");
    let second_cut = restored_once
        .save_local_cut("sys5-quiescent-repeat-cut-b")
        .expect("second quiescent save at the same program/artifact/frontier succeeds");
    let restored_twice = prepared_source(SYS5_CUT_PATCH_SOURCE)
        .restore_vertical_slice_runtime(&second_cut)
        .expect("second quiescent restore succeeds through the same activation frontier");

    let report = restored_twice
        .observer_safe_joined_report()
        .render_compact();
    assert_repeated_lifecycle_rows_are_distinct_at_same_frontier(&report, "no-patch quiescent");
    assert_observer_safe_no_raw_material(&report);
}

#[test]
fn lifecycle_determinism_restoring_same_cut_in_independent_fresh_runtimes_is_isolated() {
    let mut runtime = vertical_runtime(SYS5_CUT_PATCH_SOURCE);
    let cut = runtime
        .save_local_cut("sys5-run-isolation-cut")
        .expect("deterministic run-isolation cut succeeds");

    let restored_a = prepared_source(SYS5_CUT_PATCH_SOURCE)
        .restore_vertical_slice_runtime(&cut)
        .expect("first independent restore of the same exact cut succeeds");
    let restored_b = prepared_source(SYS5_CUT_PATCH_SOURCE)
        .restore_vertical_slice_runtime(&cut)
        .expect("second independent restore of the same exact cut succeeds");

    let rows_a = restored_a.observer_safe_joined_report().ordered_rows();
    let rows_b = restored_b.observer_safe_joined_report().ordered_rows();
    assert_eq!(
        rows_a, rows_b,
        "independent deterministic replays of the same cut must expose the same observer-safe lifecycle report prefix"
    );

    let report_a = restored_a.observer_safe_joined_report().render_compact();
    let report_b = restored_b.observer_safe_joined_report().render_compact();
    let save_a = assert_lifecycle_row_has_before_after_frontier(&report_a, "SaveCut");
    let save_b = assert_lifecycle_row_has_before_after_frontier(&report_b, "SaveCut");
    let restore_a = assert_lifecycle_row_has_before_after_frontier(&report_a, "RestoreCut");
    let restore_b = assert_lifecycle_row_has_before_after_frontier(&report_b, "RestoreCut");
    assert_eq!(
        save_a, save_b,
        "same exact cut replay must retain the same observer-safe SaveCut prefix"
    );
    assert_same_activation_frontier_lineage(&save_a, &restore_a, "run-isolation restore A");
    assert_same_activation_frontier_lineage(&save_b, &restore_b, "run-isolation restore B");
    assert_eq!(
        restore_a.get("restore_occurrence_ref"),
        restore_b.get("restore_occurrence_ref"),
        "restore occurrence refs must be deterministic per cut lineage, not allocated from process-global state"
    );
    assert_observer_safe_lifecycle_occurrence_ref(
        restore_a
            .get("restore_occurrence_ref")
            .expect("restore occurrence ref exists"),
        "independent restore occurrence ref",
    );
    assert_observer_safe_no_raw_material(&report_a);
    assert_observer_safe_no_raw_material(&report_b);

    let mut same_lineage = prepared_source(SYS5_CUT_PATCH_SOURCE)
        .restore_vertical_slice_runtime(&cut)
        .expect("same-lineage restore starts from the same cut");
    let second_cut = same_lineage
        .save_local_cut("sys5-run-isolation-second-cut")
        .expect("second save in the same lineage succeeds at the same frontier");
    let restored_same_lineage = prepared_source(SYS5_CUT_PATCH_SOURCE)
        .restore_vertical_slice_runtime(&second_cut)
        .expect("second restore in the same lineage succeeds");
    let same_lineage_report = restored_same_lineage
        .observer_safe_joined_report()
        .render_compact();
    assert_repeated_lifecycle_rows_are_distinct_at_same_frontier(
        &same_lineage_report,
        "same-lineage persisted lifecycle cursor",
    );
    assert_observer_safe_no_raw_material(&same_lineage_report);
}

#[test]
fn lifecycle_determinism_corrupt_cursor_rejects_before_allocator_without_partial_runtime() {
    let mut runtime = vertical_runtime(SYS5_CUT_PATCH_SOURCE);
    let cut = runtime
        .save_local_cut("sys5-lifecycle-cursor-cut")
        .expect("lifecycle cursor cut succeeds");

    let label = "integrity-corrupt lifecycle cursor";
    let corrupted = cut
        .clone()
        .for_test_corrupt(Sys5CutCorruptionKind::LifecycleOccurrenceCounter);
    let err =
        match prepared_source(SYS5_CUT_PATCH_SOURCE).restore_vertical_slice_runtime(&corrupted) {
            Ok(_) => panic!("{label} unexpectedly restored a partial runtime"),
            Err(err) => err,
        };
    assert_eq!(
        err.kind(),
        Sys5LocalCutPatchErrorKind::CutRejected,
        "{label} must reject before allocator because the wrapper integrity digest no longer matches"
    );
    assert!(err.rejected_before_partial_runtime());
    assert!(err.partial_runtime().is_none());
    let rendered = format!("{err:?}");
    assert!(
        !rendered.contains("sys5-lifecycle-occurrence:00000000000000000000"),
        "{label} must not wrap the lifecycle cursor back to zero: {rendered}"
    );
    assert_observer_safe_no_raw_material(&rendered);
}

#[test]
fn lifecycle_determinism_valid_max_cursor_exhausts_checked_add_without_wrapping() {
    let mut runtime = vertical_runtime(SYS5_CUT_PATCH_SOURCE);
    let cut = runtime
        .save_local_cut("sys5-valid-max-lifecycle-cursor-cut")
        .expect("baseline cut succeeds before installing max lifecycle cursor");
    let max_cursor_cut = cut
        .clone()
        .for_test_with_valid_lifecycle_occurrence_counter(u64::MAX);

    let err = match prepared_source(SYS5_CUT_PATCH_SOURCE)
        .restore_vertical_slice_runtime(&max_cursor_cut)
    {
        Ok(restored) => {
            let report = restored.observer_safe_joined_report().render_compact();
            panic!(
                "valid max lifecycle cursor must fail before runtime install and must not wrap; report was:\n{report}"
            );
        }
        Err(err) => err,
    };
    assert_eq!(
        err.kind(),
        Sys5LocalCutPatchErrorKind::LifecycleOccurrenceExhausted,
        "integrity-valid max lifecycle cursor must reach checked-add exhaustion rather than generic cut rejection"
    );
    assert!(err.rejected_before_partial_runtime());
    assert!(err.partial_runtime().is_none());
    let rendered = format!("{err:?}");
    assert!(
        !rendered.contains("sys5-lifecycle-occurrence:00000000000000000000"),
        "max lifecycle cursor must not wrap back to a zero lifecycle occurrence ref: {rendered}"
    );
    assert_observer_safe_no_raw_material(&rendered);
}

#[test]
fn lifecycle_exhaustion_after_max_minus_one_restore_rejects_patch_without_mutation() {
    let mut runtime = vertical_runtime(SYS5_CUT_PATCH_SOURCE);
    runtime
        .dispatch(Sys5VerticalAction::participant_a_attack_declared_target())
        .expect("baseline owner RMW gives semantic state to preserve");
    runtime
        .dispatch(Sys5VerticalAction::world_tick(
            "tick:sys5-max-minus-one-before-cut",
        ))
        .expect("baseline designated publish gives cache state to preserve");
    runtime
        .dispatch(Sys5VerticalAction::viewer_c_consume_world_result())
        .expect("baseline designated consume installs cache state");

    let cut = runtime
        .save_local_cut("sys5-valid-max-minus-one-lifecycle-cursor-cut")
        .expect("baseline cut succeeds before installing max-minus-one lifecycle cursor")
        .for_test_with_valid_lifecycle_occurrence_counter(u64::MAX - 1);
    let mut restored = prepared_source(SYS5_CUT_PATCH_SOURCE)
        .restore_vertical_slice_runtime(&cut)
        .expect("valid max-minus-one lifecycle cursor restores before exhaustion");
    let restored_report = restored.observer_safe_joined_report().render_compact();
    let restore_cut =
        assert_lifecycle_row_has_before_after_frontier(&restored_report, "RestoreCut");
    let restore_occurrence_ref = lifecycle_occurrence_ref(
        &restore_cut,
        "restore_occurrence_ref",
        "max-minus-one RestoreCut row",
    );
    assert_lifecycle_occurrence_counter(
        restore_occurrence_ref,
        u64::MAX - 1,
        "max-minus-one RestoreCut row",
    );
    assert!(lifecycle_rows(&restored_report, "PatchAccepted").is_empty());
    assert!(lifecycle_rows(&restored_report, "PatchRejected").is_empty());

    let identity_before = restored.active_runtime_identity_snapshot();
    let semantic_before = restored.observer_safe_runtime_snapshot();
    let cache_before = restored.designated_cache_digest("WorldAuthority.result", "ViewerC");
    let m9_before = restored.observer_safe_m9_authority_digest();
    let m8_before = restored.observer_safe_m8_trace_digest();
    let patch_lifecycle_before = restored.patch_lifecycle_row_count();
    let joined_rows_before = restored
        .observer_safe_joined_report()
        .ordered_rows()
        .to_vec();
    let compact_report_before = restored.observer_safe_joined_report().render_compact();

    let candidate = source_first_patch_candidate(
        &restored,
        "sys5-designated-plus-two-after-max-minus-one-restore",
        designated_plus_two_source(),
    );
    let err = match restored.activate_source_first_patch(candidate) {
        Ok(outcome) => panic!(
            "ordinary accepted patch candidate must fail before activation when lifecycle cursor is exhausted; outcome was {outcome:?}"
        ),
        Err(err) => err,
    };
    assert_eq!(
        err.kind(),
        Sys5LocalCutPatchErrorKind::LifecycleOccurrenceExhausted,
        "patch activation must fail at checked lifecycle occurrence allocation"
    );
    assert!(err.rejected_before_partial_runtime());
    assert!(err.partial_runtime().is_none());
    let rendered = format!("{err:?}");
    assert!(
        !rendered.contains("sys5-lifecycle-occurrence:00000000000000000000"),
        "exhausted lifecycle cursor must not wrap back to zero: {rendered}"
    );
    assert_observer_safe_no_raw_material(&rendered);

    assert_eq!(restored.active_runtime_identity_snapshot(), identity_before);
    assert_eq!(restored.observer_safe_runtime_snapshot(), semantic_before);
    assert_eq!(
        restored.designated_cache_digest("WorldAuthority.result", "ViewerC"),
        cache_before
    );
    assert_eq!(restored.observer_safe_m9_authority_digest(), m9_before);
    assert_eq!(restored.observer_safe_m8_trace_digest(), m8_before);
    assert_eq!(
        restored.patch_lifecycle_row_count(),
        patch_lifecycle_before,
        "SYS-4 patch lifecycle must not gain a row when SYS-5 lifecycle allocation fails first"
    );
    assert_eq!(
        restored.observer_safe_joined_report().ordered_rows(),
        joined_rows_before.as_slice(),
        "joined report rows must remain unchanged when lifecycle allocation fails before patch activation"
    );
    let compact_report_after = restored.observer_safe_joined_report().render_compact();
    assert_eq!(compact_report_after, compact_report_before);
    assert!(lifecycle_rows(&compact_report_after, "PatchAccepted").is_empty());
    assert!(lifecycle_rows(&compact_report_after, "PatchRejected").is_empty());
    assert_observer_safe_no_raw_material(&compact_report_after);
}

#[test]
fn patch_lifecycle_occurrences_keep_repeated_owner_rmw_rejections_ordered_and_distinct() {
    let mut runtime = vertical_runtime(SYS5_CUT_PATCH_SOURCE);
    let owner_changed = owner_rmw_plus_one_source();
    let first_candidate = source_first_patch_candidate(
        &runtime,
        "sys5-owner-rmw-repeat-identical-reject",
        owner_changed.clone(),
    );
    let second_candidate = source_first_patch_candidate(
        &runtime,
        "sys5-owner-rmw-repeat-identical-reject",
        owner_changed,
    );

    let first = runtime
        .activate_source_first_patch(first_candidate)
        .expect("first owner-RMW candidate reaches typed patch rejection");
    let second = runtime
        .activate_source_first_patch(second_candidate)
        .expect("second identical owner-RMW candidate reaches typed patch rejection");

    for (label, outcome) in [("first", &first), ("second", &second)] {
        assert_eq!(
            outcome.verdict(),
            Sys5PatchVerdict::Rejected,
            "{label} repeated owner-RMW candidate must reject"
        );
        assert_eq!(
            outcome.primary_diagnostic_kind(),
            Some(Sys5PatchDiagnosticKind::OwnerRmwExpressionChanged),
            "{label} repeated owner-RMW candidate must reject for owner-RMW expression change"
        );
        assert!(outcome.lifecycle().is_lifecycle_only_rejection());
        assert_observer_safe_no_raw_material(&format!("{outcome:?}"));
    }

    let report = runtime.observer_safe_joined_report().render_compact();
    let rejected_rows = lifecycle_rows(&report, "PatchRejected");
    assert_patch_lifecycle_rows_are_distinct_at_same_frontier(
        &rejected_rows,
        "patch_occurrence_ref",
        &[0, 1],
        "repeated identical owner-RMW rejection",
        &report,
    );
    assert_observer_safe_no_raw_material(&report);
}

#[test]
fn patch_lifecycle_occurrence_persists_from_accept_into_cut_and_restore() {
    let mut runtime = vertical_runtime(SYS5_CUT_PATCH_SOURCE);
    let plus_two = designated_plus_two_source();
    let candidate = source_first_patch_candidate(
        &runtime,
        "sys5-designated-plus-two-occurrence",
        plus_two.clone(),
    );

    let accepted = runtime
        .activate_source_first_patch(candidate)
        .expect("designated-only candidate reaches typed patch acceptance");
    assert_eq!(accepted.verdict(), Sys5PatchVerdict::Accepted);
    assert_observer_safe_no_raw_material(&format!("{accepted:?}"));

    let patch_report = runtime.observer_safe_joined_report().render_compact();
    let patch_accepted =
        assert_lifecycle_row_has_before_after_frontier(&patch_report, "PatchAccepted");
    let patch_occurrence_ref = lifecycle_occurrence_ref(
        &patch_accepted,
        "patch_occurrence_ref",
        "accepted patch lifecycle row",
    );
    assert_lifecycle_occurrence_counter(patch_occurrence_ref, 0, "accepted patch lifecycle row");

    let cut = runtime
        .save_local_cut("sys5-accepted-patch-occurrence-cut")
        .expect("accepted-patch runtime saves through typed cut");
    let save_report = runtime.observer_safe_joined_report().render_compact();
    let save_cut = assert_lifecycle_row_has_before_after_frontier(&save_report, "SaveCut");
    let cut_occurrence_ref =
        lifecycle_occurrence_ref(&save_cut, "cut_occurrence_ref", "post-patch SaveCut row");
    assert_lifecycle_occurrence_counter(cut_occurrence_ref, 1, "post-patch SaveCut row");
    assert_eq!(
        save_cut.get("after_activation_frontier").copied(),
        patch_accepted.get("after_activation_frontier").copied(),
        "post-patch SaveCut must occur at the accepted patch activation frontier"
    );

    let restored = prepared_source(plus_two)
        .restore_vertical_slice_runtime(&cut)
        .expect("accepted-patch cut restores against the patched source identity");
    let restored_report = restored.observer_safe_joined_report().render_compact();
    let restore_cut =
        assert_lifecycle_row_has_before_after_frontier(&restored_report, "RestoreCut");
    let restore_occurrence_ref = lifecycle_occurrence_ref(
        &restore_cut,
        "restore_occurrence_ref",
        "post-patch RestoreCut row",
    );
    assert_lifecycle_occurrence_counter(restore_occurrence_ref, 2, "post-patch RestoreCut row");
    assert_same_activation_frontier_lineage(&save_cut, &restore_cut, "post-patch restore");
    assert_lifecycle_occurrence_refs_are_distinct(
        &[
            patch_occurrence_ref,
            cut_occurrence_ref,
            restore_occurrence_ref,
        ],
        "accepted patch to cut/restore lifecycle cursor",
    );
    assert_observer_safe_no_raw_material(&restored_report);
}

#[test]
fn source_first_designated_only_patch_accepts_once_and_temporal_stale_candidate_rejects() {
    let mut runtime = vertical_runtime(SYS5_CUT_PATCH_SOURCE);
    runtime
        .dispatch(Sys5VerticalAction::participant_a_attack_declared_target())
        .expect("baseline owner RMW runs before patch");
    runtime
        .dispatch(Sys5VerticalAction::world_tick("tick:sys5-patch-plus-one"))
        .expect("baseline +1 designated result publishes");
    runtime
        .dispatch(Sys5VerticalAction::viewer_c_consume_world_result())
        .expect("baseline +1 result caches at ViewerC");
    runtime
        .dispatch(Sys5VerticalAction::publish_relation("bird_follow"))
        .expect("baseline relation shadow is visible before patch");

    let owner_hp_before = runtime
        .observer_safe_int("WorldAuthority", "avatar", "self", "hp")
        .expect("observer-safe hp exists before patch");
    let relation_digest_before = runtime
        .relation_semantic_digest("bird_follow")
        .expect("relation digest exists before patch")
        .to_string();
    let relation_shadow_before = runtime
        .observer_relation_shadow("ViewerC", "bird_follow")
        .expect("relation shadow exists before patch")
        .clone();
    let cache_digest_before = runtime.designated_cache_digest("WorldAuthority.result", "ViewerC");

    let plus_two = designated_plus_two_source();
    assert_ne!(plus_two, SYS5_CUT_PATCH_SOURCE);
    let first_f0 =
        source_first_patch_candidate(&runtime, "sys5-designated-plus-two-f0-a", plus_two.clone());
    let second_f0 =
        source_first_patch_candidate(&runtime, "sys5-designated-plus-two-f0-b", plus_two.clone());

    let accepted = runtime
        .activate_source_first_patch(first_f0)
        .expect("designated-only checked/projected/M9-admitted patch returns a typed outcome");
    assert_eq!(accepted.verdict(), Sys5PatchVerdict::Accepted);
    assert_eq!(accepted.primary_diagnostic_kind(), None);
    assert!(
        accepted
            .boundary_inspection()
            .runtime_received_only_checked_patch_candidate()
    );
    assert!(
        accepted
            .lifecycle()
            .contains_source_first_checked_projection_and_m9_admission()
    );
    assert!(
        accepted
            .activation_frontier()
            .is_exact_successor_of(accepted.base_frontier())
    );
    assert_observer_safe_no_raw_material(&format!("{accepted:?}"));

    assert_eq!(
        runtime
            .observer_safe_int("WorldAuthority", "avatar", "self", "hp")
            .expect("owner hp still visible"),
        owner_hp_before,
        "accepted designated-only patch must preserve owner semantic state"
    );
    assert_eq!(
        runtime.relation_semantic_digest("bird_follow"),
        Some(relation_digest_before.as_str()),
        "accepted designated-only patch must preserve relation semantic digest"
    );
    assert_eq!(
        runtime
            .observer_relation_shadow("ViewerC", "bird_follow")
            .expect("relation shadow remains installed"),
        &relation_shadow_before,
        "accepted designated-only patch must preserve imported relation shadow"
    );
    assert_ne!(
        runtime.designated_cache_digest("WorldAuthority.result", "ViewerC"),
        cache_digest_before,
        "accepted designated patch must invalidate the old +1 designated cache binding"
    );
    assert_eq!(
        runtime.designated_cache_entry_count("WorldAuthority.result", "ViewerC"),
        0,
        "old designated cache must be cleared before +2 can be observed"
    );

    let tick_after_patch = runtime
        .dispatch(Sys5VerticalAction::world_tick("tick:sys5-patch-plus-two"))
        .expect("post-patch tick uses the +2 designated expression");
    assert_eq!(tick_after_patch.typed_int(), Some(12));
    let consume_after_patch = runtime
        .dispatch(Sys5VerticalAction::viewer_c_consume_world_result())
        .expect("ViewerC consumes the post-patch +2 result");
    assert_eq!(consume_after_patch.typed_int(), Some(12));

    let identity_after_first = runtime.active_runtime_identity_snapshot();
    let semantic_after_first = runtime.observer_safe_runtime_snapshot();
    let cache_after_first = runtime.designated_cache_digest("WorldAuthority.result", "ViewerC");
    let lifecycle_after_first = runtime.patch_lifecycle_row_count();
    let stale = runtime
        .activate_source_first_patch(second_f0)
        .expect("second F0 candidate is now temporally stale");
    assert_eq!(stale.verdict(), Sys5PatchVerdict::Rejected);
    assert_eq!(
        stale.primary_diagnostic_kind(),
        Some(Sys5PatchDiagnosticKind::StaleFrontier)
    );
    assert!(stale.lifecycle().is_lifecycle_only_rejection());
    assert_eq!(
        runtime.active_runtime_identity_snapshot(),
        identity_after_first
    );
    assert_eq!(
        runtime.observer_safe_runtime_snapshot(),
        semantic_after_first
    );
    assert_eq!(
        runtime.designated_cache_digest("WorldAuthority.result", "ViewerC"),
        cache_after_first
    );
    assert_eq!(
        runtime.patch_lifecycle_row_count(),
        lifecycle_after_first + 1
    );

    let report = runtime.observer_safe_joined_report().render_compact();
    let patch_accepted = assert_lifecycle_row_has_before_after_frontier(&report, "PatchAccepted");
    assert_lifecycle_row_has_before_after_frontier(&report, "PatchRejected");
    assert_observer_safe_no_raw_material(&report);

    let cut = runtime
        .save_local_cut("sys5-local-cut-after-plus-two-patch")
        .expect("patched runtime saves through typed SYS-5 cut");
    let save_report = runtime.observer_safe_joined_report().render_compact();
    let save_cut = assert_lifecycle_row_has_before_after_frontier(&save_report, "SaveCut");
    assert_eq!(
        save_cut.get("after_activation_frontier").copied(),
        patch_accepted.get("after_activation_frontier").copied(),
        "accepted-patch cut must save exactly at the accepted patched activation frontier"
    );
    let mut restored = prepared_source(plus_two)
        .restore_vertical_slice_runtime(&cut)
        .expect("patched runtime restores against the patched source/admission identity");
    let restored_report = restored.observer_safe_joined_report().render_compact();
    let restore_cut =
        assert_lifecycle_row_has_before_after_frontier(&restored_report, "RestoreCut");
    assert_same_activation_frontier_lineage(&save_cut, &restore_cut, "accepted-patch restore");
    let restored_tick = restored
        .dispatch(Sys5VerticalAction::world_tick(
            "tick:sys5-patch-plus-two-after-restore",
        ))
        .expect("restored patched runtime continues with +2 semantics");
    assert_eq!(restored_tick.typed_int(), Some(12));
}

#[test]
fn owner_rmw_expression_patch_rejects_without_mutating_active_runtime_except_lifecycle_row() {
    let mut runtime = vertical_runtime(SYS5_CUT_PATCH_SOURCE);
    runtime
        .dispatch(Sys5VerticalAction::participant_a_attack_declared_target())
        .expect("baseline owner RMW establishes observer-safe hp before rejected patch");
    runtime
        .dispatch(Sys5VerticalAction::world_tick("tick:sys5-owner-rmw-reject"))
        .expect("baseline designated publication exists before rejected patch");
    runtime
        .dispatch(Sys5VerticalAction::viewer_c_consume_world_result())
        .expect("baseline designated cache exists before rejected patch");
    runtime
        .dispatch(Sys5VerticalAction::publish_relation("bird_follow"))
        .expect("baseline relation state exists before rejected patch");

    let identity_before = runtime.active_runtime_identity_snapshot();
    let semantic_before = runtime.observer_safe_runtime_snapshot();
    let cache_before = runtime.designated_cache_digest("WorldAuthority.result", "ViewerC");
    let m9_before = runtime.observer_safe_m9_authority_digest();
    let m8_before = runtime.observer_safe_m8_trace_digest();
    let endpoint_count_before = runtime.total_endpoint_carrier_count();
    let relation_digest_before = runtime
        .relation_semantic_digest("bird_follow")
        .expect("relation digest exists before rejected patch")
        .to_string();
    let restore_capsule_before = runtime.observer_safe_restore_capsule_digest();
    let lifecycle_before = runtime.patch_lifecycle_row_count();
    let report_rows_before = runtime.observer_safe_joined_report().ordered_rows().len();

    let owner_changed = owner_rmw_plus_one_source();
    assert_ne!(owner_changed, SYS5_CUT_PATCH_SOURCE);
    let candidate =
        source_first_patch_candidate(&runtime, "sys5-owner-rmw-plus-one-rejected", owner_changed);

    let outcome = runtime
        .activate_source_first_patch(candidate)
        .expect("owner-RMW patch candidate is checked/projected/admitted before typed rejection");
    assert_eq!(outcome.verdict(), Sys5PatchVerdict::Rejected);
    assert_eq!(
        outcome.primary_diagnostic_kind(),
        Some(Sys5PatchDiagnosticKind::OwnerRmwExpressionChanged)
    );
    assert!(outcome.lifecycle().is_lifecycle_only_rejection());
    assert!(
        outcome
            .boundary_inspection()
            .runtime_received_only_checked_patch_candidate()
    );
    assert_observer_safe_no_raw_material(&format!("{outcome:?}"));

    assert_eq!(runtime.active_runtime_identity_snapshot(), identity_before);
    assert_eq!(runtime.observer_safe_runtime_snapshot(), semantic_before);
    assert_eq!(
        runtime.designated_cache_digest("WorldAuthority.result", "ViewerC"),
        cache_before
    );
    assert_eq!(runtime.observer_safe_m9_authority_digest(), m9_before);
    assert_eq!(runtime.observer_safe_m8_trace_digest(), m8_before);
    assert_eq!(
        runtime.total_endpoint_carrier_count(),
        endpoint_count_before
    );
    assert_eq!(
        runtime.relation_semantic_digest("bird_follow"),
        Some(relation_digest_before.as_str())
    );
    assert_eq!(
        runtime.observer_safe_restore_capsule_digest(),
        restore_capsule_before
    );
    assert_eq!(runtime.patch_lifecycle_row_count(), lifecycle_before + 1);
    assert_eq!(
        runtime.observer_safe_joined_report().ordered_rows().len(),
        report_rows_before + 1,
        "rejected owner-RMW patch may append exactly one typed patch report row"
    );

    let report = runtime.observer_safe_joined_report().render_compact();
    assert_lifecycle_row_has_before_after_frontier(&report, "PatchRejected");
    assert_observer_safe_no_raw_material(&report);

    let post_reject_attack = runtime
        .dispatch(Sys5VerticalAction::participant_a_attack_declared_target())
        .expect("old owner operation remains active after rejected owner-RMW patch");
    assert_eq!(
        post_reject_attack
            .owner_mutation("WorldAuthority", "avatar", "self", "hp")
            .expect("observer-safe hp mutation remains visible")
            .old_new_int(),
        (16, 11),
        "rejected owner-RMW patch must retain the original hp - atk semantics"
    );
}

fn request_suffix(request_id: &str) -> u64 {
    request_id
        .rsplit([':', '-'])
        .next()
        .expect("request id has a numeric suffix")
        .parse()
        .expect("request id suffix is numeric")
}

fn assert_lifecycle_row_has_before_after_frontier<'a>(
    report: &'a str,
    kind: &str,
) -> std::collections::BTreeMap<&'a str, &'a str> {
    let marker = format!("lifecycle:{kind}:");
    let rows = report
        .lines()
        .filter(|row| row.starts_with(&marker))
        .collect::<Vec<_>>();
    assert_eq!(
        rows.len(),
        1,
        "observer-safe joined report must contain exactly one {kind} lifecycle row; report was:\n{report}"
    );
    let row = rows[0];
    let fields = parse_lifecycle_fields(row, &marker);
    for field in [
        "before_source_ref=",
        "after_source_ref=",
        "before_core_ref=",
        "after_core_ref=",
        "before_artifact_ref=",
        "after_artifact_ref=",
        "before_activation_frontier=",
        "after_activation_frontier=",
    ] {
        assert!(
            row.contains(field),
            "{kind} lifecycle row must expose observer-safe {field} field: {row}"
        );
    }
    assert_observer_safe_no_raw_material(row);
    fields
}

fn parse_lifecycle_fields<'a>(
    row: &'a str,
    marker: &str,
) -> std::collections::BTreeMap<&'a str, &'a str> {
    row.strip_prefix(marker)
        .expect("lifecycle row starts with expected marker")
        .split(';')
        .map(|field| {
            field
                .split_once('=')
                .unwrap_or_else(|| panic!("lifecycle field must be key=value: {field}"))
        })
        .collect()
}

fn lifecycle_rows<'a>(
    report: &'a str,
    kind: &str,
) -> Vec<std::collections::BTreeMap<&'a str, &'a str>> {
    let marker = format!("lifecycle:{kind}:");
    report
        .lines()
        .filter(|row| row.starts_with(&marker))
        .map(|row| parse_lifecycle_fields(row, &marker))
        .collect()
}

fn assert_repeated_lifecycle_rows_are_distinct_at_same_frontier(report: &str, label: &str) {
    let save_rows = lifecycle_rows(report, "SaveCut");
    let restore_rows = lifecycle_rows(report, "RestoreCut");
    let mut issues = Vec::new();

    if save_rows.len() != 2 {
        issues.push(format!(
            "{label}: expected two ordered SaveCut rows at the same activation frontier, got {}",
            save_rows.len()
        ));
    }
    if restore_rows.len() != 2 {
        issues.push(format!(
            "{label}: expected two ordered RestoreCut rows at the same activation frontier, got {}",
            restore_rows.len()
        ));
    }

    assert_lifecycle_occurrence_refs(&save_rows, "cut_occurrence_ref", "SaveCut", &mut issues);
    assert_lifecycle_occurrence_refs(
        &restore_rows,
        "restore_occurrence_ref",
        "RestoreCut",
        &mut issues,
    );
    assert_lifecycle_rows_share_same_frontier(
        &save_rows,
        "SaveCut",
        &[
            "before_source_ref",
            "after_source_ref",
            "before_core_ref",
            "after_core_ref",
            "before_artifact_ref",
            "after_artifact_ref",
            "before_activation_frontier",
            "after_activation_frontier",
        ],
        &mut issues,
    );
    assert_lifecycle_rows_share_same_frontier(
        &restore_rows,
        "RestoreCut",
        &[
            "before_source_ref",
            "after_source_ref",
            "before_core_ref",
            "after_core_ref",
            "before_artifact_ref",
            "after_artifact_ref",
            "before_activation_frontier",
            "after_activation_frontier",
        ],
        &mut issues,
    );

    if let Some(saved_frontier) = save_rows
        .first()
        .and_then(|row| row.get("after_activation_frontier"))
    {
        for (index, restore) in restore_rows.iter().enumerate() {
            for field in ["before_activation_frontier", "after_activation_frontier"] {
                if restore.get(field) != Some(saved_frontier) {
                    issues.push(format!(
                        "{label}: RestoreCut row {index} {field} must equal the exact saved activation frontier"
                    ));
                }
            }
        }
    }

    assert!(
        issues.is_empty(),
        "repeated lifecycle rows must not be deduplicated or rewritten with cut-integrity hashes:\n{}\nreport was:\n{report}",
        issues.join("\n")
    );
}

fn assert_lifecycle_occurrence_refs(
    rows: &[std::collections::BTreeMap<&str, &str>],
    field: &str,
    kind: &str,
    issues: &mut Vec<String>,
) {
    let mut seen = std::collections::BTreeSet::new();
    for (index, row) in rows.iter().enumerate() {
        let Some(occurrence_ref) = row.get(field) else {
            issues.push(format!("{kind} row {index} is missing {field}"));
            continue;
        };
        if !occurrence_ref.starts_with("sys5-lifecycle-occurrence:") {
            issues.push(format!(
                "{kind} row {index} {field} must be an opaque SYS-5 lifecycle occurrence ref, got {occurrence_ref}"
            ));
        }
        if !seen.insert(*occurrence_ref) {
            issues.push(format!(
                "{kind} row {index} reused lifecycle occurrence ref {occurrence_ref}"
            ));
        }
        assert_observer_safe_no_raw_material(occurrence_ref);
    }
}

fn assert_patch_lifecycle_rows_are_distinct_at_same_frontier(
    rows: &[std::collections::BTreeMap<&str, &str>],
    occurrence_field: &str,
    expected_counters: &[u64],
    label: &str,
    report: &str,
) {
    let mut issues = Vec::new();
    if rows.len() != expected_counters.len() {
        issues.push(format!(
            "{label}: expected {} ordered PatchRejected rows at the unchanged activation frontier, got {}",
            expected_counters.len(),
            rows.len()
        ));
    }
    assert_lifecycle_occurrence_refs(rows, occurrence_field, "PatchRejected", &mut issues);
    assert_lifecycle_rows_share_same_frontier(
        rows,
        "PatchRejected",
        &[
            "before_source_ref",
            "after_source_ref",
            "before_core_ref",
            "after_core_ref",
            "before_artifact_ref",
            "after_artifact_ref",
            "before_activation_frontier",
            "after_activation_frontier",
        ],
        &mut issues,
    );

    for (index, expected) in expected_counters.iter().enumerate() {
        let Some(row) = rows.get(index) else {
            continue;
        };
        let Some(occurrence_ref) = row.get(occurrence_field) else {
            continue;
        };
        match lifecycle_occurrence_counter_value(occurrence_ref) {
            Ok(actual) if actual == *expected => {}
            Ok(actual) => issues.push(format!(
                "{label}: PatchRejected row {index} {occurrence_field} cursor was {actual}, expected {expected}"
            )),
            Err(issue) => issues.push(format!(
                "{label}: PatchRejected row {index} {occurrence_field} is not a valid observer-safe lifecycle occurrence ref: {issue}"
            )),
        }
    }

    assert!(
        issues.is_empty(),
        "patch lifecycle occurrences must remain ordered, distinct, cursor-local, and non-deduplicated:\n{}\nreport was:\n{report}",
        issues.join("\n")
    );
}

fn lifecycle_occurrence_ref<'a>(
    row: &std::collections::BTreeMap<&'a str, &'a str>,
    field: &str,
    label: &str,
) -> &'a str {
    let occurrence_ref = row
        .get(field)
        .copied()
        .unwrap_or_else(|| panic!("{label} is missing {field}; row was: {row:?}"));
    assert_observer_safe_lifecycle_occurrence_ref(occurrence_ref, label);
    occurrence_ref
}

fn assert_lifecycle_occurrence_counter(occurrence_ref: &str, expected: u64, label: &str) {
    let actual = lifecycle_occurrence_counter_value(occurrence_ref)
        .unwrap_or_else(|issue| panic!("{label} has invalid lifecycle occurrence ref: {issue}"));
    assert_eq!(
        actual, expected,
        "{label} must use the next persisted lifecycle cursor slot"
    );
}

fn lifecycle_occurrence_counter_value(occurrence_ref: &str) -> Result<u64, String> {
    let body = occurrence_ref
        .strip_prefix("sys5-lifecycle-occurrence:")
        .ok_or_else(|| format!("missing lifecycle occurrence prefix: {occurrence_ref}"))?;
    let (counter, digest) = body
        .split_once(':')
        .ok_or_else(|| format!("missing counter/digest separator: {occurrence_ref}"))?;
    if counter.len() != 20 || !counter.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(format!(
            "counter is not 20 decimal digits: {occurrence_ref}"
        ));
    }
    if digest.is_empty() || !digest.chars().all(|ch| ch.is_ascii_hexdigit()) {
        return Err(format!("digest is not opaque hex: {occurrence_ref}"));
    }
    counter
        .parse()
        .map_err(|err| format!("counter parse failed for {occurrence_ref}: {err}"))
}

fn assert_lifecycle_occurrence_refs_are_distinct(occurrence_refs: &[&str], label: &str) {
    let mut seen = std::collections::BTreeSet::new();
    for (index, occurrence_ref) in occurrence_refs.iter().enumerate() {
        assert_observer_safe_lifecycle_occurrence_ref(occurrence_ref, label);
        assert!(
            seen.insert(*occurrence_ref),
            "{label}: occurrence ref at index {index} reused {occurrence_ref}"
        );
    }
}

fn assert_observer_safe_lifecycle_occurrence_ref(occurrence_ref: &str, label: &str) {
    assert!(
        occurrence_ref.starts_with("sys5-lifecycle-occurrence:"),
        "{label} must be an opaque SYS-5 lifecycle occurrence ref, got {occurrence_ref}"
    );
    assert_observer_safe_no_raw_material(occurrence_ref);
}

fn assert_lifecycle_rows_share_same_frontier(
    rows: &[std::collections::BTreeMap<&str, &str>],
    kind: &str,
    fields: &[&str],
    issues: &mut Vec<String>,
) {
    let Some(first) = rows.first() else {
        return;
    };
    for field in fields {
        let Some(expected) = first.get(field) else {
            issues.push(format!("{kind} row 0 is missing {field}"));
            continue;
        };
        for (index, row) in rows.iter().enumerate().skip(1) {
            if row.get(field) != Some(expected) {
                issues.push(format!(
                    "{kind} row {index} changed {field}; repeated quiescent events must stay at the same program/artifact/frontier"
                ));
            }
        }
    }
}

fn assert_same_activation_frontier_lineage(
    save_cut: &std::collections::BTreeMap<&str, &str>,
    restore_cut: &std::collections::BTreeMap<&str, &str>,
    label: &str,
) {
    let saved_frontier = save_cut
        .get("after_activation_frontier")
        .expect("SaveCut after activation frontier exists");
    assert_eq!(
        restore_cut.get("before_activation_frontier"),
        Some(saved_frontier),
        "{label}: RestoreCut.before_activation_frontier must retain the exact saved frontier"
    );
    assert_eq!(
        restore_cut.get("after_activation_frontier"),
        Some(saved_frontier),
        "{label}: RestoreCut.after_activation_frontier must retain the exact saved frontier"
    );
    for field in [
        "after_source_ref",
        "after_core_ref",
        "after_artifact_ref",
        "after_activation_frontier",
    ] {
        assert_eq!(
            restore_cut.get(field),
            save_cut.get(field),
            "{label}: RestoreCut must preserve exact {field} from SaveCut rather than replacing it with cut-integrity material"
        );
    }
}

fn assert_observer_safe_no_raw_material(text: &str) {
    for denied in [
        "/home/",
        "source_text",
        "raw_source",
        "avatar[target].hp =",
        "avatar[self].atk",
        "participant_input[self].focus +",
        "hp - atk",
        "hp + 1",
        "focus + 1",
        "focus + 2",
        "raw_authority",
        "raw_capability",
        "raw_credential",
        "raw_witness",
        "capability_secret",
        "witness_secret",
        "route_override",
        "expected_result",
    ] {
        assert!(
            !text.contains(denied),
            "observer-safe cut/patch material leaked denied fragment `{denied}` in:\n{text}"
        );
    }
}
