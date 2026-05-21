# Mir Computational Core 01

## purpose

この文書は、`P-COMP-04` で host read/write boundary まで進んだ Mir Computational Core line を読むための landing page です。

現時点でも broad runtime implementation guide ではありませんが、`samples/product-alpha1/computational/add-one-pure-mir/package.mir.json` と `scripts/mir_computational_samples.py` は one bounded direct runtime row、ten helper-executable first-floor rows、one direct accepted host read/write boundary row、and three expected `check` rejections を持ちます。

## current reading

Current alpha `AddOne` is typed external host-boundary evidence:

```text
host input -> typed adapter -> adapter-owned transform -> typed receipt
```

Future computational-core `AddOne` must be Mir-owned:

```text
host input -> typed adapter -> Mir function add_one -> typed adapter -> host output
```

The arithmetic `x + 1` must be represented, typed, executed, and later compiled as Mir computation. The adapter must only provide host input/output.

## current verification

Current executable verification commands:

```bash
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 scripts/mir_computational_samples.py matrix --format json
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json
python3 scripts/mir_computational_samples.py run comp-03-control-flow-positive --format json
python3 scripts/mir_computational_samples.py run comp-03-variables-scope-negative --format json
python3 scripts/mir_computational_samples.py run comp-04-host-io-internal-transform-positive --format json
python3 scripts/mir_computational_samples.py run comp-04-host-io-internal-transform-negative-undeclared-effect --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/add-one-pure-mir --format json
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/host-io-internal-transform/positive --format json
```

These commands now prove one bounded Mir-owned runtime row:

```text
ReadInt(41) -> add_one -> WriteInt(42)
```

They now prove `P-COMP-02`, `P-COMP-03`, and `P-COMP-04` first-floor/boundary evidence. `P-COMP-04` should be read narrowly: the accepted row proves that declared host input, Mir transform, and host output survive checker/runtime-plan/session evidence together, and the negative rows prove check-time rejection when effect / failure / capability declarations are missing. They do not prove final textual grammar, broad publish/observe/witness/handoff semantics, or backend realization.

Use repository validation alongside them:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

These repository validations prove that the docs/source hierarchy is synchronized around the executable row.

## completion gates

- `P-COMP-01`:
  spec, roadmap, sample matrix, planned/executable classification, rejected run, and AddOne non-claim.
- `P-COMP-02`:
  pure AddOne owned by Mir, with host input / Mir compute / host output as distinct observable events.
- `P-COMP-03`:
  variables, arrays, records, control-flow, and imports positive and negative rows.
- `P-COMP-04`:
  pure/effect split with accepted host read/write wrapping and effect / failure / capability rejection evidence.

## stop lines

Do not read this guide as final textual grammar, final public API, Rust-level completion, LLVM/backend implementation, or proof that the current typed external `AddOne` already proves Mir-owned computation.
