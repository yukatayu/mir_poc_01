#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
REQUIRED = [
    "CANON.md",
    "README.md",
    "AGENTS.md",
    "Documentation.md",
    "progress.md",
    "tasks.md",
    "samples_progress.md",
    "samples/README.md",
    "samples/full-system-v1/README.md",
    "samples/full-system-v1/computational/README.md",
    "samples/full-system-v1/computational/matrix.json",
    "samples/full-system-v1/computational/add-one-positive/README.md",
    "samples/full-system-v1/computational/add-one-positive/src/add-one.mir",
    "samples/full-system-v1/computational/add-one-positive/expected/parse.json",
    "samples/full-system-v1/computational/host-boundary-positive/README.md",
    "samples/full-system-v1/computational/host-boundary-positive/src/host-boundary-add-one.mir",
    "samples/full-system-v1/computational/host-boundary-positive/expected/parse.json",
    "samples/full-system-v1/computational/malformed-function-negative/README.md",
    "samples/full-system-v1/computational/malformed-function-negative/src/malformed-function.mir",
    "samples/full-system-v1/computational/malformed-function-negative/expected/parse.json",
    "samples/full-system-v1/computational/malformed-perform-negative/README.md",
    "samples/full-system-v1/computational/malformed-perform-negative/src/malformed-perform.mir",
    "samples/full-system-v1/computational/malformed-perform-negative/expected/parse.json",
    "samples/full-system-v1/computational/unresolved-import-negative/README.md",
    "samples/full-system-v1/computational/unresolved-import-negative/src/unresolved-import.mir",
    "samples/full-system-v1/computational/unresolved-import-negative/expected/parse.json",
    "samples/full-system-v1/computational/missing-type-annotation-negative/README.md",
    "samples/full-system-v1/computational/missing-type-annotation-negative/src/missing-type-annotation.mir",
    "samples/full-system-v1/computational/missing-type-annotation-negative/expected/parse.json",
    "samples/full-system-v1/computational/malformed-record-negative/README.md",
    "samples/full-system-v1/computational/malformed-record-negative/src/malformed-record.mir",
    "samples/full-system-v1/computational/malformed-record-negative/expected/parse.json",
    "samples/full-system-v1/computational/malformed-transition-negative/README.md",
    "samples/full-system-v1/computational/malformed-transition-negative/src/malformed-transition.mir",
    "samples/full-system-v1/computational/malformed-transition-negative/expected/parse.json",
    "samples/full-system-v1/computational/malformed-capability-negative/README.md",
    "samples/full-system-v1/computational/malformed-capability-negative/src/malformed-capability.mir",
    "samples/full-system-v1/computational/malformed-capability-negative/expected/parse.json",
    "samples/full-system-v1/computational/contract-clause-position-negative/README.md",
    "samples/full-system-v1/computational/contract-clause-position-negative/src/contract-clause-position.mir",
    "samples/full-system-v1/computational/contract-clause-position-negative/expected/parse.json",
    "samples/full-system-v1/world-core/README.md",
    "samples/full-system-v1/world-core/matrix.json",
    "samples/full-system-v1/world-core/world-bootstrap-positive/README.md",
    "samples/full-system-v1/world-core/world-bootstrap-positive/main/src/world-bootstrap-positive.mir",
    "samples/full-system-v1/world-core/world-bootstrap-positive/expected/manifest.json",
    "samples/full-system-v1/world-core/world-bootstrap-positive/expected/run.json",
    "samples/full-system-v1/world-core/world-observe-before-bootstrap-negative/README.md",
    "samples/full-system-v1/world-core/world-observe-before-bootstrap-negative/main/src/world-observe-before-bootstrap-negative.mir",
    "samples/full-system-v1/world-core/world-observe-before-bootstrap-negative/expected/manifest.json",
    "samples/full-system-v1/world-core/world-observe-before-bootstrap-negative/expected/run.json",
    "samples/full-system-v1/membership-chat/README.md",
    "samples/full-system-v1/membership-chat/matrix.json",
    "samples/full-system-v1/membership-chat/chat-room-message-positive/README.md",
    "samples/full-system-v1/membership-chat/chat-room-message-positive/main/src/chat-room-message-positive.mir",
    "samples/full-system-v1/membership-chat/chat-room-message-positive/expected/manifest.json",
    "samples/full-system-v1/membership-chat/chat-room-message-positive/expected/run.json",
    "samples/full-system-v1/membership-chat/chat-stale-membership-negative/README.md",
    "samples/full-system-v1/membership-chat/chat-stale-membership-negative/main/src/chat-stale-membership-negative.mir",
    "samples/full-system-v1/membership-chat/chat-stale-membership-negative/expected/manifest.json",
    "samples/full-system-v1/membership-chat/chat-stale-membership-negative/expected/run.json",
    "samples/full-system-v1/sugoroku-world/README.md",
    "samples/full-system-v1/sugoroku-world/matrix.json",
    "samples/full-system-v1/sugoroku-world/sugoroku-turn-positive/README.md",
    "samples/full-system-v1/sugoroku-world/sugoroku-turn-positive/main/src/sugoroku-turn-positive.mir",
    "samples/full-system-v1/sugoroku-world/sugoroku-turn-positive/expected/manifest.json",
        "samples/full-system-v1/sugoroku-world/sugoroku-turn-positive/expected/run.json",
        "samples/full-system-v1/sugoroku-world/sugoroku-stale-membership-negative/README.md",
        "samples/full-system-v1/sugoroku-world/sugoroku-stale-membership-negative/main/src/sugoroku-stale-membership-negative.mir",
        "samples/full-system-v1/sugoroku-world/sugoroku-stale-membership-negative/expected/manifest.json",
        "samples/full-system-v1/sugoroku-world/sugoroku-stale-membership-negative/expected/run.json",
        "samples/full-system-v1/portal-worldlink/README.md",
        "samples/full-system-v1/portal-worldlink/matrix.json",
        "samples/full-system-v1/portal-worldlink/shared/src/world-core-base.mir",
        "samples/full-system-v1/portal-worldlink/shared/src/membership-chat-base.mir",
        "samples/full-system-v1/portal-worldlink/shared/src/sugoroku-base.mir",
        "samples/full-system-v1/portal-worldlink/shared/src/portal-worldlink-base.mir",
        "samples/full-system-v1/portal-worldlink/portal-handoff-positive/README.md",
        "samples/full-system-v1/portal-worldlink/portal-handoff-positive/main/src/portal-handoff-positive.mir",
        "samples/full-system-v1/portal-worldlink/portal-handoff-positive/expected/manifest.json",
        "samples/full-system-v1/portal-worldlink/portal-handoff-positive/expected/run.json",
        "samples/full-system-v1/portal-worldlink/portal-admission-denied-negative/README.md",
        "samples/full-system-v1/portal-worldlink/portal-admission-denied-negative/main/src/portal-admission-denied-negative.mir",
        "samples/full-system-v1/portal-worldlink/portal-admission-denied-negative/expected/manifest.json",
        "samples/full-system-v1/portal-worldlink/portal-admission-denied-negative/expected/run.json",
        "samples/full-system-v1/two-shard-hard-boundary/README.md",
        "samples/full-system-v1/two-shard-hard-boundary/matrix.json",
        "samples/full-system-v1/two-shard-hard-boundary/shared/src/world-core-base.mir",
        "samples/full-system-v1/two-shard-hard-boundary/shared/src/membership-chat-base.mir",
        "samples/full-system-v1/two-shard-hard-boundary/shared/src/sugoroku-base.mir",
        "samples/full-system-v1/two-shard-hard-boundary/shared/src/portal-worldlink-base.mir",
        "samples/full-system-v1/two-shard-hard-boundary/shared/src/shard-boundary-base.mir",
        "samples/full-system-v1/two-shard-hard-boundary/shard-handoff-positive/README.md",
        "samples/full-system-v1/two-shard-hard-boundary/shard-handoff-positive/main/src/shard-handoff-positive.mir",
        "samples/full-system-v1/two-shard-hard-boundary/shard-handoff-positive/expected/manifest.json",
        "samples/full-system-v1/two-shard-hard-boundary/shard-handoff-positive/expected/run.json",
        "samples/full-system-v1/two-shard-hard-boundary/shard-missing-witness-negative/README.md",
        "samples/full-system-v1/two-shard-hard-boundary/shard-missing-witness-negative/main/src/shard-missing-witness-negative.mir",
        "samples/full-system-v1/two-shard-hard-boundary/shard-missing-witness-negative/expected/manifest.json",
        "samples/full-system-v1/two-shard-hard-boundary/shard-missing-witness-negative/expected/run.json",
        "samples/full-system-v1/gradient-observation/README.md",
        "samples/full-system-v1/gradient-observation/matrix.json",
        "samples/full-system-v1/gradient-observation/shared/src/world-core-base.mir",
        "samples/full-system-v1/gradient-observation/shared/src/membership-chat-base.mir",
        "samples/full-system-v1/gradient-observation/shared/src/sugoroku-base.mir",
        "samples/full-system-v1/gradient-observation/shared/src/portal-worldlink-base.mir",
        "samples/full-system-v1/gradient-observation/shared/src/shard-boundary-base.mir",
        "samples/full-system-v1/gradient-observation/shared/src/gradient-observation-base.mir",
        "samples/full-system-v1/gradient-observation/gradient-observe-positive/README.md",
        "samples/full-system-v1/gradient-observation/gradient-observe-positive/main/src/gradient-observe-positive.mir",
        "samples/full-system-v1/gradient-observation/gradient-observe-positive/expected/manifest.json",
        "samples/full-system-v1/gradient-observation/gradient-observe-positive/expected/run.json",
    "samples/full-system-v1/gradient-observation/gradient-write-reject-negative/README.md",
    "samples/full-system-v1/gradient-observation/gradient-write-reject-negative/main/src/gradient-write-reject-negative.mir",
    "samples/full-system-v1/gradient-observation/gradient-write-reject-negative/expected/manifest.json",
    "samples/full-system-v1/gradient-observation/gradient-write-reject-negative/expected/run.json",
    "samples/full-system-v1/projection/effectful-sugoroku-positive/main/src/effectful-sugoroku-positive.mir",
    "samples/full-system-v1/projection/effectful-sugoroku-positive/projection.request.json",
    "samples/full-system-v1/server-client/role-split-positive/main/src/role-split-positive.mir",
    "samples/full-system-v1/server-client/role-split-positive/projection.request.json",
    "samples/full-system-v1/provider-adapter/viewer-diagnostic-positive/main/src/viewer-diagnostic-positive.mir",
    "samples/full-system-v1/provider-adapter/viewer-diagnostic-positive/projection.request.json",
    "samples/full-system-v1/provider-adapter/viewer-diagnostic-positive/provider.manifest.json",
    "samples/full-system-v1/provider-adapter/renderer-pose-positive/main/src/renderer-pose-positive.mir",
    "samples/full-system-v1/provider-adapter/renderer-pose-positive/projection.request.json",
    "samples/full-system-v1/provider-adapter/renderer-pose-positive/provider.manifest.json",
    "samples/full-system-v1/provider-adapter/renderer-pose-positive/package.mir.json",
    "samples/full-system-v1-surface/README.md",
    "samples/full-system-v1-surface/syntax/README.md",
    "samples/full-system-v1-surface/syntax/matrix.json",
    "samples/full-system-v1-surface/syntax/surf-01-brace-place-positive/main/src/brace-place-positive.mir",
    "samples/full-system-v1-surface/syntax/surf-01-brace-place-positive/expected/parse.json",
    "samples/full-system-v1-surface/syntax/surf-02-bracket-place-negative/main/src/bracket-place-negative.mir",
    "samples/full-system-v1-surface/syntax/surf-02-bracket-place-negative/expected/parse.json",
    "samples/full-system-v1-surface/syntax/surf-03-record-literal-positive/main/src/record-literal-positive.mir",
    "samples/full-system-v1-surface/syntax/surf-03-record-literal-positive/expected/parse.json",
    "samples/full-system-v1-surface/syntax/surf-04-ambiguous-brace-negative/main/src/ambiguous-brace-negative.mir",
    "samples/full-system-v1-surface/syntax/surf-04-ambiguous-brace-negative/expected/parse.json",
    "samples/full-system-v1-surface/syntax/surf-05-role-instance-positive/main/src/role-instance-positive.mir",
    "samples/full-system-v1-surface/syntax/surf-05-role-instance-positive/expected/parse.json",
    "samples/full-system-v1-surface/syntax/surf-06-undeclared-place-negative/main/src/undeclared-place-negative.mir",
    "samples/full-system-v1-surface/syntax/surf-06-undeclared-place-negative/expected/parse.json",
    "samples/full-system-v1-surface/syntax/surf-07-undeclared-role-negative/main/src/undeclared-role-negative.mir",
    "samples/full-system-v1-surface/syntax/surf-07-undeclared-role-negative/expected/parse.json",
    "samples/full-system-v1-surface/syntax/surf-08-invalid-role-binder-negative/main/src/invalid-role-binder-negative.mir",
    "samples/full-system-v1-surface/syntax/surf-08-invalid-role-binder-negative/expected/parse.json",
    "samples/full-system-v1-surface/syntax/surf-09-role-named-s-positive/main/src/role-named-s-positive.mir",
    "samples/full-system-v1-surface/syntax/surf-09-role-named-s-positive/expected/parse.json",
    "samples/full-system-v1-surface/indexed-state/README.md",
    "samples/full-system-v1-surface/indexed-state/matrix.json",
    "samples/full-system-v1-surface/indexed-state/idx-01-owner-map-positive/main/src/owner-map-positive.mir",
    "samples/full-system-v1-surface/indexed-state/idx-01-owner-map-positive/expected/indexed_state.json",
    "samples/full-system-v1-surface/indexed-state/idx-02-key-authority-negative/main/src/key-authority-negative.mir",
    "samples/full-system-v1-surface/indexed-state/idx-02-key-authority-negative/expected/indexed_state.json",
    "samples/full-system-v1-surface/indexed-state/idx-03-stale-key-negative/main/src/stale-key-negative.mir",
    "samples/full-system-v1-surface/indexed-state/idx-03-stale-key-negative/expected/indexed_state.json",
    "samples/full-system-v1-surface/indexed-state/idx-04-compaction-retained-negative/main/src/compaction-retained-negative.mir",
    "samples/full-system-v1-surface/indexed-state/idx-04-compaction-retained-negative/expected/indexed_state.json",
    "samples/full-system-v1-surface/indexed-state/idx-05-nested-place-authority-negative/README.md",
    "samples/full-system-v1-surface/indexed-state/idx-05-nested-place-authority-negative/main/src/nested-place-authority-negative.mir",
    "samples/full-system-v1-surface/indexed-state/idx-05-nested-place-authority-negative/expected/indexed_state.json",
    "samples/full-system-v1-surface/elaboration/README.md",
    "samples/full-system-v1-surface/elaboration/matrix.json",
    "samples/full-system-v1-surface/elaboration/elab-01-cross-place-read-positive/README.md",
    "samples/full-system-v1-surface/elaboration/elab-01-cross-place-read-positive/main/src/cross-place-read-positive.mir",
    "samples/full-system-v1-surface/elaboration/elab-01-cross-place-read-positive/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-02-cross-place-write-positive/README.md",
    "samples/full-system-v1-surface/elaboration/elab-02-cross-place-write-positive/main/src/cross-place-write-positive.mir",
    "samples/full-system-v1-surface/elaboration/elab-02-cross-place-write-positive/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-03-private-field-auto-publish-negative/README.md",
    "samples/full-system-v1-surface/elaboration/elab-03-private-field-auto-publish-negative/main/src/private-field-auto-publish-negative.mir",
    "samples/full-system-v1-surface/elaboration/elab-03-private-field-auto-publish-negative/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/README.md",
    "samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/main/src/undeclared-generated-failure-negative.mir",
    "samples/full-system-v1-surface/elaboration/elab-04-undeclared-generated-failure-negative/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-05-source-spans-positive/README.md",
    "samples/full-system-v1-surface/elaboration/elab-05-source-spans-positive/main/src/source-spans-positive.mir",
    "samples/full-system-v1-surface/elaboration/elab-05-source-spans-positive/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-06-unsupported-statement-negative/README.md",
    "samples/full-system-v1-surface/elaboration/elab-06-unsupported-statement-negative/main/src/unsupported-statement-negative.mir",
    "samples/full-system-v1-surface/elaboration/elab-06-unsupported-statement-negative/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/README.md",
    "samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/main/src/write-failure-row-negative.mir",
    "samples/full-system-v1-surface/elaboration/elab-07-write-failure-row-negative/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-08-nested-place-read-positive/README.md",
    "samples/full-system-v1-surface/elaboration/elab-08-nested-place-read-positive/main/src/nested-place-read-positive.mir",
    "samples/full-system-v1-surface/elaboration/elab-08-nested-place-read-positive/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-09-visible-write-auto-communication-positive/README.md",
    "samples/full-system-v1-surface/elaboration/elab-09-visible-write-auto-communication-positive/main/src/visible-write-auto-communication-positive.mir",
    "samples/full-system-v1-surface/elaboration/elab-09-visible-write-auto-communication-positive/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/README.md",
    "samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/main/src/visibility-failure-row-negative.mir",
    "samples/full-system-v1-surface/elaboration/elab-10-visibility-failure-row-negative/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-11-scn01-rhs-dependency-positive/README.md",
    "samples/full-system-v1-surface/elaboration/elab-11-scn01-rhs-dependency-positive/main/src/scn01-rhs-dependency-positive.mir",
    "samples/full-system-v1-surface/elaboration/elab-11-scn01-rhs-dependency-positive/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-12-scn02-two-read-dependency-positive/README.md",
    "samples/full-system-v1-surface/elaboration/elab-12-scn02-two-read-dependency-positive/main/src/scn02-two-read-dependency-positive.mir",
    "samples/full-system-v1-surface/elaboration/elab-12-scn02-two-read-dependency-positive/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-13-non-visibility-singleton-failure-row-negative/README.md",
    "samples/full-system-v1-surface/elaboration/elab-13-non-visibility-singleton-failure-row-negative/main/src/non-visibility-singleton-failure-row-negative.mir",
    "samples/full-system-v1-surface/elaboration/elab-13-non-visibility-singleton-failure-row-negative/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-14-missing-capability-singleton-failure-row-negative/README.md",
    "samples/full-system-v1-surface/elaboration/elab-14-missing-capability-singleton-failure-row-negative/main/src/missing-capability-singleton-failure-row-negative.mir",
    "samples/full-system-v1-surface/elaboration/elab-14-missing-capability-singleton-failure-row-negative/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-15-route-unavailable-singleton-failure-row-negative/README.md",
    "samples/full-system-v1-surface/elaboration/elab-15-route-unavailable-singleton-failure-row-negative/main/src/route-unavailable-singleton-failure-row-negative.mir",
    "samples/full-system-v1-surface/elaboration/elab-15-route-unavailable-singleton-failure-row-negative/expected/elaboration.json",
    "samples/full-system-v1-surface/elaboration/elab-16-stale-membership-singleton-failure-row-negative/README.md",
    "samples/full-system-v1-surface/elaboration/elab-16-stale-membership-singleton-failure-row-negative/main/src/stale-membership-singleton-failure-row-negative.mir",
    "samples/full-system-v1-surface/elaboration/elab-16-stale-membership-singleton-failure-row-negative/expected/elaboration.json",
    "samples/full-system-v1-surface/role-admission/README.md",
    "samples/full-system-v1-surface/role-admission/matrix.json",
    "samples/full-system-v1-surface/role-admission/role-01-join-admission-positive/README.md",
    "samples/full-system-v1-surface/role-admission/role-01-join-admission-positive/main/src/join-admission-positive.mir",
    "samples/full-system-v1-surface/role-admission/role-01-join-admission-positive/expected/role_admission.json",
    "samples/full-system-v1-surface/role-admission/role-02-role-claim-without-grant-negative/README.md",
    "samples/full-system-v1-surface/role-admission/role-02-role-claim-without-grant-negative/main/src/role-claim-without-grant-negative.mir",
    "samples/full-system-v1-surface/role-admission/role-02-role-claim-without-grant-negative/expected/role_admission.json",
    "samples/full-system-v1-surface/role-admission/role-03-stale-membership-negative/README.md",
    "samples/full-system-v1-surface/role-admission/role-03-stale-membership-negative/main/src/stale-membership-negative.mir",
    "samples/full-system-v1-surface/role-admission/role-03-stale-membership-negative/expected/role_admission.json",
    "samples/full-system-v1-surface/role-admission/role-04-hash-binding-metadata-positive/README.md",
    "samples/full-system-v1-surface/role-admission/role-04-hash-binding-metadata-positive/main/src/hash-binding-metadata-positive.mir",
    "samples/full-system-v1-surface/role-admission/role-04-hash-binding-metadata-positive/expected/role_admission.json",
    "samples/full-system-v1-surface/source-patch/README.md",
    "samples/full-system-v1-surface/source-patch/matrix.json",
    "samples/full-system-v1-surface/source-patch/patch-01-visible-state-positive/README.md",
    "samples/full-system-v1-surface/source-patch/patch-01-visible-state-positive/main/src/visible-state-positive.mir",
    "samples/full-system-v1-surface/source-patch/patch-01-visible-state-positive/expected/source_patch.json",
    "samples/full-system-v1-surface/source-patch/patch-02-undeclared-failure-negative/README.md",
    "samples/full-system-v1-surface/source-patch/patch-02-undeclared-failure-negative/main/src/undeclared-failure-negative.mir",
    "samples/full-system-v1-surface/source-patch/patch-02-undeclared-failure-negative/expected/source_patch.json",
    "samples/full-system-v1-surface/source-patch/patch-03-self-grant-negative/README.md",
    "samples/full-system-v1-surface/source-patch/patch-03-self-grant-negative/main/src/self-grant-negative.mir",
    "samples/full-system-v1-surface/source-patch/patch-03-self-grant-negative/expected/source_patch.json",
    "samples/full-system-v1-surface/source-patch/patch-04-lifecycle-devtools-positive/README.md",
    "samples/full-system-v1-surface/source-patch/patch-04-lifecycle-devtools-positive/main/src/lifecycle-devtools-positive.mir",
    "samples/full-system-v1-surface/source-patch/patch-04-lifecycle-devtools-positive/expected/source_patch.json",
    "samples/full-system-v1-surface/devtools/README.md",
    "samples/full-system-v1-surface/devtools/matrix.json",
    "samples/full-system-v1-surface/devtools/dev-01-source-core-panels-positive/README.md",
    "samples/full-system-v1-surface/devtools/dev-01-source-core-panels-positive/main/src/devtools-panels-positive.mir",
    "samples/full-system-v1-surface/devtools/dev-01-source-core-panels-positive/patch/src/devtools-patch-positive.mir",
    "samples/full-system-v1-surface/devtools/dev-01-source-core-panels-positive/expected/devtools.json",
    "samples/full-system-v1-surface/devtools/dev-02-private-source-diagnostics-negative/README.md",
    "samples/full-system-v1-surface/devtools/dev-02-private-source-diagnostics-negative/main/src/private-source-diagnostics-negative.mir",
    "samples/full-system-v1-surface/devtools/dev-02-private-source-diagnostics-negative/expected/devtools.json",
    "samples/full-system-v1-surface/operational-matrix.json",
    "samples/full-system-v1-surface/world-core/README.md",
    "samples/full-system-v1-surface/world-core/e2e-01-world-core-positive/README.md",
    "samples/full-system-v1-surface/world-core/e2e-01-world-core-positive/main/src/world-core-positive.mir",
    "samples/full-system-v1-surface/world-core/e2e-01-world-core-positive/expected/operational.json",
    "samples/full-system-v1-surface/world-core/e2e-02-world-core-key-authority-negative/README.md",
    "samples/full-system-v1-surface/world-core/e2e-02-world-core-key-authority-negative/main/src/world-core-key-authority-negative.mir",
    "samples/full-system-v1-surface/world-core/e2e-02-world-core-key-authority-negative/expected/operational.json",
    "samples/full-system-v1-surface/membership-chat/README.md",
    "samples/full-system-v1-surface/membership-chat/e2e-03-membership-chat-positive/README.md",
    "samples/full-system-v1-surface/membership-chat/e2e-03-membership-chat-positive/main/src/membership-chat-positive.mir",
    "samples/full-system-v1-surface/membership-chat/e2e-03-membership-chat-positive/expected/operational.json",
    "samples/full-system-v1-surface/membership-chat/e2e-04-membership-chat-without-grant-negative/README.md",
    "samples/full-system-v1-surface/membership-chat/e2e-04-membership-chat-without-grant-negative/main/src/membership-chat-without-grant-negative.mir",
    "samples/full-system-v1-surface/membership-chat/e2e-04-membership-chat-without-grant-negative/expected/operational.json",
    "samples/full-system-v1-surface/sugoroku-world/README.md",
    "samples/full-system-v1-surface/sugoroku-world/e2e-05-sugoroku-positive/README.md",
    "samples/full-system-v1-surface/sugoroku-world/e2e-05-sugoroku-positive/main/src/sugoroku-positive.mir",
    "samples/full-system-v1-surface/sugoroku-world/e2e-05-sugoroku-positive/expected/operational.json",
    "samples/full-system-v1-surface/sugoroku-world/e2e-06-sugoroku-failure-row-negative/README.md",
    "samples/full-system-v1-surface/sugoroku-world/e2e-06-sugoroku-failure-row-negative/main/src/sugoroku-failure-row-negative.mir",
    "samples/full-system-v1-surface/sugoroku-world/e2e-06-sugoroku-failure-row-negative/expected/operational.json",
    "samples/full-system-v1-surface/portal-worldlink/README.md",
    "samples/full-system-v1-surface/portal-worldlink/e2e-07-portal-worldlink-positive/README.md",
    "samples/full-system-v1-surface/portal-worldlink/e2e-07-portal-worldlink-positive/main/src/portal-worldlink-positive.mir",
    "samples/full-system-v1-surface/portal-worldlink/e2e-07-portal-worldlink-positive/expected/operational.json",
    "samples/full-system-v1-surface/portal-worldlink/e2e-08-portal-private-link-negative/README.md",
    "samples/full-system-v1-surface/portal-worldlink/e2e-08-portal-private-link-negative/main/src/portal-private-link-negative.mir",
    "samples/full-system-v1-surface/portal-worldlink/e2e-08-portal-private-link-negative/expected/operational.json",
    "samples/full-system-v1-surface/two-shard-hard-boundary/README.md",
    "samples/full-system-v1-surface/two-shard-hard-boundary/e2e-09-two-shard-positive/README.md",
    "samples/full-system-v1-surface/two-shard-hard-boundary/e2e-09-two-shard-positive/main/src/two-shard-positive.mir",
    "samples/full-system-v1-surface/two-shard-hard-boundary/e2e-09-two-shard-positive/expected/operational.json",
    "samples/full-system-v1-surface/two-shard-hard-boundary/e2e-10-two-shard-failure-row-negative/README.md",
    "samples/full-system-v1-surface/two-shard-hard-boundary/e2e-10-two-shard-failure-row-negative/main/src/two-shard-failure-row-negative.mir",
    "samples/full-system-v1-surface/two-shard-hard-boundary/e2e-10-two-shard-failure-row-negative/expected/operational.json",
    "samples/full-system-v1-surface/gradient-observation/README.md",
    "samples/full-system-v1-surface/gradient-observation/e2e-11-gradient-observation-positive/README.md",
    "samples/full-system-v1-surface/gradient-observation/e2e-11-gradient-observation-positive/main/src/gradient-observation-positive.mir",
    "samples/full-system-v1-surface/gradient-observation/e2e-11-gradient-observation-positive/expected/operational.json",
    "samples/full-system-v1-surface/gradient-observation/e2e-12-gradient-private-negative/README.md",
    "samples/full-system-v1-surface/gradient-observation/e2e-12-gradient-private-negative/main/src/gradient-private-negative.mir",
    "samples/full-system-v1-surface/gradient-observation/e2e-12-gradient-private-negative/expected/operational.json",
    "samples/alpha/README.md",
    "samples/product-alpha1/README.md",
    "samples/product-alpha1/demo/README.md",
    "samples/product-alpha1/demo/package.mir.json",
    "samples/product-alpha1/operational/README.md",
    "samples/product-alpha1/operational/world-core/README.md",
    "samples/product-alpha1/operational/world-core/package.mir.json",
    "samples/product-alpha1/operational/membership-chat/README.md",
    "samples/product-alpha1/operational/membership-chat/package.mir.json",
    "samples/product-alpha1/operational/sugoroku-world/README.md",
    "samples/product-alpha1/operational/sugoroku-world/package.mir.json",
    "samples/product-alpha1/demo/packages/debug-layer/package.mir.json",
    "samples/product-alpha1/demo/packages/auth-layer/package.mir.json",
    "samples/product-alpha1/demo/packages/rate-limit-layer/package.mir.json",
    "samples/product-alpha1/demo/packages/placeholder-object/package.mir.json",
    "samples/product-alpha1/demo/packages/custom-avatar-preview/package.mir.json",
    "samples/product-alpha1/docker/README.md",
    "samples/product-alpha1/docker/docker-compose.product-alpha1.yml",
    "samples/product-alpha1/computational/README.md",
    "samples/product-alpha1/computational/matrix.json",
    "samples/product-alpha1/computational/add-one-pure-mir/README.md",
    "samples/product-alpha1/computational/add-one-pure-mir/add-one-pure-mir.mir",
    "samples/product-alpha1/computational/variables-scope/README.md",
    "samples/product-alpha1/computational/variables-scope/variables-scope.mir",
    "samples/product-alpha1/computational/arrays-bounds/README.md",
    "samples/product-alpha1/computational/arrays-bounds/arrays-bounds.mir",
    "samples/product-alpha1/computational/records-vec3/README.md",
    "samples/product-alpha1/computational/records-vec3/records-vec3.mir",
    "samples/product-alpha1/computational/control-flow/README.md",
    "samples/product-alpha1/computational/control-flow/control-flow.mir",
    "samples/product-alpha1/computational/imports-functions/README.md",
    "samples/product-alpha1/computational/imports-functions/imports-functions.mir",
    "samples/product-alpha1/computational/host-io-internal-transform/README.md",
    "samples/product-alpha1/computational/host-io-internal-transform/host-io-internal-transform.mir",
    "samples/product-alpha1/posegraph/README.md",
    "samples/product-alpha1/posegraph/matrix.json",
    "samples/product-alpha1/posegraph/avatar-head-transform/README.md",
    "samples/product-alpha1/posegraph/avatar-head-transform/avatar-head-transform.mir",
    "samples/product-alpha1/posegraph/anchored-object/README.md",
    "samples/product-alpha1/posegraph/anchored-object/anchored-object.mir",
    "samples/product-alpha1/posegraph/sparkle-fallback-anchor/README.md",
    "samples/product-alpha1/posegraph/sparkle-fallback-anchor/sparkle-fallback-anchor.mir",
    "samples/product-alpha1/posegraph/no-split-frame-positive/README.md",
    "samples/product-alpha1/posegraph/no-split-frame-positive/no-split-frame-positive.mir",
    "samples/product-alpha1/posegraph/split-frame-negative/README.md",
    "samples/product-alpha1/posegraph/split-frame-negative/split-frame-negative.mir",
    "samples/product-alpha1/posegraph/save-load-roundtrip/README.md",
    "samples/product-alpha1/posegraph/save-load-roundtrip/save-load-roundtrip.mir",
    "samples/product-alpha1/posegraph/stale-anchor-after-membership-advance/README.md",
    "samples/product-alpha1/posegraph/stale-anchor-after-membership-advance/stale-anchor-after-membership-advance.mir",
    "samples/product-alpha1/posegraph/anchor-switch-frontier-negative/README.md",
    "samples/product-alpha1/posegraph/anchor-switch-frontier-negative/anchor-switch-frontier-negative.mir",
    "samples/product-alpha1/posegraph/stale-anchor-reacquire-required/README.md",
    "samples/product-alpha1/posegraph/stale-anchor-reacquire-required/stale-anchor-reacquire-required.mir",
    "samples/product-alpha1/projection/README.md",
    "samples/product-alpha1/projection/matrix.json",
    "samples/product-alpha1/projection/server-client-target-manifest/server-client-target-manifest.json",
    "samples/product-alpha1/projection/packet-boundary-schema/packet-boundary-schema.json",
    "samples/product-alpha1/projection/ffi-boundary-schema/ffi-boundary-schema.json",
    "samples/product-alpha1/projection/manifest-provider-compatibility/manifest-provider-compatibility.json",
    "samples/product-alpha1/engine-adapter/README.md",
    "samples/product-alpha1/engine-adapter/matrix.json",
    "samples/product-alpha1/engine-adapter/renderer/renderer.contract.json",
    "samples/product-alpha1/engine-adapter/input-device/input-device.contract.json",
    "samples/product-alpha1/engine-adapter/asset-loader/asset-loader.contract.json",
    "samples/product-alpha1/engine-adapter/physics-spatial-query/physics-spatial-query.contract.json",
    "samples/product-alpha1/engine-adapter/host-runtime-bridge/host-runtime-bridge.contract.json",
    "samples/product-alpha1/engine-adapter/wasm-sandbox/wasm-sandbox.contract.json",
    "samples/product-alpha1/engine-adapter/native-library-bridge/native-library-bridge.contract.json",
    "samples/product-alpha1/engine-adapter/viewer-diagnostic-exporter/viewer-diagnostic-exporter.contract.json",
    "docs/hands_on/README.md",
    "docs/hands_on/product_alpha1_01.md",
    "docs/hands_on/operational_product_sample_01.md",
    "docs/hands_on/mir_computational_core_01.md",
    "docs/hands_on/transform_posegraph_01.md",
    "docs/hands_on/autonomous_execution_01.md",
    "docs/hands_on/full_system_v1_roadmap_01.md",
    "docs/hands_on/surface_mir_alpha_01.md",
    "docs/hands_on/source_patch_hotplug_01.md",
    "docs/research_abstract/README.md",
    "docs/research_abstract/product_alpha1_01.md",
    "docs/research_abstract/operational_product_sample_01.md",
    "docs/research_abstract/mir_computational_core_01.md",
    "docs/research_abstract/autonomous_execution_01.md",
    "docs/research_abstract/full_system_v1_roadmap_01.md",
    "docs/research_abstract/surface_mir_alpha_01.md",
    "scripts/textual_mir_samples.py",
    "scripts/mir_computational_samples.py",
    "scripts/posegraph_samples.py",
    "scripts/projection_boundary_samples.py",
    "scripts/engine_adapter_boundary_samples.py",
    "scripts/product_alpha1_release_check.py",
    "scripts/operational_product_samples.py",
    "scripts/full_system_v1_release_check.py",
    "scripts/surface_mir_samples.py",
    "scripts/surface_mir_release_check.py",
    "scripts/surface_mir_authoring_check.py",
    "scripts/tests/test_mir_computational_samples.py",
    "scripts/tests/test_posegraph_samples.py",
    "scripts/tests/test_projection_boundary_samples.py",
    "scripts/tests/test_engine_adapter_boundary_samples.py",
    "scripts/tests/test_product_alpha1_release_check.py",
    "scripts/tests/test_operational_product_samples.py",
    "scripts/tests/test_textual_mir_samples.py",
    "scripts/tests/test_full_system_v1_release_check.py",
    "scripts/tests/test_surface_mir_samples.py",
    "scripts/tests/test_surface_mir_release_check.py",
    "samples/practical-alpha1/README.md",
    "samples/practical-alpha1/packages/README.md",
    "samples/practical-alpha1/source/README.md",
    "samples/practical-alpha1/expected/README.md",
    "samples/practical-alpha1/docker/README.md",
    "samples/alpha/lifetime-fallback/README.md",
    "samples/alpha/contract-variance/README.md",
    "samples/alpha/cut-save-load/README.md",
    "samples/alpha/local-runtime/README.md",
    "samples/alpha/layer-insertion/README.md",
    "samples/alpha/network-docker/README.md",
    "samples/alpha/hotplug-runtime/README.md",
    "samples/alpha/avatar-runtime/README.md",
    "samples/alpha/visualization/README.md",
    "samples/alpha/e2e/README.md",
    "samples/not_implemented/README.md",
    "scripts/README.md",
    "plan/00-index.md",
    "plan/01-status-at-a-glance.md",
    "plan/02-system-overview-and-positioning.md",
    "plan/03-decision-strengths-and-boundaries.md",
    "plan/04-core-semantics-current-l2.md",
    "plan/05-fallback-lease-and-chain-semantics.md",
    "plan/06-surface-notation-status.md",
    "plan/07-parser-free-poc-stack.md",
    "plan/08-representative-programs-and-fixtures.md",
    "plan/09-helper-stack-and-responsibility-map.md",
    "plan/10-roadmap-overall.md",
    "plan/11-roadmap-near-term.md",
    "plan/12-open-problems-and-risks.md",
    "plan/13-heavy-future-workstreams.md",
    "plan/14-glossary-and-boundary-rules.md",
    "plan/15-current-l2-fixture-authoring-template.md",
    "plan/16-shared-space-membership-and-example-boundary.md",
    "plan/17-research-phases-and-autonomy-gates.md",
    "plan/18-type-proof-modelcheck-and-ordering-research-program.md",
    "plan/19-repository-map-and-taxonomy.md",
    "plan/20-projection-and-placement-roadmap.md",
    "plan/21-hotplug-attachpoint-roadmap.md",
    "plan/22-network-transport-roadmap.md",
    "plan/23-compiler-backend-llvm-guardrail-roadmap.md",
    "plan/24-avatar-follow-representative-slice-roadmap.md",
    "plan/25-typed-external-boundary-executable-roadmap.md",
    "plan/26-visual-debugger-viewer-roadmap.md",
    "plan/27-public-api-parser-gate-roadmap.md",
    "plan/28-post-p18-true-user-spec-hold-option-matrix.md",
    "plan/29-verification-layer-widening-threshold.md",
    "plan/30-attachpoint-detach-minimal-contract.md",
    "plan/31-fairy05-visibility-return-carrier-bundling.md",
    "plan/32-hotplug-real-migration-rollback-boundary.md",
    "plan/33-runtime-crate-hotplug-engine-ownership-cut.md",
    "plan/34-runtime-crate-hotplug-carrier-admission-cut.md",
    "plan/35-post-p20-hotplug-next-package-inventory.md",
    "plan/36-post-p21-rollback-durable-migration-family.md",
    "plan/37-post-p21-distributed-activation-ordering-family.md",
    "plan/38-post-p21-final-public-hotplug-abi-family.md",
    "plan/39-type-system-freeze-roadmap.md",
    "plan/40-layer-compatibility-freeze-roadmap.md",
    "plan/41-save-load-checkpoint-roadmap.md",
    "plan/42-runtime-package-avatar-roadmap.md",
    "plan/43-alpha-e2e-roadmap.md",
    "plan/44-practical-alpha1-roadmap.md",
    "plan/45-operational-alpha05-roadmap.md",
    "plan/46-operational-alpha08-roadmap.md",
    "plan/47-operational-alpha09-devtools-roadmap.md",
    "plan/48-theory-freeze-proof-obligations.md",
    "plan/49-host-io-and-session-runtime-roadmap.md",
    "plan/50-product-alpha1-public-boundary-roadmap.md",
    "plan/51-operational-product-sample-roadmap.md",
    "plan/52-portal-spatial-world-roadmap.md",
    "plan/53-mir-computational-core-roadmap.md",
    "plan/54-transform-posegraph-roadmap.md",
    "plan/55-projection-backend-roadmap.md",
    "plan/56-engine-adapter-roadmap.md",
    "plan/57-autonomous-computational-core-master-plan.md",
    "plan/58-full-system-v1-roadmap.md",
    "plan/59-textual-mir-roadmap.md",
    "plan/60-computational-runtime-roadmap.md",
    "plan/61-posegraph-runtime-roadmap.md",
    "plan/62-projection-backend-roadmap.md",
    "plan/63-engine-provider-roadmap.md",
    "plan/64-surface-mir-placement-roadmap.md",
    "plan/65-indexed-state-roadmap.md",
    "plan/66-role-admission-roadmap.md",
    "plan/67-source-patch-hotplug-roadmap.md",
    "plan/68-surface-full-system-v1-roadmap.md",
    "plan/69-consultation-synthesis-and-management-roadmap.md",
    "plan/70-lab-to-canon-reconciliation-ledger.md",
    "plan/71-g1-ordinary-assignment-target.md",
    "plan/72-g1-scn01-scn02-static-consequence-drilldown.md",
    "plan/73-g1-obl001-lean-statement-inventory.md",
    "plan/74-g1-obl001-lean-statement-draft.md",
    "plan/75-g1-scn-rhs-dependency-gap-evidence.md",
    "plan/76-g1-obl020-021-dependency-inventory.md",
    "plan/77-g1-obl021-lean-statement-draft.md",
    "plan/78-g1-obl020-lean-statement-draft.md",
    "plan/79-g1-erow-diagnostic-alignment.md",
    "plan/80-g1-diagnostic-carrier-inventory.md",
    "plan/81-g1-obl024-statement-shape-inventory.md",
    "plan/82-g1-obl025-statement-shape-inventory.md",
    "plan/83-g1-erow-repair-payload-inventory.md",
    "plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md",
    "plan/85-g1-erow-carrier-precondition-hardening.md",
    "plan/86-g1-erow002-visibility-repair-carrier-prototype.md",
    "plan/87-g1-obl025-lean-statement-draft.md",
    "plan/88-g1-erow-repair-shape-inventory.md",
    "plan/89-g1-erow001-non-visibility-singleton-fixture.md",
    "plan/90-source-traceability.md",
    "plan/91-maintenance-rules.md",
    "plan/92-g1-erow001-base-singleton-fixture-closure.md",
    "plan/93-g1-erow001-singleton-repair-assumption.md",
    "plan/94-g1-erow001-singleton-repair-prototype.md",
    "plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md",
    "plan/96-g1-erow-set-insertion-bundle-payload-inventory.md",
    "plan/97-g1-erow07-set-insertion-gate-review.md",
    "plan/98-g1-erow04-mixed-visibility-branch-inventory.md",
    "plan/99-g1-erow07-set-insertion-executable-preflight.md",
    "plan/100-g1-erow07-set-insertion-assumption-acceptance.md",
    "plan/101-g1-erow07-set-insertion-payload-model-design.md",
    "plan/102-g1-erow07-set-insertion-executable-payload-prototype.md",
    "plan/103-g1-erow07-set-insertion-negative-guard-hardening.md",
    "plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md",
    "plan/105-g1-erow07-set-insertion-exact-locus-guard-hardening.md",
    "plan/106-g1-erow07-child-bundle-partial-exclusion-fixtures.md",
    "plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md",
    "plan/108-g1-obl025-branch-local-noncoverage-refinement.md",
    "plan/109-g1-obl024-lean-statement-draft.md",
    "plan/110-g1-obl024-executable-projection-carrier.md",
    "plan/111-g1-obl024-projection-rust-fixture-guards.md",
    "plan/112-g1-obl024-replay-vocabulary-preflight.md",
    "plan/113-g1-obl024-lean-replay-vocabulary-refinement.md",
    "plan/114-g1-obl024-lean-association-vocabulary-refinement.md",
    "plan/115-g1-obl024-association-guard-hardening.md",
    "plan/116-g1-obl025-repair-completeness-guard-hardening.md",
    "plan/117-g1-obl001-020-021-statement-guard-hardening.md",
    "plan/118-g0-g1-ordinary-assignment-claim-family-drilldown.md",
    "plan/119-g0-remaining-claim-family-drilldown-priority.md",
    "plan/120-repo-triage-recut-matrix.md",
    "plan/121-g1-minimal-vertical-slice-candidate-map.md",
    "plan/122-g1-scn-exact-static-slice-manifest.md",
    "plan/123-g1-scn01-visibility-negative-actualization.md",
    "plan/124-g1-obl001-boundary-audit.md",
    "plan/125-g1-scn02-direct-local-write-blocker-review.md",
    "plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md",
    "plan/127-g1-ordinary-assignment-bridge-readiness-nonreadiness-map.md",
    "plan/128-g1-bridge-handoff-blocker-ledger.md",
    "plan/129-g1-acceptance-packet-preflight.md",
    "plan/130-g1-obl-statement-status-completion-criteria-inventory.md",
    "plan/131-g1-status-proposal-packet-outline.md",
    "plan/132-g1-status-evidence-readiness-dry-run.md",
    "plan/133-g1-requested-status-options-matrix.md",
    "plan/134-g1-obl020-scope-clarification-packet.md",
    "plan/135-g1-obl020-artifact-identity-wrapper-preflight.md",
    "plan/136-g1-obl020-artifact-annex-template.md",
    "plan/137-g1-obl001-artifact-identity-wrapper-preflight.md",
    "plan/138-g1-obl001-artifact-annex-template.md",
    "plan/139-g1-obl021-artifact-identity-wrapper-preflight.md",
    "plan/140-g1-obl021-artifact-annex-template.md",
    "plan/141-g1-status-packet-shell-unresolved-slots.md",
    "plan/142-g1-status-packet-shell-evidence-dry-run.md",
    "plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md",
    "plan/144-g1-obl020-scope-decision-reuse-audit.md",
    "specs/00-document-map.md",
    "specs/01-charter-and-decision-levels.md",
    "specs/02-system-overview.md",
    "specs/03-layer-model.md",
    "specs/04-mir-core.md",
    "specs/05-mirrorea-fabric.md",
    "specs/06-prismcascade-positioning.md",
    "specs/07-typed-effects-wiring-platform.md",
    "specs/08-cross-system-relations.md",
    "specs/09-invariants-and-constraints.md",
    "specs/10-open-questions.md",
    "specs/11-roadmap-and-workstreams.md",
    "specs/12-decision-register.md",
    "specs/13-type-system-lifetime-fallback.md",
    "specs/14-contract-subtyping-layer-compatibility.md",
    "specs/15-cut-save-load-checkpoint.md",
    "specs/16-runtime-package-adapter-hotplug.md",
    "specs/17-mirrorea-spaces-alpha-scope.md",
    "specs/18-practical-alpha1-scope.md",
    "specs/19-verification-stratification.md",
    "specs/20-cut-save-load-semantics.md",
    "specs/21-auth-layer-algebra.md",
    "specs/22-observability-devtools-semantics.md",
    "specs/23-typed-external-host-boundary.md",
    "specs/24-operational-alpha05-alpha08-readiness.md",
    "specs/25-product-alpha1-public-boundary.md",
    "specs/26-operational-product-sample-suite.md",
    "specs/27-spatial-portal-and-shard-extension-boundary.md",
    "specs/28-mir-computational-core.md",
    "specs/29-transform-posegraph-semantics.md",
    "specs/30-projection-and-backend-boundary.md",
    "specs/31-engine-wasm-ffi-adapter-boundary.md",
    "specs/32-autonomous-execution-and-completion-contract.md",
    "specs/33-full-system-v1-scope.md",
    "specs/34-textual-mir-alpha-grammar.md",
    "specs/35-mir-typed-ir-and-interpreter.md",
    "specs/36-projection-ir-and-boundary-preservation.md",
    "specs/37-posegraph-runtime-semantics.md",
    "specs/38-engine-provider-admission.md",
    "specs/39-surface-mir-placement-elaboration.md",
    "specs/40-indexed-state-semantics.md",
    "specs/41-role-admission-and-capability-grant.md",
    "specs/42-source-patch-hotplug-semantics.md",
    "specs/43-surface-mir-v1-alpha-scope.md",
    ".docs/progress-task-axes.md",
    ".docs/continuous-task-policy.md",
    ".docs/current-l2-source-sample-authoring-policy.md",
    "sub-agent-pro/mirrorea_mir_computational_core_handoff.md",
    "sub-agent-pro/full-system-completion-001/20-progress-tasks-replacement-model.md",
    "mirrorea_canon/README.md",
    "mirrorea_canon/MAP.md",
    "mirrorea_canon/INDEX.json",
    "mirrorea_canon/meta/source-hierarchy.md",
    "mirrorea_canon/adr/ADR-0012.md",
    "mirrorea_canon/plan/00-gates.md",
    "mirrorea_canon/plan/01-phases.md",
    "mirrorea_canon/spec/06-conformance.md",
    "mirrorea_canon/theory/11-metatheory-ledger.md",
    "docs/reports/TEMPLATE.md",
]

REQUIRED_TEMPLATE_HEADINGS = [
    "## Objective",
    "## Scope and assumptions",
    "## Start state / dirty state",
    "## Documents consulted",
    "## Actions taken",
    "## Files changed",
    "## Commands run",
    "## Evidence / outputs / test results",
    "## What changed in understanding",
    "## Open questions",
    "## Suggested next prompt",
    "## Plan update status",
    "## Documentation.md update status",
    "## progress.md update status",
    "## tasks.md update status",
    "## samples_progress.md update status",
    "## Reviewer findings and follow-up",
    "## Skipped validations and reasons",
    "## Commit / push status",
    "## Sub-agent session close status",
]

PROGRESS_REQUIRED_HEADINGS = [
    "## document role",
    "## project axis",
    "## final ideal",
    "## current milestone position",
    "## milestone map",
    "## line snapshots",
    "### Product Alpha line",
    "### Operational Suite line",
    "### Mir Language line",
    "### PoseGraph line",
    "### Projection/Backend line",
    "### Engine/Provider line",
    "## validation floor",
    "## non-claims",
    "## user decision items vs research-discovery items",
    "## macro phase map",
    "## feature maturity rows",
    "## recent log",
]

TASKS_REQUIRED_HEADINGS = [
    "## document role",
    "## current promoted package",
    "## ordered self-driven packages",
    "## self-driven macro phase reading",
    "## user decision gates",
    "## research discovery items",
    "## maintenance tasks",
    "## non-promoted references",
]

UNRESOLVED_TEMPLATE_PLACEHOLDERS = [
    "更新不要 / 更新済み:",
]

CANON_NOTICE_FILES = [
    "README.md",
    "AGENTS.md",
    "Documentation.md",
    "progress.md",
    "tasks.md",
]

CANON_NOTICE_PHRASES = [
    "`mirrorea_canon/`",
    "Everything outside",
    "is LAB",
    "canon wins",
]

SOURCE_HIERARCHY_LINT_FILES = [
    "CANON.md",
    "README.md",
    "AGENTS.md",
    "Documentation.md",
    "progress.md",
    "tasks.md",
    "samples_progress.md",
    "samples/README.md",
    "scripts/README.md",
]

SOURCE_HIERARCHY_LINT_DIRS = [
    ".docs",
    "docs/hands_on",
    "docs/research_abstract",
    "plan",
]

SOURCE_HIERARCHY_LINT_EXCLUDED_PREFIXES = [
    "docs/research_abstract/old/",
]

STALE_SOURCE_HIERARCHY_PATTERNS = [
    re.compile(r"(?:規範判断の正本|規範正本)は\s*`(?:\.\./)*specs/"),
    re.compile(r"`(?:\.\./)*specs/`?\s*(?:を|は|が)?\s*規範正本"),
    re.compile(
        r"normative source\s+(?:is|remains)\s+`(?:\.\./)*specs/",
        re.IGNORECASE,
    ),
    re.compile(r"normative boundary:\s*`(?:\.\./)*specs/", re.IGNORECASE),
    re.compile(r"`(?:\.\./)*specs/`?\s+as\s+normative", re.IGNORECASE),
    re.compile(r"treat\s+`(?:\.\./)*specs/`?\s+as\s+normative", re.IGNORECASE),
]

STALE_SOURCE_HIERARCHY_SPLIT_START_PATTERNS = [
    re.compile(r"`(?:\.\./)*specs/`?\s*$"),
]

STALE_SOURCE_HIERARCHY_SPLIT_FOLLOWUP_PATTERNS = [
    re.compile(r"^\s*規範正本\s*$"),
]

SOURCE_HIERARCHY_LINT_ALLOWED_LINES = {
    (
        "plan/70-lab-to-canon-reconciliation-ledger.md",
        "Legacy `specs/` as current normative source",
    ),
}

SOURCE_HIERARCHY_LINT_ALLOWED_PATTERNS = [
    re.compile(
        r"\b(?:do\s+not|don't|never)\s+treat\s+`(?:\.\./)*specs/`?\s+as\s+normative",
        re.IGNORECASE,
    ),
]

ACTIVE_READER_HOST_PATH_LINT_FILES = [
    "README.md",
    "AGENTS.md",
    "Documentation.md",
    "progress.md",
    "tasks.md",
    "samples_progress.md",
    "samples/README.md",
    "samples/current-l2/README.md",
]

ACTIVE_READER_HOST_PATH_LINT_DIRS = [
    ".docs",
    "docs/hands_on",
    "docs/research_abstract",
    "samples/alpha",
    "samples/clean-near-end",
    "samples/current-l2",
    "samples/full-system-v1",
    "samples/full-system-v1-surface",
    "samples/lean",
    "samples/practical-alpha1",
    "samples/product-alpha1",
]

ACTIVE_READER_HOST_PATH_LINT_EXCLUDED_PREFIXES = [
    "docs/research_abstract/old/",
    "samples/lean/old/",
    "tmp_faq/",
]

HOST_SPECIFIC_REPO_PATH_PATTERNS = [
    re.compile(r"/home/[^/\s]+/dev/mir_poc_01"),
    re.compile(r"/Users/[^\s`\"')]+/dev/mir_poc_01"),
]

SNAPSHOT_LAST_UPDATED_FILES = [
    "progress.md",
    "samples_progress.md",
    "tasks.md",
]

JST_TIMESTAMP_PATTERN = re.compile(r"\d{4}-\d{2}-\d{2} \d{2}:\d{2} JST")
LAST_UPDATED_PATTERN = re.compile(
    r"^(?:最終更新|Last updated):\s*(\d{4}-\d{2}-\d{2} \d{2}:\d{2} JST)\s*$",
    re.MULTILINE,
)
NUMBERED_PLAN_FILE_PATTERN = re.compile(r"^\d+-.*\.md$")


def _heading_match(text: str, heading: str) -> re.Match[str] | None:
    return re.search(rf"^{re.escape(heading)}\s*$", text, re.MULTILINE)


def _heading_positions(text: str) -> dict[str, int]:
    positions = {}
    for heading in REQUIRED_TEMPLATE_HEADINGS:
        match = _heading_match(text, heading)
        if match is not None:
            positions[heading] = match.start()
    return positions


def _heading_positions_for(text: str, headings: list[str]) -> dict[str, int]:
    positions = {}
    for heading in headings:
        match = _heading_match(text, heading)
        if match is not None:
            positions[heading] = match.start()
    return positions


def missing_template_headings(template_text: str) -> list[str]:
    positions = _heading_positions(template_text)
    return [heading for heading in REQUIRED_TEMPLATE_HEADINGS if heading not in positions]


def out_of_order_template_headings(template_text: str) -> list[str]:
    positions = _heading_positions(template_text)
    if len(positions) != len(REQUIRED_TEMPLATE_HEADINGS):
        return []
    ordered_positions = [positions[heading] for heading in REQUIRED_TEMPLATE_HEADINGS]
    if ordered_positions == sorted(ordered_positions):
        return []
    return REQUIRED_TEMPLATE_HEADINGS


def missing_headings(text: str, headings: list[str]) -> list[str]:
    positions = _heading_positions_for(text, headings)
    return [heading for heading in headings if heading not in positions]


def out_of_order_headings(text: str, headings: list[str]) -> list[str]:
    positions = _heading_positions_for(text, headings)
    if len(positions) != len(headings):
        return []
    ordered_positions = [positions[heading] for heading in headings]
    if ordered_positions == sorted(ordered_positions):
        return []
    return headings


def required_section_bodies(report_text: str) -> dict[str, str]:
    matches: list[tuple[str, re.Match[str]]] = []
    for heading in REQUIRED_TEMPLATE_HEADINGS:
        match = _heading_match(report_text, heading)
        if match is not None:
            matches.append((heading, match))

    sorted_matches = sorted(matches, key=lambda item: item[1].start())
    bodies = {}
    for index, (heading, match) in enumerate(sorted_matches):
        next_start = (
            sorted_matches[index + 1][1].start()
            if index + 1 < len(sorted_matches)
            else len(report_text)
        )
        bodies[heading] = report_text[match.end() : next_start].strip()
    return bodies


def empty_required_sections(report_text: str) -> list[str]:
    bodies = required_section_bodies(report_text)
    return [
        heading
        for heading in REQUIRED_TEMPLATE_HEADINGS
        if heading in bodies and not bodies[heading]
    ]


def unresolved_template_placeholder_sections(report_text: str) -> list[str]:
    bodies = required_section_bodies(report_text)
    unresolved = []
    for heading in REQUIRED_TEMPLATE_HEADINGS:
        body = bodies.get(heading, "")
        if any(placeholder in body for placeholder in UNRESOLVED_TEMPLATE_PLACEHOLDERS):
            unresolved.append(heading)
    return unresolved


def missing_canon_notices() -> dict[str, list[str]]:
    missing_by_file: dict[str, list[str]] = {}
    for relative_path in CANON_NOTICE_FILES:
        text = (ROOT / relative_path).read_text(encoding="utf-8")
        missing_phrases = [
            phrase for phrase in CANON_NOTICE_PHRASES if phrase not in text
        ]
        if missing_phrases:
            missing_by_file[relative_path] = missing_phrases
    return missing_by_file


def _source_hierarchy_lint_paths() -> list[Path]:
    paths: set[Path] = set()
    for relative_path in SOURCE_HIERARCHY_LINT_FILES:
        path = ROOT / relative_path
        if path.exists():
            paths.add(path)

    for relative_dir in SOURCE_HIERARCHY_LINT_DIRS:
        directory = ROOT / relative_dir
        if directory.exists():
            paths.update(directory.rglob("*.md"))

    return sorted(paths)


def stale_source_hierarchy_wording() -> dict[str, list[tuple[int, str]]]:
    hits: dict[str, list[tuple[int, str]]] = {}
    for path in _source_hierarchy_lint_paths():
        relative = path.relative_to(ROOT).as_posix()
        if any(
            relative == prefix.rstrip("/") or relative.startswith(prefix)
            for prefix in SOURCE_HIERARCHY_LINT_EXCLUDED_PREFIXES
        ):
            continue

        lines = path.read_text(encoding="utf-8").splitlines()
        for line_number, line in enumerate(lines, start=1):
            stripped = line.strip()
            if (relative, stripped) in SOURCE_HIERARCHY_LINT_ALLOWED_LINES:
                continue
            if any(
                pattern.search(line)
                for pattern in SOURCE_HIERARCHY_LINT_ALLOWED_PATTERNS
            ):
                continue
            if any(pattern.search(line) for pattern in STALE_SOURCE_HIERARCHY_PATTERNS):
                hits.setdefault(relative, []).append((line_number, stripped))
                continue
            if line_number < len(lines) and any(
                pattern.search(line)
                for pattern in STALE_SOURCE_HIERARCHY_SPLIT_START_PATTERNS
            ):
                next_line = lines[line_number]
                next_stripped = next_line.strip()
                if any(
                    pattern.search(next_line)
                    for pattern in STALE_SOURCE_HIERARCHY_SPLIT_FOLLOWUP_PATTERNS
                ):
                    hits.setdefault(relative, []).append(
                        (line_number, f"{stripped} / {next_stripped}")
                    )
    return hits


def _active_reader_host_path_lint_paths() -> list[Path]:
    paths: set[Path] = set()
    for relative_path in ACTIVE_READER_HOST_PATH_LINT_FILES:
        path = ROOT / relative_path
        if path.exists():
            paths.add(path)

    for relative_dir in ACTIVE_READER_HOST_PATH_LINT_DIRS:
        directory = ROOT / relative_dir
        if not directory.exists():
            continue
        for pattern in ("*.md", "*.json"):
            paths.update(directory.rglob(pattern))

    return sorted(paths)


def active_reader_host_absolute_paths() -> dict[str, list[tuple[int, str]]]:
    hits: dict[str, list[tuple[int, str]]] = {}
    for path in _active_reader_host_path_lint_paths():
        relative = path.relative_to(ROOT).as_posix()
        if any(
            relative == prefix.rstrip("/") or relative.startswith(prefix)
            for prefix in ACTIVE_READER_HOST_PATH_LINT_EXCLUDED_PREFIXES
        ):
            continue

        lines = path.read_text(encoding="utf-8").splitlines()
        for line_number, line in enumerate(lines, start=1):
            if any(pattern.search(line) for pattern in HOST_SPECIFIC_REPO_PATH_PATTERNS):
                hits.setdefault(relative, []).append((line_number, line.strip()))
    return hits


def snapshot_top_last_updated_timestamp(text: str) -> str | None:
    non_empty_lines = [line.strip() for line in text.splitlines() if line.strip()]
    if not non_empty_lines:
        return None

    candidate_index = 1 if non_empty_lines[0].startswith("# ") else 0
    if candidate_index >= len(non_empty_lines):
        return None

    match = LAST_UPDATED_PATTERN.fullmatch(non_empty_lines[candidate_index])
    if match is None:
        return None
    return match.group(1)


def stale_snapshot_last_updated_headers() -> dict[str, tuple[str, str]]:
    stale: dict[str, tuple[str, str]] = {}
    for relative_path in SNAPSHOT_LAST_UPDATED_FILES:
        path = ROOT / relative_path
        if not path.exists():
            continue
        text = path.read_text(encoding="utf-8")
        timestamps = [match.group(0) for match in JST_TIMESTAMP_PATTERN.finditer(text)]
        if not timestamps:
            continue

        latest = max(timestamps)
        header_timestamp = snapshot_top_last_updated_timestamp(text)
        if header_timestamp is None:
            stale[relative_path] = ("missing", latest)
            continue

        if header_timestamp < latest:
            stale[relative_path] = (header_timestamp, latest)
    return stale


def unregistered_numbered_plan_files() -> list[str]:
    plan_root = ROOT / "plan"
    if not plan_root.exists():
        return []

    registered = set(REQUIRED)

    def sort_key(path: Path) -> tuple[int, str]:
        return (int(path.name.split("-", 1)[0]), path.name)

    numbered_paths = [
        path
        for path in plan_root.iterdir()
        if path.is_file() and NUMBERED_PLAN_FILE_PATTERN.fullmatch(path.name)
    ]
    unregistered = []
    for path in sorted(numbered_paths, key=sort_key):
        relative = path.relative_to(ROOT).as_posix()
        if relative not in registered:
            unregistered.append(relative)
    return unregistered


def main() -> int:
    missing = [p for p in REQUIRED if not (ROOT / p).exists()]
    if missing:
        print("Missing required files:")
        for p in missing:
            print(" -", p)
        return 1

    unregistered_plans = unregistered_numbered_plan_files()
    if unregistered_plans:
        print("Numbered plan files are not registered in REQUIRED:")
        for p in unregistered_plans:
            print(" -", p)
        return 1

    missing_notices = missing_canon_notices()
    if missing_notices:
        print("Root entry documents are missing canon notices:")
        for path, phrases in missing_notices.items():
            print(f" - {path}: missing {', '.join(phrases)}")
        return 1

    stale_source_hierarchy_hits = stale_source_hierarchy_wording()
    if stale_source_hierarchy_hits:
        print("Reader-facing docs contain stale source-hierarchy wording:")
        for path, hits in stale_source_hierarchy_hits.items():
            for line_number, line in hits:
                print(f" - {path}:{line_number}: {line}")
        return 1

    active_host_path_hits = active_reader_host_absolute_paths()
    if active_host_path_hits:
        print("Active reader-facing docs contain host-specific repo paths:")
        for path, hits in active_host_path_hits.items():
            for line_number, line in hits:
                print(f" - {path}:{line_number}: {line}")
        return 1

    stale_snapshot_headers = stale_snapshot_last_updated_headers()
    if stale_snapshot_headers:
        print("Snapshot docs have stale last-updated headers:")
        for path, (header_timestamp, latest_timestamp) in stale_snapshot_headers.items():
            print(
                f" - {path}: header {header_timestamp}; "
                f"latest timestamp {latest_timestamp}"
            )
        return 1

    reports = sorted((ROOT / "docs" / "reports").glob("[0-9][0-9][0-9][0-9]-*.md"))
    if not reports:
        print("No numbered reports found in docs/reports")
        return 1

    template_text = (ROOT / "docs" / "reports" / "TEMPLATE.md").read_text(encoding="utf-8")
    missing_template_sections = missing_template_headings(template_text)
    if missing_template_sections:
        print("Report template is missing required sections:")
        for heading in missing_template_sections:
            print(" -", heading)
        return 1
    out_of_order_template_sections = out_of_order_template_headings(template_text)
    if out_of_order_template_sections:
        print("Report template has required sections out of order:")
        for heading in out_of_order_template_sections:
            print(" -", heading)
        return 1

    latest_report = reports[-1]
    latest_report_text = latest_report.read_text(encoding="utf-8")
    missing_latest_report_sections = missing_template_headings(latest_report_text)
    if missing_latest_report_sections:
        print(f"Latest report is missing required sections: {latest_report.name}")
        for heading in missing_latest_report_sections:
            print(" -", heading)
        return 1
    out_of_order_latest_report_sections = out_of_order_template_headings(latest_report_text)
    if out_of_order_latest_report_sections:
        print(f"Latest report has required sections out of order: {latest_report.name}")
        for heading in out_of_order_latest_report_sections:
            print(" -", heading)
        return 1
    empty_latest_report_sections = empty_required_sections(latest_report_text)
    if empty_latest_report_sections:
        print(f"Latest report has empty required sections: {latest_report.name}")
        for heading in empty_latest_report_sections:
            print(" -", heading)
        return 1
    unresolved_latest_report_sections = unresolved_template_placeholder_sections(
        latest_report_text
    )
    if unresolved_latest_report_sections:
        print(
            f"Latest report has unresolved template placeholders: {latest_report.name}"
        )
        for heading in unresolved_latest_report_sections:
            print(" -", heading)
        return 1

    progress_text = (ROOT / "progress.md").read_text(encoding="utf-8")
    missing_progress_sections = missing_headings(
        progress_text, PROGRESS_REQUIRED_HEADINGS
    )
    if missing_progress_sections:
        print("progress.md is missing required snapshot sections:")
        for heading in missing_progress_sections:
            print(" -", heading)
        return 1
    out_of_order_progress_sections = out_of_order_headings(
        progress_text, PROGRESS_REQUIRED_HEADINGS
    )
    if out_of_order_progress_sections:
        print("progress.md has required snapshot sections out of order:")
        for heading in out_of_order_progress_sections:
            print(" -", heading)
        return 1

    tasks_text = (ROOT / "tasks.md").read_text(encoding="utf-8")
    missing_tasks_sections = missing_headings(tasks_text, TASKS_REQUIRED_HEADINGS)
    if missing_tasks_sections:
        print("tasks.md is missing required task-map sections:")
        for heading in missing_tasks_sections:
            print(" -", heading)
        return 1
    out_of_order_tasks_sections = out_of_order_headings(
        tasks_text, TASKS_REQUIRED_HEADINGS
    )
    if out_of_order_tasks_sections:
        print("tasks.md has required task-map sections out of order:")
        for heading in out_of_order_tasks_sections:
            print(" -", heading)
        return 1

    print("Documentation scaffold looks complete.")
    print(f"Found {len(reports)} numbered report(s).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
