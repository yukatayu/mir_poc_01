# practical-alpha1 packages

Each subdirectory in this root is a narrow practical alpha-1 package fixture.

- The current entrypoint is `package.mir.json`.
- The loader may receive either the package directory or the file path itself.
- `SRC-*` fixtures are parser/loader inputs.
- `CHK-*` fixtures are parser/loader inputs plus `alpha_local_checker_input` for the first practical checker floor.
- `RUN-*` fixtures are parser/loader inputs plus `alpha_local_checker_input` and `alpha_local_runtime_input` for the first practical local-runtime floor.
- `HP-A1-*` fixtures are parser/loader inputs plus `alpha_local_hotplug_input` and `manifest.attach_profile` for the first practical hot-plug floor.
- `alpha_local_checker_input` is sample/test expectation input for the non-final alpha-local checker floor, not a public package schema freeze.
- `alpha_local_runtime_input` is sample/test expectation input for the non-final runtime-plan/local-runtime floor, not a public runtime schema freeze.
- `alpha_local_hotplug_input` is sample/test expectation input for the non-final hotplug-plan/hot-plug-report floor, not a public package/hot-plug ABI freeze.
- `alpha_local_hotplug_input` currently carries attach-time package-admission lanes such as `operation_kind`, `membership_epoch`, `member_incarnation`, `required_witness_refs`, `pre_attach_membership_advances`, and detach-only `detach_boundary_ref`.
- `RUN-*` fixtures still require a positive checker floor before runtime lowering; they do not bypass `P-A1-02`.
- `HP-A1-*` fixtures still require manifest-driven admission and a distinct hotplug-plan boundary; they do not bypass the practical front-door or collapse into the Alpha-0 Stage-D evidence lane.
- `HP-A1-06` is a narrow object package attach preview seam through `manifest.attach_profile = PlaceholderAvatarObjectPackage`; it is not final object attach completion.
- `HP-A1-07` is an explicit deferred detach minimal contract boundary; it is not detach runtime lifecycle execution, rollback, or migration completion.
- These fixtures are not current-scope alpha evidence sidecars.
