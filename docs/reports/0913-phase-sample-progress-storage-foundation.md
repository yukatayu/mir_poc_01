# 0913 Phase Sample Progress Storage Foundation

## Objective

- `sub-agent-pro/mirrorea_phase_samples_progress_storage_handoff_2026-04-24.md` の内容を repo 実態と照合し、phase / sample / progress / storage foundation を `specs/`、`plan/`、`samples_progress.md`、`progress.md`、`tasks.md`、`AGENTS.md`、関連 docs に反映する。
- runnable sample の progress% を evidence-backed に整理し、small VPS 前提の detachable storage discipline を safe default で追加する。

## Scope and assumptions

- この report の対象は **planning / progress / storage foundation** であり、Mirrorea system complete や final public completion を主張しない。
- `specs/` を規範正本、`plan/` を repository memory、`docs/reports/` を historical evidence、`samples_progress.md` を runnable sample dashboard として扱う。
- root disk を圧迫しないことを優先し、extra storage `vdb` は **visible だが未マウント** という現況に合わせる。
- 厚い fake E2E wrapper は追加せず、既存の clean near-end suite と Sugoroku world helper を自然合成の E2E floor として扱う。

## Documents consulted

### Handoff read

- `sub-agent-pro/mirrorea_phase_samples_progress_storage_handoff_2026-04-24.md`

### Core repository documents

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `.docs/progress-task-axes.md`

### Specs read

- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/11-roadmap-and-workstreams.md`

### Plan / repo memory read

- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`

### Directories inspected

- `specs/`
- `plan/`
- `.docs/`
- `docs/reports/`
- `samples/`
- `scripts/`
- `crates/`

## Actions taken

### Summary

- `samples_progress.md` を新規作成し、phase 0〜16 の sample / unit / E2E / debug / completion criteria / blocker / report を一望できる dashboard を追加した。
- `AGENTS.md` に `samples_progress.md` 維持ルール、progress% の evidence rule、100% rule、no fake validation、no thick fake E2E wrapper、small VPS / detachable storage discipline を追記した。
- `scripts/check_source_hierarchy.py`、`scripts/env/mirrorea_storage_env.sh`、`scripts/storage/detach_prepare.sh`、`scripts/storage/cleanup_disposable_artifacts.sh` を追加した。
- `README.md`、`Documentation.md`、`specs/00-document-map.md`、`specs/11-roadmap-and-workstreams.md`、`plan/01-status-at-a-glance.md`、`plan/09-helper-stack-and-responsibility-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/17-research-phases-and-autonomy-gates.md`、`plan/90-source-traceability.md`、`progress.md`、`tasks.md` を同期した。

### Files inspected

- `samples_progress.md` は task 開始時点では **missing** だった。
- `scripts/env/` と `scripts/storage/` は task 開始時点では **missing** だった。
- `du -sh target .git .cargo .lake` では `.cargo` と `.lake` は **missing** だったため、そのまま無視しつつ audit 結果に記録した。
- `crates/mirrorea-core/` と `crates/mirrorea-control/` は current skeleton crate のままであり、phase 8 以降の helper / carrier 実装はまだ未着手だった。

### Files changed

- created:
  - `samples_progress.md`
  - `scripts/check_source_hierarchy.py`
  - `scripts/env/mirrorea_storage_env.sh`
  - `scripts/storage/detach_prepare.sh`
  - `scripts/storage/cleanup_disposable_artifacts.sh`
  - `docs/reports/0913-phase-sample-progress-storage-foundation.md`
- updated:
  - `.gitignore`
  - `AGENTS.md`
  - `README.md`
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/90-source-traceability.md`
  - `progress.md`
  - `tasks.md`

### Samples progress created / updated

- `samples_progress.md` を required format に合わせて新規作成した。
- phase 0〜16 について、最低限
  - objective
  - representative samples
  - unit validation
  - E2E validation
  - debug / visualization output
  - completion criteria
  を記載した。
- `0/1/10/25/50/65/75/90/100` の意味を summary と active matrix の reading に反映した。
- `100%` は commit/push を含む strict rule として明記し、今回の rows では過剰申告を避けて `90%` 以下に留めた。

### Phase sample plan added

- Phase 0: repository memory / decision boundary dashboard
- Phase 1: current L2 semantics sample floor
- Phase 2: parser-free detached loop
- Phase 3: parser boundary / first checker cut
- Phase 4: shared-space membership / room boundary
- Phase 5: small decidable core / proof boundary
- Phase 6: compile-ready minimal actualization
- Phase 7: Sugoroku runtime attach
- Phase 8: avatar fairy follow / fallback anchor
- Phase 9: typed external boundary / adapter
- Phase 10: MessageEnvelope / authentication seam
- Phase 11: TermSignature / LayerSignature
- Phase 12: projection / placement
- Phase 13: network transport
- Phase 14: hot-plug package / AttachPoint
- Phase 15: visualization / IDE
- Phase 16: compiler/backend/LLVM preparation

### Storage audit

- `df -h`
  - root `/dev/vda2`: `99G` total, `68G` used, `27G` free
- `lsblk -f`
  - extra device `vdb` is visible but no filesystem / mountpoint is active
- `findmnt`
  - `/mnt/mirrorea-work` is not mounted
- `du -sh .`
  - repo is about `5.3G`
- `du -sh target .git .cargo .lake`
  - `target/` is about `5.2G`
  - `.git` is about `66M`
  - `.cargo` and `.lake` are absent in repo root

### Storage scripts added / updated

- `scripts/env/mirrorea_storage_env.sh`
  - exports safe default external-workdir variables
  - refuses `--ensure-dirs` against the unmounted default path unless `--allow-unmounted` is explicit
- `scripts/storage/detach_prepare.sh`
  - non-destructive status print only
  - shows git, disk, mounts, workdir, and disposable-candidate state
- `scripts/storage/cleanup_disposable_artifacts.sh`
  - requires `--confirm`
  - refuses unsafe unmounted default path
  - only touches known disposable directories inside configured workdir

### Progress / task / AGENTS sync

- `AGENTS.md` updated: yes
- `samples_progress.md` updated: yes
- `progress.md` updated: yes
- `tasks.md` updated: yes
- `.docs/progress-task-axes.md` updated: no
  - inspected only; current macro-phase / maturity-stage rule still matched this task

### Git commit / push status

- report authoring時点では **pending**
- final task close で `git commit --no-gpg-sign` と push を行う必要がある
- `100%` claim はその後でなければ使わない

## Evidence / outputs / test results

### Audit commands run

```bash
git status --short
git branch --show-current
df -h
lsblk -f
findmnt
du -sh . 2>/dev/null || true
du -sh target .git .cargo .lake 2>/dev/null || true
```

### Validation commands run

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
bash scripts/env/mirrorea_storage_env.sh
bash scripts/storage/detach_prepare.sh
bash scripts/storage/cleanup_disposable_artifacts.sh --list
python3 scripts/current_l2_guided_samples.py smoke-all --format json
python3 scripts/current_l2_guided_samples.py closeout --format json
python3 scripts/clean_near_end_samples.py smoke-all --format json
python3 scripts/sugoroku_world_samples.py check-all
python3 scripts/sugoroku_world_samples.py closeout --format json
cargo test -p mir-ast
cargo test -p mir-runtime
cargo test -p mir-semantics
```

### Validation results

- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
  - report count was `910` before this report file was added
- `bash scripts/env/mirrorea_storage_env.sh`
  - pass
  - `MIRROREA_WORKDIR_MOUNTED=no`
- `bash scripts/storage/detach_prepare.sh`
  - pass
  - non-destructive audit only
- `bash scripts/storage/cleanup_disposable_artifacts.sh --list`
  - pass
  - list only, no deletion
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - pass
- `python3 scripts/current_l2_guided_samples.py closeout --format json`
  - pass
- `python3 scripts/clean_near_end_samples.py smoke-all --format json`
  - pass
- `python3 scripts/sugoroku_world_samples.py check-all`
  - pass
  - 10 samples passed
- `python3 scripts/sugoroku_world_samples.py closeout --format json`
  - pass
  - 10 samples passed
- `cargo test -p mir-ast`
  - pass
- `cargo test -p mir-runtime`
  - pass
- `cargo test -p mir-semantics`
  - pass

### Validation skipped or intentionally deferred

- dedicated detached-loop CLI smoke
  - skipped in this task
  - reason: phase/sample/progress/storage foundation に集中し、phase 2 row は `75%` のまま据え置いた
- avatar fairy follow helper/sample execution
  - skipped
  - reason: active helper/sample family 自体が未作成
- external storage cutover
  - skipped
  - reason: `vdb` is not yet mounted or prepared in a documented way

## What changed in understanding

- current repo の runnable floor は clean near-end suite と Sugoroku world vertical slice に強く偏っており、phase 8 以降はまだ docs-first / queue-first の比率が高い。
- extra 200GB storage は「使う予定の policy」ではなく、**visible but unmounted** が current fact であり、先に safe env / cleanup / detach policy を置く方が正しい。
- `samples_progress.md` を separate dashboard として持つことで、`progress.md` と `tasks.md` を rough snapshot に保ちながら、sample / E2E / debug / blocker を evidence-backed に追跡しやすくなる。
- previous future-plan integration では `TermSignature registry` が next reopen point だったが、この handoff を反映した current reading では、その前に `Sugoroku sample progress alignment` と `Avatar fairy follow sample plan` を置く方が phase/sample discipline と整合的だった。

## Open questions

- `vdb` をいつ、どの filesystem / mountpoint policy で有効化するか。
- detached-loop artifact の repo-local `target/current-l2-detached` 既定を、どの package で external workdir policy に寄せるか。
- phase 8 `FAIRY-01..06` の active helper / negative sample surface をどの repo path に切るか。
- `--debug signatures` など、phase 11 / 15 の signature / visualization debug surface をどの helper に最初に出すか。
- report count / push evidence を最終 closeout でどこまで repo 内に mirror するか。

## Suggested next prompt

`Sugoroku sample progress alignment` を進めてください。`samples_progress.md` の phase 4 / 7 を per-sample あるいは finer-grained row に分割し、`summary` / `turn-trace` / `membership` / `verification` debug surface を actual command と report へ tighter に結び直してください。validation は `python3 scripts/sugoroku_world_samples.py check-all`、`python3 scripts/sugoroku_world_samples.py closeout --format json`、`python3 scripts/validate_docs.py` を最低限回し、`progress.md` / `tasks.md` / 新規 report を同じ task で同期してください。
