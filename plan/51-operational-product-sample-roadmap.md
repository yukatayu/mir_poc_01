# plan/51 — operational product sample roadmap

## 目的

`specs/26` の operational product sample suite を repository-memory として整理する。

## 決定済み

- `P-OPS-01` は `samples/product-alpha1/operational/` を新設する
- `demo/` は release-candidate workflow root として残し、operational suite と混ぜない
- current executable input は `package.mir.json`
- representative `.mir` は explanatory source
- runnable root は `WorldCore`、`MembershipChat`、`SugorokuWorld`、`PortalWorldLink`、`TwoShardHardBoundary`、`TwoShardGradientObservation`
- `future/portal-worldlink/` と shard inventory は同 root に置いてよいが、active portal root と混同せず blueprint / planned-only を維持する

## P-OPS-01 current scope

- package dependency / import chain の first canonical suite
- same-session / local transport / Docker transport / observer-safe devtools / native host launch bundle の bounded operational replay
- release-check helper
- docs / hands-on / research summary / dashboard sync

## P-OPS-03 current scope

- `MembershipChat` に one-lane direct text host boundary を actualize
- `run-local` / `session` / `export-devtools` 上で observer-safe host-I/O evidence を再現
- `scripts/operational_product_samples.py` の semantic check に direct text lane を追加

## P-OPS-13 current scope

- `MembershipChat` の current lane を bounded room-oriented `ChatText("hello room") -> "room#lobby message accepted: hello room"` に widen する
- generic `EchoText` support は product alpha host family に残しつつ、operational `membership-chat` root と starter catalog を `typed_host_io.chat_text` / `ChatText` に切り替える
- `run-local` / `session` / `export-devtools` / helper `run-membership-chat` / `check-all` 上で observer-safe room-oriented host-I/O evidence を再現する
- docs / dashboard / authoring guide を current `ChatText` lane に同期する

## P-OPS-04 current scope

- `SugorokuWorld` に bounded same-session roll / publish / witness / handoff / stale membership reject scenario を actualize
- `run-local` / `session` / `export-devtools` / `release-check` 上で同じ Sugoroku runtime evidence を再現
- `scripts/operational_product_samples.py` の semantic check に Sugoroku runtime event/route/failure evidence を追加

## P-OPS-05 current scope

- `deployments/projection/projection.profile.json` を `ops-product-projection-v0` schema-backed inventory として formalize
- `crates/mir-ast::product_alpha1` の `check` から projection target / packet / FFI inventory summary を accepted obligation として返す
- `crates/mir-runtime::product_alpha1_session` と `crates/mir-runtime::product_alpha1_devtools` から同 inventory を runtime plan / observer-safe projection panel に反映する
- `scripts/operational_product_samples.py` の `release-check` / `check-all` に projection inventory semantic check を追加する

## P-OPS-06 current scope

- `samples/product-alpha1/operational/portal-worldlink/` を active executable root として追加する
- `portal_worldlink` package kind を current product alpha executable line に追加する
- bounded same-session discrete handoff evidenceを `run-local` / observer-safe devtools / helper `release-check` / `check-all` から再現する
- `future/portal-worldlink/` blueprint root を残し、active runtime root と明示的に分ける

## P-OPS-07 current scope

- `samples/product-alpha1/operational/two-shard-hard-boundary/` を active executable root として追加する
- `two_shard_hard_boundary` package kind を current product alpha executable line に追加する
- bounded same-session two-shard offer / prepare / commit / old-owner reject / missing-witness reject / stale-config reject evidenceを `run-local` / observer-safe devtools / helper `release-check` / `check-all` から再現する
- `future/two-shard-hard-boundary/` と `spatial-shard-future.profile.json` を retained blueprint inventory として残し、active runtime root と明示的に分ける

## P-OPS-09 current scope

- `samples/product-alpha1/operational/templates/` を template-only authoring starter root として追加する
- `templates/world-core-starter/` を current validated `world_core` starter として `check` / `run-local` で再現可能にする
- `docs/hands_on/operational_package_authoring_01.md` と `docs/research_abstract/operational_package_authoring_01.md` で external developer 向け `author -> check -> run-local -> session -> export-devtools -> view --check` の bounded order を固定する
- template roots を active operational sample roots や generic release helper と混同しない

## P-OPS-08 current scope

- `native host launch bundle` を current actualized backend-adjacent path として明示する
- WASM client host と LLVM/native projection backend を docs-first comparison inventory としてだけ棚卸しする
- packet / FFI / projection boundary と auth/membership/capability/witness lane preservation requirement を backend reopen prerequisite として書き出す
- generic backend build helper や direct codegen claim は追加しない

## P-OPS-10 current scope

- `templates/membership-chat-starter/` と `templates/sugoroku-world-starter/` を validated `template_only` starter として追加する
- `membership_chat` / `sugoroku_world` starter が sibling starter roots を dependency anchor にしたまま `check` / `run-local` を通ることを固定する
- external developer 向け docs で rename obligation に加えて dependency-retarget obligation を明示する
- `portal_worldlink` / `two_shard_hard_boundary` starter は later とし、template catalog widening を mainstream world/chat/game chain で一度止める

## P-OPS-11 current scope

- `future/gradient-observation.profile.json` を observer-only widening inventory として追加する
- `spatial-shard-future.profile.json` に gradient observation profile ref を追加する
- hands-on / summary / roadmap / dashboard に gradient observation profile が runtime actualizationではなく `planned_only` inventory であることを明記する
- current active runtime root は `two-shard-hard-boundary/` のまま保持する

## P-OPS-12 current scope

- validated starter catalog を current line では `world-core` / `membership-chat` / `sugoroku-world` に留める decision を docs-first に actualize する
- portal/shard authoring は active executable roots を study/copy boundary として扱い、`future/` inventory は non-executable のまま保つ
- authoring guide / summary / roadmap / dashboard に portal/shard starter non-addition の理由と reopen condition を明記する

## P-OPS-14 current scope

- room-chat widening closeout後の queue / validator / roadmap / dashboard wording を current state に揃える
- `scripts/README.md`、`plan/51..52`、`progress.md`、`tasks.md`、`samples_progress.md` の stale reopen-point drift を解消する
- behavior change や new runtime claim を混ぜず、next reopen point を gradient observation runtime first cut に進める

## P-OPS-15 current scope

- `samples/product-alpha1/operational/two-shard-gradient-observation/` を separate runnable root として追加する
- `two_shard_gradient_observation` package kind を current product alpha executable line に追加する
- bounded same-session observer-only gradient view / handoff hint / write reject / stale-view drop / missing-freshness reject evidenceを `run-local` / observer-safe devtools / helper `release-check` / `check-all` から再現する
- `future/gradient-observation.profile.json` は non-executable inventory のまま保持しつつ、paired active runtime root ref を更新する

## P-OPS-17 current scope

- `scripts/product_alpha1_installed_binary_check.py` を追加し、built `target/debug/mirrorea-alpha` と generated native host launch bundle を current public-ish adoption probe として固定する
- `docs/hands_on/product_alpha1_01.md` と `docs/research_abstract/product_alpha1_01.md` を built-binary first reading に同期する
- `README.md`、`Documentation.md`、`plan/50..52`、`progress.md`、`tasks.md`、`samples_progress.md` を packaging-target ambiguity から `final grammar / ABI scoping` 次段へ進める

## P-OPS-18 current scope

- `specs/25` に current alpha-stable hardening target を追加し、versioned `package.mir.json`、documented `mirrorea-alpha` command family、native host launch bundle replay surface を current front door として固定する
- `scripts/product_alpha1_installed_binary_check.py` の JSON output に machine-readable compatibility scope を追加する
- `README.md`、`Documentation.md`、product alpha guide / summary、`plan/50..52`、`progress.md`、`tasks.md`、`samples_progress.md` を `shipped-surface hardening` 次段へ進める

## P-OPS-19 current scope

- current built-binary + host-bundle unit のうち、どこまでを current shipped surface として扱うかを narrow に固定する
- `crates/mirrorea-cli` の bundle stdout / `manifest.json` / `verification-report.json` と `scripts/product_alpha1_installed_binary_check.py` に machine-readable `shipped_surface` block を追加する
- bundled CLI / package root / `manifest.json` / `launch.json` / `run.sh` / `README.md` / observer-safe supporting artifacts を current shipped surface とし、other bundled reports と admin/debug local artifacts を evidence-only として明記する
- docs / roadmap / dashboard を `broader public distribution narrowing` 次段へ進める

## P-OPS-21 current scope

- `MembershipChat` の current bounded lane を helper-reported `room_chat_scope` として machine-readable に固定する
- current lane は bounded single-message room-oriented `ChatText` に留め、multi-message / transport-coupled / room-history / stdio shapes は未定義のままとする
- docs / roadmap / dashboard の next queue を portal/shard starter revisit へ進める

## P-OPS-22 current scope

- current portal/shard authoring boundary を helper-reported `portal_shard_starter_scope` として machine-readable に固定する
- validated starter catalog は `templates/sugoroku-world-starter` で止め、portal/shard authoring は active executable roots を study/copy boundary に使う current line を維持する
- `future/` portal/shard inventory は non-executable のままとし、starter duplicate actualization は行わない
- docs / roadmap / dashboard の next queue を broader Sugoroku revisit へ進める

## P-OPS-23 current scope

- current `SugorokuWorld` carrier を helper-reported `sugoroku_scope` として machine-readable に固定する
- current carrier は bounded deterministic same-session roll / publish / witness / handoff / stale-membership reject scenario に留める
- interactive turn choice surface、broader negative-row catalog、networked multi-participant control は current line では未定義のままとする
- docs / roadmap / dashboard の next queue を later room-chat reopening へ進める

## P-OPS-24 current scope

- suite `check-all` に helper-reported `widening_queue_scope` を追加する
- current room-chat reopening と portal/shard starter reopening は non-promoted queue として固定する
- current next promoted comparison を `broader_sugoroku_reopening` に進める
- docs / roadmap / dashboard の current recommendation を queue-state helper と同期する

## P-OPS-01 non-goals

- final textual grammar
- final SDK / ABI
- final server/client split
- LLVM backend
- continuous portal spatial sync
- shard replication actualization
- WAN / federation
- distributed durable save/load

## next packages

1. broader Sugoroku reopening
   room-chat reopening is now closed as non-promoted; decide later whether the current bounded deterministic Sugoroku carrier really needs broader interactive controls or additional negative rows

## current recommendation

- `P-OPS-03` で direct text host boundary は `MembershipChat` に narrow lane として actualize 済み
- `P-OPS-13` で current `MembershipChat` lane は bounded room-oriented `ChatText` に widen 済み
- `P-OPS-04` で `SugorokuWorld` の bounded scenario は current product alpha session carrier に寄せて actualize 済み
- `P-OPS-05` で projection schema と packet / FFI boundary inventory は schema-backed inventory として actualize 済み
- `P-OPS-06` で `PortalWorldLink` bounded same-session discrete handoff root は actualize 済み
- `P-OPS-07` で `TwoShardHardBoundary` bounded same-session hard-authority root は actualize 済み
- `P-OPS-09` で `templates/world-core-starter/` と bounded package authoring guide は actualize 済み
- `P-OPS-08` で current host launch bundle line を保ったまま backend feasibility inventory は docs-first に actualize 済み
- `P-OPS-10` で `templates/membership-chat-starter/` と `templates/sugoroku-world-starter/` を追加し、starter catalog を mainstream chain まで widen 済み
- `P-OPS-11` で `future/gradient-observation.profile.json` と guide を追加し、observer-only widening を `planned_only` inventory として actualize 済み
- `P-OPS-12` で starter catalog を `SugorokuWorld` までに留め、portal/shard authoring は active roots と `future/` inventory を分けて読む decision を docs-first に actualize 済み
- `P-OPS-14` で maintenance / dashboard freshness を閉じ、queue / validator / roadmap / dashboard wording を current state に同期済み
- `P-OPS-15` で separate `TwoShardGradientObservation` runnable root を actualize し、existing hard-boundary root と planned-only profile inventory を保ったまま bounded observer-only runtime evidence を切り分け済み
- `P-OPS-17` で `scripts/product_alpha1_installed_binary_check.py`、built `target/debug/mirrorea-alpha` probe、bundle `run.sh check/view` probe、product alpha guide / summary sync を追加し、current first public-ish adoption candidate を installed binary + native host launch bundle として actualize 済み
- `P-OPS-18` で `specs/25` と installed-binary helper output を使い、current hardening target を versioned `package.mir.json`、documented `mirrorea-alpha` command family、native host launch bundle replay surface に絞り込んだ
- `P-OPS-19` で helper / bundle stdout / manifest / verification report に machine-readable `shipped_surface` block を追加し、current alpha replay bundle surface と evidence-only reports/local artifacts を分けた
- `P-OPS-20` で helper に machine-readable `distribution_scope` を追加し、broader public distribution は current line では未定義、すなわち developer-built binary + generated host launch bundle 以外の archive / installer / system-package / auto-update / hosted-service shape をまだ持たないと固定した
- `P-OPS-21` で helper に machine-readable `room_chat_scope` を追加し、current `MembershipChat` lane は bounded single-message room-oriented `ChatText` に留まり、multi-message / transport-coupled / room-history / stdio shapes は未定義と固定した
- `P-OPS-22` で helper に machine-readable `portal_shard_starter_scope` を追加し、validated starter catalog が `templates/sugoroku-world-starter` で止まり、portal/shard authoring は active executable roots を study/copy boundary に使う current line を machine-readable に固定した
- `P-OPS-23` で helper に machine-readable `sugoroku_scope` を追加し、current `SugorokuWorld` carrier は bounded deterministic same-session scenario に留まり、interactive turn choice / broader negative rows / networked multi-participant control は未定義と固定した
- `P-OPS-24` で helper に machine-readable `widening_queue_scope` を追加し、current room-chat reopening と portal/shard starter reopening は non-promoted、`broader_sugoroku_reopening` が next promoted comparison であることを固定した
- 次は broader Sugoroku reopening とし、room-chat / portal-shard starter queue は narrowed state のまま維持する

## open questions

- current bounded deterministic `SugorokuWorld` carrier を broader Sugoroku reopening の promoted comparison に戻す必要が本当にあるか
- current projection inventory summary を richer projection IR / placement planner boundary にいつ widen するか
- `MembershipChat` の current bounded `ChatText` lane を保ったまま、later room-chat reopening を改めて promoted queue に戻す必要が本当にあるか
- WASM client host comparison を projection inventory の内側へ寄せるか、独立 docs inventory として維持するか
- broader Sugoroku reopening を閉じた後でも、portal/shard starter reopening や later room-chat reopening を改めて promoted queue に戻す必要が本当にあるか
