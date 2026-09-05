---
name: release
description: >-
  Cut a TimeTracker desktop release. Use when the user asks to release, bump
  the version, write release notes, tag vX.Y.Z, or ship a GitHub/Tauri update.
  A release is the version commit plus the vX.Y.Z tag; never stop after only
  committing the bump.
---

# Release TimeTracker

A release is **one workflow**: bump → changelog → commit → **tag**. Stopping after the commit is not a release. CI, GitHub Releases, and the in-app updater all key off the `v*` tag.

The tag is a git ref, not a source file. Do not write `vX.Y.Z` anywhere in the tree.

| What | Where | Example |
|---|---|---|
| App version | `src-tauri/tauri.conf.json` → `version` | `0.5.17` |
| Changelog heading | `CHANGELOG.md` → `## [0.5.17] - YYYY-MM-DD` | same number, no `v` |
| Git tag | annotated tag on the release commit | `v0.5.17` |
| CI | `.github/workflows/release.yml` on push of `v*` | draft GitHub release |

Do **not** bump `package.json` or `src-tauri/Cargo.toml` (`0.1.0` is unused).
Do **not** use `swift-v*` tags (parked native experiment).

The tag name is always `v` + `tauri.conf.json` version. The updater looks up notes by that number (`getChangelogSections` strips a leading `v`).

## Steps

Do all of these in the same turn. Do not ask whether to tag.

1. Confirm `main` and a clean tree (or only the intended release edits).
2. Read `src-tauri/tauri.conf.json` and `git tag -l 'v0.5.*'` for the current version.
3. Choose the next semver from commits since the last release tag:
   - user-facing fix → patch
   - user-facing feature → minor
   - breaking behavior → major
   - tooling-only → patch
4. Set `version` in `src-tauri/tauri.conf.json`.
5. Move `## [Unreleased]` notes in `CHANGELOG.md` to `## [X.Y.Z] - <today>` (Added / Changed / Fixed). Leave an empty `## [Unreleased]` above it. Date is today from the conversation.
6. Run `pnpm check`.
7. Commit **only** those two files as `chore: release vX.Y.Z`.
8. Tag **that** commit immediately:

```bash
git tag -a "vX.Y.Z" -m "vX.Y.Z"
```

9. Push `main` and the tag **only if the user asked to push or ship**:

```bash
git push origin main
git push origin "vX.Y.Z"
```

Never `--force` a release tag. Never tag a commit that is not the version bump. Never push the tag before `main` contains that commit.

## After the tag is pushed

- Actions builds Apple Silicon and Intel and opens a draft `TimeTracker vX.Y.Z`.
- Publishing the draft ships updater artifacts (`latest.json`).
- In-app notes come from `CHANGELOG.md`, not the GitHub release body.
