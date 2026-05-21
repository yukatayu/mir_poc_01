# Transform / PoseGraph 01

## purpose

この文書は、future Transform / PoseGraph line を読むための docs-first landing page です。

現時点では `samples/product-alpha1/posegraph/` や `scripts/posegraph_samples.py` は存在せず、runnable workflow として扱いません。

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

For the current docs/spec rebaseline, use repository validation only:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

These commands do not prove PoseGraph runtime behavior. They only prove that the planned boundary is documented and discoverable.

## future command shape

Future packages may add:

```bash
python3 scripts/posegraph_samples.py matrix --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/posegraph_samples.py run pose-02-no-split-frame-positive --format json
python3 scripts/posegraph_samples.py run pose-03-split-frame-negative --format json
```

Until those files exist and are validated, they are planned anchors only.

## future success criteria

- Head update remains a session-bound event and exports `pose_version`.
- Anchored object and target use the same `pose_snapshot_ref` in one observation frame.
- Fallback anchor carries explicit lineage, reason, freshness, and reacquire gate.
- Split-frame mismatch is rejected or exported as a machine-readable violation row.
- Save/load either restores a coherent anchor frontier or requires new witness / new epoch.

## stop lines

Do not read this guide as Unity / Unreal integration, VRM / VRChat compatibility, renderer-owned world semantics, continuous spatial sync, or active PoseGraph runtime completion.

