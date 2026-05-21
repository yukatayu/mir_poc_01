# Mir Computational Core 01

## purpose

この文書は、`P-COMP-00` 後の Mir Computational Core line を読むための docs-first landing page です。

現時点では implementation guide ではありません。`samples/product-alpha1/computational/` や `scripts/mir_computational_samples.py` はまだ存在せず、runnable workflow として扱いません。

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

For the current docs/spec rebaseline, use repository validation only:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

These commands do not prove computational-core execution. They only prove that the rebaseline docs and source hierarchy are synchronized.

## future command shape

Future packages may add:

```bash
python3 scripts/mir_computational_samples.py matrix --format json
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json
```

Until those files exist and are validated, they are planned anchors only.

## completion gates

- `P-COMP-01`:
  spec, roadmap, sample matrix, planned/executable classification, and AddOne non-claim.
- `P-COMP-02`:
  pure AddOne owned by Mir, with host input / Mir compute / host output as distinct observable events.
- `P-COMP-03`:
  variables, arrays, records, and control-flow positive and negative rows.
- `P-COMP-04`:
  pure/effect split with effect / failure / capability rejection evidence.

## stop lines

Do not read this guide as final textual grammar, final public API, Rust-level completion, LLVM/backend implementation, or proof that the current typed external `AddOne` already proves Mir-owned computation.

