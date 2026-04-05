# examples/00 — representative Mir programs（current L2 companion）

この文書は、`specs/04-mir-core.md` と `specs/10-open-questions.md` の current L2 読解を、**代表的な Mir プログラム片**として見える形にした補助文書である。
ここに置くコード片は parser-ready な最終構文ではなく、**説明用記法**として読む。未決定の token、punctuation、serialization をここで確定したことにはしない。

## この文書の位置づけ

- Mir-0 / current L2 で何が自然に書けるかを確認する。
- static 判定、runtime outcome、trace / audit の最小読解を例ごとに並べる。
- 書きづらい箇所や、未決のため決め打ちできない箇所を早めに発見する。
- parser なしで実行準備するための machine-readable fixture は `specs/examples/02-current-l2-ast-fixture-schema.md` と `crates/mir-ast/tests/fixtures/current-l2/` を参照する。

## 説明用記法

- `place name { ... }`
  - current `place` を入れ子で示す説明用記法である。
- `perform op on target` / `perform op via chain_ref`
  - `perform` と direct target / option chain 参照の current L2 候補である。最終 reserved keyword は未決定である。
- `require pred` / `ensure pred`
  - 直前の `perform` に付く statement-local clause の current L2 候補である。single-line clause は `require pred` / `ensure pred` と書き、multi-line predicate が必要な場合だけ `require:` / `ensure:` に続く 1 段深い predicate block を使う。predicate の最小断片としては bare atom、application-like form、explicit `and`、括弧 grouping までを使ってよく、改行だけで implicit conjunction は導入しない。current examples では `require` を先、`ensure` を後に置き、blank line を挟まず同じ clause suite として読む。`contract` は semantic role の名前であり、examples では独立 keyword にせず、`contract { ... }` block sugar も使わない。
- `option name on target capability cap lease guard`
  - option declaration の current L2 候補である。`declared access target`、最小 capability surface、lifetime guard を inline で置く。capability と `lease` だけでは option 間の admissibility の違いを書けない場合は、直後に indented な `admit pred` / `admit:` を option-local declared contract surface として付けなければならない。
- `chain ref = head` と、それに続く `fallback successor` / indented `@ lineage(predecessor -> successor)`
  - canonical form を examples で書くための current L2 候補である。current L2 では hanging continuation を polished first choice に置くが、短い row では `fallback successor @ lineage(predecessor -> successor)` の inline 省略形も companion-equivalent な shorthand として残してよい。`lineage(...)` は例示であり、最終 token ではない。
  - ただし representative examples の本文で fallback / preference chain 自体を主題にしているコード片では、A2 の hanging continuation を first-choice rendering とする。inline 省略形は最小 snippet や局所比較に留める。
  - A2 の追加 indent は outer / inner wrapper の入れ子を増やすものではなく、直前の `fallback <successor>` row にだけ属する edge-local metadata continuation として読む。
- `try { ... } fallback { ... }`
  - current `place` に局所な rollback を伴う `try` の block form と、その直後に置かれる explicit fallback branch の current L2 候補である。最終 keyword と punctuation は未決定である。
- blank line
  - readability のための区切りであり、独立した意味論 token としては扱わない。statement の切れ目は block structure、dedent、行頭 keyword で読む。

より詳しい候補書式は `specs/examples/01-current-l2-surface-syntax-candidates.md` を参照。

## 例の一覧

| ID | 主題 | 静的判定 | 動的焦点 |
|---|---|---|---|
| E1 | place 入れ子 + authority update + `atomic_cut` | valid | post-cut failure でも pre-cut は rollback されない |
| E2 | local `try` + `fallback` | valid | local rollback 後に explicit fallback が走る |
| E3 | valid な fallback / preference chain | valid | canonical chain 上で leftmost admissible option を選ぶ |
| E4 | malformed fallback branch | malformed | static rejection、runtime なし |
| E5 | underdeclared fallback case | underdeclared | surface-level static error、runtime なし |
| E6 | write-after-expiry | valid | write-admissible option 不在のため runtime `Reject` |
| E12 | declared target missing | underdeclared | successor target が空のため static stop |
| E13 | capability strengthening | malformed | successor capability 強化のため static stop |
| E14 | duplicate option declaration | malformed | visible option name 衝突のため static stop |
| E15 | duplicate chain declaration | malformed | visible chain name 衝突のため static stop |
| E16 | missing chain head option | malformed | visible chain head が未宣言 option のため static stop |
| E17 | missing predecessor option | malformed | edge predecessor が未宣言 option のため static stop |
| E18 | missing successor option | malformed | edge successor が未宣言 option のため static stop |

## E1 — place 入れ子 + authority update + `atomic_cut`

### コード

```text
place root {
  place session {
    place authority_cell {
      perform update_authority on profile_authority
        require write
        ensure:
          owner_is(session_user)

      atomic_cut

      perform append_audit on authority_log
        require append
    }
  }
}
```

### 期待される static 判定

- `valid`
- 理由:
  - `place` の入れ子は current Mir-0 / L2 と矛盾しない。
  - `atomic_cut` は current `place` の rollback frontier を確定する最小 cut として使われている。
  - `perform` は説明用記法であり、ここでは effect / contract / cut の関係を見るために使っている。
  - `require` / `ensure` は `update_authority` にだけ付く statement-local clause として読む。`ensure:` の下に置かれた predicate block も含めて 1 つの clause suite であり、dedent または次 statement head への移行でその suite が終わる。

### 期待される runtime outcome

- 想定シナリオ:
  - `update_authority` は成功する。
  - `append_audit` は explicit failure になる。
- outcome:
  - `authority_cell` 内の pre-cut 部分、すなわち `update_authority` は rollback されない。
  - その後は explicit failure、fallback、または compensation として扱われるべきであり、hidden rollback は起こらない。

### 最小 trace / audit 説明

- event:
  - authority update event
  - `atomic_cut`
  - audit append failure
- 説明可能であるべきこと:
  - どこで current `place` の rollback frontier が確定したか
  - failure が cut の後に起きたため、pre-cut prefix が rollback 対象外であること

## E2 — local `try` + `fallback`

### コード

```text
place root {
  place session {
    place draft_profile {
      try {
        perform stage_profile_patch on profile_draft
          require write

        perform validate_profile_patch on profile_draft
          require:
            (
              owner_is(session_user)
              and well_formed(profile_draft)
            )
      } fallback {
        perform load_last_snapshot on profile_snapshot
          require read
      }
    }
  }
}
```

### 期待される static 判定

- `valid`
- 理由:
  - `try` は current `place` に局所な rollback semantics を持つ。
  - この例では `atomic_cut` を跨いでいないため、rollback frontier は `draft_profile` の内部に閉じている。
  - current L2 では block form の `try { ... } fallback { ... }` を examples 用 companion syntax 候補として使ってよい。
  - `require` 行はそれぞれ直前の `perform` にだけ属し、multi-line predicate block を使う場合でも fallback branch や次の statement へ共有されない。
  - `owner_is(session_user) and well_formed(profile_draft)` は 1 つの predicate expression として読み、改行だけで暗黙の conjunction を増やしたり、別 clause を生やしたりしない。

### 期待される runtime outcome

- 想定シナリオ:
  - `stage_profile_patch` は成功する。
  - `validate_profile_patch` は explicit failure になる。
  - fallback branch の `load_last_snapshot` は成功する。
- outcome:
  - `draft_profile` 内の try-body の local state は rollback される。
  - その後、explicit fallback branch が実行される。
  - hidden backtracking や hidden API shadowing は入らない。

### 最小 trace / audit 説明

- event:
  - stage patch event
  - validation failure
  - current `place` 内の rollback
  - snapshot load event
- 説明可能であるべきこと:
  - rollback が `draft_profile` に局所であること
  - fallback branch が rollback の後に明示的に選ばれたこと

## E3 — valid な fallback / preference chain

### コード

```text
place root {
  place session {
    place profile_access {
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
    }
  }
}
```

### 期待される static 判定

- `valid`
- 理由:
  - 3 候補は同じ `declared access target` を共有している。
  - edge-local な `documented lineage annotation` が `primary -> mirror` と `mirror -> readonly` をそれぞれ明示している。
  - capability は `write -> write -> read` と単調に弱くなる側へしか進んでいない。
  - この例では capability だけで successor compatibility を説明できるため、追加の option-local `admit` を省略しても underdeclared にはしない。
  - `perform read_profile via profile_ref` に続く `require read` は、その request にだけ付く clause suite として読む。

### 期待される runtime outcome

- 想定シナリオ:
  - `primary` は explicit failure。
  - `mirror` も explicit failure。
  - `readonly` は read request に対して admissible であり成功する。
- outcome:
  - nested fallback は canonical chain `primary > mirror > readonly` として読める。
  - current evaluation は leftmost admissible option を順に試し、最終的に `readonly` で成功する。

### 最小 trace / audit 説明

- static 側:
  - canonical chain `primary > mirror > readonly`
- runtime 側:
  - primary failure
  - mirror failure
  - readonly success
- 説明可能であるべきこと:
  - 元の内側 / 外側の nesting ではなく、canonical chain の順序が観測可能意味を支えること

### 比較用 variant — option-local contract surface が必要な場合

```text
place root {
  place session {
    place profile_access {
      option owner_writer on profile_doc capability write lease live
        admit owner_is(session_user)
      option delegated_writer on profile_doc capability write lease live
        admit delegate_granted(session_user)

      chain profile_ref = owner_writer
        fallback delegated_writer
          @ lineage(owner_writer -> delegated_writer)

      perform write_profile via profile_ref
        require write
    }
  }
}
```

- この variant では、両 option は同じ target / capability / `lease` を持つため、option 間の admissibility の差は option-local `admit` を見ないと表せない。
- `perform` に付く `require write` は request-local condition のままであり、option 側の `admit` は各 option が success-side choice になれる追加条件として読む。
- current L2 では、`owner_writer` の `admit` が満たされなければ、その option は non-admissible skip として候補から外れ、`delegated_writer` が次の success-side candidate になる。
- この skip だけでは explicit failure は立たない。explicit failure は、admitted option を実際に試したあとに operation が失敗した場合へ残す。
- request 全体が dynamic `Reject` になるのは、後段 option まで見ても write request を満たす admissible candidate が残らない場合だけである。
- current L2 の最小 trace / audit では、`owner_writer` の skip を独立 event にする必要はなく、current request evaluation にぶら下がる audit metadata として、少なくとも `option ref = owner_writer` と `subreason = admit-miss` が読めればよい。broad family が non-admissible reason であることは metadata channel から分かれば十分で、explicit な `request ref` field や `reason kind` field は current L2 では必須にしない。
- current L2 では、ここで option-local `ensure` や outcome guarantee を追加せず、admission-side metadata だけに留める。

## E4 — malformed fallback branch（static rejection）

### コード

```text
place root {
  place session {
    place profile_access {
      option primary on profile_doc capability read lease live
      option mirror on profile_doc capability read lease live
      option archive on profile_doc capability read lease live

      chain profile_ref = primary
        fallback mirror
          @ lineage(primary -> archive)
    }
  }
}
```

### 期待される static 判定

- `malformed`
- 理由:
  - edge-local annotation が、自分の飾っている fallback edge `primary -> mirror` ではなく `primary -> archive` を指している。
  - したがって declared information だけで same-lineage continuation claim が壊れており、current L2 では static rejection に寄せる。

### 期待される runtime outcome

- runtime evaluation には入らない。
- canonical chain を持つ well-formed preference chain として扱わない。

### 最小 trace / audit 説明

- 説明可能であるべきこと:
  - malformed と判定された根拠が edge-local annotation の不一致にあること
  - hidden repair により `mirror` へ進んだことにしてはならないこと

## E5 — underdeclared fallback case（surface-level static error）

### コード

```text
place root {
  place session {
    place profile_access {
      option primary on profile_doc capability read lease live
      option mirror on profile_doc capability read lease live

      chain profile_ref = primary
        fallback mirror
    }
  }
}
```

### 期待される static 判定

- `underdeclared`
- 理由:
  - 両端に `declared access target` はあるが、edge-local な `documented lineage annotation` がない。
  - current L2 では hidden acceptance を禁止しているため、same-path 扱いして canonical chain に入れてはならない。

### 期待される runtime outcome

- runtime evaluation には入らない。
- current L2 では surface-level static error として止める。

### 最小 trace / audit 説明

- 説明可能であるべきこと:
  - target 一致だけでは same-lineage 証拠にならないこと
  - failure を dynamic `Reject` に押し込まず、宣言不足の静的問題として止めること

## E6 — write-after-expiry は runtime `Reject`

### コード

```text
place root {
  place session {
    place profile_access {
      option writer on profile_doc capability write lease expired
      option readonly on profile_doc capability read lease live

      chain profile_ref = writer
        fallback readonly
          @ lineage(writer -> readonly)

      perform write_profile via profile_ref
        require write
    }
  }
}
```

### 期待される static 判定

- `valid`
- 理由:
  - declared target と edge-local lineage annotation はそろっている。
  - capability は `write -> read` と単調に弱くなる方向であり、chain 自体は well-formed である。
  - current request は `perform write_profile via profile_ref` に付く `require write` だけであり、この例では追加の option-local `admit` がなくても runtime `Reject` の読みは保てる。

### 期待される runtime outcome

- 想定シナリオ:
  - `writer` は `lease=expired` のため success-side choice になれない。
  - 後段の `readonly` は read-only であり、write request を満たせない。
- outcome:
  - write-admissible option が残らないため explicit `Reject`。
  - この `Reject` は単一 option の admission miss そのものではなく、chain 全体で admissible candidate が尽きた結果として読む。
  - 逆に言えば、後段 option の存在は automatic success を意味しない。current request を実際に満たす admissible candidate が残るときにだけ継続できる。
  - hidden buffering、hidden resurrection、earlier option への再昇格は起こらない。

### 最小 trace / audit 説明

- event:
  - explicit `Reject`
- audit / trace metadata:
  - `writer` は current request evaluation にぶら下がる metadata として `option ref = writer` と `subreason = lease-expired`
  - `readonly` は request / capability mismatch により success-side choice になれなかったこと。この点は current L2 では formal subreason に上げず、narrative explanation に留める
- 説明可能であるべきこと:
  - chain 自体は static に well-formed だが、current evaluation の request 種別によって runtime `Reject` へ落ちること
  - `lease` expiry 自体は current L2 では dedicated event を必須にせず、`Reject` に至る non-admissible reason metadata の 1 つとして残してよいこと
  - `readonly` 側の request / capability mismatch は request-local `require write` と declared capability surface から読めるため、current L2 では formal subreason にせず narrative explanation に留めるだけでよいこと

## E12 — declared target missing は underdeclared static stop

### コード

```text
place root {
  place session {
    place profile_access {
      option primary on profile_doc capability read lease live
      option mirror on "" capability read lease live

      chain profile_ref = primary
        fallback mirror
          @ lineage(primary -> mirror)
    }
  }
}
```

### 期待される static 判定

- `underdeclared`
- 理由:
  - edge-local `lineage` 自体は整っていても、successor 側の `declared access target` が空である。
  - current L2 では declared target の欠落を hidden repair せず、same-lineage continuation claim を underdeclared として止める。

### 期待される runtime outcome

- runtime evaluation には入らない。
- empty target placeholder を dynamic `Reject` や implicit lookup に押し込まない。

### 最小 trace / audit 説明

- 説明可能であるべきこと:
  - underdeclared の根拠が lineage 証拠不足ではなく declared target 欠落にあること
  - missing target を runtime host lookup で埋めてはならないこと

## E13 — capability strengthening は malformed static stop

### コード

```text
place root {
  place session {
    place profile_access {
      option primary on profile_doc capability read lease live
      option mirror on profile_doc capability write lease live

      chain profile_ref = primary
        fallback mirror
          @ lineage(primary -> mirror)
    }
  }
}
```

### 期待される static 判定

- `malformed`
- 理由:
  - `primary -> mirror` は same-lineage continuation を主張しているが、declared capability が `read -> write` に強化されている。
  - current L2 では fallback / preference chain は capability monotonicity を破ってはならず、これは malformed に寄せる。

### 期待される runtime outcome

- runtime evaluation には入らない。
- stronger successor capability を fallback で隠して write 権限を復活させてはならない。

### 最小 trace / audit 説明

- 説明可能であるべきこと:
  - malformed の根拠が lineage annotation mismatch ではなく capability strengthening にあること
  - current L2 の same-lineage chain が monotone weakening を前提にしていること

## E14 — duplicate option declaration は malformed static stop

### コード

```text
place root {
  place session {
    place profile_access {
      option profile_ref on profile_doc capability read lease live
      option profile_ref on profile_doc capability read lease live
    }
  }
}
```

### 期待される static 判定

- `malformed`
- 理由:
  - 同じ visibility から 2 本の `option profile_ref` が見えている。
  - current L2 では visible option 名の衝突を hidden shadowing や declaration overwrite で repair しない。

### 期待される runtime outcome

- runtime evaluation には入らない。
- duplicate visible option declaration を current request evaluation に押し込まず、static stop として扱う。

### 最小 trace / audit 説明

- 説明可能であるべきこと:
  - malformed の根拠が duplicate visible option declaration にあること
  - current duplicate cluster は detached helper 側でも `checker_core.reasons` に留め、stable `reason_codes` へはまだ昇格しないこと

## E15 — duplicate chain declaration は malformed static stop

### コード

```text
place root {
  place session {
    place profile_access {
      option primary on profile_doc capability read lease live
      option mirror on profile_doc capability read lease live

      chain profile_ref = primary
      chain profile_ref = mirror
    }
  }
}
```

### 期待される static 判定

- `malformed`
- 理由:
  - 同じ visibility から 2 本の `chain profile_ref` が見えている。
  - current L2 では competing chain head を hidden choice にせず、duplicate visible chain declaration として止める。

### 期待される runtime outcome

- runtime evaluation には入らない。
- duplicate chain cluster を canonical chain choice の問題へ落とさず、static stop として扱う。

### 最小 trace / audit 説明

- 説明可能であるべきこと:
  - malformed の根拠が competing visible chain declaration にあること
  - current duplicate cluster は detached helper 側でも `checker_core.reasons` に留め、stable `reason_codes` へはまだ昇格しないこと

## E16 — missing chain head option は malformed static stop

### コード

```text
place root {
  place session {
    place profile_access {
      option primary on profile_doc capability read lease live

      chain profile_ref = ghost
    }
  }
}
```

### 期待される static 判定

- `malformed`
- 理由:
  - `chain` の `head` は同じ visible scope にある option declaration を指していなければならない。
  - 未宣言の `ghost` を head に置いても hidden declaration や hidden fallback 候補は生えない。
  - current helper cut ではこの cluster を stable actual wording として扱ってよく、`checked_reasons` と detached `reason_codes` の両方へ narrow に載せてよい。

### 期待される runtime outcome

- runtime evaluation には入らない。
- missing chain head option は malformed static stop であり、dynamic `Reject` や host-plan failure へは送らない。

### 最小 trace / audit 説明

- event は発生しない。
- 説明可能であるべきこと:
  - head 参照は visible option declaration に閉じること
  - missing head を hidden repair で埋めないこと

## E17 — missing predecessor option は malformed static stop

### コード

```text
place root {
  place session {
    place profile_access {
      option primary on profile_doc capability read lease live
      option mirror on profile_doc capability read lease live

      chain profile_ref = primary
        fallback mirror
          @ lineage(ghost -> mirror)
    }
  }
}
```

### 期待される static 判定

- `malformed`
- 理由:
  - edge の `predecessor` は visible option declaration でなければならない。
  - `lineage(ghost -> mirror)` と書いても、missing predecessor option は same-lineage continuation として成立しない。
  - current helper cut ではこの cluster を stable actual wording として扱ってよく、`checked_reasons` と detached `reason_codes` の両方へ narrow に載せてよい。

### 期待される runtime outcome

- runtime evaluation には入らない。
- missing predecessor option は chain edge の structural malformed として止める。

### 最小 trace / audit 説明

- event は発生しない。
- 説明可能であるべきこと:
  - lineage annotation は missing predecessor option を補わないこと
  - edge-local metadata と visible declaration inventory を混同しないこと

## E18 — missing successor option は malformed static stop

### コード

```text
place root {
  place session {
    place profile_access {
      option primary on profile_doc capability read lease live

      chain profile_ref = primary
        fallback ghost
          @ lineage(primary -> ghost)
    }
  }
}
```

### 期待される static 判定

- `malformed`
- 理由:
  - edge の `successor` は visible option declaration でなければならない。
  - `lineage(primary -> ghost)` と書いても、missing successor option は same-lineage fallback 候補として成立しない。
  - current helper cut ではこの cluster を stable actual wording として扱ってよく、`checked_reasons` と detached `reason_codes` の両方へ narrow に載せてよい。

### 期待される runtime outcome

- runtime evaluation には入らない。
- missing successor option は dynamic fallback へ落とさず malformed static stop として扱う。

### 最小 trace / audit 説明

- event は発生しない。
- 説明可能であるべきこと:
  - missing successor option を hidden later candidate へ repair しないこと
  - chain edge の可視宣言要件が runtime より先に効くこと

## 書いてみて見えた current L2 の穴

- `perform`、statement-local `require` / `ensure`、option chain 参照、local `try` / `fallback` については、`specs/examples/01-current-l2-surface-syntax-candidates.md` の current L2 候補でかなり安定して書けるようになった。特に `contract` を semantic role に留め、surface では statement-local clause だけを使う方針にすると、`place` / `try` / `fallback` の block 読みと競合しにくい。single-line clause を bare form、multi-line predicate を colon-headed clause block に分けると、長い predicate でも attachment を見失いにくい。
- それでも `try` / `fallback` の最終 keyword と punctuation、`contract` を独立 block にするかどうか、option-local `admit` の最終 keyword / punctuation、option-local outcome guarantee を持たせるかどうか、`lineage(...)` の最終 token、predicate に `or` / `not` / 比較演算子をどこまで入れるかはまだ足りない。
- `place` を入れ子で書く方式は例示には十分だが、cross-place transfer や same-place / cross-place の surface rule にはまだ補助 syntax が必要になる可能性がある。
- `emit` や coroutine は今回の代表例には不要だった。ただし long-lived interaction や stream 的 trace を例示し始めると、将来は別文書で必要になる可能性が高い。

## ここで決めていないこと

- ここで使った code block はすべて説明用記法であり、parser 実装用の最終 syntax ではない。
- `documented lineage annotation` の token 形、`perform` / `option` / `chain` / `on` / `via` / `try` / `fallback` / `contract` / `admit` を最終 reserved keyword にするかどうか、`try` / `fallback` の式形式や追加 sugar、`require` / `ensure` の final parser punctuation、option-local declared contract surface の exact notation、predicate block 内で `or` / `not` / precedence / blank line をどう扱うかは **未決定** である。
- cross-place 版の representative programs は今回含めない。cut family や same-place / cross-place syntax の未決定を、ここで勝手に埋めないためである。
