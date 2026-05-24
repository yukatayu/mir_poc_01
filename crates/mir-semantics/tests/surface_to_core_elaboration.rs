use mir_semantics::surface_to_core_elaboration::{
    elaborate_surface_to_core_source, surface_elaboration_diagnostic_codes,
};

#[test]
fn elaborates_cross_locus_read_into_remote_request_with_observe_edge() {
    let source = r#"
module Surface.Elab.CrossRead

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

BrowserClient[self] {
  when render fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership {
    seen_hp = player[self].hp
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert!(!report.final_public_api_frozen);
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    let request = &report.core_ir.remote_requests[0];
    assert_eq!(request.request_kind, "read");
    assert_eq!(request.requester_locus, "role:BrowserClient");
    assert_eq!(request.owner_locus, "S");
    assert_eq!(request.state_name, "player");
    assert_eq!(request.key_expr, "self");
    assert!(request.source_span.end > request.source_span.start);
    assert!(
        report.core_ir.generated_edges.iter().any(
            |edge| edge.edge_kind == "observe_request" && edge.request_id == request.request_id
        )
    );
}

#[test]
fn elaborates_nested_place_write_into_owner_directed_remote_request() {
    let source = r#"
module Surface.Elab.CrossWrite

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership {
    S {
      player[target].hp = player[target].hp - 1
    }
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    let request = &report.core_ir.remote_requests[0];
    assert_eq!(request.request_kind, "write");
    assert_eq!(request.requester_locus, "role:BrowserClient");
    assert_eq!(request.owner_locus, "S");
    assert_eq!(request.state_name, "player");
    assert_eq!(request.key_expr, "target");
    assert_eq!(request.generated_from, "nested_place_block");
    assert!(
        report
            .core_ir
            .transitions
            .iter()
            .any(|transition| transition.kind == "generated_remote_write_request")
    );
}

#[test]
fn rejects_generated_remote_request_when_failure_row_is_underdeclared() {
    let source = r#"
module Surface.Elab.UnderdeclaredFailureRow

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when render fails MissingCapability {
    seen_hp = player[self].hp
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(!report.accepted);
    assert_eq!(
        surface_elaboration_diagnostic_codes(&report),
        vec!["generated_failure_not_declared"]
    );
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    assert!(!report.core_ir.remote_requests[0].failure_row_complete);
}

#[test]
fn rejects_generated_write_request_when_failure_row_is_underdeclared() {
    let source = r#"
module Surface.Elab.UnderdeclaredWriteFailureRow

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability {
    S {
      player[target].hp = 1
    }
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(!report.accepted);
    assert_eq!(
        surface_elaboration_diagnostic_codes(&report),
        vec!["generated_failure_not_declared"]
    );
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    assert_eq!(report.core_ir.remote_requests[0].request_kind, "write");
    assert!(!report.core_ir.remote_requests[0].failure_row_complete);
}

#[test]
fn elaborates_nested_place_read_as_owner_directed_request() {
    let source = r#"
module Surface.Elab.NestedRead

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

BrowserClient[self] {
  when render fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership {
    S {
      seen_hp = player[self].hp
    }
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert_eq!(report.core_ir.remote_requests.len(), 1);
    let request = &report.core_ir.remote_requests[0];
    assert_eq!(request.request_kind, "read");
    assert_eq!(request.generated_from, "nested_place_block");
    assert_eq!(request.owner_locus, "S");
}

#[test]
fn rejects_unsupported_surface_statements_instead_of_dropping_them() {
    let source = r#"
module Surface.Elab.UnsupportedJoin

role BrowserClient
place World
place WorldAdmission

BrowserClient[self] {
  when start {
    join World as BrowserClient via WorldAdmission
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(!report.accepted);
    assert_eq!(
        surface_elaboration_diagnostic_codes(&report),
        vec!["unsupported_surface_statement_for_elaboration"]
    );
    assert_eq!(report.core_ir.remote_requests.len(), 0);
}

#[test]
fn generated_core_ir_carries_source_spans_for_transitions_and_requests() {
    let source = r#"
module Surface.Elab.SourceSpans

role BrowserClient
place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    visible observer_safe fields { hp }
}

BrowserClient[self] {
  when render fails MissingCapability, MissingWitness, RouteUnavailable, StaleMembership {
    seen_hp = player[self].hp
  }
}
"#;

    let report = elaborate_surface_to_core_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert!(!report.core_ir.source_spans.is_empty());
    assert!(
        report
            .core_ir
            .source_spans
            .iter()
            .all(|row| row.span.end > row.span.start)
    );
    assert!(
        report
            .accepted_obligations
            .iter()
            .any(|obligation| obligation.code == "surface_core_source_spans_preserved")
    );
}
