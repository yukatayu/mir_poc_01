# Question 013 — Rust / Python split の長期戦略

## 前提

- この質問の前に `plan/` と `specs/` を読んでいる前提です。
- この質問は単独で答えてください。
- current repo では、`mir-ast` / `mir-semantics` / `mir-runtime` の core actual path は Rust に寄っています。
- 一方で、docs validation、regression orchestration、detached loop helper、report scaffolding などは Python が担っています。
- repo 側の current reading は「execution-critical / stable surface は Rust-heavy、repo-local helper workflow は mixed tool」です。

## 相談したいこと

この split は長期的にどう考えるのが良いでしょうか。

聞きたいことは次です。

1. どの層までを Rust へ寄せるべきでしょうか。
2. どの helper は Python のまま残しても問題ないでしょうか。
3. public CLI、formal handoff、runtime shell、artifact emission を Rust へ寄せるタイミングはいつが自然でしょうか。
4. 「最終的に全部 Rust」ではなく mixed helper を残す場合、どういう boundary rule を置くと混乱が減るでしょうか。

## 期待する回答

- Rust-heavy core / mixed helper split の是非を評価してください。
- migration trigger と non-trigger を分けてください。
