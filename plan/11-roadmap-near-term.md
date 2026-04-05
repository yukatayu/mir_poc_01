# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 task 先までの near-term roadmap を示す。
ここに書く step 数や task 数は厳密な約束ではなく、**rough estimate** である。

## いまから数 task の主眼

近い数 task の目的は、current L2 を次の状態へ持っていくことである。

- parser-free PoC を継続的に回せる
- parser-free PoC の実行結果を process 内比較だけに閉じ込めない
- trace / audit と host coverage の operational boundary を semantics から独立に狭く切る
- notation の比較結果が docs / tests / fixtures と整合している
- helper stack の mirror drift が抑えられている
- parser 導入前に何が未決かを誤魔化さずに進められる

## 直近 2〜4 task の候補

### 候補 1. detached trace / audit serialization の最小境界整理

- docs-only minimal schema は切れたので、次は `TraceAuditSink` / `RunReport` / bundle summary から thin export boundary をどう置くかを narrow に棚卸しする
- repo 外保存・再比較・後解析に必要な field を、exact-compare core / detached non-core / human-facing explanation に分けて運ぶ
- `must_explain` は引き続き prose obligation に残し、exact compare の core を増やしすぎない
- その比較では、payload core は `RunReport` に寄せつつ、first exporter entry は `run_bundle` / `BundleRunReport` に置くのが current understanding である
- さらに bundle-first artifact の内部では、`fixture_id` / `fixture_path` / `host_plan_path` / `runtime_requirement` を `bundle_context` へ寄せ、`host_plan_coverage_failure` は aggregate-only に残すのが次の narrow step である
- その次の narrow comparison として、`host_plan_coverage_failure` を将来 typed carrier に昇格させるなら bundle failure artifact 側へ切るのが自然かどうかを確認する
- さらにその docs-only refinement として、bundle failure artifact 側へ切る typed carrier の最小 schema は `failure_kind` discriminator だけに留め、`bundle_context` や short note を typed core に混ぜないのが current understanding である
- さらにその aggregate connection として、`BatchRunSummary` が typed bundle failure を吸うなら、持たせる typed 集約は `failure_kind` ごとの histogram / kind count までに留め、bundle failure summary の薄い再掲は避けるのが current understanding である
- さらにその naming / migration refinement として、aggregate 側の最小 field 名候補は `bundle_failure_kind_counts` であり、current `host_plan_coverage_failures` list と `BatchBundleOutcome::Failed.host_plan_coverage_failure` bool を compatibility anchor として残した additive coexistence から始めるのが current understanding である
- さらに detached exporter consolidation sprint の入口として、
  - detached exporter chain の current docs-only judgment を `specs/examples/23-current-l2-detached-export-loop-consolidation.md` に集約し、
  - bundle-first の non-production tiny exporter / emitter、
  - payload core comparison に絞った minimal diff helper、
  - fixture authoring / elaboration template
  を整備して、PoC loop を「保存し、比較し、また 1 本足して回す」入口まで近づける
- その次の continuation として、
  - aggregate export 接続と artifact 保存先 / path policy の最小 cut を `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md` に整理し、
  - `target/current-l2-detached/` を current non-production default candidate として bundle artifact を保存し、
  - tiny wrapper で 1 fixture export / 2 artifact compare を回す
  ところまでを near-term の operational boundary とみなす

### 候補 2. richer host interface と coverage analysis の入口整理

- current host harness から production host interface へ直進しない
- preflight、coverage explanation、uncovered call detection のどこが本当に bottleneck かを切り分ける
- detached artifact boundary を先に切った後でも十分な部分だけを後段 task に残す

### 候補 3. parser-free representative coverage の拡張

- current L2 semantics の重要点を、まだ fixture 化されていない narrow case で regression 化する
- static-only と runtime fixture のバランスを見る
- `must_explain` は prose に残し、machine-check 範囲を増やしすぎない
- current actualization としては、`e21-try-atomic-cut-frontier` により `TryFallback` body 内 `AtomicCut` rollback frontier 更新を、`e22-try-atomic-cut-place-mismatch` により nested place mismatch 時の event-only cut を、それぞれ runtime fixture / detached loop / directory summary まで通せる状態になった
- current docs-only judgment としては、`try` / rollback locality の structural floor は first checker cut 候補 cluster に残すが、existing reason-row family helper の fourth spike にはまだ actualize せず、runtime representative と `specs/examples/51` / `52` に留める
- current docs-only refinement としては、`try` / rollback locality を dedicated AST structural helper に actualize するなら、parser/loader malformed source、AST-only floor、reason-row family と分ける dedicated carrier、runtime gate 非依存、singleton ではない structural family という entry criteria を先に満たすべきである
- current docs-only refinement としては、current parser-free phase では structural malformed source を parser でも loader でもなく static gate / dedicated AST structural helper 側へ置き、loader は carrier/schema malformed に留めるのが最小である
- current docs-only refinement としては、runtime representative `E2` / `E21` / `E22` が current phase の evidence として十分強く、malformed static family はまだ actual corpus に増やさないのが自然である
- current docs-only refinement としては、future dedicated AST structural helper を切る場合の compare contract は helper-local dedicated contract から始め、row-family 流用や detached artifact shared carrier 先行は避けるのが自然である
- current docs-only refinement としては、future dedicated AST structural helper の optional expected field 名は `expected_static.checked_try_rollback_structural_findings` が最小候補であり、focused compare shape も `subject_kind` / `finding_kind` の helper-local row list に留めるのが自然である
- current docs-only refinement としては、future dedicated AST structural helper を detached validation loop へ差し込むなら、bundle-first runtime path ではなく static gate artifact loop の helper-local smoke family に留めるのが自然である
- current docs-only refinement としては、future dedicated AST structural helper の structural verdict は `expected_static.verdict` を流用せず、`expected_static.checked_try_rollback_structural_verdict` と helper-local string enum `no_findings` / `findings_present` に留めるのが自然である
- current docs-only refinement としては、future dedicated AST structural helper を detached artifact shared carrier へ上げる閾値は、helper actualization、fixture-side field actualization、static corpus、loop stabilization、saved artifact compare need の 5 条件が揃った時点に置くのが最小であり、current state ではまだ未充足である
- current docs-only refinement としては、future dedicated AST structural helper の wrapper family は family-specific に留め、exact subcommand 名は actual helper actualization task まで deferred にするのが自然である
- current docs-only refinement としては、future dedicated AST structural helper を generic structural checker family と合流させるのは later public checker API comparison と同時に扱うのが自然である
- current docs-only refinement としては、later public checker API comparison に future dedicated AST structural helper family を載せるには、generic family 合流とは別に、actual helper / fixture contract / corpus / loop stabilization / public comparison pressure の entry criteria が要る
- current docs-only refinement としては、future dedicated AST structural helper の malformed static family は current phase の今すぐではなく、dedicated helper actualization first tranche と同時に actual corpus へ足すのが自然である
- current docs-only refinement としては、future dedicated AST structural helper の first tranche は helper code / fixture-side fields / minimal malformed static family / static gate smoke path を一体で切り、shared carrier / public checker API は外に残すのが自然である
- current docs-only refinement としては、future dedicated AST structural helper の minimal malformed static family tranche は `TryFallback` 1 件 + `AtomicCut` 1 件の two-fixture pair を最小とするのが自然である
- current actualization としては、その two-fixture first tranche の slot selection を `empty fallback_body -> missing_fallback_body` / `fallback_body AtomicCut -> disallowed_fallback_placement` に固定し、helper code / fixture-side fields / `e23` / `e24` / `smoke-try-rollback-structural-checker` まで actualize 済みである
- 次の narrow question は、second malformed static tranche を足すべきか、それとも helper-local wording / finding family を first tranche のまま数回反復してから shared carrier / public checker comparison へ進むべきかである

### 候補 4. parser 導入前の boundary inventory

- parser を書く前に最低限固定すべき syntax / companion notation / AST boundary を棚卸しする
- 何を final grammar 決定に回し、何をまだ比較候補に残すかを明確にする

## rough step estimate

以下は rough estimate であり、仕様 drift や review 指摘に応じて前後する。

| 目標 | rough step estimate | 注記 |
|---|---|---|
| PoC を継続的に回せる状態を維持しつつ drift regression を増やす | 2〜4 task | detached artifact / host coverage の切り方次第で前後する |
| PoC を「大量に回して比較しやすい」段階へ一段進める | 2〜5 task | detached trace / audit と richer host boundary の優先順位が影響する |
| detached validation loop の入口を current helper boundary を壊さずに揃える | 1〜3 task | storage/path policy、tiny wrapper、aggregate export の actual narrow cut が主論点 |
| 文法をある程度比較しながら PoC を前進させる | 4〜8 task | final parser grammar 固定はまだ含まない |
| parser 導入判断の前提整理 | 5〜10 task | 静的解析や host interface との境界が影響する |

## いまの blocker

### 1. final parser grammar 未固定

- companion notation はかなり整理されたが、parser syntax はまだ未決である
- これを早く決めすぎると semantics より syntax が先行してしまう

### 2. fallback intuition drift

- guarded option chain 読みと outer-longer-lifetime 直感の tension は、まだ prose 補助が要る

### 3. heavier workstream の入口未整理

- 型システム、静的解析、決定可能性、theorem prover 連携は、まだ entry criteria の段階である

### 4. detached trace / audit が process 内比較に閉じていること

- batch / profile まで積んでも、結果を repo 外 artifact として残しにくい
- case 数が増えると「その場で読んで終わる」運用から抜けにくい
- docs-only minimal schema はできたが、thin export boundary と保存パス規約はまだ未決である
- current non-production candidate として `target/current-l2-detached/` は置けたが、final path policy と actual aggregate exporter API はまだ未決である
- current detached validation loop には aggregate emitter sketch と `emit-aggregate` wrapper を足せるが、これは production aggregate API finalization ではない
- ただし actual narrow cut としては、aggregate emitter 内 private transform を shared support module へ切り出し、`BatchRunSummary -> detached aggregate artifact` の repo 内 callable boundary までは進めてよい
- bundle emitter 側も同様に、private transform を shared support module へ切り出し、`FixtureBundle + BundleRunReport -> detached bundle artifact` の repo 内 callable boundary までは進めてよい

### 5. review infrastructure の変動

- reviewer completion が遅い / 返らない場合があり、task の closing evidence を自前で揃える必要がある

## いまの bottleneck

- `fixture authoring / elaboration` の独立 bottleneck は引き続き残っている
- そのうえで、**detached trace / audit serialization と richer host interface の 2 項目を比べるなら**、前者を先に切る方が前進量は大きい
- richer host interface と coverage analysis の入口整理は、その後段候補である
- notation 自体は current L2 の narrow task を回すには十分安定しており、直近では operational boundary の方が重い

## 近い将来の sequencing

推奨順は次である。
ただしこれは `fixture authoring / elaboration` の独立 bottleneck を取り消すものではなく、**trace/audit と host interface の比較に限った近い sequencing** である。

1. semantics drift regression を増やす
2. detached trace / audit serialization の最小境界を切る
3. detached exporter の first entry を bundle 層から narrow に始める
4. non-production の bundle-first emitter と core-only diff helper を足す
5. bundle artifact 保存先 / path policy と aggregate export の最小 cut を整える
6. aggregate summary export の smoke を数回回し、coarse summary / typed count / list anchor の cut を確認する
7. aggregate compare helper を narrow に足し、`summary_core` compare と run-label convenience を固める
8. fixture authoring / elaboration template を detached validation loop 前提へ寄せる
9. richer host interface / coverage analysis の入口を narrow に切る
10. parser 導入前 inventory を作る
11. その後で parser / richer runtime の判断に進む

## 今の working assumption

- current L2 semantics は大きく動かさない
- parser-free PoC は継続利用する
- hard-coded named profile catalog は維持する
- machine-readable catalog asset / manifest はまだ future option に留める

## 次にやるべき narrow-scope task 候補

- detached trace / audit の docs-only schema から thin exporter 候補の carrier mapping を切り出す
- bundle / batch summary が detached artifact として最低限どこまで出せば比較可能かを棚卸しし、bundle-first exporter entry を docs に固定する
- bundle-first artifact の payload core / bundle_context / detached non-core / aggregate-only を docs-only で切り分ける
- detached exporter chain の docs-only judgment を 1 箇所へ集約し、non-production の tiny emitter / diff helper / fixture template を PoC loop 補助として足す
- detached validation loop の storage/path policy、tiny wrapper、aggregate export の actual narrow cut を docs-only から thin operational aid へ進める
- aggregate export の actual narrow cut として、example private transform を shared support module へ落とし、public API を増やさずに repo 内 callable boundary へ寄せる
- bundle export の actual narrow cut として、example private transform を shared support module へ落とし、public API を増やさずに repo 内 callable boundary へ寄せる
- first checker cut の local / structural floor を detached validation loop に接続するなら、runtime artifact と混ぜずに static gate artifact helper を別立てで足す
- detached validation loop の common path として、1 fixture の bundle export / optional reference compare / single-fixture aggregate smoke を 1 command で回す helper を足す
- static-only / malformed / underdeclared fixture の common path として、static gate artifact の emit / compare を 1 command で回す helper を足す
- first checker cut の local / structural floor について、`expected_static.reasons` の direct promotion は current fixture corpus と衝突することが確認できたので、次は additive optional `checked_reasons` を最小 dedicated carrier として導入し、typed reason code / detached-only 維持を後段比較に送る
- `checked_reasons` を bridge に置いたので、その次は typed reason code に進めてよい stable cluster と parametric shape を inventory 化する
- detached static gate artifact 側には helper-local / reference-only な `reason_codes` mirror を actualize してよいが、next narrow step はそれを first-class typed source に昇格させるか、fixture-side typed carrier を別に切るかの比較である
- その前段として、`e4` / `e5` から始めた static-only malformed / underdeclared fixture への `checked_reasons` narrow adoption を、`e12` / `e13` の stable cluster まで広げてよい
- duplicate declaration cluster は next fixture-authoring comparison point として actual corpus に入れてよいが、current helper cut では `checked_reasons` と detached `reason_codes` の stable cluster へはまだ昇格させない
- next actual corpus tranche として、missing chain head / predecessor / successor option cluster は stable malformed wording と detached `reason_codes` mirror を持つので、`e16` / `e17` / `e18` として actualize してよい
- same-lineage stable cluster inventory は current docs の範囲では `e19` によって一巡したので、その次の narrow step は first-class typed carrier actualization を急ぐか、fixture authoring 実地反復をもう少し積むかの比較になる
- current valid fixture 群では actual static gate `reasons` が空な例が多いため、`checked_reasons = []` を広く足す task は優先しない
- その next narrow step として、`checked_reasons` を auto-fill しない display-only authoring assist を current helper stack に置き、actual static gate wording の narrow adoption を review 可能にしてよい
- さらにその次段として、helper-local / reference-only `reason_codes` mirror を assist source にする display-only helper を current helper stack に置き、future typed carrier 候補 row を fixture schema に昇格させずに review 可能にしてよい
- その次の実地反復として、static-only fixture corpus を横断する readiness scan を current helper stack に置き、`checked_reasons` adoption と `reason_codes` suggestion availability を tranche 単位で観察してから future typed carrier actualization の着手順を決めてよい
- readiness scan の次段として、first actualization family は lineage edge pair family、second tranche は declared target edge pair family として比較結果を固定した
- current code / fixture corpus では、その carrier cut を current stable cluster inventory の 8 kind まで広げ、これを覆う 9 fixture で `checked_reason_codes` adoption と `reason_codes` suggestion availability が 9 / 9 で揃うところまで actualize 済みである
- さらに current readiness scan では、stable cluster 8 kind を覆う 9 fixture が `checked_reasons` / `checked_reason_codes` / actual suggestion の 3 者で揃っており、coexistence follow-up は `none` である
- さらに checker cluster roll-up では、same-lineage floor `4`、capability floor `2`、missing-option structure floor `3` まで current static-only corpus が覆っていることを source-backed に確認できる
- さらに same-lineage floor については、fixture-side `checked_reason_codes` と detached static gate artifact `reason_codes` を narrow compare する helper-local first checker spike を追加し、`e4` / `e12` の smoke が通っている
- さらに missing-option structure floor については、same-lineage spike の次段として helper-local second checker spike を追加し、`e16` / `e17` の smoke が通っている
- さらに capability strengthening floor については、`e13` / `e20` を actual corpus に揃えたうえで helper-local third checker spike を追加し、capability family だけを narrow compare する smoke まで回してよい
- duplicate declaration cluster は current cut のまま `checked_reasons` / `checked_reason_codes` / detached `reason_codes` stable inventory へは上げず、actual wording を `checker_core.reasons` と focused smoke で見る
- current cut では、same-lineage / missing-option / capability の 3 spike に shared support helper を導入しつつ family facade script を残すところまでは actualize 済みである
- current docs-only judgment では、checker-side shared family compare entry はまだ切らず、family facade 維持で止める
- 次の narrow step は、public checker cut comparison に入る前に、generic entry を later public checker API comparison と同時に扱うかどうかを見極めることである
- `TryFallback` / `AtomicCut` dedicated AST structural helper の first tranche は actualize 済みなので、次に自走で進めるなら
  - second malformed static tranche の comparison close
  - first-tranche wording / row family の drift 点検
  - saved artifact compare need が shared carrier threshold を本当に満たすかの再比較
  の順で narrow に進めるのが自然である
- current comparison としては、second malformed static tranche の問い自体は先に閉じるが、actual tranche 追加はまだ行わず、next は helper-local wording / finding family stability comparison に進むのが自然である
- current comparison としては、first-tranche wording / finding family stability では exact wording / row family を fixed working set として維持し、generic 化や alias 導入は shared carrier threshold の再比較と later generic/public comparison まで deferred にするのが自然である
- current comparison としては、saved artifact compare need の観点で shared carrier threshold を再比較しても、current helper-local checker が artifact path を直接読めるため threshold はまだ未充足とみなし、next は generic structural checker family / public checker API comparison の前提条件整理へ進むのが自然である
- current comparison としては、generic structural checker family / public checker API comparison に進む concrete pressure もまだ不足しているため、current try/rollback checker line はここで一旦 pause とみなし、next self-drivable mainline は parser boundary / first parser cut inventory 側へ移すのが自然である
- aggregate emitter sketch を current wrapper に接続し、directory summary を artifact として保存する smoke を増やす
- fixture authoring bottleneck のうち boilerplate 部分だけを `target/` 下の non-production scaffold helper へ切り出し、hand-written fixture を正本に保ったまま authoring cost を下げる
- parser-free host harness と richer host interface / coverage analysis の boundary inventory を作る
- current baseline として、current companion notation から first parser cut に入れてよい semantic cluster は narrow inventory 化済みである
- current baseline として、parser 導入前の syntax decision inventory は plan と spec に切り出し済みである
- current baseline として、first parser cut inventory は first checker cut の local / structural judgment と theorem prover / model checker 側へ残す property の entry criteria に docs-only で接続済みである
- current baseline として、actual parser spike の比較を monolithic cut ではなく checker-led staged spike として扱い、
  1. chain / declaration structural floor
  2. `try` / rollback structural floor
  3. request / admissibility cluster
  の順で source-backed priority を揃えるのが自然である
- current next narrow step としては、stage 1 parser spike の accepted parse cluster を
  - option declaration core
  - explicit edge-row family
  - edge-local lineage metadata
  - declaration-side guard slot
  に留めつつ、declaration-side guard slot は predicate fragment parse へ進めず opaque attached slot として扱う cut を維持するのが自然である
- current baseline として、stage 1 handoff は parser-side opaque slot carrier と current parser-free AST fixture schema を同一視せず、thin lowering bridge を介して `OptionDecl.lease` へ narrow に接続するのが自然である
- current baseline として、その parser-side opaque slot carrier の naming は `decl_guard_slot` を第一候補にし、thin lowering bridge は slot-only helper ではなく option-level structural transfer として読むのが自然である
- current baseline として、stage 1 actual parser spike の smoke family は `e4-malformed-lineage` と `e7-write-fallback-after-expiry` の two-fixture pair を最小 working set とし、`e3-option-admit-chain` は later-stage contrast reference に残すのが自然である
- その次段で比較するべきなのは、
  - `decl_guard_slot` 内部 carrier が raw text / token slice / opaque leaf のどれか
  - option-level structural transfer の actual private API surface
  - actual parser spike の private module / helper 配置
  であり、predicate fragment floor や option-local `admit` を stage 1 へ前倒ししない
