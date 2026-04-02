# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 task 先までの near-term roadmap を示す。
ここに書く step 数や task 数は厳密な約束ではなく、**rough estimate** である。

## いまから数 task の主眼

近い数 task の目的は、current L2 を次の状態へ持っていくことである。

- parser-free PoC を継続的に回せる
- notation の比較結果が docs / tests / fixtures と整合している
- helper stack の mirror drift が抑えられている
- parser 導入前に何が未決かを誤魔化さずに進められる

## 直近 2〜4 task の候補

### 候補 1. fallback / chain notation のさらに狭い polishing

- explicit edge-row family の companion notation をさらに読みやすくする
- E3 / E6 / E7 / E8 を使って drift を増やさないか確認する
- final parser grammar には進まない

### 候補 2. parser-free representative coverage の拡張

- current L2 semantics の重要点を、まだ fixture 化されていない narrow case で regression 化する
- static-only と runtime fixture のバランスを見る
- `must_explain` は prose に残し、machine-check 範囲を増やしすぎない

### 候補 3. parser 導入前の boundary inventory

- parser を書く前に最低限固定すべき syntax / companion notation / AST boundary を棚卸しする
- 何を final grammar 決定に回し、何をまだ比較候補に残すかを明確にする

### 候補 4. richer host interface の入口整理

- current host harness から production host interface へ直進しない
- まず、どの責務が current PoC helper にあり、どこから先が richer host interface なのかを切り分ける

## rough step estimate

以下は rough estimate であり、仕様 drift や review 指摘に応じて前後する。

| 目標 | rough step estimate | 注記 |
|---|---|---|
| PoC を継続的に回せる状態を維持しつつ drift regression を増やす | 2〜4 task | notation / fixture / helper mirror 次第で前後する |
| 文法をある程度比較しながら PoC を前進させる | 4〜8 task | final parser grammar 固定はまだ含まない |
| parser 導入判断の前提整理 | 5〜10 task | 静的解析や host interface との境界が影響する |

## いまの blocker

### 1. final parser grammar 未固定

- companion notation はかなり整理されたが、parser syntax はまだ未決である
- これを早く決めすぎると semantics より syntax が先行してしまう

### 2. fallback intuition drift

- guarded option chain 読みと outer-longer-lifetime 直感の tension は、まだ prose 補助が要る

### 3. heavier workstream の入口未整理

- 型システム、静的解析、決定可能性、theorem prover 連携は、まだ entry criteria の段階である

### 4. review infrastructure の変動

- reviewer completion が遅い / 返らない場合があり、task の closing evidence を自前で揃える必要がある

## いまの bottleneck

- syntax をどこまで companion notation のまま引っ張るか
- parser-free PoC の気持ちよさと final grammar 決定のタイミング
- helper stack を増やしすぎずに検証範囲を拡張すること

## 近い将来の sequencing

推奨順は次である。

1. semantics drift regression を増やす
2. notation / examples / helper mirror を揃える
3. parser 導入前 inventory を作る
4. richer host / trace / static analysis の入口を narrow に切る
5. その後で parser / richer runtime の判断に進む

## 今の working assumption

- current L2 semantics は大きく動かさない
- parser-free PoC は継続利用する
- hard-coded named profile catalog は維持する
- machine-readable catalog asset / manifest はまだ future option に留める

## 次にやるべき narrow-scope task 候補

- explicit edge-row companion notation の examples mirror をさらに細く整える
- fallback / `lease` regression fixture の補完を 1〜2 本だけ行う
- parser-free host harness と richer host interface の boundary inventory を作る
- parser 導入前の syntax decision inventory を plan と spec に切り出す
