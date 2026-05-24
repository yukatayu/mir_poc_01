# Surface Mir Alpha 01

## purpose

This guide is the reader-facing entry for the Surface Mir alpha line.

`P-SURF-01` adds the parser floor. `P-SURF-02` adds the indexed-state semantic
checker floor. `P-SURF-03` adds the Surface-to-Core elaboration evidence floor.
`P-SURF-04` adds the generated communication evidence floor. `P-SURF-05` adds
the report-level role admission / capability grant evidence floor.
Runtime rows and operational `samples/full-system-v1-surface/` families outside
the current `syntax/`, `indexed-state/`, `elaboration/`, and `role-admission/`
evidence roots remain planned for later packages.

## current decision

Canonical place-scope syntax:

```mir
S {
  state player[p: Participant]: Player
}
```

Rejected syntax:

```mir
S[
  state player[p: Participant]: Player
]
```

`S[ ... ]` is not sugar. `[]` is reserved for value-level indexing such as:

```mir
player[self]
array[i]
Participant[self] {
  when start fails MissingCapability { ... }
}
```

## source authority

The Surface Mir alpha source flow is:

```text
.mir source files
  -> parse
  -> typecheck
  -> elaborate to Core Mir
  -> optional generated package artifact / package.mir.json
  -> runtime / devtools / release evidence
```

`package.mir.json` remains an alpha artifact and compatibility surface. It is
not semantic source authority.

## key boundaries

- Surface Mir is user-facing source.
- Core Mir is elaboration target for checker / runtime / devtools.
- generated communication, publish, and observe edges must be visible in Core IR
  and devtools.
- `state player[p: Participant]: Player` is an S-owned indexed state map.
- `p` is a key, not owner authority.
- role claim is not authority; authority is capability grant through admission.
- capability refs must keep admission/principal/incarnation lineage; copied refs
  are not authority.
- nested foreign `S { ... }` blocks generate owner-directed requests; they are
  not ambient authority switches.
- source patch hot-plug is not eval.

## planned package sequence

```text
P-SURF-01 parser
P-SURF-02 indexed state
P-SURF-03 elaboration
P-SURF-04 auto communication
P-SURF-05 role admission
P-SURF-06 source patch
P-SURF-07 source operational suite
P-SURF-08 devtools
P-SURF-99 audit
```

## current validation

For the current parser, indexed-state checker, elaboration, generated
communication, and role-admission floors:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
cargo test -p mir-ast --test surface_mir_parser -- --nocapture
cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo test -p mir-semantics --test role_admission_capability_grant -- --nocapture
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release
```

Compatibility anchors when environment permits:

```bash
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
```

## source commands

These commands are runnable for the P-SURF-01 parser floor, P-SURF-02
indexed-state semantic checker floor, P-SURF-03 elaboration evidence floor, and
P-SURF-04 generated communication evidence floor, and P-SURF-05 role admission
evidence floor:

```bash
python3 scripts/surface_mir_samples.py matrix --format json
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release
cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
cargo test -p mir-semantics --test role_admission_capability_grant -- --nocapture
```

## non-claims

- no final public grammar.
- no Surface runtime/helper implementation beyond the parser, indexed-state
  checker, elaboration, generated communication, and role-admission evidence
  floors yet.
- no production identity provider, hardware attestation, or WAN admission.
- no final ABI / SDK.
- no production WAN/federation.
- no distributed durable save-load R3/R4.
- no arbitrary native/WASM/Unity/UE provider execution.
