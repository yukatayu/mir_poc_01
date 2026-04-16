# 406 — current L2 local finalization vs global snapshot comparison

## 目的

`atomic_cut` を global consistent cut や snapshot と short-hand で同一視しないため、

- local finalization
- global observation / snapshot

を scenario ベースで切り分ける。

## source-backed floor

- `atomic_cut` は local rollback frontier の確定である。
- global snapshot / consistent cut は current repo の settled vocabulary ではない。
- snapshot と durable commit は別 concern である。

## scenario comparison

### scenario A — local rollback frontier

```text
try {
  perform on doc_ref
  atomic_cut
  perform on audit_log
  perform on notifier
} fallback {
  perform on recovery_log
}
```

ここで `atomic_cut` が与えるのは、
fallback で戻る frontier の更新である。
global observer が consistent な room state を見たことまでは意味しない。

### scenario B — global observation

```text
room authority records turn_state
observer captures a consistent room snapshot
late joiner reads the snapshot
```

ここで欲しいのは、
複数 `place` / participant / message edge をまたぐ observation meaning である。
rollback frontier の確定そのものとは主題が違う。

## current judgment

- local finalization と global snapshot は別 family である。
- `atomic_cut` を local consistent cut と説明するときでも、
  consistent の中心が observation ではなく rollback frontier である点を省略してはならない。
- snapshot-only family を comparison candidate に残すのは自然だが、settled term にしてはならない。

## what is not decided here

- snapshot-only family の final naming
- snapshot family と durable-cut family の cross-place relation
- observation surface の final contract
