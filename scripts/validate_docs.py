#!/usr/bin/env python3
from __future__ import annotations

import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys
from dataclasses import dataclass

ROOT = Path(__file__).resolve().parents[1]
REQUIRED = [
    "CANON.md",
    "README.md",
    "AGENTS.md",
    "Documentation.md",
    "docs/project-status.md",
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
    "scripts/env/mirrorea_storage_env.sh",
    "scripts/storage/setup_mirrorea_workdisk_root.sh",
    "scripts/storage/detach_prepare.sh",
    "scripts/storage/cleanup_disposable_artifacts.sh",
    "scripts/storage/tmp_mirrorea_artifacts.sh",
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
    "scripts/tests/test_storage_workdir_guards.py",
    "scripts/tests/test_tmp_mirrorea_artifacts.py",
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
    "plan/145-g1-obl001-artifact-decision-reuse-audit.md",
    "plan/146-g1-obl001-explanation-boundary-guard-hardening.md",
    "plan/147-g1-next-line-promotion-boundary-audit.md",
    "plan/148-storage-workdir-mountpoint-guard-hardening.md",
    "plan/149-current-phase-position-reading.md",
    "plan/150-phase-position-validator-guard.md",
    "plan/151-discord-webhook-secret-validator-guard.md",
    "plan/152-discord-notification-file-inputs.md",
    "plan/153-g0-closeout-evidence-and-exit-decision-packet.md",
    "plan/154-project-control-cockpit.md",
    "plan/155-t0-g0-governance-profile-proposal.md",
    "plan/198-t0-g0-governance-profile-v2.md",
    "plan/156-t0-t2-research-autonomy-envelope.md",
    "plan/157-delegated-theory-research-governance.md",
    "plan/158-standing-bounded-autonomy.md",
    "plan/159-wrk-evidence-commit-integrity-recut.md",
    "plan/160-obl021-statement-shape-checkpoint.md",
    "plan/161-post-checkpoint-candidate-triage-and-runnable-baseline.md",
    "plan/162-post-wrk0006-candidate-selection.md",
    "plan/163-foundation-integrity-and-elaboration-outcome-audit.md",
    "plan/164-obl001-result-write-coverage-boundary.md",
    "plan/165-post-wrk0007-candidate-selection.md",
    "plan/166-mir-computational-baseline-directness-audit.md",
    "plan/167-pcomp03-rejection-phase-cross-carrier-audit.md",
    "plan/168-wrk0009-e5-skeleton-identity-selection.md",
    "plan/169-wrk0010-static-decision-attribution-selection.md",
    "plan/170-post-wrk0011-candidate-selection.md",
    "plan/171-theory-core-correspondence-and-disposition-checkpoint.md",
    "plan/172-standing-autonomy-lane-correspondence-checkpoint.md",
    "plan/173-local-predicate-constructive-decidability-selection.md",
    "plan/174-local-predicate-proposition-decidability-selection.md",
    "plan/175-post-wrk0017-axiom-profile-disposition.md",
    "plan/176-current-standing-candidate-disposition.md",
    "plan/177-thm005-telemetry-effect-boundary-selection.md",
    "plan/178-post-wrk0018-candidate-rescreen.md",
    "plan/179-independent-source-locus-audit.md",
    "plan/180-t1-t2-statement-identity-dependency-closure-audit.md",
    "plan/181-preservation-proof-prerequisite-literature-audit.md",
    "plan/182-canon-core-minimality-and-proof-interface-audit.md",
    "plan/183-transparent-cost-bound-substitutability-decision.md",
    "plan/184-post-wrk0021-autonomous-frontier-triage.md",
    "plan/185-cost-bound-substitutability-primary-literature-audit.md",
    "plan/186-canonical-elaboration-trace-interface-closure-audit.md",
    "plan/187-mircore-value-flow-and-occurrence-decision-packet.md",
    "plan/188-parser-free-chain-closure-integrity.md",
    "plan/189-autonomous-theory-frontier-revalidation.md",
    "plan/190-first-unlocking-owner-disposition.md",
    "plan/191-post-wrk0022-autonomous-frontier-triage.md",
    "plan/192-post-admission-request-validation-context-audit.md",
    "plan/193-post-admission-validation-context-literature-and-counterexample-memo.md",
    "plan/194-product-alpha1-installed-binary-replay-evidence.md",
    "plan/195-post-proposal013-autonomous-frontier-delta-audit.md",
    "plan/196-t0-t2-implementation-entry-roadmap.md",
    "plan/197-i1-bootstrap-decision-and-readiness-audit.md",
    "plan/199-selected-semantic-composition-and-inference-boundary.md",
    "plan/200-reanchored-semantic-composition-research-plan.md",
    "plan/201-c5-a2-issuance-guard-candidate-selection.md",
    "plan/202-v1-r1-presentation-refinement-candidate-selection.md",
    "plan/203-v1-r1-finite-sequence-candidate-selection.md",
    "plan/204-wrk0034-semantic-composition-no-candidate-disposition.md",
    "plan/205-c7-parametric-factorization-candidate-selection.md",
    "plan/206-c7-cumulative-erasure-countermodel-candidate-selection.md",
    "plan/207-post-wrk0036-autonomous-frontier-disposition.md",
    "plan/208-c2b-c3-value-flow-design-preparation.md",
    "plan/209-c2b-c3-relation-obligation-audit.md",
    "plan/210-c2b-c3-family-a-b-instantiation-audit.md",
    "plan/211-c2b-c3-b-primary-opaque-anchor-candidate-selection.md",
    "plan/212-c2b-c3-bundled-relational-presentation-comparison-selection.md",
    "plan/213-c2b-c3-fiberwise-relational-comparison-selection.md",
    "plan/214-post-wrk0039-autonomous-frontier-disposition.md",
    "plan/215-c2b-c3-ordinary-design-decision-packet.md",
    "plan/216-c2b-c3-cross-boundary-compatibility-audit.md",
    "plan/217-c2b-c3-carrier-neutral-conditional-comparison.md",
    "plan/218-c2b-c3-first-card-source-preflight.md",
    "plan/219-c2b-c3-minimal-semantic-residence-options.md",
    "plan/220-c2b-c3-relation-state-proof-obligation-audit.md",
    "plan/221-c2b-c3-canon-proposal-preparation.md",
    "plan/222-p017-x1-owner-terminal-exclusivity-candidate-selection.md",
    "plan/223-p017-x1-owner-negative-mutation-candidate-selection.md",
    "plan/224-p017-x1-m1-adverse-mutation-candidate-selection.md",
    "plan/225-post-wrk0043-fixture-frontier-disposition.md",
    "plan/226-post-wrk0043-cross-lane-p0a-preflight.md",
    "plan/227-p017-x1-decision-vector-and-choice-neutral-consistency.md",
    "plan/228-p017-x1-minimum-coherence-candidate-selection.md",
    "plan/229-post-wrk0044-no-successor-ordinary-design-boundary.md",
    "plan/230-p017-x1-first-ordinary-design-card-preflight.md",
    "plan/231-k0-rl-factorization-preflight.md",
    "plan/232-p017-x1-k0-rl-definitional-collapse-screen.md",
    "plan/233-p017-x1-k0-b-fact-status-screen.md",
    "plan/234-p017-x1-k0-terminal-success-positive-basis-card.md",
    "plan/235-p017-x1-typed-owner-result-role-conformance-audit.md",
    "plan/236-p017-x1-k0-owner-result-provenance-basis-and-definability-screen.md",
    "plan/237-p017-x1-k0-owner-outstanding-positive-basis-and-pending-nonconflation-card.md",
    "plan/238-p017-x1-k0-terminal-failure-positive-basis-and-failure-nonconflation-card.md",
    "plan/239-p017-x1-k0-consulted-validation-provenance-basis-and-nonconflation-screen.md",
    "plan/240-p017-x1-k0-minimum-model-hk-intake-and-fail-closed-gate.md",
    "plan/241-p017-x1-k0-hk-rs-occurrence-accounting-preflight.md",
    "plan/242-p017-x1-k0-hk-rs-integrated-conditional-candidate-selection.md",
    "plan/243-p017-x1-k0-hk-rs-l3-standing-eligibility-recheck.md",
    "plan/244-p017-x1-k0-hk-rs-source-premise-falsifier-design.md",
    "plan/245-post-wrk0045-no-successor-ordinary-x1-handoff.md",
    "plan/246-goal-first-semantic-integration-and-i1-entry.md",
    "plan/247-mir-theory-v0-i1plus-current-roadmap.md",
    "plan/249-mirrorea-i2-systems-foundation-current-roadmap.md",
    "plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md",
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
    "mirrorea_canon/working/README.md",
    "mirrorea_canon/meta/review-keys.json",
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
    "## docs/project-status.md update status",
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

PROJECT_STATUS_REQUIRED_HEADINGS = [
    "## この文書の役割",
    "## 全体の進行チェックリスト",
    "## 現在地",
    "## 現在の停止線",
    "## オーナーの確認・判断待ち",
    "## 根拠と詳細",
    "## 更新規約",
]

PROJECT_STATUS_GUARD_PHRASES = [
    "派生ビュー",
]

PROJECT_STATUS_MAX_LINES = 180
PROJECT_STATUS_SOURCE_SECTIONS = [
    "## 現在地",
    "## 現在の停止線",
    "## オーナーの確認・判断待ち",
    "## 根拠と詳細",
]
PROJECT_STATUS_SOURCE_PATH_PATTERN = re.compile(r"`([^`\n]+)`")
PROJECT_STATUS_FILE_PATH_BULLET_PATTERN = re.compile(
    r"^[ \t]*-[ \t]+`([^`\n]+)`[ \t]*$", re.MULTILINE
)
PROJECT_STATUS_CHECKED_ITEM_PATTERN = re.compile(
    r"\[[xX]\]\s+(?:G[0-7]|T[0-2]|I[1-6])\b"
)
PROJECT_STATUS_UPDATE_STATUS_HEADING = "## docs/project-status.md update status"
PROJECT_STATUS_UPDATE_DECLARATION_PATTERN = re.compile(
    r"^(更新済み|更新不要):\s*(\S.*)$", re.MULTILINE
)
PROJECT_STATUS_UPDATE_PENDING_PATTERN = re.compile(
    r"\b(?:tbd|pending)\b", re.IGNORECASE
)

SNAPSHOT_POSITION_SOURCE_SECTIONS = {
    "progress.md": "## current milestone position",
    "tasks.md": "## current promoted package",
}

UNRESOLVED_TEMPLATE_PLACEHOLDERS = [
    "更新不要 / 更新済み:",
]

CANON_NOTICE_FILES = [
    "README.md",
    "AGENTS.md",
    "Documentation.md",
    "docs/project-status.md",
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
    "docs/project-status.md",
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
    "docs/project-status.md",
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

CONCRETE_DISCORD_WEBHOOK_URL_PATTERN = re.compile(
    r"https?://(?:[A-Za-z0-9-]+\.)?discord(?:app)?\.com"
    r"/api(?:/v\d+)?/webhooks/\d+/[A-Za-z0-9_-]{20,}",
    re.IGNORECASE,
)

SECRET_SCAN_EXCLUDED_DIR_NAMES = {
    ".git",
    ".codex-discord",
    ".mypy_cache",
    ".pytest_cache",
    "__pycache__",
    "target",
}

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
WORKING_RECORD_FILE_PATTERN = re.compile(r"^WRK-\d{4}-[a-z0-9][a-z0-9-]*\.md$")
WORKING_RECORD_RELIANCE_PATTERN = re.compile(
    r"^Reliance status:[ \t]*(.*)$", re.MULTILINE
)
WORKING_RECORD_REVIEW_PATTERN = re.compile(
    r"^reviewer-fingerprint=([0-9A-Fa-f]{40}); "
    r"frozen-base=([0-9a-f]{40}); record-sha256=([0-9a-f]{64}); "
    r"decision=approved$"
)
WORKING_RECORD_FINGERPRINT_PATTERN = re.compile(r"^[0-9A-Fa-f]{40}$")
WORKING_RECORD_CANON_ANCHORS_PATTERN = re.compile(
    r"^[a-z][a-z0-9-]*/[A-Za-z0-9_.-]+@[0-9a-f]{40}:[0-9a-f]{64}"
    r"(?:, [a-z][a-z0-9-]*/[A-Za-z0-9_.-]+@[0-9a-f]{40}:[0-9a-f]{64})*$"
)
WORKING_RECORD_LAB_SNAPSHOTS_PATTERN = re.compile(
    r"^LAB:([A-Za-z0-9][A-Za-z0-9_./-]*)@([0-9a-f]{40}):([0-9a-f]{64})"
    r"(?:, LAB:[A-Za-z0-9][A-Za-z0-9_./-]*@[0-9a-f]{40}:[0-9a-f]{64})*$"
)
WORKING_RECORD_REQUIRED_HEADINGS = [
    "## Classification and authority cut",
    "## Pre-registered working question",
    "## Method and evidence plan",
    "## Results and review",
    "## Supersession",
]
WORKING_RECORD_SECTION_FIELDS = {
    "## Classification and authority cut": (
        "Standing eligibility",
        "Author",
        "Author fingerprint",
        "Canon anchors",
        "LAB inputs",
        "Permitted LAB locations",
        "Reserved surfaces",
    ),
    "## Pre-registered working question": (
        "Question",
        "Status quo",
        "Alternative",
        "Expected falsifier",
        "Rollback / reopen trigger",
    ),
    "## Method and evidence plan": ("Result class", "Commands", "Non-claims"),
    "## Results and review": (
        "Positive evidence",
        "Negative evidence",
        "Evidence artifacts",
        "Evidence commits",
        "Impact / non-effects",
        "Independent review",
    ),
    "## Supersession": ("Supersession",),
}
WORKING_RECORD_RESULT_CLASSES = {
    "reproduction",
    "literal-transcription",
    "countermodel",
    "conditional-lemma",
    "existing-lane-experiment",
}
WORKING_RECORD_PENDING_VALUES = {
    "pending",
    "not-run",
    "not-required-for-l3",
    "none",
    "n/a",
    "na",
}
WORKING_REVIEW_KEYS_PATH = "mirrorea_canon/meta/review-keys.json"
WORKING_RECORD_ALLOWED_LAB_ROOTS = (
    "docs/reports",
    "plan",
    "samples/clean-near-end",
    "samples/current-l2",
    "samples/lean",
    "samples/product-alpha1/computational",
)
WORKING_RECORD_ALLOWED_LAB_DESCENDANT_ROOTS = (
    "samples/product-alpha1/computational",
)
WORKING_RECORD_EVIDENCE_COMMIT_PATTERN = re.compile(
    r"^[0-9a-f]{40}(?:, [0-9a-f]{40})*$"
)
DIRECT_NUMBERED_REPORT_PATTERN = re.compile(
    r"^docs/reports/\d{4}-[A-Za-z0-9][A-Za-z0-9_.-]*\.md$"
)
WORKING_RECORD_CONTROL_FILES = {
    "docs/project-status.md",
    "mirrorea_canon/CHANGELOG.md",
    "mirrorea_canon/INDEX.json",
    "mirrorea_canon/MAP.md",
    "progress.md",
    "samples_progress.md",
    "tasks.md",
}


@dataclass(frozen=True)
class WorkingRecordDescriptor:
    """The immutable identity and append-only evidence state of one WRK."""

    identifier: str
    relative: str
    preregistration: tuple[str, str, str]
    permitted_locations: tuple[str, ...]
    registration: str
    evidence_commits: tuple[str, ...]
    evidence_artifact_revisions: tuple[str, ...]


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


def section_bodies(text: str, headings: list[str]) -> dict[str, str]:
    matches: list[tuple[str, re.Match[str]]] = []
    for heading in headings:
        match = _heading_match(text, heading)
        if match is not None:
            matches.append((heading, match))

    sorted_matches = sorted(matches, key=lambda item: item[1].start())
    bodies = {}
    for index, (heading, match) in enumerate(sorted_matches):
        next_start = (
            sorted_matches[index + 1][1].start()
            if index + 1 < len(sorted_matches)
            else len(text)
        )
        bodies[heading] = text[match.end() : next_start].strip()
    return bodies


def safe_repo_relative_file(relative_path: str) -> Path | None:
    """Resolve one repository-relative source path without accepting escapes."""
    candidate = PurePosixPath(relative_path)
    if (
        not relative_path
        or relative_path != relative_path.strip()
        or candidate.is_absolute()
        or not candidate.parts
        or any(part in {".", ".."} for part in candidate.parts)
    ):
        return None

    try:
        root = ROOT.resolve()
        resolved = (ROOT / candidate).resolve(strict=True)
        resolved.relative_to(root)
    except (FileNotFoundError, OSError, ValueError):
        return None

    return resolved if resolved.is_file() else None


def is_canonical_source_file(relative_path: str) -> bool:
    candidate = PurePosixPath(relative_path)
    return bool(candidate.parts) and candidate.parts[0] == "mirrorea_canon" and bool(
        safe_repo_relative_file(relative_path)
    )


def is_plan_source_file(relative_path: str) -> bool:
    candidate = PurePosixPath(relative_path)
    return bool(candidate.parts) and candidate.parts[0] == "plan" and bool(
        safe_repo_relative_file(relative_path)
    )


def is_path_like_code_span(value: str) -> bool:
    """Return true for a code span that is intended to name a filesystem path."""
    normalized = value.strip()
    if not normalized:
        return False
    if any(character.isspace() for character in normalized):
        return normalized.startswith(("/", "./", "../", "~/", ".\\", "..\\"))
    if re.fullmatch(
        r"(?:G[0-7]|T[0-2]|I[1-6])/(?:G[0-7]|T[0-2]|I[1-6])", normalized
    ):
        return False
    return (
        normalized.startswith(("/", "./", "../", "~/", ".\\", "..\\"))
        or "/" in normalized
        or "\\" in normalized
        or normalized.endswith((".md", ".json", ".mmd", ".txt"))
    )


def duplicate_required_headings(text: str, headings: list[str]) -> list[str]:
    return [
        heading
        for heading in headings
        if len(re.findall(rf"^{re.escape(heading)}\s*$", text, re.MULTILINE)) > 1
    ]


def missing_project_status_guard_phrases() -> list[str]:
    text = (ROOT / "docs" / "project-status.md").read_text(encoding="utf-8")
    return [phrase for phrase in PROJECT_STATUS_GUARD_PHRASES if phrase not in text]


def project_status_source_paths(text: str) -> dict[str, list[str]]:
    bodies = section_bodies(text, PROJECT_STATUS_REQUIRED_HEADINGS)
    paths_by_section: dict[str, list[str]] = {}
    for heading in PROJECT_STATUS_SOURCE_SECTIONS:
        body = bodies.get(heading, "")
        paths = [
            candidate
            for candidate in PROJECT_STATUS_SOURCE_PATH_PATTERN.findall(body)
            if is_path_like_code_span(candidate)
        ]
        paths_by_section[heading] = paths
    return paths_by_section


def project_status_source_path_errors(text: str) -> list[str]:
    errors = []
    for heading, paths in project_status_source_paths(text).items():
        if not paths:
            errors.append(f"{heading}: no repo-relative source path")
            continue
        for relative_path in paths:
            if safe_repo_relative_file(relative_path) is None:
                errors.append(relative_path)
    return errors


def checked_project_status_item_errors(text: str) -> list[str]:
    errors = []
    for line in text.splitlines():
        checked_items = PROJECT_STATUS_CHECKED_ITEM_PATTERN.findall(line)
        if not checked_items:
            continue
        if len(checked_items) != 1:
            errors.append(f"multiple checked items: {line.strip()}")
            continue
        canonical_paths = [
            path
            for path in PROJECT_STATUS_SOURCE_PATH_PATTERN.findall(line)
            if is_canonical_source_file(path)
        ]
        if not canonical_paths:
            errors.append(line.strip())
    return errors


def project_status_update_status_errors(
    body: str, files_changed_body: str
) -> list[str]:
    errors = []
    if PROJECT_STATUS_UPDATE_PENDING_PATTERN.search(body):
        errors.append("contains TBD or pending")
    declarations = PROJECT_STATUS_UPDATE_DECLARATION_PATTERN.findall(body)
    if len(declarations) != 1:
        errors.append("requires exactly one 更新済み: or 更新不要: declaration with a reason")
        return errors
    declaration, _reason = declarations[0]
    changed_paths = set(PROJECT_STATUS_FILE_PATH_BULLET_PATTERN.findall(files_changed_body))
    has_status_file = "docs/project-status.md" in changed_paths
    if declaration == "更新済み" and not has_status_file:
        errors.append("更新済み declaration lacks docs/project-status.md in Files changed")
    if declaration == "更新不要" and has_status_file:
        errors.append("更新不要 declaration conflicts with docs/project-status.md in Files changed")
    return errors


def out_of_order_headings(text: str, headings: list[str]) -> list[str]:
    positions = _heading_positions_for(text, headings)
    if len(positions) != len(headings):
        return []
    ordered_positions = [positions[heading] for heading in headings]
    if ordered_positions == sorted(ordered_positions):
        return []
    return headings


def required_section_bodies(report_text: str) -> dict[str, str]:
    return section_bodies(report_text, REQUIRED_TEMPLATE_HEADINGS)


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


def _tracked_secret_scan_files() -> list[Path] | None:
    try:
        result = subprocess.run(
            [
                "git",
                "-C",
                str(ROOT),
                "ls-files",
                "--cached",
                "--others",
                "--exclude-standard",
                "-z",
            ],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
    except (OSError, subprocess.CalledProcessError):
        return None

    files: list[Path] = []
    for raw_path in result.stdout.split(b"\0"):
        if not raw_path:
            continue
        try:
            relative = raw_path.decode("utf-8")
        except UnicodeDecodeError:
            continue
        path = ROOT / relative
        if path.is_file():
            files.append(path)
    return sorted(files)


def _fallback_secret_scan_files() -> list[Path]:
    files: list[Path] = []
    for path in ROOT.rglob("*"):
        if not path.is_file():
            continue
        try:
            relative = path.relative_to(ROOT)
        except ValueError:
            continue
        if any(part in SECRET_SCAN_EXCLUDED_DIR_NAMES for part in relative.parts):
            continue
        files.append(path)
    return sorted(files)


def secret_scan_candidate_files() -> list[Path]:
    files: set[Path]
    tracked = _tracked_secret_scan_files()
    if tracked is not None:
        files = set(tracked)
    else:
        files = set(_fallback_secret_scan_files())

    for relative_path in REQUIRED:
        path = ROOT / relative_path
        if path.is_file():
            files.add(path)

    reports_root = ROOT / "docs" / "reports"
    if reports_root.exists():
        files.update(reports_root.glob("[0-9][0-9][0-9][0-9]-*.md"))

    return sorted(files)


def concrete_discord_webhook_leaks() -> dict[str, list[int]]:
    hits: dict[str, list[int]] = {}
    for path in secret_scan_candidate_files():
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        relative = path.relative_to(ROOT).as_posix()
        for line_number, line in enumerate(text.splitlines(), start=1):
            if CONCRETE_DISCORD_WEBHOOK_URL_PATTERN.search(line):
                hits.setdefault(relative, []).append(line_number)
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


def snapshot_position_source_errors() -> dict[str, list[str]]:
    errors: dict[str, list[str]] = {}
    for relative_path, heading in SNAPSHOT_POSITION_SOURCE_SECTIONS.items():
        path = ROOT / relative_path
        if not path.exists():
            continue

        text = path.read_text(encoding="utf-8")
        headings = (
            PROGRESS_REQUIRED_HEADINGS
            if relative_path == "progress.md"
            else TASKS_REQUIRED_HEADINGS
        )
        body = section_bodies(text, headings).get(heading, "")
        source_paths = PROJECT_STATUS_SOURCE_PATH_PATTERN.findall(body)
        missing = []
        if not any(is_canonical_source_file(path) for path in source_paths):
            missing.append("an existing mirrorea_canon/ source file")
        if not any(is_plan_source_file(path) for path in source_paths):
            missing.append("an existing plan/ source file")
        if missing:
            errors[relative_path] = missing
    return errors


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


def working_record_front_matter(text: str) -> tuple[dict[str, str], set[str]] | None:
    match = re.match(r"\A---\n(.*?)\n---\n", text, re.DOTALL)
    if match is None:
        return None

    fields: dict[str, str] = {}
    duplicate_fields: set[str] = set()
    for line in match.group(1).splitlines():
        field = re.match(r"^(\w+):\s*(.*)$", line)
        if field is not None:
            if field.group(1) in fields:
                duplicate_fields.add(field.group(1))
            fields[field.group(1)] = field.group(2).strip()
    return fields, duplicate_fields


def working_record_field_value(body: str, label: str) -> str | None:
    match = re.search(rf"^{re.escape(label)}:[ \t]*(.*)$", body, re.MULTILINE)
    if match is None:
        return None
    value = match.group(1).strip()
    return value or None


def _git_bytes(root: Path, *args: str) -> bytes | None:
    try:
        result = subprocess.run(
            ["git", "-C", str(root), *args],
            check=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
    except (OSError, subprocess.CalledProcessError):
        return None
    return result.stdout


def _git_text(root: Path, *args: str) -> str | None:
    result = _git_bytes(root, *args)
    if result is None:
        return None
    return result.decode("utf-8", errors="replace").strip()


def _git_commit_exists(root: Path, commit: str) -> bool:
    return _git_bytes(root, "cat-file", "-e", f"{commit}^{{commit}}") is not None


def _safe_snapshot_path(value: str) -> str | None:
    path = PurePosixPath(value)
    if (
        not value
        or path.is_absolute()
        or any(part in {"", ".", ".."} for part in path.parts)
    ):
        return None
    return path.as_posix()


def _historical_canon_anchor_contents(
    root: Path, identifier: str, revision: str
) -> list[bytes]:
    paths = _git_text(
        root, "ls-tree", "-r", "--name-only", revision, "--", "mirrorea_canon"
    )
    if paths is None:
        return []
    matches: list[bytes] = []
    for source_path in paths.splitlines():
        if not source_path.endswith(".md"):
            continue
        content = _git_bytes(root, "show", f"{revision}:{source_path}")
        if content is None:
            continue
        parsed = working_record_front_matter(content.decode("utf-8", errors="replace"))
        if parsed is None:
            continue
        fields, duplicates = parsed
        if not duplicates and fields.get("id") == identifier:
            matches.append(content)
    return matches


def _permitted_lab_locations(value: str) -> list[str] | None:
    locations = [_safe_snapshot_path(item) for item in value.split(", ")]
    if (
        not locations
        or any(location is None for location in locations)
        or any(
            not _is_documented_lab_location(location)
            for location in locations
            if location is not None
        )
    ):
        return None
    return [location for location in locations if location is not None]


def _is_documented_lab_location(location: str) -> bool:
    return location in WORKING_RECORD_ALLOWED_LAB_ROOTS or any(
        location.startswith(f"{root}/")
        for root in WORKING_RECORD_ALLOWED_LAB_DESCENDANT_ROOTS
    )


def _is_permitted_lab_path(path: str, locations: list[str]) -> bool:
    return any(
        (
            location == "docs/reports"
            and DIRECT_NUMBERED_REPORT_PATTERN.fullmatch(path) is not None
        )
        or (
            location != "docs/reports"
            and (path == location or path.startswith(f"{location}/"))
        )
        for location in locations
    )


def _snapshot_digest_errors(
    root: Path,
    values: dict[str, str],
    relative: str,
) -> list[str]:
    errors: list[str] = []
    anchors = values.get("Canon anchors")
    if anchors is not None:
        for entry in anchors.split(", "):
            identifier, revision_and_digest = entry.split("@", 1)
            revision, expected_digest = revision_and_digest.split(":", 1)
            if not _git_commit_exists(root, revision):
                errors.append(f"{relative}: Canon anchor commit is not present: {revision}")
                continue
            contents = _historical_canon_anchor_contents(root, identifier, revision)
            if not contents:
                errors.append(
                    f"{relative}: Canon anchor id is absent at {revision}: {identifier}"
                )
                continue
            if len(contents) != 1:
                errors.append(
                    f"{relative}: Canon anchor id is ambiguous at {revision}: {identifier}"
                )
                continue
            if hashlib.sha256(contents[0]).hexdigest() != expected_digest:
                errors.append(f"{relative}: Canon anchor digest does not match: {identifier}")

    permitted_locations = _permitted_lab_locations(
        values.get("Permitted LAB locations", "")
    )
    if permitted_locations is None:
        errors.append(f"{relative}: Permitted LAB locations must be safe relative paths")
    for label in ("LAB inputs", "Evidence artifacts"):
        value = values.get(label)
        if value is None or value.lower() in WORKING_RECORD_PENDING_VALUES:
            continue
        if not WORKING_RECORD_LAB_SNAPSHOTS_PATTERN.fullmatch(value):
            errors.append(f"{relative}: {label} must use LAB:path@commit:sha256 entries")
            continue
        for entry in value.split(", "):
            path_and_revision, expected_digest = entry[4:].split(":", 1)
            source_path, revision = path_and_revision.rsplit("@", 1)
            source_path = _safe_snapshot_path(source_path)
            if source_path is None:
                errors.append(f"{relative}: {label} has an unsafe LAB path")
                continue
            if (
                permitted_locations is not None
                and not _is_permitted_lab_path(source_path, permitted_locations)
            ):
                errors.append(
                    f"{relative}: {label} path is outside Permitted LAB locations: {source_path}"
                )
                continue
            if not _git_commit_exists(root, revision):
                errors.append(f"{relative}: {label} commit is not present: {revision}")
                continue
            content = _git_bytes(root, "show", f"{revision}:{source_path}")
            if content is None:
                errors.append(
                    f"{relative}: {label} is absent at {revision}: {source_path}"
                )
                continue
            if hashlib.sha256(content).hexdigest() != expected_digest:
                errors.append(f"{relative}: {label} digest does not match: {source_path}")
    return errors


def _normalized_working_record_digest(text: str) -> str | None:
    normalized, count = re.subn(
        r"^Independent review:[^\n]*$",
        "Independent review: <review-metadata>",
        text,
        count=1,
        flags=re.MULTILINE,
    )
    if count != 1:
        return None
    return hashlib.sha256(normalized.encode("utf-8")).hexdigest()


def _signed_commit_fingerprint(root: Path, commit: str) -> str | None:
    if _git_bytes(root, "verify-commit", commit) is None:
        return None
    fingerprint = _git_text(root, "show", "-s", "--format=%GF", commit)
    if fingerprint is None or not WORKING_RECORD_FINGERPRINT_PATTERN.fullmatch(
        fingerprint
    ):
        return None
    return fingerprint.upper()


def _trusted_review_keys(root: Path) -> tuple[set[str], set[str]] | None:
    path = root / WORKING_REVIEW_KEYS_PATH
    try:
        data = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        return None
    if data.get("format") != 1:
        return None
    author_keys = data.get("author_fingerprints")
    reviewer_keys = data.get("reviewer_fingerprints")
    if not isinstance(author_keys, list) or not isinstance(reviewer_keys, list):
        return None
    normalized = []
    for keys in (author_keys, reviewer_keys):
        if not all(
            isinstance(key, str) and WORKING_RECORD_FINGERPRINT_PATTERN.fullmatch(key)
            for key in keys
        ):
            return None
        normalized.append({key.upper() for key in keys})
    return normalized[0], normalized[1]


def _record_history(root: Path, relative: str) -> list[str]:
    history = _git_text(root, "log", "--format=%H", "HEAD", "--", relative)
    return [] if history is None else history.splitlines()


def _reachable_commits(root: Path) -> list[str]:
    commits = _git_text(root, "rev-list", "--reverse", "HEAD")
    return [] if commits is None else commits.splitlines()


def _git_is_ancestor(root: Path, ancestor: str, descendant: str) -> bool:
    return _git_bytes(root, "merge-base", "--is-ancestor", ancestor, descendant) is not None


def _commit_local_changed_paths(root: Path, commit: str) -> list[str]:
    """Return paths authored by a commit, without merge-parent double counting."""
    parents = _commit_parents(root, commit)
    if len(parents) > 1:
        paths = _git_text(
            root, "diff-tree", "--no-commit-id", "-r", "--cc", "--name-only", commit
        )
    else:
        paths = _git_text(
            root, "diff-tree", "--root", "--no-commit-id", "-r", "--name-only", commit
        )
    return [] if paths is None else paths.splitlines()


def _working_tree_paths(root: Path) -> list[str]:
    paths = _git_text(
        root, "ls-tree", "-r", "--name-only", "HEAD", "--", "mirrorea_canon/working"
    )
    return [] if paths is None else paths.splitlines()


def _working_tree_paths_at(root: Path, commit: str) -> list[str]:
    paths = _git_text(
        root,
        "ls-tree",
        "-r",
        "--name-only",
        commit,
        "--",
        "mirrorea_canon/working",
    )
    return [] if paths is None else paths.splitlines()


def _working_record_snapshot_revisions(value: str | None) -> tuple[str, ...]:
    if value is None or value.lower() in WORKING_RECORD_PENDING_VALUES:
        return ()
    if not WORKING_RECORD_LAB_SNAPSHOTS_PATTERN.fullmatch(value):
        return ()
    revisions = []
    for entry in value.split(", "):
        path_and_revision, _digest = entry[4:].split(":", 1)
        _path, revision = path_and_revision.rsplit("@", 1)
        revisions.append(revision)
    return tuple(revisions)


def _working_record_anchor_revisions(value: str | None) -> tuple[str, ...]:
    if value is None or not WORKING_RECORD_CANON_ANCHORS_PATTERN.fullmatch(value):
        return ()
    return tuple(entry.split("@", 1)[1].split(":", 1)[0] for entry in value.split(", "))


def _working_record_evidence_commits(value: str | None) -> tuple[str, ...] | None:
    if value is None:
        return None
    if value == "none":
        return ()
    if not WORKING_RECORD_EVIDENCE_COMMIT_PATTERN.fullmatch(value):
        return None
    commits = tuple(value.split(", "))
    return commits if len(commits) == len(set(commits)) else None


def _working_record_descriptor(
    root: Path, commit: str, relative: str, text: str
) -> tuple[WorkingRecordDescriptor | None, list[str]]:
    """Parse the history-critical subset of a direct WRK entry."""
    errors: list[str] = []
    name_match = re.fullmatch(r"WRK-(\d{4})-[a-z0-9][a-z0-9-]*\.md", Path(relative).name)
    if Path(relative).parent.as_posix() != "mirrorea_canon/working" or name_match is None:
        return None, [f"{relative}@{commit[:12]}: working annex permits only direct WRK records"]
    parsed = working_record_front_matter(text)
    if parsed is None:
        return None, [f"{relative}@{commit[:12]}: missing WRK front matter"]
    front_matter, duplicates = parsed
    if duplicates:
        errors.append(f"{relative}@{commit[:12]}: duplicate front matter fields")
    expected_id = f"working/WRK-{name_match.group(1)}"
    if front_matter.get("id") != expected_id:
        errors.append(f"{relative}@{commit[:12]}: historical WRK identity does not match {expected_id}")
    if front_matter.get("status") not in {"L3-open", "L2-working"}:
        errors.append(f"{relative}@{commit[:12]}: invalid WRK status")
    missing = missing_headings(text, WORKING_RECORD_REQUIRED_HEADINGS)
    if missing or out_of_order_headings(text, WORKING_RECORD_REQUIRED_HEADINGS):
        errors.append(f"{relative}@{commit[:12]}: WRK lacks ordered required sections")
        return None, errors
    bodies = section_bodies(text, WORKING_RECORD_REQUIRED_HEADINGS)
    values = {
        label: working_record_field_value(bodies[heading], label)
        for heading, labels in WORKING_RECORD_SECTION_FIELDS.items()
        for label in labels
    }
    permitted = _permitted_lab_locations(values.get("Permitted LAB locations", ""))
    evidence_commits = _working_record_evidence_commits(values.get("Evidence commits"))
    if permitted is None:
        errors.append(f"{relative}@{commit[:12]}: invalid Permitted LAB locations")
    if evidence_commits is None:
        errors.append(f"{relative}@{commit[:12]}: Evidence commits must be none or unique 40-hex commits")
    if errors:
        return None, errors
    assert permitted is not None and evidence_commits is not None
    preregistration = tuple(
        bodies[heading]
        for heading in (
            "## Classification and authority cut",
            "## Pre-registered working question",
            "## Method and evidence plan",
        )
    )
    return (
        WorkingRecordDescriptor(
            identifier=expected_id,
            relative=relative,
            preregistration=preregistration,
            permitted_locations=tuple(permitted),
            registration=commit,
            evidence_commits=evidence_commits,
            evidence_artifact_revisions=_working_record_snapshot_revisions(
                values.get("Evidence artifacts")
            ),
        ),
        [],
    )


def _is_working_record_metadata_path(path: str, relative: str) -> bool:
    return (
        path == relative
        or path in WORKING_RECORD_CONTROL_FILES
        or DIRECT_NUMBERED_REPORT_PATTERN.fullmatch(path) is not None
    )


def _record_delta_errors(
    root: Path,
    commit: str,
    descriptor: WorkingRecordDescriptor,
    *,
    registration: bool,
) -> list[str]:
    errors: list[str] = []
    for path in _commit_local_changed_paths(root, commit):
        allowed = _is_working_record_metadata_path(path, descriptor.relative)
        if not registration:
            allowed = allowed or _is_permitted_lab_path(path, list(descriptor.permitted_locations))
        if not allowed:
            errors.append(
                f"{descriptor.relative}@{commit[:12]}: {'registration' if registration else 'evidence'} commit changes path outside its declared package: {path}"
            )
    return errors


def _registration_snapshot_errors(
    root: Path,
    descriptor: WorkingRecordDescriptor,
    text: str,
) -> list[str]:
    """Pre-registration inputs must be available before the record is created."""
    errors: list[str] = []
    parents = _commit_parents(root, descriptor.registration)
    if len(parents) != 1:
        return [f"{descriptor.relative}@{descriptor.registration[:12]}: L3 registration requires exactly one parent"]
    parent = parents[0]
    parsed = working_record_front_matter(text)
    assert parsed is not None
    bodies = section_bodies(text, WORKING_RECORD_REQUIRED_HEADINGS)
    values = {
        label: working_record_field_value(bodies[heading], label)
        for heading, labels in WORKING_RECORD_SECTION_FIELDS.items()
        for label in labels
    }
    if parsed[0].get("status") != "L3-open" or parsed[0].get("maturity") != "draft":
        errors.append(f"{descriptor.relative}@{descriptor.registration[:12]}: a new WRK must register as draft L3-open")
    reliance = working_record_field_value(
        bodies["## Results and review"], "Reliance status"
    )
    if reliance != "not-promoted":
        errors.append(f"{descriptor.relative}@{descriptor.registration[:12]}: L3 registration must be not-promoted")
    for location in descriptor.permitted_locations:
        if _git_text(root, "cat-file", "-t", f"{parent}:{location}") != "tree":
            errors.append(
                f"{descriptor.relative}@{descriptor.registration[:12]}: declared LAB location was not an existing directory before registration: {location}"
            )
    revisions = (
        _working_record_anchor_revisions(values.get("Canon anchors"))
        + _working_record_snapshot_revisions(values.get("LAB inputs"))
    )
    for revision in revisions:
        if not _git_is_ancestor(root, revision, parent):
            errors.append(f"{descriptor.relative}@{descriptor.registration[:12]}: pre-registration input does not predate registration: {revision}")
    errors.extend(_record_delta_errors(root, descriptor.registration, descriptor, registration=True))
    return errors


def _is_subsequence(prefix: tuple[str, ...], value: tuple[str, ...]) -> bool:
    iterator = iter(value)
    return all(any(candidate == expected for candidate in iterator) for expected in prefix)


def _working_record_history_errors(root: Path) -> list[str]:
    """Audit reachable WRK history as a DAG of immutable registrations and evidence."""
    commits = _reachable_commits(root)
    if not commits:
        return []
    states: dict[str, dict[str, WorkingRecordDescriptor]] = {}
    errors: list[str] = []
    for commit in commits:
        parents = _commit_parents(root, commit)
        inherited: dict[str, list[WorkingRecordDescriptor]] = {}
        for parent in parents:
            for identifier, descriptor in states.get(parent, {}).items():
                inherited.setdefault(identifier, []).append(descriptor)

        current: dict[str, WorkingRecordDescriptor] = {}
        current_text: dict[str, str] = {}
        for relative in _working_tree_paths_at(root, commit):
            within = PurePosixPath(relative).relative_to("mirrorea_canon/working")
            if within.as_posix() == "README.md":
                continue
            if len(within.parts) != 1 or not WORKING_RECORD_FILE_PATTERN.fullmatch(within.name):
                errors.append(f"{relative}@{commit[:12]}: working annex contains a non-record entry")
                continue
            raw = _git_bytes(root, "show", f"{commit}:{relative}")
            if raw is None:
                continue
            text = raw.decode("utf-8", errors="replace")
            descriptor, descriptor_errors = _working_record_descriptor(root, commit, relative, text)
            errors.extend(descriptor_errors)
            if descriptor is None:
                continue
            if descriptor.identifier in current:
                errors.append(f"{relative}@{commit[:12]}: duplicate historical WRK identity")
                continue
            current[descriptor.identifier] = descriptor
            current_text[descriptor.identifier] = text

        next_state: dict[str, WorkingRecordDescriptor] = {}
        for identifier, predecessors in inherited.items():
            baseline = predecessors[0]
            if any(
                candidate.relative != baseline.relative
                or candidate.preregistration != baseline.preregistration
                or candidate.permitted_locations != baseline.permitted_locations
                or candidate.registration != baseline.registration
                for candidate in predecessors[1:]
            ):
                errors.append(f"{identifier}@{commit[:12]}: merge has conflicting historical WRK identities")
            candidate = current.get(identifier)
            if candidate is None:
                errors.append(
                    f"{identifier}@{commit[:12]}: historical WRK identity is absent; records may not be renamed, reidentified, or deleted"
                )
                continue
            if (
                candidate.relative != baseline.relative
                or candidate.preregistration != baseline.preregistration
                or candidate.permitted_locations != baseline.permitted_locations
            ):
                errors.append(
                    f"{identifier}@{commit[:12]}: historical WRK identity or pre-registration changed"
                )
            if any(
                not _is_subsequence(previous.evidence_commits, candidate.evidence_commits)
                for previous in predecessors
            ):
                errors.append(f"{identifier}@{commit[:12]}: Evidence commits are not append-only")
            next_state[identifier] = WorkingRecordDescriptor(
                identifier=candidate.identifier,
                relative=baseline.relative,
                preregistration=baseline.preregistration,
                permitted_locations=baseline.permitted_locations,
                registration=baseline.registration,
                evidence_commits=candidate.evidence_commits,
                evidence_artifact_revisions=candidate.evidence_artifact_revisions,
            )

        for identifier, candidate in current.items():
            if identifier in inherited:
                continue
            errors.extend(_registration_snapshot_errors(root, candidate, current_text[identifier]))
            next_state[identifier] = candidate

        evidence_owner: dict[str, str] = {}
        for identifier, descriptor in next_state.items():
            for artifact_revision in descriptor.evidence_artifact_revisions:
                if artifact_revision not in descriptor.evidence_commits:
                    errors.append(
                        f"{descriptor.relative}@{commit[:12]}: Evidence artifacts must be owned by a listed Evidence commit: {artifact_revision}"
                    )
            for evidence_commit in descriptor.evidence_commits:
                owner = evidence_owner.setdefault(evidence_commit, identifier)
                if owner != identifier:
                    errors.append(f"{evidence_commit}: Evidence commit is claimed by both {owner} and {identifier}")
                if not _git_is_ancestor(root, descriptor.registration, evidence_commit) or evidence_commit == descriptor.registration:
                    errors.append(f"{descriptor.relative}@{commit[:12]}: Evidence commit must follow registration: {evidence_commit}")
                    continue
                if not _git_is_ancestor(root, evidence_commit, commit):
                    errors.append(f"{descriptor.relative}@{commit[:12]}: Evidence commit is not reachable at this record state: {evidence_commit}")
                    continue
                errors.extend(_record_delta_errors(root, evidence_commit, descriptor, registration=False))
        states[commit] = next_state
    return sorted(set(errors))


def _authoritative_worktree_errors(root: Path) -> list[str]:
    """A release-grade WRK audit must not accept uncommitted or ignored evidence."""
    outputs = (
        _git_text(root, "diff", "--name-only", "HEAD"),
        _git_text(root, "ls-files", "--others", "--exclude-standard"),
        _git_text(root, "ls-files", "--others", "--ignored", "--exclude-standard"),
    )
    disposable = {"target", ".lake", "__pycache__", ".pytest_cache", ".mypy_cache"}
    paths = {
        path
        for output in outputs
        if output is not None
        for path in output.splitlines()
        if not (set(PurePosixPath(path).parts) & disposable)
    }
    return [f"authoritative WRK audit requires a clean worktree: {path}" for path in sorted(paths)]


def _commit_parents(root: Path, commit: str) -> list[str]:
    line = _git_text(root, "rev-list", "--parents", "-n", "1", commit)
    if line is None:
        return []
    fields = line.split()
    return fields[1:]


def _committed_working_record_errors(root: Path, relative: str, text: str) -> list[str]:
    if _git_text(root, "rev-parse", "HEAD") is None:
        return [f"{relative}: working record requires a Git HEAD"]
    if _git_bytes(root, "show", f"HEAD:{relative}") != text.encode("utf-8"):
        return [f"{relative}: working record must be committed at HEAD"]
    return []


def _base_l3_errors(
    root: Path,
    relative: str,
    text: str,
    frozen_base: str,
    author_fingerprint: str,
) -> list[str]:
    content = _git_bytes(root, "show", f"{frozen_base}:{relative}")
    if content is None:
        return [f"{relative}: frozen base must contain the prior L3 record"]
    base_text = content.decode("utf-8", errors="replace")
    front_matter = working_record_front_matter(base_text)
    if front_matter is None:
        return [f"{relative}: frozen base has no WRK front matter"]
    fields, duplicates = front_matter
    if duplicates or fields.get("status") != "L3-open" or fields.get("maturity") != "draft":
        return [f"{relative}: frozen base must be a draft L3 record"]
    if fields.get("id") != working_record_front_matter(text)[0].get("id"):
        return [f"{relative}: frozen base record identity differs"]
    base_reliance = WORKING_RECORD_RELIANCE_PATTERN.findall(base_text)
    if base_reliance != ["not-promoted"]:
        return [f"{relative}: frozen base must retain Reliance status: not-promoted"]
    headings = (
        "## Classification and authority cut",
        "## Pre-registered working question",
        "## Method and evidence plan",
        "## Supersession",
    )
    if missing_headings(base_text, list(headings)):
        return [f"{relative}: frozen base lacks preserved pre-registration sections"]
    current_bodies = section_bodies(text, WORKING_RECORD_REQUIRED_HEADINGS)
    base_bodies = section_bodies(base_text, WORKING_RECORD_REQUIRED_HEADINGS)
    if any(current_bodies[heading] != base_bodies[heading] for heading in headings):
        return [f"{relative}: L2 admission rewrites pre-registration material"]
    author_value = working_record_field_value(
        base_bodies["## Classification and authority cut"], "Author fingerprint"
    )
    if author_value != author_fingerprint:
        return [f"{relative}: frozen base Author fingerprint differs"]
    return []


def _reviewed_record_errors(
    root: Path,
    relative: str,
    text: str,
    reviewer_fingerprint: str,
    author_fingerprint: str,
    frozen_base: str,
    expected_digest: str,
    admission: str | None = None,
) -> list[str]:
    errors: list[str] = []
    if not _git_commit_exists(root, frozen_base):
        return [f"{relative}: frozen base is not an existing Git commit"]
    digest = _normalized_working_record_digest(text)
    if digest != expected_digest:
        errors.append(f"{relative}: reviewed record SHA-256 does not match frozen material")
    if reviewer_fingerprint.upper() == author_fingerprint.upper():
        errors.append(f"{relative}: reviewer fingerprint must differ from Author fingerprint")
    trusted_keys = _trusted_review_keys(root)
    if trusted_keys is None:
        errors.append(f"{relative}: review-key registry is invalid")
    else:
        trusted_authors, trusted_reviewers = trusted_keys
        if author_fingerprint.upper() not in trusted_authors:
            errors.append(f"{relative}: Author fingerprint is not owner-trusted")
        if reviewer_fingerprint.upper() not in trusted_reviewers:
            errors.append(f"{relative}: reviewer fingerprint is not owner-trusted")
    errors.append(
        f"{relative}: L2 promotion is unavailable until an owner-authenticated trust anchor is configured"
    )
    if _signed_commit_fingerprint(root, frozen_base) != author_fingerprint.upper():
        errors.append(f"{relative}: frozen base must be signed by Author fingerprint")
    errors.extend(_base_l3_errors(root, relative, text, frozen_base, author_fingerprint))

    admission = admission or ( _record_history(root, relative) or [None] )[0]
    if admission is None or _git_bytes(root, "show", f"{admission}:{relative}") != text.encode("utf-8"):
        errors.append(f"{relative}: reviewed record must be its latest path revision")
        return errors
    parents = _commit_parents(root, admission)
    if parents != [frozen_base]:
        errors.append(
            f"{relative}: reviewed admission must have one parent equal to frozen base"
        )
    elif _signed_commit_fingerprint(root, admission) != reviewer_fingerprint.upper():
        errors.append(
            f"{relative}: admission commit requires a valid reviewer signature"
        )
    return errors


def _emergency_frozen_l2_errors(
    root: Path,
    relative: str,
    text: str,
    values: dict[str, str],
) -> list[str]:
    history = _record_history(root, relative)
    if not history or _git_bytes(root, "show", f"{history[0]}:{relative}") != text.encode("utf-8"):
        return [f"{relative}: emergency freeze must be the latest path revision"]
    parents = _commit_parents(root, history[0])
    if len(parents) != 1:
        return [f"{relative}: emergency freeze must have one direct parent"]
    active_text_bytes = _git_bytes(root, "show", f"{parents[0]}:{relative}")
    if active_text_bytes is None:
        return [f"{relative}: emergency freeze has no prior active L2 record"]
    active_text = active_text_bytes.decode("utf-8", errors="replace")
    if text != active_text.replace(
        "Reliance status: active", "Reliance status: frozen", 1
    ):
        return [f"{relative}: emergency freeze may change only Reliance status"]
    active_front_matter = working_record_front_matter(active_text)
    if (
        active_front_matter is None
        or active_front_matter[0].get("status") != "L2-working"
        or active_front_matter[0].get("maturity") != "reviewed"
    ):
        return [f"{relative}: emergency freeze requires a prior reviewed L2 record"]
    active_bodies = section_bodies(active_text, WORKING_RECORD_REQUIRED_HEADINGS)
    active_review = working_record_field_value(
        active_bodies["## Results and review"], "Independent review"
    )
    review_match = WORKING_RECORD_REVIEW_PATTERN.fullmatch(active_review or "")
    author_fingerprint = values.get("Author fingerprint", "")
    if review_match is None or not WORKING_RECORD_FINGERPRINT_PATTERN.fullmatch(
        author_fingerprint
    ):
        return [f"{relative}: emergency freeze has no valid prior review binding"]
    prior_admission = history[1] if len(history) > 1 else None
    return _reviewed_record_errors(
        root,
        relative,
        active_text,
        review_match.group(1),
        author_fingerprint,
        review_match.group(2),
        review_match.group(3),
        admission=prior_admission,
    )


def _historical_l2_exists(root: Path, relative: str) -> bool:
    for commit in _record_history(root, relative)[1:]:
        content = _git_bytes(root, "show", f"{commit}:{relative}")
        if content is None:
            continue
        parsed = working_record_front_matter(content.decode("utf-8", errors="replace"))
        if parsed is not None and parsed[0].get("status") == "L2-working":
            return True
    return False


def _l3_preregistration_history_errors(root: Path, relative: str, text: str) -> list[str]:
    historical_l3: str | None = None
    for commit in reversed(_record_history(root, relative)[1:]):
        content = _git_bytes(root, "show", f"{commit}:{relative}")
        if content is None:
            continue
        candidate = content.decode("utf-8", errors="replace")
        parsed = working_record_front_matter(candidate)
        if parsed is not None and parsed[0].get("status") == "L3-open":
            historical_l3 = candidate
            break
    if historical_l3 is None:
        return []
    headings = (
        "## Classification and authority cut",
        "## Pre-registered working question",
        "## Method and evidence plan",
    )
    if missing_headings(historical_l3, list(headings)):
        return [f"{relative}: prior L3 record lacks pre-registration sections"]
    current_bodies = section_bodies(text, WORKING_RECORD_REQUIRED_HEADINGS)
    historical_bodies = section_bodies(historical_l3, WORKING_RECORD_REQUIRED_HEADINGS)
    if any(current_bodies[heading] != historical_bodies[heading] for heading in headings):
        return [f"{relative}: L3 pre-registration may not be rewritten"]
    return []


def _deleted_working_record_errors(root: Path) -> list[str]:
    deleted = _git_text(
        root,
        "log",
        "--diff-filter=D",
        "--name-only",
        "--format=",
        "HEAD",
        "--",
        "mirrorea_canon/working",
    )
    if deleted is None:
        return []
    records = [
        path
        for path in deleted.splitlines()
        if WORKING_RECORD_FILE_PATTERN.fullmatch(Path(path).name)
    ]
    return [f"{path}: deleted WRK records are not allowed" for path in records]


def working_annex_errors(root: Path = ROOT, *, authoritative: bool = False) -> list[str]:
    working_root = root / "mirrorea_canon" / "working"
    errors: list[str] = []
    if not working_root.exists():
        errors.extend(_working_record_history_errors(root))
        if authoritative:
            errors.extend(_authoritative_worktree_errors(root))
        return errors

    seen_record_numbers: set[str] = set()
    for path in sorted(working_root.rglob("*")):
        if not path.is_file():
            continue
        relative = path.relative_to(root).as_posix()
        within_working = path.relative_to(working_root)
        if within_working.as_posix() == "README.md":
            continue
        if len(within_working.parts) != 1:
            errors.append(f"{relative}: nested working records are not allowed")
            continue
        if not WORKING_RECORD_FILE_PATTERN.fullmatch(path.name):
            errors.append(f"{relative}: working annex permits only README.md or WRK records")
            continue

        text = path.read_text(encoding="utf-8")
        filename_match = re.fullmatch(r"WRK-(\d{4})-[a-z0-9][a-z0-9-]*\.md", path.name)
        assert filename_match is not None
        record_number = filename_match.group(1)
        if record_number in seen_record_numbers:
            errors.append(f"{relative}: duplicate WRK-{record_number} record number")
        seen_record_numbers.add(record_number)

        parsed_front_matter = working_record_front_matter(text)
        if parsed_front_matter is None:
            errors.append(f"{relative}: missing front matter")
            continue
        front_matter, duplicate_fields = parsed_front_matter
        if duplicate_fields:
            errors.append(
                f"{relative}: duplicate front matter fields: {', '.join(sorted(duplicate_fields))}"
            )
        expected_id = f"working/WRK-{record_number}"
        if front_matter.get("id") != expected_id:
            errors.append(f"{relative}: id must be {expected_id}")
        status = front_matter.get("status")
        if status not in {"L3-open", "L2-working"}:
            errors.append(f"{relative}: front matter status must be L3-open or L2-working")
            continue
        expected_maturity = "reviewed" if status == "L2-working" else "draft"
        if front_matter.get("maturity") != expected_maturity:
            errors.append(
                f"{relative}: {status} requires front matter maturity {expected_maturity}"
            )

        missing = missing_headings(text, WORKING_RECORD_REQUIRED_HEADINGS)
        if missing:
            errors.append(f"{relative}: missing required sections: {', '.join(missing)}")
            continue
        if out_of_order_headings(text, WORKING_RECORD_REQUIRED_HEADINGS):
            errors.append(f"{relative}: required sections are out of order")
            continue

        bodies = section_bodies(text, WORKING_RECORD_REQUIRED_HEADINGS)
        values: dict[str, str] = {}
        for heading, labels in WORKING_RECORD_SECTION_FIELDS.items():
            for label in labels:
                value = working_record_field_value(bodies[heading], label)
                if value is None:
                    errors.append(f"{relative}: {heading} requires {label}")
                else:
                    values[label] = value

        anchors = values.get("Canon anchors")
        if anchors is not None and not WORKING_RECORD_CANON_ANCHORS_PATTERN.fullmatch(anchors):
            errors.append(f"{relative}: Canon anchors must use id@commit:blob-hash entries")
        lab_inputs = values.get("LAB inputs")
        if lab_inputs is not None and not WORKING_RECORD_LAB_SNAPSHOTS_PATTERN.fullmatch(
            lab_inputs
        ):
            errors.append(f"{relative}: LAB inputs must use LAB:path@commit:sha256 entries")
        if (
            anchors is not None
            and WORKING_RECORD_CANON_ANCHORS_PATTERN.fullmatch(anchors)
            and lab_inputs is not None
            and WORKING_RECORD_LAB_SNAPSHOTS_PATTERN.fullmatch(lab_inputs)
        ):
            errors.extend(_snapshot_digest_errors(root, values, relative))
        result_class = values.get("Result class")
        if result_class is not None and result_class not in WORKING_RECORD_RESULT_CLASSES:
            errors.append(f"{relative}: invalid Result class {result_class!r}")
        if values.get("Standing eligibility") != "pass":
            errors.append(f"{relative}: Standing eligibility must be pass")
        if values.get("Reserved surfaces") != "excluded":
            errors.append(f"{relative}: Reserved surfaces must be excluded")

        errors.extend(_committed_working_record_errors(root, relative, text))

        reliance_statuses = WORKING_RECORD_RELIANCE_PATTERN.findall(
            bodies["## Results and review"]
        )
        all_reliance_statuses = WORKING_RECORD_RELIANCE_PATTERN.findall(text)
        if len(reliance_statuses) != 1:
            errors.append(
                f"{relative}: require exactly one Reliance status marker in Results and review"
            )
            continue
        if len(all_reliance_statuses) != 1:
            errors.append(f"{relative}: Reliance status marker appears outside Results and review")
            continue

        reliance = reliance_statuses[0]
        permitted = (
            {"not-promoted", "frozen"}
            if status == "L3-open"
            else {"active", "frozen"}
        )
        if reliance not in permitted:
            errors.append(
                f"{relative}: Reliance status '{reliance}' is invalid for {status}"
            )
        if status == "L2-working":
            for label in (
                "Positive evidence",
                "Negative evidence",
                "Evidence artifacts",
                "Independent review",
            ):
                value = values.get(label, "").lower()
                if value in WORKING_RECORD_PENDING_VALUES:
                    errors.append(f"{relative}: {status} requires completed {label}")
            if reliance == "frozen":
                errors.extend(_emergency_frozen_l2_errors(root, relative, text, values))
            else:
                review_match = WORKING_RECORD_REVIEW_PATTERN.fullmatch(
                    values.get("Independent review", "")
                )
                if review_match is None:
                    errors.append(
                        f"{relative}: {status} requires reviewer-fingerprint=<40-hex>; frozen-base=<40-hex>; record-sha256=<64-hex>; decision=approved"
                    )
                else:
                    author_fingerprint = values.get("Author fingerprint", "")
                    if not WORKING_RECORD_FINGERPRINT_PATTERN.fullmatch(author_fingerprint):
                        errors.append(
                            f"{relative}: {status} requires Author fingerprint=<40-hex>"
                        )
                    else:
                        errors.extend(
                            _reviewed_record_errors(
                                root,
                                relative,
                                text,
                                review_match.group(1),
                                author_fingerprint,
                                review_match.group(2),
                                review_match.group(3),
                            )
                        )
        elif values.get("Independent review") != "not-required-for-L3":
            review_match = WORKING_RECORD_REVIEW_PATTERN.fullmatch(
                values.get("Independent review", "")
            )
            if review_match is None:
                errors.append(
                    f"{relative}: L3 review must be not-required-for-L3 or a frozen-base approval"
                )
            else:
                author_fingerprint = values.get("Author fingerprint", "")
                if not WORKING_RECORD_FINGERPRINT_PATTERN.fullmatch(author_fingerprint):
                    errors.append(
                        f"{relative}: reviewed L3 rollback requires Author fingerprint=<40-hex>"
                    )
                else:
                    errors.extend(
                        _reviewed_record_errors(
                            root,
                            relative,
                            text,
                            review_match.group(1),
                            author_fingerprint,
                            review_match.group(2),
                            review_match.group(3),
                        )
                    )
        elif _historical_l2_exists(root, relative):
            errors.append(
                f"{relative}: in-place L2-to-L3 demotion is prohibited; use a successor record"
            )
        else:
            errors.extend(_l3_preregistration_history_errors(root, relative, text))

    errors.extend(_working_record_history_errors(root))
    if authoritative:
        errors.extend(_authoritative_worktree_errors(root))
    return errors


def main(argv: list[str] | None = None) -> int:
    arguments = sys.argv[1:] if argv is None else argv
    authoritative_working_annex = "--authoritative-working-annex" in arguments
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

    working_errors = working_annex_errors(authoritative=authoritative_working_annex)
    if working_errors:
        print("Working annex records violate the WRK contract:")
        for error in working_errors:
            print(" -", error)
        return 1

    missing_notices = missing_canon_notices()
    if missing_notices:
        print("Root entry documents are missing canon notices:")
        for path, phrases in missing_notices.items():
            print(f" - {path}: missing {', '.join(phrases)}")
        return 1

    concrete_webhook_hits = concrete_discord_webhook_leaks()
    if concrete_webhook_hits:
        print("Tracked files contain concrete Discord webhook URLs:")
        for path, line_numbers in concrete_webhook_hits.items():
            for line_number in line_numbers:
                print(f" - {path}:{line_number}: concrete Discord webhook URL")
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
    duplicate_template_sections = duplicate_required_headings(
        template_text, REQUIRED_TEMPLATE_HEADINGS
    )
    if duplicate_template_sections:
        print("Report template has duplicate required sections:")
        for heading in duplicate_template_sections:
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
    duplicate_latest_report_sections = duplicate_required_headings(
        latest_report_text, REQUIRED_TEMPLATE_HEADINGS
    )
    if duplicate_latest_report_sections:
        print(f"Latest report has duplicate required sections: {latest_report.name}")
        for heading in duplicate_latest_report_sections:
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
    latest_report_bodies = required_section_bodies(latest_report_text)
    project_status_update_errors = project_status_update_status_errors(
        latest_report_bodies[PROJECT_STATUS_UPDATE_STATUS_HEADING],
        latest_report_bodies["## Files changed"],
    )
    if project_status_update_errors:
        print(f"Latest report has invalid project-status update status: {latest_report.name}")
        for error in project_status_update_errors:
            print(" -", error)
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

    project_status_text = (ROOT / "docs" / "project-status.md").read_text(
        encoding="utf-8"
    )
    project_status_h2_headings = re.findall(
        r"^## .+$", project_status_text, re.MULTILINE
    )
    if project_status_h2_headings != PROJECT_STATUS_REQUIRED_HEADINGS:
        print("Project status report must contain exactly the required sections in order:")
        for heading in project_status_h2_headings:
            print(" -", heading)
        return 1
    missing_project_status_sections = missing_headings(
        project_status_text, PROJECT_STATUS_REQUIRED_HEADINGS
    )
    if missing_project_status_sections:
        print("Project status report is missing required sections:")
        for heading in missing_project_status_sections:
            print(" -", heading)
        return 1
    out_of_order_project_status_sections = out_of_order_headings(
        project_status_text, PROJECT_STATUS_REQUIRED_HEADINGS
    )
    if out_of_order_project_status_sections:
        print("Project status report has required sections out of order:")
        for heading in out_of_order_project_status_sections:
            print(" -", heading)
        return 1
    missing_project_status_phrases = missing_project_status_guard_phrases()
    if missing_project_status_phrases:
        print("Project status report is missing required guard phrases:")
        for phrase in missing_project_status_phrases:
            print(" -", phrase)
        return 1
    project_status_source_errors = project_status_source_path_errors(project_status_text)
    if project_status_source_errors:
        print("Project status report has missing source paths or source sections:")
        for error in project_status_source_errors:
            print(" -", error)
        return 1
    checked_project_status_errors = checked_project_status_item_errors(
        project_status_text
    )
    if checked_project_status_errors:
        print("Checked project status items require a same-line canonical record path:")
        for error in checked_project_status_errors:
            print(" -", error)
        return 1
    project_status_lines = len(project_status_text.splitlines())
    if project_status_lines > PROJECT_STATUS_MAX_LINES:
        print(
            "Project status report exceeds the concise-view line budget: "
            f"{project_status_lines} > {PROJECT_STATUS_MAX_LINES}"
        )
        return 1

    snapshot_source_errors = snapshot_position_source_errors()
    if snapshot_source_errors:
        print("Snapshot docs are missing current-position source references:")
        for path, missing in snapshot_source_errors.items():
            print(f" - {path}: missing {', '.join(missing)}")
        return 1

    print("Documentation scaffold looks complete.")
    print(f"Found {len(reports)} numbered report(s).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
