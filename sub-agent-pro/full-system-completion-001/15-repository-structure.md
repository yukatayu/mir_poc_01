# 15 — Repository Structure

## New docs

Add:

```text
specs/33-full-system-v1-scope.md
specs/34-textual-mir-alpha-grammar.md
specs/35-mir-typed-ir-and-interpreter.md
specs/36-projection-ir-and-boundary-preservation.md
specs/37-posegraph-runtime-semantics.md
specs/38-engine-provider-admission.md

plan/58-full-system-v1-roadmap.md
plan/59-textual-mir-roadmap.md
plan/60-computational-runtime-roadmap.md
plan/61-posegraph-runtime-roadmap.md
plan/62-projection-backend-roadmap.md
plan/63-engine-provider-roadmap.md
```

## New crates or modules

Prefer small additions:

```text
crates/mir-ast/src/textual_alpha.rs
crates/mir-semantics/src/typed_ir.rs
crates/mir-semantics/src/interpreter.rs
crates/mir-runtime/src/full_system_v1_session.rs
crates/mir-runtime/src/posegraph_runtime.rs
crates/mir-runtime/src/projection_ir.rs
crates/mir-runtime/src/provider_admission.rs
```

If crate split is cleaner later:

```text
crates/mir-ir
crates/mir-checker
crates/mir-compiler
crates/mirrorea-devtools
```

But do not large-refactor unless necessary.

## New sample roots

```text
samples/full-system-v1/
  README.md
  computational/
  world-core/
  membership-chat/
  sugoroku-world/
  avatar-pose/
  portal-worldlink/
  two-shard-hard-boundary/
  gradient-observation/
  projection/
  provider-adapter/
```

Keep existing roots:

- `samples/clean-near-end/`
- `samples/product-alpha1/demo/`
- `samples/product-alpha1/operational/`
- `samples/product-alpha1/computational/`
- `samples/product-alpha1/posegraph/`

## New scripts

```text
scripts/full_system_v1_samples.py
scripts/textual_mir_samples.py
scripts/projection_v1_samples.py
scripts/posegraph_runtime_samples.py
scripts/provider_admission_samples.py
scripts/full_system_v1_release_check.py
```

## Reports

Every package writes:

```text
docs/reports/<next>-<package-name>.md
```

## Snapshot docs

Update every closeout:

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- relevant `samples/*/README.md`
- relevant `scripts/README.md`
