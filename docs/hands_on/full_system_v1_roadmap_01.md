# Full System V1 roadmap hands-on

This is a reader-facing entrypoint for the `P-FS-00` roadmap rebaseline and the current implementation packages.

It explains what is now executable at the source-first parser/checker/bounded-effectful-runtime floor and what still remains later.

## Read first

- `../../progress.md`
- `../../tasks.md`
- `../../specs/33-full-system-v1-scope.md`
- `../../plan/58-full-system-v1-roadmap.md`

## Current claim

Full System V1 is now a roadmap boundary plus bounded source-first evidence and a bounded release-check workflow. It is not yet a final product workflow.

The current repo has:

- Product Alpha-1 bounded release-candidate workflow.
- canonical operational product suite.
- first-floor Mir computational evidence.
- helper-backed PoseGraph comparison evidence plus bounded source-first PoseGraph runtime evidence.
- bounded source-first projection IR + boundary-schema evidence, bounded same-binary local role-split evidence, plus product-alpha projection inventory.
- bounded source-first provider-admission plus renderer pose backend evidence, plus product-alpha engine/provider inventory.
- bounded source-first WorldCore / MembershipChat / Sugoroku / Portal / TwoShard / Gradient operational evidence.
- bounded Full V1 release-check/report/viewer bundle evidence.

The current promoted package is:

```text
P-FSV1-99 final audit
```

## What to verify now

Run the current documentation, alpha anchors, and current Full System V1 projection/runtime anchors:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_full_system_v1_release_check
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/full_system_v1_samples.py operational-matrix --format json
python3 scripts/full_system_v1_samples.py check-operational-all --format json
python3 scripts/full_system_v1_samples.py check-all --format json
python3 scripts/posegraph_runtime_samples.py check-all --format json
python3 scripts/projection_v1_samples.py check-all --format json
python3 scripts/provider_admission_samples.py check-all --format json
python3 scripts/renderer_pose_backend_samples.py check-all --format json
python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release
cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
cargo test -p mir-runtime --test projection_ir -- --nocapture
cargo test -p mir-runtime --test provider_admission -- --nocapture
cargo test -p mir-runtime --test renderer_pose_backend -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
cargo fmt --check
git diff --check
```

The Product Alpha and operational commands preserve the bounded alpha floor. The Full System V1 commands above now prove parser, typed checker, bounded effectful runtime, PoseGraph runtime/save-load/devtools, bounded projection IR plus packet/FFI boundary-schema preservation, bounded same-binary local role split, bounded provider admission, a bounded renderer pose backend demo, and a bounded line-level release-check workflow with per-command JSON reports plus static `bundle.json` / `index.html` viewer outputs. They still do not prove final packet/FFI transport semantics, a final server/client binary split, arbitrary native/WASM execution, a final provider ABI, or a final public devtools family.

`P-FSV1-01` and `P-FSV1-02` add bounded source-first WorldCore / MembershipChat / Sugoroku / Portal / TwoShard / Gradient operational roots. The two operational helper commands in the block above validate 12 executable rows with generated package-manifest expectations plus runtime report expectations.

## Current Full System V1 commands

Current Full System V1 parser/checker/runtime commands:

```bash
python3 scripts/textual_mir_samples.py check-all --format json
python3 scripts/full_system_v1_samples.py check-all --format json
cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture
cargo test -p mir-runtime --test full_system_v1_session -- --nocapture
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
python3 scripts/posegraph_runtime_samples.py check-all --format json
python3 scripts/projection_v1_samples.py check-all --format json
python3 scripts/provider_admission_samples.py check-all --format json
python3 scripts/renderer_pose_backend_samples.py check-all --format json
python3 scripts/full_system_v1_release_check.py --format json check-all --out /tmp/mirrorea-full-v1-release
cargo test -p mir-runtime --test projection_ir -- --nocapture
cargo test -p mir-runtime --test provider_admission -- --nocapture
cargo test -p mir-runtime --test renderer_pose_backend -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
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
