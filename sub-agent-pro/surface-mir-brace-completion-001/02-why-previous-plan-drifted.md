# 02 — Why The Previous Plan Drifted

## 1. JSON / adapter に意味が逃げた

以前の Product Alpha では、`AddOne` が `typed_host_io.add_one` として external adapter 側にあった。これは typed host boundary の証拠としては有用だったが、Mir-owned computation の証拠ではなかった。

問題:

```text
AddOne が動く = Mir が計算した
```

と読めてしまう。

修正:

```text
host read -> Mir-owned transform -> host write
```

へ分ける。

## 2. Core-facing syntax を Surface と混同した

`transition ... at ...`、`perform ... via ...`、`publish`、`observe`、`witness` は Core Mir としては良い。
しかし user-facing primary syntax として全面に出すと、元の目的である「通信をほぼ考えずに system-wide meaning を書く」体験から離れる。

修正:

```text
Surface Mir:
  S { ... }
  A { when attack { S { ... } } }

Core Mir:
  transition / perform / publish / observe / witness
```

## 3. `S[ ... ]` を canonical にしようとして配列と衝突した

初期ブログの `S[...]` は直感として優れている。しかし `[]` は将来、配列、Map、IndexedState、Role instance、Place-valued expression と衝突する。

修正:

```text
Canonical: S { ... }
No S[] sugar.
```

## 4. Indexed state を曖昧にした

`S { [A] var a }` をそのまま読むと、A にある変数なのか S が A 用に持つ変数なのかが曖昧になる。

修正:

```mir
S {
  state player[p: Participant]: Player
}
```

これは S-owned Participant-indexed map。

## 5. role claim と authority を混ぜそうになった

「自分は browser client です」という claim は権限ではない。権限は parent/root/admission node が grant する。

修正:

```text
role claim != capability grant
```

## 6. 今回の修正で守るべき project axis

```text
Mir source files が意味の正本。
各 server / browser-like runtime は Mir source 由来 artifact を実行。
通信・publish・observe は自動生成。
権限・可視性・失敗・cut は型と contract に現れる。
外部 backend は semantic owner ではない。
```
