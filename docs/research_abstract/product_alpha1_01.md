# Product Alpha-1 Summary

Product/Public-ready Mirrorea Spaces alpha-1 is the first repo-local product workflow where an external developer can clone the repository and reproduce a small shared virtual-space prototype through an alpha-stable command family.

It is still alpha. It is not the final public product, final grammar, final ABI, WAN/federation, distributed durable save/load, arbitrary native package execution, or final viewer / telemetry service.

## What Is Decided

- The product front-door is versioned `package.mir.json`, not textual `.mir` final grammar.
- The developer-facing entrypoint is the Rust `mirrorea-alpha` CLI.
- The current first public-ish adoption candidate is the built `mirrorea-alpha` binary together with the generated native host launch bundle.
- The current hardening target is limited to versioned `package.mir.json`, the documented `mirrorea-alpha` command family, and the native host launch bundle replay surface.
- The current shipped surface is narrower than the full alpha CLI family: built-binary `check` / `build-native-bundle` / `demo`, bundle replay `run.sh check` / `run.sh view`, plus the bundled CLI, versioned package root, `manifest.json`, `launch.json`, `run.sh`, `README.md`, and observer-safe supporting artifacts.
- The demo package lives under `samples/product-alpha1/demo/`, separate from `samples/alpha/` and `samples/practical-alpha1/`.
- Auth, membership, capability, witness, transport, observability, and native policy remain separate lanes.
- Native output means a host launch bundle containing the compiled CLI, package files, devtools assets, reports, manifest, launch metadata, and provenance metadata.
- Native execution policy remains `Disabled`; signature metadata is provenance only.

## Workflow Shape

The release-candidate workflow is:

```text
package/source front-door
  -> checker
  -> runtime plan
  -> same-session runtime
  -> typed host-I/O
  -> hot-plug debug/auth/rate-limit/object/avatar-preview packages
  -> local/Docker transport
  -> devtools/viewer
  -> local save/load and bounded quiescent-save
  -> native host launch bundle
  -> reproducible demo / release check
```

`mirrorea-alpha demo` now runs this workflow and writes inspectable reports under its output directory. Full release-candidate readiness requires the Docker Compose TCP leg; `--skip-docker` is only a partial local probe. `scripts/product_alpha1_release_check.py check-all` runs the validation floor, focused tests, command family, native bundle run script checks, and JSON payload semantics for clean-clone validation. `scripts/product_alpha1_installed_binary_check.py check-all` is the current installed-binary adoption probe: it builds `target/debug/mirrorea-alpha`, runs the built binary directly, and replays bundle `run.sh` checks while exporting machine-readable `compatibility_scope` and `shipped_surface` boundaries without claiming final public CLI/API/ABI or final packaging.

## Evidence Boundaries

Standalone `mirrorea-alpha check` still reports package/schema acceptance and residual obligations. It does not by itself claim product alpha release readiness. Product alpha readiness is the combined release-candidate workflow evidence from `demo`, release check, focused tests, and docs.

The viewer remains a non-final static HTML/JSON viewer. It renders concrete observer-safe bundle records for the product demo panels and checks bounded forbidden raw witness/auth/capability keys, but it does not claim a complete redaction proof or final public viewer API.

The installed-binary probe does not freeze final textual `.mir` grammar, final Rust library ABI, or final viewer/devtools bundle ABI. It narrows only the current alpha-stable front door that later public hardening work should reference, and it keeps other bundled reports plus admin/debug session-store artifacts outside the current shipped-surface promise.

The Docker path is a controlled Docker Compose TCP fixture. It is not WAN or federation.

## Main Non-Claims

- final textual `.mir` grammar
- final public parser / checker / runtime / verifier ABI
- hosted product service
- production WAN/federation
- distributed durable save/load R3/R4
- arbitrary native package execution
- signature-is-safety
- full engine / avatar compatibility

## Entry Points

- hands-on guide: `../hands_on/product_alpha1_01.md`
- product sample: `../../samples/product-alpha1/demo/`
- release check: `../../scripts/product_alpha1_release_check.py`
- installed-binary probe: `../../scripts/product_alpha1_installed_binary_check.py`
- roadmap memory: `../../plan/50-product-alpha1-public-boundary-roadmap.md`
- normative boundary: `../../specs/25-product-alpha1-public-boundary.md`
