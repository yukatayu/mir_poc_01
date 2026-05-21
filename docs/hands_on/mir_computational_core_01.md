# Mir Computational Core 01

## purpose

この文書は、`P-COMP-01` で actualize された Mir Computational Core scaffold を読むための landing page です。

現時点では runtime implementation guide ではありません。`samples/product-alpha1/computational/` と `scripts/mir_computational_samples.py` は存在しますが、役割は planned-only matrix / root validation と rejected-run evidence です。

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

For the current scaffold actualization, use the dedicated planned-only commands first:

```bash
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 scripts/mir_computational_samples.py matrix --format json
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json
```

These commands prove that the computational root exists, rows are machine-readable, and attempted execution is rejected as `planned_only`.

Use repository validation alongside them:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

These commands still do not prove Mir-owned runtime execution. They only prove that the scaffold, docs, and source hierarchy are synchronized.

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
