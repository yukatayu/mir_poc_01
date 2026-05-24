use mir_semantics::surface_indexed_state::{
    check_surface_indexed_state_source, indexed_state_diagnostic_codes,
};

#[test]
fn accepts_s_owned_participant_indexed_state() {
    let source = r#"
module Surface.Indexed.Accept

place S

record Player {
  hp: Int64,
}

S {
  state player[p: Participant]: Player
    init Player { hp: 100 }
    visible observer_safe fields { hp }

  when tick {
    player[self].hp = 99
  }
}
"#;

    let report = check_surface_indexed_state_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert!(!report.final_public_api_frozen);
    assert_eq!(report.indexed_states.len(), 1);
    let state = &report.indexed_states[0];
    assert_eq!(state.owner_locus, "S");
    assert_eq!(state.state_name, "player");
    assert_eq!(state.key_name, "p");
    assert_eq!(state.keyspace_type, "Participant");
    assert_eq!(state.value_type, "Player");
    assert_eq!(state.visible_fields, vec!["hp"]);
    assert_eq!(state.authority_model, "owner_locus_or_explicit_capability");
    assert!(report.access_checks.iter().any(|row| row.accepted));
}

#[test]
fn rejects_key_write_as_authority_confusion() {
    let source = r#"
module Surface.Indexed.KeyAuthorityNegative

role BrowserClient
place S

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when cheat {
    player[self].hp = 1
  }
}
"#;

    let report = check_surface_indexed_state_source(source);

    assert!(!report.accepted);
    assert_eq!(
        indexed_state_diagnostic_codes(&report),
        vec!["indexed_state_key_is_not_authority"]
    );
}

#[test]
fn rejects_nested_place_block_as_ambient_authority_switch() {
    let source = r#"
module Surface.Indexed.NestedPlaceAuthorityNegative

role BrowserClient
place S

S {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when cheat {
    S {
      player[self].hp = 1
    }
  }
}
"#;

    let report = check_surface_indexed_state_source(source);

    assert!(!report.accepted);
    assert_eq!(
        indexed_state_diagnostic_codes(&report),
        vec!["indexed_state_nested_place_requires_generated_request"]
    );
}

#[test]
fn rejects_stale_key_access_after_leave_marker() {
    let source = r#"
module Surface.Indexed.StaleKeyNegative

place S

S {
  state player[p: Participant]: Player

  when leave_then_write {
    leave self
    player[self].hp = 0
  }
}
"#;

    let report = check_surface_indexed_state_source(source);

    assert!(!report.accepted);
    assert_eq!(
        indexed_state_diagnostic_codes(&report),
        vec!["stale_indexed_state_key"]
    );
}

#[test]
fn rejects_compaction_when_retained_evidence_mentions_key() {
    let source = r#"
module Surface.Indexed.CompactionNegative

place S

S {
  state player[p: Participant]: Player

  when compact_after_save {
    retain savepoint self
    compact player[self]
  }
}
"#;

    let report = check_surface_indexed_state_source(source);

    assert!(!report.accepted);
    assert_eq!(
        indexed_state_diagnostic_codes(&report),
        vec!["indexed_state_compaction_blocked_by_retained_evidence"]
    );
}

#[test]
fn accepts_same_state_name_under_different_owner_loci() {
    let source = r#"
module Surface.Indexed.OwnerScopedNames

place S
place T

S {
  state player[p: Participant]: Player

  when tick {
    player[self].hp = 99
  }
}

T {
  state player[p: Participant]: Player

  when tick {
    player[self].hp = 88
  }
}
"#;

    let report = check_surface_indexed_state_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert_eq!(report.indexed_states.len(), 2);
    assert_eq!(
        report
            .indexed_states
            .iter()
            .map(|state| (state.owner_locus.as_str(), state.state_name.as_str()))
            .collect::<Vec<_>>(),
        vec![("S", "player"), ("T", "player")]
    );
}

#[test]
fn rejects_unsupported_keyspace_for_alpha_indexed_state() {
    let source = r#"
module Surface.Indexed.UnsupportedKeyspace

place S

S {
  state player[p: Text]: Player
}
"#;

    let report = check_surface_indexed_state_source(source);

    assert!(!report.accepted);
    assert_eq!(
        indexed_state_diagnostic_codes(&report),
        vec!["unsupported_indexed_state_keyspace"]
    );
}
