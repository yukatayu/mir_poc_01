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

現在の主フェーズは次である。

1. current L2 semantics の drift 抑制
2. companion notation の整理
3. parser-free PoC helper stack の責務境界調整
4. long-term repository memory の外在化

つまり、広い新機能追加ではなく、**意味論・notation・検証基盤・文書構造を揃えるフェーズ**にある。

## 次のフェーズ

### 近い次フェーズ

- current L2 notation のさらなる polishing
- representative fixtures の追加と drift regression の充実
- parser をまだ書かずに進められる helper / fixture / explanation boundary の整理
- parser 導入前に何を最低限固定するかの棚卸し

### その次のフェーズ

- parser 境界の導入準備
- richer host interface の最小方針整理
- trace / audit serialization の独立化検討
- static analysis workstream の入口設計

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

- current L2 fallback / `lease` / chain reading の安定
- current companion notation の十分な説明可能性
- parser-free PoC で representative fixtures を継続的に回せること
- helper stack の責務境界が大きく崩れていないこと
- open question と settled judgment の混線が抑えられていること

## いまの判断

- current L2 は **まだ architecture and semantics のフェーズ**である。
- parser-free PoC はその意味論を machine-check する companion 基盤であって、production 実装の着手宣言ではない。
- したがって、今の roadmap では **先に meaning / notation / verification boundary を揃え、その後に parser / richer runtime / static analysis へ進む** のが妥当である。
