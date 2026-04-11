# Report 0601 — phase5 public checker shared output contract package

- Date: 2026-04-11 19:31 JST
- Author / agent: Codex
- Scope: Phase 5 checker-side current promoted line continuation through shared output contract package
- Decision levels touched: current L2 docs-first boundary only (`specs/` normative additions at L2; no L0/L1 change)

## 1. Objective

- Continue the self-driven Phase 5 checker-side line after the public checker command surface package.
- Determine what the first docs-only shared output contract cut should be, and what its minimum shape should be.
- Keep `tasks.md` / `progress.md` / `plan/` / research abstract aligned so the completed package disappears from the top promoted line.

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/277-current-l2-minimal-public-checker-api-ready-public-checker-entry-criteria-comparison.md`
- `specs/examples/278-current-l2-public-checker-entry-criteria-ready-minimal-public-checker-entry-criteria-threshold.md`
- `specs/examples/279-current-l2-minimal-public-checker-entry-criteria-ready-public-checker-command-surface-comparison.md`
- `specs/examples/280-current-l2-public-checker-command-surface-ready-minimal-public-checker-command-surface-threshold.md`
- `specs/examples/112-current-l2-phase3-resume-side-selection.md`
- `specs/examples/113-current-l2-first-checker-reconnect-family-selection.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0174-same-lineage-first-checker-spike.md`
- `docs/reports/0176-missing-option-second-checker-spike.md`
- `docs/reports/0178-capability-floor-third-checker-spike.md`
- `docs/reports/0180-shared-family-checker-support-helper.md`
- `scripts/current_l2_family_checker_support.py`
- `scripts/current_l2_same_lineage_checker.py`
- `scripts/current_l2_missing_option_checker.py`
- `scripts/current_l2_capability_checker.py`
- `scripts/current_l2_detached_loop.py`
- `scripts/current_l2_reason_code_readiness.py`
- `scripts/tests/test_current_l2_family_checker_support.py`
- `scripts/tests/test_current_l2_same_lineage_checker.py`
- `scripts/tests/test_current_l2_missing_option_checker.py`
- `scripts/tests/test_current_l2_capability_checker.py`
- `scripts/tests/test_current_l2_static_gate_loop.py`

## 3. Actions taken

- Re-read the required base docs, status snapshots, Phase 5 checker-side chain, and the parser-boundary reserve-path docs before writing the next package.
- Re-checked the shared family checker source anchors and confirmed that current shared output pressure lives in `scripts/current_l2_family_checker_support.py` summary lines, not in a generic shared entry or parser-front API.
- Added:
  - `specs/examples/281-current-l2-minimal-public-checker-command-surface-ready-shared-output-contract-comparison.md`
  - `specs/examples/282-current-l2-shared-output-contract-ready-minimal-shared-output-contract-threshold.md`
- Fixed the current first choice as:
  - shared output contract should start from the family checker shared helper summary line
  - the minimum shared output contract should be `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref`
  - `static gate artifact:` wrapper line, row snippet text, parser boundary, and verifier handoff should remain later
- Updated the mirrors in the same task:
  - `specs/00-document-map.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/13-heavy-future-workstreams.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/90-source-traceability.md`
  - `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## 4. Files changed

- `specs/examples/281-current-l2-minimal-public-checker-command-surface-ready-shared-output-contract-comparison.md`
- `specs/examples/282-current-l2-shared-output-contract-ready-minimal-shared-output-contract-threshold.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0601-phase5-public-checker-shared-output-contract-package.md`

## 5. Commands run and exact outputs

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - `Task baseline recorded.`
- `df -h .`
  - `Filesystem      Size  Used Avail Use% Mounted on`
  - `/dev/vda2        99G   94G 1010M  99% /`
- `free -h`
  - `Mem: 960Mi total / 743Mi used / 96Mi free / 216Mi available`
  - `Swap: 19Gi total / 1.5Gi used / 18Gi free`
- `date '+%Y-%m-%d %H:%M:%S %Z'`
  - `2026-04-11 19:31:57 JST`
- `python3 scripts/new_report.py --slug phase5-public-checker-shared-output-contract-package`
  - `/home/yukatayu/dev/mir_poc_01/scripts/new_report.py:48: DeprecationWarning: datetime.datetime.utcnow() is deprecated and scheduled for removal in a future version. Use timezone-aware objects to represent datetimes in UTC: datetime.datetime.now(datetime.UTC).`
  - `  text = text.replace("- Date:", f"- Date: {datetime.utcnow().isoformat()}Z")`
  - `/home/yukatayu/dev/mir_poc_01/docs/reports/0601-phase5-public-checker-shared-output-contract-package.md`
- `python3 -m unittest scripts.tests.test_current_l2_family_checker_support scripts.tests.test_current_l2_same_lineage_checker scripts.tests.test_current_l2_missing_option_checker scripts.tests.test_current_l2_capability_checker scripts.tests.test_current_l2_static_gate_loop`
  - `Ran 21 tests in 0.021s`
  - `OK`
- `python3 scripts/current_l2_detached_loop.py smoke-same-lineage-checker crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json --artifact-root /tmp/codex-phase5-public-checker-smoke --run-label shared-output-smoke --overwrite`
  - `static gate artifact: /tmp/codex-phase5-public-checker-smoke/static-gates/shared-output-smoke/e4-malformed-lineage.static-gate.json`
  - `fixture: /home/yukatayu/dev/mir_poc_01/crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
  - `artifact: /tmp/codex-phase5-public-checker-smoke/static-gates/shared-output-smoke/e4-malformed-lineage.static-gate.json`
  - `cluster: same_lineage_evidence_floor`
  - `status: matched`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 601 numbered report(s).`
- `git diff --check`
  - no output

## 6. Evidence / findings

- `specs/examples/281` では、shared output contract comparison の first candidate を detached loop wrapper 全体ではなく family checker shared helper の summary line に置き、current shared output pressure を source-backed に固定した。
- `specs/examples/282` では、その minimum を
  - `output_contract_kind`
  - `checker_cluster_name`
  - `checker_status`
  - `public_checker_payload_schema_ref`
  の 4 field に留め、`static gate artifact:` path line と row snippet text は still later に残す current first choice を固定した。
- smoke `e4-malformed-lineage` では、
  - wrapper 側の `static gate artifact: ...`
  - helper 側の `cluster: same_lineage_evidence_floor`
  - helper 側の `status: matched`
  が連続して出ており、shared output contract と wrapper line を別段で読む current judgmentを支える actual source evidence が確認できた。
- targeted Python suite は `Ran 21 tests in 0.021s` / `OK` で、shared helper と family facade / wrapper family の既存 behavior は維持されている。

## 7. Changes in understanding

- current public-looking output pressure は detached loop wrapper 全体ではなく、`scripts/current_l2_family_checker_support.py` が shared に出している `cluster` / `status` summary line にある。
- `fixture:` / `artifact:` path line や row snippet text は実際の console output として重要だが、shared output contract minimum にそのまま入れると parser boundary や verifier handoff を premature に引き寄せやすい。
- shared output contract を 1 段切ったことで、next line は parser grammar ではなく docs-only public checker boundary へ narrow に進めてよい読みが強くなった。

## 8. Open questions

- `checker_cluster_name` を later で `*_ref` family に寄せるべきか
- row snippet textual rendering を later public output contract に含めるべきか
- public checker boundary の minimum をどの relation から始めるべきか

## 9. Suggested next prompt

- `shared output contract minimum を前提に、public checker boundary を docs-only parser-front bundle としてどこまで narrow に切るべきか比較し、mirror sweep と report closeout まで進めてください。`
