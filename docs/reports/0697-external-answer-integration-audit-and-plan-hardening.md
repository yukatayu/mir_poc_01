# Report 0697 — external answer integration audit and plan hardening

- Date: 2026-04-14T10:03:26+0900
- Author / agent: Codex
- Scope: `sub-agent-pro/answer_001...016` の読解、adopt / reserve / reject-for-now の分類、current repo memory への selective integration
- Decision levels touched: L1-L2

## 1. Objective

external `answer_001...016` を current repo の canonical docs と突き合わせ、current line を壊さずに取り込める guidance だけを `plan/` と FAQ に反映する。

## 2. Scope and assumptions

- `answer_*.md` は external advice であり、規範判断の正本ではない。
- current promoted line と current blocker 読みを無理に動かさない。
- current task map 自体は変わらないため、`tasks.md` は更新不要と判断する。
- `Documentation.md` は current snapshot の top line を変えないため更新不要と判断する。
- `plan/` は更新する。
- `progress.md` は recent log と timestamp を更新する。

## 3. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `faq_003.md`
- `sub-agent-pro/answer_001.md`
- `sub-agent-pro/answer_002.md`
- `sub-agent-pro/answer_003.md`
- `sub-agent-pro/answer_004.md`
- `sub-agent-pro/answer_005.md`
- `sub-agent-pro/answer_006.md`
- `sub-agent-pro/answer_007.md`
- `sub-agent-pro/answer_008.md`
- `sub-agent-pro/answer_009.md`
- `sub-agent-pro/answer_010.md`
- `sub-agent-pro/answer_011.md`
- `sub-agent-pro/answer_012.md`
- `sub-agent-pro/answer_013.md`
- `sub-agent-pro/answer_014.md`
- `sub-agent-pro/answer_015.md`
- `sub-agent-pro/answer_016.md`

## 4. Actions taken

1. canonical docs と `answer_001...016` を再読し、各回答を
   - adopt into current repository memory
   - reserve only
   - reject for now
   に分類した。
2. `plan/09` に、Rust-heavy core + mixed helper workflow、existing public-surface bucket vocabulary と helper migration guidance の対応を追加した。
3. `plan/12` に、semantic-preservation matrix、carrier lifecycle / promotion label drift、shared-space confusion / replay table を reserve hardening risk として追加した。
4. `plan/13` に、typed work / theorem line / public surface staging / async-control / external verifier boundary について、current reserve recommendation と hypothesis を self-contained な形で追記した。
5. `plan/16` に、shared-space docs-first staged sequencing と confusion / replay hardening note を追加した。
6. `faq_003.md` に、typed work の current working hypothesis、public surface sequencing、Rust / Python split の注意書きを追記した。
7. reviewer を使って over-adoption / wording drift を監査し、指摘 7 件を narrow に修正した。
8. `progress.md` に recent log を追加し、current line が unchanged であることを明記した。

## 5. Evidence / outputs / test results

- resource check
  - `df -h .`
    - `/` は `99G` 中 `19G` 空き
  - `free -h`
    - `Mem 960Mi / available 143Mi`
- answered files count
  - `ls sub-agent-pro/answer_*.md | wc -l`
    - `16`
- review evidence
  - initial reviewer findings
    - shared-space staged sequence の later wording drift
    - helper lifecycle label drift
    - typed/model-check wording の over-adoption
    - FAQ public-surface ordering ambiguity
  - narrow re-review
    - `No actionable findings remain`
- document validation
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 696 numbered report(s).`
  - `git diff --check`
    - no output
  - `git status --short`
    - modified:
      - `faq_003.md`
      - `plan/09-helper-stack-and-responsibility-map.md`
      - `plan/12-open-problems-and-risks.md`
      - `plan/13-heavy-future-workstreams.md`
      - `plan/16-shared-space-membership-and-example-boundary.md`
      - `progress.md`
    - untracked:
      - `docs/reports/0697-external-answer-integration-audit-and-plan-hardening.md`

## 6. What changed in understanding

- 16 本の回答は、current mainline を即座に切り替える材料というより、reserve path の順序付けと boundary hardening に強く効く。
- 特に useful だったのは、
  - typed work を final grammar より checker / carrier boundary comparison に寄せて考えること
  - theorem-first / model-check-second reserve orderingを崩さないこと
  - shared-space を exhaustive final catalog ではなく staged docs-first sequence で扱うこと
  - Rust-heavy core と mixed helper workflow を silent promotion 防止の観点から明文化すること
- 一方で、concrete prover / model checker tool、memory-order family の first property、Mirrorea operational artifact family の exact naming は current repo memory にまだ早い。

## 7. Open questions

- typed work の first source-visible cut を、実際にどの comparison package で reopen するか
- theorem-first reserve の次に compare する concrete proof relation の切り方
- shared-space confusion / replay hardening note を compact table に起こす timing
- concrete theorem prover / model checker tool family の選定時期

## 8. Suggested next prompt

`tasks.md` 先頭の current line をそのまま進めつつ、必要なら次段で reserve hardening の小 package（semantic-preservation matrix か confusion/replay compact table）を comparison として切ってください。
