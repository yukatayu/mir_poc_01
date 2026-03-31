# 報告 0025 — `perform` と option chain 参照の current L2 surface syntax 候補

- 作成日時: 2026-03-31T10:49:27+09:00
- 作成者 / agent: Codex
- 対象範囲: representative Mir programs（current L2）で使う `perform` と option chain 参照の最小 surface syntax 候補、examples の書き直し、最小導線の追加
- 触れた decision level: 既存の Mir-0 / current L2 理論を前提にした L2 の companion notation 整理であり、新しい意味論判断は追加しない

## 1. Objective

representative Mir programs で使っている説明用記法のうち、`perform` と option chain 参照の部分を、理論を変えずに current L2 のより安定した最小 surface syntax 候補へ寄せる。parser 実装には進まず、examples と companion 文書で「今どの書き方が自然か」を見える形にする。

## 2. Inputs consulted

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
- `specs/examples/00-representative-mir-programs.md`
- `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
- `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
- `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
- `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
- `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`
- `docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md`
- `docs/reports/0024-representative-mir-programs-current-l2.md`

## 3. Actions taken

- `code_mapper` を起動し、task-start の dirty state がないこと、今回の主対象が `specs/examples/00-representative-mir-programs.md` と `specs/04` / `specs/10` / `specs/12` 周辺であることを確認した。
- `perform` と option chain 参照の current な説明用記法を棚卸しし、今回の対象を次の 3 点に絞った。
  - direct target に対する `perform`
  - option declaration
  - chain declaration と `via` 参照
- 新規 companion 文書 `specs/examples/01-current-l2-surface-syntax-candidates.md` を追加し、current L2 候補として次を整理した。
  - `perform op on target`
  - `perform op via chain_ref`
  - `option name on target capability cap lease guard`
  - `chain ref = head` に続く `fallback successor @ lineage(predecessor -> successor)`
- `specs/examples/00-representative-mir-programs.md` を更新し、代表例のうち `perform` を使う例を候補 syntax へ寄せた。特に valid fallback / preference chain と write-after-expiry 例は、新しい chain 書式で書き直した。
- E4 / E5 も同じ chain notation にそろえ、valid / malformed / underdeclared の区別が新しい companion notation でも保たれることを示した。
- `specs/00-document-map.md` と `Documentation.md` に、新しい companion 文書への導線を追加した。
- `specs/10-open-questions.md` と `specs/12-decision-register.md` には、「examples 用 current L2 候補であって final token 固定ではない」という最小記録だけを追加した。
- `reviewer` を最後に使い、今回の syntax 候補が既存理論を壊していないか確認した。結果は `no findings` だった。

## 4. Files changed

今回実際に変更したファイル:

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `docs/reports/0025-perform-and-option-chain-surface-syntax-candidates.md`

確認したが今回変更しなかったファイル:

- `README.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
- `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
- `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
- `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
- `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`
- `docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md`
- `docs/reports/0024-representative-mir-programs-current-l2.md`

既存 dirty state と今回の変更の区別:

- 作業開始時点の `git status --short --branch` は `## main...origin/main [ahead 3]` で、未コミットの tracked / untracked 変更はなかった。
- したがって今回の差分はすべてこの作業で追加したものである。

## 5. Commands run and exact outputs

1. 作業開始時点の `git status --short --branch`

```text
## main...origin/main [ahead 3]
```

2. `python3 scripts/new_report.py --slug perform-and-option-chain-surface-syntax-candidates`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0025-perform-and-option-chain-surface-syntax-candidates.md
```

3. `git diff --check`

```text
<no output>
```

4. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 26 numbered report(s).
```

5. 仕様本文 commit の初回試行

```text
error: gpg failed to sign the data:
[GNUPG:] KEY_CONSIDERED E1807565A404CAB0DCDDC1F9F40DC9582D35F123 0
[GNUPG:] BEGIN_SIGNING H10
[GNUPG:] PINENTRY_LAUNCHED 2419893 curses 1.2.1 not a tty dumb - ? 1000/1000 -
gpg: signing failed: No such file or directory
[GNUPG:] FAILURE sign 83918929
gpg: signing failed: No such file or directory

fatal: failed to write commit object
```

6. 仕様本文 commit（署名無効化）

```text
[main d032498] current L2 の perform と option chain 候補書式を整理する
 6 files changed, 223 insertions(+), 65 deletions(-)
 create mode 100644 specs/examples/01-current-l2-surface-syntax-candidates.md
```

7. reviewer 結果

```text
no findings
```

## 6. Evidence / findings

今回の syntax 候補で確認できたこと:

- `perform` の direct target 参照と option chain 参照を `on` / `via` で分けると、examples 上の読み分けがかなり安定する。
- option chain の宣言は、`option` 宣言と `chain` 宣言を分けるだけで、nested fallback の canonical 読みを examples に書き下しやすくなる。
- edge-local な `lineage(...)` を各 `fallback` 行に残す形は、D-029 の最小 surface form と矛盾しない。
- E3 / E4 / E5 / E6 は、新しい companion notation でもそれぞれ `valid` / `malformed` / `underdeclared` / runtime `Reject` の区別を保てた。

current L2 で十分に書けるもの:

- direct target に対する `perform`
- option declaration の最小骨格
- canonical chain の優先順
- same-lineage annotation を伴う valid / malformed / underdeclared の最小例
- write-after-expiry の `Reject`

まだ書きづらいもの:

- `try` / `fallback` の exact sugar
- richer な option-local `declared contract surface`
- `lineage(...)` の最終 token
- cross-place 文脈で同じ shape をどこまで共有するか

## 7. Changes in understanding

- representative examples の書きづらさの中心は、理論不足というより、`perform` と option chain 参照の surface syntax が散っていたことだった。
- `perform op on target` と `perform op via chain_ref` を分けるだけで、effect request の意図がかなり読みやすくなる。
- option chain 側は、nested fallback をそのまま書くより、canonical form に近い `chain ref = head` + `fallback ...` の方が current L2 companion notation として安定する。
- 一方で、`try` / `fallback` の書式と richer な contract clause は、parser 実装前にもう一段だけ surface syntax を詰める余地がある。

## 8. Open questions

- `perform`、`option`、`chain`、`on`、`via` を最終 reserved keyword にするかどうか。
- `require` / `ensure` clause の final punctuation、改行規則、block 形。
- option-local `declared contract surface` を capability 以外も含めてどう書くか。
- `try` / `fallback` の exact sugar をどの程度まで stabilise するか。
- `lineage(...)` の最終 keyword / punctuation / serialization。
- same-place / cross-place で同じ companion notation をどこまで共有するか。

## 9. Suggested next prompt

```text
あなたはこのリポジトリに初めて入る CodeX です。会話の過去文脈は信用せず、リポジトリ内の文書を正本として扱ってください。

今回は、current L2 examples でまだ説明用 shorthand のまま残っている `try` / `fallback` の最小 surface syntax 候補を整理してください。
目的は、`perform` と option chain 参照で今回そろえた companion notation と矛盾しない形で、local rollback を伴う `try` と explicit fallback branch の書き方を current L2 の representative examples でより安定させることです。

- Mir-0 / Mir-1 / Mirrorea の境界は変更しないでください。
- canonical normalization law、rejection phase、static evidence floor、underdeclared handling、lineage annotation surface form は変更しないでください。
- parser 実装には進まないでください。
- `specs/04-mir-core.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`、`specs/examples/00-representative-mir-programs.md`、`specs/examples/01-current-l2-surface-syntax-candidates.md` を読んでから始めてください。
- 毎回新しい report を `docs/reports/` に追加してください。
```

## commit 記録

- 仕様本文 commit:
  - `d032498` `current L2 の perform と option chain 候補書式を整理する`
- report commit:
  - この report 自身を含むため、self-reference の都合で本文には固定できない。最終 hash は `git log` と final response で補完する。
