# Product α-1 sample and validation matrix

## Product demo rows

### Source/front-door

- `SRC-PROD-01`: product demo package loads.
- `SRC-PROD-02`: invalid package schema rejected.
- `SRC-PROD-03`: unsupported textual `.mir` direct input returns explicit diagnostic, not silent failure.

### Checker

- `CHK-PROD-01`: product package accepted with explicit obligations.
- `CHK-PROD-02`: raw dangling reject.
- `CHK-PROD-03`: missing capability reject.
- `CHK-PROD-04`: missing witness reject.
- `CHK-PROD-05`: undeclared effect/failure reject.
- `CHK-PROD-06`: invalid distributed cut reject.
- `CHK-PROD-07`: message failure policy missing reject.
- `CHK-PROD-08`: observer-safe redaction violation reject.

### Runtime

- `RUN-PROD-01`: local product session accepted.
- `RUN-PROD-02`: `AddOne` host-I/O returns 42 from 41.
- `RUN-PROD-03`: stale membership rejected before mutation.
- `RUN-PROD-04`: missing witness rejected before mutation.

### Hot-plug

- `HP-PROD-01`: debug layer attach accepted.
- `HP-PROD-02`: auth layer attach via contract update.
- `HP-PROD-03`: rate-limit layer attach with declared failure.
- `HP-PROD-04`: object/avatar placeholder attach visible.
- `HP-PROD-05`: unsupported runtime fallback visible.
- `HP-PROD-06`: incompatible patch rejected.
- `HP-PROD-07`: detach deferred/rejected/accepted boundary visible.

### Transport

- `TR-PROD-01`: local transport accepted.
- `TR-PROD-02`: Docker transport accepted.
- `TR-PROD-03`: stale membership rejected over transport.
- `TR-PROD-04`: observer-safe route trace.

### Save/load

- `SAVE-PROD-01`: local save/load resume.
- `SAVE-PROD-02`: stale membership non-resurrection.
- `SAVE-PROD-03`: invalid distributed cut preflight reject.
- `SAVE-PROD-04`: quiescent save success.
- `SAVE-PROD-05`: post-seal send rejected/deferred.
- `SAVE-PROD-06`: in-flight not drained => quiescent save reject.

### Devtools

- `VIS-PROD-01`: event DAG panel.
- `VIS-PROD-02`: place graph panel.
- `VIS-PROD-03`: route trace panel.
- `VIS-PROD-04`: membership/config timeline.
- `VIS-PROD-05`: witness timeline.
- `VIS-PROD-06`: hot-plug lifecycle.
- `VIS-PROD-07`: save/load + quiescent save.
- `VIS-PROD-08`: message failure/recovery.
- `VIS-PROD-09`: fallback degradation.
- `VIS-PROD-10`: auth/capability decisions.
- `VIS-PROD-11`: observer-safe redaction leak test.
- `VIS-PROD-12`: viewer openability.

### Native bundle

- `NATIVE-PROD-01`: build native launch bundle.
- `NATIVE-PROD-02`: bundle run script works.
- `NATIVE-PROD-03`: manifest has non-claims.
- `NATIVE-PROD-04`: direct Mir native codegen unsupported diagnostic.

## Release validation commands

Representative final validation:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check

cargo test -p mir-ast -- --nocapture
cargo test -p mir-runtime -- --nocapture
cargo test -p mirrorea-core -- --nocapture
cargo test -p mirrorea-cli -- --nocapture

cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
cargo run -q -p mirrorea-cli -- demo --out /tmp/mirrorea-alpha1-demo --format json
cargo run -q -p mirrorea-cli -- export-devtools /tmp/mirrorea-alpha1-demo/session --out /tmp/mirrorea-alpha1-devtools --format json
cargo run -q -p mirrorea-cli -- quiescent-save /tmp/mirrorea-alpha1-demo/session --format json
cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/demo --out /tmp/mirrorea-alpha1-bundle --format json

python3 scripts/product_alpha1_release_check.py check-all --format json
```

If actual command names differ, docs must use actual implemented names.

## Release report

The release report must show:

- all rows;
- commands;
- pass/fail;
- non-goals;
- public alpha statement;
- known limitations;
- exact commit.
