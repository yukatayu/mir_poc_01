# 0293 — phase012 closeout consistency sweep smoke

## Objective

user 指示にある **Phase 0 / 1 / 2 closeout → consistency sweep → Phase 3** のうち、Phase 0 / 1 / 2 closeout の最初の実地 evidence として、

- detached validation loop の current helper surface
- current docs / spec / plan / progress の loop 説明

が大きくずれていないかを smoke level で確認する。

## Scope and assumptions

- current L2 semantics は変更しない。
- 今回は bundle-first detached loop と static-gate loop の smoke evidence を取り、current docs line と矛盾しないかを見る。
- helper API finalization や production CLI 化は行わない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/examples/23-current-l2-detached-export-loop-consolidation.md`
- `specs/examples/24-current-l2-detached-export-storage-and-aggregate-api.md`
- `specs/examples/28-current-l2-detached-fixture-validation-loop-helper.md`
- `plan/11-roadmap-near-term.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_diff_detached_artifacts.py`
- `scripts/current_l2_diff_detached_aggregates.py`

## Actions taken

1. disk / memory の current resource snapshot を確認した。
2. detached loop helper と diff helper の `--help` を読み、docs に書かれた entry surface と照合した。
3. `smoke-fixture` で runtime representative fixture を 2 本回し、bundle-first / aggregate smoke の current behavior を確認した。
4. `smoke-static-gate` で static-only representative fixture を 2 本回し、static gate diff の current behavior を確認した。
5. reviewer finding に合わせて、`specs/00-document-map.md` の `scripts/current_l2_detached_loop.py` surface に `smoke-static-gate` を追記した。
6. smoke evidence と drift 補正を `progress.md` の作業ログへ反映した。

## Files changed

- `docs/reports/0293-phase012-closeout-consistency-sweep-smoke.md`
- `specs/00-document-map.md`
- `progress.md`

## Commands run

```text
$ df -h . && free -h
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   87G  7.1G  93% /
               total        used        free      shared  buff/cache   available
Mem:           960Mi       659Mi        80Mi       1.2Mi       376Mi       300Mi
Swap:           19Gi       1.2Gi        18Gi

$ date '+%Y-%m-%d %H:%M %Z'
2026-04-08 12:59 JST

$ python3 scripts/current_l2_detached_loop.py --help
[current helper help output confirmed]

$ python3 scripts/current_l2_diff_detached_artifacts.py --help
[current helper help output confirmed]

$ python3 scripts/current_l2_diff_detached_aggregates.py --help
[current helper help output confirmed]

$ python3 scripts/current_l2_detached_loop.py smoke-fixture crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json --reference-fixture crates/mir-ast/tests/fixtures/current-l2/e6-write-after-expiry.json --run-label phase012-e3 --reference-label phase012-e6 --overwrite
[bundle-first smoke completed with payload_core differences and aggregate smoke evidence]

$ python3 scripts/current_l2_detached_loop.py smoke-static-gate crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json --reference-fixture crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json --run-label phase012-e4 --reference-label phase012-e5 --overwrite
[static-gate smoke completed with checker_core differences]
```

## Evidence / outputs / test results

Task start dirty state:

```text
## main...origin/main
```

Key evidence:

- `smoke-fixture` は
  - bundle artifact emit
  - reference fixture compare
  - full directory aggregate smoke
  - single-fixture aggregate smoke
  を 1 command で実行できた
- `smoke-static-gate` は
  - static gate artifact emit
  - reference static gate compare
  を 1 command で実行できた
- compare boundary の output は docs line と整合していた
  - bundle diff は `payload_core` に絞られ、`bundle_context` / `detached_noncore` は reference-only
  - aggregate diff は `summary_core` を比較し、`aggregate_context` は reference-only
  - static gate diff は `checker_core` を比較し、runtime trace は比較に入らない
- reviewer finding により、document map 上で `smoke-static-gate` subcommand の omission が 1 件見つかり、この task 内で補正した

Validation:

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 294 numbered report(s).

$ git diff --check
[no output]
```

## What changed in understanding

- current detached validation loop の helper surface は、Phase 0 / 1 / 2 closeout を進めるうえで十分 stable である。
- compare boundary 自体には大きな drift は無かったが、document map の helper surface 記述に `smoke-static-gate` omission が 1 件あったため、この task 内で補正した。
- したがって次は、Phase 0 / 1 / 2 closeout の consistency sweep を docs/spec mirror 全体へ広げるか、Phase 3 の staged parser line へ進む前の remaining friction だけをさらに narrow に潰すか、という sequencing になる。

## Open questions

- `smoke-fixture` の aggregate compare を current docs line のまま informational difference として使い続けるか、それとも closeout 用 convenience mode を別に足すか。

## plan/progress update status

- `plan/` 更新不要
- `progress.md` 更新あり

## Suggested next prompt

`Phase 0 / 1 / 2 closeout の consistency sweep を、README / Documentation / specs/examples / plan mirror の detached validation loop 説明に広げて、remaining drift があるか確認してください。`
