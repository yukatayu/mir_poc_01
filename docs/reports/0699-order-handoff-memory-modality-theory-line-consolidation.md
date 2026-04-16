# Report 0699 — order / handoff / memory / modality theory line consolidation

## Objective

order / memory-model / authority-handoff / syntax / modality / verifier-boundary の理論線を、
current repo の source-backed boundaryを壊さずに整理し、
`specs/` / `plan/` / `docs/reports/` / snapshot docs へ反映する。

## Scope and assumptions

- この task は mainline actualization task ではなく、docs-first research integration task として扱う。
- 規範判断の正本は `specs/`、repository memory は `plan/`、時系列証跡は `docs/reports/` に残す。
- `OPEN` / `FUTURE` / `COMPARISON` を fact に昇格させない。
- `atomic_cut` を global consistent cut / durable commit / low-level memory-order family と同一視しない。
- low-level memory-order family は reference family として保持し、current language core へ premature に入れない。
- final parser grammar、final public API、shared-space final catalog、backend success criteria、upper-layer app 仕様は固定しない。

## Documents consulted

### repo docs

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/03-decision-strengths-and-boundaries.md`
- `plan/04-core-semantics-current-l2.md`
- `plan/06-surface-notation-status.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`
- `docs/reports/0358-async-control-memory-boundary-inventory.md`
- relevant `specs/examples/` around shared-space baseline, 4-way verifier split, low-level memory-order threshold, higher-level async-control threshold, and current Phase 6 boundary docs

### primary literature used

- Leslie Lamport, *Time, Clocks, and the Ordering of Events in a Distributed System* (1978)
- K. Mani Chandy, Leslie Lamport, *Distributed Snapshots: Determining Global States of Distributed Systems* (1985)
- Hans-J. Boehm, Sarita V. Adve, *Foundations of the C++ Concurrency Memory Model* (2008)
- HSA Foundation, *HSA Platform System Architecture Specification Version 1.2* (2018)
- JF Bastien, Paul E. McKenney, *P0750R1: Consume* (2018)
- Hans Boehm, *P3475R2: Defang and deprecate memory_order::consume* (2025)
- Alan Jeffrey, James Riely, *On Thin Air Reads: Towards an Event Structures Model of Relaxed Memory* (2019 journal version)
- Maurice P. Herlihy, Jeannette M. Wing, *Linearizability: A Correctness Condition for Concurrent Objects* (1990)
- Rowan Davies, Frank Pfenning, *A Modal Analysis of Staged Computation* (2001)
- Yosihiro Yuse, Atsushi Igarashi, *A Modal Type System for Multi-Level Generating Extensions with Persistent Code* (2006)
- Ranald Clouston et al., *The Guarded Lambda-Calculus: Programming and Reasoning with Guarded Recursion for Coinductive Types* (2016)
- Lars Birkedal et al., *Modal Dependent Type Theory and Dependent Right Adjoints* (2020 print issue / 2019 online)
- Daniel Gratzer et al., *Multimodal Dependent Type Theory* (2021 journal version / 2020 conference precursor)

## Actions taken

1. `specs/10-open-questions.md` に、cut family decomposition、order / visibility / witness decomposition、thread / node parity、syntax-semantics coupling principle、modal foundation comparison、order/handoff verifier boundary、theory-lab operating disciplineの cluster を追加した。
2. `specs/11-roadmap-and-workstreams.md` を更新し、Workstream A/E/G をまたぐ docs-first theory package line を mainline actualization と separable に位置づけた。
3. `specs/12-decision-register.md` を点検したが、今回の材料は settled decision row へ昇格させるにはまだ早いと判断し、**更新しなかった**。
4. `specs/examples/405...412` を追加し、order / cut family、local finalization vs global snapshot、order / visibility / witness family、thread/node parity、syntax stimuli comparison、modal foundation comparison、adequacy corpus / verification-boundary matrix、theory-lab operating modelを docs-first 補助正本として切り出した。
5. `plan/06` `11` `12` `13` `16` `17` `18` を更新し、syntax honesty axis、near-term sequence、risk register、heavy workstreams、shared-space order/handoff minimal scenario、theory-lab autonomy line、detail-side research program を反映した。
6. `plan/00` `01` `10` `90` を更新し、index / current snapshot / overall roadmap / traceability の stale wording を同期した。
7. `Documentation.md`、`progress.md`、`tasks.md` を更新し、current mainline は維持したまま theory-lab line を薄い snapshot に mirror した。
8. `AGENTS.md` と `.docs/` は今回の task では運用差分が無かったため、**更新しなかった**。

### files changed

- `Documentation.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/06-surface-notation-status.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `tasks.md`
- `specs/examples/405-current-l2-order-cut-family-comparison.md`
- `specs/examples/406-current-l2-local-finalization-vs-global-snapshot-comparison.md`
- `specs/examples/407-current-l2-order-visibility-witness-family-comparison.md`
- `specs/examples/408-current-l2-thread-node-parity-and-lowering-boundary-note.md`
- `specs/examples/409-current-l2-order-handoff-syntax-stimuli-comparison.md`
- `specs/examples/410-current-l2-modal-foundation-comparison.md`
- `specs/examples/411-current-l2-order-handoff-adequacy-corpus-and-verification-boundary-matrix.md`
- `specs/examples/412-current-l2-theory-lab-operating-model-and-research-package-template.md`
- `docs/reports/0699-order-handoff-memory-modality-theory-line-consolidation.md`

### files inspected but left unchanged

- `specs/12-decision-register.md`
  - existing D-011〜D-018 and related current rows already cover `atomic_cut` local nucleus / later `barrier` / later `durable_cut` boundary strongly enough.
  - thread/node parity, modal foundation choice, snapshot-cut vocabulary, theory-lab roles are still comparison/open material.
- `AGENTS.md`
  - current repo process rules already cover report discipline / docs-first maintenance / subagent waiting policy.
- `.docs/*`
  - no policy mismatch specific to this task emerged.

## Evidence / outputs / test results

### resource checks

```text
$ df -h .
/dev/vda2        99G   76G   19G  81% /home

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       595Mi       161Mi       0.0Ki       361Mi       365Mi
Swap:           19Gi       680Mi        19Gi
```

### validation

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-16 18:06 JST

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 697 numbered report(s).

$ git diff --check
[no output]
```

注記:
- `validate_docs.py` は report 追加前に実行した。report 追加後は numbered report count が 1 増える想定である。
- reviewer による diff review を別途実施する。

### settled / current / OPEN judgment

- settled
  - `atomic_cut` は local finalizing cut である。
  - `barrier` と `durable_cut` は later family に残る。
  - 4-way verifier split は current source-backed floor である。
  - provider placement と witness / fairness requirement は別軸に保つ。
  - companion notation は semantic honesty を優先し、final grammar は OPEN である。
- current working line
  - thread と node は same causal language の候補を共有できるが、lowering / transport / evidence / failure / durability policy の差を残す。
  - `lambda-circle-box` は partial basis candidate、guarded / MDTT / MTT は stronger candidate として比較する。
  - theory-lab line は mainline actualization と separable に運用する。
- OPEN / comparison
  - `snapshot_cut` / `consistent_cut` naming
  - final order / witness relation vocabulary
  - final modal foundation
  - final parser grammar
  - final public verifier contract
  - concrete theorem / model-check tool binding

## What changed in understanding

- current repo にはすでに `atomic_cut` local nucleus、4-way verifier split、shared-space authority/provider/witness axis、syntax honesty floor があるため、今回の理論線は「ゼロからの提案」ではなく、**source-backed floor の上に comparison family を足す仕事** として切るのが自然だった。
- low-level memory-order family を捨てる必要はなく、reference family として保持しつつ source-level direct import を避ける line が最も整合的だった。
- `lambda-circle-box` / guarded / MDTT / MTT line は、type/proof/model-check line と別ではなく、same theory-lab package chain の modal foundation comparison として扱う方が repo memory と相性がよい。
- mainline actualization と theory-lab line を同一 package に混ぜるより、accepted comparison / adequacy corpus / operating modelだけを ratchet で昇格させる方が docs drift を減らせる。

## Open questions

1. `snapshot_cut` / `consistent_cut` family を future にどの vocabulary 名で comparison し続けるか。
2. `po / dep / pub / obs / wit / final / hb(scope)` の relation 名を repo working vocabulary にどこまで昇格させるか。
3. order/handoff adequacy corpus の falsifier package で first negative benchmark を何に置くか。
4. `lambda-circle-box` partial basis と stronger modal candidate の gap を typed attachment line とどう接続するか。
5. thread/node parity wording を `specs/` settled line に上げる threshold を何に置くか。

## Suggested next prompt

`plan/18` Track A / B / C を受けて、typed-core attachment inventory、semantic-core theorem pilot planning、model-check projection / property-family reserve inventory を 3 package まとめて docs-first に進めてください。order/handoff theory-lab line は今回追加した adequacy corpus / matrix を falsifier-first に 1 package 進め、negative benchmark と kill criteria だけを追加してください。
