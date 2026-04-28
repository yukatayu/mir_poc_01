use mirrorea_core::{MembershipRegistry, PlaceCatalog};

#[test]
fn membership_registry_tracks_epoch_and_typed_snapshot() {
    let mut registry = MembershipRegistry::default();
    registry
        .add_initial("Alice", "ParticipantPlace[Alice]")
        .expect("initial Alice");
    registry
        .add_initial("Bob", "ParticipantPlace[Bob]")
        .expect("initial Bob");
    assert_eq!(registry.membership_epoch(), 0);

    let dave = registry
        .add_member("Dave", "ParticipantPlace[Dave]")
        .expect("late join Dave");
    assert_eq!(dave.principal, "Dave");
    assert_eq!(dave.participant_place, "ParticipantPlace[Dave]");
    assert!(dave.active);
    assert_eq!(dave.incarnation, 0);
    assert_eq!(dave.joined_at_epoch, 1);
    assert_eq!(registry.membership_epoch(), 1);
    assert!(registry.active("Alice"));
    assert!(registry.active("Bob"));
    assert!(registry.active("Dave"));
    assert_eq!(
        registry.active_members(),
        vec!["Alice".to_string(), "Bob".to_string(), "Dave".to_string()]
    );

    let snapshot = registry.snapshot();
    assert_eq!(snapshot.membership_epoch, 1);
    assert_eq!(snapshot.members["Dave"].place, "ParticipantPlace[Dave]");
    assert!(snapshot.members["Dave"].active);
    assert_eq!(snapshot.members["Dave"].incarnation, 0);
    assert_eq!(snapshot.members["Dave"].joined_at_epoch, 1);
    assert_eq!(snapshot.members["Dave"].left_at_epoch, None);
}

#[test]
fn membership_registry_mark_inactive_bumps_epoch_and_incarnation() {
    let mut registry = MembershipRegistry::default();
    registry
        .add_initial("Carol", "ParticipantPlace[Carol]")
        .expect("initial Carol");

    let record = registry.mark_inactive("Carol").expect("Carol leaves");
    assert_eq!(record.principal, "Carol");
    assert!(!record.active);
    assert_eq!(record.incarnation, 1);
    assert_eq!(record.left_at_epoch, Some(1));
    assert_eq!(registry.membership_epoch(), 1);
    assert!(!registry.active("Carol"));
    assert!(registry.active_members().is_empty());
    assert_eq!(registry.inactive_members(), vec!["Carol".to_string()]);
}

#[test]
fn membership_registry_rejects_duplicate_member_and_missing_leave_target() {
    let mut registry = MembershipRegistry::default();
    registry
        .add_initial("Alice", "ParticipantPlace[Alice]")
        .expect("initial Alice");

    let duplicate = registry
        .add_member("Alice", "ParticipantPlace[Alice]")
        .expect_err("duplicate principal should fail");
    assert!(duplicate.to_string().contains("already exists"));
    assert_eq!(registry.membership_epoch(), 0);

    let missing = registry
        .mark_inactive("Unknown")
        .expect_err("missing principal should fail");
    assert!(missing.to_string().contains("Unknown"));
}

#[test]
fn membership_registry_add_initial_is_bootstrap_only() {
    let mut registry = MembershipRegistry::default();
    registry
        .add_initial("Alice", "ParticipantPlace[Alice]")
        .expect("initial Alice");
    registry
        .add_member("Bob", "ParticipantPlace[Bob]")
        .expect("late join Bob");

    let err = registry
        .add_initial("Carol", "ParticipantPlace[Carol]")
        .expect_err("add_initial should not be available after epoch advance");
    assert!(err.to_string().contains("bootstrap-only"));
    assert_eq!(registry.membership_epoch(), 1);
    assert!(!registry.active("Carol"));
}

#[test]
fn place_catalog_accepts_identical_duplicate_and_rejects_kind_drift() {
    let mut catalog = PlaceCatalog::default();
    catalog
        .register("WorldServerPlace", "world_server")
        .expect("world server");
    catalog
        .register("ParticipantPlace[Alice]", "participant")
        .expect("participant");
    catalog
        .register("WorldServerPlace", "world_server")
        .expect("identical duplicate");

    let err = catalog
        .register("WorldServerPlace", "participant")
        .expect_err("kind drift should fail");
    assert!(err.to_string().contains("WorldServerPlace"));

    assert_eq!(
        catalog.place_ids(),
        vec![
            "ParticipantPlace[Alice]".to_string(),
            "WorldServerPlace".to_string(),
        ]
    );
    assert_eq!(catalog.kind_of("WorldServerPlace"), Some("world_server"));
    assert_eq!(catalog.kind_of("MissingPlace"), None);

    let snapshot = catalog.snapshot();
    assert_eq!(snapshot.places["WorldServerPlace"], "world_server");
    assert_eq!(snapshot.places["ParticipantPlace[Alice]"], "participant");
}
