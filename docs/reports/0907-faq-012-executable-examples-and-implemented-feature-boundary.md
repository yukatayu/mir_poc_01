# Report 0907 — faq 012 executable examples and implemented feature boundary

- Date: 2026-04-23 09:14 JST
- Author / agent: Codex
- Scope: `faq_012.md` に executable code example と implementation / checking boundary を追記する
- Decision levels touched: 規範意味論の変更なし。FAQ explanation の補強のみ。

## 1. Objective

`faq_012.md` が単なる status summary に留まらず、

- 実際に実行できる clean near-end sample code
- その sample に対する actual verdict
- どの言語機能が current layer で実装済みか
- どこまで static / model-check / Lean で検査できるか
- どこから先が未完か

を正確に説明できるようにする。

## 2. Inputs consulted

- `faq_012.md`
- `docs/reports/0905-faq-012-alpha-status-and-two-big-problems.md`
- `samples/clean-near-end/typing/01_authorized_declassification.mir`
- `samples/clean-near-end/typing/04_capture_escape_rejected.mir`
- `samples/clean-near-end/order-handoff/03_handoff_before_publication_rejected.mir`
- `samples/clean-near-end/model-check/02_peterson_relaxed_counterexample.mir`
- `samples/clean-near-end/modal/01_stage_stable_later_minimal.mir`
- `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
- fresh command outputs from this task

## 3. Actions taken

1. Re-read the current `faq_012.md` structure and identified the right insertion point.
2. Pulled fresh sample-level outputs for representative examples across:
   - typing success
   - typing rejection
   - order/handoff rejection
   - model-check counterexample
   - modal success
3. Ran one Lean foundation file directly to confirm the proof fragment still succeeds.
4. Added to `faq_012.md`:
   - executable code examples
   - exact command forms
   - exact verdict highlights
   - built-in vs user-defined boundary
   - feature-by-feature implementation / checking matrix
5. Ran docs validation after the FAQ update.

## 4. Files changed

- Updated:
  - `faq_012.md`
- Added:
  - `docs/reports/0907-faq-012-executable-examples-and-implemented-feature-boundary.md`

`progress.md` 更新不要
`tasks.md` 更新不要
`plan/` 更新不要

Reason:
this task refined explanation quality only. It did not change repo status,
sample set, or roadmap state.

## 5. Commands run and exact outputs

### 1. sample-level fresh checks

```bash
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 01_authorized_declassification --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 04_capture_escape_rejected --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 03_handoff_before_publication_rejected --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 02_peterson_relaxed_counterexample --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 01_stage_stable_later_minimal --format json
```

Key exact results:

- `01_authorized_declassification`
  - `static_verdict = "valid"`
  - `terminal_outcome = "success"`
- `04_capture_escape_rejected`
  - `static_verdict = "malformed"`
  - `reason_family = "capture_escape"`
- `03_handoff_before_publication_rejected`
  - `static_verdict = "malformed"`
  - `reason_family = "handoff_before_publication"`
- `02_peterson_relaxed_counterexample`
  - `model_check_result = "counterexample"`
  - `checked_under = "relaxed_without_publication_observation_edges"`
- `01_stage_stable_later_minimal`
  - `static_verdict = "valid"`
  - `mode_constraints` include:
    - `config : stable`
    - `draw available at later stage`

### 2. Lean foundation direct check

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
```

Exact output:

- exit code `0`
- stdout empty
- stderr empty

### 3. docs validation

```bash
python3 scripts/validate_docs.py
```

Exact output:

```text
Documentation scaffold looks complete.
Found 905 numbered report(s).
```

### 4. timestamps / report setup

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
python3 scripts/new_report.py --slug faq-012-executable-examples-and-implemented-feature-boundary
date '+%Y-%m-%d %H:%M %Z'
```

Exact outputs:

- `Task baseline recorded.`
- `/home/yukatayu/dev/mir_poc_01/docs/reports/0907-faq-012-executable-examples-and-implemented-feature-boundary.md`
- `2026-04-23 09:14 JST`

## 6. Evidence / findings

- `faq_012.md` now contains actual executable `.mir` examples instead of abstract capability claims only.
- The FAQ now distinguishes:
  - static finite-index checks
  - order/handoff static checks
  - model-check second-line counterexample search
  - modal constraint checks
  - Lean proof-fragment checks
- The FAQ now explicitly states that "implemented" means
  `repo-local alpha-ready current layer actually supported by helper + active sample suite + Lean foundation`,
  not final public language completion.

## 7. Changes in understanding

- The most useful addition for `faq_012.md` was not more roadmap prose, but concrete examples that make the boundary visible.
- In particular, the feature matrix forces a cleaner distinction between:
  - implemented current layer
  - actually checked current obligations
  - deferred final public surface / contract / calculus work

## 8. Open questions

- Should `faq_012.md` eventually link directly to the corresponding `_detail` documents for each example family?
- Do we want a parallel FAQ section for built-in vocabulary versus user-defined vocabulary with a fuller list, or is the current compact list enough?
- When parser/public API work is reopened, should FAQ 12 gain a second implementation matrix focused only on public-surface completion?

## 9. Suggested next prompt

- `faq_012.md を踏まえて、final public parser / checker / verifier API residual を current code anchor 付きで整理してください。`
- `Problem 2 の public wording residual だけを切り出して、high-level relation family を principal に保つ設計で docs-first に詰めてください。`
