# Report 0908 — research abstract hands on guides

- Date: 2026-04-23T02:40:30.787767Z
- Author / agent: Codex
- Scope: `docs/research_abstract/` beginner hands-on guides for clean near-end typing, order model, model checking, modal, and Lean material.
- Decision levels touched: no new normative decision. `specs/00-document-map.md` was updated only to index the new non-normative hands-on documents.

## 1. Objective

Add丁寧な日本語の hands-on documents under `docs/research_abstract/` for:

- typing / finite-index static checks
- order-handoff / memory-order reinterpretation
- model-checking second-line mutex examples
- modal / stage-stable-later bridge
- Lean foundation proof material

The documents should be readable by beginners with light programming experience, should show runnable commands and representative outputs, and should avoid implying that final public parser / checker / verifier APIs are complete.

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/04-mir-core.md`
- `docs/research_abstract/README.md`
- `samples/clean-near-end/00_index_theories.mir`
- `samples/clean-near-end/typing/*.mir`
- `samples/clean-near-end/order-handoff/*.mir`
- `samples/clean-near-end/model-check/*.mir`
- `samples/clean-near-end/modal/*.mir`
- `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
- `scripts/clean_near_end_samples.py`
- `scripts/validate_docs.py`
- `.agents/skills/discord-report/SKILL.md`
- superpower skill docs for planning / review / verification workflow

Missing inputs: none for this docs package.

## 3. Actions taken

- Created five new beginner-oriented hands-on documents:
  - `docs/research_abstract/hands_on_typing.md`
  - `docs/research_abstract/hands_on_order_model.md`
  - `docs/research_abstract/hands_on_model_checking.md`
  - `docs/research_abstract/hands_on_modal.md`
  - `docs/research_abstract/hands_on_lean.md`
- Updated `docs/research_abstract/README.md` with a hands-on index.
- Updated `Documentation.md` with links to the new hands-on entry points.
- Updated `specs/00-document-map.md` to record that the hands-on documents are non-normative beginner guides.
- Ran a read-only reviewer sub-agent over the new documents.
- Applied reviewer fixes:
  - aligned relaxed Peterson counterexample output with active runner output;
  - added `Region.Session <= Region.Step` to the capture escape discussion;
  - corrected `cost <=` wording to upper-bound wording and added `writes: 0` / `require remote_calls: 0`;
  - added `checked_under: "interleaving_sc"` and `counterexample_shape` to broken mutex output;
  - softened “built-in vocabulary” wording so it does not sound like final public grammar;
  - replaced ambiguous “source principal” wording in beginner-facing text;
  - clarified that `current_l2_lean_sample_sync.py` is a maintenance command.
- `progress.md 更新不要`: implementation status and roadmap state did not change; this package added educational documentation only.
- `tasks.md 更新不要`: current task map did not change; no blocker or self-driven package status changed.
- `plan/ 更新不要`: no roadmap, semantics, helper-stack, or source-trace decision changed.

## 4. Files changed

New files:

- `docs/research_abstract/hands_on_typing.md`
- `docs/research_abstract/hands_on_order_model.md`
- `docs/research_abstract/hands_on_model_checking.md`
- `docs/research_abstract/hands_on_modal.md`
- `docs/research_abstract/hands_on_lean.md`
- `docs/reports/0908-research-abstract-hands-on-guides.md`

Modified files:

- `docs/research_abstract/README.md`
- `Documentation.md`
- `specs/00-document-map.md`

## 5. Commands run and exact outputs

Resource check before long docs task:

```text
$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   85G  9.3G  91% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       464Mi        72Mi        96Ki       580Mi       495Mi
Swap:           19Gi       1.2Gi        18Gi
```

Discord baseline:

```text
$ python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
Task baseline recorded.
```

Report creation:

```text
$ python3 scripts/new_report.py --slug research-abstract-hands-on-guides
docs/reports/0908-research-abstract-hands-on-guides.md
```

Docs validation:

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 906 numbered report(s).
```

Clean near-end sample list:

```text
$ python3 scripts/clean_near_end_samples.py list
16 active samples listed across typing, order-handoff, model-check, and modal.
```

Typing family:

```text
$ python3 scripts/clean_near_end_samples.py run typing --format json
01_authorized_declassification: static_verdict=valid, terminal_outcome=success
02_unauthorized_declassification_rejected: static_verdict=malformed, reason_family=authority_preorder_constraint_failed
03_label_flow_rejected: static_verdict=malformed, reason_family=label_flow_constraint_failed
04_capture_escape_rejected: static_verdict=malformed, reason_family=capture_escape, constraints_failed includes "{EphemeralToken} <= {RoomHistory}" and "Region.Session <= Region.Step"
05_cost_bound_rejected: static_verdict=malformed, reason_family=cost_bound_exceeded, constraints_failed includes "remote_calls 1 <= 0"
```

Order-handoff family:

```text
$ python3 scripts/clean_near_end_samples.py run order-handoff --format json
01_authorized_roll_publish_handoff: static_verdict=valid, terminal_outcome=success, relations include scoped_happens_before roll -> handoff
02_missing_witness_rejected: static_verdict=malformed, reason_family=missing_handoff_witness
03_handoff_before_publication_rejected: static_verdict=malformed, reason_family=handoff_before_publication
04_stage_block_authorized_handoff: static_verdict=valid, terminal_outcome=success
05_delegated_rng_service: static_verdict=valid, terminal_outcome=success
06_auditable_authority_witness: static_verdict=valid, terminal_outcome=success
```

Model-check family:

```text
$ python3 scripts/clean_near_end_samples.py run model-check --format json
01_peterson_sc_pass: model_check_result=pass, checked_under=sequential_consistency
02_peterson_relaxed_counterexample: model_check_result=counterexample, checked_under=relaxed_without_publication_observation_edges
03_broken_mutex_counterexample: model_check_result=counterexample, checked_under=interleaving_sc
```

Modal family:

```text
$ python3 scripts/clean_near_end_samples.py run modal --format json
01_stage_stable_later_minimal: static_verdict=valid, mode_constraints include "config : stable" and "draw available at later stage"
02_published_witnessed_mode_bridge: static_verdict=valid, mode_constraints include "value draw @ published(room)" and "bridge produces witnessed(draw_pub)"
```

Matrix:

```text
$ python3 scripts/clean_near_end_samples.py matrix --format json
{
  "total_samples": 16,
  "families": {
    "modal": 2,
    "model-check": 3,
    "order-handoff": 6,
    "typing": 5
  },
  "model_check_pass": ["01_peterson_sc_pass"],
  "model_check_counterexample": ["02_peterson_relaxed_counterexample", "03_broken_mutex_counterexample"]
}
```

Closeout:

```text
$ python3 scripts/clean_near_end_samples.py closeout --format json
active_sample_root: samples/clean-near-end
archive_sample_root: samples/old/2026-04-22-pre-clean-near-end
built_in_vocabulary includes module, index, policy, principal, resource, effect, place, option, chain, fallback, lineage, perform, via, require, ensure, atomic_cut, transition, stage, publish, observe, handoff, witness, model, property
user_defined_vocabulary includes SecurityLabel, FingerprintAuthority, CaptureScope, Region, CostBudget, FingerprintReleasePolicy, Public, UserSecret, KeyMaterial, Observer, Holder, Releaser, Admin, RoomHistory, EphemeralToken, SecretKeyStore, Step, Turn, Session, Alice, Bob, Carol, draw_pub, dice_owner
```

Lean foundation:

```text
$ source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
<no stdout; exit 0>
```

Whitespace check:

```text
$ git diff --check
<no stdout; exit 0>
```

Sub-agent review:

```text
reviewer sub-agent completed read-only review.
Findings were applied for active output alignment, beginner wording, and maintenance-command caution.
```

## 6. Evidence / findings

- New hands-on docs total 1632 lines:
  - typing: 529 lines
  - order_model: 301 lines
  - model_checking: 301 lines
  - modal: 225 lines
  - lean: 276 lines
- Validation confirmed active family outputs match the examples described in the hands-on docs after reviewer fixes.
- Lean foundation file checks with exit 0.
- `specs/00-document-map.md` update is indexing only; it does not introduce a new normative language decision.

## 7. Changes in understanding

- The beginner-facing documents need to avoid using `principal` in the English sense “central concept” because `principal` is also a Mir sample keyword for an authority-bearing actor.
- “built-in vocabulary” needs qualification. In these docs it means current helper-recognized vocabulary, not final public reserved words.
- `current_l2_lean_sample_sync.py` should be presented as a maintenance/sync command, not as a read-only beginner check.
- The active relaxed Peterson counterexample output summarizes the visibility failure path before critical entry; the docs should not invent extra output rows.

## 8. Open questions

- None introduced by this documentation package.
- Existing deferred items remain deferred: final public parser grammar, final public checker/runtime/verifier API, production theorem/model-check binding, and final public witness/provider/artifact contract.

## 9. Suggested next prompt

「`docs/research_abstract` の hands-on を、`faq_012.md` と existing `_detail.md` から相互リンクし、初心者向け reading path と expert evidence path を 1 本の索引に統合してください。」
