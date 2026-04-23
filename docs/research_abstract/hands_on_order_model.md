# hands-on: order_model

この文書は、Mir current-L2 の clean near-end sample suite のうち、`order-handoff` family を初めて読む人向けに説明するハンズオンです。

ここで扱う中心は、「低レベルの `memory_order_release` などをそのまま source 側の中心概念にするのではなく、公開、観測、証拠、引き渡しという高水準の関係として読む」ことです。

## 最初に結論

現在 repo で実行できることは次です。

- source sample は `publication_order`、`witness_order`、`scoped_happens_before` のような高水準関係で読む。
- `std::memory_order` の名前は source 側の中心語彙ではなく、backend/reference family に残す。
- handoff には、公開が先に起きたことと witness が必要。
- witness がない handoff は `malformed`。
- publication より前の handoff も `malformed`。
- これは order/handoff の repo-local alpha 実装であり、final public memory model syntax ではない。

## 使うコマンド

```bash
python3 scripts/clean_near_end_samples.py run order-handoff --format json
```

sample 一覧を見たい場合は次です。

```bash
python3 scripts/clean_near_end_samples.py list
```

## 言葉の準備

`order` は、出来事の前後関係です。
たとえば「サイコロを振る」より後に「結果を公開する」が来ます。

`handoff` は、所有者や操作権を別の actor へ渡すことです。
ここでは `dice_owner Alice -> Bob` のように書きます。

`publication` は、値をある scope に公開することです。
scope は「どの範囲に見えるか」を示します。

`witness` は、ある出来事が起きたことを後続の処理が確認するための証拠です。
日本語では「証拠」「受領証」「確認材料」のように読めます。

`scoped_happens_before` は、ある scope の中で「A は B より先に起きた」と言える関係です。
この名前は低レベル memory model の happens-before から直感を借りていますが、source sample ではより高水準の関係として使います。

`memory_order` は、C++ などの低レベル並行プログラミングで使われる語です。
current Mir sample は、その名前を source の中心にしません。
理由は、Mir が扱いたいのは CPU 命令そのものではなく、「何が公開され、誰が観測し、どの証拠で引き渡せるか」だからです。

## 何が組み込みで、何が組み込みではないか

この handson で読む current sample vocabulary は主に次です。
ここでの built-in は、current helper が認識している語彙という意味です。
final public parser grammar や reserved keyword set を固定するものではありません。

- `module`: sample の名前空間です。
- `use`: 共有前提を使う語です。
- `principal`: actor や user のような主体です。
- `resource`: 所有や lifetime を持つ対象です。
- `transition`: 処理全体です。
- `stage`: 処理の段階です。
- `perform`: effectful operation を呼びます。
- `publish`: 値を scope へ公開します。
- `produces witness`: witness を作ります。
- `handoff`: 所有や操作権を渡します。
- `after`: ある出来事の後であることを要求します。
- `requires witness`: witness があることを要求します。

一方で、次は組み込みではありません。

- `Alice`
- `Bob`
- `dice_owner`
- `authority_rng`
- `room`
- `draw_pub`

これらは sample が宣言または参照する domain-specific な名前です。

## Step 1: 成功する handoff を読む

成功例は `samples/clean-near-end/order-handoff/01_authorized_roll_publish_handoff.mir` です。

```mir
module CleanNearEnd.AuthorizedRollPublishHandoff

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Admin
principal Bob   : FingerprintAuthority.Holder

resource dice_owner : Player
  authority_scope room
  lifetime Session

transition handoff_turn(owner = Alice) {
  stage roll:
    draw <- perform roll_dice via authority_rng

  stage publish:
    publish draw
      scope room
      produces witness draw_pub

  stage handoff:
    handoff dice_owner Alice -> Bob
      after publish(draw)
      requires witness(draw_pub)
}
```

行ごとの読み方です。

- `module CleanNearEnd.AuthorizedRollPublishHandoff` は、この sample の名前です。
- `use CleanNearEnd.IndexTheories` は、共有された authority や lifetime の前提を使います。
- `principal Alice : FingerprintAuthority.Admin` は、Alice が高い権限を持つ主体だと宣言します。
- `principal Bob : FingerprintAuthority.Holder` は、Bob が holder 権限を持つ主体だと宣言します。
- `resource dice_owner : Player` は、サイコロの所有者を表す resource を宣言します。
- `authority_scope room` は、この resource の authority が `room` scope と関係することを示します。
- `lifetime Session` は、session の間有効だと示します。
- `transition handoff_turn(owner = Alice)` は、Alice が owner として行う処理です。
- `stage roll:` は、サイコロを振る段階です。
- `draw <- perform roll_dice via authority_rng` は、`authority_rng` を通して乱数またはサイコロ結果を得ます。
- `stage publish:` は、結果を公開する段階です。
- `publish draw` は、`draw` を公開します。
- `scope room` は、公開範囲が room であることを示します。
- `produces witness draw_pub` は、公開したことの witness を `draw_pub` という名前で作ります。
- `stage handoff:` は、所有や操作権を渡す段階です。
- `handoff dice_owner Alice -> Bob` は、`dice_owner` を Alice から Bob へ渡します。
- `after publish(draw)` は、handoff が `draw` の公開後であることを要求します。
- `requires witness(draw_pub)` は、公開 witness を持っていることを要求します。

大事なのは、handoff が単なる代入ではないことです。
「公開が先」「証拠がある」「同じ scope で先に起きたと読める」という関係が必要です。

## Step 2: 成功例を実行する

```bash
python3 scripts/clean_near_end_samples.py run order-handoff --format json
```

出力の重要部分です。

```json
{
  "sample": "01_authorized_roll_publish_handoff",
  "static_verdict": "valid",
  "terminal_outcome": "success",
  "relations": [
    ["program_order", "roll", "publish"],
    ["publication_order", "roll", "publish"],
    ["witness_order", "publish", "handoff"],
    ["scoped_happens_before", "roll", "handoff"]
  ]
}
```

読み方です。

- `program_order` は、program に書かれた stage の順序です。
- `publication_order` は、公開に関する順序です。
- `witness_order` は、witness が後続 stage に渡る順序です。
- `scoped_happens_before` は、scope 内で roll が handoff より先だと読める関係です。

この出力は、「低レベル memory order 名を直接使わず、高水準の公開と witness の関係から順序を作る」ことを示します。

## Step 3: witness がない失敗例を見る

`samples/clean-near-end/order-handoff/02_missing_witness_rejected.mir` は、公開はしているが witness を作らない例です。

重要部分は次です。

```mir
transition bad_handoff_missing_witness(owner = Alice) {
  stage roll:
    draw <- perform roll_dice via authority_rng

  stage publish:
    publish draw
      scope room

  stage handoff:
    handoff dice_owner Alice -> Bob
      after publish(draw)
}
```

読み方です。

- `publish draw` はあります。
- しかし `produces witness draw_pub` がありません。
- handoff 側にも `requires witness(draw_pub)` がありません。
- checker は「handoff の根拠が足りない」と判断します。

出力の重要部分です。

```json
{
  "sample": "02_missing_witness_rejected",
  "static_verdict": "malformed",
  "reason_family": "missing_handoff_witness",
  "entered_evaluation": false
}
```

`entered_evaluation: false` は、実行前に止めたという意味です。
不確かな handoff を実行してから失敗したわけではありません。

## Step 4: handoff が早すぎる失敗例を見る

`samples/clean-near-end/order-handoff/03_handoff_before_publication_rejected.mir` は、handoff を publish より前に書く例です。

```mir
module CleanNearEnd.HandoffBeforePublicationRejected

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Admin
principal Bob   : FingerprintAuthority.Holder

resource dice_owner : Player
  authority_scope room
  lifetime Session

transition bad_handoff_before_publish(owner = Alice) {
  stage roll:
    draw <- perform roll_dice via authority_rng

  stage handoff:
    handoff dice_owner Alice -> Bob
      requires witness(draw_pub)

  stage publish:
    publish draw
      scope room
      produces witness draw_pub
}
```

行ごとの重要点です。

- `stage roll` で `draw` を作ります。
- 次に `stage handoff` が来ています。
- `handoff` は `requires witness(draw_pub)` と書いています。
- しかし `draw_pub` はその後の `stage publish` で作られます。
- したがって handoff の時点では witness がまだありません。
- checker は `handoff_before_publication` として拒否します。

出力の重要部分です。

```json
{
  "sample": "03_handoff_before_publication_rejected",
  "static_verdict": "malformed",
  "reason_family": "handoff_before_publication",
  "entered_evaluation": false
}
```

## Step 5: memory_order 再解釈として読む

この family の目的は、C++ のような低レベル構文を Mir source の中心にすることではありません。

低レベルの考え方では、たとえば release / acquire / seq_cst などの名前で、読み書きの見え方を制御します。
Mir current-L2 の clean sample では、それを直接 source 側の中心概念にせず、次のように言い換えます。

- 何を公開したか。
- どの scope に公開したか。
- 誰がそれを観測できるか。
- 後続の処理が witness を要求しているか。
- handoff が publication より後にあるか。
- それらから scoped happens-before が組み立てられるか。

この読み替えにより、thread と node を同じ causal language で説明しやすくなります。
ただし lowering、transport、failure、durability、policy は同じではありません。
つまり「thread と distributed node が完全に同じ」という意味ではありません。

## この handson で確認できたこと

`order-handoff` family は、以下を現在の repo で確認できます。

- 公開後に witness を作り、その witness を要求する handoff は通る。
- witness がない handoff は落ちる。
- witness を作る前の handoff は落ちる。
- high-level relation family から `scoped_happens_before` を観察できる。
- この handson が詳しく読むのは `01` から `03` の最小 handoff case です。
- active runner には追加で `04_stage_block_authorized_handoff`、`05_delegated_rng_service`、`06_auditable_authority_witness` も含まれます。
- `memory_order_*` を source 側の中心構文として固定していない。

## まだ final product ではないこと

次はまだ deferred です。

- final public memory model syntax。
- final public witness object schema。
- final public provider / artifact contract。
- production lowering to backend memory order。
- distributed transport / durability semantics の完全実装。

この handson の到達点は、「repo-local alpha-ready current layer で、order/handoff の active sample を実行し、高水準 order relation と rejection case を確認できる」です。
