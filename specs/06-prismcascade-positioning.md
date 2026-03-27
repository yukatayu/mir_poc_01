# 06 — PrismCascade Positioning

## Purpose of this document

This document explains where PrismCascade sits relative to Mir and Mirrorea.
It is not the full PrismCascade specification; it is the architectural positioning summary.

## Current conclusion

PrismCascade should be developed as a **separate kernel**, not as a sub-runtime inside Mir.

## Why

### PrismCascade has a different optimization center
Its core concerns include:
- effect-only graph normalization,
- planning before execution,
- explicit memory ownership and reuse,
- CPU/GPU/transfer scheduling,
- offline/live causality distinctions.

These concerns are different enough from Mir's primary concerns that merging the runtimes too early would likely distort both.

### PrismCascade still has strong integration points with Mir
Good integration points include:
- Meta-layer effect providers (for example TTS, model inference, asset lookup),
- remote execution or remote resource delegation,
- shared identifiers for tracing and audit,
- collaborative editing and synchronization around Prism graphs.

## Strong recommendation

- Develop PrismCascade independently.
- Define narrow, explicit integration points with Mir/Mirrorea.
- Do not force Prism runtime internals into Mir's general runtime model.

## Open integration questions

- Which effect contracts should be shared between Prism and Mir?
- What is the minimum shared trace schema?
- What is the smallest sensible remote execution unit for Prism?
