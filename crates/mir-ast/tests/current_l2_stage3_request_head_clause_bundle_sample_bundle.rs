use std::fs;
use std::path::PathBuf;

use mir_ast::current_l2::{
    Stage3PerformHead, Stage3PerformTargetRef, Stage3RequestAttachmentFrameKind,
    Stage3RequestClauseSuite, Stage3RequestHeadClauseBundle, parse_stage3_request_head_clause_bundle_text,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root should exist")
        .parent()
        .expect("repo root should exist")
        .to_path_buf()
}

fn load_sample(name: &str) -> String {
    let path = repo_root()
        .join("samples")
        .join("prototype")
        .join("current-l2-parser-companion")
        .join(name);
    fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!("failed to read parser-side companion sample `{}`: {error}", path.display())
    })
}

#[test]
fn parser_companion_sample_bundle_parses_problem1_p06_slice() {
    let source = load_sample("p06-typed-proof-owner-handoff.request.txt");

    let actual = parse_stage3_request_head_clause_bundle_text(&source)
        .expect("parser-side companion sample should parse");

    assert_eq!(
        actual,
        Stage3RequestHeadClauseBundle {
            perform_head: Stage3PerformHead {
                op: "prove_owner_handoff".to_string(),
                target_ref: Stage3PerformTargetRef::Via("review_unit".to_string()),
            },
            clause_suite: Stage3RequestClauseSuite {
                require_fragment_text: Some("typed_guard".to_string()),
                ensure_fragment_text: Some("owner_is(next_owner)".to_string()),
            },
            attachment_frame_kind: Stage3RequestAttachmentFrameKind::RequestLocalTwoSlotSuite,
        }
    );
}

#[test]
fn parser_companion_sample_bundle_parses_problem2_representative_pair() {
    let late_join_source = load_sample("p07-dice-late-join-visible-history.request.txt");
    let reconnect_source = load_sample("p08-dice-stale-reconnect-refresh.request.txt");

    let late_join = parse_stage3_request_head_clause_bundle_text(&late_join_source)
        .expect("late-join parser-side companion sample should parse");
    let reconnect = parse_stage3_request_head_clause_bundle_text(&reconnect_source)
        .expect("stale-reconnect parser-side companion sample should parse");

    assert_eq!(
        late_join,
        Stage3RequestHeadClauseBundle {
            perform_head: Stage3PerformHead {
                op: "publish_visible_history".to_string(),
                target_ref: Stage3PerformTargetRef::On("authoritative_room".to_string()),
            },
            clause_suite: Stage3RequestClauseSuite {
                require_fragment_text: Some("authority_ack".to_string()),
                ensure_fragment_text: Some("history_visible_as_past".to_string()),
            },
            attachment_frame_kind: Stage3RequestAttachmentFrameKind::RequestLocalTwoSlotSuite,
        }
    );
    assert_eq!(
        reconnect,
        Stage3RequestHeadClauseBundle {
            perform_head: Stage3PerformHead {
                op: "refresh_stale_session".to_string(),
                target_ref: Stage3PerformTargetRef::Via("authoritative_room".to_string()),
            },
            clause_suite: Stage3RequestClauseSuite {
                require_fragment_text: Some("stale_session".to_string()),
                ensure_fragment_text: Some("refreshed_before_rejoin".to_string()),
            },
            attachment_frame_kind: Stage3RequestAttachmentFrameKind::RequestLocalTwoSlotSuite,
        }
    );
}
