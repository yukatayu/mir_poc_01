# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 task 先までの near-term roadmap を示す。
ここに書く step 数や task 数は厳密な約束ではなく、**rough estimate** である。

## いまから数 task の主眼

近い数 task の目的は、current L2 を次の状態へ持っていくことである。

- parser-free PoC を継続的に回せる
- parser-free PoC の実行結果を process 内比較だけに閉じ込めない
- trace / audit と host coverage の operational boundary を semantics から独立に狭く切る
- notation の比較結果が docs / tests / fixtures と整合している
- helper stack の mirror drift が抑えられている
- parser 導入前に何が未決かを誤魔化さずに進められる

## 直近 2〜4 task の候補

### 候補 1. detached trace / audit serialization の最小境界整理

- docs-only minimal schema は切れたので、次は `TraceAuditSink` / `RunReport` / bundle summary から thin export boundary をどう置くかを narrow に棚卸しする
- repo 外保存・再比較・後解析に必要な field を、exact-compare core / detached non-core / human-facing explanation に分けて運ぶ
- `must_explain` は引き続き prose obligation に残し、exact compare の core を増やしすぎない
- その比較では、payload core は `RunReport` に寄せつつ、first exporter entry は `run_bundle` / `BundleRunReport` に置くのが current understanding である
- さらに bundle-first artifact の内部では、`fixture_id` / `fixture_path` / `host_plan_path` / `runtime_requirement` を `bundle_context` へ寄せ、`host_plan_coverage_failure` は aggregate-only に残すのが次の narrow step である
- その次の narrow comparison として、`host_plan_coverage_failure` を将来 typed carrier に昇格させるなら bundle failure artifact 側へ切るのが自然かどうかを確認する
- さらにその docs-only refinement として、bundle failure artifact 側へ切る typed carrier の最小 schema は `failure_kind` discriminator だけに留め、`bundle_context` や short note を typed core に混ぜないのが current understanding である
- さらにその aggregate connection として、`BatchRunSummary` が typed bundle failure を吸うなら、持たせる typed 集約は `failure_kind` ごとの histogram / kind count までに留め、bundle failure summary の薄い再掲は避けるのが current understanding である

### 候補 2. richer host interface と coverage analysis の入口整理

- current host harness から production host interface へ直進しない
- preflight、coverage explanation、uncovered call detection のどこが本当に bottleneck かを切り分ける
- detached artifact boundary を先に切った後でも十分な部分だけを後段 task に残す

### 候補 3. parser-free representative coverage の拡張

- current L2 semantics の重要点を、まだ fixture 化されていない narrow case で regression 化する
- static-only と runtime fixture のバランスを見る
- `must_explain` は prose に残し、machine-check 範囲を増やしすぎない

### 候補 4. parser 導入前の boundary inventory

- parser を書く前に最低限固定すべき syntax / companion notation / AST boundary を棚卸しする
- 何を final grammar 決定に回し、何をまだ比較候補に残すかを明確にする

## rough step estimate

以下は rough estimate であり、仕様 drift や review 指摘に応じて前後する。

| 目標 | rough step estimate | 注記 |
|---|---|---|
| PoC を継続的に回せる状態を維持しつつ drift regression を増やす | 2〜4 task | detached artifact / host coverage の切り方次第で前後する |
| PoC を「大量に回して比較しやすい」段階へ一段進める | 2〜5 task | detached trace / audit と richer host boundary の優先順位が影響する |
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

### 4. detached trace / audit が process 内比較に閉じていること

- batch / profile まで積んでも、結果を repo 外 artifact として残しにくい
- case 数が増えると「その場で読んで終わる」運用から抜けにくい
- docs-only minimal schema はできたが、thin export boundary と保存パス規約はまだ未決である

### 5. review infrastructure の変動

- reviewer completion が遅い / 返らない場合があり、task の closing evidence を自前で揃える必要がある

## いまの bottleneck

- `fixture authoring / elaboration` の独立 bottleneck は引き続き残っている
- そのうえで、**detached trace / audit serialization と richer host interface の 2 項目を比べるなら**、前者を先に切る方が前進量は大きい
- richer host interface と coverage analysis の入口整理は、その後段候補である
- notation 自体は current L2 の narrow task を回すには十分安定しており、直近では operational boundary の方が重い

## 近い将来の sequencing

推奨順は次である。
ただしこれは `fixture authoring / elaboration` の独立 bottleneck を取り消すものではなく、**trace/audit と host interface の比較に限った近い sequencing** である。

1. semantics drift regression を増やす
2. detached trace / audit serialization の最小境界を切る
3. detached exporter の first entry を bundle 層から narrow に始める
4. richer host interface / coverage analysis の入口を narrow に切る
5. parser 導入前 inventory を作る
6. その後で parser / richer runtime の判断に進む

## 今の working assumption

- current L2 semantics は大きく動かさない
- parser-free PoC は継続利用する
- hard-coded named profile catalog は維持する
- machine-readable catalog asset / manifest はまだ future option に留める

## 次にやるべき narrow-scope task 候補

- detached trace / audit の docs-only schema から thin exporter 候補の carrier mapping を切り出す
- bundle / batch summary が detached artifact として最低限どこまで出せば比較可能かを棚卸しし、bundle-first exporter entry を docs に固定する
- bundle-first artifact の payload core / bundle_context / detached non-core / aggregate-only を docs-only で切り分ける
- parser-free host harness と richer host interface / coverage analysis の boundary inventory を作る
- parser 導入前の syntax decision inventory を plan と spec に切り出す
