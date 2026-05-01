# 08 — Comprehensive sample matrix

Each sample should eventually have:

- source sample, likely `.mir` or alpha source-ish syntax
- expected static verdict sidecar
- expected runtime outcome if runnable
- expected event DAG fragment if executable
- diagnostics for negative samples
- debug/visualization expected output where relevant

Suggested sidecar name: `<sample>.expected.json`.

## 1. Lifetime / fallback samples

| ID | Name | Kind | Purpose | Expected |
|---|---|---|---|---|
| LIF-01 | raw_dangling_reference_rejected | negative static | longer context stores shorter raw ref | static error |
| LIF-02 | fallback_extends_access_path | positive | `c > a` lets access degrade | valid, canonical chain `c>a` |
| LIF-03 | nested_inherit_chain_valid | positive | explicit chain inheritance | valid, canonical `e>b>a` |
| LIF-04 | plain_ref_does_not_inherit | positive/negative distinction | `ref(d)>a` does not use `d`'s internal fallback | valid, `d` expiry => `a` |
| LIF-05 | underdeclared_fallback_static_error | negative static | missing lineage evidence | static error |
| LIF-06 | incompatible_access_target_rejected | negative static | unrelated targets in same chain | static reject |
| LIF-07 | capability_promotion_rejected | negative static | fallback to stronger capability | static reject |
| LIF-08 | write_after_read_only_fallback_rejected | negative runtime/static | selected fallback is read-only | Reject or static error |
| LIF-09 | rollback_cannot_resurrect_lease | negative semantic | try rollback after lease expiry | lease remains expired |
| LIF-10 | atomic_cut_no_repromotion | negative semantic | degrade before/around cut | no earlier option resurrection |
| LIF-11 | bird_sparkle_anchor_inheritance | positive | sparkle inherits bird anchor chain | bird deletion keeps sparkle on FriendHead |
| LIF-12 | bird_sparkle_no_inheritance | negative/contrast | sparkle refs Bird object only | bird deletion => WorldOrigin fallback |
| LIF-13 | snapshot_selected_anchor | positive | snapshot target but not full chain | FriendHead>A, not FriendHead>Shoulder>A |
| LIF-14 | remote_observed_ref_stale_membership | negative runtime | remote value stale epoch | Reject(StaleMembership) |
| LIF-15 | remote_read_only_covariant | positive | read-only remote reference covariance | valid |

## 2. Contract variance / layer insertion samples

| ID | Name | Kind | Purpose | Expected |
|---|---|---|---|---|
| VAR-01 | logging_layer_valid | positive | observe/log only | valid overlay |
| VAR-02 | precondition_strengthening_rejected | negative static | layer requires admin beyond member | reject transparent overlay |
| VAR-03 | postcondition_weakening_rejected | negative static | delivery guarantee weakened | reject |
| VAR-04 | output_covariance_valid | positive | stronger output subtype | valid |
| VAR-05 | mutable_covariance_rejected | negative static | Java-like mutable variance problem | reject |
| VAR-06 | readonly_covariance_valid | positive | read-only covariance | valid |
| VAR-07 | failure_row_widening_rejected | negative static | RateLimited added undeclared | reject |
| VAR-08 | ratelimit_declared_failure_valid | positive/runtime | declared RateLimited failure | valid, runtime Reject(RateLimited) |
| VAR-09 | effect_row_widening_rejected | negative static | undeclared external telemetry effect | reject |
| VAR-10 | cost_degradation_rejected | negative static | latency/resource bound worsened | reject unless declared |
| VAR-11 | redaction_layer_valid | positive | observer-safe redaction | valid |
| VAR-12 | debug_layer_requires_authority | negative runtime/static | non-admin attaches debug layer | reject |
| VAR-13 | auth_layer_contract_update_valid | positive | auth added by versioned contract update | valid with activation cut |
| VAR-14 | adapter_transform_preserves_contract | positive | adapter transforms but preserves postcondition | valid |
| VAR-15 | hidden_shadowing_rejected | negative static | overlay removes interface | reject |

## 3. Cut / save / load samples

| ID | Name | Kind | Purpose | Expected |
|---|---|---|---|---|
| CUT-01 | local_try_rollback_before_cut | positive | rollback local state pre-cut | succeeds |
| CUT-02 | rollback_across_atomic_cut_rejected | negative semantic | rollback crosses cut | pre-cut state remains |
| CUT-03 | irreversible_effect_in_rollback_rejected | negative static/runtime | rollback region contains irreversible effect | reject/require compensation |
| CUT-04 | local_save_load_valid | positive | single Place snapshot | load restores local state |
| CUT-05 | inconsistent_distributed_snapshot_rejected | negative | receive included, send omitted | invalid cut |
| CUT-06 | inflight_message_channel_state_valid | positive | send included, receive omitted, channel state saved | valid cut |
| CUT-07 | observe_without_publish_rejected | negative | observe included, publish omitted | invalid cut |
| CUT-08 | witness_use_without_create_rejected | negative | witness use included, create omitted | invalid cut |
| CUT-09 | hotplug_activation_without_request_rejected | negative | activation included without request/verdict | invalid cut |
| CUT-10 | load_does_not_resurrect_expired_lease | negative semantic | saved/load stale lease | invalid or remains expired |
| CUT-11 | zcycle_checkpoint_invalid | negative | checkpoint dependency Z-cycle | unusable checkpoint |
| CUT-12 | forced_checkpoint_breaks_zcycle | positive if implemented | communication-induced checkpoint | valid |
| CUT-13 | durable_cut_deferred_in_mir0 | negative static | use durable_cut in Mir-0 | rejected/deferred diagnostic |

## 4. Place / network samples

| ID | Name | Kind | Purpose | Expected |
|---|---|---|---|---|
| NET-01 | local_queue_parity | positive | existing Sugoroku loopback parity | valid |
| NET-02 | docker_two_process_envelope | positive E2E | world + participant containers | envelope delivered |
| NET-03 | stale_membership_rejected | negative E2E | old epoch/incarnation | Reject(StaleMembership) |
| NET-04 | missing_capability_rejected | negative E2E | no capability | Reject(MissingCapability) |
| NET-05 | missing_witness_rejected | negative E2E | witness absent | Reject(MissingWitness) |
| NET-06 | route_rebinding_no_shadow | positive | route changes but interface preserved | valid |
| NET-07 | observer_safe_route_trace | positive | redacted debug output | no raw secret/witness |
| NET-08 | network_partition_explicit_failure | negative/positive | partition causes typed failure | Reject(NetworkUnavailable) or Deferred |
| NET-09 | auth_evidence_lane_preserved | positive | auth not collapsed into transport | envelope lanes preserved |
| NET-10 | transport_medium_change_preserves_contract | positive | switch local_queue/TCP | valid if contract unchanged |

## 5. Hot-plug samples

| ID | Name | Kind | Purpose | Expected |
|---|---|---|---|---|
| HP-01 | attach_sugoroku_to_empty_world | positive | runtime attach module | accepted + activation cut |
| HP-02 | attach_debug_layer_admin | positive | debug layer attach | accepted |
| HP-03 | attach_debug_layer_non_admin_rejected | negative | authority missing | rejected |
| HP-04 | attach_auth_layer_contract_valid | positive | auth with declared failure/contract update | accepted |
| HP-05 | attach_ratelimit_layer_contract_valid | positive | rate-limit with declared failure | accepted |
| HP-06 | incompatible_patch_rejected | negative | compatibility failure | rejected |
| HP-07 | missing_capability_hotplug_rejected | negative | cap missing | rejected/deferred |
| HP-08 | detach_object_no_dangling | positive | detach degrades references | valid |
| HP-09 | detach_runtime_with_dependents_rejected | negative | active dependents | rejected/deferred |
| HP-10 | runtime_library_hotplug | positive | package attach | accepted |
| HP-11 | unsigned_native_package_rejected | negative | native untrusted | rejected |
| HP-12 | signed_overcapability_package_rejected | negative | signature but excessive caps | rejected |
| HP-13 | activation_cut_prevents_hidden_rollback | negative semantic | rollback after activation | no hidden detach |
| HP-14 | hotplug_verdict_mismatch_rejected | negative | verdict request_ref mismatch | rejected |

## 6. Avatar/runtime package samples

| ID | Name | Kind | Purpose | Expected |
|---|---|---|---|---|
| AV-01 | placeholder_avatar_runtime | positive | core placeholder role | accepted |
| AV-02 | custom_mir_avatar_runtime | positive | Mir script runtime package | accepted |
| AV-03 | vrm_adapter_skeleton | planned/positive | adapter boundary only | accepted as limited/planned |
| AV-04 | vrchat_compat_adapter_skeleton | planned/positive | compatibility boundary only | accepted as limited/planned |
| AV-05 | unsupported_shader_fallback | positive | shader unsupported | fallback material |
| AV-06 | untrusted_native_avatar_rejected | negative | native untrusted | rejected |
| AV-07 | trusted_native_sandbox_limited | positive/limited | sandboxed external provider | accepted with limited caps |
| AV-08 | runtime_unavailable_placeholder | positive | no runtime support | placeholder fallback |
| AV-09 | adapter_attempts_undeclared_effect | negative | effect row violation | rejected |
| AV-10 | package_detach_active_avatar_deferred | negative/runtime | active dependent object | deferred/rejected |

## 7. Visualization/devtools samples

| ID | Name | Kind | Purpose | Expected |
|---|---|---|---|---|
| VIS-01 | event_dag_export | positive | event graph | JSON/HTML view |
| VIS-02 | place_graph_export | positive | Place graph | view |
| VIS-03 | route_trace_export | positive | envelope route | view |
| VIS-04 | witness_timeline | positive | witness create/use | view |
| VIS-05 | membership_timeline | positive | epoch/incarnation | view |
| VIS-06 | hotplug_lifecycle_view | positive | request/verdict/activation | view |
| VIS-07 | fallback_degradation_view | positive | option degradation | view |
| VIS-08 | observer_redacted_view | positive | safe observer | no raw high-label data |
| VIS-09 | admin_full_view | positive | privileged view | full allowed detail |
| VIS-10 | on_demand_trace_only | positive/perf | tracing only after attach | no trace before attach |
| VIS-11 | retention_policy_enforced | negative/positive | telemetry retention | no over-retention |
| VIS-12 | debug_layer_detach_stops_trace | positive | detach debug layer | telemetry stops |

## 8. End-to-end samples

| ID | Name | Kind | Purpose | Expected |
|---|---|---|---|---|
| E2E-01 | local_integrated_sugoroku | positive | local runtime | roll/publish/handoff |
| E2E-02 | docker_two_node_sugoroku | positive | network E2E | envelope over Docker network |
| E2E-03 | hotplug_debug_layer_runtime | positive | live layer insert | trace begins after attach |
| E2E-04 | hotplug_ratelimit_runtime | positive/negative | rate-limit behavior | Reject(RateLimited) |
| E2E-05 | avatar_runtime_package | positive | runtime package attach | placeholder/custom avatar |
| E2E-06 | local_save_load_continue | positive | local save/load | resume local state |
| E2E-07 | distributed_inconsistent_save_negative | negative | invalid cut | rejected |
| E2E-08 | reversed_library_seed_demo | non-final positive | two knowledge rooms + portal | no claim full app |
| E2E-09 | layer_auth_then_hotplug | positive/negative | auth layer affects hot-plug | correct contract handling |
| E2E-10 | package_missing_runtime_fallback | positive | browser-like fallback | placeholder |

## 9. Runner expectations

Initial runners may be helper-local. Eventually add:

```bash
python3 scripts/alpha_lifetime_fallback_samples.py check-all --format json
python3 scripts/alpha_contract_variance_samples.py check-all --format json
python3 scripts/alpha_cut_save_load_samples.py check-all --format json
python3 scripts/alpha_hotplug_runtime_samples.py check-all --format json
python3 scripts/alpha_network_docker_samples.py check-all --format json
python3 scripts/alpha_avatar_runtime_samples.py check-all --format json
python3 scripts/alpha_visualization_samples.py check-all --format json
python3 scripts/alpha_e2e_samples.py check-all --format json
```

Do not mark rows 100% until implementation, positive/negative samples, docs, report, validation, and commit/push are complete for that row's current scope.
