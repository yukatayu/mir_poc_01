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

31. cut family をどこまで明示的に分解するか。
   - source-backed な floor として固定済みなのは、`atomic_cut` が current `place` の rollback frontier だけを確定する local finalizing cut であり、`barrier` と `durable_cut` は later cut family に残すという判断である。
   - ただし、**global observation / snapshot-only cut** を、`atomic_cut` や `durable_cut` と別 family として comparison vocabulary に残すべきかは未決定である。
   - `snapshot_cut` / `consistent_cut` のような名称は current repo の settled vocabulary ではなく、comparison candidate に留める。
   - 未決なのは、observation meaning、rollback interaction、durability / evidence burden、cross-place aggregation burden、verifier handoff friendlinessの観点から、local finalization / global snapshot / ordering-only barrier / commit-bearing durable cut をどう整理するのが最も誤読に強いかである。
32. order / visibility / witness family を relation decomposition としてどこまで docs 側で整理するか。
   - C++ `memory_order_consume` / `std::kill_dependency` family は reference family として有用だが、current source language の final surface に直輸入する判断はしていない。
   - current theory line では、少なくとも `program_order`、`dependency_order`、`publication_order`、`observation_order`、`witness_order`、`finalization_order`、`scoped_happens_before` のような relation decomposition が比較候補になる。
   - 未決なのは、これらの relation 名をどこまで docs-first working vocabulary に昇格させるか、また shared-memory lowering と distributed handoff lowering の両方へ自然に接続できる分解が何かである。
33. thread / node parity をどの水準で書くべきか。
   - source-backed な floor として固定済みなのは、`place` が deployment artifact ではなく semantic locus であり、routing / transport / distributed realization は Mir-0 の外にあるという点である。
   - ただし、「thread も node も同じ causal language で扱う」という line を repo のどこまで current wording に上げるかは未決定である。
   - working hypothesis としては、thread と node を同じ source-level causal language で扱いつつ、差分は placement / transport / witness / durability / failure policy / fairness / authority に残すのが自然である。
34. syntax と semantics honesty の coupling principle をどこまで明文化するか。
   - source-backed な floor として、companion notation は final parser grammar ではなく、explicit edge-row family を維持し、packed metadata row を current companion notation に採らないことは固定済みである。
   - ただし、「自然に書けるものは自然な挙動をするべき」「理論的に破綻した動作は書けない / 書きにくいべき」という設計原則を、syntax comparison の規準としてどこまで formal に mirror するかは未決定である。
   - 少なくとも、semantic honesty、checker legibility、modal adequacy、misreading resistance を syntax comparison の主要軸に置くかは open question である。
35. modal foundation の最終候補をどこへ置くか。
   - `lambda-circle-box` / `next` + `always` line は、stage / later / always と time-ordered normalization の partial basis として有望である。
   - docs-first follow-up により、guarded lambda-calculus と MDTT/MTT cluster を stronger candidate family として並行比較し、`lambda-circle-box` を partial basis に留める current line までは固まった。
   - modal promotion-threshold note により、promotion pressure は multi-axis internalization pressure が現れた時点に置く current line まで固まった。
   - 未決なのは、guarded line と MDTT/MTT line をどの時点でさらに reduction するか、また final foundation adoption をどの threshold で判断するかである。
36. order / handoff line の property-to-boundary matrix をどこまで current docs に昇格させるか。
   - source-backed な floor として、`core_static_checker` / `theorem_prover_boundary` / `protocol_verifier_boundary` / `runtime_policy_boundary` の 4-way split は fixed 済みである。
   - adequacy corpus と property-to-boundary matrix 自体は current docs に昇格済みであり、negative falsifier coverage も first hardening まで進んだ。
   - 未決なのは、これをどの property-language bridge で theorem / protocol / runtime policy へ connect するかである。
37. theory-lab line を mainline actualization と別 package としてどう運用するか。
   - current repo では `literature scout / formalizer / prototyper / falsifier / integrator` の role split、promotion rule、research package template までは docs-first に fixed 済みである。
   - 未決なのは、tiny non-production prototype をどの line で mandatory にするか、また comparison candidate を decision register に上げる threshold をどの程度厳密に取るかである。
38. typed-core の first attachment candidate と obligation owner をどこで固定するか。
   - source-backed な floor として、current first typed attachment candidate は source syntax ではなく checker-adjacent semantic carrier 側に置くのが自然である。
   - theorem/model-check consumer artifact は current first typed source ではなく derived attachment と読む。
   - docs-first follow-up により、first source-visible typed-surface comparison は existing structural marker family を first candidate に置き、dedicated attachment block と unified obligation block を later comparison に残す current line まで固まった。
   - docs-first migration note により、principal source を checker attachment に残し、source-visible marker を mirror、handoff row を derived reserve row に留める current cut までは固まった。
   - request / predicate / `try` cluster typed-surface reserve note により、request / predicate / `try` cluster を grouped reserve cue に留める current cut までは固まった。
   - 未決なのは、`family_refs[]` の exact namespace をどうするか、capability / lineage / request / predicate / `try` cluster をどこまで同じ attachment shape に寄せるかである。
39. semantic-core theorem pilot の first lemma order / evidence floor / review-discharge stop line をどこまで固定するか。
   - source-backed な floor として、current first theorem pilot line は `canonical_normalization_law`、`no_re_promotion`、`rollback_cut_non_interference` の順で読むのが自然である。
   - また current first concrete consumer は notebook-first line に置き、tool-neutral formal hook と row-local review unit を entry に保つ。
   - docs-first follow-up により、lemma wording floor、admissible evidence floor、review artifact と theorem discharge の stop line までは固まった。
   - proof artifact / bridge stop-line refresh により、formal hook / review unit / bridge sketch / theorem discharge を collapse しない current cut と、human review checklist / walkthrough pressure first の threshold までは固まった。
   - admissible evidence contraction note により、admissible theorem evidence を symbolic refs only に contraction する current cut までは固まった。
   - 未決なのは、notebook-consumer threshold と discharge reserve をどこで切るか、concrete theorem prover brand、proof object public contract、actual discharge transport、public checker migration timingである。
40. model-check projection grain と first property-family reserve をどこまで current docs に上げるか。
   - source-backed な floor として、current line は `formal_hook -> row-local machine-facing carrier -> emitted artifact wiring -> sample-facing summary` までである。
   - current next reserve としては、small-cluster semantic projection を row-local floor の次に置き、room protocol / fairness / replay / provider receipt family は order/handoff 側 reserve に残すのが自然である。
   - docs-first follow-up により、row-local carrier から small-cluster semantic projection への bridge grain と bridge bundle floor までは固まった。
   - sample-visible property summary wording により、sample-facing summary が何を言ってよく、何を order/handoff reserve 側へ残すかの current cut までは固まった。
   - tool-binding stop-line refresh により、explicit non-goal を current cut にし、concrete tool binding を still later に残す line までは固まった。
   - 未決なのは、small-cluster projection keep/drop の exact cut、concrete model-check tool binding、first settled property language、production checker/runtime-policy contract である。
41. order / handoff line の falsifier coverage をどこまで hardening するか。
   - source-backed な floor として、cut family comparison、relation decomposition、adequacy corpus、property-to-boundary matrix は already ある。
   - docs-first follow-up により、negative falsifier coverage と higher-level candidate reduction までは current docs に昇格済みである。current first candidate は `authority_serial_transition_family`、second candidate は `witness_aware_commit_family`、`event_tree_execution_view` は derived/debug candidate、low-level `memory_order` family は retained-later reference family に留める。
   - property-language bridge note により、kept/second candidate と boundary matrix を property-language bridge へ connect する current cut までは固まった。
   - emitted-artifact schema reserve note により、refs-only reserve schema を first cut にし、consumer-shaped schema と source-surface-first schema を still later に残す line までは固まった。
   - 未決なのは、final property language、final source-surface handoff wording、final emitted artifact schema、concrete theorem / protocol tool binding とどう接続するかである。
