# Report 1136 — P-A1-00 Practical Alpha Rebaseline

- Date: 2026-05-03
- Author / agent: Codex
- Scope: practical alpha-1 progress semantics rebaseline, new spec/roadmap memory, snapshot/validator sync
- Decision levels touched: L1, L2
- 日本語要約: `P-A1-00` では、従来の Stage A..F `100%` を practical alpha-1 達成と混同しないように、alpha-0 evidence closeout と practical alpha-1 readiness を分離した。`specs/18` と `plan/44` を追加し、snapshot docs / sample dashboard / validator を practical line 基準へ同期したが、front-door parser/loader、practical runtime、product-like Docker command はまだ未実装である。

## Objective

Practical alpha-1 を repo の新しい promoted line に昇格し、今後の `100%` を practical alpha-1 readiness として読むための rebaseline package を閉じる。

## Scope and assumptions

- `sub-agent-pro/alpha-1/` は working directive として読み、規範判断は `specs/` / `plan/` / snapshot docs に mirror する。
- `samples/alpha/` は alpha-0 evidence root のまま維持し、practical front-door root に昇格しない。
- `P-A1-00` は docs/spec/taxonomy/validator package であり、practical parser/front-door/runtime 実装はまだ行わない。
- alpha-0 Stage A..F closeout は useful prerequisite evidence として保持するが、practical alpha-1 readiness `100%` とは分けて扱う。

## Start state / dirty state

- start state: `main` branch, worktree clean
- resource preflight:
  - `df -h .`: `/dev/vda2` 99G total, 65G used, 30G available
  - `free -h`: 960Mi total, 381Mi available, swap 19Gi with 1.2Gi used
- prior repo state: `P-A0-28` had closed the current-scope alpha-0 evidence line through Stage A..F and left no safe `P-A0-29`

## Documents consulted

- `sub-agent-pro/alpha-1/00-index.md`
- `sub-agent-pro/alpha-1/01-current-state-gap.md`
- `sub-agent-pro/alpha-1/02-practical-alpha1-definition.md`
- `sub-agent-pro/alpha-1/03-decisions.md`
- `sub-agent-pro/alpha-1/04-stage-roadmap.md`
- `sub-agent-pro/alpha-1/05-theory-freeze.md`
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/07-repository-structure.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `sub-agent-pro/alpha-1/10-subagent-plan.md`
- `sub-agent-pro/alpha-1/11-docs-progress-protocol.md`
- `sub-agent-pro/alpha-1/12-risk-register.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`
- `sub-agent-pro/alpha-1/14-codex-operational-rules.md`
- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/05-mirrorea-fabric.md`
- `specs/06-prismcascade-positioning.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/08-cross-system-relations.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/13-type-system-lifetime-fallback.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `specs/15-cut-save-load-checkpoint.md`
- `specs/16-runtime-package-adapter-hotplug.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/39-type-system-freeze-roadmap.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/41-save-load-checkpoint-roadmap.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `samples/README.md`
- `samples/alpha/README.md`
- `scripts/README.md`
- `docs/reports/1130-p-a0-23-stage-b-alpha-0-5-closeout.md`
- `docs/reports/1131-p-a0-24-stage-c-transport-closeout.md`
- `docs/reports/1132-p-a0-25-stage-d-lifecycle-closeout.md`
- `docs/reports/1133-p-a0-26-stage-e-devtools-closeout.md`
- `docs/reports/1134-p-a0-27-stage-f-alpha-closeout.md`
- `docs/reports/1135-p-a0-28-stage-a-imported-baseline-reconciliation.md`
- `docs/reports/TEMPLATE.md`

## Actions taken

1. Added `specs/18-practical-alpha1-scope.md` as the new normative definition of practical alpha-1 scope, non-goals, completion condition, sample-root boundary, and success statement.
2. Added `plan/44-practical-alpha1-roadmap.md` as repository memory for the practical alpha-1 stage/package sequence `PA1-0..PA1-8`.
3. Reworked `README.md`, `Documentation.md`, `specs/00`, `specs/17`, `plan/00`, `plan/01`, `plan/43`, `samples/README.md`, `samples/alpha/README.md`, and `scripts/README.md` so that alpha-0 Stage A..F remain evidence-closeout memory while practical alpha-1 becomes the promoted line.
4. Reworked `progress.md`, `tasks.md`, and `samples_progress.md` so that:
   - practical alpha-1 becomes the active stage/package map
   - alpha-0 Stage A..F remain explicit evidence-closeout reference
   - old evidence-only `100%` rows no longer appear in top-level mixed summary tables
   - next promoted package is `P-A1-01`
5. Extended validator required-doc sets for `specs/18`, `plan/44`, and `sub-agent-pro/alpha-1/`.
6. Ran four parallel read-only reviews over semantics, runtime wording, docs/progress consistency, and validator/taxonomy honesty; incorporated all substantive findings locally.

## Files changed

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/17-mirrorea-spaces-alpha-scope.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/43-alpha-e2e-roadmap.md`
- `plan/44-practical-alpha1-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/alpha/README.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
df -h .
free -h
git status --short
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
date '+%Y-%m-%d %H:%M JST'
```

## Evidence / outputs / test results

- `python3 -m unittest scripts.tests.test_validate_docs` passed `11` tests.
- `python3 scripts/check_source_hierarchy.py` reported required/present/missing = `63/63/0`.
- `python3 scripts/validate_docs.py` reported `Documentation scaffold looks complete.` and found `1137 numbered report(s)` before adding this report.
- `cargo fmt --check` passed.
- `git diff --check` passed on the draft tree before closeout commit.
- Reviewer findings:
  - one wording drift in `Documentation.md` used `alpha-1 current-scope closeout` for old Stage F; fixed to `Spaces alpha evidence`
  - `samples_progress.md` still exposed evidence-only `100` rows in mixed top-level tables; fixed by downgrading top-level maturity percentages and reserving `100%` closeout claims for explicit package/evidence maps
  - `tasks.md` / `progress.md` still implied the alpha-0 lane could be a parallel promoted line; fixed by marking it reserve/reference only
  - validator/taxonomy review found no issues

## What changed in understanding

- The existing Stage A..F line is not discarded; it is still the strongest alpha-0 evidence prerequisite. The key repair is that it must stop being the repo-wide meaning of `100%`.
- The practical alpha-1 promoted line can start before `U1` because `P-A1-01..08` mostly concern internal toolchain shape, not public product commitment.
- A dashboard that keeps evidence-only rows at top-level `100` undermines the practical/evidence split even if the text elsewhere is correct.

## Open questions

- `P-A1-01` still needs a narrow front-door choice: limited textual alpha syntax vs limited JSON/package-manifest cut. This is not yet a user blocker, but it must be fixed in the next package.
- The practical root path `samples/practical-alpha1/` is planned but intentionally not created in `P-A1-00`; `P-A1-01` should add it only alongside the first real front-door fixtures.

## Suggested next prompt

Continue with `P-A1-01 alpha-source front-door design` by defining the narrowest non-final practical source/package format, adding initial fixtures under `samples/practical-alpha1/`, and implementing parse positive/negative tests without freezing final public grammar.

## Plan update status

`plan/` 更新済み: `plan/00-index.md`、`plan/01-status-at-a-glance.md`、`plan/43-alpha-e2e-roadmap.md` を practical/evidence split に同期し、`plan/44-practical-alpha1-roadmap.md` を追加した。

## Documentation.md update status

`Documentation.md` 更新済み: alpha-0 evidence line と practical alpha-1 line を分離し、`samples/alpha/` と practical front-door root を混同しない wording に更新した。

## progress.md update status

`progress.md` 更新済み: practical alpha-1 readiness map を追加し、Stage A..F `100%` を current-scope evidence closeout reference へ降格した。

## tasks.md update status

`tasks.md` 更新済み: promoted package line を `P-A1-01` へ切り替え、alpha-0 later-family blockers を reserve lane として再配置した。

## samples_progress.md update status

`samples_progress.md` 更新済み: practical package map を追加し、alpha-0 evidence rows を explicit evidence table に隔離した上で、mixed top-level tables から evidence-only `100` を撤去した。

## Reviewer findings and follow-up

- `Ohm`:
  - finding: `Documentation.md` が `P-A0-27` を `alpha-1 current-scope closeout` と書いていた
  - follow-up: `Spaces alpha evidence` wordingへ修正
- `Beauvoir`:
  - finding: `samples_progress.md` summary が top-level `100` を残し、`Documentation.md` の `samples/alpha/` wording が root-language で強すぎた
  - follow-up: summary / matrix / E2E rows を maturity estimate へ下げ、`samples/alpha/` を `non-public evidence family` と書き換えた
- `Cicero`:
  - finding: `samples_progress.md` にまだ top-level evidence `100` が残り、Stage F 名が `alpha 1 Spaces alpha` のままで、promoted line / reserve lane / freshness markers も不揃いだった
  - follow-up: Stage F naming、reserve-lane wording、snapshot timestamps/logs を practical split に合わせて修正した
- `Banach`:
  - finding: no findings
- local follow-up:
  - review findings反映後に docs validator floor を rerun し、closeout前の draft tree で再確認する

## Skipped validations and reasons

- broad Rust runtime tests: skipped for `P-A1-00` because this package changes docs/spec/taxonomy/validators only and does not introduce or modify practical runtime/front-door code paths
- parser/front-door tests: skipped because `P-A1-00` explicitly does not implement `P-A1-01`

## Commit / push status

Pending at report write.

## Sub-agent session close status

- `Ohm`: completed
- `Beauvoir`: completed
- `Cicero`: completed
- `Banach`: completed
- close pending at report write
