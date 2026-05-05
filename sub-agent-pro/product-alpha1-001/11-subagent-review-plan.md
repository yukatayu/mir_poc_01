# Sub-agent review plan

Use available sub-agents aggressively. If named agents are unavailable, perform self-review under the same headings and report that the sub-agent was unavailable.

## Theory reviewer

Focus:

- type/checker boundaries;
- model-check vs proof line;
- cut/savepoint semantics;
- message failure/recovery;
- no ad-hoc snapshot semantics.

Questions:

- Does the implementation obey finite checker/residual obligation split?
- Are `atomic_cut`, consistent cut, quiescent cut distinguished?
- Are savepoint classes explicit?

## Runtime reviewer

Focus:

- session continuity;
- mutation order;
- hot-plug accepted/rejected/deferred;
- message state;
- save/load/quiescent-save behavior.

Questions:

- Does state mutate before rejection?
- Does attach produce activation cut?
- Does quiescent-save prove no-inflight?
- Does the session carry enough state to be reproducible?

## Devtools/UX reviewer

Focus:

- clarity of viewer;
- redaction;
- event DAG / place graph / timeline readability.

Questions:

- Can a new developer tell what happened?
- Are failure reasons visible?
- Are raw secrets redacted?
- Are product-level non-goals obvious?

## Security/auth/native reviewer

Focus:

- auth stack;
- capability grants;
- native package policy;
- signature/provenance vs safety.

Questions:

- Is auth transparent only when safe?
- Are native package semantics overclaimed?
- Are capability/failure rows checked?

## Product boundary reviewer

Focus:

- public-ish entrypoint;
- developer guide;
- packaging;
- U1 choices;
- non-goals.

Questions:

- Can an outside developer reproduce the demo?
- Is product alpha overclaiming final API?
- Is `U1` properly separated or alpha-defaulted?

## Docs/source hierarchy reviewer

Focus:

- specs vs plan vs progress vs tasks;
- samples taxonomy;
- reports;
- validation commands.

Questions:

- Are normative decisions in `specs/`?
- Are roadmap memories in `plan/`?
- Are snapshots not inventing new rules?
- Is latest report complete?
