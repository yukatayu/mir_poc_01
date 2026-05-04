# practical alpha-1 product preview

この文書は、`P-A1-08` で actualize した **first practical product-preview floor** の最短入口です。

## 何を確認するか

- preview root は `samples/practical-alpha1/previews/`
- script surface は `python3 scripts/practical_alpha1_product_preview.py`
- current actualized rows は `PE2E-01..07`
- carrier split は
  `preview manifest -> exact practical reports / exact practical devtools bundles -> non-final product-preview bundle`

## まず実行するコマンド

```bash
python3 scripts/practical_alpha1_product_preview.py list --format json
python3 scripts/practical_alpha1_product_preview.py check-all --format json
python3 scripts/practical_alpha1_product_preview.py render-html PE2E-07 --format json
python3 scripts/practical_alpha1_product_preview.py closeout --format json
```

## current row inventory

- `PE2E-01`
  local full-toolchain preview
- `PE2E-02`
  Docker full-toolchain preview
- `PE2E-03`
  debug-layer companion preview
- `PE2E-04`
  placeholder object companion preview
- `PE2E-05`
  local save/load continue preview
- `PE2E-06`
  invalid distributed save rejected preview
- `PE2E-07`
  devtools viewer preview

## 重要な non-claim

- これは full practical product prototype completion ではない
- `PE2E-04` は `HP-A1-06` placeholder object preview companion evidence に narrow される
- custom Mir avatar runtime ではない
- unsupported runtime fallback ではない
- same-session runtime attach / detach lifecycle execution ではない
- active canonical runnable-root promotion ではない
- final public CLI / viewer / transport / save-load / package-avatar API ではない
