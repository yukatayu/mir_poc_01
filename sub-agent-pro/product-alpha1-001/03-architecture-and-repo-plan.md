# Architecture and repository plan

## Desired architecture

```text
samples/product-alpha1/demo/
  package.mir.json
  packages/
    debug-layer/
    auth-layer/
    rate-limit-layer/
    placeholder-object/
    custom-avatar-preview/
  expected/
  README.md

crates/
  mirrorea-cli/                 # new or equivalent binary crate
  mirrorea-product-alpha1/      # optional library crate if cleaner
  mir-runtime/                  # can keep implementation modules if migration too heavy
  mir-ast/                      # package loader / checker / IR
  mirrorea-core/                # carriers

scripts/
  product_alpha1_release_check.py
  product_alpha1_clean_clone_check.py
  product_alpha1_viewer_check.py

docs/hands_on/
  product_alpha1_01.md

docs/research_abstract/
  product_alpha1_01.md

specs/
  25-product-alpha1-public-boundary.md
  26-message-failure-and-recovery-typing.md
  27-quiescent-savepoint-semantics.md
  28-product-alpha1-package-and-native-policy.md
  29-product-alpha1-devtools-viewer.md

plan/
  50-product-alpha1-public-boundary-roadmap.md
  51-product-alpha1-release-validation-roadmap.md
```

Codex may choose smaller names, but the distinction must remain:

- `samples/practical-alpha1/` = first-floor fixture root;
- `samples/alpha/` = alpha-0 evidence root;
- product α-1 demo root = externally reproducible product demo.

## CLI architecture

Suggested command surfaces:

```bash
mirrorea-alpha check <package>
mirrorea-alpha run-local <package>
mirrorea-alpha attach <session-or-run-dir> <package>
mirrorea-alpha host-io <session-or-run-dir> --input ...
mirrorea-alpha save <session-or-run-dir>
mirrorea-alpha load <savepoint>
mirrorea-alpha quiescent-save <session-or-run-dir>
mirrorea-alpha export-devtools <session-or-run-dir> --out <dir>
mirrorea-alpha view <devtools-dir>
mirrorea-alpha transport <package> --mode docker
mirrorea-alpha build-native-bundle <package> --out <dir>
mirrorea-alpha demo --out <dir>
```

If persistent runtime daemon is too large, use a run directory as a session carrier:

```text
target/mirrorea-alpha1-sessions/<session-id>/
  session.json
  runtime-state.json
  event-dag.json
  membership.json
  devtools/
  savepoints/
  reports/
```

This is acceptable for product α-1 if documented.

## Product demo root

Do not promote existing roots silently. Add a separate root:

```text
samples/product-alpha1/demo/
```

or:

```text
samples/practical-alpha1/product-demo/
```

Preferred: `samples/product-alpha1/demo/` because it separates product-ready demo from first-floor fixtures.

## Native launch bundle

Output shape:

```text
target/mirrorea-alpha1-bundles/<demo>/
  bin/
    mirrorea-alpha
  packages/
    package.mir.json
    ...
  devtools/
    viewer.html
    bundle.json
  reports/
    check.json
    run.json
    save.json
    quiescent-save.json
  run.sh
  README.md
  manifest.json
```

This is not direct Mir-to-machine-code codegen. It is a native host launch bundle.

## Tests

Add focused tests for:

- CLI command parsing;
- product demo check;
- product demo run;
- product demo attach;
- product demo devtools export;
- quiescent save;
- native bundle contents;
- clean-clone-ish release check script.

## Avoid

- making huge unrelated crate reshuffles;
- rewriting all previous alpha roots;
- pretending first-floor root is product root;
- changing final grammar prematurely.
