# 1021 — Problem 2 actual-package evidence refresh review

## Objective

Review the current in-progress Problem 2 actual-package evidence refresh in `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md`, focusing on:

1. whether the replacement evidence commands are real current front doors,
2. whether `p07` / `p08` are treated as historical archive/package-reading anchors rather than current runnable targets,
3. whether the `p08` replacement is stated as partial rather than a full single-sample replacement.

## Scope and assumptions

- Review scope was limited to the target file as the subject of review.
- Repo policy still required reading the ordered top-level docs and writing a fresh report.
- Validation was limited to current command surfaces, sample landing pages, and helper behavior needed to verify the target file's claims.
- `progress.md` / `tasks.md` / `samples_progress.md` were not updated because this was a review-only task, not a status-changing implementation task.
- `plan/` update unnecessary.

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `scripts/README.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `samples/clean-near-end/network-transport/README.md`
- `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md`

## Actions taken

- Read the required repo front-door and normative context in the repository-prescribed order.
- Inspected the target file with line numbers.
- Verified that each cited command exists and is accepted by the current helper / CLI surfaces.
- Executed the cited commands needed to check whether they remain real current entrypoints and whether their outputs match the wording in the target file.
- Cross-checked the historical `p07` / `p08` naming against current archive locations and current active sample landing pages.

## Files changed

- Added `docs/reports/1021-problem2-actual-package-evidence-refresh-review.md`
- `progress.md` update unnecessary
- `tasks.md` update unnecessary
- `samples_progress.md` update unnecessary

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
rg --files README.md Documentation.md progress.md specs plan/00-index.md
rg --files specs | sort
sed -n '1,220p' README.md
sed -n '1,220p' Documentation.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,220p' specs/03-layer-model.md
sed -n '1,220p' specs/09-invariants-and-constraints.md
sed -n '1,220p' plan/00-index.md
rg --files specs/examples | rg '^specs/examples/467-'
nl -ba $(rg --files specs/examples | rg '^specs/examples/467-')
sed -n '1,260p' scripts/README.md
rg -n "03_roll_publish_handoff|05_late_join_history_visible|NET-03|check-source-sample|smoke-all|current_l2_source_sample_runner|current_l2_operational_cli" -S README.md Documentation.md scripts specs samples Cargo.toml crates
python3 scripts/sugoroku_world_samples.py --help
python3 scripts/network_transport_samples.py --help
python3 scripts/current_l2_guided_samples.py --help
cargo run -q -p mir-runtime --bin mir-current-l2 -- --help
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug summary --format json
python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership --format json
python3 scripts/network_transport_samples.py run NET-03 --debug reconnect --format json
python3 scripts/network_transport_samples.py check-all --format json
python3 scripts/current_l2_guided_samples.py smoke-all --format json
cargo run -q -p mir-runtime --bin mir-current-l2 -- check-source-sample samples/clean-near-end/order-handoff/01_authorized_roll_publish_handoff.mir --format json
python3 scripts/clean_near_end_samples.py run model-check --format json
python3 scripts/current_l2_lean_sample_sync.py
nl -ba samples/clean-near-end/sugoroku-world/README.md | sed -n '40,90p'
nl -ba samples/clean-near-end/network-transport/README.md | sed -n '1,90p'
nl -ba scripts/README.md | sed -n '1,80p'
nl -ba README.md | sed -n '132,170p'
nl -ba Documentation.md | sed -n '232,266p'
rg -n "replay|stale reconnect|fail then refresh|silent merge|NET-03" samples/clean-near-end/network-transport/README.md scripts/network_transport_samples.py specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md
nl -ba scripts/network_transport_samples.py | sed -n '1,260p'
nl -ba scripts/network_transport_samples.py | sed -n '252,340p'
rg -n "p07-dice-late-join-visible-history|p08-dice-stale-reconnect-refresh" -S samples specs docs plan
find samples -path '*p07-dice-late-join-visible-history*' -o -path '*p08-dice-stale-reconnect-refresh*'
date '+%Y-%m-%d %H:%M:%S %z'
git status --short
```

## Evidence / outputs / test results

- `scripts/README.md` lists `current_l2_guided_samples.py`, `sugoroku_world_samples.py`, and `network_transport_samples.py` as current front-door runners, and describes `current_l2_guided_samples.py` as a compatibility wrapper over the active clean near-end suite.
- `samples/clean-near-end/sugoroku-world/README.md` lists:
  - `03_roll_publish_handoff --debug summary --format json`
  - `05_late_join_history_visible --debug membership`
  as active focused runs.
- `samples/clean-near-end/network-transport/README.md` lists:
  - `network_transport_samples.py run NET-03 --debug reconnect --format json`
  - `network_transport_samples.py check-all --format json`
  as active commands and describes `NET-03` specifically as a reconnect epoch guard canary.
- Executed commands succeeded:
  - `03_roll_publish_handoff` returned `terminal_outcome = success` with explicit `publication_order`, `witness_order`, and `scoped_happens_before`.
  - `05_late_join_history_visible` returned `published_history_visible = true` for late join.
  - `NET-03` returned reject with `reason_family = stale_membership_epoch` and text saying a reconnect attempt with stale membership data is rejected explicitly.
  - `check-all` returned all `NET-02..NET-05` passed.
  - `current_l2_guided_samples.py smoke-all --format json` succeeded as the current compatibility aggregate entrypoint.
  - `mir-current-l2 check-source-sample ...01_authorized_roll_publish_handoff.mir --format json` succeeded as a current CLI-shaped checker surface.
  - `current_l2_lean_sample_sync.py` completed and printed `samples/lean/manifest.json`.
- Historical `p07` / `p08` paths now live under `samples/old/2026-04-22-pre-clean-near-end/...` and `samples/lean/old/2026-04-22-pre-clean-near-end/...`, confirming archive status rather than current active runnable status.

## What changed in understanding

- Most of the target file already uses the correct posture:
  - current replacement evidence is pointed at live helper/sample front doors,
  - `p07` / `p08` are generally treated as historical readings,
  - `p08` is generally described as a split, partial replacement rather than a one-sample substitution.
- The narrow accuracy risk is that the file attributes "stale replay" coverage directly to `NET-03`, while the current helper and landing page describe `NET-03` more narrowly as a stale reconnect / membership-epoch rejection canary. The broader replay taxonomy remains later-gated.

## Open questions

- Should line 86 of the target file narrow `NET-03` to `stale reconnect reject floor` and leave `stale/incompatible replay invalidation` to the later sentence about the combined `NET-03` + family-check reading?
- Should the opening objective sentence mention historical/archive status explicitly when naming `p07` / `p08`, or is the later integration note sufficient?

## Suggested next prompt

Tighten `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md` so that `NET-03` is described as a stale reconnect / membership-epoch rejection canary, while keeping the broader `p08` replacement explicitly split across `NET-03` plus the family check and preserving `p07` / `p08` as historical archive anchors.
