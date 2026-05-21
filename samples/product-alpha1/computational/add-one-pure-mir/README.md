# comp-02-pure-add-one

Executable representative root for the first Mir-owned computation proof point.

Current runtime shape:

```text
host input -> Mir add_one -> host output
```

Current executable input is [`package.mir.json`](package.mir.json). The adjacent
`.mir` file remains explanatory only and is not a final front-door grammar.

Validation anchor:

```bash
cargo run -q -p mirrorea-cli -- run-local samples/product-alpha1/computational/add-one-pure-mir --format json
python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json
```
