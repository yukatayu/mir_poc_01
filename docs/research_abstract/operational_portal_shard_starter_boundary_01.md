# Operational Portal/Shard Starter Boundary 01 Summary

`P-OPS-12` は、portal/shard line を **starter catalog に広げない current decision** を reader-facing に固定する summary です。

## What Is Fixed

- validated `template_only` starter catalog は `world-core-starter/`、`membership-chat-starter/`、`sugoroku-world-starter/` に留める
- `portal-worldlink/`、`two-shard-hard-boundary/`、`two-shard-gradient-observation/` は active executable roots のまま扱う
- `future/portal-worldlink/`、`future/two-shard-hard-boundary/`、`gradient-observation.profile.json` は non-executable inventory のまま扱う

## Why

- portal/shard line では active runtime root と future blueprint inventory がすでに分かれている
- ここで starter duplicates を入れると active root / template / future blueprint の境界が曖昧になる
- current bounded authoring guide は mainstream `WorldCore -> MembershipChat -> SugorokuWorld` chain を先に安定させる

## What To Use Instead

- portal authoring study root:
  `samples/product-alpha1/operational/portal-worldlink/`
- shard authoring study root:
  `samples/product-alpha1/operational/two-shard-hard-boundary/`
- gradient authoring study root:
  `samples/product-alpha1/operational/two-shard-gradient-observation/`
- starter-boundary guide:
  `../hands_on/operational_portal_shard_starter_boundary_01.md`

## What It Does Not Claim

- current portal/shard starter templates
- executable `future/` roots
- current gradient starter templates
- final public scaffold CLI or final public authoring policy
