# sub-agent-pro

## 目的

このディレクトリは、外部の `GPT-5.4-pro` へ独立質問を投げるための質問束を置く場所である。

前提は次のとおり。

- まず `plan/` を読む
- 次に `specs/` を読む
- その後で `question_XXX.md` を 1 つだけ渡す

各質問は **独立** であり、他の `question_*.md` を参照しなくても答えられるように書いてある。

## 想定ワークフロー

1. `plan/00-index.md` から必要な `plan/` を読む。
2. `README.md`、`Documentation.md`、`specs/00...03`、`specs/09`、必要な subsystem spec を読む。
3. `question_XXX.md` を 1 つだけ渡す。
4. 返答は対応する `answer_XXX.md` に保存する。
5. この repo 側では、その `answer_XXX.md` を読んで研究を再開する。

## 質問一覧

- `question_001.md` 大局 roadmap と優先順位の再点検
- `question_002.md` 最終型理論の位置づけと段階的導入
- `question_003.md` full 型システム前の最小 sample-visible 型/契約 surface
- `question_004.md` `atomic_cut` と memory-order 再構築 family
- `question_005.md` 定理証明ラインの first serious formalization
- `question_006.md` モデル検査ラインの first concrete carrier と first tool cut
- `question_007.md` language core / static checker / external verifier の境界
- `question_008.md` final parser grammar freeze の進め方
- `question_009.md` `place` と process/server/executable の対応づけ
- `question_010.md` shared-space identity / admission / authority / fairness の整理
- `question_011.md` Mirrorea の route proof / overlay / patch activation / suspended task
- `question_012.md` host-facing I/O / FFI / visualizer / game engine adapter の層分け
- `question_013.md` Rust / Python split の長期戦略
- `question_014.md` public parser/checker/runtime API と CLI packaging
- `question_015.md` representative sample corpus の増やし方
- `question_016.md` いま見落としていそうな大きい理論・設計リスク

## 注意

- ここに書いてある質問文は、repo 側の current snapshot に基づく。
- ただし規範判断の正本は常に `specs/` である。
- 質問に対する回答では、できるだけ
  - 何が source-backed に言えるか
  - 何が未決か
  - そのうえでの推奨案
  - 近接 roadmap への影響
  を分けてほしい。
