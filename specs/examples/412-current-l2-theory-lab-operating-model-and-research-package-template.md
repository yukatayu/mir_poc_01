# 412 — current L2 theory-lab operating model and research package template

## 目的

mainline actualization と separable な theory-lab line を、
repo の current process discipline に沿って docs-first に整理する。

## roles

1. literature scout
   - primary source を確認する
   - safe-to-cite claim と overstatement risk を分ける
2. formalizer
   - candidate family、relation decomposition、matrix、adequacy corpus を整理する
3. prototyper
   - tiny non-production helper / simulator / compare helper を切る
4. falsifier
   - counterexample、confusion path、kill criteria を集める
5. integrator
   - accepted candidate だけを `specs/` / `plan/` / `docs/reports/` に昇格する

## operating rules

- integrator だけが main docs / plan / spec を更新する。
- exploratory candidate は sandbox / report first に留める。
- prototype は helper-local / non-production / examples-support / script-side に留める。
- accepted comparison / wording / package order だけを repo memory に昇格する。
- OPEN / FUTURE / COMPARISON を fact に昇格させない。

## research package template

| field | content |
|---|---|
| question | 何を比較したいか |
| candidate family | どの family / relation / basis を比べるか |
| adequacy corpus | 何で比較の妥当性を測るか |
| minimal prototype | 必要なら何を non-production で作るか |
| kill criteria | 何が見えたら候補を落とすか |
| promotion criteria | 何が揃ったら `plan/` / `specs/examples/` に昇格するか |

## current judgment

- theory-lab line は repo 全体から見ると advanced line だが、
  line 自体は still early-active である。
- mainline actualization task のついでに theory line を曖昧に混ぜるべきではない。
- accepted candidate だけを ratchet 方式で積み上げる。

## what is not decided here

- final subagent topology
- final prototype language / runtime
- theory-lab package の exact directory layout
