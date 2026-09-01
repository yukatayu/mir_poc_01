# AGENTS.md

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins; cite legacy `specs/` /
`plan/` as `LAB:` evidence unless mirrored into canon.

This repository is intended for repeated work by agents that may start with **no retained context**.
The repository therefore treats documentation structure as part of the project's correctness.

## ChatGPT Pro Oracle consults

When a difficult judgment, review, or stuck investigation would benefit from a
second opinion, use the browser-backed ChatGPT 5.5 Pro Extended Oracle wrappers.
Read `/home/codex/.codex/docs/oracle-chatgpt-pro.md` before first use in a
session, then follow the repo-local operating notes in
`.docs/oracle-chatgpt-pro-operations.md`.

Use `ask-chatgpt-pro-temp` for normal new consultations,
`ask-chatgpt-pro-followup` for real conversation continuation, and
`ask-chatgpt-pro` only when project-level continuity is genuinely needed. These
commands can take minutes and sometimes up to about an hour; wait patiently, check
`oracle status` / `oracle session` before retrying, and avoid duplicate runs
without concrete failure evidence.

For theory-heavy tasks, whole-project positioning, difficult roadmap choices,
or complex design review, prefer using Oracle proactively and asynchronously.
The main agent may delegate Oracle operation or monitoring to a sub-agent while
continuing non-overlapping local work, but the main agent remains responsible
for judging the result against repo evidence before mirroring anything into the
repo.

Oracle output is advisory. Mirror any useful result into the repo's normal
source hierarchy; do not treat an external chat as normative state.

## Non-negotiable rules

1. **Read in order**
   - Start with `mirrorea_canon/README.md`, then `mirrorea_canon/MAP.md`, then the task-specific canon files named from there. Use `CANON.md` as the root-level reminder of the source hierarchy.
   - Then read LAB entry points as needed: `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, and the relevant legacy `specs/` / `plan/` files as evidence, not as canon.
   - If the task asks about **current status / progress / remaining steps / roadmap**, also read `progress.md` and `tasks.md` as LAB snapshots after the canon files.
   - If the task asks about **phase recut / roadmap rewrite / progress/tasks reorganization**, also read `.docs/progress-task-axes.md` after `progress.md`.
   - If the user or task names a specific `sub-agent-pro/*.md` handoff, read that handoff in the user-specified order before continuing the standard repository sequence.
   - Keep the source hierarchy explicit: `mirrorea_canon/` = 規範正本, `specs/` / `plan/` = LAB evidence / repository memory, `docs/reports/` = 作業証跡, `progress.md` / `tasks.md` = current LAB snapshot, `samples_progress.md` = runnable sample dashboard, `.docs/` / `docs/` = reader-facing or policy docs, `sub-agent-pro/` = working directive / handoff であり規範正本ではない。
   - `progress.md` is a rough LAB status snapshot, not a normative source. Normative judgments remain in `mirrorea_canon/`; long-term historical LAB memory remains in `plan/`.

2. **Do not invent requirements**
   - If something is not decided, write **UNRESOLVED** or **OPEN QUESTION**.
   - Do not silently turn a hypothesis into a fact.
   - Under ADR-0014, research may progress autonomously in existing LAB lanes.
     An L3 `working/WRK-####` pre-registration needs the standing-eligibility
     check, alternative/falsifier, non-effects, and rollback trigger before
     evidence is relied on; it does not need an independent review to begin.
     L2 promotion is currently fail-closed pending an owner-authenticated trust
     anchor; its future route needs the rebased frozen final cut and
     independent review.
     A frozen L2 record is retained and followed by a successor, not demoted in
     place. Neither is an L0/L1 decision, implementation contract,
     or proof/ledger change.
   - The owner-approved Mir Theory v0 / I1+ Milestones 0--10 program and the
     Mirrorea I2 Systems Foundation SYS-0--SYS-7 program are both closed.
     ADR-0015 / Plan 247 and ADR-0026 / Plan 249 remain their immutable
     authority/history baselines; neither grants successor authority or remains
     a current queue. PROPOSAL-037 / ADR-0034 consume ADR-0033 and canon
     plan/05 for the active Mirrorea I3 Distributed Foundation bounded program.
     Plan 250 is the sole current roadmap; ALIGN-0 is completed and ALIGN-1 is
     the sole active goal. Program
     activation is not official I3 lifecycle entry; both transport candidates
     remain unselected. Outside program scope, follow ADR-0014's route.

3. **Respect decision levels**
   - `L0` = foundational / changing it affects the whole system.
   - `L1` = strong directional decision.
   - `L2` = design proposal under active refinement.
   - `L3` = exploratory / unresolved.
   These labels appear throughout the specs and must be preserved.

4. **One report per milestone**
   - Keep one report under `docs/reports/` for each active milestone by default.
   - Do not create reports solely for registration, evidence attachment,
     metadata links, snapshot synchronization, config/path wording, or closeout.
   - Add a forward report only for material counterevidence that must not rewrite
     the original milestone record. Never overwrite a closed report.
   - Use the report template and include every required section in order, including `plan/`, `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, and `samples_progress.md` update status, reviewer findings, skipped validations, commit / push status, and sub-agent close status.

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
   - A temporary assumption may remain a LAB working candidate. It may become a
     canon L3 pre-registration in `working/WRK-####` through ADR-0014's
     standing predicate; L2 promotion is currently fail-closed and, after an
     owner-authenticated trust anchor exists, additionally needs final-cut
     review.
     The ADR-0015 M0--M10 and ADR-0026 SYS-0--SYS-7 programs are closed and
     grant no successor authority. PROPOSAL-037 / ADR-0034 supply the current
     bounded successor authority; work outside it follows ADR-0014. Escalate changes to
     L0/L1, core/external contracts, SCN/Gate/Phase, any `theory/11` state, or a
     new moratorium-protected lane.

## Strong project-specific constraints

- The system is **specification-first**.
- The active milestone determines whether work is architecture, semantics,
  formalization, or bounded reference implementation. Do not start a later
  semantic milestone before closing the current one.
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
- claim final-public completion without evidence
- create thick fake E2E wrappers
- freeze final grammar or public APIs prematurely

## Repository organization discipline

- Keep Mir core, verification, Mirrorea runtime, adapters / host boundary, visualization, samples, and docs conceptually separate even when the current filesystem layout is still flat.
- Active executable samples must stay in documented active roots. Current active roots are `samples/clean-near-end/` for the clean suite, `samples/current-l2/` for the base source corpus, and `samples/lean/` for mechanization evidence.
- Planned skeleton families are not active samples. Keep them explicitly marked as planned and do not silently promote them into the active path.
- Historical samples should be archived rather than silently deleted.
- Generated artifacts must not be confused with source samples. If a generated artifact is committed for bridge evidence, say so explicitly in the touched docs.
- Sample or script taxonomy changes should update `samples/README.md`, `scripts/README.md`, and `samples_progress.md` in the same task.
- Every non-trivial restructuring is recorded in the current milestone report.
- No thick fake E2E wrappers: E2E samples must compose real layers and produce state / effect / witness / debug evidence.
- Heavy build artifacts must use the configured external workdir rather than repo root when that policy already exists.

## Reporting policy

The default unit is one milestone report. It accumulates the sections below
until milestone close. A second report is exceptional and requires material
counterevidence needing a forward-only record.

Every report should contain, in this order:

1. Title and identifier
2. Objective
3. Scope and assumptions
4. Start state / dirty state
5. Documents consulted
6. Actions taken
7. Files changed
8. Commands run
9. Evidence / outputs / test results
10. What changed in understanding
11. Open questions
12. Suggested next prompt
13. `plan/` update status
14. `Documentation.md` update status
15. `docs/project-status.md` update status
16. `progress.md` update status
17. `tasks.md` update status
18. `samples_progress.md` update status
19. reviewer findings and follow-up
20. skipped validations and reasons
21. commit / push status
22. sub-agent session close status

## Current-frontier discipline

- While a program is active, keep exactly one active semantic milestone and one
  designated current roadmap. A closed program may have neither; its last
  roadmap remains a closed record/regression baseline until owner direction
  designates a successor. Older `plan/` files remain repository memory, not an
  active queue. Plan 247 and Plan 249 are closed baselines. Plan 250 is the
  current roadmap and ALIGN-1 is the sole active milestone after ALIGN-0 close.
- Do not read `docs/reports/` in bulk. Read only reports directly referenced by
  current Canon, roadmap, or status.
- Open a new `WRK-####` only when it has a named direct consumer, reduces the
  current milestone blocker, cannot fit the milestone report, has an explicit
  falsifier, and has an adoption/discard rule. Do not reopen frozen/closed WRKs
  to manufacture progress.
- Compare at most the current proposal and one smallest viable alternative for
  a design question. If both fail, integrate their falsifiers before one
  successor is proposed.

## Editing policy

- `mirrorea_canon/` contains normative documents. Edit canon only through the canon process: proposal, owner decision, required ADR / changelog / index update. ADR-0015 and ADR-0026 record closed bounded programs and grant no successor authority. PROPOSAL-037 / ADR-0034 authorize only the fixed Plan 250 program. Outside it, ADR-0014 permits agent-maintained L3 work only in `working/WRK-####`, and L2 promotion remains fail-closed pending an owner-authenticated trust anchor.
- Legacy LAB `specs/` remain historical evidence and implementation memory. Edit carefully, and do not treat them as canonical unless the claim is mirrored into `mirrorea_canon/`.
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
- 通知文に backtick、quote、newline、長い commit list など shell が解釈し得る文字が入る場合は、inline `--summary` / `--next-step` ではなく、`--summary-file` / `--next-step-file` に UTF-8 text file を渡すこと。

## plan/ 維持ルール

- `plan/` は人間向けの repository memory であり、scratchpad ではない。
- semantics / examples / fixtures / helper stack / roadmap / open questions / syntax candidate / workstream sequencing / current status が変わった task では、同じ task の中で relevant な `plan/` ファイルを更新すること。
- 更新が不要な場合でも、report に **`plan/ 更新不要`** と明記すること。
- `plan/` では、決定・未決・仮説・履歴 / comparison を分けて書くこと。
- `plan/` は docs mirror と同じく repo の一級成果物として扱うこと。
- current L2 / parser-free PoC / helper stack / roadmap task では、canon files に加えて `Documentation.md`、relevant legacy specs、`plan/00-index.md` と relevant な `plan/` ファイルを LAB evidence として読むこと。ただし規範判断の正本は常に `mirrorea_canon/` とする。

## progress.md 維持ルール

- `progress.md` は repo 全体の**簡潔な進捗スナップショット**であり、scratchpad ではない。
- current status / roadmap / remaining steps / major bottleneck / validation loop の到達見込みが変わった task では、同じ task の中で `progress.md` を更新すること。
- 進捗率は primary metric にしないこと。current status は workflow readiness、evidence classification、remaining gate、blocker を中心に書くこと。
- `progress.md` の phase 整理は old `Phase 1..7` checkpoint label だけに依存せず、`.docs/progress-task-axes.md` の **macro phase** と **feature maturity stage** を併用すること。
- old `Phase 7 = FutureWork` のような巨大 bucket を再導入しないこと。
- `progress.md` では、可能な限り
  - **論理仕様**
  - **ユーザ向け仕様**
  - **実装 / 運用**
  の 3 軸で status を並べること。ここで
  - 論理仕様 = semantics / invariants / formal boundary の readiness と未決 gate
  - ユーザ向け仕様 = companion notation / examples / human-facing guidance の readiness と未決 gate
  - 実装 / 運用 = parser-free PoC / helper / validation loop / operational workflow の readiness と未決 gate
  を指す。
- `100%` は、外部開発者がその layer を実際に再現・使用できる operational workflow または product/public layer だけに使うこと。
- helper / sidecar / report / expected JSON / first-floor runner は completion ではなく evidence として分類すること。
- `progress.md` には、各章 / 層について **「着手可能か、もしくは user から追加仕様を聞く必要があるか」** を示す欄も置くこと。
  - `着手可能` = 非本質部分を先に進めても手戻りが比較的小さい
  - `要仕様確認` = user 側の目的 / 保証範囲 / 非機能要件が足りず、勝手に詰めると手戻りが大きい
  - `後段依存` = 先行 layer / 先行 decision が固まるまで本格着手しない方がよい
- `progress.md` では、決定済みの規範判断を新たに作らない。規範判断の正本は `mirrorea_canon/`、長期参照整理は LAB `plan/` に置くこと。
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
- `progress.md` には、repo の特徴機能ごとの workflow / evidence status row も置き、
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
- workflow readiness は evidence-backed にすること。最低でも sample path、validation command、blocker のどれかに紐づけること。
- `100%` は外部開発者がその layer を実際に再現・使用できる operational workflow または product/public layer だけに使うこと。helper / sidecar / report / expected JSON / first-floor runner は evidence として分類し、completion と書かないこと。
- conceptual-only row は planned / design-only / evidence category に留め、workflow-ready と書かないこと。
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
- `tasks.md` は規範判断の正本ではない。規範判断は `mirrorea_canon/`、長期比較と historical repository memory は LAB `plan/` に残すこと。
- `tasks.md` を更新しなかった場合でも、report に **`tasks.md 更新不要`** と明記すること。

## review と task close の運用

### Parent / sub-agent orchestration

- parent / main session は、current user objective（及び owner-authorized active
  goal）、Canon alignment、cross-layer integration、assignment boundary、milestone
  acceptance、final evidence を所有する。active goal がないときに新しい roadmap を
  捏造せず、sub-agent の局所成果を統合・受理する責任を手放さない。
- simple、low-risk、single-step 又は短い read-only task は、委譲コストが作業量と同程度
  なら parent が処理する。role が存在するという理由だけで sub-agent を起動しない。
- substantial で bounded、かつ独立に検証できる作業は専門 role へ委譲する。
  code mapping は `code_mapper`、non-test production source の実装と bounded debug fix
  は `implementer`、test は `test_author`、command execution と failure capture は
  `eval_runner`、routine evidence / status synchronization は `status_reporter`、
  repository-wide sequencing / dependency / evidence gate / roadmap / decision queue は
  `planner`、独立した semantic / correctness review は `reviewer` を基本とする。
- theory / normative analysis と cross-system trade-off の統合責任は parent が保持し、
  `planner`、`reviewer`、Oracle を advisory input として使う。decision は Canon の
  decision level / owner authority に従い、最終的な統合・受理判断は parent が repo
  evidence と照合して行う。
- 各委譲には bounded context packet として、pinned revision / dirty state、objective /
  non-goals、Canon / source references、preserved invariants、exact ownership、acceptance /
  validation、expected return を渡す。並列化は本当に独立した作業だけに使い、write
  ownership を重複させない。
- Oracle は repository 又は会話の暗黙文脈を持たない。consult ごとに relevant pinned
  facts、alternatives / falsifier、exact question を明示し、parent が回答を repo evidence
  と照合する。

- task はできるだけ内部で閉じる。中途で user に何度も返さない。
- user が連続 milestone の自走を依頼している場合は、milestone close ごとに brief intermediate report を返し、次を短く明示すること。
- milestone close ごとに `progress.md`、`tasks.md`、current milestone report を同期すること。micro-package ごとの新 report は作らない。
- user 入力なしで次へ進めるときは、milestone close 後も止まらずそのまま続行すること。
- user 入力が必要になったとき、または user が依頼したスコープを完了したときだけ、その turn の final `complete` を送って止まること。
- self-check、focused diff review、local validation を先に行う。
- 大局的な管理（current status、critical path、phase recut、lifecycle inventory、
  roadmap の更新）では、編集前と milestone close 前に Canon-first の独立
  `planner` review を受ける。review task は read-only として委譲し、pinned committed cut と差分を照合し、
  current blocker、direct consumer、owner/Canon と autonomous の境界、必要 evidence、
  stop / reopen trigger を確認する。source delta がなければ新しい plan を作らず、
  snapshot maintenance に限定する。reviewer としての planner は advisory であり、
  planning writer と同じ変更の final reviewer を兼ねない。
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
