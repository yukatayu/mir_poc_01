# Transform / PoseGraph 01

## purpose

この文書は、`P-POSE-02` で actualize された Transform / PoseGraph bounded helper evidence を読むための landing page です。

現時点では full runtime implementation guide ではありません。`samples/product-alpha1/posegraph/` と `scripts/posegraph_samples.py` は、accepted same-snapshot row、negative `violation_export` row、残り planned rows の machine-readable split を与える helper line です。

## current reading

PoseGraph の目標は、avatar head transform、object anchor、pose version、fallback、observation、save/load を renderer-owned hidden state ではなく Mir / Mirrorea-owned semantic state として扱うことです。

No-split-frame は次を意味します。

```text
same client session
same observation snapshot
target pose version == anchored object pose version
```

これは global simultaneous coordinates、continuous sync、WAN federation ではありません。

## current verification

For the current bounded helper evidence, use the dedicated commands first:

```bash
python3 -m unittest scripts.tests.test_posegraph_samples
python3 scripts/posegraph_samples.py matrix --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/posegraph_samples.py run pose-04-no-split-frame-positive --format json
python3 scripts/posegraph_samples.py run pose-05-split-frame-negative --format json
```

These commands prove that the PoseGraph root exists, rows are machine-readable, `pose-04` is accepted only when target and anchored object share one observation snapshot plus one `pose_version`, and `pose-05` is exported as a machine-readable `no_split_frame` violation instead of being mistaken for stable state.

Use repository validation alongside them:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

These commands still do not prove full PoseGraph runtime behavior. They only prove that the bounded helper line, docs, and source hierarchy are synchronized.

## future success criteria

- Head update remains a session-bound event and exports `pose_version`.
- Anchored object and target use the same `pose_snapshot_ref` in one observation frame.
- Fallback anchor carries explicit lineage, reason, freshness, and reacquire gate.
- Split-frame mismatch is rejected or exported as a machine-readable violation row.
- Save/load either restores a coherent anchor frontier or requires new witness / new epoch.

## stop lines

Do not read this guide as Unity / Unreal integration, VRM / VRChat compatibility, renderer-owned world semantics, continuous spatial sync, pose-aware save/load completion, devtools panel completion, or active full PoseGraph runtime completion.
