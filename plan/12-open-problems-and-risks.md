# plan/12 — 未解決問題とリスク

## 目的

この文書は、current repo の compact risk register である。
detail-side の研究手順は `plan/18`、重い将来線は `plan/13` に分ける。

## risk register

| 項目 | 種別 | current 状態 | リスク | current 対応 |
|---|---|---|---|---|
| fallback intuition drift | semantics / notation | 継続中 | outer-wrapper 直感が chain semantics を上書きする | explicit edge-row form と fixture explanation を維持 |
| final parser grammar の premature freeze | syntax | OPEN | syntax を早く決めすぎると semantics を拘束する | companion notation に留める |
| public surface hidden promotion | helper / public API | 継続中 | thin facade、shell、support-only、repo-local helper が混線する | `plan/09` の stable / support / excluded split を維持 |
| fixed-subset widening outruns docs | sample / validation | 継続中 | widening が progress/tasks/plan を追い越す | runner / ladder / regression / docs mirror を同 task で閉じる |
| cut-family conflation | semantics / theory | OPEN | `atomic_cut`、snapshot、barrier、durable cut を 1 primitive として誤読する | local finalization / ordering / observation / commit を別 family に保つ |
| low-level memory-order premature import | semantics / runtime policy | OPEN | C++ / ISA vocabulary が source surface を不必要に重くする | `memory_order` family は retained-later reference family に留める |
| syntax overcompression | syntax / UX | OPEN | packed row や hidden precedence が semantic honesty を壊す | syntax 比較軸を explicit 化する |
| thread/node false equality | distributed semantics | OPEN | lowering / transport / evidence / failure の差が消える | same causal language と lower-level asymmetry を分けて書く |
| modality underfitting | type / logic | OPEN | `lambda-circle-box` だけで place / scope / authority / witness を押し切ろうとする | partial basis と stronger candidate を分ける |
| layer-compatibility drift | type / proof architecture | OPEN | typed / theorem / model-check / order-handoff layer を足すごとに obligation export や erasure の compatibility floor が曖昧になる | layered theory stack と compatibility metatheory package を separate docs として維持する |
| typed / theorem / model-check overclaim | proof workflow | OPEN | full system を持たないのに solved に見せてしまう | boundary / pilot / stop line を package 単位で固定する |
| preview/route drift | proof workflow / helper | OPEN | helper-local preview と typed artifact support が別々に drift する | preview-alignment pre-floor を helper-local compare で固定し、public contract へは昇格させない |
| theory-lab no-falsification loop | process / research | OPEN | literature / prototype / counterexample が 1 本の長文に埋もれる | theory-lab operating model を維持する |
| queue drift / false closeout reading | snapshot / process | 継続中 | corrected prototype tranche close や actualization helper close を根拠に self-driven theory queue が消えたように読まれる | `specs/examples/458...520` と `docs/reports/0730...0801` を compare-floor / actual-adoption / actualization / residual-gate / final-layer-closeout anchor にし、narrow-but-nonzero reopen queue を actual close と drift から切り分けて snapshot へ戻す |
| shared-space confusion / replay drift | shared-space | OPEN | identity / admission / authority / replay / rejoin が混線する | room-profile と confusion/replay compact note を reserve で切る |
| host target premature fixation | integration | OPEN | visualizer / FFI / engine adapter の first target が早く固定される | docs-first boundary までは target-neutral に保つ |
| docs drift from long package chains | process | 継続中 | snapshot 文書に stale current-line が残る | `plan/` を detail memory に、`progress.md` / `tasks.md` を薄い snapshot に保つ |
| historical scope-collapse via monolithic reimport | snapshot / roadmap | OPEN | 旧資料の一体的な product vision を current-L2 mainline に戻すと separable architecture と near-end / later 境界が崩れる | owner split を `plan/10` / `plan/13` / `plan/18` に固定し、numbered queue は reopen しない |
| historical requirement amnesia | roadmap / repository memory | 継続中 | 旧資料で先回りしていた practical concern が current docs から消え、後段 reopen 時に同じ比較をやり直す | recovered historical concern owner map を維持し、mainline へは昇格させず heavy / risk / theory owner に残す |
| solver / projection / artifact-trace blow-up | proof workflow / tooling | OPEN | theorem / model-check / helper widening で solver cost、projection size、artifact trace volume が compare floor を越える | helper-local threshold と reserve-only line を維持し、public contract へ premature promotion しない |

## recovered historical concern owner map

| topic | owner doc | lane / macro | current band | reopen trigger |
|---|---|---|---|---|
| solver / projection / artifact-trace scaling | `plan/12` + `plan/18` | Lane B / `Macro 5` | risk register + later mixed gate | compare floor の latency / output size / maintainability が widening のボトルネックになったとき |
| proof promotion ladder / boundary memory | `plan/18` | Lane B / `Macro 5` | theory-side retained memory | theorem / model-check bridge を helper-local preview から先へ進めるとき |
| portal / multi-world / handoff stressor family | `plan/13` | Lane C / `Macro 6` | heavy future runtime stressor | authoritative-room first default を越えて、shared-space runtime と複数 world / portal の関係を current target に上げるとき |
| `fallback` / `lease` / `patch` / `gc_epoch` interaction inventory | `plan/12` + `plan/13` | Lane C / `Macro 6...7` | risk register + heavy future | runtime migration / finalization / revocation を repo-local helper より先へ進めるとき |
| operational trust / audit / registry / observability | `plan/13` | Lane C / `Macro 7` | heavy future | packaging / host-facing governance / audit chain を first-class target にするとき |
| benchmark family catalog | `plan/13` | `Macro 8` | heavy future / user-spec | first concrete application target と acceptance criteria を user と詰めるとき |

## current self-driven integration packages

### Package 0 — FAQ drift audit + queue reconstruction

- current status:
  close 済み。
- close reading:
  `faq_007` delta、`faq_006 -> faq_007` difference、queue drift、`p06/p07/p08` integration、current runnable evidence inventory を `docs/reports/0741` と snapshot rewrite に反映した。
- remaining risk:
  post-runnable reopen package reading を stale docs が再び上書きする drift は still 防ぐ必要がある。

### Package 1 — Problem 1 actual adoption package

- current status:
  close 済み。
- close reading:
  checker-adjacent principal、structural marker family first、theorem-first external integration target、notebook-first theorem line、row-local model-check carrier first、`p06` corrected prototype placementを `specs/examples/466` で current actual adoption package に上げた。
- remaining risk:
  stronger typed surface actual adoption、theorem discharge transport/public-contract、settled property language / concrete tool seam は still later に残る。

### Package 2 — Problem 2 actual adoption package

- current status:
  close 済み。
- close reading:
  cut family decomposition、relation decomposition principal、thread/node parity default wording、authoritative-room first default profile、low-level retained-later mapping、`p07/p08` corrected prototype placementを `specs/examples/467` で current actual adoption package に上げた。
- remaining risk:
  final source-surface handoff wording、final emitted-artifact/public-contract、stronger fairness / replay profile、exhaustive shared-space catalog は still later に残る。

### Package 3 — syntax / modality convergence

- current status:
  close 済み。
- close reading:
  semantic honesty principle、5 軸 comparison、`lambda_circle_box` partial basis、guarded / MDTT / MTT / Fitch-style multimodal keep を `specs/examples/468` で current recommendation に上げた。
- remaining risk:
  final grammar、final foundation adoption、final source marker、checker/theorem/runtime integration timingは still later に残る。

### Package 4 — near-end closeout

- current status:
  close 済み。
- close reading:
  `Documentation.md`、`tasks.md`、`progress.md`、`plan/` を `specs/examples/466...469` の actual adoption / residual mixed gate / user-spec residual に同期し、repo-local near-end success criteria と post-runnable reopen package reading を source-backed にした。
- remaining risk:
  actual adoption package close を theory solved や final public implementation complete と誤読する drift は still 防ぐ必要がある。

### Package 5 — theorem-first experimental pilot actualization

- current status:
  close 済み。
- close reading:
  notebook-first theorem line、symbolic `evidence_refs`、repo-local emitted artifact refs、compare-floor refs を helper-local theorem-first pilot manifest に束ね、`e5 / p05 / p06 / p07 / p08` representative corpus で machine-check できる current cut を `specs/examples/470` へ上げた。
- remaining risk:
  theorem public contract、actual discharge transport、proof object public schema、concrete theorem prover binding は still later に残る。

### Package 8 — order-handoff surface narrowing and stage-block secondary candidate

- current status:
  close 済み。
- close reading:
  explicit edge-row / vertical continuation を principal に維持しつつ、`stage` / `after` / `witness` family を strong secondary candidate、authoritative-room `serial` sugar を reserve に置く current cut を `specs/examples/473` へ上げた。
- remaining risk:
  final source wording、final emitted-artifact schema、`serial` sugar adoption、low-level exact source wording は still later に残る。

### Package 9 — theorem-prover experimental binding preflight

- current status:
  close 済み。
- close reading:
  notebook-first theorem lineと theorem-first pilot actualization を concrete brand や public theorem contract に上げず、brand-neutral theorem request preflight manifest と adapter-boundary refs まで helper-local に actualize する current cut を `specs/examples/474` へ上げた。
- remaining risk:
  concrete theorem prover brand、actual discharge transport、public theorem contract、proof object public schema は still later に残る。

### Package 10 — principal theory spine and Lean-first proof roadmap

- current status:
  close 済み。
- close reading:
  multimodal dependent core research direction、layered typing/proof architecture、compatibility metatheory package、Lean-first proof roadmap を `specs/examples/475` で current recommendation に上げた。これは final adopted calculus や final public proof contract ではなく、current actual-adoption / actualization floor を deeper theory spine で支える package と読む。
- remaining risk:
  final adopted calculus、exact compatibility manifest / export schema、public proof object contract、cross-tool artifact conformance、Rocq/Iris fallback trigger は still later に残る。

### Package 11 — auditable-authority-witness strengthening actualization

- current status:
  close 済み。
- close reading:
  `auditable_authority_witness` の minimal witness core を `specs/examples/476` で helper-local strengthening manifest に actualize し、claim / payload split を保ったまま `p07` reached、`p08 / p05` guard-only、`draw_result` symbolic binding ref cut を current recommendation に上げた。これは final public witness schema や provider receipt attachment ではなく、authoritative-room default profile の second strengthening package を narrow に閉じる package と読む。
- remaining risk:
  final public witness schema、final public provider receipt schema、delegated provider attestation、`delegated_rng_service` practical package、distributed fairness theorem、exhaustive shared-space catalog は still later に残る。

### Package 12 — delegated-rng-service practical actualization

- current status:
  close 済み。
- close reading:
  `delegated_rng_service` provider placement を `specs/examples/477` で helper-local practical manifest と narrow prototype `p09` に actualize し、authority-kept-commit split、optional provider attachment cut、`p09` reached / `p07,p08` guard-only、`opaque_authority_trust` first practical claim を current recommendation に上げた。これは final public provider receipt schema や delegated provider attestation public contract ではなく、provider placement を narrow practical package として閉じる package と読む。
- remaining risk:
  final public provider receipt schema、delegated provider attestation、`delegated_rng_service + auditable_authority_witness` combined public contract、distributed fairness theorem、control-plane separated carrier、exhaustive shared-space catalog は still later に残る。

### Package 13 — model-check second-line concretization

- current status:
  close 済み。
- close reading:
  model-check second line を `specs/examples/478` で helper-local second-line manifest に actualize し、row-local property preview、brand-neutral request preflight、public-checker second reserve split、`e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only を current recommendation に上げた。これは first settled property language、concrete model-check tool brand、actual public checker migrationではなく、model-check line を mixed gate 直前まで閉じる package と読む。
- remaining risk:
  first settled property language、concrete model-check tool brand / schema、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract は still later に残る。

### Package 14 — theorem discharge actual-format probe

- current status:
  close 済み。
- close reading:
  theorem discharge mixed gate を `specs/examples/479` で helper-local actual-format probe に actualize し、transport preview、public-contract preview、notebook-consumer-first boundary、`e5 / p06 / p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは actual discharge transport、public theorem contract、concrete theorem prover brand、proof object public schemaではなく、theorem line を final public contract 直前まで narrow に閉じる package と読む。
- remaining risk:
  actual discharge transport、public theorem contract、concrete theorem prover brand、proof object public schema、final public verifier contract は still later に残る。

### Package 15 — model-check property-language and tool-seam probe

- current status:
  close 済み。
- close reading:
  model-check mixed gate を `specs/examples/480` で helper-local property/tool-seam probe に actualize し、property-language probe、tool-seam probe、checker-boundary probe、`e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only を current recommendation に上げた。これは first settled property language、concrete tool brand / schema、actual public checker migration、production checker/runtime-policy contractではなく、model-check line を final public checker migration 直前まで narrow に閉じる package と読む。
- remaining risk:
  first settled property language、concrete model-check tool brand / schema、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract は still later に残る。

### Package 16 — theorem discharge / public-theorem-contract threshold default

- current status:
  close 済み。
- close reading:
  theorem discharge / public-theorem-contract threshold を `specs/examples/481` で helper-local threshold manifest に actualize し、review-unit first、discharge-entry adjacent、notebook-consumer first、brand-neutral theorem request、`e5 / p06 / p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは actual discharge transport、public theorem contract、theorem result public object、concrete theorem prover brand、proof object public schemaではなく、theorem line の threshold default を narrow に固定する package と読む。
- remaining risk:
  actual discharge transport、public theorem contract、theorem result public object、concrete theorem prover brand、proof object public schema、final public verifier contract は still later に残る。

### Package 17 — model-check property-language / tool-brand threshold default

- current status:
  close 済み。
- close reading:
  model-check property-language / tool-brand threshold を `specs/examples/482` で helper-local threshold manifest に actualize し、row-local property preview first、small-cluster semantic projection second、brand-neutral model-check request、public checker contract later、`e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only を current recommendation に上げた。これは first settled property language、concrete model-check tool brand、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract ではなく、model-check line の threshold default を narrow に固定する package と読む。
- remaining risk:
  first settled property language、concrete model-check tool brand / schema、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contract は still later に残る。

### Package 20 — theorem contract shape threshold default

- current status:
  close 済み。
- close reading:
  theorem contract shape threshold を `specs/examples/485` で helper-local threshold manifest に actualize し、refs-only reserve schema first、review-unit transport anchor、brand-neutral request manifest keep、consumer-shaped theorem payload later、`e5 / p06 / p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは actual discharge transport、public theorem contract、theorem result public object、consumer-shaped theorem payload、concrete theorem prover brand、proof object public schemaではなく、theorem line の shape threshold default を narrow に固定する package と読む。
- remaining risk:
  actual discharge transport、public theorem contract、theorem result public object、consumer-shaped theorem payload、concrete theorem prover brand、proof object public schema、final public verifier contract は still later に残る。

### Package 21 — theorem transport/public-contract coupled later gate

- current status:
  close 済み。
- close reading:
  theorem transport/public-contract coupled later gate を `specs/examples/486` で helper-local actualization manifest に actualize し、transport/public-contract adjacent but distinct、review-unit anchor、refs-only reserve schema first、consumer-shaped payload later、`e5 / p06 / p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは actual discharge transport adoption、public theorem contract adoption、theorem result public object、consumer-shaped theorem payload、concrete theorem prover brand、proof object public schemaではなく、theorem line の actual adoption 直前 gate を narrow に固定する package と読む。
- remaining risk:
  actual discharge transport adoption、public theorem contract adoption、theorem result public object、consumer-shaped theorem payload、concrete theorem prover brand、proof object public schema、final public verifier contract は still later に残る。

### Package 22 — theorem review-unit transport / notebook-contract actual adoption

- current status:
  close 済み。
- close reading:
  theorem review-unit transport / notebook-contract actual adoption を `specs/examples/487` で helper-local actual adoption manifest に actualize し、review-unit transport first、notebook-consumer contract first、brand-neutral binding reserve keep、`e5 / p06 / p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは theorem result public object、consumer-shaped theorem payload、concrete theorem prover brand、proof object public schema、final public verifier contractではなく、theorem line の current repo-local actual adoption package を narrow に固定する package と読む。
- remaining risk:
  theorem result public object、consumer-shaped theorem payload、concrete theorem prover brand、proof object public schema、final public verifier contract は still later に残る。

### Package 26 — theorem result-object preview / proof-object-schema reserve actualization

- current status:
  close 済み。
- close reading:
  theorem result-object preview / proof-object-schema reserve actualization を `specs/examples/491` で helper-local actualization manifest に actualize し、notebook-consumer object first、consumer-shaped payload preview only、proof-object-schema reserve keep、brand-neutral binding reserve keep、`e5 / p06 / p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは final public theorem result object、consumer-shaped theorem payload public contract、concrete theorem prover brand、proof object public schema、final public verifier contractではなく、theorem line の next mixed gate を repo-local preview/reserve cut まで narrow に固定する package と読む。
- remaining risk:
  final public theorem result object、consumer-shaped theorem payload public contract、concrete theorem prover brand、proof object public schema、final public verifier contract は still later に残る。

### Package 29 — theorem proof-object schema / prover-brand coupled later gate

- 位置づけ:
  theorem later mixed gate
- current recommendation:
  proof-object schema side と prover-brand side を adjacent but distinct later gate として helper-local manifest に actualize し、result-object preview keep、refs-only public-schema candidate only、brand-neutral preflight anchor keep、concrete brand not adopted を current recommendation に上げる。
- close evidence:
  `docs/reports/0772`
  `specs/examples/494`
- note:
  theorem proof-object schema / prover-brand coupled later gate を `specs/examples/494` で helper-local coupled-later manifest に actualize し、`e5 / p06 / p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは final public theorem result object、consumer-shaped theorem payload public contract、concrete theorem prover brand adoption、proof object public schema adoption、final public verifier contractではなく、theorem line の next mixed gate を repo-local coupled-later cut まで narrow に固定する package と読む。

### Package 30 — model-check tool-brand / verifier-handoff coupled later gate

- 位置づけ:
  model-check later mixed gate
- current recommendation:
  tool-brand side と verifier-handoff side を adjacent but distinct later gate として helper-local manifest に actualize し、public-checker artifact preview keep、verifier-handoff candidate only、tool-brand candidate only を current recommendation に上げる。
- close evidence:
  `docs/reports/0773`
  `specs/examples/495`
- note:
  model-check tool-brand / verifier-handoff coupled later gate を `specs/examples/495` で helper-local coupled-later manifest に actualize し、`e5 / p06 / p07 / p09` reached、`p05` guard-only を current recommendation に上げた。これは first settled property language、concrete model-check tool brand adoption、final public checker artifact adoption、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contractではなく、model-check line の next mixed gate を repo-local coupled-later cut まで narrow に固定する package と読む。

### Package 31 — order-handoff source wording / emitted-artifact coupled later gate

- 位置づけ:
  order-handoff later mixed gate
- current recommendation:
  source-wording side と emitted-artifact-schema side を adjacent but distinct later gate として helper-local manifest に actualize し、edge-row principal、stage-block secondary keep、thread/node same causal language keep、repo-local emitted artifact refs first を current recommendation に上げる。
- close evidence:
  `docs/reports/0774`
  `specs/examples/496`
- note:
  order-handoff source wording / emitted-artifact coupled later gate を `specs/examples/496` で helper-local coupled-later manifest に actualize し、`p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは final parser grammar、final public parser/checker/runtime API、final source-surface handoff wording、final emitted-artifact schema、final emitted-handoff contract、final public witness/provider/artifact contract、authoritative-room `serial` sugar adoption、low-level exact source wording、final modal foundation adoption、exhaustive shared-space catalogではなく、order-handoff line の next mixed gate を repo-local coupled-later cut まで narrow に固定する package と読む。

### Package 32 — theorem result object / payload public-contract coupled later gate

- 位置づけ:
  theorem later mixed gate
- current recommendation:
  final result-object side と consumer-shaped payload public-contract side を adjacent but distinct later gate として helper-local manifest に actualize し、notebook-consumer object first、consumer-shaped payload candidate only、proof-object-schema/prover-brand adjacent keep を current recommendation に上げる。
- close evidence:
  `docs/reports/0775`
  `specs/examples/497`
- note:
  theorem result object / payload public-contract coupled later gate を `specs/examples/497` で helper-local coupled-later manifest に actualize し、`e5 / p06 / p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは final public theorem result object、consumer-shaped theorem payload public contract、concrete theorem prover brand、proof object public schema、final public verifier contractではなく、theorem line の next mixed gate を repo-local coupled-later cut まで narrow に固定する package と読む。

### Package 33 — model-check public checker artifact / migration coupled later gate

- 位置づけ:
  model-check later mixed gate
- current recommendation:
  final public checker artifact side と actual public checker migration side を adjacent but distinct later gate として helper-local manifest に actualize し、consumer-shaped artifact candidate only、actual public checker migration candidate only、tool-brand / verifier-handoff adjacent keep を current recommendation に上げる。
- close evidence:
  `docs/reports/0776`
  `specs/examples/498`
- note:
  model-check public checker artifact / migration coupled later gate を `specs/examples/498` で helper-local coupled-later manifest に actualize し、`e5 / p06 / p07 / p09` reached、`p05` guard-only を current recommendation に上げた。これは first settled property language、concrete model-check tool brand、final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contractではなく、model-check line の next mixed gate を repo-local coupled-later cut まで narrow に固定する package と読む。

### Package 34 — witness/provider public-schema coupled later gate

- 位置づけ:
  shared-space later mixed gate
- current recommendation:
  witness-schema side と provider-receipt side と combined public-contract side を adjacent but distinct later gate として helper-local manifest に actualize し、claim/payload split first、witness/provider route non-collapse、repo-local emitted artifact refs first、public-schema candidate only を current recommendation に上げる。
- close evidence:
  `docs/reports/0777`
  `specs/examples/499`
- note:
  witness/provider public-schema coupled later gate を `specs/examples/499` で helper-local coupled-later manifest に actualize し、`p07 / p08 / p09` reached、`p05` guard-only を current recommendation に上げた。これは final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted-handoff contract、final source-surface handoff wording、exhaustive shared-space catalogではなく、shared-space line の next mixed gate を repo-local coupled-later cut まで narrow に固定する package と読む。

### Package 35 — theorem result-object route actual adoption

- 位置づけ:
  theorem later mixed gate
- current recommendation:
  review-unit transport first と notebook-consumer object first を保ったまま、result-object route first と consumer-shaped payload preview keep と proof-object-schema-prover-brand later を helper-local actual-adoption manifest に actualize し、theorem line の current recommendation に上げる。
- close evidence:
  `docs/reports/0778`
  `specs/examples/500`
- note:
  theorem result-object route actual adoption を `specs/examples/500` で helper-local actual-adoption manifest に actualize し、`e5 / p06 / p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは final public theorem result object、consumer-shaped theorem payload public contract、concrete theorem prover brand、proof object public schema、final public verifier contractではなく、theorem line の next mixed gate を repo-local actual-adoption cut まで narrow に固定する package と読む。

### Package 36 — model-check checker-artifact route actual adoption

- 位置づけ:
  model-check later mixed gate
- current recommendation:
  row-local property route first と checker-boundary contract anchor を保ったまま、checker-artifact route first と migration candidate adjacent keep を helper-local actual-adoption manifest に actualize し、model-check line の current recommendation に上げる。
- close evidence:
  `docs/reports/0779`
  `specs/examples/501`
- note:
  model-check checker-artifact route actual adoption を `specs/examples/501` で helper-local actual-adoption manifest に actualize し、`e5 / p06 / p07 / p09` reached、`p05` guard-only を current recommendation に上げた。これは first settled property language、concrete model-check tool brand、final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contractではなく、model-check line の next mixed gate を repo-local actual-adoption cut まで narrow に固定する package と読む。

### Package 37 — witness/provider route actual adoption

- 位置づけ:
  shared-space later mixed gate
- current recommendation:
  claim/payload split first と witness/provider route non-collapse と repo-local emitted artifact refs first を保ったまま、witness/provider route first と public-schema candidate keep と combined public-contract later を helper-local actual-adoption manifest に actualize し、shared-space line の current recommendation に上げる。
- close evidence:
  `docs/reports/0780`
  `specs/examples/502`
- note:
  witness/provider route actual adoption を `specs/examples/502` で helper-local actual-adoption manifest に actualize し、`p07 / p08 / p09` reached、`p05` guard-only を current recommendation に上げた。これは final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted-handoff contract、exhaustive shared-space catalogではなく、shared-space line の next mixed gate を repo-local actual-adoption cut まで narrow に固定する package と読む。

### Package 38 — order-handoff source wording route actual adoption

- 位置づけ:
  order-handoff later mixed gate
- current recommendation:
  edge-row / vertical continuation principal と readable high-level relation vocabulary keep と stage-block secondary keep と thread/node same causal language keep を保ったまま、principal source wording route first と emitted-artifact schema candidate keep を helper-local actual-adoption manifest に actualize し、order-handoff line の current recommendation に上げる。
- close evidence:
  `docs/reports/0781`
  `specs/examples/503`
- note:
  order-handoff source wording route actual adoption を `specs/examples/503` で helper-local actual-adoption manifest に actualize し、`p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは final parser grammar、final public parser/checker/runtime API、final source-surface handoff wording、final emitted-artifact schema、final emitted-handoff contract、final public witness/provider contract、authoritative-room `serial` sugar adoption、low-level `memory_order` exact source surface、final modal foundation adoption、exhaustive shared-space catalogではなく、order-handoff line の next mixed gate を repo-local actual-adoption cut まで narrow に固定する package と読む。

### Package 39 — witness/provider schema route actual adoption

- 位置づけ:
  shared-space later mixed gate
- current recommendation:
  claim/payload split first と repo-local emitted artifact refs first を保ったまま、witness-schema candidate keep + witness route first と provider-receipt candidate keep + provider route first と combined public-contract candidate keep を helper-local actual-adoption manifest に actualize し、shared-space line の current recommendation に上げる。
- close evidence:
  `docs/reports/0782`
  `specs/examples/504`
- note:
  witness/provider schema route actual adoption を `specs/examples/504` で helper-local actual-adoption manifest に actualize し、`p07 / p08 / p09` reached、`p05` guard-only を current recommendation に上げた。これは final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted-handoff contract、exhaustive shared-space catalogではなく、shared-space line の next mixed gate を repo-local actual-adoption cut まで narrow に固定する package と読む。

### Package 40 — witness/provider final public-contract reopen threshold

- 位置づけ:
  shared-space later mixed gate
- current recommendation:
  final public witness schema / final public provider receipt schema を first reopen pair、delegated provider attestation / combined provider+witness public contract を second reopen pair、final emitted-handoff contract を third reopen として helper-local threshold manifest に actualize し、shared-space line の current recommendation に上げる。
- close evidence:
  `docs/reports/0783`
  `specs/examples/505`
- note:
  witness/provider final public-contract reopen threshold を `specs/examples/505` で helper-local threshold manifest に actualize し、`p07 / p08 / p09` reached、`p05` guard-only を current recommendation に上げた。これは final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted-handoff contract、exhaustive shared-space catalogではなく、shared-space line の remaining mixed gate を ordered reopen threshold まで narrow に固定する package と読む。

### Package 41 — theorem final public-contract reopen threshold

- 位置づけ:
  theorem later mixed gate
- current recommendation:
  final public theorem result object / consumer-shaped theorem payload public contract を first reopen pair、concrete theorem prover brand / proof object public schema を second reopen pair、final public verifier contract を third reopen として helper-local threshold manifest に actualize し、theorem line の current recommendation に上げる。
- close evidence:
  `docs/reports/0784`
  `specs/examples/506`
- note:
  theorem final public-contract reopen threshold を `specs/examples/506` で helper-local threshold manifest に actualize し、`e5 / p06 / p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは final public theorem result object、consumer-shaped theorem payload public contract、concrete theorem prover brand、proof object public schema、final public verifier contract そのものではなく、theorem line の remaining mixed gate を ordered reopen threshold まで narrow に固定する package と読む。

### Package 42 — model-check final public-contract reopen threshold

- 位置づけ:
  model-check later mixed gate
- current recommendation:
  first settled property language / concrete model-check tool brand を first reopen pair、final public checker artifact / actual public checker migration を second reopen pair、actual emitted verifier handoff artifact / production checker-runtime-policy contract を third reopen pair、final public verifier contract を fourth reopen として helper-local threshold manifest に actualize し、model-check line の current recommendation に上げる。
- close evidence:
  `docs/reports/0785`
  `specs/examples/507`
- note:
  model-check final public-contract reopen threshold を `specs/examples/507` で helper-local threshold manifest に actualize し、`e5 / p06 / p07 / p09` reached、`p05` guard-only を current recommendation に上げた。これは first settled property language、concrete model-check tool brand、final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contract そのものではなく、model-check line の remaining mixed gate を ordered reopen threshold まで narrow に固定する package と読む。

### Package 43 — theorem Lean-first non-production stub pilot actualization

- 位置づけ:
  theorem later mixed gate / external integration actualization
- current recommendation:
  theorem-first external integration target を brand-neutral preflight だけに止めず、review-unit first / brand-neutral preflight anchor keep / Lean-first non-production stub only / repo-local emitted stub refs first を helper-local actualization manifest に actualize し、theorem line の current recommendation に上げる。
- close evidence:
  `docs/reports/0786`
  `specs/examples/508`
- note:
  theorem Lean-first non-production stub pilot actualization を `specs/examples/508` で helper-local actualization manifest に actualize し、`e5 / p06 / p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは actual Lean tool execution、actual discharge transport、public theorem contract、proof object public schema、final public verifier contract そのものではなく、theorem-first external integration line の concrete brand pressure を repo-local emitted stub pilot まで narrow に固定する package と読む。

### Package 44 — theorem review-unit to Lean-stub repo-local artifact-conformance bridge

- 位置づけ:
  theorem later mixed gate / external integration compare-floor actualization
- current recommendation:
  theorem Lean-first non-production stub pilot の次段として、runtime/static representative source sample `e2 / e5` から formal hook / review unit / Lean stub artifact の pair alignment を repo-local helper と regression bundle に actualize し、artifact-conformance second stage を current recommendation に上げる。
- close evidence:
  `docs/reports/0787`
  `specs/examples/509`
- note:
  theorem review-unit to Lean-stub repo-local artifact-conformance bridge を `specs/examples/509` で helper-local actualization manifest に actualize し、runtime `e2` / static `e5` representative sample と regression 22-command bundle を current recommendation に上げた。これは actual Lean tool execution、prototype-wide trace alignment、public theorem contract、proof object public schema、cross-tool public artifact-conformance contract、final public verifier contract そのものではなく、theorem-first external integration line の second-stage conformance floor を repo-local bridge まで narrow に固定する package と読む。

### Package 45 — theorem Lean-stub representative trace-alignment bridge

- 位置づけ:
  theorem later mixed gate / representative prototype bridge actualization
- current recommendation:
  theorem repo-local artifact-conformance bridge の次段として、representative runtime/static/prototype corpus `e2 / e5 / p06 / p07 / p08` と guard-only `p05` に対する review-unit / Lean stub pair alignment を helper-local runtime test に actualize し、theorem line の current representative bridge に上げる。
- close evidence:
  `docs/reports/0788`
  `specs/examples/510`
- note:
  theorem Lean-stub representative trace-alignment bridge を `specs/examples/510` で helper-local actualization manifest に actualize し、`e2 / e5 / p06 / p07 / p08` reached と `p05` guard-only を current recommendation に上げた。これは actual Lean tool execution、prototype-wide exhaustive alignment、public theorem contract、proof object public schema、cross-tool public artifact-conformance contract、final public verifier contract そのものではなく、theorem second-stage bridge を representative prototype corpus まで narrow に固定する package と読む。

### Package 27 — model-check public-checker artifact preview / verifier-handoff reserve actualization

- current status:
  close 済み。
- close reading:
  model-check public-checker artifact preview / verifier-handoff reserve actualization を `specs/examples/492` で helper-local actualization manifest に actualize し、consumer-shaped artifact preview only、verifier-handoff reserve keep、brand-neutral tool-binding reserve keep、`e5 / p06 / p07 / p09` reached、`p05` guard-only を current recommendation に上げた。これは first settled property language、concrete model-check tool brand、final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contractではなく、model-check line の next mixed gate を repo-local preview/reserve cut まで narrow に固定する package と読む。
- remaining risk:
  first settled property language、concrete model-check tool brand、final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contract は still later に残る。

### Package 28 — witness/provider public-contract / emitted-handoff coupled later gate

- 位置づけ:
  shared-space / order-handoff later mixed gate
- current recommendation:
  witness/provider public-contract side と emitted-handoff contract side を adjacent but distinct later gate として helper-local manifest に actualize し、claim/payload split first、witness/provider route non-collapse、repo-local emitted artifact refs first、combined public contract later、final emitted-handoff contract later を current recommendation に上げる。
- close evidence:
  `docs/reports/0771`
  `specs/examples/493`
- note:
  witness/provider public-contract / emitted-handoff coupled later gate を `specs/examples/493` で helper-local coupled-later manifest に actualize し、`p07 / p08 / p09` reached、`p05` guard-only を current recommendation に上げた。これは final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted-handoff contract、final source-surface handoff wording、exhaustive shared-space catalogではなく、shared-space / order-handoff line の next mixed gate を repo-local coupled-later cut まで narrow に固定する package と読む。

### Package 23 — model-check row-local property / checker-boundary actual adoption

- current status:
  close 済み。
- close reading:
  model-check row-local property / checker-boundary actual adoption を `specs/examples/488` で helper-local actual adoption manifest に actualize し、row-local property route first、checker-boundary contract first、brand-neutral tool-binding reserve keep、`e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only を current recommendation に上げた。これは first settled property language、concrete model-check tool brand、consumer-shaped public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contractではなく、model-check line の current repo-local actual adoption package を narrow に固定する package と読む。
- remaining risk:
  first settled property language、concrete model-check tool brand、consumer-shaped public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contract は still later に残る。

### Package 24 — witness/provider/artifact public-shape actual adoption

- current status:
  close 済み。
- close reading:
  witness/provider/artifact public-shape actual adoption を `specs/examples/489` で helper-local actual adoption manifest に actualize し、claim/payload split first、witness route / provider route non-collapse、repo-local emitted artifact refs first、`p07 / p08 / p09` reached、`p05` guard-only を current recommendation に上げた。これは final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted handoff contract、exhaustive shared-space catalogではなく、shared-space reserve public-shape line の current repo-local actual adoption package を narrow に固定する package と読む。
- remaining risk:
  final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted handoff contract、exhaustive shared-space catalog は still later に残る。

### Package 25 — order-handoff surface actual adoption

- current status:
  close 済み。
- close reading:
  order-handoff surface actual adoption を `specs/examples/490` で helper-local actual adoption manifest に actualize し、edge-row / vertical continuation principal、readable relation vocabulary、stage-block secondary keep、repo-local emitted artifact refs first、`p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは final parser grammar、final source-surface handoff wording、final emitted handoff contract、final public witness/provider/artifact contract、authoritative-room `serial` sugar adoption、low-level exact source wording、final modal foundation adoptionではなく、order-handoff line の current repo-local actual adoption package を narrow に固定する package と読む。
- remaining risk:
  final parser grammar、final source-surface handoff wording、final emitted handoff contract、final public witness/provider/artifact contract、authoritative-room `serial` sugar adoption、low-level exact source wording、final modal foundation adoption は still later に残る。

### Package 18 — witness/provider/artifact public-shape threshold default

- current status:
  close 済み。
- close reading:
  witness/provider/artifact public-shape threshold を `specs/examples/483` で helper-local threshold manifest に actualize し、claim/payload split first、repo-local emitted artifact refs first、optional attachment refs only、combined public contract later、`p07 / p08 / p09` reached、`p05` guard-only を current recommendation に上げた。これは final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted handoff contract ではなく、shared-space reserve public-shape line の threshold default を narrow に固定する package と読む。
- remaining risk:
  final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted handoff contract、exhaustive shared-space catalog は still later に残る。

### Package 19 — order-handoff surface / artifact threshold default

- current status:
  close 済み。
- close reading:
  order-handoff surface / artifact threshold を `specs/examples/484` で helper-local threshold manifest に actualize し、edge-row vertical continuation principal、readable relation vocabulary、stage-block secondary candidate、repo-local emitted artifact refs first、`p07 / p08` reached、`p05` guard-only を current recommendation に上げた。これは final source-surface handoff wording、final emitted handoff contract、final public witness schema、authoritative-room `serial` sugar adoption、low-level exact source wording ではなく、order-handoff later mixed gate の threshold default を narrow に固定する package と読む。
- remaining risk:
  final source-surface handoff wording、final emitted handoff contract、final public witness/provider/artifact contract、authoritative-room `serial` sugar adoption、low-level exact source wording、final modal foundation adoption は still later に残る。

### Package 6 — authoritative-room vertical-slice emitted-artifact ratchet

- current status:
  close 済み。
- close reading:
  `p07 / p08` current default room profile を helper-local vertical-slice manifest に束ね、relation refs、authority-handoff refs、runtime evidence refs、repo-local emitted artifact refs を final emitted schema に上げずに machine-check できる current cut を `specs/examples/471` へ上げた。
- remaining risk:
  final emitted-artifact/public-contract、final public witness schema / provider attachment、`delegated_rng_service` practical package、distributed fairness theorem、exhaustive shared-space catalog は still later に残る。

### mixed-gate pre-floor — verifier preview alignment

- current status:
  close 済み。
- close reading:
  helper-local `verification_preview` / `artifact_preview` を final public contract に昇格させず、sample-local preview-aligned typed artifact route を compare floor に置く current cut が fixed 済みである。
- remaining risk:
  theorem discharge transport / public-contract、settled property language / tool seam 自体は still later に残る。

### mixed-gate pre-floor — model-check projection helper compare

- current status:
  close 済み。
- close reading:
  helper-local projection pre-floor により、row-local `model_check_concrete_carriers`、small-cluster projection reserve、property-language seam、tool-binding seam を distinct な compare floor として representative runtime/static/guarded/prototype corpus で machine-check できる current cut が fixed 済みである。
- remaining risk:
  settled property language adoption、concrete tool seam adoption、production checker/runtime-policy contract 自体は still later に残る。

### mixed-gate pre-floor — theorem discharge helper compare

- current status:
  close 済み。
- close reading:
  helper-local theorem discharge pre-floor により、row-local `proof_notebook_review_unit`、abstract discharge-entry reserve、transport seam、public-contract seam を distinct な compare floor として representative runtime/static/guarded/prototype corpus で machine-check できる current cut が fixed 済みである。
- remaining risk:
  actual discharge transport adoption、public theorem contract adoption、concrete theorem prover binding、proof object public schema 自体は still later に残る。

## current mixed-gate open problems

### 1. stronger typed-surface actual adoption

- current reading:
  threshold framing は fixed 済みである。
  **open problem は、stronger typed surface を actual adoption 判定へ送る concrete mixed gate をどこに置くか** である。

### 2. theorem discharge result / public-contract concretization

- current reading:
  notebook-first threshold、admissible evidence contraction、abstract discharge-entry reserve は fixed 済みである。
  preview-alignment pre-floor も fixed 済みであり、helper-local preview と sample-local typed artifact route の compare floor はある。
  theorem discharge pre-floor も fixed 済みであり、row-local review unit / discharge-entry reserve / transport seam / public-contract seam を distinct な helper-local compare floor として representative corpus で machine-check できる。
  theorem discharge actual-format probe も fixed 済みであり、transport preview / public-contract preview / notebook-consumer-first boundary を helper-local actualization floor として representative corpus で machine-check できる。
  theorem discharge / public-theorem-contract threshold default も fixed 済みであり、review-unit first / discharge-entry adjacent / notebook-consumer-first / brand-neutral theorem request を helper-local threshold floor として representative corpus で machine-check できる。
  **open problem は、actual discharge transport / theorem result / public contract をどの mixed gate で actual adoption に送るか** である。

### 3. model-check settled property language / concrete tool seam

- current reading:
  row-local carrier、small-cluster projection keep/drop、property-language/tool-binding later gate は fixed 済みである。
  preview-alignment pre-floor も fixed 済みであり、runtime/static/guarded/prototype corpus を same compare floor で見られる。
  model-check projection pre-floor も fixed 済みであり、row-local carrier / small-cluster projection / property-language seam / tool-binding seam を distinct な helper-local compare floor として representative corpus で machine-check できる。
  model-check second-line concretization と property/tool-seam probe と threshold default も fixed 済みであり、row-local property preview / brand-neutral request preflight / public-checker second reserve split / property-language probe / tool-seam probe / checker-boundary probe / row-local-property-preview-first threshold / small-cluster-projection-second threshold を helper-local actualization floor として representative corpus で machine-check できる。
  **open problem は、first settled property language と concrete tool seam をどの mixed gate で concretize するか** である。

### 4. stronger modal foundation actual adoption

- current reading:
  `lambda_circle_box` partial basis、guarded / MDTT / MTT / Fitch-style multimodal keep、boundary-pressure trigger は fixed 済みである。
  **open problem は、final foundation adoption をどの mixed gate で判断するか** である。

### 5. shared-space final fairness / replay operational profile

- current reading:
  fairness / replay mixed-gate boundary は fixed 済みである。
  **open problem は、final operational catalog と fairness / replay operational profile をどこで mixed gate から user-spec-required gate へ送るか** である。

### 6. public operational CLI installed packaging / success-criteria

- current reading:
  installed-binary / packaging success-criteria mixed-gate boundary は fixed 済みである。
  **open problem は、installed-binary promotion と backend / tooling success criteria をどこで mixed gate concretization として扱うか** である。

## current recommendation

- live な first-line integration package はもう不要であり、`M1/M2/M3` threshold package、theorem contract shape threshold、theorem coupled later gate、theorem review-unit actual adoption、theorem public-seam compression、model-check public-seam compression、order-handoff-witness-provider final public seam compression、representative Lean sample set actual Lean execution も close 済みである。`specs/examples/520`、`521`、`522`、`523`、`524` により、current remaining self-driven work は actual Lean execution helper/CLI hardening と later mixed gate tracking に整理され、Lean formal skeleton / proof obligations first slice、IFC secret valid/invalid concrete example、source-side authority pair、source-side label-flow negative は committed `samples/lean/` corpus と source-side prototype corpus として fixed 済みである。remaining work 全体は later mixed gate / user-spec residual に分かれる。
- principal theory spine / proof roadmap は current recommendation に上がったが、final calculus adoption や production proof ecosystem finalization は `plan/13` 側に残す。
- heavy actual adoption beyond current defaults、concrete tool binding、final public contract は `plan/13` 側に残す。
- reserve integration lane の room-default strengthening / packaging reserve は theory-lab line と並走してよい。
- ただし、どれも current executable lane を置き換えない。
