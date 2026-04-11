# Continuous Task Policy

## Purpose

この文書は、user から **複数の task package を続けて自走してほしい** と依頼されたときの
repo-local 運用を簡潔に固定する。

## Current policy

- 連続 task を始める前に、通常の Discord 運用と同様に `begin` を 1 回だけ実行する。
- `tasks.md` では、連続 task 全体の到達点、task package の順序、rough estimate、blocker を snapshot として明示する。
- 各 task package の close は **自然な checkpoint** とみなし、その時点で
  - user へ簡潔な中間報告を返す
  - `discord-report` skill で `progress` を送る
  を行ってよい。前回通知から 1 時間未満でもよい。
- `complete` は、連続 task 全体に対する最終返答の直前に 1 回だけ送る。
- task package close の中間報告では、少なくとも
  - 何を閉じたか
  - 何を検証したか
  - 次に進む package は何か
  を短く示す。
- 通知 failure は主作業の failure にしない。

## Scope note

- この運用は、短い単発 task まで一律に `progress` を増やすためのものではない。
- long-running research / implementation を **package 単位で ratchet する task** にだけ適用する。
