# examples/15 — current L2 fallback semantic reconciliation and compact syntax

この文書は、current L2 parser-free PoC 基盤を前提に、fallback / `lease` / monotone degradation の semantic reconciliation と、fallback / preference chain の compact syntax candidate comparison をまとめる補助文書である。
ここで行うのは current L2 reading の整理と notation 候補の比較だけであり、final parser grammar、production manifest、runtime semantics の変更は行わない。

## この文書の位置づけ

- `specs/04-mir-core.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` にある current L2 fallback 読みを、representative examples と parser-free PoC fixture の観点から読み直す。
- 「寿命の長い外側 option へ fallback して使い続けたい」という直感と、current L2 が採る guarded option chain の読みが、どこで一致し、どこでズレるかを明示する。
- そのうえで `specs/examples/01-current-l2-surface-syntax-candidates.md` の chain declaration を補助する compact syntax candidate を 2〜3 案比較し、current L2 companion notation としてどこまで推せるかを整理する。

## current L2 の fallback 読み

current L2 で固定している最小 reading は次である。

- `fallback` は lifetime extension container ではなく、1 つの logical access path に対する guarded option chain として読む。
- canonical chain は left-to-right の優先順を表し、元の nested outer / inner 形そのものは保持しない。
- degradation は monotone であり、same semantic lineage で earlier option への再昇格を許さない。
- `lease-expired` は option-local miss metadata であり、later write-capable option があればそちらを試行できる。そこで success-side condition が満たされれば継続し、どの後段候補でも成立しなければ `Reject` になる。
- `try` / rollback / `atomic_cut` は local state や rollback frontier を制約しても、chain の degradation order 自体は巻き戻さない。

この reading は、E3 variant、E6、E7、E8 の parser-free fixture で machine-check されている。

## user intuition との一致点とズレ

### 比較表

| 論点 | current L2 reading | 「寿命の長い外側 option へ fallback して使い続けたい」直感 | 整理 |
|---|---|---|---|
| fallback の基本像 | guarded option chain | outer option が長寿命の包みになり、その内側で fallback する | ここが最も大きいズレである |
| `A > B > C` の向き | `A` から `B`、`C` へ left-to-right に degrade する | outer / inner nesting に意味が残り、`A` に戻れる余地がありそうに見える | current L2 は nested shape を flatten して優先順だけを残す |
| earlier option への再昇格 | 禁止 | outer option がまだ生きていれば戻れそうに見える | current L2 では戻らない |
| `lease-expired` 後の write | later write-capable option があればその候補を試行でき、どの候補でも成立しなければ `Reject` | 外側の長寿命 option に戻って継続したい | same-lineage の later option に進む限りは一致、earlier option へ戻る読みは不一致 |
| `try` / rollback / `atomic_cut` | local state は戻っても degradation order は戻らない | rollback で outer option へ戻れるように見える | current L2 では戻らない |

### 一致する部分

- 後段にまだ使える option があり、その option が同じ semantic lineage 上の admissible candidate なら、request を継続できる。
- write-after-expiry でも、later write-capable option が存在すればその候補を試行でき、success-side condition が満たされれば request を継続できる。
- fallback は hidden acceptance ではなく、明示された候補列に沿って進む。

### drift している部分

- current L2 は outer option を「寿命の長い外側 wrapper」として扱わない。
- nested syntax の内側 / 外側は canonical chain に flatten されるため、「外側だから最後に戻れる」という直感は current L2 では成立しない。
- rollback や cut を見たときに「戻れる」と感じやすいが、戻るのは local state であって degradation order ではない。

### `A > B > C` と nested 直感

current L2 では、`fallback(A, fallback(B, C))` でも `fallback(fallback(A, B), C)` でも、同じ logical access path / semantic lineage を共有し、monotone degradation を保つなら canonical chain `A > B > C` へ畳まれる。
このとき意味として残るのは「`A` が使えなければ `B`、さらに使えなければ `C`」だけであり、「`A` が outer だから長く生きる」という意味は残らない。

したがって、「`B` に `C` を fallback しつつ outer の `A` を寿命の長い避難先として保持する」読みは、current L2 とは両立しない。

## drift への 2 案比較

### 案1. current L2 semantics を維持し、drift を prose / examples / regression で明示する

この案では、guarded option chain と canonical flattening をそのまま維持する。

- 利点:
  - E3 / E6 / E7 / E8 の fixture と machine-check がそのまま使える。
  - rollback / `atomic_cut` / dynamic `Reject` の既存境界を崩さない。
  - parser-free PoC の evaluation state や step semantics に persisted degradation state を追加しなくてよい。
- 欠点:
  - outer / inner の視覚から lifetime extension を想像しやすいので、notation と prose で補助しないと誤読が残る。

### 案2. semantics 側を見直し、outer-longer-lifetime fallback を再導入する

この案では、nested outer / inner 形に寿命や再昇格の意味を戻す。

- 利点:
  - 「外側の option はまだ使い続けられる」という直感とは合わせやすい。
- 欠点:
  - canonical flattening を弱めるため、0019 の canonical normalization law と衝突する。
  - `try` / rollback / `atomic_cut` と degradation order の関係を再設計する必要がある。
  - `Reject` の境界、fixture schema、evaluation state、step semantics、host harness の読みを広く見直す必要がある。
  - current L2 の「minimal and machine-checkable」から外れやすい。

### current L2 として採るべき方針

current L2 では **案1を維持するのが妥当**である。

理由は次のとおりである。

- 最小性:
  - 既存の canonical law、rejection phase、static evidence floor、underdeclared handling と整合する。
- machine-checkability:
  - E3 / E6 / E7 / E8 ですでに drift regression を固定できている。
- rollback / cut 境界:
  - rollback が local state を巻き戻しても degradation order を巻き戻さない読みを保てる。
- `Reject` boundary:
  - later write-capable option を試行できる場合と、どの候補も成立せず `Reject` になる場合を明快に分けられる。

したがって、current L2 では semantics を変えるのではなく、「outer-longer-lifetime と読まない」ことを prose と notation で明示するのが最小である。

## compact syntax candidate comparison

以下は final parser syntax ではなく、current L2 companion notation の比較候補である。
主要比較は Candidate A と Candidate B に置き、Candidate C は補助比較対象としてだけ残す。

### Candidate A — explicit edge-row form

```text
chain profile_ref = writer
  fallback delegated_writer @ lineage(writer -> delegated_writer)
  fallback readonly @ lineage(delegated_writer -> readonly)
```

- 見た目:
  - もっとも説明的で、各行が「1 本の fallback edge」であることが明確。
- indentation:
  - 行の役割が一定で、インデントとの相性は良い。
- vertical reading:
  - 上から下へ degradation order を追える。
- non-C-ness:
  - operator 連結ではなく、statement / continuation 風で C 的ではない。
- 欠点:
  - `fallback` の反復がやや重く、compact さでは一歩譲る。

### Candidate B — ladder form with leading `>`

```text
chain profile_ref:
  writer
  > delegated_writer @ lineage(writer -> delegated_writer)
  > readonly @ lineage(delegated_writer -> readonly)
```

- 見た目:
  - かなり簡潔で、視線を縦に落としやすい。
- indentation:
  - head と後続候補の階段が分かりやすい。
- vertical reading:
  - degradation の「下方向」感が最も強い。
- non-C-ness:
  - infix expression ではなく line-leading に使えば C っぽさは弱められる。
- 欠点:
  - `>` が operator 風に見えやすく、canonical chain を expression と誤読させる余地がある。
  - edge-local annotation を「行ごとの successor metadata」ではなく「チェーン演算子の付属情報」に見せやすい。

### Candidate A / B の直接比較

| 比較軸 | Candidate A: explicit edge-row | Candidate B: line-leading `>` ladder | 現時点の評価 |
|---|---|---|---|
| 視覚的な簡潔さ | `fallback` の反復があり、やや重い | 最も短く、視線を落としやすい | B が優勢 |
| インデントとの相性 | continuation line として素直 | 階段状で見やすいが、`chain profile_ref:` への head 変更も伴う | ほぼ互角、見た目は B がやや優勢 |
| 縦方向の流れ | 上から下へ読める | degradation の下方向感が最も強い | B が優勢 |
| outer / inner 直感の抑制 | `fallback` 行の反復で「edge を並べている」と読める | flatten された列に見えやすく、nested wrapper 直感は減る | ここだけ見ると B も有力 |
| guarded option chain の honesty | row ごとに fallback edge と lineage が見える | ordered expression や compact algebra に見えやすい | A が優勢 |
| rollback / cut / `Reject` 境界の誤読 | 後段候補を順に試す request evaluation と読みやすい | compile-time canonical order や total ordering の印象が先に立ちやすい | A が優勢 |
| earlier option への再昇格禁止 | edge ごとの monotone successor として読みやすい | `>` を単なる優先順位記号と取ると、runtime re-entry 不可が prose 依存になりやすい | A が優勢 |
| examples の書き換えコスト | 既存 mirror をそのまま使える | `specs/examples/00`、`specs/examples/01`、`specs/examples/15`、関連 report の prose mirror をまとめて触る必要がある | A が優勢 |
| C 的 / AST エンコード感の少なさ | statement / declaration 風で無難 | operator 感は残るが、line-leading にすればかなり抑えられる | 互角に近いが A が安全 |

この比較から、**B は見た目の compactness と vertical reading では魅力があるが、current L2 で必要な edge-locality と request-evaluation 境界を薄めやすい**、というのが最小整理になる。

### Candidate C — prose-like `then` ladder

```text
chain profile_ref:
  writer
  then delegated_writer @ lineage(writer -> delegated_writer)
  then readonly @ lineage(delegated_writer -> readonly)
```

- 見た目:
  - `>` より柔らかく、operator 感が薄い。
- indentation:
  - 縦方向の流れは保てる。
- vertical reading:
  - 読みやすいが、edge row というより narrative sequence に寄る。
- non-C-ness:
  - 3 案の中で最も C 的でない。
- 欠点:
  - compact さでは Candidate A/B に劣る。
  - `then` が「次に実行する」という control-flow 風に見え、guarded option chain より通常の処理順に寄ってしまう。

## provisional current L2 companion notation の判断

current L2 で **いま暫定的に最も筋が良いのは Candidate A の explicit edge-row form** である。

理由は次のとおりである。

- current semantics を無理なく表す:
  - chain が outer lifetime container ではなく、edge-local lineage を持つ canonical candidate list であることが最も素直に見える。
- parser 圧を上げにくい:
  - 新しい operator 風 token を持ち込まずに済む。
- drift を減らせる:
  - `fallback` を毎行に残すことで、「ここでやっているのは nested control-flow ではなく successor declaration だ」と読みやすい。
- examples mirror を安定に保てる:
  - representative examples、surface candidate 文書、0079/0080 で固めた prose を最小変更で維持できる。

一方で **将来 compact 化を再検討するなら Candidate B が最有力**である。
ただし current L2 では、`>` の operator 感、canonical chain の expression 化、edge-local lineage の付属情報化を避けたいので、まだ昇格させない。

## current L2 に残すこと / まだ決めないこと

### current L2 に残すこと

- semantics は guarded option chain のまま維持する。
- degradation は left-to-right monotone degradation として読む。
- earlier option への再昇格は禁止する。
- `lease-expired` の後に later write-capable option があればその候補を試行し、どの候補も成立しなければ `Reject`。
- compact syntax 比較の結果として、current L2 companion notation は explicit edge-row form を暫定維持する。

### まだ決めないこと

- final parser syntax
- compact syntax candidate を本当に parser grammar へ昇格させるか
- machine-readable catalog asset / manifest
- selector grammar / alias grammar の長期固定
- path canonicalization policy
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`
