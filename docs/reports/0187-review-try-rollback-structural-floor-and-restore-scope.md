# 0187 — review fallback for try/rollback structural floor と restore-scope boundary

## Objective

`0186` の差分に対して reviewer を 1 回だけ使う運用を試し、completion が得られない場合は local diff review fallback を明示する。

## Scope and assumptions

- reviewer completion が得られない current environment を前提にする。
- semantic / helper boundary / factual count correction の focused review だけを扱う。

## Documents consulted

- `docs/reports/0186-try-rollback-structural-floor-and-restore-scope.md`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `crates/mir-ast/tests/fixtures/current-l2/e22-try-atomic-cut-place-mismatch.json`
- `crates/mir-ast/tests/fixtures/current-l2/e22-try-atomic-cut-place-mismatch.host-plan.json`
- `specs/examples/39-current-l2-static-reason-code-readiness-scan.md`
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
- `progress.md`

## Actions taken

1. reviewer 起動を試みたが、current environment では completion を待つための usable handle が返らなかった。
2. retry 相当の再試行を 1 回だけ検討したが、同じ制約で completion 取得手段が無いことを確認した。
3. AGENTS fallback に従い、local diff review を実施した。
4. review 観点は次に絞った。
   - `e22` fixture と test が current runtime semantics に沿っているか
   - `specs/examples/51` と related docs の current judgment が code anchor と矛盾しないか
   - readiness scan 実測反映の count correction が factual に正しいか

## Evidence / outputs / test results

- `git diff --check`
  - 無出力
- local diff review で確認したこと
  - `e22` は nested `PlaceBlock(profile_annotation)` 内 `AtomicCut` を使い、`place_anchor = draft_profile` と `current_place = profile_annotation` の mismatch を明示している
  - host plan sidecar は `stage_profile_patch` success、`validate_profile_patch` unsatisfied、`load_last_snapshot` success だけを与えており、contrast case を過不足なく固定している
  - `current_l2_minimal_interpreter.rs` の new test は terminal outcome、event sequence、final `place_store` の 3 点を直接押さえている
  - `specs/examples/51` の current code anchor は `RollbackFrame { place_anchor, restore_snapshot }` と whole-store restore を正しく書いており、checker cut に dynamic gate / restore scope を入れない current judgment と矛盾しない
  - readiness scan の current reading は `11 / 11 / 9 / 9 / 9` と capability floor `2` に揃っている

## What changed in understanding

- reviewer completion を infrastructure 上 guaranteed に待てない場合でも、local diff review fallback を report に残して semantic / factual review trail を切らさない必要がある。
- 今回の差分については、local diff review の範囲では重大な inconsistency は見つからなかった。

## Open questions

- current environment で reviewer completion handle を stable に取得する方法があるか。
- 将来 reviewer tool の待機方法が変わる場合、AGENTS の fallback wording を追加で mirror する必要があるか。

## Suggested next prompt

`TryFallback` / `AtomicCut` の structural floor を current checker helper family に接続する helper-local fourth checker spike が必要かどうかを比較し、必要なら narrow helper と smoke evidence まで進めてください。
