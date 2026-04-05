# 0188 — try-rollback locality smoke convenience

## Objective

detached validation loop の current helper stack に、`e22` mismatch と `e21` frontier を representative contrast pair として 1 command で回せる non-production convenience を追加し、helper stack / progress / traceability を整える。

## Scope and assumptions

- current detached validation loop は non-production helper stack に留める。
- public API / normative spec は広げない。
- `TryFallback` / `AtomicCut` の semantics 自体は変更しない。
- `smoke-fixture` の thin delegation convenience だけを扱う。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
- `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_detached_loop.py`

## Actions taken

1. `scripts/current_l2_detached_loop.py` に `smoke-try-rollback-locality` subcommand を追加し、`e22-try-atomic-cut-place-mismatch` を primary、`e21-try-atomic-cut-frontier` を reference とする representative pair を default path / label 付きで `smoke-fixture` へ委譲する thin wrapper を入れた。
2. `scripts/tests/test_current_l2_detached_loop.py` に focused test を追加し、delegation target path と default labels が drift しないことを固定した。
3. `plan/07-parser-free-poc-stack.md`、`plan/09-helper-stack-and-responsibility-map.md`、`plan/90-source-traceability.md`、`progress.md` を更新し、helper stack / research log / traceability に current cut を反映した。

## Files changed

- `scripts/current_l2_detached_loop.py`
- `scripts/tests/test_current_l2_detached_loop.py`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```bash
python3 -m unittest scripts.tests.test_current_l2_detached_loop
python3 scripts/current_l2_detached_loop.py smoke-try-rollback-locality --overwrite
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_current_l2_detached_loop`
  - loop wrapper tests pass
  - delegation test で `e22` / `e21` の default fixture path と default labels を確認した
- `python3 scripts/current_l2_detached_loop.py smoke-try-rollback-locality --overwrite`
  - `e22` mismatch artifact と `e21` frontier artifact を current default labels で emit した
  - bundle diff では `payload_core.event_kinds` の差だけが primary signal として表示された
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - numbered report count が current report chain まで追従した
- `git diff --check`
  - 無出力

## What changed in understanding

- `TryFallback` / `AtomicCut` の locality contrast は current runtime representative pair として十分 stable なので、pair-specific smoke convenience を thin wrapper として持ってよい。
- ただし current cut では、これは detached validation loop の運用改善に留め、public helper boundary や normative spec には昇格させない方が自然である。

## Open questions

- future first checker spike へ `TryFallback` / `AtomicCut` を接続するとき、dedicated smoke convenience をさらに足す必要があるか。
- representative pair ごとの convenience を増やしすぎると wrapper surface が散るので、どの粒度で止めるか。

## Suggested next prompt

`TryFallback` / `AtomicCut` の structural floor を current checker helper family に接続する helper-local fourth spike が必要かを比較し、必要なら smoke convenience まで含めて narrow に actualize してください。
