# Report 0711 — micro phase self driven window

- Date: 2026-04-16T23:01:39.413706Z
- Author / agent: Codex
- Scope: `progress.md`、`tasks.md`、`plan/10`、`plan/11`、`plan/17`、`plan/18`、`.docs/progress-task-axes.md` を中心に、current self-driven window の micro phase grouping を read-only で再整理する。規範変更や snapshot 更新は行わない。
- Decision levels touched: なし。L0-L3 の規範判断は変更していない。

## 1. Objective

current self-driven window を brief に説明しやすい micro-phase grouping を提案し、

- どこまでが完了まで self-driven か
- 各 micro phase がどの macro / layer の rough percentage window を跨ぐか
- micro phase map 導入時の drift risk

を practical に整理する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `specs/00-document-map.md`（見出しと progress/tasks/plan role 周辺）
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `tasks.md`

## 3. Actions taken

- AGENTS 指示に従い、基礎文書を順に再読した。
- current lane split、macro phase、autonomy gate、current self-driven queue を抽出した。
- `plan/11` の既存 5-label micro phase map を、user-facing により concise な grouping へ圧縮できるか確認した。
- report only を追加し、repo snapshot 文書自体は更新しない判断を残した。

## 4. Files changed

- 追加: `docs/reports/0711-micro-phase-self-driven-window.md`
- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要
- 既存の未コミット変更 `.docs/progress-task-axes.md` は read-only で扱い、今回の task では変更しなかった

## 5. Commands run and exact outputs

```bash
$ git status --short
 M .docs/progress-task-axes.md
```

```bash
$ df -h . && free -h
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   77G   18G  82% /

               total        used        free      shared  buff/cache   available
Mem:           960Mi       782Mi        84Mi       140Ki       248Mi       177Mi
Swap:           19Gi       1.3Gi        18Gi
```

```bash
$ python3 scripts/new_report.py --slug micro-phase-self-driven-window
/home/yukatayu/dev/mir_poc_01/docs/reports/0711-micro-phase-self-driven-window.md
```

- `sed -n` で `README.md`、`Documentation.md`、`progress.md`、`.docs/progress-task-axes.md`、`specs/01`、`specs/02`、`specs/03`、`specs/09`、`plan/00`、`plan/10`、`plan/11`、`plan/17`、`plan/18`、`tasks.md` を読んだ。
- `rg -n` で `specs/00-document-map.md` と `plan/18` の relevant section を抽出した。

## 6. Evidence / findings

### recommendation

current self-driven window は、package 7 本をそのまま見せるより、次の **4 micro phases** に圧縮して示すのが最も practical である。

1. `MP-A Immediate Theory Trigger`
   - 対象: package 1
   - macro: `Macro 5/6`
   - self-driven closeout: 可能
   - rough window:
     `typed / theorem / model-check bridge 82% / 76% / 62%`
     + adjacent `ordering 63% / 54% / 15%`
     + adjacent `syntax / modality 44% / 40% / 5%`
   - stop line: stronger foundation の実昇格や final foundation adoption には進まない

2. `MP-B Immediate Execution Widen`
   - 対象: package 2
   - macro: `Macro 4`
   - self-driven closeout: 可能
   - rough window:
     `fixed-subset source sample line 83% / 78% / 83%`
   - stop line: broader malformed family promotion や core semantics 拡張には進まない

3. `MP-C Theory Later-Gate Framing`
   - 対象: package 3〜5
   - macro: `Macro 5/6 reserve`
   - self-driven closeout: **boundary-prep まで**可能
   - rough window:
     主帯は `typed / theorem / model-check bridge 82% / 76% / 62%`
     補助読みとして `typed-core boundary 約64%`、`theorem pilot boundary 約78%`、`model-check boundary 約72%`
   - stop line:
     stronger typed surface 実昇格、
     theorem public contract 具体化、
     concrete model-check tool / settled property language 確定
     には進まない

4. `MP-D Integration Boundary Framing`
   - 対象: package 6〜7
   - macro: `Macro 6 reserve` + `Macro 7 reserve`
   - self-driven closeout: **boundary-prep まで**可能
   - rough window:
     `shared-space docs-first boundary 57% / 48% / 27%`
     + `host-facing integration boundary 51% / 43% / 34%`
     + packaging touchpoint として `backend / public packaging 29% / 25% / 24%`
   - stop line:
     final fairness / replay operational profile、
     installed-binary promotion、
     packaging success criteria の確定には進まない

### why this grouping

- `plan/11` の 5-label map は package ordering として妥当だが、user-facing の近接説明では `MP-6A` と `MP-7A` を統合した方が lane の意味を落とさず簡潔になる。
- package 3〜5 は Track A/B/C の別 package だが、すべて「later-gate framing」という同じ closeout kind に属しているため、micro phase としては 1 つに束ねてよい。
- package 1 と package 2 は immediate だが、theory-lab と execution の lane split を保つため分けておく方が自然である。

### drift risks

1. lane collapse
   - theory-lab / execution / reserve integration を 1 本の linear phase に見せると、parallel lane の読みが消える。
2. gate inflation
   - `Macro 6/7` の boundary-prep closeout を、operational realization や final packaging まで self-driven と誤読しやすい。
3. percentage over-reading
   - rough percentage window を cumulative completion や normative KPI のように扱うと drift する。
4. snapshot mismatch
   - `progress.md`、`tasks.md`、`plan/11` で micro phase 名や package 範囲がずれると、current queue の読みがすぐ壊れる。

### guardrails if adopted

- micro phase 名には lane と closeout kind を入れる
- 各 micro phase に `self-driven closeout` と `stop line` を必ず添える
- percentage は「rough window」「current-L2 scoped reading」と明記する
- `plan/11` は package order、`progress.md` / `tasks.md` は compressed micro phase view と役割分担を固定する

## 7. Changes in understanding

- current self-driven queue は「promoted immediate 2 本だけ」ではなく、boundary-prep reserve を含めた 7 package / 5 micro-phase 相当の幅を持つ、という読みが既に docs 間で揃っている。
- ただし brief recommendation 用には、5 micro phases のままより 4 micro phases へ圧縮した方が practical で、autonomy gate も保持できる。
- `Macro 6` と `Macro 7` は mixed gate だが、boundary-prep closeout までは明確に self-driven として扱ってよい。

## 8. Open questions

- この 4-phase grouping を実際に `plan/11` / `tasks.md` / `progress.md` に反映して current official map にするか。
- 反映する場合、`plan/11` 側は現行の 5-label package-oriented map を残し、snapshot 側だけ 4-label view にするか。
- `MP-D` を 1 本に束ねるか、現行どおり `Macro 6` / `Macro 7` を分けて 2 本に保つか。

## 9. Suggested next prompt

`この 4 micro-phase grouping を official snapshot に採用する前提で、progress.md / tasks.md / plan/11 を最小差分で更新して。plan/11 は package order を残しつつ、snapshot 側は 4-label view に寄せて。`
