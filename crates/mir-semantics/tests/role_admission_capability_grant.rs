use mir_semantics::surface_role_admission::{
    check_surface_role_admission_source, surface_role_admission_diagnostic_codes,
};

#[test]
fn browser_client_join_generates_admission_grant_and_witness() {
    let source = r#"
module Surface.Role.JoinAccepted

role BrowserClient {
  supports renderer.pose_v1
}

place World
place WorldAdmission

record Player {
  hp: Int64,
}

World {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when start fails MissingCapability {
    join World as BrowserClient via WorldAdmission
    World {
      player[self].hp = 1
    }
  }
}
"#;

    let report = check_surface_role_admission_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert_eq!(report.role_claims.len(), 1);
    assert_eq!(report.admission_requests.len(), 1);
    assert_eq!(report.admission_verdicts.len(), 1);
    assert_eq!(report.admission_verdicts[0].verdict, "accepted");
    assert!(
        report
            .capability_grants
            .iter()
            .any(|grant| grant.capability == "WriteState(World)"
                && grant.authority_source == "admission_grant")
    );
    assert_eq!(report.admission_witnesses.len(), 1);
    assert_eq!(report.authority_checks.len(), 1);
    assert!(report.authority_checks[0].accepted);
    assert_eq!(
        report.authority_checks[0].authority_source.as_deref(),
        Some("admission_grant")
    );
}

#[test]
fn role_claim_without_grant_cannot_write_server_owned_state() {
    let source = r#"
module Surface.Role.ClaimWithoutGrant

role BrowserClient
place World

record Player {
  hp: Int64,
}

World {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when attack(target: Participant) fails MissingCapability {
    World {
      player[target].hp = 1
    }
  }
}
"#;

    let report = check_surface_role_admission_source(source);

    assert!(!report.accepted);
    assert_eq!(
        surface_role_admission_diagnostic_codes(&report),
        vec!["role_claim_without_capability_grant"]
    );
    assert_eq!(report.authority_checks.len(), 1);
    assert!(!report.authority_checks[0].accepted);
    assert_eq!(
        report.authority_checks[0].required_capability,
        "WriteState(World)"
    );
}

#[test]
fn stale_membership_message_is_rejected_without_resurrecting_authority() {
    let source = r#"
module Surface.Role.StaleMembership

role BrowserClient
place World
place WorldAdmission

record Player {
  hp: Int64,
}

World {
  state player[p: Participant]: Player
}

BrowserClient[self] {
  when start fails MissingCapability {
    join World as BrowserClient via WorldAdmission
  }

  when stale_delivery fails StaleMembership {
    stale_message World epoch_0000 incarnation_0000
    World {
      player[self].hp = 1
    }
  }
}
"#;

    let report = check_surface_role_admission_source(source);

    assert!(!report.accepted);
    assert_eq!(
        surface_role_admission_diagnostic_codes(&report),
        vec![
            "stale_membership_message_rejected",
            "stale_membership_authority_rejected"
        ]
    );
    assert_eq!(report.stale_rejections.len(), 1);
    assert_eq!(report.stale_rejections[0].target_place, "World");
    assert_eq!(report.authority_checks.len(), 1);
    assert!(!report.authority_checks[0].accepted);
    assert_eq!(
        report.authority_checks[0].reason_code.as_deref(),
        Some("stale_membership")
    );
}

#[test]
fn package_runtime_hash_binding_is_metadata_not_safety_proof() {
    let source = r#"
module Surface.Role.HashBindingMetadata

role BrowserClient
place World
place WorldAdmission

BrowserClient[self] {
  when start fails MissingCapability {
    join World as BrowserClient via WorldAdmission
    bind_hash package pkg_hash_v1 runtime runtime_hash_v1
  }
}
"#;

    let report = check_surface_role_admission_source(source);

    assert!(report.accepted, "{:?}", report.diagnostics);
    assert_eq!(report.optional_hash_bindings.len(), 1);
    assert_eq!(report.optional_hash_bindings[0].package_hash, "pkg_hash_v1");
    assert_eq!(
        report.optional_hash_bindings[0].runtime_hash,
        "runtime_hash_v1"
    );
    assert!(!report.optional_hash_bindings[0].semantic_safety_proof);
}
