# plan/14 — 用語集と境界ルール

## 目的

この文書は、current repo で混同しやすい用語を短く定義し、formal token と日本語 prose の使い分けを固定する。

## formal token と日本語 prose の使い分け

- formal token:
  - `perform`
  - `try`
  - `fallback`
  - `Reject`
  - `atomic_cut`
  - `admit`
  - `lease`
  - `ProfileCatalog`
  - `bundle`
  - `profile`
- 日本語 prose:
  - 概念の説明
  - 比較理由
  - 判断と未決の整理
  - risk / roadmap / history

日本語 prose では、formal token をそのまま残してよい。
ただし token が current L2 で特別な意味を持つ場合は、その意味を prose で明示する。

## 主要用語

### Mir

- この repo の主眼である意味論コア言語
- current L2 / parser-free PoC の中心対象

### Mirrorea

- 分散 fabric / control plane 側の層
- Mir と関連するが、current PoC の主眼そのものではない

### Place

- participant / principal そのものではない
- state / queue / capability / visibility / observation frontier を持つ execution locus
- system-wide source から place-specific program へ projection するときの基本単位

### TermSignature

- term / transition / effect / message / adapter をまたぐ signature inventory
- current repo では helper-local / report-local evidence carrier
- final public signature schema を意味しない

### LayerSignature

- auth / verification / visualization / transport / telemetry などの layer を
  `requires / provides / transforms / checks / emits / laws` で読む current carrier
- current repo では first cut の evidence-oriented naming に留める

### VerificationLayer

- finite-index checker、theorem bridge、model-check second line、runtime policy preview、
  visualization / telemetry lane を typed layer composition として読む current explanation 語彙
- hidden verifier builtin や final public verifier contract を意味しない

### MessageEnvelope

- transport insertion seam 上に見える message carrier
- authentication / authorization / membership / capability / witness を collapse しない current split を支える

### AuthEvidence

- authentication に関する evidence carrier
- current helper-local baseline は `auth none` だが、final public kind / session / signature protocol は未決

### VisualizationProtocol

- static view / runtime view / label / authority / redaction / telemetry を含む可視化 carrier の current explanation 語彙
- helper-local preview と final public viewer contract を分けて読む

### effect-based OS-like substrate

- Mirrorea / adapter / visualization / telemetry の内側を説明するための current interpretation
- standard I/O を Mir core primitive に戻したり、subsystem boundary を collapse したりする根拠にはしない

### PrismCascade

- media graph kernel / optional project / side-track に近い比較対象
- Mir runtime semantics に安易に折り畳まない

### fallback

- current L2 では **guarded option chain**
- outer-longer-lifetime wrapper ではない

### preference chain

- fallback candidate の canonical chain
- same-lineage の left-to-right monotone degradation を持つ

### lease

- option-local lifetime guard
- `lease-expired` は option-local miss metadata として現れる

### admit

- option-local admissibility に関わる current L2 token
- non-admissible metadata と narrative explanation を伴いうる

### Reject

- request-level rejection
- current L2 では later candidate でも成立しない場合の終端 outcome になりうる

### atomic_cut

- local execution / rollback 境界に関わる token
- degradation order 自体は巻き戻さない

### durable_cut

- 別系統の cut family の論点
- current L2 fallback / parser-free PoC task では直接拡張しない

### barrier

- 現段階では current L2 parser-free PoC の対象外
- 無断で広げない

### bundle

- fixture 本体、expected verdicts、必要なら host plan sidecar を束ねた current helper 単位

### batch

- bundle 群を directory 単位で discovery / execute / summarize する helper 層

### selection helper

- bundle 群を `runtime-only` / `static-only` / `single-fixture` で選別する helper 層

### selection profile helper

- primitive selection mode を組み合わせた request / profile を扱う helper 層

### named profile catalog

- small hard-coded alias layer
- `smoke-runtime` などの human-friendly preset 名を既存 request に解決する

### manifest

- machine-readable catalog / package / preset asset の総称として比較対象に出るが、current では未採用

## current L2 で特別注意する語

### outer / inner

- nested syntax の視覚を指すことはある
- しかし current L2 chain semantics の正本ではない
- outer / inner 直感は drift を起こしやすい

### monotone degradation

- same-lineage chain が later option へ一方向に degrade すること
- earlier option への再昇格は含まない

### no re-promotion

- current L2 settled judgment
- rollback / `atomic_cut` があっても earlier option へ戻らない

### checker floor

- local / structural / decidable floor として current checker / helper が先に discharge する範囲
- same-lineage floor、missing option structure、capability strengthening prohibition、`try` / `atomic_cut` structural floor を含む

### residual proof obligation

- checker floor の後にも global law 名として external boundary に残りうる obligation
- 代表例は `canonical_normalization_law / no_re_promotion` と `rollback_cut_non_interference / hidden_rollback_absence`

### explicit edge-row family

- current L2 companion notation の settled family boundary
- A2 hanging lineage continuation は polished first choice、A1 inline row は companion-equivalent shorthand
- `lineage(...)` の最終 token / punctuation は未決定

### machine-check

- helper / fixture / tests が exact compare する範囲
- `must_explain` はここに入れない

### human-facing explanation

- prose / report / docs が担う説明責務
- `must_explain` や長文 narrative はここに残す

## 混同しやすい対

| 語 | 混同相手 | current repo での区別 |
|---|---|---|
| fallback | outer wrapper | current L2 では wrapper ではなく guarded option chain |
| `lease` | request-global lifetime | option-local guard |
| `Reject` | hidden skip / soft miss | request-level explicit outcome |
| `atomic_cut` | degradation reset | rollback frontier には関わるが degradation order は戻さない |
| bundle | manifest | bundle は current helper 単位、manifest は future option |
| profile | named alias | profile は request carrier、named alias はその上の薄い catalog |
| docs mirror | code source of truth | docs は説明、code は implementation source、tests は public behavior coverage |
| helper-local preview | final public API | preview は repo-local evidence floor、public API は final mixed gate |
| generated artifact | source sample | generated artifact は bridge evidence または reserve、source sample ではない |

## boundary rule の短いまとめ

- semantics を決めるのは `specs/`
- current status / roadmap / boundary を整理するのが `plan/`
- history と change rationale は `docs/reports/`
- helper 実装の source of truth は `crates/mir-semantics/src/harness.rs` などの code
- public behavior coverage は tests に置く
