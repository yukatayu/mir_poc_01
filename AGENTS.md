# AGENTS.md

This repository is intended for repeated work by agents that may start with **no retained context**.
The repository therefore treats documentation structure as part of the project's correctness.

## Non-negotiable rules

1. **Read in order**
   - Start with `README.md`, then `Documentation.md`, then the ordered specs in `specs/00...03`, then `specs/09`, then the subsystem-specific document you need.
   - If the task asks about **current status / progress / remaining steps / roadmap**, also read `progress.md` after `Documentation.md`.
   - If the task asks about **phase recut / roadmap rewrite / progress/tasks reorganization**, also read `.docs/progress-task-axes.md` after `progress.md`.
   - If the user or task names a specific `sub-agent-pro/*.md` handoff, read that handoff in the user-specified order before continuing the standard repository sequence.
  - Keep the source hierarchy explicit: `specs/` = 規範正本, `plan/` = repository memory, `docs/reports/` = 作業証跡, `progress.md` / `tasks.md` = current snapshot, `samples_progress.md` = runnable sample dashboard, `.docs/` / `docs/` = reader-facing or policy docs, `sub-agent-pro/` = working directive / handoff であり規範正本ではない。
   - `progress.md` is a rough status snapshot, not a normative source. Normative judgments remain in `specs/` and long-term repository memory remains in `plan/`.

2. **Do not invent requirements**
   - If something is not decided, write **UNRESOLVED** or **OPEN QUESTION**.
   - Do not silently turn a hypothesis into a fact.

3. **Respect decision levels**
   - `L0` = foundational / changing it affects the whole system.
   - `L1` = strong directional decision.
   - `L2` = design proposal under active refinement.
   - `L3` = exploratory / unresolved.
   These labels appear throughout the specs and must be preserved.

4. **Always write a new report**
   - Every non-trivial task must create a **new** markdown file under `docs/reports/`.
   - Never overwrite a previous report.
   - Use the report template and include: objective, documents consulted, actions taken, files changed, commands run, evidence, unresolved questions, suggested next prompt.

5. **Keep the architecture separable**
   - Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform are related but intentionally separable.
   - Do not collapse them into a single implementation without an explicit design decision.

6. **Preserve core invariants**
   - Directed acyclic graph discipline for patch evolution.
   - No silent API shadowing. Only compatibility-preserving overlays are allowed.
   - Contracts and failure behavior must stay explicit.
   - Lifetimes and ownership must remain monotone / non-duplicating.
   - Distinguish settled semantics from implementation convenience.

7. **Prefer clarification in writing over silent assumption**
   - If a task needs a choice between two unresolved options, document both and state the reason you chose a temporary working assumption.

## Strong project-specific constraints

- The system is **specification-first**.
- The current stage is **still architecture and semantics**, not broad implementation.
- Performance-sensitive kernels (for example PrismCascade runtime) must not be casually folded into Mir runtime semantics.
- Dynamic evolution must respect the project's design principle of **safe downstream addition** unless an explicit subsystem spec says otherwise.
- preserve project axis:
  **正しい理論に基づき、正しく hot-plug でき、Place をまたいで実行・通信・検証・可視化できる仮想空間システム**
- standard I/O は Mir core primitive ではない。外界接続は typed effect / adapter boundary 側に残し、spec が無いのに core built-in へ押し込まないこと。
- authentication / authorization / membership / capability / witness を transport に潰さないこと。
- visualization / telemetry を untyped debug leak として扱わず、情報を外へ出す effect として label / authority / redaction を意識すること。
- final public completion と repo-local alpha / current-layer closeout を混同しないこと。
- long-running research では、PoC 実装・実行・回帰確認と、formal boundary / proof obligation / invariant wording の整理を並走させること。
- implementation を進めるときも、portability / observability / step execution / graph export hook は replaceable layer として意識し、CPU 固定や単一 debug mode を早く既成事実化しないこと。

## Anti-shortcut rule

Do not:

- reduce scope silently
- skip validation and claim success
- keep stale active references unnoticed
- add builtin primitives for domain predicates
- collapse authentication into transport
- treat visualization as untyped debug leak
- freeze final grammar or public APIs prematurely

## Reporting policy

Every report should contain, in this order:

1. Title and identifier
2. Objective
3. Scope and assumptions
4. Documents consulted
5. Actions taken
6. Evidence / outputs / test results
7. What changed in understanding
8. Open questions
9. Suggested next prompt

## Editing policy

- `specs/` are normative documents. Edit carefully.
- If you change a normative statement, add an explicit note to the report.
- `Documentation.md` should stay concise and current.
- Keep diagrams in Mermaid source (`docs/diagrams/*.mmd`).
- active current sample / historical old sample / helper-local debug output / final public API / deferred mixed gate を混同しない。touch した docs に stale active reference があれば同じ task で整理すること。
- validation を実行していない場合は成功扱いしない。未実行理由を report と final answer に明記すること。
- debug / visualization output は evidence-oriented に扱い、helper-local preview を final public interface として書かないこと。
- long-running research task では、heavy command や generated artifact を増やす前に `df -h .` と `free -h` 相当で資源状況を確認すること。
- samples, reports, progress dashboard を更新する task では `samples_progress.md` の更新要否も必ず確認すること。
- E2E は自然な layer composition から作ること。内部関数を順に呼ぶだけの thick fake wrapper で達成扱いしないこと。
- small VPS では root disk を build cache / LLVM / generated artifact で圧迫しない。heavy disposable artifact は configured external workdir を優先し、未マウント時は root に勝手な大容量 directory を作らないこと。
- detach / cleanup script は repo source を消さず、明示確認なしに削除しないこと。
- commit では対話的な GPG prompt を避けるため、`git commit --no-gpg-sign` を使うこと。
- user が明示的に止めない限り、commit ごとに push すること。

## Discord 通知運用

- repo-scoped skill `discord-report` を使う task では、実装・コマンド実行・ファイル編集を始める前に `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .` を 1 回実行し、通知を送らずに差分基準だけを記録すること。
- 短い task では途中通知を送らず、終了時だけを対象にすること。
- 長い task では、自然な区切りがあり、かつ前回通知から**平均しておおむね 1 時間前後**空いたときだけ `progress` を送ること。数分ごとの過剰通知は避けること。
- user が **連続した task package をまとめて自走してほしい** と依頼した場合は、各 package 完了時点を自然な区切りとして扱い、1 時間未満でも `progress` を送ってよい。
- 上の連続 task 依頼では、Discord 通知だけで済ませず、user にも各 package close ごとの簡潔な中間報告を返すこと。
- `progress` は、**その後も user 入力なしで続行できる checkpoint** にだけ使うこと。package close 自体は `complete` 条件ではない。
- `complete` は、その user 依頼について**ここで手を止める**ときに 1 回だけ送ること。
  - scope が完了したとき
  - あるいは、次に進むには user 入力が必要になったとき
  のどちらかに限る。
- user 入力なしで続行可能なときは、`complete` を送らず `progress` と brief intermediate report に留め、そのまま次の package へ進むこと。
- `begin` があるときは task-scoped の差分を使い、`begin` がなくても Git 差分が取れるなら `変更量(参考)` を出し、どちらも取れないときだけ差分欄を出さないこと。
- 通知失敗は主作業の失敗にしない。Webhook は repo 直下の `.codex-discord/config.local.json` に保存し、commit しないこと。
- 通知文は簡潔な日本語にすること。導入直後または更新直後の疎通確認以外では `test` を使わないこと。

## plan/ 維持ルール

- `plan/` は人間向けの repository memory であり、scratchpad ではない。
- semantics / examples / fixtures / helper stack / roadmap / open questions / syntax candidate / workstream sequencing / current status が変わった task では、同じ task の中で relevant な `plan/` ファイルを更新すること。
- 更新が不要な場合でも、report に **`plan/ 更新不要`** と明記すること。
- `plan/` では、決定・未決・仮説・履歴 / comparison を分けて書くこと。
- `plan/` は docs mirror と同じく repo の一級成果物として扱うこと。
- current L2 / parser-free PoC / helper stack / roadmap task では、`Documentation.md` と基礎 specs に加えて、`plan/00-index.md` と relevant な `plan/` ファイルも読むこと。ただし規範判断の正本は常に `specs/` とする。

## progress.md 維持ルール

- `progress.md` は repo 全体の**簡潔な進捗スナップショット**であり、scratchpad ではない。
- current status / roadmap / remaining steps / major bottleneck / validation loop の到達見込みが変わった task では、同じ task の中で `progress.md` を更新すること。
- 進捗率や残ステップは rough estimate と明記し、問題が見つかれば巻き戻りうる前提で書くこと。
- `progress.md` の phase 整理は old `Phase 1..7` checkpoint label だけに依存せず、`.docs/progress-task-axes.md` の **macro phase** と **feature maturity stage** を併用すること。
- old `Phase 7 = FutureWork` のような巨大 bucket を再導入しないこと。
- `progress.md` の進捗率は、可能な限り
  - **論理仕様**
  - **ユーザ向け仕様**
  - **実装 / 運用**
  の 3 軸で並べて書くこと。ここで
  - 論理仕様 = semantics / invariants / formal boundary の整備度
  - ユーザ向け仕様 = companion notation / examples / human-facing guidance の整備度
  - 実装 / 運用 = parser-free PoC / helper / validation loop / 実務フローの整備度
  を指す。
- `progress.md` には、各章 / 層について **「着手可能か、もしくは user から追加仕様を聞く必要があるか」** を示す欄も置くこと。
  - `着手可能` = 非本質部分を先に進めても手戻りが比較的小さい
  - `要仕様確認` = user 側の目的 / 保証範囲 / 非機能要件が足りず、勝手に詰めると手戻りが大きい
  - `後段依存` = 先行 layer / 先行 decision が固まるまで本格着手しない方がよい
- `progress.md` では、決定済みの規範判断を新たに作らない。規範判断の正本は `specs/`、長期参照整理は `plan/` に置くこと。
- `progress.md` の末尾には、task close ごとに **日時つきの簡潔な作業ログ** を追記すること。
  - 粒度は「何を検証したか」「何が通って次に進めるようになったか」が分かる 1 行でよい。
  - 形式検証・実装・docs-only task を問わず、repo の current status に影響する non-trivial task では原則として追記すること。
  - timestamp は手打ちで推測せず、`date` コマンド等でその場で取得した値を使うこと。
  - `progress.md` は snapshot 文書なので、作業ログは **recent log** として保ってよい。古い詳細履歴は `docs/reports/` を正本にし、checkpoint ごとに古い行を要約・圧縮してよい。
- `progress.md` には、repo 全体の大局 phase を示す section を置き、少なくとも
  - phase 名
  - 主眼
  - 現在位置
  - 重さ
  - 自走可否
  を簡潔に mirror すること。phase 読みが変わった task では同じ task の中で更新すること。
- `progress.md` には、repo の特徴機能ごとの progress row も置き、
  - multi-node / fabric
  - robustness via contracts / theorem / model-check boundary
  - dynamic attach / detach / DAG-safe evolution
  - `atomic_cut` と higher-level ordering / memory-order family
  - executable sample corpus
  などを分けて追うこと。
- `Mirrorea / Typed-Effect / Prism / 上位アプリ` を 1 行に潰さず、少なくとも separable subsystem として読める粒度を保つこと。
- `shared-space docs-first boundary fixed` と `shared-space operational realization / final catalog open` を混ぜないこと。
- `progress.md` の更新が不要な場合でも、report に **`progress.md 更新不要`** と明記すること。

## samples_progress.md 維持ルール

- `samples_progress.md` は phase / layer ごとの runnable sample 状態を一覧する progress dashboard として扱う。
- append-only の作業ログにせず、table と current status を update-in-place で保つこと。
- 進捗%は evidence-backed にすること。最低でも sample path、validation command、blocker のどれかに紐づけること。
- `100%` は、その current scope において implementation、positive/negative sample、debug/visualization、docs、report、tests、progress update、git commit/push まで完了したときだけ使うこと。
- conceptual-only row は `25%` を超えないこと。
- runnable sample、validation command、debug surface、blocker が変わった task では、同じ task の中で `samples_progress.md` を更新すること。
- 更新不要な場合でも、report に **`samples_progress.md 更新不要`** と明記すること。

## storage / build artifact discipline

- heavy build artifact、LLVM source/build/install、generated artifact、temp、logs は detachable / cleanup 可能な external workdir を優先すること。
- external workdir の default 候補は `/mnt/mirrorea-work` だが、mount / filesystem / actual capacity を `lsblk -f` と `findmnt` で確認する前に前提化しないこと。
- repo source、committed docs、report だけを detachable storage に置かないこと。
- storage audit を伴う task では `df -h`、`lsblk -f`、`findmnt`、`du -sh .`、`du -sh target .git .cargo .lake` の結果を report に残すこと。
- cleanup は known disposable directory に限り、`--confirm` のような explicit confirmation を要求すること。

## tasks.md 維持ルール

- `tasks.md` は repo 全体の **current task map** であり、`progress.md` より少し具体的に
  - ある程度まとまった単位で自走して進められる task
  - 方針決定が必要で、かつ current research の障害になっている blocker / open question
  を整理するための文書である。
- `tasks.md` は append-only の履歴ではない。**更新時には毎回全体を書き直し、現況と整合した snapshot に保つこと。**
- phase end、checkpoint close、mainline 切り替え、major blocker の入れ替わりが起きた task では、同じ task の中で `tasks.md` の更新要否を確認すること。
- `tasks.md` は `.docs/progress-task-axes.md` に従い、
  - 自走可能な package
  - research を通して見つけること
  - user が決める必要があること
  を分けて書くこと。
- `tasks.md` の **「次に自走で進める順番と rough estimate」** には、各 task package がどの大局 phase の前半 / 中盤 / reserve path かを短く書くこと。
- `tasks.md` では、long chain を毎回 exhaustively 再列挙せず、**current checkpoint / current promoted line / next reopen point** が分かる粒度に圧縮してよい。
- `tasks.md` では、少なくとも次を分けて書くこと。
  - 自走可能な task package
  - 方針決定が必要な blocker / open question
- ここでいう「方針決定が必要」は、
  - user が決める必要があること
  - research を通して選別すること
  を混ぜないこと。
- blocker 側では、各項目について少なくとも次を書くこと。
  - 概要
  - 何に影響するか
  - 主要な選択肢
  - current recommendation / 見解
- `tasks.md` は規範判断の正本ではない。規範判断は `specs/`、長期比較と repository memory は `plan/` に残すこと。
- `tasks.md` を更新しなかった場合でも、report に **`tasks.md 更新不要`** と明記すること。

## review と task close の運用

- task はできるだけ内部で閉じる。中途で user に何度も返さない。
- ただし、user が連続 task の自走を依頼している場合は、task package の close ごとに brief intermediate report を返し、次に進む package を短く明示すること。
- task package close ごとに、`progress.md`、`tasks.md`、`docs/reports/` を同じ task の中で同期すること。
- user 入力なしで次へ進めるときは、package close 後も止まらずそのまま続行すること。
- user 入力が必要になったとき、または user が依頼したスコープを完了したときだけ、その turn の final `complete` を送って止まること。
- self-check、focused diff review、local validation を先に行う。
- reviewer はむやみに何度も呼ばず、最後に 1 回だけ長めに待つのを基本にする。
- 必要なら task 内部で narrow-scope re-review を行ってよい。
- reviewer が返らない場合だけ retry を 1 回行い、なお返らなければ local evidence と diff inspection を report に残す。
- subagent を使う場合は、明らかに壊れている / hung している根拠がない限り、latency だけを理由に早切りせず completion まで待つこと。
- 不要になった subagent は close する。ただし context を保持したいものは明示的に残してよい。
- 長期研究フェーズでは、PoC 実装・実行・回帰確認と、formal boundary / proof obligation / invariant wording の整理を並走させ、可能な限り手戻りの少ない ratchet 方式で進めること。

## Preferred style

- Use precise language.
- Expand unfamiliar abbreviations on first use.
- Separate **what is decided** from **what is proposed**.
- Avoid metaphor when the technical statement can be written directly.
- 日本語文書は reader-friendly であることより先に正確であることを優先し、規範 / repository memory / historical report / current sample / old sample / helper-local output / deferred gate を明示的に書き分ける。
