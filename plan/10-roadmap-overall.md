# plan/10 — 全体ロードマップ

## 目的

この文書は、Mir を主眼にした全体ロードマップを大きな段階ごとに整理する。
ここでいうロードマップは「何を先に固め、何をまだ急がないか」を明示するためのものであり、厳密な日程計画ではない。

## 現在までにかなり進んだ段階

### 1. 基礎境界の明文化

- Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform / shared space の境界整理
- L0 / L1 / L2 / L3 の decision level 導入
- invariants / constraints / open questions / decision register の整備

### 2. current L2 の representative example と companion docs

- representative programs
- current L2 surface syntax candidate
- fallback / preference chain の current reading
- predicate sublanguage
- option-local `admit`
- capability mismatch / non-admissible metadata

### 3. parser-free PoC 基盤

- AST fixture schema
- evaluation state schema
- step semantics
- oracle API
- minimal interpreter skeleton
- host harness
- host plan sidecar
- bundle loader
- batch runner
- selection helper
- selection profile helper
- named profile catalog

これは「PoC を一応回せる」ではなく、**current L2 reading を machine-check できる最小検証基盤が揃った**という意味で、すでに大きく前進している。

## 現在のフェーズ

現在の主フェーズは、単なる drift suppression だけではなく、
**Phase 1 / 2 / 3 / 4 / 5 の self-driven closeout / freeze を fixed entry criteria としたうえで、Phase 6 前半の compile-ready minimal PoC を維持しつつ、syntax-backed fixed-subset sample verification path を actualize する段階**
と読むのが自然である。

具体的には次が主線である。

1. `mir-ast` stage 1 / stage 2 non-production parser carrier を fixed entry criteria とする
2. `mir-semantics` / `mir-runtime` をまたぐ non-production checker/runtime first tranche を fixed entry criteria とする
3. compile-ready verification / formal hook first tranche を fixed entry criteria とする
4. compile-ready checkpoint close 後の next reopen sequencing を parser second tranche first に fixed し、first package 実装と reserve formal tool binding inventory fixed 後の follow-up package を narrow に整理する
5. fixed-subset source corpus / mapping / lowering / runner / verification ladder と proof-notebook review-unit pilot を、final grammar や concrete backend に逆流させず段階 actualize する

つまり、広い production 実装へ進む前に、
**意味論・verification boundary・PoC operational baseline・actual compile path の入口**を揃えるフェーズにある。

phase をもう少し細かく見たい場合は、`plan/17-research-phases-and-autonomy-gates.md` を参照する。

## 次のフェーズ

### 近い次フェーズ

- second widened authored-row actualization (`e21`)
- admit-family guard comparison (`e3`)
- theorem-side plain bridge sketch actualization
- theorem-side compare-ready bridge sketch second reopen

### その次のフェーズ

- proof / model-check handoff の first concrete tool cut
- richer host interface の actual widening を still later に残したまま、parser second tranche widen を比較する
- parser / checker / runtime public surface の second tranche inventory

### さらに先のフェーズ

- 型システムの強化
- 決定可能性 / complexity の整理
- theorem prover / external verifier 連携方針
- richer runtime / scheduler / distributed execution 境界

## workstream の優先順位

### 先にやるべきもの

- Mir current L2 semantics の drift 防止
- parser-free PoC の継続検証
- notation と docs mirror の安定化
- helper stack の public behavior / thin delegation boundary の維持

### まだ急がないもの

- production parser
- full production runtime
- LLVM-family backend / external codegen binding
- machine-readable catalog manifest
- richer host interface の一般化
- multi-request scheduler
- `Approximate` / `Compensate`
- PrismCascade の深い runtime 統合

## optional / side-track / upper-layer の位置づけ

### Mir

- 主眼
- current L2 と parser-free PoC の中心
- いま最も投資すべき層

### Mirrorea

- Mir の周辺で重要だが、current L2 PoC の直接対象ではない
- 命名、routing、overlay、fabric 側の設計が主

### Typed-Effect Wiring Platform

- Mir と連動する重要な workstream
- ただし current L2 PoC の直接 machine-check 範囲はまだ限定的

### PrismCascade

- optional / side-track に近い
- 比較対象かつ上位応用層であり、Mir semantics の主眼そのものではない
- performance-sensitive kernel を Mir runtime semantics に安易に混ぜない

### shared space / upper-layer / VRSNS / “裏返した図書館”

- 上位構想として重要
- ただし現在の主戦場はそこではなく、Mir core とその companion verification stack である
- ただし docs-first の practical example、membership / consistency boundary comparison、authority / resource ownership / delegated capability / activation rule / RNG provider placement の比較、layer responsibility の切り分けまでは self-driven に進めてよい
- active 化規則、authority / auth、consistency mode catalog、RNG / fairness model などの finalization は user 仕様確認が必要

## current L2 から将来層へ進むための gate

次の段階へ進むには、少なくとも次が必要である。

- current L2 fallback / `lease` / chain reading と invariant bridge の安定
- current companion notation の十分な説明可能性
- parser-free PoC で representative fixtures を継続的に回せること
- helper stack の責務境界が大きく崩れていないこと
- open question と settled judgment の混線が抑えられていること

## いまの判断

- current L2 は **まだ architecture and semantics のフェーズ**である。
- parser-free PoC はその意味論を machine-check する companion 基盤であり、Phase 2 closeout fixed 後も production 実装の着手宣言ではない。
- したがって、今の roadmap では **meaning / notation / verification boundary を fixed entry criteria として保ったまま、Phase 6 の minimal parser / checker / runtime / formal-hook first trancheを non-production に narrow actualize し、その後に source corpus / lowering / runner / verification ladder を fixed subset だけで積み上げる** のが妥当である。
