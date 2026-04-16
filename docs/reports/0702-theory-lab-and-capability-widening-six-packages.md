# Report 0702 — theory lab and capability widening six packages

- Date: 2026-04-16T12:44:42.293348Z
- Author / agent: Codex
- Scope: `tasks.md` 先頭 6 package の closeout
- Decision levels touched: `L2`, `L3`

## 1. Objective

次の 6 package を、execution lane と theory-lab lane を混ぜずに close する。

1. `stable malformed capability second source-backed widening actualization`
2. `first source-visible typed-surface comparison`
3. `first theorem lemma family wording hardening`
4. `model-check carrier to projection bridge note`
5. `order / handoff candidate reduction after falsifier hardening`
6. `modal foundation comparison follow-up`

あわせて、snapshot 文書、traceability、source-sample regression bundle、research abstract、FAQ を current lane に同期する。

## 2. Scope and assumptions

- 規範判断の正本は `specs/` とし、`plan/` は repository memory、`progress.md` と `tasks.md` は薄い snapshot として更新する。
- `atomic_cut` を local finalization の current nucleus から動かさない。
- low-level `memory_order` / `kill_dependency` family を current source surface に昇格させない。
- theorem/model-check typed line は docs-first comparison / bridge note に留め、concrete tool binding や final calculus adoption を決めない。
- `e13/e20` widening は current authored source sample / lowerer / runner / ladder / emitted helper / regression helper の narrow actualization に留め、duplicate cluster と broader malformed-static family は later に残す。

## 3. Documents consulted

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
- `specs/examples/401-current-l2-public-operational-cli-concrete-shell-naming-ready-stable-malformed-capability-second-source-backed-widening-actualization-comparison.md`
- `specs/examples/402-current-l2-stable-malformed-capability-second-source-backed-widening-actualization-ready-minimal-stable-malformed-capability-second-source-backed-widening-threshold.md`
- `specs/examples/413-current-l2-typed-core-attachment-inventory-and-obligation-allocation-refresh.md`
- `specs/examples/414-current-l2-semantic-core-theorem-pilot-planning.md`
- `specs/examples/415-current-l2-model-check-projection-and-property-family-reserve-inventory.md`
- `specs/examples/416-current-l2-order-handoff-falsifier-loop-and-adequacy-corpus-hardening.md`
- `specs/examples/410-current-l2-modal-foundation-comparison.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `faq_004.md`
- `samples/current-l2/README.md`
- `.docs/current-l2-source-sample-authoring-policy.md`

## 4. Actions taken

1. `e13/e20` capability pair を source-authored static-stop pair として actual code へ反映した。
   - `samples/current-l2/` に 2 sample を追加した。
   - `mir-runtime` の lowering / runner / verification ladder / emitted artifact wiring を widen した。
   - regression helper inventory を authored dozen に更新した。
2. theory-lab 側の 5 package を `specs/examples/418...422` として追加した。
   - typed source-visible comparison
   - theorem lemma wording hardening
   - model-check bridge grain note
   - order/handoff candidate reduction
   - modal follow-up
3. snapshot / memory を current lane に同期した。
   - `Documentation.md`
   - `progress.md`
   - `tasks.md`
   - `plan/00` `01` `08` `09` `10` `11` `12` `17` `18` `90`
   - `docs/research_abstract/phase5...`
   - `docs/research_abstract/phase6...`
   - `faq_004.md`
   - `.docs/current-l2-source-sample-authoring-policy.md`
4. reviewer 指摘を反映した。
   - report 本文を補完した。
   - regression helper に emitted-artifact wiring test を追加した。
   - stale `faq_04.md` path を `faq_004.md` に修正した。

## 5. Files changed

### new files

- `docs/reports/0702-theory-lab-and-capability-widening-six-packages.md`
- `samples/current-l2/e13-malformed-capability-strengthening.txt`
- `samples/current-l2/e20-malformed-late-capability-strengthening.txt`
- `specs/examples/417-current-l2-stable-malformed-capability-second-source-backed-widening-actualization.md`
- `specs/examples/418-current-l2-first-source-visible-typed-surface-comparison.md`
- `specs/examples/419-current-l2-first-theorem-lemma-family-wording-hardening.md`
- `specs/examples/420-current-l2-model-check-carrier-to-projection-bridge-note.md`
- `specs/examples/421-current-l2-order-handoff-candidate-reduction-after-falsifier-hardening.md`
- `specs/examples/422-current-l2-modal-foundation-comparison-follow-up.md`

### updated files

- `.docs/current-l2-source-sample-authoring-policy.md`
- `Documentation.md`
- `crates/mir-runtime/src/current_l2.rs`
- `crates/mir-runtime/tests/current_l2_source_lowering.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_emitted_artifact_wiring.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
- `crates/mir-runtime/tests/current_l2_source_sample_verification_ladder.rs`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `faq_004.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/current-l2/README.md`
- `scripts/current_l2_source_sample_regression.py`
- `scripts/tests/test_current_l2_source_sample_regression.py`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `tasks.md`

### updated not needed

- `specs/11-roadmap-and-workstreams.md` 更新不要
- `specs/12-decision-register.md` 更新不要
- `plan/13-heavy-future-workstreams.md` 更新不要
- `AGENTS.md` 更新不要

## 6. Commands run and exact outputs

### resource checks

```text
$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda1        99G   76G   19G  81% /
```

```text
$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       600Mi        71Mi       2.0Mi       288Mi       360Mi
Swap:           19Gi       5.2Gi        14Gi
```

### verification

```text
$ cargo test -p mir-runtime --test current_l2_source_lowering
test result: ok. 12 passed; 0 failed
```

```text
$ cargo test -p mir-runtime --test current_l2_source_sample_runner
test result: ok. 14 passed; 0 failed
```

```text
$ cargo test -p mir-runtime --test current_l2_source_sample_verification_ladder
test result: ok. 12 passed; 0 failed
```

```text
$ cargo test -p mir-runtime --test current_l2_source_sample_emitted_artifact_wiring
test result: ok. 5 passed; 0 failed
```

```text
$ cargo test -p mir-semantics --test current_l2_formal_hook_support
test result: ok. 5 passed; 0 failed
```

```text
$ cargo test -p mir-semantics --test current_l2_proof_notebook_review_unit_support
test result: ok. 4 passed; 0 failed
```

```text
$ cargo test -p mir-semantics --test current_l2_model_check_carrier_support
test result: ok. 4 passed; 0 failed
```

```text
$ python3 scripts/current_l2_source_sample_regression.py inventory
current L2 fixed-subset authored inventory
... authored dozen rows listed ...
```

```text
$ python3 scripts/current_l2_source_sample_regression.py regression --run-label pkg0702 --artifact-root target/current-l2-source-sample-regression-pkg0702
[1/16] runtime lowering test
...
[16/16] static formal hook smoke for e23-malformed-try-fallback-missing-fallback-body
all regression commands passed
```

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 701 numbered report(s).
```

```text
$ git diff --check
[no output]
```

## 7. Evidence / outputs / test results

- authored source sample は decet から dozen に widen された。
  - new rows: `e13-malformed-capability-strengthening`, `e20-malformed-late-capability-strengthening`
- source sample runner accepted set は `e13/e20` を含む current authored dozen に同期した。
- verification ladder は `e13/e20` を `fixture_static_cluster` formal-hook reached row として扱う。
- emitted artifact wiring path は regression helper の blessed bundle に追加した cargo test で cover される。
- theory-lab package close により、
  - typed source-visible comparison
  - theorem wording floor
  - model-check bridge grain
  - order/handoff candidate reduction
  - modal follow-up
  が source-backed docs-first current cut として整理された。

## 8. What changed in understanding

- capability family widening は `e13` singleton ではなく `e13/e20` pair で閉じる方が current authored corpus と regression path に自然である。
- source-visible typed work は new typed block を作るより、existing structural markers を grouped reading する方が checker-adjacent principal source と整合する。
- theorem pilot は lemma statement floor と review/discharge stop line を先に固めるべきであり、`proof_notebook_review_unit` を proof object と読んではならない。
- model-check bridge は row-local carrier と room-protocol line の間に small-cluster semantic projection を置くのが current repo には自然である。
- order/handoff line は falsifier hardening 後に kept / second / derived / retained-later を reduction しておくと、property-language bridge task が進めやすい。
- modal line は `lambda-circle-box` の usefulness を保持しつつ、guarded / MDTT / MTT cluster を stronger family として比較し続けるのが自然である。

## 9. Open questions

- checker attachment から handoff row への migration をどの field family / grouping で書くか
- proof artifact / bridge sketch / discharge line の stop line をどこまで docs-first で縮めるか
- sample-visible theorem/model-check property summary の first wording をどこまで narrow にするか
- order/handoff property language を `pub / obs / wit / final / hb(scope)` relation decomposition にどう接続するか
- modal promotion threshold をどこで triggered とみなすか
- duplicate cluster と broader malformed-static family の execution-lane reopen ordering

## 10. Suggested next prompt

`tasks.md` の current line に従い、次の 6 package を自走で進めてください。優先順は 1) public operational CLI concrete shell actualization、2) shared-space room-profile・host binding bridge-only note、3) checker attachment から handoff row への migration note、4) proof artifact / bridge stop-line refresh、5) sample-visible property summary wording、6) order / handoff property-language bridge note です。snapshot と report、必要なら regression bundle まで同期してください。
