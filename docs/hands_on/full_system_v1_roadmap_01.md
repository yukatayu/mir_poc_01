# Full System V1 roadmap hands-on

This is a reader-facing entrypoint for the `P-FS-00` roadmap rebaseline and the first six implementation packages.

It explains what is now executable at the source-first parser/checker/bounded-effectful-runtime floor and what still remains later.

## Read first

- `../../progress.md`
- `../../tasks.md`
- `../../specs/33-full-system-v1-scope.md`
- `../../plan/58-full-system-v1-roadmap.md`

## Current claim

Full System V1 is now a roadmap boundary plus bounded first-floor source-first evidence. It is not yet a product workflow.

The current repo has:

- Product Alpha-1 bounded release-candidate workflow.
- canonical operational product suite.
- first-floor Mir computational evidence.
- helper-backed PoseGraph comparison evidence plus bounded source-first PoseGraph runtime evidence.
- projection/backend inventory.
- engine/provider inventory.

The current promoted package is:

```text
P-PROJ-02 projection IR realization
```

## What to verify now

Run the current documentation and alpha anchor checks:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/operational_product_samples.py check-all --format json
cargo fmt --check
git diff --check
```

The Product Alpha and operational commands preserve the current runnable floor. They do not prove projection IR, server/client split, or provider admission.

## Planned Full System V1 commands

Current Full System V1 parser/checker/runtime commands:

```bash
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/full_system_v1_samples.py check-all --format json
cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
python3 scripts/posegraph_runtime_samples.py check-all --format json
```

These are still future commands and must not be treated as current validation:

```bash
python3 scripts/projection_v1_samples.py check-all --format json
python3 scripts/provider_admission_samples.py check-all --format json
python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release
```

## Stop lines

Do not claim:

- final public grammar completion.
- final ABI / SDK completion.
- Rust-level language completion.
- LLVM/native codegen completion.
- server/client split compiler completion.
- Unity / Unreal / WASM / native provider execution completion.
- production WAN/federation.
- distributed durable save/load R3/R4.
- arbitrary native package execution.
- arbitrary WASM execution.
