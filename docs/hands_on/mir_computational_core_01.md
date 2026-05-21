# Mir Computational Core 01

## purpose

この文書は、`P-COMP-02` で最初の executable row まで進んだ Mir Computational Core line を読むための landing page です。

現時点でも broad runtime implementation guide ではありませんが、`samples/product-alpha1/computational/add-one-pure-mir/package.mir.json` と `scripts/mir_computational_samples.py` は one bounded executable row を持ちます。残りの rows は still planned-only です。

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
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/add-one-pure-mir --format json
```

These commands now prove one bounded Mir-owned runtime row:

```text
ReadInt(41) -> add_one -> WriteInt(42)
```

They do not prove `P-COMP-03` / `P-COMP-04`, final textual grammar, or broad first-floor completion.

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
  variables, arrays, records, and control-flow positive and negative rows.
- `P-COMP-04`:
  pure/effect split with effect / failure / capability rejection evidence.

## stop lines

Do not read this guide as final textual grammar, final public API, Rust-level completion, LLVM/backend implementation, or proof that the current typed external `AddOne` already proves Mir-owned computation.
