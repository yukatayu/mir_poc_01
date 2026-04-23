# samples/lean

このディレクトリは、repo が Lean でどこまで mechanization を進めているかを **repo-local かつ inspectable** に保存するための場所です。

## layout

- `foundations/`
  - small actual proof fragment
  - finite-index first layer、IFC example、proof skeleton の最小 mechanization を置く
- `clean-near-end/`
  - active clean sample suite から生成した theorem stub
  - Lean は通るが、full domain discharge を意味しない
- `manifest.json`
  - foundations と generated stub corpus の verification result
- `old/2026-04-22-pre-clean-near-end/`
  - pre-clean-near-end corpus の archive

## current reading

- foundations は actual proof fragment
- generated stub は proof bridge の足場
- old corpus は historical appendix

## 実行コマンド

```bash
python3 scripts/current_l2_lean_sample_sync.py
```

## 境界

- Lean built-in として repo が使うのは Lean 自体の構文と基本型
- security label、authority-sensitive predicate、capture / lifetime / cost model、review-unit / stub 構造は foundation file の user-defined definition
- final public theorem contract や full discharge をここで確定したわけではない
