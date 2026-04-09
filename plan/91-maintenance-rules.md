# plan/91 — maintenance rules

## 目的

この文書は、今後の PoC 検証ループで `plan/` をどう維持するかを固定する。
`plan/` は scratchpad ではなく、人間向けの repository memory である。

## `plan/` を更新するタイミング

次のいずれかが変わった task では、同じ task の中で relevant な `plan/` ファイルを更新する。

- semantics
- representative examples
- fixtures
- helper stack
- parser-free PoC behavior
- roadmap
- open questions
- syntax candidate
- workstream sequencing
- current status

## update 不要の扱い

変更が `plan/` に影響しない場合は、その task の report に **`plan/ 更新不要`** と明記する。
無言で skip してはいけない。

## `plan/` の書き方

- 人間向けの日本語正本として書く
- 比喩より technical statement を優先する
- 次を分けて書く
  - 決定
  - 未決
  - 仮説 / 一時的 working assumption
  - 履歴 / comparison / rejected option

## `plan/` と他文書の関係

- `specs/`:
  - 規範文書
- `docs/reports/`:
  - 時系列の作業証跡
- `plan/`:
  - 現況・計画・責務境界・open questions・workstream sequencing を再編成した長期参照文書

`plan/` 更新は docs mirror と同じく、repo の一級成果物として扱う。

## current L2 / PoC task で最低限見るもの

current L2 / parser-free PoC / helper stack / roadmap task では、少なくとも次を確認する。

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. `plan/00-index.md`
9. 関連する `plan/` 本文
10. 必要な `specs/examples/`、report chain、code anchor

ただし、**規範判断の正本は常に `specs/`** である。
`plan/` はその現況整理と導線であり、規範文書を置き換えない。

## mirror 扱いの文書

current task で変更が入ったら、次の mirror 群の更新要否を確認する。

- `plan/`
- `progress.md`
- `Documentation.md`
- `specs/00-document-map.md`
- relevant `specs/examples/`
- relevant report

## `progress.md` の扱い

- `progress.md` は repo 全体の簡潔な進捗スナップショットである。
- 規範判断の正本にはしない。規範判断は `specs/`、長期参照整理は `plan/` に残す。
- current status / roadmap / remaining steps / major bottleneck / validation loop の到達見込みが変わった task では、同じ task の中で `progress.md` の更新要否を確認する。
- `progress.md` の進捗率は、可能な限り
  - 論理仕様
  - ユーザ向け仕様
  - 実装 / 運用
  の 3 軸で並べる。
- 3 軸の意味は次の通りである。
  - 論理仕様: semantics / invariants / formal boundary の整備度
  - ユーザ向け仕様: companion notation / representative examples / human-facing guidance の整備度
  - 実装 / 運用: parser-free PoC / helper / validation loop / 実務フローの整備度
- さらに各章 / 層には、
  - `着手可能`
  - `要仕様確認`
  - `後段依存`
  のいずれかで、いま agent が進めてよいかどうかを示す欄を置く。
- この欄の意味は次の通りである。
  - `着手可能`: 非本質部分を先に進めても手戻りが比較的小さい
  - `要仕様確認`: user 側の目的 / 保証範囲 / 非機能要件が不足しており、勝手に詰めると手戻りが大きい
  - `後段依存`: 先行 layer / 先行 decision が固まるまで本格着手しない方がよい
- 更新しなかった場合も、report に **`progress.md 更新不要`** と明記する。
- `progress.md` の末尾には、task close ごとに日時つきの簡潔な作業ログを追記する。
  - 粒度は「何を検証したか」「何が通って次に進めるようになったか」が読める 1 行でよい。
  - docs-only task でも current status に影響する non-trivial change なら原則追記する。
  - timestamp は手打ちせず、`date` コマンド等でその場で取得した値を使う。
- `progress.md` には、repo 全体の大局 phase を示す section を置き、
  - phase 名
  - 主眼
  - 現在位置
  - 重さ
  - 自走可否
  を簡潔に mirror する。phase 読みが変わった task では同じ task の中で更新する。

## `tasks.md` の扱い

- `tasks.md` は repo 全体の **current task map** である。
- `progress.md` より具体的に、
  - ある程度まとまった単位で自走して進められる task
  - 方針決定が必要で current research の障害になっている blocker
  を、日本語で読みやすく整理する。
- `tasks.md` は append-only 履歴ではなく、**更新時には毎回全体を書き直して snapshot に保つ**。
- phase end、checkpoint close、mainline 切り替え、major blocker の入れ替わりが起きた task では、同じ task の中で `tasks.md` 更新要否を確認する。
- blocker 項目では、少なくとも
  - 概要
  - 影響範囲
  - 主要な選択肢
  - current recommendation
  を書く。
- `tasks.md` は規範判断の正本ではない。規範判断は `specs/`、長期比較は `plan/` に残す。
- 更新しなかった場合でも、report に **`tasks.md 更新不要`** と明記する。

## review 運用

PoC ループの task は、できるだけ task 内部で閉じる。

- 中途で user に何度も返さない
- self-check、focused diff review、local validation を先に行う
- reviewer はむやみに何度も呼ばず、**最後に 1 回長めに待つ**のを基本にする
- 必要なら task 内部で narrow-scope re-review を行ってよい
- reviewer が返らない場合だけ retry を 1 回行う
- なお返らなければ local evidence と diff inspection を report に残す

## long-running research task の運用メモ

- heavy command や generated artifact を増やす前に、`df -h .` と `free -h` 相当で disk / memory を確認する。
- commit は対話的な GPG prompt を避けるため `git commit --no-gpg-sign` を使う。
- user が明示的に止めない限り、commit ごとに push する。
- long-running research では、PoC 実装・実行・回帰確認と、formal boundary / proof obligation / invariant wording の整理を並走させる。
- portability / observability / step execution / graph export hook は replaceable layer として意識し、CPU 固定や単一 debug mode を早く既成事実化しない。
- subagent を使う場合は、明らかな hang / broken evidence がない限り、latency だけを理由に早切りせず completion まで待つ。
- 不要になった subagent は close し、保持したい context があるものだけ意図的に残す。
- 長期研究フェーズでは、PoC 実装・実行・回帰確認と、formal boundary / proof obligation / invariant wording の整理を並走させ、ratchet 方式で手戻りを減らす。

## report に必ず残すこと

- `plan/` を更新したファイル
- `plan/` を更新した理由
- もし更新しなかったなら `plan/ 更新不要`
- task 開始時の dirty state
- local validation の evidence
- reviewer の completion 有無

## `plan/` の見直し頻度

- 少なくとも non-trivial task ごとに relevant file を確認する
- helper stack、notation、fixture coverage、roadmap が変わった task では、見直しを省略しない
- 大きな phase change の後は `plan/01`、`plan/10`、`plan/11`、`plan/12` をまとめて見直す
