# Operational Authoring Templates

This directory holds validated authoring starters for external developers.

- current status: `template_only`
- these roots are not counted as active operational sample roots
- current executable input remains `package.mir.json`
- representative `.mir` files remain explanatory only
- copy from here when starting a new package, then rename package ids / policy ids / contract ids / labels / retention scopes before treating the result as your own package

Current templates:

- `world-core-starter/`
  validated minimal `world_core` starter for `check` and `run-local`
- `membership-chat-starter/`
  validated `membership_chat` starter for `check` and `run-local`; retarget `dependencies` after copying it
- `sugoroku-world-starter/`
  validated `sugoroku_world` starter for `check` and `run-local`; retarget `dependencies` after copying it

Not yet in catalog:

- no `portal-worldlink-starter/`
- no `two-shard-hard-boundary-starter/`
- current decision keeps portal/shard authoring on the active executable roots and keeps `future/` blueprints non-executable

Guide:

- `docs/hands_on/operational_package_authoring_01.md`
  bounded starter-selection, rename, and `author -> check -> run-local -> session -> export-devtools -> view --check` order
- `docs/hands_on/operational_portal_shard_starter_boundary_01.md`
  current portal/shard authoring boundary and the reason the starter catalog stops at `SugorokuWorld`
