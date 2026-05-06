# 11 — validation plan

## 1. Always run

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## 2. Existing product alpha CLI floor

Run if product sample touches product alpha CLI, schema, session, transport, devtools, bundle, or demo docs.

```bash
cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture
cargo test -p mir-runtime --test product_alpha1_session -- --nocapture
cargo test -p mir-runtime --test product_alpha1_transport_devtools -- --nocapture
cargo test -p mirrorea-cli --test alpha_cli -- --nocapture
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
```

If Docker is unavailable, do not claim release readiness for Docker path. Record skip reason.

## 3. Operational sample direct commands

If operational packages are executable, run:

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/world-core --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/membership-chat --format json
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/operational/sugoroku-world --format json
```

Then:

```bash
session_dir=$(mktemp -d /tmp/mirrorea-ops-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/operational/sugoroku-world --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- session 'session#operational-sugoroku' --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/debug-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/auth-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- attach 'session#operational-sugoroku' samples/product-alpha1/operational/packages/rate-limit-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- save 'session#operational-sugoroku' --savepoint 'savepoint#ops-r0' --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- quiescent-save 'session#operational-sugoroku' --savepoint 'savepoint#ops-r2' --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- load 'savepoint#ops-r0' --session 'session#operational-sugoroku' --format json
```

If session id differs, update docs/tests consistently.

## 4. Transport

```bash
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- transport 'session#operational-sugoroku' --mode local --format json
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- transport 'session#operational-sugoroku' --mode docker --format json
```

Docker rules:

- If Docker succeeds, release evidence can include Docker path.
- If Docker skipped, mark as partial and do not claim Docker operational readiness.

## 5. Devtools

```bash
viewer_dir=$(mktemp -d /tmp/mirrorea-ops-viewer-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$session_dir" cargo run -q -p mirrorea-cli -- export-devtools 'session#operational-sugoroku' --out "$viewer_dir" --format json
cargo run -q -p mirrorea-cli -- view "$viewer_dir" --check --format json
```

Check for:

- event DAG panel
- route panel
- membership/config panel
- witness panel
- hot-plug lifecycle panel
- save/load panel
- observer-safe redaction marker

## 6. Native host bundle

```bash
bundle_dir=$(mktemp -d /tmp/mirrorea-ops-bundle-XXXXXX)
cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/operational/sugoroku-world --out "$bundle_dir" --format json
sh "$bundle_dir/run.sh" check
sh "$bundle_dir/run.sh" view
```

Check manifest for:

- direct_mir_to_machine_code_supported = false
- arbitrary_native_execution_supported = false
- package_native_execution_claimed = false
- signature_is_safety_claimed = false
- NativeExecutionPolicy = Disabled unless explicitly changed

## 7. Orchestration script

If `scripts/operational_product_samples.py` is added:

```bash
python3 scripts/operational_product_samples.py list --format json
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/operational_product_samples.py closeout --format json
python3 -m unittest scripts.tests.test_operational_product_samples
```

## 8. Semantic JSON checks

Do not rely only on command exit status. Validate payload fields:

- verdict accepted/rejected/deferred as expected
- session id present
- event DAG non-empty
- host-I/O or game action present
- auth/capability/membership/witness lanes separate
- savepoint class correct
- quiescent flags present for R2
- observer-safe view does not include raw witness/auth evidence
- native bundle non-claims present

## 9. Validation report language

Use exact wording:

```text
passed in current local environment
Docker passed / Docker skipped with non-claim
non-final product alpha operational sample
not final public product
not direct native codegen
```
