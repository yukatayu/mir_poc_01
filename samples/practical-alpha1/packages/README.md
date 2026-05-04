# practical-alpha1 packages

Each subdirectory in this root is a narrow practical alpha-1 package fixture.

- The current entrypoint is `package.mir.json`.
- The loader may receive either the package directory or the file path itself.
- `SRC-*` fixtures are parser/loader inputs.
- `CHK-*` fixtures are parser/loader inputs plus `alpha_local_checker_input` for the first practical checker floor.
- `RUN-*` fixtures are parser/loader inputs plus `alpha_local_checker_input` and `alpha_local_runtime_input` for the first practical local-runtime floor.
- `HP-A1-*` fixtures are parser/loader inputs plus `alpha_local_hotplug_input` and `manifest.attach_profile` for the first practical hot-plug floor.
- `AV-A1-*` fixtures are parser/loader inputs plus `alpha_local_hotplug_input` and `manifest.attach_profile` for the first practical avatar preview companion floor.
- `TR-A1-*` fixtures are parser/loader inputs plus `alpha_local_checker_input` and `alpha_local_transport_input` for the first practical transport floor.
- `SL-A1-*` fixtures are parser/loader inputs plus `alpha_local_checker_input`, `alpha_local_runtime_input`, and `alpha_local_save_load_input` for the first practical local save/load floor.
- `PE2E-*` preview manifests live under `../previews/` and reference these fixtures plus exact practical reports for the first practical product-preview floor.
- `alpha_local_checker_input` is sample/test expectation input for the non-final alpha-local checker floor, not a public package schema freeze.
- `alpha_local_runtime_input` is sample/test expectation input for the non-final runtime-plan/local-runtime floor, not a public runtime schema freeze.
- `alpha_local_hotplug_input` is sample/test expectation input for the non-final hotplug-plan/hot-plug-report floor, not a public package/hot-plug ABI freeze.
- `alpha_local_transport_input` is sample/test expectation input for the non-final transport-plan/transport-report floor, not a public transport ABI freeze.
- `alpha_local_save_load_input` is sample/test expectation input for the non-final save-load-plan/saved-frontier/save-load-report floor, not a public save-load ABI freeze.
- `alpha_local_hotplug_input` currently carries attach-time package-admission lanes such as `operation_kind`, `membership_epoch`, `member_incarnation`, `required_witness_refs`, `pre_attach_membership_advances`, and detach-only `detach_boundary_ref`.
- `alpha_local_transport_input` currently carries practical transport lanes such as `transport_surface`, `bridge_kind`, `runtime_places`, `bootstrap_participants`, `request_envelope`, `required_capabilities`, `required_witness_refs`, `auth_bindings_required`, and `pre_dispatch_membership_advances`.
- `alpha_local_save_load_input` currently carries practical save/load lanes such as `scenario_kind`, `required_base_terminal_outcome`, saved-frontier history/owner requirements, `resumed_dispatch_program`, `resumed_envelope`, and `post_restore_membership_advances`.
- `RUN-*` fixtures still require a positive checker floor before runtime lowering; they do not bypass `P-A1-02`.
- `HP-A1-*` fixtures still require manifest-driven admission and a distinct hotplug-plan boundary; they do not bypass the practical front-door or collapse into the Alpha-0 Stage-D evidence lane.
- `AV-A1-*` fixtures still require the practical front-door, a distinct hotplug-plan boundary, and exact hot-plug source reports; they do not bypass the practical front-door, collapse into the Alpha-0 Stage-D evidence lane, or claim native execution.
- `TR-A1-*` fixtures still require a positive checker floor and a distinct transport-plan boundary; they do not bypass the practical front-door or collapse into the Alpha-0 Stage-C evidence lane.
- `SL-A1-*` fixtures still require a positive checker floor, a distinct runtime-plan boundary, and one exact practical local-runtime frontier; they do not bypass the practical front-door or collapse into the Alpha-0 Stage-B save/load supporting subset.
- `PE2E-*` preview manifests still require the practical front-door fixtures and exact expected reports/devtools bundles; they do not collapse practical runtime / hot-plug / transport / save-load carriers into a monolithic product runtime.
- `HP-A1-06` is a narrow object package attach preview seam through `manifest.attach_profile = PlaceholderAvatarObjectPackage`; it is not final object attach completion.
- `HP-A1-07` is an explicit deferred detach minimal contract boundary; it is not detach runtime lifecycle execution, rollback, or migration completion.
- `AV-A1-01/02/03` are non-final practical avatar preview rows only; they are not native execution, final avatar package ABI completion, same-session product runtime completion, or VRM / VRChat / Unity compatibility.
- `TR-A1-01..07` are non-final practical transport rows only; they are not WAN/federation, local save/load, devtools export, product prototype, or final public transport ABI completion.
- `SL-A1-01/02` are non-final practical local save/load rows only; they are not distributed durable save/load, stale witness/stale lease non-resurrection completion, queue/channel/transport persistence, product prototype, or final public save-load ABI completion.
- These fixtures are not current-scope alpha evidence sidecars.
