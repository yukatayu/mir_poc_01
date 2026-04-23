# hands-on: model_checking

この文書は、Mir current-L2 の clean near-end sample suite のうち、`model-check` family を初めて読む人向けに説明するハンズオンです。

ここで扱うのは、mutex や weak memory のように、有限の型制約だけでは安全性を言い切れない問題です。
この repo では、小さな有限表で全部調べられる検査を first line と呼びます。
一方で、並行実行の順序のように実行パターンを探索する必要があるものは second line と呼び、model checking に分けています。

## 最初に結論

現在 repo で実行できることは次です。

- Peterson の相互排除アルゴリズムを sequential consistency で確認する sample は `pass`。
- publication / observation edge がない relaxed model では counterexample が出る。
- broken mutex でも counterexample が出る。
- これらは「型検査だけで全部解く」問題ではなく、model-check second line の問題として扱う。

## 使うコマンド

```bash
python3 scripts/clean_near_end_samples.py run model-check --format json
```

## 言葉の準備

`model checking` は、あり得る状態や実行順序を調べて、性質が破れる例がないか探す方法です。
ここでは「2 人が同時に critical section に入らないか」を調べます。

`property` は、守りたい性質です。
この sample では `mutual_exclusion`、つまり相互排除です。

`mutual exclusion` は、同時に 2 人が critical section に入らないという性質です。
日本語では「排他的に 1 人だけ入れる」と読めます。

`critical section` は、同時に複数人が入ると壊れる領域です。
たとえば同じ共有データを書き換える部分です。

`counterexample` は、守りたい性質を破る具体的な実行例です。
単なるエラー名ではなく、「この順で読めると両方が入ってしまう」という証拠です。

`memory model` は、書き込みが他の actor からいつ見えるかを決める規則です。
`sequential_consistency` は、全 actor の操作が 1 本の順序に並んで見える強い model です。
`relaxed_without_publication_observation_edges` は、書いた値が相手からすぐ見えるとは限らない弱い model です。

## 何が組み込みで、何が組み込みではないか

この handson で読む current sample vocabulary は主に次です。
ここでの built-in は、current helper が認識している語彙という意味です。
final public parser grammar や reserved keyword set を固定するものではありません。

- `module`: sample の名前空間です。
- `model`: model-check 対象を宣言します。
- `actor`: 並行に動く参加者を宣言します。
- `shared`: 共有変数を宣言します。
- `process`: actor が実行する手順を宣言します。
- `wait while`: 条件が成り立つ間待つ操作です。
- `critical`: critical section に入る marker です。
- `memory_model`: 見え方の規則を選びます。
- `property`: 守りたい性質を宣言します。
- `never`: その状態が起きてはならないことを表します。

一方で、次は sample 固有の名前です。

- `PetersonSC`
- `PetersonRelaxedNoPublication`
- `A`
- `B`
- `flag`
- `turn`
- `mutual_exclusion`

## Step 1: sequential consistency で通る例を読む

成功例は `samples/clean-near-end/model-check/01_peterson_sc_pass.mir` です。

```mir
module CleanNearEnd.PetersonScPass

model PetersonSC {
  actor A
  actor B

  shared flag[A] : Bool = false
  shared flag[B] : Bool = false
  shared turn : Actor = A

  process A {
    flag[A] <- true
    turn <- B
    wait while flag[B] == true and turn == B
    critical A
    flag[A] <- false
  }

  process B {
    flag[B] <- true
    turn <- A
    wait while flag[A] == true and turn == A
    critical B
    flag[B] <- false
  }

  memory_model sequential_consistency

  property mutual_exclusion:
    never (in_critical(A) and in_critical(B))
}
```

行ごとの読み方です。

- `module CleanNearEnd.PetersonScPass` は、この sample の名前です。
- `model PetersonSC` は、Peterson algorithm を sequential consistency で調べる model です。
- `actor A` と `actor B` は、同時に動く 2 人の参加者です。
- `shared flag[A] : Bool = false` は、A が入りたいかどうかを示す共有 boolean です。
- `shared flag[B] : Bool = false` は、B が入りたいかどうかを示す共有 boolean です。
- `shared turn : Actor = A` は、同時に入りたい場合にどちらへ譲るかを示す共有変数です。
- `process A` は、A が実行する手順です。
- `flag[A] <- true` は、A が「入りたい」と意思表示します。
- `turn <- B` は、競合したら B に譲ると書きます。
- `wait while flag[B] == true and turn == B` は、B も入りたくて、かつ B の番なら待つという意味です。
- `critical A` は、A が critical section に入ったことを示す marker です。
- `flag[A] <- false` は、A が出たので意思表示を下げます。
- `process B` は、B 側の対称な手順です。
- `memory_model sequential_consistency` は、全操作が 1 本の順序に並ぶ強い見え方で調べるという意味です。
- `property mutual_exclusion:` は、守りたい性質の名前です。
- `never (in_critical(A) and in_critical(B))` は、A と B が同時に critical section にいる状態は起きてはならないという意味です。

## Step 2: sequential consistency の結果を見る

```bash
python3 scripts/clean_near_end_samples.py run model-check --format json
```

出力の重要部分です。

```json
{
  "sample": "01_peterson_sc_pass",
  "model_check_result": "pass",
  "property": "mutual_exclusion",
  "checked_under": "sequential_consistency"
}
```

読み方です。

- `model_check_result: pass` は、指定した範囲では property を破る counterexample が見つからなかったという意味です。
- `property: mutual_exclusion` は、調べた性質です。
- `checked_under: sequential_consistency` は、強い memory model の下で調べたという意味です。

## Step 3: relaxed memory で破れる例を読む

次は `samples/clean-near-end/model-check/02_peterson_relaxed_counterexample.mir` です。
構造は PetersonSC とほぼ同じですが、memory model が違います。

```mir
module CleanNearEnd.PetersonRelaxedCounterexample

model PetersonRelaxedNoPublication {
  actor A
  actor B

  shared flag[A] : Bool = false
  shared flag[B] : Bool = false
  shared turn : Actor = A

  process A {
    flag[A] <- true
    turn <- B
    wait while flag[B] == true and turn == B
    critical A
    flag[A] <- false
  }

  process B {
    flag[B] <- true
    turn <- A
    wait while flag[A] == true and turn == A
    critical B
    flag[B] <- false
  }

  memory_model relaxed_without_publication_observation_edges

  property mutual_exclusion:
    never (in_critical(A) and in_critical(B))
}
```

重要な違いは 1 行です。

```mir
memory_model relaxed_without_publication_observation_edges
```

これは、「書いた flag が相手にすぐ観測される」という保証を置かない model です。
A が `flag[A] <- true` と書いても、B がその値をまだ見ていない可能性があります。
B についても同じです。

## Step 4: counterexample を見る

出力の重要部分です。

```json
{
  "sample": "02_peterson_relaxed_counterexample",
  "model_check_result": "counterexample",
  "property": "mutual_exclusion",
  "checked_under": "relaxed_without_publication_observation_edges",
  "counterexample_shape": [
    "A writes flag[A] but B has not observed it",
    "A writes turn <- B",
    "A reads flag[B] as false",
    "B writes flag[B] but A has not observed it",
    "B writes turn <- A",
    "B reads flag[A] as false"
  ]
}
```

この counterexample を日本語で読むと次です。

1. A は `flag[A]` を true にしたが、B からはまだ見えていない。
2. A は `turn <- B` と書く。
3. A は `flag[B]` を false と読んでしまう。
4. B も `flag[B]` を true にしたが、A からはまだ見えていない。
5. B は `turn <- A` と書く。
6. B は `flag[A]` を false と読んでしまう。

この shape は critical entry 直前までの visibility failure を要約しています。
両者が相手の flag を観測できないまま待ち条件を抜ける道筋を示します。
その結果、`never (in_critical(A) and in_critical(B))` を破る counterexample として扱われます。

## Step 5: broken mutex を見る

`samples/clean-near-end/model-check/03_broken_mutex_counterexample.mir` は、より直接的に壊れた mutex の例です。

この sample の目的は、「この種のバグは finite-index typing だけで片付けるより、model-check second line で見るべきだ」と示すことです。

出力の重要部分です。

```json
{
  "sample": "03_broken_mutex_counterexample",
  "model_check_result": "counterexample",
  "property": "mutual_exclusion",
  "checked_under": "interleaving_sc",
  "counterexample_shape": [
    "A checks flag[B] and sees false",
    "B checks flag[A] and sees false",
    "A sets flag[A]",
    "A enters critical",
    "B sets flag[B]",
    "B enters critical"
  ],
  "explanation": "interleaving or visibility permits both actors to enter critical section"
}
```

`interleaving` は、A の操作と B の操作がどの順番で混ざるかという意味です。
`visibility` は、ある actor の書き込みが別の actor から見えているかという意味です。

この sample の active output は `checked_under: "interleaving_sc"` です。
counterexample は、A と B が互いの flag を false と読んでから critical section に入る interleaving を示します。
つまり、少なくともこの sample では、順番の混ざり方だけで相互排除が破れることを確認しています。

## なぜ typing ではなく model-check second line なのか

typing は、有限の label、authority、capture、region、cost のような static constraint を解くのに向いています。

一方で、mutex の安全性は次に依存します。

- A と B の操作がどの順番で交ざるか。
- 書き込みが相手にいつ見えるか。
- 待ち条件がどの値を読んだか。
- critical section に入る marker が同時に立つ実行があるか。

これは「`Observer <= Releaser` が true か false か」のような単純な有限 preorder 比較ではありません。
そのため current repo は、型検査の first line と model-check の second line を分けています。

## この handson で確認できたこと

`model-check` family は、以下を現在の repo で確認できます。

- sequential consistency の下では Peterson sample が `pass` する。
- relaxed memory では counterexample が出る。
- broken mutex でも counterexample が出る。
- mutex / weak-memory は model-check second line として扱う。

## まだ final product ではないこと

次はまだ deferred です。

- production model checker binding。
- full state-space exploration の public API。
- counterexample artifact の final schema。
- source language から checker への final lowering。
- memory order family の final public syntax。

この handson の到達点は、「repo-local alpha-ready current layer で、mutex safety が typing first line ではなく model-check second line に置かれていることを実行結果で確認できる」です。
