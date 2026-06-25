# 01 — Final Decisions For This Package

この文書は、今回の rebaseline / implementation chain で迷わないための最終判断を列挙する。

## D1. Canonical place scope は `S { ... }`

採用する。

```mir
S {
  state player[p: Participant]: Player
}
```

採用しない。

```mir
S[
  state player[p: Participant]: Player
]
```

理由:

- `[]` は配列・Map・indexed state の参照に使う。
- 将来 `matrix[i][j]`, `player[p]`, `servers[i]`, `pose[frame]` などが自然に増える。
- `S[ ... ]` を place-scope に使うと、将来の place-valued expression、role instance、array indexing と衝突しやすい。
- `S { ... }` は初期案 `S[...]` の直感を保ちつつ、`[]` を値参照用に残せる。

## D2. `{}` の衝突は namespace + statement/expression context で解く

`S { ... }` は place-scope statement / item。
`Player { hp: 100 }` は record literal expression。

衝突回避規則:

1. Place / Role / Type / Value は alpha では重複不可の名前空間として扱う。
2. `S { ... }` が place block になるには、`S` が Place / Role path として解決される必要がある。
3. `Player { ... }` が record literal になるには、`Player` が type path として解決され、expression context にある必要がある。
4. place block 内の body は BlockItems であり、record field list ではない。
5. `S { hp: 100 }` のように place block か record literal か曖昧なものは、名前解決で一意でなければ diagnostic にする。

## D3. Surface Mir と Core Mir を分ける

Surface Mir:

```mir
A {
  when attack(target: Participant) {
    S {
      player[target].hp -= player[self].atk
    }
  }
}
```

Core Mir / elaborated form:

```text
transition attack by self at A
  send AttackRequest to S
transition apply_attack at S
  read player[self].atk
  write player[target].hp
  publish hp_changed if visible
```

User は基本的に Surface を書く。
Core は checker / runtime / devtools / diagnostics の対象。

## D4. `transition ... at ...` は expert / Core-facing syntax

`transition ... at ...` は消さない。
ただし user-facing primary syntax は location block / event block にする。

## D5. 通信は自動 elaboration する

ユーザは原則として MessageEnvelope / publish / observe / remote read/write を手書きしない。

ただし:

- 生成された通信は Core IR と devtools に必ず出す。
- visibility / capability / failure row / membership freshness を破る場合は reject。
- 自動 publish は visible fields のみ。
- contract-level witness は明示または policy で生成する。

## D6. Indexed state は S-owned map

```mir
S {
  state player[p: Participant]: Player
}
```

意味:

```text
S owns state map player.
Keyspace is Participant.
player[p] is an entry stored at S.
p is not owner and not write authority.
```

## D7. role claim は authority ではない

```mir
role BrowserClient {
  supports renderer.pose_v1
}

BrowserClient[self] {
  when start {
    join World as BrowserClient via WorldAdmission
  }
}
```

この role claim は自己申告であり、authority ではない。
WorldAdmission が membership / capability / witness を grant して初めて権限が得られる。

## D8. Source patch hot-plug は eval ではない

```text
.mir source patch
  -> parse
  -> typecheck
  -> elaborate
  -> compatibility check
  -> HotPlugRequest
  -> HotPlugVerdict
  -> activation_cut
  -> devtools trace
```

直接 eval しない。

## D9. C-like computational core を first target にする

最初の target:

```text
variables / records / arrays / functions / if / match / while / for / arithmetic
```

まだ入れない:

```text
full Rust ownership / traits / generics / async / F* dependent source language
```

## D10. 本番実用とほぼ同じ流れの意味

この implementation chain が完走した場合、期待する到達点は次。

```text
developer が `.mir` source を書く
  -> check-source
  -> run source-derived session
  -> patch-source で dynamic hot-plug
  -> local/Docker controlled runtime で server/browser-like roles を実行
  -> devtools で generated Core IR / communication / patch lifecycle / indexed state を確認
```

これは **本番実用にかなり近い運用 flow** である。
ただし **production final product** ではない。
final public SDK / WAN federation / distributed durable save-load / LLVM codegen / arbitrary native execution は後段。
