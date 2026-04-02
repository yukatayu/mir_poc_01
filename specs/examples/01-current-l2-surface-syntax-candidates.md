# examples/01 — current L2 surface syntax candidates

この文書は、`specs/examples/00-representative-mir-programs.md` で使う**current L2 の最小 surface syntax 候補**をまとめる補助文書である。
ここで整理するのは、`perform`、option chain 参照、`try` / `fallback`、`require` / `ensure` clause、statement separator / block nesting に関する**書き方の候補**だけであり、Mir-0 / Mir-1 / Mirrorea の境界や、canonical normalization law、rejection phase、static evidence floor、underdeclared handling 自体は変更しない。

## この文書の位置づけ

- current L2 の representative examples を、過度に ad-hoc ではない一定の書式で読めるようにする。
- parser-ready な最終 grammar や reserved keyword 集合は固定しない。
- `perform`、option declaration、chain 参照、`try` / `fallback`、statement-local clause、block nesting のうち、current L2 で最も摩擦の少ない最小表記だけを候補として置く。
- parser なしの machine-readable carrier が必要な場合は、syntax ではなく意味側の構造へ正規化した `specs/examples/02-current-l2-ast-fixture-schema.md` を参照する。

## current L2 の最小候補

### 1. direct target に対する `perform`

direct な effect request は、次の形を current L2 の最小候補とする。

```text
perform update_authority on profile_authority
  require write
  ensure owner_is(session_user)
```

- `perform` は依然として説明用記法であり、最終 reserved keyword ではない。
- `on <target>` は direct な `declared access target` を表す候補である。
- current L2 examples では、head statement とそれに続く clause suite 全体を 1 つの statement cluster として読む。

### 2. `contract` の surface policy と `require` / `ensure` の最小 clause form

current L2 では、`contract` を独立した block keyword に上げず、`require` / `ensure` を直前の `perform` に付く statement-local clause として書く形を最小候補とする。
比較対象としては、`contract { ... }` block form を optional sugar として許す案もあるが、current L2 の companion notation では採らない。
理由は、representative examples がすでに `place` / `try` / `fallback` を brace-delimited block として使っており、`contract` まで block head にすると、statement-local clause attachment より block nesting の見た目が前面に出て読みがぶれやすくなるためである。

```text
perform update_authority on profile_authority
  require write
  ensure:
    owner_is(session_user)
```

- `contract` はここでは semantic role の名前であり、representative examples に mandatory な surface keyword とはしない。
- single-line clause は `require pred` / `ensure pred` と書き、keyword の後ろに空白 1 つだけを置く。current L2 では追加の trailing punctuation を要求しない。
- multi-line predicate が必要な場合だけ、header line を `require:` / `ensure:` と書き、その直下に 1 段深い predicate block を置く。
- predicate block は、複数 clause の列挙ではなく**1 つの predicate を複数行に折り返したもの**として読む。predicate block の内部では blank line を使わない。
- current L2 では clause の既定順を `require` の後に `ensure` とする。ただしこれは examples を読みやすく保つための companion 規則であり、final grammar の順序制約を固定するものではない。
- current L2 の representative examples では、statement-local clause を持てる head は `perform` だけに限定する。`atomic_cut`、`option`、`chain`、`place`、`try`、`fallback` に clause をぶら下げない。
- `require` と `ensure` の間に blank line は入れず、同じ clause suite として隣接させるのを既定とする。
- `contract { require { ... } ensure { ... } }` のような block form は **未決定** の future syntax 候補として残すが、current L2 companion notation の optional sugar としては採用しない。

#### predicate sublanguage の最小候補

current L2 の companion notation では、`require` / `ensure` にぶら下がる predicate は、representative examples を安定して読めるだけの**最小 predicate 断片**に留める。

```text
require write

ensure owner_is(session_user)

require:
  (
    owner_is(session_user)
    and well_formed(profile_draft)
  )
```

- atomic predicate は、bare atom (`write`, `read`, `append`) か、application-like form (`owner_is(session_user)`, `well_formed(profile_draft)`) のどちらかで書いてよい。
- 複数 predicate を 1 つの clause で結ぶ必要がある場合、current L2 では explicit `and` だけを最小 connective として使ってよい。
- grouping が必要なら `(` `)` を使う。current L2 companion notation では、これで十分とする。
- multi-line predicate block の改行は、predicate expression の継続を示す whitespace としてだけ読む。改行そのものでは implicit conjunction を導入しない。
- multi-line predicate block 内の blank line は current L2 では使わない。clause attachment と predicate continuation の読みを混ぜないためである。
- `or`、`not`、比較演算子の最小集合、precedence table、predicate block の内部でどこまで indentation-sensitive に読むかは **未決定** である。

### 3. option declaration と option-local declared contract surface

option declaration は、同一 chain の候補を個別に宣言する最小形として、次を current L2 候補とする。

```text
option primary on profile_doc capability write lease live
option mirror on profile_doc capability write lease live
option readonly on profile_doc capability read lease live
```

- `option <name>` は chain 内で参照する候補名を導入する current L2 候補である。
- `on <target>` は option-local な `declared access target` を表す。
- `capability <cap>` と `lease <guard>` は、current L2 examples で最小限必要な declared capability surface と lifetime guard を inline で置く候補である。
- current L2 では、capability と `lease` だけで successor compatibility を説明できる例では、それ以上の option-local contract 情報を要求しない。
- 逆に、capability と `lease` だけでは successor 判定や admissibility の違いを書き分けられない場合、current L2 companion notation では option-local declared contract surface を admission-side metadata として明示しなければならない。省略したまま same-lineage / successor 判定へ進めてはならない。

#### option-local declared contract surface の最小候補

current L2 で option declaration 側へ追加してよい最小 metadata は、option が success-side choice になれる条件を表す admission-side information だけに留める。

```text
option owner_writer on profile_doc capability write lease live
  admit owner_is(session_user)

option delegated_writer on profile_doc capability write lease live
  admit:
    (
      delegate_granted(session_user)
      and well_formed(profile_draft)
    )
```

- option-local declared contract surface は、current L2 では `admit pred` または `admit:` に続く 1 段深い predicate block で書く companion 候補とする。
- `admit` は statement-local `require` / `ensure` を option declaration に流用しないための別 marker である。これは examples 用の companion token であり、最終 reserved keyword を固定する判断ではない。
- `admit` がぶら下がる先は直前の `option` declaration だけである。`perform` に付く `require` / `ensure` と clause attachment を共有しない。
- `admit` に書ける predicate 断片は、current L2 では statement-local clause と同じ最小 fragment に揃えてよい。すなわち bare atom、application-like form、explicit `and`、括弧 grouping までを使ってよい。
- option-local metadata として current L2 で先に認めるのは admission-side だけであり、option-local `ensure`、option-local `invariant`、outcome guarantee の書式はまだ導入しない。
- したがって current L2 では、`perform` に付く `require` / `ensure` は request-local contract、option declaration に付く `admit` は option-local admissibility を表す、という役割分担に留める。
- `admit` を使うのは、capability と `lease` だけでは option 間の admissibility の違いを書けない場合に限る。その場合は省略してはならない。capability-only case では省略するのを既定とする。
- `admit` の最終 keyword / punctuation、option-local outcome metadata を別 marker で持つかどうか、option-local contract surface を line-based continuation 以外でも書けるようにするかは **未決定** である。

#### option-local `admit` と runtime admissibility

current L2 では、`admit` が不成立だった option を runtime でどう読むかについて、少なくとも次の 3 読みがありうる。

- non-admissible skip
- explicit failure
- dynamic `Reject`

このうち current L2 の最小読解として採るのは non-admissible skip である。

- `admit` が満たされない option は、その request 評価において success-side candidate から外れるだけである。
- この時点では explicit failure outcome を立てない。explicit failure は、admitted だった option を実際に試した後に operation 側で失敗した場合へ残す。
- この時点では request 全体を直ちに dynamic `Reject` にもしない。dynamic `Reject` は、後続 option も含めて current request を満たす admissible candidate が残らなかったときにだけ立つ。
- したがって canonical chain の runtime 読みは「leftmost admitted and request-compatible option を選ぶ」であり、`admit` 不成立はその候補を silently repair するのではなく、単に success-side 候補集合から外す。
- この読解でも hidden acceptance は起きない。理由は、`admit` の存在自体が static に明示されており、かつ不成立時は earlier option へ戻るのではなく canonical chain の後段へしか進めないためである。
- current L2 の最小観測面としては、`admit` 不成立を dedicated skip event にする必要はない。event surface には request-level outcome を優先して残し、`admit` 不成立は audit / trace 側の skip reason metadata に留めてよい。
- 同様に `lease` expiry も、候補が success-side choice から外れた理由として audit / trace 側に残してよく、その時点で dedicated event を必須にはしない。
- したがって current L2 では、`admit` miss と `lease` expiry をまず同じ大分類の non-admissible reason として扱い、その下位 reason を区別して記録する最小読解を採る。
- この最小読解では、少なくとも `admit-miss` と `lease-expired` を別 reason として区別できれば十分である。最終的な field 名、reason code 名、serialization は **未決定** である。
- 将来 dedicated skip event を導入する余地は残すが、current L2 では固定しない。

#### non-admissible reason audit metadata の最小 shape

current L2 では、`admit` miss や `lease` expiry を dedicated skip event にせず audit / trace metadata に留めるなら、少なくとも次の概念的 shape があれば E3 比較用 variant と E6 を説明できる。

- metadata は current request evaluation に結び付いていなければならない。
- どの option が success-side candidate から外れたかを指す `option ref` が必要である。
- E3 比較用 variant と E6 を区別して読むには、少なくとも `admit-miss` と `lease-expired` を識別できる `subreason` が必要である。
- broad family として non-admissible reason であることは metadata channel 自体から分かれば十分であり、current L2 では独立した `reason kind` field を必須にしない。
- explicit な `request ref` field も current L2 の最小要件には含めない。metadata が current request evaluation に request-local にぶら下がっていれば、conceptual shape としては十分だからである。
- したがって current L2 の最小 shape は、「request に結び付いた option-local miss record があり、その record から `option ref` と `subreason` を読めること」と言い換えてよい。
- `option ref` がないと、E3 比較用 variant で `owner_writer` が外れたのか、E6 で `writer` が `lease-expired` だったのかを説明できない。
- `subreason` がないと、E3 の `admit-miss` と E6 の `lease-expired` を同じ broad bucket に潰してしまい、representative examples が要求する説明力を失う。
- detached serialization や cross-trace correlation を後で導入するなら、`request ref` またはそれに相当する carrier が必要になる可能性は残る。この点は **未決定** である。
- 同様に、field 名、reason code 名、serialization、key ordering は current L2 では固定しない。ここで固定するのは概念的 shape だけである。
- current L2 では、`admit-miss` と `lease-expired` だけを formal subreason として残し、capability mismatch は narrative audit explanation に留める。理由は、前二者が option-local miss metadata だけで表せるのに対し、capability mismatch は request-local `require` と declared capability surface の比較から読む方が E6 を小さく保てるためである。
- したがって current L2 では、capability mismatch を formal subreason にしなくても E6 を十分に説明できる。formal subreason 化は将来の taxonomy 拡張候補としては残すが、現時点では **未決定** とする。

### 4. chain declaration

nested fallback の canonical form を examples で安定して書くために、chain declaration は次を current L2 候補とする。

```text
chain profile_ref = primary
  fallback mirror
    @ lineage(primary -> mirror)
  fallback readonly
    @ lineage(mirror -> readonly)
```

- `chain <name> = <head-option>` は canonical chain の先頭候補を置く current L2 候補である。
- 後続候補は、優先順に `fallback <successor>` を縦に並べる。
- edge-local な `documented lineage annotation` は、各 `fallback` row にぶら下がる indented continuation line として `@ lineage(predecessor -> successor)` を置いてよい。current L2 では、この hanging continuation を polished rendering の第一候補とする。
- ただし row が短く収まり、視認性を落とさない場合に限って、`fallback mirror @ lineage(primary -> mirror)` のような inline 省略形を companion-equivalent な短い書き方として残してよい。
- representative examples の本文で fallback / preference chain 自体を読むコード片では、A2 を first-choice rendering にしてよい。A1 は valid / malformed / underdeclared の最小 snippet や局所比較のように、行数を増やさない方が読みやすい場面へ残す。
- `lineage(...)` は例示用 token であり、最終 keyword / punctuation / serialization は **未決定** である。
- この書式は canonical form を書くための候補であり、preference chain を新しい core primitive へ昇格させるものではない。
- この書式は outer option を寿命の長い container として表すものではない。current L2 では canonical chain の優先順だけを表し、元の nested outer / inner 形そのものは観測可能意味に残さない。
- `target` / `capability` / `lease` は option declaration 側の option-local surface に残し、chain row へ再掲しないのを current L2 の既定とする。chain row は successor relation と edge-local lineage を表すだけに留める。
- compact syntax candidate の比較は `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md` にまとめる。current L2 では、line-leading `>` ladder のようなより短い案を比較対象として残しつつも、edge-local lineage と request-evaluation 境界を最も誤読させにくい explicit edge-row family を暫定 companion notation として維持する。その family の中では、hanging continuation を使う rendering を polished first choice に置き、inline 省略形は短い chain 向けの companion-equivalent shorthand として残す。
- notation rollout の current L2 判断としては、A2 を docs-only の候補に留めず、representative examples のうち fallback / preference chain を主題にするコード片へ limited rollout してよい。fixture JSON や machine-check surface はこの rollout の対象にしない。

### 5. chain 経由の `perform`

option chain を使う effect request は、次の形を current L2 候補とする。

```text
perform read_profile via profile_ref
  require read
```

- `via <chain-ref>` は、request が option chain を通じて admissible option を探すことを示す current L2 候補である。
- `via` を使う場合、`target` は option declaration 側に置き、`perform` 側で重複記述しないのを既定とする。
- どの option が runtime で実際に選ばれるかは `via` 自体には書かず、canonical order と current evaluation で決まる。

### 6. statement separator と block nesting

current L2 の representative examples では、statement separator と block nesting を次の最小規則で読む。

```text
place root {
  place session {
    perform update_authority on profile_authority
      require write
      ensure owner_is(session_user)

    atomic_cut

    try {
      perform load_profile on profile_snapshot
        require read
    } fallback {
      perform load_backup on backup_snapshot
        require read
    }
  }
}
```

- `place`, `try`, `fallback` は brace-delimited block head として読む。
- `perform`、`atomic_cut`、`option`、`chain` は current block に属する ordinary statement として読む。
- 直後の indented `require` / `ensure` 行は、直前の `perform` statement にだけ属する。single-line clause でも multi-line predicate block でも、perform line より浅く dedent した時点で clause attachment は終わる。
- `require:` / `ensure:` の header line に続く、さらに 1 段深い predicate block は、その clause の continuation として読む。predicate block から clause indent に戻った時点では clause 自体はまだ継続してよく、その後の次 clause または dedent / 次 statement head で clause suite が終わる。
- predicate block 内の改行は continuation であり、boolean connective の省略を意味しない。複数 predicate を結ぶなら `and` を明示しなければならない。
- `chain ref = head` の直後に indented された `fallback successor ...` 行は、その chain declaration に属する continuation line として読む。
- polished rendering を使う場合、さらに 1 段深い `@ lineage(...)` 行は、直前の `fallback <successor>` row にだけ属する edge-local metadata continuation として読む。metadata indent から `fallback` row の indent へ戻った時点ではその row 自体は終わっており、次の `fallback` row、dedent、または次 statement head が来た時点で chain continuation 全体が進む。
- 行頭 keyword が `perform`、`atomic_cut`、`option`、`chain`、`place`、`try`、`}` のいずれかに切り替わった時点でも、直前の clause suite は終わったものとして読む。
- blank line は readability のためだけに使い、意味論上の独立 separator token とはしない。
- current L2 では `;` や `,` のような明示 separator は要求しない。statement の切れ目は block structure、dedent、行頭 keyword で読む。
- `require` / `ensure` を clause suite の外へ浮かせたり、複数の `perform` に共有させたりする書き方は、current L2 companion notation には含めない。

### 7. local rollback を伴う `try`

local rollback を伴う `try` は、current L2 では block form だけを最小候補とする。

```text
try {
  perform stage_profile_patch on profile_draft
    require write

  perform validate_profile_patch on profile_draft
    require well_formed
} fallback {
  perform load_last_snapshot on profile_snapshot
    require read
}
```

- current `place` の入れ子がすでに rollback scope を与えるので、`try` 自体に追加の scope 指定句は要求しない。
- current L2 では、`try` の式形式や value-returning form は導入しない。representative examples では block form だけを使う。
- `fallback { ... }` は、直前の `try` に後置される explicit fallback branch を表す current L2 候補である。
- この `fallback` は chain declaration の `fallback successor` と同じ token を共有していても、構文形が異なる。前者は block branch、後者は canonical chain の後続候補である。

### 8. `atomic_cut` と並ぶときの読み

`atomic_cut` は `try` body の普通の statement として置き、`try` 専用の別書式は導入しない。

```text
try {
  perform update_authority on profile_authority
    require write

  atomic_cut

  perform append_audit on authority_log
    require append
} fallback {
  perform load_last_snapshot on authority_snapshot
    require read
}
```

- block formを保つことで、`atomic_cut` が rollback frontier を切る ordinary statement であることを読みやすく保つ。
- failure が `atomic_cut` の後で起きた場合でも、pre-cut 部分は rollback されない。これは意味論上の制約であり、surface syntax 側に追加 token を要求しない。

## `declared access target` と lineage annotation の役割分担

- `declared access target` は option-local に残す。
- `documented lineage annotation` は edge-local に残す。
- current L2 で same logical access path / semantic lineage の static 証拠になるのは、この両方の組だけである。
- したがって `on <target>` だけでは same-lineage continuation を証明できず、`lineage(...)` だけでも access path の anchor を欠く。

## valid / malformed / underdeclared の最小例

### valid

```text
option primary on profile_doc capability read lease live
option mirror on profile_doc capability read lease live

chain profile_ref = primary
  fallback mirror @ lineage(primary -> mirror)
```

- 両端 option に `declared access target` がある。
- `fallback` 行の edge-local annotation が、飾っている edge を正しく指している。

### malformed

```text
option primary on profile_doc capability read lease live
option mirror on profile_doc capability read lease live
option archive on profile_doc capability read lease live

chain profile_ref = primary
  fallback mirror @ lineage(primary -> archive)
```

- edge-local annotation が、飾っている edge ではなく別 option を指しているため malformed である。

### underdeclared

```text
option primary on profile_doc capability read lease live
option mirror on profile_doc capability read lease live

chain profile_ref = primary
  fallback mirror
```

- `declared access target` はあるが edge-local annotation が欠けている。
- current L2 では hidden acceptance を認めず、surface-level static error に留める。

## representative examples への当てはめ

この候補書式を使うと、valid な fallback / preference chain は次のように書ける。

```text
option primary on profile_doc capability write lease live
option mirror on profile_doc capability write lease live
option readonly on profile_doc capability read lease live

chain profile_ref = primary
  fallback mirror
    @ lineage(primary -> mirror)
  fallback readonly
    @ lineage(mirror -> readonly)

perform read_profile via profile_ref
  require read
```

write-after-expiry は次のように書ける。

```text
option writer on profile_doc capability write lease expired
option readonly on profile_doc capability read lease live

chain profile_ref = writer
  fallback readonly
    @ lineage(writer -> readonly)

perform write_profile via profile_ref
  require write
```

- chain 自体は well-formed である。
- runtime では `writer` が success-side choice になれず、後段の `readonly` も write request を満たせないため `Reject` になる。

capability と `lease` だけでは option 間の admissibility の差を書き分けられない場合は、次のように option-local declared contract surface を足さなければならない。

```text
option owner_writer on profile_doc capability write lease live
  admit owner_is(session_user)
option delegated_writer on profile_doc capability write lease live
  admit delegate_granted(session_user)

chain profile_ref = owner_writer
  fallback delegated_writer
    @ lineage(owner_writer -> delegated_writer)

perform write_profile via profile_ref
  require write
```

- この例では、両 option は同じ target / capability / `lease` を持つので、admissibility の違いは option-local `admit` に置く方が読みやすい。
- `perform` 側の `require write` は request-local condition のままであり、option 側の `admit` と別役割を保つ。
- runtime では、先頭 option の `admit` が満たされなければ、その option は non-admissible skip として候補集合から外れ、次の fallback successor を見る。
- ここで explicit failure を立てる必要はない。explicit failure は admitted option を試した後の failure にだけ残す。
- ここで dynamic `Reject` を立てるのは、後続 option も含めて write request を満たす admissible candidate が尽きた場合だけである。
- current L2 の最小観測面では、`owner_writer` の skip は dedicated event にせず、audit / trace に `non-admissible reason = admit-miss` を残せば足りる。
- outcome-side guarantee を option 側へどう書くかは、current L2 では **未決定** のまま残す。

local `try` + `fallback` は次のように書ける。

```text
try {
  perform stage_profile_patch on profile_draft
    require write

  perform validate_profile_patch on profile_draft
    require well_formed
} fallback {
  perform load_last_snapshot on profile_snapshot
    require read
}
```

- `try` body は current `place` に局所な rollback region を表す。
- fallback branch は rollback の後に explicit に選ばれる branch であり、option chain declaration ではない。

## ここであえて決めていないこと

- `perform`、`option`、`chain`、`on`、`via` を最終 reserved keyword にするかどうか。
- `contract` を独立した surface keyword として立てるかどうか。
- current L2 companion notation として、single-line clause を `require pred` / `ensure pred`、multi-line predicate を `require:` / `ensure:` に続く predicate block と読み、predicate 断片として bare atom、application-like form、explicit `and`、括弧 grouping を使うところまでは採ってよい。ただし final parser syntax としての punctuation、`or` / `not` / precedence table、predicate block 内の boolean expression grammar、blank line 許可は未決定である。
- blank line 以外の explicit separator token を最終 grammar で導入するかどうか。
- option-local declared contract surface の最終 keyword / punctuation をどうするか。また `admit` 以外の marker が必要かどうか。
- option-local outcome guarantee / invariant を current L2 companion notation に含めるかどうか。
- `try` / `fallback` の最終 keyword、`try { ... } fallback { ... }` を唯一の block form にするか、将来 sugar や式形式を許すか。
- `lineage(...)` の最終 token、同じ意味を持つ sugar、same-place / cross-place で形を共有するかどうか。

ここで固定するのは、current L2 の representative examples を安定して書き直せるだけの最小候補であり、final parser syntax ではない。
