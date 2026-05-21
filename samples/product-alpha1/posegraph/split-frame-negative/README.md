# split-frame-negative

Helper-executable representative root for the negative split-frame mismatch row.

`package.mir.json` is consumed by `python3 scripts/posegraph_samples.py run pose-05-split-frame-negative --format json` and must return `violation_export` rather than stable acceptance when snapshot or `pose_version` drift is present.
