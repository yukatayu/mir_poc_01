# 報告 0018 — fallback / 選択鎖 / `lease` の最小意味論整理

- 作成日時: 2026-03-30T23:35:00+09:00
- 作成者 / agent: Codex
- 対象範囲: `fallback`、preference chain、`lease`、guarded reference、期限切れ後書き込み、`try` / `atomic_cut` との整合に関する最小意味論整理
- 触れた decision level: 既存の Mir-0 境界は維持しつつ、より広い Mir 側の最小意味を L2 として補う

## 1. 目的

過去の断片的な記述として残っていた primitive fallback、preference chain、guarded reference、monotone lifetime の関係を、Mir-0 を広げずに矛盾なく読める最小仕様へ整える。
今回は新しい大理論や巨大な型拡張は導入せず、既存の `fallback` / `try` / `atomic_cut` / failure lattice と整合する最小の読み方だけを固定する。

## 2. 範囲と前提

- `AGENTS.md` の指示に従い、`README.md`、`Documentation.md`、`specs/00..04`、`specs/09`、`specs/10`、`specs/11`、`specs/12`、`docs/reports/0016`、`docs/reports/0017` を指定順で読んだ。
- 作業開始時点の git dirty state は空だった。既存 dirty state はない。
- 現行 repo とその local history を確認したが、`lease`、`GuardedRef`、`prefer`、`write after expiry` は正本に未導入だった。
- したがって今回は、`lease` を新しい独立理論としてではなく、既存の monotone lifetime / guarded reference を読むための最小 guard 語彙としてだけ置く。
- Mir-0 に残すものは従来どおり primitive fallback と monotone lifetime だけであり、preference chain の完全代数、surface syntax、dedicated observation surface は **未決定** として残す。

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
- `docs/reports/0016-japanese-canonicalization-and-semantic-drift-check.md`
- `docs/reports/0017-terminology-audit-and-cross-reference-alignment.md`

## 4. 実施内容

- `code_mapper` を最初に起動し、対象文書と git 状態を read-only で確認した。
- `fallback`、`preference chain`、`guarded reference`、`monotone degradation`、`atomic_cut`、`try`、`lease`、`期限切れ` で repo 全体と git history を検索し、現行正本で何が明示され、何が未導入かを切り分けた。
- `specs/04-mir-core.md` に、Mir-0 を広げないことと、この節自体が L2 の current reading であることを明記したうえで、「fallback / preference chain / `lease` の最小意味」節を追加した。
- 初回版では `specs/09-invariants-and-constraints.md` に `lease` 非復活を invariant として持ち上げてしまったが、reviewer 指摘を受けて撤回し、`specs/09` は元の強度へ戻した。
- `specs/10-open-questions.md` では、今回固定した最小理解と、なお未決定な surface syntax / algebra / observation surface を切り分けた。
- `specs/12-decision-register.md` に D-024 を追加し、Mir-0 は維持したまま、より広い Mir 側で何を最小判断として採るかを記録した。
- `reviewer` を最後に起動し、Mir-0 を広げていないか、`lease` を過剰に first-class 化していないかを点検した。
- commit は 2 本に分けた。
  - 1 本目は仕様本文だけを記録する `fallback と lease の最小意味を整理する`
  - 2 本目は report を記録する `fallback / preference chain / lease の作業報告を追加する`

今回実際に変更したファイル:

- `specs/04-mir-core.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`

確認したが今回変更しなかったファイル:

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0016-japanese-canonicalization-and-semantic-drift-check.md`
- `docs/reports/0017-terminology-audit-and-cross-reference-alignment.md`

## 5. コマンドと正確な出力

1. `git status --short`

```text
```

2. `rg -n "fallback|lease|prefer|preference chain|GuardedRef|寿命|劣化|再昇格|atomic_cut|try|期限切れ|write|書き込み" README.md Documentation.md specs docs/reports/0016-japanese-canonicalization-and-semantic-drift-check.md docs/reports/0017-terminology-audit-and-cross-reference-alignment.md`

```text
Documentation.md:30:  - fallback / preference chains の完全意味論
docs/reports/0016-japanese-canonicalization-and-semantic-drift-check.md:17:- formal token、identifier、decision ID、L0〜L3 ラベル、failure lattice の名前、`atomic_cut` / `durable_cut` / `place` などの形式語彙は保持し、説明文だけを日本語化した。
docs/reports/0016-japanese-canonicalization-and-semantic-drift-check.md:174:specs/04-mir-core.md: fallback:6->8; patch:0->2; overlay:2->4
docs/reports/0016-japanese-canonicalization-and-semantic-drift-check.md:207:  - `Mir-0`, `Mir-1`, `atomic_cut`, `durable_cut`, `place`, `fallback`, `overlay alias`, `Reject`, `Approximate`, `Compensate`, `require`, `ensure`, `invariant` は保持した。
docs/reports/0016-japanese-canonicalization-and-semantic-drift-check.md:215:- `overlay`, `patch`, `fallback` の増加は、既存に分散していた説明文を formal token に寄せて表現統一したことによる表記揺れの収束であり、仕様内容を増やしたものではない。
docs/reports/0016-japanese-canonicalization-and-semantic-drift-check.md:221:  - Mir 全体と Mir-0 の区別、`perform` の地位、`atomic_cut` / `durable_cut` の境界は意味変更に直結するため、formal token を保持したまま説明だけを日本語化した。
docs/reports/0017-terminology-audit-and-cross-reference-alignment.md:169:fatal: failed to write commit object
specs/11-roadmap-and-workstreams.md:23:- fallback / try / cut を支援する
specs/12-decision-register.md:17:| D-011 | Mir-0 boundary | Mir-0 を、event DAG、`place`、最小 effect request operation（ここでは `perform` と表記）、effect / contract、最小 failure space、primitive fallback、local `try` / rollback、place-local `atomic_cut`、linear resource から成る最小 kernel として固定し、`barrier`、`durable_cut`、完全な fallback normalization、`emit`、coroutine semantics、overlay alias detail は後段へ送る | L1 | Workstream A を安定化させつつ、より広い Mir / Mirrorea machinery は決めない |
specs/12-decision-register.md:18:| D-012 | Mir-0 の cut / 語彙境界 | `atomic_cut` は current `place` の rollback frontier だけを確定する Mir-0 の最小 finalizing cut とする。単一 process 全体の同期、分散合意、永続化完了は Mir-0 に含めない。`durable_cut` と `barrier` は Mir-1 以降の cut vocabulary 側に送る。`perform` は当面、最小 effect request operation を説明するための記法であり、Mir-0 の規範的な表面構文は未決定とする | L1 | Mir-0 を広げず、cut と語彙の境界だけを狭く固定する |
specs/12-decision-register.md:26:| D-020 | `all_of` aggregate failure justification | `all_of` では full coverage 不成立だけでは aggregate failure を確定しない。aggregate failure event を正当化できるのは、required member の local attempt / local prefix に explicit local failure が立って同じ aggregate attempt の full coverage が不可能になった場合、または aggregate success より前にその aggregate attempt が failed outcome として明示的に閉じられた場合に限る。member-local failure event 自体は aggregate failure event の代用ではない。timeout-like budget、policy cancellation、retry exhaustion などの closure reason taxonomy は Mir-1 の独立語彙にせず、Mirrorea の operational policy / audit に残す | L1 | coverage 不足と failure 確定を分離しつつ、aggregate failure の十分条件だけを最小限で固定する |
specs/10-open-questions.md:7:1. Mir-0 の primitive fallback を超えた、preference chain / fallback normalization の最終形式化。
specs/10-open-questions.md:8:2. `durable_cut` は Mir-0 に含めない。Mir-1 側では、`atomic_cut` に abstract persistence requirement を伴う durable commit guarantee を追加する cut vocabulary 候補として扱う。
specs/10-open-questions.md:31:   - `all_of` では、required member local failure がまだ出ていない場合でも、同じ aggregate attempt が aggregate success event より前に failed outcome として明示的に閉じられたなら、aggregate failure event を出してよい。Mir-1 が意味として要求するのは、その failed closure が当該 aggregate attempt の success を打ち切ることだけであり、timeout-like budget、policy cancellation、retry exhaustion、route / transport closure などの具体理由は Mirrorea の operational policy / audit に残す。
specs/10-open-questions.md:34:   - justification source が explicit failed closure である場合、audit はその aggregate attempt を failed outcome として閉じた act を参照できなければならない。timeout-like budget、policy cancellation、retry exhaustion などの具体理由分類は、その act に付随する Mirrorea 側表現に残し、Mir-1 では固定しない。
specs/01-charter-and-decision-levels.md:51:- preference chain の正確な形
specs/09-invariants-and-constraints.md:8:2. cut は明示的 decision boundary を作る。Mir-0 では `atomic_cut` だけを最小の place-local cut primitive として扱う。`atomic_cut` は current `place` の rollback frontier を確定するが、単一 process 全体・分散系・永続化の finalization は意味しない。`durable_cut` のような後段の cut vocabulary は scope を広げうるが、同じ explicit-finalization discipline を守らなければならない。
specs/09-invariants-and-constraints.md:21:9. primitive fallback を超えて完全に形式化された後の preference / fallback chain は、曖昧さのない monotone form へ正規化されなければならない。
specs/09-invariants-and-constraints.md:26:11. rollback は、compensation へ変換されない限り finalizing cut を越えてはならない。Mir-0 では、これは current `place` 内の rollback が先行する `atomic_cut` を越えられないことを意味する。
specs/04-mir-core.md:22:- より広い Mir の cut vocabulary では以前から `barrier`、`atomic_cut`、`durable_cut` が言及されていたが、Mir-0 が固定するのは `atomic_cut` だけである。`barrier` と `durable_cut` の扱いは後段へ送る。
specs/04-mir-core.md:34:- lifetime と fallback 風の degradation は monotone に保たれなければならない。
specs/04-mir-core.md:35:- preference chain と guarded reference は現在も活発な設計論点である。
specs/04-mir-core.md:82:### 5. Primitive fallback
specs/04-mir-core.md:84:- Mir-0 は、明示的 failure または monotone degradation の後に、次の contract-compatible option へ進む primitive として fallback を含む。
specs/04-mir-core.md:85:- fallback は、隠れた backtracking、隠れた API shadowing、linear resource の duplication を導入してはならない。
specs/04-mir-core.md:86:- nested fallback / preference chain の完全な normalization は Mir-0 の外にある。
specs/04-mir-core.md:88:### 6. local rollback を伴う `try`
specs/04-mir-core.md:90:- Mir-0 は、local rollback semantics を持つ `try` を含む。
specs/04-mir-core.md:92:- Mir-0 における `try` の rollback frontier は current `place` 内で閉じており、途中に `atomic_cut` があればそこで停止する。
specs/04-mir-core.md:96:### 7. `atomic_cut`
specs/04-mir-core.md:98:- Mir-0 は、最小の place-local finalizing cut primitive として `atomic_cut` を含む。
specs/04-mir-core.md:99:- Mir-0 において `atomic_cut` が確定するのは current `place` の rollback frontier だけである。
specs/04-mir-core.md:100:- `atomic_cut` は、単一 process 全体の同期点、複数 `place` をまたぐ合意点、永続化完了点を意味しない。
specs/04-mir-core.md:101:- `place` の内部では、`atomic_cut` は rollback が通過できない明示的 decision boundary を作る。
specs/04-mir-core.md:102:- `atomic_cut` の後で failure が表面化しても、同じ `place` の pre-cut 部分は rollback されない。以後は compensation、fallback、または明示的 failure として扱う。
specs/04-mir-core.md:103:- Mir-0 は `atomic_cut` の rollback / finalization 上の役割を固定するが、それだけで persistence や distributed finalization の semantics を決めることはしない。
specs/04-mir-core.md:108:- Mir-0 は、rollback、fallback、後続の進化 machinery による duplication を防ぐために必要な最小 linear-resource discipline を含む。
specs/04-mir-core.md:115:- `durable_cut` は Mir-0 に含めない。Mir-1 側では、`atomic_cut` に abstract persistence requirement を伴う durable commit guarantee を追加する cut vocabulary 候補として扱う。
specs/04-mir-core.md:126:- **UNRESOLVED**: fallback の完全 normalization と、preference chain の完全代数。
Documentation.md:30:  - fallback / preference chains の完全意味論
```

3. `for rev in $(git rev-list --all); do git grep -n -E "GuardedRef|lease|寿命つき参照|保護付き参照|期限切れ|再昇格" "$rev" -- README.md Documentation.md specs docs/reports || true; done`

```text
```

4. `python3 scripts/new_report.py --slug fallback-preference-chain-and-lease-semantics`

```text
/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).
  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")
/home/yukatayu/dev/mir_poc_01/docs/reports/0018-fallback-preference-chain-and-lease-semantics.md
```

5. 変更後の `git status --short`

```text
 M specs/04-mir-core.md
 M specs/10-open-questions.md
 M specs/12-decision-register.md
?? docs/reports/0018-fallback-preference-chain-and-lease-semantics.md
```

6. `git diff --check`

```text
```

7. `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 19 numbered report(s).
```

8. `reviewer` 初回結果

```text
1. High: `lease`/preference-chain 整理が、文書内のレベル付け上は L2 ではなく L0/L1 相当に読めてしまいます。[specs/04-mir-core.md:5](/home/yukatayu/dev/mir_poc_01/specs/04-mir-core.md#L5) では「明示的に別記されていない限り」この文書は L0/L1 扱いです。しかし追加節 [specs/04-mir-core.md:113](/home/yukatayu/dev/mir_poc_01/specs/04-mir-core.md#L113) はレベル注記なしで「固定する」と書いており、`lease`、guarded option chain、write-after-expiry=`Reject` まで core 側の確定事項として読まれます。これは [specs/12-decision-register.md:30](/home/yukatayu/dev/mir_poc_01/specs/12-decision-register.md#L30) の D-024 が L2 としている整理と噛み合っていませんし、[specs/10-open-questions.md:7](/home/yukatayu/dev/mir_poc_01/specs/10-open-questions.md#L7) が「最終形式化」を未解決として残していることとも緊張します。さらに [specs/09-invariants-and-constraints.md:20](/home/yukatayu/dev/mir_poc_01/specs/09-invariants-and-constraints.md#L20) と [specs/09-invariants-and-constraints.md:26](/home/yukatayu/dev/mir_poc_01/specs/09-invariants-and-constraints.md#L26) が `lease` 非復活を invariant 側に持ち上げているため、Mir-0 を広げない／`lease` を大きな理論語彙として固定しすぎない、という意図より強く読まれるリスクがあります。

他は no additional findings です。今回の4ファイル内では、primitive fallback / monotone lifetime / `try` / `atomic_cut` との直接矛盾や、write-after-expiry が failure lattice / contract discipline を壊す明白な破綻までは見当たりませんでした。
```

9. reviewer 指摘対応後の `git diff --check`

```text
```

10. reviewer 再確認

```text
no findings
```

11. `git commit --no-gpg-sign -m "fallback と lease の最小意味を整理する"`

```text
[main 7d60b11] fallback と lease の最小意味を整理する
 3 files changed, 29 insertions(+), 1 deletion(-)
```

## 6. 証拠 / 所見

今回固定した最小意味:

- `fallback` は引き続き primitive であり、preference chain はその一次 primitive ではなく正規形である。
- `lease` は later cut vocabulary や新 failure class ではなく、各 option の lifetime guard を説明するための最小語彙としてだけ置いた。
- monotone degradation と再昇格禁止は、既存の lifetime monotonicity を guarded option chain に引き延ばした読みとして整理した。
- `try` rollback は local state を巻き戻せても、期限切れた `lease` や degradation order を復活させないとした。これにより rollback と lifetime monotonicity を両立させた。
- write-after-expiry は hidden write を許さず、後段の write-capable option への explicit fallback がなければ `Reject` とした。これにより failure の明示性と contract discipline を崩さない。
- reviewer 指摘により、上の整理は L0/L1 invariant ではなく L2 の current reading としてだけ扱うべきことが明確になった。そのため `specs/04` にレベル注記を加え、`specs/09` への `lease` 持ち上げは撤回した。
- commit 粒度は、仕様本文と report を分離した。これにより、理論判断と監査記録を履歴上でも独立に追いやすくした。

今回 core に残したもの:

- primitive fallback
- preference chain は nested fallback の正規形であること
- `lease` は lifetime guard に留まること
- monotone degradation と再昇格禁止
- write-after-expiry の最小既定規則

今回 open question に送ったもの:

- `lease` / `GuardedRef` の最終 surface syntax
- preference chain の完全代数と完全 normalization law
- static capability rule / capability lattice
- `lease` 期限切れを dedicated event として観測するかどうか

AMBIGUOUS:

- `lease` という語自体は repo の現行正本に存在しなかったため、今回は guarded reference と monotone lifetime を読むための最小補助語彙として採用した。将来 formal token に昇格させるかは未決定である。
- fallback 付き参照の「reference」が pointer-like reference なのか、より広い capability / access path なのかは原文側で未固定だったため、今回は最も狭い共有部分として「論理的 access path」と表現した。
- 後段 option での弱い成功を ordinary success とみなすか `Approximate` とみなすかは contract surface に依存すると整理したが、その contract surface 自体は未完成である。

## 7. 今回整理して分かったこと

- 現行正本にすでにあるのは、Mir-0 の primitive fallback、monotone lifetime、rollback / cut discipline までであり、preference chain と guarded reference の関係は未接続だった。
- `lease` を独立 primitive にせず lifetime guard へ留めると、Mir-0 を広げずに「期限切れ後の単調劣化」「再昇格禁止」「write-after-expiry」の最小説明が可能になる。
- `atomic_cut` は今回の論点に直接新しい意味を足さず、rollback 越境禁止を維持するだけで整合が取れる。

## 8. 未解決事項

- `lease` を本当に formal token として採用するか。
- `GuardedRef` のような convenience vocabulary を導入するか。
- preference chain の完全 normalization law と完全代数をどこまで固定するか。
- read / write / degrade をまたぐ capability lattice を静的に持つか。
- `lease` 期限切れの dedicated event / audit surface を別途持つか。

## 9. Suggested next prompt

```text
あなたはこのリポジトリに初めて入る CodeX です。会話の過去文脈は信用せず、リポジトリ内の文書を正本として扱ってください。

今回は、fallback / preference chain / lease の最小意味論を前提に、`lease` 期限切れの観測面を整理してください。
目的は、新しい failure class を増やさずに、期限切れが explicit failure / fallback / audit のどこで観測されるべきかを最小限の意味論として定めることです。

- Mir-0 を広げないでください。
- `lease` を later cut vocabulary に接続しないでください。
- `specs/04-mir-core.md`、`specs/09-invariants-and-constraints.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md`、`docs/reports/0018-fallback-preference-chain-and-lease-semantics.md` を読んでから始めてください。
- event surface と audit surface を分け、期限切れを dedicated event にする必要があるかを検討してください。
- 毎回新しい report を `docs/reports/` に追加してください。
```
