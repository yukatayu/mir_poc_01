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

- product same-session runtime command behavior beyond `check`
- same-session product demo command path
- local and Docker transport product command path
- message failure/recovery checker/runtime report
- bounded R2 quiescent save implementation
- product viewer UX over product demo bundle
- native launch bundle
- clean-clone release-candidate validation

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

Target:

- extend `samples/product-alpha1/demo/`
- connect check -> runtime plan -> run-local -> typed host-I/O -> hot-plug -> local/Docker transport -> observe in one session carrier or documented product run directory
- preserve exact carrier split from practical and operational lines

Validation:

- product demo workflow command passes
- same-session state transition evidence exists
- negative membership / capability / witness behavior remains visible
- negative auth behavior and explicit contract-update behavior remain visible
- `transport --mode local` and `transport --mode docker` product paths are implemented or explicitly reported as release blockers

Non-claim:

- no distributed durable save/load
- no production WAN / federation
- no accepted detach execution unless explicitly implemented

### `P-A1-28` — message failure/recovery and quiescent save

Target:

- introduce `MessageState`, `TransportContract`, `RecoveryPolicy`
- implement bounded R2 `quiescent-save` for controlled local/Docker scope
- report `NoInFlight`, `AllPlacesSealed`, `NoPostCutSend`
- add positive and negative quiescent-save rows

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

### `P-A1-29` — product devtools viewer UX

Target:

- product-level static HTML/local viewer
- product overview, place graph, event DAG, route graph, membership timeline, witness timeline, hot-plug lifecycle, save/quiescent-save, message recovery, fallback, auth/capability, redaction, retention panels
- observer-safe leak tests

Validation:

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
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/demo --format json
cargo run -q -p mirrorea-cli -- demo --out /tmp/mirrorea-alpha1-demo --format json
cargo run -q -p mirrorea-cli -- session /tmp/mirrorea-alpha1-demo/session --format json
cargo run -q -p mirrorea-cli -- attach /tmp/mirrorea-alpha1-demo/session samples/product-alpha1/demo/packages/debug-layer --format json
cargo run -q -p mirrorea-cli -- transport samples/product-alpha1/demo --mode local --format json
cargo run -q -p mirrorea-cli -- transport samples/product-alpha1/demo --mode docker --format json
cargo run -q -p mirrorea-cli -- save /tmp/mirrorea-alpha1-demo/session --format json
cargo run -q -p mirrorea-cli -- load /tmp/mirrorea-alpha1-demo/session/savepoints/latest --format json
cargo run -q -p mirrorea-cli -- export-devtools /tmp/mirrorea-alpha1-demo/session --out /tmp/mirrorea-alpha1-devtools --format json
cargo run -q -p mirrorea-cli -- view /tmp/mirrorea-alpha1-devtools --check --format json
cargo run -q -p mirrorea-cli -- quiescent-save /tmp/mirrorea-alpha1-demo/session --format json
cargo run -q -p mirrorea-cli -- build-native-bundle samples/product-alpha1/demo --out /tmp/mirrorea-alpha1-bundle --format json
python3 scripts/product_alpha1_release_check.py check-all --format json
```

Actual command names may differ only if docs and validation scripts are updated together.

## blocker split

### self-driven implementation packages

- product demo same-session runtime
- message recovery and quiescent-save bounded implementation
- product viewer UX
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

- `P-A1-27` product demo same-session runtime

Queue authority remains `progress.md` / `tasks.md`.
