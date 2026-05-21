# Autonomous Execution 01

## purpose

This guide explains how to read the post-`P-COMP-00` autonomous execution line.

It is not a final public-product promise. It is the execution contract for continuing through computational core, PoseGraph, projection inventory, and engine-adapter boundary without stopping for user questions.

## current default

After the user asks for execution, the agent should continue package-by-package:

```text
P-COMP-01
P-POSE-01
P-PROJ-01
P-ENG-01
front-half closeout
P-COMP-02
P-COMP-03
P-COMP-04
P-POSE-02
closeout audit
```

If a final-product choice is encountered, the agent should keep the current alpha-local boundary, mark the wider choice as `user-spec-required`, and continue lower-layer work.

`P-COMP-01` is scaffold actualization, not runtime completion. It should create planned-only computational roots, helper matrix, and tests. The first runtime implementation proof point is `P-COMP-02`, where a narrow `mir-semantics` computational module owns pure `add_one` while host input/output stay typed external adapters.

## package close command floor

Every package should at least run:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Implementation packages must also run their focused helper/test commands.

## non-claims

This autonomous line does not claim final textual grammar, final ABI/SDK, final distribution, direct LLVM/backend completion, server/client binary generation, bounded native/WASM provider admission, arbitrary native/WASM execution, Unity/VRM compatibility, WAN/federation, or R3/R4 durable distributed save/load.
