# 出典と参照範囲

本文の[Sx]は以下を指す。外部研究は設計と証明対象の比較に使用しており、そこでの定理がF0.3のコードについて成立するとみなしていない。

| ID | 一次資料・版 | 参照した範囲と使い方 |
|---|---|---|
| S1 | yukatayu/mir_poc_01, commit `19a6decfbea0815b7b4265e8fadfb399270aaed2`, 2026-09-09. https://github.com/yukatayu/mir_poc_01/commit/19a6decfbea0815b7b4265e8fadfb399270aaed2 | GitHub connectorでbranch metadataを確認。現在の基準版。全repositoryを監査したという意味ではない。 |
| S2 | 同版 `mirrorea_canon/adr/ADR-0043.md`. https://github.com/yukatayu/mir_poc_01/blob/19a6decfbea0815b7b4265e8fadfb399270aaed2/mirrorea_canon/adr/ADR-0043.md | 本文全体。I3-3の有限受理とpause、非宣言。 |
| S3 | Yongwang Zhao, David Sanan. *Rely-guarantee Reasoning about Concurrent Reactive Systems: The PiCore Framework, Languages Integration and Applications*, arXiv:2309.09148v1, 2023. https://arxiv.org/abs/2309.09148 | abstractと書誌を確認。異なる反応的言語のrely/guarantee adapterを比較する位置づけ。詳細定理をF0.3へ移植したものではない。 |
| S4 | Sebastian Eggert, Ron van der Meyden. *Dynamic Intransitive Noninterference Revisited*, arXiv:1601.05187v1, 2016. https://arxiv.org/html/1601.05187v1 | abstract、導入、§2のモデル・観測・policy、動的意味の説明。permissionとprohibition、観測モデルを分ける必要性。全文の全証明を精査したという主張ではない。 |
| S5 | Huan Sun et al. *Generalized Security-Preserving Refinement for Concurrent Systems*, arXiv:2511.06862v1, 2025. https://arxiv.org/abs/2511.06862 | abstract。並行システムの情報流を保つrefinementを、単一traceの対応と混同しないための比較。 |
| S6 | Cedar Policy Language Reference Guide. *Policy validation*. https://docs.cedarpolicy.com/policies/validation.html | validationとauthorization evaluationの区別、schema変更と再検査。F0.3がCedar実装という意味ではない。 |
| S7 | Jim Gray, Leslie Lamport. *Consensus on Transaction Commit*, arXiv:cs/0408036. https://arxiv.org/abs/cs/0408036 | abstract、書誌。transaction commitの安全性・可用性の条件を明示する背景。 |
| S8 | Li-yao Xia et al. *Interaction Trees: Representing Recursive and Impure Programs in Coq*, arXiv:1906.00046v2 / POPL 2020. https://arxiv.org/abs/1906.00046 | abstract。再帰とeffect handlerを表す候補carrier。Coq libraryをF0.3へ導入してはいない。 |
| S9 | Daniel Gratzer. *Normalization for multimodal type theory*, arXiv:2301.11842v4, 2026-03-14 / LMCS 22(1), 2026. https://arxiv.org/abs/2301.11842 | abstractと版履歴。mode theoryに対する正規化・型検査の条件を、Mirのruntime保証から分ける。 |
| S10 | 添付 `mirrorea_system_map_v1/MASTER.md`, `requirements.json` | 全体要件、30判断、保証とシナリオ、αの能力。提案であり自動承認ではない。 |
| S11 | 添付 F0.1/F0.2研究成果、各FOUNDATION/PROOFS/ledgerおよびコード | 既存研究候補。コード再利用はbase/の二ファイル。新旧の命題を加算して全体証明としない。 |

照合日は2026-09-09。一次資料のabstractだけを読んだものと、本文の該当箇所を読んだものを区別した。研究課題を先行論文が解決済みであるとの推定や、未知の全先行研究を調べ尽くしたとの主張はしていない。
