# 09 — 不変条件と制約

この文書は、明示的に改訂されるまでは強く扱うべき制約を列挙する。

## グラフ / 因果の不変条件

1. semantic event structure は有向かつ非循環である。
2. cut は明示的 decision boundary を作る。Mir-0 では `atomic_cut` だけを最小の place-local cut primitive として扱う。`atomic_cut` は current `place` の rollback frontier を確定するが、単一 process 全体・分散系・永続化の finalization は意味しない。`durable_cut` のような後段の cut vocabulary は scope を広げうるが、同じ explicit-finalization discipline を守らなければならない。
3. 意味の上で隠れた backward edge を許す mechanism は、既定では疑わしい。

## 進化の不変条件

4. 既定の進化モードは arbitrary upstream rewiring ではなく downstream addition である。
5. API shadowing は既定設計規則として禁止される。
6. compatibility-preserving overlay は domain behavior を黙って狭めてはならない。

## ownership / lifetime の不変条件

7. linear resource は continuation trick や patching trick によって duplication されてはならない。
8. lifetime degradation は monotone である。
9. primitive fallback を超えて完全に形式化された後の preference / fallback chain は、曖昧さのない monotone form へ正規化されなければならない。

## failure / rollback の不変条件

10. failure は意味論上明示的であり続けるべきである。
11. rollback は、compensation へ変換されない限り finalizing cut を越えてはならない。Mir-0 では、これは current `place` 内の rollback が先行する `atomic_cut` を越えられないことを意味する。
12. rollback 不可能な effect は mark するか、隔離するか、compensate しなければならない。

## 統合の不変条件

13. PrismCascade runtime の内部は、その planning guarantee を弱めるなら Mir runtime semantics に押し込んではならない。
14. legacy integration では、どの guarantee が Mir semantics の内部に残り、どれが残らないかを明確に記述しなければならない。
15. engine-specific concept は core language semantics に漏れ込んではならない。

## tooling の不変条件

16. 主要な設計変更には report があるべきである。
17. specs は何が未解決かを記さなければならない。
18. visual tool は、ad-hoc な hidden state ではなく、同じ graph / contract model から説明可能でなければならない。
