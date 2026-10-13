# Current Task Map (LAB)

最終更新: 2026-10-13 12:59 JST

**Canon notice:** `mirrorea_canon/` is normative. Everything outside
`mirrorea_canon/` is LAB; if LAB conflicts with canon, canon wins. This snapshot
creates no Canon/THM/OBL/SCN/Gate/Phase decision or new roadmap.

## document role

Plan250 remains the sole Canon-authorized roadmap, separately owner-paused after accepted I3-3 under ADR-0043. I3-4 requires explicit owner resume; Plans247/249 are closed baselines. This task-local W4 research is independently authorized.

## current promoted package

Canon current-position source: `mirrorea_canon/adr/ADR-0043.md`.
LAB dependency/current-task memory: `plan/proof-first-foundation-correspondence.md`.

W4-Dを2026-10-13のowner指示で再開し、実3プロセスとprivate QUICによる元sourceのS→T→Sを検証しました。source cursor・native受理・parent ordinalは各3、Sの実Bodyは2回、Tは1回です。完了済みの受信データ・受付／解決・Ready／Finishを事前確保した履歴へ移し、次の処理でも保持します。実データ2件を用いたBusy拒否でも、呼び出し元と現在の状態を保持しました。外部未採用38pathの履歴v4で全1400件が通過し、専用feature条件3か所とテスト2件を補ったv6は予算付き2件、test-seamsなしの履歴8件／全列・peer設定2件、通常default／private QUIC buildを確認済みです。実分散graph、network DATA保持中の権限更新、次activation／all3 refreeze、D統合とEは残っています。

現行owner指示（2026-10-13・再開）: 同じW4 goalでDの受理後にEまで続行し、E完了又は週間残量30%未満での検証・保存可能な区切りで停止します。残量確認は1時間以上あけます。主担当一人、gpt-6-astra/xhigh、sub-agent・Oracle・通知・hostshareなし。W5+・Plan250/I3-4・新goal・規範採用は開始しません。2026-10-07の作業停止と50%閾値は、この指示で更新されました。

W4-A/B/C are closed only within their bounded LAB scopes. Current component: ORIGINAL-SEQUENCE-INTEGRATION-v1.json / notes39. Fixed scope: docs/proof-first/W4_D_IMPLEMENTATION_HANDOFF.md, whose older D-after-stop/model-switch preamble is historical. Exact source/commands/continuation: RESUME.md and W4_CHECK.json; one Report2614. Adopted Rust is restored.

## ordered self-driven packages

次に自走で進める順番とrough estimate。

残りの実作業時間の粗い目安は、Dの分散観測／受信中の権限境界4–8h、次activation／all3 re-freeze2–4h、D統合2–4h、計8–16hです。S→T→Sは通過しましたが、未解決境界の検証が残るため従来の幅を維持します。E16–40hを含むW4残りは24–56h程度。W5保存・復旧40–100h、W6秘密を守る観測40–100h、W7限定検証済みα統合24–60hは今回の自走対象外の暫定値です。調査と反例で延伸し得る実作業時間であり、期限や上限ではありません。

| Package / macro position | Direct consumer and required evidence | Readiness / hours |
|---|---|---|
| D distributed observation/current authority / Macro3/6 middle | Same actual source/transport/native occurrence graph; real DATA retained during actual authority update; no counter-derived graph or C-local substitution. | 着手可能・次の直接consumer; 4–8h |
| D next activation/refreeze / Macro3/6 middle | Same world and runtime, actual subsequent all3 prepare/publish/activate; complete earlier pending/history/results/observations preserved. | 後段依存; 2–4h |
| D integrated acceptance / Macro3/6 close | Then-final source positive/falsifier/regression; all-entry source/control/native/proof/resource correspondence and obligations. | 後段依存; Astra xhigh; 2–4h |
| E fresh campaign and A–E/119 union / Macro3/6 close | Fresh network/fault/observer/bypass/I3 campaign and retained unresolved obligations. | 承認済み・D受理に依存; 16–40h |
| W5/W6/W7 | Save/recovery, secret observation/debug and finite verified-alpha integration. | 今回は対象外; future profile-dependent estimates40–100/40–100/24–60h |

## self-driven macro phase reading

Macro0 preserves reproducible evidence; Macro1/5 establish semantics/proofs; Macro2/3 implement references; Macro6 connects actual processes/transport. No whole-project phase recut. Full original S→T→S is now finite execution evidence; the distributed observation/authority/re-activation contract and integrated D acceptance are still open. General models, finite Rust controls and actual process tests remain separate evidence classes.

## user decision gates

The 2026-10-13 owner resume lifts the earlier operational pause and changes the weekly stop threshold to30%. No further input is needed for these bounded D/E tasks. L0/L1, authority/privacy weakening, public API/ABI/wire, production adoption, public/paid operations, Q18/H/H2/C/C2 adoption and Canon/Plan250 resume remain owner-reserved. Do not create another goal or resume W5+.

## research discovery items

| Item | Impact / alternatives / position / falsifier |
|---|---|
| Source custody/native I3 | Genuine source cursor/full Core/arguments and native one-use admission remain separate. Original derives owner/ordinal; free-standing IDs or matching peer cannot replace custody. |
| History/resource preservation | Move only genuine completed Body/Ack records into already funded per-ordinal slots. Missing completion, wrong grant, replay/refund or loss of actual caller capsule falsifies the candidate. |
| Distributed observation | Retain actual send/receive/native records before joining reference-only output. Test-only arrays, local C graph or reconstructed counts do not establish the required provenance. |
| Current authority | Held network DATA alone does not prevent actual update. Preserve all child observations/native state, require actual all3 prepare→parent publication→activation, and revalidate current use; transport or historic result is insufficient. |
| Later activation | Preserve earlier full runtime/source/result/control history in the same world. A new source activation differs from protocol revision/M9 generation; no generic reconnect reset or recreated runtime. |

## maintenance tasks

Maintain one Report2614, existing plan memory, CURRENT_GOAL/RESUME, W4_CHECK and append-only READ_LEDGER. Preserve every failed source/run and exact pins/logs/restoration. Heavy builds serial with df/free audit,8GiB/-j1/testthreads1 and incremental/debug off. No cleanup request is active; existing experiments/proofs/logs and browser state remain. No notifications/subagents/Oracle/hostshare.

週間残量は2026-10-13T03:56:20.624Zの自分の実セッション記録で76%（使用24%、10080分window）でした。確認は2026-10-13T03:56:56.040574+00:00、次は2026-10-13T04:56:56.040574+00:00以降です。確認時点では30%の停止条件に達していません。

## non-promoted references

History v4: focused history6, actual sequence/profile2, prior Issue11 and full1400 pass; subsequent normal default check fails3 E0433 for unguarded QUIC visitors. Final v6 adds three matching feature guards and two budgeted tests: normal default/privateQUIC checks2, budgeted archival/refusal2, no-test-seams history8 and sequence2 pass. v3 history5/6 passed with owner field-mutation work delaying DATA until the unchanged control deadline; v4 moves only those assertions after real transport. No full1402 run is claimed: enabled privateQUIC logic is unchanged from v4 after removing the three cfg lines, and all other production bytes are identical.

No D/E acceptance yet. Real distributed graph from the same native/transport events, actual authority update while network DATA is held, subsequent source activation in the same world/all3 re-freeze and integrated correspondence remain. No adopted Rust, Canon/public interface/roadmap change, new active sample root, fresh Lean campaign, independent review, OS/recovery/global allocator or broad privacy claim.

Old full1386/first Ack and full-sequence RED remain immutable history. R01–R12 and119 requirements retain their original U/D/adoption status. Samples keep existing paths and workflow classification.
