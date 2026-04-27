# avatar_fairy_follow_representative_slice_01

legacy phase 8 sample-family label `avatar fairy follow / fallback anchor` の current runnable floor を最短で追うための page です。

current macro-phase reading は `Macro 6 reserve` です。

## run

```bash
python3 scripts/avatar_follow_samples.py list
python3 scripts/avatar_follow_samples.py check-all --format json
python3 scripts/avatar_follow_samples.py closeout --format json
```

focused runs:

```bash
python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json
python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug anchors --format json
python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json
python3 scripts/avatar_follow_samples.py run 04_invalid_cross_anchor_chain_rejected --format json
python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json
```

## what this proves

- remote head follow can be written with explicit fallback lineage
- visibility-loss-only fallback can stay local without claiming transport recovery
- leave-triggered stale anchor rejection can be observed without hidden repair
- invalid cross-anchor lineage stays an explicit rejection
- detached-anchor safety can be kept as a verification-oriented canary

## what this does not prove

- final public avatar runtime API
- final public visualization protocol
- real transport / auth / session semantics
- hot-plug / `AttachPoint` lifecycle implementation
- production engine / host integration
- `FAIRY-05` reacquire-after-return の runnable widening

## active vs planned vs historical

- active representative slice:
  `samples/clean-near-end/avatar-follow/`
- planned residual family:
  `samples/not_implemented/avatar-fairy-follow/`
- historical prototype anchor:
  `samples/prototype/current-l2-dynamic-attach-detach/`

`FAIRY-05` だけが residual planned family に残っています。混同しないことが重要です。

current repo-local reading では、`FAIRY-05` を active helper に昇格させる前に、
explicit state timeline / anchor switch evidence が要る、という gate だけを固定しています。

- `UNRESOLVED`: visibility-return witness をどの carrier に載せるか
- `UNRESOLVED`: helper-local CLI/debug surface の exact name
- current working assumption:
  candidate label として `state_timeline` / `anchor_switch` を使ってよい

これは planning-only です。final public visualization API や final public avatar runtime
surface として固定したものではありません。
