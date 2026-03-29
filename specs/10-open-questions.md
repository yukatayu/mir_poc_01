# 10 — Open Questions

This file intentionally records unresolved matters.

## Mir semantics

1. Final formalization of preference chains / fallback normalization beyond Mir-0 primitive fallback.
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
   - cross-place 文脈の scope rule は、その aggregate attempt に対してどの member local observations / persistence evidence が aggregate success / failure 判定に数えられるかを定める明示的 rule を意味する。
   - cross-place 文脈の `prefix` は、単一の merged / total prefix ではなく、participating `place` ごとの local prefix の有限対応を意味する。single-place 文脈では `prefix` は従来どおり local prefix を指す。
   - single-place な success / failure event は member-local observation として保持される。cross-place `durable_cut` 自体を観測するには、それらを束ねる aggregate success / failure event が必要であり、これは Mir-1 の意味語彙として扱う。
   - cross-place successful `durable_cut` は、aggregate attempt に対して明示された scope rule が required member local observations / persistence evidence によって満たされたことを示す aggregate success event が立ったときにだけ観測される。
   - cross-place failed `durable_cut` は、同じ aggregate attempt について aggregate success event より前に aggregate failure event が明示されたときに観測される。
   - cross-place の audit surface には、少なくとも aggregate attempt、participating `place` の集合、対応する local attempts / local prefixes、scope rule の参照、aggregate outcome、どの member observations / evidence が outcome 判定に数えられたかを説明できる trace が残らなければならない。
   - Mir-1 が現時点で必須とする cross-place scope rule profile は `all_of` のみである。`all_of` は、aggregate attempt に含まれる participating `place` のすべてについて、対応する required local observations / persistence evidence が success 判定に数えられることを要求する。
   - `all_of` は single-place observation を conjunction として束ねる最小 profile であり、participating `place` の一部だけで aggregate success を許す追加意味を持ち込まない。
   - `all_of` における aggregate evidence の最小要件は、participating `place` の全員について、対応する local attempt / local prefix に対する success 側の local observation と、それを支える persistence evidence が outcome 判定に数えられる形で揃っていることである。Mir-1 が要求するのは、この full coverage が成立していることだけである。
   - `all_of` の event surface に Mir-1 が残す最小情報は、aggregate attempt に対して full coverage が成立したことを示す aggregate success event、またはそれより前に failure outcome が明示されたことを示す aggregate failure event である。各 `place` ごとの evidence payload や具象表現を event surface で inline することは要求しない。
   - `all_of` の audit surface には、participating `place` ごとに対応する local attempt / local prefix、counted local observation の有無、supporting persistence evidence が outcome 判定に数えられたか、を追跡できる trace が残らなければならない。ack / signature / proof / log record などの具象形式と、その表現方法は Mirrorea の裁量に残す。
   - `all_of` で full coverage が成立していないことは aggregate success を正当化しないが、それだけで aggregate failure を確定しない。full coverage 不足だけでは、まだ evidence 収集中なのか、同じ aggregate attempt が失敗として確定したのかを区別できないためである。
   - `all_of` では、required member の一つに対応する local attempt / local prefix で explicit local failure event が立てば、その aggregate attempt は full coverage を達成できない。これは aggregate failure event を正当化する十分条件になるが、aggregate failure が outcome surface で観測されたことになるのは、従来どおり explicit aggregate failure event が立ったときだけである。
   - `all_of` では、required member local failure がまだ出ていない場合でも、同じ aggregate attempt が aggregate success event より前に failed outcome として明示的に閉じられたなら、aggregate failure event を出してよい。Mir-1 が意味として要求するのは、その failed closure が当該 aggregate attempt の success を打ち切ることだけであり、timeout-like budget、policy cancellation、retry exhaustion、route / transport closure などの具体理由は Mirrorea の operational policy / audit に残す。
   - `all_of` の audit surface は aggregate failure に対して、少なくとも aggregate attempt、participating / required member の集合、各 required member に対応する local attempt / local prefix、aggregate failure を正当化した source、および failure 観測時点または closure 時点の missing coverage 状態を説明できなければならない。
   - justification source が required member local failure である場合、audit はその member と対応する explicit local failure event を参照できなければならない。
   - justification source が explicit failed closure である場合、audit はその aggregate attempt を failed outcome として閉じた act を参照できなければならない。timeout-like budget、policy cancellation、retry exhaustion などの具体理由分類は、その act に付随する Mirrorea 側表現に残し、Mir-1 では固定しない。
   - missing coverage snapshot の最小基準時点は、same aggregate attempt が aggregate failure outcome として明示的に観測された決定点である。Mir-1 は少なくともこの一時点の snapshot を要求し、それ以前の途中経過や複数時点比較までは要求しない。
   - justification source が explicit failed closure である場合、closure がそのまま aggregate failure outcome を明示するなら、その closure 時点が missing coverage snapshot の基準時点になる。required member local failure が justification source の場合は、member-local failure 自体ではなく aggregate failure 観測時点を基準時点とする。
   - missing coverage を説明する最小要件は、その基準時点で、どの required member が counted success-side coverage を持っていたか、どの member が local failure により coverage 不可能になっていたか、どの member が未充足のまま残っていたかを区別できることである。
   - したがって Mir-1 が要求する最小粒度は per-required-member 粒度である。各 member の state は、対応する local attempt / local prefix 参照と結びついた上で、covered、failure により不可能、未充足の少なくとも三通りを区別できれば足りる。event-by-event の履歴や per-prefix の追加 snapshot を Mir-1 で義務づけない。
   - `reason_ref` のような field 名や独立語彙は Mir-1 では固定しない。Mir-1 が要求するのは、justification source と missing coverage 状態が参照可能であることだけであり、その ID 形式、serialization、audit layout は Mirrorea の裁量に残す。
   - `quorum-like` profile は将来拡張候補としては残すが、現時点の Mir-1 には含めない。導入には、participating `place` の一部だけで十分とする条件や、そのとき aggregate evidence / audit に何を要求するかを追加で意味づける必要があるためである。
   - `implementation-defined` は Mir-1 の profile 語彙にはしない。Mirrorea が profile 実現で裁量を持てるのは、Mir-1 がすでに名前と意味を与えた profile について、同じ aggregate success / failure 観測を保つ形で実現する範囲に限る。
   - local durable failure と distributed durable failure は、現時点では別 failure class に分けない。どちらも同じ durable guarantee failure の実現上の差分とみなす。
   - distributed finalization は最小意味そのものではなく、複数 `place` / 実ノードにまたがる場合の operational realization 側に送る。
   - 未決定なのは、`quorum-like` を将来の Mir-1 拡張 profile として本当に採用するか、`all_of` の per-place evidence 参照を event surface にどこまで露出するか、local evidence と distributed outcome の関係をどこまで Mir-1 で語るか、timeout-like closure や policy-driven closure を Mir-1 の独立語彙に引き上げる必要が将来あるか、`reason_ref` 相当の参照語彙を Mir-1 で標準化する必要があるか、snapshot の複数時点比較を Mir-1 で語る必要があるか、audit の粒度を per-place / per-attempt / per-prefix のどこまで固定するか、将来 profile family が増えたときに aggregate evidence / aggregate failure justification / audit minimum requirements / snapshot semantics の意味差分をどこまで先に固定するか、`Approximate` を許す contract 条件である。
3. `barrier` は Mir-0 に含めない。Mir-1 に残すとすれば ordering primitive 候補であり、commit-like primitive として扱うべきではない。未決定なのは、独立語彙として残す価値が本当にあるかどうかである。
4. `perform` は現時点では最小 effect request operation の説明用記法であり、Mir-0 の規範的な表面構文として採用するかどうかは未決定である。
5. Exact relationship between `emit`, effect handlers, and structured event routing.
6. Final coroutine model:
   - one-shot vs multi-shot,
   - suspension restrictions,
   - patch interaction,
   - lifetime crossing rules.
7. Whether and how to support constrained multi-shot using existing research on linear continuations.
8. Exact syntax and static rules for overlays and route rebinding, including overlay alias details.
9. Exact surface syntax and static rules for `place` introduction and cross-place transfer without collapsing routing semantics into Mir-0.

## Mirrorea / routing

10. Final route proof representation.
11. Interaction between route changes and suspended tasks/coroutines.
12. How much of scaling belongs to Mirrorea vs external orchestration.
13. Mir-1 で定義された later cut vocabulary, especially `durable_cut`, を Mirrorea がどの operational boundary から引き受けるか。現在の最小理解では、Mirrorea は durable commit guarantee、aggregate success / failure event の意味、scope rule 自体の意味づけは行わず、storage / replication / distributed finalization / member observation の収集と相関 / aggregate event と audit representation の実現責務だけを負う。さらに `all_of` aggregate evidence についても、Mir-1 が固定するのは full coverage 条件だけであり、per-place evidence reference の表現、圧縮、共有形式は Mirrorea の裁量に残す。`all_of` aggregate failure audit についても同様であり、justification source、missing coverage snapshot、`reason_ref` 相当の参照表現をどう ID 化・直列化・配置するかは Mirrorea の裁量に残す。snapshot 基準時点より前の中間状態を保存するか、複数 snapshot を保持するか、どの圧縮表現を使うかも Mirrorea の裁量に残す。scope rule profile についても、Mirrorea が独自 profile を意味語彙として追加することはせず、Mir-1 が意味づけた `all_of` などの profile について同じ aggregate success / failure 観測を保つ形で実現するだけに留まる。未決定なのは、その実現に必要な最小 protocol surface、evidence の具象形式、将来 profile 拡張時の相互運用条件である。

## PrismCascade

14. Precise integration surface with Mir / Mirrorea.
15. Audio block semantics.
16. `fps=0` or equivalent call-set semantics.
17. Exact color/HDR/format negotiation model.
18. Degree of remote execution granularity.

## Typed-Effects Wiring Platform

19. Whether it should be a subsystem of Mirrorea or a sibling project.
20. Exact contract language for effect declarations.
21. Treatment of opaque or partially wrapped legacy effects.
22. Scale limits and abstraction methods for state↔event graph visualization.

## Reversed Library / application layer

23. Relationship between virtual-reality social mode and Reversed Library mode:
    - one continuous mode or explicit mode switch?
24. Knowledge classification strategy:
    - humanities / sciences / practical knowledge,
    - relation to existing library classification systems.
25. Progression / capability unlock design.
26. How to incorporate play-theory evaluation perspectives (for example Caillois-like axes) without distorting the architecture.
27. Synchronized web browsing model.
28. Mir-based GUI-programming substrate.
29. Mapping of earlier prototype diagrams to final Mir syntax and semantics.
