# Report 0905 — faq 012 alpha status and two big problems

- Date: 2026-04-23 08:20 JST
- Author / agent: Codex
- Scope: current status synthesis for `faq_012.md`, with explicit clarification that the two big problems include theory construction, implementation, and semantics-aligned public syntax
- Decision levels touched: no normative semantic change; documentation / explanation refresh only

## 1. Objective

Write `faq_012.md` so that it accurately answers:

- where the repository currently stands
- whether the two big problems are already completely solved
- whether language-side implementation is already complete
- where the repo sits in the full roadmap
- which remaining decisions are mixed-gate versus true user-spec gate

while reflecting the post-`0904` clean near-end alpha closeout rather than the
older `p..` / problem-bundle explanation layer.

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `samples/clean-near-end/README.md`
- `faq_007.md`
- `faq_010.md`
- `faq_011.md`
- `docs/reports/0904-clean-near-end-alpha-release-completion.md`
- user clarification in this task:
  - Problem 1 = 型システム / 定理証明 / モデル検査
  - Problem 2 = `memory_order` 再解釈の実装
  - both include theory construction, implementation, and semantics-aligned public syntax

## 3. Actions taken

1. Re-read the current snapshot documents and normative anchors in the
   repository-prescribed order.
2. Checked how the repository itself defines `二大問題` in prior FAQs and
   handoff material.
3. Re-bound that phrase to the user's stricter definition for this task.
4. Pulled fresh current-state anchors with:
   - `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   - `python3 scripts/current_l2_guided_samples.py closeout --format json`
5. Wrote `faq_012.md` as a post-`0904` explanation refresh.
6. Updated `specs/00-document-map.md` so the new FAQ is discoverable from the
   repository's document map.

## 4. Files changed

- Added:
  - `faq_012.md`
- Updated:
  - `specs/00-document-map.md`
  - this report

`plan/` 更新不要
`progress.md` 更新不要
`tasks.md` 更新不要

Reason:
this task clarified status and completion boundaries, but did not change the
underlying repo state itself.

## 5. Commands run and exact outputs

- `python3 scripts/new_report.py --slug faq-012-alpha-status-and-two-big-problems`
  - output:
    `/home/yukatayu/dev/mir_poc_01/docs/reports/0905-faq-012-alpha-status-and-two-big-problems.md`
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - exit 0
  - key result:
    - typing family reported 1 valid + 4 malformed
    - order-handoff family reported 4 valid + 2 malformed
    - model-check family reported 1 pass + 2 counterexamples
    - modal family reported 2 valid
    - matrix reported `total_samples = 16`
- `python3 scripts/current_l2_guided_samples.py closeout --format json`
  - exit 0
  - key result:
    - `active_sample_root = samples/clean-near-end`
    - `archive_sample_root = samples/old/2026-04-22-pre-clean-near-end`
- `date '+%Y-%m-%d %H:%M %Z'`
  - output:
    `2026-04-23 08:20 JST`

## 6. Evidence / findings

### 6.1 Current status finding

The repository has genuinely moved beyond the state described in `faq_011.md`.
The principal current layer is now:

- clean near-end active suite
- first finite-index typing layer
- order/handoff high-level relation family
- model-check second line for mutex / weak memory
- Lean foundations + clean generated stubs

This is stronger than "near-end with corrected prototypes" and should be read as
`repo-local alpha-ready current layer`.

### 6.2 Completion-bound finding

Even with that progress, the user's stricter definition of the two big problems
is **not yet complete**.

Why:

- Problem 1 is not yet complete in final theory / final public syntax / final
  public theorem+model-check contract terms.
- Problem 2 is not yet complete in final public stance for `memory_order`,
  final source wording, final emitted/public contract, or semantics-aligned
  public syntax terms.
- The language side is not yet complete because final parser grammar, final
  public parser/checker/runtime/verifier API, and packaging remain deferred.

### 6.3 Roadmap finding

The repo is currently best described as:

- beyond comparison-floor and first executable-floor uncertainty
- at `repo-local alpha-ready current layer`
- before final public syntax / public API / production-binding completion

### 6.4 Remaining-input finding

The remaining work separates cleanly into:

- mixed-gate narrowing
  - final typed source principal
  - final order/handoff public wording
  - theorem/model-check/witness-provider public contracts
  - parser/public API boundary
- true user-spec gate
  - packaging / distribution target
  - installed binary / FFI / host integration target
  - broader application target
  - completion criteria for "clean public syntax"

## 7. Changes in understanding

- The biggest explanatory change after `0904` is not just "more progress"; it
  is that the active principal sample root changed. `faq_012.md` therefore
  needed to stop speaking in old prototype / problem-bundle terms as if those
  were still the active principal line.
- The user’s clarification matters materially. Under a looser definition, one
  might say the repo has "implemented the two big lines." Under the stricter
  definition that includes theory completion and semantics-aligned public
  syntax, the correct answer is still no.
- The most accurate single phrase for the current state is
  `repo-local alpha-ready current layer`, not `final public language
  implementation`.

## 8. Open questions

- Problem 1:
  - What is the final public typed source principal?
  - What is the final public theorem/model-check contract?
  - How much of the current conceptual typing spine becomes visible public syntax?
- Problem 2:
  - Does the repo keep high-level relation family as the sole public principal?
  - If not, where does low-level `memory_order` surface appear?
  - What is the final public wording and emitted/public contract?
- Cross-cutting:
  - What counts as completion for final public syntax?
  - Is packaging / installed binary in-scope?
  - What is the next external or application target?

## 9. Suggested next prompt

Read `faq_012.md` and then decide whether the next package should target:

1. final public syntax / parser boundary narrowing
2. Problem 1 public contract narrowing
3. Problem 2 public wording / public contract narrowing
4. packaging / installed-binary target definition
