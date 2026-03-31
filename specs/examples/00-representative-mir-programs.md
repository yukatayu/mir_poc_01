# examples/00 — representative Mir programs（current L2 companion）

この文書は、`specs/04-mir-core.md` と `specs/10-open-questions.md` の current L2 読解を、**代表的な Mir プログラム片**として見える形にした補助文書である。
ここに置くコード片は parser-ready な最終構文ではなく、**説明用記法**として読む。未決定の token、punctuation、serialization をここで確定したことにはしない。

## この文書の位置づけ

- Mir-0 / current L2 で何が自然に書けるかを確認する。
- static 判定、runtime outcome、trace / audit の最小読解を例ごとに並べる。
- 書きづらい箇所や、未決のため決め打ちできない箇所を早めに発見する。

## 説明用記法

- `place name { ... }`
  - current `place` を入れ子で示す説明用記法である。
- `perform op on target` / `perform op via chain_ref`
  - `perform` と direct target / option chain 参照の current L2 候補である。最終 reserved keyword は未決定である。
- `require pred` / `ensure pred`
  - 直前の `perform` に付く statement-local clause の current L2 候補である。current examples では `require` を先、`ensure` を後に置き、blank line を挟まず同じ clause suite として読む。`contract` は semantic role の名前であり、examples では独立 keyword にせず、`contract { ... }` block sugar も使わない。
- `option name on target capability cap lease guard`
  - option declaration の current L2 候補である。`declared access target`、最小 capability surface、lifetime guard を inline で置く。
- `chain ref = head` と、それに続く `fallback successor @ lineage(predecessor -> successor)`
  - canonical form を examples で書くための current L2 候補である。`lineage(...)` は例示であり、最終 token ではない。
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

## E1 — place 入れ子 + authority update + `atomic_cut`

### コード

```text
place root {
  place session {
    place authority_cell {
      perform update_authority on profile_authority
        require write
        ensure owner_is(session_user)

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
  - `require` / `ensure` は `update_authority` にだけ付く statement-local clause として読み、dedent と blank line でその clause suite が終わる。

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
          require well_formed
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
  - `require` 行はそれぞれ直前の `perform` にだけ属し、fallback branch や次の statement へ共有されない。

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
        fallback mirror @ lineage(primary -> mirror)
        fallback readonly @ lineage(mirror -> readonly)

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
  - この例では capability だけで successor compatibility を説明できるため、追加の option-local contract clause を省略しても underdeclared にはしない。
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
        fallback mirror @ lineage(primary -> archive)
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
        fallback readonly @ lineage(writer -> readonly)

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
  - current request は `perform write_profile via profile_ref` に付く `require write` だけであり、追加の option-local contract clause がなくても runtime `Reject` の読みは保てる。

### 期待される runtime outcome

- 想定シナリオ:
  - `writer` は `lease=expired` のため success-side choice になれない。
  - 後段の `readonly` は read-only であり、write request を満たせない。
- outcome:
  - write-admissible option が残らないため explicit `Reject`。
  - hidden buffering、hidden resurrection、earlier option への再昇格は起こらない。

### 最小 trace / audit 説明

- event:
  - writer lease-expiry による不適格化
  - readonly の capability mismatch
  - explicit `Reject`
- 説明可能であるべきこと:
  - chain 自体は static に well-formed だが、current evaluation の request 種別によって runtime `Reject` へ落ちること

## 書いてみて見えた current L2 の穴

- `perform`、statement-local `require` / `ensure`、option chain 参照、local `try` / `fallback` については、`specs/examples/01-current-l2-surface-syntax-candidates.md` の current L2 候補でかなり安定して書けるようになった。特に `contract` を semantic role に留め、surface では statement-local clause だけを使う方針にすると、`place` / `try` / `fallback` の block 読みと競合しにくい。
- それでも `try` / `fallback` の最終 keyword と punctuation、`contract` を独立 block にするかどうか、richer な option-local contract surface、`lineage(...)` の最終 token はまだ足りない。
- `place` を入れ子で書く方式は例示には十分だが、cross-place transfer や same-place / cross-place の surface rule にはまだ補助 syntax が必要になる可能性がある。
- `emit` や coroutine は今回の代表例には不要だった。ただし long-lived interaction や stream 的 trace を例示し始めると、将来は別文書で必要になる可能性が高い。

## ここで決めていないこと

- ここで使った code block はすべて説明用記法であり、parser 実装用の最終 syntax ではない。
- `documented lineage annotation` の token 形、`perform` / `option` / `chain` / `on` / `via` / `try` / `fallback` / `contract` を最終 reserved keyword にするかどうか、`try` / `fallback` の式形式や追加 sugar、`require` / `ensure` の最終 punctuation は **未決定** である。
- cross-place 版の representative programs は今回含めない。cut family や same-place / cross-place syntax の未決定を、ここで勝手に埋めないためである。
