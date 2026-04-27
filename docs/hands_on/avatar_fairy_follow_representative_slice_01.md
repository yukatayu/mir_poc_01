# avatar_fairy_follow_representative_slice_01

phase 8 `avatar fairy follow / fallback anchor` の current runnable floor を最短で追うための page です。

## run

```bash
python3 scripts/avatar_follow_samples.py list
python3 scripts/avatar_follow_samples.py check-all --format json
python3 scripts/avatar_follow_samples.py closeout --format json
```

focused runs:

```bash
python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json
python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json
python3 scripts/avatar_follow_samples.py run 04_invalid_cross_anchor_chain_rejected --format json
python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json
```

## what this proves

- remote head follow can be written with explicit fallback lineage
- leave-triggered stale anchor rejection can be observed without hidden repair
- invalid cross-anchor lineage stays an explicit rejection
- detached-anchor safety can be kept as a verification-oriented canary

## what this does not prove

- final public avatar runtime API
- final public visualization protocol
- real transport / auth / session semantics
- hot-plug / `AttachPoint` lifecycle implementation
- production engine / host integration

## active vs planned vs historical

- active representative slice:
  `samples/clean-near-end/avatar-follow/`
- planned residual family:
  `samples/not_implemented/avatar-fairy-follow/`
- historical prototype anchor:
  `samples/prototype/current-l2-dynamic-attach-detach/`

混同しないことが重要です。
