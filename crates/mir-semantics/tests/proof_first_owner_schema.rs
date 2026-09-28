//! Nonproduction checked-artifact projection controls for W4-C. These tests
//! neither install metadata nor assign security labels or authenticate imports.
use mir_ast::surface_v0::FixtureSource;
use mir_semantics::surface_v0_pipeline::{
    CheckedExpressionTree, CheckedIndexedStateSchema, CheckedStateFieldSchema, CheckedSurfaceV0,
    M7DiagnosticKind, TypedStateRead, check_and_elaborate_surface_v0,
};

fn source(body: &str, extra_state: &str) -> String {
    format!(
        "module Mirrorea.SchemaOrigin\n\
         locus A\nlocus S\nlocus T\nprincipal self\ntype Player\n\
         state player[id: Player] at S {{\n\
           hp: Int\nshown: Int\nunused: Int\nvisible observer_safe fields (shown)\n}}\n\
         state team[id: Player] at T {{\n hp: Int\n }}\n\
         {extra_state}\nRole[self] at A {{\n\
           when refresh(target: Player, amount: Int) fails \
           (StaleMembership, MissingCapability, MissingWitness, VisibilityDenied, RouteUnavailable) {{\n\
             {body}\n\
           }}\n}}\nwith auth MembershipAuth\nverify finite_refinement\n"
    )
}

fn checked(body: &str) -> CheckedSurfaceV0 {
    check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/schema_origin.mir",
        source(body, ""),
    ))
    .expect("ordinary source checks")
}

// Walk executable structure, rather than trusting a caller-supplied read list.
fn tree_reads<'a>(tree: &'a CheckedExpressionTree, out: &mut Vec<&'a TypedStateRead>) {
    match tree {
        CheckedExpressionTree::StateRead(read) => out.push(read),
        CheckedExpressionTree::ParameterRead { .. } | CheckedExpressionTree::IntegerLiteral(_) => {}
        CheckedExpressionTree::Binary { left, right, .. } => {
            tree_reads(left, out);
            tree_reads(right, out);
        }
    }
}

fn declaration<'a>(
    artifact: &'a CheckedSurfaceV0,
    read: &TypedStateRead,
) -> (&'a CheckedIndexedStateSchema, &'a CheckedStateFieldSchema) {
    // Global namespace uniqueness precedes owner filtering. Keep both owning
    // declaration and field, including their separate source locations.
    let states: Vec<_> = artifact
        .static_environment()
        .indexed_state_schemas()
        .iter()
        .filter(|state| state.name() == read.namespace())
        .collect();
    assert_eq!(
        states.len(),
        1,
        "namespace uniqueness precedes field resolution"
    );
    let state = states[0];
    let candidates: Vec<_> = state
        .fields()
        .iter()
        .filter(|field| Some(field.name()) == read.field())
        .collect();
    assert_eq!(candidates.len(), 1);
    let field = candidates[0];
    assert_eq!(state.owner_locus(), read.owner_locus());
    assert_eq!(field.type_name(), read.value_type());
    (state, field)
}

// Optional external evidence export from the ACTUAL checked artifact. No expected
// runtime/transport trace is generated. The proof driver consumes these facts.
fn export_artifact(name: &str, artifact: &CheckedSurfaceV0) {
    let Some(root) = std::env::var_os("MIR_PROOF_SCHEMA_EXPORT") else {
        return;
    };
    fn read(read: &TypedStateRead) -> serde_json::Value {
        serde_json::json!({"namespace":read.namespace(),"index":read.index(),
            "field":read.field(),"owner":read.owner_locus(),"type":read.value_type(),
            "source_ref":format!("{:?}",read.source_ref())})
    }
    fn tree(value: &CheckedExpressionTree) -> serde_json::Value {
        match value {
            CheckedExpressionTree::StateRead(r) => serde_json::json!({"state":read(r)}),
            CheckedExpressionTree::ParameterRead { name, .. } => {
                serde_json::json!({"parameter":name})
            }
            CheckedExpressionTree::IntegerLiteral(i) => serde_json::json!({"integer":i.value()}),
            CheckedExpressionTree::Binary {
                operator,
                left,
                right,
                ..
            } => serde_json::json!({
                "operator":format!("{operator:?}"),"left":tree(left),"right":tree(right)}),
        }
    }
    let states: Vec<_> = artifact.static_environment().indexed_state_schemas().iter().map(|s| {
        let fields: Vec<_> = s.fields().iter().map(|f| serde_json::json!({"name":f.name(),
            "type":f.type_name(),"visibility":f.visibility_channel(),"source_ref":format!("{:?}",f.source_ref())})).collect();
        serde_json::json!({"name":s.name(),"index_name":s.index_name(),"index_type":s.index_type(),
            "owner":s.owner_locus(),"fields":fields,"source_ref":format!("{:?}",s.source_ref())})
    }).collect();
    let operations: Vec<_> = artifact
        .evaluations()
        .iter()
        .filter_map(|e| {
            let op = e.owner_rmw_core()?;
            Some(
                serde_json::json!({"name":e.name(),"actor":e.actor_authority_origin(),
            "owner":op.owner_locus(),"source_ref":format!("{:?}",e.source_ref()),
            "target":read(op.target()),"tree":tree(op.expression().tree()),
            "retained_reads":op.same_owner_reads().iter().map(read).collect::<Vec<_>>()}),
            )
        })
        .collect();
    let output = serde_json::json!({"module":artifact.static_environment().module(),
        "checked_identity":artifact.program_identity().stable_key(),
        "structural_entries":artifact.program_identity().structural_entries(),
        "states":states,"operations":operations});
    let path = std::path::Path::new(&root).join(format!("{name}.json"));
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    serde_json::to_writer_pretty(file, &output).unwrap();
}

#[test]
fn complete_tree_footprint_retains_each_checked_declaration_and_repeated_read() {
    let artifact =
        checked("at S { player[target].shown = player[self].hp + amount - player[target].hp }");
    export_artifact("ordered_reads", &artifact);
    let owner = artifact.evaluations()[0].owner_rmw_core().unwrap();
    let mut reads = Vec::new();
    tree_reads(owner.expression().tree(), &mut reads);
    assert_eq!(reads, owner.same_owner_reads().iter().collect::<Vec<_>>());
    assert_eq!(reads.len(), 2);
    assert_eq!(reads[0].index(), Some("self"));
    assert_eq!(reads[1].index(), Some("target"));
    assert_ne!(reads[0].source_ref(), reads[1].source_ref());
    for read in &reads {
        let (state, field) = declaration(&artifact, read);
        assert_eq!(state.index_type(), "Player");
        assert_eq!(field.visibility_channel(), None);
        assert_ne!(field.source_ref(), &read.source_ref());
    }
    let (_, target) = declaration(&artifact, owner.target());
    assert_eq!(target.visibility_channel(), Some("observer_safe"));
    // Unused declarations remain in the schema but are not invented reads.
    assert!(
        artifact.static_environment().indexed_state_schemas()[0]
            .fields()
            .iter()
            .any(|field| field.name() == "unused")
    );
    assert!(reads.iter().all(|read| read.field() != Some("unused")));
    assert!(reads.iter().all(|read| read.field() != Some("shown")));
}

#[test]
fn constant_assignment_still_has_target_declaration_without_a_value_read() {
    let artifact = checked("at S { player[target].shown = 5 }");
    export_artifact("constant", &artifact);
    let owner = artifact.evaluations()[0].owner_rmw_core().unwrap();
    let mut reads = Vec::new();
    tree_reads(owner.expression().tree(), &mut reads);
    assert!(reads.is_empty());
    assert!(owner.same_owner_reads().is_empty());
    assert_eq!(declaration(&artifact, owner.target()).1.name(), "shown");
}

#[test]
fn different_owner_namespaces_keep_distinct_fields_in_one_checked_artifact() {
    let artifact = checked(
        "at S { player[target].hp = player[target].hp + amount }\n\
         at T { team[target].hp = team[target].hp - amount }",
    );
    export_artifact("two_owners", &artifact);
    let owners: Vec<_> = artifact
        .evaluations()
        .iter()
        .filter_map(|evaluation| evaluation.owner_rmw_core())
        .collect();
    assert_eq!(owners.len(), 2);
    let fields: Vec<_> = owners
        .iter()
        .map(|owner| {
            let mut reads = Vec::new();
            tree_reads(owner.expression().tree(), &mut reads);
            assert_eq!(reads, owner.same_owner_reads().iter().collect::<Vec<_>>());
            assert_eq!(reads.len(), 1);
            for read in reads {
                declaration(&artifact, read);
            }
            let (state, field) = declaration(&artifact, owner.target());
            (state.name(), state.owner_locus(), field.name())
        })
        .collect();
    assert_eq!(fields, [("player", "S", "hp"), ("team", "T", "hp")]);
    // This proves retained M7 facts, not an M8 multi-owner runtime/restore path.
}

#[test]
fn duplicate_namespace_under_another_owner_is_not_hidden_by_partitioning() {
    for fields in ["hp: Int", "mp: Int"] {
        let result = check_and_elaborate_surface_v0(FixtureSource::new(
            "tests/inline/schema_origin.mir",
            source(
                "at S { player[target].hp = 5 }",
                &format!("state player[id: Player] at T {{ {fields} }}"),
            ),
        ));
        assert_eq!(
            result.unwrap_err().primary().kind(),
            M7DiagnosticKind::DuplicateDeclaration
        );
    }
}

#[test]
fn checked_identity_changes_with_unused_declaration_not_just_executed_footprint() {
    let original = source("at S { player[target].hp = 5 }", "");
    // Same byte count keeps later source spans equal, isolating the schema change.
    let changed = original.replace("unused: Int", "spared: Int");
    let first = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/schema_origin.mir",
        original,
    ))
    .unwrap();
    let second = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/schema_origin.mir",
        changed,
    ))
    .unwrap();
    assert_ne!(first.program_identity(), second.program_identity());
    assert_eq!(
        first.evaluations()[0]
            .owner_rmw_core()
            .expect("first owner Core exists"),
        second.evaluations()[0]
            .owner_rmw_core()
            .expect("second owner Core exists")
    );
}

#[test]
fn parenthesized_expression_is_outside_the_existing_checked_profile() {
    let result = check_and_elaborate_surface_v0(FixtureSource::new(
        "tests/inline/schema_origin.mir",
        source(
            "at S { player[target].shown = player[self].hp + (amount - player[target].hp) }",
            "",
        ),
    ));
    assert_eq!(
        result.unwrap_err().primary().kind(),
        M7DiagnosticKind::UnsupportedExpression
    );
}
