# examples/01 — current L2 surface syntax candidates

この文書は、`specs/examples/00-representative-mir-programs.md` で使う**current L2 の最小 surface syntax 候補**をまとめる補助文書である。
ここで整理するのは、`perform`、option chain 参照、`try` / `fallback`、`require` / `ensure` clause、statement separator / block nesting に関する**書き方の候補**だけであり、Mir-0 / Mir-1 / Mirrorea の境界や、canonical normalization law、rejection phase、static evidence floor、underdeclared handling 自体は変更しない。

## この文書の位置づけ

- current L2 の representative examples を、過度に ad-hoc ではない一定の書式で読めるようにする。
- parser-ready な最終 grammar や reserved keyword 集合は固定しない。
- `perform`、option declaration、chain 参照、`try` / `fallback`、statement-local clause、block nesting のうち、current L2 で最も摩擦の少ない最小表記だけを候補として置く。

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
    owner_is(
      session_user
    )
```

- `contract` はここでは semantic role の名前であり、representative examples に mandatory な surface keyword とはしない。
- single-line clause は `require pred` / `ensure pred` と書き、keyword の後ろに空白 1 つだけを置く。current L2 では追加の trailing punctuation を要求しない。
- multi-line predicate が必要な場合だけ、header line を `require:` / `ensure:` と書き、その直下に 1 段深い predicate block を置く。
- predicate block は、複数 clause の列挙ではなく**1 つの predicate を複数行に折り返したもの**として読む。predicate block の内部では blank line を使わない。
- current L2 では clause の既定順を `require` の後に `ensure` とする。ただしこれは examples を読みやすく保つための companion 規則であり、final grammar の順序制約を固定するものではない。
- current L2 の representative examples では、statement-local clause を持てる head は `perform` だけに限定する。`atomic_cut`、`option`、`chain`、`place`、`try`、`fallback` に clause をぶら下げない。
- `require` と `ensure` の間に blank line は入れず、同じ clause suite として隣接させるのを既定とする。
- `contract { require { ... } ensure { ... } }` のような block form は **未決定** の future syntax 候補として残すが、current L2 companion notation の optional sugar としては採用しない。

### 3. option declaration

option declaration は、同一 chain の候補を個別に宣言する最小形として、次を current L2 候補とする。

```text
option primary on profile_doc capability write lease live
option mirror on profile_doc capability write lease live
option readonly on profile_doc capability read lease live
```

- `option <name>` は chain 内で参照する候補名を導入する current L2 候補である。
- `on <target>` は option-local な `declared access target` を表す。
- `capability <cap>` と `lease <guard>` は、current L2 examples で最小限必要な declared capability surface と lifetime guard を inline で置く候補である。
- richer な declared contract surface が必要な場合の exact notation は **未決定** である。current L2 では、capability だけで successor compatibility を説明できる例ではそれ以上を inline で固定しない。
- 逆に、capability だけでは successor 判定が足りない例では、option-local contract surface の追加明示が必要になりうる。そこは current L2 の候補から外し、未決定として残す。

### 4. chain declaration

nested fallback の canonical form を examples で安定して書くために、chain declaration は次を current L2 候補とする。

```text
chain profile_ref = primary
  fallback mirror @ lineage(primary -> mirror)
  fallback readonly @ lineage(mirror -> readonly)
```

- `chain <name> = <head-option>` は canonical chain の先頭候補を置く current L2 候補である。
- 後続候補は、優先順に `fallback <successor>` を縦に並べる。
- edge-local な `documented lineage annotation` は、各 `fallback` 行に `@ lineage(predecessor -> successor)` として付ける。
- `lineage(...)` は例示用 token であり、最終 keyword / punctuation / serialization は **未決定** である。
- この書式は canonical form を書くための候補であり、preference chain を新しい core primitive へ昇格させるものではない。

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
- `chain ref = head` の直後に indented された `fallback successor ...` 行は、その chain declaration に属する continuation line として読む。
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
  fallback mirror @ lineage(primary -> mirror)
  fallback readonly @ lineage(mirror -> readonly)

perform read_profile via profile_ref
  require read
```

write-after-expiry は次のように書ける。

```text
option writer on profile_doc capability write lease expired
option readonly on profile_doc capability read lease live

chain profile_ref = writer
  fallback readonly @ lineage(writer -> readonly)

perform write_profile via profile_ref
  require write
```

- chain 自体は well-formed である。
- runtime では `writer` が success-side choice になれず、後段の `readonly` も write request を満たせないため `Reject` になる。

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
- current L2 companion notation として、single-line clause を `require pred` / `ensure pred`、multi-line predicate を `require:` / `ensure:` に続く predicate block と読むところまでは採ってよい。ただし final parser syntax としての punctuation、predicate block 内の boolean expression grammar、blank line 許可は未決定である。
- blank line 以外の explicit separator token を最終 grammar で導入するかどうか。
- richer な `declared contract surface` を option declaration にどう書くか。
- `try` / `fallback` の最終 keyword、`try { ... } fallback { ... }` を唯一の block form にするか、将来 sugar や式形式を許すか。
- `lineage(...)` の最終 token、同じ意味を持つ sugar、same-place / cross-place で形を共有するかどうか。

ここで固定するのは、current L2 の representative examples を安定して書き直せるだけの最小候補であり、final parser syntax ではない。
