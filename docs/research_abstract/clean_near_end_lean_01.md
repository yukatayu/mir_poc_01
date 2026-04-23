# clean near-end Lean 01

## 要約

current Lean line は 2 層構成です。

- `samples/lean/foundations/`
  finite-index first layer、IFC concrete example、proof skeleton を小さな actual proof として置く
- `samples/lean/clean-near-end/`
  active clean near-end sample suite から生成した theorem stub を置く

## 何が built-in か

Lean 側で built-in なのは `inductive`、`structure`、`def`、`theorem`、`namespace`、`Bool`、`Nat`、`Prop`、`String`、`List` などの Lean 自体の構文と基本型です。

## 何が user-defined か

foundation では、次のような概念を **Lean ファイル内で定義** しています。

- `SecurityLabel`
- `flowsTo`
- `CanDeclassify`
- `Lifetime`
- `Capability`
- `CaptureSet`
- `ReviewUnit`
- `LeanStub`

つまり、Mir 側の domain predicate も Lean built-in ではなく、foundation file の局所定義です。

## 現在の評価

- foundations は small mechanization fragment として有効
- generated stub corpus は repo-local proof bridge の足場として有効
- ただし generated stub は domain proof 完了の宣言ではない

## 実行コマンド

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

## detail

foundation file 全文、代表的な generated stub、manifest に含まれる actual verification result は `clean_near_end_lean_01_detail.md` を参照してください。
