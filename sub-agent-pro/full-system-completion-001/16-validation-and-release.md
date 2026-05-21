# 16 — Validation and Release

## Always run

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Existing anchors to preserve

```bash
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/product_alpha1_installed_binary_check.py --format json check-all --out /tmp/mirrorea-alpha1-installed-binary-check
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
```

## New validation anchors

```bash
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/full_system_v1_samples.py check-all --format json
python3 scripts/posegraph_runtime_samples.py check-all --format json
python3 scripts/projection_v1_samples.py check-all --format json
python3 scripts/provider_admission_samples.py check-all --format json
python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release
```

## Cargo tests

Add focused tests and run affected crates.

At minimum as features land:

```bash
cargo test -p mir-ast --test textual_mir_alpha -- --nocapture
cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
cargo test -p mir-runtime --test projection_ir -- --nocapture
cargo test -p mir-runtime --test provider_admission -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
```

## Release command

Full V1 release check must produce:

- parser/checker report
- runtime report
- devtools viewer
- projection report
- provider admission report
- native host bundle
- sample outputs
- negative case outputs

## JSON expected principles

- positive rows must include explicit accepted evidence
- negative rows must include expected rejection code
- no absence-of-error as proof
- no planned row counted as runnable
- skip Docker paths are partial only
