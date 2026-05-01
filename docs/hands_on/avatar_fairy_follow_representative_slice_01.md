# avatar_fairy_follow_representative_slice_01

legacy phase 8 sample-family label `avatar fairy follow / fallback anchor` の current runnable floor を最短で追うための page です。

family-local historical lane label は `Macro 6 reserve` です。live repo 全体の macro phase / queue authority は `progress.md` と `tasks.md` を参照してください。

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
positive reacquire-after-return sample、negative missing-return-witness または stale-membership companion、
explicit `state_timeline` / `anchor_switch` evidence、docs/report/snapshot sync が要る、という gate を固定しています。
helper closeout は `planned_sample_paths` と `fairy05_reopen_gate` を返し、この gate を
planning-only inventory として visible にします。

- `UNRESOLVED`: visibility-return witness をどの carrier に載せるか
- `UNRESOLVED`: helper-local CLI/debug surface の exact name
- current working assumption:
  candidate label として `state_timeline` / `anchor_switch` を使ってよい

`state_timeline` / `anchor_switch` は planning-only candidate label であり、current debug mode ではありません。
これは planning-only です。final public visualization API や final public avatar runtime
surface として固定したものではありません。

## current `R3` narrow line

2026-04-28 の `R3` docs-first closeout では、
helper closeout implementation inventory は `carrier_choice = UNRESOLVED` のまま残しつつ、
repository memory 側の provisional recommendation を次へ narrow にしました。

- timeline event だけでは switch frontier が弱い
- anchor-switch event だけでは ordering / witness lineage が弱い
- witness event だけでは helper-local evidence として抽象度が高い
- current provisional recommendation は
  typed bundle over `state_timeline` + `anchor_switch`

visibility-return witness は standalone top-level carrier ではなく、
timeline witness refs の内側に置く reading を current line に置きます。
