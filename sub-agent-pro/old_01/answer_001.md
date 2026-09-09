2026-04-13〜2026-04-14 の文書群に基づくと、**Macro 4 / 5 / 6 / 7 / 8 という大局の切り方自体は source-backed に妥当**です。ただし、**近接順序はそのままだと 1 点ずれています**。current near-term reading では、`Macro 6` の shared-space docs-first follow-up checkpoint を先に一旦閉じ、その後に `Macro 5` の sample-visible theorem/model-check milestone を閉じ、さらにその後に `Macro 7` の docs-first I/O / host-facing boundary を固定する流れが正本です。しかも現時点の **current mainline は `Macro 4` の stable malformed capability second source-backed widening actualization** で、**reserve line は `Macro 7` の public operational CLI concrete shell actualization** に置かれています。したがって、あなたの大局把握はかなり近いのですが、近接順序は「4→5→7→4→7」という interleave を含む、というのが source-backed な読みです。   

## まず結論

**source-backed に妥当**なのは次です。
Mir が主眼で、repo は architecture-first だが fixed-subset runnable path をすでに持つこと。Macro 4 は active、Macro 5 は early-active、Macro 6 は docs-first boundary only、Macro 7 は inventory plus later-gate order、Macro 8 は not started / user spec required という読み。さらに、自走でよい line は semantic kernel の narrow refinement、parser-free regression / helper maintenance、compile-ready minimal actualization の narrow widening、fixed-subset source sample expansion、theorem-side bridge の compare-ready / first pilot、sample-visible theorem/model-check milestone の narrow actualizationまでで、Mirrorea/shared-space は docs-first boundary まで、public operational surface と docs-first I/O / host-facing boundary も boundary までなら self-driven です。  

**未決**として残すべきなのは、final parser grammar、final public parser/checker/runtime API、concrete theorem/model-check production contract、shared-space final catalog、backend/codegen public surface、upper-layer application contract、raw FFI / game engine direct binding actualizationです。重い将来 workstream 側では full 型システム、静的解析の本格化、定理証明可能性、決定可能性、backend / codegen、higher-level async-control / memory-model もまだ heavy retained-later として扱われています。ここを current mainline に混ぜないのが現行方針です。    

**推奨**は、閉じた checkpoint を不用意に reopen せず、いまは `Macro 4` の capability source-backed widening actualization を先に閉じ、その後段に CLI concrete shell actualization を reserve として扱うことです。sample-visible theorem/model-check milestone や docs-first I/O boundary は「次にやる line」ではなく「すでに固定済みの前段 checkpoint」として保持するのが正しいです。  

## 1. 先に進めるべき line / later gate に留めるべき line

先に進めるべき line は、かなり明確です。第一に、**stable malformed capability second source-backed widening actualization**。これが current mainline です。第二に、その line を支える範囲での fixed-subset runnable path の ratchet 維持です。つまり parser-free helper maintenance、source-sample expansion、narrow compile-ready actualization、theorem-side compare-ready / first pilot までは進めてよいが、いま再び broad な reopen に戻すべきではありません。第三に、public/dev surface 側は **library-before-CLI later ordering** を維持しつつ narrow に扱うことです。`run_current_l2_source_sample` を first later cut、`run_current_l2_runtime_skeleton` を later support cut、Rust-side concrete shell を separate reserve に置く、という現在の薄い publicization が source-backed です。   

later gate に留めるべき line も明瞭です。**final parser grammar、final public parser/checker/runtime API、production-grade theorem/model-check binding、shared-space final activation/authority/auth/identity/admission/consistency/fairness catalog、backend/codegen public surface、upper-layer app contract、raw FFI / game engine direct binding** は stop line です。特に theorem/model-check は、review-unit first pilot や model-check concrete carrier first actualization までは current line に入っていますが、**concrete tool binding は still later** です。backend も public operational surface success criteria が later gate のままなので、今ここへ入るのは source-backed ではありません。   

## 2. 今の順序の見落とし / 危険な前提

いちばん大きい見落としは、**macro phase を strict waterfall とみなす前提**です。`plan/17` は明確に、phase は strict waterfall ではなく複数 line が並走してよいと書いています。現実の near-term order も、Macro 6 の docs-first checkpoint → Macro 5 の theorem/model-check milestone → Macro 7 の I/O boundary → Macro 4 の malformed widening → Macro 7 の CLI reserve、という interleave になっています。つまり macro number は「厳密な実行順」ではなく、「何の系統か」を表すラベルです。  

次の危険な前提は、**sample-visible theorem/model-check milestone が済んだら、そのまま concrete tool binding や public checker へ進める**という読みです。文書は逆で、formal tool binding 未選定を明示的な OPEN / reserve path に置き、theorem-first を first reserve、model-check side を second reserve にしています。`proof_notebook_review_unit` は current first concrete consumer ですが、production binding ではありません。ここを混ぜると checker/runtime surface が逆流する、というリスクも明記されています。

三つ目は、**docs-first I/O / host-facing boundary を production host interface と誤読すること**です。現在 fixed なのは、privileged `stdin/stdout` を言語 core に入れず、capability-scoped port / adapter boundary を docs-first に切る first cut だけです。final naming、actual host interface contract、visualizer / host substrate / FFI / engine adapter の first target profile は later gate に残っています。したがって「I/O boundary を閉じた = host integration が決まった」ではありません。  

四つ目は、**source sample の widening と formal tool binding と backend binding を同時に太らせる前提**です。risk register は、fixed-subset source-sample path を grammar widening や concrete tool binding と同時に太らせると verification staging の境界が崩れやすい、とかなり強く警告しています。現行方針は、source-backed widening を narrow に進め、tool binding と backend は後段に残す、です。 

## 3. theorem/model-check milestone と shared-space docs-first boundary の間に別 line を挟むべきか

source-backed に言うと、**「間に何かを挟むべきか」ではなく、順序自体を修正すべき**です。current near-term reading では、**shared-space docs-first follow-up checkpoint の方が先**です。そのうえで、その shared-space side checkpoint と sample-visible theorem/model-check milestone のあいだに実際に置かれている重要 line は、

* model-check/public-checker second reserve inventory
* model-check concrete carrier first actualization gate / actualization
* source-sample emitted verification artifact wiring
  です。
  その後に sample-facing theorem/model-check evidence summary / bless-review flow が来ます。つまり、あなたの順序で「theorem milestone の後に shared-space boundaryを置く」のではなく、**shared-space docs-first → model-check carrier line → emitted wiring → sample-facing theorem/model-check summary** という方が current documents に忠実です。    

なので、この問いへの推奨はこうです。**shared-space docs-first boundary を theorem milestone の後ろへ送らない**こと。そして、bridge の実体としては **model-check concrete carrier first actualization** と **source-sample emitted verification artifact wiring** を独立 line として意識することです。ここを 1 つの「theorem/model-check milestone」に潰してしまうと、tool-neutral formal hook → concrete carrier → emitted route → sample-facing surface という実際の段階差が見えにくくなります。  

## 4. full 型システム / backend の前に必要な checkpoint

full 型システムに入る前の source-backed checkpoint は、`plan/13` がかなり明確です。最低限必要なのは、

* current L2 semantics の failure space と fallback 読みが十分安定していること
* parser 境界の最小 shape が見えていること
  です。加えて inventory note では、first parser cut 候補が見え、same-lineage / missing-option / capability / `try`-rollback locality などの first checker cut 候補も inventory 化できること、しかも static-only corpus で same-lineage floor `4`、capability floor `2`、missing-option structure floor `3` の source-backed baseline が見え始めていることが前提として挙がっています。ただし、そこで **final checker API へ固定してはいけない** とも明記されています。つまり go/no-go 的には、「意味論安定 + parser minimum shape 可視 + checker floor inventory / baseline 確保」までは必要ですが、「public checker freeze」はまだ不要で、むしろ早すぎます。 

backend / codegen に入る前の checkpoint はもっと厳しいです。必要なのは、**parser 前提の syntax / AST / runtime boundary がもう少し安定していること**。しかも現状は、fixed-subset source corpus、parser-to-`Program` lowering、syntax-backed runner、verification ladder がまだ narrow 化の途中なので、backend を先につなぐと semantics / syntax / lowering の柔軟性を失いやすい、と文書がはっきり書いています。したがって backend への安全な進行条件は、「syntax/AST/runtime boundary が stabilize し、source corpus〜lowering〜runner〜verification ladder の narrow化が一段落し、thin-facade / support cut / host-facing boundary が separate gate のまま保てること」です。現時点はまだ **backend inventory を持ってよいが、backend binding を mainline に入れる段階ではない** というのが source-backed です。 

## rough ordering（近接 10 package 前後）

今の文書群から素直に圧縮すると、近接 ordering は次のように読むのが最も自然です。

1. shared-space docs-first follow-up checkpoint の close
2. model-check/public-checker second reserve inventory
3. model-check concrete carrier first actualization gate / actualization
4. source-sample emitted verification artifact wiring
5. sample-facing theorem/model-check evidence summary / bless-review flow
6. docs-first I/O / host-facing port boundary
7. missing-option reopen comparison → source-backed widening first cut
8. final public parser/checker/runtime first later gate（runtime-led thin facade）
9. CLI second gate / runtime skeleton later support / shell naming package 群
10. capability second reopen comparison → capability second source-backed widening actualization（current mainline）
11. public operational CLI concrete shell actualization（current reserve）
    このうち 1〜9 は多くが「すでに close / fixed 済みの checkpoint」で、**今から reopen する推奨 line ではなく、現在 line の前提**として扱うのが正しいです。実務的には 10 を先に閉じ、その後で 11 を narrow に進める、が current recommendation です。   

## 今やるべきではない line

最後に明示しておくと、**今やるべきではない line** は次です。

* final parser grammar の凍結
* final public parser / checker / runtime API の finalization
* concrete theorem/model-check production contract / concrete tool binding
* shared-space final catalog の固定
* backend / codegen public surface
* raw FFI / game engine direct binding actualization
* full 型システムの本格固定
* higher-level async-control / low-level memory-order family の core 先行導入
* application target / upper-layer contract の finalization
  これらは later gate か heavy future workstream です。いま入ると、current L2 semantics、source-sample ladder、helper boundary、thin-facade publicization を premature に凍らせる側に働きます。    

要するに、あなたの macro 切り方は正しいです。ただし **近接順序は「shared-space docs-first → model-check carrier / emitted wiring → sample-visible theorem/model-check → docs-first I/O → malformed widening / thin-facade / CLI reserve」へ修正**した方が、現在の repo 読みと一致します。そして「次に本当に進めるべき line」は、closed checkpoint の再議論ではなく、**stable malformed capability second source-backed widening actualization** を閉じることです。  
