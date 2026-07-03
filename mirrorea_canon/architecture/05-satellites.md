---
id: arch/05-satellites
status: L1-fixed
maturity: draft
depends_on: [arch/01-strata]
summary: PrismCascade と Typed-Effect Wiring Platform の位置づけ。canon v1 の外周、狭い統合点。
open_items: [OPEN-031]
---

# 05 — 衛星系

**PrismCascade**(media kernel)は独立カーネルとして開発する(LAB D-006 継承)。最適化中心(effect-only graph 正規化、事前 planning、メモリ所有、CPU/GPU スケジューリング、offline/live 区別)が Mir と異なるため、runtime 統合はしない。統合点は狭く: Meta 層 effect provider(TTS・推論・asset)、remote 実行委譲、trace 連結用共有 ID、Prism graph 周辺の協調編集。Mir 側からは BND-007 の provider として現れる。

**Typed-Effect Wiring Platform** は「外部 effect を可視・型付き・契約対応・再配線可能にする」隣接層。Mir の言語意味論ではなく、adapter 境界(ADR-0011)の運用面の一般化である。canon v1 では概念位置のみ固定し、仕様化は S5 応用が要求した時点で開始する。

共有してよいもの: 識別子、必要最小の契約 schema、trace 連結戦略、互換な範囲の effect 語彙。**してはならないこと**: どちらかを Mir runtime に押し込むこと、Mirrorea が全 application logic を吸収すること。

OPEN-031: Prism との最小共有 trace schema と最小 remote 実行単位(統合が現実化する PHASE-I5 以降に決定)。
