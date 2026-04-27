# 0926 Network Transport Plan

## Objective

phase 13 `Network transport` を docs-first で repo に固定し、
`local_queue` / `provider_boundary` current anchor から loopback / reconnect / failure matrix への widening ladder を整理する。

## Scope and assumptions

- current scope は docs-first roadmap、planned sample family、reader-facing summary の追加である。
- final transport ABI、cryptographic session protocol、concrete broker / socket choice、multi-server consensus は固定しない。
- worktree には unrelated current-L2 CLI formatting diff が残っているため、この package では stage / commit に含めない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/05-mirrorea-fabric.md`
- `specs/07-typed-effects-wiring-platform.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/20-projection-and-placement-roadmap.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `samples/not_implemented/typed-external-boundary/README.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/typed_external_boundary_adapter_plan_01.md`

## Actions taken

1. `plan/22-network-transport-roadmap.md` を新規追加した。
2. `samples/not_implemented/network-transport/README.md` を追加し、phase 13 planned family `NET-01..05` を固定した。
3. `docs/research_abstract/network_transport_plan_01.md` を追加し、reader-facing summary を置いた。
4. README / Documentation / progress / tasks / samples_progress / specs / plan / report を、next promoted package が `Compiler/backend/LLVM preparation` である current snapshot に同期した。
5. transport widening が auth / membership / capability / witness / visualization / telemetry を collapse しないことを docs-first invariant として明文化した。
6. stale queue を避けるため、`samples_progress.md` の phase 13 row と `progress.md` / `tasks.md` の next-package snapshot を current queue に合わせて更新した。

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/not_implemented/README.md`
- `samples/not_implemented/network-transport/README.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/20-projection-and-placement-roadmap.md`
- `plan/21-hotplug-attachpoint-roadmap.md`
- `plan/22-network-transport-roadmap.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/network_transport_plan_01.md`
- `docs/reports/0926-network-transport-plan.md`

## Evidence / outputs / test results

- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug envelopes --format json`
  - pass
- `python3 scripts/sugoroku_world_samples.py run 05_late_join_history_visible --debug membership`
  - pass
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- network transport package は real socket/broker 実装の前に、`local_queue` baseline と `provider_boundary` baseline を 1 つの widening ladder で整理しておくだけでも、auth/transport collapse をかなり抑制できる。
- reconnect を transport convenience に寄せず、membership epoch / incarnation と witness path を維持する requirement として書くことで、phase 4 / phase 7 / phase 13 の接続が見えやすくなる。

## Open questions

- `loopback_socket` / `network_link` / broker family をどの transport seam 名で public に出すか。
- reconnect / retry と replay / duplicate detection をどこまで transport layer に置き、どこから lifecycle / policy layer に渡すか。
- network route trace を visualization / telemetry line へ widen するときの retention / redaction policy。

## Suggested next prompt

`Compiler/backend/LLVM preparation` package を進めてください。external workdir、cargo registry cache、LLVM path、minimal probe、detach-safe cleanup、root-disk guardrail を docs / progress / tasks / samples_progress / report まで同じ task で閉じてください。
