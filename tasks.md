# Current Task Map (LAB)

最終更新: 2026-10-13 16:34 JST

**Canon notice:** `mirrorea_canon/` is the normative source. Everything outside is LAB; if LAB conflicts with canon, canon wins. This snapshot creates no Canon/THM/OBL/SCN/Gate/Phase decision or new roadmap.

## document role

Plan250 remains the sole Canon-authorized roadmap, separately owner-paused after accepted I3-3 under ADR-0043. I3-4 requires explicit owner resume; Plans247/249 are closed baselines. This task-local W4 research is independently authorized.

## current promoted package

Canon current-position source: `mirrorea_canon/adr/ADR-0043.md`.
LAB dependency/current-task memory: `plan/proof-first-foundation-correspondence.md`.

W4-Dは、実Flowを各Modeが所有し、同じ3プロセスでDATAと実native履歴を保持したまま2回目の権限更新を検証済みです。実処理の完了待ち、全3者のprepare→publish→activate、古いrequest／Ackの拒否、更新後に発行した処理と履歴graphを確認しました。外部未採用39pathのauthority v8で通常default／private QUIC build、新規13件をseamsあり／なし、関連回帰を通過。次source activation／all3 refreeze、D統合とEは残っています。

現行owner指示（2026-10-13・再開）: 同じW4 goalでDの受理後にEまで続行し、E完了又は週間残量30%未満での検証・保存可能な区切りで停止します。残量確認は1時間以上あけます。主担当一人、gpt-6-astra/xhigh、sub-agent・Oracle・通知・hostshareなし。W5+・Plan250/I3-4・新goal・規範採用は開始しません。2026-10-07の作業停止と50%閾値は、この指示で更新されました。

W4-A/B/C remain bounded LAB closes. Current component: ORIGINAL-AUTHORITY-CYCLE-INTEGRATION-v1.json / notes42, including ORIGINAL-FLOW-CUSTODY-INTEGRATION-v1.json. Fixed D contract: docs/proof-first/W4_D_IMPLEMENTATION_HANDOFF.md; its earlier model/stop/50% preamble is historical. Exact evidence: RESUME.md, W4_CHECK.json and one Report2614. Adopted Rust is restored.

## ordered self-driven packages

次に自走で進める順番とrough estimate。

残りの実作業時間の粗い目安は、Dの次source activation／all3 refreeze2–5h、D統合3–6h、計5–11hです。DATA保持中の権限更新は通過しましたが、残る所有・資源・形式対応で反例が出れば延伸します。E16–40hを含むW4残りは21–51h程度。W5保存・復旧40–100h、W6秘密を守る観測40–100h、W7限定検証済みα統合24–60hは今回の自走対象外の暫定値です。実作業時間の幅であり、期限や上限ではありません。

| Package / macro position | Direct consumer and required evidence | Readiness / hours |
|---|---|---|
| D next source activation/refreeze / Macro3/6 middle | Finite authentic next activation in same world/runtime, retaining old source/native/control/Flow/graph and all3 physical preparation; no cursor or spent transport reset. | 着手可能・次の直接consumer; 2–5h |
| D integrated acceptance / Macro3/6 close | Then-final source positive/falsifier/regression, all-entry proof/resource correspondence and exact disposition of D Body-contained M8 queue versus lower queued-use controls. | 後段依存; Astra xhigh; 3–6h |
| E fresh campaign and A–E/119 union / Macro3/6 close | Fresh network/fault/observer/bypass/I3 campaign; retained unresolved obligations. | 承認済み・D受理に依存; 16–40h |
| W5/W6/W7 | Save/recovery, secret observation/debug and finite verified-alpha integration. | 今回は対象外; future profile-dependent40–100/40–100/24–60h |

## self-driven macro phase reading

Macro0 preserves reproducible evidence; Macro1/5 establish semantics/proofs; Macro2/3 implement references; Macro6 connects actual processes/transport. No whole-project phase recut. Actual original S→T→S, same-event graph, owned Flow and same3-process G2 under retained DATA are finite execution evidence. New source activation and D acceptance remain open. General models, Rust controls and process tests remain distinct.

## user decision gates

No further input is needed for bounded D/E under the2026-10-13 resume. L0/L1, authority/privacy weakening, public API/ABI/wire, production adoption, public/paid operations, Q18/H/H2/C/C2 adoption and Canon/Plan250 resume remain owner-reserved. No new goal or W5+ resume.

## research discovery items

| Item | Impact / alternatives / position / falsifier |
|---|---|
| Source custody/native I3 | Full original cursor/Core/arguments and native one-use admission remain distinct; matching peer/free IDs cannot grant custody. |
| Preparation/transport custody | Actual Flow application/session/control retained inside Mode; actual3 observation maps and store/source/native/Flow state preserved through G2. Quinn/kernel internals remain lower environment. |
| Authority and DATA history | Move only complete funded prior stages; close issuance before healthy drain. Historical Report DATA uses exact original correspondence; native Ack also requires CURRENT lineage/floor. Actual fault controls preserve old outcome and pending state. |
| Later activation | Distinct source activation differs from M9/protocol revision. Need finite authentic producer and fresh funded slots; preserve spent Flow budgets and old histories. Reset/recreated runtime fails same-world requirement. |
| Queued-use scope | Actual D Body contains enqueue/serve/retained outcome in one interval; no M9 update inside it. Existing lower C queued refresh cases are separate evidence; integrated disposition remains explicit. |

## maintenance tasks

One Report2614, existing plan, CURRENT_GOAL/RESUME, W4_CHECK and append-only READ_LEDGER. Preserve failures/pins/logs/restoration. Heavy runs serial with resource audit,8GiB/-j1/testthreads1 and debug/incremental off. No cleanup, notifications, subagents, Oracle or hostshare.

週間残量は2026-10-13T06:59:29.442Zの自分の実セッション記録で97%（使用3%、10080分window）でした。確認は2026-10-13T06:59:50.914515+00:00、次は2026-10-13T07:59:50.914515+00:00以降です。確認時点では30%の停止条件に達していません。

## non-promoted references

Authority v8 at the identical39-path cut: normal default/privateQUIC checks pass;13 actual authority-cycle tests pass both with and without i3-process-test-seams. Serial regression counts are [13, 17, 22, 5, 8, 3, 2, 2, 58]: authority13 combined, then no-seams source Ack, owner, graph, actual lower held-M9, full prepared ACK, in-place preparation, whole Mode binding and control module respectively. These are filtered suites, not a whole-library or fresh Lean campaign. Flow v2 earlier had normal2 plus actual custody/field and focused regressions; those immutable receipts retain their own cut.

No D/E acceptance. Distinct next source activation in the same world, its all3 refreeze, full D proof/resource/entry correspondence and E fresh campaign remain. D keeps actual M8 enqueue/serve inside the single Body interval; I3 Awaiting/ServeReserved is not an M8 queued use. Lower held-M9 controls and D close/drain evidence must be reconciled explicitly at integrated acceptance. No adopted Rust, Canon/public contract, independent review, recovery or broad secrecy/noninterference claim.

Graph-v6 entire module191, old full1386/first Ack and sequence full1400 retain their historical cuts. R01–R12 and119 requirements retain original U/D/adoption status. Existing sample roots/workflow classification unchanged.
