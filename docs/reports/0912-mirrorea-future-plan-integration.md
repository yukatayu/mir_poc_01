# Report 0912 — Mirrorea future plan integration

- Date: 2026-04-27 09:13 JST
- Author / agent: Codex
- Scope: `sub-agent-pro/mirrorea_future_plan_full_handoff_2026-04-24.md` の repo 統合、AGENTS/docs/specs/plan/progress/tasks の同期、validation、conflict 整理。
- Decision levels touched: L1/L2 の整理を含む。`specs/12-decision-register.md` は evidence 不足のため未更新。

## 1. Objective

`sub-agent-pro/mirrorea_future_plan_full_handoff_2026-04-24.md` に書かれた Mirrorea future-axis を、
repo の現況、既存 spec、sample、helper、reporting discipline に照らして点検し、
規範 spec / repository memory / reader-facing summary / task snapshot / evidence report の各層へ
正確に反映する。

この task の completion は **future plan integration complete** であり、
Mirrorea system complete や final public completion を意味しない。

## 2. Scope and assumptions

- handoff は作業指示であり、規範正本ではない。
- 規範判断の正本は `specs/`、長期 repository memory は `plan/`、実行証跡は `docs/reports/`。
- `.docs/` は存在確認と既存 policy 読取を行ったが、この task では update 不要と判断した。
- current runnable floor は clean near-end suite と Sugoroku world vertical slice であり、
  `crates/mirrorea-core` / `crates/mirrorea-control` は placeholder skeleton のまま読む。
- `specs/12-decision-register.md` は、handoff だけでは新規 decision entry を立てる evidence が足りないため、
  この task では更新しない。

## 3. Documents consulted

### handoff files read

- `sub-agent-pro/mirrorea_future_plan_full_handoff_2026-04-24.md`
- `sub-agent-pro/codex_clean_mir_near_end_completion_with_new_samples_2026-04-22.md`
- `sub-agent-pro/codex_sugoroku_runtime_attachment_handoff_2026-04-23.md`

### repository control / snapshot docs

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `.docs/progress-task-axes.md`
- `.docs/continuous-task-policy.md`
- `.docs/current-l2-source-sample-authoring-policy.md`
- `docs/reports/TEMPLATE.md`
- `docs/research_abstract/README.md`

### specs consulted

- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`

### plan consulted

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
- `plan/90-source-traceability.md`
- `plan/91-maintenance-rules.md`

## 4. Repository state summary

- clean near-end current layer は runnable。
- Sugoroku world vertical slice は repo-local logical multi-place emulator として runnable。
- `Place != participant` の読み、membership epoch/incarnation、runtime attach sample は既に sample/helper 側に存在する。
- Mirrorea future-axis で必要とされた `TermSignature` / `LayerSignature` / `MessageEnvelope` /
  `AuthEvidence` / `VisualizationProtocol` / `AttachPoint` などは、live repo にはまだ actual 実装されていない。
- standard I/O を Mir core primitive にしない方向、external world を typed effect boundary で扱う方向、
  auth を transport に潰さない方向は、今回 docs/specs/plan に明示し直した。
- `scripts/avatar_follow_samples.py` は未存在であり、avatar fairy follow slice は future package と読む。

## 5. Actions taken

- handoff と repo 現況を照合し、future-axis で必要な概念を分類した。
- `AGENTS.md` に source hierarchy、handoff 読取 rule、anti-shortcut rule、no fake validation、
  final/public completion の慎重な扱い、old/current sample distinction、typed visualization/telemetry、
  auth/transport 分離、package close ごとの snapshot/report 同期を追記した。
- `README.md` と `Documentation.md` に Mirrorea future-axis の reader-facing summary を追加した。
- `docs/research_abstract/README.md` を更新し、`docs/research_abstract/mirrorea_future_axis_01.md` を新規追加した。
- `specs/00` に handoff の document-map 位置づけを追加した。
- `specs/05` に Place / projection / attach-detach / layered interpretation を future direction として追記した。
- `specs/07` に no-stdio-core / typed external effect adapter / typed visualization-telemetry direction を追記した。
- `specs/10` に Mirrorea future-runtime addendum と open question cluster を追加した。
- `specs/11` に package 1..12 の recommended order と principle addendum を追加した。
- `plan/01`、`plan/11`、`plan/12`、`plan/13`、`plan/16`、`plan/17`、`plan/90` を future-axis queue に合わせて更新した。
- `plan/10` には macro roadmap 文書としての位置づけを保ちつつ、近接 queue は `plan/11` / `tasks.md` を正とする note を追加した。
- `progress.md` と `tasks.md` を package 1 close / package 2 close / package 3 reopen point に合わせて全面更新した。

## 6. Files changed

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `specs/00-document-map.md`
- `specs/05-mirrorea-fabric.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `docs/reports/0912-mirrorea-future-plan-integration.md`

### update status summary

- AGENTS.md updated: yes
- `.docs/` updated: no
- reader-facing docs updated: yes
- specs updated: yes
- plan updated: yes
- progress/tasks updated: yes
- `specs/12-decision-register.md` updated: no

## 7. Commands run and exact outputs

Resource check before heavy work:

```text
$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/nvme0n1p2  916G  596G  274G  69% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:            15Gi       2.1Gi        11Gi       267Mi       2.4Gi        13Gi
Swap:          4.0Gi       1.5Mi       4.0Gi
```

Discord baseline:

```text
$ python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
Task baseline recorded.
```

Validation run:

```text
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 910 numbered report(s).
```

```text
$ python3 scripts/current_l2_guided_samples.py smoke-all --format json
exit 0
```

```text
$ python3 scripts/current_l2_guided_samples.py closeout --format json
exit 0
```

```text
$ python3 scripts/sugoroku_world_samples.py closeout --format json
exit 0
```

```text
$ cargo test -p mir-ast
exit 0
```

```text
$ cargo test -p mir-runtime
exit 0
```

```text
$ cargo test -p mir-semantics
exit 0
```

Existence check for future helper:

```text
$ rg --files scripts | rg 'avatar|follow'
exit 1
```

Timestamp anchor:

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-27 09:13 JST
```

```text
$ git diff --check
exit 0
```

## 8. Evidence / findings

### validation results

- docs validation passed
- clean near-end smoke passed
- clean near-end closeout passed
- Sugoroku world closeout passed
- `cargo test -p mir-ast` passed
- `cargo test -p mir-runtime` passed
- `cargo test -p mir-semantics` passed
- `mir-semantics` test run included actual Lean execution test pass when toolchain was available

### handoff concepts now mirrored into repo docs/memory

- Mirrorea project axis
- Place distinction
- no standard I/O as Mir core primitive
- typed external effect boundary
- auth / authorization / membership / capability / witness separation
- transport insertion seam as future package, not baked-in final design
- visualization / telemetry as typed, labeled, authority-aware effect
- projection / placement as system-wide source to place-specific program property
- hot-plug Patch / AttachPoint as future docs-first package
- Sugoroku hardening and avatar fairy follow as ordered vertical slices
- reporting discipline / anti-shortcut policy / no fake validation rule

### skipped validations and reasons

- `python3 scripts/avatar_follow_samples.py closeout`
  - skipped
  - reason: helper script does not exist yet; package 8 is still future work
- `python3 scripts/clean_near_end_samples.py smoke-all`
  - not used
  - reason: current repo has `scripts/current_l2_guided_samples.py smoke-all --format json` as the active suite-wide smoke entrypoint

### conflict / stale-reference handling

- hard conflict requiring manual user resolution was not found in consulted normative files
- historical or helper-local wording that can mislead was normalized as follows:
  - `world` is documented as current sample host/server-side sugar, not Mir core primitive
  - `plan/10` is explicitly marked as macro repository memory, while near-term queue authority is moved to `plan/11` and `tasks.md`
- `specs/12-decision-register.md` was intentionally left unchanged because the handoff alone is not sufficient evidence for a new settled decision entry

## 9. What changed in understanding

- Mirrorea future-axis is now better read as a **typed layering and projection problem** than as a thin extension of the current Sugoroku sample.
- Sugoroku remains the current representative shared-space sample, but its `world` wording must not silently promote a builtin world primitive.
- The next meaningful implementation floor is not “real networking” but shared carriers and laws:
  `TermSignature` -> `LayerSignature` -> `MessageEnvelope/AuthEvidence` -> `VisualizationProtocol`.
- visualization and telemetry cannot remain helper-local debug leaks if the repo wants place-spanning evidence with redaction and authority boundaries.
- hot-plug is not a detached afterthought; it is tied to projection, compatibility, activation cut, and migration wording from the start.

## 10. Open questions

### remaining mixed gates

- exact `TermSignature` carrier granularity
- `LayerSignature` composition law and ordering surface
- `MessageEnvelope` / `AuthEvidence` / transport seam minimum schema
- visualization / telemetry redaction and authority law
- projection / placement validity reporting shape
- `Patch Req Prov Δ` / `AttachPoint` / activation cut / migration contract minimum surface

### remaining true user-spec gates

- final public host / packaging target
- whether repo-local helper floor should become installed binary / host-facing contract
- broader shared-space catalog or product target after minimal subset
- final public auth / visualization retention / ecosystem target

## 11. Risks

- goal drift: local convenience work could collapse the project axis into a narrow sample-only optimization
- auth collapse: authentication could be accidentally baked into transport or queue plumbing
- visualization leak: debug output could drift into untyped information leak instead of labeled/redacted evidence
- place conflation: participant / principal / place / world sugar could be re-merged by future edits
- premature freeze: final grammar / public APIs / production binding could be falsely implied by helper-local work

## 12. Suggested next prompt

Implement package 3 `TermSignature registry` as a docs-first plus helper-local carrier task:
define the signature fields, wire one Sugoroku sample and one clean near-end sample to emit signature-oriented summaries,
add a `--debug signatures` style output, run the existing smoke/closeout commands, and update `progress.md`, `tasks.md`, and a new report.
