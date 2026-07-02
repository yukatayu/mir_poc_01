# ChatGPT Pro Oracle Operations

## Purpose

This document records repo-local operating guidance for consulting ChatGPT 5.5
Pro Extended through the browser with the installed Oracle wrappers.

This is an operational policy document, not normative project semantics.
Oracle answers are advisory review input. They do not replace:

- user decisions.
- `specs/` as normative source.
- `plan/` as repository memory.
- `progress.md` / `tasks.md` as current snapshots.
- `docs/reports/` as task evidence.

If an Oracle answer changes project understanding, mirror the distilled result
into the appropriate repo document. Do not leave important project state only in
an external chat transcript.

## Required manual

Before using Oracle for the first time in a session, read:

```text
/home/codex/.codex/docs/oracle-chatgpt-pro.md
```

The manual is outside this repo because it documents the local machine setup.
This repo document records how to use that setup for this project.

## When to consult

Use Oracle proactively when a second opinion is likely to improve quality,
especially for:

- difficult architectural or semantic judgments.
- theory-heavy questions.
- whole-project positioning or roadmap questions.
- review of a complex plan before it becomes normative.
- hidden-failure-mode searches.
- stuck investigations where local evidence is not enough.
- final challenge review for high-risk design or implementation changes.

Do not use Oracle as a shortcut around local reading, validation, or repo source
hierarchy. The local repo remains the primary evidence source.

For theory-heavy or whole-project tasks, prefer sending a focused Oracle consult
early enough that its answer can run in parallel with local reading, validation,
or implementation. Treat it as an asynchronous reviewer, not as a blocker for
all local progress unless the next local step genuinely depends on its answer.

## Commands

New consultation in the default ChatGPT Project:

```bash
ask-chatgpt-pro -p "State the goal, constraints, local evidence, and exact question."
```

Attach local context when the answer depends on it:

```bash
ask-chatgpt-pro \
  -p "Review this plan for hidden failure modes. Return concrete risks and fixes." \
  --file "plan/some-plan.md" \
  --file "docs/reports/some-report.md"
```

Continue an existing Oracle browser conversation:

```bash
oracle status --hours 24 --limit 20
ask-chatgpt-pro-followup <session-id> -p "Follow-up question based on the previous answer."
```

Use `ask-chatgpt-pro-followup` for normal back-and-forth. Do not use
`--browser-follow-up` for ordinary conversation continuation.

One-off temporary chat:

```bash
ask-chatgpt-pro-temp -p "One-off question"
```

Use temporary chat only when a one-off consult is explicitly desired. The
default should be `ask-chatgpt-pro` so related project consultations stay
grouped in the configured ChatGPT Project.

## Waiting policy

Browser Oracle runs can be slow. Treat them as long-running review jobs, not
short shell commands.

- Wait patiently in minute-scale intervals.
- A normal run may take a few minutes.
- Some high-quality Pro Extended answers may take up to about one hour.
- Do not start a duplicate run just because output is slow.
- If a run appears quiet, inspect status before deciding it is stuck:

```bash
oracle status --hours 24 --limit 20
oracle session <session-id>
```

Only abandon or retry when there is concrete evidence of failure, such as a
reported tool error, a browser/login interruption that cannot proceed, or a
session that has clearly stopped without producing usable output.

## Async and sub-agent coordination

It is acceptable to let a sub-agent operate the Oracle command when that keeps
the main task moving, especially for long-running theory review or broad-plan
critique. Use this pattern carefully:

1. The main agent defines the exact question, repo source hierarchy, attached
   files, and expected output shape.
2. The sub-agent starts or monitors the Oracle run and reports the session id,
   status, and distilled result.
3. The main agent continues non-overlapping local work while the Oracle run is
   pending.
4. The main agent reads the Oracle result critically, compares it against repo
   evidence, and decides what, if anything, to mirror into repo documents.

Do not allow an Oracle-running sub-agent to make normative repo changes from the
external answer alone. Promotion still requires the ordinary repo edit,
validation, report, commit, and push discipline.

## Prompting rules

Ask for critique, risks, and concrete alternatives. Include enough local context
to prevent the model from inventing a different project.

Good prompts usually include:

- objective.
- source hierarchy reminder.
- decision level or normative boundary.
- files attached or quoted.
- what has already been validated.
- exact open question.

When using Oracle for this repo, remind it that:

- `specs/` is normative source.
- `plan/` is non-normative repository memory.
- `progress.md` and `tasks.md` are current snapshots.
- reports are evidence trail.
- unresolved items must remain marked as unresolved.

## Handling results

After an Oracle consult:

1. Read the answer critically against local repo evidence.
2. Keep useful content as distilled project memory or report findings, not as a
   raw transcript dump.
3. Preserve advisory status unless a later explicit repo edit promotes the
   point through the normal source hierarchy.
4. Document the consult in the task report, including whether it was used, not
   used, skipped, or failed.
5. Never commit browser profiles, local Oracle state, credentials, webhooks, or
   temporary chat artifacts.

## Non-goals

- Oracle does not decide user decision gates.
- Oracle does not create normative `specs/` decisions by itself.
- Oracle does not replace tests, validators, local reviewers, or sub-agents.
- Oracle output should not be used to smuggle final public API, grammar,
  transport, runtime, viewer, telemetry, provider, or product claims into the
  repo.
