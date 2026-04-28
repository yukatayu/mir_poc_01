# 0996 post-P21 final public hot-plug ABI docs-first audit

## Objective

`0995` の次に来る docs-first package
`post-P21 final public hot-plug ABI family`
について、
current snapshot / roadmap / repository memory の監査を行い、
最小 file set、stale wording、docs-first stop line、
および true user-spec commitment 前に残る self-driven package の有無を整理する。

## Scope and assumptions

- 規範判断の正本は `specs/` とする。
- `plan/` は repository memory、`progress.md` / `tasks.md` / `samples_progress.md` は snapshot として扱う。
- user 指示どおり、plan/docs/spec snapshot files を中心に監査し、unrelated code changes は scope 外とする。
- worktree には audit 前から Rust code の未コミット変更があるため、本 task では触らない。
- current scope は audit + report のみであり、public ABI freeze 自体は行わない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/05-mirrorea-fabric.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/27-public-api-parser-gate-roadmap.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- `plan/35-post-p20-hotplug-next-package-inventory.md`
- `plan/36-post-p21-rollback-durable-migration-family.md`
- `plan/37-post-p21-distributed-activation-ordering-family.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/reports/0995-post-p21-distributed-activation-ordering-docs-first-hardening.md`

## Actions taken

1. repository 指定順序に従って front-door docs、progress snapshot、core specs、relevant `plan/` memory を読んだ。
2. `P18` / `U1` / `R7` / `P21` / post-`P21` first-second recommendation の current wording を line-by-line で照合した。
3. remaining third recommendation に対して、contradiction、missing bridge statement、minimum sync surface を抽出した。
4. audit-only task として report のみ追加し、既存 snapshot / plan / spec は未変更とした。

### Files changed

- `docs/reports/0996-post-p21-final-public-hotplug-abi-audit.md`
- `plan/` 更新不要
- `progress.md` 更新不要
- `tasks.md` 更新不要
- `samples_progress.md` 更新不要

## Evidence / outputs / test results

### Commands run

- `rg -n "0995|hot-plug ABI|hotplug ABI|final public hot-plug ABI|post-P21" -S README.md Documentation.md progress.md tasks.md samples_progress.md specs plan docs`
- `nl -ba README.md`
- `nl -ba Documentation.md`
- `nl -ba progress.md`
- `nl -ba tasks.md`
- `nl -ba samples_progress.md`
- `nl -ba specs/00-document-map.md`
- `nl -ba specs/01-charter-and-decision-levels.md`
- `nl -ba specs/02-system-overview.md`
- `nl -ba specs/03-layer-model.md`
- `nl -ba specs/05-mirrorea-fabric.md`
- `nl -ba specs/09-invariants-and-constraints.md`
- `nl -ba specs/11-roadmap-and-workstreams.md`
- `nl -ba plan/00-index.md`
- `nl -ba plan/17-research-phases-and-autonomy-gates.md`
- `nl -ba plan/27-public-api-parser-gate-roadmap.md`
- `nl -ba plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- `nl -ba plan/35-post-p20-hotplug-next-package-inventory.md`
- `nl -ba plan/36-post-p21-rollback-durable-migration-family.md`
- `nl -ba plan/37-post-p21-distributed-activation-ordering-family.md`
- `nl -ba plan/90-source-traceability.md`
- `nl -ba docs/research_abstract/README.md`
- `nl -ba docs/hands_on/current_phase_closeout_01.md`
- `nl -ba docs/reports/0995-post-p21-distributed-activation-ordering-docs-first-hardening.md`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`

### Outputs

- audit で確認した key stale/contradictory wording:
  - `plan/28` は `hot-plug は helper-local package-manager preview inventory まで` と書いており、`P21` 後の runtime-private engine-state actualization を反映していない。
  - `tasks.md` current status と `docs/hands_on/current_phase_closeout_01.md` next queue は remaining third recommendation を `final public ABI` と略記しており、hot-plug 以外の public ABI と混線しうる。
  - `plan/90` traceability table は `plan/36` / `plan/37` row が table 外へ落ちており、current traceability snapshot として壊れている。
- recommended new-file pattern:
  - `plan/38-post-p21-final-public-hotplug-abi-family.md`
  - `docs/research_abstract/post_p21_final_public_hotplug_abi_family_01.md`
  - `docs/hands_on/post_p21_final_public_hotplug_abi_family_01.md`
  - `docs/reports/0997-post-p21-final-public-hotplug-abi-docs-first-hardening.md`

### Verification

- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- current queue には、true user-spec commitment の前に **ちょうど 1 つ** remaining self-driven mixed-gate package が残っている。
  それが post-`P21` final public hot-plug ABI family の docs-first hardening である。
- この package の core job は public ABI を freeze することではなく、
  `plan/27` mixed gate と `plan/28` true user-spec hold の間に
  hot-plug-specific freeze prerequisite を bridge として明文化することにある。
- `plan/28` の hot-plug wording と `plan/90` traceability table は、`0995` 後の current line に対して既に stale / malformed であり、
  third recommendation hardening と同じ task で直した方がよい。

## Open questions

- third recommendation も `plan/36` / `plan/37` と同じ対称性を保つために dedicated `plan/38` + companion docs を必須にするか。
- docs-first close の definition を
  `freeze prerequisite fixed`
  までで止めるか、
  それとも public candidate naming inventory まで `plan/38` に載せるか。
- `progress.md` / `README.md` / `Documentation.md` で、
  third recommendation close 後は actual user-spec commitment しか残らないことを
  どこまで explicit に書くか。

## Suggested next prompt

`0995` の次として post-`P21` final public hot-plug ABI family を docs-first に harden してください。`plan/38-post-p21-final-public-hotplug-abi-family.md`、`docs/research_abstract/post_p21_final_public_hotplug_abi_family_01.md`、`docs/hands_on/post_p21_final_public_hotplug_abi_family_01.md` を追加し、`plan/00` / `plan/17` / `plan/27` / `plan/28` / `plan/35` / `plan/90`、`specs/11`、`progress.md`、`tasks.md`、`samples_progress.md`、`README.md`、`Documentation.md`、`docs/research_abstract/README.md`、`docs/hands_on/current_phase_closeout_01.md` を同期してください。helper-local anchor naming、runtime-private state naming、`AttachPoint` / `Patch` packaging identity を public ABI に昇格させず、stop line は freeze prerequisite 固定までに留めてください。
