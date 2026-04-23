# Sugoroku world runtime attachment samples

This directory is an active clean near-end sample family for a repo-local Mir / Mirrorea vertical slice.

It demonstrates:

- empty world bootstrap
- runtime Sugoroku game attachment
- logical multi-Place execution in one OS process
- server-appointed admin
- admin-only start/reset
- dice-owner-only roll
- roll -> publish -> witness -> handoff
- late join published-history visibility
- leave epoch / incarnation invalidation
- owner leave reassignment
- reset interleaving model-check
- detach as a TODO lifecycle boundary

This is not final public parser grammar and does not implement real networking, consensus, durable distributed commit, or final public runtime API.
