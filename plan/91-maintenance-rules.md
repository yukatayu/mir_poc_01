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
- 更新しなかった場合も、report に **`progress.md 更新不要`** と明記する。

## review 運用

PoC ループの task は、できるだけ task 内部で閉じる。

- 中途で user に何度も返さない
- self-check、focused diff review、local validation を先に行う
- reviewer はむやみに何度も呼ばず、**最後に 1 回長めに待つ**のを基本にする
- 必要なら task 内部で narrow-scope re-review を行ってよい
- reviewer が返らない場合だけ retry を 1 回行う
- なお返らなければ local evidence と diff inspection を report に残す

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
