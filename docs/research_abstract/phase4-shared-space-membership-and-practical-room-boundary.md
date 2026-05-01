# Phase 4: shared-space membership と practical room boundary

## この phase の意味

Phase 4 は、thread / node / room のような共有空間を **同じ因果言語で書きつつ、運用境界は分ける** ための層です。
current repo では authoritative-room first の最小読みに重点があります。

## 2026-04-23 時点で固まっていること

この節は dated repository-memory anchor です。live status / macro phase / next queue authority は `../../progress.md` と `../../tasks.md` を参照してください。

- thread と node は同じ causal language で記述できる
- ただし lowering / evidence / transport / failure / durability / policy は別層
- `publication_order` / `witness_order` / `handoff` を room-scale sample で読める
- delegated RNG sample により provider boundary を current layer で確認できる

## まだ残ること

- exhaustive shared-space final catalog
- final public witness / provider / emitted-artifact contract
- distributed fairness / durability / transport failure の詳細

## この phase と current active sample の対応

current clean near-end order-handoff sample は、shared-space 全 catalog を与えるものではありません。
あくまで authoritative-room first の minimal working subset を示す current line です。

## 関連する summary / detail

- `clean_near_end_order_model_01.md`
- `clean_near_end_order_model_01_detail.md`
