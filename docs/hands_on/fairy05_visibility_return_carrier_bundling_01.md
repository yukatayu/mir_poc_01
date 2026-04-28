# FAIRY-05 visibility-return carrier bundling 01

`R3` の current hands-on では、active representative slice と helper closeout を読みながら、
`FAIRY-05` を active 化せずに carrier-choice matrix だけを narrow にします。

## 実行コマンド

```bash
python3 scripts/avatar_follow_samples.py check-all --format json
python3 scripts/avatar_follow_samples.py closeout --format json
python3 scripts/avatar_follow_samples.py run 01_follow_remote_head_with_local_fallback --debug anchors --format json
python3 scripts/avatar_follow_samples.py run 02_remote_head_not_visible_falls_back_to_local --debug anchors --format json
python3 scripts/avatar_follow_samples.py run 03_remote_avatar_leaves_falls_back_to_local --debug membership --format json
python3 scripts/avatar_follow_samples.py run 06_model_check_no_detached_anchor_observed --debug verification --format json
```

## まず見る場所

- helper closeout の `fairy05_reopen_gate`
- `samples/not_implemented/avatar-fairy-follow/05_follow_target_reacquired_after_return.mir`
- active helper runs の `anchors` / `membership` / `verification`

## current reading

- `FAIRY-05` は still planned
- helper closeout `carrier_choice = UNRESOLVED` は implementation inventory
- docs-first provisional recommendation は
  typed bundle over `state_timeline` + `anchor_switch`
- visibility-return witness は timeline witness refs の内側に置く reading を current line に置く

## stop line

- `FAIRY-05` を active runnable widening と呼ばない
- `state_timeline` / `anchor_switch` を current debug mode と呼ばない
- final public avatar / visualization API を claim しない

## 関連

- `../research_abstract/fairy05_visibility_return_carrier_bundling_01.md`
- `../../plan/31-fairy05-visibility-return-carrier-bundling.md`
- `avatar_fairy_follow_representative_slice_01.md`
