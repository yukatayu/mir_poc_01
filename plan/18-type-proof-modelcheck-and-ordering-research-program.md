# plan/18 — 型 / 定理証明 / モデル検査 / ordering 研究計画

## 目的

この文書は、次の 4 系統を 1 箇所で detail-side に管理する。

- typed-core / checker boundary
- theorem-side pilot
- model-check / protocol-verification reserve line
- async-control / ordering / `memory_order` reinterpretation

ここでの目的は、**current executable mainline を壊さずに研究を前へ進めること** である。
full strong type system や production tool binding を immediate target にしない。

## current source-backed floor

### すでにあるもの

- tool-neutral formal hook
- `proof_notebook_review_unit` first pilot
- row-local machine-facing `model_check_concrete_carriers`
- source-sample emitted verification artifact wiring
- sample-facing theorem/model-check evidence summary and bless/review flow
- docs-first I/O / host-facing boundary
- shared-space identity / admission / authority の docs-first boundary
- `atomic_cut` の place-local executable nucleus

### まだ無いもの

- full strong type system
- concrete theorem prover binding
- concrete model-check tool binding
- first settled property language
- higher-level ordering / fairness / witness-aware handoff の settled surface
- low-level `memory_order` family の settled reinterpretation

## current reading

- 型 / 証明 / モデル検査は **見通しが厳しすぎて止まっている線ではない**。
  すでに carrier、hook、pilot、sample-facing summary があるため、boundary と pilot plan は十分進められる。
- ordering / `memory_order` 再解釈も **完全に未来の夢想ではない**。
  ただし、いまは theory-first boundary inventory を進める段階であり、implementation-ready ではない。
- current mainline は引き続き `Macro 4` と `Macro 7` で閉じる。
  この文書の 4 系統は **adjacent research program** として並走する。

## Track A. typed-core / checker boundary

### 主題

- typed work をどの carrier / checker boundary / source-visible surface へ最初に接続するか。
- full type calculus ではなく、**first attachment candidate** を決める。

### source-backed な出発点

- typed evidence refs
- contract rows
- checker-side boundary inventory
- source-visible candidates
  - `require` / `ensure`
  - capability / `lease` / `admit`
  - declared target
  - lineage-related floor

### current recommendation

- first attachment は semantic carrier / checker boundary から比較する。
- source-visible syntax を immediately typed syntax にしない。
- first cut では、
  - obligation owner
  - typed attachment candidate
  - stop line
  を fixed すればよい。

### self-driven package order

1. obligation allocation matrix refresh
2. typed-core attachment inventory
3. first source-visible typed-surface comparison

### stop line

- full type calculus
- inference / annotation design
- final typed syntax
- public typed API

## Track B. theorem-side pilot

### 主題

- semantic core の invariant を theorem-side pilot へ落とす。
- proof object、review notebook、verifier handoff row を collapse しない。

### current recommendation

- first theorem line は syntax tree 全体ではなく semantic-core relation library から始める。
- proof order は次の順を第一候補にする。
  1. `canonical_normalization_law`
  2. `no_re_promotion`
  3. `rollback_cut_non_interference`

### self-driven package order

1. semantic-core theorem pilot planning
2. first lemma family wording hardening
3. proof artifact / bridge stop-line refresh

### stop line

- concrete theorem prover brand
- proof object public contract
- review workflow finalization

## Track C. model-check / protocol verification

### 主題

- current machine-facing carrier の次に、何を projection / property family として固定するか。
- theorem line と public checker line を混ぜない。

### source-backed な出発点

- row-local machine-facing `model_check_concrete_carriers`
- emitted artifact wiring
- sample-facing summary
- public checker / verifier handoff の retained docs chain

### current recommendation

- first projection は row-local / small-cluster に留める。
- first property family は transition-system 全体より narrow に取る。
- concrete model-check tool binding は still later に残す。

### self-driven package order

1. model-check projection / property-family reserve inventory
2. model-check carrier to projection bridge note
3. sample-visible property summary wording

### stop line

- concrete tool brand
- full protocol family
- production checker / runtime policy contract

## Track D. async-control / ordering / `memory_order` reinterpretation

### 主題

- `atomic_cut` の local nucleus と higher-level ordering / fairness / witness-aware handoff をどう分けるか。
- low-level `memory_order` 語彙を immediate surface にしない。

### source-backed な出発点

- `atomic_cut` の place-local executable nucleus
- proof/runtime policy boundary inventory
- shared-space identity / admission / authority boundary
- retained theorem-side handoff / authority-side docs chain

### current recommendation

- `atomic_cut` は current executable nucleus に留める。
- higher-level ordering は
  - authority-serial
  - witness-aware handoff
  - replay / fairness / room policy
  の family として inventory 化する。
- low-level `memory_order` vocabulary は immediate candidate にしない。

### self-driven package order

1. async invariant-dependency labeling
2. authority-serial minimal contract-row inventory
3. witness-aware handoff boundary note
4. low-level vocabulary exclusion note

### stop line

- hardware-memory-like semantics
- scheduler semantics finalization
- runtime implementation
- proof / model-check concrete binding

## Track E. shared-space / host-I/O との接続

### shared-space 側

- current docs-first boundary は
  - membership identity
  - admission / visibility
  - authority / ownership
  まである。
- type/proof/model-check/ordering line は、この boundary を越えて final catalog を勝手に作らない。

### host-I/O 側

- current docs-first boundary は capability-scoped input/output port / adapter boundary に留める。
- theorem/model-check artifact や ordering artifact を、immediate に concrete host contract へ昇格しない。

## recommended execution order

1. `Macro 4` stable malformed capability second source-backed widening actualization
2. `Macro 7` public operational CLI concrete shell actualization
3. Track A package 1
4. Track A package 2
5. Track B package 1
6. Track C package 1
7. shared-space room-profile / confusion-replay compact table
8. host-I/O binding artifact / bridge-only note
9. Track D package 1
10. Track D package 2

## autonomy reading

### self-driven で進めてよい

- Track A packages 1〜2
- Track B package 1
- Track C package 1
- Track D packages 1〜2

### mixed gate で止める

- concrete theorem / model-check tool binding
- room / host / protocol final contract
- external integration target actualization

## user が later に決めること

- final shared-space catalog
- first external integration target
- first application target
- final public packaging success criteria

## short conclusion

- 型 / 定理証明 / モデル検査は、**boundary と pilot plan を進められる段階** にある。
- ordering / `memory_order` 再解釈も、**theory-first inventory を進められる段階** にある。
- ただし、どちらも current mainline implementation へ即昇格させる段階ではない。
