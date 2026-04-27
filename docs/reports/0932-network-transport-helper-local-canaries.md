# 0932 Network Transport Helper-Local Canaries

## Objective

phase 13 `Network transport` の残 package `NET-02..05` を、
repo-local alpha current layer を壊さずに helper-local executable canary として actualize する。

## Scope and assumptions

- current scope は helper-local evidence surface に限る。
- final public transport ABI、real socket / broker、cryptographic session protocol、durable replay は scope 外に残す。
- transport は auth / authorization / membership / capability / witness を mint しない。
- `NET-*` sample ID、helper JSON rows、debug 出力は repo-local canary であり、規範正本や final public interface ではない。
- worktree には unrelated current-L2 Rust diff が残っているため、本 package の stage / commit には含めない。

## Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `samples/not_implemented/network-transport/README.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/network_transport_plan_01.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/22-network-transport-roadmap.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0926-network-transport-plan.md`
- `docs/reports/0929-network-transport-loopback-preview.md`
- `docs/reports/0931-hotplug-attachpoint-executable-widening.md`

read-only parallel sub-agent findings:

- `019dcea5-f93d-74f2-9f05-4433bccbc98f` `Hubble`
- `019dcea5-f9c3-7ac2-a5e7-c75ace9535a3` `Darwin`
- `019dcea5-fa49-7e60-bb3b-639af2b7ab10` `McClintock`

## Actions taken

1. `scripts/tests/test_network_transport_samples.py` を追加し、`NET-02..05` の expected shape を先に固定した。
2. `scripts/network_transport_samples.py` を追加し、dedicated helper-local transport harness を実装した。
   - `NET-02`: `scripts/sugoroku_world_samples.py` child process 2 本による subprocess JSON bridge route trace
   - `NET-03`: `06_leave_non_owner` を使った stale `membership_epoch` / `member_incarnation` reconnect reject
   - `NET-04`: timeout / queue-full / route-not-found / detach-after-send の typed failure matrix
   - `NET-05`: observer-safe redacted route trace
3. active landing page と hands-on を追加した。
   - `samples/clean-near-end/network-transport/README.md`
   - `docs/hands_on/network_transport_canaries_01.md`
4. sample / script taxonomy docs を更新し、active helper-local canary と planned backlog を分離した。
   - `samples/README.md`
   - `samples/not_implemented/README.md`
   - `samples/not_implemented/network-transport/README.md`
   - `scripts/README.md`
5. snapshot / roadmap / reader-facing docs を同期し、transport package close と next promoted line を反映した。
   - `README.md`
   - `Documentation.md`
   - `progress.md`
   - `tasks.md`
   - `samples_progress.md`
   - `docs/hands_on/current_phase_closeout_01.md`
   - `docs/research_abstract/README.md`
   - `docs/research_abstract/network_transport_plan_01.md`
   - `plan/01-status-at-a-glance.md`
   - `plan/08-representative-programs-and-fixtures.md`
   - `plan/09-helper-stack-and-responsibility-map.md`
   - `plan/11-roadmap-near-term.md`
   - `plan/17-research-phases-and-autonomy-gates.md`
   - `plan/22-network-transport-roadmap.md`
   - `specs/10-open-questions.md`
   - `specs/11-roadmap-and-workstreams.md`

## Files changed

- `scripts/network_transport_samples.py`
- `scripts/tests/test_network_transport_samples.py`
- `samples/clean-near-end/network-transport/README.md`
- `samples/clean-near-end/sugoroku-world/README.md`
- `samples/README.md`
- `samples/not_implemented/README.md`
- `samples/not_implemented/network-transport/README.md`
- `scripts/README.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/hands_on/README.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/network_transport_canaries_01.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/network_transport_plan_01.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/22-network-transport-roadmap.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `docs/reports/0932-network-transport-helper-local-canaries.md`

## Evidence / outputs / test results

red step:

- `python3 -m unittest scripts.tests.test_network_transport_samples`
  - fail (`ModuleNotFoundError: No module named 'network_transport_samples'`)

green / closeout step:

- `python3 -m unittest scripts.tests.test_network_transport_samples`
  - pass (`Ran 9 tests`)
- `python3 -m unittest scripts.tests.test_sugoroku_world_samples`
  - pass (`Ran 34 tests`)
- `python3 scripts/network_transport_samples.py run NET-02 --debug route-trace`
  - pass
- `python3 scripts/network_transport_samples.py run NET-03 --debug reconnect`
  - pass
- `python3 scripts/network_transport_samples.py run NET-04 --debug failures`
  - pass
- `python3 scripts/network_transport_samples.py run NET-05 --debug route-trace`
  - pass
- `python3 scripts/network_transport_samples.py check-all --format json`
  - pass (`NET-02..05` all green)
- `python3 scripts/network_transport_samples.py closeout --format json`
  - pass
- `python3 scripts/sugoroku_world_samples.py check-all --transport loopback_socket --format json`
  - pass (`NET-01` floor remains green)
- `cargo run -q -p mir-runtime --bin mir-clean-near-end -- run-sample 05_delegated_rng_service --format json`
  - pass (`provider_boundary` remains separate transport-adjacent anchor)
- `python3 scripts/check_source_hierarchy.py`
  - pass (`required 23, missing 0`)
- `python3 scripts/validate_docs.py`
  - pass after final report sync (`Found 930 numbered report(s).`)
- `git diff --check`
  - pass

skipped in this package:

- `cargo test -p mir-runtime`
  - skipped; no Rust runtime code changed in this package
- `python3 scripts/avatar_follow_samples.py closeout --format json`
  - skipped; phase 8 residual widening is the next package, not this one

## What changed in understanding

- `NET-02` の process-boundary proof は dedicated helper に切る方が安全だった。`sugoroku_world_samples.py` の global `--transport` surface をさらに widen せず、subprocess JSON bridge だけを別 helper に隔離したことで、preview seam を final ABI らしく見せるリスクを抑えられた。
- `NET-02` route trace では freshness / auth / capability lane も explicit に残す必要があった。process-boundary canary でも `membership_epoch` / `member_incarnation` / claimed authority / capability / authorization checks を traceable に保ち、observer-safe view とは別 lane に分ける必要がある。
- `NET-03..05` は real transport actualization ではなく、helper-local derived canary として読む方が正確だった。特に `NET-04` は typed failure matrix を explicit に見せる package であり、timeout scheduler や queue implementation を持ったわけではない。
- active path と planned backlog の分離を transport lane でも明示する必要があった。`samples/clean-near-end/network-transport/` を active landing page にし、`samples/not_implemented/network-transport/` を future source/backlog path に据え直した。

## Open questions

- `NET-02` の subprocess JSON bridge を将来 `network_link` preview へ寄せるか、それとも separate harness のまま保つか。
- `NET-03` reconnect guard を avatar / hot-plug lifecycle lane とどこで接続するか。
- `NET-04` typed failure matrix を future adapter lane へどこまで共有するか。
- `NET-05` observer-safe route trace を final visualization schema へどう持ち上げるか。
- exact transport seam name、final public transport ABI、session/signature protocol、durable replay / multi-server consensus は引き続き `OPEN`。

## Suggested next prompt

`Avatar fairy follow residual widening` package を進めてください。`FAIRY-02` visibility-loss fallback と `FAIRY-05` reacquire-after-return を active helper に取り込むか、planned family のまま据え置くかを、sample / docs / progress / report を同一 task で同期しながら判断してください。
