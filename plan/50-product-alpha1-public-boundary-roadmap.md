# plan/50 — product alpha-1 public boundary roadmap

## purpose

この文書は、Mirrorea Spaces **product/public-ready alpha-1** line の
repository-memory roadmap を置く。

規範判断の正本は `specs/25-product-alpha1-public-boundary.md` である。
この plan は、package sequence、alpha defaults、validation direction、non-claim inventory を保持する。

## current repo state

Current repo already has:

- practical alpha-1 first-floor source / checker / runtime / hot-plug / transport / save-load / devtools / product-preview evidence
- bounded operational alpha-0.5 local observable runtime workflow
- bounded operational alpha-0.8 same-session hot-plug runtime workflow
- bounded operational alpha-0.9 session-bound devtools workflow
- bounded practical alpha-1 integrated workflow carrier

Current repo still lacks product alpha-1:

- native launch bundle
- clean-clone release-candidate validation

Recent closed first cuts:

- local and Docker transport product command path
- product viewer UX over product demo bundle

## alpha defaults from `P-A1-25`

The alpha `U1` defaults for this line are:

| Concern | Alpha default | Non-claim |
|---|---|---|
| entrypoint | Rust CLI / binary `mirrorea-alpha` or equivalent crate command | final public CLI/API |
| package format | versioned `package.mir.json` alpha schema | final textual `.mir` grammar |
| host target | native process, local/Docker controlled environment | hosted service / browser / engine adapter |
| viewer | non-final static HTML or local viewer over JSON bundle | final telemetry service |
| native output | native launch bundle | direct Mir-to-machine-code |
| native execution | `NativeExecutionPolicy = Disabled` by default | arbitrary native package execution |

These are alpha work defaults, not final product decisions.

## product alpha-1 package line

### `P-A1-25` — product/public alpha-1 boundary recut

Status:

- actualized by `specs/25` and this roadmap

Delivered:

- product alpha definition
- alpha `U1` defaults
- product package sequence
- required CLI / package / runtime / transport / save / viewer / native bundle boundaries
- explicit non-goals

Non-claim:

- no product CLI implemented
- no product demo root added
- no product alpha workflow-ready claim

### `P-A1-26` — alpha CLI and package schema stabilization

Status:

- actualized by `crates/mirrorea-cli`, `crates/mir-ast::product_alpha1`, and `samples/product-alpha1/demo`

Target:

- add `mirrorea-cli` or equivalent Rust binary crate
- expose the canonical `mirrorea-alpha` command family:
  `check`, `run-local`, `session`, `attach`, `transport`, `save`, `load`,
  `quiescent-save`, `export-devtools`, `view`, `build-native-bundle`, `demo`
- define versioned package schema / compatibility policy
- add product demo package schema fixture root if needed for tests
- return explicit unsupported / not-yet-implemented diagnostics for commands whose behavior lands in later packages

Delivered:

- binary `mirrorea-alpha`
- product `package.mir.json` schema version `mirrorea-product-alpha1-v0`
- `check` command with explicit accepted evidence and residual obligations
- direct `.mir` non-goal diagnostic
- structured unsupported diagnostics for later command family
- `samples/product-alpha1/demo/` fixture root

Validation:

- `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture`
- `cargo test -p mirrorea-cli --test alpha_cli -- --nocapture`
- `cargo run -q -p mirrorea-cli -- check samples/product-alpha1/demo --format json`
- CLI command parsing tests
- command availability / unsupported diagnostic tests for the full alpha family
- product package schema tests
- docs / source hierarchy checks

Non-claim:

- no final CLI API
- no final textual grammar
- no full product demo runtime completion

### `P-A1-27` — product demo same-session runtime

Status:

- actualized by `crates/mir-runtime::product_alpha1_session`, `mirrorea-alpha run-local` / `session` / `attach`, and `samples/product-alpha1/demo`

Target:

- extend `samples/product-alpha1/demo/`
- connect check -> runtime plan -> run-local -> typed host-I/O -> hot-plug -> observe in one local file-backed session carrier
- preserve exact carrier split from practical and operational lines

Delivered:

- product same-session carrier with event DAG, membership, witness, route, hot-plug lifecycle, host-I/O, save-load placeholder, and message recovery state
- `mirrorea-alpha run-local` writes the product session to a local session store
- `mirrorea-alpha session` reads the same persisted session
- `mirrorea-alpha attach` mutates the same session with the debug-layer package
- core `MessageEnvelope`, `HotPlugRequest`, and `HotPlugVerdict` validation are used in the product runtime path
- typed host-I/O `AddOne` is represented as request/response observation in the same event DAG
- observer-safe export keeps raw witness/auth evidence redacted

Validation:

- product demo `run-local` / `session` / `attach` command path passes
- same-session state transition evidence exists
- membership / capability / witness / auth lanes remain separate in the route and attach decision surfaces
- save/load and message recovery state are carried but execution remains explicitly residual
- at this P-A1-27 closeout point, `transport --mode local` and `transport --mode docker` product paths remained release blockers for later packages

Non-claim:

- no distributed durable save/load
- no production WAN / federation
- no accepted detach execution unless explicitly implemented
- no local/Docker transport command completion in P-A1-27
- no quiescent-save execution
- no final product viewer or native launch bundle

### `P-A1-28` — message failure/recovery and quiescent save

Status:

- actualized by `crates/mir-runtime::product_alpha1_session`, `mirrorea-alpha save` / `load` / `quiescent-save`, and the product CLI/runtime tests

Target:

- introduce `MessageState`, `TransportContract`, `RecoveryPolicy`
- implement bounded R2 `quiescent-save` for controlled local session scope
- report `NoInFlight`, `AllPlacesSealed`, `NoPostCutSend`
- add positive and negative quiescent-save rows

Delivered:

- same-session `MessageState`, `TransportContract`, `RecoveryPolicy`, DAG-linked failure observation, and modal obligation rows for bounded timeout / retry-then-reject / reject behavior
- local R0 `save` and `load` commands over the product alpha-1 local session store
- bounded local R2 `quiescent-save` command with session-carried `NoInFlight`, `AllPlacesSealed`, and `NoPostCutSend` preflight/report flags
- rejected quiescent-save evidence when an in-flight message remains
- savepoint roundtrip evidence, load-admissibility rejection across stale membership / stale witness / stale auth-capability / accepted activation cuts, and same-session restore of runtime snapshot, event DAG, membership, witness, route graph, hot-plug lifecycle, auth/capability decisions, save-load state, and recovery state

Validation:

- quiescent save success
- post-seal send reject/defer
- in-flight not drained reject
- failure/recovery report rows
- R2 report shows the implemented R1/load-admissibility subset and the three quiescence flags
- stale membership / witness / lease non-resurrection status is explicit as implemented or residual
- provenance connectivity, package-version compatibility, and external-effect isolation/compensation status are explicit

Non-claim:

- no R3/R4 durable distributed save/load
- no exactly-once transport
- no WAN partition recovery

### `P-A1-29` — product local/Docker transport plus devtools viewer UX

Status:

- closed by `P-A1-29`
- workflow-ready first cut for same-session product transport and non-final viewer
- not product alpha release-ready until native bundle and release validation close

Target:

- product `transport --mode local|docker` command behavior over the same session carrier, keeping transport/auth/membership/capability/witness lanes separate
- product-level static HTML/local viewer
- product overview, place graph, event DAG, route graph, membership timeline, witness timeline, hot-plug lifecycle, save/quiescent-save, message recovery, fallback, auth/capability, redaction, retention panels
- observer-safe leak tests

Delivered:

- `crates/mir-runtime::product_alpha1_transport` adds same-session transport mutation reports, loopback TCP local round trip, Docker Compose TCP endpoint functions, explicit lane preservation, and WAN/federation non-claims.
- `samples/product-alpha1/docker/docker-compose.product-alpha1.yml` runs a world server and participant client over Docker Compose TCP with the mounted `mirrorea-alpha` binary and selected session JSON.
- `crates/mir-runtime::product_alpha1_devtools` exports a non-final product devtools JSON bundle and static HTML viewer over the product session carrier.
- `mirrorea-alpha transport`, `export-devtools`, and `view --check` are implemented. `build-native-bundle` and `demo` remain later-package unsupported diagnostics.

Validation:

- local transport command probe
- Docker Compose TCP transport command probe or explicit environment-gated skip with non-claim
- viewer openability
- JSON bundle structure check
- redaction tests
- active view role / redaction level / retention scope visible in viewer output
- explicit admin/debug `kept_later` marker if full admin view is not implemented
- panel-to-demo narrative check for product overview, place graph, event DAG, route graph, save/quiescent-save, message recovery, and auth/capability decisions

Non-claim:

- no final public viewer API
- no durable telemetry backend

### `P-A1-30` — native launch bundle

Target:

- `build-native-bundle`
- bundle compiled runtime/CLI, packages, viewer assets, reports, manifest, run script
- explicit native policy diagnostics

Validation:

- bundle builds
- run script check/demo paths work
- manifest carries non-claims
- manifest records `NativeExecutionPolicy = Disabled` unless a later package implements sandboxed execution
- bundle check rejects arbitrary native package execution
- signature/provenance is reported separately from semantic safety
- direct Mir-to-machine-code request returns an unsupported diagnostic

Non-claim:

- no direct Mir-to-machine-code
- no arbitrary native package execution
- no signature-is-safety claim

### `P-A1-31` — product alpha-1 release candidate

Target:

- clean clone guide
- `docs/hands_on/product_alpha1_01.md`
- `docs/research_abstract/product_alpha1_01.md`
- full product validation script
- release report

Validation:

- full product release validation floor
- clean worktree
- commit / push complete

Non-claim:

- still alpha, not final product
- final grammar / ABI / WAN / distributed durable save/load remain non-goals

## desired product sample root

Preferred layout:

```text
samples/product-alpha1/demo/
  README.md
  package.mir.json
  packages/
    debug-layer/
    auth-layer/
    rate-limit-layer/
    placeholder-object/
    custom-avatar-preview/
  expected/
```

This root must stay separate from:

- `samples/alpha/` alpha-0 evidence root
- `samples/practical-alpha1/` first-floor fixture root
- `samples/clean-near-end/` current-L2 active floor

## desired native launch bundle

Output shape:

```text
<out>/
  bin/mirrorea-alpha
  packages/
  devtools/
  reports/
  manifest.json
  run.sh
  README.md
```

The bundle is a host launch bundle.
It is not direct Mir native codegen.

## release validation direction

Representative release validation:

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
tmpdir=$(mktemp -d /tmp/mirrorea-alpha1-demo-XXXXXX)
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- session 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- attach 'session#product-alpha1-demo' samples/product-alpha1/demo/packages/debug-layer --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- demo --out /tmp/mirrorea-alpha1-demo --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode local --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- transport 'session#product-alpha1-demo' --mode docker --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- save 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- load latest --session 'session#product-alpha1-demo' --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- export-devtools 'session#product-alpha1-demo' --out /tmp/mirrorea-alpha1-devtools --format json
cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-alpha1-devtools --check --format json
MIRROREA_ALPHA_SESSION_DIR="$tmpdir" cargo run -q -p mirrorea-cli -- quiescent-save 'session#product-alpha1-demo' --format json
cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/demo --out /tmp/mirrorea-alpha1-bundle --format json
python3 scripts/product_alpha1_release_check.py check-all --format json
```

Actual command names may differ only if docs and validation scripts are updated together.

## blocker split

### self-driven implementation packages

- native launch bundle
- clean-clone validation docs

### research-discovery items

- exact breadth of product checker finite fragment
- whether accepted detach execution remains deferred or gains a bounded implementation
- how much admin/debug view is implemented vs explicitly kept-later

### user/final decision items

- final public grammar
- final public ABI
- hosted service vs installed product
- production WAN/federation
- distributed durable save/load
- engine adapter and avatar compatibility scope

## next reopen point

Next promoted package:

- `P-A1-30` native launch bundle

Queue authority remains `progress.md` / `tasks.md`.
