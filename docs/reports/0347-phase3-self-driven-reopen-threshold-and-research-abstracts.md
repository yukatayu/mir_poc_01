# Report 0347 — phase3 self driven reopen threshold and research abstracts

- Date: 2026-04-08
- Author / agent: Codex
- Scope: Phase 3 closeout checkpoint を前提に、current checkpoint でなお self-driven に reopen すべきかを docs-first で比較し、その結論を `specs/examples/120` と top-level mirror に反映する。同時に、Phase 0 / 1 / 2 / 3 の本質的成果を `docs/research_abstract/` に condensed summary として追加する。
- Decision levels touched:
  - L2: Phase 3 self-driven reopen threshold
  - L2: phase summary / current checkpoint reading の mirror update

## 1. Objective

- Phase 3 current tranche closeout の後に、なお self-driven に parser boundary / first checker cut を reopen すべきかを narrow に比較する。
- current checkpoint では Phase 3 self-driven portion を reserve path に戻す方が自然かどうかを判断する。
- Phase 0 / 1 / 2 / 3 の本質的成果を、非規範の condensed summary として `docs/research_abstract/` に整理する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/examples/29-current-l2-first-parser-subset-inventory.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/119-current-l2-reconnect-freeze-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/reports/0345-current-l2-phase3-closeout-sweep.md`
- `docs/reports/0346-review-phase3-closeout-checkpoint.md`

## 3. Actions taken

1. Phase 3 closeout 後にまだ自走で reopen できる候補を、parser subset refinement / public checker precondition comparison / `E21` / `E22` mirror threshold / reserve path 化の 4 案で比較した。
2. `specs/examples/120-current-l2-phase3-self-driven-reopen-threshold.md` を追加し、current checkpoint では Phase 3 self-driven portion は一旦尽きたと読む current judgment を固定した。
3. `README.md` には `docs/research_abstract/` の directory entry だけを追加し、`Documentation.md` と `specs/00-document-map.md` に research abstract と Phase 3 reserve-path judgment の導線を追加した。
4. `plan/11`、`plan/17`、`progress.md` を更新し、Phase 3 を active mainline ではなく reserve path として読む current snapshot へ揃えた。
5. `docs/research_abstract/` を新設し、Phase 0 / 1 / 2 / 3 の condensed summary を追加した。
6. `plan/90-source-traceability.md` に今回の addendum を追記した。
7. reviewer を 1 回だけ回し、`progress.md` の section title と `plan/90` の provenance wording に出た hygiene finding を同 task 内で補正した。

## 4. Files changed

- Added: `specs/examples/120-current-l2-phase3-self-driven-reopen-threshold.md`
- Added: `docs/research_abstract/README.md`
- Added: `docs/research_abstract/phase0-repository-memory-and-decision-boundary.md`
- Added: `docs/research_abstract/phase1-current-l2-semantics-stabilization.md`
- Added: `docs/research_abstract/phase2-parser-free-poc-and-detached-validation-loop.md`
- Added: `docs/research_abstract/phase3-parser-boundary-and-first-checker-cut.md`
- Added: `docs/reports/0348-review-phase3-reopen-threshold-and-abstracts.md`
- Modified: `README.md`
- Modified: `Documentation.md`
- Modified: `specs/00-document-map.md`
- Modified: `plan/11-roadmap-near-term.md`
- Modified: `plan/17-research-phases-and-autonomy-gates.md`
- Modified: `plan/90-source-traceability.md`
- Modified: `progress.md`
- Modified: `docs/reports/0347-phase3-self-driven-reopen-threshold-and-research-abstracts.md`

## 5. Commands run and exact outputs

- `git status --short --branch`
  - `## main...origin/main`
  - `?? docs/reports/0347-phase3-self-driven-reopen-threshold-and-research-abstracts.md`
- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-08 20:04 JST`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 349 numbered report(s).`
- `git diff --check`
  - no output

`cargo test` は今回は実行していない。理由は、変更が docs / plan / progress / report / abstract のみで code path を触っていないためである。

## 6. Evidence / findings

- `specs/examples/29`、`30`、`73`、`119` を並べて見ると、Phase 3 の current evidence はすでに
  - private staged spike の representative actualization
  - reconnect freeze threshold
  まで揃っており、残る候補は wording refinement か public boundary precondition comparison に寄りやすい。
- したがって current checkpoint では、Phase 3 を「未完了の active line」として扱うより、
  「later pressure が出たときだけ reopen する reserve path」と読む方が自然である。
- `docs/research_abstract/` は規範文書ではないが、Phase ごとの本質的成果を素早く辿る入口として有用である。

## 7. Changes in understanding

- Phase 3 self-driven portion は current checkpoint では一旦尽きたと読める。
- 以後の主線は、
  - Phase 2 maintenance tail
  - Phase 4 side line
  - Phase 5 inventory line
  に移してよい。
- Phase 3 は later pressure が出たときだけ reopen する reserve path として扱うのが、current repo の phase reading に最も整合する。

## 8. Open questions

- Phase 3 reserve path を実際に reopen する pressure は、どの条件で十分とみなすか
- Phase 5 inventory から Phase 3 / Phase 6 へいつ橋を架けるか
- `docs/research_abstract/` をどの粒度で維持するか

## 9. Suggested next prompt

```text
current mainline を Phase 2 maintenance tail / Phase 4 shared-space boundary / Phase 5 small decidable core inventory に移した前提で、
次に promote する subline を source-backed に比較してください。
```
