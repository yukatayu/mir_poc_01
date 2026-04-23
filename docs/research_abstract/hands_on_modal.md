# hands-on: modal

この文書は、Mir current-L2 の clean near-end sample suite のうち、`modal` family を初めて読む人向けに説明するハンズオンです。

ここでいう `modal` は、「値がどの段階、どの場所、どの公開状態で使えるか」を表す読み方です。
数学的には modal type の系譜がありますが、この handson では専門用語を先に使わず、sample の動きから説明します。

## 最初に結論

現在 repo で実行できることは次です。

- `stable` な config を stage の中で使う sample が通る。
- `later` に作られた値を後の stage で publish する sample が通る。
- `published(room)` の値を witness と結び、`witnessed(draw_pub)` へ bridge する sample が通る。
- raw `◯` や `□` は final public syntax として exposed していない。
- current source では `stable`、`later`、`published(room)`、`witnessed(...)` のような mode vocabulary を使う。

## 使うコマンド

```bash
python3 scripts/clean_near_end_samples.py run modal --format json
```

## 言葉の準備

`mode` は、値がどの状態で使えるかを表す印です。
たとえば「今すぐ使える」「後の stage で使える」「公開済み」「witness で確認済み」のような違いを表します。

`stable` は、stage が変わっても安定して使えるものです。
ここでは `GameConfig` のような設定を stable として読みます。

`later` は、今この瞬間ではなく、後の stage で使える値です。
サイコロを振って得た `draw` は、次の publish stage で使う値として `later` と示されます。

`published(room)` は、値が `room` scope に公開された状態です。

`witnessed(draw_pub)` は、`draw_pub` という witness によって確認済みの状態です。

`bridge` は、ある mode の値を別の mode と対応づける操作です。
ここでは、公開済みの値を witness と結び、後続で確認済みとして扱えるようにします。

## 何が組み込みで、何が組み込みではないか

この handson で読む current sample vocabulary は主に次です。
ここでの built-in は、current helper が認識している語彙という意味です。
final public parser grammar や reserved keyword set を固定するものではありません。

- `module`: sample の名前空間です。
- `stable`: 安定して使える値を宣言します。
- `transition`: 処理全体です。
- `stage`: 処理の段階です。
- `use stable`: stable な値を stage 内で使います。
- `perform`: effectful operation を呼びます。
- `produces value ... @ later`: 後の stage で使える値を作ります。
- `publish`: 値を公開します。
- `requires value ... @ later`: later mode の値が必要だと示します。
- `produces witness`: witness を作ります。
- `bridge`: mode 間の対応を作ります。
- `from published(room)`: 公開済み mode から始めます。
- `to witnessed(draw_pub)`: witness 済み mode へ移します。

一方で、次は sample 固有の名前です。

- `GameConfig`
- `config`
- `draw`
- `draw_pub`
- `room`

## Step 1: stable と later の最小例を読む

最小例は `samples/clean-near-end/modal/01_stage_stable_later_minimal.mir` です。

```mir
module CleanNearEnd.StageStableLaterMinimal

stable config : GameConfig

transition staged_roll {
  stage prepare:
    cfg <- use stable config

  stage roll:
    draw <- perform roll_dice(cfg)
      produces value draw @ later

  stage publish:
    publish draw
      requires value draw @ later
      produces witness draw_pub
}
```

行ごとの読み方です。

- `module CleanNearEnd.StageStableLaterMinimal` は、この sample の名前です。
- `stable config : GameConfig` は、`config` が安定して使える設定値だと宣言します。
- `transition staged_roll` は、段階を持つ処理を宣言します。
- `stage prepare:` は、準備段階です。
- `cfg <- use stable config` は、stable な `config` を取り出し、`cfg` という名前で使います。
- `stage roll:` は、サイコロを振る段階です。
- `draw <- perform roll_dice(cfg)` は、`cfg` を使ってサイコロを振り、結果を `draw` に入れます。
- `produces value draw @ later` は、`draw` が後の stage で使える値として作られたことを示します。
- `stage publish:` は、公開段階です。
- `publish draw` は、`draw` を公開します。
- `requires value draw @ later` は、公開するには `draw` が later mode で利用可能である必要があるという条件です。
- `produces witness draw_pub` は、公開の witness を `draw_pub` として作ります。

ここでの重要点は、「stage をまたいで値を使ってよい理由」を mode と witness で明示することです。
隠れた global variable として値を使うのではありません。

## Step 2: stable / later sample を実行する

```bash
python3 scripts/clean_near_end_samples.py run modal --format json
```

出力の重要部分です。

```json
{
  "sample": "01_stage_stable_later_minimal",
  "static_verdict": "valid",
  "mode_constraints": [
    "config : stable",
    "draw available at later stage",
    "publish requires later draw and produces witness"
  ]
}
```

読み方です。

- `static_verdict: valid` は、mode constraint が通ったことを示します。
- `config : stable` は、config が stable として扱われたことを示します。
- `draw available at later stage` は、roll stage の結果が後の stage で使えることを示します。
- `publish requires later draw and produces witness` は、publish が later value を要求し、witness を作ることを示します。

## Step 3: published から witnessed への bridge を読む

次は `samples/clean-near-end/modal/02_published_witnessed_mode_bridge.mir` です。

```mir
module CleanNearEnd.PublishedWitnessedModeBridge

transition published_to_witnessed_bridge {
  stage publish:
    publish draw
      scope room
      produces witness draw_pub
      produces value draw @ published(room)

  stage bridge:
    bridge draw
      from published(room)
      to witnessed(draw_pub)
      requires witness(draw_pub)
}
```

行ごとの読み方です。

- `module CleanNearEnd.PublishedWitnessedModeBridge` は、この sample の名前です。
- `transition published_to_witnessed_bridge` は、公開から witness 済み mode へ進む処理です。
- `stage publish:` は、値を公開する段階です。
- `publish draw` は、`draw` を公開します。
- `scope room` は、公開先が `room` scope であることを示します。
- `produces witness draw_pub` は、公開の witness を作ります。
- `produces value draw @ published(room)` は、`draw` が `room` に公開済みの値になったことを示します。
- `stage bridge:` は、mode を橋渡しする段階です。
- `bridge draw` は、`draw` の mode を対応づけます。
- `from published(room)` は、出発点が `room` に公開済みの mode であることを示します。
- `to witnessed(draw_pub)` は、到達点が `draw_pub` によって確認済みの mode であることを示します。
- `requires witness(draw_pub)` は、その bridge には witness が必要だと示します。

この sample では、「公開されたから何でも自由に使える」とは読ませません。
公開済みであることと、witness によって確認済みであることを、別の mode として扱います。

## Step 4: lambda-circle-box との対応を読む

古典的な説明では、次のような記法が出ることがあります。

```text
◯ A
□ A
```

この repo の current active sample では、これらを final public syntax として露出していません。
説明上は、概ね次の対応として読みます。

```text
◯ A  ≈  A @ later
□ A  ≈  A @ stable
```

つまり、

- `later` は「後で使える」。
- `stable` は「段階をまたいでも安定して使える」。
- `published(room)` は「room に公開済み」。
- `witnessed(draw_pub)` は「draw_pub で確認済み」。

という current Mir vocabulary に持ち上げています。

## この handson で確認できたこと

`modal` family は、以下を現在の repo で確認できます。

- stable value を stage 内で使える。
- later value を後続 stage で要求できる。
- publish が witness を作る。
- published mode と witnessed mode を bridge できる。
- raw `◯` / `□` を final public syntax として固定していない。

## まだ final product ではないこと

次はまだ deferred です。

- final modal source syntax。
- final parser grammar。
- full multimodal type theory。
- all mode bridge proof obligations の complete theorem discharge。
- public verifier API。

この handson の到達点は、「repo-local alpha-ready current layer で、stage / stable / later / published / witnessed の基本 sample が実行できる」です。
