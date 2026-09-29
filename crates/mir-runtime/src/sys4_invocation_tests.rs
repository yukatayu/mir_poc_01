use super::*;

#[cfg(test)]
mod proof_first_invocation_inventory_tests {
    use super::*;
    use crate::proof_first_owner_schema_tests::invocation_source_fixture;
    use serde_json::{Value, json};
    const SOURCE: &str =
        include_str!("../../mir-ast/tests/fixtures/surface-v0/sys4_ow1_endpoint_crossing.mir");
    fn owner_fields(value: &mut Value) -> &mut Value {
        let operations = value["locus_programs"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|row| row["key"] == "S")
            .unwrap()["value"]["operations"]
            .as_array_mut()
            .unwrap();
        &mut operations
            .iter_mut()
            .find(|op| op["placement"]["kind"] == "owner_rmw")
            .unwrap()["placement"]["fields"]
    }
    fn mutate_signature(signature: &mut Value, kind: &str) {
        match kind {
            "erasure" => signature["parameters"] = json!([]),
            "parameter-type" => signature["parameters"][1]["type_name"] = json!("Player"),
            "parameter-order" => signature["parameters"].as_array_mut().unwrap().reverse(),
            "parameter-span" => {
                signature["parameters"][1]["source_ref"]["path"] =
                    json!("changed-parameter-source.mir")
            }
            "operation" => signature["name"] = json!("other"),
            "actor" => signature["actor"] = json!("other"),
            "owner" => signature["owner_locus"] = json!("A"),
            "kind" => signature["kind"] = json!("consumer_local_projection"),
            "statement-span" => signature["source_ref"]["path"] = json!("changed-statement.mir"),
            _ => panic!("unknown mutation"),
        }
    }
    fn mutate_payload(payload: &mut Value, kind: &str) {
        match kind {
            "target" => payload["target"]["index"] = json!("self"),
            "expression" => {
                payload["expression"]["tree"]["operator"] = json!("add");
                payload["expression"]["operator_chain"] = json!(["+"]);
            }
            "expression-tree" => payload["expression"]["tree"]["operator"] = json!("add"),
            "operator-chain" => payload["expression"]["operator_chain"] = json!(["+"]),
            "budget" => {
                assert!(payload["owner_admission_budget"].is_object());
                payload["owner_admission_budget"] = Value::Null;
            }
            _ => panic!("unknown payload mutation"),
        }
    }
    #[test]
    fn proof_first_invocation_private_fields_and_each_bootstrap_guard() {
        let source = SOURCE
            .replace("target: Player)", "target: Player, unused: Int)")
            .replace(
                "RouteUnavailable)",
                "RouteUnavailable, DeadlineExpired) within owner_ticks 1",
            );
        let (program, admission) = invocation_source_fixture(&source);
        let original = serde_json::to_value(admission.i3_private_snapshot().unwrap()).unwrap();
        let projection =
            serde_json::to_value(program.i3_private_projection_snapshot().unwrap()).unwrap();
        assert!(program.matches_owner_execution_inventory(&admission.instance));
        SealedFabricAdmission::from_i3_private_snapshot(
            serde_json::from_value(original.clone()).unwrap(),
            &program,
        )
        .unwrap();
        LocalFabric::bootstrap(program.clone(), admission.clone(), BackendProfile::St).unwrap();
        for (kind, lower_accepts) in [
            ("erasure", true),
            ("parameter-type", true),
            ("parameter-order", true),
            ("parameter-span", true),
            ("operation", false),
            ("actor", false),
            ("owner", false),
            ("kind", false),
            ("statement-span", false),
            ("target", true),
            ("expression", true),
            ("expression-tree", false),
            ("operator-chain", false),
            ("budget", true),
        ] {
            let mut image = original.clone();
            let plan = &mut image["instance"]["owner_execution_plans"][0];
            if matches!(
                kind,
                "target" | "expression" | "expression-tree" | "operator-chain" | "budget"
            ) {
                mutate_payload(plan, kind)
            } else {
                mutate_signature(&mut plan["signature"], kind)
            }
            assert!(
                SealedFabricAdmission::from_i3_private_snapshot(
                    serde_json::from_value(image.clone()).unwrap(),
                    &program
                )
                .is_err(),
                "sealed restore accepted {kind}"
            );
            let decoded = M8RuntimeInstance::from_i3_private_snapshot(
                serde_json::from_value(image["instance"].clone()).unwrap(),
            );
            assert_eq!(
                decoded.is_ok(),
                lower_accepts,
                "lower structural decode classification: {kind}"
            );
            if let Ok(instance) = decoded {
                assert!(!program.matches_owner_execution_inventory(&instance));
                // Bypass the earlier sealed conversion deliberately to test the
                // final bootstrap guard independently; no public entry is added.
                let mut direct = admission.clone();
                direct.instance = instance;
                assert!(
                    LocalFabric::bootstrap(program.clone(), direct, BackendProfile::St).is_err(),
                    "bootstrap accepted {kind}"
                );
            }
            let mut image = projection.clone();
            let fields = owner_fields(&mut image);
            if matches!(
                kind,
                "target" | "expression" | "expression-tree" | "operator-chain" | "budget"
            ) {
                mutate_payload(&mut fields["core"], kind)
            } else {
                mutate_signature(&mut fields["signature"], kind)
            }
            let projected = FabricProgram::from_i3_private_projection_snapshot(
                serde_json::from_value(image).unwrap(),
            );
            if let Ok(changed) = projected {
                assert!(
                    !changed.matches_owner_execution_inventory(&admission.instance),
                    "projection {kind} must disagree with original plan"
                );
                assert!(
                    SealedFabricAdmission::from_i3_private_snapshot(
                        serde_json::from_value(original.clone()).unwrap(),
                        &changed
                    )
                    .is_err()
                );
                assert!(
                    LocalFabric::bootstrap(changed, admission.clone(), BackendProfile::St).is_err()
                );
            }
        }
        let mut old = original.clone();
        old["instance"]["version"] = json!(2);
        assert!(
            SealedFabricAdmission::from_i3_private_snapshot(
                serde_json::from_value(old).unwrap(),
                &program
            )
            .is_err()
        );
        let mut old = projection.clone();
        old["version"] = json!(1);
        assert!(
            FabricProgram::from_i3_private_projection_snapshot(
                serde_json::from_value(old).unwrap()
            )
            .is_err()
        );
        let mut absent = original;
        absent["instance"]["owner_execution_plans"][0]
            .as_object_mut()
            .unwrap()
            .remove("signature");
        assert!(serde_json::from_value::<Sys4I3PrivateSealedAdmissionSnapshot>(absent).is_err());
        let mut absent = projection;
        owner_fields(&mut absent)
            .as_object_mut()
            .unwrap()
            .remove("signature");
        assert!(
            serde_json::from_value::<crate::sys3_i3_private_snapshot::I3PrivateProjectionSnapshot>(
                absent
            )
            .is_err()
        );
    }
    #[test]
    fn proof_first_invocation_patch_shape_binds_owner_signature_and_core_separately() {
        // Owner-local source has no requester fragment: that unrelated branch
        // cannot hide a missing owner-signature comparison.
        let source = SOURCE
            .replace("Role[self] at A", "Role[self] at S")
            .replace("target: Player)", "target: Player, unused: Int)");
        let (program, _) = invocation_source_fixture(&source);
        let original =
            serde_json::to_value(program.i3_private_projection_snapshot().unwrap()).unwrap();
        let before = sorted_owner_rmw_fragment_shapes(&program);
        assert_eq!(before.len(), 1);
        for kind in ["signature-only", "core-only"] {
            let mut altered = original.clone();
            let fields = owner_fields(&mut altered);
            if kind == "signature-only" {
                mutate_signature(&mut fields["signature"], "parameter-type");
            } else {
                mutate_payload(&mut fields["core"], "expression");
            }
            let changed = FabricProgram::from_i3_private_projection_snapshot(
                serde_json::from_value(altered).unwrap(),
            )
            .unwrap();
            assert_ne!(
                sorted_owner_rmw_fragment_shapes(&changed),
                before,
                "{kind} cannot disappear from actual patch shape"
            );
            assert!(
                !Sys4PatchCompatibility::between(&program, &changed).matches(),
                "actual patch compatibility must reject {kind}"
            );
        }
    }
    #[test]
    fn proof_first_invocation_mixed_producer_and_inventory_presentation_preserve_all_rows() {
        use crate::sys5_local_slice::{Sys5SourceInput, build_project};
        use mir_ast::surface_v0::FixtureSource;
        use mir_semantics::surface_v0_pipeline::private_snapshot::SnapshotCheckedEvaluationSignature;
        use mir_semantics::surface_v0_pipeline::{
            CheckedEvaluationKind, check_and_elaborate_surface_v0,
        };
        let source = include_str!("../../../samples/clean-near-end/mirrorea-i2-local-toy/main.mir");
        for params in [
            "target: Player, unused: Int, amount: Int",
            "target: Player, unused: Int, unused: Int",
        ] {
            let text = source.replace("target: Player)", &format!("{params})"));
            let checked = check_and_elaborate_surface_v0(FixtureSource::new(
                "tests/inline/invocation_mixed.mir",
                text.clone(),
            ))
            .unwrap();
            let project = build_project(Sys5SourceInput::inline(
                "tests/inline/invocation_mixed.mir",
                text,
            ))
            .unwrap();
            let (program, admission) = project
                .prepare_canonical_local_st_admission()
                .unwrap()
                .into_parts_for_sys4();
            assert_eq!(
                program.checked_program_identity(),
                checked.program_identity()
            );
            let image = serde_json::to_value(admission.i3_private_snapshot().unwrap()).unwrap();
            let plans = image["instance"]["owner_execution_plans"]
                .as_array()
                .unwrap();
            assert_eq!(plans.len(), 4);
            assert!(checked.static_environment().evaluation_signatures().len() > plans.len());
            for plan in plans {
                let original = checked
                    .static_environment()
                    .evaluation_signatures()
                    .iter()
                    .find(|signature| {
                        signature.kind() == CheckedEvaluationKind::OwnerRmw
                            && signature.name() == plan["evaluation"].as_str().unwrap()
                    })
                    .unwrap();
                assert_eq!(
                    plan["signature"],
                    serde_json::to_value(SnapshotCheckedEvaluationSignature::from_checked(
                        original
                    ))
                    .unwrap()
                );
            }
            let attack = plans.iter().find(|p| p["evaluation"] == "attack").unwrap();
            assert_eq!(
                attack["signature"]["parameters"].as_array().unwrap().len(),
                3
            );
            assert_ne!(
                attack["signature"]["parameters"][1]["source_ref"],
                attack["signature"]["parameters"][2]["source_ref"]
            );
            let mut reordered = image.clone();
            reordered["instance"]["owner_execution_plans"]
                .as_array_mut()
                .unwrap()
                .reverse();
            SealedFabricAdmission::from_i3_private_snapshot(serde_json::from_value(reordered).unwrap(),&program).expect("inventory presentation order does not change owner selection with these distinct operation names");
            for kind in ["missing", "extra", "duplicate"] {
                let mut changed = image.clone();
                let rows = changed["instance"]["owner_execution_plans"]
                    .as_array_mut()
                    .unwrap();
                if kind == "missing" {
                    rows.remove(0);
                } else {
                    let mut row = rows[0].clone();
                    if kind == "extra" {
                        row["evaluation"] = json!("extra");
                        row["signature"]["name"] = json!("extra");
                    }
                    rows.push(row);
                }
                assert!(
                    SealedFabricAdmission::from_i3_private_snapshot(
                        serde_json::from_value(changed).unwrap(),
                        &program
                    )
                    .is_err(),
                    "{kind}"
                );
            }
        }
    }
}

#[cfg(test)]
mod proof_first_invocation_handoff_tests {
    use super::*;
    use crate::proof_first_owner_schema_tests::{
        InvocationBodyProbe, invocation_source_fixture_at,
    };
    #[test]
    fn proof_first_invocation_handoff_preserves_full_argument_map() {
        let source =
            include_str!("../../mir-ast/tests/fixtures/surface-v0/sys4_ow1_endpoint_crossing.mir")
                .replace("target: Player)", "target: Player, unused: Int)")
                .replace(
                    "RouteUnavailable)",
                    "RouteUnavailable, DeadlineExpired) within owner_ticks 1",
                );
        let probe = InvocationBodyProbe::for_path("tests/inline/invocation_handoff.mir");
        for mutation in [
            "none",
            "value",
            "missing",
            "extra",
            "changed-and-reserialized",
            "unused-value",
            "unused-value-reserialized",
        ] {
            let (program, admission) =
                invocation_source_fixture_at(&source, "tests/inline/invocation_handoff.mir");
            let mut fabric =
                LocalFabric::bootstrap(program, admission, BackendProfile::St).unwrap();
            let before = fabric.proof_probe_actual_owner_state("S");
            fabric
                .install_i3_owner_admission_gate("invocation-test-runtime")
                .unwrap();
            let submission = fabric
                .submit_source_action(
                    SourceAction::owner_operation("attack")
                        .with_argument("target", "self")
                        .with_argument("unused", "7"),
                )
                .unwrap();
            let mut carrier = fabric
                .take_outbound_process_carrier("A", submission.envelope_id())
                .unwrap();
            let pending = fabric.i3_pending_owner_request_binding(&carrier).unwrap();
            let semantic = "invocation-test-semantic-request";
            let runtime = "invocation-test-runtime";
            let mut bytes =
                serde_json::to_vec(&carrier.i3_private_process_snapshot().unwrap()).unwrap();
            let issuance = fabric
                .stage_i3_owner_admission_issuance(
                    &pending,
                    &carrier,
                    semantic,
                    runtime,
                    bytes.clone(),
                )
                .unwrap();
            let permit = fabric
                .issue_i3_owner_admission_permit(
                    issuance,
                    &pending,
                    &carrier,
                    semantic,
                    runtime,
                    bytes.clone(),
                )
                .unwrap();
            if mutation != "none" {
                let MailboxPayload::OwnerRequest { arguments } = &mut carrier.envelope.payload
                else {
                    panic!("owner carrier required")
                };
                match mutation {
                    "value" | "changed-and-reserialized" => {
                        arguments.insert("target".into(), "target".into());
                    }
                    "unused-value" | "unused-value-reserialized" => {
                        arguments.insert("unused".into(), "8".into());
                    }
                    "missing" => {
                        arguments.remove("unused");
                    }
                    "extra" => {
                        arguments.insert("self".into(), "target".into());
                    }
                    _ => unreachable!(),
                }
                if matches!(
                    mutation,
                    "changed-and-reserialized" | "unused-value-reserialized"
                ) {
                    bytes = serde_json::to_vec(&carrier.i3_private_process_snapshot().unwrap())
                        .unwrap();
                }
            }
            probe.reset();
            let result = fabric.accept_i3_owner_request_with_permit(
                carrier, &pending, semantic, runtime, bytes, permit,
            );
            assert_eq!(result.is_ok(), mutation == "none", "{mutation}: {result:?}");
            if mutation == "none" {
                assert_eq!(probe.counts()[0], 1);
                assert_eq!(probe.counts()[3], 1);
                assert_eq!(
                    probe.argument_maps(),
                    vec![BTreeMap::from([
                        ("target".to_string(), "self".to_string()),
                        ("unused".to_string(), "7".to_string()),
                    ])],
                    "the consumed full map includes unchanged unused values"
                );
                assert_eq!(
                    fabric.semantic_snapshot().int("S", "player", "self", "hp"),
                    Some(90)
                );
                assert_eq!(
                    fabric
                        .semantic_snapshot()
                        .int("S", "player", "target", "hp"),
                    Some(200)
                );
            } else {
                assert_eq!(
                    probe.counts(),
                    [0; 5],
                    "{mutation} entered body/accessed owner state"
                );
                assert_eq!(fabric.proof_probe_actual_owner_state("S"), before);
                assert!(
                    fabric
                        .loci
                        .get("S")
                        .unwrap()
                        .incoming_mailbox
                        .pending
                        .is_empty()
                );
            }
        }
        // An annotated operation without the live handoff cannot reach M8.
        let (program, admission) =
            invocation_source_fixture_at(&source, "tests/inline/invocation_handoff.mir");
        let mut fabric = LocalFabric::bootstrap(program, admission, BackendProfile::St).unwrap();
        let before = fabric.proof_probe_actual_owner_state("S");
        probe.reset();
        assert!(
            fabric
                .dispatch_source_action(
                    SourceAction::owner_operation("attack")
                        .with_argument("target", "self")
                        .with_argument("unused", "7")
                )
                .is_err()
        );
        assert_eq!(probe.counts(), [0; 5]);
        assert_eq!(fabric.proof_probe_actual_owner_state("S"), before);
    }
}
