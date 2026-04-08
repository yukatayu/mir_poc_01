# AGENTS.md

This repository is intended for repeated work by agents that may start with **no retained context**.
The repository therefore treats documentation structure as part of the project's correctness.

## Non-negotiable rules

1. **Read in order**
   - Start with `README.md`, then `Documentation.md`, then the ordered specs in `specs/00...03`, then `specs/09`, then the subsystem-specific document you need.
   - If the task asks about **current status / progress / remaining steps / roadmap**, also read `progress.md` after `Documentation.md`.
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
- long-running research では、PoC 実装・実行・回帰確認と、formal boundary / proof obligation / invariant wording の整理を並走させること。
- implementation を進めるときも、portability / observability / step execution / graph export hook は replaceable layer として意識し、CPU 固定や単一 debug mode を早く既成事実化しないこと。

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
- long-running research task では、heavy command や generated artifact を増やす前に `df -h .` と `free -h` 相当で資源状況を確認すること。
- commit では対話的な GPG prompt を避けるため、`git commit --no-gpg-sign` を使うこと。
- user が明示的に止めない限り、commit ごとに push すること。

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
- `progress.md` の更新が不要な場合でも、report に **`progress.md 更新不要`** と明記すること。

## review と task close の運用

- task はできるだけ内部で閉じる。中途で user に何度も返さない。
- self-check、focused diff review、local validation を先に行う。
- reviewer はむやみに何度も呼ばず、最後に 1 回だけ長めに待つのを基本にする。
- 必要なら task 内部で narrow-scope re-review を行ってよい。
- reviewer が返らない場合だけ retry を 1 回行い、なお返らなければ local evidence と diff inspection を report に残す。
- subagent を使う場合は、明らかに壊れている / hung している根拠がない限り、latency だけを理由に早切りせず completion まで待つこと。
- 不要になった subagent は close する。ただし context を保持したいものは明示的に残してよい。

## Preferred style

- Use precise language.
- Expand unfamiliar abbreviations on first use.
- Separate **what is decided** from **what is proposed**.
- Avoid metaphor when the technical statement can be written directly.
