# Phase 2: parser-free PoC と detached validation loop

## この phase の意味

Phase 2 は、final parser を作る前に **sample / helper / validation loop を回せる状態** を作る層です。
repo が今どこまで動くかを、parser-free な helper と fixture-friendly な command で確認できるようにします。

## 2026-04-23 時点で固まっていること

この節は dated repository-memory anchor です。live status / macro phase / next queue authority は `../../progress.md` と `../../tasks.md` を参照してください。

- `scripts/current_l2_guided_samples.py` による suite-level smoke / closeout
- `scripts/clean_near_end_samples.py` による family-level run / matrix / closeout
- `scripts/current_l2_lean_sample_sync.py` による Lean foundations / generated stub の同期
- `python3 scripts/validate_docs.py` による docs scaffold check

## この phase が current layer にどう効いているか

repo-local alpha floor は、この detached validation loop があることで再確認可能です。
つまり、final parser がまだなくても、

- active sample が何本あるか
- 各 sample が valid / malformed / counterexample のどれか
- Lean foundation が同期しているか

を repo 内で確かめられます。

## まだ残ること

- final parser / AST carrier を detached loop とどう reconnect するか
- public checker / runtime API に helper command をどう昇格させるか
- external verifier / model-check tool との concrete binding

## 関連する summary / detail

- `clean_near_end_typing_01_detail.md`
- `clean_near_end_order_model_01_detail.md`
- `clean_near_end_modal_01_detail.md`
- `clean_near_end_lean_01_detail.md`
