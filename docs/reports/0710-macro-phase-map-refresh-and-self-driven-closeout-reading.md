# Report 0710 — macro phase map refresh and self-driven closeout reading

- Date: 2026-04-17T08:00:44+0900
- Author / agent: Codex
- Scope: `progress.md` の macro phase map を見直し、どの macro phase なら self-driven closeout と読めるか、どの macro phase は boundary-prep までに留まるかを明示する。`tasks.md` と relevant `plan/` / `.docs/` もこの読みへ追従させる。
- Decision levels touched: L2 / L3

## 1. Objective

- macro phase map のうち、どこまでを self-driven closeout と読んでよいかを明示する。
- `tasks.md` に「self-driven で closeout まで持って行ける macro phase 読み」を追加する。
- `Macro 6` / `Macro 7` の boundary-prep と full closeout を混同しない形に整理する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `.docs/progress-task-axes.md`
- `plan/01-status-at-a-glance.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `docs/reports/0709-self-driven-queue-reassessment-and-snapshot-refresh.md`

## 3. Actions taken

1. `progress.md` / `tasks.md` / `plan/01` / `plan/17` / `plan/18` を再読し、macro phase ごとに self-driven closeout の読みを整理した。
2. `.docs/progress-task-axes.md` の writing rules を更新し、macro phase map には
   - self-driven closeout まで行けるか
   - boundary-prep までか
   - user specification が必要か
   を mirror する、と明記した。
3. `progress.md` の macro phase map を更新し、
   - `Macro 0〜5` は current closeout まで self-driven
   - `Macro 6〜7` は boundary-prep まで self-driven
   - `Macro 8` は user specification 必須
   と読める形に揃えた。
4. `tasks.md` に「自走して closeout まで持って行ける macro phase 読み」を追加した。
5. `plan/01` に、`Macro 6` / `Macro 7` の autonomy gate が boundary-prep まで self-driven だと読める wording を揃えた。
6. `plan/90` に今回の traceability addendum を追加した。

## 4. Files changed

- `.docs/progress-task-axes.md`
- `plan/01-status-at-a-glance.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0710-macro-phase-map-refresh-and-self-driven-closeout-reading.md`

更新不要:

- `Documentation.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `specs/`

## 5. Commands run and exact outputs

```bash
df -h .
# Filesystem      Size  Used Avail Use% Mounted on
# /dev/vda2        99G   77G   18G  82% /

free -h
#                total        used        free      shared  buff/cache   available
# Mem:           960Mi       779Mi        78Mi       140Ki       257Mi       180Mi
# Swap:           19Gi       1.3Gi        18Gi

date '+%Y-%m-%d %H:%M:%S %Z'
# 2026-04-17 08:00:44 JST

python3 scripts/validate_docs.py
# Documentation scaffold looks complete.
# Found 710 numbered report(s).

git diff --check
# <no output>
```

## 6. Evidence / findings

- macro phase map の current closeout reading は次で安定した。
  - `Macro 0〜3`: closeout まで self-driven
  - `Macro 4`: current fixed-subset widening closeout まで self-driven
  - `Macro 5`: current boundary / pilot / framing closeout まで self-driven
  - `Macro 6〜7`: boundary-prep まで self-driven、full closeout は mixed gate
  - `Macro 8`: user specification 必須
- `tasks.md` では、package queue と別に「macro phase の self-driven closeout 読み」を置くことで、current lane の終点と repo-wide finalization を混同しにくくなった。
- `Macro 6` / `Macro 7` の扱いは、`mixed` と書くだけでは強すぎ、`boundary-prep までは self-driven` まで mirror した方が current repo memory と整合する。

## 7. Changes in understanding

- current repo では、まず macro phase map 側で self-driven closeout の読みを安定させる方が重要だった。
- user が欲しかったのは micro bundle ではなく、「macro phase ごとにどこまで自走で閉じられるか」の可視化であり、`progress.md` / `tasks.md` はその粒度に戻した方が自然だった。
- `Macro 6` / `Macro 7` は full closeout 不能でも、boundary-prep までは自走可能と明記すべきだった。

## 8. Open questions

- `Macro 4` / `Macro 5` の closeout 読みを、将来さらに分割した phase wording が必要になるかは、その時点の queue 長で再評価してよい。
- `Macro 6` / `Macro 7` の boundary-prep 範囲をどこまで tasks 側で露出するかは、reserve queue の増減に応じて再調整してよい。

## 9. Suggested next prompt

`tasks.md` の self-driven package queue から、まとめて進めたい package 数を指定してください。指定がなければ current order どおりに自走できます。
