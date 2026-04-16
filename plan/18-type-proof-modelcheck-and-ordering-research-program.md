# plan/18 — 型 / 定理証明 / モデル検査 / ordering / syntax / modality 研究計画

## 目的

この文書は、次の理論線を 1 箇所で detail-side に管理する。

- typed-core / checker boundary
- theorem-side pilot
- model-check / protocol-verification reserve line
- cut / order / visibility / witness decomposition
- authority-handoff / replay / payload boundary
- syntax honesty / surface discipline
- modal foundation comparison
- adequacy corpus / verifier-boundary matrix

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
- higher-level async-control first cut を `authority_serial_transition_family` に置く docs-first comparison
- low-level memory-order family を retained-later reference family に留める docs-first comparison

### まだ無いもの

- full strong type system
- concrete theorem prover binding
- concrete model-check tool binding
- first settled property language
- higher-level ordering / fairness / witness-aware handoff の settled public surface
- low-level `memory_order` family の settled reinterpretation
- final parser grammar
- final public verifier contract

## current reading

- 型 / 証明 / モデル検査は **見通しが厳しすぎて止まっている線ではない**。
  すでに carrier、hook、pilot、sample-facing summary があるため、boundary と pilot plan は十分進められる。
- ordering / `memory_order` 再解釈も **完全に未来の夢想ではない**。
  ただし、いまは theory-first boundary inventory を進める段階であり、implementation-ready ではない。
- syntax / modality line も、final grammar や final calculus はまだ先だが、
  semantic cluster / comparison axis / stronger-foundation candidate の整理は今進められる。
- current mainline は引き続き `Macro 4` と `Macro 7` で閉じる。
  この文書の理論線は **adjacent theory-lab program** として並走する。

## theory-lab operating model

### roles

1. literature scout
   - primary source の bibliographic check
   - safe-to-cite claim と overstatement risk の整理
2. formalizer
   - candidate family、relation decomposition、property matrix、adequacy corpus を整理
3. prototyper
   - tiny non-production prototype / simulator / compare helper を切る
4. falsifier
   - counterexample、misreading path、kill criteria を集める
5. integrator
   - accepted candidate だけを `specs/` / `plan/` / `docs/reports/` に昇格する

### rules

- integrator だけが main docs / plan / spec を更新する。
- exploratory candidate は sandbox / report first に留める。
- accepted candidate だけを repo memory に昇格する。
- prototype は helper-local / non-production / examples-support / script-side に留める。
- comparison candidate を decision register に premature に押し込まない。

### research package template

- question
- candidate family
- adequacy corpus
- minimal prototype / executable semantics
- kill criteria
- promotion criteria

## primary literature pack

### distributed order / cuts

- Lamport 1978
  - partial order / happened-before / logical clocks
- Chandy–Lamport 1985
  - consistent snapshot / stable property / snapshot vs commit distinction

### shared-memory / scoped synchronization / dependency ordering

- Boehm–Adve 2008
  - language-level memory model pressure
- HSA System Architecture 1.2
  - scopes / ownership / visibility / order decomposition reference
- WG21 P0750R1
  - dependency ordering as reference family
- WG21 P3475R2
  - direct surface import risks for `consume`
- optional comparison: WG21 P1239

### relaxed memory / event structures / correctness conditions

- Jeffrey–Riely 2019
  - event structures / thin-air / well-justification
- Herlihy–Wing 1990
  - linearizability as comparison point for room/object-level seriality

### staged / modal / multimodal foundations

- Davies–Pfenning 2001
- Yuse–Igarashi 2006
- Guarded Lambda-Calculus
- Modal Dependent Type Theory and Dependent Right Adjoints
- Multimodal Dependent Type Theory

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

## Track D. cut / order / visibility / witness decomposition

### 主題

- `atomic_cut` の local nucleus と higher-level ordering / handoff / fairness family をどう分けるか。
- low-level `memory_order` 語彙を immediate surface にしない。

### candidate decomposition

#### cut family

- local-finalizing cut
- observation / snapshot cut
- ordering-only barrier
- commit / evidence-bearing cut

#### order family

- `po`
- `dep`
- `pub`
- `obs`
- `wit`
- `final`
- `hb(scope)`

#### authority-handoff family

- owner slot
- transition stage family
- stage sequence
- stage-local obligation
- handoff epoch ref
- witness-aware handoff
- replay attachment
- payload ref

### current recommendation

- `atomic_cut` は current executable nucleus に留める。
- higher-level family は
  - `authority_serial_transition_family`
  - `witness_aware_commit_family`
  - `event_tree_execution_view`
  の順で比較する。
- low-level `memory_order` family は retained-later reference family に留める。
- `kill_dependency` line は dependency relation をどこで切るかの reference family として扱い、current final surface にはしない。

### self-driven package order

1. cut-family comparison and naming discipline
2. order / visibility / witness relation inventory
3. thread / node parity note
4. verifier-boundary matrix refresh

### stop line

- hardware-memory-like source surface
- scheduler semantics finalization
- runtime implementation
- proof / model-check concrete binding

## Track E. syntax honesty / stimuli extraction

### 主題

- natural surface と semantic honesty をどう両立するか。
- rough syntax sketch を adoption 候補ではなく comparison material として扱う。

### current recommendation

- final grammar を急がず、semantic cluster を先に固定する。
- syntax candidate は少なくとも
  - semantic honesty
  - checker legibility
  - modal adequacy
  - misreading resistance
  の 4 軸で比較する。
- rough stimuli は
  - publication / handoff / witness
  - room-local serial transition sugar
  - low-level / backend fence family
  - high-level room macro
  の structural extraction 材料として読む。

### self-driven package order

1. syntax-stimuli comparison note
2. misreading catalog / kill criteria
3. semantic-cluster-first parser note

### stop line

- final parser grammar
- final punctuation / precedence table
- source-surface adoption of low-level fence syntax

## Track F. modal foundation comparison

### 主題

- `lambda-circle-box` partial basis と guarded / MDTT / MTT stronger candidate をどう切り分けるか。

### current recommendation

- `lambda-circle-box` は stage / later / always の partial basis candidate として扱う。
- guarded recursion / guarded lambda-calculus、MDTT / dependent right adjoints、MTT / multimodal line を stronger candidate として比較する。
- final adoption は OPEN に残す。

### self-driven package order

1. partial-basis vs stronger-candidate table
2. place / scope / authority / witness coverage gap note
3. theory-lab follow-up criteria

### stop line

- final modal calculus adoption
- syntax-level modality markers
- implementation-level type checker commitment

## Track G. adequacy corpus / verifier-boundary matrix

### adequacy corpus family

1. authoritative room: A rolls, publication, handoff to B, B sees witnessed past
2. same scenario の shared-memory / same-process analog
3. rollback before local cut
4. rollback after local cut
5. snapshot-only cut vs durable-cut difference
6. late join / rejoin after handoff
7. leave / reconnect with stale message
8. delegated RNG provider with authority commit
9. append-friendly room where authority-serial transition is overkill
10. observation / `emit`-bearing variant

### property-to-boundary matrix target

- same-owner / same-authority structural floor
- stage sequence well-formedness
- witness field presence
- local cut non-interference
- no hidden re-promotion analog
- handoff requires prior publication
- handoff requires witness
- replay / duplicate invalidation
- room-level single-transition seriality
- late-join / rejoin safety
- fairness claim
- distributed provider / receipt validation
- emit / observation ordering

### current recommendation

- adequacy corpus は docs-only で揃えてよい。
- falsifier packages は counterexample and confusion path を集める。
- matrix は 4-way split を維持して整理する。

## Track H. shared-space / host-I/O との接続

### shared-space 側

- current docs-first boundary は
  - membership identity
  - admission / visibility
  - authority / ownership
  まである。
- type / proof / model-check / ordering line は、この boundary を越えて final catalog を勝手に作らない。

### host-I/O 側

- current docs-first boundary は capability-scoped input/output port / adapter boundary に留める。
- theorem/model-check artifact や ordering artifact を、immediate に concrete host contract へ昇格しない。

## recommended execution order

1. `Macro 4` stable malformed capability second source-backed widening actualization
2. `Macro 7` public operational CLI concrete shell actualization
3. Track A package 1
4. Track B package 1
5. Track C package 1
6. Track D package 1
7. Track G package 1
8. Track E package 1
9. Track F package 1

## autonomy reading

### self-driven で進めてよい

- Track A packages 1〜2
- Track B package 1
- Track C package 1
- Track D packages 1〜4
- Track E packages 1〜3
- Track F packages 1〜3
- Track G packages 1〜2

### mixed gate で止める

- concrete theorem / model-check tool binding
- room / host / protocol final contract
- external integration target actualization
- final modal/type foundation adoption

## user が later に決めること

- final shared-space catalog
- first external integration target
- first application target
- final public packaging success criteria

## short conclusion

- 型 / 定理証明 / モデル検査は、**boundary と pilot plan を進められる段階** にある。
- order / memory / authority-handoff / syntax / modality line も、**comparison と adequacy-corpus を進められる段階** にある。
- ただし、いずれも current mainline implementation へ即昇格させる段階ではない。
