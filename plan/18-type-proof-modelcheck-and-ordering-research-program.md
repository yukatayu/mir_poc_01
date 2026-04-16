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

ここでの目的は、**current execution lane を壊さずに研究を前へ進めること** である。
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
- Chandy–Lamport 1985

### shared-memory / scoped synchronization / dependency ordering

- Boehm–Adve 2008
- HSA System Architecture 1.2
- WG21 P0750R1
- WG21 P3475R2
- optional comparison: WG21 P1239

### relaxed memory / event structures / correctness conditions

- Jeffrey–Riely 2019
- Herlihy–Wing 1990

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
- source-visible candidates:
  `require` / `ensure`、capability / `lease` / `admit`、declared target、lineage-related floor

### current recommendation

- first attachment は semantic carrier / checker boundary から比較する。
- source-visible syntax を immediately typed syntax にしない。
- current first typed attachment candidate は checker-adjacent semantic carrier に置く。
- current package では
  - obligation owner
  - typed attachment candidate
  - stop line
  を fixed すればよい。

### current package close

- obligation allocation matrix refresh
- typed-core attachment inventory
- first source-visible typed-surface comparison

### current package chain detail

- checker attachment から handoff row への migration note
  - principal source は checker attachment に残す
  - source-visible marker は mirror に留める
  - handoff row は derived reserve row に留める
- request / predicate / `try` cluster typed-surface reserve note
  - request / predicate / `try` cluster は grouped reserve cue に留める
  - checker attachment principal は維持する
- typed-surface family unification keep/drop note
  - source-visible structural marker family と reserve cluster family を two-tier split に置く
  - shared attachment shape と stronger typed surface promotion は later threshold に残す

### next package order

1. no immediate near-term package
   - stronger typed surface promotion は later threshold で再評価する

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
- current first concrete consumer は notebook-first line に維持する。

### current package close

- semantic-core theorem pilot planning
- first theorem lemma family wording hardening

### current package close detail

- proof artifact / bridge stop-line refresh
  - formal hook / review unit / bridge sketch / theorem discharge を collapse しない
  - notebook-first review pressure を first threshold に置く
- admissible evidence contraction note
  - admissible theorem evidence は symbolic refs only に contraction する
  - review prose は discharge evidence に混ぜない
- notebook-consumer threshold and discharge reserve note
  - abstract discharge-entry row を first threshold に置く
  - concrete discharge result / transport / public theorem contract は later gate に残す

### next package order

1. no immediate near-term package
   - theorem discharge transport / public-contract gate は later reserve に残す

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
- room protocol / fairness / replay / provider receipt family は order / handoff 側 reserve に残す。

### current package close

- model-check projection / property-family reserve inventory
- model-check carrier to projection bridge note

### current package close detail

- sample-visible property summary wording
  - row-local carrier floor を machine-facing に保つ
  - small-cluster projection を reserve に留める
  - room protocol / fairness / replay family を order/handoff reserve に残す
- tool-binding stop-line refresh
  - explicit non-goal を固定し、concrete tool binding を still later に残す
- model-check small-cluster projection keep/drop refresh
  - same-subject stage-local small cluster を keep line に置く
  - typed reserve / theorem discharge / room protocol family を drop する

### next package order

1. no immediate near-term package
   - first settled property language / concrete tool seam は later gate で再評価する

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

- `program_order`
- `dependency_order`
- `publication_order`
- `observation_order`
- `witness_order`
- `finalization_order`
- `scoped_happens_before`

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
- higher-level family の current reduction は
  - `authority_serial_transition_family` を first candidate
  - `witness_aware_commit_family` を second candidate
  - `event_tree_execution_view` を derived/debug candidate
  に置く。
- low-level `memory_order` family は retained-later reference family に留める。
- `kill_dependency` line は dependency relation をどこで切るかの reference family として扱い、current final surface にはしない。

### near-term package order

1. cut-family comparison and naming discipline
2. order / visibility / witness relation inventory
3. thread / node parity note
4. verifier-boundary matrix refresh
5. falsifier loop / adequacy corpus hardening
6. candidate reduction after falsifier hardening
7. order / handoff property-language bridge note
8. order / handoff emitted-artifact schema reserve note
9. order / handoff source-surface wording reserve note

### current package close detail

- `4A cut/order falsifiers`
  - cut conflation と relation-collapse の negative family
- `4B handoff/replay/provider falsifiers`
  - publication / witness / epoch / receipt / fairness の negative family
- `4C boundary-matrix hardening`
  - property row と adequacy corpus id の接続
- `4D candidate reduction`
  - `authority_serial_transition_family` / `witness_aware_commit_family` / `event_tree_execution_view` / low-level reference family の keep/drop line

current package chain では、negative corpus coverage、higher-level candidate reduction、property-language bridge までを docs-first に固める。
actual protocol projection、final emitted-artifact schema、tool binding は still later に残す。
さらに current reserve line では、
refs-only reserve schema を first cut とし、consumer-shaped schema と source-surface-first schema は still later に残す。

### latest package close detail

- order / handoff source-surface wording reserve note
  - snake_case relation family 名を principal wording に保つ
  - plain-language stage wording を explanation layer に重ねる
  - low-level fence-like wording と room macro wording は still later に残す

### stop line

- hardware-memory-like source surface
- scheduler semantics finalization
- runtime implementation
- proof / model-check concrete binding

## Track E. syntax honesty / stimuli extraction

### 主題

- natural surface と semantic honesty をどう両立するか。
- rough syntax sketch を adoption 候補ではなく comparison material として扱う。

### rough stimuli の current読み

- stimulus A:
  publication / handoff / witness 分離の刺激
- stimulus B:
  room-local serial transition sugar の刺激
- stimulus C:
  low-level / backend fence family の刺激
- stimulus D:
  high-level room macro / transaction-like sugar の刺激

### current recommendation

- final grammar を急がず、semantic cluster を先に固定する。
- syntax candidate は少なくとも
  - semantic honesty
  - checker legibility
  - modal adequacy
  - misreading resistance
  の 4 軸で比較する。

### near-term package order

1. syntax-stimuli comparison note
2. misreading catalog / kill criteria
3. semantic-cluster-first parser note

### stop line

- final parser grammar
- final punctuation / precedence table
- source-surface adoption of low-level fence syntax

## Track F. modal foundation comparison

### 主題

- `lambda-circle-box` を partial basis に留めつつ、
  guarded / MDTT / MTT / Fitch-style stronger candidate へどう接続するか。

### current recommendation

- `lambda-circle-box` は stage / later / always の micro-core には有力と読む。
- ただし full language は place / scope / visibility / authority / witness / durability の multi-axis を要するため、
  guarded lambda-calculus と MDTT/MTT cluster を stronger candidate family として並行比較する。

### near-term package order

1. partial-basis note refresh
2. stronger-candidate comparison
3. modal foundation comparison follow-up
4. stop-line and promotion-threshold note
5. guarded-vs-MDTT/MTT reduction timing note
6. modality internalization trigger note

### stop line

- final foundation adoption
- full dependent calculus
- implementation binding

## current near-term theory-lab sequence

1. modality internalization trigger note
2. malformed duplicate-cluster source-authored static-stop pair actualization comparison

## current recommendation

- theory-lab line は execution lane の「あとで」ではなく、今進めてよい adjacent program として扱う。
- ただし accepted candidate を current executable / public surface へ直結させない。
