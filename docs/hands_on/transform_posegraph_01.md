# Transform / PoseGraph 01

## purpose

この文書は、`P-POSE-01` で actualize された Transform / PoseGraph scaffold を読むための landing page です。

現時点では runtime implementation guide ではありません。`samples/product-alpha1/posegraph/` と `scripts/posegraph_samples.py` は存在しますが、役割は planned-only matrix / root validation と rejected-run evidence です。

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

For the current scaffold actualization, use the dedicated planned-only commands first:

```bash
python3 -m unittest scripts.tests.test_posegraph_samples
python3 scripts/posegraph_samples.py matrix --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/posegraph_samples.py run pose-04-no-split-frame-positive --format json
python3 scripts/posegraph_samples.py run pose-05-split-frame-negative --format json
```

These commands prove that the PoseGraph root exists, rows are machine-readable, and attempted execution is rejected as `planned_only`.

Use repository validation alongside them:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

These commands still do not prove PoseGraph runtime behavior. They only prove that the scaffold, docs, and source hierarchy are synchronized.

## future success criteria

- Head update remains a session-bound event and exports `pose_version`.
- Anchored object and target use the same `pose_snapshot_ref` in one observation frame.
- Fallback anchor carries explicit lineage, reason, freshness, and reacquire gate.
- Split-frame mismatch is rejected or exported as a machine-readable violation row.
- Save/load either restores a coherent anchor frontier or requires new witness / new epoch.

## stop lines

Do not read this guide as Unity / Unreal integration, VRM / VRChat compatibility, renderer-owned world semantics, continuous spatial sync, or active PoseGraph runtime completion.
