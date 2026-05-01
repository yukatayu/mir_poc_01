# post-`P18` true user-spec hold 01

## この文書の役割

この文書は、`U1` post-`P18` true user-spec hold option matrix を
短い command sequence で追う landing page です。

- final public freeze の確認手順ではありません
- actual product target を commit する手順でもありません
- option inventory と current recommendation を確認する手順です

## まず実行するコマンド

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
sed -n '1,220p' plan/28-post-p18-true-user-spec-hold-option-matrix.md
sed -n '/^## user decision blockers/,/^## research-discovery items/p' tasks.md
sed -n '/^## research-discovery items/,/^## validation floor for this snapshot/p' tasks.md
sed -n '/^## closed package memory and active gate/,/^## validation anchors/p' progress.md
git diff --check
```

## これで確認できること

- `P18` repo-side first cut の後に残る user-spec hold line が option inventory として分離されていること
- packaging shape / host target / first shipped public surface / final shared-space operational catalog breadth が current docs で同じ軸に揃っていること
- provisional recommendation が overclaim を避けたまま reader-facing に置かれていること

## これではまだ確認できないこと

- actual installed binary adoption
- actual host integration target adoption
- final public parser / checker / runtime / verifier API
- final public adapter / viewer / projection / hot-plug / transport ABI
- final shared-space operational catalog adoption

## 関連文書

- `current_phase_closeout_01.md`
- `../research_abstract/post_p18_true_user_spec_hold_option_matrix_01.md`
- `../../plan/28-post-p18-true-user-spec-hold-option-matrix.md`
