use mir_ast::surface_alpha::{
    SurfaceExprKind, SurfacePlaceItem, SurfaceStmt, parse_surface_mir_report,
};

fn diagnostic_codes(source: &str) -> Vec<String> {
    parse_surface_mir_report(source)
        .diagnostics
        .into_iter()
        .map(|diagnostic| diagnostic.code)
        .collect()
}

#[test]
fn parses_canonical_brace_place_scope_with_indexed_state() {
    let source = r#"
module Surface.Syntax

place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
}
"#;

    let report = parse_surface_mir_report(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert!(!report.final_public_grammar_frozen);
    let module = report.module.expect("accepted source should carry module");
    assert_eq!(module.module_path, "Surface.Syntax");
    assert_eq!(module.place_blocks.len(), 1);
    let block = &module.place_blocks[0];
    assert_eq!(block.place_ref, "S");
    let SurfacePlaceItem::State(state) = &block.items[0] else {
        panic!("expected indexed state item");
    };
    assert_eq!(state.state_name, "player");
    assert_eq!(state.owner_place, "S");
    assert_eq!(state.index.as_ref().expect("indexed state").name, "p");
    assert_eq!(
        state.index.as_ref().expect("indexed state").key_type_text,
        "Participant"
    );
    assert_eq!(state.value_type_text, "Player");
}

#[test]
fn rejects_bracket_place_scope_syntax() {
    let source = r#"
module Surface.Syntax

S[
  state player[p: Participant]: Player
]
"#;

    assert_eq!(
        diagnostic_codes(source),
        vec!["bracket_place_scope_not_supported"]
    );
}

#[test]
fn rejects_undeclared_place_block_head() {
    let source = r#"
module Surface.Syntax

Unknown {
  state player[p: Participant]: Player
}
"#;

    assert_eq!(
        diagnostic_codes(source),
        vec!["undeclared_place_block_head"]
    );
}

#[test]
fn rejects_undeclared_role_instance_head() {
    let source = r#"
module Surface.Syntax

Unknown[self] {
  when start {
  }
}
"#;

    assert_eq!(
        diagnostic_codes(source),
        vec!["undeclared_role_instance_head"]
    );
}

#[test]
fn rejects_declared_place_as_role_instance_head() {
    let source = r#"
module Surface.Syntax

place World

World[self] {
  when start {
  }
}
"#;

    assert_eq!(
        diagnostic_codes(source),
        vec!["bracket_place_scope_not_supported"]
    );
}

#[test]
fn rejects_empty_role_instance_binder() {
    let source = r#"
module Surface.Syntax

role BrowserClient

BrowserClient[] {
  when start {
  }
}
"#;

    assert_eq!(
        diagnostic_codes(source),
        vec!["invalid_role_instance_binder"]
    );
}

#[test]
fn rejects_expression_role_instance_binder() {
    let source = r#"
module Surface.Syntax

role BrowserClient

BrowserClient[self + other] {
  when start {
  }
}
"#;

    assert_eq!(
        diagnostic_codes(source),
        vec!["invalid_role_instance_binder"]
    );
}

#[test]
fn parses_role_named_s_as_role_instance_when_declared_role() {
    let source = r#"
module Surface.Syntax

role S

S[self] {
  when start {
  }
}
"#;

    let report = parse_surface_mir_report(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    let module = report.module.expect("accepted source should carry module");
    assert_eq!(module.role_instance_blocks.len(), 1);
    assert_eq!(module.role_instance_blocks[0].role_ref, "S");
    assert_eq!(module.role_instance_blocks[0].instance_ref, "self");
}

#[test]
fn parses_record_literal_in_state_initializer() {
    let source = r#"
module Surface.Syntax

place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    init Player { hp: 1 }
}
"#;

    let report = parse_surface_mir_report(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    let module = report.module.expect("accepted source should carry module");
    let SurfacePlaceItem::State(state) = &module.place_blocks[0].items[0] else {
        panic!("expected state item");
    };
    let init = state.initial_value.as_ref().expect("state initializer");
    let SurfaceExprKind::RecordLiteral {
        record_name,
        fields,
    } = &init.kind
    else {
        panic!("expected record literal, got {:?}", init.kind);
    };
    assert_eq!(record_name, "Player");
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0].field_name, "hp");
}

#[test]
fn rejects_ambiguous_place_and_type_brace_construct() {
    let source = r#"
module Surface.Syntax

place Player

record Player {
  hp: Int64,
}

Player {
  state player[p: Participant]: Player
}
"#;

    assert_eq!(diagnostic_codes(source), vec!["ambiguous_brace_construct"]);
}

#[test]
fn parses_role_instance_when_join_block() {
    let source = r#"
module Surface.Syntax

role BrowserClient {
  supports renderer.pose_v1
}

principal self
place World
place WorldAdmission

BrowserClient[self] {
  when start {
    join World as BrowserClient via WorldAdmission
  }
}
"#;

    let report = parse_surface_mir_report(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    let module = report.module.expect("accepted source should carry module");
    assert_eq!(module.roles.len(), 1);
    assert_eq!(module.role_instance_blocks.len(), 1);
    let block = &module.role_instance_blocks[0];
    assert_eq!(block.role_ref, "BrowserClient");
    assert_eq!(block.instance_ref, "self");
    assert_eq!(block.whens.len(), 1);
    assert_eq!(block.whens[0].event_name, "start");
    assert!(block.whens[0].failure_row.is_empty());
    assert_eq!(block.whens[0].body.len(), 1);
    let SurfaceStmt::Join(join) = &block.whens[0].body[0] else {
        panic!("expected join statement");
    };
    assert_eq!(join.target_place, "World");
    assert_eq!(join.role_ref, "BrowserClient");
    assert_eq!(join.admission_place, "WorldAdmission");
}

#[test]
fn rejects_bare_role_block() {
    let source = r#"
module Surface.Syntax

role BrowserClient

BrowserClient {
  when start {
  }
}
"#;

    assert_eq!(
        diagnostic_codes(source),
        vec!["bare_role_block_not_supported"]
    );
}

#[test]
fn parses_when_failure_row_and_nested_place_request_body() {
    let source = r#"
module Surface.Syntax

role BrowserClient
place World

BrowserClient[self] {
  when send_chat(text: Text) fails MissingCapability, RouteUnavailable {
    World {
      last_message[self].text = text
    }
  }
}
"#;

    let report = parse_surface_mir_report(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    let module = report.module.expect("accepted source should carry module");
    let when = &module.role_instance_blocks[0].whens[0];
    assert_eq!(
        when.failure_row,
        vec!["MissingCapability", "RouteUnavailable"]
    );
    let SurfaceStmt::NestedPlaceBlock(block) = &when.body[0] else {
        panic!("expected nested place block");
    };
    assert_eq!(block.place_ref, "World");
}

#[test]
fn rejects_compound_assignment_until_lowering_is_defined() {
    for operator in ["+", "-"] {
        let source = format!(
            r#"
module Surface.Syntax

role BrowserClient

BrowserClient[self] {{
  when start {{
    player[self].hp {operator}= 1
  }}
}}
"#
        );

        assert_eq!(
            diagnostic_codes(&source),
            vec!["compound_assignment_not_supported"],
            "operator {operator} must not silently lower as ordinary assignment"
        );
    }
}

#[test]
fn rejects_compound_assignment_in_braced_or_nested_contexts() {
    for source in [
        r#"
module Surface.Syntax

role BrowserClient

BrowserClient[self] {
  when start {
    player[Key { id: 1 }].hp += 1
  }
}
"#,
        r#"
module Surface.Syntax

role BrowserClient

BrowserClient[self] {
  when start {
    player[self += 1].hp = 0
  }
}
"#,
        r#"
module Surface.Syntax

role BrowserClient

BrowserClient[self] {
  when start {
    player[self].hp = next_hp += 1
  }
}
"#,
        r#"
module Surface.Syntax

role BrowserClient

BrowserClient[self] {
  when start {
    player[self].hp = next_hp -= 1
  }
}
"#,
    ] {
        assert_eq!(
            diagnostic_codes(source),
            vec!["compound_assignment_not_supported"],
            "compound tokens must not reach ordinary assignment lowering"
        );
    }
}

#[test]
fn rejects_unlowered_let_and_if_before_assignment_dispatch() {
    for source in [
        r#"
module Surface.Syntax

role BrowserClient

BrowserClient[self] {
  when start {
    let value = 1
  }
}
"#,
        r#"
module Surface.Syntax

role BrowserClient

BrowserClient[self] {
  when start {
    if true {
    }
  }
}
"#,
    ] {
        assert_eq!(
            diagnostic_codes(source),
            vec!["unsupported_surface_statement"],
            "unlowered statement forms must not be accepted as assignment or raw syntax"
        );
    }
}

#[test]
fn rejects_equality_from_assignment_dispatch() {
    for source in [
        r#"
module Surface.Syntax

role BrowserClient

BrowserClient[self] {
  when start {
    player[self].hp == 1
  }
}
"#,
        r#"
module Surface.Syntax

role BrowserClient

BrowserClient[self] {
  when start {
    player[self].hp = next_hp == 1
  }
}
"#,
        r#"
module Surface.Syntax

role BrowserClient

BrowserClient[self] {
  when start {
    player[self == other].hp = 0
  }
}
"#,
    ] {
        assert_eq!(
            diagnostic_codes(source),
            vec!["unsupported_surface_expression_operator"],
            "equality must not be parsed as an assignment with an equals-prefixed RHS"
        );
    }
}
