# Minimal Alpha-1 Patterns 01

この guide は、alpha-1 の current runnable floor を **最小で実用的な pattern** として読む入口です。

目的は、product demo 全体を長く辿る前に、次を短い command set で確認できるようにすることです。

- product alpha release-candidate workflow は実行可能だが final public product ではない。
- operational suite の Sugoroku workflow は実行可能だが final catalog breadth ではない。
- Mir-owned computation は bounded row として実行可能だが broad effect semantics ではない。
- host I/O は Mir core builtin ではなく typed external boundary である。
- PoseGraph no-split-frame は same-client same-observation-snapshot helper evidence であり global simultaneity ではない。
- projection / engine-adapter は machine-readable inventory だが codegen / provider admission ではない。

## Pattern Matrix

まず matrix を見ます。

```bash
python3 scripts/minimal_alpha1_patterns.py list --format json
python3 scripts/minimal_alpha1_patterns.py matrix --format json
```

`matrix` は `P-PAT-01` の pattern list を返します。default strict check は、次の 4 family です。

- `computational`
- `posegraph`
- `projection`
- `engine_adapter`

Product release-candidate workflow と operational Sugoroku workflow は workflow anchor として matrix に出ますが、default strict check では heavy workflow を毎回走らせません。heavy workflow まで含める場合は後述の `--include-workflows` を使います。

## Strict Default Check

通常の最小検証は次です。

```bash
python3 -m unittest scripts.tests.test_minimal_alpha1_patterns
python3 scripts/minimal_alpha1_patterns.py check-all --format json
```

この check は、単に helper が exit 0 で終わることだけを見ません。現在の期待値として次を固定します。

- computational: 15 rows、7 accepted、5 expected runtime rejections、3 expected check rejections、planned 0
- PoseGraph: 9 rows、1 accepted、1 `violation_export`、7 planned
- projection: 4 planned rows、accepted compatibility row 1、rejected compatibility row 1
- engine-adapter: 8 provider rows、`NativeExecutionPolicy = Disabled`、`WasmExecutionPolicy = InventoryOnly`、semantic owner `mir_mirrorea`

行数や expected rejection ID が drift すると `status = rejected` になります。

## Run Minimal Patterns

Positive executable rows:

```bash
python3 scripts/minimal_alpha1_patterns.py run mir-compute-add-one --format json
python3 scripts/minimal_alpha1_patterns.py run mir-compute-host-io-transform --format json
python3 scripts/minimal_alpha1_patterns.py run posegraph-no-split-frame --format json
```

Negative / boundary rows:

```bash
python3 scripts/minimal_alpha1_patterns.py run mir-compute-missing-effect-reject --format json
python3 scripts/minimal_alpha1_patterns.py run posegraph-split-frame-violation --format json
python3 scripts/minimal_alpha1_patterns.py run projection-inventory-boundary --format json
python3 scripts/minimal_alpha1_patterns.py run engine-adapter-wasm-inventory --format json
```

Expected readings:

- `mir-compute-add-one` is accepted as bounded `ReadInt -> add_one -> WriteInt`.
- `mir-compute-host-io-transform` is accepted only when the host read/write boundary is declared.
- `mir-compute-missing-effect-reject` is a `check_rejection`, not a runtime fallback.
- `posegraph-no-split-frame` is accepted only for same snapshot / same pose version.
- `posegraph-split-frame-violation` exports `violation_export`.
- `projection-inventory-boundary` rejects attempted execution as `planned_only`.
- `engine-adapter-wasm-inventory` rejects attempted execution as `planned_only`.

## Include Workflow Anchors

To include the heavier product and operational workflow anchors:

```bash
out_dir=$(mktemp -d /tmp/mirrorea-minimal-alpha1-patterns-XXXXXX)
python3 scripts/minimal_alpha1_patterns.py check-all --include-workflows --out "$out_dir" --format json
```

If Docker is unavailable, the product release command can be run as a partial local probe:

```bash
out_dir=$(mktemp -d /tmp/mirrorea-minimal-alpha1-patterns-local-XXXXXX)
python3 scripts/minimal_alpha1_patterns.py check-all --include-workflows --skip-docker --out "$out_dir" --format json
```

`--skip-docker` must not be recorded as release-candidate evidence. It is a local partial probe.

## Theory Anchors

- Mir core has no standard I/O builtin. Host input/output stay typed external adapter boundaries.
- Pure computation and effectful boundary calls are separate. Undeclared effect / failure / capability rows reject.
- Place is execution locus, not participant identity.
- PoseGraph no-split-frame is same-client same-observation-snapshot evidence, not global simultaneity.
- Projection target / packet / FFI inventory is not server/client code generation.
- Engine / WASM / native provider inventory is not provider admission.

## Stop Lines

Do not read this guide or `scripts/minimal_alpha1_patterns.py` as final textual grammar, final public API/SDK, final product readiness, direct LLVM/native backend, final server/client split, arbitrary native/WASM execution, WAN/federation, distributed durable save/load, or full PoseGraph runtime completion.
