# Report 2043 — post-P-A1-31 final docs audit

- Date: 2026-05-05 19:41 JST
- Author / agent: Codex
- Scope: non-report documentation / source hierarchy / release-candidate status audit
- Decision levels touched: `L1` status wording only; no new normative requirement added

## Objective

Confirm that the Product Alpha-1 release-candidate line is accurately reflected in non-report docs, without auditing historical `docs/reports/` contents.

## Scope and assumptions

- Scope includes root docs, current dashboards, product alpha specs/plan memory, sample/script docs, source hierarchy checks, and executable release validation.
- Scope excludes semantic review of historical `docs/reports/`, per user request.
- Assumption: Docker / Docker Compose remain available for the full release-candidate validation path. `--skip-docker` remains a partial local probe only.

## Start state / dirty state

- Start branch: `main`.
- Start worktree: clean after `P-A1-31` closeout commits `afa6269` and `c1b5fa7`.
- No user-authored dirty changes were present at the start of this audit.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/25-product-alpha1-public-boundary.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `docs/hands_on/product_alpha1_01.md`
- `docs/research_abstract/product_alpha1_01.md`
- `docs/hands_on/README.md`
- `docs/research_abstract/README.md`
- `samples/README.md`
- `samples/product-alpha1/README.md`
- `samples/product-alpha1/demo/README.md`
- `samples/product-alpha1/docker/README.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/product_alpha1_release_check.py`

## Actions taken

- Audited non-report docs for stale `P-A1-31` status wording and overclaim/non-claim drift.
- Updated `README.md` opening status so it mentions both the current-L2 alpha-ready layer and the Product Alpha release-candidate workflow.
- Updated `specs/25-product-alpha1-public-boundary.md` success/status wording from pre-`P-A1-31` conditional language to post-`P-A1-31` release-candidate language.
- Updated `plan/50-product-alpha1-public-boundary-roadmap.md` current repo state to remove stale “still lacks product alpha-1” wording.
- Updated `progress.md` multi-node/fabric row to account for the bounded local/Docker product alpha workflow while keeping WAN/distributed durability as non-goals.
- Updated `tasks.md` distributed durable save/load recommendation to point to an explicit final-public/durability gate rather than an already-completed α-0.5/α-0.8 milestone.
- Added `sub-agent-pro/product-alpha1-001/` to `check_source_hierarchy.py` and synchronized `scripts/README.md`.

## Files changed

- `README.md`
- `specs/25-product-alpha1-public-boundary.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- `progress.md`
- `tasks.md`
- `scripts/check_source_hierarchy.py`
- `scripts/README.md`
- `docs/reports/2043-post-p-a1-31-final-docs-audit.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
sed -n '1,240p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,260p' progress.md
sed -n '1,280p' tasks.md
sed -n '1,260p' samples_progress.md
sed -n '1,260p' docs/hands_on/product_alpha1_01.md
sed -n '1,260p' docs/research_abstract/product_alpha1_01.md
sed -n '1,260p' samples/product-alpha1/README.md
sed -n '1,260p' samples/product-alpha1/demo/README.md
sed -n '1,240p' scripts/README.md
sed -n '1,260p' specs/00-document-map.md
sed -n '1,520p' specs/25-product-alpha1-public-boundary.md
sed -n '1,760p' plan/50-product-alpha1-public-boundary-roadmap.md
sed -n '1,220p' samples/README.md
sed -n '1,200p' samples/product-alpha1/docker/README.md
sed -n '1,200p' docs/hands_on/README.md
sed -n '1,200p' docs/research_abstract/README.md
rg -n --glob '!docs/reports/**' <stale/overclaim patterns> ...
date '+%Y-%m-%d %H:%M %Z'
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-final-audit-dPthEj
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Validation commands were run after this report was created so the report itself was included in scaffold validation.

## Evidence / outputs / test results

- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-final-audit-dPthEj`: pass with `status = accepted`, `product_alpha1_release_candidate_ready = true`, `failed_commands = []`, `planned_commands = 29`.
- Docker cleanup probe after release check found no remaining `mirrorea-product-a1-*` containers or networks.
- `python3 -m unittest scripts.tests.test_validate_docs`: pass, 13 tests.
- `python3 scripts/check_source_hierarchy.py`: pass, required `106`, present `106`.
- `python3 scripts/validate_docs.py`: pass, `1195` numbered reports including this report.
- `cargo fmt --check`: pass.
- `git diff --check`: pass.

## What changed in understanding

The Product Alpha release-candidate goal is broadly achieved in the code and major docs, but a few status rows still described the pre-`P-A1-31` state. The correction is wording/status alignment only; it does not broaden claims beyond local/Docker alpha scope.

## Open questions

- Final public grammar / ABI remains a separate gate.
- Production WAN/federation remains a separate gate.
- Distributed durable save/load R3/R4 remains a separate gate.
- Arbitrary native package execution and final viewer/telemetry service remain non-goals.

## Suggested next prompt

Open an explicit final-public gate scoping package, or run periodic maintenance/doc freshness audit after future product alpha changes.

## Plan update status

`plan/` 更新済み: `plan/50-product-alpha1-public-boundary-roadmap.md` current repo state was corrected for post-`P-A1-31`.

## Documentation.md update status

`Documentation.md 更新不要`: it already reflected `P-A1-31` release-candidate workflow and final-public non-goals accurately.

## progress.md update status

`progress.md` 更新済み: timestamp, multi-node/fabric row, and recent log were updated.

## tasks.md update status

`tasks.md` 更新済み: timestamp and distributed durable save/load reopen wording were updated.

## samples_progress.md update status

`samples_progress.md 更新不要`: product sample dashboard already reflected release-candidate status, Docker skip non-readiness, and final-public non-goals.

## Reviewer findings and follow-up

No sub-agent reviewers were used for this audit. Self-review findings:

- Stale status wording in `plan/50` could make readers think `demo` / release validation still lacked implementation.
- `specs/25` success wording still used pre-closeout conditional language.
- `progress.md` understated product alpha local/Docker fabric readiness in one maturity row.
- `check_source_hierarchy.py` did not require the product alpha handoff directory committed during P-A1-31.

All findings above were addressed in this task.

## Skipped validations and reasons

- Historical `docs/reports/` semantic audit skipped by user request.
- Final public grammar / ABI, production WAN/federation, distributed durable save/load, arbitrary native execution, and final viewer/telemetry validation were not run because they remain explicit non-goals.

## Commit / push status

- Audit commit: `90072cd` (`docs: audit product alpha1 release candidate status`), pushed to `main`.
- Report metadata follow-up: pending at this update; final hash is recorded in the user-facing closeout answer to avoid a self-referential metadata loop.

## Sub-agent session close status

No sub-agents were opened for this audit.
