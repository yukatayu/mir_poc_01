# Product Alpha-1 Hands-On

この guide は、clean clone した外部開発者が Product/Public-ready Mirrorea Spaces alpha-1 の **alpha-stable** toolchain を再現するための入口です。

これは final public product ではありません。final textual `.mir` grammar、final public ABI、WAN/federation、distributed durable save/load、arbitrary native execution、signature-is-safety、final public viewer / telemetry service は non-goal です。

## Prerequisites

- Rust toolchain with Cargo
- Python 3
- Docker and Docker Compose for the Docker TCP transport path
- POSIX shell for generated native launch bundle `run.sh`

## Validate The Repository

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
cargo test -p mir-ast -- --nocapture
cargo test -p mir-runtime -- --nocapture
cargo test -p mirrorea-core -- --nocapture
cargo test -p mirrorea-cli -- --nocapture
```

## Run The Product Demo

The shortest release-candidate command is:

```bash
demo_dir=$(mktemp -d /tmp/mirrorea-alpha1-demo-XXXXXX)
cargo run -q -p mirrorea-cli -- demo --out "$demo_dir" --format json
```

The demo writes:

- `reports/check.json`
- `reports/run-local.json`
- `reports/attach-*.json`
- `reports/save.json`
- `reports/load.json`
- `reports/quiescent-save.json`
- `reports/transport-local.json`
- `reports/transport-docker.json` when Docker is enabled
- `reports/export-devtools.json`
- `reports/view.json`
- `reports/build-native-bundle.json`
- `reports/demo.json`
- `devtools/index.html`
- `devtools/bundle.json`
- `sessions/session-*-observer-safe.json`
- `session-store/*.session.json`
- `native-bundle/`

`sessions/` is the observer-safe demo artifact. `session-store/` is the local admin/debug session store used to reopen the same carrier with `MIRROREA_ALPHA_SESSION_DIR`; it is not an observer-safe publication artifact.

If Docker is unavailable, use the local-only probe and record the Docker path as an environment-gated non-claim. The JSON status is `partial`, and `product_alpha1_release_candidate_ready` remains `false`:

```bash
cargo run -q -p mirrorea-cli -- demo --out /tmp/mirrorea-alpha1-demo-local --skip-docker --format json
```

## Inspect The Viewer

Open the generated static viewer:

```text
$demo_dir/devtools/index.html
```

The viewer is non-final. It renders concrete JSON records from the exported session bundle: overview, redacted place graph, event DAG, routes, membership frontier counts, witness relation summary, hot-plug lifecycle, save/load timeline, message recovery, fallback, auth/capability decision counts, redaction, and retention. Observer-safe mode must not expose raw witness/auth/capability secrets; the local `session-store/` remains an admin/debug replay artifact.

## Run The Command Family Manually

```bash
cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json
tmpdir=$(mktemp -d /tmp/mirrorea-alpha1-session-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- session 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/debug-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/auth-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/rate-limit-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/placeholder-object --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/custom-avatar-preview --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- save 'session#product-alpha1-demo' --savepoint 'savepoint#r0-release' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- load 'savepoint#r0-release' --session 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- quiescent-save 'session#product-alpha1-demo' --savepoint 'savepoint#r2-release' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode local --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode docker --format json
viewer_dir=$(mktemp -d /tmp/mirrorea-alpha1-viewer-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- export-devtools 'session#product-alpha1-demo' --out "$viewer_dir" --format json
cargo run -q -p mirrorea-cli -- view "$viewer_dir" --check --format json
bundle_dir=$(mktemp -d /tmp/mirrorea-alpha1-bundle-XXXXXX)
cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/demo --out "$bundle_dir" --format json
sh "$bundle_dir/run.sh" check
sh "$bundle_dir/run.sh" view
```

## Create A Small Variant

Copy the demo package to a temporary directory and edit `runtime_input.host_io.request_payload.value`. The `expected_response.value` must remain exactly request plus one.

```bash
variant=$(mktemp -d /tmp/mirrorea-alpha1-variant-XXXXXX)
cp -R samples/product-alpha1/demo/. "$variant/"
python3 - "$variant/package.mir.json" <<'PY'
import json, pathlib, sys
path = pathlib.Path(sys.argv[1])
payload = json.loads(path.read_text())
payload["runtime_input"]["host_io"]["request_payload"]["value"] = 7
payload["runtime_input"]["host_io"]["expected_response"]["value"] = 8
path.write_text(json.dumps(payload, indent=2) + "\n")
PY
cargo run -q -p mirrorea-cli -- check "$variant" --format json
variant_demo=$(mktemp -d /tmp/mirrorea-alpha1-variant-demo-XXXXXX)
cargo run -q -p mirrorea-cli -- demo "$variant" --out "$variant_demo" --skip-docker --format json
```

The variant command above is a local probe unless Docker is included and the required product attach matrix is preserved.

## Build The Native Launch Bundle

```bash
bundle_dir=$(mktemp -d /tmp/mirrorea-alpha1-bundle-XXXXXX)
cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/demo --out "$bundle_dir" --format json
sh "$bundle_dir/run.sh" check
sh "$bundle_dir/run.sh" view
```

The bundle is a host launch bundle around the compiled Rust CLI and versioned package files. It is not package-native execution and does not convert Mir source to machine code.

## Release Check

```bash
release_dir=$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX)
python3 scripts/product_alpha1_release_check.py --format json check-all --out "$release_dir"
```

Use `--skip-docker` only when Docker / Docker Compose are unavailable. It is a local probe, exits with a non-release status in release check, and must not be recorded as release-candidate evidence.
