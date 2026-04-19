# Report 0730 — FAQ 006 theory-line reintegration and queue reconstruction

- Date: 2026-04-17 20:26 JST
- Author / agent: Codex
- Scope: `faq_006.md` を current explanation delta として監査し、二大問題の current first line / retained alternatives / stop line / true user-spec gate を repo の current snapshot / plan / spec / sample mapping に再統合する。
- Decision levels touched: L1 mirror refresh, L2 theory-lab current-first-line integration, process/snapshot discipline

## 1. Objective

- `faq_006.md` と `specs/` / `plan/` / `tasks.md` / `progress.md` の drift を監査する。
- `p06` / `p07` / `p08` の current role を source-backed / helper-local / still-open の粒度で固定する。
- typed/theorem/model-check、order/handoff、syntax/modality について current first line / retained alternatives / stop line / true user-spec gate を source-backed に再統合する。
- `current self-driven queue = 0 package` drift を解消し、next self-driven packages を snapshot / plan に戻す。

## 2. Scope and assumptions

- `faq_006.md` は current explanation delta として扱い、規範判断の正本には昇格させない。
- current task の主眼は mainline actualization ではなく theory-lab integrator line の再編成である。
- final parser grammar、final public verifier contract、shared-space final catalog、concrete theorem/model-check production binding はこの task で fixed しない。

## 3. Documents consulted

- Core docs: `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `.docs/progress-task-axes.md`
- Normative docs: `specs/00`, `01`, `02`, `03`, `04`, `05`, `07`, `09`, `10`, `11`, `12`
- Plan docs: `plan/00`, `01`, `03`, `04`, `06`, `08`, `09`, `10`, `11`, `12`, `13`, `14`, `16`, `17`, `18`, `90`, `91`
- Current explanation delta: `faq_006.md`
- Shared-space examples: `specs/examples/121`〜`125`
- Theory-lab examples: `specs/examples/126`, `127`, `128`, `130`, `133`, `134`, `135`, `405`〜`422`, `433`〜`448`, `456`, `457`
- Sample docs: `samples/current-l2/README.md`, `samples/prototype/README.md`, `samples/not_implemented/README.md`

## 4. Actions taken

1. AGENTS 順で core docs / plan / specs / examples / sample READMEs を読み、`faq_006.md` と snapshot docs の drift を監査した。
2. `p06` / `p07` / `p08` が
   - corrected prototype
   - helper-local preview
   - non-final public surface
   としてどこまで統合済みかを確認した。
3. existing comparison / threshold notes を束ねる integration note `specs/examples/458...` を追加し、二大問題と syntax/modality の current first line / retained alternatives / stop line / user-spec gate を明文化した。
4. `Documentation.md`、`tasks.md`、`progress.md`、`plan/01`、`plan/11`、`plan/12`、`plan/13`、`plan/16`、`plan/17`、`plan/18`、`plan/90`、`specs/00`、`specs/10`、`specs/11`、`specs/12`、`samples/prototype/README.md` を更新し、queue drift を解消した。

## 5. Files changed

- Added: `specs/examples/458-current-l2-faq006-drift-audit-first-line-stop-line-and-queue-reconstruction.md`
- Added: `docs/reports/0730-faq006-theory-line-reintegration-and-queue-reconstruction.md`
- Updated: `Documentation.md`
- Updated: `tasks.md`
- Updated: `progress.md`
- Updated: `plan/01-status-at-a-glance.md`
- Updated: `plan/11-roadmap-near-term.md`
- Updated: `plan/12-open-problems-and-risks.md`
- Updated: `plan/13-heavy-future-workstreams.md`
- Updated: `plan/16-shared-space-membership-and-example-boundary.md`
- Updated: `plan/17-research-phases-and-autonomy-gates.md`
- Updated: `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- Updated: `plan/90-source-traceability.md`
- Updated: `specs/00-document-map.md`
- Updated: `specs/10-open-questions.md`
- Updated: `specs/11-roadmap-and-workstreams.md`
- Updated: `specs/12-decision-register.md`
- Updated: `samples/prototype/README.md`

## 6. Commands run and exact outputs

```text
$ python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
[baseline recorded]

$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/               99G   78G   17G  83% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       614Mi        82Mi       1.0Mi       421Mi       346Mi
Swap:           19Gi       1.0Gi        18Gi

$ git status --short
[no output]

$ date '+%Y-%m-%d %H:%M %Z'
2026-04-17 20:26 JST
```

## 7. Evidence / findings

- `faq_006.md` は、repo の current issue が「未着手」ではなく「promotion / stop-line / contract concretization」に移ったと明示している。
- これに対して `Documentation.md`、`tasks.md`、`progress.md`、`plan/01`、`plan/11`、`plan/17` は、`p06` / `p07` / `p08` close を根拠に `current self-driven queue = 0 package` と読んでおり、theory-lab line の remaining self-driven integration package を潰していた。
- `specs/examples/456` は `p06` を typed/theorem/model-check sample-visible corrected prototype tranche と明記し、final typed calculus / theorem contract / settled property languageを non-goal にしている。
- `specs/examples/457` は `p07` / `p08` を order/handoff corrected prototype third tranche と明記し、final source wording / fairness profile / shared-space final contract を non-goal にしている。
- `specs/examples/445` / `446` / `447` は stronger typed surface actual adoption、theorem discharge/public-contract concretization、settled property language/tool seam を still later gate に残しており、snapshot docs の「mixed gate only」への短絡が強すぎた。
- `specs/examples/405` / `407` / `408` / `411` / `442` は order/handoff line の current first line を already 持っている。
- `specs/examples/410` / `422` / `437` / `444` は modal foundation を partial basis + stronger candidate family + trigger で扱っており、`lambda_circle_box` single-family adoption ではない。

## 8. Changes in understanding

- corrected prototype tranche close は、theory-lab solved を意味しない。
- current self-driven package は actual adoption package ではなく、first-line integration / stop-line hardening / queue reconstruction package として残る。
- problem 1 の current first line は
  checker attachment principal + notebook-first theorem + row-local model-check であり、
  stronger typed surface / public theorem contract / settled property language は later gate である。
- problem 2 の current first line は
  cut family decomposition + relation decomposition principal + `authority_serial_transition_family` first + low-level reference family retained-later
  であり、
  final source wording / emitted schema / fairness profile は later gate である。
- syntax / modality line は
  semantic honesty axis + `lambda_circle_box` partial basis + guarded / MDTT / MTT / Fitch-style multimodal retained family
  で読み直すのが自然である。

## 9. Open questions

- stronger typed surface を actual adoption 判定へ送る concrete mixed gate をどこに置くか。
- theorem discharge result / receipt / proof-object public contract をどの concrete consumer pressure で reopen するか。
- model-check settled property language と concrete tool seam をどの concrete consumer pressure で reopen するか。
- order/handoff final source wording と final emitted-artifact schema をどの package で narrow するか。
- fairness / replay final operational profile と shared-space final catalog をどの mixed/user-spec gate で扱うか。

## 10. Suggested next prompt

`specs/examples/458` と更新後の `tasks.md` / `progress.md` を前提に、Package B だけを対象にして、typed/theorem/model-check line の current recommendation と kill criteria をさらに sample / negative corpus / helper-local preview まで硬化してください。mainline actualization には踏み込まず、public verifier contract は deferred のまま保ってください。
