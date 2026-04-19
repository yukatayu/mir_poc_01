# 10 — 未解決事項

この文書は、意図的に未解決事項を記録する。

## Mir の意味論

1. Mir-0 の primitive fallback を超えた、preference chain / fallback normalization の最終形式化。
   - 現時点の最小理解では、fallback 付き参照または guarded reference は、1 つの論理的 access path に対する有限の guarded option chain として読む。
   - preference chain は一次 primitive ではなく nested fallback の正規形であり、`lease` は各 option の lifetime guard を表す最小語彙に留める。
   - `lease` の期限切れは monotone degradation の一種であり、同じ semantic lineage で earlier option への再昇格を許さない。
   - write は current option の contract が許すときだけ成立し、write-capable option の `lease` が期限切れた後は、後段の write-capable option への explicit fallback がない限り `Reject` として扱う。
   - canonical normalization の最小 law は left-to-right flattening であり、`fallback(x, y)` は `canon(x) ++ canon(y)` として canonical chain へ畳む。ただし同じ logical access path / semantic lineage を共有し、後段が monotone degradation を保つ場合に限る。
   - canonical form が一意に表すのは候補の優先順と各候補の guard / contract / capability であり、元の nested syntax の内側 / 外側そのものではない。
   - incompatible branch の最小分類は、same logical access path / semantic lineage を共有しない branch、monotone degradation を壊す branch、declared contract surface / capability surface だけで contract-compatible fallback successor になれない branch である。
   - これらの incompatibility が declared target / lineage / contract / capability だけで判定できる場合、その nested fallback は evaluation に入る前に static rejection する。canonical chain を作った後の runtime `Reject` に落として hidden repair してはならない。
   - 逆に canonical chain 自体が well-formed であり、current evaluation で `lease` 期限切れ、`require` 不成立、explicit failure、write-after-expiry 後の後段不在が起きる場合は、従来どおり dynamic `Reject` に残す。
   - current L2 で static same-lineage 判定に使う最小 declared information は、各 option の `declared access target` と、fallback successor が前段 option と同じ semantic lineage を継続することを示す `documented lineage annotation` である。`declared access target` だけでは same logical access path / semantic lineage の証拠として足りない。
   - current L2 で `documented lineage annotation` に要求する最小 surface form は、ちょうど 1 本の fallback edge を飾る edge-local annotation である。最低限、predecessor option 参照、successor option 参照、same-lineage continuation の affirmative claim を持てばよい。説明用には `lineage(A -> B)` と書いてよいが、最終 keyword / punctuation は未固定である。
   - `declared access target` は option-local に残し、`documented lineage annotation` は edge-local に使う。static same-lineage 判定は、この両方がそろって初めて admissible である。annotation 単独でも target 単独でも足りない。
   - current L2 は option-local tag、chain-level blanket annotation、global lineage identifier を要求しない。将来 sugar として許すかは未決定である。
   - current L2 で contract-compatible fallback successor を static に残す最小条件は、前段 option と後段 option について、`declared capability surface` が強化されておらず、`declared contract surface` に successor 使用を明示的に打ち消す矛盾がないことである。
   - `declared access target` 不一致、`documented lineage annotation` による same-lineage 否定、`declared capability surface` の明示的強化、`declared contract surface` の明示的矛盾は static rejection に寄せる。
   - edge-local annotation が自分の飾る fallback edge の predecessor / successor を正しく指していれば `valid` であり、別の edge や別の option を指していれば `malformed` である。`declared access target` または annotation のどちらかが欠ければ `underdeclared` である。
   - underdeclared fallback case とは、最小 declared information のどこかが欠けていて、same-lineage 判定または successor 判定を current L2 の floor まで持ち上げられない branch を指す。少なくとも、`declared access target` はあるが `documented lineage annotation` がない場合、`declared access target` 自体が不足している場合、`declared contract surface` / `declared capability surface` が不足して successor 判定できない場合を含む。
   - current L2 では、underdeclared fallback case は surface-level static error とする。canonical chain へ hidden に入れず、dynamic `Reject` へ押し込まない。
   - `elaboration obligation` は current L2 の admitted path には含めない。将来導入する場合でも、canonicalization / evaluation 前に同じ static evidence floor を discharge しなければならない。
   - **未決定**: `lease` / `GuardedRef` の最終 surface syntax、preference chain の完全代数、redundant option の collapse rule、`documented lineage annotation` の最終 keyword / punctuation / serialization と option-local / chain-level sugar の扱い、same-place / cross-place で同じ annotation shape をどこまで共有するか、`declared contract surface` の明示的矛盾をどこまで static に比較するか、`declared capability surface` を read / write 以上に広げずに扱えない mixed case を static rejection と dynamic `Reject` のどちらに寄せるか、`lease` 期限切れの dedicated observation surface、future extension として `elaboration obligation` を認める必要が本当にあるか。
2. `durable_cut` は Mir-0 に含めない。Mir-1 側では、`atomic_cut` に abstract persistence requirement を伴う durable commit guarantee を追加する cut vocabulary 候補として扱う。
   - 最小意味に含めるのは、successful として観測された pre-cut prefix が local rollback / process restart / route rebinding の後に未確定へ戻らないことを要求する durable commit guarantee である。
   - persistence はその guarantee を支える abstract requirement として前提するが、具体的な storage / replication / consensus mechanism は Mir-1 では固定しない。
   - successful `durable_cut` は、ある cut attempt が特定の pre-cut prefix に対して persistence evidence に支持されたことを示す明示的な event が立ったときにだけ観測されたものとみなす。
   - durable guarantee を確立できなかった場合は、successful な `durable_cut` は成立していない失敗試行として扱う。既定分類は `Reject` であり、cut 後の外部化された obligations を明示的に巻き戻す必要がある場合だけ `Compensate` を使う。
   - failed `durable_cut` は、同じ cut attempt について successful event が立つ前に failure outcome が明示されたときに観測される。したがって失敗観測点は durable side の内側ではなく、cut 成立判定の outcome surface にある。
   - `Approximate` は durability を弱めた代替結果を contract が明示的に許す場合にしか入る余地がなく、`durable_cut` failure の既定分類にはしない。
   - audit surface には、少なくとも cut attempt、対象の pre-cut prefix、success / failure outcome、そして success を支えた persistence evidence の観測有無を説明できる trace が残らなければならない。
   - persistence evidence は、durable guarantee がその pre-cut prefix に対して成立したことを支持する抽象的証拠を意味する。Mir-1 が要求する最小情報は、どの cut attempt / prefix を支持するかと、その結果が success 観測を正当化するかである。
   - cross-place 文脈では、各 participating `place` に local cut attempt があり、cross-place `durable_cut` はそれらを束ねる aggregate attempt を観測単位として扱う。single-place 文脈で `attempt` と言うときは local cut attempt を指す。
   - cross-place 文脈の scope rule は、その aggregate attempt に対してどの member local observation / persistence evidence が aggregate success / failure 判定に数えられるかを定める明示的 rule を意味する。
   - cross-place 文脈の `prefix` は、単一の merged / total prefix ではなく、participating `place` ごとの local prefix の有限対応を意味する。single-place 文脈では `prefix` は従来どおり local prefix を指す。
   - single-place な success / failure event は member-local observation として保持される。cross-place `durable_cut` 自体を観測するには、それらを束ねる aggregate success / failure event が必要であり、これは Mir-1 の意味語彙として扱う。
   - cross-place successful `durable_cut` は、aggregate attempt に対して明示された scope rule が required member local observation / persistence evidence によって満たされたことを示す aggregate success event が立ったときにだけ観測される。
   - cross-place failed `durable_cut` は、同じ aggregate attempt について aggregate success event より前に aggregate failure event が明示されたときに観測される。
   - cross-place の audit surface には、少なくとも aggregate attempt、participating `place` の集合、対応する local attempt / local prefix、scope rule の参照、aggregate outcome、どの member observation / evidence が outcome 判定に数えられたかを説明できる trace が残らなければならない。
   - Mir-1 が現時点で必須とする cross-place scope rule profile は `all_of` のみである。`all_of` は、aggregate attempt に含まれる participating `place` のすべてについて、対応する required local observation / persistence evidence が success 判定に数えられることを要求する。
   - `all_of` は single-place observation を conjunction として束ねる最小 profile であり、participating `place` の一部だけで aggregate success を許す追加意味を持ち込まない。
   - `all_of` における aggregate evidence の最小要件は、participating `place` の全員について、対応する local attempt / local prefix に対する success 側の local observation と、それを支える persistence evidence が outcome 判定に数えられる形で揃っていることである。Mir-1 が要求するのは、この full coverage が成立していることだけである。
   - `all_of` の event surface に Mir-1 が残す最小情報は、aggregate attempt に対して full coverage が成立したことを示す aggregate success event、またはそれより前に failure outcome が明示されたことを示す aggregate failure event である。各 `place` ごとの evidence payload や具象表現を event surface で inline することは要求しない。
   - `all_of` の audit surface には、participating `place` ごとに対応する local attempt / local prefix、counted local observation の有無、supporting persistence evidence が outcome 判定に数えられたかを追跡できる trace が残らなければならない。ack / signature / proof / log record などの具象形式と、その表現方法は Mirrorea の裁量に残す。
   - `all_of` で full coverage が成立していないことは aggregate success を正当化しないが、それだけで aggregate failure を確定しない。full coverage 不足だけでは、まだ evidence 収集中なのか、同じ aggregate attempt が失敗として確定したのかを区別できないためである。
   - `all_of` では、required member の一つに対応する local attempt / local prefix で explicit local failure event が立てば、その aggregate attempt は full coverage を達成できない。これは aggregate failure event を正当化する十分条件になるが、aggregate failure が outcome surface で観測されたことになるのは、従来どおり explicit aggregate failure event が立ったときだけである。
   - `all_of` では、required member local failure がまだ出ていない場合でも、同じ aggregate attempt が aggregate success event より前に failed outcome として明示的に閉じられたなら、aggregate failure event を出してよい。Mir-1 が意味として要求するのは、その failed closure が当該 aggregate attempt の success を打ち切ることだけであり、timeout-like budget、policy cancellation、retry exhaustion、route / transport closure などの具体理由は Mirrorea の operational policy / audit に残す。
   - `all_of` の audit surface は aggregate failure に対して、少なくとも aggregate attempt、participating / required member の集合、各 required member に対応する local attempt / local prefix、aggregate failure を正当化した source、および failure 観測時点または closure 時点の missing coverage 状態を説明できなければならない。
   - justification source が required member local failure である場合、audit はその member と対応する explicit local failure event を参照できなければならない。
   - justification source が explicit failed closure である場合、audit はその aggregate attempt を failed outcome として閉じた act を参照できなければならない。timeout-like budget、policy cancellation、retry exhaustion などの具体理由分類は、その act に付随する Mirrorea 側表現に残し、Mir-1 では固定しない。
   - missing coverage snapshot の最小基準時点は、same aggregate attempt が aggregate failure outcome として明示的に観測された決定点である。Mir-1 は少なくともこの一時点の snapshot を要求し、それ以前の途中経過や複数時点比較までは要求しない。
   - justification source が explicit failed closure である場合、closure がそのまま aggregate failure outcome を明示するなら、その closure 時点が missing coverage snapshot の基準時点になる。required member local failure が justification source の場合は、member-local failure 自体ではなく aggregate failure 観測時点を基準時点とする。
   - missing coverage を説明する最小要件は、その基準時点で、どの required member が counted success-side coverage を持っていたか、どの member が local failure により coverage 不可能になっていたか、どの member が未充足のまま残っていたかを区別できることである。
   - したがって Mir-1 が要求する最小粒度は per-required-member 粒度である。各 member の state は、対応する local attempt / local prefix 参照と結びついた上で、counted success-side coverage あり、local failure により coverage 不可能、未充足の少なくとも三通りを区別できれば足りる。event-by-event の履歴や per-prefix の追加 snapshot を Mir-1 で義務づけない。
   - ただし上の三状態は、現時点では `all_of` aggregate failure audit と missing coverage snapshot を説明するための最小抽象区別に留める。Mir-1 の独立した outcome / contract 語彙として名前付きで引き上げることはしない。
   - `all_of` の aggregate success / failure、full coverage、failure justification の意味は、従来どおり counted success-side coverage の充足と explicit failure justification だけで定まる。contract が上の三状態名を直接参照することは Mir-1 では要求しない。
   - `reason_ref` のような field 名や独立語彙は Mir-1 では固定しない。Mir-1 が要求するのは、justification source と required member ごとの上記区別が参照可能であることだけであり、その ID 形式、serialization、audit layout、state naming は Mirrorea の裁量に残す。
   - `quorum-like` profile は将来拡張候補としては残すが、現時点の Mir-1 には含めない。導入には、participating `place` の一部だけで十分とする条件や、そのとき aggregate evidence / audit に何を要求するかを追加で意味づける必要があるためである。
   - `implementation-defined` は Mir-1 の profile 語彙にはしない。Mirrorea が profile 実現で裁量を持てるのは、Mir-1 がすでに名前と意味を与えた profile について、同じ aggregate success / failure 観測を保つ形で実現する範囲に限る。
   - local durable failure と distributed durable failure は、現時点では別 failure class に分けない。どちらも同じ durable guarantee failure の実現上の差分とみなす。
   - distributed finalization は最小意味そのものではなく、複数 `place` / 実ノードにまたがる場合の operational realization 側に送る。
   - 未決定なのは、`quorum-like` を将来の Mir-1 拡張 profile として本当に採用するか、`all_of` の per-place evidence 参照を event surface にどこまで露出するか、local evidence と distributed outcome の関係をどこまで Mir-1 で語るか、timeout-like closure や policy-driven closure を Mir-1 の独立語彙に引き上げる必要が将来あるか、coverage state の三状態を将来 Mir-1 の独立語彙や contract language に引き上げる必要があるか、`reason_ref` 相当の参照語彙を Mir-1 で標準化する必要があるか、snapshot の複数時点比較を Mir-1 で語る必要があるか、audit の粒度を per-place / per-attempt / per-prefix のどこまで固定するか、将来 profile family が増えたときに aggregate evidence / aggregate failure justification / audit minimum requirements / snapshot semantics の意味差分をどこまで先に固定するか、`Approximate` を許す contract 条件である。
3. `barrier` は Mir-0 に含めない。Mir-1 に残すとすれば ordering primitive 候補であり、commit-like primitive として扱うべきではない。未決定なのは、独立語彙として残す価値が本当にあるかどうかである。
4. `perform` は現時点では最小 effect request operation の説明用記法であり、Mir-0 の規範的な表面構文として採用するかどうかは未決定である。
   - current L2 の representative examples では、direct target に対して `perform op on target`、option chain に対して `perform op via chain_ref` という companion syntax 候補を使ってよい。
   - ただしこれは examples 用の安定した表記候補であり、`perform`、`on`、`via` を最終 reserved keyword として固定する判断ではない。
   - option chain 側の候補書式として `option name on target capability cap lease guard` と `chain ref = head` / `fallback successor` / indented `@ lineage(...)` を current L2 で使ってよいが、これも final parser syntax ではない。current L2 では hanging continuation を polished first choice に置くが、短い row に対する inline 省略形 `fallback successor @ lineage(...)` も companion-equivalent shorthand として残してよい。notation rollout としては、representative examples のうち fallback / preference chain 自体を主題にする code block には hanging continuation を first choice として広げてよいが、fixture schema / interpreter / tests まで同期させる必要はない。
   - `contract` は semantic role の名前としては使うが、current L2 の representative examples では独立 block keyword にしない。`require` / `ensure` は indented な statement-local clause として直前の `perform` に付けてよい。
   - current L2 の companion notation では、`contract { require { ... } ensure { ... } }` のような block form を optional sugar としても採用しない。`place` / `try` / `fallback` の既存 block nesting と競合させず、clause attachment の読みを 1 つに保つためである。
   - current L2 の companion notation では、statement separator に dedicated token を要求しない。`place` / `try` / `fallback` の brace-delimited block、`perform` / `atomic_cut` / `option` / `chain` の statement line、直後の indented clause / continuation line、dedent によって最小の読みを与える。
   - current L2 の companion notation では、single-line clause を `require pred` / `ensure pred` と書き、multi-line predicate が必要な場合だけ `require:` / `ensure:` に続く 1 段深い predicate block を使ってよい。predicate block は 1 つの predicate を複数行に折り返したものとして読む。
   - current L2 の最小 predicate fragment としては、bare atom、application-like form、explicit `and`、括弧 grouping を使ってよい。predicate block 内の改行は continuation としてだけ読み、改行だけで implicit conjunction を導入しない。blank line も current L2 では許さない。
- option declaration は引き続き `option name on target capability cap lease guard` を最小 head form とし、capability と `lease` だけでは option 間の admissibility の違いを書けない場合は、indented な option-local `admit pred` / `admit:` を admission-side metadata として明示しなければならない。省略したまま判定に進む branch は underdeclared 側に残る。
- option-local `admit` は `perform` に付く statement-local `require` / `ensure` を option declaration に流用しないための companion marker であり、request-local clause attachment を壊さないために別 token として扱う。current L2 では option-local `ensure` / `invariant` や outcome-side guarantee を導入しない。
- option-local `admit` に書く predicate 断片は current L2 では statement-local clause と同じ最小 fragment に揃えてよい。ただしこれは examples 用の安定した表記候補であり、option-local contract surface の final parser syntax を固定する判断ではない。
- current L2 では、`admit` 不成立は non-admissible skip と読む。すなわち、その option は current request の success-side candidate から外れるだけであり、その時点では explicit failure outcome も dynamic `Reject` も立てない。explicit failure は admitted option を試した後の failure に残し、dynamic `Reject` は後続 option も含めて admissible candidate が尽きたときにだけ立てる。
- current L2 の最小観測面としては、`admit` 不成立や `lease` expiry を dedicated skip event として event surface に上げる必要はない。event surface には request-level outcome を優先して残し、individual option miss は audit / trace 側の non-admissible reason metadata に留めてよい。
- その場合、`admit` miss と `lease` expiry は同じ大分類の non-admissible reason に入れてよいが、少なくとも `admit-miss` と `lease-expired` を区別できるだけの subreason は残すべきである。最終的な field 名、reason code 名、serialization は未決定である。
- dedicated skip event を導入しない current L2 読解で E3 比較用 variant と E6 を説明するための最小 conceptual shape は、current request evaluation に結び付いた audit metadata として、少なくとも `option ref` と `admit-miss` / `lease-expired` を区別する `subreason` を持つことである。broad family として non-admissible reason であることは metadata channel 自体から読めればよく、独立した `reason kind` field を current L2 で必須にはしない。
- explicit な `request ref` field は current L2 の最小要件には含めない。metadata 自体が current request evaluation に request-local に結び付いていれば十分だからである。ただし detached serialization や cross-trace correlation を導入する場合に `request ref` 相当が必要かどうかは **未決定** である。
- current L2 では、`admit-miss` と `lease-expired` だけを formal subreason として残し、capability mismatch は request-local `require` と declared capability surface から読む narrative audit explanation に留める。これにより E6 の説明は足りる。
- current L2 の parser-free AST fixture では、`place` / `perform on` / `perform via` / `option` / `chain` / `try` / `fallback` / `atomic_cut` / statement-local `require` / `ensure` / option-local `admit` を syntax token ではなく意味構造として保持してよい。`contract` は surface keyword にせず semantic role object として持てば足りる。
- current L2 の parser-free 最小 interpreter では、少なくとも `cursor_stack`、`place_stack`、snapshot 可能な `place_store`、`current_request`、option chain 用 `chain_cursor`、local rollback 用 `rollback_stack`、request-level `trace_audit_sink`、`terminal_outcome` を持てば E1 / E2 / E3 variant / E6 を動かすのに足りる。`atomic_cut` frontier は current L2 では、active な rollback frame がある場合に限り `rollback_stack` 内の restore snapshot 更新として表してよい。
- current L2 の parser-free 最小 interpreter では、Program / PlaceBlock / PerformOn / PerformVia / TryFallback / AtomicCut を 1-step 単位で進める最小 semantics を companion reading として置いてよい。current L2 では、step ごとの失敗伝播は persisted state field ではなく meta-level control result として扱い、`TryFallback` がそれを catch して rollback / fallback へ変換し、root まで未処理で届いた場合にだけ `terminal_outcome` を確定すればよい。
- `OptionDecl` と `ChainDecl` は current L2 では declaration-only node として runtime no-op にしてよく、`PerformVia` が必要になった時点で current `place` から見える immutable fixture carrier から chain order と option metadata を解決すれば足りる。declaration environment を独立 state component に上げるかどうかは **未決定** である。
- current L2 の parser-free 最小 interpreter では、oracle boundary を `PredicateOracle` と `EffectOracle` に分けてよい。`PredicateOracle` は request-local `require` / `ensure` と option-local `admit` の真偽だけを返し、`EffectOracle` は admitted request に対して success-side carrier または `explicit_failure` を返す。`lease` expiry、capability mismatch、chain exhaustion による `Reject` は interpreter 側の structural rule として扱い、non-admissible skip や request-level `Reject` を effect oracle の local carrier に混ぜない。
- effect success が返す success-side carrier は current L2 では opaque なままでよく、可視な `place_store` 更新は request-local `ensure` が通った後にだけ起こる。`ensure` 不成立は current L2 では `explicit_failure` に寄せ、success-side carrier を commit しない。
- ただし `PerformVia` では、well-formed chain 上の admitted option に対する request-local `require` 不成立、admitted option の `explicit_failure`、および admitted option success 後の `ensure` 不成立が後続候補の尽きた地点で現れた場合、最終 request-level outcome は current L2 の broader dynamic `Reject` reading に従って `Reject` に畳む。local cause は trace / audit に残してよいが、effect oracle 自体に `Reject` を返させない。
- current L2 の host harness では、machine-readable host plan を fixture JSON に埋め込まず、fixture path の `.json` 拡張子を `.host-plan.json` に置き換えた隣接 sidecar asset として扱ってよい。これにより AST fixture schema と host-side verification plan の責務を分離できる。
- current L2 の host plan rule は、runtime に入る fixture で実際に発生する oracle call を明示的に被覆しなければならない。未被覆 call を permissive success default で通す reading は current L2 では採らない。
- current L2 の oracle trait skeleton は infallible でもよいが、host harness 実装は未被覆 call を fail-closed placeholder verdict と violation 記録で扱い、少なくとも synthetic success-side commit を発明しない。
- optional field による wildcard rule は current L2 でも使ってよいが、複数 rule が同じ oracle input を同時に受理できる overlap は forbidden とし、loader / harness が reject してよい。first-match / last-match の precedence policy は current L2 に入れない。
- current L2 では、fixture JSON と `.host-plan.json` sidecar を 1 組の bundle として load してよい。runtime に入る fixture は adjacent sidecar を必須とし、static-only fixture は sidecar 無しでも bundle として扱ってよい。
- current L2 の bundle helper は、static gate、runtime outcome、trace / audit expectation、host plan coverage を一括で走らせてよい。ただし `must_explain` は machine-check に上げず human-facing explanation obligation に残す。
- current L2 では、fixture directory 直下の `*.json` から `.host-plan.json` を除外して bundle candidate を discovery し、runtime bundle / static-only bundle を `expected_runtime.enters_evaluation` から振り分ける batch runner を置いてよい。summary report は少なくとも total bundles、runtime bundles、static-only bundles、passed / failed、discovery failure、host-plan coverage failure を返してよい。ただし `must_explain` は batch runner でも machine-check に上げない。
- current L2 では、batch runner の上に `runtime-only` / `static-only` / `single-fixture` を選べる selection helper を置いてよい。`single-fixture` は fixture stem または path 指定を受け、選別後の bundles / failures だけを summary に残してよい。runtime/static-only selection で pre-classification discovery failure をどう長期的に扱うか、bundle manifest、selector grammar の長期固定、path canonicalization policy は未決定である。
- current L2 では、selection helper の上に `profile_name` を持つ小さな profile layer を置いてよい。profile layer は optional な `runtime-only` / `static-only` scope と optional な `single-fixture` selector を 1 つの request として扱い、`runtime-only + single-fixture` のような複合指定を selected batch 実行へ畳んでよい。ただし sidecar discovery や runtime/static-only classification は再実装せず、selection helper の primitive mode を合成するだけに留める。`single-fixture` の unknown selector は hidden skip にせず error のままとし、pre-classification discovery failure を hidden に落とさない current L2 方針も維持する。bundle manifest、selector grammar の長期固定、path canonicalization policy、profile request の最終 field naming は未決定である。
- current L2 では、selection profile helper の上に small named profile catalog / preset table を置いてよい。catalog は `specs/examples/13-current-l2-profile-catalog.md` に列挙した human-friendly alias を既存 `SelectionRequest` / `SelectionProfile` へ解決するだけに留め、sidecar discovery、runtime/static-only classification、primitive selection を再実装しない。summary では `profile_name` に加えて `resolved_request` を読めてよい。現状の small alias set 規模では hard-coded table のままで十分であり、catalog を manifest や sidecar に外出しする案は comparison 対象に留める。ただし alias grammar の長期固定、catalog externalization をいつ再検討するか、path canonicalization policy は未決定である。
- ただし capability mismatch を将来同じ non-admissible taxonomy の formal subreason に昇格させる必要があるか、また `admit` 不成立や `lease` expiry を dedicated skip event として立てる必要があるかは **未決定** である。
- ただし fixture carrier の field 名、JSON などの serialization、detached trace representation、`request ref` / `reason kind` を独立 field にするか、predicate term carrier をどこまで細かくするか、surface punctuation と fixture field の対応を固定するかは **未決定** である。
- ただし evaluation state の field 名、snapshot ref の具体表現、ambient place frontier を `rollback_stack` と別 field に切り出すか、multi-request scheduler や event id を current L2 の最小 state に含めるか、trace / audit sink の detached serialization をどうするかは **未決定** である。
- ただし host plan schema の field 名、`schema_version` の長期固定、detached trace / audit serialization、production host interface との接続、未被覆 call を preflight で完全検出する richer coverage analysis を導入するかは **未決定** である。
- ただし bundle manifest を導入するか、current L2 の directory discovery rule を長期固定するか、sidecar を持つ static-only fixture をどこまで許すかは **未決定** である。
- ただし step signal 名、predicate oracle の richer API、effect oracle の host interface / serialization、success-side carrier の concrete layout、direct target の request-level `Reject` を将来 oracle carrier に入れる必要があるか、`Approximate` / `Compensate` を parser-free 最小 interpreter の oracle carrier にどの時点で織り込むかは **未決定** である。
- ただし `contract` を最終 reserved keyword にするか、`contract { require { ... } ensure { ... } }` のような block form を将来導入するか、`require` / `ensure` の final parser punctuation、option-local `admit` の最終 keyword / punctuation、option-local outcome metadata を別 marker で持つか、`or` / `not` / precedence table を含む predicate grammar、predicate block 内の blank line 許可、explicit separator token をどうするかは未決定である。
5. `try` は現時点では local rollback semantics を持つ primitive であり、representative examples では block form の `try { ... } fallback { ... }` を current L2 companion syntax 候補として使ってよい。
   - current `place` の入れ子がすでに rollback scope を与えるので、現時点の候補では `try` に追加の scope 指定句を要求しない。
   - `fallback { ... }` は直前の `try` に後置される explicit branch として読む。option chain の `fallback successor` と token を共有していても、構文形が異なる以上、同じ construct として固定しない。
   - ただしこれは examples 用の安定した block-form 候補であり、`try` / `fallback` の最終 keyword、punctuation、式形式、追加 sugar を固定する判断ではない。
6. `emit`、effect handler、structured event routing の正確な関係。
7. final coroutine model:
   - one-shot と multi-shot のどちらか
   - suspension restriction
   - patch interaction
   - lifetime crossing rule
8. 線形 continuation に関する既存研究を使って、制約付き multi-shot を支援するかどうか、またどう支援するか。
9. overlay と route rebinding の正確な構文と静的規則。overlay alias detail を含む。
10. routing semantics を Mir-0 に潰さずに、`place` 導入と cross-place transfer の正確な surface syntax と静的規則をどう定めるか。

## Mirrorea / routing

11. route proof representation の最終形。
12. route change と suspended task / coroutine の相互作用。
13. scaling のどこまでが Mirrorea に属し、どこからが external orchestration に属するか。
14. Mir-1 で定義された later cut vocabulary、特に `durable_cut` を Mirrorea がどの operational boundary から引き受けるか。現在の最小理解では、Mirrorea は durable commit guarantee、aggregate success / failure event の意味、scope rule 自体の意味づけは行わず、storage / replication / distributed finalization / member observation の収集と相関 / aggregate event と audit representation の実現責務だけを負う。さらに `all_of` aggregate evidence についても、Mir-1 が固定するのは full coverage 条件だけであり、per-place evidence reference の表現、圧縮、共有形式は Mirrorea の裁量に残す。`all_of` aggregate failure audit についても同様であり、justification source、missing coverage snapshot、`reason_ref` 相当の参照表現をどう ID 化・直列化・配置するかは Mirrorea の裁量に残す。snapshot 基準時点より前の中間状態を保存するか、複数 snapshot を保持するか、どの圧縮表現を使うかも Mirrorea の裁量に残す。scope rule profile についても、Mirrorea が独自 profile を意味語彙として追加することはせず、Mir-1 が意味づけた `all_of` などの profile について同じ aggregate success / failure 観測を保つ形で実現するだけに留まる。未決定なのは、その実現に必要な最小 protocol surface、evidence の具象形式、将来 profile 拡張時の相互運用条件である。

## PrismCascade

15. Mir / Mirrorea との正確な integration surface。
16. audio block semantics。
17. `fps=0` またはそれに相当する call-set semantics。
18. 色 / HDR / format negotiation model の正確な形。
19. remote execution granularity をどの程度まで細かくするか。

## Typed-Effects Wiring Platform

20. これを Mirrorea の subsystem と見るべきか、それとも sibling project と見るべきか。
21. effect declaration のための正確な contract language。
22. opaque または部分的にしか wrap されていない legacy effect の扱い。
23. state↔event graph visualization の scale limit と abstraction method。

## Reversed Library / アプリケーション層

24. virtual-reality social mode と Reversed Library mode の関係:
    - ひと続きの mode とするか、明示的 mode switch を置くか。
25. 知識分類戦略:
    - 人文学 / 科学 / 実践知
    - 既存の図書分類体系との関係
26. progression / capability unlock の設計。
27. アーキテクチャを歪めずに、play-theory evaluation perspective（例: Caillois-like axis）をどう取り入れるか。
28. synchronized web browsing モデル。
29. Mir-based GUI プログラミング基盤。
30. 以前の prototype diagram を最終的な Mir syntax / semantics にどう対応づけるか。

## order / handoff / syntax / modality 理論線

### current integration reading

`specs/examples/458`、`459`、`460`、`461`、`462` までの current reading では、
次は already source-backed な current first line として読める。

- typed / theorem / model-check line:
  checker attachment principal、notebook-first theorem、row-local model-check carrier first
- order / handoff line:
  cut family decomposition、relation decomposition principal、`authority_serial_transition_family` first
- syntax / modality line:
  semantic honesty axis、`lambda_circle_box` partial basis、guarded / MDTT / MTT / Fitch-style multimodal retained family

したがって current open question は
「何も分からない」ことではなく、
actual adoption / final contract / final surface / concrete tool binding を
どの mixed gate へ送るかに狭まっている。

31. cut family をどこまで明示的に分解するか。
   - actual adoption package として固定済みなのは、`atomic_cut` を current `place` の rollback frontier だけを確定する local finalizing cut nucleus に置き、`barrier` を ordering-only later family、`durable_cut` を commit-bearing / evidence-bearing later family に残す判断である。
   - `snapshot_cut` / `consistent_cut` のような名称は current repo の settled vocabulary ではなく、comparison candidate に留める。
   - したがって current open question は、cut family を分解するかどうかではなく、observation-only comparison candidate を final public wording へどこまで mirror するかに狭まっている。
32. order / visibility / witness family を relation decomposition としてどこまで docs 側で整理するか。
   - actual adoption package として、`program_order`、`dependency_order`、`publication_order`、`observation_order`、`witness_order`、`finalization_order`、`scoped_happens_before` を current working vocabulary に上げ、relation decomposition principal と読む current line は fixed 済みである。
   - C++ `memory_order_consume` / `std::kill_dependency` family は backend-aligned reference family として retained し、current source principal にはしない。
   - したがって current open question は、low-level exact surface を source principal にするかではなく、final public wording / emitted-artifact contract へ relation decomposition をどこまで mirror するかである。
33. thread / node parity をどの水準で書くべきか。
   - actual adoption package として、`thread と node は同じ causal language で書く` / `違いは lowering / evidence / transport / failure / durability / policy に残す` を current default wording に上げた。
   - `thread == node` の short-hand は current line で採らない。
   - したがって current open question は、この wording を final public companion / emitted-artifact docs へどこまで mirror するかである。
34. syntax と semantics honesty の coupling principle をどこまで明文化するか。
   - source-backed な floor として、companion notation は final parser grammar ではなく、explicit edge-row family を維持し、packed metadata row を current companion notation に採らないことは固定済みである。
   - actual recommendation として、`自然に書けるものは自然な挙動をするべき`、`理論的に破綻した動作は書けない / 書きにくいべき`、`semantic honesty > compactness`、5 evaluation axes を current syntax comparison principle に上げた。
   - surface narrowing note により、explicit edge-row / vertical continuation principal、`stage` / `after` / `witness` strong secondary candidate、authoritative-room `serial` sugar reserve まで narrowing は進んだ。
   - したがって current open question は、この principle を final public parser grammar へ落とすことではなく、principal / secondary / reserve の current narrowing をどの mixed gate まで helper-local に保つかである。
35. modal foundation の最終候補をどこへ置くか。
   - `lambda-circle-box` / `next` + `always` line は current partial basis candidate に留め、guarded lambda-calculus、MDTT/MTT cluster、Fitch-style multimodal family を stronger candidate family として retained する current recommendationは fixed 済みである。
   - syntax / modality current recommendation note により、5 軸 comparison、threshold-gated parallel keep、boundary-pressure trigger を含む current first line / retained alternatives / stop line は 1 本に再統合済みである。
   - 未決なのは、stronger family をどの mixed gate で further reduction するか、また final foundation adoption / final source marker をどの package で判断するかである。
36. order / handoff line の property-to-boundary matrix をどこまで current docs に昇格させるか。
   - source-backed な floor として、`core_static_checker` / `theorem_prover_boundary` / `protocol_verifier_boundary` / `runtime_policy_boundary` の 4-way split と property-to-boundary matrix 自体は current docs に昇格済みである。
   - actual adoption package 後の current open question は、matrix を theorem-first pilot / model-check second line / runtime-policy reserve へどう bridge するかであり、matrix 自体の existence ではない。
37. theory-lab line を mainline actualization と別 package としてどう運用するか。
   - current repo では `literature scout / formalizer / prototyper / falsifier / integrator` の role split、promotion rule、research package template は fixed 済みであり、`specs/examples/458...465` compare-floor、`466...469` actual-adoption floor、`470...474` helper-local actualization / narrowing floor、`478...483` theorem/model-check/shared-space threshold floor の分離も current docs に上がっている。
   - `specs/examples/481`、`482`、`483`、`484`、`485`、`486`、`487`、`488`、`489`、`490`、`491`、`492`、`493`、`494`、`495`、`496`、`497`、`498`、`499`、`500`、`501`、`502`、`503`、`504`、`505`、`506`、`507`、`508`、`509` により、`M1/M2/M3` threshold package、theorem/model-check/order-handoff/shared-space actual adoption package、theorem result-object preview actualization package、theorem proof-object schema/prover-brand coupled-later package、theorem result-object route actual-adoption package、theorem final-public-contract reopen-threshold package、model-check public-checker preview actualization package、model-check tool-brand/verifier-handoff coupled-later package、model-check public-checker-artifact/migration coupled-later package、model-check checker-artifact route actual-adoption package、model-check final-public-contract reopen-threshold package、shared-space/order-handoff coupled-later package、order-handoff source-wording/emitted-artifact coupled-later package、theorem result-object/payload public-contract coupled-later package、witness/provider public-schema coupled-later package、witness/provider route actual-adoption package、order-handoff source-wording route actual-adoption package、witness/provider schema-route actual-adoption package、witness/provider final-public-contract reopen-threshold package、theorem Lean-first non-production stub pilot actualization package、theorem repo-local artifact-conformance bridge package は close 済みであり、current principal self-driven queue は live package none ではなく representative trace-alignment / reserve surface reopen package に narrowed した。
   - 未決なのは、next reopen 順と、tiny non-production helper をどの mixed gate で mandatory にするかである。
38. typed-core の first attachment candidate と obligation owner をどこで固定するか。
   - actual adoption package として、current first typed attachment candidate は checker-adjacent semantic carrier principal に置き、source-visible structural marker family first、request / predicate / `try` cluster grouped reserve、`p06` corrected prototype bridge-floor evidenceまでを fixed 済みである。
   - stronger typed-surface promotion threshold framing も current docs に昇格済みである。
   - 未決なのは、`family_refs[]` の exact namespace、shared attachment shape、stronger typed surface を experimental adoption に送る mixed gate である。
39. semantic-core theorem pilot の first lemma order / evidence floor / review-discharge stop line をどこまで固定するか。
   - source-backed な floor として、current first theorem pilot line は `canonical_normalization_law`、`no_re_promotion`、`rollback_cut_non_interference` の順、notebook-first review-unit first、symbolic-ref-only evidence floor、abstract discharge-entry reserve に置く。
   - theorem-first external integration target は current default に上がっており、`p06`、preview alignment pre-floor、theorem discharge pre-floor、theorem-first pilot actualization、brand-neutral theorem binding preflight、theorem Lean-first non-production stub pilot actualization、theorem review-unit to Lean-stub repo-local artifact-conformance bridge、theorem Lean-stub representative trace-alignment bridge、theorem discharge actual-format probe、theorem discharge / public-theorem-contract threshold default、theorem contract shape threshold default、theorem transport/public-contract coupled later gate、theorem review-unit transport / notebook-contract actual adoption、theorem result-object preview / proof-object-schema reserve actualization、theorem result-object route actual adoption、theorem proof-object schema / prover-brand coupled later gate、theorem result-object / payload public-contract coupled later gate、theorem final public-contract reopen threshold、theorem public seam compression after local Lean-unavailable probe、theorem toolchain probe / reopen manifest、theorem actual Lean execution narrow probe、theorem actual Lean execution representative prototype widening は representative corpus で machine-check 済みである。
   - `specs/examples/497` により、final result-object candidate side と consumer-shaped payload public-contract candidate side を adjacent but distinct later gate として helper-local actualization 済みである。
   - `specs/examples/500` により、review-unit transport first / notebook-consumer object first / consumer-shaped payload preview keep / proof-object-schema-prover-brand later の repo-local actual adoption package まで helper-local actualization 済みである。
   - `specs/examples/514` により、theorem final public-contract threshold、Lean-stub representative bridge、actual Lean execution environment stop line を 1 本に束ねた residual compression package まで helper-local actualization 済みである。
   - `specs/examples/516` により、toolchain-ready / reopen-manifest floor まで helper-local actualization 済みである。
   - `specs/examples/518` により、representative static sample `e5` の actual Lean execution floor まで helper-local actualization 済みである。
   - `specs/examples/519` により、representative prototype trio `p06 / p07 / p08` の actual Lean execution floor まで helper-local actualization 済みである。
   - 未決なのは、final public theorem result object、consumer-shaped theorem payload public contract、theorem prover concrete brand、proof object public schema、actual Lean execution helper/CLI orchestration timing、broader theorem-relevant corpus widening timing、final public verifier contract、public checker migration timingである。
40. model-check projection grain と first property-family reserve をどこまで current docs に上げるか。
   - source-backed な floor として、current line は `formal_hook -> row-local machine-facing carrier -> emitted artifact wiring -> sample-facing summary` に置き、small-cluster semantic projection を row-local floor の次の reserve に残す。
   - theorem-first external integration target を current default に採るため、model-check line は current second line と読み、`specs/examples/478` により row-local property preview / request preflight / public-checker second reserve split まで helper-local actualization 済みである。
   - `specs/examples/480` により property-language probe / tool-seam probe / checker-boundary probe まで helper-local actualization 済みであり、`specs/examples/482` により row-local-property-preview first / small-cluster-projection second / brand-neutral request keep / public-checker-contract-later default まで helper-local actualization 済みである。
   - `specs/examples/488` により、row-local property route first / checker-boundary contract first / brand-neutral tool-binding reserve keep の repo-local actual adoption package まで helper-local actualization 済みである。
   - `specs/examples/492` により、consumer-shaped public-checker artifact preview only / verifier-handoff reserve keep / brand-neutral tool-binding reserve keep の repo-local actualization package まで helper-local actualization 済みである。
   - `specs/examples/495` により、public-checker artifact preview keep / verifier-handoff candidate only / tool-brand candidate only の coupled-later package まで helper-local actualization 済みである。
   - `specs/examples/498` により、consumer-shaped public checker artifact candidate side と actual public checker migration candidate side を adjacent but distinct later gate として helper-local actualization 済みである。
   - `specs/examples/501` により、row-local property route first / checker-boundary contract anchor / consumer-shaped checker-artifact candidate only / migration candidate adjacent keep の repo-local actual-adoption package まで helper-local actualization 済みである。
   - `specs/examples/507` により、property-language and tool-brand first / public-checker-artifact and migration second / verifier-handoff and runtime-policy-contract third / final public verifier contract fourth の reopen threshold まで helper-local actualization 済みである。
   - 未決なのは、concrete model-check tool binding、first settled property language、concrete model-check tool brand、final public checker artifact、actual public checker migration、actual emitted verifier handoff artifact、production checker/runtime-policy contract、final public verifier contract である。
42. witness/provider public-shape line の route-first cut をどこまで current docs に上げるか。
   - source-backed な floor として、claim/payload split first、witness/provider route non-collapse、repo-local emitted artifact refs first の public-shape actual adoption と、witness-schema candidate only / provider-receipt candidate only / combined public-contract candidate only の public-schema coupled-later gate は current docs に昇格済みである。
   - `specs/examples/502` により、witness/provider route first / public-schema candidate keep / combined public-contract later / final emitted-handoff contract adjacent keep の repo-local actual-adoption package まで helper-local actualization 済みである。
   - `specs/examples/504` により、witness-schema candidate keep + witness route first / provider-receipt candidate keep + provider route first / combined public-contract candidate keep / final emitted-handoff contract adjacent keep の repo-local actual-adoption package まで helper-local actualization 済みである。
   - `specs/examples/505` により、public-schema pair first / delegated provider attestation + combined provider+witness public contract second / final emitted-handoff contract third の reopen threshold まで helper-local actualization 済みである。
   - 未決なのは、final public witness schema、final public provider receipt schema、delegated provider attestation、combined provider+witness public contract、final emitted-handoff contract、exhaustive shared-space catalog である。
43. order-handoff line の principal source-wording route をどこまで current docs に上げるか。
   - source-backed な floor として、edge-row / vertical continuation principal、readable high-level relation vocabulary keep、stage-block secondary keep、thread/node same causal language keep の surface actual adoption と、source-wording candidate only / emitted-artifact schema candidate only の coupled-later gate は current docs に昇格済みである。
   - `specs/examples/503` により、principal source wording route first / emitted-artifact schema candidate keep / final source wording later の repo-local actual-adoption package まで helper-local actualization 済みである。
   - `specs/examples/515` により、order-handoff residual と witness/provider final public-contract residual を 1 本の helper-local matrix に圧縮する package まで actualization 済みである。
   - 未決なのは、final source-surface handoff wording、final emitted-artifact schema、final emitted-handoff contract、final public witness/provider contract、final modal foundation adoption、exhaustive shared-space catalog である。
41. order / handoff line の falsifier coverage をどこまで hardening するか。
   - source-backed な floor として、cut family comparison、relation decomposition、adequacy corpus、property-to-boundary matrix は already ある。
   - docs-first follow-up により、negative falsifier coverage と higher-level candidate reduction までは current docs に昇格済みである。current first candidate は `authority_serial_transition_family`、second candidate は `witness_aware_commit_family`、`event_tree_execution_view` は derived/debug candidate、low-level `memory_order` family は retained-later reference family に留める。
   - property-language bridge note により、kept/second candidate と boundary matrix を property-language bridge へ connect する current cut までは固まった。
   - emitted-artifact schema reserve note により、refs-only reserve schema を first cut にし、consumer-shaped schema と source-surface-first schema を still later に残す line までは固まった。
   - order/handoff source-surface wording reserve note により、snake_case relation family 名を principal wording に保ち、plain-language stage wording を explanation layer に重ね、low-level fence-like wording と room macro wording を still later に残す current cut までは固まった。
   - order / handoff current first-line integration note により、`p07/p08` corrected prototype、authority-handoff ladder、fairness/replay mixed-gate boundaryを含む current first line / retained alternatives / stop line / mixed gate は 1 本に再統合済みである。
   - `p07/p08` corrected prototype と current runner / CLI / compare-floor tests により、authoritative-room first default profile の runnable evidence も current docs に統合済みである。
   - `specs/examples/484` により、edge-row principal、stage-block secondary、repo-local emitted artifact refs first の threshold default まで helper-local actualization 済みである。
   - `specs/examples/490` により、edge-row principal / stage-block secondary keep / repo-local emitted artifact refs first の repo-local actual adoption package まで helper-local actualization 済みである。
   - `specs/examples/493` により、witness/provider public-contract side と emitted-handoff contract side を adjacent but distinct later gate として helper-local actualization 済みである。
   - `specs/examples/496` により、source-wording candidate side と emitted-artifact-schema candidate side を adjacent but distinct later gate として helper-local actualization 済みである。
   - 未決なのは、final source-surface handoff wording、final emitted artifact schema、final public witness/provider/artifact contract、authoritative-room `serial` sugar adoption、low-level exact source wording と final property-language bridge / final modal foundation をどう接続するかである。
42. shared-space fairness / replay line をどこで mixed gate と読むか。
   - source-backed な floor として、identity / admission / authority / room-profile-host bridge までは docs-first に進んでいる。
   - actual adoption package として、first default profile
     - `authority-ack`
     - `single room authority`
     - `authoritative serial transition`
     - `authority_rng`
     - late join visible history as past
     - stale reconnect fail then refresh
     - replay invalidation rather than silent merge
     - no distributed fairness theorem required in first completion line
     は current recommendation に上がった。
   - `specs/examples/476` により、`auditable_authority_witness` strengthening 自体は helper-local actualization floor まで current recommendation に上がった。current cut は、room profile に `fairness_claim = auditable_authority_witness` だけを残し、minimal witness core は audit / receipt side に `witness_kind` / `action_ref` / `draw_slot` / `draw_result` の 4-field bundle として置き、`draw_result` は final public scalar receipt ではなく symbolic binding ref に留める reading である。
   - `specs/examples/477` により、`delegated_rng_service` practical package 自体も helper-local actualization floor まで current recommendation に上がった。current cut は、`fairness_source = delegated_rng_service`、`fairness_claim = opaque_authority_trust`、authority-kept-commit、provider-draw-only、optional provider attachment を first practical line に置く reading である。
   - `specs/examples/483` により、claim/payload split first、repo-local emitted artifact refs first、optional attachment refs only、combined public contract later の public-shape threshold default まで helper-local actualization 済みである。
   - `specs/examples/489` により、witness route / provider route non-collapse、repo-local emitted artifact refs first、combined public contract later の repo-local actual adoption package まで helper-local actualization 済みである。
   - `specs/examples/499` により、witness-schema candidate side、provider-receipt candidate side、combined public-contract candidate side を adjacent but distinct later gate として helper-local actualization 済みである。
   - `specs/examples/515` により、public-schema pair、delegated attestation + combined contract、final emitted-handoff residual の compression matrix まで helper-local actualization 済みである。
   - 未決なのは、final public provider receipt schema、delegated provider attestation、final public witness schema、combined provider+witness public contract、final emitted handoff contract、final fairness policy、final replay invalidation protocol taxonomy、exhaustive room-profile catalog である。
43. public operational CLI installed-binary promotion と packaging success criteria をどこで mixed gate と読むか。
   - source-backed な floor として、runtime-led thin facade、current-L2 scoped Rust shell actualization、public operational CLI concrete shell naming / actualization は fixed 済みである。
   - installed-binary / packaging success-criteria mixed-gate boundary note により、current shell actualization と installed-binary promotion を分ける current line は fixed 済みである。
   - user-authorized default により、repo-local runnable CLI + tests + emitted artifacts + reproducible compare floor は near-end success criteria に上がった。
   - 未決なのは、installed binary adoption、final CLI hierarchy、FFI / engine adapter / host integration target、final public parser / checker / runtime packaging success criteriaである。
44. near-end closeout をどこまで queue close と書いてよいか。
   - near-end closeout after actual adoption defaults note、helper-local actualization notes、`specs/examples/481`、`482`、`483`、`484`、`485`、`486`、`487`、`488`、`489`、`490`、`491`、`492`、`493`、`494`、`495`、`496`、`497`、`498`、`499`、`500`、`501`、`502`、`503`、`504`、`505`、`506`、`507` により、actual adoption / actualization / threshold package `466...507` close 後の current reading は theorem / model-check / order-handoff / shared-space actual adoption close と theorem result-object preview / theorem result-object actual adoption / theorem proof-object schema-prover-brand coupled-later / theorem result-object-payload public-contract coupled-later / theorem final-public-contract reopen-threshold close / model-check public-checker preview / model-check tool-brand-verifier-handoff coupled-later / model-check public-checker-artifact-migration coupled-later actualization / model-check checker-artifact-route actual-adoption close / model-check final-public-contract reopen-threshold close と witness/provider public-contract / emitted-contract coupled-later close と witness/provider public-schema coupled-later close と witness/provider route actual-adoption close と order-handoff source-wording / emitted-artifact coupled-later close と order-handoff source-wording route actual-adoption close と witness/provider schema-route actual-adoption close と witness/provider final-public-contract reopen-threshold close、queue を強く narrow に読める地点まで進んだ。
   - ただし、これは theory solved や final adoption を意味しない。`specs/examples/520` と `531` により、remaining work は mixed gate / reserve integration / residual user-spec gate だけでなく、layered strong typing / IFC first-fragment を保った residual mixed-gate compression package chain として読み直す方が正確である。
   - theorem public seam compression、model-check public seam compression、order-handoff / witness-provider public seam compression、representative Lean sample set actual Lean execution により、execution floor は reached 済みである。未決なのは、その後段 closeout queue を actual Lean execution hardening だけに narrow 化しすぎず、どこまで source-backed package として保つかである。
   - 未決なのは、final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema と final public checker artifact / concrete tool seam と final public witness/provider/artifact contract をどの順で reopen するか、および strong typing / IFC / formal skeleton package をどこで helper-local implementation floor へ接続するかである。
45. principal theory spine をどこで final adopted calculus と区別して扱うか。
   - `specs/examples/475` により、current theory-lab direction は multimodal dependent core を principal theory spine に置き、`lambda-circle-box` を partial basis candidate に留めつつ、guarded / MDTT / MTT / Fitch-style multimodal family を stronger family として retained する reading に上がった。
   - current recommendation は final calculus adoption ではなく、typed / theorem / model-check / order-handoff を一段深い共通 spine で読む docs-first integration judgment である。
   - 未決なのは、この spine を final adopted calculus、final source marker、final parser grammar のどれとも混線させずに、どの mixed gate で further reduction するかである。
46. layered theory stack と compatibility metatheory をどこまで current package に上げるか。
   - `specs/examples/475` により、multimodal dependent core、linear capability/ownership、effect rows、decidable refinement / contract、information-flow / label、theorem/model-check bridge の layered stack と、layer-compatibility / obligation export / weakening / erasure の compatibility metatheory package は current recommendation に上がった。
   - ただし current package は exact judgment syntax、full elaborator、cross-layer serialization、public export schema を final にしたわけではない。
   - 未決なのは、compatibility manifest / export floor / obligation handoff shape をどの mixed gate で helper-local から public に近づけるかである。
47. Lean-first proof roadmap をどこまで current package judgment と読むか。
   - `specs/examples/475` により、proof formalization は Lean-first を current recommendation に置き、Rocq/Coq + Iris line は runtime concurrency / separation-logic pressure が強まった場合の fallback family として retained する reading に上がった。
   - current package で fixed したのは、semantic-core relation library first、repo-local review-unit/Lean-stub artifact-conformance bridge second、room/fabric concurrency later という staged roadmap であって、concrete production prover binding や final public proof artifact contract ではない。
   - 未決なのは、prototype-wide exhaustive trace alignment、actual theorem discharge transport、proof object public contract、cross-tool public artifact-conformance contract、fallback trigger をどこで concretize するかである。
48. final-layer closeout queue をどこまで actual package として戻すか。
   - `specs/examples/520` と `531` により、current queue は
     - layered strong typing / IFC first-fragment
     - residual theorem/model-check mixed-gate compression
     - residual order-handoff/shared-space mixed-gate compression
     を含む reopened self-driven package chain として読む current default に上がった。`specs/examples/532` により、そこから theorem/model-check residual mixed-gate compression は close 済みであり、current live queue は residual order-handoff/shared-space mixed-gate compression と later true user-spec residual に narrow に移ったと読む方が正確である。
   - `specs/examples/521` により、Lean formal skeleton / proof obligations first slice は `samples/lean/foundations/CurrentL2ProofSkeleton.lean` と representative Lean sample set committed Lean corpus に actualize 済みであり、generated current-L2 theorem stub は `sorry` を含む artifact well-formedness / bridge-alignment evidence、`foundations/` は actual small proof fragment として読む current floor が fixed した。
   - `specs/examples/522` により、Package 56 の IFC side も `samples/lean/foundations/CurrentL2IfcSecretExamples.lean` まで進み、secret-key valid/invalid と explicit authority declassification の concrete example を helper-local foundation として読む current floor が追加で fixed した。
   - `specs/examples/523` により、`p10 / p11` source-side explicit authority pair も corrected prototype / verifier preview / model-check projection / theorem Lean actualization representative set として actualize 済みである。
   - `specs/examples/524` により、`p12` source-side label-flow negative も corrected prototype / verifier preview / model-check projection / theorem Lean actualization representative set として actualize 済みであり、Package 56 の first actual adoption package は close してよい。
  - `specs/examples/525` により、`p09` delegated RNG provider placement も representative Lean sample set carry-over として actualize 済みであり、`specs/examples/528` により `p13 / p14` late-join visibility negative pair も representative Lean sample set / committed Lean corpus へ carry over 済みである。Package 58 の broader coverage は helper-local representative widening から始めてよい。
   - `specs/examples/526` により、`run-source-sample` helper summary も `surface_preview` を通して principal companion / stage-block secondary / serial reserve を inspectable にでき、Package 58 helper/CLI hardening は actual package として進めてよい。
   - `specs/examples/527` により、`p13 / p14` late-join visibility drift pair も helper-local static stop として actualize 済みであり、Package 58 の order-handoff negative corpus tightening は first pair を close してよい。
   - `specs/examples/529` により、source-side IFC trio `p10 / p11 / p12` も sample-local `typed_checker_hint_preview` として actualize 済みであり、`family_refs[] + coverage_state` mirror は current checker-adjacent helper-local floor に上げてよい。
   - `specs/examples/530` により、theorem result-object preview と model-check public-checker preview も `run-source-sample` helper summary へ widen 済みであり、Package 58 の remaining diagnostics widening は close してよい。`specs/examples/531` により、Package 59 closeout sync も close してよく、next line は Package 60/61 residual mixed-gate compression に移す。
   - `specs/examples/532` により、Package 60 の theorem/model-check residual mixed-gate compression も close してよく、theorem/model-check final public-contract reopen threshold は `run-source-sample` helper summary で inspectable に保てる。current live queue は Package 61 order-handoff/shared-space residual mixed-gate compression へさらに narrow に移してよい。
   - `specs/examples/533` により、Package 61 の order-handoff/shared-space residual mixed-gate compression も close してよく、order-handoff source wording residual / emitted-artifact residual / witness-provider public-seam residual は `run-source-sample` helper summary で inspectable に保てる。
   - `specs/examples/534` により、Package 62 の typed/IFC helper-to-checker ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の actual checker payload family threshold は `run-source-sample` helper summary で inspectable に保てる。
   - `specs/examples/535` により、Package 63 の checker payload row-family ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の checker payload row-family threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/536` により、Package 64 の checker payload row-detail ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の checker payload row-detail threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/537` により、Package 65 の checker payload row-body ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の checker payload row-body threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/538` により、Package 66 の checker payload supported-kind-summary ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の checker payload supported-kind summary threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/539` により、Package 67 の checker payload public-schema sketch ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の checker payload public-schema sketch threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/540` により、Package 68 の public-checker-api sketch ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の public checker API sketch threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/541` により、Package 69 の public-checker entry-criteria ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の public-checker entry criteria threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/542` により、Package 70 の public-checker command-surface ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の public-checker command-surface threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/543` により、Package 71 の shared-output-contract ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の shared-output-contract threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/544` により、Package 72 の public-checker-boundary ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の public-checker-boundary threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/545` により、Package 73 の verifier-handoff-surface ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の verifier-handoff-surface threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/546` により、Package 74 の minimal-parser-subset-freeze ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の minimal-parser-subset-freeze threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/547` により、Package 75 の parser-to-checker-reconnect-freeze ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の parser-to-checker-reconnect-freeze threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/548` により、Package 76 の phase1-semantics-closeout ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の phase1-semantics-closeout threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/549` により、Package 77 の phase2-parser-free-poc-closeout ratchet も first cut は close してよく、`p10 / p11 / p12` source-side IFC trio の phase2-parser-free-poc-closeout threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/550` により、Package 78 の phase4-shared-space-self-driven-closeout ratchet も first cut は close してよく、`p07 / p08 / p09` source-side shared-space trio の phase4 shared-space self-driven closeout threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/551` により、Package 79 の phase5-proof-protocol-runtime-policy-handoff-closeout ratchet も first cut は close してよく、`p07 / p08 / p09` source-side shared-space trio の phase5 handoff closeout threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/552` により、Package 80 の phase6-actual-parser-ast-carrier-first-tranche ratchet も first cut は close してよく、`p07 / p08 / p09` source-side shared-space trio の phase6 parser / AST carrier first tranche threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/553` により、Package 81 の phase6-actual-checker-runtime-skeleton-first-tranche ratchet も first cut は close してよく、`p07 / p08 / p09` source-side shared-space trio の phase6 checker/runtime first tranche threshold は `run-source-sample` helper summary で inspectable に保てる。`specs/examples/554` により、Package 82 の phase6-compile-ready-verification-and-formal-hook ratchet も first cut は close してよく、`p07 / p08 / p09` source-side shared-space trio の phase6 compile-ready verification / formal hook threshold は `run-source-sample` helper summary で inspectable に保てる。current live queue は Package 83 phase6-next-reopen-sequencing ratchet と later mixed/user-spec residual へ移してよい。
   - これは final public language completion を意味せず、representative Lean sample set actual Lean execution floor の後段 closeout debt をどこで処理するかを整理した current package である。
   - 未決なのは、secret-key / IFC valid-invalid / authority / label-flow の source-side trioを beyond first checker fragment へどこまで広げるか、sample-local helper preview を public checker entry criteria / actual command surface へどこまで ratchet するか、final public theorem/model-check/order-handoff/shared-space seam をどの reopen order で later mixed gate に保つかである。
