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
- corrected runnable prototype sample は `samples/prototype/` に置き、exact rough stimulus は `samples/not_implemented/` に残す current bucket policyで、sample-driven falsifier / usability comparison を補助してよい。
- typed marker family の current sample-visible corrected prototype tranche は `p06-typed-proof-owner-handoff`、`p10-typed-authorized-fingerprint-declassification`、`p11-typed-unauthorized-fingerprint-release`、`p12-typed-classified-fingerprint-publication-block` までで current cut を close 済みである。
- order/handoff corrected prototype third tranche は `p07` late join visibility / `p08` stale reconnect refresh までで current cut を close 済みである。
- corrected prototype tranche close は theory-lab solved を意味しない。
  current queue reading は、`specs/examples/458...465` を compare-floor anchor、`specs/examples/466...469` を actual-adoption / near-end-closeout anchor、`specs/examples/470...474` を helper-local actualization / narrowing anchor、`specs/examples/475...519` を deeper-theory / reserve / second-line / theorem-model-check / shared-space / order-handoff / actual-execution actualization / actual-adoption anchor、`specs/examples/520` を final-layer closeout defaults / reopened self-driven queue anchor とする reading にある。
- sample debugging 用の helper-local preview として、`debug_*` または `_debug_` を含み `_output` / `_pipe` で終わる target の record を CLI `debug_outputs` へ出してよい。
  ただしこれは final stdio / final host-I/O / final transcript schema を意味しない。
- theorem/model-check bridge の current floorを sample-visible にする helper-local preview として、`formal_hook_status` / `subject_kind` / obligation list を CLI `verification_preview` へ出してよい。
  ただしこれは final public verifier contract を意味しない。
- current mixed-gate pre-floor として、sample-local preview-aligned typed artifact route を test/support helper に置き、`verification_preview` / `artifact_preview` の compare floor を固定してよい。
  ただしこれは final public verifier contract、final emitted schema、concrete tool seam を意味しない。
- current mixed-gate pre-floor として、sample-local model-check projection pre-floor route を test/support helper に置き、row-local carrier / small-cluster projection / property-language seam / tool-binding seam の compare floor を固定してよい。
  ただしこれは settled property language adoption、concrete tool seam adoption、production checker/runtime-policy contract を意味しない。
- current mixed-gate pre-floor として、sample-local theorem discharge pre-floor route を test/support helper に置き、row-local review unit / discharge-entry reserve / transport seam / public-contract seam の compare floor を固定してよい。
  ただしこれは actual discharge transport adoption、public theorem contract adoption、concrete theorem prover binding、proof object public schemaを意味しない。
- current helper-local mixed-gate actualization として、sample-local theorem discharge actual-format probe route を test/support helper に置き、transport preview / public-contract preview / notebook-consumer-first boundary の probe floor を固定してよい。
  ただしこれは actual discharge transport、public theorem contract、concrete theorem prover brand、proof object public schemaを意味しない。
- current helper-local mixed-gate threshold として、sample-local theorem discharge / public-theorem-contract threshold route を test/support helper に置き、review-unit first / discharge-entry adjacent / notebook-consumer-first / brand-neutral theorem request default の threshold floor を固定してよい。
  ただしこれは actual discharge transport、public theorem contract、theorem result public object、concrete theorem prover brand、proof object public schemaを意味しない。
- current helper-local mixed-gate actualization として、sample-local model-check property/tool-seam probe route を test/support helper に置き、property-language probe / tool-seam probe / checker-boundary probe の floor を固定してよい。
  ただしこれは first settled property language、concrete model-check tool brand、actual public checker migration、production checker/runtime-policy contract を意味しない。
- current helper-local mixed-gate threshold として、sample-local model-check property/tool threshold route を test/support helper に置き、row-local-property-preview first / small-cluster-projection second / brand-neutral request keep / public-checker-contract-later の threshold floor を固定してよい。
  ただしこれは first settled property language、concrete model-check tool brand、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract を意味しない。
- current helper-local mixed-gate actual adoption として、sample-local model-check row-local property actual-adoption route を test/support helper に置き、row-local property route first / checker-boundary contract first / brand-neutral tool-binding reserve keep の repo-local actual adoption floor を固定してよい。
  ただしこれは first settled property language、concrete model-check tool brand、consumer-shaped public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contract を意味しない。
- current helper-local mixed-gate actual adoption として、sample-local witness/provider/artifact public-shape actual-adoption route を test/support helper に置き、claim/payload split first / witness-provider route non-collapse / optional attachment refs only / repo-local emitted artifact refs first の repo-local actual adoption floor を固定してよい。
- current helper-local mixed-gate actualization として、sample-local witness/provider public-contract / emitted-handoff coupled-later route を test/support helper に置き、claim/payload split first / witness-provider route non-collapse / repo-local emitted artifact refs first / combined public contract later / final emitted-handoff contract later の coupled-later floor を固定してよい。
  ただしこれは final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted handoff contract、exhaustive shared-space catalog を意味しない。
- current helper-local mixed-gate actual adoption として、sample-local order-handoff surface actual-adoption route を test/support helper に置き、edge-row principal / stage-block secondary keep / repo-local emitted artifact refs first の repo-local actual adoption floor を固定してよい。
  ただしこれは final parser grammar、final source-surface handoff wording、final emitted handoff contract、final public witness/provider/artifact contract、authoritative-room `serial` sugar adoption、low-level exact source wording、final modal foundation adoption を意味しない。
- principal theory spine / proof roadmap integration として、multimodal dependent core research direction、layered theory stack、compatibility metatheory package、Lean-first proof roadmap を current recommendation に上げてよい。
  ただしこれは final adopted calculus、final proof object contract、production theorem/model-check binding を意味しない。

## principal direction after handoff integration

### principal theory spine

- current theory-lab direction は **multimodal dependent core** を principal spine に置く。
- `lambda_circle_box` は partial basis candidate に留める。
- guarded lambda-calculus、MDTT / MTT cluster、Fitch-style multimodal family は stronger candidate family として retained する。
- current package は final adopted calculus を fixed するためではなく、typed / theorem / model-check / order-handoff line を deeper theory spine で一貫して読むための package である。

### layered theory stack

current recommendation は、次の layered stack を separate responsibility として保つことである。

1. multimodal dependent core
2. linear capability / ownership layer
3. effect-row / operation layer
4. decidable refinement / contract layer
5. information-flow / label layer
6. theorem / model-check bridge layer

- 各 layer は collapse せず、obligation export / erasure / weakening の compatibility metatheory で接続する。
- current repo は all-in-one final type system を immediate target にしない。
- local theory layer を module / region ごとに差し込む場合も、compatibility metatheory の floor を先に要求する。

### conceptual judgment sketch

- current docs-first sketch は、
  `Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ O`
  のように
  modality/context、ordinary context、linear context、type、mode、effect row、obligation export
  を読み分ける方向に置く。
- これは exact parser grammar や final checker syntax の固定ではなく、layer split を説明する conceptual floor である。

### IFC / label-model first fragment

- current closeout package では、IFC を vague future line に残さず、layered theory stack の first-class package として扱う。
- current minimal sketch は、
  - `LabelModel`
  - `flows_to`
  - `join`
  - `meet`
  - `flows_dec`
  - `Labeled ℓ A`
  を reference floor に置く。
- first checker fragment では、
  - explicit data flow respects `flows_to`
  - pc-flow respected
  - labeled capability move/share respects policy
  - declassification / endorsement require explicit authority
  を current recommendation に置く。
- covert-channel theorem、distributed trust finalization、final label API は mixed gate / later proof package に残す。

### compatibility metatheory package

- current package で最低限必要なのは、
  - layer weakening / erasure がどこまで admissible か
  - obligation export がどの boundary を越えられるか
  - theorem/model-check artifact がどの layer までを参照してよいか
  - local theory layer の import/export がどの compatibility floor を要するか
  を separate に書くことである。
- exact manifest schema、serialization、public artifact contract は mixed gate に残す。

### proof route and staged roadmap

- current recommendation は **Lean-first** である。
- Rocq/Coq + Iris family は runtime concurrency / separation-logic pressure が強まったときの fallback family として retained する。
- staged roadmap は次を first line に置く。
  1. semantic-core relation library
  2. emitted artifact / obligation export conformance
  3. authoritative-room / fabric concurrency pressure に応じた stronger metatheory
- current execution / helper-local actualization を止めて production proof stack へ一気に移行する必要はない。

### final-layer closeout package reading

- `specs/examples/520` により、current self-driven queue は next active line を
  1. layered strong typing / IFC first-fragment
  2. Lean formal skeleton / proof obligations
  3. helper / CLI hardening and broader coverage
  4. near-end closeout sync
  に reopened して読む。
- `specs/examples/521`、`522`、`523`、`524`、`525`、`526`、`527`、`528` により、`Lean formal skeleton / proof obligations` の first slice、IFC secret valid/invalid concrete example、source-side authority pair、source-side label-flow negative、delegated RNG provider placement carry-over、order-handoff helper CLI surface preview、order-handoff negative static-stop pair、order-handoff negative pair theorem-side Lean carry-over は `samples/lean/foundations/CurrentL2ProofSkeleton.lean`、`CurrentL2IfcSecretExamples.lean`、representative Lean sample set committed Lean corpus、export/sync helper、source-side corrected prototype trio、broader order-handoff representative slice、helper-local CLI summary、source-sample runner static gate に actualize 済みである。current live queue はそこから先の helper/CLI hardening、IFC helper widening、later mixed gate に narrowed すると読む。
- これは final public theorem/model-check contract や final parser/public API を reopen したことを意味しない。

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
- current typed attachment は layered theory stack の 2〜4 layer を薄く参照する reading で十分であり、full dependent calculus や final elaborator を immediate target にしない。
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

1. current promoted near-term package
   - current first-line integration は `specs/examples/459` で close 済みである
2. recurring integration package
   - `Package E` close 済みとして、snapshot / plan / spec の current line を near-end reading に同期する
3. next topic は mixed gate
   - stronger typed surface promotion の実昇格は mixed gate に残す

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
- proof route は Lean-first を current recommendation に置き、artifact conformance bridge を second stage に置く。
- theorem review-unit to Lean-stub repo-local artifact-conformance bridge は `specs/examples/509` で close 済みである。
- theorem Lean-stub representative trace-alignment bridge は `specs/examples/510` で close 済みである。

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

1. current promoted near-term package
   - current first-line integration は `specs/examples/459` で close 済みである
2. recurring integration package
   - `Package E` close 済みとして、snapshot / plan / spec の current line を near-end reading に同期する
3. helper-local actualization package
   - theorem-first experimental pilot actualization は `specs/examples/470` で close 済みである
4. next topic は mixed gate
   - theorem prover experimental binding preflight は `specs/examples/474` で close 済みである
   - theorem Lean-first non-production stub pilot actualization は `specs/examples/508` で close 済みである
   - theorem review-unit to Lean-stub repo-local artifact-conformance bridge は `specs/examples/509` で close 済みである
   - theorem Lean-stub representative trace-alignment bridge は `specs/examples/510` で close 済みである
   - order-handoff serial-scope reserve surface は `specs/examples/511` で close 済みである
   - witness/provider emitted-contract representative trace-alignment bridge は `specs/examples/512` で close 済みである
   - theorem actual Lean execution availability probe は `specs/examples/513` で close 済みである
   - theorem public-seam compression は `specs/examples/514` で close 済みである
   - order-handoff / witness-provider final public seam compression は `specs/examples/515` で close 済みである
   - theorem toolchain probe / reopen manifest は `specs/examples/516` で close 済みである
   - model-check public-seam compression は `specs/examples/517` で close 済みである
   - theorem actual Lean execution narrow probe は `specs/examples/518` で close 済みである
   - theorem actual Lean execution representative prototype widening は `specs/examples/519` で close 済みである
   - final-layer closeout defaults / reopened self-driven queue は `specs/examples/520` で close 済みである
   - Lean sample corpus and first foundations は `specs/examples/521` で close 済みである
   - next active line は layered strong typing / IFC first-fragment、actual Lean execution helper/CLI hardening と broader coverage、および later mixed gate に置く
   - theorem discharge transport / public-contract の実昇格は mixed gate に残す
   - preview-alignment pre-floor は `specs/examples/463` で close 済みである
   - theorem discharge pre-floor は `specs/examples/465` で close 済みである

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
- model-check bridge は layered theory stack の bridge layer に置き、typed core や theorem discharge line を public contract level で collapse しない。

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

1. current promoted near-term package
   - current first-line integration は `specs/examples/459` で close 済みである
2. recurring integration package
   - `Package E` close 済みとして、snapshot / plan / spec の current line を near-end reading に同期する
3. next topic は mixed gate
   - first settled property language / concrete tool seam の実昇格は mixed gate に残す
   - preview-alignment pre-floor は `specs/examples/463` で close 済みである
   - model-check projection pre-floor は `specs/examples/464` で close 済みである

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
- order / handoff surface narrowing and stage-block secondary candidate
  - explicit edge-row / vertical continuation を source principal に維持する
  - `stage` / `after` / `witness` family を strong secondary experimental candidate に留める
  - authoritative-room `serial` sugar は reserve に残す

### current promoted package

- `Package C` は `specs/examples/460` で close 済みである。
  - cut family decomposition、relation decomposition、authority-handoff family、thread/node parity、wording reserve を
    current first line / retained alternatives / mixed-gate boundary として再統合した

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
  - lowering friendliness
  の 5 軸で比較する。
- order / handoff surface では explicit edge-row / vertical continuation を principal に保ち、`stage-block + witness` は secondary candidate に留める。
- authoritative-room `serial` sugar は room-specific reserve に留める。

### near-term package order

1. syntax-stimuli comparison note
2. misreading catalog / kill criteria
3. semantic-cluster-first parser note
4. current promoted near-term package
   - `Package D` で syntax axis と stop line を `specs/examples/461` へ再統合する

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
7. current promoted near-term package
   - `Package D` で `lambda_circle_box` partial basis、guarded / MDTT / MTT / Fitch-style multimodal keep、boundary-pressure trigger を `specs/examples/461` へ current first line として再統合する
8. final adoption は mixed gate に残す

### stop line

- final foundation adoption
- full dependent calculus
- implementation binding

## current near-term theory-lab sequence

1. actual adoption packages `466...469` は close 済み
2. helper-local actualization / narrowing packages `470...474` は close 済み
3. principal theory spine / Lean-first proof roadmap package `475` は close 済み
4. reserve strengthening actualization package `476` は close 済み
5. reserve practical actualization package `477` は close 済み
6. model-check second-line concretization package `478` は close 済み
7. theorem discharge actual-format probe package `479` は close 済み
8. model-check property/tool-seam probe package `480` は close 済み
9. theorem discharge / public-theorem-contract threshold package `481` は close 済み
10. model-check property/tool threshold package `482` は close 済み
11. witness/provider/artifact public-shape threshold package `483` は close 済み
12. order-handoff surface / artifact threshold package `484` は close 済み
13. theorem contract shape threshold package `485` は close 済み
14. theorem transport/public-contract coupled later gate package `486` は close 済み
15. theorem review-unit transport / notebook-contract actual adoption package `487` は close 済み
16. theorem result-object preview / proof-object-schema reserve actualization package `491` は close 済み
17. model-check public-checker artifact preview / verifier-handoff reserve actualization package `492` は close 済み
18. witness/provider public-contract / emitted-handoff coupled-later package `493` は close 済み
19. theorem proof-object schema / prover-brand coupled-later package `494` は close 済み
20. model-check tool-brand / verifier-handoff coupled-later package `495` は close 済み
21. order-handoff source wording / emitted-artifact coupled-later package `496` は close 済み
22. theorem result object / payload public-contract coupled-later package `497` は close 済み
23. model-check public checker artifact / migration coupled-later package `498` は close 済み
24. witness/provider public-schema coupled-later package `499` は close 済み
25. theorem result-object route actual-adoption package `500` は close 済み
26. model-check checker-artifact route actual-adoption package `501` は close 済み
27. witness/provider route actual-adoption package `502` は close 済み
28. order-handoff source-wording route actual-adoption package `503` は close 済み
29. witness/provider schema-route actual-adoption package `504` は close 済み
30. witness/provider final-public-contract reopen-threshold package `505` は close 済み
31. theorem final-public-contract reopen-threshold package `506` は close 済み
32. model-check final-public-contract reopen-threshold package `507` は close 済み
33. corrected runnable version の current floorは already reached
34. order-handoff serial-scope reserve surface package `511` は close 済み
35. witness/provider emitted-contract representative trace-alignment bridge package `512` は close 済み
36. theorem actual Lean execution availability probe package `513` は close 済み
37. theorem public-seam compression package `514` は close 済み
38. order-handoff / witness-provider final public seam compression package `515` は close 済み
39. theorem toolchain probe / reopen manifest package `516` は close 済み
40. model-check public-seam compression package `517` は close 済み
41. theorem actual Lean execution narrow probe package `518` は close 済み
42. theorem actual Lean execution representative prototype widening package `519` は close 済み
43. final-layer closeout defaults / reopened self-driven queue package `520` は close 済み
44. current active line は layered strong typing / IFC first-fragment、actual Lean execution helper/CLI hardening と broader coverage、および later mixed gate
44. remaining actual-adoption topics は mixed gate
   - stronger typed surface promotion
   - final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema / final public verifier contract
   - first settled property language / concrete model-check tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact / production checker/runtime-policy contract / final public verifier contract
   - final source-surface handoff wording / final emitted-artifact schema
   - witness / provider / emitted-artifact public-shape reserve

## current recommendation

- theory-lab line は execution lane の「あとで」ではなく、adjacent program として扱う。
- current snapshot では compare-floor、actual-adoption package、helper-local actualization package、surface narrowing、theorem-prover binding preflight、principal theory spine / Lean-first proof roadmap package、reserve strengthening actualization package、reserve practical actualization package、model-check second-line concretization package、theorem discharge/public-contract threshold package、model-check property/tool threshold package、witness/provider/artifact public-shape threshold package、order-handoff surface/artifact threshold package、witness/provider public-contract / emitted-contract coupled-later package、witness/provider public-schema coupled-later package、theorem result-object route actual-adoption package、theorem final-public-contract reopen-threshold package、model-check checker-artifact route actual-adoption package、model-check final-public-contract reopen-threshold package、witness/provider route actual-adoption package、order-handoff source-wording route actual-adoption package、witness/provider schema-route actual-adoption package、witness/provider final-public-contract reopen-threshold package、order-handoff serial-scope reserve surface package、witness/provider emitted-contract representative trace-alignment bridge package、theorem actual Lean execution availability probe package、theorem toolchain probe / reopen manifest package、model-check public-seam compression package、theorem actual Lean execution narrow probe package、theorem actual Lean execution representative prototype widening package、final-layer closeout defaults / reopened queue package、Lean sample corpus and first foundations package は close 済みであり、corrected runnable version の current floorも reached である。remaining work は live queue nonzero の layered strong typing / IFC first-fragment、actual Lean execution hardening と later mixed gate / user-spec residual に残る。
- ただし accepted candidate を current executable / public surface へ直結させない。
