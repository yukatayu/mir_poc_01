# 報告 0023 — fallback / preference chain lineage annotation の最小 surface form 整理

- 作成日時: 2026-03-31T02:33:22+09:00
- 作成者 / agent: Codex
- 対象範囲: canonical normalization law・incompatible branch rejection phase・static evidence floor・underdeclared case handling を前提にした `documented lineage annotation` の最小 surface form
- 触れた decision level: 既存の Mir-0 境界と L2 の canonical law / rejection phase / static evidence floor / underdeclared handling を維持したまま、`documented lineage annotation` の書式を 1 段具体化した

## 1. 目的

same logical access path / semantic lineage の static 証拠として、current L2 で `documented lineage annotation` に最低限何を書けばよいかを明文化する。今回は canonical law・rejection phase・static evidence floor・underdeclared case handling 自体は変えず、それらを支える最小の surface form だけを定める。

## 2. 範囲と前提

- `AGENTS.md` の指示に従い、`README.md`、`Documentation.md`、`specs/00..04`、`specs/09`、`specs/10`、`specs/11`、`specs/12`、`docs/reports/0018`、`docs/reports/0019`、`docs/reports/0020`、`docs/reports/0021`、`docs/reports/0022` を指定順で読んだ。
- 作業開始時点の git dirty state は `## main...origin/main [ahead 6]` だけで、tracked / untracked の作業中変更はなかった。
- 既存の canonical normalization law、incompatible branch rejection phase、static evidence floor、underdeclared case handling は変更しない。
- `code_mapper` を最初に使い、中心ファイルが `specs/04`、`specs/10`、`specs/12` であること、L2 を越える抽象を足さないこと、global ID や capability theory に広げないことを確認した。
- `reviewer` を最後に使い、edge-local な最小書式が canonical law や rejection phase を強めすぎていないかを点検した。

## 3. 参照文書

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
- `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
- `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
- `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
- `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`

## 4. 実施内容

- `code_mapper` を最初に起動し、task-start dirty state が空であること、今回の中心ファイルが `specs/04`、`specs/10`、`specs/12` であること、global lineage ID や大きな capability 理論へ広げないことを確認した。
- `specs/04-mir-core.md` の `static evidence floor（L2）` 節で、`documented lineage annotation` の最小 surface form を「ちょうど 1 本の fallback edge を飾る edge-local annotation」と定義した。
- 同じ節で、その annotation が最低限持つ情報を次の 3 つに固定した。
  - predecessor option 参照
  - successor option 参照
  - same logical access path / semantic lineage continuation の affirmative claim
- `declared access target` は各 option に残る別情報であり、`documented lineage annotation` の内部へ埋め込まず、same-lineage の static 証拠としては「両端 option の `declared access target` + edge-local な `documented lineage annotation` の組」を要求する形に整理した。
- option-local tag、chain-level blanket annotation、global lineage identifier は current L2 の最小要件から外した。
- valid / malformed / underdeclared を区別できるよう、説明用の小さな例を `specs/04-mir-core.md` に追加した。ただし `lineage(A -> B)` は説明用記法であり、最終 reserved token ではないことを同じ節で明記した。
- `specs/10-open-questions.md` には上の current L2 判断を反映しつつ、最終 keyword / punctuation / serialization、option-local / chain-level sugar、same-place / cross-place で形を共有するかどうかを **未決定** として残した。
- `specs/12-decision-register.md` には D-029 を追加し、最小 surface form の current L2 判断を decision register に記録した。
- 作業中に `docs/reports/0024-fallback-preference-chain-lineage-annotation-surface-form.md` という空テンプレートが一時的に見えたが、最終成果物には含めず、作業完了時点では report 一覧に残っていないことを確認した。
- commit は 2 本に分ける。
  - 1 本目は仕様本文だけを記録する `lineage annotation の最小書式を整理する`
  - 2 本目は本 report を記録する `lineage annotation 最小書式の作業報告を追加する`
  - ただし 2 本目の commit はこの report 自身を含むため、self-reference の都合で本文中に最終 hash を固定値で埋め込めない。実 hash は final response と `git log` で補完する。

今回実際に変更したファイル:

- `specs/04-mir-core.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md`

確認したが今回変更しなかったファイル:

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
- `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
- `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
- `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
- `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`

## 5. コマンドと正確な出力

1. 作業開始時点の `git status --short --branch`

```text
## main...origin/main [ahead 6]
```

2. `python3 scripts/new_report.py --slug fallback-preference-chain-lineage-annotation-surface-form`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md
```

3. `date --iso-8601=seconds`

```text
2026-03-31T02:33:22+09:00
```

4. `git status --short --branch`（report scaffold 生成直後）

```text
## main...origin/main [ahead 6]
 M specs/04-mir-core.md
 M specs/10-open-questions.md
 M specs/12-decision-register.md
?? docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md
?? docs/reports/0024-fallback-preference-chain-lineage-annotation-surface-form.md
```

5. reviewer 結果

```text
no findings
```

6. 仕様本文 commit

```text
c708252 lineage annotation の最小書式を整理する
 specs/04-mir-core.md          | 34 +++++++++++++++++++++++++++++++++-
 specs/10-open-questions.md    |  6 +++++-
 specs/12-decision-register.md |  1 +
 3 files changed, 39 insertions(+), 2 deletions(-)
```

7. `git diff --check`

```text
<no output>
```

8. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 24 numbered report(s).
```

9. `git status --short --branch`（report commit 前）

```text
## main...origin/main [ahead 7]
?? docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md
```

## 6. 証拠 / 所見

今回の current L2 判断:

- `documented lineage annotation` は option-local でも global ID でもなく、ちょうど 1 本の fallback edge に付く edge-local annotation として扱う。
- same-lineage の最小 static 証拠として必要なのは、両端 option にある `declared access target` と、そのちょうど 1 本の edge を指す `documented lineage annotation` の組である。
- annotation 単独では access target の anchor を欠くため足りず、`declared access target` 単独でも same-lineage continuation の明示を欠くため足りない。
- current L2 は predecessor / successor 参照と affirmative claim だけを最小要件とし、option-local tag、chain-level blanket annotation、global lineage identifier を要求しない。

valid / malformed / underdeclared の区別:

- `valid`: edge-local annotation が自分の飾る fallback edge の predecessor / successor を正しく指し、両端 option が `declared access target` を持つ。
- `malformed`: annotation 自体はあるが、別の edge や別の option を指しており、declared information だけで same-lineage claim が壊れている。
- `underdeclared`: `declared access target` または edge-local annotation のどちらかが欠けており、same-lineage continuation を証明できない。current L2 では surface-level static error である。

既存理論との整合:

- canonical normalization law は変更していない。今回の整理は、canonical chain に入る前の same-lineage 証拠を書式面で 1 段具体化しただけである。
- incompatible branch rejection phase も変更していない。変えたのは、static evidence floor を満たすために最低限必要な annotation 形だけである。
- static evidence floor の core 条件も維持している。追加したのは「その `documented lineage annotation` は edge-local で 3 情報を持つ」という最小書式だけである。
- underdeclared handling も維持しており、annotation 欠落や target 欠落は hidden acceptance にせず surface-level static error に残している。
- reviewer は `no findings` だったため、edge-local な最小書式、説明用記法、underdeclared / malformed / valid の切り分けは、既存の canonical law・rejection phase・static evidence floor を壊していないと判断した。

## 7. 今回整理して分かったこと

- `declared access target` と `documented lineage annotation` の役割を分けると、target 一致だけでは same-lineage 証拠にならない理由を短く書けるようになった。
- 最小 surface form を edge-local に限定すると、option-local tag や global ID を持ち込まずに valid / malformed / underdeclared を区別できる。
- 説明用記法と規範的 token を分けて書くことで、current L2 を不必要に hard-code せずに例を置けることが確認できた。

## 8. 未解決事項

- `documented lineage annotation` の最終 keyword / punctuation / serialization
- edge-local な最小 form から option-local / chain-level sugar をどこまで許すか
- same-place / cross-place 文脈で同じ annotation shape をどこまで共有するか
- `declared contract surface` の「明示的矛盾」をどの粒度まで static に比較するか
- `declared capability surface` を read / write 以上に広げずに扱えない mixed case を static rejection と dynamic `Reject` のどちらに寄せるか

## 9. Suggested next prompt

```text
あなたはこのリポジトリに初めて入る CodeX です。会話の過去文脈は信用せず、リポジトリ内の文書を正本として扱ってください。

今回は、fallback / preference chain canonical normalization law・incompatible branch rejection phase・static evidence floor・underdeclared case handling・documented lineage annotation の最小 surface form を前提に、
same-place / cross-place 文脈で `documented lineage annotation` の shape を共有すべきかを整理してください。
目的は、current L2 の edge-local な最小書式を保ったまま、cross-place でも同じ annotation 形で十分か、それとも別の sugar / representation を許すべきかを最小限だけ明文化することです。

- Mir-0 を広げないでください。
- canonical law / rejection phase / static evidence floor / underdeclared handling / 最小 surface form 自体は変更しないでください。
- 新しい capability theory や distributed protocol detail を導入しないでください。
- `specs/04-mir-core.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`、`docs/reports/0020`、`docs/reports/0021`、`docs/reports/0022`、`docs/reports/0023` を読んでから始めてください。
- 毎回新しい report を `docs/reports/` に追加してください。
```
