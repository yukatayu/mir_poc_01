# Phase 3: parser boundary と first checker cut

## この phase の意味

Phase 3 は、parser-free current layer から **first checker cut** へ進むときの境界を整理する層です。
ここでは「何を helper-local current layer として残し、何を final public parser / checker API に昇格させるか」を分けます。

## 2026-04-23 時点の current reading

- active clean near-end suite は parser-free helper line として成立している
- first strong typing layer は finite decidable index fragment として成立している
- checker が読む constraint family は current sample line に接続している
- ただし final public parser grammar と final public checker API はまだ deferred

## この phase の重要な境界

- current helper command があること
- final public command surface が決まっていること

は別です。
現状の repo は前者まで進んでいますが、後者を完了とは言えません。

## まだ残ること

- parser subset freeze
- parser から checker / runtime への reconnect boundary
- public checker payload / artifact / verifier contract の確定

## 関連する summary

- `clean_near_end_typing_01.md`
- `clean_near_end_order_model_01.md`
- `clean_near_end_lean_01.md`
